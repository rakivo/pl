#[derive(Debug, PartialEq)]
pub enum Expr {
    Int(i64),
    Flt(f64),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

impl Expr {
    pub fn eval_int(&self) -> i64 {
        match *self {
            Expr::Int(ival) => ival,
            Expr::Flt(fval) => fval as _,
            Expr::Add(ref lhs, ref rhs) => lhs.eval_int() + rhs.eval_int(),
            Expr::Sub(ref lhs, ref rhs) => lhs.eval_int() - rhs.eval_int(),
            Expr::Mul(ref lhs, ref rhs) => lhs.eval_int() * rhs.eval_int(),
            Expr::Div(ref lhs, ref rhs) => {
                let rval = rhs.eval_int();
                if rval == 0 {
                    0
                } else {
                    lhs.eval_int() / rval
                }
            }
        }
    }

    pub fn eval_flt(&self) -> f64 {
        match *self {
            Expr::Int(ival) => ival as _,
            Expr::Flt(fval) => fval,
            Expr::Add(ref lhs, ref rhs) => lhs.eval_flt() + rhs.eval_flt(),
            Expr::Sub(ref lhs, ref rhs) => lhs.eval_flt() - rhs.eval_flt(),
            Expr::Mul(ref lhs, ref rhs) => lhs.eval_flt() * rhs.eval_flt(),
            Expr::Div(ref lhs, ref rhs) => {
                let rval = rhs.eval_flt();
                if rval == 0.0 {
                    0.0
                } else {
                    lhs.eval_flt() / rval
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_expression() {
        let add_expr = Expr::Add(
            Box::new(Expr::Value(1)),
            Box::new(Expr::Add(
                Box::new(Expr::Value(2)),
                Box::new(Expr::Value(3)),
            )),
        );

        println!("{:?}", add_expr);
    }

    #[test]
    fn test_add_and_subexpression() {
        let add_sub_expr = Expr::Add(
            Box::new(Expr::Value(1)),
            Box::new(Expr::Sub(
                Box::new(Expr::Value(2)),
                Box::new(Expr::Value(3)),
            )),
        );

        println!("add_sub_expr = {:?}", add_sub_expr);

        let sub_add_expr = Expr::Sub(
            Box::new(Expr::Add(
                Box::new(Expr::Value(1)),
                Box::new(Expr::Value(2)),
            )),
            Box::new(Expr::Value(3)),
        );

        println!("sub_add_expr = {:?}", sub_add_expr);
    }
}

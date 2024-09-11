use crate::lexer::{Loc, Token};

#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Flt(f64)
}

#[derive(Debug, Clone)]
pub struct VarDecl<'a> {
    pub value: Value,
    pub name_token: Box::<Token<'a>>,
}

#[derive(Debug, Clone)]
pub struct FnCall<'a> {
    pub args: Vec::<Value>,
    pub name_token: Box::<Token<'a>>,
}

#[derive(Debug, Clone)]
pub enum AstKind<'a> {
    FnCall(Box::<FnCall<'a>>),
    VarDecl(Box::<VarDecl<'a>>)
}

#[derive(Debug, Clone)]
pub struct Ast<'a> {
    pub id: usize,
    pub loc: Box::<Loc>,
    pub kind: AstKind<'a>,
    pub next: usize,
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    Int(i64),
    Flt(f64),
    Add(Box::<Expr>, Box::<Expr>),
    Sub(Box::<Expr>, Box::<Expr>),
    Mul(Box::<Expr>, Box::<Expr>),
    Div(Box::<Expr>, Box::<Expr>),
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
                if rval == 0 { 0 } else { lhs.eval_int() / rval }
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
                if rval == 0.0 { 0.0 } else { lhs.eval_flt() / rval }
            }
        }
    }
}

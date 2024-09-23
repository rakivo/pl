use crate::parser::{Ctx, SymMap};
use crate::lexer::{Loc, Token, TokenKind};

#[derive(Debug, Clone)]
pub enum Type {
    I64, F64
}

impl Type {
    pub fn try_from_token(t: &Token) -> Result::<Self, ()> {
        if t.kind != TokenKind::Type { return Err(()) }
        match t.string {
            "i64" => Ok(Self::I64),
            "f64" => Ok(Self::F64),
            _ => Err(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct VarDecl<'a> {
    pub value: Box::<Expr<'a>>,
    pub name_token: Box::<Token<'a>>,
}

#[derive(Debug, Clone)]
pub struct FnCall<'a> {
    pub args: Vec::<Box::<Expr<'a>>>,
    pub name_token: Box::<Token<'a>>,
}

impl Type {
    pub fn to_il_str(&self) -> &'static str {
        match self {
            Self::I64 => "l",
            Self::F64 => "d",
        }
    }
}

#[derive(Debug, Clone)]
pub struct FnArg<'a> {
    pub ty: Type,
    pub name_token: Box::<Token<'a>>,
}

#[derive(Debug, Clone)]
pub struct Fn<'a> {
    pub ret_ty: Option::<Type>,
    pub args: Vec::<FnArg<'a>>,
    pub body: Vec::<Box::<Ast<'a>>>,
    pub name_token: Box::<Token<'a>>,
}

#[derive(Debug, Clone)]
pub enum AstKind<'a> {
    Fn(Box::<Fn<'a>>),
    FnCall(Box::<FnCall<'a>>),
    VarDecl(Box::<VarDecl<'a>>)
}

#[derive(Debug, Clone)]
pub struct Ast<'a> {
    pub id: usize,
    pub ctx: Box::<Ctx<'a>>,
    pub loc: Box::<Loc>,
    pub kind: AstKind<'a>,
    pub next: usize,
}

pub struct Asts<'a> {
    pub id: usize,
    pub asts: Vec::<Box::<Ast<'a>>>,
}

impl<'a> Asts<'a> {
    const RESERVE: usize = 1024;

    #[inline]
    pub fn new() -> Self {
        Self {
            id: 0,
            asts: Vec::with_capacity(Self::RESERVE),
        }
    }

    #[inline(always)]
    pub fn new_ast<'b>(&self, ctx: Box::<Ctx<'b>>, loc: Box::<Loc>, kind: AstKind<'b>) -> Ast<'b> {
        Ast {ctx, id: self.id, next: self.id + 1, loc, kind}
    }

    #[inline(always)]
    pub fn id(&self, id: usize) -> &Ast {
        unsafe { self.asts.get_unchecked(id) }
    }

    #[inline(always)]
    pub fn append_ast(&mut self, ast: Box::<Ast<'a>>) {
        self.asts.push(ast);
    }

    #[inline(always)]
    pub fn append(&mut self, ctx: Box::<Ctx<'a>>, loc: Box::<Loc>, kind: AstKind<'a>) {
        let ast = Ast {ctx, id: self.id, next: self.id + 1, loc, kind};
        self.asts.push(Box::new(ast));
        self.id += 1;
    }
}

#[derive(Debug, Clone)]
pub enum Expr<'a> {
    I64(i64),
    F64(f64),
    Lit(Box::<Token<'a>>),
    Add(Box::<Expr<'a>>, Box::<Expr<'a>>),
    Sub(Box::<Expr<'a>>, Box::<Expr<'a>>),
    Mul(Box::<Expr<'a>>, Box::<Expr<'a>>),
    Div(Box::<Expr<'a>>, Box::<Expr<'a>>),
}

impl<'a> Expr<'a> {
    pub fn eval_int(&self, sym_map: &SymMap) -> i64 {
        match self {
            Expr::I64(ival) => *ival,
            Expr::F64(fval) => *fval as _,
            Expr::Lit(..) => todo!(),
            Expr::Add(ref lhs, ref rhs) => lhs.eval_int(sym_map) + rhs.eval_int(sym_map),
            Expr::Sub(ref lhs, ref rhs) => lhs.eval_int(sym_map) - rhs.eval_int(sym_map),
            Expr::Mul(ref lhs, ref rhs) => lhs.eval_int(sym_map) * rhs.eval_int(sym_map),
            Expr::Div(ref lhs, ref rhs) => {
                let rval = rhs.eval_int(sym_map);
                if rval == 0 { 0 } else { lhs.eval_int(sym_map) / rval }
            }
        }
    }

    pub fn eval_flt(&self, sym_map: &SymMap) -> f64 {
        match *self {
            Expr::I64(ival) => ival as _,
            Expr::F64(fval) => fval,
            Expr::Lit(..) => todo!(),
            Expr::Add(ref lhs, ref rhs) => lhs.eval_flt(sym_map) + rhs.eval_flt(sym_map),
            Expr::Sub(ref lhs, ref rhs) => lhs.eval_flt(sym_map) - rhs.eval_flt(sym_map),
            Expr::Mul(ref lhs, ref rhs) => lhs.eval_flt(sym_map) * rhs.eval_flt(sym_map),
            Expr::Div(ref lhs, ref rhs) => {
                let rval = rhs.eval_flt(sym_map);
                if rval == 0.0 { 0.0 } else { lhs.eval_flt(sym_map) / rval }
            }
        }
    }
}

use crate::lexer::{Loc, Token};

#[derive(Debug, Clone)]
pub enum VarValue {
    Int(i64),
    Flt(f64)
}

#[derive(Debug, Clone)]
pub struct VarDecl<'a> {
    pub value: VarValue,
    pub name_token: Box::<Token<'a>>,
}

#[derive(Debug, Clone)]
pub enum AstKind<'a> {
    Poisoned,
    VarDecl(Box::<VarDecl<'a>>)
}

#[derive(Debug, Clone)]
pub struct Ast<'a> {
    pub id: usize,
    pub loc: Box::<Loc>,
    pub kind: AstKind<'a>,
    pub next: usize,
}

impl Ast<'_> {
    pub fn alloc_poisoned() -> Self {
        Ast {
            id: 0,
            kind: AstKind::Poisoned,
            loc: unsafe { Box::from_raw(0 as _) },
            next: 0,
        }
    }
}

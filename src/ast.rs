use crate::lexer::Token;

#[repr(u8)]
#[derive(Debug)]
pub enum Value {
    I32(i32) = 4,
}

#[derive(Debug)]
pub struct VarDecl<'a> {
    pub v: Value,
    pub name_token: Box::<Token<'a>>,
}

#[derive(Debug)]
pub enum AstKind<'a> {
    VarDecl(VarDecl<'a>)
}

#[derive(Debug)]
pub struct Ast<'a> {
    pub loc_id: usize,
    pub ast_id: usize,
    pub next_id: usize,
    pub ast_kind: AstKind<'a>
}

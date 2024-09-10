use crate::lexer::Token;

#[derive(Debug)]
pub enum OpKind<'a> {
    Sum(Box::<Expr<'a>>, Box::<Expr<'a>>)
}

#[derive(Debug)]
pub enum Expr<'a> {
    Op(OpKind<'a>),
    Expr(Box::<Expr<'a>>),
    Int(Box::<Token<'a>>),
    Lit(Box::<Token<'a>>)
}

#[derive(Debug)]
pub struct VarDecl<'a> {
    pub value: Expr<'a>,
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
    pub kind: AstKind<'a>
}

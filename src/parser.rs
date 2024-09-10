use crate::{
    ctx::RefCtx,
    append_ast,
    last_astid,
    locid_to_string,
    ast::{
        Ast,
        Expr,
        OpKind,
        AstKind,
        VarDecl,
    },
    lexer::{
        Token,
        Tokens,
        Tokens2D,
        TokenKind,
    }
};

use std::process::exit;

pub struct Parser<'a> {
    ctx: &'a RefCtx<'a>,
    curr_line: Tokens<'a>,
}

enum Item<'a> {
    Expr(Expr<'a>),
    Int(Box::<Token<'a>>),
    Lit(Box::<Token<'a>>),
}

impl<'a> Parser<'a> {
    #[inline(always)]
    pub fn new(ctx: &'a RefCtx<'a>, dummy: Tokens<'a>) -> Self {
        Self {
            ctx,
            curr_line: dummy,
        }
    }

    #[inline]
    fn type_check_token<F, E>(&self, idx: usize, cond: F, err: E) -> &Token<'a>
    where
        F: FnOnce(&Token) -> bool,
        E: FnOnce((&'a str, usize))
    {
        if let Some(t) = self.curr_line.get(idx) {
            if cond(t) { t } else { err((t.string, t.loc_id)); exit(1) }
        } else { err(("<eof>", self.curr_line[idx].loc_id)); exit(1) }
    }

    #[inline]
    fn type_check_token_owned<F, E>(&self, idx: usize, cond: F, err: E) -> Box::<Token<'a>>
    where
        F: FnOnce(&Token) -> bool,
        E: FnOnce((&'a str, usize))
    {
        if let Some(t) = self.curr_line.get(idx) {
            if cond(t) {
                self.curr_line[idx].to_owned()
            } else { err((t.string, t.loc_id)); exit(1) }
        } else { err(("<eof>", self.curr_line[idx].loc_id)); exit(1) }
    }

    fn parse_expr(&self, tokens: Vec::<&Box::<Token::<'a>>>) -> Expr<'a> {
        fn parse_term<'a>(token: &Box::<Token<'a>>) -> Expr<'a> {
            match token.kind {
                TokenKind::Int => Expr::Int(token.to_owned()),
                TokenKind::Lit => Expr::Lit(token.to_owned()),
                _ => panic!("Expected a term, found: {:?}", token.kind),
            }
        }

        fn parse_sum<'a>(ctx: &RefCtx, tokens: Vec::<&Box::<Token<'a>>>, mut i: usize) -> (Expr<'a>, usize) {
            let mut expr = parse_term(&tokens[i]);
            i += 1;

            while i < tokens.len() {
                let token = &tokens[i];
                match token.kind {
                    TokenKind::Plus => {
                        i += 1;
                        if i >= tokens.len() {
                            panic!("{l} error: unexpected <eof> after `+`",
                                   l = locid_to_string!(ctx, token.loc_id));
                        }
                        let rhs = parse_term(&tokens[i]);
                        expr = Expr::Op(OpKind::Sum(Box::new(expr), Box::new(rhs)));
                    }
                    _ => break
                }
                i += 1;
            }

            (expr, i)
        }

        parse_sum(&self.ctx, tokens, 0).0
    }

    fn parse_decl(&self, idx: &mut usize) -> VarDecl<'a> {
        *idx += 1;

        let name_token = self.type_check_token_owned(*idx, |t| {
            matches!(t.kind, TokenKind::Lit)
        }, |(string, loc_id)| {
            panic!("{l} error: expected literal after the type, but got: {string}",
                   l = locid_to_string!(self.ctx, loc_id))
        });

        *idx += 1;

        let eq_token = self.type_check_token(*idx, |t| {
            matches!(t.kind, TokenKind::Equal)
        }, |(string, loc_id)| {
            panic!("{l} error: expected equal after the name, but got: {string}",
                   l = locid_to_string!(self.ctx, loc_id))
        });

        *idx += 1;

        if *idx > self.curr_line.len() {
            let s = self.curr_line.get(1).map(|t| t.string).unwrap_or("<eof>");
            panic!("{l} error: expected expr after the equal sign, but got: {s}",
                   l = locid_to_string!(self.ctx, eq_token.loc_id))
        }

        let expr_tokens = self.curr_line[*idx..].iter()
            .take_while(|t| !matches!(t.kind, TokenKind::Semicolon))
            .collect();

        let value = self.parse_expr(expr_tokens);
        VarDecl { value, name_token }
    }

    fn parse_line(&mut self) {
        let mut idx = 0;
        while idx < self.curr_line.len() {
            let ref token = self.curr_line[idx];
            match token.kind {
                TokenKind::Type => {
                    let decl = self.parse_decl(&mut idx);
                    let ast = Ast {
                        loc_id: token.loc_id,
                        ast_id: last_astid!(self.ctx),
                        next_id: last_astid!(self.ctx) + 1,
                        kind: AstKind::VarDecl(decl),
                    };

                    append_ast!(self.ctx, ast);
                },
                _ => idx += 1
            }
        }
    }

    #[inline(always)]
    pub fn parse(&mut self, tokens: Tokens2D<'a>) {
        for line in tokens {
            self.curr_line = line;
            self.parse_line();
        }
    }
}

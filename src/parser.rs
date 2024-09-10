use crate::{
    ctx::RefCtx,
    append_ast,
    last_astid,
    locid_to_string,
    ast::{
        Ast,
        Value,
        AstKind,
        VarDecl
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

impl<'a> Parser<'a> {
    #[inline(always)]
    pub fn new(ctx: &'a RefCtx<'a>, dummy: Tokens<'a>) -> Self {
        Self {
            ctx,
            curr_line: dummy,
        }
    }

    fn type_check_token<F, E>(&self, idx: usize, cond: F, err: E) -> Box::<Token<'a>>
    where
        F: FnOnce(&Token) -> bool,
        E: FnOnce((&'a str, usize))
    {
        if let Some(t) = self.curr_line.get(idx) {
            if cond(t) {
                Box::new(self.curr_line[idx])
            } else {
                err((t.string, t.loc_id)); exit(1)
            }
        } else { err(("<eof>", self.curr_line[idx].loc_id)); exit(1) }
    }

    fn parse_decl(&self, idx: &mut usize) -> VarDecl<'a> {
        let ref ty_token = self.curr_line[*idx];

        *idx += 1;

        let name_token = self.type_check_token(*idx, |t| {
            matches!(t.kind, TokenKind::Literal)
        }, |(string, loc_id)| {
            panic!("{l} error: expected literal after the type, but got: {string}",
                   l = locid_to_string!(self.ctx, loc_id))
        });

        *idx += 1;

        let _ = self.type_check_token(*idx, |t| {
            matches!(t.kind, TokenKind::Equal)
        }, |(string, loc_id)| {
            panic!("{l} error: expected equal after the name, but got: {string}",
                   l = locid_to_string!(self.ctx, loc_id))
        });

        *idx += 1;

        let v_token = self.type_check_token(*idx, |t| {
            matches!(t.kind, TokenKind::Integer)
        }, |(string, loc_id)| {
            panic!("{l} error: expected integer after the equal, but got: {string}",
                   l = locid_to_string!(self.ctx, loc_id))
        });

        let v = match ty_token.string {
            "i32" => {
                let int = v_token.string.parse::<i32>()
                    .expect("failed to parse to integer bruv");

                Value::I32(int)
            },
            _ => todo!()
        };

        *idx += 1;

        VarDecl { v, name_token }
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
                        ast_kind: AstKind::VarDecl(decl),
                    };

                    append_ast!(self.ctx, ast);
                },
                _ => idx += 1
            }
        }
    }

    pub fn parse(&mut self, tokens: Tokens2D<'a>) {
        for line in tokens {
            self.curr_line = line;
            self.parse_line();
        }
    }
}

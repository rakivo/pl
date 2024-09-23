use super::ast::Expr;
use crate::{ast::AstKind, SymMap, Token, TokenKind, TokensRefs};

pub struct ExprParser<'a> {
    curr_idx: usize,
    curr_token: Box::<Token<'a>>,
    sym_map: &'a SymMap<'a>,
    tokens: TokensRefs<'a>,
}

impl<'a> ExprParser<'a> {
    #[inline]
    pub fn new(tokens: TokensRefs<'a>, sym_map: &'a SymMap<'a>) -> Self {
        ExprParser {
            curr_token: tokens[0].to_owned(),
            tokens,
            sym_map,
            curr_idx: 1,
        }
    }

    #[inline]
    fn accept_it(&mut self) -> bool {
        if self.curr_idx == self.tokens.len() {
            true
        } else {
            self.curr_token = self.tokens[self.curr_idx].to_owned();
            self.curr_idx += 1;
            false
        }
    }

    fn accept(&mut self, tt: TokenKind) {
        if self.curr_token.kind != tt {
            panic! {
                "expected to accept token of kind {:?}, got token of kind {:?}",
                tt,
                self.curr_token.kind,
            };
        }

        self.accept_it();
    }

    // start ::= expr
    pub fn parse(&mut self) -> Box::<Expr<'a>> {
        let program_ast = self.parse_expr();
        assert!(self.tokens.len() == self.curr_idx);
        program_ast
    }

    // expr ::= term (+ expr | - expr | epsilon)
    fn parse_expr(&mut self) -> Box::<Expr<'a>> {
        let term_ast = self.parse_term();

        match self.curr_token.kind {
            TokenKind::Plus => {
                self.accept_it();
                let expr_ast = self.parse_expr();
                Box::new(Expr::Add(term_ast, expr_ast))
            }

            TokenKind::Minus => {
                self.accept_it();
                let expr_ast = self.parse_expr();
                Box::new(Expr::Sub(term_ast, expr_ast))
            }

            _ => term_ast,
        }
    }

    // term ::= factor (* term | / term | epsilon)
    fn parse_term(&mut self) -> Box::<Expr<'a>> {
        let factor_ast = self.parse_factor();

        match self.curr_token.kind {
            TokenKind::Asterisk => {
                self.accept_it();
                let term_ast = self.parse_term();
                Box::new(Expr::Mul(factor_ast, term_ast))
            }

            TokenKind::Slash => {
                self.accept_it();
                let term_ast = self.parse_term();
                Box::new(Expr::Div(factor_ast, term_ast))
            }

            _ => factor_ast,
        }
    }

    // factor ::= ( expr ) | integer
    fn parse_factor(&mut self) -> Box::<Expr<'a>> {
        match self.curr_token.kind {
            TokenKind::LParen => {
                self.accept_it();
                let expr_ast = self.parse_expr();
                self.accept(TokenKind::RParen);
                expr_ast
            }

            TokenKind::Int => Box::new(Expr::I64(self.get_int())),
            TokenKind::Flt => Box::new(Expr::F64(self.get_flt())),

            TokenKind::Lit => if let Some(ref sym) = self.sym_map.get(self.curr_token.string) {
                self.accept_it();
                match &sym.kind {
                    AstKind::VarDecl(vd) => vd.value.to_owned(),
                    _ => todo!()
                }
            } else {
                panic!("{loc} error: undefined symbol: {string}",
                       loc = self.curr_token.loc,
                       string = self.curr_token.string)
            }

            _ => panic! {
                "`parse_factor`: unexpected token: `{}` of kind {:?}.",
                self.curr_token.string,
                self.curr_token.kind
            }
        }
    }

    // integer ::= ... -2 | -1 | 0 | 1 | 2 ...
    fn get_int(&mut self) -> i64 {
        if self.curr_token.kind == TokenKind::Int {
            let ret = self.curr_token.string.parse().unwrap();
            self.accept_it();
            ret
        } else {
            panic!("token is not an int")
        }
    }

    fn get_flt(&mut self) -> f64 {
        if self.curr_token.kind == TokenKind::Flt {
            let ret = self.curr_token.string.parse().unwrap();
            self.accept_it();
            ret
        } else {
            panic!("token is not a float")
        }
    }
}

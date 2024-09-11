use super::ast::Expr;
use crate::{Token, TokenKind, TokensRefs, VarMap, ast::VarValue};

pub struct ExprParser<'a> {
    curr_idx: usize,
    curr_token: Box::<Token<'a>>,
    var_map: &'a VarMap<'a>,
    tokens: &'a TokensRefs<'a>,
}

impl<'a> ExprParser<'a> {
    #[inline]
    pub fn new(tokens: &'a TokensRefs<'a>, var_map: &'a VarMap) -> Self {
        ExprParser {
            tokens,
            var_map,
            curr_idx: 1,
            curr_token: tokens[0].to_owned(),
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
    pub fn parse(&mut self) -> Expr {
        let program_ast = self.parse_expr();
        assert!(self.tokens.len() == self.curr_idx);
        program_ast
    }

    // expr ::= term (+ expr | - expr | epsilon)
    fn parse_expr(&mut self) -> Expr {
        let term_ast = self.parse_term();

        match self.curr_token.kind {
            TokenKind::Plus => {
                self.accept_it();
                let expr_ast = self.parse_expr();
                Expr::Add(Box::new(term_ast), Box::new(expr_ast))
            }

            TokenKind::Minus => {
                self.accept_it();
                let expr_ast = self.parse_expr();
                Expr::Sub(Box::new(term_ast), Box::new(expr_ast))
            }

            TokenKind::Poisoned => panic!("`parse_expr`: got an illegal value."),

            _ => term_ast,
        }
    }

    // term ::= factor (* term | / term | epsilon)
    fn parse_term(&mut self) -> Expr {
        let factor_ast = self.parse_factor();

        match self.curr_token.kind {
            TokenKind::Asterisk => {
                self.accept_it();
                let term_ast = self.parse_term();
                Expr::Mul(Box::new(factor_ast), Box::new(term_ast))
            }

            TokenKind::Slash => {
                self.accept_it();
                let term_ast = self.parse_term();
                Expr::Div(Box::new(factor_ast), Box::new(term_ast))
            }

            TokenKind::Poisoned => panic!("`parse_term`: got an illegal value."),

            _ => factor_ast,
        }
    }

    // factor ::= ( expr ) | integer
    fn parse_factor(&mut self) -> Expr {
        match self.curr_token.kind {
            TokenKind::LParen => {
                self.accept_it();
                let expr_ast = self.parse_expr();
                self.accept(TokenKind::RParen);
                expr_ast
            }

            TokenKind::Int => Expr::Int(self.get_int()),
            TokenKind::Flt => Expr::Flt(self.get_flt()),

            TokenKind::Lit => if let Some(vd) = self.var_map.get(self.curr_token.string) {
                self.accept_it();
                match vd.value {
                    VarValue::Int(ival) => Expr::Int(ival),
                    VarValue::Flt(fval) => Expr::Flt(fval)
                }
            } else {
                panic!("{loc} error: undefined symbol: {name}",
                       loc = self.curr_token.loc,
                       name = self.curr_token.string);
            }

            _ => panic! {
                "`parse_factor`: unexpected token of kind {:?}.",
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

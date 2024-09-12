use crate::{
    expr_parser::ExprParser,
    lexer::{Loc, Token, TokenKind, Tokens, Tokens2D},
    ast::{Ast, Fn, Asts, Type, FnArg, AstKind, FnCall, Value, VarDecl},
};

use std::process::exit;
use std::collections::HashMap;

pub type VarMap<'a> = HashMap::<&'a str, Box::<VarDecl<'a>>>;

pub struct Parser<'a, 'b> {
    pub asts: Asts<'a>,
    line_cur: usize,
    idx: usize,
    // current line
    cl: &'b Tokens<'a>,
    tokens: &'b Tokens2D<'a>,
    var_map: VarMap<'a>
}

impl<'a, 'b> Parser<'a, 'b> {
    #[inline]
    pub fn new(tokens: &'b Tokens2D<'a>) -> Self {
        Self {
            asts: Asts::new(),
            tokens,
            cl: &tokens[0],
            idx: 0,
            line_cur: 0,
            var_map: HashMap::new()
        }
    }

    #[inline]
    fn type_check_token<F, E>(&self, cond: F, err: E) -> &Box::<Token<'a>>
    where
        F: FnOnce(&Token) -> bool,
        E: FnOnce(&'a str, &Box::<Loc>)
    {
        if let Some(t) = self.cl.get(self.idx) {
            if cond(t) { t } else { err(t.string, &t.loc); exit(1) }
        } else { err("<eof>", &self.cl[self.idx].loc); exit(1) }
    }

    #[inline]
    fn type_check_token_owned<F, E>(&self, cond: F, err: E) -> Box::<Token<'a>>
    where
        F: FnOnce(&Token) -> bool,
        E: FnOnce(&'a str, &Box::<Loc>)
    {
        if let Some(t) = self.cl.get(self.idx) {
            if cond(t) {
                self.cl[self.idx].to_owned()
            } else { err(t.string, &t.loc); exit(1) }
        } else { err("<eof>", &self.cl[self.idx].loc); exit(1) }
    }

    #[inline]
    fn advance(&mut self) {
        self.idx += 1;
        if self.idx >= self.cl.len() {
            self.idx = 0;
            self.line_cur += 1;
            self.cl = &self.tokens[self.line_cur];
        }
    }

    fn parse_decl(&mut self) -> VarDecl<'a> {
        let ref ty_token = self.cl[self.idx];

        self.advance();

        let name_token = self.type_check_token_owned(|t| {
            matches!(t.kind, TokenKind::Lit)
        }, |string, loc| {
            panic!("{loc} error: expected literal after the type, but got: {string}")
        });

        self.advance();
        self.advance();

        let mut expr_tokens = Vec::with_capacity(self.cl.len() - self.idx);
        while self.cl[self.idx].kind != TokenKind::Semicolon {
            expr_tokens.push(&self.cl[self.idx]);
            self.advance();
        }

        let _ = self.type_check_token(|t| {
            matches!(t.kind, TokenKind::Semicolon)
        }, |string, loc| {
            panic!("{loc} error: expected semicolon after decl, but got: {string}")
        });

        self.advance();

        match ty_token.kind {
            TokenKind::IntType => VarDecl {
                name_token,
                value: Value::Int(ExprParser::new(expr_tokens, &self.var_map).parse().eval_int()),
            },
            TokenKind::FltType => VarDecl {
                name_token,
                value: Value::Flt(ExprParser::new(expr_tokens, &self.var_map).parse().eval_flt()),
            },
            _ => unreachable!()
        }
    }

    fn parse_fn_call(&mut self) -> FnCall<'a> {
        let name_token = self.cl[self.idx].to_owned();
        self.advance();

        if !matches! {
            self.cl.get(self.idx),
            Some(t) if t.kind == TokenKind::LParen
        } {
            panic!("{loc} expected lparen bruv",
                   loc = name_token.loc);
        }

        self.advance();

        let mut args = Vec::with_capacity(self.cl.len() - self.idx);
        while self.idx < self.cl.len() {
            let ref t = self.cl[self.idx];
            let value = match t.kind {
                TokenKind::RParen => break,
                TokenKind::Int => Value::Int(ExprParser::new(vec![t], &self.var_map).parse().eval_int()),
                TokenKind::Flt => Value::Flt(ExprParser::new(vec![t], &self.var_map).parse().eval_flt()),
                TokenKind::Lit => if let Some(lit) = self.var_map.get(t.string) {
                    lit.value.to_owned()
                } else {
                    panic!("{loc} undefined symbol: {string}",
                            loc = t.loc, string = t.string)
                }
                _ => panic!("{loc} expected int or float bruv, but got: {got}",
                            loc = t.loc, got = t.string)
            };

            self.idx += 1;
            if self.idx + 1 < self.cl.len() {
                _ = self.type_check_token(|t| {
                    t.kind == TokenKind::Comma
                }, |string, loc| {
                    panic!("{loc} expected comma after an argument, but got: {string}")
                });
                self.idx += 1;
            }

            args.push(value);
        }

        if !matches! {
            self.cl.get(self.idx),
            Some(t) if t.kind == TokenKind::RParen
        } {
            panic!("{loc} rparen was not met bruv", loc = name_token.loc);
        }

        FnCall {args, name_token}
    }

    fn parse_fn(&mut self) -> Fn<'a> {
        let ref fn_token = self.cl[self.idx];

        self.advance();

        let name_token = self.type_check_token_owned(|t| {
            matches!(t.kind, TokenKind::Lit)
        }, |string, loc| {
            panic!("{loc} error: expected literal after the fn keyword, but got: {string}")
        });

        self.advance();

        if !matches! {
            self.cl.get(self.idx),
            Some(t) if t.kind == TokenKind::LParen
        } {
            panic!("{loc} expected lparen bruv",
                   loc = name_token.loc);
        }

        self.advance();

        let mut args = Vec::with_capacity(self.cl.len() - self.idx);
        while self.idx < self.cl.len() {
            let ref t = self.cl[self.idx];
            let ty = match t.kind {
                TokenKind::RParen  => break,
                TokenKind::IntType => Type::I64,
                TokenKind::FltType => Type::F64,
                _ => panic!("{loc} expected type but got: {string}",
                            loc = t.loc, string = t.string)
            };

            self.advance();
            let name_token = self.type_check_token_owned(|t| {
                matches!(t.kind, TokenKind::Lit)
            }, |string, loc| {
                panic!("{loc} error: expected literal after the type, but got: {string}")
            });

            self.advance();
            let arg = FnArg { ty, name_token };
            args.push(arg);
        }

        if !matches! {
            self.cl.get(self.idx),
            Some(t) if t.kind == TokenKind::RParen
        } {
            panic!("{loc} rparen was not met bruv", loc = name_token.loc);
        }

        self.advance();

        let ret_ty = if self.cl[self.idx].kind == TokenKind::LCurly {
            None
        } else {
            let _ = self.type_check_token_owned(|t| {
                matches!(t.kind, TokenKind::Minus)
            }, |string, loc| {
                panic!("{loc} error: Expected arrow after rparen, but got: {string}")
            });

            self.advance();

            let _ = self.type_check_token_owned(|t| {
                matches!(t.kind, TokenKind::RAngleBracket)
            }, |string, loc| {
                panic!("{loc} error: Expected arrow after rparen, but got: {string}")
            });

            self.advance();

            let ty = match self.cl[self.idx].kind {
                TokenKind::IntType => Type::I64,
                TokenKind::FltType => Type::F64,
                _ => panic!("{loc} error: Expected arrow after rparen, but got: {string}",
                            loc = self.cl[self.idx].loc,
                            string = self.cl[self.idx].string)
            };

            self.advance();
            Some(ty)
        };

        while self.cl.is_empty() {
            self.advance();
        }

        let ref lcurly_token = 'lcurly: loop {
            if self.cl.is_empty() {
                panic!("{loc} expected `{{` bruv",
                       loc = name_token.loc);
            }

            let ref t = self.cl[self.idx];
            self.advance();
            if t.kind == TokenKind::LCurly {
                break 'lcurly t;
            }
        };

        while self.cl.is_empty() {
            self.advance();
        }

        let curr_id = self.asts.id;
        let mut body = Asts::new();
        while !self.parse_line_buf(true, &mut body) {
            self.line_cur += 1;
            if self.line_cur == self.tokens.len() {
                panic!("{loc} eww no rcurly matched bruv",
                       loc = lcurly_token.loc)
            }
            self.cl = &self.tokens[self.line_cur];
        }

        self.advance();

        Fn { ret_ty, body: body.asts, args, name_token }
    }

    fn parse_line_buf(&mut self, expect_rcurly: bool, asts_buf: &mut Asts<'a>) -> bool {
        while self.idx < self.cl.len() {
            let ref token = self.cl[self.idx];
            match token.kind {
                TokenKind::RCurly => if expect_rcurly {
                    return true
                } else {
                    panic!("{loc} unexpected `}}` bruv",
                           loc = token.loc)
                }
                TokenKind::Fn => {
                    let fn_ = Box::new(self.parse_fn());
                    asts_buf.append(token.loc.to_owned(), AstKind::Fn(fn_));
                }
                TokenKind::Lit => {
                    let fcall = Box::new(self.parse_fn_call());
                    asts_buf.append(token.loc.to_owned(), AstKind::FnCall(fcall));
                }
                TokenKind::IntType | TokenKind::FltType => {
                    let decl = Box::new(self.parse_decl());
                    self.var_map.insert(decl.name_token.string, decl.to_owned());
                    asts_buf.append(token.loc.to_owned(), AstKind::VarDecl(decl));
                }
                _ => self.idx += 1
            }
        } false
    }

    fn parse_line(&mut self, expect_rcurly: bool) -> bool {
        let mut asts = Asts::new();
        let met = self.parse_line_buf(expect_rcurly, &mut asts);
        for ast in asts.asts {
            self.asts.append(ast.loc, ast.kind);
        }
        met
    }

    #[inline(always)]
    pub fn parse(&mut self) {
        while self.line_cur < self.tokens.len() {
            self.cl = &self.tokens[self.line_cur];
            self.parse_line(false);
            self.line_cur += 1;
        }
    }
}

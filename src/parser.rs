use crate::{
    expr_parser::ExprParser,
    lexer::{Loc, Token, TokenKind, Tokens},
    ast::{Ast, AstKind, Asts, Expr, Fn, FnArg, FnCall, Type, VarDecl},
};

use std::process::exit;
use std::collections::HashMap;

pub type SymMap<'a> = HashMap::<&'a str, Box::<Ast<'a>>>;

#[derive(Debug, Clone)]
pub enum Ctx<'a> {
    Fn(SymMap<'a>),
    Global(SymMap<'a>),
    FnCall(SymMap<'a>),
}

impl<'a> Ctx<'a> {
    #[inline(always)]
    pub fn sym_map(&self) -> &SymMap<'a> {
        match self {
            Ctx::Fn(sm) | Ctx::FnCall(sm) | Ctx::Global(sm) => sm
        }
    }

    #[inline(always)]
    pub fn sym_map_mut(&mut self) -> &mut SymMap<'a> {
        match self {
            Ctx::Fn(sm) | Ctx::FnCall(sm) | Ctx::Global(sm) => sm
        }
    }

    #[inline(always)]
    pub fn empty_fn() -> Ctx<'static> {
        Ctx::Fn(SymMap::new())
    }

    #[inline(always)]
    pub fn empty_fncall() -> Ctx<'static> {
        Ctx::FnCall(SymMap::new())
    }

    #[inline(always)]
    pub fn empty_global() -> Ctx<'static> {
        Ctx::Global(SymMap::new())
    }
}

pub struct Parser<'a, 'b> {
    idx: usize,
    eof: bool,
    tokens: &'b Tokens<'a>,
    ctx: Box::<Ctx<'a>>,
}

impl<'a, 'b> Parser<'a, 'b> {
    #[inline]
    pub fn new(tokens: &'b Tokens<'a>) -> Self {
        Self {
            tokens,
            idx: 0,
            eof: false,
            ctx: Box::new(Ctx::Global(HashMap::new()))
        }
    }

    #[inline]
    fn type_check_token<F, E>(&self, cond: F, err: E) -> &Box::<Token<'a>>
    where
        F: FnOnce(&Token) -> bool,
        E: FnOnce(&'a str, &Box::<Loc>)
    {
        if let Some(t) = self.tokens.get(self.idx) {
            if cond(t) { t } else { err(t.string, &t.loc); exit(1) }
        } else { err("<eof>", &self.tokens[self.idx].loc); exit(1) }
    }

    #[inline]
    fn type_check_token_owned<F, E>(&self, cond: F, err: E) -> Box::<Token<'a>>
    where
        F: FnOnce(&Token) -> bool,
        E: FnOnce(&'a str, &Box::<Loc>)
    {
        if let Some(t) = self.tokens.get(self.idx) {
            if cond(t) {
                self.tokens[self.idx].to_owned()
            } else { err(t.string, &t.loc); exit(1) }
        } else { err("<eof>", &self.tokens[self.idx].loc); exit(1) }
    }

    #[inline]
    fn advance(&mut self) {
        self.idx += 1;
        if self.idx >= self.tokens.len() {
            self.idx = 0;
            self.eof = true;
        }
    }

    fn parse_decl(&mut self) -> VarDecl<'a> {
        let ref ty_token = self.tokens[self.idx];

        self.advance();

        let ty = Type::try_from_token(&ty_token).unwrap();
        if self.tokens[self.idx].kind == TokenKind::Asterisk {
            todo!();
            // ty = ty.to_ptr();
            // self.advance();
        }

        let name_token = self.type_check_token_owned(|t| {
            matches!(t.kind, TokenKind::Lit)
        }, |string, loc| {
            panic!("{loc} error: expected literal after the type, but got: {string}")
        });

        self.advance();
        self.advance();

        let mut expr_tokens = Vec::new();
        while self.tokens[self.idx].kind != TokenKind::Semicolon && !self.eof {
            expr_tokens.push(&self.tokens[self.idx]);
            self.advance();
        }

        self.type_check_token(|t| {
            matches!(t.kind, TokenKind::Semicolon)
        }, |string, loc| {
            panic!("{loc} error: expected semicolon after decl, but got: {string}")
        });

        self.advance();

        let expr = ExprParser::new(expr_tokens, self.ctx.sym_map()).parse();
        let value = match ty {
            Type::I64 => Box::new(Expr::I64(expr.eval_int(self.ctx.sym_map()))),
            Type::F64 => Box::new(Expr::F64(expr.eval_flt(self.ctx.sym_map()))),
        };

        VarDecl {
            name_token,
            value,
        }
    }

    fn parse_fn_call(&mut self) -> FnCall<'a> {
        let name_token = self.tokens[self.idx].to_owned();
        self.advance();

        if !matches! {
            self.tokens.get(self.idx),
            Some(t) if t.kind == TokenKind::LParen
        } {
            panic!("{loc} expected lparen bruv",
                   loc = name_token.loc);
        }

        self.advance();

        let mut args = Vec::new();
        while self.idx < self.tokens.len() && !self.eof {
            let ref t = self.tokens[self.idx];
            if t.kind == TokenKind::RParen { break }

            let mut expr_tokens = Vec::new();
            while !matches!(self.tokens[self.idx].kind, TokenKind::Comma | TokenKind::RParen) && !self.eof {
                expr_tokens.push(&self.tokens[self.idx]);
                self.advance();
            }

            let expr = ExprParser::new(expr_tokens, self.ctx.sym_map()).parse();
            let value = match t.kind {
                TokenKind::RParen => break,
                TokenKind::Int => Box::new(Expr::I64(expr.eval_int(self.ctx.sym_map()))),
                TokenKind::Flt => Box::new(Expr::F64(expr.eval_flt(self.ctx.sym_map()))),
                TokenKind::Lit => Box::new(Expr::Lit(t.to_owned())),
                _ => panic!("{loc} expected int or float bruv, but got: {got}",
                            loc = t.loc, got = t.string)
            };

            if !matches! {
                self.tokens.get(self.idx),
                Some(t) if t.kind == TokenKind::RParen
            } {
                self.type_check_token(|t| {
                    t.kind == TokenKind::Comma
                }, |string, loc| {
                    panic!("{loc} expected comma after an argument, but got: {string}")
                });
                self.advance();
            }

            args.push(value);
        }

        if !matches! {
            self.tokens.get(self.idx),
            Some(t) if t.kind == TokenKind::RParen
        } {
            panic!("{loc} rparen was not met bruv", loc = name_token.loc);
        }

        self.advance();
        FnCall {args, name_token}
    }

    fn parse_fn(&mut self) -> Fn<'a> {
        self.advance();

        let name_token = self.type_check_token_owned(|t| {
            matches!(t.kind, TokenKind::Lit)
        }, |string, loc| {
            panic!("{loc} error: expected literal after the fn keyword, but got: {string}")
        });

        self.advance();

        if !matches! {
            self.tokens.get(self.idx),
            Some(t) if t.kind == TokenKind::LParen
        } {
            panic!("{loc} expected lparen bruv",
                   loc = name_token.loc);
        }

        self.advance();

        let mut args = Vec::new();
        while self.idx < self.tokens.len() && !self.eof {
            let ref t = self.tokens[self.idx];

            if t.kind == TokenKind::RParen { break }

            let Ok(ty) = Type::try_from_token(t) else {
                panic!("{loc} expected type but got: {string}",
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
            self.tokens.get(self.idx),
            Some(t) if t.kind == TokenKind::RParen
        } {
            panic!("{loc} rparen was not met bruv", loc = name_token.loc);
        }

        self.advance();

        let ret_ty = if self.tokens[self.idx].kind == TokenKind::LCurly {
            None
        } else {
            self.type_check_token_owned(|t| {
                matches!(t.kind, TokenKind::Minus)
            }, |string, loc| {
                panic!("{loc} error: Expected arrow after rparen, but got: {string}")
            });

            self.advance();

            self.type_check_token_owned(|t| {
                matches!(t.kind, TokenKind::RAngleBracket)
            }, |string, loc| {
                panic!("{loc} error: Expected arrow after rparen, but got: {string}")
            });

            self.advance();

            let Ok(ty) = Type::try_from_token(&self.tokens[self.idx]) else {
                panic!("{loc} error: Expected arrow after rparen, but got: {string}",
                    loc = self.tokens[self.idx].loc,
                    string = self.tokens[self.idx].string)
            };

            self.advance();
            Some(ty)
        };

        let ref lcurly_token = 'lcurly: loop {
            if self.tokens.is_empty() {
                panic!("{loc} expected `{{` bruv", loc = name_token.loc);
            }

            let ref t = self.tokens[self.idx];
            self.advance();
            if t.kind == TokenKind::LCurly {
                break 'lcurly t;
            }
        };

        let mut body = Asts::new();
        while !self.parse_line(true, &mut body) {
            self.advance();
            if self.eof {
                panic!("{loc} eww no rcurly matched bruv", loc = lcurly_token.loc)
            }
        }

        self.advance();

        Fn { ret_ty, body: body.asts, args, name_token }
    }

    #[inline(always)]
    fn append(&self, asts: &mut Asts<'a>, loc: Box::<Loc>, kind: AstKind<'a>) {
        asts.append(self.ctx.to_owned(), loc, kind);
    }

    #[inline(always)]
    fn new_ast(&self, asts: &mut Asts<'a>, loc: Box::<Loc>, kind: AstKind<'a>) -> Ast<'a> {
        asts.new_ast(self.ctx.to_owned(), loc, kind)
    }

    #[inline(always)]
    fn set_ctx(&mut self, ctx: Ctx<'a>) {
        self.ctx = Box::new(ctx)
    }

    fn parse_line(&mut self, expect_matching: bool, asts: &mut Asts<'a>) -> bool {
        while self.idx < self.tokens.len() && !self.eof {
            if self.eof { break }
            let ref token = self.tokens[self.idx];
            match token.kind {
                TokenKind::RCurly |
                TokenKind::RParen => if expect_matching {
                    return true
                } else {
                    panic!("{loc} unexpected `}}` bruv", loc = token.loc)
                }
                TokenKind::Fn => {
                    let fn_ = Box::new(self.parse_fn());
                    self.set_ctx(Ctx::empty_fn());
                    self.append(asts, token.loc.to_owned(), AstKind::Fn(fn_));
                }
                TokenKind::Lit => {
                    let fcall = Box::new(self.parse_fn_call());
                    self.append(asts, token.loc.to_owned(), AstKind::FnCall(fcall));
                }
                TokenKind::Type => {
                    let decl = Box::new(self.parse_decl());
                    let name = decl.name_token.string;
                    let ast = self.new_ast(asts, token.loc.to_owned(), AstKind::VarDecl(decl));
                    let ptr = Box::new(ast);
                    self.ctx.sym_map_mut().insert(name, ptr.to_owned());
                    asts.append_ast(ptr);
                    asts.id += 1;
                }
                _ => self.idx += 1
            }
        } false
    }

    #[inline(always)]
    pub fn parse(&mut self) -> Asts {
        let mut asts = Asts::new();
        while self.idx < self.tokens.len() && !self.eof {
            if self.eof { break }
            self.parse_line(false, &mut asts);
            self.advance();
        } asts
    }
}

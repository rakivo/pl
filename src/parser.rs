use crate::{
    eval::parser::ExprParser,
    ast::{Ast, AstKind, VarValue, VarDecl},
    lexer::{Loc, Token, TokenKind, Tokens, Tokens2D}
};

use std::process::exit;
use std::collections::HashMap;

pub struct Asts<'a> {
    pub id: usize,
    pub asts: Vec::<Ast<'a>>,
}

impl<'a> Asts<'a> {
    const RESERVE: usize = 1024;

    #[inline]
    fn new() -> Self {
        Self {
            id: 0,
            asts: Vec::with_capacity(Self::RESERVE),
        }
    }

    #[inline(always)]
    pub fn id(&self, id: usize) -> &Ast {
        unsafe { self.asts.get_unchecked(id) }
    }

    #[inline(always)]
    fn append(&mut self, loc: Box::<Loc>, kind: AstKind<'a>) {
        let ast = Ast {id: self.id, next: self.id + 1, loc, kind};
        self.asts.push(ast);
        self.id += 1;
    }
}

pub type VarMap<'a> = HashMap::<&'a str, Box::<VarDecl<'a>>>;

pub struct Parser<'a> {
    pub asts: Asts<'a>,
    curr_line: Tokens<'a>,
    var_map: VarMap<'a>
}

impl<'a> Parser<'a> {
    #[inline(always)]
    pub fn new(dummy: Tokens<'a>) -> Self {
        Self {
            asts: Asts::new(),
            curr_line: dummy,
            var_map: HashMap::new()
        }
    }

    #[inline]
    fn type_check_token<F, E>(&self, idx: usize, cond: F, err: E) -> &Box::<Token<'a>>
    where
        F: FnOnce(&Token) -> bool,
        E: FnOnce((&'a str, &Box::<Loc>))
    {
        if let Some(t) = self.curr_line.get(idx) {
            if cond(t) { t } else { err((t.string, &t.loc)); exit(1) }
        } else { err(("<eof>", &self.curr_line[idx].loc)); exit(1) }
    }

    #[inline]
    fn type_check_token_owned<F, E>(&self, idx: usize, cond: F, err: E) -> Box::<Token<'a>>
    where
        F: FnOnce(&Token) -> bool,
        E: FnOnce((&'a str, &Box::<Loc>))
    {
        if let Some(t) = self.curr_line.get(idx) {
            if cond(t) {
                self.curr_line[idx].to_owned()
            } else { err((t.string, &t.loc)); exit(1) }
        } else { err(("<eof>", &self.curr_line[idx].loc)); exit(1) }
    }

    fn parse_decl(&self, idx: &mut usize) -> VarDecl<'a> {
        let ref ty_token = self.curr_line[*idx];

        *idx += 1;

        let name_token = self.type_check_token_owned(*idx, |t| {
            matches!(t.kind, TokenKind::Lit)
        }, |(string, loc)| {
            panic!("{loc} error: expected literal after the type, but got: {string}")
        });

        *idx += 1;
        *idx += 1;

        let expr_tokens = self.curr_line[*idx..].iter()
            .collect::<Vec::<_>>();

        *idx += 1;

        match ty_token.kind {
            TokenKind::IntType => VarDecl {
                name_token,
                value: VarValue::Int(ExprParser::new(&expr_tokens, &self.var_map).parse().eval_int()),
            },
            TokenKind::FltType => VarDecl {
                name_token,
                value: VarValue::Flt(ExprParser::new(&expr_tokens, &self.var_map).parse().eval_flt()),
            },
            _ => unreachable!()
        }
    }

    fn parse_line(&mut self) {
        let mut idx = 0;
        while idx < self.curr_line.len() {
            let ref token = self.curr_line[idx];
            match token.kind {
                TokenKind::IntType | TokenKind::FltType => {
                    let decl = Box::new(self.parse_decl(&mut idx));
                    self.var_map.insert(decl.name_token.string, decl.to_owned());
                    self.asts.append(token.loc.to_owned(), AstKind::VarDecl(decl));
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

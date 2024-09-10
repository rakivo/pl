use crate::ctx::{Ctx, RefCtx};
use crate::{
    append_loc,
    last_loc_to_string
};

use std::str::Lines;
use std::fmt::Display;
use std::iter::{Peekable, Enumerate};

pub type Tokens<'a> = Vec::<Token<'a>>;
pub type Tokens2D<'a> = Vec::<Tokens<'a>>;
pub type IoResultRef<'a, T> = Result<T, &'a std::io::Error>;
type LinesIterator<'a> = Peekable::<Enumerate::<Lines<'a>>>;

pub struct File {
    pub len: usize,
    pub bytes: [u8; 256],
}

impl Display for File {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{s}", s = unsafe { std::str::from_utf8_unchecked(&self.bytes[..self.len]) })
    }
}

pub struct Loc {
    pub row: usize,
    pub col: usize,
    pub file_id: usize
}

impl Loc {
    pub fn to_string(&self, ctx: &Ctx) -> String {
        format!("{f}:{r}:{c}:",
                f = ctx.fileid(self.file_id),
                r = self.row + 1,
                c = self.col + 1)
    }
}

#[derive(Debug, Clone)]
pub enum TokenKind {
    Type,
    Equal,
    Integer,
    Literal,
    Semicolon,
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
   pub kind: TokenKind,
   pub loc_id: usize,
   pub string: &'a str,
}

impl Token<'_> {
    #[inline]
    pub fn to_string(&self, ctx: &Ctx) -> String {
        format!("{l} {k:?} {s}",
                l = ctx.locid(self.loc_id).to_string(ctx),
                k = self.kind,
                s = self.string)
    }
}

pub struct Lexer<'a> {
    row: usize,
    file_id: usize,
    ctx: &'a RefCtx<'a>,
    lines: LinesIterator<'a>,
    pub tokens: Tokens2D<'a>
}

impl<'a> Lexer<'a> {
    #[inline]
    pub fn new(
        ctx: &'a RefCtx<'a>,
        file_path: &'a str,
        content: IoResultRef<'a, &'a String>
    ) -> IoResultRef<'a, Self> {
        let lexer = Self {
            ctx,
            row: 0,
            tokens: Vec::with_capacity(128),
            lines: content?.lines().enumerate().peekable(),
            file_id: ctx.borrow_mut().append_file(file_path),
        };
        Ok(lexer)
    }

    const SEPARATORS: &'static [char] = &[';', '='];

    fn split_whitespace_preserve_indices(input: &str) -> Vec::<(usize, &str)> {
        let (s, e, mut ret) = input.char_indices().fold((0, 0, Vec::with_capacity(input.len() / 2)),
            |(s, e, mut ret), (i, c)|
        {
            let is_sepa = Self::SEPARATORS.contains(&c);
            if c.is_whitespace() || is_sepa {
                if s != i {
                    ret.push((s, &input[s..i]))
                }
                if is_sepa {
                    ret.push((s + i - s, &input[i..=i]))
                }
                (i + c.len_utf8(), e, ret)
            } else {
                (s, i + c.len_utf8(), ret)
            }
        });

        if s != e && !input[s..].is_empty() {
            ret.push((s, &input[s..]))
        }

        ret
    }

    pub const TYPES: &'static [&'static str] = &[
        "i32"
    ];

    fn token_kind(&self, string: &str) -> TokenKind {
        let first = string.as_bytes()[0];
        match first as _ {
            '=' => TokenKind::Equal,
            ';' => TokenKind::Semicolon,
            '0'..='9' => TokenKind::Integer,
            'a'..='z' | 'A'..='Z' => if Self::TYPES.contains(&string) {
                TokenKind::Type
            } else {
                TokenKind::Literal
            }
            _ => panic!("{loc} error: unexpected token: {string}", loc = last_loc_to_string!(self.ctx))
        }
    }

    fn lex_line(&mut self, line: &'a str) {
        let strs = Self::split_whitespace_preserve_indices(line);
        let len = strs.len();
        let tokens = strs.into_iter().fold(Vec::with_capacity(len),
            |mut tokens, (col, string)|
        {
            let kind = self.token_kind(string);
            let loc = Loc {
                row: self.row,
                col,
                file_id: self.file_id
            };
            let token = Token {
                kind,
                string,
                loc_id: append_loc!(self.ctx, loc),
            };
            tokens.push(token);
            tokens
        });
        self.tokens.push(tokens);
    }

    pub fn lex(&mut self) {
        while let Some((row, line)) = self.lines.next() {
            self.row = row;
            self.lex_line(line);
        }
    }
}

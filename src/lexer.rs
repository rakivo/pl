use std::ptr;
use std::str::Lines;
use std::fmt::{Debug, Display};
use std::iter::{Peekable, Enumerate};

pub type Tokens<'a> = Vec::<Box::<Token<'a>>>;
pub type Tokens2D<'a> = Vec::<Tokens<'a>>;
pub type TokensRefs<'a> = Vec::<&'a Box::<Token<'a>>>;
pub type IoResultRef<'a, T> = Result<T, &'a std::io::Error>;

type LinesIterator<'a> = Peekable::<Enumerate::<Lines<'a>>>;

#[derive(Clone)]
pub struct FilePath {
    pub len: usize,
    pub bytes: [u8; 256],
}

impl FilePath {
    #[inline]
    pub fn new(file_path: &str) -> Self {
        let mut bytes = [0; 256];
        unsafe {
            ptr::copy(file_path.as_ptr(), bytes.as_mut_ptr(), file_path.len());
        }

        Self {len: file_path.len(), bytes: bytes}
    }
}

impl Display for FilePath {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{s}", s = unsafe { std::str::from_utf8_unchecked(&self.bytes[..self.len]) })
    }
}

#[derive(Clone)]
pub struct Loc {
    pub row: usize,
    pub col: usize,
    pub file_path: Box::<FilePath>
}

impl Display for Loc {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{f}:{r}:{c}:",
               f = self.file_path,
               r = self.row + 1,
               c = self.col + 1)
    }
}

impl Debug for Loc {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenKind {
    Poisoned,
    Int,
    Flt,
    Lit,
    FltType,
    IntType,
    Plus,
    Asterisk,
    LParen,
    Minus,
    RParen,
    Slash,
    Equal,
    Semicolon,
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
   pub loc: Box::<Loc>,
   pub kind: TokenKind,
   pub string: &'a str,
}

impl Display for Token<'_> {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{l} {k:?} {s}",
                l = self.loc,
                k = self.kind,
                s = self.string)
    }
}

pub struct Lexer<'a> {
    row: usize,
    lines: LinesIterator<'a>,
    file_path: Box::<FilePath>,
    pub tokens: Tokens2D<'a>
}

impl<'a> Lexer<'a> {
    #[inline]
    pub fn new(
        file_path: &'a str,
        content: IoResultRef<'a, &'a String>
    ) -> IoResultRef<'a, Self> {
        let lexer = Self {
            row: 0,
            tokens: Vec::with_capacity(128),
            lines: content?.lines().enumerate().peekable(),
            file_path: Box::new(FilePath::new(file_path))
        };
        Ok(lexer)
    }

    const SEPARATORS: &'static [char] = &[';', '=', '*', '/', '-', '+', '(', ')'];

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
        "i64", "f64"
    ];

    fn token_kind(&self, string: &str, err_loc: &Loc) -> TokenKind {
        let first = string.as_bytes()[0];
        match first as _ {
            '+' => TokenKind::Plus,
            '=' => TokenKind::Equal,
            '*' => TokenKind::Asterisk,
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '-' => TokenKind::Minus,
            '/' => TokenKind::Slash,
            ';' => TokenKind::Semicolon,
            '0'..='9' => if string.parse::<i64>().is_ok() {
                TokenKind::Int
            } else if string.parse::<f64>().is_ok() {
                TokenKind::Flt
            } else {
                panic!("{err_loc} error: failed to parse number: {string}")
            }
            'a'..='z' | 'A'..='Z' => if let Some(idx) = Self::TYPES.iter().position(|s| s == &string) {
                match idx {
                    0 => TokenKind::IntType,
                    1 => TokenKind::FltType,
                    _ => unreachable!()
                }
            } else {
                TokenKind::Lit
            }
            _ => panic!("{err_loc} error: unexpected token: {string}")
        }
    }

    fn lex_line(&mut self, line: &'a str) {
        let strs = Self::split_whitespace_preserve_indices(line);
        let tokens = strs.into_iter().map(|(col, string)| {
            let loc = Loc {
                row: self.row,
                col,
                file_path: self.file_path.to_owned()
            };
            Token {
                kind: self.token_kind(string, &loc),
                string,
                loc: Box::new(loc),
            }
        }).map(Box::new).collect();
        self.tokens.push(tokens);
    }

    #[inline]
    pub fn lex(&mut self) {
        while let Some((row, line)) = self.lines.next() {
            self.row = row;
            self.lex_line(line);
        }
    }
}

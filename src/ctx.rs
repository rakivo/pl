use crate::{
    ast::Ast,
    lexer::{Loc, File},
};

use std::ptr;
use std::cell::RefCell;

pub type RefCtx<'a> = RefCell::<Ctx<'a>>;

pub struct Ctx<'a> {
    locs:  Vec::<Loc>,
    files: Vec::<File>,
    pub asts: Vec::<Ast<'a>>,
}

impl<'a> Ctx<'a> {
    #[inline]
    pub fn new() -> Self {
        Self {
            asts:  Vec::with_capacity(1024),
            locs:  Vec::with_capacity(1024),
            files: Vec::with_capacity(128)
        }
    }

    #[inline(always)]
    pub fn locid(&self, idx: usize) -> &Loc {
        unsafe { self.locs.get_unchecked(idx) }
    }

    #[inline(always)]
    pub fn astid(&self, idx: usize) -> &'a Ast {
        unsafe { self.asts.get_unchecked(idx) }
    }

    #[inline(always)]
    pub fn fileid(&self, idx: usize) -> &File {
        unsafe { self.files.get_unchecked(idx) }
    }

    pub fn asts(&self) -> &Vec::<Ast> {
        &self.asts
    }

    #[inline(always)]
    pub fn append_loc(&mut self, loc: Loc) -> usize {
        self.locs.push(loc);
        self.locs.len() - 1
    }

    #[inline(always)]
    pub fn append_ast(&mut self, ast: Ast<'a>) -> usize {
        self.asts.push(ast);
        self.asts.len() - 1
    }

    #[inline(always)]
    pub fn append_file(&mut self, file_path: &str) -> usize {
        let mut bytes = [0; 256];
        unsafe {
            ptr::copy(file_path.as_ptr(), bytes.as_mut_ptr(), file_path.len());
        }

        let file = File {
            len: file_path.len(),
            bytes: bytes,
        };

        self.files.push(file);
        self.files.len() - 1
    }

    #[inline(always)]
    pub fn last_loc(&self) -> &Loc {
        unsafe { self.locs.last().unwrap_unchecked() }
    }

    #[inline(always)]
    pub fn last_ast(&self) -> &'a Ast {
        unsafe { self.asts.last().unwrap_unchecked() }
    }

    #[inline(always)]
    pub fn last_file(&self) -> &File {
        unsafe { self.files.last().unwrap_unchecked() }
    }

    #[inline(always)]
    pub fn last_astid(&self) -> usize {
        self.asts.len()
    }
}

#[macro_export]
macro_rules! locid {
    ($ctx: expr, $loc_id: expr) => {
        $ctx.borrow().locid($loc_id)
    };
}

#[macro_export]
macro_rules! locid_to_string {
    ($ctx: expr, $loc_id: expr) => {
        $ctx.borrow().locid($loc_id).to_string(&$ctx.borrow())
    };
}

#[macro_export]
macro_rules! last_loc {
    ($ctx: expr) => {
        $ctx.borrow().last_loc()
    };
}

#[macro_export]
macro_rules! last_loc_to_string {
    ($ctx: expr) => {
        $ctx.borrow().last_loc().to_string(&$ctx.borrow())
    };
}

#[macro_export]
macro_rules! fileid {
    ($ctx: expr, $file_id: expr) => {
        $ctx.borrow().fileid($file_id)
    };
}

#[macro_export]
macro_rules! astid {
    ($ctx: expr, $ast_id: expr) => {
        $ctx.borrow().astid($ast_id)
    };
}

#[macro_export]
macro_rules! append_loc {
    ($ctx: expr, $loc: expr) => {
        $ctx.borrow_mut().append_loc($loc)
    };
}

#[macro_export]
macro_rules! append_file {
    ($ctx: expr, $file: expr) => {
        $ctx.borrow_mut().append_file($file)
    };
}

#[macro_export]
macro_rules! append_ast {
    ($ctx: expr, $ast: expr) => {
        $ctx.borrow_mut().append_ast($ast)
    };
}

#[macro_export]
macro_rules! last_astid {
    ($ctx: expr) => {
        $ctx.borrow().last_astid()
    };
}

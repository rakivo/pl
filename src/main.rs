use std::env;
use std::fs::read_to_string;
use std::cell::RefCell;

mod ctx;
mod ast;
mod lexer;
mod parser;
mod compiler;

use ctx::*;
use lexer::*;
use parser::*;
use compiler::*;

fn main() -> IoResultRef::<'static, ()> {
    let argv = env::args().collect::<Vec::<_>>();
    if argv.len() < 2 {
        panic!("usage: {program} <file_path>", program = argv[0]);
    }

    let ref file_path = argv[1];
    let content = read_to_string(file_path);

    let ctx = RefCell::new(Ctx::new());
    let mut lexer = Lexer::new(&ctx, file_path, content.as_ref()).map_err(|err| {
        panic!("error: failed to read file: `{file_path}`: {e}", e = err)
    })?;

    lexer.lex();

    if lexer.tokens.is_empty() { return Ok(()) }

    let mut parser = Parser::new(&ctx, lexer.tokens[0].to_vec());
    parser.parse(lexer.tokens);

    // ctx.borrow().asts.iter().for_each(|ast| println!("{ast:?}"));
    let mut compiler = Compiler::new(&ctx, file_path).unwrap();
    compiler.compile().unwrap();

    Ok(())
}

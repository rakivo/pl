use std::env;
use std::fs::read_to_string;

mod ast;
mod lexer;
mod parser;
mod compiler;
mod expr_parser;

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

    let mut lexer = Lexer::new(file_path, content.as_ref()).map_err(|err| {
        panic!("error: failed to read file: `{file_path}`: {e}", e = err)
    })?;

    lexer.lex();

    if lexer.tokens.is_empty() { return Ok(()) }

    let mut parser = Parser::new(&lexer.tokens);
    let asts = parser.parse();

    // asts.asts.iter().for_each(|ast| {
    //     println!("{ast:?}");
    // });

    let mut compiler = Compiler::new(file_path).unwrap();
    compiler.compile(asts).unwrap();

    Ok(())
}

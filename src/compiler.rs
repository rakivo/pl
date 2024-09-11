use crate::parser::Asts;
use crate::ast::{Ast, AstKind, VarValue};

use std::fs::File;
use std::io::Write;

pub struct Compiler {
    s: File,
    gen_file_path: String,
}

const TAB: &'static str = "\t";

macro_rules! writetln {
    ($dst: expr, $($arg: tt)*) => {{
        write!($dst, "{TAB}")?;
        writeln!($dst, $($arg)*)
    }};
}

impl Compiler {
    pub fn new(_file_path: &str) -> std::io::Result::<Self> {
        let gen_file_path = "out.ssa".to_owned();
        let s = File::create(&gen_file_path)?;
        let compiler = Self { s, gen_file_path };
        Ok(compiler)
    }

    fn compile_ast(&mut self, ast: Ast) -> std::io::Result::<()> {
        match ast.kind {
            AstKind::VarDecl(vd) => {
                match vd.value {
                    VarValue::Int(int) => writetln!(self.s, "%{name} =l %{int}", name = vd.name_token.string)?,
                    VarValue::Flt(flt) => writetln!(self.s, "%{name} =d %{flt}", name = vd.name_token.string)?
                }
            }
            AstKind::Poisoned => unreachable!()
        }

        Ok(())
    }

    pub fn compile(&mut self, asts: Asts) -> std::io::Result::<()> {
        writeln!(self.s, "export function w $_start() {{")?;
        writeln!(self.s, "@start")?;

        for ast in asts.asts {
            self.compile_ast(ast)?;
        }

        writetln!(self.s, "ret")?;
        writeln!(self.s, "}}")?;

        Ok(())
    }
}

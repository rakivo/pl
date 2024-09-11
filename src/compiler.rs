use crate::parser::Asts;
use crate::ast::{Ast, AstKind, Value};

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

macro_rules! writet {
    ($dst: expr, $($arg: tt)*) => {{
        write!($dst, "{TAB}")?;
        write!($dst, $($arg)*)
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
                    Value::Int(int) => writetln!(self.s, "%{name} =l copy {int}", name = vd.name_token.string)?,
                    Value::Flt(flt) => writetln!(self.s, "%{name} =d copy {bits}",
                                                 name = vd.name_token.string,
                                                 bits = flt.to_bits())?
                }
            }
            AstKind::FnCall(fc) => {
                writet!(self.s, "call ${name}(", name = fc.name_token.string)?;
                for (idx, arg) in fc.args.iter().enumerate() {
                    match arg {
                        Value::Int(int) => write!(self.s, "l {int}")?,
                        Value::Flt(flt) => write!(self.s, "d {bits}",
                                                  bits = flt.to_bits())?,
                    };
                    if idx + 1 < fc.args.len() { write!(self.s, ", ")?; }
                }
                writeln!(self.s, ")")?;
            }
        }

        Ok(())
    }

    pub fn compile(&mut self, asts: Asts) -> std::io::Result::<()> {
        writeln!(self.s, "export function w $_start() {{")?;
        writeln!(self.s, "@start")?;

        for ast in asts.asts {
            self.compile_ast(ast)?;
        }

        writetln!(self.s, "call $syscall1(w 60, w 0)")?;
        writetln!(self.s, "ret")?;
        writeln!(self.s, "}}")?;

        Ok(())
    }
}

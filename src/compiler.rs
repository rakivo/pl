use crate::ctx::RefCtx;

use std::fs::File;
use std::io::Write;

pub struct Compiler<'a> {
    s: File,
    ctx: &'a RefCtx<'a>,
    gen_file_path: String,
}

const TAB: &'static str = "\t";

macro_rules! writetln {
    ($dst: expr, $($arg: tt)*) => {{
        write!($dst, "{TAB}")?;
        writeln!($dst, $($arg)*)
    }};
}

impl<'a> Compiler<'a> {
    pub fn new<'b>(ctx: &'a RefCtx<'a>, _file_path: &str) -> std::io::Result::<Self> {
        let gen_file_path = "out.ssa".to_owned();
        let s = File::create(&gen_file_path)?;
        let compiler = Self { s, ctx, gen_file_path };
        Ok(compiler)
    }

    pub fn compile(&mut self) -> std::io::Result::<()> {
        writeln!(self.s, "export function w $_start() {{")?;
        writeln!(self.s, "@start")?;
        writetln!(self.s, "ret")?;
        writeln!(self.s, "}}")?;

        Ok(())
    }
}

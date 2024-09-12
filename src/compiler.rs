use crate::ast::{Ast, Type, Fn, Asts, AstKind, VarDecl, FnCall, Value};

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

    fn compile_var_decl(&mut self, vd: &VarDecl) -> std::io::Result::<()> {
        match vd.value {
            Value::Int(int) => writetln!(self.s, "%{name} =l copy {int}",
                                         name = vd.name_token.string)?,

            Value::Flt(flt) => writetln!(self.s, "%{name} =d copy {bits}",
                                         name = vd.name_token.string,
                                         bits = flt.to_bits())?
        };

        Ok(())
    }

    fn compile_fn(&mut self, fn_: &Fn) -> std::io::Result::<()> {
        write!(self.s, "function")?;
        let ty = match fn_.ret_ty {
            Some(Type::I64) => "l",
            Some(Type::F64) => "d",
            None => "",
        };
        write!(self.s, " {ty} ${name}(", name = fn_.name_token.string)?;
        for (idx, arg) in fn_.args.iter().enumerate() {
            let ref name = arg.name_token.string;
            match arg.ty {
                Type::I64 => write!(self.s, "l %{name}")?,
                Type::F64 => write!(self.s, "d %{name}")?,
            };
            if idx + 1 < fn_.args.len() { write!(self.s, ", ")?; }
        }

        writeln!(self.s, ") {{")?;
        writeln!(self.s, "@start")?;
        for ast in fn_.body.iter() {
            self.compile_ast(ast)?;
        }

        match fn_.ret_ty {
            Some(Type::I64) => writetln!(self.s, "ret 0")?,
            Some(Type::F64) => writetln!(self.s, "ret 0")?,
            None => writetln!(self.s, "ret")?
        };
        writeln!(self.s, "}}")?;

        Ok(())
    }

    fn compile_print(&mut self, fc: &FnCall) -> std::io::Result::<()> {
        for arg in fc.args.iter() {
            match arg {
                Value::Int(int) => writetln!(self.s, "call $print_i64(l {int}, w 1)")?,
                Value::Flt(flt) => writetln!(self.s, "call $print_f64(d {bits}, w 1)",
                                             bits = flt.to_bits())?,
            };
        }
        Ok(())
    }

    fn compile_fn_call(&mut self, fc: &FnCall) -> std::io::Result::<()> {
        if fc.name_token.string.eq("print") {
            return self.compile_print(fc)
        }

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
        Ok(())
    }

    fn compile_ast(&mut self, ast: &Ast) -> std::io::Result::<()> {
        match &ast.kind {
            AstKind::Fn(fn_)     => self.compile_fn(&fn_),
            AstKind::VarDecl(vd) => self.compile_var_decl(&vd),
            AstKind::FnCall(fc)  => self.compile_fn_call(&fc),
        }?;

        Ok(())
    }

    pub fn compile(&mut self, asts: Asts) -> std::io::Result::<()> {
        for ast in asts.asts.iter() {
            self.compile_ast(ast)?;
        }

        writeln!(self.s, "export function w $_start() {{")?;
        writeln!(self.s, "@start")?;
        writetln!(self.s, "call $syscall1(w 60, w 0)")?;
        writetln!(self.s, "ret")?;
        writeln!(self.s, "}}")?;

        Ok(())
    }
}

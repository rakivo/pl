use crate::lexer::Token;
use crate::parser::SymMap;
use crate::ast::{
    Ast, Type, Fn,
    Asts, AstKind, VarDecl, FnCall, Expr
};

use std::fs::File;
use std::io::Write;
use std::ops::Deref;

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

pub struct Compiler<'a> {
    s: File,
    sym_map: &'a SymMap<'a>,
    gen_file_path: String,
}

impl<'a> Compiler<'a> {
    pub fn new(_file_path: &str, sym_map: &'a SymMap<'a>) -> std::io::Result::<Self> {
        let gen_file_path = "out.ssa".to_owned();
        let s = File::create(&gen_file_path)?;
        let compiler = Self { s, sym_map, gen_file_path };
        Ok(compiler)
    }

    fn compile_var_decl(&mut self, vd: &VarDecl) -> std::io::Result::<()> {
        writet!(self.s, "%{name} =", name = vd.name_token.string)?;
        fn compile_expr(s: &mut File, expr: &Box::<Expr>, sym_map: &SymMap) -> std::io::Result::<()> {
            match expr.deref() {
                Expr::Lit(lit) => if let Some(sym) = sym_map.get(lit.string) {
                    match sym.kind {
                        AstKind::VarDecl(ref vd) => compile_expr(s, &vd.value, sym_map)?,
                        _ => todo!()
                    }
                } else {
                    panic!("{loc} error: undefined symbol: {string}",
                           loc = lit.loc, string = lit.string)
                }
                Expr::I64(int) => writeln!(s, "l copy {int}")?,
                Expr::F64(flt) => writeln!(s, "d copy {bits}", bits = flt.to_bits())?,
                _ => todo!()
            };
            Ok(())
        }
        compile_expr(&mut self.s, &vd.value, &self.sym_map)
    }

    fn compile_fn(&mut self, fn_: &Fn) -> std::io::Result::<()> {
        write!(self.s, "function")?;
        let ret_ty = fn_.ret_ty.as_ref()
            .map(Type::to_il_str)
            .unwrap_or_default();

        write!(self.s, " {ret_ty} ${name}(", name = fn_.name_token.string)?;
        for (idx, arg) in fn_.args.iter().enumerate() {
            let ref name = arg.name_token.string;
            write!(self.s, "{ty} %{name}", ty = arg.ty.to_il_str())?;
            if idx + 1 < fn_.args.len() { write!(self.s, ", ")?; }
        }

        writeln!(self.s, ") {{")?;
        writeln!(self.s, "@start")?;
        for ast in fn_.body.iter() {
            self.compile_ast(ast)?;
        }

        match fn_.ret_ty {
            Some(Type::I64)  => writetln!(self.s, "ret 0")?,
            Some(Type::F64)  => writetln!(self.s, "ret 0")?,
            None => writetln!(self.s, "ret")?
        };
        writeln!(self.s, "}}")?;

        Ok(())
    }

    fn compile_print(&mut self, fc: &FnCall) -> std::io::Result::<()> {
        fn get_type(lit: &Box::<Token>, sym_map: &SymMap) -> Option::<Type> {
            if let Some(ref sym) = sym_map.get(lit.string) {
                let AstKind::VarDecl(ref vd) = &sym.kind else { todo!() };
                match vd.value.deref() {
                    Expr::I64(..) => Some(Type::I64),
                    Expr::F64(..) => Some(Type::F64),
                    Expr::Lit(lit) => get_type(lit, sym_map),
                    _ => todo!()
                }
            } else { None }
        }

        for arg in fc.args.iter() {
            match arg.deref() {
                Expr::Lit(lit) => if let Some(ty) = get_type(lit, self.sym_map) {
                    let ref string = lit.string;
                    match ty {
                        Type::I64 => writetln!(self.s, "call $print_i64(l %{string}, w 1)")?,
                        Type::F64 => writetln!(self.s, "call $print_f64(d %{string}, w 1)")?
                    }
                },
                Expr::I64(int) => writetln!(self.s, "call $print_i64(l {int}, w 1)")?,
                Expr::F64(flt) => writetln!(self.s, "call $print_f64(d {bits}, w 1)",
                                             bits = flt.to_bits())?,
                _ => todo!(),
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
            match arg.deref() {
                Expr::I64(int) => write!(self.s, "l {int}")?,
                Expr::F64(flt) => write!(self.s, "d {bits}",
                                         bits = flt.to_bits())?,
                _ => todo!(),
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
            AstKind::FnCall(fc)  => self.compile_fn_call(&fc)
        }
    }

    pub fn compile(&mut self, asts: Asts) -> std::io::Result::<()> {
        for ast in asts.asts.iter() {
            self.compile_ast(ast)?;
        }

        writeln!(self.s, "export function w $_start() {{")?;
        writeln!(self.s, "@start")?;
        writetln!(self.s, "%argc =l call $argc()")?;
        writetln!(self.s, "call $main(l %argc)")?;
        writetln!(self.s, "call $syscall1(w 60, w 0)")?;
        writetln!(self.s, "ret")?;
        writeln!(self.s, "}}")?;

        Ok(())
    }
}

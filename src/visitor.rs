use super::compiler::Compiler;

pub trait Visitor<'a>: Sized {
    /// Visits each node in tree, examines it, and outputs the IASM code
    fn compile_token(&self, compiler: &mut Compiler);
}

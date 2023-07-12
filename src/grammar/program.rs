use nom::{error::context, multi::many1};

use crate::{compiler::Compiler, parse::Parse, visitor::Visitor};

use super::{expression::Expr, token::Token};

#[derive(Debug, PartialEq)]
pub struct Program(Vec<Expr>);

impl Parse<'_> for Program {
    fn parse(input: &'_ str) -> crate::parse::ParseResult<'_, Self> {
        let (remaining, code) = context("Expression", many1(Expr::parse))(input)?;

        Ok((remaining, Program(code)))
    }
}

impl Token for Program {
    fn display_token(&self) -> String {
        todo!()
        // self.0
        //     .iter()
        //     .map(|expr| expr.display_token())
        //     .collect::<Vec<String>>()
        //     .join("\n")
    }
}

impl Visitor<'_> for Program {
    fn compile_token(&self, compiler: &mut Compiler) {
        self.0.iter().for_each(|expr| expr.compile_token(compiler));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_program() {
        let p = Program::parse("-1 +9 * -8 - (3*x) ");
        assert!(p.is_ok())
    }

    #[test]
    fn test_compile() {
        let (_, expected) = Program::parse("-1 +9 * -8 - (3*2) ").unwrap();
        let mut compiler = Compiler::new();
        Program::compile_token(&expected, &mut compiler);
        assert_eq!(compiler.assembly.len(), 9);
        assert_eq!(compiler.free_reg.len(), 30);
        assert_eq!(compiler.used_reg.len(), 1);
    }

    #[test]
    fn test_display() {
        // let value = Program(vec![
        //     Expr {
        //         left: Box::new(ExprValue::Factor(Factor::Integer(FactorValue::Int(6)))),
        //         op: Box::new(ExprValue::Operator(Operator::AddOp)),
        //         right: Box::new(ExprValue::Factor(Factor::Integer(FactorValue::Int(7)))),
        //     },
        //     Expr {
        //         left: Box::new(ExprValue::Factor(Factor::Integer(FactorValue::Int(-4)))),
        //         op: Box::new(ExprValue::Operator(Operator::SubOp)),
        //         right: Box::new(ExprValue::Factor(Factor::Integer(FactorValue::Int(9)))),
        //     },
        // ]);

        // let expected = "6 + 7\n-4 - 9";

        // assert_eq!(value.display_token(), expected);
    }
}

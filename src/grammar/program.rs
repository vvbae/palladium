use nom::{
    bytes::complete::tag,
    combinator::{map, opt},
    error::context,
    multi::many1,
    sequence::tuple,
};

use crate::parse::Parse;

use super::{compiler::Compiler, expression::Expr, token::Token, visitor::Visitor};

#[derive(Debug, PartialEq)]
pub struct Program(Vec<Expr>);

impl Parse<'_> for Program {
    fn parse(input: &'_ str) -> crate::parse::ParseResult<'_, Self> {
        let (remaining, code) = context(
            "Expression",
            many1(map(tuple((Expr::parse, opt(tag("\n")))), |(expr, _)| expr)),
        )(input)?;

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

    use crate::grammar::{factor::Factor, operator::Operator, term::Term};

    use super::*;

    #[test]
    fn test_parse_program() {
        let expected = Program(vec![
            Expr {
                left: Box::new(Term {
                    left: Box::new(Factor::Integer(-9)),
                    op: Some(Box::new(Operator::SubOp)),
                    right: Some(Box::new(Factor::Integer(8))),
                }),
                op: None,
                right: None,
            },
            Expr {
                left: Box::new(Term {
                    left: Box::new(Factor::Float(-9.8)),
                    op: Some(Box::new(Operator::SubOp)),
                    right: Some(Box::new(Factor::Integer(8))),
                }),
                op: Some(Box::new(Operator::AddOp)),
                right: Some(Box::new(Term {
                    left: Box::new(Factor::Float(2.0)),
                    op: Some(Box::new(Operator::MultOp)),
                    right: Some(Box::new(Factor::Identifier("x".to_string()))),
                })),
            },
        ]);

        let (_, value) = Program::parse("(-9- 8)\n(-9.8  - 8 ) + (2.0*x)").unwrap();

        assert_eq!(value, expected);
    }

    #[test]
    fn test_compile() {
        let program = Program(vec![
            Expr {
                left: Box::new(Term {
                    left: Box::new(Factor::Integer(-9)),
                    op: Some(Box::new(Operator::SubOp)),
                    right: Some(Box::new(Factor::Integer(8))),
                }),
                op: Some(Box::new(Operator::AddOp)),
                right: Some(Box::new(Term {
                    left: Box::new(Factor::Integer(8)),
                    op: Some(Box::new(Operator::MultOp)),
                    right: Some(Box::new(Factor::Integer(1))),
                })),
            },
            Expr {
                left: Box::new(Term {
                    left: Box::new(Factor::Integer(-9)),
                    op: Some(Box::new(Operator::SubOp)),
                    right: Some(Box::new(Factor::Integer(8))),
                }),
                op: Some(Box::new(Operator::AddOp)),
                right: Some(Box::new(Term {
                    left: Box::new(Factor::Integer(8)),
                    op: Some(Box::new(Operator::MultOp)),
                    right: Some(Box::new(Factor::Integer(1))),
                })),
            },
        ]);
        let mut compiler = Compiler::new();
        Program::compile_token(&program, &mut compiler);

        assert_eq!(compiler.assembly.len(), 14);

        assert_eq!(
            compiler.assembly,
            vec![
                "LOAD $0 #-9",
                "LOAD $1 #8",
                "SUB $0 $1 $2",
                "LOAD $0 #8",
                "LOAD $1 #1",
                "MUL $0 $1 $3",
                "ADD $2 $3 $0",
                "LOAD $2 #-9",
                "LOAD $3 #8",
                "SUB $2 $3 $1",
                "LOAD $2 #8",
                "LOAD $3 #1",
                "MUL $2 $3 $4",
                "ADD $1 $4 $2"
            ]
        );

        assert_eq!(compiler.free_reg.len(), 29);
        assert_eq!(compiler.used_reg.len(), 2);
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

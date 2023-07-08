use nom::{error::context, multi::many1};

use crate::parse::Parse;

use super::{expression::Expr, token::Token, visitor::Visitor};

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
        self.0
            .iter()
            .map(|expr| expr.display_token())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl Visitor for Program {
    fn visit_token<T: Token>(&self) {
        println!("Visiting program");
        self.display_token();
        println!("Done visiting program");
    }
}

#[cfg(test)]
mod tests {
    use crate::grammar::{
        expression::ExprValue,
        factor::{Factor, FactorValue},
        operator::Operator,
    };

    use super::*;

    #[test]
    fn test_parse_program() {
        let expected = Program(vec![Expr {
            left: Box::new(ExprValue::Factor(Factor::Integer(FactorValue::Int(6)))),
            op: Box::new(ExprValue::Operator(Operator::AddOp)),
            right: Box::new(ExprValue::Factor(Factor::Integer(FactorValue::Int(7)))),
        }]);

        let (_, value) = Program::parse("6 + 7").unwrap();

        assert_eq!(value, expected);
    }

    #[test]
    fn test_display() {
        let value = Program(vec![
            Expr {
                left: Box::new(ExprValue::Factor(Factor::Integer(FactorValue::Int(6)))),
                op: Box::new(ExprValue::Operator(Operator::AddOp)),
                right: Box::new(ExprValue::Factor(Factor::Integer(FactorValue::Int(7)))),
            },
            Expr {
                left: Box::new(ExprValue::Factor(Factor::Integer(FactorValue::Int(-4)))),
                op: Box::new(ExprValue::Operator(Operator::SubOp)),
                right: Box::new(ExprValue::Factor(Factor::Integer(FactorValue::Int(9)))),
            },
        ]);

        let expected = "6 + 7\n-4 - 9";

        assert_eq!(value.display_token(), expected);
    }
}

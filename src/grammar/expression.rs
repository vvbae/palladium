use nom::{character::complete::multispace0, error::context, sequence::tuple};

use crate::parse::Parse;

use super::{factor::Factor, operator::Operator, token::Token, visitor::Visitor};

#[derive(Debug, PartialEq)]
pub enum ExprValue {
    Factor(Factor),
    Operator(Operator),
}

impl Token for ExprValue {
    fn display_token(&self) -> String {
        match &self {
            ExprValue::Factor(fac) => Factor::display_token(fac),
            ExprValue::Operator(op) => Operator::display_token(op),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub left: Box<ExprValue>, // box(pointer): known size at compile time
    pub op: Box<ExprValue>,
    pub right: Box<ExprValue>,
}

impl Parse<'_> for Expr {
    fn parse(input: &'_ str) -> crate::parse::ParseResult<'_, Self> {
        let (remaining, (left, _, op, _, right)) = context(
            "Expression",
            tuple((
                Factor::parse,
                multispace0,
                Operator::parse,
                multispace0,
                Factor::parse,
            )),
        )(input)?;

        Ok((
            remaining,
            Expr {
                left: Box::new(ExprValue::Factor(left)),
                op: Box::new(ExprValue::Operator(op)),
                right: Box::new(ExprValue::Factor(right)),
            },
        ))
    }
}

impl Token for Expr {
    fn display_token(&self) -> String {
        let left: &ExprValue = &*self.left;
        let op: &ExprValue = &*self.op;
        let right: &ExprValue = &*self.right;

        format!(
            "{} {} {}",
            left.display_token(),
            op.display_token(),
            right.display_token()
        )
    }
}

impl Visitor for Expr {
    fn visit_token<T: Token>(&self) {
        println!("Visiting an expression");
        println!("{}", self.display_token());
        println!("Done visiting expression");
    }
}

#[cfg(test)]
mod tests {
    use crate::grammar::factor::FactorValue;

    use super::*;

    #[test]
    fn test_parse_expr() {
        let expected = Expr {
            left: Box::new(ExprValue::Factor(Factor::Integer(FactorValue::Int(6)))),
            op: Box::new(ExprValue::Operator(Operator::AddOp)),
            right: Box::new(ExprValue::Factor(Factor::Integer(FactorValue::Int(7)))),
        };

        let (_, value) = Expr::parse("6 + 7").unwrap();

        assert_eq!(value, expected);
    }

    #[test]
    fn test_display() {
        let expected = Expr {
            left: Box::new(ExprValue::Factor(Factor::Integer(FactorValue::Int(6)))),
            op: Box::new(ExprValue::Operator(Operator::AddOp)),
            right: Box::new(ExprValue::Factor(Factor::Integer(FactorValue::Int(7)))),
        };

        assert_eq!("6 + 7", expected.display_token());
    }
}

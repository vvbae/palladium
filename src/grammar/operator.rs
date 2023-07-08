use nom::{branch::alt, bytes::complete::tag, error::context};

use crate::parse::{Parse, ParseResult};

use super::{token::Token, visitor::Visitor};

#[derive(Debug, PartialEq)]
pub enum Operator {
    AddOp,
    SubOp,
    MultOp,
    DivOp,
}

pub fn parse_add(input: &str) -> ParseResult<'_, Operator> {
    let (remaining, _) = context("Add Operator", tag("+"))(input)?;

    Ok((remaining, Operator::AddOp))
}

pub fn parse_sub(input: &str) -> ParseResult<'_, Operator> {
    let (remaining, _) = context("Subtract Operator", tag("-"))(input)?;

    Ok((remaining, Operator::SubOp))
}

pub fn parse_mul(input: &str) -> ParseResult<'_, Operator> {
    let (remaining, _) = context("Multiply Operator", tag("*"))(input)?;

    Ok((remaining, Operator::MultOp))
}

pub fn parse_div(input: &str) -> ParseResult<'_, Operator> {
    let (remaining, _) = context("Divide Operator", tag("/"))(input)?;

    Ok((remaining, Operator::DivOp))
}

impl Token for Operator {
    fn display_token(&self) -> String {
        match &self {
            Operator::AddOp => format!("+"),
            Operator::SubOp => format!("-"),
            Operator::MultOp => format!("*"),
            Operator::DivOp => format!("/"),
        }
    }
}

impl Visitor for Operator {
    fn visit_token<T: Token>(&self) {
        println!("{}", self.display_token())
    }
}

impl Parse<'_> for Operator {
    fn parse(input: &'_ str) -> crate::parse::ParseResult<'_, Self> {
        let (remaining, op) = context(
            "Operator",
            alt((parse_add, parse_sub, parse_mul, parse_div)),
        )(input)?;

        Ok((remaining, op))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let expected = vec![
            Operator::AddOp,
            Operator::SubOp,
            Operator::MultOp,
            Operator::DivOp,
        ];
        let (_, add) = parse_add("+").unwrap();
        let (_, sub) = parse_sub("-").unwrap();
        let (_, mul) = parse_mul("*").unwrap();
        let (_, div) = parse_div("/").unwrap();

        assert_eq!(vec![add, sub, mul, div], expected);
    }

    #[test]
    fn test_display() {
        let expected = vec!["+", "-", "*", "/"];

        let add = Operator::AddOp;
        let sub = Operator::SubOp;
        let mul = Operator::MultOp;
        let div = Operator::DivOp;

        assert_eq!(
            vec![
                add.display_token(),
                sub.display_token(),
                mul.display_token(),
                div.display_token()
            ],
            expected
        );
    }
}

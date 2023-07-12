use nom::{branch::alt, bytes::complete::tag, error::context};

use crate::{
    compiler::Compiler,
    parse::{Parse, ParseResult},
    visitor::Visitor,
};

use super::token::Token;

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
            Operator::AddOp => "+".to_string(),
            Operator::SubOp => "-".to_string(),
            Operator::MultOp => "*".to_string(),
            Operator::DivOp => "/".to_string(),
        }
    }
}

impl Visitor<'_> for Operator {
    fn compile_token(&self, compiler: &mut Compiler) {
        let result_reg = compiler.free_reg.pop().unwrap();
        let right_reg = compiler.used_reg.pop().unwrap();
        let left_reg = compiler.used_reg.pop().unwrap();

        let op = match &self {
            Operator::AddOp => "ADD",
            Operator::SubOp => "SUB",
            Operator::MultOp => "MUL",
            Operator::DivOp => "DIV",
        };

        let line = format!("{} ${} ${} ${}", op, left_reg, right_reg, result_reg);
        compiler.assembly.push(line);
        compiler.used_reg.push(result_reg);
        compiler.free_reg.push(right_reg);
        compiler.free_reg.push(left_reg);
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
        let (_, add) = Operator::parse("+").unwrap();
        let (_, sub) = Operator::parse("-").unwrap();
        let (_, mul) = Operator::parse("*").unwrap();
        let (_, div) = Operator::parse("/").unwrap();

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

    #[test]
    fn test_compile_add() {
        let operator = Operator::AddOp;
        let mut compiler = Compiler::new();
        compiler.used_reg.push(1);
        compiler.used_reg.push(2);
        Operator::compile_token(&operator, &mut compiler);

        assert_eq!(compiler.assembly.len(), 1);
        assert_eq!(compiler.assembly[0], "ADD $1 $2 $0");
        assert_eq!(compiler.free_reg.len(), 32);
        assert_eq!(compiler.used_reg.len(), 1);
    }

    #[test]
    fn test_compile_sub() {
        let operator = Operator::SubOp;
        let mut compiler = Compiler::new();
        compiler.used_reg.push(1);
        compiler.used_reg.push(2);
        Operator::compile_token(&operator, &mut compiler);

        assert_eq!(compiler.assembly.len(), 1);
        assert_eq!(compiler.assembly[0], "SUB $1 $2 $0");
        assert_eq!(compiler.free_reg.len(), 32);
        assert_eq!(compiler.used_reg.len(), 1);
    }
}

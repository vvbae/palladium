use nom::{
    bytes::complete::tag, character::complete::digit1, combinator::opt, error::context,
    sequence::tuple,
};

use crate::parse::{Parse, ParseResult};

use super::{token::Token, visitor::Visitor};

#[derive(Debug, PartialEq)]
pub enum FactorValue {
    Int(i64),
    Str(String),
}

impl Token for FactorValue {
    fn display_token(&self) -> String {
        match self {
            FactorValue::Int(v) => format!("{:?}", v),
            FactorValue::Str(v) => format!("{:?}", v),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Factor {
    Integer(FactorValue),
}

pub fn parse_int(input: &str) -> ParseResult<'_, Factor> {
    let (remaining, (sign, val)) =
        context("Integer Operand", tuple((opt(tag("-")), digit1)))(input)?;

    let val = match sign {
        Some(_) => FactorValue::Int(-1 * val.parse::<i64>().unwrap()),
        None => FactorValue::Int(val.parse::<i64>().unwrap()),
    };

    Ok((remaining, Factor::Integer(val)))
}

impl Parse<'_> for Factor {
    fn parse(input: &'_ str) -> crate::parse::ParseResult<'_, Self> {
        let (remaining, val) = context("Factor", parse_int)(input)?;

        Ok((remaining, val))
    }
}

impl Token for Factor {
    fn display_token(&self) -> String {
        match &self {
            Factor::Integer(fac_val) => fac_val.display_token(),
            _ => format!("todo!"),
        }
    }
}

impl Visitor for Factor {
    fn visit_token<T: Token>(&self) {
        println!("{}", self.display_token())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        let expected = vec![
            Factor::Integer(FactorValue::Int(32)),
            Factor::Integer(FactorValue::Int(-32)),
        ];
        let (_, val1) = parse_int("32").unwrap();
        let (_, val2) = parse_int("-32").unwrap();
        assert_eq!(vec![val1, val2], expected);
    }

    #[test]
    fn test_display() {
        let expected = Factor::Integer(FactorValue::Int(32));
        assert_eq!("32", expected.display_token());
    }
}

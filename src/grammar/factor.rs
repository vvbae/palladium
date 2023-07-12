use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{alpha1, digit1},
    combinator::{map, opt},
    error::context,
    sequence::{delimited, tuple},
    Parser,
};

use crate::{
    compiler::Compiler,
    parse::{Parse, ParseResult},
    visitor::Visitor,
};

use super::{expression::Expr, token::Token};

/// <number> | <identifier> | (<expression>)
#[derive(Debug, PartialEq)]
pub enum Factor {
    Integer(i64),
    Float(f64),
    Identifier(String),
    Expr(Expr),
}

/// Parse an integer ie. 98, -3
pub fn parse_int(input: &str) -> ParseResult<'_, Factor> {
    let (remaining, (sign, val)) = context("Integer", tuple((opt(tag("-")), digit1)))(input)?;

    let val = match sign {
        Some(_) => -val.parse::<i64>().unwrap(),
        None => val.parse::<i64>().unwrap(),
    };

    Ok((remaining, Factor::Integer(val)))
}

/// Parse a float ie. 98.4, -3.2
pub fn parse_float(input: &str) -> ParseResult<'_, Factor> {
    let (remaining, val) = context(
        "Float",
        map(
            tuple((opt(tag("-")), digit1, tag("."), digit1)),
            |(sign, left, _, right)| {
                let value = format!("{}.{}", left, right).parse::<f64>().unwrap();
                match sign {
                    Some(_) => -1.0 * value,
                    None => value,
                }
            },
        ),
    )(input)?;

    Ok((remaining, Factor::Float(val)))
}

/// Parse an identifier ie. identi
pub fn parse_identifier(input: &str) -> ParseResult<'_, Factor> {
    let (remaining, val) = context("Identifier", alpha1)(input)?;

    Ok((remaining, Factor::Identifier(val.to_owned())))
}

/// Parse a parenthesized expression ie. (9 *-8)
pub fn parse_paren_expr(input: &str) -> ParseResult<'_, Factor> {
    let (remaining, val) = context(
        "Parenthesized Expression",
        delimited(tag("("), take_until(")"), tag(")")).and_then(Expr::parse),
    )(input)?;

    Ok((remaining, Factor::Expr(val)))
}

impl Parse<'_> for Factor {
    fn parse(input: &'_ str) -> crate::parse::ParseResult<'_, Self> {
        let (remaining, val) = context(
            "Factor",
            alt((parse_identifier, parse_float, parse_int, parse_paren_expr)),
        )(input)?;

        Ok((remaining, val))
    }
}

impl Token for Factor {
    fn display_token(&self) -> String {
        todo!()
        // TODO:
        // match &self {
        //     Factor::Integer(fac_val) => fac_val.display_token(),
        //     _ => format!("todo!"),
        // }
    }
}

impl Visitor<'_> for Factor {
    fn compile_token(&self, compiler: &mut Compiler) {
        match &self {
            Factor::Integer(val) => {
                let next_reg = compiler.free_reg.pop().unwrap();
                let line = format!("LOAD ${} #{}", next_reg, val);
                compiler.used_reg.push(next_reg);
                compiler.assembly.push(line);
            }
            Factor::Float(_) => todo!(),
            Factor::Identifier(_) => todo!(),
            Factor::Expr(expr) => expr.compile_token(compiler),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::grammar::{operator::Operator, term::Term};

    use super::*;

    #[test]
    fn test_parse() {
        let (_, int1) = Factor::parse("32").unwrap();
        let (_, int2) = Factor::parse("-32").unwrap();
        let (_, flo1) = Factor::parse("32.55").unwrap();
        let (_, flo2) = Factor::parse("-32.0").unwrap();
        let (_, id) = Factor::parse("x").unwrap();
        let (_, expr) = Factor::parse("( -9 * 8 )").unwrap();
    }

    #[test]
    fn test_compile() {
        let factor = Factor::Integer(-9);
        let mut compiler = Compiler::new();
        Factor::compile_token(&factor, &mut compiler);

        assert_eq!(compiler.assembly.len(), 1);
        assert_eq!(compiler.assembly[0], "LOAD $0 #-9");
        assert_eq!(compiler.free_reg.len(), 30);
        assert_eq!(compiler.used_reg.len(), 1);
    }

    // #[test]
    // fn test_display() {
    //     let expected = Factor::Integer(32);
    //     assert_eq!("32", expected.display_token());
    // }
}

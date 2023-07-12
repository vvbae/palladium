use nom::{
    branch::alt, character::complete::multispace0, combinator::map, error::context, multi::many0,
    sequence::tuple,
};

use crate::{
    compiler::Compiler,
    parse::{Parse, ParseResult},
    visitor::Visitor,
};

use super::{
    factor::Factor,
    operator::{parse_div, parse_mul, Operator},
};

#[derive(Debug, PartialEq)]
pub struct Term {
    pub left: Box<Factor>,
    pub rest: Option<Vec<(Operator, Factor)>>,
}

impl Parse<'_> for Term {
    fn parse(input: &'_ str) -> ParseResult<'_, Self> {
        let (remaining, val) = context(
            "Term",
            map(
                tuple((
                    Factor::parse,
                    many0(tuple((
                        multispace0,
                        alt((parse_mul, parse_div)),
                        multispace0,
                        Factor::parse,
                        multispace0,
                    ))),
                )),
                |(left, rest)| match rest.len() {
                    0 => Self {
                        left: Box::new(left),
                        rest: None,
                    },
                    _ => {
                        let right = rest
                            .into_iter()
                            .map(|(_, op, _, right, _)| (op, right))
                            .collect();
                        Self {
                            left: Box::new(left),
                            rest: Some(right),
                        }
                    }
                },
            ),
        )(input)?;

        Ok((remaining, val))
    }
}

// impl Token for Term {
//     fn display_token(&self) -> String {
//         match &self {
//             Factor::Integer(fac_val) => fac_val.display_token(),
//             _ => format!("todo!"),
//         }
//     }
// }

impl Visitor<'_> for Term {
    fn compile_token(&self, compiler: &mut Compiler) {
        self.left.compile_token(compiler);
        if let Some(rest) = &self.rest {
            rest.iter().for_each(|(op, right)| {
                right.compile_token(compiler);
                op.compile_token(compiler);
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let val1 = Term::parse("(3*4)*2");
        let val2 = Term::parse("4*(3-4)");
        assert!(val1.is_ok());
        assert!(val2.is_ok());
    }

    #[test]
    fn test_compile() {
        let (_, expected) = Term::parse("-9*(8-2)").unwrap();
        let mut compiler = Compiler::new();
        Term::compile_token(&expected, &mut compiler);
        assert_eq!(compiler.assembly.len(), 5);
        assert_eq!(compiler.free_reg.len(), 30);
        assert_eq!(compiler.used_reg.len(), 1);
    }

    // #[test]
    // fn test_display() {
    //     let expected = Factor::Integer(FactorValue::Int(32));
    //     assert_eq!("32", expected.display_token());
    // }
}

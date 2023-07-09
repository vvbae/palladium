use nom::{
    bytes::complete::tag,
    character::complete::multispace0,
    combinator::{map, opt},
    error::context,
    sequence::tuple,
};

use crate::parse::{Parse, ParseResult};

use super::{compiler::Compiler, factor::Factor, operator::Operator, visitor::Visitor};

/// <factor> | <factor> <operator> <factor>
#[derive(Debug, PartialEq)]
pub struct Term {
    pub left: Box<Factor>,
    pub op: Option<Box<Operator>>,
    pub right: Option<Box<Factor>>,
}

impl Parse<'_> for Term {
    fn parse(input: &'_ str) -> ParseResult<'_, Self> {
        let (remaining, val) = context(
            "Term",
            map(
                tuple((
                    opt(tag("(")),
                    multispace0,
                    Factor::parse,
                    opt(tuple((
                        multispace0,
                        Operator::parse,
                        multispace0,
                        Factor::parse,
                        multispace0,
                        opt(tag(")")),
                    ))),
                )),
                |(_, _, left, rest)| match rest {
                    Some((_, op, _, right, _, _)) => Self {
                        left: Box::new(left),
                        op: Some(Box::new(op)),
                        right: Some(Box::new(right)),
                    },
                    None => Self {
                        left: Box::new(left),
                        op: None,
                        right: None,
                    },
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
        if let Some(right) = &self.right {
            right.compile_token(compiler);
        }
        if let Some(op) = &self.op {
            op.compile_token(compiler);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let expected = vec![
            Term {
                left: Box::new(Factor::Integer(9)),
                op: None,
                right: None,
            },
            Term {
                left: Box::new(Factor::Integer(-9)),
                op: Some(Box::new(Operator::AddOp)),
                right: Some(Box::new(Factor::Integer(8))),
            },
        ];
        let (_, val1) = Term::parse("9").unwrap();
        let (_, val2) = Term::parse("-9+ 8").unwrap();
        assert_eq!(vec![val1, val2], expected);
    }

    #[test]
    fn test_compile() {
        let term = Term {
            left: Box::new(Factor::Integer(-9)),
            op: Some(Box::new(Operator::AddOp)),
            right: Some(Box::new(Factor::Integer(8))),
        };
        let mut compiler = Compiler::new();
        Term::compile_token(&term, &mut compiler);

        assert_eq!(compiler.assembly.len(), 3);
        assert_eq!(compiler.assembly[0], "LOAD $0 #-9");
        assert_eq!(compiler.assembly[1], "LOAD $1 #8");
        assert_eq!(compiler.assembly[2], "ADD $0 $1 $2");
        assert_eq!(compiler.free_reg.len(), 30);
        assert_eq!(compiler.used_reg.len(), 1);
    }

    // #[test]
    // fn test_display() {
    //     let expected = Factor::Integer(FactorValue::Int(32));
    //     assert_eq!("32", expected.display_token());
    // }
}

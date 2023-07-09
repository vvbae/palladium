use nom::{
    character::complete::multispace0,
    combinator::{map, opt},
    error::context,
    sequence::tuple,
};

use crate::parse::{Parse, ParseResult};

use super::{
    compiler::Compiler, factor::Factor, operator::Operator, term::Term, token::Token,
    visitor::Visitor,
};

/// <term> | <term> <operator> <term>
#[derive(Debug, PartialEq)]
pub struct Expr {
    pub left: Box<Term>, // box(pointer): known size at compile time
    pub op: Option<Box<Operator>>,
    pub right: Option<Box<Term>>,
}

impl Parse<'_> for Expr {
    fn parse(input: &'_ str) -> ParseResult<'_, Self> {
        let (remaining, val) = context(
            "Expression",
            map(
                tuple((
                    Term::parse,
                    opt(tuple((
                        multispace0,
                        Operator::parse,
                        multispace0,
                        Term::parse,
                    ))),
                )),
                |(left, rest)| match rest {
                    Some((_, op, _, right)) => Self {
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

impl Token for Expr {
    fn display_token(&self) -> String {
        todo!()
        // let left: &ExprValue = &*self.left;
        // let op: &ExprValue = &*self.op;
        // let right: &ExprValue = &*self.right;

        // format!(
        //     "{} {} {}",
        //     left.display_token(),
        //     op.display_token(),
        //     right.display_token()
        // )
    }
}

impl Visitor<'_> for Expr {
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
        ];
        let (_, val1) = Expr::parse("(-9- 8)").unwrap();
        let (_, val2) = Expr::parse("(-9.8  - 8 ) + (2.0*x)").unwrap();
        assert_eq!(vec![val1, val2], expected);
    }

    #[test]
    fn test_compile() {
        let expr = Expr {
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
        };
        let mut compiler = Compiler::new();
        Expr::compile_token(&expr, &mut compiler);

        assert_eq!(compiler.assembly.len(), 7);

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
            ]
        );

        assert_eq!(compiler.free_reg.len(), 30);
        assert_eq!(compiler.used_reg.len(), 1);
    }

    // #[test]
    // fn test_display() {
    // let expected = Expr {
    //     left: Box::new(ExprValue::Factor(Factor::Integer(FactorValue::Int(6)))),
    //     op: Box::new(ExprValue::Operator(Operator::AddOp)),
    //     right: Box::new(ExprValue::Factor(Factor::Integer(FactorValue::Int(7)))),
    // };

    // assert_eq!("6 + 7", expected.display_token());
    // }
}

use nom::{
    character::complete::multispace0, combinator::map, error::context, multi::many0,
    sequence::tuple,
};

use crate::{
    compiler::Compiler,
    parse::{Parse, ParseResult},
    visitor::Visitor,
};

use super::{operator::Operator, term::Term, token::Token};

/// <term> | <term> <operator> <term>
#[derive(Debug, PartialEq)]
pub struct Expr {
    pub left: Box<Term>, // box(pointer): known size at compile time
    pub rest: Option<Vec<(Operator, Term)>>,
}

impl Parse<'_> for Expr {
    fn parse(input: &'_ str) -> ParseResult<'_, Self> {
        let (remaining, val) = context(
            "Expression",
            map(
                tuple((
                    multispace0,
                    Term::parse,
                    many0(tuple((
                        multispace0,
                        Operator::parse,
                        multispace0,
                        Term::parse,
                    ))),
                )),
                |(_, left, rest)| match rest.len() {
                    0 => Self {
                        left: Box::new(left),
                        rest: None,
                    },
                    _ => {
                        let right = rest
                            .into_iter()
                            .map(|(_, op, _, right)| (op, right))
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
        let v1 = Expr::parse("(-9.8  - 8 ) + (2.0*x)");
        let v2 = Expr::parse(" -1 +9 * -8 - (3*x)  ");
        assert!(v1.is_ok());
        assert!(v2.is_ok());
    }

    #[test]
    fn test_compile() {
        let (_, expected) = Expr::parse("-1 +9 * -8 - (3*1) ").unwrap();
        let mut compiler = Compiler::new();
        Expr::compile_token(&expected, &mut compiler);
        assert_eq!(compiler.assembly.len(), 9);
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

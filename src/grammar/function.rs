//! def somefunction(arg1, arg2):
//!     expressions
//! \n

use super::expression::Expr;
pub struct Function {
    name: String, // `a-zA-Z0-9`
    args: Vec<String>,
    body: Vec<Expr>,
}

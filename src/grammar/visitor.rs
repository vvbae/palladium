use super::token::Token;

pub trait Visitor {
    /// Visits each node in tree, examines it, and outputs the IASM code
    fn visit_token<T: Token>(&self);
}

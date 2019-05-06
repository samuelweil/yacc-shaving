#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Token {
    Integer(i64),
    Plus,
    Minus
}
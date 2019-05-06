use nom::*;
use nom::types::*;

use std::str::FromStr;

mod token;
use token::Token;

named!(operator_plus<CompleteByteSlice, Token>,
    do_parse!(tag!("+") >> (Token::Plus))
);

named!(operator_minus<CompleteByteSlice, Token>,
    do_parse!(tag!("-") >> (Token::Minus))
);

named!(operators<CompleteByteSlice, Token>,
    alt!(operator_plus | operator_minus)
);

named!(lex_token<CompleteByteSlice, Token>,
    alt_complete!(operators)
);

named!(lex_tokens<CompleteByteSlice, Vec<Token> >,
    ws!(many0!(lex_token))
);

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! check {
        ($func:ident, $inp:expr, $exp:expr) => (
            assert_eq!($func(CompleteByteSlice($inp)), Ok((CompleteByteSlice(b""), $exp)))
        )
    }

    #[test]
    fn test_operators() {
        check!(operator_plus, b"+", Token::Plus);
        check!(operator_minus, b"-", Token::Minus);
    }
}

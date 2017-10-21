#[derive(Clone)]
pub enum Token {
    EndOfFile,
    Identifier(String),

    // math
    Plus,
    Minus,
    Multiply,
    Divide,
    Log,
    Ln,
    Exponent,
    NaturalExponent,
    SquareRoot,

    // bit ops
    BShiftR,
    BShiftL,
    BAnd,
    BOr,
    BXOr,

    // delimiters
    Semicolon,
    LeftParen,
    RightParen,

    // constants
    E,
    Pi,
}

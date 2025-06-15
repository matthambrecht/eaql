use std::fmt;

#[derive(Debug)]
pub enum TokenType {
    // Single Char Tokens
    OpenParen, CloseParen, EoqToken,
    Comma,
    
    // One or Two Char Tokens
    Gte, Lte, Equal, Lt, Gt,

    // Literals
    Identifier, StringLiteral, NumberLiteral,

    // Keywords
    DeleteKeyword, CreateKeyword, SortHelper, SortType,
    WildcardKeyword, FilterKeyword, PostProcessorEntrance,
    Database, Get, From, And, Order, Sort, Not
}

#[derive(Debug)]
pub struct Token {
    token_type: TokenType,
    literal: String,
    lexeme: String,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        literal: &String,
        lexeme: &String
    ) -> Token {
        Token {
            token_type: token_type,
            literal: literal.to_owned(),
            lexeme: lexeme.to_owned()
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token {{ Type: {:#?}, Literal: {}, Lexeme: {} }}",
            self.token_type, self.literal, self.lexeme)
    }
}


// These are for finding unique cases of tokens
pub const SINGLE_START_TOKENS: &[char] = &[
    '(', ')', '!', '.', ';', ','
];


pub const SINGLE_DOUBLE_START_TOKENS: &[char] = &[
    '<', '>'
];
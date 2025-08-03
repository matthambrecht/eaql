use std::{collections::HashMap, fmt};
use lazy_static::lazy_static;

#[derive(Debug, PartialEq, Clone, Copy)]
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
    Database, Get, From, And, Or, Order, Sort, Not, LimitKeyword,

    // Defaults
    UnknownToken, WhitespaceToken, NullToken
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub lexeme: String,
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
    '<', '>', '='
];


lazy_static! {
    pub static ref IDENTIFER_STOPS: Vec<char> = {
        let mut rv: Vec<char> = vec![];

        rv.extend_from_slice(SINGLE_DOUBLE_START_TOKENS);
        rv.extend_from_slice(SINGLE_START_TOKENS);
        rv.extend_from_slice(&['\"', ' ']);

        return rv;
    };

    pub static ref SYSTEM_KEYWORDS: HashMap<&'static str, TokenType> = {
        return HashMap::from([
            ("delete", TokenType::DeleteKeyword),
            ("remove", TokenType::DeleteKeyword),

            ("create", TokenType::CreateKeyword),
            ("make", TokenType::CreateKeyword),
            ("add", TokenType::CreateKeyword),

            ("in", TokenType::SortHelper),
            ("by", TokenType::SortHelper),

            ("ascending", TokenType::SortType),
            ("descending", TokenType::SortType),

            ("any", TokenType::WildcardKeyword),
            ("all", TokenType::WildcardKeyword),
            ("everything", TokenType::WildcardKeyword),

            ("where", TokenType::FilterKeyword),
            ("whenever", TokenType::FilterKeyword),
            ("wherever", TokenType::FilterKeyword),

            ("then", TokenType::PostProcessorEntrance),
            ("afterwords", TokenType::PostProcessorEntrance),
            ("after", TokenType::PostProcessorEntrance),

            ("limit", TokenType::LimitKeyword),

            ("database", TokenType::Database),

            ("find", TokenType::Get),
            ("retrieve", TokenType::Get),
            ("get", TokenType::Get),

            ("from", TokenType::From),

            ("and", TokenType::And),

            ("order", TokenType::Order),

            ("sort", TokenType::Sort),

            ("not", TokenType::Not),

            ("is", TokenType::Equal),

            ("or", TokenType::Or),

            // Tokens to be ignored by the lexer
            ("me", TokenType::NullToken),
            ("the", TokenType::NullToken),
            ("it", TokenType::NullToken),
            ("in", TokenType::NullToken),
            ("to", TokenType::NullToken)
        ]);
    };
}
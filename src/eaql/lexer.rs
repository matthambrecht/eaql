use std::fmt;
use crate::utils::logger;
use crate::eaql::tokens::{
    Token,
    TokenType,
    SINGLE_START_TOKENS,
    SINGLE_DOUBLE_START_TOKENS};

#[derive(Debug)]
struct Lexer {
    tokens: Vec<Token>
}

impl Lexer {
    pub fn handle_single_token(
        _query: &String,
        c: char,
        current: &mut usize
    ) -> Token {
        let token_type: TokenType = match c {
            n if ['.', '!', '.'].contains(&n) => TokenType::EoqToken,
            ')' => TokenType::CloseParen,
            '(' => TokenType::OpenParen,
            ',' => TokenType::Comma,
            _ => TokenType::EoqToken,
        };

        *current += 1;

        return Token::new(
            token_type,
            &"".to_string(),
            &c.to_string()
        );
    }

    pub fn next_token(
        query: &String,
        current: &mut usize,
    ) -> Token {
        let c = query.chars().nth(*current).unwrap();

        if SINGLE_START_TOKENS.contains(&c) {
            return Lexer::handle_single_token(query, c, current);
        } else {
            logger::error("Unknown token found!!!");
        }
    }

    pub fn new(query: &String) -> Lexer {
        let mut toks: Vec<Token> = vec![];
        let mut start: usize = 0;
        let mut current: usize = 0;

        while current < query.len() {
            toks.push(Lexer::next_token(query, &mut current));
            start = current;
        }

        Lexer {
            tokens: toks
        }
    }
}

impl fmt::Display for Lexer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Lexer {{ Tokens: {:#?} }}",
            self.tokens)
    }
}

pub fn scan_tokens(query: &String) {
    logger::debug(&format!("Raw Input: '{}'", query));
    let lexer: Lexer = Lexer::new(query);
    println!("{}", lexer);
}

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
    // Look ahead for one token
    fn peek_one(
        query: &String,
        current: &usize
    ) -> Option<char> {
        if current + 1 >= query.len() {
            return None;
        }

        return query.chars().nth(*current + 1);
    }

    // Handle non-complex tokens like '('
    fn handle_single_token(
        _query: &String,
        c: &char,
        current: &mut usize
    ) -> Token {
        let token_type: TokenType = match c {
            n if [';', '!', '.'].contains(&n) => TokenType::EoqToken,
            ')' => TokenType::CloseParen,
            '(' => TokenType::OpenParen,
            ',' => TokenType::Comma,
            _ => logger::error("{Lexer/Single} Unknown token found!")
        };

        *current += 1;

        return Token::new(
            token_type,
            &"".to_string(),
            &c.to_string()
        );
    }

    /* Handle slightly complex tokens like '>' which might be 
    succeeded by '=' */
    fn handle_single_double_token(
        query: &String,
        c: char,
        current: &mut usize
    ) -> Token {
        let start = *current;
        let peeked_token: Option<char> = Lexer::peek_one(query, current);
        let token_type: TokenType = 
        if c == '>' {
            if peeked_token == Some('=') {
                *current += 2;
                TokenType::Gte
           } else {
                *current += 1;
                TokenType::Gt
           }
        } else if c == '<' {
            if peeked_token == Some('=') {
                *current += 2;
                TokenType::Lte
            } else {
                *current += 1;
                TokenType::Lt
            }
        } else if c == '=' {
            *current += 1;
            TokenType::Equal
        } else {
            logger::error("{Lexer/SingleDouble} Unknown token found!");
        };
        let end: usize = *current;

        return Token::new(
            token_type,
            &"".to_string(),
            &query[start..end].to_string()
        );
    }

    fn next_token(
        query: &String,
        current: &mut usize,
    ) -> Token {
        let c = query.chars().nth(*current).unwrap();

        if SINGLE_START_TOKENS.contains(&c) {
            return Lexer::handle_single_token(
                query,
                &c,
                current);
        } else if SINGLE_DOUBLE_START_TOKENS.contains(&c) {
            return Lexer::handle_single_double_token(
                query,
                c,
                current);
        } else {
            logger::error("Unknown token found!");
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


// Begin Lexer Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_single_tokens() {
        let input: String = "()!.;,".to_string();
        let test_lexer: Lexer = Lexer::new(&input);
        let expected: Vec<Token> = vec![
            Token::new(
                TokenType::OpenParen,
                &"".to_string(),
                &"(".to_string(),
            ),
            Token::new(
                TokenType::CloseParen,
                &"".to_string(),
                &")".to_string(),
            ),
            Token::new(
                TokenType::EoqToken,
                &"".to_string(),
                &"!".to_string(),
            ),
            Token::new(
                TokenType::EoqToken,
                &"".to_string(),
                &".".to_string(),
            ),
            Token::new(
                TokenType::EoqToken,
                &"".to_string(),
                &";".to_string(),
            ),
            Token::new(
                TokenType::Comma,
                &"".to_string(),
                &",".to_string(),
            ),
        ];

        assert_eq!(expected, test_lexer.tokens);
    }

    #[test]
    fn test_basic_single_double_tokens() {
        let input: String = "<><=>==".to_string();
        let test_lexer: Lexer = Lexer::new(&input);
        let expected: Vec<Token> = vec![
            Token::new(
                TokenType::Lt,
                &"".to_string(),
                &"<".to_string(),
            ),
            Token::new(
                TokenType::Gt,
                &"".to_string(),
                &">".to_string(),
            ),
            Token::new(
                TokenType::Lte,
                &"".to_string(),
                &"<=".to_string(),
            ),
            Token::new(
                TokenType::Gte,
                &"".to_string(),
                &">=".to_string(),
            ),
            Token::new(
                TokenType::Equal,
                &"".to_string(),
                &"=".to_string(),
            ),
        ];

        assert_eq!(expected, test_lexer.tokens);
    }
}
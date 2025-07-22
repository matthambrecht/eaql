use std::fmt;
use crate::eaql::{
    language::{
        tokens::{
            Token,
            TokenType,
            IDENTIFER_STOPS,
            SINGLE_DOUBLE_START_TOKENS,
            SINGLE_START_TOKENS,
            SYSTEM_KEYWORDS
        }
    }
};

#[derive(Debug)]
pub struct Lexer {
    pub tokens: Vec<Token>
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

    fn peek_decimal(
        query: &String,
        start: &mut usize,
        current: &mut usize,
    ) {
        loop {
            if *current >= query.len() {
                return;
            }

            *current += 1;
    
            let slice = &query.as_bytes()[*start..*current];
            let s = match std::str::from_utf8(slice) {
                Ok(s) => s,
                Err(_) => {
                    *current -= 1;
                    return;
                }
            };
    
            if s.parse::<f64>().is_err() {
                *current -= 1;
                return;
            }
        }
    }

    fn peek_string(
        query: &String,
        current: &mut usize,
        token_type: &mut TokenType
    ) -> () {
        loop {
            if *current + 1 >= query.len() {
                *token_type = TokenType::UnknownToken;
                return;
            }

            *current += 1;
    
            if query.chars().nth(*current).unwrap() == '\"' {
                return;
            }
        }
    }

    fn peek_identifier(
        query: &String,
        start: &mut usize,
        current: &mut usize,
        token_type: &mut TokenType,
    ) -> () {
        loop {
            if *current >= query.len() {
                return;
            }

            if IDENTIFER_STOPS.contains(
                &query.chars().nth(*current).unwrap()) {
                let tmp = &query[*start..*current];

                if let Some(keyword_token) = SYSTEM_KEYWORDS.get(tmp) {
                    *token_type = keyword_token.clone();
                }

                return;
            }

            *current += 1;
        }
    }

    fn handle_single_token(
        _query: &String,
        c: char,
        current: &mut usize
    ) -> Result<Token, String> {
        let token_type: TokenType = match c {
            n if [';', '!', '.'].contains(&n) => TokenType::EoqToken,
            ')' => TokenType::CloseParen,
            '(' => TokenType::OpenParen,
            ',' => TokenType::Comma,
            _ => TokenType::UnknownToken
        };

        *current += 1;

        return Ok(Token::new(
            token_type,
            &"".to_string(),
            &c.to_string()
        ));
    }

    /* Handle slightly complex tokens like '>' which might be 
    succeeded by '=' */
    fn handle_single_double_token(
        query: &String,
        c: char,
        current: &mut usize
    ) -> Result<Token, String> {
        let slice_start = *current;
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
            TokenType::UnknownToken
        };

        let slice_end: usize = *current;

        return Ok(Token::new(
            token_type,
            &"".to_string(),
            &query[slice_start..slice_end].to_string()
        ));
    }

    fn handle_default(
        query: &String,
        c: char,
        current: &mut usize,
        start: &mut usize
    ) -> Result<Token, String> {
        let mut token_type: TokenType;
        let literal: String;
        let slice_start: usize = *current;

        if c == ' ' { // Whitespace
            token_type = TokenType::WhitespaceToken;
            literal = " ".to_string()
        } else if c.is_digit(10) ||
            (
                c == '-' &&
                Lexer::peek_one(query, &current).is_some_and(|x: char| x.is_digit(10))
            ) { // Number literals
            token_type = TokenType::NumberLiteral;

            if c == '-' &&
               Lexer::peek_one(query, &current).is_some_and(|x: char| x.is_digit(10)) {
                *current += 1;
            }

            Lexer::peek_decimal(
                query,
                start,
                current);
                
            literal = query[*start..*current].to_string();
            *current -= 1;
        } else if c == '\"' {
            token_type = TokenType::StringLiteral;

            Lexer::peek_string(
                query,
                current,
                &mut token_type
            );

            if token_type == TokenType::UnknownToken {
                *current += 1; // We weren't able to increment in the loop
                return Ok(Token::new(
                    token_type,
                    &"".to_string(),
                    &query[slice_start..*current].to_string()
                ));
            }
            
            literal = query[*start + 1..*current].to_string();
        } else { // This is where we handle an identifier, or keyword
            token_type = TokenType::Identifier;

            Lexer::peek_identifier(
                query,
                start,
                current,
                &mut token_type);
            
            // Keywords don't need literals
            literal = if token_type == TokenType::Identifier {
                query[*start..*current].to_string()
            } else {
                "".to_string()
            };

            *current -= 1;
        }

        *current += 1;
        let slice_end: usize = *current;

        return Ok(Token::new(
            token_type,
            &literal,
            &query[slice_start..slice_end].to_string()
        ));
    }

    fn next_token(
        query: &String,
        current: &mut usize,
        start: &mut usize,
    ) -> Result<Token, String> {
        let c: char = query.chars().nth(*current).unwrap();

        if SINGLE_START_TOKENS.contains(&c) {
            return Lexer::handle_single_token(
                query,
                c,
                current);
        } else if SINGLE_DOUBLE_START_TOKENS.contains(&c) {
            return Lexer::handle_single_double_token(
                query,
                c,
                current);
        } else {
            return Lexer::handle_default(
                query,
                c,
                current,
                start)
        }
    }

    pub fn new(query: &String) -> Result<Lexer, String> {
        let mut toks: Vec<Token> = vec![];
        let mut warnings: Vec<String> = vec![];
        let mut start: usize = 0;
        let mut current: usize = 0;

        while current < query.len() {
            let token: Result<Token, String> = Lexer::next_token(
                query,
                &mut current,
                &mut start);
            
            if token.is_err() {
                warnings.push(token.err().unwrap());
            } else {
                toks.push(token.unwrap());
            }

            start = current;
        }

        Ok(Lexer {
            tokens: toks
                .into_iter()
                .filter(|x: &Token| x.token_type != TokenType::WhitespaceToken)
                .collect()
        })
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

pub fn scan_tokens(query: &String) -> Result<Lexer, String>{
    Lexer::new(query)
}


// Begin Lexer Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_single_tokens() {
        let input: String = "()!.;,".to_string();
        let test_lexer: Result<Lexer, String> = Lexer::new(&input);

        assert!(!test_lexer.is_err());

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

        assert_eq!(expected, test_lexer.unwrap().tokens);
    }

    #[test]
    fn test_basic_single_double_tokens() {
        let input: String = "<><=>==".to_string();
        let test_lexer: Result<Lexer, String> = Lexer::new(&input);
        
        assert!(!test_lexer.is_err());

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

        assert_eq!(expected, test_lexer.unwrap().tokens);
    }

    #[test]
    fn test_basic_string_literal_base() {
        let input: String = "\"Hi1234\"".to_string();
        let test_lexer: Result<Lexer, String> = Lexer::new(&input);

        assert!(!test_lexer.is_err());

        let expected: Vec<Token> = vec![
            Token::new(
                TokenType::StringLiteral,
                &"Hi1234".to_string(),
                &"\"Hi1234\"".to_string(),
            ),
        ];

        assert_eq!(expected, test_lexer.unwrap().tokens);
    }

    #[test]
    fn test_basic_string_literal_error_1() {
        let input: String = "\"Hi1234".to_string();
        let test_lexer: Result<Lexer, String> = Lexer::new(&input);

        assert!(!test_lexer.is_err());

        let expected: Vec<Token> = vec![
            Token::new(
                TokenType::UnknownToken,
                &"".to_string(),
                &"\"Hi1234".to_string(),
            ),
        ];

        assert_eq!(expected, test_lexer.unwrap().tokens);
    }

    #[test]
    fn test_basic_string_literal_error_2() {
        let input: String = "\"".to_string();
        let test_lexer: Result<Lexer, String> = Lexer::new(&input);
        
        assert!(!test_lexer.is_err());
        
        let expected: Vec<Token> = vec![
            Token::new(
                TokenType::UnknownToken,
                &"".to_string(),
                &"\"".to_string(),
            ),
        ];

        assert_eq!(expected, test_lexer.unwrap().tokens);
    }

    #[test]
    fn test_basic_string_literal_edge() {
        let input: String = "\"\"".to_string();
        let test_lexer: Result<Lexer, String> = Lexer::new(&input);

        assert!(!test_lexer.is_err());

        let expected: Vec<Token> = vec![
            Token::new(
                TokenType::StringLiteral,
                &"".to_string(),
                &"\"\"".to_string(),
            ),
        ];

        assert_eq!(expected, test_lexer.unwrap().tokens);
    }

    #[test]
    fn test_basic_number_literal_base() {
        let input: String = "1234".to_string();
        let test_lexer: Result<Lexer, String> = Lexer::new(&input);

        assert!(!test_lexer.is_err());

        let expected: Vec<Token> = vec![
            Token::new(
                TokenType::NumberLiteral,
                &"1234".to_string(),
                &"1234".to_string(),
            ),
        ];

        assert_eq!(expected, test_lexer.unwrap().tokens);
    }

    #[test]
    fn test_basic_number_literal_decimal() {
        let input: String = "12.34".to_string();
        let test_lexer: Result<Lexer, String> = Lexer::new(&input);
        
        assert!(!test_lexer.is_err());
        
        let expected: Vec<Token> = vec![
            Token::new(
                TokenType::NumberLiteral,
                &"12.34".to_string(),
                &"12.34".to_string(),
            ),
        ];

        assert_eq!(expected, test_lexer.unwrap().tokens);
    }

    #[test]
    fn test_basic_number_literal_negative_decimal() {
        let input: String = "-12.34".to_string();
        let test_lexer: Result<Lexer, String> = Lexer::new(&input);

        assert!(!test_lexer.is_err());

        let expected: Vec<Token> = vec![
            Token::new(
                TokenType::NumberLiteral,
                &"-12.34".to_string(),
                &"-12.34".to_string(),
            ),
        ];

        assert_eq!(expected, test_lexer.unwrap().tokens);
    }

    #[test]
    fn test_basic_keyword_literal_1() {
        let input: String = "get all from place.".to_string();
        let test_lexer: Result<Lexer, String> = Lexer::new(&input);
        
        assert!(!test_lexer.is_err());

        let expected: Vec<Token> = vec![
            Token::new(
                TokenType::Get,
                &"".to_string(),
                &"get".to_string(),
            ),
            Token::new(
                TokenType::WildcardKeyword,
                &"".to_string(),
                &"all".to_string(),
            ),
            Token::new(
                TokenType::From,
                &"".to_string(),
                &"from".to_string(),
            ),
            Token::new(
                TokenType::Identifier,
                &"place".to_string(),
                &"place".to_string(),
            ),
            Token::new(
                TokenType::EoqToken,
                &"".to_string(),
                &".".to_string(),
            ),
        ];

        assert_eq!(expected, test_lexer.unwrap().tokens);
    }

    #[test]
    fn test_basic_keyword_literal_2() {
        let input: String = "retrieve everything from place whenever name is \"Coffee\" and cost >= 2.43!".to_string();
        let test_lexer: Result<Lexer, String> = Lexer::new(&input);
        
        assert!(!test_lexer.is_err());

        let expected: Vec<Token> = vec![
            Token::new(
                TokenType::Get,
                &"".to_string(),
                &"retrieve".to_string(),
            ),
            Token::new(
                TokenType::WildcardKeyword,
                &"".to_string(),
                &"everything".to_string(),
            ),
            Token::new(
                TokenType::From,
                &"".to_string(),
                &"from".to_string(),
            ),
            Token::new(
                TokenType::Identifier,
                &"place".to_string(),
                &"place".to_string(),
            ),
            Token::new(
                TokenType::FilterKeyword,
                &"".to_string(),
                &"whenever".to_string(),
            ),
            Token::new(
                TokenType::Identifier,
                &"name".to_string(),
                &"name".to_string(),
            ),
            Token::new(
                TokenType::Equal,
                &"".to_string(),
                &"is".to_string(),
            ),
            Token::new(
                TokenType::StringLiteral,
                &"Coffee".to_string(),
                &"\"Coffee\"".to_string(),
            ),
            Token::new(
                TokenType::And,
                &"".to_string(),
                &"and".to_string()
            ),
            Token::new(
                TokenType::Identifier,
                &"cost".to_string(),
                &"cost".to_string()
            ),
            Token::new(
                TokenType::Gte,
                &"".to_string(),
                &">=".to_string()
            ),
            Token::new(
                TokenType::NumberLiteral,
                &"2.43".to_string(),
                &"2.43".to_string(),
            ),
            Token::new(
                TokenType::EoqToken,
                &"".to_string(),
                &"!".to_string(),
            ),
        ];

        assert_eq!(expected, test_lexer.unwrap().tokens);
    }
}
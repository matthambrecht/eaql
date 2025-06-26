use std::fmt;
use crate::eaql::tokens::{Token, TokenType};
use crate::eaql::parser::helpers::{
    get_tab,
    valid_until_warning, 
    validate_length,
    peek_one
};

#[derive(Debug)]
pub struct GetNode {
    _table: TableNode,
    _columns: ColumnNode,
    _where: Option<WhereNode>,
    _filter: Option<FilterNode>,
    _postprocessor: Option<PostProcessorNode>,

    _depth: u16
}

#[derive(Debug)]
pub struct TableNode {}    

#[derive(Debug, PartialEq)]
pub struct ColumnNode {
    column_names: Vec<String>,
    is_wildcard: bool,

    _depth: u16 
}

#[derive(Debug)]
pub struct WhereNode {}
#[derive(Debug)]
pub struct FilterNode {}
#[derive(Debug)]
pub struct PostProcessorNode {}

impl GetNode {
   pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16) -> Result<GetNode, String>{
        let start_idx: usize = *idx;

        validate_length(
            tokens,
            idx,
            true)?;

        let _columns: Result<ColumnNode, String> = ColumnNode::parse(
            tokens,
            idx,
            depth + 1);

        if _columns.is_err() {
            return Err("Something went wrong parsing column selection!".to_string())
        } else {
            print!("{:#?}", _columns);
        }
        // Ok(GetNode {
        //     _table: TableNode {},
        //     _columns: ColumnNode { column_names: vec![], is_wildcard: true, _depth: 2 },
        //     _where: Some(WhereNode {}),
        //     _filter: Some(FilterNode {}),
        //     _postprocessor: Some(PostProcessorNode {}),
        //     _depth: depth
        // })
        Err(valid_until_warning(tokens, &start_idx))
    }
}


impl ColumnNode {
    pub fn recurse_build(
        tokens: &Vec<Token>,
        cols: &mut Vec<String>,
        idx: &mut usize,
    ) -> Result<(), String> {
        let next_token: TokenType = peek_one(tokens, &idx);
        print!("{:#?}", next_token);
        if next_token != TokenType::And && 
        next_token != TokenType::Comma {
            print!("WTF");
            if tokens[*idx].token_type == TokenType::Identifier {
                cols.push(tokens[*idx].literal.clone());
                *idx += 1;
                return Ok(());
            }
        } else if next_token == TokenType::And || 
        next_token == TokenType::Comma {
            print!("WTH");
            if tokens[*idx].token_type == TokenType::Identifier {
                cols.push(tokens[*idx].literal.clone());
                *idx += 2;
                
                ColumnNode::recurse_build(
                    tokens,
                    cols,
                    idx
                )?;

                return Ok(());
            }
        }
        
        return Err("Something went wrong parsing column names, \
make sure they're in a valid list notation.".to_string()
        );
    }

    pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16) -> Result<ColumnNode, String> {
        if tokens[*idx].token_type == TokenType::WildcardKeyword {
            *idx += 1;

            return Ok(ColumnNode {
                is_wildcard: true,
                column_names: vec![],

                _depth: depth
            });
        }

        let mut column_names: Vec<String> = vec![];
        
        ColumnNode::recurse_build(tokens, &mut column_names, idx)?;

        return Ok(
            ColumnNode {
                is_wildcard: false,
                column_names: column_names,

                _depth: depth
        });
    }
}

impl fmt::Display for GetNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(GetNode){}",
            get_tab(self._depth),
            self._columns,
        )
    }
}

impl fmt::Display for ColumnNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(ColumnNode)
{}is_wildcard: {:#?}
{}column_names: {:#?}
",
            get_tab(self._depth),
            get_tab(self._depth + 1),
            self.is_wildcard,
            get_tab(self._depth + 1),
            self.column_names
        )
    }
}

// Begin Lexer Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_parsing_error() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::Identifier,
                &"id".to_string(),
                &"id".to_string(),
            ),
            Token::new(
                TokenType::Comma,
                &"".to_string(),
                &",".to_string(),
            ),
            Token::new(
                TokenType::And,
                &"".to_string(),
                &"and".to_string(),
            ),
            Token::new(
                TokenType::Identifier,
                &"time".to_string(),
                &"time".to_string()
            )
        ];

        let mut idx: usize = 0;
        let depth: u16 = 0;
        
        match ColumnNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(val) => assert!(false, "Output was expected to error but returned -> {}", val),
                Err(err) => assert!(true, "Output errored out -> {}", err)
        }
    }
 
    #[test]
    fn test_column_parsing_normal_wildcard() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::WildcardKeyword,
                &"".to_string(),
                &"all".to_string(),
            ),
        ];

        let expected: ColumnNode = ColumnNode {
            column_names: vec![],
            is_wildcard: true,
            _depth: 0
        };
        let mut idx: usize = 0;
        let depth: u16 = 0;
        
        match ColumnNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(val) => assert_eq!(expected, val),
                Err(err) => assert!(false, "Output errored out -> {}", err)
            }
    }

    #[test]
    fn test_column_parsing_normal_single() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::Identifier,
                &"id".to_string(),
                &"id".to_string(),
            ),
        ];

        let expected: ColumnNode = ColumnNode {
            column_names: vec![
                "id".to_string(),
            ],
            is_wildcard: false,
            _depth: 0
        };
        let mut idx: usize = 0;
        let depth: u16 = 0;
        
        match ColumnNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(val) => assert_eq!(expected, val),
                Err(err) => assert!(false, "Output errored out -> {}", err)
            }
    }

    #[test]
    fn test_column_parsing_normal_multiple() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::Identifier,
                &"id".to_string(),
                &"id".to_string(),
            ),
            Token::new(
                TokenType::Comma,
                &"".to_string(),
                &",".to_string(),
            ),
            Token::new(
                TokenType::Identifier,
                &"cost".to_string(),
                &"cost".to_string(),
            ),
            Token::new(
                TokenType::And,
                &"".to_string(),
                &"and".to_string(),
            ),
            Token::new(
                TokenType::Identifier,
                &"time".to_string(),
                &"time".to_string()
            )
        ];

        let expected: ColumnNode = ColumnNode {
            column_names: vec![
                "id".to_string(),
                "cost".to_string(),
                "time".to_string()
            ],
            is_wildcard: false,
            _depth: 0
        };
        let mut idx: usize = 0;
        let depth: u16 = 0;
        
        match ColumnNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(val) => assert_eq!(expected, val),
                Err(err) => assert!(false, "Output errored out -> {}", err)
            }
    }
}
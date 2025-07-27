use std::{fmt, usize};
use crate::{
    eaql::language::{
        parser::{
            conditional::ConditionNode,
            helpers::{
                get_tab, peek_one, validate_length
            },
        }, tokens::{
            Token, TokenType
        }
    },
    utils::colors::{
        colorize,
        AnsiColor
    }
};

#[derive(Debug)]
pub struct GetNode {
    _table: TableNode,
    _columns: ColumnNode,
    _filter: Option<FilterNode>,
    _postprocessor: Option<PostProcessorNode>,

    _depth: u16
}

#[derive(Debug, PartialEq)]
pub struct TableNode {
    table_name: String,

    _literal: String,
    _depth: u16
}    

#[derive(Debug, PartialEq)]
pub struct ColumnNode {
    column_names: Vec<String>,
    is_wildcard: bool,

    _literal: String,
    _depth: u16 
}

#[derive(Debug, PartialEq)]
pub struct FilterNode {
    condition: ConditionNode,

    _literal: String,
    _depth: u16
}

#[derive(Debug)]
pub struct PostProcessorNode {
    _depth: u16
}

impl GetNode {
   pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16) -> Result<GetNode, String>{
        validate_length(
            tokens,
            idx,
            true)?;

        let _columns: ColumnNode = match ColumnNode::parse(
            tokens,
            idx,
            depth + 1) {
                Ok(column) => {
                    column
                },
                Err(err) => {
                    return Err(err);
                },
        };

        let _table: TableNode = match TableNode::parse(
            tokens,
            idx,
            depth + 1) {
                Ok(table) => {
                    table
                },
                Err(err) => {
                    return Err(err);
                },
        };

        let _filter: Option<FilterNode> = match FilterNode::parse(
            tokens,
            idx,
            depth + 1) {
                Ok(filter) => {
                   filter 
                },
                Err(err) => {
                    return Err(err);
                },
            };

        Ok(GetNode {
            _table: _table,
            _columns: _columns,
            _filter: _filter,
            _postprocessor: Some(PostProcessorNode {
                _depth: depth + 1
            }),
            _depth: depth
        })
    }

    pub fn transpile(
        &self,
    ) -> (String, String) {
        let columns: (String, String) = self._columns.transpile();
        let table: (String, String) = self._table.transpile();
        let filter: Option<(String, String)> = match &self._filter {
            Some(filter) => Some(filter.transpile()),
            None => None
        };

        (
            [
                Some(columns.0),
                Some(table.0),
                filter.as_ref().map(|f| f.0.clone()),
            ]
                .into_iter()
                .flatten()
                .collect::<Vec<String>>()
                .join(" "),
            [
                Some(columns.1),
                Some(table.1),
                filter.as_ref().map(|f| f.1.clone()),
            ]
                .into_iter()
                .flatten()
                .collect::<Vec<String>>()
                .join(" "),
        )
    }
}

impl TableNode {
    pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16
    ) -> Result<TableNode, String> {
        let start_idx: usize = *idx;

        if tokens[*idx].token_type == TokenType::From &&
        peek_one(tokens, idx) == TokenType::Identifier {
            *idx += 1;
        } else {
            return Err(format!(
                "From-like keyword required for table selection, got \"{}\" instead",
                tokens[*idx].lexeme
            ));
        }

        *idx += 1;

        return Ok(
            TableNode {
                table_name: tokens[*idx - 1].literal.clone(),

                _literal: {
                    tokens[start_idx..*idx].iter()
                        .map(|v| v.lexeme.as_str())
                        .collect::<Vec<&str>>()
                        .join(" ")
                        },


                _depth: depth
        });
    }

    pub fn transpile(
        &self
    ) -> (String, String) {
        (
            colorize(&self._literal, AnsiColor::Blue),
            colorize(&format!("FROM {}", self.table_name), AnsiColor::Blue)
        )
    }
}

impl ColumnNode {
    pub fn recurse_build(
        tokens: &Vec<Token>,
        cols: &mut Vec<String>,
        idx: &mut usize,
    ) -> Result<(), String> {
        let next_token: TokenType = peek_one(tokens, &idx);
        if next_token != TokenType::And && 
        next_token != TokenType::Comma {
            if tokens[*idx].token_type == TokenType::Identifier {
                cols.push(tokens[*idx].literal.clone());
                *idx += 1;
                return Ok(());
            }
        } else if next_token == TokenType::And || 
        next_token == TokenType::Comma {
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
        depth: u16
    ) -> Result<ColumnNode, String> {
        // We subtract 1 from this because Get keyword has been processed already
        let start_idx: usize = *idx - 1; 

        if tokens[*idx].token_type == TokenType::WildcardKeyword {
            *idx += 1;

            return Ok(ColumnNode {
                is_wildcard: true,
                column_names: vec![],

                _literal: {
                    tokens[start_idx..*idx].iter()
                        .map(|v| v.lexeme.as_str())
                        .collect::<Vec<&str>>()
                        .join(" ")
                        },
                _depth: depth
            });
        }

        let mut column_names: Vec<String> = vec![];
        
        ColumnNode::recurse_build(tokens, &mut column_names, idx)?;

        return Ok(
            ColumnNode {
                is_wildcard: false,
                column_names: column_names,

                _literal: {
                    tokens[start_idx..*idx].iter()
                        .map(|v| v.lexeme.as_str())
                        .collect::<Vec<&str>>()
                        .join(" ")
                    },
                _depth: depth
        });
    }

    pub fn transpile(
        &self
    ) -> (String, String) {
        let columns: String = if self.is_wildcard {
            "*".to_string()
        } else {
            self.column_names.join(", ")
        };


        (
            colorize(&self._literal, AnsiColor::Yellow),
            colorize(&format!("SELECT {}", columns), AnsiColor::Yellow)
        )
    }
}

impl FilterNode {
    pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16) -> Result<Option<FilterNode>, String> {

        if tokens[*idx].token_type == TokenType::FilterKeyword {
            let start_idx: usize = *idx;
            *idx += 1;

            let condition_node: ConditionNode = match ConditionNode::parse(
                tokens,
                idx,
                depth + 1) {
                    Ok(condition) => {
                        condition
                    },
                    Err(err) => {
                        return Err(err);
                    }
            };

            return Ok(
                Some(FilterNode {
                    condition: condition_node,
                    
                    _literal: {
                        tokens[start_idx..*idx - 1].iter()
                            .map(|v| v.lexeme.as_str())
                            .collect::<Vec<&str>>()
                            .join(" ")
                        },
                    _depth: depth
            }));
        }

        return Ok(None);
    }

    pub fn transpile(
        &self
    ) -> (String, String) {
        (
            colorize(&self._literal, AnsiColor::Cyan),
            colorize(&format!("WHERE {}", self.condition.transpile()), AnsiColor::Cyan)
        )
    }
}


// Display Functions
impl fmt::Display for GetNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(GetNode){}{}{}{}",
            get_tab(self._depth),
            self._columns,
            self._table,
            self._filter
                .as_ref()
                .map(|v| v as &dyn fmt::Display)
                .unwrap_or(&format!("
{}(FilterNode)
{}N/A",
                get_tab(self._depth + 1),
                get_tab(self._depth + 2))),
            self._postprocessor
                .as_ref()
                .map(|v| v as &dyn fmt::Display)
                .unwrap_or(&format!("
{}(PostProcessorNode)
{}N/A",
                get_tab(self._depth + 1),
                get_tab(self._depth + 2)))
        )
    }
}

impl fmt::Display for PostProcessorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(PostProcessorNode)
{}N/A",
            get_tab(self._depth),
            get_tab(self._depth + 1),
          )
    }
}

impl fmt::Display for FilterNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(FilterNode){}",
            get_tab(self._depth),
            self.condition
        )
    }
}

impl fmt::Display for TableNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(TableNode)
{}table_name: {:?}",
            get_tab(self._depth),
            get_tab(self._depth + 1),
            self.table_name,
        )
    }
}

impl fmt::Display for ColumnNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(ColumnNode)
{}is_wildcard: {:?}
{}column_names: {:?}",
            get_tab(self._depth),
            get_tab(self._depth + 1),
            self.is_wildcard,
            get_tab(self._depth + 1),
            self.column_names,
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

            _literal: "all".to_string(),
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

            _literal: "id".to_string(),
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

            _literal: "id , cost and time".to_string(),
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
    fn test_table_error() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::From,
                &"".to_string(),
                &"from".to_string(),
            ),
        ];

        let mut idx: usize = 0;
        let depth: u16 = 0;
        
        match TableNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(val) => assert!(false, "Output was expected to error but returned -> {}", val),
                Err(err) => assert!(true, "Output errored out -> {}", err)
        }
    }

    #[test]
    fn test_table_normal() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::From,
                &"".to_string(),
                &"from".to_string(),
            ),
            Token::new(
                TokenType::Identifier,
                &"table_name".to_string(),
                &"table_name".to_string(),
            ),
        ];

        let expected: TableNode = TableNode {
            table_name: "table_name".to_string(),
            
            _literal: "from table_name".to_string(),
            _depth: 0
        };
        let mut idx: usize = 0;
        let depth: u16 = 0;
        
        match TableNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(val) => assert_eq!(expected, val),
                Err(err) => assert!(false, "Output errored out -> {}", err)
            }
    }
}
/*
This handles our database mutators and accessors
(creation and deletion of databases)

Make the database db_name.
*/


use std::{fmt, usize};
use crate::{
    language::{
        parser::{
            helpers::{
                get_tab, validate_length, peek_one
            }, 
            parser::ImpliedAction,
        }, tokens::{
            Token, TokenType
        }
    },
    utils::{
        colors::{
            AnsiColor,
            colorize, 
        },
        logger
    }
};

#[derive(Debug)]
pub struct DatabaseNode { 
    pub _create: Option<CreateNode>,
    pub _destroy: Option<DestroyNode>,
    pub _use: Option<UseNode>,
    pub _show: Option<ShowNode>,

    _literal: String,
    _depth: u16
}

#[derive(Debug, PartialEq)]
pub struct ShowNode {
    _depth: u16
}    

#[derive(Debug, PartialEq)]
pub struct DestroyNode {
    databases: Vec<String>,

    _literal: String,
    _depth: u16
}

#[derive(Debug, PartialEq)]
pub struct CreateNode {
    name: String,

    _literal: String,
    _depth: u16
}

#[derive(Debug, PartialEq)]
pub struct UseNode {
    name: String,

    _literal: String,
    _depth: u16
}

impl DatabaseNode {
   pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16,
        action: ImpliedAction,
    ) -> Result<DatabaseNode, String>{
        validate_length(
            tokens ,
            idx,
            true)?;

        
        let mut database_node: DatabaseNode = DatabaseNode {
            _create: None,
            _destroy: None,
            _use: None,
            _show: None,

            _literal: {
                tokens[*idx - 2..*idx].iter()
                    .map(|v| v.lexeme.as_str())
                    .collect::<Vec<&str>>()
                    .join(" ")
                },
            _depth: depth
        };

        match action {
            ImpliedAction::Create => {
                database_node._create = match CreateNode::parse(
                    tokens,
                    idx,
                    depth + 1
                ) {
                    Ok(state) => Some(state),
                    Err(msg) => return Err(msg)
                }},
            ImpliedAction::Delete => {
                database_node._destroy = match DestroyNode::parse(
                    tokens,
                    idx,
                    depth + 1
                ) {
                    Ok(state) => Some(state),
                    Err(msg) => return Err(msg)
                }},
            ImpliedAction::Show => {
                database_node._show = match ShowNode::parse(
                    tokens,
                    idx,
                    depth + 1
                ) {
                    Ok(state) => Some(state),
                    Err(msg) => return Err(msg)
                }},
            ImpliedAction::Use => {
                database_node._use = match UseNode::parse(
                    tokens,
                    idx,
                    depth + 1
                ) {
                    Ok(state) => Some(state),
                    Err(msg) => return Err(msg)
                }},
            _ => return Err(format!(
                "Got unexpected action requested for database -> '{:?}'", action)
            )
        };    

        if tokens[*idx].token_type != TokenType::EoqToken {
            return Err(format!(
                "Unexpected token '{}', expected end-of-query token by this point.",
                tokens[*idx].lexeme))
        }

        return Ok(database_node);
    }

    /// Outputs current AST node transpiled with color         
    /// and it's raw query counterpart. Output are used by
    /// the Transpiler REPL.
    pub fn transpile_color(
        &self,
    ) -> (String, String) {
        let pair: (String, (String, String)) = match (
            &self._create,
            &self._destroy,
            &self._show,
            &self._use,
        ) {
            (Some(op), _, _, _) => ("CREATE DATABASE ".to_string(), op.transpile_color()),
            (_, Some(op), _, _) => ("DROP DATABASE ".to_string(), op.transpile_color()),
            (_, _, Some(op), _) => ("SHOW DATABASE".to_string(), op.transpile_color()),
            (_, _, _, Some(op)) => ("USE DATABASE ".to_string(), op.transpile_color()),
            _ => logger::error("No database operation provided"),
        };
        
        (
            colorize(&self._literal, AnsiColor::Yellow) + 
                if *&pair.1.0.len() != 0 { &" " } else { "" } + 
                &pair.1.0,
            format!(
                "{}{}",
                colorize(&pair.0, AnsiColor::Yellow),
                &pair.1.1
            )
        )
    }

    /// Outputs current AST node transpile to raw SQL.
    pub fn transpile_raw(
        &self
    ) -> String {
        match (
            &self._create,
            &self._destroy,
            &self._show,
            &self._use,
        ) {
            (Some(op), _, _, _) => "CREATE DATABASE ".to_string() + &op.transpile_raw(),
            (_, Some(op), _, _) => "DROP DATABASE ".to_string() + &op.transpile_raw(),
            (_, _, Some(op), _) => "SHOW DATABASE".to_string() + &op.transpile_raw(),
            (_, _, _, Some(op)) => "USE DATABASE ".to_string() + &op.transpile_raw(),
            _ => logger::error("No database operation provided"),
        }
    }
}

impl CreateNode {
   pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16,
    ) -> Result<CreateNode, String> {
        validate_length(tokens, idx, true)?;
        
        if tokens[*idx].token_type != TokenType::Identifier {
            return Err(format!("Expected identifier, got '{:?}' instead!", tokens[*idx].token_type));
        }

        *idx += 1;
        validate_length(tokens, idx, true)?;
        
        return Ok(CreateNode {
            name: tokens[*idx - 1].literal.clone(),

            _literal: tokens[*idx - 1].lexeme.clone(),
            _depth: depth
        })
    }

    /// Outputs current AST node transpiled with color         
    /// and it's raw query counterpart. Output are used by
    /// the Transpiler REPL.
    pub fn transpile_color(
        &self 
    ) -> (String, String) {
        (
            colorize(&self._literal.as_str(), AnsiColor::Blue),
            colorize(&self.name.as_str(), AnsiColor::Blue),
        )
    }

    /// Outputs current AST node transpiled to raw SQL.
    pub fn transpile_raw(
        &self
    ) -> String {
        self.name.clone()
    }
}

impl DestroyNode {
    pub fn recurse_build(
        tokens: &Vec<Token>,
        dbs: &mut Vec<String>,
        idx: &mut usize,
    ) -> Result<(), String> {
        let next_token: TokenType = peek_one(tokens, &idx);
        if 
            next_token != TokenType::And && 
            next_token != TokenType::Comma
        {
            if tokens[*idx].token_type == TokenType::Identifier {
                dbs.push(tokens[*idx].literal.clone());
                *idx += 1;
                return Ok(());
            }
        } else if 
            next_token == TokenType::And || 
            next_token == TokenType::Comma 
        {
            if tokens[*idx].token_type == TokenType::Identifier {
                dbs.push(tokens[*idx].literal.clone());
                *idx += 2;
                
                DestroyNode::recurse_build(
                    tokens,
                    dbs,
                    idx
                )?;

                return Ok(());
            }
        }
        
        return Err("Something went wrong parsing database names, \
make sure they're in a valid list notation.".to_string()
        );
    }

    /// Takes current node type and given the current location in the
    /// query defined by the borrowed index, makes an attempt to parse
    /// this node and associated subnodes for the Abstract Syntax Tree.
    pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16,
    ) -> Result<DestroyNode, String> {
        validate_length(tokens, idx, true)?;

        let start_idx: usize = *idx; 
        let mut db_names: Vec<String> = vec![];

        DestroyNode::recurse_build(tokens, &mut db_names, idx)?;
        validate_length(tokens, idx, true)?;
        
        return Ok(DestroyNode {
            databases: db_names,
            
            _literal: {
                tokens[start_idx..*idx].iter()
                    .map(|v| v.lexeme.as_str())
                    .collect::<Vec<&str>>()
                    .join(" ")
            },
            _depth: depth
        })
    }

    /// Outputs current AST node transpiled with color         
    /// and it's raw query counterpart. Output are used by
    /// the Transpiler REPL.
    pub fn transpile_color(
        &self 
    ) -> (String, String) {
        (
            colorize(&self._literal.as_str(), AnsiColor::Blue),
            colorize( &self.databases.join(", "), AnsiColor::Blue),
        )
    }

    /// Outputs current AST node transpile to raw SQL.
    pub fn transpile_raw(
        &self
    ) -> String {
        self.databases.join(", ")
    }
}

impl UseNode {
   pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16,
    ) -> Result<UseNode, String> {
        validate_length(tokens, idx, true)?;
        
        if tokens[*idx].token_type != TokenType::Identifier {
            return Err(format!("Expected identifier, got '{:?}' instead!", tokens[*idx].token_type));
        }

        *idx += 1;
        validate_length(tokens, idx, true)?;
        
        return Ok(UseNode {
            name: tokens[*idx - 1].literal.clone(),

            _literal: tokens[*idx - 1].lexeme.clone(),
            _depth: depth
        })
    }

    /// Outputs current AST node transpiled with color         
    /// and it's raw query counterpart. Output are used by
    /// the Transpiler REPL.
    pub fn transpile_color(
        &self 
    ) -> (String, String) {
        (
            colorize(&self._literal.as_str(), AnsiColor::Blue),
            colorize(&self.name.as_str(), AnsiColor::Blue),
        )
    }

    /// Outputs current AST node transpiled to raw SQL.
    pub fn transpile_raw(
        &self
    ) -> String {
        self.name.clone()
    }
}

impl ShowNode {
   pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16,
    ) -> Result<ShowNode, String> {
        validate_length(tokens, idx, true)?;
        
        return Ok(ShowNode {
            _depth: depth
        })
    }

    /// Outputs current AST node transpiled with color         
    /// and it's raw query counterpart. Output are used by
    /// the Transpiler REPL.
    pub fn transpile_color(
        &self 
    ) -> (String, String) {
        (
            "".to_string(),
            "".to_string(),
        )
    }
    
    /// Outputs current AST node transpiled to raw SQL.
    pub fn transpile_raw(
        &self
    ) -> String {
        "".to_string()
    }
}

// Display Functions
impl fmt::Display for DatabaseNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(DatabaseNode){}{}{}{}",
            get_tab(self._depth),
            self._create
                .as_ref()
                .map(|v| v as &dyn fmt::Display)
                .unwrap_or(&""),
            self._destroy
                .as_ref()
                .map(|v| v as &dyn fmt::Display)
                .unwrap_or(&""),
            self._show
                .as_ref()
                .map(|v| v as &dyn fmt::Display)
                .unwrap_or(&""),
            self._use
                .as_ref()
                .map(|v| v as &dyn fmt::Display)
                .unwrap_or(&""),
            
        )
    }
}

impl fmt::Display for CreateNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(CreateNode)
{}name: {}",
            get_tab(self._depth),
            get_tab(self._depth + 1),
            self.name
        )
    }
}

impl fmt::Display for DestroyNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(DestroyNode)
{}databases: {:?}",
            get_tab(self._depth),
            get_tab(self._depth + 1),
            self.databases
        )
    }
}

impl fmt::Display for UseNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(UseNode)
{}name: {}",
            get_tab(self._depth),
            get_tab(self._depth + 1),
            self.name
        )
    }
}

impl fmt::Display for ShowNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(ShowNode)",
            get_tab(self._depth),
        )
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test_show_normal() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::EoqToken,
                &"".to_string(),
                &"!".to_string(),
            ),
        ];

        let mut idx: usize = 0;
        let depth: u16 = 0;
        let expected: ShowNode = ShowNode {
            _depth: depth
        };

        match ShowNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(val) => assert_eq!(val, expected),
                Err(err) => assert!(false, "Output errored out -> {}", err)
        }
    }
  
    #[test]
    fn unit_test_use_normal() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::Identifier,
                &"test_db".to_string(),
                &"test_db".to_string(),
            ),
            Token::new(
                TokenType::EoqToken,
                &"".to_string(),
                &"!".to_string(),
            ),
        ];

        let mut idx: usize = 0;
        let depth: u16 = 0;
        let expected: UseNode = UseNode {
            name: "test_db".to_string(),
            _literal: "test_db".to_string(),
            _depth: depth
        };

        match UseNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(val) => assert_eq!(val, expected),
                Err(err) => assert!(true, "Output errored out -> {}", err)
        }
    }
 
    #[test]
    fn unit_test_use_error() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::Get,
                &"get".to_string(),
                &"get".to_string(),
            ),
            Token::new(
                TokenType::EoqToken,
                &"".to_string(),
                &"!".to_string(),
            ),
        ];

        let mut idx: usize = 0;
        let depth: u16 = 0;

        match UseNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(val) => assert!(false, "Output was expected to error but returned -> {}", val),
                Err(_) => assert!(true, "Output expected to error and did.")
            }
    }
    
    #[test]
    fn unit_test_create_normal() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::Identifier,
                &"test_db".to_string(),
                &"test_db".to_string(),
            ),
            Token::new(
                TokenType::EoqToken,
                &"".to_string(),
                &"!".to_string(),
            ),
        ];

        let mut idx: usize = 0;
        let depth: u16 = 0;
        let expected: CreateNode = CreateNode {
            name: "test_db".to_string(),
            _literal: "test_db".to_string(),
            _depth: depth
        };

        match CreateNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(val) => assert_eq!(val, expected),
                Err(err) => assert!(false, "Output errored out -> {}", err)
        }
    }
 
    #[test]
    fn unit_test_create_error() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::Get,
                &"get".to_string(),
                &"get".to_string(),
            ),
            Token::new(
                TokenType::EoqToken,
                &"".to_string(),
                &"!".to_string(),
            ),
        ];

        let mut idx: usize = 0;
        let depth: u16 = 0;

        match CreateNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(val) => assert!(false, "Output was expected to error but returned -> {}", val),
                Err(_) => assert!(true, "Output expected to error and did.")
            }
    }

    #[test]
    fn unit_test_destroy_normal_single() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::Identifier,
                &"test_db".to_string(),
                &"test_db".to_string(),
            ),
            Token::new(
                TokenType::EoqToken,
                &"".to_string(),
                &"!".to_string(),
            ),
        ];

        let mut idx: usize = 0;
        let depth: u16 = 0;
        let expected: DestroyNode = DestroyNode {
            databases: vec!["test_db".to_string()],
            _literal: "test_db".to_string(),
            _depth: depth
        };

        match DestroyNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(val) => assert_eq!(val, expected),
                Err(err) => assert!(false, "Output errored out -> {}", err)
        }
    }
 
    #[test]
    fn unit_test_destroy_normal_multiple() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::Identifier,
                &"test_db_1".to_string(),
                &"test_db_1".to_string(),
            ),
            Token::new(
                TokenType::Comma,
                &",".to_string(),
                &",".to_string(),
            ),
            Token::new(
                TokenType::Identifier,
                &"test_db_2".to_string(),
                &"test_db_2".to_string(),
            ),
            Token::new(
                TokenType::Comma,
                &",".to_string(),
                &",".to_string(),
            ),
            Token::new(
                TokenType::Identifier,
                &"test_db_3".to_string(),
                &"test_db_3".to_string(),
            ),
            Token::new(
                TokenType::EoqToken,
                &"".to_string(),
                &"!".to_string(),
            ),
        ];

        let mut idx: usize = 0;
        let depth: u16 = 0;
        let expected: DestroyNode = DestroyNode {
            databases: vec![
                "test_db_1".to_string(),
                "test_db_2".to_string(),
                "test_db_3".to_string(),
                ],
            _literal: "test_db_1 , test_db_2 , test_db_3".to_string(),
            _depth: depth
        };

        match DestroyNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(val) => assert_eq!(val, expected),
                Err(err) => assert!(false, "Output errored out -> {}", err)
        }
    }

    #[test]
    fn unit_test_destroy_error_multiple() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::Identifier,
                &"test_db_1".to_string(),
                &"test_db_1".to_string(),
            ),
            Token::new(
                TokenType::Comma,
                &",".to_string(),
                &",".to_string(),
            ),
            Token::new(
                TokenType::Comma,
                &",".to_string(),
                &",".to_string(),
            ),
            Token::new(
                TokenType::Identifier,
                &"test_db_2".to_string(),
                &"test_db_2".to_string(),
            ),
        ];

        let mut idx: usize = 0;
        let depth: u16 = 0;

        match DestroyNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(val) => assert!(false, "Output was expected to error but returned -> {}", val),
                Err(_) => assert!(true, "Output expected to error and did.")
            }
    }
    #[test]
    fn unit_test_destroy_error() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::Get,
                &"get".to_string(),
                &"get".to_string(),
            ),
            Token::new(
                TokenType::EoqToken,
                &"".to_string(),
                &"!".to_string(),
            ),
        ];

        let mut idx: usize = 0;
        let depth: u16 = 0;

        match DestroyNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(val) => assert!(false, "Output was expected to error but returned -> {}", val),
                Err(_) => assert!(true, "Output expected to error and did.")
            }
    }
}
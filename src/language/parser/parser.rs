use std::{fmt};
use crate::{
    utils::{
        logger
    },
    language::{
        parser::{
            helpers::{
                validate_length, get_tab
            },
            get::GetNode
        },
        tokens::{
            Token, TokenType
        },
        parser::database::{
            DatabaseNode
        }
    }
};

#[derive(Debug, PartialEq)]
pub enum ImpliedAction {
    Use,
    Delete,
    Create,
    Show,
    _Rename
}

#[derive(Debug)]
pub struct Query {
    _get: Option<GetNode>,
    _database: Option<DatabaseNode>,
    _depth: u16
}

impl ImpliedAction {
    pub fn try_from(value: TokenType) -> Result<ImpliedAction, String> {
        match value {
            TokenType::CreateKeyword => Ok(ImpliedAction::Create),
            TokenType::DeleteKeyword => Ok(ImpliedAction::Delete),
            TokenType::ShowKeyword => Ok(ImpliedAction::Show),
            TokenType::UseKeyword => Ok(ImpliedAction::Use),
            _ => Err(format!("Invalid action token type encountered -> got {:?}", value))
        }
    }
}

impl Query {
    /// Takes current node type and given the current location in the
    /// query defined by the borrowed index, makes an attempt to parse
    /// this node and associated subnodes for the Abstract Syntax Tree.
    pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16
    ) -> Result<Query, String> {
        validate_length(
            &tokens,
            &idx,
            true)?;
        
        if tokens[*idx].token_type == TokenType::Get {
            *idx += 1;

            let get_node: GetNode = GetNode::parse(
                &tokens, idx, depth + 1
            )?;
            return Ok(Query {
                _get: Some(get_node),
                _database: None,
                _depth: depth
            });
        } else if vec![
            TokenType::CreateKeyword,
            TokenType::DeleteKeyword,
            TokenType::UseKeyword,
            TokenType::ShowKeyword
        ].contains(&tokens[*idx].token_type) {
            validate_length(tokens, &(*idx + 1), true)?;
            *idx += 1;

            // Peek ahead to see if it's a database or table
            if tokens[*idx].token_type == TokenType::Database {
                *idx += 1;

                let database_node: DatabaseNode = DatabaseNode::parse(
                    &tokens, 
                    idx, 
                    depth + 1,
                    ImpliedAction::try_from(tokens[*idx - 2].token_type)?
                )?;

                return Ok(Query {
                    _get: None,
                    _database: Some(database_node),
                    _depth: depth 
                });
            } else {
                return Err(
                    format!(
                        "Query recieved an action keyword, but received an invalid target keyword `{:?}`. Valid targets are: `Database`",
                        tokens[*idx].token_type
                    )
                )
            }
        }

        return Err("Couldn't determine requested action. Please review documentation for valid query actions.".to_string());
    }

    /// Outputs current AST node transpiled with color         
    /// and it's raw query counterpart. Output are used by
    /// the Transpiler REPL.
    pub fn transpile_color(
        &self
    ) -> (String, String) {
        if let Some(get) = &self._get {
            return get.transpile_color();
        } else if let Some(database) = &self._database {
            return database.transpile_color();
        } else {
            logger::error("A fatal error occurred while transpiling your query!");
        };
    }

    /// Ouputs current AST node tranpile to raw SQL.
    pub fn transpile_raw(
        &self
    ) -> String {
        if let Some(get) = &self._get {
            return get.transpile_raw();
        } else if let Some(database) = &self._database {
            return database.transpile_raw();
        } else {
            logger::error("A fatal error occurred while transpiling your query!");
        };
    }        
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"{}(Query){}{}",
            get_tab(self._depth),
            self._get
                .as_ref()
                .map(|v| v as &dyn fmt::Display)
                .unwrap_or(&""),
            self._database
                .as_ref()
                .map(|v| v as &dyn fmt::Display)
                .unwrap_or(&""))
    }
}

/// Main parsing interface. Takes a vector of tokens
/// produced by the Lexer class, and attempts to parse
/// it into an Abstract Syntax Tree.
pub fn parse(tokens: &Vec<Token>) -> Result<Query, String> {
    let mut idx: usize = 0;

    Query::parse(tokens, &mut idx, 0)
}

/* Template for Nodes 
impl TemplateNode {
    pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16) -> Result<TemplateNode, String> {
        
        return Err("ERROR PLACEHOLDER".to_string());
    }
}
*/
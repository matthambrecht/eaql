use std::{fmt};
use crate::eaql::parser::helpers::{validate_length, get_tab}; 
use crate::eaql::tokens::{Token, TokenType};
use crate::eaql::parser::get::{
    GetNode
};

#[derive(Debug)]
pub struct Query {
    _get: Option<GetNode>,
    _depth: u16
}

impl Query {
    pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16) -> Result<Query, String> {
        validate_length(
            &tokens,
            &idx,
            "Begining of Query",
            true)?;
        
        if tokens[*idx].token_type == TokenType::Get {
            *idx += 1;

            let get_node: GetNode = GetNode::parse(
                &tokens, idx, "Get Statement Start", depth + 1
            )?;
            return Ok(Query {
                _get: Some(get_node), 
                _depth: depth
            });
        }

        return Err("Query failed all requirements! Please review documentation.".to_string());
    }
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"{}(Query){}",
            get_tab(self._depth),
            self._get
                .as_ref()
                .map(|v| v as &dyn fmt::Display)
                .unwrap_or(&""))
    }
}

pub fn parse(tokens: &Vec<Token>) -> Result<Query, String> {
    let mut idx: usize = 0;

    Query::parse(tokens, &mut idx, 0)
}
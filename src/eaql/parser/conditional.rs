use std::fmt;
use crate::eaql::{
    parser::helpers::{get_tab, valid_until_warning, validate_length}, 
    tokens::{Token, TokenType}};

#[derive(Debug, PartialEq)]
pub struct ConditionNode {
    _or_condition: Option<OrConditionNode>,

    _depth: u16
}

#[derive(Debug, PartialEq)]
pub struct OrConditionNode {

}

#[derive(Debug, PartialEq)]
pub struct AndConditionNode {}

#[derive(Debug, PartialEq)]
pub struct NotConditionNode {
    _primary_condtion: PrimaryCondtionNode
}

#[derive(Debug, PartialEq)]
pub struct PrimaryCondtionNode {
    _expression: ExpressionNode
}

#[derive(Debug, PartialEq)]
pub struct ExpressionNode {
    _identifier: Token,
    _comparison_operator: Token,
    _literal: Token,

    _depth: u16
}

impl ConditionNode {
    pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16) -> Result<ConditionNode, String> {
        
        return Err("ERROR PLACEHOLDER".to_string());
    }
}

impl ExpressionNode {
    pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16) -> Result<ExpressionNode, String> {
        validate_length(tokens, &(*idx + 2), true)?;

        let identifier: Token;
        let comparison_operator: Token;
        let literal: Token;

        if tokens[*idx].token_type == TokenType::Identifier {
            identifier = tokens[*idx].clone();
            *idx += 1;
        } else {
            return Err(valid_until_warning(tokens, idx));
        }
        
        if vec![
            TokenType::Equal,
            TokenType::Lte,
            TokenType::Lte,
            TokenType::Gt,
            TokenType::Gte
        ].contains(&tokens[*idx].token_type) {
            comparison_operator = tokens[*idx].clone();
            *idx += 1;
        } else {
            return Err(valid_until_warning(tokens, idx));
        }
        
        if tokens[*idx].token_type == TokenType::StringLiteral
        || tokens[*idx].token_type == TokenType::NumberLiteral {
            literal = tokens[*idx].clone();
            *idx += 1;
        } else {
            return Err(valid_until_warning(tokens, idx));
        }
    
        return Ok(ExpressionNode {
            _identifier: identifier,
            _comparison_operator: comparison_operator,
            _literal: literal,

            _depth: depth
            }
        )
    }
}

// Display functions
impl fmt::Display for ExpressionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(ExpressionNode)
{}variable: {:#?}
{}operator: {:#?}
{}value: {:#?}
",
            get_tab(self._depth),
            get_tab(self._depth + 1),
            self._identifier.lexeme,
            get_tab(self._depth + 1),
            self._comparison_operator.token_type,
            get_tab(self._depth + 1),
            self._literal.literal
        )
    }
}

// Begin Conditional Tests 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expression_parsing_normal() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::Identifier,
                &"".to_string(),
                &"id".to_string(),
            ),
            Token::new(
                TokenType::Equal,
                &"".to_string(),
                &"is".to_string(),
            ),
            Token::new(
                TokenType::NumberLiteral,
                &"5".to_string(),
                &"5".to_string(),
            )
        ];

        let mut idx: usize = 0;
        let depth: u16 = 0;
        let expected: ExpressionNode = ExpressionNode {
            _identifier: Token::new(
                TokenType::Identifier,
                &"".to_string(),
                &"id".to_string(),
            ),
            _comparison_operator: Token::new(
                TokenType::Equal,
                &"".to_string(),
                &"is".to_string(),
            ),
            _literal: Token::new(
                TokenType::NumberLiteral,
                &"5".to_string(),
                &"5".to_string(),
            ),

            _depth: 0
        };

        match ExpressionNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(val) => {
                    assert_eq!(expected, val)
                },
                Err(err) => assert!(false, "Output errored out -> {}", err)
        }
    }

    #[test]
    fn test_expression_parsing_error() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::Identifier,
                &"".to_string(),
                &"id".to_string(),
            ),
            Token::new(
                TokenType::Equal,
                &"".to_string(),
                &"is".to_string(),
            ),
            Token::new(
                TokenType::Get,
                &"".to_string(),
                &"get".to_string(),
            )
        ];

        let mut idx: usize = 0;
        let depth: u16 = 0;

        match ExpressionNode::parse(
            &input,
            &mut idx,
            depth) {
                Ok(_val) => assert!(false, "Output expected to error!"),
                Err(err) => assert!(true, "Output errored out -> {}", err)
        }
    }
}
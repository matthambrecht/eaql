use std::{fmt};
use crate::eaql::{
    parser::helpers::{get_tab, valid_until_warning, validate_length}, 
    tokens::{Token, TokenType}};

#[derive(Debug, PartialEq)]
pub struct ConditionNode {
    _condition: ConditionChild,

    _depth: u16
}

#[derive(Debug, PartialEq)]
pub enum ConditionChild {
    Op(Box<OperandNode>),
    Expr(Box<ExpressionNode>),
    Bool(Box<BoolNode>)
}

#[derive(Debug, PartialEq)]
pub struct OperandNode {
    _type: String,

    _ls: ConditionChild,
    _rs: ConditionChild,

    _depth: u16
}

#[derive(Debug, PartialEq)]
pub struct ExpressionNode {
    _identifier: Token,
    _comparison_operator: Token,
    _literal: Token,

    _depth: u16
}

#[derive(Debug, PartialEq)]
pub struct BoolNode {
    _value: bool,

    _depth: u16
}

impl ConditionNode {
    pub fn recurse_build(
        parent_op: String,
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16
    ) -> Result<ConditionChild, String> {
        Err("None".to_string())
    }

    pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16
    ) -> Result<ConditionNode, String> {
        let root: OperandNode = OperandNode {
            _type: "OR".to_string(),
            
            _ls: match ConditionNode::recurse_build(
                "OR".to_string(),
                tokens,
                idx,
                depth + 1
            ) {
                Ok(node) => node,
                Err(msg) => return Err(msg)
            },
            _rs: match ConditionNode::recurse_build(
                "OR".to_string(),
                tokens,
                idx,
                depth + 1
            ) {
                Ok(node) => node,
                Err(msg) => return Err(msg)
            },

            _depth: depth + 1,
        };

        return Ok(
            ConditionNode {
                _condition: ConditionChild::Bool(Box::new(
                    BoolNode { _value: false, _depth: depth + 1 })),
                _depth: depth
            }
        )
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
            TokenType::Lt,
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
impl fmt::Display for ConditionChild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConditionChild::Op(node) => write!(f, "{node}"),
            ConditionChild::Expr(node) => write!(f, "{node}"),
            ConditionChild::Bool(node) => write!(f, "{node}"),
        }
    }
}

impl fmt::Display for ConditionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(Condition){}{}",
            get_tab(self._depth),
            get_tab(self._depth + 1),
            self._condition
        )
    }
}

impl fmt::Display for OperandNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(Operand){}{}",
            get_tab(self._depth),
            self._ls,
            self._rs,
        )
    }
}

impl fmt::Display for BoolNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(Bool::{})",
            get_tab(self._depth),
            self._value,
        )
    }
}



impl fmt::Display for ExpressionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(Expression)
{}variable: {:?}
{}operator: {:?}
{}value: {:?}",
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
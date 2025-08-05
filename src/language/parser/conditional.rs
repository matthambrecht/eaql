use std::fmt;
use crate::{
    language::{
        parser::helpers::{
            get_tab, valid_until_warning, validate_length
        }, 
        tokens::{
            Token, TokenType
        }
    },
    utils::logger
};

#[derive(Debug, PartialEq)]
pub struct ConditionNode {
    _condition: ConditionChild,
    _literal: String,

    _depth: u16,
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

fn update_depths(
    node: &mut ConditionChild
) -> () {
    match node {
        ConditionChild::Op(state) => {
            state._depth += 1;
            update_depths(&mut state._ls);
            update_depths(&mut state._rs);
            return;
        },
        ConditionChild::Bool(state) => state._depth += 1,
        ConditionChild::Expr(state) => state._depth += 1,
    }
}

fn handle_open_paren(
    tokens: &Vec<Token>,
    idx: &mut usize,
    depth: u16,
    finished: &mut bool,
    closing_paren: &mut bool,
    closing_or: &mut bool,
) -> Result<ConditionChild, String> {
    let mut ret: ConditionChild = ConditionChild::Op(Box::new(OperandNode {
            _type: "OR".to_string(),
            _depth: depth,
            _ls: match recurse_down(
                        tokens, 
                        idx, 
                        depth + 1, 
                        "OR".to_string(), 
                        finished, 
                        closing_paren, 
                        closing_or) {
                Ok(node) => node,
                Err(msg) => return Err(msg)
            },
            _rs: match recurse_down(
                        tokens, 
                        idx, 
                        depth + 1, 
                        "OR".to_string(), 
                        finished, 
                        closing_paren, 
                        closing_or) {
                Ok(node) => node,
                Err(msg) => return Err(msg)
            }
    }));

    match tokens[*idx].token_type {
        TokenType::And => {
            *idx += 1;
            *closing_paren = false;

            // This really only matters if we're debugging
            // just makes the depths line up.
            if logger::LOG_LEVEL <= logger::DEBUG.0 {
                update_depths(&mut ret);
            }

            return Ok(ConditionChild::Op(Box::new(
                OperandNode {
                    _type: "AND".to_string(),
                    _ls: ret,
                    _rs: match recurse_down(
                        tokens, 
                        idx, 
                        depth + 1, 
                        "AND".to_string(), 
                        finished, 
                        closing_paren, 
                        closing_or) {
                            Ok(node) => node,
                            Err(msg) => return Err(msg)
                    },

                    _depth: depth
                }
            )));
        },
        TokenType::Or => {
            *idx += 1;
            *closing_paren = false;

            // This really only matters if we're debugging
            // just makes the depths line up.
            if logger::LOG_LEVEL <= logger::DEBUG.0 {
                update_depths(&mut ret);
            }
         
            return Ok(ConditionChild::Op(Box::new(
                OperandNode {
                    _type: "OR".to_string(),
                    _ls: ret,
                    _rs: match recurse_down(
                        tokens, 
                        idx, 
                        depth + 1, 
                        "OR".to_string(), 
                        finished, 
                        closing_paren, 
                        closing_or) {
                            Ok(node) => node,
                            Err(msg) => return Err(msg)
                    },

                    _depth: depth
                }
            )));
        },
        TokenType::CloseParen => {
            *idx += 1;
            
            return Ok(ret);
        },
        TokenType::PostProcessorEntrance => {
            *closing_paren = false;
            *finished = true;
            
            return Ok(ret);
        },
        TokenType::EoqToken => {
            *closing_paren = false;
            *finished = true;

            return Ok(ret);
        },
        _ => return Err(format!("Something went wrong parsing a nested conditional: Expected a closing parentheses, \
        post-processor entrance, end-of-query, 'and' or 'or', \
        but got '{}' instead.", tokens[*idx].lexeme))
    }
}

fn handle_and(
    tokens: &Vec<Token>,
    idx: &mut usize,
    depth: u16,
    finished: &mut bool,
    closing_paren: &mut bool,
    closing_or: &mut bool,
) -> Result<ConditionChild, String> {
    Ok(ConditionChild::Op(Box::new(OperandNode {
            _type: "AND".to_string(),
            _depth: depth,
            _ls: match recurse_down(
                        tokens, 
                        idx, 
                        depth + 1, 
                        "AND".to_string(), 
                        finished, 
                        closing_paren, 
                        closing_or) {
                Ok(node) => node,
                Err(msg) => return Err(msg)
            },
            _rs: match recurse_down(
                        tokens, 
                        idx, 
                        depth + 1, 
                        "AND".to_string(), 
                        finished, 
                        closing_paren, 
                        closing_or) {
                Ok(node) => node,
                Err(msg) => return Err(msg)
            }
    })))
}

fn handle_literal(
    tokens: &Vec<Token>,
    idx: &mut usize,
    depth: u16,
    finished: &mut bool,
    closing_paren: &mut bool,
    closing_or: &mut bool,
) -> Result<ConditionChild, String> {
    let ls: ConditionChild = ConditionChild::Expr(
        Box::new(match ExpressionNode::parse(tokens, idx, depth + 1) {
            Ok(node) => node,
            Err(msg) => return Err(msg)
        })
    );
    let rs: ConditionChild = match recurse_down(
                        tokens, 
                        idx, 
                        depth + 1, 
                        "AND".to_string(), 
                        finished, 
                        closing_paren, 
                        closing_or) {
        Ok(node) => node,
        Err(msg) => return Err(msg)
    };

    return Ok(
        ConditionChild::Op(Box::new(OperandNode {
            _type: "AND".to_string(),
            _depth: depth,
            _ls: ls,
            _rs: rs
        }))
    );
}

fn handle_close(
    parent_node: &String,
    depth: u16
) -> ConditionChild {
    // AND default to true, OR defaults to false
    if *parent_node == "AND" {
        ConditionChild::Bool(Box::new(
            BoolNode { _value: true, _depth: depth }
        ))
    } else {
        ConditionChild::Bool(Box::new(
            BoolNode { _value: false, _depth: depth }
        ))
    }
}

fn handle_or(
    closing_or: &mut bool,
    parent_node: &String,
    depth: u16,
) -> ConditionChild {
    *closing_or = true;
    handle_close(&parent_node, depth)
}

fn handle_close_paren(
    closing_paren: &mut bool,
    parent_node: &String,
    depth: u16,
) -> ConditionChild {
    *closing_paren = true;
    handle_close(&parent_node, depth)
}

fn parse_child(
    tokens: &Vec<Token>,
    idx: &mut usize,
    depth: u16,
    parent_node: String,
    finished: &mut bool,
    closing_paren: &mut bool,
    closing_or: &mut bool,
) -> Result<ConditionChild, String> {
    match tokens[*idx].token_type {
        TokenType::And => {
            *idx += 1;
            return handle_and(tokens, idx, depth, finished, closing_paren, closing_or);
        },
        TokenType::Or => {
            *idx += 1;
            return Ok(handle_or(closing_or, &parent_node, depth))
        },
        TokenType::OpenParen => {
            *idx += 1;
            return handle_open_paren(tokens, idx, depth, finished, closing_paren, closing_or);
        },
        TokenType::CloseParen => {
            *idx += 1;
            return Ok(handle_close_paren(closing_paren, &parent_node, depth))
        },
        TokenType::PostProcessorEntrance => {
            if *closing_paren {
                return Err("Found end of conditional, but there are unclosed parentheses!".to_string())
            }

            *finished = true;

            return Ok(handle_close(&parent_node, depth))
        },
        TokenType::EoqToken => {
            if *closing_paren {
                return Err("Found end of conditional, but there are unclosed parentheses!".to_string())
            }

            *finished = true;

            return Ok(handle_close(&parent_node, depth))
        },
        TokenType::Identifier => {
            return handle_literal(tokens, idx, depth, finished, closing_paren, closing_or);
        },
        _ => {
            return Err(format!(
                "Unexpected token found while parsing conditional expression -> {}",
                tokens[*idx].lexeme
            ))
        }  
    }
}

fn recurse_down(
    tokens: &Vec<Token>,
    idx: &mut usize,
    depth: u16,
    parent_node: String,
    finished: &mut bool,
    closing_paren: &mut bool,
    closing_or: &mut bool,
) -> Result<ConditionChild, String> {
    // We aren't finished yet so we might need to parse
    // properly
    if *finished {
        return Ok(handle_close(&parent_node, depth));
    }

    // If we have seen a closing or we need to close out
    // unless our parent node is an OR, then we need to add
    // and OR node and begin parsing from there
    if *closing_paren {
        return Ok(handle_close(&parent_node, depth));
    } else if *closing_or {
        if parent_node == "OR" {
            *closing_or = false;

            return Ok(
                ConditionChild::Op(Box::new(OperandNode {
                    _type: "OR".to_string(),
                    _depth: depth,
                    _ls: match recurse_down(
                        tokens, 
                        idx, 
                        depth + 1, 
                        "OR".to_string(), 
                        finished, 
                        closing_paren, 
                        closing_or) {
                            Ok(node) => node,
                            Err(msg) => return Err(msg)
                    },
                    _rs: match recurse_down(
                        tokens, 
                        idx, 
                        depth + 1, 
                        "OR".to_string(), 
                        finished, 
                        closing_paren, 
                        closing_or) {
                            Ok(node) => node,
                            Err(msg) => return Err(msg)
                    }
                }))
            );
        }

        return Ok(handle_close(&parent_node, depth));
    }

    return parse_child(tokens, idx, depth, parent_node, finished, closing_paren, closing_or)
}

impl ConditionNode {
    /// Takes current node type and given the current location in the
    /// query defined by the borrowed index, makes an attempt to parse
    /// this node and associated subnodes for the Abstract Syntax Tree.
    pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16
    ) -> Result<ConditionNode, String> {
        let mut finished: bool = false;
        let mut closing_paren: bool = false;
        let mut closing_or: bool = false;
        let start_idx: usize = *idx;

        let ret: ConditionChild = ConditionChild::Op(Box::new(OperandNode {
            _type: "OR".to_string(),
            _depth: depth + 1,
            _ls: match recurse_down(
                        tokens, 
                        idx, 
                        depth + 2, 
                        "OR".to_string(), 
                        &mut finished, 
                        &mut closing_paren,
                        &mut closing_or) {
                Ok(node) => node,
                Err(msg) => return Err(msg)
            },
            _rs: match recurse_down(
                        tokens, 
                        idx, 
                        depth + 2, 
                        "OR".to_string(), 
                        &mut finished, 
                        &mut closing_paren,
                        &mut closing_or) {
                Ok(node) => node,
                Err(msg) => return Err(msg)
            }
        }));
        
        return Ok(
            ConditionNode {
                _condition: ret,
                _depth: depth,
                _literal: {
                    tokens[start_idx..*idx].iter()
                        .map(|v| if v.token_type == TokenType::Equal {
                            "="
                        } else {
                            v.lexeme.as_str()
                        })
                        .collect::<Vec<&str>>()
                        .join(" ")
                }
            }
        )
    }

    /// Outputs current AST node transpiled with color         
    /// and it's raw query counterpart. Output are used by
    /// the Transpiler REPL.
    pub fn transpile_color(
        &self
    ) -> String {
       format!("{}", self._literal) 
    }

    /// Outputs current AST node transpiled to raw SQL
    pub fn transpile_raw(
        &self
    ) -> String {
       format!("{}", self._literal) 
    }
}

impl ExpressionNode {
    /// Takes current node type and given the current location in the
    /// query defined by the borrowed index, makes an attempt to parse
    /// this node and associated subnodes for the Abstract Syntax Tree.
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
"\n{}(Operand::{}){}{}",
            get_tab(self._depth),
            self._type,
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
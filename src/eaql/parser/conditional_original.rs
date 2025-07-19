use std::{fmt};
use crate::eaql::{
    parser::helpers::{get_tab, valid_until_warning, validate_length}, 
    tokens::{Token, TokenType}};

#[derive(Debug, PartialEq)]
pub enum OrChild {
    Or(Box<OrConditionNode>),
    And(Box<AndConditionNode>),
}

#[derive(Debug, PartialEq)]
pub enum AndChild {
    Or(Box<OrConditionNode>),
    Expression(Box<ExpressionNode>),
}

#[derive(Debug, PartialEq)]
pub struct ConditionNode {
    _or_condition: Option<OrConditionNode>,

    _depth: u16
}

#[derive(Debug, PartialEq)]
pub struct OrConditionNode {
    _ls: OrChild,
    _rs: Option<OrChild>,

    _depth: u16
}

#[derive(Debug, PartialEq)]
pub struct AndConditionNode {
    _ls: AndChild,
    _rs: Option<AndChild>,

    _depth: u16
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
        depth: u16
    ) -> Result<ConditionNode, String> {
        let mut finished: bool = false;
        let mut num_paren: i16 = 0;

        let processed: OrConditionNode  = match OrConditionNode::parse(
            tokens,
            idx,
            depth + 1,
            &mut finished,
            &mut num_paren) {
                Ok(child) => {
                    child
                },
                Err(err) => {
                    return Err(err);
                }
            };

        return Ok(
            ConditionNode {
                _or_condition: Some(processed),
                _depth: depth
            }
        )
    }
}

impl OrConditionNode {
    pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16,
        finished: &mut bool,
        num_paren: &mut i16,
    ) -> Result<OrConditionNode, String> {
        validate_length(tokens, &(*idx + 1), true)?;
        
        /* If the first character is a parentheses we start a new OR node, otherwise we start 
        AND node */
        let ls = if tokens[*idx].token_type == TokenType::OpenParen {
            *idx += 1;
            match OrConditionNode::parse(tokens, idx, depth + 1, finished, num_paren) {
                Ok(node) => OrChild::Or(Box::new(node)),
                Err(err) => return Err(err),
            }
        } else {
            match AndConditionNode::parse(tokens, idx, depth + 1, finished, num_paren) {
                Ok(node) => OrChild::And(Box::new(node)),
                Err(err) => return Err(err),
            }
        };

        /* After we've branched back up we check if we need to go back down because
        of an open parentheses. */
        let rs = if !*finished {
            // We have an open parentheses so it becomes an OrNode
            if tokens[*idx].token_type == TokenType::OpenParen {
                *idx += 1;
                *num_paren += 1;

                match OrConditionNode::parse(tokens, idx, depth + 1, finished, num_paren) {
                    Ok(node) => Some(
                        OrChild::Or(Box::new(node))),
                    Err(err) => return Err(err),
                }
            // We have an "and" so we need to move forward and evaluate the right side as an AndNode
            } else if tokens[*idx].token_type == TokenType::And {
                *idx += 1;

                match AndConditionNode::parse(tokens, idx, depth + 1, finished, num_paren) {
                
                    Ok(node) => Some(
                        OrChild::And(Box::new(node))),
                    Err(err) => return Err(err),
                }
            // We have an "or" so we need to move forward and evaluate the right side as an OrNode
            } else if tokens[*idx].token_type == TokenType::Or { 
                *idx += 1;

                match OrConditionNode::parse(tokens, idx, depth + 1, finished, num_paren) {
                    Ok(node) => Some(
                        OrChild::Or(Box::new(node))),
                    Err(err) => return Err(err),
                }
            // We have a closed parentheses, close out our current or block
            } else if tokens[*idx].token_type == TokenType::CloseParen {
                *num_paren -= 1;
                *idx += 1;
                None
            // We have an ending token, finish up
            } else if vec![
                    TokenType::EoqToken,
                    TokenType::PostProcessorEntrance].contains(&tokens[*idx].token_type.clone()) {
                *finished = true;
                *idx += 1;
                None
            // We just have a regular expression from this point, move down to AndNode 
            } else { 
                match AndConditionNode::parse(tokens, idx, depth + 1, finished, num_paren) {
                    Ok(node) => Some(
                        OrChild::And(Box::new(node))),
                    Err(err) => return Err(err),
                }
            }
        } else {
            None
        };

        Ok(OrConditionNode {
            _ls: ls,
            _rs: rs,
            _depth: depth,
        })
    }
}
            
impl AndConditionNode {
    pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16,
        finished: &mut bool,
        num_paren: &mut i16
    ) -> Result<AndConditionNode, String> {
        validate_length(tokens, &(*idx + 1), true)?;
        
        // If the first character is a parentheses we start a new OR
        let ls: AndChild = if tokens[*idx].token_type == TokenType::OpenParen {
            *idx += 1;
            match OrConditionNode::parse(tokens, idx, depth + 1, finished, num_paren) {
                Ok(node) => AndChild::Or(Box::new(node)),
                Err(err) => return Err(err),
            }
        /* Otherwise we have to evaluate from here and let the expression handle possible errors
        All base expressions will be stored in the LS of an And node */
        } else {
            match ExpressionNode::parse(tokens, idx, depth + 1) {
                Ok(node) => AndChild::Expression(Box::new(node)),
                Err(err) => return Err(err),
            }
        };

        let rs: Option<AndChild> = if !*finished {
            if tokens[*idx].token_type == TokenType::OpenParen {
                *idx += 1;
                *num_paren += 1;

                match OrConditionNode::parse(tokens, idx, depth + 1, finished, num_paren) {
                    Ok(node) => Some(AndChild::Or(Box::new(node))),
                    Err(err) => return Err(err),
                }
            } else {
                // If the next KW is 'and', RS just becomes whatever is beyond that
                if tokens[*idx].token_type == TokenType::And {
                    *idx += 1;

                    match OrConditionNode::parse(tokens, idx, depth + 1, finished, num_paren) {
                        Ok(node) => Some(AndChild::Or(Box::new(node))),
                        Err(err) => return Err(err),
                    }
                // Next keyword is 'or' means we move up the eval tree and use the RS of our parent OR
                } else if tokens[*idx].token_type == TokenType::Or {
                    *idx += 1;
                    None
                } else if vec![
                    TokenType::EoqToken,
                    TokenType::PostProcessorEntrance].contains(&tokens[*idx].token_type.clone()
                // Next keyword is closed parentheses, we need to move up a branch
                ) {
                    *finished = true;
                    None
                } else if tokens[*idx].token_type == TokenType::CloseParen { 
                    *idx += 1;
                    *num_paren -= 1;
                    None
                // We need a valid keyword at this point otherwise something is wrong
                } else {
                    return Err(valid_until_warning(tokens, idx));
                }
            }
        } else {
            None
        };

        Ok(AndConditionNode {
            _ls: ls,
            _rs: rs,
            _depth: depth,
        })
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
impl fmt::Display for AndChild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AndChild::Or(node) => write!(f, "{node}"),
            AndChild::Expression(node) => write!(f, "{node}"),
        }
    }
}

impl fmt::Display for OrChild {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrChild::And(node) => write!(f, "{node}"),
            OrChild::Or(node) => write!(f, "{node}"),
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
            self._or_condition
                .as_ref()
                .map(|v| v as &dyn fmt::Display)
                .unwrap_or(&format!(
"{}N/A",
            get_tab(self._depth + 1)
            )),
        )
    }
}

impl fmt::Display for AndConditionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(AndNode){}{}",
            get_tab(self._depth),
            self._ls,
            self._rs
                .as_ref()
                .map(|v| v as &dyn fmt::Display)
                .unwrap_or(&format!("
{}(True)",
            get_tab(self._depth + 1))),
        )
    }
}

impl fmt::Display for OrConditionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(OrNode){}{}",
            get_tab(self._depth),
            self._ls,
            self._rs
                .as_ref()
                .map(|v| v as &dyn fmt::Display)
                .unwrap_or(&format!("
{}(False)",
            get_tab(self._depth + 1))),
        )
    }
}

impl fmt::Display for ExpressionNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
"\n{}(ExpressionNode)
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
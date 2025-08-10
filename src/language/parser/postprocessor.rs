use crate::{
    language::{
        parser::helpers::{get_tab, peek_one, validate_length},
        tokens::{Token, TokenType},
    },
    utils::colors::{AnsiColor, colorize},
};
use std::{fmt, usize};

#[derive(PartialEq, Debug)]
pub struct LimitNode {
    limit: i32,

    _literal: String,
    _depth: u16,
}

#[derive(PartialEq, Debug)]
pub struct PostProcessorNode {
    pub limit: Option<LimitNode>,

    _depth: u16,
}

impl LimitNode {
    /// Takes current node type and given the current location in the
    /// query defined by the borrowed index, makes an attempt to parse
    /// this node and associated subnodes for the Abstract Syntax Tree.
    pub fn parse(tokens: &Vec<Token>, idx: &mut usize, depth: u16) -> Result<LimitNode, String> {
        validate_length(tokens, idx, true)?;

        let start_idx: usize = *idx - 1;

        if tokens[*idx].token_type != TokenType::NumberLiteral {
            return Err(format!(
                "Limit post-processor expects a number literal, got -> {:?}",
                tokens[*idx].token_type
            ));
        }

        return Ok(LimitNode {
            limit: match tokens[*idx].literal.parse::<i32>() {
                Ok(state) => state,
                Err(_) => {
                    return Err(format!(
                        "Limit post-processor expects 32-bit integer, got -> {}",
                        tokens[*idx].lexeme
                    ));
                }
            },

            _literal: {
                tokens[start_idx..*idx + 1]
                    .iter()
                    .map(|v| v.lexeme.as_str())
                    .collect::<Vec<&str>>()
                    .join(" ")
            },
            _depth: depth,
        });
    }

    /// Outputs current AST node transpiled with color         
    /// and it's raw query counterpart. Output are used by
    /// the Transpiler REPL.
    pub fn transpile_color(&self) -> (String, String) {
        (
            colorize(&self._literal, AnsiColor::Magenta),
            colorize(&format!("LIMIT {}", self.limit), AnsiColor::Magenta),
        )
    }

    /// Outputs current AST node transpiled to raw SQL.
    pub fn transpile_raw(&self) -> String {
        format!("LIMIT {}", self.limit)
    }
}

impl PostProcessorNode {
    pub fn handle_postprocessor(
        tokens: &Vec<Token>,
        final_node: &mut PostProcessorNode,
        depth: u16,
        idx: &mut usize,
    ) -> Result<(), String> {
        validate_length(tokens, idx, true)?;

        match tokens[*idx].token_type {
            TokenType::And => {
                *idx += 1;
            }
            TokenType::LimitKeyword => {
                *idx += 1;

                final_node.limit = match LimitNode::parse(tokens, idx, depth + 1) {
                    Ok(state) => Some(state),
                    Err(msg) => return Err(msg),
                };
            }
            _ => {
                return Err(format!(
                    "Unexpected token, expected post-processor entrance keyword or list continuation, got -> \"{}\"",
                    tokens[*idx].lexeme
                ));
            }
        };

        Ok(())
    }

    pub fn recurse_build(
        tokens: &Vec<Token>,
        final_node: &mut PostProcessorNode,
        depth: u16,
        idx: &mut usize,
    ) -> Result<(), String> {
        let next_token: TokenType = peek_one(tokens, &idx);

        // End of query, required after postprocessor nodes
        if next_token == TokenType::EoqToken {
            *idx += 1;
            return Ok(());
        } else {
            PostProcessorNode::handle_postprocessor(tokens, final_node, depth, idx)?;

            PostProcessorNode::recurse_build(tokens, final_node, depth, idx)?;

            return Ok(());
        }
    }

    /// Takes current node type and given the current location in the
    /// query defined by the borrowed index, makes an attempt to parse
    /// this node and associated subnodes for the Abstract Syntax Tree.
    pub fn parse(
        tokens: &Vec<Token>,
        idx: &mut usize,
        depth: u16,
    ) -> Result<Option<PostProcessorNode>, String> {
        validate_length(tokens, idx, true)?;

        if tokens[*idx].token_type != TokenType::PostProcessorEntrance {
            return Ok(None);
        }

        *idx += 1;

        let mut final_node: PostProcessorNode = PostProcessorNode {
            limit: None,
            _depth: depth,
        };

        PostProcessorNode::recurse_build(tokens, &mut final_node, depth, idx)?;

        return Ok(Some(final_node));
    }

    /// Outputs current AST node transpiled with color         
    /// and it's raw query counterpart. Output are used by
    /// the Transpiler REPL.
    pub fn transpile_color(&self) -> (String, String) {
        let mut final_lexeme: Vec<String> = vec![];
        let mut final_transpiled: Vec<String> = vec![];

        if self.limit.is_some() {
            let limit_transpiled: (String, String) = self.limit.as_ref().unwrap().transpile_color();

            final_lexeme.push(limit_transpiled.0);
            final_transpiled.push(limit_transpiled.1);
        }

        (final_lexeme.join(" "), final_transpiled.join(" "))
    }

    /// outputs current ast node transpiled to raw sql.
    pub fn transpile_raw(&self) -> String {
        let mut final_transpiled: Vec<String> = vec![];

        if self.limit.is_some() {
            let limit_transpiled: String = self.limit.as_ref().unwrap().transpile_raw();

            final_transpiled.push(limit_transpiled);
        }

        final_transpiled.join(" ")
    }
}

impl fmt::Display for PostProcessorNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n{}(PostProcessorNode){}",
            get_tab(self._depth),
            self.limit
                .as_ref()
                .map(|v| v as &dyn fmt::Display)
                .unwrap_or(&"")
        )
    }
}

impl fmt::Display for LimitNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\n{}(LimitNode)
{}limit: {:?}",
            get_tab(self._depth),
            get_tab(self._depth + 1),
            self.limit,
        )
    }
}

// Begin PostProcessor Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unit_test_postprocessor_error() {
        let input: Vec<Token> = vec![Token::new(
            TokenType::PostProcessorEntrance,
            &"".to_string(),
            &"then".to_string(),
        )];

        let mut idx: usize = 0;
        let depth: u16 = 0;

        match PostProcessorNode::parse(&input, &mut idx, depth) {
            Ok(_) => assert!(false, "Output was expected to error!"),
            Err(err) => assert!(true, "Output errored out -> {}", err),
        }
    }

    #[test]
    fn unit_test_postprocessor_limit_error_float() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::PostProcessorEntrance,
                &"".to_string(),
                &"then".to_string(),
            ),
            Token::new(
                TokenType::LimitKeyword,
                &"".to_string(),
                &"limit".to_string(),
            ),
            Token::new(
                TokenType::NumberLiteral,
                &"5.5".to_string(),
                &"5.5".to_string(),
            ),
            Token::new(TokenType::EoqToken, &"".to_string(), &".".to_string()),
        ];

        let mut idx: usize = 0;
        let depth: u16 = 0;

        match PostProcessorNode::parse(&input, &mut idx, depth) {
            Ok(_) => assert!(false, "Output was expected to error!"),
            Err(err) => assert!(true, "Output errored out -> {}", err),
        }
    }

    #[test]
    fn unit_test_postprocessor_limit_normal() {
        let input: Vec<Token> = vec![
            Token::new(
                TokenType::PostProcessorEntrance,
                &"".to_string(),
                &"then".to_string(),
            ),
            Token::new(
                TokenType::LimitKeyword,
                &"".to_string(),
                &"limit".to_string(),
            ),
            Token::new(TokenType::NumberLiteral, &"5".to_string(), &"5".to_string()),
            Token::new(TokenType::EoqToken, &"".to_string(), &".".to_string()),
        ];

        let expected: PostProcessorNode = PostProcessorNode {
            limit: Some(LimitNode {
                limit: 5,

                _depth: 1,
                _literal: "limit 5".to_string(),
            }),
            _depth: 0,
        };
        let mut idx: usize = 0;
        let depth: u16 = 0;

        match PostProcessorNode::parse(&input, &mut idx, depth) {
            Ok(val) => match val {
                Some(node) => assert_eq!(node, expected),
                None => assert!(false, "Output returned nothing but something was expected!"),
            },
            Err(err) => assert!(false, "Output errored out -> {}", err),
        }
    }
}

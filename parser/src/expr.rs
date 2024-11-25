use crate::Parser;
use crate::{
    errors::{self, Error},
    tokens,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Number(f64),
    Boolean(bool),
    Identifier(String),
    String(String),
    /// An Accessor is a way to access a value in a data structure.
    /// For example, in the expression `a.b.c`, `a` is the root, `b` is the first accessor, and `c` is the second accessor.
    /// We use expressions to represent accessors because they can also be used with numbers and other expressions. (e.g. `10.floor`)
    Acessor(Vec<Expr>),
    /// A function call is an expression that calls a function with a list of arguments.
    /// It is just represented as an expression with a parenthesized list of arguments.
    /// For example, `add(1, 2)` is represented as `Call(Identifier("add"), [Number(1), Number(2)])`
    Call(Box<Expr>, Vec<Expr>),
    BinaryOp {
        op: tokens::Token,
        lhs: Box<Expr>,
        rhs: Box<Expr>,
    },
}

/// This impl block group the expression parsing functions together
/// By definition, an expression is a block of code that produces a value.
/// For example, "1+2" is an expression that produces the value 3.
/// But "let x = ..." is a statement, not an expression, because it doesn't produce a value.
impl Parser {
    // Utilities
    fn binary_op(&mut self, op: tokens::Token, lhs: Expr, rhs: Expr) -> Expr {
        Expr::BinaryOp {
            op,
            lhs: Box::new(lhs),
            rhs: Box::new(rhs),
        }
    }

    // Parsing functions
    pub fn parse_expr(&mut self) -> Result<Expr, Error> {
        let mut lhs = self.parse_add_sub()?;
        while let Some(op) = &self.current_token {
            match op {
                tokens::Token::Equal => {
                    self.next_token();
                    let rhs = self.parse_expr()?;
                    lhs = self.binary_op(tokens::Token::Equal, lhs, rhs);
                }
                tokens::Token::NotEqual => {
                    self.next_token();
                    let rhs = self.parse_expr()?;
                    lhs = self.binary_op(tokens::Token::NotEqual, lhs, rhs);
                }
                tokens::Token::LessThan => {
                    self.next_token();
                    let rhs = self.parse_expr()?;
                    lhs = self.binary_op(tokens::Token::LessThan, lhs, rhs);
                }
                tokens::Token::GreaterThan => {
                    self.next_token();
                    let rhs = self.parse_expr()?;
                    lhs = self.binary_op(tokens::Token::GreaterThan, lhs, rhs);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    pub fn parse_add_sub(&mut self) -> Result<Expr, Error> {
        let mut lhs = self.parse_term()?;
        while let Some(op) = &self.current_token {
            match op {
                tokens::Token::Plus => {
                    self.next_token();
                    let rhs = self.parse_term()?;
                    lhs = self.binary_op(tokens::Token::Plus, lhs, rhs);
                }
                tokens::Token::Minus => {
                    self.next_token();
                    let rhs = self.parse_term()?;
                    lhs = self.binary_op(tokens::Token::Minus, lhs, rhs);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    pub fn parse_term(&mut self) -> Result<Expr, Error> {
        let mut lhs = self.parse_call()?;
        while let Some(op) = &self.current_token {
            match op {
                tokens::Token::Star => {
                    self.next_token();
                    let rhs = self.parse_call()?;
                    lhs = self.binary_op(tokens::Token::Star, lhs, rhs);
                }
                tokens::Token::Divider => {
                    self.next_token();
                    let rhs = self.parse_call()?;
                    lhs = self.binary_op(tokens::Token::Divider, lhs, rhs);
                }
                tokens::Token::Modulo => {
                    self.next_token();
                    let rhs = self.parse_call()?;
                    lhs = self.binary_op(tokens::Token::Modulo, lhs, rhs);
                }
                _ => break,
            }
        }
        Ok(lhs)
    }

    pub fn parse_call(&mut self) -> Result<Expr, Error> {
        let mut lhs = self.parse_accessors()?;
        while let Some(tokens::Token::LParen) = &self.current_token {
            self.next_token();
            let mut args = Vec::new();
            while self.current_token != Some(tokens::Token::RParen) {
                args.push(self.parse_expr()?);
                if self.current_token == Some(tokens::Token::Comma) {
                    self.next_token();
                }
            }
            self.next_token();
            lhs = Expr::Call(Box::new(lhs), args);
        }
        Ok(lhs)
    }

    pub fn parse_accessors(&mut self) -> Result<Expr, Error> {
        let mut lhs = self.parse_factor()?;
        while let Some(tokens::Token::Point) = &self.current_token {
            self.next_token();

            let rhs = self.parse_factor()?;
            lhs = Expr::Acessor(vec![lhs, rhs]);
        }
        Ok(lhs)
    }

    pub fn parse_factor(&mut self) -> Result<Expr, Error> {
        // TODO: Remove the clone here

        match &self.current_token.clone() {
            Some(tokens::Token::Number(n)) => {
                self.next_token();
                let n = *n;
                Ok(Expr::Number(n))
            }
            Some(tokens::Token::String(s)) => {
                self.next_token();
                let s = s.clone();
                Ok(Expr::String(s))
            }
            Some(tokens::Token::Identifier(id)) => {
                self.next_token();
                let id = id.clone();
                match id.as_str() {
                    "true" => Ok(Expr::Boolean(true)),
                    "false" => Ok(Expr::Boolean(false)),
                    _ => Ok(Expr::Identifier(id)),
                }
            }
            Some(tokens::Token::LParen) => self.parse_paren(),
            None => Err(self.error(errors::ErrorType::UnexpectedEndOfFile)),
            _ => Err(self.error(errors::ErrorType::UnexpectedToken(
                self.current_token.clone().unwrap(),
            ))),
        }
    }

    pub fn parse_paren(&mut self) -> Result<Expr, Error> {
        let l_par_pos = self.pos;
        self.next_token();
        let expr = self.parse_expr()?;
        if self.current_token != Some(tokens::Token::RParen) {
            return Err(Error::new(
                errors::ErrorType::UnClosedParenthesis,
                l_par_pos,
                self.input.clone(),
            ));
        }
        self.next_token();
        Ok(expr)
    }
}

#[cfg(test)]
mod tests {
    use tokens::Token;

    use super::*;

    fn parse(input: &str) -> Result<Expr, Error> {
        Parser {
            input: input.to_string(),
            pos: 0,
            current_token: None,
            next_token: None,
        }
        .parse_expr()
    }

    #[test]
    fn test_parse_number() {
        assert_eq!(parse("42"), Ok(Expr::Number(42.0)));
    }

    #[test]
    fn test_parse_addition() {
        assert_eq!(
            parse("1 + 2"),
            Ok(Expr::BinaryOp {
                op: Token::Plus,
                lhs: Box::new(Expr::Number(1.0)),
                rhs: Box::new(Expr::Number(2.0)),
            })
        );
    }

    #[test]
    fn test_parse_multiplication() {
        assert_eq!(
            parse("4 * 2"),
            Ok(Expr::BinaryOp {
                op: Token::Star,
                lhs: Box::new(Expr::Number(4.0)),
                rhs: Box::new(Expr::Number(2.0)),
            })
        );
    }

    #[test]
    fn test_parse_parentheses() {
        assert_eq!(
            parse("(1 + 2) * 3"),
            Ok(Expr::BinaryOp {
                op: Token::Star,
                lhs: Box::new(Expr::BinaryOp {
                    op: Token::Plus,
                    lhs: Box::new(Expr::Number(1.0)),
                    rhs: Box::new(Expr::Number(2.0)),
                }),
                rhs: Box::new(Expr::Number(3.0)),
            })
        );
    }

    #[test]
    fn test_unexpected_token() {
        assert!(parse("1 +").is_err());
    }
    #[test]
    fn test_unclosed_parenthesis() {
        assert!(parse("(1 + 2").is_err());
    }

    #[test]
    fn test_unexpected_end_of_file() {
        assert!(parse("1 + 2 *").is_err());
    }

    #[test]
    fn test_invalid_token() {
        assert!(parse("1 + @").is_err());
    }

    #[test]
    fn test_nested_parentheses() {
        assert_eq!(
            parse("((1 + 2) * 3)"),
            Ok(Expr::BinaryOp {
                op: Token::Star,
                lhs: Box::new(Expr::BinaryOp {
                    op: Token::Plus,
                    lhs: Box::new(Expr::Number(1.0)),
                    rhs: Box::new(Expr::Number(2.0)),
                }),
                rhs: Box::new(Expr::Number(3.0)),
            })
        );
    }

    #[test]
    fn test_multiple_operations() {
        assert_eq!(
            parse("1 + 2 * 3 - 4 / 2"),
            Ok(Expr::BinaryOp {
                op: Token::Minus,
                lhs: Box::new(Expr::BinaryOp {
                    op: Token::Plus,
                    lhs: Box::new(Expr::Number(1.0)),
                    rhs: Box::new(Expr::BinaryOp {
                        op: Token::Star,
                        lhs: Box::new(Expr::Number(2.0)),
                        rhs: Box::new(Expr::Number(3.0)),
                    }),
                }),
                rhs: Box::new(Expr::BinaryOp {
                    op: Token::Divider,
                    lhs: Box::new(Expr::Number(4.0)),
                    rhs: Box::new(Expr::Number(2.0)),
                }),
            })
        );
    }

    #[test]
    fn test_whitespace_handling() {
        assert_eq!(
            parse(" 1 +  2 * 3"),
            Ok(Expr::BinaryOp {
                op: Token::Plus,
                lhs: Box::new(Expr::Number(1.0)),
                rhs: Box::new(Expr::BinaryOp {
                    op: Token::Star,
                    lhs: Box::new(Expr::Number(2.0)),
                    rhs: Box::new(Expr::Number(3.0)),
                }),
            })
        );
    }
}

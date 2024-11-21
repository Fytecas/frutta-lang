use crate::{
    errors::{self, Error},
    expr::Expr,
    tokens, Parser,
};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Let(Expr, Expr),
    Return(Expr),
    Expr(Expr),
    Block(Vec<Statement>),
    Fn{
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
    },
}

impl Parser {
    // Parsing functions
    pub fn parse_statement(&mut self) -> Result<Statement, Error> {
        match &self.current_token {
            Some(tokens::Token::Identifier(key)) => {
                let key = key.clone();
                self.parse_identifier(key)
            }
            Some(tokens::Token::LBrace) => self.parse_block().map(Statement::Block),
            _ => self.parse_expr().map(Statement::Expr),
        }
    }

    pub fn parse_identifier(&mut self, key: String) -> Result<Statement, Error> {
        match key.as_str() {
            "let" => self.parse_let(),
            "fn" => self.parse_fn(),
            "return" => self.parse_return(),
            _ => self.parse_expr().map(Statement::Expr),
        }
    }

    pub fn parse_let(&mut self) -> Result<Statement, Error> {
        self.next_token();
        let name = self.parse_expr()?;

        if !matches!(name, Expr::Identifier(_)) && !matches!(name, Expr::Acessor(_)){
            return Err(self.error(errors::ErrorType::ExpectedToken(tokens::Token::Identifier("".into()))));
        }

        if self.current_token != Some(tokens::Token::Assign) {
            return Err(self.error(errors::ErrorType::ExpectedToken(tokens::Token::Assign)));
        }

        self.next_token();

        let value = self.parse_expr()?;
        Ok(Statement::Let(name, value))
    }

    pub fn parse_fn(&mut self) -> Result<Statement, Error> {
        self.next_token();

        let name = match &self.current_token {
            Some(tokens::Token::Identifier(name)) => name.clone(),
            _ => {
                return Err(self.error(errors::ErrorType::ExpectedToken(
                    tokens::Token::Identifier("".into()),
                )))
            }
        };

        self.next_token();

        if self.current_token != Some(tokens::Token::LParen) {
            return Err(self.error(errors::ErrorType::ExpectedToken(tokens::Token::LParen)));
        }

        self.next_token();

        let mut params = Vec::new();
        let got_comma = true;
        while self.current_token != Some(tokens::Token::RParen) {
            match &self.current_token {
                Some(tokens::Token::Identifier(param)) => {
                    if got_comma {
                        params.push(param.clone());
                        self.next_token();
                    } else {
                        return Err(self.error(errors::ErrorType::ExpectedToken(tokens::Token::Comma)));
                    }
                }
                Some(tokens::Token::Comma) => {
                    self.next_token();
                    continue;
                }
                Some(_) => {
                    return Err(self.error(errors::ErrorType::ExpectedToken(tokens::Token::Identifier(
                        "".into(),
                    )))
                    )
                }
                None => {
                    return Err(self.error(errors::ErrorType::UnexpectedEndOfFile));
                }
            }
        }

        self.next_token();

        let body = self.parse_block()?;
        Ok(Statement::Fn { name, params, body })
    }

    pub fn parse_block(&mut self) -> Result<Vec<Statement>, Error> {
        // Check if the next token is a LBrace
        let l_brace_pos = self.pos;
        if let tokens::Token::LBrace = self.current_token.as_ref().unwrap() {
            self.next_token();
        } else {
            return Err(Error::new(
                errors::ErrorType::ExpectedToken(tokens::Token::LBrace),
                l_brace_pos,
                self.input.clone(),
            ));
        }
        let mut statements = Vec::new();
        while self.current_token != Some(tokens::Token::RBrace) {
            statements.push(self.parse_statement()?);
        }
        Ok(statements)
    }

    pub fn parse_return(&mut self) -> Result<Statement, Error> {
        self.next_token();
        let expr = self.parse_expr()?;
        Ok(Statement::Return(expr))
    }
}
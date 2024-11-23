use crate::{
    errors::{self, Error},
    expr::Expr,
    tokens,
    Parser,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Return(Expr),
    Expr(Expr),
    Block(Vec<Statement>),
    Fn{
        name: String,
        params: Vec<String>,
        body: Vec<Statement>
    },
    /// Variable assignment
    Assign(String, Expr),
    If{
        condition: Expr,
        body: Vec<Statement>,
        else_body: Vec<Statement>,
    }
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
            "fn" => self.parse_fn(),
            "return" => self.parse_return(),
            "if" => self.parse_if(),
            _ if matches!(self.next_token, Some(tokens::Token::Assign)) => {
                self.parse_assign(key)
            }
            _ => self.parse_expr().map(Statement::Expr),
        }
    }

    pub fn parse_if(&mut self) -> Result<Statement, Error> {
        self.next_token();
        let condition = self.parse_expr()?;
        let body = self.parse_block()?;
        let else_body = if self.current_token == Some(tokens::Token::Identifier("else".into())) {
            self.next_token();
            self.parse_block()?
        } else {
            Vec::new()
        };
        Ok(Statement::If {
            condition,
            body,
            else_body,
        })
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
        self.next_token();
        Ok(statements)
    }

    pub fn parse_return(&mut self) -> Result<Statement, Error> {
        self.next_token();
        let expr = self.parse_expr()?;
        Ok(Statement::Return(expr))
    }

    fn parse_assign(&mut self, name: String) -> Result<Statement, Error> {
        self.next_token();
        self.next_token();
        let value = self.parse_expr()?;
        Ok(Statement::Assign(name, value))
    }
}
use errors::Error;
use statement::Statement;

pub mod errors;
pub mod tokens;
pub mod expr;
pub mod statement;

/// The parser is responsible for transforming a sequence of tokens(your program) into an abstract syntax tree, which is a tree representation of the program.
/// In this, we use a algorithm called recursive descent parsing, which is a top-down parsing technique that constructs a parse tree from the top and the input is read from left to right.
pub struct Parser {
    pub input: String,
    pub pos: usize,
    pub current_token: Option<tokens::Token>,
}

impl Parser {
    /// Parse a program from a string
    /// A program is a sequence of statements, which are returned as a Block statement for convenience.
    pub fn parse(input: &str) -> Result<Statement, Error> {
        let mut parser = Parser {
            input: input.to_string(),
            pos: 0,
            current_token: None
        };
        parser.next_token();
        let mut statements = vec![];
        while parser.current_token.is_some() {
            statements.push(parser.parse_statement()?);
        }
        Ok(Statement::Block(statements))
    }

    /// Tokenize the next token in the input and store it in the current_token field
    /// This basically turn the next part of the input into a token and store it in the current_token field. <br/>
    /// For example, if the input is `let x = 1+1*(3-5)`, the first token would be `let`, the second would be `x`, etc...
    pub fn next_token(&mut self) {
        let result = tokens::Token::tokenize_first(&self.input[self.pos..]);
        if let Some((token, rest)) = result {
            self.pos += self.input[self.pos..].len() - rest.len();
            self.current_token = Some(token);
        } else {
            self.current_token = None;
        }
    }

    /// Shortcut to generate an error
    fn error(&self, error_type: errors::ErrorType) -> Error {
        Error::new(error_type, self.pos, self.input.clone())
    }
}
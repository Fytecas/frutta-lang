use std::fmt::Debug;

use colored::Colorize;

use crate::tokens::Token;

#[derive(Debug, PartialEq)]
pub enum ErrorType {
    UnexpectedToken(Token),
    UnexpectedEndOfFile,
    UnClosedParenthesis,
    ExpectedToken(Token),
}

impl ErrorType {
    fn message(&self) -> String {
        match self {
            ErrorType::UnexpectedToken(token) => format!("Unexpected token: {:?}", token),
            ErrorType::UnexpectedEndOfFile => "Unexpected end of file".to_string(),
            ErrorType::UnClosedParenthesis => "Unclosed parenthesis".to_string(),
            ErrorType::ExpectedToken(token) => format!("Expected token: {:?}", token),
        }
    }
}

#[derive(PartialEq)]
pub struct Error {
    error_type: ErrorType,
    pos: usize,
    input: String,
}

impl Error {
    pub fn new(error_type: ErrorType, pos: usize, input: String) -> Self {
        Self {
            error_type,
            pos,
            input,
        }
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let line_number = self.input[..self.pos].lines().count();
        let column_number = self.input[..self.pos]
            .lines()
            .last()
            .map_or(0, |line| line.len());

        let mut error_message = format!(
            "{} at {}:{}\n",
            self.error_type.message().red().bold(),
            line_number,
            column_number
        );

        let input_line = self
            .input
            .lines()
            .nth(line_number - 1)
            .unwrap_or(&self.input);
        let marker_line = "-".repeat(column_number - 1);

        error_message.push_str(&format!(
            "| {}\n| {}{}",
            input_line,
            marker_line.blue().bold(),
            "^".red().bold()
        ));

        write!(f, "{}", error_message)
    }
}

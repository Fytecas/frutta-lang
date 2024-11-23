/// Represents a token in the input string.
/// A token is a single unit of input that the parser can understand.
/// For example, the input "1+2" has three tokens: Number(1.0), Plus, Number(2.0).
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Divider,
    Assign,
    Comma,
    Point,
    SemiColon,
    GreaterThan,
    LessThan,
    NotEqual,
    Equal,
    Identifier(String),
}
// TODO: Add String, Boolean, and None

impl Token {
    /// Tokenize the first token in the input string.
    /// Returns the token and the remaining input string.
    /// for example if the input is "1+2", this function will return
    /// Some((Token::Number(1.0), "+2"))
    pub fn tokenize_first(input: &str) -> Option<(Token, &str)> {
        let mut chars = input.chars();
        while let Some(char) = chars.next() {
            return match char {
                '+' => (Token::Plus, chars.as_str()),
                ',' => (Token::Comma, chars.as_str()),
                '=' => {
                    if let Some('=') = chars.as_str().chars().next() {
                        chars.next();
                        (Token::Equal, chars.as_str())
                    } else {
                        (Token::Assign, chars.as_str())
                    }
                },
                '!' => {
                    if let Some('=') = chars.as_str().chars().next() {
                        chars.next();
                        (Token::NotEqual, chars.as_str())
                    } else {
                        panic!("Unexpected character: {}", char)
                    }
                },
                '.' => (Token::Point, chars.as_str()),
                '{' => (Token::LBrace, chars.as_str()),
                '}' => (Token::RBrace, chars.as_str()),
                '<' => (Token::LessThan, chars.as_str()),
                '>' => (Token::GreaterThan, chars.as_str()),
                ';' => (Token::SemiColon, chars.as_str()),
                '-' => (Token::Minus, chars.as_str()),
                '*' => (Token::Star, chars.as_str()),
                '(' => (Token::LParen, chars.as_str()),
                ')' => (Token::RParen, chars.as_str()),
                '/' => (Token::Divider, chars.as_str()),
                c if c.is_digit(10) => {
                    let mut num = String::new();
                    num.push(c);
                    while let Some(c) = chars.as_str().chars().next() {
                        // TODO: Add support for scientific notation
                        // TODO: Add support for hexadecimal numbers
                        // TODO IMPORTANT: Add support for acessors on numbers, like 10.floor
                        if c.is_digit(10) || c == '.' {
                            num.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    let num = num.parse().unwrap();
                    (Token::Number(num), chars.as_str())
                }
                c if c.is_alphabetic() => {
                    let mut id = String::new();
                    id.push(c);
                    while let Some(c) = chars.as_str().chars().next() {
                        if c.is_alphabetic() {
                            id.push(c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    (Token::Identifier(id), chars.as_str())
                }
                ' ' => continue,
                '\n' => continue,
                _ => panic!("Unexpected character: {}", char),
            }.into();
        }
        None
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize_first() {
        assert_eq!(
            Token::tokenize_first("1+2"),
            Some((Token::Number(1.0), "+2"))
        );
        assert_eq!(Token::tokenize_first("+2"), Some((Token::Plus, "2")));
        assert_eq!(Token::tokenize_first("2"), Some((Token::Number(2.0), "")));
        assert_eq!(Token::tokenize_first("(abc"), Some((Token::LParen, "abc")));
        assert_eq!(Token::tokenize_first(")"), Some((Token::RParen, "")));
        assert_eq!(
            Token::tokenize_first("abc"),
            Some((Token::Identifier("abc".to_string()), ""))
        );
        assert_eq!(Token::tokenize_first("0.1"), Some((Token::Number(0.1), "")));
        assert_eq!(Token::tokenize_first(""), None);

        assert_eq!(Token::tokenize_first(" 1+2"), Some((Token::Number(1.0), "+2")));
        assert_eq!(Token::tokenize_first(" +2"), Some((Token::Plus, "2")));
        assert_eq!(Token::tokenize_first(" 2"), Some((Token::Number(2.0), "")));
        assert_eq!(Token::tokenize_first(" (abc"), Some((Token::LParen, "abc")));
        assert_eq!(Token::tokenize_first(" )"), Some((Token::RParen, "")));
        assert_eq!(Token::tokenize_first(" abc"), Some((Token::Identifier("abc".to_string()), "")));
        assert_eq!(Token::tokenize_first(" 0.1"), Some((Token::Number(0.1), "")));
        assert_eq!(Token::tokenize_first(" "), None);
    }
}

use std::fmt::Display;

#[derive(Debug)]
pub enum LexerError {
    UnexpectedCharacter(char),
    UnsupportedLetter(char),
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LexerError::UnexpectedCharacter(c) => {
                write!(f, "unexpected character '{}'", c)
            }
            LexerError::UnsupportedLetter(c) => {
                write!(
                    f,
                    "unsupported letter '{}': this lexer only supports numbers and math operators",
                    c
                )
            }
        }
    }
}

#[derive(Debug)]
pub enum Token {
    Number(f64),

    Add,
    Subtract,
    Multiply,
    Divide,
    Exponential,
    Modulo,

    LeftParen,
    RightParen,
}

//TODO: Add the tokenization of numbers
pub fn tokenize(input: &str) -> Result<Vec<Token>, LexerError> {
    let mut tokens = Vec::new();

    let mut iterator = input.chars().peekable();

    while let Some(&c) = iterator.peek() {
        match c {
            c if c.is_whitespace() => {
                iterator.next();
            }
            c if c.is_alphabetic() => {
                return Err(LexerError::UnsupportedLetter(c));
            }

            '+' => {
                iterator.next();
                tokens.push(Token::Add);
            }
            '-' => {
                iterator.next();
                tokens.push(Token::Subtract);
            }
            '/' => {
                iterator.next();
                tokens.push(Token::Divide);
            }
            '*' => {
                iterator.next();
                tokens.push(Token::Multiply);
            }
            '^' => {
                iterator.next();
                tokens.push(Token::Exponential);
            }
            '(' => {
                iterator.next();
                tokens.push(Token::LeftParen);
            }
            ')' => {
                iterator.next();
                tokens.push(Token::RightParen);
            }
            '%' => {
                iterator.next();
                tokens.push(Token::Modulo);
            }

            _ => return Err(LexerError::UnexpectedCharacter(c)),
        }
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenize_addition() {
        let tokens = tokenize("+").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Add));
    }

    #[test]
    fn tokenize_subtraction() {
        let tokens = tokenize("-").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Subtract));
    }

    #[test]
    fn tokenize_multiplication() {
        let tokens = tokenize("*").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Multiply));
    }

    #[test]
    fn tokenize_division() {
        let tokens = tokenize("/").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Divide));
    }

    #[test]
    fn tokenize_exponential() {
        let tokens = tokenize("^").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Exponential));
    }

    #[test]
    fn tokenize_parentheses() {
        let tokens = tokenize("()").unwrap();

        assert_eq!(tokens.len(), 2);
        assert!(matches!(tokens[0], Token::LeftParen));
        assert!(matches!(tokens[1], Token::RightParen));
    }

    #[test]
    fn tokenize_modulo() {
        let tokens = tokenize("%").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Modulo));
    }

    #[test]
    fn tokenize_all_operators() {
        let tokens = tokenize("+ - * / ^ ( ) %").unwrap();

        assert_eq!(tokens.len(), 7);

        assert!(matches!(tokens[0], Token::Add));
        assert!(matches!(tokens[1], Token::Subtract));
        assert!(matches!(tokens[2], Token::Multiply));
        assert!(matches!(tokens[3], Token::Divide));
        assert!(matches!(tokens[4], Token::Exponential));
        assert!(matches!(tokens[5], Token::LeftParen));
        assert!(matches!(tokens[6], Token::RightParen));
        assert!(matches!(tokens[7], Token::Modulo));
    }

    #[test]
    fn ignore_spaces() {
        let tokens = tokenize("   +   ").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Add));
    }

    #[test]
    fn ignore_tabs() {
        let tokens = tokenize("\t+\t").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Add));
    }

    #[test]
    fn ignore_newlines() {
        let tokens = tokenize("\n+\n").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Add));
    }

    #[test]
    fn ignore_mixed_whitespace() {
        let tokens = tokenize(" \t\n + \n\t ").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Add));
    }

    #[test]
    fn tokenize_expression() {
        let tokens = tokenize("1 + 2").unwrap();

        // This will fail until numbers are implemented.
        assert_eq!(tokens.len(), 3);

        assert!(matches!(tokens[0], Token::Number(1.0)));
        assert!(matches!(tokens[1], Token::Add));
        assert!(matches!(tokens[2], Token::Number(2.0)));
    }

    #[test]
    fn tokenize_complex_expression() {
        let tokens = tokenize("(1 + 2) * 3").unwrap();

        assert_eq!(tokens.len(), 7);

        assert!(matches!(tokens[0], Token::LeftParen));
        assert!(matches!(tokens[1], Token::Number(1.0)));
        assert!(matches!(tokens[2], Token::Add));
        assert!(matches!(tokens[3], Token::Number(2.0)));
        assert!(matches!(tokens[4], Token::RightParen));
        assert!(matches!(tokens[5], Token::Multiply));
        assert!(matches!(tokens[6], Token::Number(3.0)));
    }

    #[test]
    fn tokenize_integer() {
        let tokens = tokenize("123").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Number(123.0)));
    }

    #[test]
    fn tokenize_zero() {
        let tokens = tokenize("0").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Number(0.0)));
    }

    #[test]
    fn tokenize_decimal() {
        let tokens = tokenize("123.45").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Number(123.45)));
    }

    #[test]
    fn tokenize_number_starting_with_decimal() {
        let tokens = tokenize(".5").unwrap();

        assert_eq!(tokens.len(), 1);
        assert!(matches!(tokens[0], Token::Number(0.5)));
    }

    #[test]
    fn tokenize_multiple_numbers() {
        let tokens = tokenize("123 456 789").unwrap();

        assert_eq!(tokens.len(), 3);

        assert!(matches!(tokens[0], Token::Number(123.0)));
        assert!(matches!(tokens[1], Token::Number(456.0)));
        assert!(matches!(tokens[2], Token::Number(789.0)));
    }

    #[test]
    fn tokenize_decimal_expression() {
        let tokens = tokenize("10.5 + 2.75").unwrap();

        assert_eq!(tokens.len(), 3);

        assert!(matches!(tokens[0], Token::Number(10.5)));
        assert!(matches!(tokens[1], Token::Add));
        assert!(matches!(tokens[2], Token::Number(2.75)));
    }

    #[test]
    fn reject_letters() {
        let result = tokenize("x");

        assert!(matches!(result, Err(LexerError::UnsupportedLetter('x'))));
    }

    #[test]
    fn reject_unexpected_character() {
        let result = tokenize("$");

        assert!(matches!(result, Err(LexerError::UnexpectedCharacter('$'))));
    }

    #[test]
    fn reject_letters_inside_expression() {
        let result = tokenize("2 + x");

        assert!(matches!(result, Err(LexerError::UnsupportedLetter('x'))));
    }

    #[test]
    fn reject_multiple_unsupported_characters() {
        let result = tokenize("2 + $");

        assert!(matches!(result, Err(LexerError::UnexpectedCharacter('$'))));
    }

    #[test]
    fn tokenize_empty_input() {
        let tokens = tokenize("").unwrap();

        assert!(tokens.is_empty());
    }

    #[test]
    fn tokenize_only_whitespace() {
        let tokens = tokenize("   \t\n  ").unwrap();

        assert!(tokens.is_empty());
    }

    #[test]
    fn tokenize_consecutive_operators() {
        let tokens = tokenize("+-*/^").unwrap();

        assert_eq!(tokens.len(), 5);

        assert!(matches!(tokens[0], Token::Add));
        assert!(matches!(tokens[1], Token::Subtract));
        assert!(matches!(tokens[2], Token::Multiply));
        assert!(matches!(tokens[3], Token::Divide));
        assert!(matches!(tokens[4], Token::Exponential));
    }
}

use super::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Modulo,
}

#[derive(Debug, Clone, PartialEq)]
enum Expr {
    Number(f64),
    Unary {
        operator: Operator,
        right: Box<Expr>,
    },
    Binary {
        left: Box<Expr>,
        operator: Operator,
        right: Box<Expr>,
    },
}

#[derive(Debug, Clone, PartialEq)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens: tokens,
            current: 0,
        }
    }

    fn peek(&self) -> &Token {
        // Sees what is the token without consuming it
        &self.tokens[self.current]
    }

    fn advance(&mut self) -> Token {
        // Consumes the token and advance
        let token = self.tokens[self.current].clone();
        self.current += 1;
        token
    }

    fn consume(&mut self, expected: Token) {
        // Checks wheter the token is the expcpted one and if so consume it
        // otherwise panic

        //NOTE: std::mem::discriminant gives a value representing which
        //enum variant something is. This is necessary since enum variants can take values
        //and if we did enum1 == enum2 this would also compare the values which is something we
        //don't wnat here
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(&expected) {
            self.advance();
        } else {
            //TODO: Improve this error message
            panic!("Unexpected token")
        }
    }

    fn parse(&mut self) -> Expr {
        return self.expression();
    }

    fn expression(&mut self) -> Expr {
        return self.additive();
    }

    fn additive(&mut self) -> Expr {
        let mut expr = self.multiplicative();

        loop {
            let operator = match &self.peek() {
                Token::Add => Operator::Add,
                Token::Subtract => Operator::Subtract,
                _ => break,
            };

            self.advance();

            let right = self.multiplicative();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn multiplicative(&mut self) -> Expr {
        let mut expr = self.unary();

        loop {
            let operator = match self.peek() {
                Token::Multiply => Operator::Multiply,
                Token::Divide => Operator::Divide,
                Token::Modulo => Operator::Modulo,
                _ => break,
            };
            self.advance();

            let right = self.unary();

            expr = Expr::Binary {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    fn unary(&mut self) -> Expr {
        if *self.peek() == Token::Add {
            self.advance();

            let right = self.unary();

            return Expr::Unary {
                operator: Operator::Add,
                right: Box::new(right),
            };
        }

        if *self.peek() == Token::Subtract {
            self.advance();

            let right = self.unary();

            return Expr::Unary {
                operator: Operator::Subtract,
                right: Box::new(right),
            };
        }

        return self.power();
    }

    fn power(&mut self) -> Expr {
        let left = self.primary();

        if *self.peek() == Token::Exponential {
            self.advance();

            let right = self.power();

            return Expr::Binary {
                left: Box::new(left),
                operator: Operator::Power,
                right: Box::new(right),
            };
        }

        left
    }

    fn primary(&mut self) -> Expr {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(&Token::Number(0.0)) {
            let token = self.advance();

            if let Token::Number(value) = token {
                return Expr::Number(value);
            }
        }

        if std::mem::discriminant(self.peek()) == std::mem::discriminant(&Token::LeftParen) {
            self.advance();

            let expression = self.expression();

            self.consume(Token::RightParen);

            return expression;
        }

        panic!("Expected Number or '('");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse(tokens: Vec<Token>) -> Expr {
        let mut parser = Parser::new(tokens);
        parser.parse()
    }

    #[test]
    fn parses_number() {
        let ast = parse(vec![Token::Number(5.0), Token::EOF]);

        assert_eq!(ast, Expr::Number(5.0));
    }

    #[test]
    fn parses_addition() {
        let ast = parse(vec![
            Token::Number(1.0),
            Token::Add,
            Token::Number(2.0),
            Token::EOF,
        ]);

        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Number(1.0)),
                operator: Operator::Add,
                right: Box::new(Expr::Number(2.0)),
            }
        );
    }

    #[test]
    fn parses_subtraction() {
        let ast = parse(vec![
            Token::Number(5.0),
            Token::Subtract,
            Token::Number(3.0),
            Token::EOF,
        ]);

        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Number(5.0)),
                operator: Operator::Subtract,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn parses_multiplication() {
        let ast = parse(vec![
            Token::Number(2.0),
            Token::Multiply,
            Token::Number(3.0),
            Token::EOF,
        ]);

        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Number(2.0)),
                operator: Operator::Multiply,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn parses_division() {
        let ast = parse(vec![
            Token::Number(6.0),
            Token::Divide,
            Token::Number(2.0),
            Token::EOF,
        ]);

        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Number(6.0)),
                operator: Operator::Divide,
                right: Box::new(Expr::Number(2.0)),
            }
        );
    }

    #[test]
    fn parses_modulo() {
        let ast = parse(vec![
            Token::Number(10.0),
            Token::Modulo,
            Token::Number(3.0),
            Token::EOF,
        ]);

        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Number(10.0)),
                operator: Operator::Modulo,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }

    #[test]
    fn parses_exponentiation() {
        let ast = parse(vec![
            Token::Number(2.0),
            Token::Exponential,
            Token::Number(3.0),
            Token::EOF,
        ]);

        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Number(2.0)),
                operator: Operator::Power,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }
    #[test]
    fn multiplication_has_higher_precedence_than_addition() {
        let ast = parse(vec![
            Token::Number(1.0),
            Token::Add,
            Token::Number(2.0),
            Token::Multiply,
            Token::Number(3.0),
            Token::EOF,
        ]);

        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Number(1.0)),
                operator: Operator::Add,
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Number(2.0)),
                    operator: Operator::Multiply,
                    right: Box::new(Expr::Number(3.0)),
                }),
            }
        );
    }
    #[test]
    fn parentheses_override_precedence() {
        let ast = parse(vec![
            Token::LeftParen,
            Token::Number(1.0),
            Token::Add,
            Token::Number(2.0),
            Token::RightParen,
            Token::Multiply,
            Token::Number(3.0),
            Token::EOF,
        ]);

        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Number(1.0)),
                    operator: Operator::Add,
                    right: Box::new(Expr::Number(2.0)),
                }),
                operator: Operator::Multiply,
                right: Box::new(Expr::Number(3.0)),
            }
        );
    }
    #[test]
    fn multiplication_is_left_associative() {
        let ast = parse(vec![
            Token::Number(2.0),
            Token::Multiply,
            Token::Number(3.0),
            Token::Multiply,
            Token::Number(4.0),
            Token::EOF,
        ]);

        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Binary {
                    left: Box::new(Expr::Number(2.0)),
                    operator: Operator::Multiply,
                    right: Box::new(Expr::Number(3.0)),
                }),
                operator: Operator::Multiply,
                right: Box::new(Expr::Number(4.0)),
            }
        );
    }
    #[test]
    fn exponentiation_is_right_associative() {
        let ast = parse(vec![
            Token::Number(2.0),
            Token::Exponential,
            Token::Number(3.0),
            Token::Exponential,
            Token::Number(4.0),
            Token::EOF,
        ]);

        assert_eq!(
            ast,
            Expr::Binary {
                left: Box::new(Expr::Number(2.0)),
                operator: Operator::Power,
                right: Box::new(Expr::Binary {
                    left: Box::new(Expr::Number(3.0)),
                    operator: Operator::Power,
                    right: Box::new(Expr::Number(4.0)),
                }),
            }
        );
    }
    #[test]
    fn parses_unary_minus() {
        let ast = parse(vec![Token::Subtract, Token::Number(5.0), Token::EOF]);

        assert_eq!(
            ast,
            Expr::Unary {
                operator: Operator::Subtract,
                right: Box::new(Expr::Number(5.0)),
            }
        );
    }
    #[test]
    fn parses_unary_plus() {
        let ast = parse(vec![Token::Add, Token::Number(5.0), Token::EOF]);

        assert_eq!(
            ast,
            Expr::Unary {
                operator: Operator::Add,
                right: Box::new(Expr::Number(5.0)),
            }
        );
    }
    #[test]
    fn unary_minus_and_exponentiation() {
        let ast = parse(vec![
            Token::Subtract,
            Token::Number(2.0),
            Token::Exponential,
            Token::Number(3.0),
            Token::EOF,
        ]);

        let expected = Expr::Unary {
            operator: Operator::Subtract,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Number(2.0)),
                operator: Operator::Power,
                right: Box::new(Expr::Number(3.0)),
            }),
        };

        assert_eq!(ast, expected);
    }
}

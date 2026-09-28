use super::parser::{Expr, Operator};

fn evaluate(expression: &Expr) -> f64 {
    match expression {
        Expr::Number(value) => *value,

        Expr::Unary { operator, right } => {
            let value = evaluate(right);
            match operator {
                Operator::Add => value,
                Operator::Subtract => -value,
                _ => panic!("Invalid unary operator"),
            }
        }

        Expr::Binary {
            left,
            operator,
            right,
        } => {
            let left = evaluate(left);
            let right = evaluate(right);

            match operator {
                Operator::Add => left + right,
                Operator::Subtract => left - right,
                Operator::Multiply => left * right,
                Operator::Divide => left / right,
                Operator::Power => left.powf(right),
                Operator::Modulo => left % right,
                _ => panic!("Invalid Operation"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn number(value: f64) -> Expr {
        Expr::Number(value)
    }

    fn binary(left: Expr, operator: Operator, right: Expr) -> Expr {
        Expr::Binary {
            left: Box::new(left),
            operator,
            right: Box::new(right),
        }
    }

    fn unary(operator: Operator, right: Expr) -> Expr {
        Expr::Unary {
            operator,
            right: Box::new(right),
        }
    }

    #[test]
    fn evaluates_number() {
        let expr = number(42.0);

        assert_eq!(evaluate(&expr), 42.0);
    }

    #[test]
    fn evaluates_addition() {
        let expr = binary(number(2.0), Operator::Add, number(3.0));

        assert_eq!(evaluate(&expr), 5.0);
    }

    #[test]
    fn evaluates_subtraction() {
        let expr = binary(number(10.0), Operator::Subtract, number(4.0));

        assert_eq!(evaluate(&expr), 6.0);
    }

    #[test]
    fn evaluates_multiplication() {
        let expr = binary(number(6.0), Operator::Multiply, number(7.0));

        assert_eq!(evaluate(&expr), 42.0);
    }

    #[test]
    fn evaluates_division() {
        let expr = binary(number(10.0), Operator::Divide, number(2.0));

        assert_eq!(evaluate(&expr), 5.0);
    }

    #[test]
    fn evaluates_modulo() {
        let expr = binary(number(10.0), Operator::Modulo, number(3.0));

        assert_eq!(evaluate(&expr), 1.0);
    }

    #[test]
    fn evaluates_power() {
        let expr = binary(number(2.0), Operator::Power, number(3.0));

        assert_eq!(evaluate(&expr), 8.0);
    }

    #[test]
    fn evaluates_nested_power() {
        let expr = Expr::Binary {
            left: Box::new(Expr::Number(2.0)),
            operator: Operator::Power,
            right: Box::new(Expr::Binary {
                left: Box::new(Expr::Number(3.0)),
                operator: Operator::Power,
                right: Box::new(Expr::Number(2.0)),
            }),
        };

        assert_eq!(evaluate(&expr), 512.0);
    }

    #[test]
    fn evaluates_unary_plus() {
        let expr = unary(Operator::Add, number(5.0));

        assert_eq!(evaluate(&expr), 5.0);
    }

    #[test]
    fn evaluates_unary_minus() {
        let expr = unary(Operator::Subtract, number(5.0));

        assert_eq!(evaluate(&expr), -5.0);
    }

    #[test]
    fn evaluates_double_unary_minus() {
        // --5
        let expr = unary(Operator::Subtract, unary(Operator::Subtract, number(5.0)));

        assert_eq!(evaluate(&expr), 5.0);
    }

    #[test]
    fn evaluates_mixed_unary_operators() {
        // -+5
        let expr = unary(Operator::Subtract, unary(Operator::Add, number(5.0)));

        assert_eq!(evaluate(&expr), -5.0);
    }

    #[test]
    fn evaluates_nested_addition_and_multiplication() {
        // 2 + (3 * 4)
        //
        //       +
        //      / \
        //     2   *
        //        / \
        //       3   4

        let expr = binary(
            number(2.0),
            Operator::Add,
            binary(number(3.0), Operator::Multiply, number(4.0)),
        );

        assert_eq!(evaluate(&expr), 14.0);
    }

    #[test]
    fn evaluates_parenthesized_shape() {
        // (2 + 3) * 4
        //
        //       *
        //      / \
        //     +   4
        //    / \
        //   2   3

        let expr = binary(
            binary(number(2.0), Operator::Add, number(3.0)),
            Operator::Multiply,
            number(4.0),
        );

        assert_eq!(evaluate(&expr), 20.0);
    }

    #[test]
    fn evaluates_left_associative_subtraction() {
        // (10 - 3) - 2

        let expr = binary(
            binary(number(10.0), Operator::Subtract, number(3.0)),
            Operator::Subtract,
            number(2.0),
        );

        assert_eq!(evaluate(&expr), 5.0);
    }

    #[test]
    fn evaluates_left_associative_division() {
        // (100 / 10) / 2

        let expr = binary(
            binary(number(100.0), Operator::Divide, number(10.0)),
            Operator::Divide,
            number(2.0),
        );

        assert_eq!(evaluate(&expr), 5.0);
    }

    #[test]
    fn evaluates_right_associative_power() {
        // 2 ^ (3 ^ 2)
        //
        // 2 ^ 9 = 512

        let expr = binary(
            number(2.0),
            Operator::Power,
            binary(number(3.0), Operator::Power, number(2.0)),
        );

        assert_eq!(evaluate(&expr), 512.0);
    }

    #[test]
    fn evaluates_complex_expression() {
        // (2 + 3) * 4 - 10 / 2
        //
        // 20 - 5 = 15

        let expr = binary(
            binary(
                binary(number(2.0), Operator::Add, number(3.0)),
                Operator::Multiply,
                number(4.0),
            ),
            Operator::Subtract,
            binary(number(10.0), Operator::Divide, number(2.0)),
        );

        assert_eq!(evaluate(&expr), 15.0);
    }

    #[test]
    fn evaluates_decimal_numbers() {
        let expr = binary(number(1.5), Operator::Add, number(2.5));

        assert_eq!(evaluate(&expr), 4.0);
    }

    #[test]
    fn evaluates_negative_result() {
        let expr = binary(number(3.0), Operator::Subtract, number(10.0));

        assert_eq!(evaluate(&expr), -7.0);
    }

    #[test]
    fn evaluates_zero() {
        let expr = number(0.0);

        assert_eq!(evaluate(&expr), 0.0);
    }

    #[test]
    fn evaluates_multiplication_by_zero() {
        let expr = binary(number(123.0), Operator::Multiply, number(0.0));

        assert_eq!(evaluate(&expr), 0.0);
    }

    #[test]
    fn division_by_zero_returns_infinity() {
        let expr = binary(number(1.0), Operator::Divide, number(0.0));

        let result = evaluate(&expr);

        assert!(result.is_infinite());
        assert!(result.is_sign_positive());
    }

    #[test]
    fn zero_divided_by_zero_returns_nan() {
        let expr = binary(number(0.0), Operator::Divide, number(0.0));

        let result = evaluate(&expr);

        assert!(result.is_nan());
    }
}

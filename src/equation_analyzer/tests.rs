#[cfg(test)]
mod tests {
    use crate::equation_analyzer::eq_data_builder::get_eq_data;
    use crate::equation_analyzer::pipeline::evaluator::evaluate;
    use crate::equation_analyzer::pipeline::parser::parse;
    use crate::equation_analyzer::pipeline::tokenizer::get_tokens;
    use crate::equation_analyzer::structs::token::TokenType::{
        CloseParen, End, Equal, Ln, Log, Minus, Number, OpenParen, Plus, Power, Sin, Slash, Star,
        X, Y, _E,
    };
    use crate::equation_analyzer::structs::token::{Token, TokenType};
    use std::f32::consts::{E, PI};

    fn is_close(x1: f32, x2: f32) -> bool {
        (x1 - x2).abs() < 0.00001
    }

    #[test]
    fn get_eq_data_test_linear() {
        let test_eq = "y = 2x + 1";
        let points = vec![(-1_f32, -1_f32), (0_f32, 1_f32), (1_f32, 3_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.literal, test_eq);
        assert_eq!(actual.points, points);
        assert_eq!(actual.zeros, vec![-0.5_f32]);
    }

    #[test]
    fn get_eq_data_test_linear_2() {
        let test_eq = "y = -2x + 1";
        let ans = vec![(-1_f32, 3_f32), (0_f32, 1_f32), (1_f32, -1_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.literal, test_eq);
        assert_eq!(actual.points, ans);
        assert_eq!(actual.zeros, vec![0.5_f32]);
    }

    #[test]
    fn get_eq_data_test_linear_3() {
        let test_eq = "y = -x + 1";
        let ans = vec![(-1_f32, 2_f32), (0_f32, 1_f32), (1_f32, 0_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.literal, test_eq);
        assert_eq!(actual.points, ans);
        assert_eq!(actual.zeros, vec![1_f32]);
    }

    #[test]
    fn get_eq_data_test_quad() {
        let test_eq = "y = x^2 + 2x + 1";
        let ans = vec![(-1_f32, 0_f32), (0_f32, 1_f32), (1_f32, 4_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.literal, test_eq);
        assert_eq!(actual.points, ans);
        assert_eq!(actual.zeros[0], -1_f32);
        assert!(actual.zeros[1].is_nan());
    }

    #[test]
    fn get_eq_data_test_quad_1() {
        let test_eq = "y = -2x^2 + 2x + 1";
        let points = vec![(-1_f32, -3_f32), (0_f32, 1_f32), (1_f32, 1_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.literal, test_eq);
        assert_eq!(actual.points, points);
        assert!(actual.zeros.contains(&-0.36602783));
        assert!(actual.zeros.contains(&1.3660278));
    }

    #[test]
    fn get_eq_data_test_quad_2() {
        let test_eq = "y = x^2 - 1";
        let points = vec![(-1_f32, 0_f32), (0_f32, -1_f32), (1_f32, 0_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.literal, test_eq);
        assert_eq!(actual.points, points);
        assert!(actual.zeros.contains(&1_f32));
        assert!(actual.zeros.contains(&-1_f32));
    }

    #[test]
    fn get_eq_data_test_quad_3() {
        let test_eq = "y = x^2 + 1";
        let points = vec![(-1_f32, 2_f32), (0_f32, 1_f32), (1_f32, 2_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.literal, test_eq);
        assert_eq!(actual.points, points);
        assert!(actual.zeros.is_empty());
    }

    #[test]
    fn get_eq_data_test_sin() {
        let test_eq = "y = sin( x )";
        let expected = vec![
            (-PI, 0_f32),
            (-PI / 2_f32, -1_f32),
            (0_f32, 0_f32),
            (PI / 2_f32, 1_f32),
            (PI, 0_f32),
        ];

        let actual = get_eq_data(test_eq, -PI, PI, PI / 2_f32).unwrap();

        for ((x_1, y_1), (x_2, y_2)) in actual.points.iter().zip(expected) {
            assert!(is_close(*x_1, x_2));
            assert!(is_close(*y_1, y_2));
        }

        assert_eq!(actual.literal, test_eq);
        assert!(actual.zeros.is_empty());
    }

    #[test]
    fn get_eq_data_test_cos() {
        let test_eq = "y = cos( x + 3.1415926 )";
        let expected = vec![
            (-PI, 1_f32),
            (-PI / 2_f32, 0_f32),
            (0_f32, -1_f32),
            (PI / 2_f32, 0_f32),
            (PI, 1_f32),
        ];

        let actual = get_eq_data(test_eq, -PI, PI, PI / 2_f32).unwrap();

        for ((x_1, y_1), (x_2, y_2)) in actual.points.iter().zip(expected) {
            assert!(is_close(*x_1, x_2));
            assert!(is_close(*y_1, y_2));
        }

        assert_eq!(actual.literal, test_eq);
        assert!(actual.zeros.is_empty());
    }

    #[test]
    fn get_eq_data_test_sqrt() {
        let test_eq = "y = sqrt( x^2 )";
        let expected = vec![
            (2_f32, 2_f32),
            (2.25_f32, 2.25_f32),
            (2.5_f32, 2.5_f32),
            (2.75_f32, 2.75_f32),
            (3_f32, 3_f32),
        ];

        let actual = get_eq_data(test_eq, 2_f32, 3_f32, 0.25_f32).unwrap();

        for ((x_1, y_1), (x_2, y_2)) in actual.points.iter().zip(expected) {
            assert!(is_close(*x_1, x_2));
            assert!(is_close(*y_1, y_2));
        }

        assert_eq!(actual.literal, test_eq);
        assert!(actual.zeros.is_empty());
    }

    #[test]
    fn get_eq_data_test_log() {
        let test_eq = "y = log_10( 10 ^ x ) + x";
        let expected = vec![
            (1_f32, 2_f32),
            (2_f32, 4_f32),
            (3_f32, 6_f32),
            (4_f32, 8_f32),
            (5_f32, 10_f32),
            (6_f32, 12_f32),
            (7_f32, 14_f32),
            (8_f32, 16_f32),
            (9_f32, 18_f32),
            (10_f32, 20_f32),
        ];

        let actual = get_eq_data(test_eq, 1_f32, 10_f32, 1_f32).unwrap();

        for ((x_1, y_1), (x_2, y_2)) in actual.points.iter().zip(expected) {
            assert_eq!(*x_1, x_2);
            assert_eq!(*y_1, y_2);
        }

        assert_eq!(actual.literal, test_eq);
        assert!(actual.zeros.is_empty());
    }

    fn get_token(t_t: TokenType, lit: &str) -> Token {
        Token {
            token_type: t_t,
            literal: lit.to_owned(),
        }
    }

    #[test]
    fn parse_test_1() {
        //y = 3 + 4 * ( 2 - 1 )
        let test = vec![
            get_token(TokenType::Y, "y"),
            get_token(TokenType::Equal, "="),
            get_token(TokenType::Number, "3"),
            get_token(TokenType::Plus, "+"),
            get_token(TokenType::Number, "4"),
            get_token(TokenType::Star, "*"),
            get_token(TokenType::OpenParen, "("),
            get_token(TokenType::Number, "2"),
            get_token(TokenType::Minus, "-"),
            get_token(TokenType::Number, "1"),
            get_token(TokenType::CloseParen, ")"),
            get_token(TokenType::End, "end"),
        ];
        let ans = vec!["3", "4", "2", "1", "-", "*", "+"];

        assert_eq!(parse(test).unwrap(), ans);
    }

    #[test]
    fn parse_test_2() {
        //2 ^ x;
        let test = vec![
            get_token(TokenType::Number, "2"),
            get_token(TokenType::Power, "^"),
            get_token(TokenType::X, "1x^1"),
        ];
        assert_eq!(parse(test).unwrap(), vec!["2", "1x^1", "^"]);
    }

    #[test]
    fn parse_test_3() {
        //3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3;
        let test = vec![
            get_token(TokenType::Number, "3"),
            get_token(TokenType::Plus, "+"),
            get_token(TokenType::Number, "4"),
            get_token(TokenType::Star, "*"),
            get_token(TokenType::Number, "2"),
            get_token(TokenType::Slash, "/"),
            get_token(TokenType::OpenParen, "("),
            get_token(TokenType::Number, "1"),
            get_token(TokenType::Minus, "-"),
            get_token(TokenType::Number, "5"),
            get_token(TokenType::CloseParen, ")"),
            get_token(TokenType::Power, "^"),
            get_token(TokenType::Number, "2"),
            get_token(TokenType::Power, "^"),
            get_token(TokenType::Number, "3"),
            get_token(TokenType::End, "end"),
        ];
        let ans = vec![
            "3", "4", "2", "*", "1", "5", "-", "2", "3", "^", "^", "/", "+",
        ];

        assert_eq!(parse(test).unwrap(), ans);
    }

    #[test]
    fn parse_test_4() {
        //"3 ^ 2 + 4 * ( 2 - 1 )";
        let test = vec![
            get_token(TokenType::Number, "3"),
            get_token(TokenType::Power, "^"),
            get_token(TokenType::Number, "2"),
            get_token(TokenType::Plus, "+"),
            get_token(TokenType::Number, "4"),
            get_token(TokenType::Star, "*"),
            get_token(TokenType::OpenParen, "("),
            get_token(TokenType::Number, "2"),
            get_token(TokenType::Minus, "-"),
            get_token(TokenType::Number, "1"),
            get_token(TokenType::CloseParen, ")"),
            get_token(TokenType::End, "end"),
        ];

        let ans = vec!["3", "2", "^", "4", "2", "1", "-", "*", "+"];
        assert_eq!(parse(test).unwrap(), ans);
    }

    #[test]
    fn parse_test_5() {
        //sin( max( ( 2 + 0 ) , 3 ) / ( 3 * π ) )
        let test = vec![
            get_token(TokenType::Sin, "sin("),
            get_token(TokenType::Max, "max("),
            get_token(TokenType::OpenParen, "("),
            get_token(TokenType::Number, "2"),
            get_token(TokenType::Plus, "+"),
            get_token(TokenType::Number, "0"),
            get_token(TokenType::CloseParen, ")"),
            get_token(TokenType::Comma, ","),
            get_token(TokenType::Number, "3"),
            get_token(TokenType::CloseParen, ")"),
            get_token(TokenType::Slash, "/"),
            get_token(TokenType::OpenParen, "("),
            get_token(TokenType::Number, "3"),
            get_token(TokenType::Star, "*"),
            get_token(TokenType::_Pi, "π"),
            get_token(TokenType::CloseParen, ")"),
            get_token(TokenType::CloseParen, ")"),
            get_token(TokenType::End, "end"),
        ];

        let ans = vec!["2", "0", "+", "3", "max(", "3", "π", "*", "/", "sin("];
        assert_eq!(parse(test).unwrap(), ans);
    }

    #[test]
    fn test_1() {
        let eq = "y = 32.2";

        let ans = vec![
            get_token(Y, "y"),
            get_token(Equal, "="),
            get_token(Number, "32.2"),
            get_token(End, "end"),
        ];

        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_2() {
        let eq = "y=e +47- x";
        let ans = vec![
            get_token(Y, "y"),
            get_token(Equal, "="),
            get_token(_E, "e"),
            get_token(Plus, "+"),
            get_token(Number, "47"),
            get_token(Minus, "-"),
            get_token(X, "1x^1"),
            get_token(End, "end"),
        ];

        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_3() {
        let eq = "y=3x";
        let ans = vec![
            get_token(Y, "y"),
            get_token(Equal, "="),
            get_token(X, "3x^1"),
            get_token(End, "end"),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_4() {
        let eq = "y= 3x^2 +x";
        let ans = vec![
            get_token(Y, "y"),
            get_token(Equal, "="),
            get_token(X, "3x^2"),
            get_token(Plus, "+"),
            get_token(X, "1x^1"),
            get_token(End, "end"),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_5() {
        let eq = "y= sin(3x+ 2 )";
        let ans = vec![
            get_token(Y, "y"),
            get_token(Equal, "="),
            get_token(Sin, "sin("),
            get_token(X, "3x^1"),
            get_token(Plus, "+"),
            get_token(Number, "2"),
            get_token(CloseParen, ")"),
            get_token(End, "end"),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_6() {
        let eq = "y= ln(3x+ 2 ) * ( 3--2)/ 6";
        let ans = vec![
            get_token(Y, "y"),
            get_token(Equal, "="),
            get_token(Ln, "ln("),
            get_token(X, "3x^1"),
            get_token(Plus, "+"),
            get_token(Number, "2"),
            get_token(CloseParen, ")"),
            get_token(Star, "*"),
            get_token(OpenParen, "("),
            get_token(Number, "3"),
            get_token(Minus, "-"),
            get_token(Number, "-2"),
            get_token(CloseParen, ")"),
            get_token(Slash, "/"),
            get_token(Number, "6"),
            get_token(End, "end"),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_7() {
        let eq = "y=log_3(x )";
        let ans = vec![
            get_token(Y, "y"),
            get_token(Equal, "="),
            get_token(Log, "log_3("),
            get_token(X, "1x^1"),
            get_token(CloseParen, ")"),
            get_token(End, "end"),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_8() {
        let eq = "y=3 ^x";
        let ans = vec![
            get_token(Y, "y"),
            get_token(Equal, "="),
            get_token(Number, "3"),
            get_token(Power, "^"),
            get_token(X, "1x^1"),
            get_token(End, "end"),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn get_and_eval_rpn_test_trig() {
        let test = "min( (max( ( 2 + 0 ) , 3 ) / ( 3 * 3 )) , 2 )";
        let ans = vec!["2", "0", "+", "3", "max(", "3", "3", "*", "/", "2", "min("];
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let eval = evaluate(&parsed_eq, f32::NAN);
        assert_eq!(parsed_eq, ans);
        assert!(is_close(eval.unwrap(), 0.33333334));
    }

    #[test]
    fn get_and_eval_rpn_test_trig_2() {
        let test = "1 + sin( max( 2 , 3 ) / 3 * 3.1415916 )";
        let ans = vec![
            "1",
            "2",
            "3",
            "max(",
            "3",
            "/",
            "3.1415916",
            "*",
            "sin(",
            "+",
        ];
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let eval = evaluate(&parsed_eq, f32::NAN);
        assert_eq!(parsed_eq, ans);
        assert!(is_close(eval.unwrap(), 1_f32));
    }

    #[test]
    fn eval_rpn_test_1() {
        let test = "3 + 4 * ( 2 - 1 )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_2() {
        let test = "3 + 4 * 2 - 1";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert_eq!(ans, 10_f32);
    }

    #[test]
    fn eval_rpn_test_3() {
        let test = "y = 3 + 4 * ( 2 - x )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, 1_f32).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_4() {
        let test = "y = x^2 + x + 3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, 2_f32).unwrap();
        assert_eq!(ans, 9_f32);
    }

    #[test]
    fn eval_rpn_test_5() {
        let test = "y = x^2 + 2x + 3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, 2_f32).unwrap();
        assert_eq!(ans, 11_f32);
    }

    #[test]
    fn eval_rpn_test_6() {
        let test = "-2 + 3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert_eq!(ans, 1_f32);
    }

    #[test]
    fn eval_rpn_test_7() {
        let test = "-e + -π";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert_eq!(ans, -E + -PI);
    }

    #[test]
    fn eval_rpn_test_8() {
        let test = "y = 2 ^ x^2";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, 2_f32).unwrap();
        assert_eq!(ans, 16_f32);
    }

    #[test]
    fn eval_rpn_test_9() {
        let test = "y = 2 ^ 3x";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, 2_f32).unwrap();
        assert_eq!(ans, 64_f32);
    }

    #[test]
    fn eval_rpn_test_10() {
        let test = "y = 2 ^ (2x + 1 )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, 2_f32).unwrap();
        assert_eq!(ans, 32_f32);
    }

    #[test]
    fn eval_rpn_test_trig_2() {
        let test = "sin( 3.1415926)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_trig_3() {
        let test = " sin( 3.1415926 )/ 2";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_trig_4() {
        let test = "sin( 3.1415926/2 )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_trig_5() {
        let test = "cos(3.1415926 ) / 2";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, -0.5f32));
    }

    #[test]
    fn eval_rpn_test_trig_6() {
        let test = "tan( 3.1415926 )+ cos( 3.1415926+3.1415926 ) + sin( 2 *3.1415926 )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_trig_7() {
        let test = "sin( -π )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_trig_max() {
        let test = "tan( 3.1415926) +max( 0 ,3.1415926) +sin(2 * 3.1415926)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, PI));
    }

    #[test]
    fn eval_rpn_test_trig_max_2() {
        let test = "max(sin(3.1415926) , max(( 2^3 ),6 ))";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, 8_f32));
    }

    #[test]
    fn eval_rpn_test_abs() {
        let test = "abs(2 - 3^2)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_abs_2() {
        let test = "abs(2 *3 - 3^2)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert_eq!(ans, 3_f32);
    }

    #[test]
    fn eval_rpn_test_sqrt() {
        let test = "sqrt(1764)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert_eq!(ans, 42_f32);
    }

    #[test]
    fn eval_rpn_test_min() {
        let test = "min(max(5,8), max(7,9))";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert_eq!(ans, 8_f32);
    }

    #[test]
    fn eval_rpn_test_ln() {
        let test = "ln(e)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_log() {
        let test = "log_10(10)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_log_add() {
        let test = "log_10(10) + log_10(10)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, 2_f32));
    }

    #[test]
    fn eval_rpn_test_log_add_2() {
        let test = "log_10(10) + log_10(10) + log_10(10)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, 3_f32));
    }

    #[test]
    fn eval_rpn_test_log_add_3() {
        let test = "log_10(10) + log_10(5 + 5)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, 2_f32));
    }

    #[test]
    fn eval_rpn_test_log_base_7() {
        let test = "log_7(49)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, 2_f32));
    }

    #[test]
    fn eval_rpn_test_invalid_coefficient() {
        let test = "y = ax^2";
        let tokens = get_tokens(test).unwrap_err();
        assert_eq!(tokens, "Invalid input at character 4");
    }

    #[test]
    fn minus_test_1() {
        let test = "3-3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert_eq!(ans, 0_f32);
    }

    #[test]
    fn minus_test_2() {
        let test = "3- 3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert_eq!(ans, 0_f32);
    }

    #[test]
    fn minus_test_3() {
        let test = "log_3(3)- 3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert_eq!(ans, -2_f32);
    }

    #[test]
    fn minus_test_4() {
        let test = "3--3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert_eq!(ans, 6_f32);
    }

    #[test]
    fn extra_pow_test() {
        let test = "2^2-3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert_eq!(ans, 1_f32);
    }

    #[test]
    fn extra_pow_test_2() {
        let test = "10^10-3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert_eq!(ans, 9999999997_f32);
    }

    #[test]
    fn eval_rpn_test_invalid_power() {
        let test = "y = 3x^a";
        let tokens = get_tokens(test).unwrap_err();
        assert_eq!(tokens, "Invalid power");
    }

    #[test]
    fn eval_rpn_test_power() {
        let test = "3^2";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();

        assert!(is_close(ans, 9_f32));
    }
}

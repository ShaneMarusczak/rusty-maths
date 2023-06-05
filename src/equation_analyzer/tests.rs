#[cfg(test)]
mod rm_tests {
    use crate::equation_analyzer::calculator::{calculate, plot};
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
    }

    #[test]
    fn get_eq_data_test_linear_2() {
        let test_eq = "y = -2x + 1";
        let ans = vec![(-1_f32, 3_f32), (0_f32, 1_f32), (1_f32, -1_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.literal, test_eq);
        assert_eq!(actual.points, ans);
    }

    #[test]
    fn get_eq_data_test_linear_3() {
        let test_eq = "y = -x + 1";
        let ans = vec![(-1_f32, 2_f32), (0_f32, 1_f32), (1_f32, 0_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.literal, test_eq);
        assert_eq!(actual.points, ans);
    }

    #[test]
    fn get_eq_data_test_quad() {
        let test_eq = "y = x^2 + 2x + 1";
        let ans = vec![(-1_f32, 0_f32), (0_f32, 1_f32), (1_f32, 4_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.literal, test_eq);
        assert_eq!(actual.points, ans);
    }

    #[test]
    fn get_eq_data_test_quad_1() {
        let test_eq = "y = -2x^2 + 2x + 1";
        let points = vec![(-1_f32, -3_f32), (0_f32, 1_f32), (1_f32, 1_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.literal, test_eq);
        assert_eq!(actual.points, points);
    }

    #[test]
    fn get_eq_data_test_quad_2() {
        let test_eq = "y = x^2 - 1";
        let points = vec![(-1_f32, 0_f32), (0_f32, -1_f32), (1_f32, 0_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.literal, test_eq);
        assert_eq!(actual.points, points);
    }

    #[test]
    fn get_eq_data_test_quad_3() {
        let test_eq = "y = x^2 + 1";
        let points = vec![(-1_f32, 2_f32), (0_f32, 1_f32), (1_f32, 2_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.literal, test_eq);
        assert_eq!(actual.points, points);
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
    }

    fn get_token_n(t_t: TokenType, numeric_value_1: f32, numeric_value_2: f32) -> Token {
        Token {
            token_type: t_t,
            numeric_value_1,
            numeric_value_2,
        }
    }

    fn get_token(t_t: TokenType) -> Token {
        get_token_n(t_t, 0.0, 0.0)
    }

    #[test]
    fn parse_test_1() {
        //y = 3 + 4 * ( 2 - 1 )
        let test = vec![
            get_token(Y),
            get_token(Equal),
            get_token_n(Number, 3.0, 0.0),
            get_token(Plus),
            get_token_n(Number, 4.0, 0.0),
            get_token(Star),
            get_token(OpenParen),
            get_token_n(Number, 2.0, 0.0),
            get_token(Minus),
            get_token_n(Number, 1.0, 0.0),
            get_token(CloseParen),
            get_token(End),
        ];
        let ans = vec![
            get_token_n(Number, 3.0, 0.0),
            get_token_n(Number, 4.0, 0.0),
            get_token_n(Number, 2.0, 0.0),
            get_token_n(Number, 1.0, 0.0),
            get_token(Minus),
            get_token(Star),
            get_token(Plus),
        ];

        assert_eq!(parse(test).unwrap(), ans);
    }

    #[test]
    fn parse_test_2() {
        //2 ^ x;
        let test = vec![
            get_token_n(Number, 2.0, 0.0),
            get_token(Power),
            get_token_n(X, 1.0, 1.0),
            get_token(End),
        ];
        assert_eq!(
            parse(test).unwrap(),
            vec![
                get_token_n(Number, 2.0, 0.0),
                get_token_n(X, 1.0, 1.0),
                get_token(Power)
            ]
        );
    }

    #[test]
    fn parse_test_3() {
        //3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3;
        let test = vec![
            get_token_n(Number, 3.0, 0.0),
            get_token(Plus),
            get_token_n(Number, 4.0, 0.0),
            get_token(Star),
            get_token_n(Number, 2.0, 0.0),
            get_token(Slash),
            get_token(OpenParen),
            get_token_n(Number, 1.0, 0.0),
            get_token(Minus),
            get_token_n(Number, 5.0, 0.0),
            get_token(CloseParen),
            get_token(Power),
            get_token_n(Number, 2.0, 0.0),
            get_token(Power),
            get_token_n(Number, 3.0, 0.0),
            get_token(End),
        ];
        let ans = vec![
            get_token_n(Number, 3.0, 0.0),
            get_token_n(Number, 4.0, 0.0),
            get_token_n(Number, 2.0, 0.0),
            get_token(Star),
            get_token_n(Number, 1.0, 0.0),
            get_token_n(Number, 5.0, 0.0),
            get_token(Minus),
            get_token_n(Number, 2.0, 0.0),
            get_token_n(Number, 3.0, 0.0),
            get_token(Power),
            get_token(Power),
            get_token(Slash),
            get_token(Plus),
        ];

        assert_eq!(parse(test).unwrap(), ans);
    }

    #[test]
    fn parse_test_4() {
        //sin( max( ( 2 + 0 ) , 3 ) / ( 3 * π ) )
        let test = vec![
            get_token(Sin),
            get_token(TokenType::Max),
            get_token(OpenParen),
            get_token_n(Number, 2.0, 0.0),
            get_token(Plus),
            get_token_n(Number, 0.0, 0.0),
            get_token(CloseParen),
            get_token(TokenType::Comma),
            get_token_n(Number, 3.0, 0.0),
            get_token(CloseParen),
            get_token(Slash),
            get_token(OpenParen),
            get_token_n(Number, 3.0, 0.0),
            get_token(Star),
            get_token(TokenType::_Pi),
            get_token(CloseParen),
            get_token(CloseParen),
            get_token(End),
        ];

        let ans = vec![
            get_token_n(Number, 2.0, 0.0),
            get_token_n(Number, 0.0, 0.0),
            get_token(Plus),
            get_token_n(Number, 3.0, 0.0),
            get_token(TokenType::Max),
            get_token_n(Number, 3.0, 0.0),
            get_token(TokenType::_Pi),
            get_token(Star),
            get_token(Slash),
            get_token(Sin),
        ];
        assert_eq!(parse(test).unwrap(), ans);
    }

    #[test]
    fn parse_test_6_no_eof() {
        //2 ^ x;
        let test = vec![
            get_token_n(Number, 2.0, 0.0),
            get_token(Power),
            get_token_n(X, 1.0, 1.0),
        ];
        assert_eq!(parse(test).unwrap_err(), "No end token found");
    }

    #[test]
    fn parse_test_bad_function() {
        //2 ^ x;
        let test = vec![
            get_token(Sin),
            get_token(Power),
            get_token_n(X, 1.0, 1.0),
            get_token(End),
        ];
        assert_eq!(parse(test).unwrap_err(), "Invalid function");
    }

    #[test]
    fn parse_test_bad_parens() {
        //2 ^ x;
        let test = vec![
            get_token(OpenParen),
            get_token(Power),
            get_token_n(X, 1.0, 1.0),
            get_token(End),
        ];
        assert_eq!(parse(test).unwrap_err(), "Invalid opening parenthesis");
    }

    #[test]
    fn parse_test_bad_parens_2() {
        //2 ^ x;
        let test = vec![
            get_token(CloseParen),
            get_token(Power),
            get_token_n(X, 1.0, 1.0),
            get_token(End),
        ];
        assert_eq!(parse(test).unwrap_err(), "Invalid closing parenthesis");
    }

    #[test]
    fn test_1() {
        let eq = "y = 32.2";

        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token_n(Number, 32.2, 0.0),
            get_token(End),
        ];

        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_2() {
        let eq = "y=e +47- x";
        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token(_E),
            get_token(Plus),
            get_token_n(Number, 47.0, 0.0),
            get_token(Minus),
            get_token_n(X, 1.0, 1.0),
            get_token(End),
        ];

        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_3() {
        let eq = "y=3x";
        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token_n(X, 3.0, 1.0),
            get_token(End),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_4() {
        let eq = "y= 3x^2 +x";
        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token_n(X, 3.0, 2.0),
            get_token(Plus),
            get_token_n(X, 1.0, 1.0),
            get_token(End),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_5() {
        let eq = "y= sin(3x+ 2 )";
        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token(Sin),
            get_token_n(X, 3.0, 1.0),
            get_token(Plus),
            get_token_n(Number, 2.0, 0.0),
            get_token(CloseParen),
            get_token(End),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_6() {
        let eq = "y= ln(3x+ 2 ) * ( 3--2)/ 6";
        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token(Ln),
            get_token_n(X, 3.0, 1.0),
            get_token(Plus),
            get_token_n(Number, 2.0, 0.0),
            get_token(CloseParen),
            get_token(Star),
            get_token(OpenParen),
            get_token_n(Number, 3.0, 0.0),
            get_token(Minus),
            get_token_n(Number, -2.0, 0.0),
            get_token(CloseParen),
            get_token(Slash),
            get_token_n(Number, 6.0, 0.0),
            get_token(End),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_7() {
        let eq = "y=log_3(x )";
        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token_n(Log, 3.0, 0.0),
            get_token_n(X, 1.0, 1.0),
            get_token(CloseParen),
            get_token(End),
        ];
        let tokens = get_tokens(eq).unwrap();
        assert_eq!(tokens, ans);
    }

    #[test]
    fn test_8() {
        let eq = "y=3 ^x";
        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token_n(Number, 3.0, 0.0),
            get_token(Power),
            get_token_n(X, 1.0, 1.0),
            get_token(End),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn get_and_eval_rpn_test_trig() {
        let test = "min( (max( ( 2 + 0 ) , 3 ) / ( 3 * 3 )) , 2 )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let eval = evaluate(&parsed_eq, f32::NAN);
        assert!(is_close(eval.unwrap(), 0.33333334));
    }

    #[test]
    fn get_and_eval_rpn_test_trig_2() {
        let test = "1 + sin( max( 2 , 3 ) / 3 * 3.1415916 )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let eval = evaluate(&parsed_eq, f32::NAN);
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
        let test = " sin( π )/ 2";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_trig_4() {
        let test = "sin( π/2 )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_trig_5() {
        let test = "cos(π ) / 2";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, -0.5f32));
    }

    #[test]
    fn eval_rpn_test_trig_6() {
        let test = "tan( π )+ cos( π+π ) + sin( 2 *π )";
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
    fn eval_rpn_test_trig_8() {
        let test = "sin( π )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_trig_max() {
        let test = "tan( π ) +max( 0 ,π) +sin(2 * π)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert!(is_close(ans, PI));
    }

    #[test]
    fn eval_rpn_test_trig_max_2() {
        let test = "max(sin(π) , max(( 2^3 ),6 ))";
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
    fn extra_pow_test_4() {
        let test = "2^(2-3)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();
        assert_eq!(ans, 0.5);
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
    fn extra_pow_test_3() {
        let test = "10^10-3";
        let ans = calculate(test).unwrap();
        assert_eq!(ans, 9999999997.0);
    }

    #[test]
    fn modulo_test() {
        let test = "10 %% 3";
        let ans = calculate(test).unwrap();
        assert_eq!(ans, 1_f32);
    }

    #[test]
    fn percent_test() {
        let test = "10 % 30";
        let ans = calculate(test).unwrap();
        assert_eq!(ans, 3_f32);
    }

    #[test]
    fn factorial_test() {
        let test = "6!";
        let ans = calculate(test).unwrap();
        assert_eq!(ans, 720_f32);
    }

    #[test]
    fn factorial_test_2() {
        let test = "(8-2)!";
        let ans = calculate(test).unwrap();
        assert_eq!(ans, 720_f32);
    }

    #[test]
    fn factorial_test_3() {
        let test = "(8-2 + 2^2)! - 1";
        let ans = calculate(test).unwrap();
        assert_eq!(ans, 3628799_f32);
    }

    #[test]
    fn factorial_test_4() {
        let test = "0!";
        let ans = calculate(test).unwrap();
        assert_eq!(ans, 1_f32);
    }

    #[test]
    fn factorial_test_err() {
        let test = "(2.2)!";
        let ans = calculate(test).unwrap_err();
        assert_eq!(ans, "Factorial is only defined for positive whole numbers");
    }

    #[test]
    fn plot_test_linear() {
        let test_eq = "y = 2x +1";
        let points = vec![(-1_f32, -1_f32), (0_f32, 1_f32), (1_f32, 3_f32)];

        let actual = plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual, points);
    }

    #[test]
    fn plot_test_linear_1() {
        let test_eq = "y = π*x^2";
        let points = vec![(-1_f32, 3.1415927), (0_f32, 0_f32), (1_f32, 3.1415927)];

        let actual = plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual, points);
    }

    #[test]
    fn plot_test_quad() {
        let test_eq = "y = x^2";
        let points = vec![(-1_f32, 1_f32), (0_f32, 0_f32), (1_f32, 1_f32)];

        let actual = plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual, points);
    }

    #[test]
    fn plot_test_trig() {
        let test_eq = "y = tan(x)";
        let points = vec![(-1_f32, -1.5574077), (0_f32, 0_f32), (1_f32, 1.5574077)];

        let actual = plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        for ((x_1, y_1), (x_2, y_2)) in actual.iter().zip(points) {
            assert!(is_close(*x_1, x_2));
            assert!(is_close(*y_1, y_2));
        }
    }

    #[test]
    fn invalid_char_test() {
        let test = "3 ? 3";
        let tokens = get_tokens(test).unwrap_err();
        assert_eq!(tokens, "Invalid input at character 3");
    }

    #[test]
    fn eval_rpn_test_empty_eq() {
        let test = "";
        let tokens = get_tokens(test).unwrap_err();
        assert_eq!(tokens, "Invalid equation supplied");
    }

    #[test]
    fn eval_rpn_test_invalid_underscore() {
        let test = "sin_(5)";
        let tokens = get_tokens(test).unwrap_err();
        assert_eq!(tokens, "Invalid input at character 3");
    }

    #[test]
    fn eval_rpn_test_invalid_log() {
        let test = "log_(5)";
        let tokens = get_tokens(test).unwrap_err();
        assert_eq!(tokens, "Invalid use of log");
    }

    #[test]
    fn eval_rpn_test_invalid_power() {
        let test = "y = 3x^a";
        let tokens = get_tokens(test).unwrap_err();
        assert_eq!(tokens, "Invalid power");
    }

    #[test]
    fn eval_rpn_test_fn_name() {
        let test = "cro(5)";
        let tokens = get_tokens(test).unwrap_err();
        assert_eq!(tokens, "Invalid function name cro");
    }

    // #[test]
    // fn evaluator_bad_token() {
    //     let test = vec![String::from("5"), String::from("5"), String::from("cro(")];
    //     let tokens = evaluate(&test, f32::NAN).unwrap_err();
    //     assert_eq!(tokens, "Unknown token: cro(");
    // }

    #[test]
    fn eval_rpn_test_power() {
        let test = "3^2";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, f32::NAN).unwrap();

        assert!(is_close(ans, 9_f32));
    }

    #[test]
    fn test_calculate_with_complex_equation() {
        let test = "2*(3+4*(5-6))/7 + 8^(9/10)";
        let expected_result = 6.212_305;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn pi_test() {
        let test = "π + π - π - π";
        let expected_result = 0_f32;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn pi_test_2() {
        let test = "max(π,1)+sin(π) - sin(π) - π + sqrt(π) - sqrt(π) - 2^π + 2^π - π^2 + π^2";
        let expected_result = 0_f32;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn pi_test_3() {
        let test = "((min(5,π) + 2*π - 3*π)*2*π)/2*π";
        let expected_result = 0_f32;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }
}

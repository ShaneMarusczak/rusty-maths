#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::approx_constant, clippy::excessive_precision, clippy::manual_range_contains)]
mod rm_tests {
    use crate::equation_analyzer::utils::Point;

    // Import calculator
    use crate::equation_analyzer::calculator;

    // Internal testing utilities
    use crate::equation_analyzer::pipeline::evaluator::evaluate;
    use crate::equation_analyzer::pipeline::parser::parse;
    use crate::equation_analyzer::pipeline::tokenizer::StreamingTokenizer;
    use crate::equation_analyzer::structs::token::TokenType::{
        CloseParen, End, Equal, Ln, Log, Minus, Number, OpenParen, Plus, Power, Sin, Slash, Star,
        UnaryMinus, Y, _E,
    };
    use crate::equation_analyzer::structs::token::{Token, TokenType};
    use crate::utilities::abs_f32;
    use std::f32::consts::{E, PI};

    fn is_close(x1: f32, x2: f32) -> bool {
        abs_f32(x1 - x2) < f32::EPSILON
    }

    // Helper function to get tokens from an equation
    fn get_tokens(eq: &str) -> Result<Vec<Token>, String> {
        let tokenizer = StreamingTokenizer::new(eq)?;
        let tokens: Result<Vec<Token>, String> = tokenizer.collect();
        tokens
    }

    #[test]
    fn plot_test_linear() {
        let test_eq = "y = 2x + 1";
        let points = vec![
            Point::new(-1_f32, -1_f32),
            Point::new(0_f32, 1_f32),
            Point::new(1_f32, 3_f32),
        ];

        let actual = calculator::plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual, points);
    }

    #[test]
    fn plot_test_exp() {
        let test_eq = "y = x^x";
        let points = vec![
            Point::new(0_f32, 1_f32),
            Point::new(1_f32, 1_f32),
            Point::new(2_f32, 4_f32),
        ];

        let actual = calculator::plot(test_eq, 0f32, 2f32, 1_f32).unwrap();

        assert_eq!(actual, points);
    }

    #[test]
    fn plot_test_exp_2() {
        let test_eq = "y = x^(2x) + 1";
        let points = vec![
            Point::new(0_f32, 2_f32),
            Point::new(1_f32, 2_f32),
            Point::new(2_f32, 17_f32),
        ];

        let actual = calculator::plot(test_eq, 0f32, 2f32, 1_f32).unwrap();

        assert_eq!(actual, points);
    }

    #[test]
    fn plot_test_exp_3() {
        let test_eq = "y = x^2x + 1";
        let points = vec![
            Point::new(0_f32, 2_f32),
            Point::new(1_f32, 2_f32),
            Point::new(2_f32, 17_f32),
        ];

        let actual = calculator::plot(test_eq, 0f32, 2f32, 1_f32).unwrap();

        assert_eq!(actual, points);
    }

    #[test]
    fn plot_test_exp_4() {
        let test_eq = "y = x^x^x^x";
        let points = vec![
            Point::new(0_f32, 1_f32),
            Point::new(1_f32, 1_f32),
            Point::new(2_f32, 65536.),
        ];

        let actual = calculator::plot(test_eq, 0f32, 2f32, 1_f32).unwrap();

        assert_eq!(actual, points);
    }

    #[test]
    fn plot_test_linear_2() {
        let test_eq = "y = -2x + 1";
        let ans = vec![
            Point::new(-1_f32, 3_f32),
            Point::new(0_f32, 1_f32),
            Point::new(1_f32, -1_f32),
        ];

        let actual = calculator::plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual, ans);
    }

    #[test]
    fn plot_test_linear_3() {
        let test_eq = "y = -x + 1";
        let ans = vec![
            Point::new(-1_f32, 2_f32),
            Point::new(0_f32, 1_f32),
            Point::new(1_f32, 0_f32),
        ];

        let actual = calculator::plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual, ans);
    }

    #[test]
    fn plot_test_quad() {
        let test_eq = "y = x^2 + 2x + 1";
        let ans = vec![
            Point::new(-1_f32, 0_f32),
            Point::new(0_f32, 1_f32),
            Point::new(1_f32, 4_f32),
        ];

        let actual = calculator::plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual, ans);
    }

    #[test]
    fn plot_test_quad_1() {
        let test_eq = "y = -2x^2 + 2x + 1";
        let points = vec![
            Point::new(-1_f32, -3_f32),
            Point::new(0_f32, 1_f32),
            Point::new(1_f32, 1_f32),
        ];

        let actual = calculator::plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual, points);
    }

    #[test]
    fn plot_test_quad_2() {
        let test_eq = "y = x^2 - 1";
        let points = vec![
            Point::new(-1_f32, 0_f32),
            Point::new(0_f32, -1_f32),
            Point::new(1_f32, 0_f32),
        ];

        let actual = calculator::plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual, points);
    }

    #[test]
    fn plot_test_quad_3() {
        let test_eq = "y = x^2 + 1";
        let points = vec![
            Point::new(-1_f32, 2_f32),
            Point::new(0_f32, 1_f32),
            Point::new(1_f32, 2_f32),
        ];

        let actual = calculator::plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual, points);
    }

    #[test]
    fn plot_test_sin() {
        let test_eq = "y = sin( x )";
        let expected = vec![
            (-PI, 0_f32),
            (-PI / 2_f32, -1_f32),
            (0_f32, 0_f32),
            (PI / 2_f32, 1_f32),
            (PI, 0_f32),
        ];

        let actual = calculator::plot(test_eq, -PI, PI, PI / 2_f32).unwrap();

        for (p, (x_2, y_2)) in actual.iter().zip(expected) {
            assert!(is_close(p.x, x_2));
            assert!(is_close(p.y, y_2));
        }
    }

    #[test]
    fn plot_test_cos() {
        let test_eq = "y = cos( x+ 3.14159265358979323846        )";
        let expected = vec![
            (-PI, 1_f32),
            (-PI / 2_f32, 0_f32),
            (0_f32, -1_f32),
            (PI / 2_f32, 0_f32),
            (PI, 1_f32),
        ];

        let actual = calculator::plot(test_eq, -PI, PI, PI / 2_f32).unwrap();

        for (p, (x_2, y_2)) in actual.iter().zip(expected) {
            assert!(is_close(p.x, x_2));
            assert!(is_close(p.y, y_2));
        }
    }

    #[test]
    fn plot_test_sqrt() {
        let test_eq = "y = sqrt( x^2 )";
        let expected = vec![
            (2_f32, 2_f32),
            (2.25_f32, 2.25_f32),
            (2.5_f32, 2.5_f32),
            (2.75_f32, 2.75_f32),
            (3_f32, 3_f32),
        ];

        let actual = calculator::plot(test_eq, 2_f32, 3_f32, 0.25_f32).unwrap();

        for (p, (x_2, y_2)) in actual.iter().zip(expected) {
            assert!(is_close(p.x, x_2));
            assert!(is_close(p.y, y_2));
        }
    }

    #[test]
    fn plot_test_log() {
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

        let actual = calculator::plot(test_eq, 1_f32, 10_f32, 1_f32).unwrap();

        for (p, (x_2, y_2)) in actual.iter().zip(expected) {
            assert_eq!(p.x, x_2);
            assert_eq!(p.y, y_2);
        }
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

        assert_eq!(parse(test.into_iter().map(Ok)).unwrap(), ans);
    }

    #[test]
    fn parse_test_2() {
        //2 ^ 3;
        let test = vec![
            get_token_n(Number, 2.0, 0.0),
            get_token(Power),
            get_token_n(Number, 3.0, 0.0),
            get_token(End),
        ];
        assert_eq!(
            parse(test.into_iter().map(Ok)).unwrap(),
            vec![
                get_token_n(Number, 2.0, 0.0),
                get_token_n(Number, 3.0, 0.0),
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

        assert_eq!(parse(test.into_iter().map(Ok)).unwrap(), ans);
    }

    #[test]
    fn parse_test_6_no_eof() {
        //2 ^ 16;
        let test = vec![
            get_token_n(Number, 2.0, 0.0),
            get_token(Power),
            get_token_n(Number, 16.0, 0.0),
        ];
        assert_eq!(
            parse(test.into_iter().map(Ok)).unwrap_err(),
            "No end token found"
        );
    }

    #[test]
    fn parse_test_bad_function() {
        //2 ^ x;
        let test = vec![
            get_token(Sin),
            get_token(Power),
            get_token_n(Number, 1.0, 1.0),
            get_token(End),
        ];
        assert_eq!(
            parse(test.into_iter().map(Ok)).unwrap_err(),
            "Invalid function"
        );
    }

    #[test]
    fn parse_test_bad_parens() {
        //2 ^ x;
        let test = vec![
            get_token(OpenParen),
            get_token(Power),
            get_token_n(Number, 1.0, 1.0),
            get_token(End),
        ];
        assert_eq!(
            parse(test.into_iter().map(Ok)).unwrap_err(),
            "Invalid opening parenthesis"
        );
    }

    #[test]
    fn parse_test_bad_parens_2() {
        let test = vec![
            get_token(CloseParen),
            get_token(Power),
            get_token_n(Number, 1.0, 1.0),
            get_token(End),
        ];
        assert_eq!(
            parse(test.into_iter().map(Ok)).unwrap_err(),
            "Invalid closing parenthesis"
        );
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
        let eq = "y=e +47- 9";
        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token(_E),
            get_token(Plus),
            get_token_n(Number, 47.0, 0.0),
            get_token(Minus),
            get_token_n(Number, 9.0, 0.0),
            get_token(End),
        ];

        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_3() {
        let eq = "y=3";
        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token_n(Number, 3.0, 0.0),
            get_token(End),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_4() {
        let eq = "y= 3^2 +9";
        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token_n(Number, 3.0, 0.0),
            get_token(Power),
            get_token_n(Number, 2.0, 0.0),
            get_token(Plus),
            get_token_n(Number, 9.0, 0.0),
            get_token(End),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_5() {
        let eq = "y= sin(3+ 2 )";
        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token(Sin),
            get_token_n(Number, 3.0, 0.0),
            get_token(Plus),
            get_token_n(Number, 2.0, 0.0),
            get_token(CloseParen),
            get_token(End),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_6() {
        let eq = "y= ln(3+ 2 ) * ( 3--2)/ 6";
        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token(Ln),
            get_token_n(Number, 3.0, 0.0),
            get_token(Plus),
            get_token_n(Number, 2.0, 0.0),
            get_token(CloseParen),
            get_token(Star),
            get_token(OpenParen),
            get_token_n(Number, 3.0, 0.0),
            get_token(Minus),
            get_token(UnaryMinus),
            get_token_n(Number, 2.0, 0.0),
            get_token(CloseParen),
            get_token(Slash),
            get_token_n(Number, 6.0, 0.0),
            get_token(End),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_7() {
        let eq = "y=log_3(3 )";
        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token_n(Log, 3.0, 0.0),
            get_token_n(Number, 3.0, 0.0),
            get_token(CloseParen),
            get_token(End),
        ];
        let tokens = get_tokens(eq).unwrap();
        assert_eq!(tokens, ans);
    }

    #[test]
    fn test_8() {
        let eq = "y=3 ^10";
        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token_n(Number, 3.0, 0.0),
            get_token(Power),
            get_token_n(Number, 10.0, 0.0),
            get_token(End),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_9() {
        let eq = "y= 3^(-1/2)";
        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token_n(Number, 3.0, 0.0),
            get_token(Power),
            get_token(OpenParen),
            get_token(UnaryMinus),
            get_token_n(Number, 1.0, 0.0),
            get_token(Slash),
            get_token_n(Number, 2.0, 0.0),
            get_token(CloseParen),
            get_token(End),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn test_10() {
        let eq = "y= 3^-2";
        let ans = vec![
            get_token(Y),
            get_token(Equal),
            get_token_n(Number, 3.0, 0.0),
            get_token(Power),
            get_token(UnaryMinus),
            get_token_n(Number, 2.0, 0.0),
            get_token(End),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn eval_rpn_test_1() {
        let test = "3 + 4 * ( 2 - 1 )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_2() {
        let test = "3 + 4 * 2 - 1";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 10_f32);
    }

    #[test]
    fn eval_rpn_test_3() {
        let test = "y = 3 + 4 * ( 2 - 1 )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_4() {
        let test = "y = 16^(1/2) + 16 + 3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 23_f32);
    }

    #[test]
    fn eval_rpn_test_5() {
        let test = "y = 2^2 + 2*2 + 3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 11_f32);
    }

    #[test]
    fn eval_rpn_test_6() {
        let test = "-2 + 3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 1_f32);
    }

    #[test]
    fn eval_rpn_test_7() {
        let test = "-e + -π";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, -E + -PI);
    }

    #[test]
    fn eval_rpn_test_8() {
        let test = "y = 2 ^ 2^2";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 16_f32);
    }

    #[test]
    fn eval_rpn_test_9() {
        let test = "y = 2 ^ (3*2)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 64_f32);
    }

    #[test]
    fn eval_rpn_test_10() {
        let test = "y = 2 ^ (2*2 + 1 )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 32_f32);
    }

    #[test]
    fn eval_rpn_test_trig_2() {
        let test = "sin( 3.14159265358979323846)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_trig_3() {
        let test = " sin( π )/ 2";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_trig_4() {
        let test = "sin( π/2 )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_trig_5() {
        let test = "cos(π ) / 2";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert!(is_close(ans, -0.5f32));
    }

    #[test]
    fn eval_rpn_test_trig_6() {
        let test = "tan( π )+ cos( π+π ) + sin( 2 *π )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap().round();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_trig_7() {
        let test = "sin( -π )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_trig_8() {
        let test = "sin( π )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_abs() {
        let test = "abs(2 - 3^2)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_abs_2() {
        let test = "abs(2 *3 - 3^2)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 3_f32);
    }

    #[test]
    fn eval_rpn_test_sqrt() {
        let test = "sqrt(1764)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 42_f32);
    }

    #[test]
    fn eval_rpn_test_min() {
        let test = "min(5,8,7,9)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 5_f32);
    }

    #[test]
    fn eval_rpn_test_ln() {
        let test = "ln(e)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_log() {
        let test = "log_10(10)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_log_add() {
        let test = "log_10(10) + log_10(10)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert!(is_close(ans, 2_f32));
    }

    #[test]
    fn eval_rpn_test_log_add_2() {
        let test = "log_10(10) + log_10(10) + log_10(10)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert!(is_close(ans, 3_f32));
    }

    #[test]
    fn eval_rpn_test_log_add_3() {
        let test = "log_10(10) + log_10(5 + 5)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert!(is_close(ans, 2_f32));
    }

    #[test]
    fn eval_rpn_test_log_base_7() {
        let test = "log_7(49)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
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
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 0_f32);
    }

    #[test]
    fn minus_test_2() {
        let test = "3- 3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 0_f32);
    }

    #[test]
    fn minus_test_3() {
        let test = "log_3(3)- 3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, -2_f32);
    }

    #[test]
    fn minus_test_4() {
        let test = "3--3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 6_f32);
    }

    #[test]
    fn extra_pow_test() {
        let test = "2^2-3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 1_f32);
    }

    #[test]
    fn extra_pow_test_4() {
        let test = "2^(2-3)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 0.5);
    }

    #[test]
    fn extra_pow_test_2() {
        let test = "10^10-3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();
        assert_eq!(ans, 9999999997_f32);
    }

    #[test]
    fn extra_pow_test_3() {
        let test = "10^10-3";
        let ans = calculator::calculate(test).unwrap();
        assert_eq!(ans, 9999999997.0);
    }

    #[test]
    fn modulo_test() {
        let test = "10 %% 3";
        let ans = calculator::calculate(test).unwrap();
        assert_eq!(ans, 1_f32);
    }

    #[test]
    fn percent_test() {
        let test = "10 % 30";
        let ans = calculator::calculate(test).unwrap();
        assert_eq!(ans, 3_f32);
    }

    #[test]
    fn factorial_test() {
        let test = "6!";
        let ans = calculator::calculate(test).unwrap();
        assert_eq!(ans, 720_f32);
    }

    #[test]
    fn factorial_test_2() {
        let test = "(8-2)!";
        let ans = calculator::calculate(test).unwrap();
        assert_eq!(ans, 720_f32);
    }

    #[test]
    fn factorial_test_3() {
        let test = "(8-2 + 2^2)! - 1";
        let ans = calculator::calculate(test).unwrap();
        assert_eq!(ans, 3628799_f32);
    }

    #[test]
    fn factorial_test_4() {
        let test = "0!";
        let ans = calculator::calculate(test).unwrap();
        assert_eq!(ans, 1_f32);
    }

    #[test]
    fn factorial_test_err() {
        let test = "(2.2)!";
        let ans = calculator::calculate(test).unwrap_err();
        assert_eq!(ans, "Factorial is only defined for non-negative integers");
    }

    #[test]
    fn plot_test_linear_7() {
        let test_eq = "y = 2x +1";
        let points = vec![
            Point::new(-1_f32, -1_f32),
            Point::new(0_f32, 1_f32),
            Point::new(1_f32, 3_f32),
        ];

        let actual = calculator::plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual, points);
    }

    #[test]
    fn plot_test_linear_1() {
        let test_eq = "y = π*x^2";
        let points = vec![
            Point::new(-1_f32, 3.1415927),
            Point::new(0_f32, 0_f32),
            Point::new(1_f32, 3.1415927),
        ];

        let actual = calculator::plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual, points);
    }

    #[test]
    fn plot_test_quad_dup() {
        let test_eq = "y = x^2";
        let points = vec![
            Point::new(-1_f32, 1_f32),
            Point::new(0_f32, 0_f32),
            Point::new(1_f32, 1_f32),
        ];

        let actual = calculator::plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual, points);
    }

    #[test]
    fn plot_test_trig() {
        let test_eq = "y = tan(x)";
        let points = vec![(-1_f32, -1.5574077), (0_f32, 0_f32), (1_f32, 1.5574077)];

        let actual = calculator::plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        for (p, (x_2, y_2)) in actual.iter().zip(points) {
            assert!(is_close(p.x, x_2));
            assert!(is_close(p.y, y_2));
        }
    }

    #[test]
    fn plot_test_max() {
        let test_eq = "y = max(x,0.5)";
        let points = vec![(-1_f32, 0.5), (0_f32, 0.5), (1_f32, 1_f32)];

        let actual = calculator::plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        for (p, (x_2, y_2)) in actual.iter().zip(points) {
            assert!(is_close(p.x, x_2));
            assert!(is_close(p.y, y_2));
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

    // #[test]
    // fn eval_rpn_test_invalid_power() {
    //     let test = "y = 3x^a";
    //     let tokens = get_tokens(test).unwrap_err();
    //     assert_eq!(tokens, "Invalid power");
    // }

    #[test]
    fn eval_rpn_test_fn_name() {
        let test = "cro(5)";
        let tokens = get_tokens(test).unwrap_err();
        assert_eq!(tokens, "Invalid function name cro");
    }

    #[test]
    fn eval_rpn_test_power() {
        let test = "3^2";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens.into_iter().map(Ok)).unwrap();
        let ans = evaluate(parsed_eq.iter().copied(), None).unwrap();

        assert!(is_close(ans, 9_f32));
    }

    #[test]
    fn test_calculate_with_complex_equation() {
        let test = "2*(3+4*(5-6))/7 + 8^(9/10)";
        let expected_result = 6.21230459;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn pi_test() {
        let test = "π + π - π - π";
        let expected_result = 0_f32;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn pi_test_2() {
        let test = "π+sin(π) - sin(π) - π + sqrt(π) - sqrt(π) - 2^π + 2^π - π^2 + π^2";
        let expected_result = 0_f32;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn pi_test_3() {
        let test = "((π + 2*π - 3*π)*2*π)/2*π";
        let expected_result = 0_f32;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn avg_test() {
        let test = "avg(1,2,9)";
        let expected_result = 4_f32;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn avg_test_2() {
        let test = "avg(1,2,9) + avg(1,2,9)";
        let expected_result = 8_f32;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn avg_test_3() {
        let test = "avg(1,2,9) + sin(avg(-12,1,2,9))";
        let expected_result = 4_f32;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn avg_test_4() {
        // Now allows expressions in parameters (frame-based evaluation)
        let test = "avg(1,2,sin(0))";
        let expected_result = 1.0; // avg(1, 2, 0) = 1.0
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn min_test() {
        let test = "min(1,2,9)";
        let expected_result = 1_f32;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn max_test() {
        let test = "max(1,2,9)";
        let expected_result = 9_f32;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn max_min_test() {
        let test = "max(1,2,9) + min(1,2,9)";
        let expected_result = 10_f32;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn max_min_avg_test() {
        let test = "max(1,2,9) + min(1,2,9) + avg(1,2,9)";
        let expected_result = 14_f32;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn med_test() {
        let test = "med(2,1,9)";
        let expected_result = 2_f32;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn med_test_2() {
        let test = "med(1,2,9,11)";
        let expected_result = 5.5;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn mode_test() {
        let test = "mode(1,2,3,4,5,2)";
        let expected_result = 2_f32;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn mode_test_2() {
        let test = "mode(1.1,2.3,3.4,3.4,5,2)";
        let expected_result = 3.4;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn mode_test_uniform() {
        // Uniform distribution: all values appear once, no mode exists
        let test = "mode(1, 2, 3, 4, 5)";
        let actual_result = calculator::calculate(test).unwrap();
        assert!(
            actual_result.is_nan(),
            "Uniform distribution should return NaN"
        );
    }

    #[test]
    fn mode_test_multimodal() {
        // Multimodal: 1 and 3 both appear twice, should return average (1+3)/2 = 2
        let test = "mode(1, 1, 3, 3, 5)";
        let expected_result = 2.0;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn mode_test_multimodal_three_modes() {
        // Three modes: 1, 2, and 3 all appear twice, average = (1+2+3)/3 = 2
        let test = "mode(1, 1, 2, 2, 3, 3)";
        let expected_result = 2.0;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn test_nested_variadic_not_supported() {
        // Nested variadic functions NOW WORK with frame-based evaluation!
        let result = calculator::calculate("avg(1, min(2, 3), 4)");
        // avg(1, 2, 4) = 7/3 ≈ 2.333...
        assert!(result.is_ok(), "Nested variadic functions should work");
        assert!(is_close(result.unwrap(), 7.0 / 3.0));
    }

    #[test]
    fn test_nested_variadic_min_in_max() {
        // Nested variadic functions work with frame-based evaluation!
        let result = calculator::calculate("max(1, min(2, 3))");
        assert!(result.is_ok(), "Nested variadic functions should work");
        // max(1, 2) = 2
        assert_eq!(result.unwrap(), 2.0);
    }

    #[test]
    fn test_nested_variadic_mode_in_avg() {
        // Nested variadic functions work with frame-based evaluation!
        let result = calculator::calculate("avg(mode(1, 1, 2), 5)");
        assert!(result.is_ok(), "Nested variadic functions should work");
        // mode(1, 1, 2) = 1, so avg(1, 5) = 3
        assert_eq!(result.unwrap(), 3.0);
    }

    #[test]
    fn test_variadic_with_arithmetic_outside() {
        // Arithmetic outside variadic functions works fine
        assert_eq!(calculator::calculate("avg(2, 4, 9) + 1").unwrap(), 6.0); // 5 + 1
        assert_eq!(calculator::calculate("min(3, 5, 6) * 2").unwrap(), 6.0); // 3 * 2
        assert_eq!(calculator::calculate("max(2, 4, 6) - 1").unwrap(), 5.0); // 6 - 1
    }

    #[test]
    fn test_variadic_parameters_must_be_simple() {
        // Variadic functions NOW ACCEPT full expressions with frame-based evaluation!
        let result = calculator::calculate("avg(1+1, 2)");
        // avg(2, 2) = 2.0
        assert!(
            result.is_ok(),
            "Arithmetic inside variadic params should work"
        );
        assert!(is_close(result.unwrap(), 2.0));
    }

    #[test]
    fn choice_test() {
        let test = "ch(5,2)";
        let expected_result = 10.0;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn choice_test_2() {
        let test = "ch( 20 ,9)";
        let expected_result = 167960.0;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn choice_test_3() {
        let test = "ch(20,  9   ) - 10000";
        let expected_result = 157960.0;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn choice_test_error() {
        let test = "ch(20,9, 0)";
        let expected_result = "choice requires exactly 2 parameters, got 3";
        let actual_result = calculator::calculate(test).unwrap_err();
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn choice_test_error_2() {
        let test = "ch(20,9.1)";
        let expected_result = "Parameter 2 must be an integer, got 9.1";
        let actual_result = calculator::calculate(test).unwrap_err();
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn a_trig_test() {
        let test = "asin(sin(0.5)) + acos(cos(0.5)) + atan(tan(0.5))";
        let expected_result = 1.5;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn y_test() {
        let test = "y";
        let expected_result = "Invalid equation supplied";
        let actual_result = calculator::calculate(test).unwrap_err();
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn neg_test() {
        let test = "-(5 + 2)";
        let expected_result = -7.0;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn neg_test_2() {
        let test = "-5 + (2)";
        let expected_result = -3.0;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn neg_test_3() {
        let test = "-sqrt(4)";
        let expected_result = -2.0;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn neg_test_4() {
        let test = "----sqrt(4)";
        let expected_result = 2.0;
        let actual_result = calculator::calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn neg_sqrt_test() {
        let test = "sqrt(-4)";
        let actual_result = calculator::calculate(test).unwrap();
        assert!(actual_result.is_nan());
    }

    // ========== Additional Expression Tests ==========

    #[test]
    fn complex_arithmetic_1() {
        let test = "5 + 3 * 2 - 8 / 4";
        assert_eq!(calculator::calculate(test).unwrap(), 9.0);
    }

    #[test]
    fn complex_arithmetic_2() {
        let test = "((15 + 5) * 2 - 10) / 5";
        assert_eq!(calculator::calculate(test).unwrap(), 6.0);
    }

    #[test]
    fn complex_arithmetic_3() {
        let test = "100 - 25 * 2 + 10 / 2";
        assert_eq!(calculator::calculate(test).unwrap(), 55.0);
    }

    #[test]
    fn power_chain_1() {
        let test = "3^2^2";
        assert_eq!(calculator::calculate(test).unwrap(), 81.0);
    }

    #[test]
    fn power_chain_2() {
        let test = "(3^2)^2";
        assert_eq!(calculator::calculate(test).unwrap(), 81.0);
    }

    #[test]
    fn power_negative() {
        let test = "2^-3";
        assert_eq!(calculator::calculate(test).unwrap(), 0.125);
    }

    #[test]
    fn power_fractional() {
        let test = "16^0.5";
        assert_eq!(calculator::calculate(test).unwrap(), 4.0);
    }

    #[test]
    fn nested_parens_1() {
        let test = "(((2 + 3) * 4) - 5) + 1";
        assert_eq!(calculator::calculate(test).unwrap(), 16.0);
    }

    #[test]
    fn nested_parens_2() {
        let test = "((10 - (3 + 2)) * 2) + 1";
        assert_eq!(calculator::calculate(test).unwrap(), 11.0);
    }

    #[test]
    fn mixed_operations_1() {
        let test = "sin(0) + cos(0) + tan(0)";
        assert!(is_close(calculator::calculate(test).unwrap(), 1.0));
    }

    #[test]
    fn mixed_operations_2() {
        let test = "sqrt(16) + abs(-5) - 3";
        assert_eq!(calculator::calculate(test).unwrap(), 6.0);
    }

    #[test]
    fn trig_nested_1() {
        let test = "sin(cos(0))";
        let result = calculator::calculate(test).unwrap();
        assert!(is_close(result, 0.8414709848));
    }

    #[test]
    fn trig_nested_2() {
        let test = "cos(sin(0))";
        assert_eq!(calculator::calculate(test).unwrap(), 1.0);
    }

    #[test]
    fn trig_with_arithmetic() {
        let test = "2 * sin(π / 2) + 3";
        assert!(is_close(calculator::calculate(test).unwrap(), 5.0));
    }

    #[test]
    fn log_operations_1() {
        let test = "ln(e) + ln(e^2)";
        assert!(is_close(calculator::calculate(test).unwrap(), 3.0));
    }

    #[test]
    fn log_operations_2() {
        let test = "log_10(100) + log_10(1000)";
        assert!(is_close(calculator::calculate(test).unwrap(), 5.0));
    }

    #[test]
    fn log_operations_3() {
        let test = "log_2(8) * log_2(4)";
        assert!(is_close(calculator::calculate(test).unwrap(), 6.0));
    }

    #[test]
    fn constants_e_and_pi() {
        let test = "e * π";
        let result = calculator::calculate(test).unwrap();
        assert!(is_close(result, E * PI));
    }

    #[test]
    fn constants_arithmetic() {
        let test = "π^2 + e^2";
        let result = calculator::calculate(test).unwrap();
        assert!(is_close(result, PI * PI + E * E));
    }

    #[test]
    fn division_chain() {
        let test = "100 / 5 / 2";
        assert_eq!(calculator::calculate(test).unwrap(), 10.0);
    }

    #[test]
    fn multiplication_chain() {
        let test = "2 * 3 * 4 * 5";
        assert_eq!(calculator::calculate(test).unwrap(), 120.0);
    }

    #[test]
    fn mixed_trig_1() {
        let test = "sin(π/6) + cos(π/3)";
        let result = calculator::calculate(test).unwrap();
        assert!(is_close(result, 1.0));
    }

    #[test]
    fn mixed_trig_2() {
        let test = "tan(π/4) * 2";
        let result = calculator::calculate(test).unwrap();
        assert!(is_close(result, 2.0));
    }

    #[test]
    fn abs_nested() {
        let test = "abs(abs(-5) - 10)";
        assert_eq!(calculator::calculate(test).unwrap(), 5.0);
    }

    #[test]
    fn sqrt_chain() {
        let test = "sqrt(sqrt(256))";
        assert_eq!(calculator::calculate(test).unwrap(), 4.0);
    }

    #[test]
    fn sqrt_with_operations() {
        let test = "sqrt(9) + sqrt(16) - sqrt(25)";
        assert_eq!(calculator::calculate(test).unwrap(), 2.0);
    }

    #[test]
    fn modulo_operations() {
        let test = "17 %% 5";
        assert_eq!(calculator::calculate(test).unwrap(), 2.0);
    }

    #[test]
    fn modulo_with_arithmetic() {
        let test = "(20 %% 7) + 3";
        assert_eq!(calculator::calculate(test).unwrap(), 9.0);
    }

    #[test]
    fn factorial_with_multiplication() {
        let test = "5! * 2";
        assert_eq!(calculator::calculate(test).unwrap(), 240.0); // 5! * 2 = 120 * 2
    }

    #[test]
    fn large_expression_1() {
        let test = "((5 + 3) * 2^3 - 10) / 2 + sqrt(144)";
        assert_eq!(calculator::calculate(test).unwrap(), 39.0);
    }

    #[test]
    fn large_expression_2() {
        let test = "abs(-10) + sqrt(100) - ln(e) * 5 + 2^3";
        assert_eq!(calculator::calculate(test).unwrap(), 23.0);
    }

    #[test]
    fn large_expression_3() {
        let test = "cos(0) * 100 + sin(0) * 50 - tan(0) * 25";
        assert!(is_close(calculator::calculate(test).unwrap(), 100.0));
    }

    #[test]
    fn precedence_test_1() {
        let test = "2 + 3 * 4^2";
        assert_eq!(calculator::calculate(test).unwrap(), 50.0);
    }

    #[test]
    fn precedence_test_2() {
        let test = "10 - 6 / 2 + 1";
        assert_eq!(calculator::calculate(test).unwrap(), 8.0);
    }

    #[test]
    fn precedence_test_3() {
        let test = "5 * 2^3 + 10 / 2";
        assert_eq!(calculator::calculate(test).unwrap(), 45.0);
    }

    #[test]
    fn inverse_trig_1() {
        let test = "asin(0.5)";
        let result = calculator::calculate(test).unwrap();
        assert!(is_close(result, PI / 6.0));
    }

    #[test]
    fn inverse_trig_2() {
        let test = "acos(0.5)";
        let result = calculator::calculate(test).unwrap();
        assert!(is_close(result, PI / 3.0));
    }

    #[test]
    fn inverse_trig_3() {
        let test = "atan(1)";
        let result = calculator::calculate(test).unwrap();
        assert!(is_close(result, PI / 4.0));
    }

    #[test]
    fn combined_functions_1() {
        let test = "sqrt(abs(-16)) + ln(e^3)";
        assert!(is_close(calculator::calculate(test).unwrap(), 7.0));
    }

    #[test]
    fn combined_functions_2() {
        let test = "log_2(16) * sqrt(9) - abs(-2)";
        assert_eq!(calculator::calculate(test).unwrap(), 10.0);
    }

    #[test]
    fn decimal_operations_1() {
        let test = "3.5 + 2.7 - 1.2";
        assert!(is_close(calculator::calculate(test).unwrap(), 5.0));
    }

    #[test]
    fn decimal_operations_2() {
        let test = "10.5 * 2.0 / 3.0";
        assert_eq!(calculator::calculate(test).unwrap(), 7.0);
    }

    #[test]
    fn decimal_power() {
        let test = "2.5^2";
        assert_eq!(calculator::calculate(test).unwrap(), 6.25);
    }

    #[test]
    fn negative_operations_1() {
        let test = "-5 + -3";
        assert_eq!(calculator::calculate(test).unwrap(), -8.0);
    }

    #[test]
    fn negative_operations_2() {
        let test = "-10 * -2";
        assert_eq!(calculator::calculate(test).unwrap(), 20.0);
    }

    #[test]
    fn negative_operations_3() {
        let test = "(-4)^2";
        assert_eq!(calculator::calculate(test).unwrap(), 16.0);
    }

    #[test]
    fn complex_nested_1() {
        let test = "sqrt(log_2(64) + ln(e^2))";
        let result = calculator::calculate(test).unwrap();
        assert!((result - 2.828427).abs() < 0.001);
    }

    #[test]
    fn complex_nested_2() {
        let test = "abs(sin(π) - cos(0))";
        let result = calculator::calculate(test).unwrap();
        assert!((result - 1.0).abs() < 0.001);
    }

    #[test]
    fn zero_operations() {
        let test = "0 * 1000 + 0 / 5 + 0^10";
        assert_eq!(calculator::calculate(test).unwrap(), 0.0);
    }

    #[test]
    fn one_operations() {
        let test = "1^100 * 1 / 1 + 1 - 1";
        assert_eq!(calculator::calculate(test).unwrap(), 1.0);
    }

    // ========== Edge Case Tests ==========

    #[test]
    fn division_by_zero() {
        let test = "5 / 0";
        let result = calculator::calculate(test).unwrap();
        assert!(result.is_infinite(), "Should return infinity");
    }

    #[test]
    fn zero_divided_by_zero() {
        let test = "0 / 0";
        let result = calculator::calculate(test).unwrap();
        assert!(result.is_nan());
    }

    #[test]
    fn very_large_number() {
        let test = "999999999999.0 * 999999999999.0";
        let result = calculator::calculate(test).unwrap();
        assert!(result.is_finite() || result.is_infinite());
    }

    #[test]
    fn very_small_positive() {
        let test = "0.0000001 * 0.0000001";
        let result = calculator::calculate(test).unwrap();
        assert!(result >= 0.0 && result < 0.001);
    }

    #[test]
    fn deeply_nested_parens() {
        let test = "((((((1 + 1))))))";
        assert_eq!(calculator::calculate(test).unwrap(), 2.0);
    }

    #[test]
    fn many_negatives() {
        let test = "------5";
        assert_eq!(calculator::calculate(test).unwrap(), 5.0);
    }

    #[test]
    fn negative_zero() {
        let test = "-0";
        assert_eq!(calculator::calculate(test).unwrap(), 0.0);
    }

    #[test]
    fn sqrt_of_zero() {
        let test = "sqrt(0)";
        assert_eq!(calculator::calculate(test).unwrap(), 0.0);
    }

    #[test]
    fn log_of_one() {
        let test = "ln(1)";
        assert_eq!(calculator::calculate(test).unwrap(), 0.0);
    }

    #[test]
    fn log_base_10_of_one() {
        let test = "log_10(1)";
        assert_eq!(calculator::calculate(test).unwrap(), 0.0);
    }

    #[test]
    fn zero_to_zero_power() {
        let test = "0^0";
        assert_eq!(calculator::calculate(test).unwrap(), 1.0);
    }

    #[test]
    fn zero_to_negative_power() {
        let test = "0^-1";
        let result = calculator::calculate(test).unwrap();
        assert!(result.is_infinite(), "Should return infinity");
    }

    #[test]
    fn negative_to_fractional_power() {
        let test = "(-4)^0.5";
        let result = calculator::calculate(test).unwrap();
        assert!(result.is_nan());
    }

    #[test]
    fn lots_of_whitespace() {
        let test = "   2    +    3    *    4   ";
        assert_eq!(calculator::calculate(test).unwrap(), 14.0);
    }

    #[test]
    fn no_whitespace() {
        let test = "2+3*4-1/2";
        assert_eq!(calculator::calculate(test).unwrap(), 13.5);
    }

    #[test]
    fn multiple_decimal_operations() {
        let test = "1.5 + 2.5 - 0.5 * 0.5";
        assert_eq!(calculator::calculate(test).unwrap(), 3.75);
    }

    #[test]
    fn chained_subtractions() {
        let test = "10 - 5 - 3 - 1";
        assert_eq!(calculator::calculate(test).unwrap(), 1.0);
    }

    #[test]
    fn chained_divisions() {
        let test = "100 / 10 / 2";
        assert_eq!(calculator::calculate(test).unwrap(), 5.0);
    }

    #[test]
    fn operations_on_e() {
        let test = "e^1 - e + e";
        assert!(is_close(calculator::calculate(test).unwrap(), E));
    }

    #[test]
    fn operations_on_pi() {
        let test = "π * 2 / 2";
        assert!(is_close(calculator::calculate(test).unwrap(), PI));
    }

    #[test]
    fn sin_of_zero() {
        let test = "sin(0)";
        assert!(is_close(calculator::calculate(test).unwrap(), 0.0));
    }

    #[test]
    fn cos_of_zero() {
        let test = "cos(0)";
        assert_eq!(calculator::calculate(test).unwrap(), 1.0);
    }

    #[test]
    fn tan_of_zero() {
        let test = "tan(0)";
        assert!(is_close(calculator::calculate(test).unwrap(), 0.0));
    }

    #[test]
    fn abs_of_zero() {
        let test = "abs(0)";
        assert_eq!(calculator::calculate(test).unwrap(), 0.0);
    }

    #[test]
    fn ln_of_e() {
        let test = "ln(e)";
        assert!(is_close(calculator::calculate(test).unwrap(), 1.0));
    }

    #[test]
    fn log_base_of_itself() {
        let test = "log_5(5)";
        assert!(is_close(calculator::calculate(test).unwrap(), 1.0));
    }

    #[test]
    fn negative_abs() {
        let test = "abs(-1000000)";
        assert_eq!(calculator::calculate(test).unwrap(), 1000000.0);
    }

    #[test]
    fn double_abs() {
        let test = "abs(abs(-5))";
        assert_eq!(calculator::calculate(test).unwrap(), 5.0);
    }

    #[test]
    fn min_of_negatives() {
        let test = "min(-5, -10, -3)";
        assert_eq!(calculator::calculate(test).unwrap(), -10.0);
    }

    #[test]
    fn max_of_negatives() {
        let test = "max(-5, -10, -3)";
        assert_eq!(calculator::calculate(test).unwrap(), -3.0);
    }

    #[test]
    fn avg_of_same_number() {
        let test = "avg(5, 5, 5, 5)";
        assert_eq!(calculator::calculate(test).unwrap(), 5.0);
    }

    #[test]
    fn single_number_expression() {
        let test = "42";
        assert_eq!(calculator::calculate(test).unwrap(), 42.0);
    }

    #[test]
    fn single_constant() {
        let test = "π";
        assert!(is_close(calculator::calculate(test).unwrap(), PI));
    }

    #[test]
    fn just_e() {
        let test = "e";
        assert!(is_close(calculator::calculate(test).unwrap(), E));
    }

    #[test]
    fn parentheses_around_single_number() {
        let test = "(((42)))";
        assert_eq!(calculator::calculate(test).unwrap(), 42.0);
    }

    #[test]
    fn long_chain_addition() {
        let test = "1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1";
        assert_eq!(calculator::calculate(test).unwrap(), 10.0);
    }

    #[test]
    fn alternating_signs() {
        let test = "10 - 5 + 3 - 2 + 1";
        assert_eq!(calculator::calculate(test).unwrap(), 7.0);
    }

    #[test]
    fn power_of_one() {
        let test = "999^1";
        assert_eq!(calculator::calculate(test).unwrap(), 999.0);
    }

    #[test]
    fn one_to_any_power() {
        let test = "1^999";
        assert_eq!(calculator::calculate(test).unwrap(), 1.0);
    }

    #[test]
    fn negative_one_to_even_power() {
        let test = "(-1)^2";
        assert_eq!(calculator::calculate(test).unwrap(), 1.0);
    }

    #[test]
    fn negative_one_to_odd_power() {
        let test = "(-1)^3";
        assert_eq!(calculator::calculate(test).unwrap(), -1.0);
    }

    #[test]
    fn sqrt_of_one() {
        let test = "sqrt(1)";
        assert_eq!(calculator::calculate(test).unwrap(), 1.0);
    }

    #[test]
    fn sqrt_of_very_small_1() {
        let test = "sqrt(0.01)";
        let result = calculator::calculate(test).unwrap();
        let expected = 0.1;
        assert!(
            (result - expected).abs() < 0.001,
            "sqrt(0.01) = {}, expected ~{}",
            result,
            expected
        );
    }

    #[test]
    fn sqrt_of_very_small_2() {
        let test = "sqrt(0.0001)";
        let result = calculator::calculate(test).unwrap();
        let expected = 0.01;
        assert!(
            (result - expected).abs() < 0.001,
            "sqrt(0.0001) = {}, expected ~{}",
            result,
            expected
        );
    }

    #[test]
    fn sqrt_of_very_small_3() {
        let test = "sqrt(0.000001)";
        let result = calculator::calculate(test).unwrap();
        let expected = 0.001;
        assert!(
            (result - expected).abs() < 0.001,
            "sqrt(0.000001) = {}, expected ~{}",
            result,
            expected
        );
    }

    #[test]
    fn sqrt_of_very_small_4() {
        let test = "sqrt(0.00000001)";
        let result = calculator::calculate(test).unwrap();
        let expected = 0.0001;
        assert!(
            (result - expected).abs() < 0.001,
            "sqrt(0.00000001) = {}, expected ~{}",
            result,
            expected
        );
    }

    #[test]
    fn sqrt_of_very_small_5() {
        let test = "sqrt(0.25)";
        let result = calculator::calculate(test).unwrap();
        let expected = 0.5;
        assert!(
            (result - expected).abs() < 0.001,
            "sqrt(0.25) = {}, expected ~{}",
            result,
            expected
        );
    }

    #[test]
    fn sqrt_of_2() {
        let test = "sqrt(2)";
        let result = calculator::calculate(test).unwrap();
        let expected = 1.41421356;
        assert!(
            (result - expected).abs() < 0.001,
            "sqrt(2) = {}, expected ~{}",
            result,
            expected
        );
    }

    #[test]
    fn sqrt_of_4() {
        let test = "sqrt(4)";
        assert_eq!(calculator::calculate(test).unwrap(), 2.0);
    }

    #[test]
    fn sqrt_of_9() {
        let test = "sqrt(9)";
        assert_eq!(calculator::calculate(test).unwrap(), 3.0);
    }

    #[test]
    fn sqrt_of_16() {
        let test = "sqrt(16)";
        assert_eq!(calculator::calculate(test).unwrap(), 4.0);
    }

    #[test]
    fn sqrt_of_50() {
        let test = "sqrt(50)";
        let result = calculator::calculate(test).unwrap();
        let expected = 7.071067812;
        assert!(
            (result - expected).abs() < 0.001,
            "sqrt(50) = {}, expected ~{}",
            result,
            expected
        );
    }

    #[test]
    fn sqrt_of_100() {
        let test = "sqrt(100)";
        assert_eq!(calculator::calculate(test).unwrap(), 10.0);
    }

    #[test]
    fn sqrt_of_123() {
        let test = "sqrt(123)";
        let result = calculator::calculate(test).unwrap();
        let expected = 11.09053651;
        assert!(
            (result - expected).abs() < 0.01,
            "sqrt(123) = {}, expected ~{}",
            result,
            expected
        );
    }

    #[test]
    fn sqrt_of_625() {
        let test = "sqrt(625)";
        assert_eq!(calculator::calculate(test).unwrap(), 25.0);
    }

    #[test]
    fn sqrt_of_10000() {
        let test = "sqrt(10000)";
        assert_eq!(calculator::calculate(test).unwrap(), 100.0);
    }

    #[test]
    fn sqrt_of_999999() {
        let test = "sqrt(999999)";
        let result = calculator::calculate(test).unwrap();
        let expected = 999.9995;
        assert!(
            (result - expected).abs() < 0.1,
            "sqrt(999999) = {}, expected ~{}",
            result,
            expected
        );
    }

    #[test]
    fn sqrt_of_1000000() {
        let test = "sqrt(1000000)";
        assert_eq!(calculator::calculate(test).unwrap(), 1000.0);
    }

    #[test]
    fn sqrt_of_very_large_1() {
        let test = "sqrt(10000000000)";
        let result = calculator::calculate(test).unwrap();
        let expected = 100000.0;
        assert!(
            (result - expected).abs() < 1.0,
            "sqrt(1e10) = {}, expected ~{}",
            result,
            expected
        );
    }

    #[test]
    fn sqrt_of_very_large_2() {
        let test = "sqrt(1000000000000000000)";
        let result = calculator::calculate(test).unwrap();
        let expected = 1000000000.0;
        assert!(
            (result - expected).abs() < 1000.0,
            "sqrt(1e18) = {}, expected ~{}",
            result,
            expected
        );
    }

    #[test]
    fn modulo_by_one() {
        let test = "5 %% 1";
        assert_eq!(calculator::calculate(test).unwrap(), 0.0);
    }

    #[test]
    fn modulo_zero() {
        let test = "0 %% 5";
        assert_eq!(calculator::calculate(test).unwrap(), 0.0);
    }

    // ========== Factorial Edge Cases ==========

    #[test]
    fn factorial_of_zero() {
        let test = "0!";
        assert_eq!(calculator::calculate(test).unwrap(), 1.0);
    }

    #[test]
    fn factorial_of_negative() {
        let test = "(-5)!";
        let result = calculator::calculate(test);
        assert!(result.is_err(), "Factorial of negative should error");
    }

    #[test]
    fn factorial_of_non_integer() {
        let test = "5.5!";
        let result = calculator::calculate(test);
        assert!(result.is_err(), "Factorial of non-integer should error");
    }

    #[test]
    fn factorial_large() {
        let test = "20!";
        let result = calculator::calculate(test).unwrap();
        let expected = 2432902008176640000.0;
        assert!(
            (result - expected).abs() / expected < 0.01,
            "20! = {}, expected ~{}",
            result,
            expected
        );
    }

    // ========== Operator Precedence & Associativity ==========

    #[test]
    fn unary_minus_vs_power() {
        let test = "-2^2";
        assert_eq!(calculator::calculate(test).unwrap(), -4.0);
    }

    #[test]
    fn power_right_associative() {
        let test = "2^3^2";
        assert_eq!(calculator::calculate(test).unwrap(), 512.0); // 2^(3^2) = 2^9 = 512
    }

    #[test]
    fn double_negative() {
        let test = "--5";
        assert_eq!(calculator::calculate(test).unwrap(), 5.0);
    }

    #[test]
    fn triple_negative() {
        let test = "---5";
        assert_eq!(calculator::calculate(test).unwrap(), -5.0);
    }

    // ========== Trigonometric Precision ==========

    #[test]
    fn sin_of_pi() {
        let test = "sin(π)";
        let result = calculator::calculate(test).unwrap();
        assert!(result.abs() < 0.0001, "sin(π) should be ≈0, got {}", result);
    }

    #[test]
    fn cos_of_pi_over_2() {
        let test = "cos(π/2)";
        let result = calculator::calculate(test).unwrap();
        assert!(
            result.abs() < 0.0001,
            "cos(π/2) should be ≈0, got {}",
            result
        );
    }

    #[test]
    fn tan_of_pi_over_2() {
        let test = "tan(π/2)";
        let result = calculator::calculate(test).unwrap();
        assert!(
            result.abs() > 1000.0 || result.is_infinite(),
            "tan(π/2) should be very large or infinite, got {}",
            result
        );
    }

    #[test]
    fn tan_of_pi_over_4() {
        let test = "tan(π/4)";
        let result = calculator::calculate(test).unwrap();
        assert!(
            (result - 1.0).abs() < 0.001,
            "tan(π/4) should be 1, got {}",
            result
        );
    }

    // ========== Logarithm Edge Cases ==========

    #[test]
    fn log_2_of_2() {
        let test = "log_2(2)";
        assert_eq!(calculator::calculate(test).unwrap(), 1.0);
    }

    #[test]
    fn log_10_of_10() {
        let test = "log_10(10)";
        assert_eq!(calculator::calculate(test).unwrap(), 1.0);
    }

    #[test]
    fn ln_of_e_constant() {
        let test = "ln(e)";
        let result = calculator::calculate(test).unwrap();
        assert!(
            (result - 1.0).abs() < 0.0001,
            "ln(e) should be 1, got {}",
            result
        );
    }

    #[test]
    fn ln_of_negative() {
        let test = "ln(-5)";
        let result = calculator::calculate(test).unwrap();
        assert!(result.is_nan(), "ln(-5) should be NaN");
    }

    #[test]
    fn log_of_negative() {
        let test = "log_2(-8)";
        let result = calculator::calculate(test).unwrap();
        assert!(result.is_nan(), "log_2(-8) should be NaN");
    }

    // ========== Function Argument Counts ==========

    #[test]
    fn avg_single_argument() {
        let test = "avg(5)";
        assert_eq!(calculator::calculate(test).unwrap(), 5.0);
    }

    #[test]
    fn max_single_argument() {
        let test = "max(5)";
        assert_eq!(calculator::calculate(test).unwrap(), 5.0);
    }

    #[test]
    fn min_single_argument() {
        let test = "min(5)";
        assert_eq!(calculator::calculate(test).unwrap(), 5.0);
    }

    // ========== Malformed Expressions (Should Error) ==========

    #[test]
    fn ends_with_operator() {
        let test = "5 +";
        let result = calculator::calculate(test);
        assert!(
            result.is_err(),
            "Expression ending with operator should error"
        );
    }

    #[test]
    fn starts_with_operator() {
        let test = "+ 5";
        let result = calculator::calculate(test);
        assert!(
            result.is_err(),
            "Expression starting with operator should error"
        );
    }

    #[test]
    fn missing_operator() {
        let test = "5 5";
        let result = calculator::calculate(test);
        assert!(result.is_err(), "Missing operator should error");
    }

    #[test]
    fn empty_parentheses() {
        let test = "()";
        let result = calculator::calculate(test);
        assert!(result.is_err(), "Empty parentheses should error");
    }

    #[test]
    fn operator_with_empty_parens() {
        let test = "5 + ()";
        let result = calculator::calculate(test);
        assert!(
            result.is_err(),
            "Operator with empty parentheses should error"
        );
    }

    // ========== Constants ==========

    #[test]
    fn e_times_pi() {
        let test = "e * π";
        let result = calculator::calculate(test).unwrap();
        let expected = std::f32::consts::E * std::f32::consts::PI;
        assert!(
            (result - expected).abs() < 0.001,
            "e * π = {}, expected {}",
            result,
            expected
        );
    }

    #[test]
    fn pi_divided_by_pi() {
        let test = "π / π";
        assert_eq!(calculator::calculate(test).unwrap(), 1.0);
    }

    #[test]
    fn e_to_zero_power() {
        let test = "e^0";
        assert_eq!(calculator::calculate(test).unwrap(), 1.0);
    }

    #[test]
    fn pi_squared() {
        let test = "π^2";
        let result = calculator::calculate(test).unwrap();
        let expected = std::f32::consts::PI * std::f32::consts::PI;
        assert!(
            (result - expected).abs() < 0.001,
            "π² = {}, expected {}",
            result,
            expected
        );
    }

    // ========== DEEP NESTING TESTS ==========

    #[test]
    fn nested_2_levels_avg_min() {
        // avg(1, min(2, 3), 4) = avg(1, 2, 4) = 7/3
        let test = "avg(1, min(2, 3), 4)";
        let result = calculator::calculate(test).unwrap();
        assert!(is_close(result, 7.0 / 3.0));
    }

    #[test]
    fn nested_2_levels_max_avg() {
        // max(5, avg(1, 2, 3)) = max(5, 2) = 5
        let test = "max(5, avg(1, 2, 3))";
        assert_eq!(calculator::calculate(test).unwrap(), 5.0);
    }

    #[test]
    fn nested_3_levels_avg_max_min() {
        // avg(1, max(2, min(3, 4)), 5) = avg(1, max(2, 3), 5) = avg(1, 3, 5) = 3
        let test = "avg(1, max(2, min(3, 4)), 5)";
        assert_eq!(calculator::calculate(test).unwrap(), 3.0);
    }

    #[test]
    fn nested_3_levels_min_avg_max() {
        // min(10, avg(max(1, 2), 3, 4), 20) = min(10, avg(2, 3, 4), 20) = min(10, 3, 20) = 3
        let test = "min(10, avg(max(1, 2), 3, 4), 20)";
        assert_eq!(calculator::calculate(test).unwrap(), 3.0);
    }

    #[test]
    fn nested_4_levels_deep() {
        // max(1, min(2, avg(3, max(4, 5))))
        // = max(1, min(2, avg(3, 5)))
        // = max(1, min(2, 4))
        // = max(1, 2)
        // = 2
        let test = "max(1, min(2, avg(3, max(4, 5))))";
        assert_eq!(calculator::calculate(test).unwrap(), 2.0);
    }

    #[test]
    fn nested_4_levels_very_deep() {
        // avg(min(max(avg(1, 2), 3), 4), 5)
        // = avg(min(max(1.5, 3), 4), 5)
        // = avg(min(3, 4), 5)
        // = avg(3, 5)
        // = 4
        let test = "avg(min(max(avg(1, 2), 3), 4), 5)";
        assert_eq!(calculator::calculate(test).unwrap(), 4.0);
    }

    #[test]
    fn nested_5_levels_extreme() {
        // min(1, max(2, avg(3, min(4, max(5, 6)))))
        // = min(1, max(2, avg(3, min(4, 6))))
        // = min(1, max(2, avg(3, 4)))
        // = min(1, max(2, 3.5))
        // = min(1, 3.5)
        // = 1
        let test = "min(1, max(2, avg(3, min(4, max(5, 6)))))";
        assert_eq!(calculator::calculate(test).unwrap(), 1.0);
    }

    #[test]
    fn nested_multiple_at_same_level() {
        // avg(min(1, 2), max(3, 4), min(5, 6))
        // = avg(1, 4, 5)
        // = 10/3
        let test = "avg(min(1, 2), max(3, 4), min(5, 6))";
        let result = calculator::calculate(test).unwrap();
        assert!(is_close(result, 10.0 / 3.0));
    }

    #[test]
    fn nested_with_regular_functions() {
        // max(abs(-5), min(sqrt(16), 10))
        // = max(5, min(4, 10))
        // = max(5, 4)
        // = 5
        let test = "max(abs(-5), min(sqrt(16), 10))";
        assert_eq!(calculator::calculate(test).unwrap(), 5.0);
    }

    #[test]
    fn variadic_inside_regular_function() {
        // sin(avg(0, π/2)) = sin(π/4) ≈ 0.707
        let test = "sin(avg(0, π/2))";
        let result = calculator::calculate(test).unwrap();
        let expected = (std::f32::consts::PI / 4.0).sin();
        assert!(is_close(result, expected));
    }

    #[test]
    fn regular_function_inside_variadic() {
        // avg(sin(0), cos(0), sqrt(4))
        // = avg(0, 1, 2)
        // = 1
        let test = "avg(sin(0), cos(0), sqrt(4))";
        assert_eq!(calculator::calculate(test).unwrap(), 1.0);
    }

    #[test]
    fn deeply_nested_with_arithmetic() {
        // max(1 + 2, min(3 * 4, avg(5 - 1, 6 / 2)))
        // = max(3, min(12, avg(4, 3)))
        // = max(3, min(12, 3.5))
        // = max(3, 3.5)
        // = 3.5
        let test = "max(1 + 2, min(3 * 4, avg(5 - 1, 6 / 2)))";
        assert_eq!(calculator::calculate(test).unwrap(), 3.5);
    }

    #[test]
    fn nested_median_in_mode() {
        // mode(med(1, 2, 3), med(1, 2, 3), 5)
        // = mode(2, 2, 5)
        // = 2
        let test = "mode(med(1, 2, 3), med(1, 2, 3), 5)";
        assert_eq!(calculator::calculate(test).unwrap(), 2.0);
    }

    #[test]
    fn nested_all_statistics() {
        // max(avg(1, 2), min(3, 4), med(5, 6, 7), mode(8, 8, 9))
        // = max(1.5, 3, 6, 8)
        // = 8
        let test = "max(avg(1, 2), min(3, 4), med(5, 6, 7), mode(8, 8, 9))";
        assert_eq!(calculator::calculate(test).unwrap(), 8.0);
    }

    // ========== UNARY MINUS TESTS ==========

    #[test]
    fn unary_minus_basic() {
        assert_eq!(calculator::calculate("-5").unwrap(), -5.0);
        assert_eq!(calculator::calculate("-0").unwrap(), 0.0);
        assert_eq!(calculator::calculate("-100.5").unwrap(), -100.5);
    }

    #[test]
    fn unary_minus_double_negative() {
        // --5 = -(-5) = 5
        assert_eq!(calculator::calculate("--5").unwrap(), 5.0);
    }

    #[test]
    fn unary_minus_triple_negative() {
        // ---5 = -(-(-5)) = -5
        assert_eq!(calculator::calculate("---5").unwrap(), -5.0);
    }

    #[test]
    fn unary_minus_with_addition() {
        // -5 + 3 = -2
        assert_eq!(calculator::calculate("-5 + 3").unwrap(), -2.0);
        // 3 + -5 = -2
        assert_eq!(calculator::calculate("3 + -5").unwrap(), -2.0);
    }

    #[test]
    fn unary_minus_with_subtraction() {
        // -5 - 3 = -8
        assert_eq!(calculator::calculate("-5 - 3").unwrap(), -8.0);
        // 3 - -5 = 8
        assert_eq!(calculator::calculate("3 - -5").unwrap(), 8.0);
    }

    #[test]
    fn unary_minus_with_multiplication() {
        // -5 * 3 = -15
        assert_eq!(calculator::calculate("-5 * 3").unwrap(), -15.0);
        // 5 * -3 = -15
        assert_eq!(calculator::calculate("5 * -3").unwrap(), -15.0);
        // -5 * -3 = 15
        assert_eq!(calculator::calculate("-5 * -3").unwrap(), 15.0);
    }

    #[test]
    fn unary_minus_with_division() {
        // -10 / 2 = -5
        assert_eq!(calculator::calculate("-10 / 2").unwrap(), -5.0);
        // 10 / -2 = -5
        assert_eq!(calculator::calculate("10 / -2").unwrap(), -5.0);
        // -10 / -2 = 5
        assert_eq!(calculator::calculate("-10 / -2").unwrap(), 5.0);
    }

    #[test]
    fn unary_minus_power_precedence() {
        // -2^2 should be -(2^2) = -4, not (-2)^2 = 4
        assert_eq!(calculator::calculate("-2^2").unwrap(), -4.0);
        // -2^3 = -(2^3) = -8
        assert_eq!(calculator::calculate("-2^3").unwrap(), -8.0);
        // -3^2 = -(3^2) = -9
        assert_eq!(calculator::calculate("-3^2").unwrap(), -9.0);
    }

    #[test]
    fn unary_minus_power_with_parens() {
        // (-2)^2 = 4
        assert_eq!(calculator::calculate("(-2)^2").unwrap(), 4.0);
        // (-2)^3 = -8
        assert_eq!(calculator::calculate("(-2)^3").unwrap(), -8.0);
        // (-3)^2 = 9
        assert_eq!(calculator::calculate("(-3)^2").unwrap(), 9.0);
    }

    #[test]
    fn unary_minus_in_exponent() {
        // 2^-1 = 0.5
        assert_eq!(calculator::calculate("2^-1").unwrap(), 0.5);
        // 2^-2 = 0.25
        assert_eq!(calculator::calculate("2^-2").unwrap(), 0.25);
        // 2^-3 = 0.125
        assert_eq!(calculator::calculate("2^-3").unwrap(), 0.125);
        // 10^-1 = 0.1
        assert_eq!(calculator::calculate("10^-1").unwrap(), 0.1);
    }

    #[test]
    fn unary_minus_complex_exponent() {
        // 2^-2^2 = 2^(-4) = 0.0625
        let result = calculator::calculate("2^-2^2").unwrap();
        assert!(is_close(result, 0.0625));
    }

    #[test]
    fn unary_minus_with_parens() {
        // -(5 + 3) = -8
        assert_eq!(calculator::calculate("-(5 + 3)").unwrap(), -8.0);
        // -(10 - 3) = -7
        assert_eq!(calculator::calculate("-(10 - 3)").unwrap(), -7.0);
        // -(2 * 3) = -6
        assert_eq!(calculator::calculate("-(2 * 3)").unwrap(), -6.0);
    }

    #[test]
    fn unary_minus_with_functions() {
        // -sin(π/2) = -1
        let result = calculator::calculate("-sin(π/2)").unwrap();
        assert!(is_close(result, -1.0));

        // -abs(-5) = -5
        assert_eq!(calculator::calculate("-abs(-5)").unwrap(), -5.0);

        // -sqrt(16) = -4
        assert_eq!(calculator::calculate("-sqrt(16)").unwrap(), -4.0);
    }

    #[test]
    fn unary_minus_in_variadic_functions() {
        // min(-1, -2, -3) = -3
        assert_eq!(calculator::calculate("min(-1, -2, -3)").unwrap(), -3.0);

        // max(-1, -2, -3) = -1
        assert_eq!(calculator::calculate("max(-1, -2, -3)").unwrap(), -1.0);

        // avg(-1, -2, -3) = -2
        assert_eq!(calculator::calculate("avg(-1, -2, -3)").unwrap(), -2.0);
    }

    #[test]
    fn unary_minus_mixed_in_variadic() {
        // avg(-5, 5, -3, 3) = 0
        assert_eq!(calculator::calculate("avg(-5, 5, -3, 3)").unwrap(), 0.0);

        // min(-10, 5, -3, 8) = -10
        assert_eq!(calculator::calculate("min(-10, 5, -3, 8)").unwrap(), -10.0);

        // max(-10, 5, -3, 8) = 8
        assert_eq!(calculator::calculate("max(-10, 5, -3, 8)").unwrap(), 8.0);
    }

    #[test]
    fn unary_minus_with_expressions_in_variadic() {
        // avg(-2 * 3, -4 + 1, -10 / 2)
        // = avg(-6, -3, -5)
        // = -14/3
        let test = "avg(-2 * 3, -4 + 1, -10 / 2)";
        let result = calculator::calculate(test).unwrap();
        assert!(is_close(result, -14.0 / 3.0));
    }

    #[test]
    fn unary_minus_nested_in_variadic() {
        // max(-min(1, 2), -min(3, 4))
        // = max(-1, -3)
        // = -1
        assert_eq!(
            calculator::calculate("max(-min(1, 2), -min(3, 4))").unwrap(),
            -1.0
        );

        // min(-max(1, 2), -max(3, 4))
        // = min(-2, -4)
        // = -4
        assert_eq!(
            calculator::calculate("min(-max(1, 2), -max(3, 4))").unwrap(),
            -4.0
        );
    }

    #[test]
    fn unary_minus_deeply_nested() {
        // -avg(-min(-max(-1, -2), -3), -4)
        // = -avg(-min(-(-1), -3), -4)
        // = -avg(-min(1, -3), -4)
        // = -avg(-(-3), -4)
        // = -avg(3, -4)
        // = -(-0.5)
        // = 0.5
        let test = "-avg(-min(-max(-1, -2), -3), -4)";
        assert_eq!(calculator::calculate(test).unwrap(), 0.5);
    }

    #[test]
    fn unary_minus_with_constants() {
        // -π ≈ -3.14159
        let result = calculator::calculate("-π").unwrap();
        assert!(is_close(result, -std::f32::consts::PI));

        // -e ≈ -2.71828
        let result = calculator::calculate("-e").unwrap();
        assert!(is_close(result, -std::f32::consts::E));

        // -π + -e ≈ -5.85987
        let result = calculator::calculate("-π + -e").unwrap();
        let expected = -std::f32::consts::PI + -std::f32::consts::E;
        assert!(is_close(result, expected));
    }

    #[test]
    fn unary_minus_factorial() {
        // -5! = -120
        assert_eq!(calculator::calculate("-5!").unwrap(), -120.0);

        // (-2)! should error (factorial of negative)
        assert!(calculator::calculate("(-2)!").is_err());
    }

    #[test]
    fn unary_minus_modulo() {
        // -7 %% 3 = -1
        assert_eq!(calculator::calculate("-7 %% 3").unwrap(), -1.0);

        // 7 %% -3 = 1
        assert_eq!(calculator::calculate("7 %% -3").unwrap(), 1.0);

        // -7 %% -3 = -1
        assert_eq!(calculator::calculate("-7 %% -3").unwrap(), -1.0);
    }

    #[test]
    fn combined_nested_and_unary() {
        // max(-abs(-5), min(-2^2, avg(-1, -2, -3)))
        // = max(-5, min(-4, -2))
        // = max(-5, -4)
        // = -4
        let test = "max(-abs(-5), min(-2^2, avg(-1, -2, -3)))";
        assert_eq!(calculator::calculate(test).unwrap(), -4.0);
    }

    #[test]
    fn combined_extreme_nesting_and_unary() {
        // -min(-max(-avg(-1, -min(-2, -3)), -4), -5)
        // Working from inside out:
        // -min(-2, -3) = -(-3) = 3
        // -avg(-1, 3) = -avg(-1, 3) = -1
        // -max(-1, -4) = -(-1) = 1
        // -min(1, -5) = -(-5) = 5
        let test = "-min(-max(-avg(-1, -min(-2, -3)), -4), -5)";
        assert_eq!(calculator::calculate(test).unwrap(), 5.0);
    }

    #[test]
    fn nested_with_all_operators_and_unary() {
        // avg((-2)^2, -3 * 4, 10 / -2, -5 + 3, 6 - -2)
        // = avg(4, -12, -5, -2, 8)
        // = -7/5 = -1.4
        let test = "avg((-2)^2, -3 * 4, 10 / -2, -5 + 3, 6 - -2)";
        assert_eq!(calculator::calculate(test).unwrap(), -1.4);
    }

    #[test]
    fn six_levels_deep_nested() {
        // avg(1, min(2, max(3, avg(4, min(5, max(6, 7))))))
        // = avg(1, min(2, max(3, avg(4, min(5, 7)))))
        // = avg(1, min(2, max(3, avg(4, 5))))
        // = avg(1, min(2, max(3, 4.5)))
        // = avg(1, min(2, 4.5))
        // = avg(1, 2)
        // = 1.5
        let test = "avg(1, min(2, max(3, avg(4, min(5, max(6, 7))))))";
        assert_eq!(calculator::calculate(test).unwrap(), 1.5);
    }

    #[test]
    fn nested_with_median_and_mode() {
        // max(med(1, 2, 3, 4, 5), mode(6, 6, 7), avg(8, 9))
        // = max(3, 6, 8.5)
        // = 8.5
        let test = "max(med(1, 2, 3, 4, 5), mode(6, 6, 7), avg(8, 9))";
        assert_eq!(calculator::calculate(test).unwrap(), 8.5);
    }

    #[test]
    fn nested_with_choice() {
        // This uses the binomial coefficient function
        // avg(ch(5, 2), ch(4, 2))
        // = avg(10, 6)
        // = 8
        let test = "avg(ch(5, 2), ch(4, 2))";
        assert_eq!(calculator::calculate(test).unwrap(), 8.0);
    }

    // ========== 50 COMPREHENSIVE STRESS TESTS ==========

    // === Operator Precedence & Associativity (10 tests) ===

    #[test]
    fn stress_precedence_factorial_over_power() {
        // 2^3! should be 2^6 = 64, not (2^3)! = 8!
        assert_eq!(calculator::calculate("2^3!").unwrap(), 64.0);
    }

    #[test]
    fn stress_precedence_factorial_over_multiply() {
        // 2 * 3! should be 2 * 6 = 12
        assert_eq!(calculator::calculate("2 * 3!").unwrap(), 12.0);
    }

    #[test]
    fn stress_left_associativity_subtraction_chain() {
        // 100 - 20 - 10 - 5 should be ((100 - 20) - 10) - 5 = 65
        assert_eq!(calculator::calculate("100 - 20 - 10 - 5").unwrap(), 65.0);
    }

    #[test]
    fn stress_left_associativity_division_chain() {
        // 1000 / 10 / 5 / 2 should be (((1000 / 10) / 5) / 2) = 10
        assert_eq!(calculator::calculate("1000 / 10 / 5 / 2").unwrap(), 10.0);
    }

    #[test]
    fn stress_right_associativity_triple_power() {
        // 2^2^3 should be 2^(2^3) = 2^8 = 256, not (2^2)^3 = 64
        assert_eq!(calculator::calculate("2^2^3").unwrap(), 256.0);
    }

    #[test]
    fn stress_modulo_multiply_same_level() {
        // 17 %% 5 * 3 should be (17 %% 5) * 3 = 2 * 3 = 6
        assert_eq!(calculator::calculate("17 %% 5 * 3").unwrap(), 6.0);
    }

    #[test]
    fn stress_percent_division_same_level() {
        // 50 % 20 / 2 should be (50 % 20) / 2 = 10 / 2 = 5
        assert_eq!(calculator::calculate("50 % 20 / 2").unwrap(), 5.0);
    }

    #[test]
    fn stress_complex_precedence_tower() {
        // 1 + 2^3! * 4 - 5 / (2 + 3)
        // = 1 + 2^6 * 4 - 5 / 5
        // = 1 + 64 * 4 - 1
        // = 1 + 256 - 1
        // = 256
        assert_eq!(
            calculator::calculate("1 + 2^3! * 4 - 5 / (2 + 3)").unwrap(),
            256.0
        );
    }

    #[test]
    fn stress_unary_minus_factorial_power() {
        // -2^3! = -(2^6) = -64
        assert_eq!(calculator::calculate("-2^3!").unwrap(), -64.0);
    }

    #[test]
    fn stress_quad_negative() {
        // ----10 = 10
        assert_eq!(calculator::calculate("----10").unwrap(), 10.0);
    }

    // === Deeply Nested Functions (10 tests) ===

    #[test]
    fn stress_seven_level_nesting() {
        // max(1, min(2, avg(3, max(4, min(5, avg(6, max(7, 8)))))))
        // Working from inside: max(7,8)=8, avg(6,8)=7, min(5,7)=5, max(4,5)=5,
        // avg(3,5)=4, min(2,4)=2, max(1,2)=2
        let test = "max(1, min(2, avg(3, max(4, min(5, avg(6, max(7, 8)))))))";
        assert_eq!(calculator::calculate(test).unwrap(), 2.0);
    }

    #[test]
    fn stress_trig_tower() {
        // sin(cos(sin(cos(0))))
        // cos(0)=1, sin(1)≈0.8414, cos(0.8414)≈0.6663, sin(0.6663)≈0.6186
        let test = "sin(cos(sin(cos(0))))";
        let result = calculator::calculate(test).unwrap();
        let expected = 0_f32.cos().sin().cos().sin();
        assert!(is_close(result, expected));
    }

    #[test]
    fn stress_nested_abs_chains() {
        // abs(abs(abs(abs(abs(-42)))))
        assert_eq!(
            calculator::calculate("abs(abs(abs(abs(abs(-42)))))").unwrap(),
            42.0
        );
    }

    #[test]
    fn stress_nested_sqrt_chain() {
        // sqrt(sqrt(sqrt(sqrt(256))))
        // = sqrt(sqrt(sqrt(16))) = sqrt(sqrt(4)) = sqrt(2) ≈ 1.414
        let result = calculator::calculate("sqrt(sqrt(sqrt(sqrt(256))))").unwrap();
        assert!((result - 1.414).abs() < 0.01);
    }

    #[test]
    fn stress_variadic_pyramid() {
        // avg(min(1,2), avg(3,4), max(5,6), min(7,8), avg(9,10))
        // = avg(1, 3.5, 6, 7, 9.5)
        // = 27/5 = 5.4
        let test = "avg(min(1,2), avg(3,4), max(5,6), min(7,8), avg(9,10))";
        assert_eq!(calculator::calculate(test).unwrap(), 5.4);
    }

    #[test]
    fn stress_all_trig_nested() {
        // asin(sin(acos(cos(atan(tan(0.5))))))
        // Should approximately return 0.5 due to inverse relationships
        let test = "asin(sin(acos(cos(atan(tan(0.5))))))";
        let result = calculator::calculate(test).unwrap();
        assert!((result - 0.5).abs() < 0.01);
    }

    #[test]
    fn stress_power_tower_in_variadic() {
        // max(2^3, 3^2, avg(4^2, 2^4))
        // = max(8, 9, avg(16, 16))
        // = max(8, 9, 16)
        // = 16
        assert_eq!(
            calculator::calculate("max(2^3, 3^2, avg(4^2, 2^4))").unwrap(),
            16.0
        );
    }

    #[test]
    fn stress_factorial_in_nested_variadic() {
        // min(5!, avg(4!, 3!), max(2!, 1!))
        // = min(120, avg(24, 6), max(2, 1))
        // = min(120, 15, 2)
        // = 2
        assert_eq!(
            calculator::calculate("min(5!, avg(4!, 3!), max(2!, 1!))").unwrap(),
            2.0
        );
    }

    #[test]
    fn stress_mixed_functions_deep() {
        // sqrt(abs(ln(e^max(2, avg(3, min(4, 5))))))
        // = sqrt(abs(ln(e^max(2, 3.5))))
        // = sqrt(abs(ln(e^3.5)))
        // = sqrt(abs(3.5))
        // = sqrt(3.5)
        // ≈ 1.8708
        let test = "sqrt(abs(ln(e^max(2, avg(3, min(4, 5))))))";
        let result = calculator::calculate(test).unwrap();
        let expected = 3.5_f32.sqrt();
        assert!((result - expected).abs() < 0.01);
    }

    #[test]
    fn stress_parentheses_explosion() {
        // ((((((((1 + 2) * 3) - 4) / 2) + 5) * 2) - 3) + 1)
        // = (((((3 * 3) - 4) / 2) + 5) * 2) - 3) + 1
        // = ((((9 - 4) / 2) + 5) * 2) - 3) + 1
        // = (((5 / 2) + 5) * 2) - 3) + 1
        // = ((2.5 + 5) * 2) - 3) + 1
        // = (7.5 * 2) - 3) + 1
        // = 15 - 3 + 1
        // = 13
        let test = "((((((((1 + 2) * 3) - 4) / 2) + 5) * 2) - 3) + 1)";
        assert_eq!(calculator::calculate(test).unwrap(), 13.0);
    }

    // === Variable x Coefficient Edge Cases (8 tests) ===

    #[test]
    fn stress_large_coefficient() {
        // 999.999x + 1 where x = 0 = 1
        assert_eq!(calculator::calculate("999.999x + 1").unwrap(), 1.0);
    }

    #[test]
    fn stress_tiny_coefficient() {
        // 0.00001x + 10 where x = 0 = 10
        assert_eq!(calculator::calculate("0.00001x + 10").unwrap(), 10.0);
    }

    #[test]
    fn stress_coefficient_power_chain() {
        // 2x^3 + 5 where x = 0 = 5
        assert_eq!(calculator::calculate("2x^3 + 5").unwrap(), 5.0);
    }

    #[test]
    fn stress_multiple_x_terms_complex() {
        // 5x + 3x - 2x + x + 10 where x = 0 = 10
        assert_eq!(
            calculator::calculate("5x + 3x - 2x + x + 10").unwrap(),
            10.0
        );
    }

    #[test]
    fn stress_x_in_variadic() {
        // avg(x + 1, 2x + 2, 3x + 3) where x = 0 = avg(1, 2, 3) = 2
        assert_eq!(
            calculator::calculate("avg(x + 1, 2x + 2, 3x + 3)").unwrap(),
            2.0
        );
    }

    #[test]
    fn stress_x_in_trig() {
        // sin(π*x + π) where x = 0 = sin(π) ≈ 0
        let result = calculator::calculate("sin(π*x + π)").unwrap();
        assert!(result.abs() < 0.001);
    }

    #[test]
    fn stress_coefficient_with_parens() {
        // (2 + 3)*x + 7 where x = 0 = 7
        assert_eq!(calculator::calculate("(2 + 3)*x + 7").unwrap(), 7.0);
    }

    #[test]
    fn stress_x_factorial() {
        // (x + 5)! where x = 0 = 5! = 120
        assert_eq!(calculator::calculate("(x + 5)!").unwrap(), 120.0);
    }

    // === Mathematical Identities & Properties (8 tests) ===

    #[test]
    fn stress_pythagorean_identity() {
        // sin^2(π/3) + cos^2(π/3) should equal 1
        let test = "sin(π/3)^2 + cos(π/3)^2";
        let result = calculator::calculate(test).unwrap();
        assert!(is_close(result, 1.0));
    }

    #[test]
    fn stress_log_power_identity() {
        // ln(e^5) should equal 5
        let test = "ln(e^5)";
        let result = calculator::calculate(test).unwrap();
        assert!(is_close(result, 5.0));
    }

    #[test]
    fn stress_power_product_rule() {
        // 2^3 * 2^4 = 2^7 = 128
        assert_eq!(calculator::calculate("2^3 * 2^4").unwrap(), 128.0);
    }

    #[test]
    fn stress_sqrt_product_rule() {
        // sqrt(4) * sqrt(9) = sqrt(36) = 6
        assert_eq!(calculator::calculate("sqrt(4) * sqrt(9)").unwrap(), 6.0);
    }

    #[test]
    fn stress_associative_addition() {
        // (2 + 3) + 4 = 2 + (3 + 4)
        let a = calculator::calculate("(2 + 3) + 4").unwrap();
        let b = calculator::calculate("2 + (3 + 4)").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn stress_associative_multiplication() {
        // (2 * 3) * 4 = 2 * (3 * 4)
        let a = calculator::calculate("(2 * 3) * 4").unwrap();
        let b = calculator::calculate("2 * (3 * 4)").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn stress_log_base_change() {
        // log_2(8) = ln(8) / ln(2) = 3
        let test1 = "log_2(8)";
        let test2 = "ln(8) / ln(2)";
        let r1 = calculator::calculate(test1).unwrap();
        let r2 = calculator::calculate(test2).unwrap();
        assert!(is_close(r1, r2));
    }

    #[test]
    fn stress_euler_identity_component() {
        // e^(iπ) + 1 = 0, but we can test e^0 + 1 = 2
        assert_eq!(calculator::calculate("e^0 + 1").unwrap(), 2.0);
    }

    // === Extreme Value Tests (7 tests) ===

    #[test]
    fn stress_very_large_factorial() {
        // 15! = 1307674368000
        let result = calculator::calculate("15!").unwrap();
        assert!(result > 1_000_000_000_000.0);
    }

    #[test]
    fn stress_very_small_division() {
        // 1 / 10000000
        let result = calculator::calculate("1 / 10000000").unwrap();
        assert!(result > 0.0 && result < 0.001);
    }

    #[test]
    fn stress_large_power() {
        // 10^6 = 1000000
        assert_eq!(calculator::calculate("10^6").unwrap(), 1000000.0);
    }

    #[test]
    fn stress_decimal_precision() {
        // 0.1 + 0.2 (floating point precision test)
        let result = calculator::calculate("0.1 + 0.2").unwrap();
        assert!((result - 0.3).abs() < 0.0001);
    }

    #[test]
    fn stress_alternating_large_small() {
        // 1000000 + 0.00001 - 1000000 (floating point precision limits)
        // Due to f32 precision, this might not preserve the small value perfectly
        let result = calculator::calculate("1000000 + 0.00001 - 1000000").unwrap();
        assert!(result.abs() < 1.0); // Close to 0 due to precision limits
    }

    #[test]
    fn stress_negative_infinity_approach() {
        // ln(0.0000000001) should be very negative
        let result = calculator::calculate("ln(0.0000000001)").unwrap();
        assert!(result < -10.0);
    }

    #[test]
    fn stress_infinity_division() {
        // 1 / 0.0000000001
        let result = calculator::calculate("1 / 0.0000000001").unwrap();
        assert!(result > 1_000_000_000.0);
    }

    // === Variadic Function Stress Tests (7 tests) ===

    #[test]
    fn stress_twenty_argument_avg() {
        let test = "avg(1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20)";
        assert_eq!(calculator::calculate(test).unwrap(), 10.5);
    }

    #[test]
    fn stress_min_with_duplicates() {
        // min(5, 5, 5, 5, 5, 5)
        assert_eq!(calculator::calculate("min(5, 5, 5, 5, 5, 5)").unwrap(), 5.0);
    }

    #[test]
    fn stress_max_with_duplicates() {
        // max(7, 7, 7, 7)
        assert_eq!(calculator::calculate("max(7, 7, 7, 7)").unwrap(), 7.0);
    }

    #[test]
    fn stress_median_large_set() {
        // med(1,2,3,4,5,6,7,8,9,10,11) = 6
        assert_eq!(
            calculator::calculate("med(1,2,3,4,5,6,7,8,9,10,11)").unwrap(),
            6.0
        );
    }

    #[test]
    fn stress_mode_uniform_distribution() {
        // mode(1,2,3,4,5) - all unique, should return NaN
        let result = calculator::calculate("mode(1,2,3,4,5)").unwrap();
        assert!(result.is_nan());
    }

    #[test]
    fn stress_mode_triple_tie() {
        // mode(1,1,2,2,3,3) - three modes, average = 2
        assert_eq!(calculator::calculate("mode(1,1,2,2,3,3)").unwrap(), 2.0);
    }

    #[test]
    fn stress_choice_large_numbers() {
        // ch(10, 5) = 252
        assert_eq!(calculator::calculate("ch(10, 5)").unwrap(), 252.0);
    }
}

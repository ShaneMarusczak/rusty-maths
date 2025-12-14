#[cfg(test)]
mod rm_tests {
    use crate::equation_analyzer::vec_pipeline::calculator::Point;

    // Import all three pipeline implementations
    use crate::equation_analyzer::vec_pipeline::calculator as vec_calc;
    use crate::equation_analyzer::hybrid_pipeline::calculator as hybrid_calc;
    use crate::equation_analyzer::full_pipeline::calculator as full_calc;

    // Legacy imports for backward compatibility and internal testing
    use crate::equation_analyzer::pipeline::evaluator::evaluate;
    use crate::equation_analyzer::pipeline::parser::parse;
    use crate::equation_analyzer::pipeline::tokenizer::get_tokens;
    use crate::equation_analyzer::structs::token::TokenType::{
        CloseParen, End, Equal, Ln, Log, Minus, Number, OpenParen, Plus, Power, Sin, Slash, Star,
        Y, _E,
    };
    use crate::equation_analyzer::structs::token::{Token, TokenType};
    use crate::utilities::abs_f32;
    use std::f32::consts::{E, PI};

    fn is_close(x1: f32, x2: f32) -> bool {
        abs_f32(x1 - x2) < f32::EPSILON
    }

    // Test all three calculators produce identical results
    fn calculate(eq: &str) -> Result<f32, String> {
        let vec_result = vec_calc::calculate(eq)?;
        let hybrid_result = hybrid_calc::calculate(eq)?;
        let full_result = full_calc::calculate(eq)?;

        // Handle NaN specially (NaN != NaN)
        if vec_result.is_nan() {
            assert!(
                hybrid_result.is_nan(),
                "Vec is NaN but Hybrid is {} for '{}'",
                hybrid_result, eq
            );
            assert!(
                full_result.is_nan(),
                "Vec is NaN but Full is {} for '{}'",
                full_result, eq
            );
        } else {
            assert!(
                (vec_result - hybrid_result).abs() < 0.0001,
                "Vec vs Hybrid differ for '{}': {} vs {}",
                eq, vec_result, hybrid_result
            );
            assert!(
                (vec_result - full_result).abs() < 0.0001,
                "Vec vs Full differ for '{}': {} vs {}",
                eq, vec_result, full_result
            );
        }

        Ok(vec_result)
    }

    // Test all three plot functions produce identical results
    fn plot(eq: &str, x_min: f32, x_max: f32, step: f32) -> Result<Vec<Point>, String> {
        let vec_result = vec_calc::plot(eq, x_min, x_max, step)?;
        let hybrid_result = hybrid_calc::plot(eq, x_min, x_max, step)?;
        let full_result = full_calc::plot(eq, x_min, x_max, step)?;

        assert_eq!(
            vec_result.len(), hybrid_result.len(),
            "Vec vs Hybrid length differ for '{}': {} vs {}",
            eq, vec_result.len(), hybrid_result.len()
        );
        assert_eq!(
            vec_result.len(), full_result.len(),
            "Vec vs Full length differ for '{}': {} vs {}",
            eq, vec_result.len(), full_result.len()
        );

        for (i, ((v, h), f)) in vec_result.iter().zip(hybrid_result.iter()).zip(full_result.iter()).enumerate() {
            assert_eq!(v.x, h.x, "Vec vs Hybrid x differ at index {} for '{}'", i, eq);
            assert_eq!(v.x, f.x, "Vec vs Full x differ at index {} for '{}'", i, eq);

            // Handle NaN in y values
            if v.y.is_nan() {
                assert!(h.y.is_nan(), "Vec y is NaN but Hybrid is {} at index {} for '{}'", h.y, i, eq);
                assert!(f.y.is_nan(), "Vec y is NaN but Full is {} at index {} for '{}'", f.y, i, eq);
            } else {
                assert!(
                    (v.y - h.y).abs() < 0.0001,
                    "Vec vs Hybrid y differ at index {} for '{}': {} vs {}",
                    i, eq, v.y, h.y
                );
                assert!(
                    (v.y - f.y).abs() < 0.0001,
                    "Vec vs Full y differ at index {} for '{}': {} vs {}",
                    i, eq, v.y, f.y
                );
            }
        }

        Ok(vec_result)
    }

    #[test]
    fn plot_test_linear() {
        let test_eq = "y = 2x + 1";
        let points = vec![
            Point::new(-1_f32, -1_f32),
            Point::new(0_f32, 1_f32),
            Point::new(1_f32, 3_f32),
        ];

        let actual = plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

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

        let actual = plot(test_eq, 0f32, 2f32, 1_f32).unwrap();

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

        let actual = plot(test_eq, 0f32, 2f32, 1_f32).unwrap();

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

        let actual = plot(test_eq, 0f32, 2f32, 1_f32).unwrap();

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

        let actual = plot(test_eq, 0f32, 2f32, 1_f32).unwrap();

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

        let actual = plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

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

        let actual = plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

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

        let actual = plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

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

        let actual = plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

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

        let actual = plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

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

        let actual = plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

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

        let actual = plot(test_eq, -PI, PI, PI / 2_f32).unwrap();

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

        let actual = plot(test_eq, -PI, PI, PI / 2_f32).unwrap();

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

        let actual = plot(test_eq, 2_f32, 3_f32, 0.25_f32).unwrap();

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

        let actual = plot(test_eq, 1_f32, 10_f32, 1_f32).unwrap();

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

        assert_eq!(parse(test).unwrap(), ans);
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
            parse(test).unwrap(),
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

        assert_eq!(parse(test).unwrap(), ans);
    }

    #[test]
    fn parse_test_6_no_eof() {
        //2 ^ 16;
        let test = vec![
            get_token_n(Number, 2.0, 0.0),
            get_token(Power),
            get_token_n(Number, 16.0, 0.0),
        ];
        assert_eq!(parse(test).unwrap_err(), "No end token found");
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
        assert_eq!(parse(test).unwrap_err(), "Invalid function");
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
        assert_eq!(parse(test).unwrap_err(), "Invalid opening parenthesis");
    }

    #[test]
    fn parse_test_bad_parens_2() {
        let test = vec![
            get_token(CloseParen),
            get_token(Power),
            get_token_n(Number, 1.0, 1.0),
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
            get_token_n(Number, -1.0, 0.0),
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
            get_token_n(Number, -2.0, 0.0),
            get_token(End),
        ];
        assert_eq!(get_tokens(eq).unwrap(), ans);
    }

    #[test]
    fn eval_rpn_test_1() {
        let test = "3 + 4 * ( 2 - 1 )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_2() {
        let test = "3 + 4 * 2 - 1";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 10_f32);
    }

    #[test]
    fn eval_rpn_test_3() {
        let test = "y = 3 + 4 * ( 2 - 1 )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_4() {
        let test = "y = 16^(1/2) + 16 + 3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 23_f32);
    }

    #[test]
    fn eval_rpn_test_5() {
        let test = "y = 2^2 + 2*2 + 3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 11_f32);
    }

    #[test]
    fn eval_rpn_test_6() {
        let test = "-2 + 3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 1_f32);
    }

    #[test]
    fn eval_rpn_test_7() {
        let test = "-e + -π";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, -E + -PI);
    }

    #[test]
    fn eval_rpn_test_8() {
        let test = "y = 2 ^ 2^2";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 16_f32);
    }

    #[test]
    fn eval_rpn_test_9() {
        let test = "y = 2 ^ (3*2)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 64_f32);
    }

    #[test]
    fn eval_rpn_test_10() {
        let test = "y = 2 ^ (2*2 + 1 )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 32_f32);
    }

    #[test]
    fn eval_rpn_test_trig_2() {
        let test = "sin( 3.14159265358979323846)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_trig_3() {
        let test = " sin( π )/ 2";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_trig_4() {
        let test = "sin( π/2 )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_trig_5() {
        let test = "cos(π ) / 2";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert!(is_close(ans, -0.5f32));
    }

    #[test]
    fn eval_rpn_test_trig_6() {
        let test = "tan( π )+ cos( π+π ) + sin( 2 *π )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap().round();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_trig_7() {
        let test = "sin( -π )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_trig_8() {
        let test = "sin( π )";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert!(is_close(ans, 0_f32));
    }

    #[test]
    fn eval_rpn_test_abs() {
        let test = "abs(2 - 3^2)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 7_f32);
    }

    #[test]
    fn eval_rpn_test_abs_2() {
        let test = "abs(2 *3 - 3^2)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 3_f32);
    }

    #[test]
    fn eval_rpn_test_sqrt() {
        let test = "sqrt(1764)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 42_f32);
    }

    #[test]
    fn eval_rpn_test_min() {
        let test = "min(5,8,7,9)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 5_f32);
    }

    #[test]
    fn eval_rpn_test_ln() {
        let test = "ln(e)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_log() {
        let test = "log_10(10)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert!(is_close(ans, 1_f32));
    }

    #[test]
    fn eval_rpn_test_log_add() {
        let test = "log_10(10) + log_10(10)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert!(is_close(ans, 2_f32));
    }

    #[test]
    fn eval_rpn_test_log_add_2() {
        let test = "log_10(10) + log_10(10) + log_10(10)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert!(is_close(ans, 3_f32));
    }

    #[test]
    fn eval_rpn_test_log_add_3() {
        let test = "log_10(10) + log_10(5 + 5)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert!(is_close(ans, 2_f32));
    }

    #[test]
    fn eval_rpn_test_log_base_7() {
        let test = "log_7(49)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
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
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 0_f32);
    }

    #[test]
    fn minus_test_2() {
        let test = "3- 3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 0_f32);
    }

    #[test]
    fn minus_test_3() {
        let test = "log_3(3)- 3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, -2_f32);
    }

    #[test]
    fn minus_test_4() {
        let test = "3--3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 6_f32);
    }

    #[test]
    fn extra_pow_test() {
        let test = "2^2-3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 1_f32);
    }

    #[test]
    fn extra_pow_test_4() {
        let test = "2^(2-3)";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
        assert_eq!(ans, 0.5);
    }

    #[test]
    fn extra_pow_test_2() {
        let test = "10^10-3";
        let tokens = get_tokens(test).unwrap();
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();
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
    fn plot_test_linear_7() {
        let test_eq = "y = 2x +1";
        let points = vec![
            Point::new(-1_f32, -1_f32),
            Point::new(0_f32, 1_f32),
            Point::new(1_f32, 3_f32),
        ];

        let actual = plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

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

        let actual = plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

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

        let actual = plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual, points);
    }

    #[test]
    fn plot_test_trig() {
        let test_eq = "y = tan(x)";
        let points = vec![(-1_f32, -1.5574077), (0_f32, 0_f32), (1_f32, 1.5574077)];

        let actual = plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        for (p, (x_2, y_2)) in actual.iter().zip(points) {
            assert!(is_close(p.x, x_2));
            assert!(is_close(p.y, y_2));
        }
    }

    #[test]
    fn plot_test_max() {
        let test_eq = "y = max(x,0.5)";
        let points = vec![(-1_f32, 0.5), (0_f32, 0.5), (1_f32, 1_f32)];

        let actual = plot(test_eq, -1f32, 1_f32, 1_f32).unwrap();

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
        let parsed_eq = parse(tokens).unwrap();
        let ans = evaluate(&parsed_eq, None).unwrap();

        assert!(is_close(ans, 9_f32));
    }

    #[test]
    fn test_calculate_with_complex_equation() {
        let test = "2*(3+4*(5-6))/7 + 8^(9/10)";
        let expected_result = 6.21230459;
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
        let test = "π+sin(π) - sin(π) - π + sqrt(π) - sqrt(π) - 2^π + 2^π - π^2 + π^2";
        let expected_result = 0_f32;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn pi_test_3() {
        let test = "((π + 2*π - 3*π)*2*π)/2*π";
        let expected_result = 0_f32;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn avg_test() {
        let test = "avg(1,2,9)";
        let expected_result = 4_f32;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn avg_test_2() {
        let test = "avg(1,2,9) + avg(1,2,9)";
        let expected_result = 8_f32;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn avg_test_3() {
        let test = "avg(1,2,9) + sin(avg(-12,1,2,9))";
        let expected_result = 4_f32;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn avg_test_4() {
        let test = "avg(1,2,sin(0))";
        let expected_result = "Params can only be numbers";
        let actual_result = calculate(test).unwrap_err();
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn min_test() {
        let test = "min(1,2,9)";
        let expected_result = 1_f32;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn max_test() {
        let test = "max(1,2,9)";
        let expected_result = 9_f32;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn max_min_test() {
        let test = "max(1,2,9) + min(1,2,9)";
        let expected_result = 10_f32;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn max_min_avg_test() {
        let test = "max(1,2,9) + min(1,2,9) + avg(1,2,9)";
        let expected_result = 14_f32;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn med_test() {
        let test = "med(2,1,9)";
        let expected_result = 2_f32;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn med_test_2() {
        let test = "med(1,2,9,11)";
        let expected_result = 5.5;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn mode_test() {
        let test = "mode(1,2,3,4,5,2)";
        let expected_result = 2_f32;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn mode_test_2() {
        let test = "mode(1.1,2.3,3.4,3.4,5,2)";
        let expected_result = 3.4;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn mode_test_uniform() {
        // Uniform distribution: all values appear once, no mode exists
        let test = "mode(1, 2, 3, 4, 5)";
        let actual_result = calculate(test).unwrap();
        assert!(actual_result.is_nan(), "Uniform distribution should return NaN");
    }

    #[test]
    fn mode_test_multimodal() {
        // Multimodal: 1 and 3 both appear twice, should return average (1+3)/2 = 2
        let test = "mode(1, 1, 3, 3, 5)";
        let expected_result = 2.0;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn mode_test_multimodal_three_modes() {
        // Three modes: 1, 2, and 3 all appear twice, average = (1+2+3)/3 = 2
        let test = "mode(1, 1, 2, 2, 3, 3)";
        let expected_result = 2.0;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn choice_test() {
        let test = "ch(5,2)";
        let expected_result = 10.0;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn choice_test_2() {
        let test = "ch( 20 ,9)";
        let expected_result = 167960.0;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn choice_test_3() {
        let test = "ch(20,  9   ) - 10000";
        let expected_result = 157960.0;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn choice_test_error() {
        let test = "ch(20,9, 0)";
        let expected_result = "Choice function takes two parameters, found 3.";
        let actual_result = calculate(test).unwrap_err();
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn choice_test_error_2() {
        let test = "ch(20,9.1)";
        let expected_result = "Choice is only defined for positive whole numbers";
        let actual_result = calculate(test).unwrap_err();
        assert_eq!(expected_result, actual_result);
    }

    #[test]
    fn a_trig_test() {
        let test = "asin(sin(0.5)) + acos(cos(0.5)) + atan(tan(0.5))";
        let expected_result = 1.5;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn y_test() {
        let test = "y";
        let expected_result = "Invalid equation supplied";
        let actual_result = calculate(test).unwrap_err();
        assert_eq!(actual_result, expected_result);
    }

    #[test]
    fn neg_test() {
        let test = "-(5 + 2)";
        let expected_result = -7.0;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn neg_test_2() {
        let test = "-5 + (2)";
        let expected_result = -3.0;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn neg_test_3() {
        let test = "-sqrt(4)";
        let expected_result = -2.0;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn neg_test_4() {
        let test = "----sqrt(4)";
        let expected_result = 2.0;
        let actual_result = calculate(test).unwrap();
        assert!(is_close(actual_result, expected_result));
    }

    #[test]
    fn neg_sqrt_test() {
        let test = "sqrt(-4)";
        let actual_result = calculate(test).unwrap();
        assert!(actual_result.is_nan());
    }
}

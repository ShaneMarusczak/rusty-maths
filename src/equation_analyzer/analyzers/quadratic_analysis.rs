use crate::equation_analyzer::analyzers::analyzer_helper::{get_multiplier, starts_or_ends_with_y};
use crate::equation_analyzer::structs::token::{Token, TokenType};

///Detects if the given equation is in the form 'y = ax^2 + bx + c'
/// Note: a can not be 0
pub(crate) fn detect_quad(eq: &[Token]) -> bool {
    if !eq.iter().any(|t| t.literal.contains("x^2"))
        || eq.len() > 8
        || eq.len() < 4
        || !(starts_or_ends_with_y(eq))
    {
        return false;
    }

    let mut pow_count = 0;

    let mut x_count = 0;

    for token in eq {
        if token.literal.contains('^') && !token.literal.ends_with('1') {
            pow_count += 1;
        }
        if token.token_type == TokenType::X {
            x_count += 1;
        }
        if token.token_type == TokenType::End {
            continue;
        }
        if token.literal.len() > 1
            && !token.literal.contains('x')
            && token.literal.parse::<f32>().is_err()
        {
            return false;
        }
    }

    if pow_count != 1 || ![1, 2].contains(&x_count) {
        return false;
    }

    true
}

pub(crate) fn get_abc(eq: &[Token]) -> (f32, f32, f32) {
    let mut a = 0_f32;
    let mut b = 0_f32;
    let mut c = 0_f32;

    for (i, token) in eq.iter().enumerate() {
        if token.literal == "-1x^2" {
            a = -1_f32;
        } else if token.literal == "1x^2" {
            a = 1_f32;
        } else if token.literal.contains("x^2") {
            a = token
                .literal
                .split("x^2")
                .next()
                .unwrap()
                .parse::<f32>()
                .unwrap()
                * get_multiplier(i, eq);
        } else if token.literal == "-1x^1" {
            b = -1_f32;
        } else if token.literal == "1x^1" {
            b = 1_f32;
        } else if token.literal.contains('x') {
            b = token
                .literal
                .split('x')
                .next()
                .unwrap()
                .parse::<f32>()
                .unwrap()
                * get_multiplier(i, eq);
        } else if let Ok(n) = token.literal.parse::<f32>() {
            c = n * get_multiplier(i, eq);
        }
    }

    (a, b, c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equation_analyzer::pipeline::tokenizer::get_tokens;

    #[test]
    fn detect_quad_1() {
        let test_eq = "y = x^2 + x + 42";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_quad(&tokens));
    }

    #[test]
    fn detect_quad_2() {
        let test_eq = "y = 2x^2 + 3";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_quad(&tokens));
    }

    #[test]
    fn detect_quad_3() {
        let test_eq = "y = sin( x^2 )";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(!detect_quad(&tokens));
    }

    #[test]
    fn detect_quad_4() {
        let test_eq = "y = x^3 + x^2";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(!detect_quad(&tokens));
    }

    #[test]
    fn detect_quad_5() {
        let test_eq = "y = -2x^2 + 3x + 2";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_quad(&tokens));
    }

    #[test]
    fn detect_quad_6() {
        let test_eq = "y = x + 17";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(!detect_quad(&tokens));
    }

    #[test]
    fn detect_quad_7() {
        let test_eq = "y = x^3";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(!detect_quad(&tokens));
    }

    #[test]
    fn detect_quad_8() {
        let test_eq = "x^2 = y";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_quad(&tokens));
    }

    #[test]
    fn detect_quad_9() {
        let test_eq = "x^2 = y + 1";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(!detect_quad(&tokens));
    }

    #[test]
    fn detect_quad_10() {
        let test_eq = "y = x^2 + x + x";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(!detect_quad(&tokens));
    }

    #[test]
    fn get_abc_test() {
        let test_eq = "y = -2x^2 + 3x + 2";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_quad(&tokens));
        assert_eq!(get_abc(&tokens), (-2_f32, 3_f32, 2_f32));
    }

    #[test]
    fn get_abc_test_2() {
        let test_eq = "y = x^2 + x + 2";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_quad(&tokens));
        assert_eq!(get_abc(&tokens), (1_f32, 1_f32, 2_f32));
    }

    #[test]
    fn get_abc_test_3() {
        let test_eq = "y = x^2";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_quad(&tokens));
        assert_eq!(get_abc(&tokens), (1_f32, 0_f32, 0_f32));
    }

    #[test]
    fn get_abc_test_4() {
        let test_eq = "y = x^2 + 7x";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_quad(&tokens));
        assert_eq!(get_abc(&tokens), (1_f32, 7_f32, 0_f32));
    }

    #[test]
    fn get_abc_test_5() {
        let test_eq = "y = x^2 - 7x";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_quad(&tokens));
        assert_eq!(get_abc(&tokens), (1_f32, -7_f32, 0_f32));
    }

    #[test]
    fn get_abc_test_6() {
        let test_eq = "y = -x^2 - 7x - 2";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_quad(&tokens));
        assert_eq!(get_abc(&tokens), (-1_f32, -7_f32, -2_f32));
    }

    #[test]
    fn get_abc_test_7() {
        let test_eq = "y = -x^2 - -7x - 2";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_quad(&tokens));
        assert_eq!(get_abc(&tokens), (-1_f32, 7_f32, -2_f32));
    }

    #[test]
    fn get_abc_test_8() {
        let test_eq = "y = 2 + x^2 + 3x";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_quad(&tokens));
        assert_eq!(get_abc(&tokens), (1_f32, 3_f32, 2_f32));
    }
}

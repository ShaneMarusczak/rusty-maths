use crate::equation_analyzer::analyzers::analyzer_helper::{get_multiplier, starts_or_ends_with_y};
use crate::equation_analyzer::structs::token::{Token, TokenType};

///Detects if the given equation is in the form 'y = mx + b'
pub(crate) fn detect_linear(eq: &[Token]) -> bool {
    if eq.len() > 6
        || eq.len() < 4
        || !(starts_or_ends_with_y(eq))
        || eq
            .iter()
            .any(|t| t.token_type != TokenType::X && t.literal.contains('^'))
        || eq
            .iter()
            .any(|t| t.token_type == TokenType::X && !t.literal.ends_with('1'))
    {
        return false;
    }

    let mut x_count = 0;

    for token in eq {
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

    if !(x_count == 0 || x_count == 1) {
        return false;
    }
    true
}

pub(crate) fn get_zero(eq: &[Token]) -> f32 {
    let (m, b) = get_m_b(eq);
    if m == 0_f32 {
        return f32::NAN;
    }
    -b / m
}

fn get_m_b(eq: &[Token]) -> (f32, f32) {
    let mut m = 0_f32;
    let mut b = 0_f32;

    for (i, token) in eq.iter().enumerate() {
        if token.literal.starts_with("1x") {
            m = 1_f32;
        } else if token.literal.starts_with("-1x") {
            m = -1_f32;
        } else if token.literal.contains('x') {
            m = token
                .literal
                .split('x')
                .next()
                .unwrap()
                .parse::<f32>()
                .unwrap()
                * get_multiplier(i, eq);
        } else if let Ok(n) = token.literal.parse::<f32>() {
            b = n;
        }
    }
    (m, b)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::equation_analyzer::pipeline::tokenizer::get_tokens;

    #[test]
    fn detect_linear_1() {
        let test_eq = "y = x^2 + x + 42";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(!detect_linear(&tokens));
    }

    #[test]
    fn detect_linear_2() {
        let test_eq = "y = 2x^2 + 3";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(!detect_linear(&tokens));
    }

    #[test]
    fn detect_linear_3() {
        let test_eq = "y = sin( x^2 )";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(!detect_linear(&tokens));
    }

    #[test]
    fn detect_linear_4() {
        let test_eq = "y = x^3 + x^2";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(!detect_linear(&tokens));
    }

    #[test]
    fn detect_linear_5() {
        let test_eq = "y = -2x^2 + 3x + 2";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(!detect_linear(&tokens));
    }

    #[test]
    fn detect_linear_6() {
        let test_eq = "y = x + 17";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_linear(&tokens));
    }

    #[test]
    fn detect_linear_7() {
        let test_eq = "y = x^3";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(!detect_linear(&tokens));
    }

    #[test]
    fn detect_linear_8() {
        let test_eq = "x^2 = y";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(!detect_linear(&tokens));
    }

    #[test]
    fn detect_linear_9() {
        let test_eq = "x^2 = y + 1";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(!detect_linear(&tokens));
    }

    #[test]
    fn detect_linear_10() {
        let test_eq = "y = max( x^2 , 10 )";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(!detect_linear(&tokens));
    }

    #[test]
    fn detect_linear_11() {
        let test_eq = "y = x^2 + x + x";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(!detect_linear(&tokens));
    }

    #[test]
    fn detect_linear_12() {
        let test_eq = "y = 42x + 7";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_linear(&tokens));
        assert_eq!(get_m_b(&tokens), (42_f32, 7_f32));
        assert_eq!(get_zero(&tokens), -(7_f32 / 42_f32));
    }

    #[test]
    fn detect_linear_13() {
        let test_eq = "y = 7";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_linear(&tokens));
        assert_eq!(get_m_b(&tokens), (0_f32, 7_f32));
        assert!(get_zero(&tokens).is_nan());
    }

    #[test]
    fn detect_linear_14() {
        let test_eq = "12 + x = y";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_linear(&tokens));
        assert_eq!(get_m_b(&tokens), (1_f32, 12_f32));
        assert_eq!(get_zero(&tokens), -12_f32);
    }

    #[test]
    fn detect_linear_15() {
        let test_eq = "122 - 2x = y";
        let tokens = get_tokens(test_eq).unwrap();
        assert!(detect_linear(&tokens));
        assert_eq!(get_m_b(&tokens), (-2_f32, 122_f32));
        assert_eq!(get_zero(&tokens), -(122_f32 / -2_f32));
    }
}

///Detects if the given equation is in the form 'y = mx + b'
pub fn detect_linear(eq: &str) -> bool {
    if eq.split_whitespace().count() > 5 ||
       eq.split_whitespace().count() < 3 ||
       !(eq.starts_with("y =") || eq.ends_with("= y")) ||
       eq.contains('^') {
       return false;
    }

    let mut x_count = 0;

    for token in eq.split_whitespace() {
        if token.contains('x') {
            x_count += 1;
        }

        if token.len() > 1 && !token.contains('x') && token.parse::<f32>().is_err() {
            return false;
        }
    }

    if x_count > 1 {
        return false;
    }
    true
}

pub fn get_zero(eq: &str) -> f32 {
    let (m, b) = get_m_b(eq);
    if m == 0_f32 {
        return f32::NAN;
    }
    -b / m
}

fn get_m_b(eq: &str) -> (f32, f32) {
    let mut m = 0_f32;
    let mut b = 0_f32;

    let split_eq = eq.split_whitespace().collect::<Vec<&str>>();

    for (i, token) in eq.split_whitespace().enumerate() {
        if token == "x" {
            m = 1_f32;
        }
        else if token == "-x" {
            m = -1_f32;
        }
        else if token.contains('x') {
            m = token.split('x').next().unwrap().parse::<f32>().unwrap() * get_multiplier(i, &split_eq);
        }
        else if let Ok(n) = token.parse::<f32>() {
            b = n;
        }
    }
    (m, b)
}

fn get_multiplier(i: usize, split_eq: &[&str]) -> f32 {
    let mut multiplier = 1_f32;
    if i != 0 && split_eq[i - 1] == "-" {
        multiplier = -1_f32;
    }
    multiplier
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_linear_1() {
        let test_eq = "y = x^2 + x + 42";
        assert_eq!(detect_linear(test_eq), false);
    }

    #[test]
    fn detect_linear_2() {
        let test_eq = "y = 2x^2 + 3";
        assert_eq!(detect_linear(test_eq), false);
    }

    #[test]
    fn detect_linear_3() {
        let test_eq = "y = sin ( x^2 )";
        assert_eq!(detect_linear(test_eq), false);
    }

    #[test]
    fn detect_linear_4() {
        let test_eq = "y = x^3 + x^2";
        assert_eq!(detect_linear(test_eq), false);
    }

    #[test]
    fn detect_linear_5() {
        let test_eq = "y = -2x^2 + 3x + 2";
        assert_eq!(detect_linear(test_eq), false);
    }

    #[test]
    fn detect_linear_6() {
        let test_eq = "y = x + 17";
        assert_eq!(detect_linear(test_eq), true);
    }

    #[test]
    fn detect_linear_7() {
        let test_eq = "y = x^3";
        assert_eq!(detect_linear(test_eq), false);
    }

    #[test]
    fn detect_linear_8() {
        let test_eq = "x^2 = y";
        assert_eq!(detect_linear(test_eq), false);
    }

    #[test]
    fn detect_linear_9() {
        let test_eq = "x^2 = y + 1";
        assert_eq!(detect_linear(test_eq), false);
    }

    #[test]
    fn detect_linear_10() {
        let test_eq = "y = max ( x^2 , 10 )";
        assert_eq!(detect_linear(test_eq), false);
    }

    #[test]
    fn detect_linear_11() {
        let test_eq = "y = x^2 + x + x";
        assert_eq!(detect_linear(test_eq), false);
    }

    #[test]
    fn detect_linear_12() {
        let test_eq = "y = 42x + 7";
        assert_eq!(detect_linear(test_eq), true);
        assert_eq!(get_m_b(test_eq), (42_f32, 7_f32));
        assert_eq!(get_zero(test_eq), -(7_f32 / 42_f32));
    }

    #[test]
    fn detect_linear_13() {
        let test_eq = "y = 7";
        assert_eq!(detect_linear(test_eq), true);
        assert_eq!(get_m_b(test_eq), (0_f32, 7_f32));
        assert!(get_zero(test_eq).is_nan());
    }

    #[test]
    fn detect_linear_14() {
        let test_eq = "12 + x = y";
        assert_eq!(detect_linear(test_eq), true);
        assert_eq!(get_m_b(test_eq), (1_f32, 12_f32));
        assert_eq!(get_zero(test_eq), -12_f32);

    }

    #[test]
    fn detect_linear_15() {
        let test_eq = "122 - 2x = y";
        assert_eq!(detect_linear(test_eq), true);
        assert_eq!(get_m_b(test_eq), (-2_f32, 122_f32));
        assert_eq!(get_zero(test_eq), -(122_f32 / -2_f32));
    }
}
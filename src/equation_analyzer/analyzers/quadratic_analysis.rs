//TODO:THESE ARE BUSTED AFTER THE SCANNER, MAYBE PASS IN THE TOKENS?

///Detects if the given equation is in the form 'y = ax^2 + bx + c'
/// Note: a can not be 0
pub fn detect_quad(eq: &str) -> bool {
    if !eq.contains("x^2")
        || eq.split_whitespace().count() > 7
        || eq.split_whitespace().count() < 3
        || !(eq.starts_with("y =") || eq.ends_with("= y"))
    {
        return false;
    }

    let mut pow_count = 0;

    let mut x_count = 0;

    for token in eq.split_whitespace() {
        if token.contains('^') {
            pow_count += 1;
        }
        if token.contains('x') {
            x_count += 1;
        }
        if token.len() > 1 && !token.contains('x') && token.parse::<f32>().is_err() {
            return false;
        }
    }

    if pow_count != 1 || ![1, 2].contains(&x_count) {
        return false;
    }

    true
}

pub fn get_abc(eq: &str) -> (f32, f32, f32) {
    let mut a = 0_f32;
    let mut b = 0_f32;
    let mut c = 0_f32;

    let split_eq = eq.split_whitespace().collect::<Vec<&str>>();

    for (i, term) in eq.split_whitespace().enumerate() {
        if term == "-x^2" {
            a = -1_f32;
        } else if term == "x^2" {
            a = 1_f32;
        } else if term.contains("x^2") {
            a = term.split("x^2").next().unwrap().parse::<f32>().unwrap()
                * get_multiplier(i, &split_eq);
        } else if term == "-x" {
            b = -1_f32;
        } else if term == "x" {
            b = 1_f32;
        } else if term.contains('x') {
            b = term.split('x').next().unwrap().parse::<f32>().unwrap()
                * get_multiplier(i, &split_eq);
        } else if let Ok(n) = term.parse::<f32>() {
            c = n * get_multiplier(i, &split_eq);
        }
    }

    (a, b, c)
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
    fn detect_quad_1() {
        let test_eq = "y = x^2 + x + 42";
        assert_eq!(detect_quad(test_eq), true);
    }

    #[test]
    fn detect_quad_2() {
        let test_eq = "y = 2x^2 + 3";
        assert_eq!(detect_quad(test_eq), true);
    }

    #[test]
    fn detect_quad_3() {
        let test_eq = "y = sin( x^2 )";
        assert_eq!(detect_quad(test_eq), false);
    }

    #[test]
    fn detect_quad_4() {
        let test_eq = "y = x^3 + x^2";
        assert_eq!(detect_quad(test_eq), false);
    }

    #[test]
    fn detect_quad_5() {
        let test_eq = "y = -2x^2 + 3x + 2";
        assert_eq!(detect_quad(test_eq), true);
    }

    #[test]
    fn detect_quad_6() {
        let test_eq = "y = x + 17";
        assert_eq!(detect_quad(test_eq), false);
    }

    #[test]
    fn detect_quad_7() {
        let test_eq = "y = x^3";
        assert_eq!(detect_quad(test_eq), false);
    }

    #[test]
    fn detect_quad_8() {
        let test_eq = "x^2 = y";
        assert_eq!(detect_quad(test_eq), true);
    }

    #[test]
    fn detect_quad_9() {
        let test_eq = "x^2 = y + 1";
        assert_eq!(detect_quad(test_eq), false);
    }

    #[test]
    fn detect_quad_10() {
        let test_eq = "y = x^2 + x + x";
        assert_eq!(detect_quad(test_eq), false);
    }

    #[test]
    fn get_abc_test() {
        let test_eq = "y = -2x^2 + 3x + 2";
        assert_eq!(detect_quad(test_eq), true);
        assert_eq!(get_abc(test_eq), (-2_f32, 3_f32, 2_f32));
    }

    #[test]
    fn get_abc_test_2() {
        let test_eq = "y = x^2 + x + 2";
        assert_eq!(detect_quad(test_eq), true);
        assert_eq!(get_abc(test_eq), (1_f32, 1_f32, 2_f32));
    }

    #[test]
    fn get_abc_test_3() {
        let test_eq = "y = x^2";
        assert_eq!(detect_quad(test_eq), true);
        assert_eq!(get_abc(test_eq), (1_f32, 0_f32, 0_f32));
    }

    #[test]
    fn get_abc_test_4() {
        let test_eq = "y = x^2 + 7x";
        assert_eq!(detect_quad(test_eq), true);
        assert_eq!(get_abc(test_eq), (1_f32, 7_f32, 0_f32));
    }

    #[test]
    fn get_abc_test_5() {
        let test_eq = "y = x^2 - 7x";
        assert_eq!(detect_quad(test_eq), true);
        assert_eq!(get_abc(test_eq), (1_f32, -7_f32, 0_f32));
    }

    #[test]
    fn get_abc_test_6() {
        let test_eq = "y = -x^2 - 7x - 2";
        assert_eq!(detect_quad(test_eq), true);
        assert_eq!(get_abc(test_eq), (-1_f32, -7_f32, -2_f32));
    }

    #[test]
    fn get_abc_test_7() {
        let test_eq = "y = -x^2 - -7x - 2";
        assert_eq!(detect_quad(test_eq), true);
        assert_eq!(get_abc(test_eq), (-1_f32, 7_f32, -2_f32));
    }

    #[test]
    fn get_abc_test_8() {
        let test_eq = "y = 2 + x^2 + 3x";
        assert_eq!(detect_quad(test_eq), true);
        assert_eq!(get_abc(test_eq), (1_f32, 3_f32, 2_f32));
    }
}

use crate::equation_analyzer::linear_analysis::detect_linear;
use crate::equation_analyzer::quadratic_analysis::{detect_quad, get_abc};
use crate::equation_analyzer::rpn::{eval_rpn, get_rpn};
use crate::utilities::quadratic_eq_f32;

mod linear_analysis;
mod operands;
mod quadratic_analysis;
mod rpn;

pub fn get_eq_data(
    eq: &str,
    x_min: f32,
    x_max: f32,
    step_size: f32,
) -> Result<EquationData, String> {
    let rpn = get_rpn(eq)?;

    let mut points = vec![];
    let mut zeros = vec![];

    //TODO: Should these be REGEX?
    if detect_linear(eq) {
        let z = linear_analysis::get_zero(eq);
        if !z.is_nan() {
            zeros.push(z);
        }
    } else if detect_quad(eq) {
        let (a, b, c) = get_abc(eq);

        if let Ok(z) = quadratic_eq_f32(a, b, c) {
            zeros.push(z.0);
            zeros.push(z.1);
        }
    }

    let mut x_cur = x_min;
    while x_cur <= x_max {
        points.push((x_cur, eval_rpn(&rpn, x_cur)?));
        x_cur += step_size;
    }
    Ok(EquationData { points, zeros })
}

#[derive(Debug, PartialEq)]
pub struct EquationData {
    pub points: Vec<(f32, f32)>,
    pub zeros: Vec<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    fn is_close(x1: f32, x2: f32) -> bool {
        (x1 - x2).abs() < 0.00001
    }

    #[test]
    fn get_eq_data_test_linear() {
        let test_eq = "y = 2x + 1";
        let points = vec![(-1_f32, -1_f32), (0_f32, 1_f32), (1_f32, 3_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.points, points);
        assert_eq!(actual.zeros, vec![-0.5_f32]);
    }

    #[test]
    fn get_eq_data_test_linear_2() {
        let test_eq = "y = -2x + 1";
        let ans = vec![(-1_f32, 3_f32), (0_f32, 1_f32), (1_f32, -1_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.points, ans);
        assert_eq!(actual.zeros, vec![0.5_f32]);
    }

    #[test]
    fn get_eq_data_test_linear_3() {
        let test_eq = "y = -x + 1";
        let ans = vec![(-1_f32, 2_f32), (0_f32, 1_f32), (1_f32, 0_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.points, ans);
        assert_eq!(actual.zeros, vec![1_f32]);
    }

    #[test]
    fn get_eq_data_test_quad() {
        let test_eq = "y = x^2 + 2x + 1";
        let ans = vec![(-1_f32, 0_f32), (0_f32, 1_f32), (1_f32, 4_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.points, ans);
        assert_eq!(actual.zeros[0], -1_f32);
        assert!(actual.zeros[1].is_nan());
    }

    #[test]
    fn get_eq_data_test_quad_1() {
        let test_eq = "y = -2x^2 + 2x + 1";
        let points = vec![(-1_f32, -3_f32), (0_f32, 1_f32), (1_f32, 1_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.points, points);
        assert!(actual.zeros.contains(&-0.36602783));
        assert!(actual.zeros.contains(&1.3660278));
    }

    #[test]
    fn get_eq_data_test_quad_2() {
        let test_eq = "y = x^2 - 1";
        let points = vec![(-1_f32, 0_f32), (0_f32, -1_f32), (1_f32, 0_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

        assert_eq!(actual.points, points);
        assert!(actual.zeros.contains(&1_f32));
        assert!(actual.zeros.contains(&-1_f32));
    }

    #[test]
    fn get_eq_data_test_quad_3() {
        let test_eq = "y = x^2 + 1";
        let points = vec![(-1_f32, 2_f32), (0_f32, 1_f32), (1_f32, 2_f32)];

        let actual = get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap();

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
        assert!(actual.zeros.is_empty());
    }

    #[test]
    fn get_eq_data_test_cos() {
        let test_eq = "y = cos( x + π )";
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
        assert!(actual.zeros.is_empty());
    }
}

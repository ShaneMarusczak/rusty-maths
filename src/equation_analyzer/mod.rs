use crate::equation_analyzer::rpn::{eval_rpn, get_rpn};

mod rpn;
mod operands;

pub fn get_eq_data(eq: &str, x_min: f32, x_max: f32, step_size: f32) -> Result<EquationData, String> {
    //TODO: Detect equation type ( y = x ^ 2 should trigger quadratic analysis, y = sin ( x ) trig... etc)

    let rpn = get_rpn(eq)?;

    let mut points = vec![];

    let mut x_cur = x_min;
    while x_cur <=x_max {
        points.push((x_cur, eval_rpn(&rpn, x_cur)?));
        x_cur += step_size;
    }
    Ok(EquationData { points })
}

#[derive(Debug, PartialEq)]
pub struct EquationData {
    pub points: Vec<(f32, f32)>,
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;
    use super::*;

    fn is_close(x1: f32, x2: f32) -> bool {
        (x1 - x2).abs() < 0.00001
    }

    #[test]
    fn get_eq_data_test_quad() {
        let test_eq = "y = x ^ 2 + 2 * x + 1";
        let ans = vec![(-1_f32, 0_f32), (0_f32, 1_f32), (1_f32, 4_f32)];

        assert_eq!(get_eq_data(test_eq, -1f32, 1_f32, 1_f32).unwrap().points, ans);
    }

    #[test]
    fn get_eq_data_test_sin() {
        let test_eq = "y = sin ( x )";
        let expected = vec![(-PI, 0_f32), (-PI / 2_f32, -1_f32), (0_f32, 0_f32) , (PI / 2_f32, 1_f32), (PI, 0_f32)];

        let actual = get_eq_data(test_eq, -PI, PI, PI / 2_f32).unwrap().points;

        for ((x_1, y_1),(x_2, y_2)) in actual.iter().zip(expected) {
            assert!(is_close(*x_1,x_2));
            assert!(is_close(*y_1, y_2));
        }
    }

    #[test]
    fn get_eq_data_test_cos() {
        let test_eq = "y = cos ( x + π )";
        let expected = vec![(-PI, 1_f32), (-PI / 2_f32, 0_f32), (0_f32, -1_f32) , (PI / 2_f32, 0_f32), (PI, 1_f32)];

        let actual = get_eq_data(test_eq, -PI, PI, PI / 2_f32).unwrap().points;

        for ((x_1, y_1),(x_2, y_2)) in actual.iter().zip(expected) {
            assert!(is_close(*x_1,x_2));
            assert!(is_close(*y_1, y_2));
        }
    }

    #[test]
    fn get_eq_data_test_sqrt() {
        let test_eq = "y = sqrt ( x ^ 2 )";
        let expected = vec![(2_f32, 2_f32), (2.25_f32, 2.25_f32), (2.5_f32, 2.5_f32) , (2.75_f32, 2.75_f32), (3_f32, 3_f32)];

        let actual = get_eq_data(test_eq, 2_f32, 3_f32, 0.25_f32).unwrap().points;

        for ((x_1, y_1),(x_2, y_2)) in actual.iter().zip(expected) {
            assert!(is_close(*x_1,x_2));
            assert!(is_close(*y_1, y_2));
        }
    }

    #[test]
    fn get_eq_data_test_log() {
        let test_eq = "y = log_10 ( 10 ^ x ) + x";
        let expected = vec![(1_f32, 2_f32), (2_f32, 4_f32),
                                      (3_f32, 6_f32), (4_f32, 8_f32),
                                      (5_f32, 10_f32), (6_f32, 12_f32),
                                      (7_f32, 14_f32), (8_f32, 16_f32),
                                      (9_f32, 18_f32), (10_f32, 20_f32)];

        let actual = get_eq_data(test_eq, 1_f32, 10_f32, 1_f32).unwrap().points;

        println!("{:?}", actual);

        for ((x_1, y_1),(x_2, y_2)) in actual.iter().zip(expected) {
            assert_eq!(*x_1,x_2);
            assert_eq!(*y_1, y_2);
        }
    }

}


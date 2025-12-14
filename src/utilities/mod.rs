use crate::linear_algebra::Vector;
use rand::distributions::Uniform;
use rand::Rng;

/// Calculates the accuracy of a classification model
///
/// Accuracy = (TP + TN) / (TP + FP + FN + TN)
///
/// # Arguments
///
/// * `t_p` - True positives
/// * `f_p` - False positives
/// * `f_n` - False negatives
/// * `t_n` - True negatives
///
/// # Returns
///
/// The accuracy as a value between 0.0 and 1.0, or 0.0 if total is zero
///
/// # Examples
///
/// ```
/// use rusty_maths::utilities::accuracy;
///
/// let acc = accuracy(70, 4930, 13930, 981070);
/// assert_eq!(acc, 0.98114);
/// ```
pub fn accuracy(t_p: isize, f_p: isize, f_n: isize, t_n: isize) -> f64 {
    let correct = t_p + t_n;
    let total = t_p + f_p + f_n + t_n;
    if total == 0 {
        return 0.0;
    }
    correct as f64 / total as f64
}

/// Calculates the precision of a classification model
///
/// Precision = TP / (TP + FP)
///
/// Measures the proportion of positive predictions that were correct
///
/// # Arguments
///
/// * `t_p` - True positives
/// * `f_p` - False positives
///
/// # Returns
///
/// The precision as a value between 0.0 and 1.0, or 0.0 if denominator is zero
///
/// # Examples
///
/// ```
/// use rusty_maths::utilities::precision;
///
/// let p = precision(70, 4930);
/// assert_eq!(p, 0.014);
/// ```
pub fn precision(t_p: isize, f_p: isize) -> f64 {
    let denominator = t_p + f_p;
    if denominator == 0 {
        return 0.0;
    }
    t_p as f64 / denominator as f64
}

/// Calculates the recall (sensitivity) of a classification model
///
/// Recall = TP / (TP + FN)
///
/// Measures the proportion of actual positives that were correctly identified
///
/// # Arguments
///
/// * `t_p` - True positives
/// * `f_n` - False negatives
///
/// # Returns
///
/// The recall as a value between 0.0 and 1.0, or 0.0 if denominator is zero
///
/// # Examples
///
/// ```
/// use rusty_maths::utilities::recall;
///
/// let r = recall(70, 13930);
/// assert_eq!(r, 0.005);
/// ```
pub fn recall(t_p: isize, f_n: isize) -> f64 {
    let denominator = t_p + f_n;
    if denominator == 0 {
        return 0.0;
    }
    t_p as f64 / denominator as f64
}

/// Calculates the F1 score of a classification model
///
/// F1 = 2 * (Precision * Recall) / (Precision + Recall)
///
/// The F1 score is the harmonic mean of precision and recall
///
/// # Arguments
///
/// * `t_p` - True positives
/// * `f_p` - False positives
/// * `f_n` - False negatives
///
/// # Returns
///
/// The F1 score as a value between 0.0 and 1.0, or 0.0 if denominator is zero
///
/// # Examples
///
/// ```
/// use rusty_maths::utilities::f1_score;
///
/// let f1 = f1_score(100, 0, 0);
/// assert_eq!(f1, 1.0);
/// ```
pub fn f1_score(t_p: isize, f_p: isize, f_n: isize) -> f64 {
    let p = precision(t_p, f_p);
    let r = recall(t_p, f_n);
    let denominator = p + r;

    if denominator == 0.0 {
        return 0.0;
    }

    2_f64 * p * r / denominator
}

pub fn train_test_split<X: Clone, Y: Clone>(
    xs: &[X],
    ys: &[Y],
    test_pct: f64,
) -> (Vec<X>, Vec<X>, Vec<Y>, Vec<Y>) {
    let mut idxs = vec![];
    for i in 0..xs.len() {
        idxs.push(i);
    }

    let (train_idxs, test_idxs) = split_data(&idxs, 1_f64 - test_pct);

    let mut x_train = vec![];
    let mut x_test = vec![];
    let mut y_train = vec![];
    let mut y_test = vec![];

    for i in train_idxs {
        x_train.push(xs[i].clone());
        y_train.push(ys[i].clone());
    }

    for i in test_idxs {
        x_test.push(xs[i].clone());
        y_test.push(ys[i].clone());
    }

    (x_train, x_test, y_train, y_test)
}

///Returns a sorted copy of a Vector
pub fn sort_vec_cop(v: &Vector) -> Vector {
    let mut v_c = v.to_vec();
    v_c.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    v_c
}

///Split data into fractions [prob, 1 - prob]
pub fn split_data<T: Clone>(data: &[T], prob: f64) -> (Vec<T>, Vec<T>) {
    let shuffled = shuffle_vector(data);
    let cut = (data.len() as f64 * prob).floor() as usize;

    let front = shuffled[..cut].to_vec();
    let back = shuffled[cut..].to_vec();

    (front, back)
}

///Returns a shuffled version of passed Vec
pub fn shuffle_vector<T: Clone>(v: &[T]) -> Vec<T> {
    let mut v_clone = v.to_vec();
    let n = v.len();
    let mut rng = rand::thread_rng();
    for i in 0..(n - 1) {
        let uniform = Uniform::from(i..n);
        let j = rng.sample(uniform);
        v_clone.swap(i, j);
    }
    v_clone
}

///Returns the absolute value of an f64
///
/// ```
///# use rusty_maths::utilities::abs;
///assert_eq!(abs(-747_f64), 747_f64);
///assert_eq!(abs(-45.43), 45.43);
///assert_eq!(abs(101.41), 101.41);
/// ```
pub fn abs(num: f64) -> f64 {
    if num < 0.0 {
        return -num;
    }
    num
}

///Returns the absolute value of an f32
///
/// ```
///# use rusty_maths::utilities::abs_f32;
///assert_eq!(abs_f32(-747_f32), 747_f32);
///assert_eq!(abs_f32(-45.43), 45.43);
///assert_eq!(abs_f32(101.41), 101.41);
/// ```
pub fn abs_f32(num: f32) -> f32 {
    if num < 0.0 {
        return -num;
    }
    num
}

///Returns the square root of an f32
///
/// ```
///# use rusty_maths::utilities::{abs_f32, square_root_f32};///
///assert_eq!(square_root_f32(625_f32), 25_f32);
///assert!(abs_f32(square_root_f32(1.23456) - f32::sqrt(1.23456)) <= 0.0001);
/// ```
pub fn square_root_f32(num: f32) -> f32 {
    if num == 0.0 {
        return 0.0;
    }

    // Use exponential search to find bounds quickly
    if num >= 1.0 {
        // For num >= 1, double until we overshoot
        let mut i = 1_f32;
        while i * i < num {
            i *= 2_f32;
        }
        if i * i == num {
            i
        } else {
            square_f32(num, i / 2_f32, i)
        }
    } else {
        // For num < 1, halve until we undershoot
        let mut i = 1_f32;
        while i * i > num {
            i /= 2_f32;
        }
        if i * i == num {
            i
        } else {
            square_f32(num, i, i * 2_f32)
        }
    }
}

fn square_f32(num: f32, i: f32, j: f32) -> f32 {
    let mid = (i + j) / 2_f32;
    let mul = mid * mid;

    // Check if we've converged: exact match, bounds are close, or hit floating point precision limit
    if mul == num || abs_f32(j - i) < 0.00001 || mid == i || mid == j {
        mid
    } else if mul < num {
        square_f32(num, mid, j)
    } else {
        square_f32(num, i, mid)
    }
}

///Returns the square root of an f64
///
/// ```
///# use rusty_maths::utilities::{abs, square_root};///
///assert_eq!(square_root(625_f64), 25_f64);
///assert!(abs(square_root(1.23456789) - f64::sqrt(1.23456789)) <= 0.0000001);
/// ```
pub fn square_root(num: f64) -> f64 {
    if num == 0.0 {
        return 0.0;
    }

    // Use exponential search to find bounds quickly
    if num >= 1.0 {
        // For num >= 1, double until we overshoot
        let mut i = 1_f64;
        while i * i < num {
            i *= 2_f64;
        }
        if i * i == num {
            i
        } else {
            square(num, i / 2_f64, i)
        }
    } else {
        // For num < 1, halve until we undershoot
        let mut i = 1_f64;
        while i * i > num {
            i /= 2_f64;
        }
        if i * i == num {
            i
        } else {
            square(num, i, i * 2_f64)
        }
    }
}

fn square(num: f64, i: f64, j: f64) -> f64 {
    let mid = (i + j) / 2_f64;
    let mul = mid * mid;

    // Check if we've converged: exact match, bounds are close, or hit floating point precision limit
    if mul == num || abs(j - i) < 0.000000001 || mid == i || mid == j {
        mid
    } else if mul < num {
        square(num, mid, j)
    } else {
        square(num, i, mid)
    }
}

/// Computes the factorial of a number
///
/// # Panics
///
/// Panics if `num` is negative or greater than 20 (to prevent overflow)
pub(crate) fn factorial(num: isize) -> isize {
    if num < 0 {
        panic!("factorial is not defined for negative numbers");
    }
    if num > 20 {
        panic!("factorial overflow: maximum supported value is 20");
    }
    let mut ans = 1;
    for i in 1..=num {
        ans *= i;
    }
    ans
}

//TODO: Move these to quadratic_analysis.rs
///Solves for x in ax² + bx + c = 0
///
/// Returns an Option<(f32, f32), String>
///
/// ```
///# use rusty_maths::utilities::quadratic_eq_f32;
///assert_eq!(quadratic_eq_f32(2.0, 3.0, -5.0).unwrap(), (1.0, -2.5));
///
///assert_eq!(quadratic_eq_f32(-0.5, 1.0, -0.5).unwrap().0, 1_f32);
///assert!(quadratic_eq_f32(-0.5, 1.0, -0.5).unwrap().1.is_nan());
///
///assert_eq!(quadratic_eq_f32(-1_f32, 0_f32, -1_f32).unwrap_err(), "No Real Solutions");
/// ```
pub fn quadratic_eq_f32(a: f32, b: f32, c: f32) -> Result<(f32, f32), String> {
    let neg_b = -b;
    let b_sq = b * b;
    let four_a_c = 4_f32 * a * c;
    let two_a = 2_f32 * a;
    if b_sq - four_a_c < 0_f32 {
        return Err(String::from("No Real Solutions"));
    }
    if b_sq - four_a_c == 0_f32 {
        return Ok((neg_b / two_a, f32::NAN));
    }
    let sqrt__ = square_root_f32(b_sq - four_a_c);
    Ok(((neg_b + sqrt__) / two_a, (neg_b - sqrt__) / two_a))
}

///Solves for x in ax² + bx + c = 0
///
/// Returns an Option<(f64, f64), String>
///
/// ```
///# use rusty_maths::utilities::quadratic_eq;
///assert_eq!(quadratic_eq(2.0, 3.0, -5.0).unwrap(), (1.0, -2.5));
///
///assert_eq!(quadratic_eq(-0.5, 1.0, -0.5).unwrap().0, 1_f64);
///assert!(quadratic_eq(-0.5, 1.0, -0.5).unwrap().1.is_nan());
///
///assert_eq!(quadratic_eq(-1_f64, 0_f64, -1_f64).unwrap_err(), "No Real Solutions");
/// ```
pub fn quadratic_eq(a: f64, b: f64, c: f64) -> Result<(f64, f64), String> {
    let neg_b = -b;
    let b_sq = b * b;
    let four_a_c = 4_f64 * a * c;
    let two_a = 2_f64 * a;
    if b_sq - four_a_c < 0_f64 {
        return Err(String::from("No Real Solutions"));
    }
    if b_sq - four_a_c == 0_f64 {
        return Ok((neg_b / two_a, f64::NAN));
    }
    let sqrt__ = square_root(b_sq - four_a_c);
    Ok(((neg_b + sqrt__) / two_a, (neg_b - sqrt__) / two_a))
}

pub fn get_str_section(str: &str, start: usize, end: usize) -> String {
    str.chars().skip(start).take(end - start).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadratic_eq_test() {
        //Two real solutions
        assert_eq!(quadratic_eq(-2.0, 1.0, 6.0).unwrap(), (-1.5, 2.0));

        //High precision
        //assert_eq!(quadratic_eq(-0.25, 1.3, 4.7).unwrap(), (-2.456, 7.656));
        assert!(abs(quadratic_eq(-0.25, 1.3, 4.7).unwrap().0 + 2.456) < 0.001);
        assert!(abs(quadratic_eq(-0.25, 1.3, 4.7).unwrap().1 - 7.656) < 0.001);

        //one real solution, first item is solution, second item in tuple will be NaN
        assert_eq!(quadratic_eq(-2.0, 2.0, -0.5).unwrap().0, 0.5);
        assert!(quadratic_eq(-2.0, 2.0, -0.5).unwrap().1.is_nan());

        //no real solutions, this does not handle imaginary values    yet.....
        //Todo: Implement imaginary numbers
        assert_eq!(
            quadratic_eq(-2.0, 2.0, -2.0).unwrap_err(),
            "No Real Solutions"
        );
    }

    #[test]
    fn quadratic_eq_f32_test() {
        //Two real solutions
        assert_eq!(quadratic_eq_f32(-2.0, 1.0, 6.0).unwrap(), (-1.5, 2.0));

        //one real solution, first item is solution, second item in tuple will be NaN
        assert_eq!(quadratic_eq_f32(-2.0, 2.0, -0.5).unwrap().0, 0.5);
        assert!(quadratic_eq_f32(-2.0, 2.0, -0.5).unwrap().1.is_nan());

        //no real solutions, this does not handle imaginary values    yet.....
        //Todo: Implement imaginary numbers
        assert_eq!(
            quadratic_eq_f32(-2.0, 2.0, -2.0).unwrap_err(),
            "No Real Solutions"
        );
    }

    #[test]
    fn square_root_test() {
        assert_eq!(square_root(144_f64), 12_f64);
        assert_eq!(square_root(1764_f64), 42_f64);
        assert!(abs(square_root(14.5) - f64::sqrt(14.5)) <= 0.0000001);
        assert!(abs(square_root(214.532) - f64::sqrt(214.532)) <= 0.0000001);
    }

    #[test]
    fn split_data_test() {
        let mut data = vec![];
        for n in 0..1000 {
            data.push(n as f64);
        }

        let (mut train, mut test) = split_data(&data, 0.75);

        assert_eq!(train.len(), 750);
        assert_eq!(test.len(), 250);

        train.append(&mut test);

        assert_eq!(sort_vec_cop(&train), data);
    }

    #[test]
    fn test_train_test() {
        let mut xs = vec![];
        let mut ys = vec![];

        for n in 0..1000 {
            xs.push(n as f64);
            ys.push((n * 2) as f64);
        }

        let (x_tr, x_tst, y_tr, y_tst) = train_test_split(&xs, &ys, 0.25);

        assert_eq!(x_tr.len(), 750);
        assert_eq!(x_tr.len(), y_tr.len());

        assert_eq!(x_tst.len(), 250);
        assert_eq!(x_tst.len(), y_tst.len());

        assert!(x_tr.iter().zip(y_tr).all(|x| *x.0 == x.1 / 2_f64));
        assert!(x_tst.iter().zip(y_tst).all(|x| *x.0 == x.1 / 2_f64));
    }

    #[test]
    fn accuracy_test() {
        assert_eq!(accuracy(70, 4930, 13930, 981070), 0.98114);
    }

    #[test]
    fn precision_test() {
        assert_eq!(precision(70, 4930), 0.014);
    }

    #[test]
    fn recall_test() {
        assert_eq!(recall(70, 13930), 0.005);
    }

    #[test]
    fn f1_score_test() {
        // Normal case
        // precision(70, 4930) = 70 / 5000 = 0.014
        // recall(70, 13930) = 70 / 14000 = 0.005
        // f1 = 2 * (0.014 * 0.005) / (0.014 + 0.005) ≈ 0.00737
        let f1 = f1_score(70, 4930, 13930);
        assert!(f1 > 0.007 && f1 < 0.008);

        // Perfect precision and recall
        assert_eq!(f1_score(100, 0, 0), 1.0);

        // Edge case: all zeros should return 0
        assert_eq!(f1_score(0, 0, 0), 0.0);
    }

    #[test]
    fn precision_edge_cases_test() {
        // Division by zero case
        assert_eq!(precision(0, 0), 0.0);

        // Perfect precision
        assert_eq!(precision(100, 0), 1.0);

        // No true positives
        assert_eq!(precision(0, 100), 0.0);
    }

    #[test]
    fn recall_edge_cases_test() {
        // Division by zero case
        assert_eq!(recall(0, 0), 0.0);

        // Perfect recall
        assert_eq!(recall(100, 0), 1.0);

        // No true positives
        assert_eq!(recall(0, 100), 0.0);
    }

    #[test]
    fn accuracy_edge_cases_test() {
        // Division by zero case
        assert_eq!(accuracy(0, 0, 0, 0), 0.0);

        // Perfect accuracy
        assert_eq!(accuracy(100, 0, 0, 100), 1.0);

        // Zero accuracy
        assert_eq!(accuracy(0, 100, 100, 0), 0.0);
    }

    #[test]
    fn get_str_section_test() {
        let s = "hello world";
        assert_eq!(get_str_section(s, 0, 5), "hello");
        assert_eq!(get_str_section(s, 6, 11), "world");
        assert_eq!(get_str_section(s, 0, 0), "");
        assert_eq!(get_str_section(s, 3, 7), "lo w");
    }
}

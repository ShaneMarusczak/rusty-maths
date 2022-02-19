use crate::linear_algebra::{dot_product, sum_of_squares, Vector};
use crate::utilities::{sort_vec_cop, square_root};

///Returns the mean of a Vector
pub fn mean(v: &Vector) -> f64 {
    v.iter().fold(0 as f64, |acc, x| acc + x) / v.len() as f64
}

fn median_odd(v: &Vector) -> f64 {
    let v_sort = sort_vec_cop(v);

    v_sort[v_sort.len() / 2]
}

fn median_even(v: &Vector) -> f64 {
    let v_sort = sort_vec_cop(v);
    let hi_midp = v_sort.len() / 2;

    (v_sort[hi_midp - 1] + v_sort[hi_midp]) / 2_f64
}

///Returns the median of a Vector
pub fn median(v: &Vector) -> f64 {
    if v.len() % 2 == 0 {
        median_even(v)
    } else {
        median_odd(v)
    }
}

///Returns the pth-percentile value in v
pub fn quantile(v: &Vector, p: f64) -> f64 {
    let p_index = (p * v.len() as f64).floor() as usize;
    sort_vec_cop(v)[p_index]
}

pub fn data_range(v: &Vector) -> f64 {
    v.iter().fold(f64::NEG_INFINITY, |a, &b| a.max(b))
        - v.iter().fold(f64::INFINITY, |a, &b| a.min(b))
}

///Translates v by subtracting its mean (result has mean of 0)
pub fn de_mean(v: &Vector) -> Vector {
    let x_bar = mean(v);
    v.iter().map(|x| x - x_bar).collect()
}

pub fn variance(v: &Vector) -> f64 {
    let n = v.len();
    assert!(n >= 2, "variance requires at least two elements");

    let deviations = de_mean(v);
    sum_of_squares(&deviations) / (n - 1) as f64
}

pub fn standard_deviation(v: &Vector) -> f64 {
    square_root(variance(v))
}

pub fn interquartile_range(v: &Vector) -> f64 {
    quantile(v, 0.75) - quantile(v, 0.25)
}

pub fn covariance(v: &Vector, w: &Vector) -> f64 {
    assert_eq!(v.len(), w.len(), "vectors must be the same length");
    de_mean(v);
    dot_product(&de_mean(v), &de_mean(w)) / (v.len() - 1) as f64
}

pub fn correlation(v: &Vector, w: &Vector) -> f64 {
    let std_v = standard_deviation(v);
    let std_w = standard_deviation(w);

    if std_v > 0_f64 && std_w > 0_f64 {
        covariance(v, w) / std_v / std_w
    } else {
        0_f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_sort_copy_test() {
        let v = vec![9 as f64, 5 as f64, 3 as f64, 2 as f64, 1 as f64];
        let z = vec![1 as f64, 2 as f64, 3 as f64, 5 as f64, 9 as f64];
        let v_sort = sort_vec_cop(&v);
        assert_eq!(v_sort, z);
    }

    #[test]
    fn vec_sort_test() {
        let mut v = vec![9 as f64, 5 as f64, 3 as f64, 2 as f64, 1 as f64];
        let z = vec![1 as f64, 2 as f64, 3 as f64, 5 as f64, 9 as f64];
        let v = sort_vec_cop(&mut v);
        assert_eq!(v, z);
    }

    #[test]
    fn mean_test() {
        assert_eq!(
            mean(&vec![2 as f64, 5 as f64, 7 as f64, 10 as f64]),
            6 as f64
        );
    }

    #[test]
    fn median_test() {
        let v = vec![1 as f64, 10 as f64, 2 as f64, 9 as f64, 5 as f64];
        assert_eq!(median(&v), 5 as f64);

        let v_2 = vec![1 as f64, 9 as f64, 2 as f64, 10 as f64];
        assert_eq!(median(&v_2), (9 as f64 + 2 as f64) / 2 as f64);
    }

    #[test]
    fn quantile_test() {
        let v = vec![1 as f64, 2 as f64, 3 as f64, 4 as f64, 5 as f64];
        assert_eq!(quantile(&v, 0.45), 3 as f64);
        assert_eq!(quantile(&v, 0.7), 4 as f64);

        //works for odd length vectors
        assert_eq!(quantile(&v, 0.5), median(&v));
    }

    #[test]
    fn data_range_test() {
        let v = vec![1 as f64, 2 as f64, 3 as f64, 4 as f64, 5 as f64];
        assert_eq!(data_range(&v), 4 as f64);
    }

    #[test]
    fn de_mean_test() {
        let v = vec![1 as f64, 2 as f64, 3 as f64];
        assert_eq!(de_mean(&v), vec![-1 as f64, 0 as f64, 1 as f64]);
    }

    #[test]
    fn variance_test() {
        let v = vec![99 as f64, 85 as f64, 100 as f64];
        let var = variance(&v);
        assert!(70.32 < var && var < 70.34);
    }

    #[test]
    fn standard_deviation_test() {
        let v = vec![45 as f64, 32 as f64, 20 as f64];
        let st_dev = standard_deviation(&v);
        assert!(12.50 < st_dev && st_dev < 12.51);
    }

    #[test]
    fn interquartile_test() {
        let v = vec![
            10_f64, 11_f64, 234_f64, 23_f64, 210_f64, 100_f64, 99_f64, 156_f64,
        ];
        assert_eq!(interquartile_range(&v), 187 as f64)
    }

    #[test]
    fn covariance_test() {
        let v_1 = vec![
            1_f64, 3_f64, 2_f64, 5_f64, 8_f64, 7_f64, 12_f64, 2_f64, 4_f64,
        ];
        let v_2 = vec![
            8_f64, 6_f64, 9_f64, 4_f64, 3_f64, 3_f64, 2_f64, 7_f64, 7_f64,
        ];

        let cov = covariance(&v_1, &v_2);

        assert!(-8.07 < cov && cov < -8.06);
    }

    #[test]
    fn correlation_test() {
        let v_1 = vec![
            1_f64, 3_f64, 2_f64, 5_f64, 8_f64, 7_f64, 12_f64, 2_f64, 4_f64,
        ];
        let v_2 = vec![
            8_f64, 6_f64, 9_f64, 4_f64, 3_f64, 3_f64, 2_f64, 7_f64, 7_f64,
        ];

        let cov = correlation(&v_1, &v_2);

        assert!(-0.91 < cov && cov < -0.90);
    }
}

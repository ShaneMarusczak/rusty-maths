use std::f64;
use rand::distributions::Uniform;
use rand::Rng;
use crate::linear_algebra::Vector;

pub fn accuracy(t_p: isize, f_p: isize, f_n: isize, t_n: isize) -> f64 {
    let correct = t_p + t_n;
    let total = t_p + f_p + f_n + t_n;
    correct as f64 / total as f64
}

pub fn precision(t_p: isize, f_p: isize) -> f64 {
    t_p as f64 / (t_p as f64 + f_p as f64)
}

pub fn recall(t_p: isize, f_n: isize) -> f64 {
    t_p as f64 / (t_p as f64 + f_n as f64)
}

pub fn f1_score(t_p: isize, f_p: isize) -> f64 {
    let p = precision(t_p, f_p);
    let r = precision(t_p,f_p);

    2_f64 * p * r / (p + r)
}

pub fn train_test_split<X: Clone, Y: Clone>(xs: &Vec<X>, ys: &Vec<Y>, test_pct: f64) -> (Vec<X>, Vec<X>, Vec<Y>, Vec<Y>) {
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
    let mut slow_p : usize = 0;
    let mut fast_p: usize = 1;
    while slow_p < v_c.len() {
        while fast_p < v_c.len() {
            if v_c[fast_p] < v_c[slow_p] {
                let temp = v_c[slow_p];
                v_c[slow_p] = v_c[fast_p];
                v_c[fast_p] = temp;
            }
            fast_p += 1;
        }
        slow_p += 1;
        fast_p = slow_p + 1;
    }
    v_c
}

///Split data into fractions [prob, 1 - prob]
pub fn split_data<T: Clone>(data: &Vec<T>, prob: f64) -> (Vec<T>, Vec<T>) {
    let shuffled = shuffle_vector(&data);
    let cut = (data.len() as f64 * prob).floor() as usize;

    let front = shuffled[..cut].to_vec();
    let back = shuffled[cut..].to_vec();

    (front, back)
}

///Returns a shuffled version of passed Vec
pub fn shuffle_vector<T: Clone>(v: &Vec<T>) -> Vec<T> {
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
///assert_eq!(abs(-747 as f64), 747 as f64);
///assert_eq!(abs(-45.43), 45.43);
///assert_eq!(abs(101.41), 101.41);
/// ```
pub fn abs(num: f64) -> f64{
    if num < 0 as f64 {
        return -num;
    }
    num
}

///Returns the square root of an f64
///
/// ```
///# use rusty_maths::utilities::{abs, square_root};///
///assert_eq!(square_root(625 as f64), 25 as f64);
///assert!(abs(square_root(1.23456789) - f64::sqrt(1.23456789)) <= 0.0000001);
/// ```
pub fn square_root(num: f64) -> f64 {
    let mut i = 1 as f64;

    loop {
        if i * i == num {
            return i;
        }
        else if i * i > num
        {
            return square(num, i - 1 as f64, i);
        }
        i += 1 as f64;
    }
}

fn square(num: f64, i: f64, j: f64) -> f64{
    let mid = (i + j) / 2 as f64;
    let mul = mid * mid;

    return if mul == num || abs(mul - num) < 0.0000001 {
        mid
    } else if mul < num {
        square(num, mid, j)
    } else {
        square(num, i, mid)
    }
}

///Solves for x in ax² + bx + c = 0
///
/// Returns an Option<(f64, f64), String>
///
/// ```
///# use rusty_maths::utilities::quadratic_eq;
///assert_eq!(quadratic_eq(2.0, 3.0, -5.0).unwrap(), (1.0, -2.5));
///
///assert_eq!(quadratic_eq(-0.5, 1.0, -0.5).unwrap().0, 1 as f64);
///assert!(quadratic_eq(-0.5, 1.0, -0.5).unwrap().1.is_nan());
///
///assert_eq!(quadratic_eq(-1 as f64, 0 as f64, -1 as f64).unwrap_err(), "No Real Solutions");
/// ```
pub fn quadratic_eq(a: f64, b: f64, c: f64) -> Result<(f64, f64), String>{
    let neg_b = -b;
    let b_sq = b * b;
    let four_a_c = 4 as f64 * a * c;
    let two_a = 2 as f64 * a;
    if b_sq - four_a_c < 0 as f64 {
        return Err(String::from("No Real Solutions"));
    }
    if b_sq - four_a_c == 0 as f64 {
        #[allow(deprecated)] return Ok((neg_b / two_a, f64::NAN ));
    }
    let sqrt__ = square_root(b_sq - four_a_c);
    Ok(( (neg_b + sqrt__) / two_a, (neg_b - sqrt__ ) / two_a))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quadratic_eq_test(){
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
        assert_eq!(quadratic_eq(-2.0, 2.0, -2.0).unwrap_err(), "No Real Solutions");
    }

    #[test]
    fn square_root_test(){
        assert_eq!(square_root(144 as f64), 12 as f64);
        assert_eq!(square_root( 1764 as f64), 42 as f64);
        assert!(abs(square_root(14.5) - f64::sqrt(14.5)) <= 0.0000001);
        assert!(abs(square_root(214.532) - f64::sqrt(214.532)) <= 0.0000001);
    }

    #[test]
    fn split_data_test(){
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
    fn test_train_test(){
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
    fn accuracy_test(){
        assert_eq!(accuracy(70, 4930, 13930, 981070), 0.98114);
    }

    #[test]
    fn precision_test(){
        assert_eq!(precision(70, 4930), 0.014);
    }

    #[test]
    fn recall_test(){
        assert_eq!(recall(70, 13930), 0.005);
    }
}
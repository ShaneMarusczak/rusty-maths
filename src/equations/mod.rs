use std::f64;

///Returns the absolute value of an f64
///
/// ```
///# use rusty_maths::equations::abs;
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
///# use rusty_maths::equations::{abs, square_root};///
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
///# use rusty_maths::equations::quadratic_eq;
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
        return Ok((neg_b / two_a , f64::NAN ));
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
}
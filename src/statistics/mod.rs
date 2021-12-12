type Vector = Vec<f64>;

///Returns the mean of a Vector
pub fn mean(v: &Vector) -> f64 {
    v.iter().fold(0 as f64, |acc, x| acc + x) / v.len() as f64
}

///Returns a sorted copy of a Vector
pub fn sort_vec_cop(v: &Vector) -> Vector {
    let mut v_c = v.clone();
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

///Sorts a vector in place
pub fn sort_vec(v: &mut Vector) {
    let mut slow_p : usize = 0;
    let mut fast_p: usize = 1;
    while slow_p < v.len() {
        while fast_p < v.len() {
            if v[fast_p] < v[slow_p] {
                let temp = v[slow_p];
                v[slow_p] = v[fast_p];
                v[fast_p] = temp;
            }
            fast_p += 1;
        }
        slow_p += 1;
        fast_p = slow_p + 1;
    }
}

fn median_odd(v: &Vector) -> f64 {
    let v_sort = sort_vec_cop(v);

    v_sort[v_sort.len() / 2]
}

fn median_even(v: &Vector) -> f64 {
    let v_sort = sort_vec_cop(v);
    let hi_midp = v_sort.len() / 2;

    (v_sort[hi_midp - 1] + v_sort[hi_midp]) / 2 as f64
}

///Returns the median of a Vector
pub fn median(v: &Vector) -> f64 {
    if v.len() % 2 == 0 {median_even(v)} else {median_odd(v)}
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
        sort_vec(&mut v);
        assert_eq!(v, z);
    }

    #[test]
    fn mean_test() {
        assert_eq!(mean(&vec![2 as f64, 5 as f64, 7 as f64, 10 as f64]), 6 as f64);
    }

    #[test]
    fn median_test() {
        let v = vec![1 as f64, 10 as f64, 2 as f64, 9 as f64, 5 as f64];
        assert_eq!(median(&v), 5 as f64);

        let v_2 = vec![1 as f64, 9 as f64, 2 as f64, 10 as f64];
        assert_eq!(median(&v_2), (9 as f64 + 2 as f64) / 2 as f64)
    }
}
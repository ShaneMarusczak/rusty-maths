use crate::utilities::square_root;

pub type Vector = Vec<f64>;
pub type Matrix = Vec<Vector>;

///Adds corresponding elements
pub fn vec_add(v: &Vector, w: &Vector) -> Vector {
    assert_eq!(v.len(), w.len(), "vectors must be the same length");
    v.iter().zip(w).map(|t| t.0 + t.1).collect()
}

///Subtracts corresponding elements
pub fn vec_subtract(v: &Vector, w: &Vector) -> Vector {
    assert_eq!(v.len(), w.len(), "vectors must be the same length");
    v.iter().zip(w).map(|t| t.0 - t.1).collect()
}

///Sums all corresponding elements
pub fn vector_sum(vectors: &Matrix) -> Vector {
    assert!(!vectors.is_empty(), "no vectors provided");
    let num_elems = vectors[0].len();
    assert!(
        vectors.iter().all(|v| v.len() == num_elems),
        "vectors must be the same length"
    );

    let mut vec: Vector = Vec::with_capacity(num_elems);

    for i in 0..num_elems {
        vec.push(0_f64);
        for v in vectors {
            vec[i] += v[i];
        }
    }
    vec
}

///Multiplies every element by C
pub fn scalar_multiply(c: f64, vector: &Vector) -> Vector {
    vector.iter().map(|n| c * n).collect()
}

///Computes element-wise average
pub fn vector_mean(vectors: &Matrix) -> Vector {
    let n = vectors.len();
    scalar_multiply(1_f64 / n as f64, &vector_sum(vectors))
}

///Computes v_1 * w_1 + ... + v_n * w_n
pub fn dot_product(v: &Vector, w: &Vector) -> f64 {
    assert_eq!(v.len(), w.len(), "vectors must be the same length");
    v.iter().zip(w).fold(0_f64, |acc, t| acc + (t.0 * t.1))
}

///Computes v_1 * v_1 + ... + v_n * v_n
pub fn sum_of_squares(v: &Vector) -> f64 {
    dot_product(v, v)
}

///Returns the magnitude (or length) of v
pub fn magnitude(v: &Vector) -> f64 {
    square_root(sum_of_squares(v))
}

///Computes (v_1 - w_1) ** 2 + ... + (v_n - w_n) ** 2
pub fn squared_distance(v: &Vector, w: &Vector) -> f64 {
    sum_of_squares(&vec_subtract(v, w))
}

///Computes the distance between v and w
pub fn distance(v: &Vector, w: &Vector) -> f64 {
    magnitude(&vec_subtract(v, w))
}

///Returns (number of rows, number of columns) in m
pub fn shape(m: Matrix) -> (usize, usize) {
    (m.len(), if !m.is_empty() { m[0].len() } else { 0 })
}

///Returns a reference to the i-th row of m
pub fn get_row(m: &Matrix, i: usize) -> &Vector {
    &m[i]
}

///Returns the j-th column of m as a new Vector
pub fn get_column(m: &Matrix, j: usize) -> Vector {
    m.iter().map(|v| v[j]).collect()
}

///Returns a num_r x num_cols matrix
/// whose (i,j)-th entry is entry_fn(i, j)
pub fn make_matrix(num_r: usize, num_c: usize, entry_fn: &dyn Fn((usize, usize)) -> f64) -> Matrix {
    let mut m: Matrix = vec![];
    for i in 0..num_r {
        m.push(vec![]);
        for j in 0..num_c {
            m[i].push(entry_fn((i, j)));
        }
    }
    m
}

///Returns the n x n identity matrix
pub fn identity_matrix(n: usize) -> Matrix {
    make_matrix(n, n, &|(i, j)| if i == j { 1 } else { 0 } as f64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec_add_test() {
        let v: Vector = vec![1_f64, 2_f64, 3_f64];
        let w: Vector = vec![4_f64, 5_f64, 6_f64];
        let z: Vector = vec![5_f64, 7_f64, 9_f64];

        assert_eq!(vec_add(&v, &w), z);
    }

    #[test]
    fn vec_sub_test() {
        let v: Vector = vec![5_f64, 7_f64, 9_f64];
        let w: Vector = vec![4_f64, 5_f64, 6_f64];
        let z: Vector = vec![1_f64, 2_f64, 3_f64];

        assert_eq!(vec_subtract(&v, &w), z);
    }

    #[test]
    fn vector_sum_test() {
        let v1: Vector = vec![1_f64, 2_f64];
        let v2: Vector = vec![3_f64, 4_f64];
        let v3: Vector = vec![5_f64, 6_f64];
        let v4: Vector = vec![7_f64, 8_f64];

        let vectors: Matrix = vec![v1, v2, v3, v4];

        let v5: Vector = vec![16_f64, 20_f64];

        assert_eq!(vector_sum(&vectors), v5);
    }

    #[test]
    fn scalar_multiply_test() {
        let c = 2_f64;
        let vector = vec![1_f64, 2_f64, 3_f64];

        let z = vec![2_f64, 4_f64, 6_f64];

        assert_eq!(scalar_multiply(c, &vector), z);
    }

    #[test]
    fn vector_mean_test() {
        let v1 = vec![1_f64, 2_f64];
        let v2 = vec![3_f64, 4_f64];
        let v3 = vec![5_f64, 6_f64];

        let vectors = vec![v1, v2, v3];

        let z = vec![3_f64, 4_f64];

        assert_eq!(vector_mean(&vectors), z);
    }

    #[test]
    fn dot_product_test() {
        let v = vec![1_f64, 2_f64, 3_f64];
        let w = vec![4_f64, 5_f64, 6_f64];

        assert_eq!(dot_product(&v, &w), 32_f64);
    }

    #[test]
    fn sum_of_squares_test() {
        let v: Vector = vec![1_f64, 2_f64, 3_f64];

        assert_eq!(sum_of_squares(&v), 14_f64);
    }

    #[test]
    fn magnitude_test() {
        let v: Vector = vec![3_f64, 4_f64];

        assert_eq!(magnitude(&v), 5_f64);
    }

    #[test]
    fn shape_test() {
        let m: Matrix = vec![vec![1_f64, 2_f64, 3_f64], vec![4_f64, 5_f64, 6_f64]];

        assert_eq!(shape(m), (2_usize, 3_usize));
    }

    #[test]
    fn get_row_test() {
        let r1 = vec![1_f64, 2_f64, 3_f64];
        let r2 = vec![4_f64, 5_f64, 6_f64];
        let m: Matrix = vec![r1, r2];

        assert_eq!(get_row(&m, 1), &vec![4_f64, 5_f64, 6_f64]);
    }

    #[test]
    fn get_column_test() {
        let m: Matrix = vec![vec![1_f64, 2_f64, 3_f64], vec![4_f64, 5_f64, 6_f64]];

        assert_eq!(get_column(&m, 1), vec![2_f64, 5_f64]);
    }

    #[test]
    fn identity_matrix_test() {
        let zero = 0_f64;
        let one = 1_f64;
        let n = 5;
        let m: Matrix = vec![
            vec![one, zero, zero, zero, zero],
            vec![zero, one, zero, zero, zero],
            vec![zero, zero, one, zero, zero],
            vec![zero, zero, zero, one, zero],
            vec![zero, zero, zero, zero, one],
        ];

        assert_eq!(identity_matrix(n), m);
    }
}

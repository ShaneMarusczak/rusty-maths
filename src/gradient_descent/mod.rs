use crate::linear_algebra::{dot_product, scalar_multiply, vec_add, Vector};
use crate::utilities::shuffle_vector;

const H_RES: f64 = 0.0001;

///Computes the sum of squared elements in v
pub fn sum_of_squares(v: &Vector) -> f64 {
    dot_product(v, v)
}

/// Approximates the derivative of a function using the difference quotient
///
/// # Arguments
///
/// * `f` - The function to differentiate
/// * `x` - The point at which to approximate the derivative
/// * `h` - A small step size for the approximation
///
/// # Returns
///
/// The approximate derivative f'(x) ≈ (f(x + h) - f(x)) / h
pub fn difference_quotient(f: &dyn Fn(f64) -> f64, x: f64, h: f64) -> f64 {
    (f(x + h) - f(x)) / h
}

/// Returns the i-th partial difference quotient of f at v
///
/// Computes the partial derivative with respect to the i-th variable
///
/// # Arguments
///
/// * `f` - The multivariate function to differentiate
/// * `v` - The point at which to approximate the partial derivative
/// * `i` - The index of the variable with respect to which to differentiate
/// * `h` - A small step size for the approximation
pub fn partial_difference_quotient(f: &dyn Fn(Vector) -> f64, v: &Vector, i: usize, h: f64) -> f64 {
    let f_v = f(v.to_owned());
    let w = v
        .iter()
        .enumerate()
        .map(|(j, v_j)| v_j + if j == i { h } else { 0_f64 })
        .collect::<Vec<f64>>();
    (f(w) - f_v) / h
}

/// Estimates the gradient of a multivariate function at a point
///
/// Returns a vector of partial derivatives, one for each input dimension
///
/// # Arguments
///
/// * `f` - The multivariate function
/// * `v` - The point at which to estimate the gradient
pub fn estimate_gradient(f: &dyn Fn(Vector) -> f64, v: &Vector) -> Vector {
    let mut vec = vec![];
    for i in 0..v.len() {
        vec.push(partial_difference_quotient(f, v, i, H_RES));
    }
    vec
}

///Moves 'step_size' in the 'gradient' direction from v
pub fn gradient_step(v: &Vector, gradient: &Vector, step_size: f64) -> Vector {
    assert_eq!(v.len(), gradient.len(), "vectors must be the same length");
    let step = scalar_multiply(step_size, gradient);
    vec_add(v, &step)
}

/// Computes the gradient of the sum of squares function
///
/// For f(v) = v₁² + v₂² + ... + vₙ², returns ∇f(v) = [2v₁, 2v₂, ..., 2vₙ]
pub fn sum_of_squares_gradient(v: &Vector) -> Vector {
    v.iter().map(|v_i| 2_f64 * v_i).collect()
}

/// Computes the gradient for linear regression at a single data point
///
/// For a linear model y = slope * x + intercept, computes the gradient
/// of the squared error with respect to [slope, intercept]
///
/// # Arguments
///
/// * `x` - The input feature value
/// * `y` - The target value
/// * `theta` - The parameters [slope, intercept]
///
/// # Returns
///
/// The gradient vector [∂error/∂slope, ∂error/∂intercept]
pub fn linear_gradient(x: f64, y: f64, theta: &Vector) -> Vector {
    let slope = theta[0];
    let intercept = theta[1];

    let predicted = slope.mul_add(x, intercept);

    let error = predicted - y;

    vec![2_f64 * error * x, 2_f64 * error]
}

/// Splits a dataset into mini-batches for stochastic gradient descent
///
/// # Arguments
///
/// * `data_set` - The complete dataset to split
/// * `batch_size` - The size of each mini-batch
/// * `shuffle` - Whether to shuffle the batch order
///
/// # Returns
///
/// A vector of mini-batches, each containing `batch_size` elements
///
/// # Note
///
/// If the dataset size is not evenly divisible by batch_size,
/// the last partial batch is dropped
pub fn mini_batches<T: Clone>(data_set: &[T], batch_size: usize, shuffle: bool) -> Vec<Vec<T>> {
    let mut batch_starts = vec![];
    for start in 0..data_set.len() {
        if start % batch_size == 0 {
            batch_starts.push(start);
        }
    }
    if shuffle {
        batch_starts = shuffle_vector(&batch_starts);
    }
    let mut rv: Vec<Vec<T>> = vec![];
    for start in batch_starts {
        let end = start + batch_size;
        rv.push(data_set[start..end].to_vec());
    }
    rv
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::linear_algebra::{distance, vector_mean};
    use rand::Rng;

    const LEARNING_RATE: f64 = 0.001;

    fn get_inputs() -> Vec<(f64, f64)> {
        let mut inputs = vec![];
        for x in -50..50 {
            inputs.push((x as f64, (20 * x + 5) as f64));
        }
        inputs
    }

    #[test]
    fn gradient_test() {
        let mut v = vec![];
        let mut rng = rand::thread_rng();

        for _ in 0..3 {
            v.push(rng.gen_range(-10..10) as f64);
        }

        for _ in 0..1000 {
            let grad = sum_of_squares_gradient(&v);
            v = gradient_step(&v, &grad, -0.01);
        }

        assert!(distance(&v, &vec![0_f64, 0_f64, 0_f64]) < 0.001);
    }

    #[test]
    fn linear_gradient_test() {
        let inputs = get_inputs();

        let mut rng = rand::thread_rng();

        let mut theta = vec![rng.gen_range(-1_f64..1_f64), rng.gen_range(-1_f64..1_f64)];

        for _ in 0..5000 {
            let mut l_g = vec![];
            for (x, y) in &inputs {
                l_g.push(linear_gradient(*x, *y, &theta));
            }

            let grad = vector_mean(&l_g);

            theta = gradient_step(&theta, &grad, -LEARNING_RATE);
        }

        let slope = theta[0];
        let intercept = theta[1];

        assert!(19.9 < slope && slope < 20.1);
        assert!(4.9 < intercept && intercept < 5.1);
    }

    #[test]
    fn mini_batch_test() {
        let inputs = get_inputs();

        let mut rng = rand::thread_rng();

        let mut theta = vec![rng.gen_range(-1_f64..1_f64), rng.gen_range(-1_f64..1_f64)];

        for _ in 0..1000 {
            let m_b: Vec<Vec<(f64, f64)>> = mini_batches(&inputs, 20, true);
            for batch in m_b {
                let mut l_g = vec![];
                for (x, y) in &batch {
                    l_g.push(linear_gradient(*x, *y, &theta));
                }

                let grad = vector_mean(&l_g);

                theta = gradient_step(&theta, &grad, -LEARNING_RATE);
            }
        }

        let slope = theta[0];
        let intercept = theta[1];

        assert!(19.9 < slope && slope < 20.1);
        assert!(4.9 < intercept && intercept < 5.1);
    }

    #[test]
    fn mini_batch_test_stochastic() {
        let inputs = get_inputs();

        let mut rng = rand::thread_rng();

        let mut theta = vec![rng.gen_range(-1_f64..1_f64), rng.gen_range(-1_f64..1_f64)];

        for _ in 0..100 {
            for (x, y) in &inputs {
                let grad = linear_gradient(*x, *y, &theta);
                theta = gradient_step(&theta, &grad, -LEARNING_RATE);
            }
        }

        let slope = theta[0];
        let intercept = theta[1];

        assert!(19.9 < slope && slope < 20.1);
        assert!(4.9 < intercept && intercept < 5.1);
    }
}

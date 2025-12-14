use crate::linear_algebra::Vector;

/// Trait for activation functions
pub trait Activation: Clone {
    /// Apply the activation function
    fn activate(&self, x: f64) -> f64;

    /// Compute the derivative of the activation function
    fn derivative(&self, x: f64) -> f64;

    /// Apply activation to entire vector
    fn activate_vector(&self, v: &Vector) -> Vector {
        v.iter().map(|&x| self.activate(x)).collect()
    }

    /// Apply derivative to entire vector
    fn derivative_vector(&self, v: &Vector) -> Vector {
        v.iter().map(|&x| self.derivative(x)).collect()
    }
}

/// ReLU (Rectified Linear Unit) activation function
///
/// f(x) = max(0, x)
/// f'(x) = 1 if x > 0, else 0
#[derive(Clone, Copy, Debug)]
pub struct ReLU;

impl Activation for ReLU {
    fn activate(&self, x: f64) -> f64 {
        x.max(0.0)
    }

    fn derivative(&self, x: f64) -> f64 {
        if x > 0.0 {
            1.0
        } else {
            0.0
        }
    }
}

/// Sigmoid activation function
///
/// f(x) = 1 / (1 + e^(-x))
/// f'(x) = f(x) * (1 - f(x))
#[derive(Clone, Copy, Debug)]
pub struct Sigmoid;

impl Activation for Sigmoid {
    fn activate(&self, x: f64) -> f64 {
        1.0 / (1.0 + (-x).exp())
    }

    fn derivative(&self, x: f64) -> f64 {
        let fx = self.activate(x);
        fx * (1.0 - fx)
    }
}

/// Tanh (Hyperbolic Tangent) activation function
///
/// f(x) = tanh(x)
/// f'(x) = 1 - tanh²(x)
#[derive(Clone, Copy, Debug)]
pub struct Tanh;

impl Activation for Tanh {
    fn activate(&self, x: f64) -> f64 {
        x.tanh()
    }

    fn derivative(&self, x: f64) -> f64 {
        let tanh_x = x.tanh();
        1.0 - tanh_x * tanh_x
    }
}

/// Linear activation function (identity)
///
/// f(x) = x
/// f'(x) = 1
#[derive(Clone, Copy, Debug)]
pub struct Linear;

impl Activation for Linear {
    fn activate(&self, x: f64) -> f64 {
        x
    }

    fn derivative(&self, _x: f64) -> f64 {
        1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relu_test() {
        let relu = ReLU;
        assert_eq!(relu.activate(5.0), 5.0);
        assert_eq!(relu.activate(-5.0), 0.0);
        assert_eq!(relu.activate(0.0), 0.0);

        assert_eq!(relu.derivative(5.0), 1.0);
        assert_eq!(relu.derivative(-5.0), 0.0);
    }

    #[test]
    fn sigmoid_test() {
        let sigmoid = Sigmoid;
        assert!(sigmoid.activate(0.0) > 0.49 && sigmoid.activate(0.0) < 0.51);
        assert!(sigmoid.activate(100.0) > 0.99);
        assert!(sigmoid.activate(-100.0) < 0.01);

        let deriv = sigmoid.derivative(0.0);
        assert!(deriv > 0.24 && deriv < 0.26);
    }

    #[test]
    fn tanh_test() {
        let tanh = Tanh;
        assert!(tanh.activate(0.0) < 0.01 && tanh.activate(0.0) > -0.01);
        assert!(tanh.activate(100.0) > 0.99);
        assert!(tanh.activate(-100.0) < -0.99);

        assert_eq!(tanh.derivative(0.0), 1.0);
    }

    #[test]
    fn linear_test() {
        let linear = Linear;
        assert_eq!(linear.activate(5.0), 5.0);
        assert_eq!(linear.activate(-5.0), -5.0);
        assert_eq!(linear.derivative(100.0), 1.0);
    }
}

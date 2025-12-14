use crate::linear_algebra::{dot_product, Vector};
use crate::neural_network::activations::Activation;
use rand::Rng;

/// Trait for neural network layers
pub trait Layer {
    /// Forward pass: compute output given input
    fn forward(&mut self, input: &Vector) -> Vector;

    /// Backward pass: compute gradients and return gradient w.r.t. input
    fn backward(&mut self, grad_output: &Vector, learning_rate: f64) -> Vector;

    /// Get the output size of this layer
    fn output_size(&self) -> usize;
}

/// Dense (Fully Connected) layer
///
/// Performs the operation: output = weights * input + bias
pub struct Dense {
    weights: Vec<Vector>, // Each row is weights for one output neuron
    biases: Vector,
    input_size: usize,
    output_size: usize,
    // Cached values for backpropagation
    last_input: Vector,
}

impl Dense {
    /// Creates a new Dense layer with random initialization
    ///
    /// # Arguments
    ///
    /// * `input_size` - Number of input features
    /// * `output_size` - Number of output neurons
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_maths::neural_network::layer::Dense;
    ///
    /// let layer = Dense::new(10, 5); // 10 inputs, 5 outputs
    /// ```
    pub fn new(input_size: usize, output_size: usize) -> Self {
        let mut rng = rand::thread_rng();

        // Xavier/Glorot initialization: weights ~ U(-sqrt(6/(in+out)), sqrt(6/(in+out)))
        let limit = (6.0 / (input_size + output_size) as f64).sqrt();

        let weights: Vec<Vector> = (0..output_size)
            .map(|_| {
                (0..input_size)
                    .map(|_| rng.gen_range(-limit..limit))
                    .collect()
            })
            .collect();

        let biases = vec![0.0; output_size];

        Dense {
            weights,
            biases,
            input_size,
            output_size,
            last_input: vec![],
        }
    }

    /// Creates a Dense layer with specific weights and biases (useful for testing)
    pub fn with_weights(weights: Vec<Vector>, biases: Vector) -> Self {
        let output_size = weights.len();
        let input_size = if output_size > 0 { weights[0].len() } else { 0 };

        Dense {
            weights,
            biases,
            input_size,
            output_size,
            last_input: vec![],
        }
    }
}

impl Layer for Dense {
    fn forward(&mut self, input: &Vector) -> Vector {
        assert_eq!(
            input.len(),
            self.input_size,
            "Input size mismatch: expected {}, got {}",
            self.input_size,
            input.len()
        );

        // Cache input for backward pass
        self.last_input = input.clone();

        // Compute output = weights * input + bias
        self.weights
            .iter()
            .zip(&self.biases)
            .map(|(w, &b)| dot_product(w, input) + b)
            .collect()
    }

    fn backward(&mut self, grad_output: &Vector, learning_rate: f64) -> Vector {
        assert_eq!(grad_output.len(), self.output_size);

        // Compute gradient w.r.t. input
        let mut grad_input = vec![0.0; self.input_size];
        for (w, &grad_out) in self.weights.iter().zip(grad_output) {
            for (i, &w_i) in w.iter().enumerate() {
                grad_input[i] += w_i * grad_out;
            }
        }

        // Update weights and biases
        for (i, grad_out) in grad_output.iter().enumerate() {
            // Update weights: w -= learning_rate * grad_out * input
            for j in 0..self.input_size {
                self.weights[i][j] -= learning_rate * grad_out * self.last_input[j];
            }
            // Update bias: b -= learning_rate * grad_out
            self.biases[i] -= learning_rate * grad_out;
        }

        grad_input
    }

    fn output_size(&self) -> usize {
        self.output_size
    }
}

/// Activation layer - applies an activation function element-wise
pub struct ActivationLayer<A: Activation> {
    activation: A,
    size: usize,
    // Cached values for backpropagation
    last_input: Vector,
}

impl<A: Activation> ActivationLayer<A> {
    /// Creates a new Activation layer
    ///
    /// # Arguments
    ///
    /// * `activation` - The activation function to use
    /// * `size` - The size of the input/output
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_maths::neural_network::layer::ActivationLayer;
    /// use rusty_maths::neural_network::activations::ReLU;
    ///
    /// let layer = ActivationLayer::new(ReLU, 10);
    /// ```
    pub fn new(activation: A, size: usize) -> Self {
        ActivationLayer {
            activation,
            size,
            last_input: vec![],
        }
    }
}

impl<A: Activation> Layer for ActivationLayer<A> {
    fn forward(&mut self, input: &Vector) -> Vector {
        assert_eq!(input.len(), self.size);

        // Cache input for backward pass
        self.last_input = input.clone();

        // Apply activation function
        self.activation.activate_vector(input)
    }

    fn backward(&mut self, grad_output: &Vector, _learning_rate: f64) -> Vector {
        assert_eq!(grad_output.len(), self.size);

        // Gradient = grad_output * activation'(input)
        let derivatives = self.activation.derivative_vector(&self.last_input);

        grad_output
            .iter()
            .zip(derivatives)
            .map(|(&g, d)| g * d)
            .collect()
    }

    fn output_size(&self) -> usize {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neural_network::activations::ReLU;

    #[test]
    fn dense_forward_test() {
        let weights = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let biases = vec![0.5, 1.0];
        let mut layer = Dense::with_weights(weights, biases);

        let input = vec![1.0, 2.0];
        let output = layer.forward(&input);

        // First neuron: 1*1 + 2*2 + 0.5 = 5.5
        // Second neuron: 3*1 + 4*2 + 1.0 = 12.0
        assert_eq!(output, vec![5.5, 12.0]);
    }

    #[test]
    fn activation_layer_test() {
        let mut layer = ActivationLayer::new(ReLU, 3);
        let input = vec![-1.0, 0.0, 2.0];
        let output = layer.forward(&input);

        assert_eq!(output, vec![0.0, 0.0, 2.0]);
    }

    #[test]
    fn dense_backward_test() {
        let weights = vec![vec![1.0, 2.0]];
        let biases = vec![0.0];
        let mut layer = Dense::with_weights(weights, biases);

        let input = vec![1.0, 1.0];
        layer.forward(&input);

        let grad_output = vec![1.0];
        let grad_input = layer.backward(&grad_output, 0.1);

        // Gradient w.r.t. input should be the weights
        assert_eq!(grad_input, vec![1.0, 2.0]);
    }
}

use crate::linear_algebra::Vector;
use crate::neural_network::layer::Layer;

/// A sequential neural network
pub struct Network {
    layers: Vec<Box<dyn Layer>>,
}

impl Network {
    /// Creates a new empty network
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_maths::neural_network::network::Network;
    ///
    /// let network = Network::new();
    /// ```
    pub fn new() -> Self {
        Network { layers: vec![] }
    }

    /// Adds a layer to the network
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_maths::neural_network::network::Network;
    /// use rusty_maths::neural_network::layer::{Dense, ActivationLayer};
    /// use rusty_maths::neural_network::activations::ReLU;
    ///
    /// let mut network = Network::new();
    /// network.add(Box::new(Dense::new(10, 5)));
    /// network.add(Box::new(ActivationLayer::new(ReLU, 5)));
    /// ```
    pub fn add(&mut self, layer: Box<dyn Layer>) {
        self.layers.push(layer);
    }

    /// Performs a forward pass through the network
    ///
    /// # Arguments
    ///
    /// * `input` - The input vector
    ///
    /// # Returns
    ///
    /// The output vector after passing through all layers
    pub fn forward(&mut self, input: &Vector) -> Vector {
        let mut output = input.clone();
        for layer in &mut self.layers {
            output = layer.forward(&output);
        }
        output
    }

    /// Performs a backward pass and updates weights
    ///
    /// # Arguments
    ///
    /// * `grad_output` - The gradient of the loss w.r.t. the output
    /// * `learning_rate` - The learning rate for weight updates
    fn backward(&mut self, grad_output: &Vector, learning_rate: f64) {
        let mut grad = grad_output.clone();
        for layer in self.layers.iter_mut().rev() {
            grad = layer.backward(&grad, learning_rate);
        }
    }

    /// Trains the network on a single example
    ///
    /// # Arguments
    ///
    /// * `input` - The input vector
    /// * `target` - The target output vector
    /// * `learning_rate` - The learning rate
    ///
    /// # Returns
    ///
    /// The mean squared error for this example
    pub fn train_step(&mut self, input: &Vector, target: &Vector, learning_rate: f64) -> f64 {
        // Forward pass
        let output = self.forward(input);

        // Compute loss (MSE) and gradient
        let mut loss = 0.0;
        let mut grad_output = vec![0.0; output.len()];

        for (i, (&y_pred, &y_true)) in output.iter().zip(target).enumerate() {
            let error = y_pred - y_true;
            loss += error * error;
            grad_output[i] = 2.0 * error; // Gradient of MSE
        }

        loss /= output.len() as f64;

        // Backward pass
        self.backward(&grad_output, learning_rate);

        loss
    }

    /// Trains the network on a dataset for multiple epochs
    ///
    /// # Arguments
    ///
    /// * `inputs` - Vector of input vectors
    /// * `targets` - Vector of target output vectors
    /// * `learning_rate` - The learning rate
    /// * `epochs` - Number of training epochs
    ///
    /// # Returns
    ///
    /// Vector of average losses per epoch
    ///
    /// # Examples
    ///
    /// ```
    /// use rusty_maths::neural_network::network::Network;
    /// use rusty_maths::neural_network::layer::{Dense, ActivationLayer};
    /// use rusty_maths::neural_network::activations::{ReLU, Linear};
    ///
    /// let mut network = Network::new();
    /// network.add(Box::new(Dense::new(2, 4)));
    /// network.add(Box::new(ActivationLayer::new(ReLU, 4)));
    /// network.add(Box::new(Dense::new(4, 1)));
    /// network.add(Box::new(ActivationLayer::new(Linear, 1)));
    ///
    /// let inputs = vec![vec![0.0, 0.0], vec![0.0, 1.0]];
    /// let targets = vec![vec![0.0], vec![1.0]];
    ///
    /// let losses = network.train(&inputs, &targets, 0.1, 10);
    /// ```
    pub fn train(
        &mut self,
        inputs: &[Vector],
        targets: &[Vector],
        learning_rate: f64,
        epochs: usize,
    ) -> Vec<f64> {
        assert_eq!(inputs.len(), targets.len());

        let mut epoch_losses = vec![];

        for _epoch in 0..epochs {
            let mut total_loss = 0.0;

            for (input, target) in inputs.iter().zip(targets) {
                let loss = self.train_step(input, target, learning_rate);
                total_loss += loss;
            }

            let avg_loss = total_loss / inputs.len() as f64;
            epoch_losses.push(avg_loss);
        }

        epoch_losses
    }

    /// Makes a prediction on a single input
    ///
    /// # Arguments
    ///
    /// * `input` - The input vector
    ///
    /// # Returns
    ///
    /// The network's prediction
    pub fn predict(&mut self, input: &Vector) -> Vector {
        self.forward(input)
    }

    /// Gets the number of layers in the network
    pub fn num_layers(&self) -> usize {
        self.layers.len()
    }
}

impl Default for Network {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::neural_network::activations::{Linear, ReLU, Tanh};
    use crate::neural_network::layer::{ActivationLayer, Dense};

    #[test]
    fn network_creation_test() {
        let mut network = Network::new();
        assert_eq!(network.num_layers(), 0);

        network.add(Box::new(Dense::new(2, 3)));
        network.add(Box::new(ActivationLayer::new(ReLU, 3)));

        assert_eq!(network.num_layers(), 2);
    }

    #[test]
    fn network_forward_test() {
        let mut network = Network::new();
        network.add(Box::new(Dense::new(2, 3)));
        network.add(Box::new(ActivationLayer::new(ReLU, 3)));

        let input = vec![1.0, 2.0];
        let output = network.forward(&input);

        assert_eq!(output.len(), 3);
    }

    #[test]
    fn network_training_test() {
        // Simple test: learn to output the sum of two inputs
        let mut network = Network::new();
        network.add(Box::new(Dense::new(2, 4)));
        network.add(Box::new(ActivationLayer::new(ReLU, 4)));
        network.add(Box::new(Dense::new(4, 1)));
        network.add(Box::new(ActivationLayer::new(Linear, 1)));

        let inputs = vec![
            vec![0.0, 0.0],
            vec![0.0, 1.0],
            vec![1.0, 0.0],
            vec![1.0, 1.0],
        ];
        let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![2.0]];

        let losses = network.train(&inputs, &targets, 0.01, 100);

        // Loss should decrease
        assert!(losses[losses.len() - 1] < losses[0]);
    }

    /// XOR problem test - demonstrates the network can learn non-linearly separable functions
    ///
    /// This test is ignored by default because it takes time to train.
    /// Run with: `cargo test -- --ignored` or `cargo test xor_learning_test -- --ignored`
    #[test]
    #[ignore]
    fn xor_learning_test() {
        use crate::neural_network::activations::Sigmoid;

        // XOR is the classic non-linearly separable problem
        // It requires a hidden layer to solve
        let mut network = Network::new();
        network.add(Box::new(Dense::new(2, 4))); // Input: 2, Hidden: 4
        network.add(Box::new(ActivationLayer::new(ReLU, 4)));
        network.add(Box::new(Dense::new(4, 1))); // Output: 1
        network.add(Box::new(ActivationLayer::new(Sigmoid, 1)));

        // XOR truth table
        let inputs = vec![
            vec![0.0, 0.0],
            vec![0.0, 1.0],
            vec![1.0, 0.0],
            vec![1.0, 1.0],
        ];
        let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![0.0]];

        println!("\nTraining network on XOR problem...");
        let losses = network.train(&inputs, &targets, 0.5, 5000);

        // Print training progress
        println!("Initial loss: {:.6}", losses[0]);
        println!("Final loss:   {:.6}", losses[losses.len() - 1]);

        // Verify the loss decreased significantly
        assert!(
            losses[losses.len() - 1] < 0.1,
            "Network should learn XOR with final loss < 0.1, got {}",
            losses[losses.len() - 1]
        );

        println!("\nTesting predictions:");

        // Test each input and verify it learned XOR
        let test_cases = [
            (vec![0.0, 0.0], 0.0, "0 XOR 0 = 0"),
            (vec![0.0, 1.0], 1.0, "0 XOR 1 = 1"),
            (vec![1.0, 0.0], 1.0, "1 XOR 0 = 1"),
            (vec![1.0, 1.0], 0.0, "1 XOR 1 = 0"),
        ];

        for (input, expected, description) in &test_cases {
            let prediction = network.predict(input);
            let output = prediction[0];

            println!(
                "  {} -> {:.4} (expected: {:.1})",
                description, output, expected
            );

            // Check that the output is close to the expected value
            // Using 0.2 threshold to account for sigmoid activation
            if *expected == 0.0 {
                assert!(
                    output < 0.2,
                    "Expected ~0.0, got {} for input {:?}",
                    output,
                    input
                );
            } else {
                assert!(
                    output > 0.8,
                    "Expected ~1.0, got {} for input {:?}",
                    output,
                    input
                );
            }
        }

        println!("\n✓ Network successfully learned XOR function!");
    }

    /// AND gate test - demonstrates learning a linearly separable function
    ///
    /// This test is ignored by default because it takes time to train.
    /// Run with: `cargo test and_gate_test -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn and_gate_test() {
        use crate::neural_network::activations::Sigmoid;

        // AND gate is linearly separable - should learn quickly
        let mut network = Network::new();
        network.add(Box::new(Dense::new(2, 1))); // No hidden layer needed
        network.add(Box::new(ActivationLayer::new(Sigmoid, 1)));

        // AND truth table
        let inputs = vec![
            vec![0.0, 0.0],
            vec![0.0, 1.0],
            vec![1.0, 0.0],
            vec![1.0, 1.0],
        ];
        let targets = vec![vec![0.0], vec![0.0], vec![0.0], vec![1.0]];

        println!("\nTraining network on AND gate...");
        let losses = network.train(&inputs, &targets, 0.5, 1000);

        println!("Initial loss: {:.6}", losses[0]);
        println!("Final loss:   {:.6}", losses[losses.len() - 1]);

        assert!(
            losses[losses.len() - 1] < 0.05,
            "Network should learn AND with final loss < 0.05, got {}",
            losses[losses.len() - 1]
        );

        println!("\nTesting predictions:");
        let test_cases = [
            (vec![0.0, 0.0], 0.0, "0 AND 0 = 0"),
            (vec![0.0, 1.0], 0.0, "0 AND 1 = 0"),
            (vec![1.0, 0.0], 0.0, "1 AND 0 = 0"),
            (vec![1.0, 1.0], 1.0, "1 AND 1 = 1"),
        ];

        for (input, expected, description) in &test_cases {
            let prediction = network.predict(input);
            let output = prediction[0];
            println!(
                "  {} -> {:.4} (expected: {:.1})",
                description, output, expected
            );

            if *expected == 0.0 {
                assert!(output < 0.2, "Expected ~0.0, got {}", output);
            } else {
                assert!(output > 0.8, "Expected ~1.0, got {}", output);
            }
        }

        println!("\n✓ Network successfully learned AND gate!");
    }

    /// OR gate test - another linearly separable function
    ///
    /// This test is ignored by default because it takes time to train.
    /// Run with: `cargo test or_gate_test -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn or_gate_test() {
        use crate::neural_network::activations::Sigmoid;

        let mut network = Network::new();
        network.add(Box::new(Dense::new(2, 1)));
        network.add(Box::new(ActivationLayer::new(Sigmoid, 1)));

        // OR truth table
        let inputs = vec![
            vec![0.0, 0.0],
            vec![0.0, 1.0],
            vec![1.0, 0.0],
            vec![1.0, 1.0],
        ];
        let targets = vec![vec![0.0], vec![1.0], vec![1.0], vec![1.0]];

        println!("\nTraining network on OR gate...");
        let losses = network.train(&inputs, &targets, 0.5, 1000);

        println!("Initial loss: {:.6}", losses[0]);
        println!("Final loss:   {:.6}", losses[losses.len() - 1]);

        assert!(losses[losses.len() - 1] < 0.05);

        println!("\nTesting predictions:");
        let test_cases = [
            (vec![0.0, 0.0], 0.0, "0 OR 0 = 0"),
            (vec![0.0, 1.0], 1.0, "0 OR 1 = 1"),
            (vec![1.0, 0.0], 1.0, "1 OR 0 = 1"),
            (vec![1.0, 1.0], 1.0, "1 OR 1 = 1"),
        ];

        for (input, expected, description) in &test_cases {
            let prediction = network.predict(input);
            let output = prediction[0];
            println!(
                "  {} -> {:.4} (expected: {:.1})",
                description, output, expected
            );

            if *expected == 0.0 {
                assert!(output < 0.2);
            } else {
                assert!(output > 0.8);
            }
        }

        println!("\n✓ Network successfully learned OR gate!");
    }

    /// Linear regression test - learn y = 2x + 1
    ///
    /// This test is ignored by default because it takes time to train.
    /// Run with: `cargo test linear_regression_test -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn linear_regression_test() {
        let mut network = Network::new();
        network.add(Box::new(Dense::new(1, 1)));
        network.add(Box::new(ActivationLayer::new(Linear, 1)));

        // Generate training data for y = 2x + 1
        let inputs: Vec<Vector> = (0..20).map(|i| vec![i as f64 * 0.1]).collect();
        let targets: Vec<Vector> = inputs.iter().map(|x| vec![2.0 * x[0] + 1.0]).collect();

        println!("\nTraining network to learn y = 2x + 1...");
        let losses = network.train(&inputs, &targets, 0.01, 1000);

        println!("Initial loss: {:.6}", losses[0]);
        println!("Final loss:   {:.6}", losses[losses.len() - 1]);

        assert!(losses[losses.len() - 1] < 0.01);

        println!("\nTesting predictions:");
        let test_inputs = vec![0.0, 0.5, 1.0, 1.5, 2.0];

        for &x in &test_inputs {
            let prediction = network.predict(&vec![x]);
            let expected = 2.0 * x + 1.0;
            let output = prediction[0];

            println!("  x={:.1} -> {:.4} (expected: {:.4})", x, output, expected);
            assert!((output - expected).abs() < 0.1);
        }

        println!("\n✓ Network successfully learned linear function!");
    }

    /// Sine approximation test - learn to approximate sin(x)
    ///
    /// This test is ignored by default because it takes time to train.
    /// Run with: `cargo test sine_approximation_test -- --ignored --nocapture`
    ///
    /// Uses Tanh activations which are well-suited for smooth periodic functions
    /// with range [-1, 1]. Architecture: 1 -> 32 -> 32 -> 32 -> 1
    #[test]
    #[ignore]
    fn sine_approximation_test() {
        use std::f64::consts::PI;

        let mut network = Network::new();
        network.add(Box::new(Dense::new(1, 32)));
        network.add(Box::new(ActivationLayer::new(Tanh, 32)));
        network.add(Box::new(Dense::new(32, 32)));
        network.add(Box::new(ActivationLayer::new(Tanh, 32)));
        network.add(Box::new(Dense::new(32, 32)));
        network.add(Box::new(ActivationLayer::new(Tanh, 32)));
        network.add(Box::new(Dense::new(32, 1)));
        network.add(Box::new(ActivationLayer::new(Tanh, 1)));

        // Generate training data for sin(x) in range [0, 2π]
        // More training points for better approximation
        let mut inputs = vec![];
        let mut targets = vec![];
        for i in 0..200 {
            let x = (i as f64 / 200.0) * 2.0 * PI;
            inputs.push(vec![x]);
            targets.push(vec![x.sin()]);
        }

        println!("\nTraining network to approximate sin(x)...");
        println!("Architecture: 1 -> 32(Tanh) -> 32(Tanh) -> 32(Tanh) -> 1(Tanh)");
        println!("Training points: {}", inputs.len());

        let losses = network.train(&inputs, &targets, 0.005, 10000);

        println!("Initial loss: {:.6}", losses[0]);
        println!("Final loss:   {:.6}", losses[losses.len() - 1]);

        assert!(
            losses[losses.len() - 1] < 0.01,
            "Final loss too high: {}",
            losses[losses.len() - 1]
        );

        println!("\nTesting predictions:");
        let test_points = vec![
            0.0,
            PI / 6.0,
            PI / 4.0,
            PI / 3.0,
            PI / 2.0,
            2.0 * PI / 3.0,
            PI,
            4.0 * PI / 3.0,
            3.0 * PI / 2.0,
            2.0 * PI,
        ];

        for &x in &test_points {
            let prediction = network.predict(&vec![x]);
            let expected = x.sin();
            let output = prediction[0];

            println!(
                "  sin({:.4}) -> {:.4} (expected: {:.4}, error: {:.4})",
                x,
                output,
                expected,
                (output - expected).abs()
            );
            assert!(
                (output - expected).abs() < 0.05,
                "Prediction too far from expected at x={}: {} vs {} (error: {})",
                x,
                output,
                expected,
                (output - expected).abs()
            );
        }

        println!("\n✓ Network successfully approximated sine function with high precision!");
    }

    /// Circle classification test - classify points inside/outside a circle
    ///
    /// This test is ignored by default because it takes time to train.
    /// Run with: `cargo test circle_classification_test -- --ignored --nocapture`
    #[test]
    #[ignore]
    fn circle_classification_test() {
        use crate::neural_network::activations::Sigmoid;

        let mut network = Network::new();
        network.add(Box::new(Dense::new(2, 8)));
        network.add(Box::new(ActivationLayer::new(ReLU, 8)));
        network.add(Box::new(Dense::new(8, 1)));
        network.add(Box::new(ActivationLayer::new(Sigmoid, 1)));

        // Generate training data: points inside unit circle = 1, outside = 0
        let mut inputs = vec![];
        let mut targets = vec![];

        for i in 0..100 {
            let x = (i as f64 / 25.0) - 2.0; // Range: -2 to 2
            for j in 0..100 {
                let y = (j as f64 / 25.0) - 2.0;
                let distance_squared = x * x + y * y;
                let inside = if distance_squared <= 1.0 { 1.0 } else { 0.0 };

                inputs.push(vec![x, y]);
                targets.push(vec![inside]);
            }
        }

        println!("\nTraining network to classify points inside/outside unit circle...");
        println!("Training on {} points", inputs.len());
        let losses = network.train(&inputs, &targets, 0.1, 500);

        println!("Initial loss: {:.6}", losses[0]);
        println!("Final loss:   {:.6}", losses[losses.len() - 1]);

        assert!(losses[losses.len() - 1] < 0.1);

        println!("\nTesting predictions:");
        let test_cases = [
            (vec![0.0, 0.0], 1.0, "center (0, 0)"),
            (vec![0.5, 0.5], 1.0, "inside (0.5, 0.5)"),
            (vec![0.7, 0.7], 0.0, "outside (0.7, 0.7)"),
            (vec![2.0, 0.0], 0.0, "far outside (2, 0)"),
            (vec![0.0, 0.9], 1.0, "edge inside (0, 0.9)"),
        ];

        for (input, expected, description) in &test_cases {
            let prediction = network.predict(input);
            let output = prediction[0];
            println!(
                "  {} -> {:.4} (expected: {:.1})",
                description, output, expected
            );

            if *expected == 0.0 {
                assert!(output < 0.3, "Expected outside, got {}", output);
            } else {
                assert!(output > 0.7, "Expected inside, got {}", output);
            }
        }

        println!("\n✓ Network successfully learned circle classification!");
    }
}

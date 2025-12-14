//! Neural Network module
//!
//! A simple but functional neural network implementation with support for:
//! - Dense (Fully Connected) layers
//! - Multiple activation functions (ReLU, Sigmoid, Tanh, Linear)
//! - Forward and backward propagation
//! - Training with gradient descent
//!
//! # Examples
//!
//! ```
//! use rusty_maths::neural_network::network::Network;
//! use rusty_maths::neural_network::layer::{Dense, ActivationLayer};
//! use rusty_maths::neural_network::activations::{ReLU, Sigmoid};
//!
//! // Create a network for binary classification
//! let mut network = Network::new();
//! network.add(Box::new(Dense::new(2, 4)));     // Input layer: 2 -> 4
//! network.add(Box::new(ActivationLayer::new(ReLU, 4)));
//! network.add(Box::new(Dense::new(4, 1)));     // Output layer: 4 -> 1
//! network.add(Box::new(ActivationLayer::new(Sigmoid, 1)));
//!
//! // Training data (XOR problem)
//! let inputs = vec![
//!     vec![0.0, 0.0],
//!     vec![0.0, 1.0],
//!     vec![1.0, 0.0],
//!     vec![1.0, 1.0],
//! ];
//! let targets = vec![
//!     vec![0.0],
//!     vec![1.0],
//!     vec![1.0],
//!     vec![0.0],
//! ];
//!
//! // Train the network
//! let losses = network.train(&inputs, &targets, 0.1, 1000);
//!
//! // Make predictions
//! let prediction = network.predict(&vec![0.0, 1.0]);
//! ```

pub mod activations;
pub mod layer;
pub mod network;

pub use activations::{Activation, Linear, ReLU, Sigmoid, Tanh};
pub use layer::{ActivationLayer, Dense, Layer};
pub use network::Network;

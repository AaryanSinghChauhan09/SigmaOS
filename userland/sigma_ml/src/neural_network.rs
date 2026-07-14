use crate::tensor::Tensor;

#[derive(Debug, Clone)]
pub struct Layer {
    pub input_dim: usize,
    pub output_dim: usize,
}

pub struct NeuralNetwork {
    pub layers: Vec<Layer>,
}

impl Default for NeuralNetwork {
    fn default() -> Self {
        Self::new()
    }
}

impl NeuralNetwork {
    pub fn new() -> Self {
        Self { layers: Vec::new() }
    }

    pub fn add_layer(&mut self, input_dim: usize, output_dim: usize) {
        self.layers.push(Layer { input_dim, output_dim });
    }

    pub fn forward(&self, input: &Tensor) -> Result<Tensor, String> {
        if self.layers.is_empty() {
            return Ok(input.clone());
        }
        // Simplified layer transformation logic: multiply first layer dims
        let first_layer = &self.layers[0];
        if input.shape.is_empty() || input.shape[0] != first_layer.input_dim {
            return Err("Input dimension mismatch".to_string());
        }

        let output_data = vec![1.0; first_layer.output_dim];
        Ok(Tensor::new(vec![first_layer.output_dim], output_data))
    }
}

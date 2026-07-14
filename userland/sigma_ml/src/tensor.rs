#[derive(Debug, Clone)]
pub struct Tensor {
    pub shape: Vec<usize>,
    pub data: Vec<f32>,
}

impl Tensor {
    pub fn new(shape: Vec<usize>, data: Vec<f32>) -> Self {
        Self { shape, data }
    }

    pub fn elementwise_add(&self, other: &Tensor) -> Result<Tensor, String> {
        if self.shape != other.shape {
            return Err("Shape mismatch".to_string());
        }
        let added_data = self.data.iter().zip(other.data.iter()).map(|(a, b)| a + b).collect();
        Ok(Tensor {
            shape: self.shape.clone(),
            data: added_data,
        })
    }
}

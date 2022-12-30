#[derive(Debug)]
pub struct Matrix {
    data: Vec<f32>,
    n: usize,
}

impl Matrix {
    pub fn new(n: usize) -> Self {
        Self {
            data: vec![0.0f32; n * n],
            n,
        }
    }

    pub fn fill(&mut self, v: f32) {
        self.data.fill(v);
    }

    pub fn mul(&self, other: &Self) -> Self {
        let mut m = Matrix::new(self.n);

        for r in 0..self.n {
            let offset = self.n * r;

            for c in 0..self.n {
                let mut result = 0.0f32;

                for s in 0..self.n {
                    result += self.data[offset + s] * other.data[c + s * self.n];
                }

                m.data[offset + c] = result;
            }
        }

        m
    }
}

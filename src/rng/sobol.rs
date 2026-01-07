#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SobolError;

#[derive(Debug, Clone)]
pub struct Sobol {
    dim: usize,
    index: u64,
    x: Vec<u32>,
    directions: Vec<[u32; 32]>,
}

impl Sobol {
    pub fn new(dim: usize) -> Result<Self, SobolError> {
        if dim == 0 {
            return Err(SobolError);
        }
        if dim == 1 {
            return Self::with_directions(default_directions_dim1());
        }
        Err(SobolError)
    }

    pub fn with_directions(directions: Vec<[u32; 32]>) -> Result<Self, SobolError> {
        if directions.is_empty() {
            return Err(SobolError);
        }
        let dim = directions.len();
        let x = vec![0u32; dim];
        Ok(Self {
            dim,
            index: 0,
            x,
            directions,
        })
    }

    #[inline]
    pub fn dimension(&self) -> usize {
        self.dim
    }

    #[inline]
    pub fn index(&self) -> u64 {
        self.index
    }

    pub fn reset(&mut self) {
        self.index = 0;
        for v in &mut self.x {
            *v = 0;
        }
    }

    pub fn next_point(&mut self, out: &mut [f64]) -> Result<(), SobolError> {
        if out.len() != self.dim {
            return Err(SobolError);
        }
        if self.index >= (1u64 << 32) {
            return Err(SobolError);
        }
        for (dst, &val) in out.iter_mut().zip(self.x.iter()) {
            *dst = u32_to_unit_f64(val);
        }
        let c = (self.index.wrapping_add(1)).trailing_zeros() as usize;
        for i in 0..self.dim {
            self.x[i] ^= self.directions[i][c];
        }
        self.index = self.index.wrapping_add(1);
        Ok(())
    }

    pub fn next_vec(&mut self) -> Result<Vec<f64>, SobolError> {
        let mut out = vec![0.0f64; self.dim];
        self.next_point(&mut out)?;
        Ok(out)
    }

    pub fn seek(&mut self, index: u64) -> Result<(), SobolError> {
        if index >= (1u64 << 32) {
            return Err(SobolError);
        }
        self.index = index;
        let gray = index ^ (index >> 1);
        for dim in 0..self.dim {
            let mut acc = 0u32;
            let mut g = gray;
            let mut bit = 0usize;
            while g != 0 {
                if g & 1 == 1 {
                    acc ^= self.directions[dim][bit];
                }
                g >>= 1;
                bit += 1;
            }
            self.x[dim] = acc;
        }
        Ok(())
    }

    pub fn advance(&mut self, delta: u64) -> Result<(), SobolError> {
        let target = self.index.wrapping_add(delta);
        self.seek(target)
    }
}

#[inline]
fn u32_to_unit_f64(value: u32) -> f64 {
    (value as f64) / (u32::MAX as f64 + 1.0)
}

fn default_directions_dim1() -> Vec<[u32; 32]> {
    let mut v = [0u32; 32];
    for (i, value) in v.iter_mut().enumerate() {
        *value = 1u32 << (31 - i);
    }
    vec![v]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sobol_dim1_sequence() {
        let mut sobol = Sobol::new(1).unwrap();
        let expected = [0.0, 0.5, 0.75, 0.25, 0.375, 0.875];
        for &value in &expected {
            let point = sobol.next_vec().unwrap();
            assert_eq!(point.len(), 1);
            assert_eq!(point[0], value);
        }
    }

    #[test]
    fn sobol_seek_matches_expected_point() {
        let mut sobol = Sobol::new(1).unwrap();
        sobol.seek(4).unwrap();
        let point = sobol.next_vec().unwrap();
        assert_eq!(point[0], 0.375);
    }
}

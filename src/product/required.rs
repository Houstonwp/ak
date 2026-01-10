#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequiredDataLayoutError;

/// Fixed required data dimensions for a product.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequiredDataLayout {
    policy_scalars: usize,
    state_vectors: usize,
}

impl RequiredDataLayout {
    pub fn new(
        policy_scalars: usize,
        state_vectors: usize,
    ) -> Result<Self, RequiredDataLayoutError> {
        if policy_scalars == 0 && state_vectors == 0 {
            return Err(RequiredDataLayoutError);
        }
        Ok(Self {
            policy_scalars,
            state_vectors,
        })
    }

    pub const fn policy_scalars(&self) -> usize {
        self.policy_scalars
    }

    pub const fn state_vectors(&self) -> usize {
        self.state_vectors
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequiredDataBufferError;

/// SoA required data storage: per-policy scalars and per-state vectors.
#[derive(Debug, Clone)]
pub struct RequiredDataBuffer {
    layout: RequiredDataLayout,
    n_states: usize,
    policy_scalars: Vec<f64>,
    state_vectors: Vec<f64>,
}

impl RequiredDataBuffer {
    pub fn new(
        layout: RequiredDataLayout,
        n_states: usize,
    ) -> Result<Self, RequiredDataBufferError> {
        if n_states == 0 {
            return Err(RequiredDataBufferError);
        }
        let scalars = vec![0.0; layout.policy_scalars()];
        let vectors_len = layout
            .state_vectors()
            .checked_mul(n_states)
            .ok_or(RequiredDataBufferError)?;
        let vectors = vec![0.0; vectors_len];
        Ok(Self {
            layout,
            n_states,
            policy_scalars: scalars,
            state_vectors: vectors,
        })
    }

    pub fn from_parts(
        layout: RequiredDataLayout,
        n_states: usize,
        policy_scalars: Vec<f64>,
        state_vectors: Vec<f64>,
    ) -> Result<Self, RequiredDataBufferError> {
        if n_states == 0 {
            return Err(RequiredDataBufferError);
        }
        if policy_scalars.len() != layout.policy_scalars() {
            return Err(RequiredDataBufferError);
        }
        let expected = layout
            .state_vectors()
            .checked_mul(n_states)
            .ok_or(RequiredDataBufferError)?;
        if state_vectors.len() != expected {
            return Err(RequiredDataBufferError);
        }
        Ok(Self {
            layout,
            n_states,
            policy_scalars,
            state_vectors,
        })
    }

    pub const fn layout(&self) -> RequiredDataLayout {
        self.layout
    }

    pub const fn n_states(&self) -> usize {
        self.n_states
    }

    pub fn policy_scalar(&self, index: usize) -> f64 {
        debug_assert!(index < self.policy_scalars.len());
        self.policy_scalars[index]
    }

    pub fn set_policy_scalar(&mut self, index: usize, value: f64) {
        debug_assert!(index < self.policy_scalars.len());
        self.policy_scalars[index] = value;
    }

    pub fn state_vector(&self, field: usize) -> &[f64] {
        let start = self.state_vector_offset(field);
        &self.state_vectors[start..start + self.n_states]
    }

    pub fn state_vector_mut(&mut self, field: usize) -> &mut [f64] {
        let start = self.state_vector_offset(field);
        &mut self.state_vectors[start..start + self.n_states]
    }

    fn state_vector_offset(&self, field: usize) -> usize {
        debug_assert!(field < self.layout.state_vectors());
        field * self.n_states
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_data_buffer_indexes_match_layout() {
        let layout = RequiredDataLayout::new(2, 3).unwrap();
        let mut data = RequiredDataBuffer::new(layout, 4).unwrap();
        data.set_policy_scalar(1, 12.5);
        assert_eq!(data.policy_scalar(1), 12.5);

        data.state_vector_mut(2)[3] = -7.0;
        assert_eq!(data.state_vector(2)[3], -7.0);
        assert_eq!(data.state_vector(0).len(), 4);
    }
}

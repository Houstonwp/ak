use crate::rng::RngCore;

use super::{Amount, ProductState, RequiredDataBuffer, RequiredDataLayout};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProductDefinitionError;

/// Fixed definition of a product's dimensions and required data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProductDefinition {
    pub n_states: usize,
    pub n_kinds: usize,
    pub required_data: RequiredDataLayout,
}

impl ProductDefinition {
    pub fn new(
        n_states: usize,
        n_kinds: usize,
        required_data: RequiredDataLayout,
    ) -> Result<Self, ProductDefinitionError> {
        if n_states == 0 || n_kinds == 0 {
            return Err(ProductDefinitionError);
        }
        Ok(Self {
            n_states,
            n_kinds,
            required_data,
        })
    }
}

/// Product interface describing cashflow kinds, states, and required data.
///
/// Determinism: given identical inputs and an RNG stream in the same state,
/// implementations must produce identical outputs.
pub trait Product {
    /// Returns the fixed definition of the product's dimensions.
    fn definition(&self) -> &ProductDefinition;

    /// Provides the initial state for the projection.
    fn initial_state(&self) -> ProductState;

    /// Generates required data into the provided buffer.
    ///
    /// Determinism: output must be fully determined by inputs and the RNG stream.
    fn generate_required_data(
        &self,
        time_index: usize,
        state: &ProductState,
        rng: &mut dyn RngCore,
        out: &mut RequiredDataBuffer,
    );

    /// Writes cashflows for the current state into the provided buffer.
    ///
    /// The output slice length must match `definition().n_kinds`.
    /// Determinism: output must be fully determined by inputs.
    fn cashflows(
        &self,
        time_index: usize,
        state: &ProductState,
        data: &RequiredDataBuffer,
        out: &mut [Amount],
    );

    /// Computes the next state after applying transitions.
    ///
    /// Determinism: output must be fully determined by inputs and the RNG stream.
    fn next_state(
        &self,
        time_index: usize,
        state: &ProductState,
        data: &RequiredDataBuffer,
        rng: &mut dyn RngCore,
    ) -> ProductState;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::RequiredDataLayoutError;

    #[test]
    fn product_definition_rejects_invalid_shapes() {
        let layout = RequiredDataLayout::new(1, 1).unwrap();
        assert!(ProductDefinition::new(0, 1, layout).is_err());
        assert!(ProductDefinition::new(1, 0, layout).is_err());
    }

    #[test]
    fn product_definition_accepts_valid_shapes() -> Result<(), RequiredDataLayoutError> {
        let layout = RequiredDataLayout::new(1, 2)?;
        let def = ProductDefinition::new(2, 3, layout).unwrap();
        assert_eq!(def.n_states, 2);
        assert_eq!(def.n_kinds, 3);
        assert_eq!(def.required_data, layout);
        Ok(())
    }
}

use crate::product::{CashflowBuffer, Product, ProductDefinition, RequiredDataBuffer};
use crate::{Date, Frequency};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModelConfig {
    pub start: Date,
    pub frequency: Frequency,
    pub steps: usize,
}

/// Model interface to transform products into state-indexed cashflows.
///
/// Determinism: with identical inputs and an RNG stream in the same state,
/// implementations must produce identical outputs in the provided buffers.
pub trait Model {
    fn run(
        &self,
        product: &dyn Product,
        config: &ModelConfig,
        rng: &mut dyn crate::rng::RngCore,
        cashflows: &mut CashflowBuffer,
        data: &mut RequiredDataBuffer,
    ) -> Result<(), ModelError>;
}

pub fn validate_buffers(
    definition: &ProductDefinition,
    steps: usize,
    cashflows: &CashflowBuffer,
    data: &RequiredDataBuffer,
) -> Result<(), ModelError> {
    if cashflows.n_states() != definition.n_states
        || cashflows.n_kinds() != definition.n_kinds
        || cashflows.len_steps() != steps
    {
        return Err(ModelError);
    }
    if data.n_states() != definition.n_states || data.layout() != definition.required_data {
        return Err(ModelError);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::product::{ProductDefinition, RequiredDataLayout};

    #[test]
    fn validate_buffers_accepts_matching_layouts() {
        let layout = RequiredDataLayout::new(1, 2).unwrap();
        let definition = ProductDefinition::new(2, 3, layout).unwrap();
        let data = RequiredDataBuffer::new(layout, 2).unwrap();
        let times = vec![
            Date::new(2024, 1, 1).unwrap(),
            Date::new(2024, 2, 1).unwrap(),
        ];
        let cashflows = CashflowBuffer::new(2, 3, times).unwrap();

        assert!(validate_buffers(&definition, 2, &cashflows, &data).is_ok());
    }

    #[test]
    fn validate_buffers_rejects_mismatched_data() {
        let layout = RequiredDataLayout::new(1, 2).unwrap();
        let definition = ProductDefinition::new(2, 3, layout).unwrap();
        let wrong_layout = RequiredDataLayout::new(2, 2).unwrap();
        let data = RequiredDataBuffer::new(wrong_layout, 2).unwrap();
        let times = vec![Date::new(2024, 1, 1).unwrap()];
        let cashflows = CashflowBuffer::new(2, 3, times).unwrap();

        assert!(validate_buffers(&definition, 1, &cashflows, &data).is_err());
    }
}

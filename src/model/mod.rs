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
///
/// # Examples
///
/// ```rust
/// use ak::model::{validate_buffers, Model, ModelConfig, ModelError};
/// use ak::product::{
///     Amount, CashflowBuffer, Product, ProductDefinition, ProductState, RequiredDataBuffer,
///     RequiredDataLayout,
/// };
/// use ak::rng::RngCore;
/// use ak::{Date, Frequency};
///
/// struct DemoProduct {
///     definition: ProductDefinition,
/// }
///
/// impl Product for DemoProduct {
///     fn definition(&self) -> &ProductDefinition {
///         &self.definition
///     }
///
///     fn initial_state(&self) -> ProductState {
///         ProductState::new(0, 1, Amount::zero())
///     }
///
///     fn generate_required_data(
///         &self,
///         _time_index: usize,
///         _state: &ProductState,
///         _rng: &mut dyn RngCore,
///         out: &mut RequiredDataBuffer,
///     ) {
///         out.set_policy_scalar(0, 1.0);
///     }
///
///     fn cashflows(
///         &self,
///         _time_index: usize,
///         _state: &ProductState,
///         _data: &RequiredDataBuffer,
///         out: &mut [Amount],
///     ) {
///         out[0] = Amount::zero();
///     }
///
///     fn next_state(
///         &self,
///         _time_index: usize,
///         state: &ProductState,
///         _data: &RequiredDataBuffer,
///         _rng: &mut dyn RngCore,
///     ) -> ProductState {
///         *state
///     }
/// }
///
/// struct DemoModel;
///
/// impl Model for DemoModel {
///     fn run(
///         &self,
///         product: &dyn Product,
///         config: &ModelConfig,
///         rng: &mut dyn RngCore,
///         cashflows: &mut CashflowBuffer,
///         data: &mut RequiredDataBuffer,
///     ) -> Result<(), ModelError> {
///         validate_buffers(product.definition(), config.steps, cashflows, data)?;
///         product.generate_required_data(0, &product.initial_state(), rng, data);
///         Ok(())
///     }
/// }
///
/// struct ZeroRng;
///
/// impl RngCore for ZeroRng {
///     fn next_u32(&mut self) -> u32 {
///         0
///     }
/// }
///
/// let layout = RequiredDataLayout::new(1, 0).unwrap();
/// let definition = ProductDefinition::new(1, 1, layout).unwrap();
/// let product = DemoProduct { definition };
/// let config = ModelConfig {
///     start: Date::new(2024, 1, 1).unwrap(),
///     frequency: Frequency::Monthly,
///     steps: 1,
/// };
/// let times = vec![config.start];
/// let mut cashflows = CashflowBuffer::new(1, 1, times).unwrap();
/// let mut data = RequiredDataBuffer::new(layout, 1).unwrap();
/// let mut rng = ZeroRng;
/// DemoModel
///     .run(&product, &config, &mut rng, &mut cashflows, &mut data)
///     .unwrap();
/// ```
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
    use crate::product::{Product, ProductState};
    use crate::product::{ProductDefinition, RequiredDataLayout};
    use crate::rng::RngCore;

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

    struct TestProduct {
        definition: ProductDefinition,
    }

    impl Product for TestProduct {
        fn definition(&self) -> &ProductDefinition {
            &self.definition
        }

        fn initial_state(&self) -> ProductState {
            ProductState::new(0, 1, crate::product::Amount::zero())
        }

        fn generate_required_data(
            &self,
            _time_index: usize,
            _state: &ProductState,
            _rng: &mut dyn RngCore,
            out: &mut RequiredDataBuffer,
        ) {
            out.set_policy_scalar(0, 3.5);
            out.state_vector_mut(0)[0] = 1.25;
        }

        fn cashflows(
            &self,
            _time_index: usize,
            _state: &ProductState,
            _data: &RequiredDataBuffer,
            out: &mut [crate::product::Amount],
        ) {
            out[0] = crate::product::Amount::zero();
        }

        fn next_state(
            &self,
            _time_index: usize,
            state: &ProductState,
            _data: &RequiredDataBuffer,
            _rng: &mut dyn RngCore,
        ) -> ProductState {
            *state
        }
    }

    struct TestModel;

    impl Model for TestModel {
        fn run(
            &self,
            product: &dyn Product,
            config: &ModelConfig,
            rng: &mut dyn RngCore,
            cashflows: &mut CashflowBuffer,
            data: &mut RequiredDataBuffer,
        ) -> Result<(), ModelError> {
            validate_buffers(product.definition(), config.steps, cashflows, data)?;
            product.generate_required_data(0, &product.initial_state(), rng, data);
            Ok(())
        }
    }

    struct ZeroRng;

    impl RngCore for ZeroRng {
        fn next_u32(&mut self) -> u32 {
            0
        }
    }

    #[test]
    fn model_run_rejects_required_data_layout_mismatch() {
        let layout = RequiredDataLayout::new(1, 1).unwrap();
        let definition = ProductDefinition::new(2, 1, layout).unwrap();
        let product = TestProduct { definition };
        let config = ModelConfig {
            start: Date::new(2024, 1, 1).unwrap(),
            frequency: Frequency::Monthly,
            steps: 1,
        };
        let times = vec![config.start];
        let mut cashflows = CashflowBuffer::new(2, 1, times).unwrap();
        let wrong_layout = RequiredDataLayout::new(2, 1).unwrap();
        let mut data = RequiredDataBuffer::new(wrong_layout, 2).unwrap();
        let mut rng = ZeroRng;

        assert!(
            TestModel
                .run(&product, &config, &mut rng, &mut cashflows, &mut data)
                .is_err()
        );
    }

    #[test]
    fn model_run_populates_required_data_from_product() {
        let layout = RequiredDataLayout::new(1, 1).unwrap();
        let definition = ProductDefinition::new(1, 1, layout).unwrap();
        let product = TestProduct { definition };
        let config = ModelConfig {
            start: Date::new(2024, 1, 1).unwrap(),
            frequency: Frequency::Monthly,
            steps: 1,
        };
        let times = vec![config.start];
        let mut cashflows = CashflowBuffer::new(1, 1, times).unwrap();
        let mut data = RequiredDataBuffer::new(layout, 1).unwrap();
        let mut rng = ZeroRng;

        TestModel
            .run(&product, &config, &mut rng, &mut cashflows, &mut data)
            .unwrap();

        assert_eq!(data.policy_scalar(0), 3.5);
        assert_eq!(data.state_vector(0)[0], 1.25);
    }
}

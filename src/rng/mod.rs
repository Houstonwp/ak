pub mod mgk32a;
pub mod sobol;

/// Deterministic jump-ahead for reproducible streams.
pub trait JumpAhead {
    type Error;

    fn advance(&mut self, delta: u128) -> Result<(), Self::Error>;
}

/// Block splitting via jump-ahead for parallel stream construction.
pub trait BlockSplit: Sized {
    type Seed;
    type Error;

    fn for_stream(seed: Self::Seed, stream: u128, stride: u128) -> Result<Self, Self::Error>;
}

/// Core 32-bit RNG interface.
///
/// Determinism: given the same internal state, implementations must return the
/// same sequence of values.
pub trait RngCore {
    fn next_u32(&mut self) -> u32;

    #[inline]
    fn next_u64(&mut self) -> u64 {
        let hi = self.next_u32() as u64;
        let lo = self.next_u32() as u64;
        (hi << 32) | lo
    }

    #[inline]
    fn fill_bytes(&mut self, out: &mut [u8]) {
        let mut i = 0;
        while i + 8 <= out.len() {
            let v = self.next_u64().to_le_bytes();
            out[i..i + 8].copy_from_slice(&v);
            i += 8;
        }
        if i < out.len() {
            let v = self.next_u64().to_le_bytes();
            let remaining = out.len() - i;
            out[i..].copy_from_slice(&v[..remaining]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RngCore;

    struct CounterRng {
        next: u32,
    }

    impl CounterRng {
        fn new(start: u32) -> Self {
            Self { next: start }
        }
    }

    impl RngCore for CounterRng {
        fn next_u32(&mut self) -> u32 {
            let value = self.next;
            self.next = self.next.wrapping_add(1);
            value
        }
    }

    #[test]
    fn next_u64_combines_two_u32s() {
        let mut rng = CounterRng::new(1);
        let value = rng.next_u64();
        let expected = ((1u64) << 32) | 2u64;
        assert_eq!(value, expected);
    }

    #[test]
    fn fill_bytes_handles_short_and_partial_buffers() {
        let mut rng = CounterRng::new(0);
        let mut short = [0u8; 3];
        rng.fill_bytes(&mut short);
        let expected_short = 1u64.to_le_bytes();
        assert_eq!(&short, &expected_short[..3]);

        let mut buffer = [0u8; 9];
        rng.fill_bytes(&mut buffer);
        let first_value = (2u64 << 32) | 3u64;
        let first = first_value.to_le_bytes();
        let second_value = (4u64 << 32) | 5u64;
        let second = second_value.to_le_bytes();
        let mut expected = [0u8; 9];
        expected[..8].copy_from_slice(&first);
        expected[8] = second[0];
        assert_eq!(buffer, expected);
    }
}

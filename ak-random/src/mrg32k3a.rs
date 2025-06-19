use rand_core::{
    SeedableRng,
    block::{BlockRng, BlockRngCore},
};

pub const N: usize = 16;

pub struct Mrg32k3aCore {
    s10: u64,
    s11: u64,
    s12: u64,
    s20: u64,
    s21: u64,
    s22: u64,
}

impl Mrg32k3aCore {
    pub const M1: u64 = 4_294_967_087;
    pub const M2: u64 = 4_294_967_283;
    pub const A12: u64 = 1_403_580;
    pub const A13: u64 = 810_728;
    pub const A21: u64 = 527_612;
    pub const A23: u64 = 1_370_589;
    pub const CORR1: u64 = Self::M1 * Self::A13;
    pub const CORR2: u64 = Self::M2 * Self::A23;
    pub const NORM: f64 = f64::from_bits(0x3DF0_0000_0D00_000B);

    #[inline]
    pub fn stafford_mix_13(z: u64) -> u64 {
        let z = (z ^ (z >> 30)) * 0xBF58476D1CE4E5B9;
        let z = (z ^ (z >> 27)) * 0x94D049BB133111EB;
        (z >> 1) ^ (z >> 32)
    }

    pub fn set_seed(&mut self, seed: u64) {
        let mut seed = seed + 0x9e3779b97f4a7c15;
        self.s10 = Self::stafford_mix_13(seed) % Self::M1;
        seed += 0x9e3779b97f4a7c15;
        self.s11 = Self::stafford_mix_13(seed) % Self::M1;
        seed += 0x9e3779b97f4a7c15;
        self.s12 = Self::stafford_mix_13(seed) % Self::M1;
        seed += 0x9e3779b97f4a7c15;
        self.s20 = Self::stafford_mix_13(seed) % Self::M2;
        seed += 0x9e3779b97f4a7c15;
        self.s21 = Self::stafford_mix_13(seed) % Self::M2;
    }

    #[inline]
    pub fn step(&mut self) -> f64 {
        let mut r = self.s12 - self.s22;
        r -= Self::M1 * ((r - 1) >> 63);

        let p = (Self::A12 * self.s11 - Self::A13 * self.s10 + Self::CORR1) % Self::M1;
        self.s10 = self.s11;
        self.s11 = self.s12;
        self.s12 = p;

        let p = (Self::A21 * self.s21 - Self::A23 * self.s20 + Self::CORR2) % Self::M2;
        self.s20 = self.s21;
        self.s21 = self.s22;
        self.s22 = p;
        r as f64 * Self::NORM
    }

    pub fn set_stream() {
        unimplemented!()
    }
}

impl Default for Mrg32k3aCore {
    fn default() -> Self {
        Self {
            s10: 12345,
            s11: 12345,
            s12: 12345,
            s20: 12345,
            s21: 12345,
            s22: 12345,
        }
    }
}

impl BlockRngCore for Mrg32k3aCore {
    type Item = f64;
    type Results = [f64; N];

    fn generate(&mut self, results: &mut Self::Results) {
        for r in results.iter_mut() {
            *r = self.step();
        }
    }
}

impl SeedableRng for Mrg32k3aCore {
    type Seed = [u8; 32];

    fn from_seed(seed: Self::Seed) -> Self {
        let mut rng = Self::default();
        let seed_u64 = u64::from_le_bytes(seed[0..8].try_into().unwrap());
        rng.set_seed(seed_u64);
        rng
    }
}

pub struct Mrg32k3a {
    pub core: BlockRng<Mrg32k3aCore>,
}

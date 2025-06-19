use rand_core::{
    SeedableRng,
    RngCore,
    block::{BlockRng64, BlockRngCore},
};

pub const N: usize = 16;

const A1P72: [[u64; 3]; 3] = [
    [82758667, 1871391091, 4127413238],
    [3672831523, 69195019, 1871391091],
    [3672091415, 3528743235, 69195019],
];

const A2P72: [[u64; 3]; 3] = [
    [1511326704, 3759209742, 1610795712],
    [4292754251, 1511326704, 3889917532],
    [3859662829, 4292754251, 3708466080],
];

const A1P134: [[u64; 3]; 3] = [
    [1702500920, 1849582496, 1656874625],
    [828554832, 1702500920, 1512419905],
    [1143731069, 828554832, 102237247],
];

const A2P134: [[u64; 3]; 3] = [
    [796789021, 1464208080, 607337906],
    [1241679051, 796789021, 614055150],
    [1401213391, 1241679051, 1998098159],
];

#[derive(Clone, Debug)]
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
    pub const M2: u64 = 4_294_944_443;
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

    fn mat_vec_mul(m: u64, a: &[[u64; 3]; 3], s: &mut [u64; 3]) {
        let mut res = [0u64; 3];
        for i in 0..3 {
            let mut total = 0u128;
            for j in 0..3 {
                total += (a[i][j] as u128 * s[j] as u128) % m as u128;
            }
            res[i] = (total % m as u128) as u64;
        }
        s.copy_from_slice(&res);
    }

    fn apply_matrix(&mut self, a1: &[[u64; 3]; 3], a2: &[[u64; 3]; 3]) {
        let mut v1 = [self.s10, self.s11, self.s12];
        let mut v2 = [self.s20, self.s21, self.s22];
        Self::mat_vec_mul(Self::M1, a1, &mut v1);
        Self::mat_vec_mul(Self::M2, a2, &mut v2);
        self.s10 = v1[0];
        self.s11 = v1[1];
        self.s12 = v1[2];
        self.s20 = v2[0];
        self.s21 = v2[1];
        self.s22 = v2[2];
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
        seed += 0x9e3779b97f4a7c15;
        self.s22 = Self::stafford_mix_13(seed) % Self::M2;
    }

    #[inline]
    pub fn step_u64(&mut self) -> u64 {
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
        r
    }

    #[inline]
    pub fn step(&mut self) -> f64 {
        self.step_u64() as f64 * Self::NORM
    }

    pub fn advance_substream(&mut self) {
        self.apply_matrix(&A1P72, &A2P72);
    }

    pub fn advance_stream(&mut self) {
        self.apply_matrix(&A1P134, &A2P134);
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
    type Item = u64;
    type Results = [u64; N];

    fn generate(&mut self, results: &mut Self::Results) {
        for r in results.iter_mut() {
            *r = self.step_u64();
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

#[derive(Clone)]
pub struct Mrg32k3a {
    pub core: BlockRng64<Mrg32k3aCore>,
}

impl rand_core::RngCore for Mrg32k3a {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        self.core.next_u32()
    }

    #[inline]
    fn next_u64(&mut self) -> u64 {
        self.core.next_u64()
    }

    #[inline]
    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.core.fill_bytes(dest);
    }

}

impl SeedableRng for Mrg32k3a {
    type Seed = <Mrg32k3aCore as SeedableRng>::Seed;

    fn from_seed(seed: Self::Seed) -> Self {
        Self { core: BlockRng64::from_seed(seed) }
    }
}

impl Mrg32k3a {
    pub fn advance_substream(&mut self) {
        self.core.core.advance_substream();
    }

    pub fn advance_stream(&mut self) {
        self.core.core.advance_stream();
    }
}

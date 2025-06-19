use rand_core::{
    SeedableRng,
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
        let z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        let z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        (z >> 1) ^ (z >> 32)
    }

    fn mat_vec_mul(m: u64, a: &[[u64; 3]; 3], s: &mut [u64; 3]) {
        let mut res = [0u64; 3];
        for i in 0..3 {
            let mut total = 0u128;
            (0..3).for_each(|j| {
                total += (a[i][j] as u128 * s[j] as u128) % m as u128;
            });
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
        let mut seed = seed.wrapping_add(0x9e3779b97f4a7c15);
        self.s10 = Self::stafford_mix_13(seed) % Self::M1;
        seed = seed.wrapping_add(0x9e3779b97f4a7c15);
        self.s11 = Self::stafford_mix_13(seed) % Self::M1;
        seed = seed.wrapping_add(0x9e3779b97f4a7c15);
        self.s12 = Self::stafford_mix_13(seed) % Self::M1;
        seed = seed.wrapping_add(0x9e3779b97f4a7c15);
        self.s20 = Self::stafford_mix_13(seed) % Self::M2;
        seed = seed.wrapping_add(0x9e3779b97f4a7c15);
        self.s21 = Self::stafford_mix_13(seed) % Self::M2;
        seed = seed.wrapping_add(0x9e3779b97f4a7c15);
        self.s22 = Self::stafford_mix_13(seed) % Self::M2;
    }

    #[inline]
    pub fn step_u64(&mut self) -> u64 {
        let mut r = self.s12.wrapping_sub(self.s22);
        r = r.wrapping_sub(Self::M1 * ((r.wrapping_sub(1)) >> 63));

        let p = (Self::A12
            .wrapping_mul(self.s11)
            .wrapping_sub(Self::A13.wrapping_mul(self.s10))
            .wrapping_add(Self::CORR1))
            % Self::M1;
        self.s10 = self.s11;
        self.s11 = self.s12;
        self.s12 = p;

        let p = (Self::A21
            .wrapping_mul(self.s21)
            .wrapping_sub(Self::A23.wrapping_mul(self.s20))
            .wrapping_add(Self::CORR2))
            % Self::M2;
        self.s20 = self.s21;
        self.s21 = self.s22;
        self.s22 = p;
        r
    }

    #[inline]
    pub fn step(&mut self) -> f64 {
        self.step_u64() as f64 * Self::NORM
    }

    pub fn advance_substreams(&mut self, n: u64) {
        for _ in 0..n {
            self.apply_matrix(&A1P72, &A2P72);
        }
    }

    pub fn advance_substream(&mut self) {
        self.advance_substreams(1);
    }

    pub fn advance_streams(&mut self, n: u64) {
        for _ in 0..n {
            self.apply_matrix(&A1P134, &A2P134);
        }
    }

    pub fn advance_stream(&mut self) {
        self.advance_streams(1);
    }

    pub fn set_stream(&mut self, n: u64) {
        self.advance_streams(n);
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
        Self {
            core: BlockRng64::from_seed(seed),
        }
    }
}

impl Mrg32k3a {
    pub fn advance_substreams(&mut self, n: u64) {
        self.core.core.advance_substreams(n);
    }

    pub fn advance_substream(&mut self) {
        self.advance_substreams(1);
    }

    pub fn advance_streams(&mut self, n: u64) {
        self.core.core.advance_streams(n);
    }

    pub fn advance_stream(&mut self) {
        self.advance_streams(1);
    }

    pub fn set_stream(&mut self, n: u64) {
        self.core.core.set_stream(n);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand_core::RngCore;
    use rayon::prelude::*;

    #[test]
    fn sequence_from_default_seed() {
        let mut rng = Mrg32k3aCore::default();
        let expected = [
            18446744069414584529,
            545508589,
            545508589,
            1729634130,
            18446744067398777457,
            18446744068605071855,
            1174362367,
            18446744067747663158,
            893945877,
            2017763182,
        ];
        for &e in &expected {
            assert_eq!(rng.step_u64(), e);
        }
    }

    #[test]
    fn sequence_after_set_seed() {
        let mut rng = Mrg32k3aCore::default();
        rng.set_seed(1);
        let expected = [
            3950718346,
            18446744065775893456,
            2489117118,
            2300407520,
            18446744067822805725,
        ];
        for &e in &expected {
            assert_eq!(rng.step_u64(), e);
        }
    }

    #[test]
    fn block_rng_matches_core() {
        let mut core = Mrg32k3aCore::default();
        let mut rng = Mrg32k3a {
            core: BlockRng64::new(Mrg32k3aCore::default()),
        };
        for _ in 0..N * 2 {
            assert_eq!(rng.next_u64(), core.step_u64());
        }
    }

    /// Ensure running several RNG instances in parallel yields the same
    /// sequences as generating them sequentially on a single thread.
    #[test]
    fn multithreaded_consistency() {
        let seeds = [1u64, 2, 3, 4];
        // Pre-compute the sequences produced by each seed on the current
        // thread. These serve as the single-threaded reference results.
        let expected: Vec<Vec<u64>> = seeds
            .iter()
            .map(|&s| {
                let mut r = Mrg32k3aCore::default();
                r.set_seed(s);
                (0..128).map(|_| r.step_u64()).collect::<Vec<_>>()
            })
            .collect();

        let results: Vec<Vec<u64>> = seeds
            .par_iter()
            .map(|&s| {
                let mut r = Mrg32k3aCore::default();
                r.set_seed(s);
                (0..128).map(|_| r.step_u64()).collect::<Vec<_>>()
            })
            .collect();

        for (res, exp) in results.into_iter().zip(expected) {
            assert_eq!(res, exp);
        }
    }

    #[test]
    fn set_stream_equivalent_to_advancing() {
        let mut a = Mrg32k3aCore::default();
        for _ in 0..3 {
            a.advance_stream();
        }

        let mut b = Mrg32k3aCore::default();
        b.set_stream(3);

        for _ in 0..32 {
            assert_eq!(a.step_u64(), b.step_u64());
        }
    }

    #[test]
    fn multithreaded_streams() {
        let streams = rayon::current_num_threads() as u64;

        let expected: Vec<Vec<u64>> = (0..streams)
            .map(|s| {
                let mut r = Mrg32k3aCore::default();
                r.set_stream(s);
                (0..128).map(|_| r.step_u64()).collect::<Vec<_>>()
            })
            .collect();

        let results: Vec<Vec<u64>> = (0..streams)
            .into_par_iter()
            .map(|s| {
                let mut r = Mrg32k3aCore::default();
                r.set_stream(s);
                (0..128).map(|_| r.step_u64()).collect::<Vec<_>>()
            })
            .collect();

        for (res, exp) in results.into_iter().zip(expected) {
            assert_eq!(res, exp);
        }
    }
}

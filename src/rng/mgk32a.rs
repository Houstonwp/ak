use crate::rng::RngCore;

const M1: u64 = 4_294_967_087;
const M2: u64 = 4_294_944_443;
const A12: u64 = 1_403_580;
const A13N: u64 = 810_728;
const A21: u64 = 527_612;
const A23N: u64 = 1_370_589;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SeedError;

#[derive(Debug, Clone, Copy)]
pub struct Mgk32a {
    s1: [u64; 3],
    s2: [u64; 3],
}

impl Mgk32a {
    pub fn new(seed: [u64; 6]) -> Result<Self, SeedError> {
        let s1 = [seed[0], seed[1], seed[2]];
        let s2 = [seed[3], seed[4], seed[5]];
        if !valid_component(&s1, M1) || !valid_component(&s2, M2) {
            return Err(SeedError);
        }
        Ok(Self { s1, s2 })
    }

    pub fn from_seed64(seed: u64) -> Self {
        let mut sm = SplitMix64::new(seed);
        let mut s1 = [0u64; 3];
        let mut s2 = [0u64; 3];
        for i in 0..3 {
            s1[i] = 1 + (sm.next_u64() % (M1 - 1));
            s2[i] = 1 + (sm.next_u64() % (M2 - 1));
        }
        Self { s1, s2 }
    }

    #[inline]
    pub fn next_f64(&mut self) -> f64 {
        let u = self.next_u32() as u64;
        (u as f64) / ((M1 + 1) as f64)
    }

    #[inline]
    pub fn state(&self) -> [u64; 6] {
        [
            self.s1[0], self.s1[1], self.s1[2], self.s2[0], self.s2[1], self.s2[2],
        ]
    }

    pub fn advance(&mut self, delta: u128) {
        if delta == 0 {
            return;
        }
        let m1 = Matrix3::mrg32k3a_m1();
        let m2 = Matrix3::mrg32k3a_m2();
        let m1p = m1.pow(delta, M1);
        let m2p = m2.pow(delta, M2);

        self.s1 = m1p.mul_vec(self.s1, M1);
        self.s2 = m2p.mul_vec(self.s2, M2);
    }

    pub fn for_stream(seed: [u64; 6], stream: u128, stride: u128) -> Result<Self, SeedError> {
        let mut rng = Self::new(seed)?;
        rng.advance(stream.saturating_mul(stride));
        Ok(rng)
    }
}

impl RngCore for Mgk32a {
    #[inline]
    fn next_u32(&mut self) -> u32 {
        let x = mod_m(
            (A12 as i128) * (self.s1[1] as i128) - (A13N as i128) * (self.s1[2] as i128),
            M1,
        );
        self.s1 = [x, self.s1[0], self.s1[1]];

        let y = mod_m(
            (A21 as i128) * (self.s2[0] as i128) - (A23N as i128) * (self.s2[2] as i128),
            M2,
        );
        self.s2 = [y, self.s2[0], self.s2[1]];

        let u = if x > y { x - y } else { x + M1 - y };
        u as u32
    }
}

#[inline]
fn valid_component(v: &[u64; 3], modulus: u64) -> bool {
    v.iter().all(|&x| (1..modulus).contains(&x))
}

#[inline]
fn mod_m(value: i128, modulus: u64) -> u64 {
    let m = modulus as i128;
    let mut v = value % m;
    if v < 0 {
        v += m;
    }
    v as u64
}

#[derive(Clone, Copy)]
struct Matrix3 {
    a00: u64,
    a01: u64,
    a02: u64,
    a10: u64,
    a11: u64,
    a12: u64,
    a20: u64,
    a21: u64,
    a22: u64,
}

impl Matrix3 {
    const fn mrg32k3a_m1() -> Self {
        Self {
            a00: 0,
            a01: A12,
            a02: M1 - A13N,
            a10: 1,
            a11: 0,
            a12: 0,
            a20: 0,
            a21: 1,
            a22: 0,
        }
    }

    const fn mrg32k3a_m2() -> Self {
        Self {
            a00: A21,
            a01: 0,
            a02: M2 - A23N,
            a10: 1,
            a11: 0,
            a12: 0,
            a20: 0,
            a21: 1,
            a22: 0,
        }
    }

    fn pow(self, mut exp: u128, modulus: u64) -> Self {
        let mut base = self;
        let mut acc = Self::identity();
        while exp > 0 {
            if exp & 1 == 1 {
                acc = acc.mul(base, modulus);
            }
            exp >>= 1;
            if exp > 0 {
                base = base.mul(base, modulus);
            }
        }
        acc
    }

    const fn identity() -> Self {
        Self {
            a00: 1,
            a01: 0,
            a02: 0,
            a10: 0,
            a11: 1,
            a12: 0,
            a20: 0,
            a21: 0,
            a22: 1,
        }
    }

    fn mul(self, other: Self, modulus: u64) -> Self {
        let m = modulus as u128;
        let mul = |a: u64, b: u64| (a as u128) * (b as u128) % m;

        let a00 =
            (mul(self.a00, other.a00) + mul(self.a01, other.a10) + mul(self.a02, other.a20)) % m;
        let a01 =
            (mul(self.a00, other.a01) + mul(self.a01, other.a11) + mul(self.a02, other.a21)) % m;
        let a02 =
            (mul(self.a00, other.a02) + mul(self.a01, other.a12) + mul(self.a02, other.a22)) % m;

        let a10 =
            (mul(self.a10, other.a00) + mul(self.a11, other.a10) + mul(self.a12, other.a20)) % m;
        let a11 =
            (mul(self.a10, other.a01) + mul(self.a11, other.a11) + mul(self.a12, other.a21)) % m;
        let a12 =
            (mul(self.a10, other.a02) + mul(self.a11, other.a12) + mul(self.a12, other.a22)) % m;

        let a20 =
            (mul(self.a20, other.a00) + mul(self.a21, other.a10) + mul(self.a22, other.a20)) % m;
        let a21 =
            (mul(self.a20, other.a01) + mul(self.a21, other.a11) + mul(self.a22, other.a21)) % m;
        let a22 =
            (mul(self.a20, other.a02) + mul(self.a21, other.a12) + mul(self.a22, other.a22)) % m;

        Self {
            a00: a00 as u64,
            a01: a01 as u64,
            a02: a02 as u64,
            a10: a10 as u64,
            a11: a11 as u64,
            a12: a12 as u64,
            a20: a20 as u64,
            a21: a21 as u64,
            a22: a22 as u64,
        }
    }

    fn mul_vec(self, v: [u64; 3], modulus: u64) -> [u64; 3] {
        let m = modulus as u128;
        let a0 = ((self.a00 as u128) * (v[0] as u128)
            + (self.a01 as u128) * (v[1] as u128)
            + (self.a02 as u128) * (v[2] as u128))
            % m;
        let a1 = ((self.a10 as u128) * (v[0] as u128)
            + (self.a11 as u128) * (v[1] as u128)
            + (self.a12 as u128) * (v[2] as u128))
            % m;
        let a2 = ((self.a20 as u128) * (v[0] as u128)
            + (self.a21 as u128) * (v[1] as u128)
            + (self.a22 as u128) * (v[2] as u128))
            % m;
        [a0 as u64, a1 as u64, a2 as u64]
    }
}

struct SplitMix64 {
    state: u64,
}

impl SplitMix64 {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u64(&mut self) -> u64 {
        let mut z = self.state.wrapping_add(0x9E3779B97F4A7C15);
        self.state = z;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        z ^ (z >> 31)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rng::RngCore;

    #[test]
    fn mgk32a_known_sequence() {
        let mut rng = Mgk32a::new([12345; 6]).unwrap();
        let expected = [
            545_508_589,
            1_368_065_410,
            1_327_943_761,
            3_546_985_096,
            951_893_194,
        ];
        let mut actual = [0u32; 5];
        for v in &mut actual {
            *v = rng.next_u32();
        }
        assert_eq!(actual, expected);
    }

    #[test]
    fn mgk32a_advance_matches_iter() {
        let seed = [1, 2, 3, 4, 5, 6];
        let mut advanced = Mgk32a::new(seed).unwrap();
        let mut iterated = Mgk32a::new(seed).unwrap();
        advanced.advance(1_000);
        for _ in 0..1_000 {
            iterated.next_u32();
        }
        assert_eq!(advanced.state(), iterated.state());
        assert_eq!(advanced.next_u32(), iterated.next_u32());
    }
}

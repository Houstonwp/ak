pub mod gaussian;

pub trait RNG: Clone {
    fn init(&mut self, dimensions: usize);
    fn next_uniform(&mut self, v: &mut Vec<f64>);
    fn next_normal(&mut self, v: &mut Vec<f64>);
    fn dimensions(&self) -> usize;
}

#[derive(Default)]
pub struct Mrg32k3a {
    pub a: f64,
    pub b: f64,
    pub xn: f64,
    pub xn1: f64,
    pub xn2: f64,
    pub yn: f64,
    pub yn1: f64,
    pub yn2: f64,
    pub dimensions: usize,
    pub antithetic: bool,
    pub cached_uniforms: Vec<f64>,
    pub cached_normals: Vec<f64>,
}

impl Mrg32k3a {
    pub const M1: f64 = 4294967087.0;
    pub const M2: f64 = 4294944443.0;
    pub const A12: f64 = 1403580.0;
    pub const A13: f64 = 810728.0;
    pub const A21: f64 = 527612.0;
    pub const A23: f64 = 1370589.0;
    pub const M1P1: f64 = 4294967088.0;

    pub fn seed(&mut self, a: u32, b: u32) {
        self.a = a.into();
        self.b = b.into();

        self.reset();
    }

    pub fn reset(&mut self) {
        self.xn = self.a;
        self.xn1 = self.a;
        self.xn2 = self.a;
        self.yn = self.b;
        self.yn1 = self.b;
        self.yn2 = self.b;
        self.antithetic = false;
    }

    pub fn init(&mut self, dimensions: usize) {
        self.dimensions = dimensions;
        self.cached_uniforms = vec![0.0; dimensions];
        self.cached_normals = vec![0.0; dimensions];
    }

    pub fn new(a: u32, b: u32, dimensions: usize) -> Self {
        let mut rng = Mrg32k3a {
            ..Default::default()
        };

        rng.seed(a, b);
        rng.init(dimensions);
        rng
    }

    pub fn next_number(&mut self) -> f64 {
        let mut x = Mrg32k3a::A12 * self.xn1 - Mrg32k3a::A13 * self.xn2;
        x -= (x / Mrg32k3a::M1).floor() * Mrg32k3a::M1;
        if x < 0.0 {
            x += Mrg32k3a::M1;
        }
        self.xn2 = self.xn1;
        self.xn1 = self.xn;
        self.xn = x;

        let mut y = Mrg32k3a::A21 * self.yn - Mrg32k3a::A23 * self.yn2;
        y -= (y / Mrg32k3a::M2).floor() * Mrg32k3a::M2;
        if y < 0.0 {
            y += Mrg32k3a::M2;
        }
        self.yn2 = self.yn1;
        self.yn1 = self.yn;
        self.yn = y;

        if x > y {
            (x - y) / Mrg32k3a::M1P1
        } else {
            (x - y + Mrg32k3a::M1) / Mrg32k3a::M1P1
        }
    }

    pub fn next_uniform(&mut self, v: &mut [f64]) {
        if self.antithetic {
            for (dst, &src) in v.iter_mut().zip(self.cached_uniforms.iter()) {
                *dst = 1.0 - src;
            }
            self.antithetic = false;
        } else {
            for dst in v.iter_mut() {
                *dst = self.next_number();
            }
            self.cached_uniforms.copy_from_slice(v);
            self.antithetic = true;
        }
    }

    pub fn next_normal(&mut self, v: &mut [f64]) {
        if self.antithetic {
            for (dst, &src) in v.iter_mut().zip(self.cached_normals.iter()) {
                *dst = -src;
            }
            self.antithetic = false;
        } else {
            for dst in v.iter_mut() {
                *dst = self.next_number();
            }
            self.cached_normals.copy_from_slice(v);
            self.antithetic = true;
        }
    }
}

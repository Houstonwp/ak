pub mod mgk32a;
pub mod sobol;

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

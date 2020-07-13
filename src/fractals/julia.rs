use num::complex::Complex;

#[allow(dead_code)]
pub fn julia(mut z: Complex<f64>, c: Complex<f64>, limit: u32) -> Option<u32> {
    for i in 0..limit {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
    }

    None
}

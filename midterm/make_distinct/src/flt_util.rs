use std::cmp::Ordering;
use rand::{rngs::ThreadRng, Rng, seq::SliceRandom};
use std::hash::{Hash, Hasher};
use std::mem::transmute;
use crate::vinit::compute_distinct_required;

#[derive(Copy, Clone)]
pub struct F64 {
    x: f64,
}

pub fn create_rd_flt(rng: &mut ThreadRng) -> F64 {
    F64 { x: rng.gen::<f64>() * rng.gen::<u16>() as f64 }
}

pub fn create_scaled_rd_flt(rng: &mut ThreadRng, scale: f64) -> F64 {
    F64 { x: rng.gen::<f64>() * scale }
}

impl Eq for F64 {}
impl PartialEq for F64 {
    fn eq(&self, other: &F64) -> bool {
        (self.x - other.x).abs() < f64::EPSILON
    }
}

impl Ord for F64 {
    fn cmp(&self, o: &F64) -> Ordering {
        let diff = self.x - o.x;

        if diff < 0.0 {
            Ordering::Less
        } else if diff > 0.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for F64 {
    fn partial_cmp(&self, o: &F64) -> Option<Ordering> {
        let diff = self.x - o.x;
        Some(if diff < 0.0 {
            Ordering::Less
        } else if diff > 0.0 {
            Ordering::Greater
        } else {
            Ordering::Equal
        })
    }
}

impl Hash for F64 {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mts_mask: u64 = 0xfffffffffffff;
        let exp_mask: u64 = 0x7ff0000000000000;
        let sign_mask: u64 = 1 << 63;

        let int_x: u64 = unsafe {
            transmute::<f64, u64>(self.x)
        };

        let x_mantissa = int_x & mts_mask;
        let x_exponent = int_x & exp_mask;
        let x_sign = int_x & sign_mask;

        [x_mantissa, x_exponent, x_sign].hash(state);
    }
}

pub fn create_flt_vec(n: usize, f: f64, rng: &mut ThreadRng) -> Vec<F64> {
    let distinct_required = compute_distinct_required(n, f);
    let distinct_vec: Vec<F64> = (0..distinct_required)
        .map(|_| create_rd_flt(rng))
        .collect();
    (0..n)
        .map(|_| distinct_vec.choose(rng).unwrap().to_owned())
        .collect()
}

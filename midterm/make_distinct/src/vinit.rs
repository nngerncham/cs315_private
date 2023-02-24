use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
use random_string::generate;

pub fn compute_distinct_required(n: usize, f: f64) -> usize {
    let approx = f * (n as f64);
    approx as usize
}

pub fn create_int_vec(n: usize, f: f64, rng: &mut ThreadRng) -> Vec<i32> {
    let distinct_required = compute_distinct_required(n, f);
    let distinct_vec: Vec<i32> = (0..distinct_required).map(|_| rng.gen::<i32>()).collect();
    (0..n)
        .map(|_| distinct_vec.choose(rng).unwrap().to_owned())
        .collect()
}

pub fn create_str_vec(n: usize, f: f64, rng: &mut ThreadRng) -> Vec<String> {
    let distinct_required = compute_distinct_required(n, f);
    let distinct_vec: Vec<String> = (0..distinct_required)
        .map(|_| {
            let length = rng.gen::<usize>() % 30;
            generate(length, "abcdefghijklmnopqrstuvwxyz")
        })
        .collect();
    (0..n)
        .map(|_| distinct_vec.choose(rng).unwrap().to_owned())
        .collect()
}

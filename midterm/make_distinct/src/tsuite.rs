use crate::cat::create_cat_vec;
use crate::unique::*;
use crate::vinit::*;
use crate::flt_util::*;
use core::arch::x86_64::_rdtsc;
use rand::rngs::ThreadRng;

const NS: [usize; 5] = [100, 10_000, 100_000, 1_000_000, 10_000_000];
const FORS: [f64; 4] = [0.1, 0.25, 0.5, 0.75];

pub fn test_int(rng: &mut ThreadRng) -> Vec<String> {
    let mut results = Vec::new();

    for &n in &NS {
        for &f in &FORS {
            let sample = create_int_vec(n, f, rng);

            unsafe {
                let start = _rdtsc();
                let unique_res = unique_hashset(&sample);
                let stop = _rdtsc();

                results.push(format!("hashset,int,{},{},{},{}", n, f, stop - start, unique_res.len() as f64 / sample.len() as f64));

                let start = _rdtsc();
                let unique_res = unique_sorted(&sample);
                let stop = _rdtsc();

                results.push(format!("sorted,int,{},{},{},{}", n, f, stop - start, unique_res.len() as f64 / sample.len() as f64));
            }
        }
    }

    results
}

pub fn test_flt(rng: &mut ThreadRng) -> Vec<String> {
    let mut results = Vec::new();

    for &n in &NS {
        for &f in &FORS {
            let sample = create_flt_vec(n, f, rng);

            unsafe {
                let start = _rdtsc();
                let unique_res = unique_hashset(&sample);
                let stop = _rdtsc();

                results.push(format!("hashset,float,{},{},{},{}", n, f, stop - start, unique_res.len() as f64 / sample.len() as f64));

                let start = _rdtsc();
                let unique_res = unique_sorted(&sample);
                let stop = _rdtsc();

                results.push(format!("sorted,float,{},{},{},{}", n, f, stop - start, unique_res.len() as f64 / sample.len() as f64));
            }
        }
    }

    results
}

pub fn test_str(rng: &mut ThreadRng) -> Vec<String> {
    let mut results = Vec::new();

    for &n in &NS {
        for &f in &FORS {
            let sample = create_str_vec(n, f, rng);

            unsafe {
                let start = _rdtsc();
                let unique_res = unique_hashset(&sample);
                let stop = _rdtsc();

                results.push(format!("hashset,string,{},{},{},{}", n, f, stop - start, unique_res.len() as f64 / sample.len() as f64));

                let start = _rdtsc();
                let unique_res = unique_sorted(&sample);
                let stop = _rdtsc();

                results.push(format!("sorted,string,{},{},{},{}", n, f, stop - start, unique_res.len() as f64 / sample.len() as f64));
            }
        }
    }

    results
}

pub fn test_cat(rng: &mut ThreadRng) -> Vec<String> {
    let mut results = Vec::new();

    for &n in &NS {
        for &f in &FORS {
            let sample = create_cat_vec(n, f, rng);

            unsafe {
                let start = _rdtsc();
                let unique_res = unique_hashset(&sample);
                let stop = _rdtsc();

                results.push(format!("hashset,cat,{},{},{},{}", n, f, stop - start, unique_res.len() as f64 / sample.len() as f64));

                let start = _rdtsc();
                let unique_res = unique_sorted(&sample);
                let stop = _rdtsc();

                results.push(format!("sorted,cat,{},{},{},{}", n, f, stop - start, unique_res.len() as f64 / sample.len() as f64));
            }
        }
    }

    results
}

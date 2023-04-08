mod dac;
mod linear;
mod point;

use crate::dac::closest_pairs;
use crate::linear::lin_closest_pair;
use crate::point::Point;

use csv::Writer;
use rand::prelude::*;
use std::arch::x86_64::_rdtsc;
use std::collections::HashSet;
use std::error::Error;

fn create_points(n: usize) -> Vec<Point> {
    let mut rng = rand::thread_rng();
    (0..n)
        .into_iter()
        .map(|_| (rng.gen_range(0..1000), rng.gen_range(0..1000)))
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    let ns = [1_000, 10_000, 100_000, 1_000_000];
    let trials = 100;

    let mut wtr = Writer::from_path("results.csv")?;
    wtr.write_record(&["size", "type", "cycles"])?;

    for n in ns {
        for _ in 0..trials {
            let points = create_points(n);
            let unique_points: HashSet<Point> = HashSet::from_iter(points.into_iter());
            let points = Vec::from_iter(unique_points.into_iter());
            let size = points.len();

            unsafe {
                let start = _rdtsc();
                let _ = closest_pairs(&points);
                let end = _rdtsc();

                wtr.write_record(&[
                    format!("{}", size),
                    String::from("dac"),
                    format!("{}", end - start),
                ])?;
            }

            unsafe {
                let start = _rdtsc();
                let _ = lin_closest_pair(&points);
                let end = _rdtsc();

                wtr.write_record(&[
                    format!("{}", size),
                    String::from("linear"),
                    format!("{}", end - start),
                ])?;
            }
        }
    }

    Ok(())
}

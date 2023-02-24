use crate::flt_util::{create_scaled_rd_flt, F64};
use crate::vinit::compute_distinct_required;
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};
use random_string::generate;
use std::cmp::Ordering;

#[derive(Hash, Clone)]
pub struct Cat {
    name: String,
    age: u8,
    weight: F64,
    owner_phone_numbers: Vec<u32>,
}

impl Eq for Cat {}
impl PartialEq for Cat {
    fn eq(&self, o: &Cat) -> bool {
        self.name == o.name && self.weight == o.weight && self.age == o.age
    }
}

impl Ord for Cat {
    fn cmp(&self, o: &Cat) -> Ordering {
        if self.age > o.age {
            return Ordering::Greater;
        } else if self.age < o.age {
            return Ordering::Less;
        }

        if self.weight < o.weight {
            return Ordering::Less;
        } else if self.weight > o.weight {
            return Ordering::Greater;
        }

        if self.name < o.name {
            return Ordering::Less;
        } else if self.name > o.name {
            return Ordering::Greater;
        }

        Ordering::Equal
    }
}

impl PartialOrd for Cat {
    fn partial_cmp(&self, o: &Cat) -> Option<Ordering> {
        if self.age > o.age {
            return Some(Ordering::Greater);
        } else if self.age < o.age {
            return Some(Ordering::Less);
        }

        if self.weight < o.weight {
            return Some(Ordering::Less);
        } else if self.weight > o.weight {
            return Some(Ordering::Greater);
        }

        if self.name < o.name {
            return Some(Ordering::Less);
        } else if self.name > o.name {
            return Some(Ordering::Greater);
        }

        Some(Ordering::Equal)
    }
}

fn create_owner_number(rng: &mut ThreadRng) -> Vec<u32> {
    let owner_count = rng.gen::<u8>() % 5;
    (0..owner_count)
        .map(|_| rng.gen::<u32>() % 9_999_999)
        .collect()
}

fn birth_cat(rng: &mut ThreadRng) -> Cat {
    Cat {
        name: generate(8, "abcdefghijklmnopqrstuvwxyz"),
        age: rng.gen::<u8>(),
        weight: create_scaled_rd_flt(rng, 20.0),
        owner_phone_numbers: create_owner_number(rng),
    }
}

pub fn create_cat_vec(n: usize, f: f64, rng: &mut ThreadRng) -> Vec<Cat> {
    let distinct_required = compute_distinct_required(n, f);
    let distinct_vec: Vec<Cat> = (0..distinct_required).map(|_| birth_cat(rng)).collect();
    (0..n)
        .map(|_| distinct_vec.choose(rng).unwrap().to_owned())
        .collect()
}

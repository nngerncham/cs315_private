use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;

pub fn unique_hashset<T>(data: &Vec<T>) -> Vec<T>
where
    T: Hash + Eq + Clone,
{
    if data.len() == 0 {
        return data.to_vec();
    }

    let new_data: Vec<T> = data.to_vec();
    let unique_set: HashSet<T> = HashSet::from_iter(new_data);
    Vec::from_iter(unique_set)
}

pub fn unique_sorted<T>(data: &Vec<T>) -> Vec<T>
where
    T: Clone + Ord,
{
    if data.len() == 0 {
        return data.to_vec();
    }

    let mut sorted_vec = data.to_vec();
    sorted_vec.sort();

    let mut final_vec = Vec::from([sorted_vec[0].clone()]);
    for i in 1..sorted_vec.len() {
        if sorted_vec[i - 1].cmp(&sorted_vec[i]).is_ne() {
            final_vec.push(sorted_vec[i].clone());
        }
    }

    final_vec
}

#[allow(dead_code)]
pub fn unique_sorted_by<T, F>(data: &Vec<T>, mut compare: F) -> Vec<T>
where
    T: Clone,
    F: FnMut(&T, &T) -> Ordering,
{
    if data.len() == 0 {
        return data.to_vec();
    }

    let mut sorted_vec = data.to_vec();
    sorted_vec.sort_by(&mut compare);

    let mut final_vec = Vec::from([sorted_vec[0].clone()]);
    for i in 1..sorted_vec.len() {
        if compare(&sorted_vec[i - 1], &sorted_vec[i]).is_ne() {
            final_vec.push(sorted_vec[i].clone());
        }
    }

    final_vec
}

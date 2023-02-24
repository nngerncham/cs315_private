mod cat;
mod flt_util;
mod tsuite;
mod unique;
mod vinit;

use crate::tsuite::*;
use project_root::get_project_root;
use rand::thread_rng;
use csv::Writer;

fn main() {
    let res_path = get_project_root().unwrap().join("results.csv");
    let mut wtr = Writer::from_path(res_path).unwrap();
    let _ = wtr.write_record(&["method", "data_type", "size", "fraction", "time"]);
    let _ = wtr.flush();

    let mut rng = thread_rng();

    let int_results = test_int(&mut rng);
    int_results.iter().for_each(|rec| {
        let _ = wtr.write_record(rec.split(','));
    });
    let _ = wtr.flush();

    let flt_results = test_flt(&mut rng);
    flt_results.iter().for_each(|rec| {
        let _ = wtr.write_record(rec.split(','));
    });
    let _ = wtr.flush();

    let str_results = test_str(&mut rng);
    str_results.iter().for_each(|rec| {
        let _ = wtr.write_record(rec.split(','));
    });
    let _ = wtr.flush();

    let cat_results = test_cat(&mut rng);
    cat_results.iter().for_each(|rec| {
        let _ = wtr.write_record(rec.split(','));
    });
    let _ = wtr.flush();
}

use std::fs;
use std::env;
use ndarray::{Array, Array1, Array2, Axis};

struct SpecLANL {
    md: f64,
    mw: f64,
    vd: f64,
    vw: f64,
    fnu: Array2<f32>,
    la: Array2<f32>,
    t: Array<f32>
}

impl SpecLANL {
    fn new(md: f64, mw: f64, vd: f64, vw: f64, filename: &str) -> SpecLANL {
        let buf = load_dat_file_as_string(filename);

        let (t, la, fnu) = load

        SpecLANL { md: md, mw: mw, vd: vd, vw: vw }
    }
}

fn load_dat_file_as_string(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Couldn't read file");
    contents
}

fn  load_arrays_from_dat_file(filename: &str) 
        -> (Array<f32>, Array2<f32>, Array2<f32>) {
        
    let buf = load_dat_file_as_string(filename);

    (t, la, fnu)
}

fn main() {
    println!("Hello, world!");
}

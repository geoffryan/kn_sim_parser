use std::fs;
use std::env;
use std::str::FromStr;
//use ndarray::prelude::*;
use ndarray::{Array1, Array2, Array3, arr0};
use hdf5::{File, Result};

#[derive(Debug)]
enum Topo {
    P,
    S,
}

#[derive(Debug)]
enum LanthWind {
    One,
    Two,
}

struct SpecLANL {
    topo: Topo,
    wind: LanthWind,
    md: f64,
    mw: f64,
    vd: f64,
    vw: f64,
    t: Array1<f32>,
    la: Array2<f32>,
    th: Array2<f32>,
    fla: Array3<f32>,
}

impl Topo {
    fn val(&self) -> i32 {
        match self {
            Topo::S => 0,
            Topo::P => 1
        }
    }
    
    fn str_val(&self) -> &str {
        match self {
            Topo::S => "S",
            Topo::P => "P"
        }
    }
}

impl LanthWind {
    fn val(&self) -> i32 {
        match self {
            LanthWind::One => 1,
            LanthWind::Two => 2,
        }
    }
}

impl SpecLANL {
    fn new(filename: &str) -> SpecLANL {

        let (topo, wind, md, mw, vd, vw) = parse_filename(filename);
        let (t, la, th, fla) = load_arrays_from_dat_file(filename);

        SpecLANL { topo: topo, wind: wind, md: md, mw: mw, vd: vd, vw: vw,
                    t: t, la: la, th: th, fla: fla}
    }

    fn gen_h5_filename(&self) -> String {
        format!("LANL_KNSpec_T{}_wind{}_md{:.3}_vd{:.3}_mw{:.3}_vw{:.3}.h5",
                self.topo.str_val(), self.wind.val(), self.md, self.vd,
                self.mw, self.vw)
    }

    fn save_h5(&self, directory: &str) -> Result<()> {

        let mut filename = String::from_str(directory).unwrap();
        if filename.len() > 0 {
            filename.push_str("/");
        }
        filename.push_str(&self.gen_h5_filename());

        println!("Saving {}", filename);

        let file = File::create(filename)?;
        //let builder = file.new_dataset_builder();

        file.new_dataset_builder().with_data(&arr0(self.topo.val()))
                                  .create("topo")?;
        file.new_dataset_builder().with_data(&arr0(self.wind.val()))
                                  .create("wind")?;
        file.new_dataset_builder().with_data(&arr0(self.md))
                                  .create("md_Msolar")?;
        file.new_dataset_builder().with_data(&arr0(self.mw))
                                  .create("mw_Msolar")?;
        file.new_dataset_builder().with_data(&arr0(self.vd))
                                  .create("vd_c")?;
        file.new_dataset_builder().with_data(&arr0(self.vw))
                                  .create("vw_c")?;
        file.new_dataset_builder().with_data(&self.t).create("t_days")?;
        file.new_dataset_builder().with_data(&self.la).create("lambda_cm")?;
        file.new_dataset_builder().with_data(&self.th).create("theta_rad")?;
        file.new_dataset_builder().with_data(&self.fla)
                                  .create("fla_cgs_per_angstrom")?;

        Ok(())
    }
}

fn parse_filename(filename: &str) -> (Topo, LanthWind, f64, f64, f64, f64) {
    let toks: Vec<&str> = filename.split("/").last().unwrap().split("_").collect();

    let topo = match toks[1] {
        "TP" => Topo::P,
        _ => Topo::S};
    let wind = match toks[5] {
        "wind2" => LanthWind::Two,
        _ => LanthWind::One};

    let md = toks[7][2..].parse::<f64>().unwrap();
    let vd = toks[8][2..].parse::<f64>().unwrap();
    let mw = toks[9][2..].parse::<f64>().unwrap();
    let vw = toks[10][2..].parse::<f64>().unwrap();

    (topo, wind, md, mw, vd, vw)
}

fn load_dat_file_as_string(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Couldn't read file");
    contents
}

fn load_arrays_from_dat_file(filename: &str) 
        -> (Array1<f32>, Array2<f32>, Array2<f32>, Array3<f32>) {
        
    println!("Loading {}", filename);
    let buf = load_dat_file_as_string(filename);

    let mut tv: Vec<f32> = Vec::new();
    let mut lav: Vec<f32> = Vec::new();
    let mut flav: Vec<f32> = Vec::new();
   
    println!("Parsing!");
    for block in buf.trim().split("\n\n\n") {
        let (tb, lab, mut flab) = parse_block(block);

        tv.push(tb);
        lav = lab;
        flav.append(&mut flab);
    }

    let nt = tv.len();
    let nla = lav.len() / 2;
    let nth = flav.len() / (nt * nla);

    let t = Array1::from_vec(tv);
    let la = Array2::from_shape_vec((nla, 2), lav).unwrap();
    let fla = Array3::from_shape_vec((nt, nla, nth), flav).unwrap();

    let mut thv: Vec<f32> = Vec::new();
    for i in 0..nth {
        // bins are equal in solid angle
        thv.push((1.0 - ((2*i) as f32) / (nth as f32)).acos());
        thv.push((1.0 - ((2*(i+1)) as f32) / (nth as f32)).acos());
    }

    let th = Array2::from_shape_vec((nth, 2), thv).unwrap();

    (t, la, th, fla)
}

fn parse_block(block: &str) -> (f32, Vec<f32>, Vec<f32>) 
{
    let lines: Vec<&str> = block.split("\n").collect();

    let t: f32 = lines[0].split_whitespace().last().unwrap().parse().unwrap();

    let mut la: Vec<f32> = Vec::new();
    let mut fla: Vec<f32>  = Vec::new();

    for i in 1..lines.len() {
        let words: Vec<&str> = lines[i].split_whitespace().collect();
        la.push(words[0].parse().unwrap());
        la.push(words[1].parse().unwrap());

        for j in 2..words.len() {
            fla.push(words[j].parse().unwrap());
        }
    }

    (t, la, fla)
}

fn main() -> Result<(), ()> {

    let args: Vec<String> = env::args().collect();

    let nargs = args.len();

    if nargs < 3 {
        println!("\nusage: $ lanl_parser [input_files.dat ...] output_dir");
        println!("\nThis utility parses the LANL Kilonova spectrum .dat files \
                 into HDF5 files.  The output files are placed in \
                 output_dir. No transformations are done other than parsing \
                 into single-precision floating-point values (f32).\n");
        return Ok(());
    }

    let output_dir = &args[nargs-1];

    for filename in args[1..(nargs-1)].iter() {
        let spec = SpecLANL::new(filename);
        spec.save_h5(&output_dir).unwrap();
    }

    Ok(())
}

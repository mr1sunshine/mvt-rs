use mvt::{decode, FeatureWithJson};

use std::fs::File;
use std::io::prelude::*;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut f = File::open(&args[1]).unwrap();
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).unwrap();

    let tile = decode::<FeatureWithJson>(&buffer).unwrap();
    println!("{:?}", tile);
}
#[macro_use]
extern crate clap;
use clap::App;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let yml = load_yaml!("cli.yml");
    let m = App::from_yaml(yml).get_matches();

    let nbins = value_t!(m, "nbins", usize).unwrap();

    let mut hist: Vec<Vec<u32>> = vec![vec![0; nbins]; nbins];

    let file = File::open(m.value_of("INPUT").unwrap())
        .expect("Error reading file");
    for line in BufReader::new(file).lines() {
        let line = line.unwrap();
        let mut vals = line.split_whitespace();
        vals.next();
        let c1: usize = vals.next().unwrap().parse().expect("Invalid file format");
        let c2: usize = vals.next().unwrap().parse().expect("Invalid file format");
        if (c1 < nbins) & (c2 < nbins) {
            hist[c1][c2] += 1;
        }
    }

    for row in hist {
        let line = row.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(m.value_of("sep").unwrap());
        println!("{}", line);
    }
}

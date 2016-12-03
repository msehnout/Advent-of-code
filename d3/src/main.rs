extern crate regex;

use regex::Regex;
use std::str::FromStr;
use std::cmp;
use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::io::Read;
use std::fs::File;

fn is_triangle(sample: (u32, u32, u32)) -> u32 {
    let mut v = vec![sample.0, sample.1, sample.2];
    v.sort();
    if v[2] < (v[1] + v[0]) {
        1
    } else {
        0
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let input: String = if args.len() < 2 {
        println!("No input");
        return;
    } else {
        args.pop().unwrap()
    };
    println!("I'm going to read file: {}", input);
    let f = File::open(input).unwrap();
    let mut reader = BufReader::new(&f);
    let mut file = String::new();
    let _ = reader.read_to_string(&mut file).unwrap();

    // First part:

    let re = Regex::new(r"\s*(\d+)\s*(\d+)\s*(\d+)\n").unwrap();
    let num_of_triang: u32 = re.captures_iter(&file).map(|cap| {
        // let sample = (u32::from_str(cap.at(1).unwrap()).unwrap(),u32::from_str(cap.at(2).unwrap()).unwrap(),u32::from_str(cap.at(3).unwrap()).unwrap());
        // println!("[{}],[{}],[{}] -> {}", sample.0, sample.1, sample.2, is_triangle(sample));
        // sample
        (u32::from_str(cap.at(1).unwrap()).unwrap(),u32::from_str(cap.at(2).unwrap()).unwrap(),u32::from_str(cap.at(3).unwrap()).unwrap())
    }).map(is_triangle).sum();
    println!("Number of triangles: {}", num_of_triang);

    // Second part:
    let mut i = 0;
    let mut acc = 0;
    let mut s1 = (0,0,0);
    let mut s2 = (0,0,0);
    let mut s3 = (0,0,0);
    let re = Regex::new(r"(\d+)\s+(\d+)\s+(\d+)").unwrap();
    for line in file.lines() {
        let cap = re.captures(line).unwrap();
        let sample = (u32::from_str(cap.at(1).unwrap()).unwrap(),u32::from_str(cap.at(2).unwrap()).unwrap(),u32::from_str(cap.at(3).unwrap()).unwrap());
        if i == 0 {
            s1.0 = sample.0;
            s2.0 = sample.1;
            s3.0 = sample.2;
            i += 1;
        } else if i == 1 {
            s1.1 = sample.0;
            s2.1 = sample.1;
            s3.1 = sample.2;
            i += 1;
        } else {
            s1.2 = sample.0;
            s2.2 = sample.1;
            s3.2 = sample.2;
            i = 0;
            acc += is_triangle(s1) + is_triangle(s2) + is_triangle(s3);
        }
    }
    println!("Number of triangles: {}", acc);
}

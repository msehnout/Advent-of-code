extern crate regex;

use std::io::{self, Read};
use regex::{Regex, Captures};

fn decompress1(line: &str, re: &Regex) -> String {
    let mut pos = 0;
    let mut result = String::new();
    while let Some(cap) = re.captures(&line[pos..]) {
        let (start, end) = cap.pos(0).unwrap();
        let num_of_chars: usize = cap.at(1).unwrap().parse().unwrap();
        let repeat: usize = cap.at(2).unwrap().parse().unwrap();
        result.push_str(&line[pos..pos+start]);
        for _ in 0..repeat {
            result.push_str(&line[pos+end..pos+end+num_of_chars]);
        }
        pos += end+num_of_chars;
    }
    result.push_str(&line[pos..]);
    result
}

fn decompress(line: &str, re: &Regex) -> usize {
    let mut pos = 0;
    let mut result = 0;
    while let Some(cap) = re.captures(&line[pos..]) {
        let (start, end) = cap.pos(0).unwrap();
        let num_of_chars: usize = cap.at(1).unwrap().parse().unwrap();
        let repeat: usize = cap.at(2).unwrap().parse().unwrap();
        result += line[pos..pos+start].len();
        for _ in 0..repeat {
            result += decompress(&line[pos+end..pos+end+num_of_chars], &re);
        }
        pos += end+num_of_chars;
    }
    result += line[pos..].len();
    result
}

fn main() {
    let re = Regex::new(r"\((\d+)x(\d+)\)").unwrap();
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    // PART 1:
    let mut sum=0;
    for line in buffer.lines() {
        let result = decompress1(&line, &re);     
        sum += result.len();
        // println!("{} {}", result, result.len());
    }
    println!("Total length v1: {}", sum);
    // PART 2:
    let mut sum=0;
    for line in buffer.lines() {
        let result = decompress(&line, &re);     
        sum += result;
    }
    println!("Total length v2: {}", sum);
}

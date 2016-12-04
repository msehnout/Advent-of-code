extern crate regex;

use regex::Regex;
use std::io::{self, Read};
use std::collections::btree_map::BTreeMap;
use std::str::FromStr;

fn parse(line: &str) -> (&str, u32, &str) {
    let re = Regex::new(r"((?:[a-z]+-)+)([0-9]+)\[([a-z]{5})\]").unwrap();
    let cap = re.captures(line).unwrap();
    (cap.at(1).unwrap(), u32::from_str(cap.at(2).unwrap()).unwrap(), cap.at(3).unwrap())
}

fn checksum(code: &str) -> String {
    let mut list = String::from(code)
        .chars()
        .filter(|&c| c != '-')
        .collect::<Vec<_>>();
    let mut count = BTreeMap::new();
    list.sort();
    let _ = list.into_iter()
        .map(|c|{
            *count.entry(c).or_insert(0) += 1;
        }).collect::<Vec<_>>();
    let mut list_char_num = count
        .into_iter()
        .collect::<Vec<(char, u32)>>();
    list_char_num.sort_by(|a, b| {
        if b.1 != a.1 {
            b.1.cmp(&a.1)
        } else {
            //b.0.cmp(&a.0)
            a.0.cmp(&b.0)
        }
    });
    let mut ret = String::new();
    for i in 0..5 {
        ret.push(list_char_num[i].0);
    }
    ret
}

fn shift_cipher(line: &mut str, id: u32) -> Option<String> {
    let alphabet_lower: &str = "abcdefghijklmnopqrstuvwxyz";
    let mut res = String::new();
    for c in line.chars() {
        if c == '-' {
            res.push(' ');
        } else {
            let pos: usize = alphabet_lower.chars().position(|b| c == b).unwrap();
            let new_pos: usize = (pos + id as usize) % 26;
            res.push(alphabet_lower.clone().chars().nth(new_pos).unwrap());
        }
    }
    if res.contains("north") {
        Some(res)
    } else {
        None
    }
}

fn process(line: &str) -> u32 {
    let (code, id, sum) = parse(line);
    let mut shift = String::from(code);
    let local_sum = checksum(code);
    if local_sum == sum {
        match shift_cipher(&mut shift, id) {
            Some(s) => println!("{} - {}", s, id),
            None => {}
        }
        return id;
    } else {
        return 0;
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let sum: u32 = buffer.lines().map(process).sum();
    println!("Sum is: {}", sum);
}

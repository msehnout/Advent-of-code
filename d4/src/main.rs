extern crate regex;

use regex::Regex;
use std::io::{self, Read};
use std::cmp::Ordering;
use std::collections::btree_map::BTreeMap;
use std::str::FromStr;

struct CharNum(char, u32);

impl PartialEq for CharNum {
    fn eq(&self, other: &CharNum) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for CharNum {}

impl PartialOrd for CharNum {
    fn partial_cmp(&self, other: &CharNum) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CharNum {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.1 != other.1 {
            self.1.cmp(&other.1)
        } else {
            //self.0.cmp(&other.0)
            other.0.cmp(&self.0)
        }
    }
}

fn shift_cipher(line: &mut str, id: u32) {
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
    println!("{} - {}", res, id);
}

fn process(line: &str) -> u32 {
    let re = Regex::new(r"((?:[a-z]+-)+)([0-9]+)\[([a-z]{5})\]").unwrap();
    let cap = re.captures(line).unwrap();
    let name = cap.at(1).unwrap();
    let mut shift = String::from(name);
    let id = cap.at(2).unwrap();
    let checksum = cap.at(3).unwrap();
    //println!("{} - {} - {}", name, id, checksum);
    let mut list: Vec<char> = String::from(name).chars().filter(|&c| c != '-').collect();
    list.sort();
    //println!("{:?} - {} - {}", list, id, checksum);
    let mut count = BTreeMap::new();
    for c in list {
        *count.entry(c).or_insert(0) += 1;
    }
    let mut list2 = Vec::new();
    for (char, count) in &count {
        list2.push(CharNum(*char, *count as u32));
    }
    // reverse sorting
    list2.sort_by(|a, b| b.cmp(a));
    let mut check = String::new();
    for i in 0..5 {
        check.push(list2[i].0);
    }
    //println!("{} - {} - {}", check, id, checksum);
    if check == checksum {
        let id = u32::from_str(id).unwrap();
        shift_cipher(&mut shift, id);
        return id;
    } else {
        return 0;
    }
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    // for line in buffer.lines() {
    //     println!("{}, {}", line, process(line));
    // }
    let sum: u32 = buffer.lines().map(process).sum();
    println!("Sum is: {}", sum);

    assert_eq!(CharNum('a', 5).cmp(&CharNum('a',6)), Ordering::Less);
    assert_eq!(CharNum('x', 7).cmp(&CharNum('a',4)), Ordering::Greater);
}

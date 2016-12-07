extern crate onig;

use onig::Regex;

use std::io::{self, Read};

fn tls_support(input: &str) -> usize {
    let hypernet = Regex::new(r#"\[\w*(\w)(?!\1)(\w)\2\1"#).unwrap();
    let abba = Regex::new(r#"(\w)(?!\1)(\w)\2\1"#).unwrap();
    input.lines().filter(|ipv7| {
        // println!("{} - h: {:?} - a: {:?}", ipv7, !hypernet.is_match(ipv7), abba.is_match(ipv7));
        // !hypernet.is_match(ipv7) && abba.is_match(ipv7)
        // println!("{} - h: {:?} - a: {:?}", ipv7, hypernet.find(ipv7).is_none(), abba.find(ipv7).is_some());
        hypernet.find(ipv7).is_none() && abba.find(ipv7).is_some()
    }).count()
}

fn ssl_support(input: &str) -> usize {
    let ssl = Regex::new(r#"(?:^|\])\w*(\w)(?!\1)(\w)\1.*\[\w*\2\1\2"#).unwrap();
    let ssl2 = Regex::new(r#"\[\w*(\w)(?!\1)(\w)\1.*\]\w*\2\1\2"#).unwrap();
    input.lines().filter(|ipv7| {
        ssl.find(ipv7).is_some() || ssl2.find(ipv7).is_some()
    }).count()
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    println!("Number of IPv7 addresses with TLS support: {}", tls_support(&buffer));
    println!("Number of IPv7 addresses with SSL support: {}", ssl_support(&buffer));
}

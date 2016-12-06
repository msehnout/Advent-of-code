use std::io::{self, Read};
use std::collections::btree_map::BTreeMap;

fn get_message(input: &[&str]) -> String {
    let msg_size = input[0].len();
    let mut counter = (0..msg_size).map(|_| BTreeMap::new()).collect::<Vec<_>>();
    for line in input {
        for (num, c) in line.chars().enumerate() {
            *counter[num].entry(c).or_insert(0) += 1;
        }
    }
    let mut msg = String::new();
    for i in 0..msg_size {
        let list_char_num = (&counter[i])
            .into_iter()
            .collect::<Vec<(&char, &usize)>>();
        let (c, _): (&char, &usize) = *list_char_num.iter().min_by_key(|x| x.1).unwrap();
        msg.push(*c);
    }
    msg
}

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let input = buffer.lines().collect::<Vec<&str>>();
    println!("Message: {}", get_message(&input));
}

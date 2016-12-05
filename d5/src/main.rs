extern crate md5;

fn print_hex(hash: &[u8]) -> Vec<u8> {
    let mut buffer = Vec::new();
    for n in hash {
        buffer.extend_from_slice(&format!("{:02x}", n).as_bytes());
    }
    buffer
}

fn get_passwd(door_id: &str) -> String {
    let mut passwd: Vec<Option<u8>> = vec![None; 8];//Vec::with_capacity(8); //new();
    let mut iter: u64 = 0; //3231920;//0;
    let mut filled_in = 0;
    while filled_in < 8 {
        let mut builder = String::from(door_id);
        builder.push_str(&iter.to_string());
        //println!("{}", builder);
        let hash = md5::compute(builder.as_bytes());
        let hash_str = String::from_utf8(print_hex(&hash)).unwrap();
        //println!("{:?} - {:?} - {:?}", &[0,0,0,0,0],  &hash[0..5], String::from_utf8(print_hex(&hash)).unwrap());
        if hash_str.starts_with("00000") {
            // PART1: passwd.push(hash_str.as_bytes()[5]);//.clone().chars().nth(5).unwrap());
            // PART1: println!("Passwd progress: {}", String::from_utf8(passwd.clone()).unwrap());
            let pos = hash_str.as_bytes()[5];
            if pos > 47 && pos < 56 {
                let pos = pos - 48;
                if passwd[pos as usize].is_none() {
                    passwd[pos as usize] = Some(hash_str.as_bytes()[6]);
                    filled_in += 1;
                    println!("Passwd progress: {}", String::from_utf8(passwd
                        .clone()
                        .into_iter()
                        .map(|c| {
                            match c {
                                Some(c) => c,
                                None => 45,
                            }
                        })
                        .collect::<Vec<_>>()
                    ).unwrap());
                }
            }
        }
        iter += 1;
    }
    String::from_utf8(passwd
        .into_iter()
        .map(Option::unwrap)
        .collect::<Vec<_>>()
    ).unwrap()
}

fn main() {
    // TODO read as arg
    let input = "ugkcyxxp";
    //let input = "abc";
    println!("Input: {}", input);
    let passwd = get_passwd(&input);
    println!("Passwd: {}", passwd);
}

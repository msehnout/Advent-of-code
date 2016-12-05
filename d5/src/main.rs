extern crate md5;

fn print_hex(hash: &[u8]) -> Vec<u8> {
    let mut buffer = Vec::new();
    for n in hash {
        buffer.extend_from_slice(&format!("{:02x}", n).as_bytes());
    }
    buffer
}

fn get_passwd(door_id: &str) -> String {
    let mut passwd: Vec<u8> = Vec::new();
    let mut iter: u64 = 0; //3231920;//0;
    while passwd.len() < 8 {
        let mut builder = String::from(door_id);
        builder.push_str(&iter.to_string());
        //println!("{}", builder);
        let hash = md5::compute(builder.as_bytes());
        let hash_str = String::from_utf8(print_hex(&hash)).unwrap();
        //println!("{:?} - {:?} - {:?}", &[0,0,0,0,0],  &hash[0..5], String::from_utf8(print_hex(&hash)).unwrap());
        if hash_str.starts_with("00000") {
            passwd.push(hash_str.as_bytes()[5]);//.clone().chars().nth(5).unwrap());
            println!("Passwd progress: {}", String::from_utf8(passwd.clone()).unwrap());

        }
        // if iter > 3231940 {
        //     break;
        // }
        //println!("Passwd: {:?}", passwd);
        iter += 1;
    }
    String::from_utf8(passwd).unwrap()
}

fn main() {
    // TODO read as arg
    let input = "ugkcyxxp";
    println!("Input: {}", input);
    let passwd = get_passwd(&input);
    println!("Passwd: {}", passwd);
}

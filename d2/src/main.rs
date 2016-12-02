use std::env;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

/// 1,2,3
/// 4,5,6
/// 7,8,9
///
/// (0,0) .. (2,0)
///       .
/// (0,2)  . (2,2)
struct Keyboard {
    x: u8,
    y: u8,
}

impl Default for Keyboard {
    fn default() -> Keyboard {
        Keyboard {
            x: 1,
            y: 1,
        }
    }
}

impl Keyboard {
    fn up(&mut self) {
        if self.y > 0 {
            self.y -= 1;
        }
    }

    fn down(&mut self) {
        if self.y < 2 {
            self.y += 1;
        }
    }

    fn right(&mut self) {
        if self.x < 2 {
            self.x += 1;
        }
    }

    fn left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    fn get_num(&self) -> u8 {
        match (self.y,self.x) {
            (0,0) => 1,
            (0,1) => 2,
            (0,2) => 3,
            (1,0) => 4,
            (1,1) => 5,
            (1,2) => 6,
            (2,0) => 7,
            (2,1) => 8,
            (2,2) => 9,
            _ => 0,
        }
    }

}

///     1
///   2 3 4
/// 5 6 7 8 9
///   A B C
///     D
struct Keyboard2 {
    val: char,
}

impl Default for Keyboard2 {
    fn default() -> Keyboard2 {
        Keyboard2 {
            val: '5',
        }
    }
}

impl Keyboard2 {
    fn up(&mut self) {
        self.val = match self.val {
            '1' => '1',
            '2' => '2',
            '3' => '1',
            '4' => '4',
            '5' => '5',
            '6' => '2',
            '7' => '3',
            '8' => '4',
            '9' => '9',
            'A' => '6',
            'B' => '7',
            'C' => '8',
            'D' => 'B',
            _ => 'X',
        }
    }

    fn down(&mut self) {
        self.val = match self.val {
            '1' => '3',
            '2' => '6',
            '3' => '7',
            '4' => '8',
            '5' => '5',
            '6' => 'A',
            '7' => 'B',
            '8' => 'C',
            '9' => '9',
            'A' => 'A',
            'B' => 'D',
            'C' => 'C',
            'D' => 'D',
            _ => 'X',
        }
    }

    fn right(&mut self) {
        self.val = match self.val {
            '1' => '1',
            '2' => '3',
            '3' => '4',
            '4' => '4',
            '5' => '6',
            '6' => '7',
            '7' => '8',
            '8' => '9',
            '9' => '9',
            'A' => 'B',
            'B' => 'C',
            'C' => 'C',
            'D' => 'D',
            _ => 'X',
        }
    }

    fn left(&mut self) {
        self.val = match self.val {
            '1' => '1',
            '2' => '2',
            '3' => '2',
            '4' => '3',
            '5' => '5',
            '6' => '5',
            '7' => '6',
            '8' => '7',
            '9' => '8',
            'A' => 'A',
            'B' => 'A',
            'C' => 'B',
            'D' => 'D',
            _ => 'X',
        }
    }

    fn get_num(&self) -> char {
        self.val
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
    let file = BufReader::new(&f);
    let mut key = Keyboard2::default();
    let mut ret = Vec::new();
    for line in file.lines() {
        for c in line.unwrap().chars() {
            match c {
                'R' => key.right(),
                'L' => key.left(),
                'U' => key.up(),
                'D' => key.down(),
                _ => println!("Weird input..")
            }
        }
        ret.push(key.get_num());
    }
    print!("Result: ");
    for n in ret {
        print!("{}", n);
    }
    println!("");
}

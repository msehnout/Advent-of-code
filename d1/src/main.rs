use std::io;

enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_left(&mut self) {
        match *self {
            Direction::North => *self = Direction::West,
            Direction::South => *self = Direction::East,
            Direction::East => *self = Direction::North,
            Direction::West => *self = Direction::South,
        }
    }

    fn turn_right(&mut self) {
        match *self {
            Direction::North => *self = Direction::East,
            Direction::South => *self = Direction::West,
            Direction::East => *self = Direction::South,
            Direction::West => *self = Direction::North,
        }
    }
}
#[derive(PartialEq, PartialOrd, Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

struct PointWithHist {
    p: Point,
    hist: Vec<Point>,
}

impl PointWithHist {
    fn go(&mut self, dir: &Direction, len: i32) -> Option<Point> {
        for _ in 0..len {
            match *dir {
                Direction::North => self.p.y += 1,
                Direction::South => self.p.y -= 1,
                Direction::East => self.p.x += 1,
                Direction::West => self.p.x -= 1,
            }
            if self.hist.contains(&self.p) {
                return Some(self.p);
            }
            self.hist.push(self.p);
        }
        None
    }
}

fn main() {
    // Read input file through stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Ouch, sth went wrong!");
    // Strip new line
    let new_len = input.trim().len();
    input.truncate(new_len);
    // Debug output
    println!("Input line:\n{}", input);
    // Split into vector of coordinates
    let input: Vec<&str> = input.split(", ").collect();
    let mut dir = Direction::North;
    let mut p = PointWithHist { 
        p: Point {
            x: 0,
            y: 0,
        },
        hist: Vec::new(),
    };
    // Ugly hack to push point (0,0) into the vec
    p.hist.push(p.p);
    for s in input.into_iter() {
        if s.chars().next() == Some('R') {
            dir.turn_right();
        } else {
            dir.turn_left();
        }
        let len: i32 = s.trim_left_matches(char::is_alphabetic).parse().unwrap();
        if let Some(x) = p.go(&dir, len) {
            println!("First match is x: {}, y: {}", x.x, x.y);
            return;
        }
    }
    let res = p.p.x.abs() + p.p.y.abs();
    println!("Result: {} blocks away", res);
    println!("Path: {:?}", p.hist);
}

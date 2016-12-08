extern crate regex;

use std::io::{self, Read};
use std::collections::BTreeSet;
use std::str::FromStr;

use regex::Regex;

type AocScreen = BTreeSet<(u32,u32)>;

const SCR_WIDTH: u32 = 50;
const SCR_HEIGHT: u32 = 6;

fn create_rect(screen: &mut AocScreen, width: u32, height: u32) {
    for y in 0..height {
        for x in 0..width {
            screen.insert((y,x));
        }
    }
}

fn rotate_col(screen: &mut AocScreen, col: u32, num: u32) {
    *screen = screen.iter()
        .map(|&(y,x)|{
            if col == x {
                ((y+num)%SCR_HEIGHT, x)
            } else {
                (y,x)
            }
        })
        .collect();
}

fn rotate_row(screen: &mut AocScreen, row: u32, num: u32) {
    *screen = screen.iter()
        .map(|&(y,x)|{
            if row == y {
                (y, (x+num)%SCR_WIDTH)
            } else {
                (y,x)
            }
        })
        .collect();
}

fn print_screen(screen: &AocScreen) {
    for y in 0..SCR_HEIGHT {
        for x in 0..SCR_WIDTH {
            if screen.contains(&(y,x)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!("");
    }
}

fn main() {
    let mut scr = AocScreen::new();
    let rect = Regex::new(r"^rect (\d+)x(\d+)$").unwrap();
    let column = Regex::new(r"^rotate column x=(\d+) by (\d+)$").unwrap();
    let row = Regex::new(r"^rotate row y=(\d+) by (\d+)$").unwrap();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    for line in buffer.lines() {
        if rect.is_match(line) {
            let caps = rect.captures(line).unwrap();
            let width = u32::from_str(caps.at(1).unwrap()).unwrap();
            let height = u32::from_str(caps.at(2).unwrap()).unwrap();
            create_rect(&mut scr, width, height);
        } else if column.is_match(line) {
            let caps = column.captures(line).unwrap();
            let col = u32::from_str(caps.at(1).unwrap()).unwrap();
            let num = u32::from_str(caps.at(2).unwrap()).unwrap();
            rotate_col(&mut scr, col, num);
        } else if row.is_match(line) {
            let caps = row.captures(line).unwrap();
            let row = u32::from_str(caps.at(1).unwrap()).unwrap();
            let num = u32::from_str(caps.at(2).unwrap()).unwrap();
            rotate_row(&mut scr, row, num);
        }
    }
    print_screen(&scr);
    println!("Set size: {}", scr.len());
}

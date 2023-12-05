use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use regex::Regex;

pub fn _day4() {
    fn parse<T: FromIterator<i32>>(numbers: &str) -> T {
        numbers
            .trim()
            .split(' ')
            .filter_map(|x| {
                if x.is_empty() {
                    None
                } else {
                    x.parse::<i32>().ok()
                }
            })
            .collect()
    }
    let contents = BufReader::new(File::open("src/input4.txt").unwrap());
    let parser = Regex::new(r".+:(.+)\|(.+)").unwrap();
    let mut sum = 0;
    for line in contents.lines() {
        if let Some(c) = parser.captures(line.unwrap().as_str()) {
            let winning_numbers: HashSet<i32> = parse(c.get(1).unwrap().as_str());
            let my_number: Vec<i32> = parse(c.get(2).unwrap().as_str());
            let mut point = 0;
            for n in my_number {
                if winning_numbers.contains(&n) {
                    if point == 0 {
                        point = 1;
                    } else {
                        point *= 2;
                    }
                }
            }
            sum += point;
        } else {
            panic!("illegal format");
        }
    }
    println!("day 4 part 1: {}", sum);
}

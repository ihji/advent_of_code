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
    let mut win_numbers = Vec::new();
    for line in contents.lines() {
        if let Some(c) = parser.captures(line.unwrap().as_str()) {
            let winning_numbers: HashSet<i32> = parse(c.get(1).unwrap().as_str());
            let my_number: Vec<i32> = parse(c.get(2).unwrap().as_str());
            let mut point = 0;
            let mut win = 0;
            for n in my_number {
                if winning_numbers.contains(&n) {
                    if point == 0 {
                        point = 1;
                    } else {
                        point *= 2;
                    }
                    win += 1;
                }
            }
            sum += point;
            win_numbers.push(win);
        } else {
            panic!("illegal format");
        }
    }
    println!("day 4 part 1: {}", sum);

    let mut rewards = vec![0; win_numbers.len()];
    for i in (0..rewards.len()).rev() {
        let total: i32 = (&rewards[i..=i + win_numbers[i]]).iter().sum();
        rewards[i] = total + 1;
    }
    let rewards_sum: i32 = rewards.iter().sum();
    println!("day 4 part 2: {}", rewards_sum);
}

use std::{collections::HashMap, fs::read_to_string};

use regex::Regex;

pub fn _day8() {
    let contents = read_to_string("src/input8.txt").unwrap();
    let regex = Regex::new(r"([A-Z]{3}) = \(([A-Z]{3}), ([A-Z]{3})\)").unwrap();
    let cmd: Vec<char> = contents.lines().next().unwrap().chars().collect();
    let map: HashMap<&str, (&str, &str)> = contents
        .lines()
        .skip(2)
        .map(|x| {
            let (_, [src, left, right]) = regex.captures(x).unwrap().extract();
            (src, (left, right))
        })
        .collect();
    let mut ptr = 0;
    let mut cur = "AAA";
    while cur != "ZZZ" {
        let (left, right) = map.get(cur).unwrap();
        cur = match cmd[ptr % cmd.len()] {
            'L' => left,
            'R' => right,
            _ => panic!("impossible"),
        };
        ptr += 1;
    }
    println!("day 8 part 1: {}", ptr);

    let cur: Vec<&str> = map
        .clone()
        .into_keys()
        .filter(|x| x.ends_with("A"))
        .collect();
    let mut ptrs = vec![];
    for mut c in cur {
        ptr = 0;
        while !c.ends_with("Z") {
            let (left, right) = map.get(c).unwrap();
            c = match cmd[ptr % cmd.len()] {
                'L' => left,
                'R' => right,
                _ => panic!("impossible"),
            };
            ptr += 1;
        }
        ptrs.push(ptr);
    }
    let lcm = ptrs.into_iter().reduce(num::integer::lcm).unwrap();
    println!("day 8 part 2: {}", lcm);
}

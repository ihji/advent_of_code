use std::fs::read_to_string;

use regex::Regex;

#[derive(Debug)]
struct Mapping {
    src: i64,
    dst: i64,
    len: i64,
}

#[derive(Debug)]
struct Map {
    mapping: Vec<Mapping>,
}

impl Map {
    fn new(mapping: Vec<Mapping>) -> Self {
        Self { mapping }
    }
    fn get(&self, src: i64) -> i64 {
        for m in self.mapping.iter() {
            if src >= m.src && src < m.src + m.len {
                return m.dst + src - m.src;
            }
        }
        return src;
    }
}

pub fn _day5() {
    let contents = read_to_string("src/input5.txt").unwrap();
    let regex = Regex::new(
        r"seeds:((\d|\s)+)
seed-to-soil map:((\d|\s)+)
soil-to-fertilizer map:((\d|\s)+)
fertilizer-to-water map:((\d|\s)+)
water-to-light map:((\d|\s)+)
light-to-temperature map:((\d|\s)+)
temperature-to-humidity map:((\d|\s)+)
humidity-to-location map:((\d|\s)+)",
    )
    .unwrap();
    let capture = regex.captures(&contents).unwrap();
    let maps: Vec<Map> = vec![3, 5, 7, 9, 11, 13, 15]
        .iter()
        .map(|x| {
            Map::new(
                capture
                    .get(*x)
                    .unwrap()
                    .as_str()
                    .trim()
                    .split('\n')
                    .map(|y| {
                        let split: Vec<&str> = y.split(' ').collect();
                        Mapping {
                            src: split[1].parse().unwrap(),
                            dst: split[0].parse().unwrap(),
                            len: split[2].parse().unwrap(),
                        }
                    })
                    .collect(),
            )
        })
        .collect();
    let seeds: Vec<i64> = capture
        .get(1)
        .unwrap()
        .as_str()
        .trim()
        .split(' ')
        .map(|x| x.parse().unwrap())
        .collect();
    let mut min_location = i64::MAX;
    for seed in seeds {
        let mut location = seed;
        for m in maps.iter() {
            location = m.get(location);
        }
        min_location = min_location.min(location);
    }
    println!("day 5 part 1: {}", min_location);
}

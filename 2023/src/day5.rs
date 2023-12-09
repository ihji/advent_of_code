use std::fs::read_to_string;

use regex::Regex;

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
struct Interval {
    from: i64,
    to: i64,
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
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
    fn new(mut mapping: Vec<Mapping>) -> Self {
        mapping.sort();
        Self { mapping }
    }
    fn map(src: i64, m: &Mapping) -> i64 {
        if src >= m.src && src < m.src + m.len {
            return m.dst + src - m.src;
        } else {
            return src;
        }
    }
    fn get(&self, mut src: Interval) -> Vec<Interval> {
        let mut output = vec![];
        for m in self.mapping.iter() {
            // <m> [src]
            if src.from >= m.src + m.len {
                continue;
            }
            // [ < ] >
            if m.src > src.from && m.src <= src.to {
                let min = src.to.min(m.src - 1);
                output.push(Interval {
                    from: src.from,
                    to: min,
                });
                src.from = min + 1;
            }
            // < [ > ]
            if src.from >= m.src && src.from < m.src + m.len {
                let min = src.to.min(m.src + m.len - 1);
                output.push(Interval {
                    from: Self::map(src.from, m),
                    to: Self::map(min, m),
                });
                src.from = min + 1;
            }
            if src.from > src.to {
                return output;
            }
        }
        if src.from <= src.to {
            output.push(Interval {
                from: src.from,
                to: src.to,
            });
        }
        return output;
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
    let mut location: Vec<Interval> = seeds
        .iter()
        .map(|x| Interval { from: *x, to: *x })
        .collect();
    let mut location2: Vec<Interval> = seeds
        .chunks(2)
        .map(|x| Interval {
            from: x[0],
            to: x[0] + x[1] - 1,
        })
        .collect();
    for m in maps.iter() {
        location = location.into_iter().flat_map(|x| m.get(x)).collect();
        location2 = location2.into_iter().flat_map(|x| m.get(x)).collect();
    }
    location.sort();
    location2.sort();

    println!("day 5 part 1: {}", location[0].from);
    println!("day 5 part 2: {}", location2[0].from);
}

use std::collections::HashMap;
use std::fs::read_to_string;

pub fn _day1() {
    let contents = read_to_string("src/input1.txt").unwrap();

    let mut sum = 0;
    for line in contents.lines() {
        let numbers: Vec<char> = line.chars().filter(|x| x.is_numeric()).collect();
        let target = format!("{}{}", numbers[0], numbers[numbers.len() - 1]);
        sum += target.parse::<i32>().expect("should be a number");
    }
    println!("day 1 part 1: {}", sum);

    let number_map = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    sum = 0;
    for line in contents.lines() {
        let mut numbers = Vec::new();
        for i in 0..line.len() {
            let slice = &line[i..];
            let first = slice.chars().next().unwrap();
            if first.is_numeric() {
                numbers.push(first);
            } else {
                for (k, v) in number_map.clone().into_iter() {
                    if slice.starts_with(k) {
                        numbers.push(char::from_digit(v as u32, 10).expect("should be a digit"));
                        break;
                    }
                }
            }
        }
        let target = format!("{}{}", numbers[0], numbers[numbers.len() - 1]);
        sum += target.parse::<i32>().expect("should be a number");
    }
    println!("day 1 part 2: {}", sum);
}

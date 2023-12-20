use std::fs::read_to_string;

use num::Zero;

pub fn _day9() {
    let contents = read_to_string("src/input9.txt").unwrap();
    let inputs: Vec<Vec<i32>> = contents
        .lines()
        .map(|x| x.split(" ").map(|x| x.parse().unwrap()).collect())
        .collect();
    let mut first_sum = 0;
    let mut last_sum = 0;
    for mut input in inputs {
        let mut firsts = vec![];
        let mut lasts = vec![];
        while input.iter().any(|x| !x.is_zero()) {
            firsts.push(*input.first().unwrap());
            lasts.push(*input.last().unwrap());
            input = input.windows(2).map(|x| x[1] - x[0]).collect();
        }
        last_sum += lasts.iter().sum::<i32>();
        first_sum += firsts.into_iter().rev().reduce(|acc, e| e - acc).unwrap();
    }
    println!("day 9 part 1: {}", last_sum);
    println!("day 9 part 2: {}", first_sum);
}

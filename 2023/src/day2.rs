use regex::Regex;
use std::fs::read_to_string;

pub fn day2() -> Option<()> {
    fn get_numbers(line: &str) -> (i32, i32, i32) {
        let red = Regex::new(r"(\d+) red").unwrap();
        let green = Regex::new(r"(\d+) green").unwrap();
        let blue = Regex::new(r"(\d+) blue").unwrap();
        let r = red
            .captures(line)
            .map_or("0", |x| x.get(1).unwrap().as_str())
            .parse::<i32>()
            .expect("should be a number");
        let g = green
            .captures(line)
            .map_or("0", |x| x.get(1).unwrap().as_str())
            .parse::<i32>()
            .expect("should be a number");
        let b = blue
            .captures(line)
            .map_or("0", |x| x.get(1).unwrap().as_str())
            .parse::<i32>()
            .expect("should be a number");
        (r, g, b)
    }

    // 12 red, 13 green, 14 blue
    let game = Regex::new(r"Game (\d+):").unwrap();
    let contents = read_to_string("src/input2.txt").unwrap();
    let mut sum = 0;
    for line in contents.lines() {
        let n = game
            .captures(line)?
            .get(1)?
            .as_str()
            .parse::<i32>()
            .expect("should be a number");
        sum += n;
        for part in line.split(";") {
            let (r, g, b) = get_numbers(part);
            if r > 12 || g > 13 || b > 14 {
                sum -= n;
                break;
            }
        }
    }
    println!("day 2 part 1: {}", sum);

    sum = 0;
    for line in contents.lines() {
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for part in line.split(";") {
            let (r, g, b) = get_numbers(part);
            max_red = max_red.max(r);
            max_green = max_green.max(g);
            max_blue = max_blue.max(b);
        }
        sum += max_red * max_green * max_blue;
    }
    println!("day 2 part 2: {}", sum);

    Some(())
}

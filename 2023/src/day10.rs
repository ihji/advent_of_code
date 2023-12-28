use std::fs::read_to_string;

pub fn _day10() {
    fn is_valid(map: &Vec<Vec<char>>, (x, y): &(i32, i32)) -> bool {
        *x >= 0 && *x < map.len() as i32 && *y >= 0 && *y < map[0].len() as i32
    }
    fn next(map: &Vec<Vec<char>>, (x, y): (i32, i32)) -> Vec<(i32, i32)> {
        let c = map[x as usize][y as usize];
        match c {
            '.' => vec![],
            '|' => vec![(x - 1, y), (x + 1, y)],
            '-' => vec![(x, y - 1), (x, y + 1)],
            'L' => vec![(x - 1, y), (x, y + 1)],
            'J' => vec![(x - 1, y), (x, y - 1)],
            '7' => vec![(x, y - 1), (x + 1, y)],
            'F' => vec![(x, y + 1), (x + 1, y)],
            'S' => vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)],
            _ => panic!("unknown symbol"),
        }
        .into_iter()
        .filter(|x| is_valid(map, x))
        .collect()
    }
    let contents = read_to_string("src/input10.txt").unwrap();
    let map: Vec<Vec<char>> = contents.lines().map(|x| x.chars().collect()).collect();

    let mut cur = (-1, -1);
    for (i, v) in map.iter().enumerate() {
        for (j, c) in v.iter().enumerate() {
            if *c == 'S' {
                cur = (i as i32, j as i32);
            }
        }
    }
    let mut step = 0;
    let start = cur;
    let mut prev = cur;
    loop {
        let n = next(&map, cur)
            .into_iter()
            .filter(|x| *x != prev)
            .next()
            .unwrap();
        prev = cur;
        cur = n;
        step += 1;
        if cur == start {
            break;
        }
    }
    println!("day 10 part 1: {}", step / 2);
}

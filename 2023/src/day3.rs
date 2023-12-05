use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(PartialEq, Eq, Hash)]
enum Sym {
    Star { x: usize, y: usize },
    Other,
    None,
}

impl Sym {
    fn is_symbol(&self) -> bool {
        match self {
            Sym::Star { x: _, y: _ } => true,
            Sym::Other => true,
            Sym::None => false,
        }
    }
}

fn in_range(mat: &Vec<Vec<char>>, (i, j): (i32, i32)) -> bool {
    i >= 0 && i < mat.len() as i32 && j >= 0 && j < mat[0].len() as i32
}

fn is_symbol(mat: &Vec<Vec<char>>, (i, j): (i32, i32)) -> Sym {
    if in_range(mat, (i, j)) {
        match mat[i as usize][j as usize] {
            x if x.is_numeric() || x == '.' => Sym::None,
            x if x == '*' => Sym::Star {
                x: i as usize,
                y: j as usize,
            },
            _ => Sym::Other,
        }
    } else {
        Sym::None
    }
}

fn get_adj_symbols(mat: &Vec<Vec<char>>, (i, j): (i32, i32)) -> Vec<Sym> {
    vec![
        is_symbol(mat, (i - 1, j)),
        is_symbol(mat, (i + 1, j)),
        is_symbol(mat, (i, j - 1)),
        is_symbol(mat, (i, j + 1)),
        is_symbol(mat, (i - 1, j - 1)),
        is_symbol(mat, (i - 1, j + 1)),
        is_symbol(mat, (i + 1, j - 1)),
        is_symbol(mat, (i + 1, j + 1)),
    ]
}

pub fn _day3() {
    let contents = read_to_string("src/input3.txt").unwrap();
    let mat: Vec<Vec<char>> = contents.lines().map(|x| x.chars().collect()).collect();
    let mut sum = 0;
    let mut stars = HashMap::new();
    for (i, chars) in mat.iter().enumerate() {
        let mut numbers = String::new();
        let mut adj = HashSet::new();
        for (j, c) in chars.iter().enumerate() {
            if c.is_numeric() {
                numbers.push(*c);
                adj.extend(get_adj_symbols(&mat, (i as i32, j as i32)));
            } else if numbers.len() > 0 {
                if adj.iter().any(|x| x.is_symbol()) {
                    let n: i32 = numbers.parse().unwrap();
                    sum += n;
                    adj.iter().for_each(|x| match x {
                        Sym::Star { x, y } => {
                            stars.entry((*x, *y)).or_insert(Vec::new()).push(n);
                        }
                        _ => (),
                    });
                    adj.clear();
                }
                numbers.clear();
            }
        }
        if numbers.len() > 0 {
            if adj.iter().any(|x| x.is_symbol()) {
                let n: i32 = numbers.parse().unwrap();
                sum += n;
                adj.iter().for_each(|x| match x {
                    Sym::Star { x, y } => {
                        stars.entry((*x, *y)).or_insert(Vec::new()).push(n);
                    }
                    _ => (),
                });
                adj.clear();
            }
            numbers.clear();
        }
    }
    println!("day 3 part 1: {}", sum);

    let sum2: i32 = stars
        .values()
        .filter(|x| x.len() == 2)
        .map(|x| x[0] * x[1])
        .sum();
    println!("day 3 part 2: {}", sum2);
}

use std::fs::read_to_string;

trait WithPoints {
    fn get_points(&self) -> i32;
}

mod part1 {
    use std::{collections::HashMap, str::FromStr};
    #[derive(Hash, Debug, Ord, PartialOrd, Eq, PartialEq)]
    enum Card {
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        J,
        Q,
        K,
        A,
    }

    #[derive(Debug)]
    struct ParseCardError;

    impl FromStr for Card {
        type Err = ParseCardError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" => Ok(Card::A),
                "K" => Ok(Card::K),
                "Q" => Ok(Card::Q),
                "J" => Ok(Card::J),
                "T" => Ok(Card::Ten),
                "9" => Ok(Card::Nine),
                "8" => Ok(Card::Eight),
                "7" => Ok(Card::Seven),
                "6" => Ok(Card::Six),
                "5" => Ok(Card::Five),
                "4" => Ok(Card::Four),
                "3" => Ok(Card::Three),
                "2" => Ok(Card::Two),
                _ => Err(ParseCardError),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Ord)]
    pub struct Hand {
        cards: [Card; 5],
        points: i32,
    }

    impl super::WithPoints for Hand {
        fn get_points(&self) -> i32 {
            self.points
        }
    }

    impl Hand {
        pub fn new(cards: &str, points: &str) -> Self {
            Hand {
                cards: cards
                    .chars()
                    .map(|x| x.to_string().parse().unwrap())
                    .collect::<Vec<Card>>()
                    .try_into()
                    .unwrap(),
                points: points.parse().unwrap(),
            }
        }
        fn get_score(&self) -> i32 {
            let mut kinds = HashMap::new();
            for card in self.cards.iter() {
                *kinds.entry(card).or_insert(0) += 1;
            }
            let mut values: Vec<i32> = kinds.into_values().collect();
            values.sort();
            match values[..] {
                [5] => 6,
                [1, 4] => 5,
                [2, 3] => 4,
                [1, 1, 3] => 3,
                [1, 2, 2] => 2,
                [1, 1, 1, 2] => 1,
                _ => 0,
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match self.get_score().partial_cmp(&other.get_score()) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
            self.cards.partial_cmp(&other.cards)
        }
    }
}

mod part2 {
    use std::{collections::HashMap, str::FromStr};
    #[derive(Hash, Debug, Ord, PartialOrd, Eq, PartialEq)]
    enum Card {
        J,
        Two,
        Three,
        Four,
        Five,
        Six,
        Seven,
        Eight,
        Nine,
        Ten,
        Q,
        K,
        A,
    }

    #[derive(Debug)]
    struct ParseCardError;

    impl FromStr for Card {
        type Err = ParseCardError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "A" => Ok(Card::A),
                "K" => Ok(Card::K),
                "Q" => Ok(Card::Q),
                "J" => Ok(Card::J),
                "T" => Ok(Card::Ten),
                "9" => Ok(Card::Nine),
                "8" => Ok(Card::Eight),
                "7" => Ok(Card::Seven),
                "6" => Ok(Card::Six),
                "5" => Ok(Card::Five),
                "4" => Ok(Card::Four),
                "3" => Ok(Card::Three),
                "2" => Ok(Card::Two),
                _ => Err(ParseCardError),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Ord)]
    pub struct Hand {
        cards: [Card; 5],
        points: i32,
    }

    impl super::WithPoints for Hand {
        fn get_points(&self) -> i32 {
            self.points
        }
    }

    impl Hand {
        pub fn new(cards: &str, points: &str) -> Self {
            Hand {
                cards: cards
                    .chars()
                    .map(|x| x.to_string().parse().unwrap())
                    .collect::<Vec<Card>>()
                    .try_into()
                    .unwrap(),
                points: points.parse().unwrap(),
            }
        }
        fn get_score(&self) -> i32 {
            let mut kinds = HashMap::new();
            let mut num_j = 0;
            for card in self.cards.iter() {
                if *card == Card::J {
                    num_j += 1;
                } else {
                    *kinds.entry(card).or_insert(0) += 1;
                }
            }
            let mut values: Vec<i32> = kinds.into_values().collect();
            values.sort();
            if values.len() == 0 {
                values.push(num_j);
            } else {
                *values.last_mut().unwrap() += num_j;
            }
            match values[..] {
                [5] => 6,
                [1, 4] => 5,
                [2, 3] => 4,
                [1, 1, 3] => 3,
                [1, 2, 2] => 2,
                [1, 1, 1, 2] => 1,
                _ => 0,
            }
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match self.get_score().partial_cmp(&other.get_score()) {
                Some(core::cmp::Ordering::Equal) => {}
                ord => return ord,
            }
            self.cards.partial_cmp(&other.cards)
        }
    }
}

pub fn _day7() {
    fn get_sum<T: WithPoints>(hands: Vec<T>) -> i32 {
        hands
            .iter()
            .enumerate()
            .map(|(x, y)| y.get_points() * (x + 1) as i32)
            .sum()
    }
    let contents = read_to_string("src/input7.txt").unwrap();

    let (mut hands1, mut hands2): (Vec<part1::Hand>, Vec<part2::Hand>) = contents
        .lines()
        .map(|line| {
            let token: Vec<&str> = line.split(' ').collect();
            (
                part1::Hand::new(token[0], token[1]),
                part2::Hand::new(token[0], token[1]),
            )
        })
        .unzip();
    hands1.sort();
    hands2.sort();

    println!("day 7 part 1: {}", get_sum(hands1));
    println!("day 7 part 2: {}", get_sum(hands2));
}

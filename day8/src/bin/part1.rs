use std::collections::HashMap;

use regex::Regex;

const PIN: &str = include_str!("../../input.txt");

#[derive(Debug)]
enum Direction {
    L,
    R,
}

impl Direction {
    fn new(c: char) -> Direction {
        match c {
            'L' => Direction::L,
            _ => Direction::R,
        }
    }
}

fn main() {
    let mut result = 0;

    let dre = Regex::new("(L|R)+").unwrap();
    let lre = Regex::new(r"(?<label>\w{3}) = \((?<left>\w{3}), (?<right>\w{3})\)").unwrap();

    let dirs: Vec<Direction> = dre.captures(PIN).unwrap()[0]
        .chars()
        .map(|c| Direction::new(c))
        .collect();
    let labels: HashMap<String, (String, String)> = lre
        .captures_iter(PIN)
        .map(|c| {
            (
                c["label"].to_owned(),
                (c["left"].to_owned(), c["right"].to_owned()),
            )
        })
        .collect();

    let mut curl: &str = "AAA";

    'out: loop {
        for d in dirs.iter() {
            result += 1;

            match d {
                Direction::L => curl = &labels[curl].0,
                Direction::R => curl = &labels[curl].1,
            }

            if curl == "ZZZ" {
                break 'out;
            }
        }
    }

    println!("{result}");
}

use std::{collections::HashMap, borrow::BorrowMut};
use rayon::iter::ParallelIterator;

use rayon::iter::ParallelBridge;
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
    let lre = Regex::new(r"(?<label>(\w|\d){3}) = \((?<left>(\w|\d){3}), (?<right>(\w|\d){3})\)").unwrap();

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

    let mut curl: Vec<String> = labels.keys().filter(|l| l.ends_with('A')).map(|l| l.to_string()).collect();

    'out: loop {
        for d in dirs.iter() {
            result += 1;

            match d {
                Direction::L => curl.iter_mut().par_bridge().for_each(|l| *l = labels[l].0.clone()),
                Direction::R => curl.iter_mut().par_bridge().for_each(|l| *l = labels[l].1.clone()),
            };

            if curl.iter().filter(|l| l.ends_with('Z')).count() == curl.len() {
                break 'out;
            }
        }
    }

    println!("{result}");
}

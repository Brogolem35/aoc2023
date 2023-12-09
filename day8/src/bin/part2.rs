use num::Integer;
use std::{borrow::BorrowMut, collections::HashMap};

use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;

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
    let dre = Regex::new("(L|R)+").unwrap();
    let lre = Regex::new(r"(?<label>(\w|\d){3}) = \((?<left>(\w|\d){3}), (?<right>(\w|\d){3})\)")
        .unwrap();

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

    let result: i64 = labels
        .keys()
        .filter(|l| l.ends_with('A'))
        .map(|l| l.to_string())
        .map(|l| first_z(&l, &labels, &dirs))
        .reduce(|a, x| a.lcm(&x))
        .unwrap();

    println!("{result}");
}

fn first_z(
    start: &String,
    labels: &HashMap<String, (String, String)>,
    dirs: &Vec<Direction>,
) -> i64 {
    let mut res = 0;
    let mut curl = start;

    loop {
        for d in dirs.iter() {
            res += 1;

            match d {
                Direction::L => curl = &labels[curl].0,
                Direction::R => curl = &labels[curl].1,
            }

            if curl.ends_with('Z') {
                return res;
            }
        }
    }
}

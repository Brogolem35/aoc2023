use regex::Regex;

const PIN: &str = include_str!("../../input.txt");

#[derive(Debug)]
struct History {
    values: Vec<i32>,
}

impl History {
    fn prediction(&self) -> i32 {
        let mut iters: Vec<Vec<i32>> = vec![self.values.clone()];

        while iters.last().unwrap().iter().filter(|x| **x == 0).count() != iters.last().unwrap().len(){
            iters.push(iters.last().unwrap().windows(2).map(|x| x[1] - x[0]).collect());
        }

        iters.iter().map(|v| v.last().unwrap()).sum()
    }
}

fn main() {
    let re = Regex::new(r"-?\d+").unwrap();

    let vals = PIN.lines().map(|l| History {
        values: re
            .captures_iter(l)
            .map(|c| c[0].parse::<i32>().unwrap())
            .collect(),
    });

    let result: i32 = vals.map(|h| h.prediction()).sum();

    println!("{result}");
}

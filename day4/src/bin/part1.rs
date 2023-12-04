use regex::Regex;

const PIN: &str = include_str!("../../input.txt");

fn main() {
    let mut total = 0;

    let numre = Regex::new(r"\d+").unwrap();
    let tre = Regex::new(r"Card +\d+:").unwrap();
    let pin = tre.replace_all(PIN, "");
    let lines = pin.lines();

    for l in lines {
        let split: Vec<&str> = l.split("|").collect();
        let lnums: Vec<i32> = numre
            .captures_iter(split[0])
            .map(|n| n[0].parse::<i32>().unwrap())
            .collect();
        let rnums = numre
            .captures_iter(split[1])
            .map(|n| n[0].parse::<i32>().unwrap());
        let res = rnums.filter(|n| lnums.contains(n)).collect::<Vec<i32>>();
        let reslen = res.len();

        total += if reslen == 0 {
            0
        } else {
            2i32.pow((reslen - 1) as u32)
        };
    }

    println!("{total}")
}

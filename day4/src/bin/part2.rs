use regex::Regex;

const PIN: &str = include_str!("../../input.txt");

fn main() {
    let mut total = 0;

    let numre = Regex::new(r"\d+").unwrap();
    let tre = Regex::new(r"Card +\d+:").unwrap();
    let pin = tre.replace_all(PIN, "");
    let lines: Vec<&str> = pin.lines().collect();

    let mut cards: Vec<i32> = vec![0; lines.len()];

    for (i, l) in lines.iter().enumerate() {
        let val = cards.get(i).unwrap_or(&0) + 1;

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

        if reslen != 0 {
            cards[(i + 1)..=(i + reslen)]
                .iter_mut()
                .for_each(|x| *x += 1 * val);
        }

        total += val;
    }

    println!("{total}")
}

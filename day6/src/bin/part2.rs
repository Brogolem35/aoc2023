use regex::Regex;

const PIN: &str = include_str!("../../input.txt");

fn main() {
    let pin = PIN.replace(' ', "");
    let re = Regex::new(r"\d+").unwrap();
    let lines: Vec<&str> = pin.lines().collect();
    
    let time = re.find_iter(lines[0]).map(|s| s.as_str().parse::<i64>().unwrap());
    let dist = re.find_iter(lines[1]).map(|s| s.as_str().parse::<i64>().unwrap());
    let td = time.zip(dist);

    let time_range = td.map(|(t, d)| (1..=(t-1), d, t));

    let result = time_range.map(|(t, d, tm)| t.filter(move |t| t * (tm - t) > d).count()).reduce(|a, b| a*b).unwrap();

    println!("{result}");
}

use regex::Regex;
use rayon::prelude::*;

const PIN: &str = include_str!("../../input.txt");

#[derive(Debug)]
struct Map {
    dest: i64,
    src: i64,
    range: i64,
}

impl Map {
    fn seed_dest(&self, seed: i64) -> Option<i64> {
        if seed >= self.src && seed <= (self.src + self.range - 1) {
            Some(seed + (self.dest - self.src))
        } else {
            None
        }
    }
}

fn main() {
    let sre = Regex::new(r"seeds: ((\d+ ?)+)").unwrap();
    let duore = Regex::new(r"\d+ \d+").unwrap();

    let smre = Regex::new(r"seed-to-soil map:(\n(\d+ \d+ \d+))*").unwrap();
    let fmre = Regex::new(r"soil-to-fertilizer map:(\n(\d+ \d+ \d+))*").unwrap();
    let wmre = Regex::new(r"fertilizer-to-water map:(\n(\d+ \d+ \d+))*").unwrap();
    let lmre = Regex::new(r"water-to-light map:(\n(\d+ \d+ \d+))*").unwrap();
    let tmre = Regex::new(r"light-to-temperature map:(\n(\d+ \d+ \d+))*").unwrap();
    let hmre = Regex::new(r"temperature-to-humidity map:(\n(\d+ \d+ \d+))*").unwrap();
    let lomre = Regex::new(r"humidity-to-location map:(\n(\d+ \d+ \d+))*").unwrap();

    let seednumstr = &sre.captures(PIN).unwrap()[1];
    let seedls = duore.captures_iter(seednumstr).map(|s| {
        let s = s[0]
            .split(' ')
            .map(|x| x.parse::<i64>().unwrap())
            .collect::<Vec<i64>>();
        s[0]..(s[0] + s[1])
    });

    let smap: Vec<Map> = smre.captures(PIN).unwrap()[0]
        .lines()
        .skip(1)
        .map(|s| {
            let s: Vec<i64> = s.split(' ').map(|s| s.parse::<i64>().unwrap()).collect();
            Map {
                dest: s[0],
                src: s[1],
                range: s[2],
            }
        })
        .collect();

    let fmap: Vec<Map> = fmre.captures(PIN).unwrap()[0]
        .lines()
        .skip(1)
        .map(|s| {
            let s: Vec<i64> = s.split(' ').map(|s| s.parse::<i64>().unwrap()).collect();
            Map {
                dest: s[0],
                src: s[1],
                range: s[2],
            }
        })
        .collect();

    let wmap: Vec<Map> = wmre.captures(PIN).unwrap()[0]
        .lines()
        .skip(1)
        .map(|s| {
            let s: Vec<i64> = s.split(' ').map(|s| s.parse::<i64>().unwrap()).collect();
            Map {
                dest: s[0],
                src: s[1],
                range: s[2],
            }
        })
        .collect();

    let lmap: Vec<Map> = lmre.captures(PIN).unwrap()[0]
        .lines()
        .skip(1)
        .map(|s| {
            let s: Vec<i64> = s.split(' ').map(|s| s.parse::<i64>().unwrap()).collect();
            Map {
                dest: s[0],
                src: s[1],
                range: s[2],
            }
        })
        .collect();

    let tmap: Vec<Map> = tmre.captures(PIN).unwrap()[0]
        .lines()
        .skip(1)
        .map(|s| {
            let s: Vec<i64> = s.split(' ').map(|s| s.parse::<i64>().unwrap()).collect();
            Map {
                dest: s[0],
                src: s[1],
                range: s[2],
            }
        })
        .collect();

    let hmap: Vec<Map> = hmre.captures(PIN).unwrap()[0]
        .lines()
        .skip(1)
        .map(|s| {
            let s: Vec<i64> = s.split(' ').map(|s| s.parse::<i64>().unwrap()).collect();
            Map {
                dest: s[0],
                src: s[1],
                range: s[2],
            }
        })
        .collect();

    let lomap: Vec<Map> = lomre.captures(PIN).unwrap()[0]
        .lines()
        .skip(1)
        .map(|s| {
            let s: Vec<i64> = s.split(' ').map(|s| s.parse::<i64>().unwrap()).collect();
            Map {
                dest: s[0],
                src: s[1],
                range: s[2],
            }
        })
        .collect();

    let result = seedls.par_bridge()
        .map(|seeds| {
            seeds.into_par_iter().map(|seed| {
                let res = map_all(seed, &smap);
                let res = map_all(res, &fmap);
                let res = map_all(res, &wmap);
                let res = map_all(res, &lmap);
                let res = map_all(res, &tmap);
                let res = map_all(res, &hmap);
                let res = map_all(res, &lomap);

                res
            }).min().unwrap()
        })
        .min()
        .unwrap();

    println!("{result}");
}

fn map_all(seed: i64, maps: &Vec<Map>) -> i64 {
    for m in maps {
        if let Some(i) = m.seed_dest(seed) {
            return i;
        }
    }

    seed
}

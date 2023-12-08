use std::collections::HashMap;

use regex::Regex;

const PIN: &str = include_str!("../../input.txt");

#[derive(Debug, PartialEq, Eq, PartialOrd, Hash, Clone, Copy)]
enum CardType {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl CardType {
    fn new(c: char) -> Option<CardType> {
        match c {
            '2' => Some(CardType::Two),
            '3' => Some(CardType::Three),
            '4' => Some(CardType::Four),
            '5' => Some(CardType::Five),
            '6' => Some(CardType::Six),
            '7' => Some(CardType::Seven),
            '8' => Some(CardType::Eight),
            '9' => Some(CardType::Nine),
            'T' => Some(CardType::Ten),
            'J' => Some(CardType::Joker),
            'Q' => Some(CardType::Queen),
            'K' => Some(CardType::King),
            'A' => Some(CardType::Ace),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd)]
enum HandType {
    None,
    High,
    OnePair,
    TwoPair,
    Threek,
    Full,
    FourK,
    FiveK,
}

impl HandType {
    fn new(cards: &Vec<CardType>) -> HandType {
        let mut hm: HashMap<CardType, u32> = HashMap::new();

        for c in cards {
            hm.entry(*c).and_modify(|e| *e += 1).or_insert(1);
        }

        let jcnt = hm.remove(&CardType::Joker).unwrap_or(0);

        if hm.values().filter(|n| **n == 5).count() == 1
            || jcnt == 5
            || hm.values().filter(|n| (**n + jcnt) == 5).count() >= 1
        {
            HandType::FiveK
        } else if hm.values().filter(|n| **n == 4).count() == 1
            || jcnt == 4
            || hm.values().filter(|n| (**n + jcnt) == 4).count() >= 1
        {
            HandType::FourK
        } else if ((hm.values().filter(|n| **n == 3).count() == 1 || jcnt == 3)
            && (hm.values().filter(|n| **n == 2).count() == 1 || jcnt == 2))
            || (hm.values().filter(|n| **n == 2).count() == 2 && jcnt == 1)
        {
            HandType::Full
        } else if hm.values().filter(|n| **n == 3).count() == 1
            || jcnt == 3
            || hm.values().filter(|n| (**n + jcnt) == 3).count() >= 1
        {
            HandType::Threek
        } else if hm.values().filter(|n| **n == 2).count() == 2 {
            HandType::TwoPair
        } else if hm.values().filter(|n| **n == 2).count() == 1
            || hm.values().filter(|n| (**n + jcnt) == 2).count() >= 1
        {
            HandType::OnePair
        } else if (hm.values().filter(|n| **n == 1).count() == 5)
            || (hm.values().filter(|n| **n == 1).count() == 4 && jcnt == 1)
        {
            HandType::High
        } else {
            HandType::None
        }
    }
}

#[derive(Debug, Eq)]
struct Hand {
    htype: HandType,
    cards: Vec<CardType>,
    bet: u64,
}

impl Hand {
    fn new(cards: &str, bet: u64) -> Hand {
        let cards = cards.chars().map(|c| CardType::new(c).unwrap()).collect();
        let htype = HandType::new(&cards);

        Hand { htype, cards, bet }
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.htype == other.htype && self.cards.eq(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.htype == other.htype {
            self.cards.partial_cmp(&other.cards)
        } else if self.htype > other.htype {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.htype == other.htype {
            self.cards.partial_cmp(&other.cards).unwrap()
        } else if self.htype > other.htype {
            std::cmp::Ordering::Greater
        } else {
            std::cmp::Ordering::Less
        }
    }
}

fn main() {
    let re = Regex::new(r"(?<cards>(\w|\d)+) (?<bet>\d+)").unwrap();
    let mut res = re
        .captures_iter(PIN)
        .map(|c| (c["cards"].to_string(), c["bet"].parse::<u64>().unwrap()))
        .map(|h| Hand::new(&h.0, h.1))
        .collect::<Vec<Hand>>();
    res.sort();

    let res = res
        .iter()
        .enumerate()
        .map(|(i, h)| h.bet * ((i + 1) as u64))
        .sum::<u64>();

    println!("{res}");
}

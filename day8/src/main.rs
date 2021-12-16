use std::{collections::HashMap, iter::Map, str::FromStr};

struct Digit(u8);

struct Entry {
    signals: Vec<String>,
    digits: Vec<String>,
}

impl FromStr for Entry {
    type Err = std::convert::Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input: Vec<Vec<String>> = input
            .split("|")
            .map(|v| {
                v.split(" ")
                    .map(str::trim)
                    .filter(|v| v.len() != 0)
                    .map(|v| {
                        let mut chars: Vec<char> = v.chars().collect();
                        chars.sort();
                        String::from_iter(chars)
                    })
                    .collect()
            })
            .collect();

        Ok(Entry {
            signals: input[0].clone(),
            digits: input[1].clone(),
        })
    }
}

fn get_number_of_distinguishible_digits(entries: &[Entry]) -> usize {
    entries
        .iter()
        .flat_map(|v| &v.digits)
        .fold(0, |accum, digit| match digit.len() {
            2 | 4 | 3 | 7 => accum + 1,
            _ => accum,
        })
}

fn has_chars(base: &String, pattern: &String) -> bool {
    pattern.chars().all(|v| base.contains(v))
}

fn map_signals_to_numbers(signals: &[String]) -> HashMap<String, u8> {
    let one = signals.iter().find(|v| v.len() == 2).unwrap();
    let four = signals.iter().find(|v| v.len() == 4).unwrap();
    let seven = signals.iter().find(|v| v.len() == 3).unwrap();
    let eight = signals.iter().find(|v| v.len() == 7).unwrap();

    let mut mapping = HashMap::new();
    mapping.insert(one.clone(), 1);
    mapping.insert(four.clone(), 4);
    mapping.insert(seven.clone(), 7);
    mapping.insert(eight.clone(), 8);

    for signal in signals {
        if signal.len() == 5 {
            let four_minus_one: String = four.chars().filter(|v| !one.contains(*v)).collect();
            if one.chars().all(|v| signal.contains(v)) {
                mapping.insert(signal.clone(), 3);
            } else if four_minus_one.chars().all(|v| signal.contains(v)) {
                mapping.insert(signal.clone(), 5);
            } else {
                mapping.insert(signal.clone(), 2);
            }
        } else if signal.len() == 6 {
            if four.chars().all(|v| signal.contains(v)) {
                mapping.insert(signal.clone(), 9);
            } else if seven.chars().all(|v| signal.contains(v)) {
                mapping.insert(signal.clone(), 0);
            } else {
                mapping.insert(signal.clone(), 6);
            }
        }
        // other length are already handled above...
    }

    mapping
}

fn get_digits_sum(entries: &[Entry]) -> usize {
    let mut sum = 0;
    for entry in entries {
        let mapping = map_signals_to_numbers(&entry.signals);

        let mut num = 0;
        for digit in &entry.digits {
            num += *mapping.get(digit).unwrap() as usize;
            num *= 10;
        }

        sum += num / 10;
    }

    sum
}

fn main() {
    let input = include_str!("../input.txt");

    let entries: Vec<Entry> = input
        .lines()
        .map(str::parse::<Entry>)
        .map(Result::unwrap)
        .collect();

    println!("part 1: {}", get_number_of_distinguishible_digits(&entries));
    println!("part 2: {}", get_digits_sum(&entries))
}

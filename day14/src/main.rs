use std::{collections::HashMap, hash::Hash};

fn bruteforce_polymerisation(
    iteration: usize,
    initial_polymer: &Vec<char>,
    insertion_rules: &HashMap<(char, char), char>,
) -> HashMap<(char, char), usize> {
    let mut polymer = HashMap::new();
    for pair in initial_polymer
        .iter()
        .cloned()
        .zip(initial_polymer.iter().cloned().skip(1))
    {
        let entry = polymer.entry(pair).or_insert(0);
        *entry += 1;
    }

    for _ in 0..iteration {
        let mut new_polymer = HashMap::<(char, char), usize>::new();

        for (pair, count) in polymer {
            if let Some(permutation) = insertion_rules.get(&pair) {
                let entry = new_polymer.entry((pair.0, *permutation)).or_default();
                *entry += count;
                let entry = new_polymer.entry((*permutation, pair.1)).or_default();
                *entry += count;
            } else {
                let entry = new_polymer.entry(pair).or_default();
                *entry += count;
            }
        }

        polymer = new_polymer;
    }

    polymer
}

fn get_polymerisation_process_result(
    iteration: usize,
    polymer: &Vec<char>,
    insertion_rules: &HashMap<(char, char), char>,
) -> usize {
    let final_polymer = bruteforce_polymerisation(iteration, polymer, insertion_rules);
    let mut counter = HashMap::<char, usize>::new();
    for ((_, elem), count) in final_polymer {
        let entry = counter.entry(elem).or_default();
        *entry += count;
    }

    let max_count = *counter.iter().map(|(_, count)| count).max().unwrap();
    let min_count = *counter.iter().map(|(_, count)| count).min().unwrap();

    max_count - min_count
}

fn main() {
    let input = include_str!("../input.txt");

    let mut lines = input.lines();
    let polymer: Vec<char> = lines.next().unwrap().chars().collect();
    lines.next();

    let mut insertion_rules = HashMap::<(char, char), char>::new();
    for line in lines {
        let token = line
            .split(" -> ")
            .map(|v| v.chars().collect())
            .collect::<Vec<Vec<char>>>();
        insertion_rules.insert((token[0][0], token[0][1]), token[1][0]);
    }

    println!(
        "part 1: {}",
        get_polymerisation_process_result(10, &polymer, &insertion_rules)
    );

    println!(
        "part 2: {}",
        get_polymerisation_process_result(40, &polymer, &insertion_rules)
    );
}

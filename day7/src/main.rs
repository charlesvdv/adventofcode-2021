use std::borrow::{Borrow, BorrowMut};

fn get_median(elem: &Vec<usize>) -> Vec<usize> {
    let mut elem = elem.clone();
    elem.sort();

    let middle = elem.len() / 2;
    if (elem.len() % 2 == 0) {
        vec![elem[middle], elem[middle + 1]]
    } else {
        vec![elem[middle]]
    }
}

fn calculate_least_fuel_consumption(position: &Vec<usize>) -> usize {
    get_median(position)
        .iter()
        .map(|median| {
            position.iter().fold(0, |accum, val| {
                accum + (*val as isize - *median as isize).abs()
            })
        })
        .min()
        .unwrap() as usize
}

fn get_exponential_cost(diff: isize) -> usize {
    ((diff * (diff + 1)) as f64 / 2f64).round() as usize
}

fn calculate_least_fuel_consumption_with_exponential_rate(position: &Vec<usize>) -> usize {
    let average_position = (position.iter().sum::<usize>() as f64 / position.len() as f64);

    [average_position.floor(), average_position.ceil()]
        .iter()
        .map(|average| {
            position.iter().fold(0, |accum, val| {
                accum + get_exponential_cost((*val as isize - *average as isize).abs())
            })
        })
        .min()
        .unwrap() as usize
}

fn main() {
    let input = include_str!("../input.txt");

    let position: Vec<usize> = input
        .split(",")
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    println!("part 1: {}", calculate_least_fuel_consumption(&position));
    println!(
        "part 2: {}",
        calculate_least_fuel_consumption_with_exponential_rate(&position)
    );
}

use core::num;
use std::{convert::Infallible, str::FromStr};

#[derive(Clone)]
struct BinaryNumber(u32);

impl FromStr for BinaryNumber {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut num: u32 = 0b0;
        for (index, char) in s.chars().rev().enumerate() {
            let index_val: u32 = char.to_digit(10).unwrap();
            num = num | (index_val << index);
        }

        Ok(BinaryNumber(num))
    }
}

impl BinaryNumber {
    fn from_u32(num: u32) -> BinaryNumber {
        BinaryNumber(num)
    }

    fn bit_size() -> usize {
        12
    }

    fn bit_value(&self, pos: usize) -> u32 {
        ((0b1 << pos) & self.0) >> pos
    }
}

fn get_most_common_bit_for_pos(pos: usize, numbers: &Vec<BinaryNumber>) -> u32 {
    let majority_limit: u32 = (numbers.len() as f64 / 2.0).ceil() as u32;
    let count = numbers
        .iter()
        .fold(0, |accum, item| accum + item.bit_value(pos));

    if count < majority_limit {
        0
    } else {
        1
    }
}

fn get_most_common_bit_number(numbers: &Vec<BinaryNumber>) -> BinaryNumber {
    let mut val: u32 = 0b0;

    for pos in 0..BinaryNumber::bit_size() {
        if get_most_common_bit_for_pos(pos, numbers) == 1 {
            val = val | 0b1 << pos;
        }
    }

    BinaryNumber::from_u32(val)
}

fn get_number_from_bit_criteria<F>(numbers: &Vec<BinaryNumber>, criteria: &F) -> BinaryNumber
where
    F: Fn(u32, usize, &BinaryNumber) -> bool,
{
    let mut numbers = numbers.clone();
    for pos in (0..BinaryNumber::bit_size()).rev() {
        let most_common_bit = get_most_common_bit_for_pos(pos, &numbers);
        let tmp_numbers: Vec<BinaryNumber> = numbers
            .clone()
            .into_iter()
            .filter(move |v| criteria(most_common_bit, pos, v))
            .collect();
        if tmp_numbers.is_empty() {
            return numbers.last().unwrap().clone();
        } else if tmp_numbers.len() == 1 {
            return tmp_numbers[0].clone();
        } else {
            numbers = tmp_numbers;
        }
    }

    numbers.last().unwrap().clone()
}

fn oxigen_generator_rating_criteria(
    most_common_val: u32,
    pos: usize,
    number: &BinaryNumber,
) -> bool {
    let number_val_at_pos = (number.0 >> pos) & 0b1;
    most_common_val == number_val_at_pos
}

fn co2_scrubber_rating_criteria(most_common_val: u32, pos: usize, number: &BinaryNumber) -> bool {
    let number_val_at_pos = (number.0 >> pos) & 0b1;
    most_common_val != number_val_at_pos
}

fn main() {
    let input = include_str!("../input.txt");

    let numbers: Vec<BinaryNumber> = input
        .lines()
        .map(|v| v.parse::<BinaryNumber>().unwrap())
        .collect();

    let gamma_rate = get_most_common_bit_number(&numbers);

    let mut mask = 0b0;
    for pos in 0..input.lines().next().unwrap().len() {
        mask = mask | (0b1 << pos);
    }
    let epsilon_rate = (!gamma_rate.0) & mask;

    println!("part 1: {}", gamma_rate.0 * epsilon_rate);

    let oxigen_generator_rating =
        get_number_from_bit_criteria(&numbers, &oxigen_generator_rating_criteria);
    let co2_scrubber_rating = get_number_from_bit_criteria(&numbers, &co2_scrubber_rating_criteria);

    println!(
        "part 2: {}",
        oxigen_generator_rating.0 * co2_scrubber_rating.0
    );
}

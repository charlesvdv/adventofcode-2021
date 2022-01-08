use std::cmp;
use std::collections::HashSet;

fn get_x_power_range(x_range: &(i32, i32)) -> Vec<i32> {
    let mut result = HashSet::new();

    for power in 1..1000 {
        let mut i = power;
        let mut accum = power;

        while accum <= x_range.1 && i >= 0 {
            if accum >= x_range.0 {
                result.insert(power);
            }

            i -= 1;
            accum += i;
        }
    }

    let mut result = result.into_iter().collect::<Vec<i32>>();
    result.sort();
    result
}

fn get_y_power_range(y_range: &(i32, i32)) -> Vec<i32> {
    let mut result = HashSet::new();

    for power in -1000..1000 {
        let mut i = power;
        let mut accum = i;

        while accum >= y_range.0 {
            if accum <= y_range.1 {
                result.insert(power);
            }

            i -= 1;
            accum += i;
        }
    }

    let mut result = result.into_iter().collect::<Vec<i32>>();
    result.sort();
    result
}

fn is_solution_possible(start: &(i32, i32), x_range: &(i32, i32), y_range: &(i32, i32)) -> bool {
    let mut current = start.clone();
    let mut velocity = start.clone();

    while current.0 <= x_range.1 && current.1 >= y_range.0 {
        if current.0 >= x_range.0 && current.1 <= y_range.1 {
            return true;
        }

        velocity = (cmp::max(velocity.0 - 1, 0), velocity.1 - 1);
        current = (current.0 + velocity.0, current.1 + velocity.1);
    }

    return false;
}

fn main() {
    let input = include_str!("../input.txt");

    let input = input
        .lines()
        .next()
        .unwrap()
        .strip_prefix("target area: ")
        .unwrap();

    let coord = input
        .split(", ")
        .map(|v| v.split("=").nth(1).unwrap())
        .map(|v| {
            v.split("..")
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let x_range = (coord[0][0], coord[0][1]);
    let y_range = (coord[1][0], coord[1][1]);

    let max_y_power = get_y_power_range(&y_range).iter().cloned().max().unwrap();
    println!("part 1: {}", (max_y_power * (max_y_power + 1)) / 2);

    let y_power_range = get_y_power_range(&y_range);

    let solution = get_x_power_range(&x_range)
        .iter()
        .map(|x| {
            y_power_range
                .iter()
                .map(|y| (*x, *y))
                .collect::<Vec<(i32, i32)>>()
        })
        .flatten()
        .filter(|start| is_solution_possible(start, &x_range, &y_range))
        .collect::<Vec<(i32, i32)>>();

    println!("part 2: {}", solution.len())
}

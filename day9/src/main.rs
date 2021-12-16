use std::collections::BTreeSet;

fn get_low_points_loc(map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut points = Vec::new();

    // First, get the local minimun per line
    for (y, line) in map.iter().enumerate() {
        let mut previous_height = u8::MAX;
        for (x, (current_height, next_height)) in line.iter().zip(&line[1..]).enumerate() {
            if current_height < &previous_height && current_height < next_height {
                points.push((x, y));
            } else if x == map[0].len() - 2 && next_height < current_height {
                points.push((x + 1, y));
            }

            previous_height = *current_height;
        }
    }

    // Second, check if the point is also a local minimum compared to its bottom and top neighbours.
    points.retain(|point| {
        let mut retain = true;
        let point_height = map[point.1][point.0];
        if point.1 > 0 {
            if point_height > map[point.1 - 1][point.0] {
                retain = false;
            }
        }

        if point.1 + 1 < map.len() {
            if point_height > map[point.1 + 1][point.0] {
                return false;
            }
        }

        retain
    });

    points
}

fn is_point_in_map_bound(map: &Vec<Vec<u8>>, point: &(usize, usize)) -> bool {
    point.0 >= 0 && point.1 >= 0 && point.0 < map[0].len() && point.1 < map.len()
}

fn get_bassin_size(map: &Vec<Vec<u8>>, point: &(usize, usize)) -> usize {
    let mut visited_points = BTreeSet::new();
    let mut to_visit = vec![*point];

    while !to_visit.is_empty() {
        let visited_point = to_visit.pop().unwrap();
        visited_points.insert(visited_point);

        let (x, y) = visited_point;

        let mut new_points = vec![(x + 1, y), (x, y + 1)];
        if x > 0 {
            new_points.push((x - 1, y));
        }
        if y > 0 {
            new_points.push((x, y - 1));
        }

        new_points
            .iter()
            .filter(|p| is_point_in_map_bound(map, p))
            .filter(|p| !visited_points.contains(p))
            .filter(|(x, y)| map[*y][*x] != 9)
            .for_each(|p| to_visit.push(*p));

        // println!("{:?} {:?}", visited_points, to_visit);
    }

    visited_points.len()
}

fn get_bassins_sizes(map: &Vec<Vec<u8>>, lowest_points: &Vec<(usize, usize)>) -> Vec<usize> {
    lowest_points
        .iter()
        .map(|point| get_bassin_size(map, point))
        .collect()
}

fn main() {
    let input = include_str!("../input.txt");

    let map: Vec<Vec<u8>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|v| v.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect();

    let low_points_loc = get_low_points_loc(&map);
    println!(
        "part 1: {}",
        low_points_loc
            .iter()
            .fold(0, |accum: usize, (x, y)| accum + (map[*y][*x] as usize) + 1)
    );

    let mut bassins_size = get_bassins_sizes(&map, &low_points_loc);
    bassins_size.sort();
    println!(
        "part 2: {}",
        bassins_size.iter().rev().take(3).product::<usize>()
    );
}

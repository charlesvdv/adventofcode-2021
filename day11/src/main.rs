use std::collections::HashSet;

fn get_pos_neighbours(pos: &(usize, usize), map: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    let pos = (pos.0 as isize, pos.1 as isize);

    vec![
        (pos.0 - 1, pos.1),
        (pos.0 - 1, pos.1 + 1),
        (pos.0, pos.1 + 1),
        (pos.0 + 1, pos.1 + 1),
        (pos.0 + 1, pos.1),
        (pos.0 + 1, pos.1 - 1),
        (pos.0, pos.1 - 1),
        (pos.0 - 1, pos.1 - 1),
    ]
    .into_iter()
    .filter(|(x, y)| *x >= 0 && *y >= 0)
    .filter(|(x, y)| *x < map[0].len() as isize && *y < map.len() as isize)
    .map(|(x, y)| (x as usize, y as usize))
    .collect()
}

fn execute_one_step(map: &mut Vec<Vec<u32>>) -> usize {
    map.iter_mut()
        .for_each(|line| line.iter_mut().for_each(|v| *v = *v + 1));

    let mut to_visit: Vec<(usize, usize)> = map
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, glow)| **glow > 9)
                .map(|(x, _)| (x, y))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect();

    let mut visited = HashSet::new();
    to_visit.iter().for_each(|v| {
        let _ = visited.insert(*v);
    });

    while !to_visit.is_empty() {
        let pos = to_visit.pop().unwrap();

        for nb in get_pos_neighbours(&pos, &map) {
            map[nb.1][nb.0] += 1;
            if map[nb.1][nb.0] > 9 && !visited.contains(&nb) {
                to_visit.push(nb);
                visited.insert(nb);
            }
        }
    }

    let flash_count = map.iter().flatten().filter(|v| **v > 9).count();
    map.iter_mut()
        .for_each(|line| line.iter_mut().filter(|v| **v > 9).for_each(|v| *v = 0));

    flash_count
}

fn get_number_of_flashes_after_nth_steps(step_count: usize, map: Vec<Vec<u32>>) -> usize {
    let mut map = map;
    let mut flash_count = 0;
    for _ in 0..step_count {
        flash_count += execute_one_step(&mut map);
    }

    flash_count
}

fn get_sync_step(map: Vec<Vec<u32>>) -> usize {
    let octopus_count = map.len() * map[0].len();
    let mut step = 0;

    let mut map = map;
    loop {
        step += 1;
        if octopus_count == execute_one_step(&mut map) {
            break;
        }
    }

    return step;
}

fn main() {
    let input = include_str!("../input.txt");

    let map: Vec<Vec<u32>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    println!(
        "part 1: {}",
        get_number_of_flashes_after_nth_steps(100, map.clone())
    );
    println!("part 2: {}", get_sync_step(map),);
}

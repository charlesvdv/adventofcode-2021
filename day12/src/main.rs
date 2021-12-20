use std::collections::{HashMap, HashSet};

fn is_small_cave(name: &str) -> bool {
    name.chars().all(|v| v.is_ascii_lowercase()) && name != "start" && name != "end"
}

fn get_all_paths(
    map: &HashMap<String, Vec<String>>,
    current_path: Vec<String>,
    visited_small_caves: HashSet<String>,
    include_twice_small_caves: bool,
) -> Vec<Vec<String>> {
    let mut paths = Vec::new();

    if let Some(linked_caves) = map.get(current_path.last().unwrap()) {
        for cave in linked_caves {
            let mut include_twice_small_caves = include_twice_small_caves;

            if visited_small_caves.contains(cave) {
                if include_twice_small_caves {
                    include_twice_small_caves = false;
                } else {
                    continue;
                }
            }

            let mut new_path = current_path.clone();
            new_path.push(cave.clone());

            if cave == "end" {
                paths.push(new_path);
            } else {
                let mut new_visited_small_caves = visited_small_caves.clone();
                if is_small_cave(cave) {
                    new_visited_small_caves.insert(cave.clone());
                }

                for resulting_path in get_all_paths(
                    map,
                    new_path,
                    new_visited_small_caves,
                    include_twice_small_caves,
                ) {
                    paths.push(resulting_path);
                }
            }
        }
    }

    paths
}

fn insert_into_map(map: &mut HashMap<String, Vec<String>>, key: &str, val: &str) {
    if key == "end" {
        return;
    }
    if val == "start" {
        return;
    }
    if let Some(value) = map.get_mut(key) {
        value.push(String::from(val));
    } else {
        map.insert(String::from(key), vec![String::from(val)]);
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for line in input.lines() {
        let tokens: Vec<String> = line.split("-").map(String::from).collect();
        insert_into_map(&mut map, &tokens[0], &tokens[1]);
        insert_into_map(&mut map, &tokens[1], &tokens[0]);
    }

    println!(
        "part 1: {}",
        get_all_paths(&map, vec![String::from("start")], HashSet::new(), false)
            .into_iter()
            .count()
    );
    println!(
        "part 2: {}",
        get_all_paths(&map, vec![String::from("start")], HashSet::new(), true)
            .into_iter()
            .count()
    );
}

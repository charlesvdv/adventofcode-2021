use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

#[derive(Clone)]
struct Map {
    map: Vec<Vec<u32>>,
    multiplier: usize,
}

impl Map {
    fn multiply(mut self, multiplier: usize) -> Map {
        self.multiplier = multiplier;
        self
    }

    fn size(&self) -> (usize, usize) {
        (
            self.map[0].len() * self.multiplier,
            self.map.len() * self.multiplier,
        )
    }

    fn get_cost(&self, (x, y): &(usize, usize)) -> u32 {
        let (limit_x, limit_y) = (self.map[0].len(), self.map.len());

        let (multiplier_x, multiplier_y) = (x / limit_x, y / limit_y);
        let (base_x, base_y) = (x % limit_x, y % limit_y);

        let mut cost = self.map[base_y][base_x];

        for _ in 0..(multiplier_x + multiplier_y) {
            cost += 1;
            if cost == 10 {
                cost = 1;
            }
        }

        cost
    }
}

impl FromStr for Map {
    type Err = std::convert::Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let map = input
            .lines()
            .map(|v| v.chars().map(|v| v.to_digit(10).unwrap()).collect())
            .collect::<Vec<Vec<u32>>>();

        Ok(Map { map, multiplier: 1 })
    }
}

#[derive(Eq)]
struct State {
    pos: (usize, usize),
    path_cost: u32,
    heuristic_cost: u32,
}

impl State {
    fn cost(&self) -> u32 {
        self.heuristic_cost + self.path_cost
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.cost() == other.cost()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.cost().partial_cmp(&self.cost()).unwrap())
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost().cmp(&self.cost())
    }
}

fn get_lowest_risk_path(map: &Map) -> u32 {
    let mut potential_paths = BinaryHeap::<State>::new();

    let end_pos = (map.size().0 - 1, map.size().1 - 1);
    potential_paths.push(State {
        pos: (0, 0),
        path_cost: 0,
        heuristic_cost: (end_pos.0 + end_pos.1) as u32,
    });

    let mut current_path_cost = HashMap::<(usize, usize), u32>::new();
    current_path_cost.insert((0, 0), 0);

    while potential_paths.peek().unwrap().pos != end_pos {
        let state = potential_paths.pop().unwrap();

        [(-1, 0), (1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(|(x, y)| (x + state.pos.0 as i32, y + state.pos.1 as i32))
            .filter(|(x, y)| *x >= 0 && *y >= 0 && *x <= end_pos.0 as i32 && *y <= end_pos.1 as i32)
            .map(|(x, y)| State {
                pos: (x as usize, y as usize),
                path_cost: state.path_cost + map.get_cost(&(x as usize, y as usize)),
                heuristic_cost: ((end_pos.0 - x as usize) + (end_pos.1 - y as usize)) as u32,
            })
            .for_each(|state| {
                let entry = current_path_cost.entry(state.pos).or_insert(u32::MAX);
                if state.path_cost < *entry {
                    *entry = state.path_cost;
                    potential_paths.push((state));
                }
            });
    }

    potential_paths.peek().unwrap().path_cost
}

fn main() {
    let input = include_str!("../input.txt");

    // let map = input
    //     .lines()
    //     .map(|v| v.chars().map(|v| v.to_digit(10).unwrap()).collect())
    //     .collect::<Vec<Vec<u32>>>();

    let map = input.parse::<Map>().unwrap();

    println!("part 1: {}", get_lowest_risk_path(&map));

    let map_5 = map.multiply(5);
    println!("part 2: {}", get_lowest_risk_path(&map_5));
}

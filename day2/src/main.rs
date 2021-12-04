use std::str::FromStr;

enum Command {
    Forward(usize),
    Up(usize),
    Down(usize),
}

impl FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        let mut split = s.split(" ");
        let direction = split.next().unwrap();
        let value = split.next().unwrap().parse::<usize>().unwrap();

        match direction {
            "forward" => Ok(Command::Forward(value)),
            "up" => Ok(Command::Up(value)),
            "down" => Ok(Command::Down(value)),
            _ => unreachable!(),
        }
    }
}

fn calculate_pos(commands: &Vec<Command>) -> (usize, usize) {
    commands.iter().fold((0, 0), |(x, depth), cmd| match cmd {
        Command::Forward(v) => (x + v, depth),
        Command::Up(v) => (x, depth - v),
        Command::Down(v) => (x, depth + v),
    })
}

fn calculate_pos_with_aim(commands: &Vec<Command>) -> (usize, usize) {
    let result = commands
        .iter()
        .fold((0, 0, 0), |(x, depth, aim), cmd| match cmd {
            Command::Forward(v) => (x + v, depth + (aim * v), aim),
            Command::Up(v) => (x, depth, aim - v),
            Command::Down(v) => (x, depth, aim + v),
        });

    (result.0, result.1)
}

fn main() {
    let input = include_str!("../input.txt");
    let commands: Vec<Command> = input
        .lines()
        .map(|v| v.parse::<Command>().unwrap())
        .collect();

    let pos = calculate_pos(&commands);
    println!("part 1: {}", pos.0 * pos.1);

    let pos = calculate_pos_with_aim(&commands);
    println!("part 2: {}", pos.0 * pos.1);
}

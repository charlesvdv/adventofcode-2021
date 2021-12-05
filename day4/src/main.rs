use std::str::FromStr;

#[derive(Clone, Debug)]
struct Number {
    number: usize,
    marked: bool,
}

impl Number {
    fn new(number: usize) -> Self {
        Number {
            number,
            marked: false,
        }
    }

    fn mark(&mut self) {
        self.marked = true;
    }

    fn is_marked(&self) -> bool {
        self.marked
    }

    fn number(&self) -> usize {
        self.number
    }
}

#[derive(Clone, Debug)]
struct Board(Vec<Vec<Number>>);

impl Board {
    fn parse(board_lines: &[&str]) -> Board {
        let board: Vec<Vec<Number>> = board_lines
            .iter()
            .map(|v| {
                v.split(' ')
                    .map(|v| v.trim())
                    .filter(|v| !v.is_empty())
                    .map(|v| v.parse::<usize>().unwrap())
                    .map(|v| Number::new(v))
                    .collect()
            })
            .collect();

        Board(board)
    }

    fn mark_number(&mut self, number: usize) {
        self.0.iter_mut().flatten().for_each(|v| {
            if v.number() == number {
                v.mark()
            }
        })
    }

    fn is_winner(&self) -> bool {
        let line_winning = self.0.iter().any(|line| line.iter().all(Number::is_marked));
        if line_winning {
            return true;
        }

        debug_assert_eq!(self.0.len(), self.0[0].len());
        for index in 0..self.0.len() {
            if self.0.iter().all(|v| v[index].is_marked()) {
                return true;
            }
        }

        return false;
    }

    fn score(&self) -> usize {
        self.0.iter().flatten().fold(0, |accum, number| {
            if number.is_marked() {
                accum
            } else {
                accum + number.number()
            }
        })
    }
}

fn parse(input: &str) -> (Vec<usize>, Vec<Board>) {
    let mut input_lines = input.lines().peekable();

    let numbers_drawn: Vec<usize> = input_lines
        .next()
        .unwrap()
        .split(',')
        .map(|v| v.parse::<usize>().unwrap())
        .collect();

    let mut boards = Vec::new();
    while input_lines.peek().is_some() {
        input_lines.next().unwrap(); // consume empty lines

        let board_lines: Vec<&str> = input_lines.by_ref().take(5).collect();
        boards.push(Board::parse(&board_lines));
    }

    (numbers_drawn, boards)
}

fn winning_board(numbers_drawns: &Vec<usize>, mut boards: Vec<Board>) -> (usize, Board) {
    for number in numbers_drawns {
        boards
            .iter_mut()
            .for_each(|board| board.mark_number(*number));

        if let Some(board) = boards.iter().find(|board| board.is_winner()) {
            return (*number, board.clone());
        }
    }

    unreachable!();
}

fn looser_board(numbers_drawns: &Vec<usize>, mut boards: Vec<Board>) -> (usize, Board) {
    let mut boards = boards;
    for number in numbers_drawns {
        boards
            .iter_mut()
            .for_each(|board| board.mark_number(*number));

        if boards.len() == 1 && boards[0].is_winner() {
            return (*number, boards.last().unwrap().clone());
        }

        boards = boards
            .into_iter()
            .filter(|board| !board.is_winner())
            .collect();
    }

    unreachable!();
}

fn main() {
    let input = include_str!("../input.txt");

    let (numbers_drawns, boards) = parse(input);

    let (winning_number, winning_board) = winning_board(&numbers_drawns, boards.clone());
    println!("part 1: {}", winning_number * winning_board.score());

    let (loosing_number, loosing_board) = looser_board(&numbers_drawns, boards.clone());
    println!("part 2: {}", loosing_number * loosing_board.score());
}

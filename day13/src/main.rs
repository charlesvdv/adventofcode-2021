use std::{collections::HashSet, fmt};

enum Fold {
    Vertical(usize),
    Horizontal(usize),
}

fn get_point_pos_after_fold(fold: &Fold, (x, y): &(usize, usize)) -> (usize, usize) {
    match fold {
        &Fold::Vertical(pos) => {
            if *x < pos {
                (*x, *y)
            } else {
                (pos - (*x - pos), *y)
            }
        }
        &Fold::Horizontal(pos) => {
            if *y < pos {
                (*x, *y)
            } else {
                (*x, (pos - (*y - pos)))
            }
        }
    }
}

#[derive(Clone)]
struct Paper(HashSet<(usize, usize)>);

impl Paper {
    fn new() -> Self {
        Paper(HashSet::new())
    }

    fn add_point(&mut self, (x, y): &(usize, usize)) {
        self.0.insert((*x, *y));
    }

    fn fold(self, fold: &Fold) -> Self {
        let mut paper = Paper::new();

        for point in self.0 {
            paper.add_point(&get_point_pos_after_fold(fold, &point));
        }

        paper
    }
}

impl fmt::Display for Paper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut points: Vec<&(usize, usize)> = self.0.iter().collect();
        points.sort_by(|(xa, ya), (xb, yb)| {
            if ya != yb {
                ya.partial_cmp(yb).unwrap()
            } else {
                xa.partial_cmp(xb).unwrap()
            }
        });

        let max_x = *self.0.iter().map(|(x, _)| x).max().unwrap();
        let max_y = *self.0.iter().map(|(_, y)| y).max().unwrap();

        let mut points = points.into_iter().peekable();
        for y in 0..=max_y {
            for x in 0..=max_x {
                if let Some(point) = points.peek() {
                    if point == &&(x, y) {
                        write!(f, "#")?;
                        points.next();
                        continue;
                    }
                }
                write!(f, " ")?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn parse(input: &str) -> (Paper, Vec<Fold>) {
    let mut lines = input.lines();

    let mut paper = Paper::new();

    for line in &mut lines {
        if line.is_empty() {
            break;
        }

        let val: Vec<usize> = line
            .split(",")
            .map(str::parse)
            .map(Result::unwrap)
            .collect();

        paper.add_point(&(val[0], val[1]));
    }

    let mut folds = Vec::new();

    for line in lines {
        let line = line.strip_prefix("fold along ").unwrap();

        let val: Vec<&str> = line.split("=").collect();
        if val[0] == "x" {
            folds.push(Fold::Vertical(val[1].parse().unwrap()))
        } else {
            folds.push(Fold::Horizontal(val[1].parse().unwrap()))
        }
    }

    (paper, folds)
}

fn main() {
    let input = include_str!("../input.txt");

    let (paper, folds) = parse(input);

    println!("part 1: {}", paper.clone().fold(&folds[0]).0.iter().count());

    let folded_paper = folds
        .iter()
        .fold(paper.clone(), |intermediate_paper, fold| {
            intermediate_paper.fold(fold)
        });

    println!("part 2:");
    println!("{}", folded_paper);
}

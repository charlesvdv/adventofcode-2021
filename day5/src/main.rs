use std::cmp::{max, min};
use std::{cmp::Ordering, str::FromStr};

#[derive(Eq, PartialEq, Ord, Clone, Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
    fn zero() -> Point {
        Point { x: 0, y: 0 }
    }

    fn distance(&self, point: &Point) -> f32 {
        (((self.x.pow(2) - point.x.pow(2)) + (self.y.pow(2) - point.y.pow(2))) as f32).sqrt()
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let zero = Point::zero();
        self.distance(&zero).partial_cmp(&other.distance(&zero))
    }
}

impl FromStr for Point {
    type Err = std::convert::Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let part: Vec<usize> = input
            .split(",")
            .map(str::trim)
            .map(|v| v.parse::<usize>().unwrap())
            .collect();

        Ok(Point {
            x: part[0],
            y: part[1],
        })
    }
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn new(a: &Point, b: &Point) -> Line {
        Line {
            start: min(a, b).clone(),
            end: max(a, b).clone(),
        }
    }
}

impl FromStr for Line {
    type Err = std::convert::Infallible;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let points: Vec<Point> = input
            .split("->")
            .map(str::trim)
            .map(|v| v.parse::<Point>().unwrap())
            .collect();

        Ok(Line::new(&points[0], &points[1]))
    }
}

fn line_to_points_simple(line: &Line) -> Vec<Point> {
    if line.start.x == line.end.x {
        (line.start.y..=line.end.y)
            .map(|v| Point::new(line.start.x, v))
            .collect()
    } else if line.start.y == line.end.y {
        (line.start.x..=line.end.x)
            .map(|v| Point::new(v, line.start.y))
            .collect()
    } else {
        Vec::new()
    }
}

fn line_to_points_with_diagonal(line: &Line) -> Vec<Point> {
    if line.start.x == line.end.x {
        (line.start.y..=line.end.y)
            .map(|v| Point::new(line.start.x, v))
            .collect()
    } else if line.start.y == line.end.y {
        (line.start.x..=line.end.x)
            .map(|v| Point::new(v, line.start.y))
            .collect()
    } else {
        let length = (line.end.x - line.start.x) as isize;
        let x_direction: isize = if line.end.x as isize - line.start.x as isize > 0 {
            1
        } else {
            -1
        };
        let y_direction: isize = if line.end.y as isize - line.start.y as isize > 0 {
            1
        } else {
            -1
        };

        (0..=length)
            .map(|v| {
                let x = (v * x_direction) + line.start.x as isize;
                let y = (v * y_direction) + line.start.y as isize;

                Point::new(x as usize, y as usize)
            })
            .collect()
    }
}

struct CounterMap(Vec<Vec<i32>>);

impl CounterMap {
    fn new() -> CounterMap {
        CounterMap(Vec::new())
    }

    fn add_point(&mut self, point: &Point) {
        while self.0.len() <= point.y {
            self.0.push(Vec::new())
        }
        while self.0[point.y].len() <= point.x {
            self.0[point.y].push(0);
        }

        self.0[point.y][point.x] = self.0[point.y][point.x] + 1;
    }
}

fn get_map_overlap<F>(lines: &[Line], line_to_points: F) -> usize
where
    F: Fn(&Line) -> Vec<Point>,
{
    let mut map = CounterMap::new();

    lines
        .iter()
        .map(line_to_points)
        .flatten()
        .for_each(|v| map.add_point(&v));

    map.0.iter().flatten().filter(|v| **v > 1).count()
}

fn main() {
    let input = include_str!("../input.txt");

    let lines: Vec<Line> = input.lines().map(|v| v.parse::<Line>().unwrap()).collect();

    println!("part 1: {}", get_map_overlap(&lines, line_to_points_simple));
    println!(
        "part 2: {}",
        get_map_overlap(&lines, line_to_points_with_diagonal)
    );
}

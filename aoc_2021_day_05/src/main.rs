use lazy_static::lazy_static;
use regex::Regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let lines = parser(path).unwrap();

    let res_1 = solve_part_1(&lines);
    let res_2 = solve_part_2(&lines);

    println!("*-*-*-*-*- Day 05 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }

    fn points_intersected(&self) -> Vec<Point> {
        let mut result = vec![];

        let x_range: Vec<u32> = {
            if self.start.x > self.end.x {
                (self.end.x..=self.start.x).rev().collect()
            } else {
                (self.start.x..=self.end.x).collect()
            }
        };
        let y_range: Vec<u32> = {
            if self.start.y > self.end.y {
                (self.end.y..=self.start.y).rev().collect()
            } else {
                (self.start.y..=self.end.y).collect()
            }
        };

        if self.start.y == self.end.y {
            for x in x_range {
                result.push(Point::new(x, self.start.y));
            }
        } else if self.start.x == self.end.x {
            for y in y_range {
                result.push(Point::new(self.start.x, y));
            }
        } else {
            for (x, y) in x_range.into_iter().zip(y_range.into_iter()) {
                result.push(Point::new(x, y));
            }
        }

        result
    }
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

fn parser(path: &Path) -> io::Result<Vec<Line>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
            (?P<start_x>\d+)
            ,
            (?P<start_y>\d+)
            \x20->\x20
            (?P<end_x>\d+)
            ,
            (?P<end_y>\d+)
        ").unwrap();
    }

    let mut result = vec![];

    for line in buf.lines() {
        if let Ok(text) = line {
            let caps = RE.captures(&text).unwrap();

            let l = Line {
                start: Point::new(caps["start_x"].parse().unwrap(), caps["start_y"].parse().unwrap()),
                end: Point::new(caps["end_x"].parse().unwrap(), caps["end_y"].parse().unwrap()),
            };

            result.push(l);
        }
    }

    Ok(result)
}

fn solve_part_1(lines: &Vec<Line>) -> u32 {
    let mut map = HashMap::new();

    for line in lines.iter().filter(|l| l.is_horizontal_or_vertical()) {
        for pt in line.points_intersected() {
            if let Some(total) = map.get_mut(&pt) {
                *total = 1;
            } else {
                map.insert(pt, 0);
            }
        }
    }

    map.values().sum()
}

fn solve_part_2(lines: &Vec<Line>) -> u32 {
    let mut map = HashMap::new();

    for line in lines.iter() {
        for pt in line.points_intersected() {
            if let Some(total) = map.get_mut(&pt) {
                *total = 1;
            } else {
                map.insert(pt, 0);
            }
        }
    }

    map.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_points_intersected() {
        let l1 = Line {
            start: Point::new(0, 9),
            end: Point::new(5, 9),
        };

        let intersected = l1.points_intersected();
        for i in 0..=5 {
            assert_eq!(Point::new(i, 9), intersected[i as usize]);
        }
    }

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let lines = parser(path).unwrap();
        let res = solve_part_1(&lines);
        assert_eq!(5, res);
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input.txt");
        let lines = parser(path).unwrap();
        let res = solve_part_2(&lines);
        assert_eq!(12, res);
    }
}

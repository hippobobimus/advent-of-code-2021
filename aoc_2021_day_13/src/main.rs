use std::collections::HashSet;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let (points, folds) = parse(path).unwrap();

    let res_1 = solve_part_1(points.clone(), &folds);

    println!("*-*-*-*-*- Day 08 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2:");
    solve_part_2(points, &folds);
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct Point {
    x: usize,
    y: usize,
}

enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

impl Fold {
    fn fold(&self, mut point: Point) -> Point {
        match self {
            Self::Horizontal(row) => {
                if point.y < *row { return point; }
                point.y = 2 * row - point.y;
            },
            Self::Vertical(col) => {
                if point.x < *col { return point; }
                point.x = 2 * col - point.x;
            },
        }

        point
    }
}

fn parse(path: &Path) -> io::Result<(HashSet<Point>, Vec<Fold>)> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let (points, folds): (Vec<String>, Vec<String>) = buf.lines()
        .map(|l| l.unwrap())
        .partition(|l| l.starts_with(|c: char| c.is_digit(10)));

    let points = points.iter()
        .map(|line| {
            let (x, y) = line.split_once(",").unwrap();
            Point { x: x.parse::<usize>().unwrap(), y: y.parse::<usize>().unwrap() }
        })
        .collect();

    let folds = folds.iter().skip(1)
        .map(|l| l.trim_start_matches("fold along "))
        .map(|line| {
            let (axis, n) = line.split_once("=").unwrap();
            match axis {
                "x" => Fold::Vertical(n.parse::<usize>().unwrap()),
                "y" => Fold::Horizontal(n.parse::<usize>().unwrap()),
                _ => panic!("Failed to parse fold instruction!"),
            }
        })
        .collect();

    Ok((points, folds))
}

fn solve_part_1(mut points: HashSet<Point>, folds: &Vec<Fold>) -> usize {
    points = points.drain().map(|p| folds[0].fold(p)).collect();

    points.len()
}

fn solve_part_2(mut points: HashSet<Point>, folds: &Vec<Fold>) {
    for fold in folds {
        points = points.drain().map(|p| fold.fold(p)).collect();
    }

    let (mut max_x, mut max_y) = (0, 0);
    for p in points.iter() {
        max_x = max_x.max(p.x);
        max_y = max_y.max(p.y);
    }

    let mut visualisation = vec![vec!["."; max_x + 1]; max_y + 1];
    for point in points.iter() {
        visualisation[point.y][point.x] = "#";
    }

    for row in visualisation {
        for element in row {
            print!("{}", element);
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let (points, folds) = parse(path).unwrap();
        let res = solve_part_1(points, &folds);
        assert_eq!(17, res);
    }
}

use lazy_static::lazy_static;
use ndarray::prelude::*;

use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::mem;
use std::path::Path;


fn main() {
    let path = Path::new("input.txt");
    let scanners = parse(path).unwrap();

    let located_scanners = locate_all_scanners(scanners);

    println!("*-*-*-*-*- Day 19 -*-*-*-*-*\n");
    println!("Answer to part 1: Total unique beacons = {}",
             total_unique_beacons(&located_scanners));
    println!("Answer to part 2: Max manhattan distance = {}",
             max_manhattan_distance(&located_scanners));
}

fn parse(path: &Path) -> io::Result<Vec<Scanner>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let mut result = vec![];
    let mut points = vec![];

    for line in buf.lines().flatten() {
        if line.starts_with("---") {
            continue;
        }
        if line.is_empty() {
            result.push(Scanner::new(mem::take(&mut points)));
            continue;
        }

        let xyz = line.split(',')
            .map(|s| i32::from_str_radix(s, 10).unwrap())
            .collect::<Vec<i32>>();

        points.push(array![xyz[0], xyz[1], xyz[2]]);
    }
    result.push(Scanner::new(mem::take(&mut points)));

    Ok(result)
}

fn locate_all_scanners(mut scanners: Vec<Scanner>) -> Vec<Scanner> {
    let mut located_scanners = vec![];

    // scanner 0 is the origin.
    scanners[0].location = Some(array![0, 0, 0]);

    while !scanners.is_empty() {
        let reference_scanner = find_next_reference_scanner(&mut scanners);

        for scanner in scanners.iter_mut() {
            if scanner.location.is_some() { continue; }

            scanner.try_to_locate(&reference_scanner);
        }

        located_scanners.push(reference_scanner);
    }

    located_scanners
}

fn find_next_reference_scanner(scanners: &mut Vec<Scanner>) -> Scanner {
    for i in 0..scanners.len() {
        if scanners[i].location.is_some() {
            return scanners.remove(i);
        }
    }
    unreachable!()
}

fn manhattan_distance(a: &Point, b: &Point) -> i32 {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs() + (a[2] - b[2]).abs()
}

fn max_manhattan_distance(located_scanners: &Vec<Scanner>) -> i32 {
    let mut result = 0;

    for a in located_scanners.iter() {
        for b in located_scanners.iter() {
            if a == b { continue; }

            result = result.max(manhattan_distance(a.location.as_ref().unwrap(),
                                                   b.location.as_ref().unwrap()));

        }
    }

    result
}

fn total_unique_beacons(located_scanners: &Vec<Scanner>) -> usize {
    let mut unique_beacons = HashSet::new();

    for s in located_scanners.iter() {
        for beacon in s.global_beacons.iter() {
            unique_beacons.insert(beacon);
        }
    }

    unique_beacons.len()
}

#[derive(Eq, PartialEq, Debug)]
struct Scanner {
    local_beacons: Vec<Point>,
    global_beacons: Vec<Point>,
    location: Option<Point>,
}

impl Scanner {
    fn new(local_beacons: Vec<Point>) -> Self {
        Self {
            global_beacons: local_beacons.clone(),
            local_beacons,
            location: None,
        }
    }

    fn rotate(&mut self, rotation: &Rotation) {
        for (loc_pt, glob_pt) in self.local_beacons.iter().zip(self.global_beacons.iter_mut()) {
            *glob_pt = rotation.dot(loc_pt);
        }
    }

    fn translate(&mut self, translation: &Translation) {
        for glob_pt in self.global_beacons.iter_mut() {
            *glob_pt += translation;
        }
    }

    fn _transform(&mut self, rotation: &Rotation, translation: &Translation) {
        for (loc_pt, glob_pt) in self.local_beacons.iter().zip(self.global_beacons.iter_mut()) {
            *glob_pt = rotation.dot(loc_pt) + translation;
        }
    }

    fn try_to_locate(&mut self, reference_scanner: &Scanner) -> bool {
        for rotation in ROTATIONS.iter() {
            self.rotate(rotation);

            for point_a in reference_scanner.global_beacons.iter() {
                let transformed_beacons_a: HashSet<Point> = reference_scanner.global_beacons
                    .iter()
                    .map(|p| p - point_a)
                    .collect();

                for point_b in self.global_beacons.iter() {
                    let transformed_beacons_b: HashSet<Point> = self.global_beacons
                        .iter()
                        .map(|p| p - point_b)
                        .collect();

                    // test overlap
                    if transformed_beacons_a.intersection(&transformed_beacons_b).count() >= 6 {
                        let translation = point_a - point_b;

                        self.location = Some(translation.clone());

                        self.translate(&translation);

                        return true;
                    }
                }
            }
        }
        false
    }
}

type Point = Array1<i32>;
type Rotation = Array2<i32>;
type Translation = Array1<i32>;

lazy_static! {
    static ref ROTATIONS: Vec<Rotation> = generate_rotation_matrices();
}

fn generate_rotation_matrices() -> Vec<Rotation> {
    let first = vec![
        array![[1, 0, 0],
               [0, 1, 0],
               [0, 0, 1]],
        array![[0, 1, 0],
               [0, 0, 1],
               [1, 0, 0]],
        array![[0, 0, 1],
               [1, 0, 0],
               [0, 1, 0]],
    ];

    let second = vec![
        array![[1, 0, 0],
               [0, 1, 0],
               [0, 0, 1]],
        array![[-1, 0, 0],
               [0, -1, 0],
               [0, 0, 1]],
        array![[-1, 0, 0],
               [0, 1, 0],
               [0, 0, -1]],
        array![[1, 0, 0],
               [0, -1, 0],
               [0, 0, -1]],
    ];

    let third = vec![
        array![[1, 0, 0],
               [0, 1, 0],
               [0, 0, 1]],
        array![[0, 0, -1],
               [0, -1, 0],
               [-1, 0, 0]],
    ];

    let mut result = vec![];

    for a in first.iter() {
        for b in second.iter() {
            for c in third.iter() {
                result.push(a.dot(&b.dot(c)));
            }
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let scanners = parse(path).unwrap();
        let located_scanners = locate_all_scanners(scanners);
        let res = total_unique_beacons(&located_scanners);
        assert_eq!(79, res);
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input.txt");
        let scanners = parse(path).unwrap();
        let located_scanners = locate_all_scanners(scanners);
        let res = max_manhattan_distance(&located_scanners);
        assert_eq!(3621, res);
    }
}

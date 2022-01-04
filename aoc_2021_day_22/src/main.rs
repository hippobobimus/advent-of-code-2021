use lazy_static::lazy_static;
use regex::Regex;

use std::cmp;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let path = Path::new("input.txt");
    let commands = parse(path).unwrap();

    let mut reactor = Reactor::new();
    reactor.process_commands(commands);

    let part_1_sub_region = Region::new(Point::new(-50, -50, -50), Point::new(50, 50, 50));

    println!("*-*-*-*-*- Day 22 -*-*-*-*-*\n");
    println!("Answer to part 1: Total powered cubes in sub-region = {}",
             reactor.total_powered_cubes_in_region(&part_1_sub_region));
    println!("Answer to part 2: Total powered cubes = {}", reactor.total_powered_cubes());
}

struct Reactor {
    powered_regions: Vec<Region>,
}

impl Reactor {
    fn new() -> Self {
        Self { powered_regions: vec![] }
    }

    fn process_commands(&mut self, commands: Vec<Command>) {
        for cmd in commands {
            self.process_cmd(cmd);
        }
    }

    fn process_cmd(&mut self, cmd: Command) {
        // Find regions intersecting the command region, discard those fully enclosed by it.
        let mut intersecting_regions = vec![];
        let mut i = 0;
        while i < self.powered_regions.len() {
            if self.powered_regions[i].is_intersecting(&cmd.region) {
                let r = self.powered_regions.remove(i);
                if !r.is_enclosed_by(&cmd.region) {
                    intersecting_regions.push(r);
                }
            } else {
                i += 1;
            }
        }

        for mut r in intersecting_regions {
            if let Some(new_regions_vec) = r.split(&cmd.region.planes()) {
                self.powered_regions.extend(new_regions_vec);
            }
        }

        if cmd.state == State::On {
            self.powered_regions.push(cmd.region);
        }
    }

    fn total_powered_cubes(&self) -> i64 {
        self.powered_regions.iter().fold(0, |acc, r| acc + r.total_cubes())
    }

    fn total_powered_cubes_in_region(&self, region: &Region) -> i64 {
        let mut result = 0;
        for r in self.powered_regions.iter().filter(|r| r.is_intersecting(&region)) {
            if r.is_enclosed_by(&region) {
                result += r.total_cubes();
            } else {
                let min = Point::new(
                    cmp::max(r.min.x, region.min.x),
                    cmp::max(r.min.y, region.min.y),
                    cmp::max(r.min.z, region.min.z),
                );
                let max = Point::new(
                    cmp::min(r.max.x, region.max.x),
                    cmp::min(r.max.y, region.max.y),
                    cmp::min(r.max.z, region.max.z),
                );
                let overlapping_region = Region::new(min, max);
                result += overlapping_region.total_cubes();
            }
        }
        result
    }
}

#[derive(Debug, Copy, Clone)]
struct Region {
    min: Point,
    max: Point,
}

impl Region {
    fn new(min: Point, max: Point) -> Self {
        Self { min, max }
    }

    fn is_enclosed_by(&self, other: &Region) -> bool {
        self.min.x >= other.min.x && self.min.y >= other.min.y && self.min.z >= other.min.z
            && self.max.x <= other.max.x && self.max.y <= other.max.y && self.max.z <= other.max.z
    }

    fn is_intersecting(&self, other: &Region) -> bool {
        !(self.min.x >= other.max.x || self.min.y >= other.max.y || self.min.z >= other.max.z
            || self.max.x <= other.min.x || self.max.y <= other.min.y || self.max.z <= other.min.z)
    }

    fn total_cubes(&self) -> i64 {
        (self.max.x - self.min.x).abs() * (self.max.y - self.min.y).abs() * (self.max.z - self.min.z).abs()
    }

    fn split(&mut self, planes: &Vec<Plane>) -> Option<Vec<Region>> {
        let mut result = vec![];
        for p in planes.iter() {
            if let Some(new_region) = self.split_along_plane(p) {
                result.push(new_region);
            }
        }

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }

    fn split_along_plane(&mut self, plane: &Plane) -> Option<Region> {
        match plane {
            &Plane::Right(x) => {
                if x > self.max.x || x < self.min.x {
                    return None;
                }

                let new_region = Region::new(Point::new(x, self.min.y, self.min.z), self.max);
                self.max.x = x;

                return Some(new_region);
            },
            &Plane::Left(x) => {
                if x > self.max.x || x < self.min.x {
                    return None;
                }

                let new_region = Region::new(self.min, Point::new(x, self.max.y, self.max.z));
                self.min.x = x;

                return Some(new_region);
            },
            &Plane::Top(y) => {
                if y > self.max.y || y < self.min.y {
                    return None;
                }

                let new_region = Region::new(Point::new(self.min.x, y, self.min.z), self.max);
                self.max.y = y;

                return Some(new_region);
            },
            &Plane::Bottom(y) => {
                if y > self.max.y || y < self.min.y {
                    return None;
                }

                let new_region = Region::new(self.min, Point::new(self.max.x, y, self.max.z));
                self.min.y = y;

                return Some(new_region);
            },
            &Plane::Front(z) => {
                if z > self.max.z || z < self.min.z {
                    return None;
                }

                let new_region = Region::new(Point::new(self.min.x, self.min.y, z), self.max);
                self.max.z = z;

                return Some(new_region);
            },
            &Plane::Back(z) => {
                if z > self.max.z || z < self.min.z {
                    return None;
                }

                let new_region = Region::new(self.min, Point::new(self.max.x, self.max.y, z));
                self.min.z = z;

                return Some(new_region);
            },
        }
    }

    fn planes(&self) -> Vec<Plane> {
        vec![
            Plane::Right(self.max.x),
            Plane::Left(self.min.x),
            Plane::Top(self.max.y),
            Plane::Bottom(self.min.y),
            Plane::Front(self.max.z),
            Plane::Back(self.min.z),
        ]
    }
}

#[derive(Debug)]
enum Plane {
    // y-z plane, with x coord.
    Right(i64), // max x
    Left(i64), // min x
    // x-z plane, with y coord.
    Top(i64), // max y
    Bottom(i64), // min y
    // x-y plane, with z coord.
    Front(i64), // max z
    Back(i64), // min z
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new(x: i64, y: i64, z: i64) -> Self {
        Self { x, y, z }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum State {
    On,
    Off,
}

impl FromStr for State {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "on" => Ok(State::On),
            "off" => Ok(State::Off),
            _ => Err("Invalid input."),
        }
    }
}

#[derive(Debug)]
struct Command {
    state: State,
    region: Region,
}

fn parse(path: &Path) -> io::Result<Vec<Command>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let mut result = vec![];

    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?x)
            (?P<state>[onf]{2,3})
            \x20
            x=(?P<x_min>-?\d+)..(?P<x_max>-?\d+)
            ,
            y=(?P<y_min>-?\d+)..(?P<y_max>-?\d+)
            ,
            z=(?P<z_min>-?\d+)..(?P<z_max>-?\d+)
        ").unwrap();
    }

    for line in buf.lines().flatten() {
        let caps = RE.captures(&line).unwrap();

        let state = caps.name("state")
            .unwrap()
            .as_str()
            .parse::<State>()
            .expect("Failed to parse state");

        let x_min = i64::from_str_radix(caps.name("x_min").unwrap().as_str(), 10).unwrap();
        let x_max = i64::from_str_radix(caps.name("x_max").unwrap().as_str(), 10).unwrap();
        let y_min = i64::from_str_radix(caps.name("y_min").unwrap().as_str(), 10).unwrap();
        let y_max = i64::from_str_radix(caps.name("y_max").unwrap().as_str(), 10).unwrap();
        let z_min = i64::from_str_radix(caps.name("z_min").unwrap().as_str(), 10).unwrap();
        let z_max = i64::from_str_radix(caps.name("z_max").unwrap().as_str(), 10).unwrap();

        let cmd = Command {
            state,
            region: Region::new(Point::new(x_min, y_min, z_min), Point::new(x_max + 1, y_max + 1, z_max + 1)),
        };

        result.push(cmd);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    lazy_static! {
        static ref PART_1_REGION: Region = Region::new(Point::new(-50, -50, -50), Point::new(51, 51, 51));
    }

    #[test]
    fn test_small_input() {
        let path = Path::new("small_test_input.txt");
        let commands = parse(path).unwrap();

        let mut reactor = Reactor::new();
        reactor.process_commands(commands);

        assert_eq!(39, reactor.total_powered_cubes());
        assert_eq!(39, reactor.total_powered_cubes_in_region(&PART_1_REGION));
    }

    #[test]
    fn test_larger_input() {
        let path = Path::new("test_input_1.txt");
        let commands = parse(path).unwrap();

        let mut reactor = Reactor::new();
        reactor.process_commands(commands);

        assert_eq!(39769202357779, reactor.total_powered_cubes());
        assert_eq!(590784, reactor.total_powered_cubes_in_region(&PART_1_REGION));
    }

    #[test]
    fn test_largest_input() {
        let path = Path::new("test_input_2.txt");
        let commands = parse(path).unwrap();

        let mut reactor = Reactor::new();
        reactor.process_commands(commands);

        assert_eq!(2758514936282235, reactor.total_powered_cubes());
        assert_eq!(474140, reactor.total_powered_cubes_in_region(&PART_1_REGION));
    }
}

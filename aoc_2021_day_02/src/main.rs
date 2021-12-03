use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let input = parser(path).unwrap();

    let res_1 = solve_part_1(&input);
    let res_2 = solve_part_2(&input);

    println!("*-*-*-*-*- Day 02 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}

struct Position {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

struct Instruction {
    dir: Direction,
    val: u32,
}

enum Direction {
    Forward,
    Down,
    Up,
}
impl Direction {
    fn new(s: &str) -> Direction {
        match s {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => panic!("invalid input: does not correspond to a direction!"),
        }
    }
}

fn parser(path: &Path) -> io::Result<Vec<Instruction>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let result = buf.lines()
        .map(|l| {
            let line = l.expect("Line read error");
            let line = line.split_whitespace().collect::<Vec<&str>>();

            let dir = Direction::new(line[0]);
            let val = line[1].parse::<u32>().expect("Failed to parse magnitude");

            Instruction { dir, val }
        })
        .collect();

    Ok(result)
}

fn solve_part_1(input: &Vec<Instruction>) -> i32 {
    let position = input.iter().fold(Position { depth: 0, horizontal: 0, aim: 0 }, |mut pos, instr| {
        match instr.dir {
            Direction::Forward => pos.horizontal += instr.val as i32,
            Direction::Down => pos.depth += instr.val as i32,
            Direction::Up => pos.depth -= instr.val as i32,
        }

        pos
    });

    position.depth * position.horizontal
}

fn solve_part_2(input: &Vec<Instruction>) -> i32 {
    let position = input.iter().fold(Position { depth: 0, horizontal: 0, aim: 0 }, |mut pos, instr| {
        match instr.dir {
            Direction::Forward => {
                pos.horizontal += instr.val as i32;
                pos.depth += instr.val as i32 * pos.aim;
            },
            Direction::Down => pos.aim += instr.val as i32,
            Direction::Up => pos.aim -= instr.val as i32,
        }

        pos
    });

    position.depth * position.horizontal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let input = parser(path).unwrap();
        let res = solve_part_1(&input);
        assert_eq!(150, res);
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input.txt");
        let input = parser(path).unwrap();
        let res = solve_part_2(&input);
        assert_eq!(900, res);
    }
}

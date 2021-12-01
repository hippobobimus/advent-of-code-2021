use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let input = parser(path).unwrap();

    let res_1 = solve_part_1(&input);
    let res_2 = solve_part_2(&input);

    println!("*-*-*-*-*- Day 01 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}

fn parser(path: &Path) -> io::Result<Vec<i32>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);
    
    let lines = buf.lines()
        .map(|l| l.expect("Line read error").parse::<i32>().expect("Cannot parse line into i32"))
        .collect();

    Ok(lines)
}

fn solve_part_1(input: &Vec<i32>) -> i32 {
    input.windows(2)
        .fold(0, |acc, win| {
            return if win[1] > win[0] {
                acc + 1
            } else {
                acc
            }
    })
}

fn solve_part_2(input: &Vec<i32>) -> i32 {
    input.windows(3)
        .map(|win| win.iter().sum())
        .collect::<Vec<i32>>()
        .windows(2)
        .fold(0, |acc, win| {
            return if win[1] > win[0] {
                acc + 1
            } else {
                acc
            }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let input = parser(path).unwrap();
        let res = solve_part_1(&input);
        assert_eq!(7, res);
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input.txt");
        let input = parser(path).unwrap();
        let res = solve_part_2(&input);
        assert_eq!(5, res);
    }
}

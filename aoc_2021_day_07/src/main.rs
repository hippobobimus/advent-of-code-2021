use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let initial_positions = parser(path).unwrap();

    let res_1 = solve(true, &initial_positions);
    let res_2 = solve(false, &initial_positions);

    println!("*-*-*-*-*- Day 07 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}

fn parser(path: &Path) -> io::Result<Vec<i32>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let lines = buf.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let result = lines[0].split_terminator(",").map(|x| x.parse::<i32>().unwrap()).collect();

    Ok(result)
}

fn solve(part_1: bool, initial_positions: &Vec<i32>) -> i32 {
    let mut freq_map = HashMap::new();

    for p in initial_positions {
        if let Some(freq) = freq_map.get_mut(p) {
            *freq += 1;
        } else {
            freq_map.insert(p, 1);
        }
    }

    let total_crabs = initial_positions.len() as i32;
    let sum_of_initial_positions: i32 = initial_positions.iter().sum();

    // initially set based on moving to position 0.
    let mut fuel_usage = if part_1 {
        sum_of_initial_positions
    } else {
        initial_positions.iter().fold(0, |acc, x| acc + x * (x + 1) / 2)
    };

    let mut pos = 0;
    let mut static_crabs = freq_map.get(&pos).unwrap_or(&0);
    let mut fuel_delta = if part_1 {
        total_crabs - 2 * static_crabs
    } else {
        sum_of_initial_positions - static_crabs
    };

    while fuel_delta > 0 {
        fuel_usage -= fuel_delta;

        pos += 1;
        static_crabs = freq_map.get(&pos).unwrap_or(&0);

        if part_1 {
            fuel_delta -= 2 * static_crabs;
        } else {
            fuel_delta -= total_crabs + static_crabs;
        }
    }

    fuel_usage
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let initial_positions = parser(path).unwrap();
        let res = solve(true, &initial_positions);
        assert_eq!(37, res);
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input.txt");
        let initial_positions = parser(path).unwrap();
        let res = solve(false, &initial_positions);
        assert_eq!(168, res);
    }
}

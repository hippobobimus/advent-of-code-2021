use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let grid = parse(path).unwrap();

    let res_1 = solve(true, grid.clone());
    let res_2 = solve(false, grid);

    println!("*-*-*-*-*- Day 08 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}

fn parse(path: &Path) -> io::Result<Vec<Vec<u32>>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    // add an outer perimeter of 10s to the grid to ease indexing.
    let mut result: Vec<Vec<u32>> = buf.lines()
        .map(|l| {
            let mut row = l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>();
            row.insert(0, 10);
            row.push(10);
            row
        })
        .collect();

    result.insert(0, vec![10; 12]);
    result.push(vec![10; 12]);

    Ok(result)
}

fn solve(part_1: bool, mut grid: Vec<Vec<u32>>) -> u32 {
    let mut result = 0;

    for step in 1.. {
        for x in 1..11 {
            for y in 1..11 {
                process(x, y, &mut grid);
            }
        }

        let mut total_flashed = 0;

        for x in 1..11 {
            for y in 1..11 {
                if grid[x][y] == 10 {
                    total_flashed += 1;
                    grid[x][y] = 0;
                }
            }
        }

        if part_1 {
            result += total_flashed;
            if step == 100 {
                return result;
            }
        } else {
            if total_flashed == 100 {
                return step;
            }
        }
    }

    result
}

fn process(x: usize, y: usize, grid: &mut Vec<Vec<u32>>) {
    if grid[x][y] == 10 { return; } // already flashed.

    grid[x][y] += 1;

    if grid[x][y] <= 9 { return; } // hasn't flashed.

    process(x - 1, y - 1, grid);
    process(x - 1, y, grid);
    process(x - 1, y + 1, grid);
    process(x, y - 1, grid);
    process(x, y + 1, grid);
    process(x + 1, y - 1, grid);
    process(x + 1, y, grid);
    process(x + 1, y + 1, grid);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let grid = parse(path).unwrap();
        let res = solve(true, grid);
        assert_eq!(1656, res);
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input.txt");
        let grid = parse(path).unwrap();
        let res = solve(false, grid);
        assert_eq!(195, res);
    }
}

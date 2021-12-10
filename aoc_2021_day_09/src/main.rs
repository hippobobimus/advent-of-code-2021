use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let grid = parser(path).unwrap();

    let res_1 = solve(true, grid.clone());
    let res_2 = solve(false, grid);

    println!("*-*-*-*-*- Day 08 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}

fn parser(path: &Path) -> io::Result<Vec<Vec<u8>>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let mut grid = vec![];

    for line in buf.lines() {
        grid.push(
            line.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        );
    }

    Ok(grid)
}

fn solve(part_1: bool, mut grid: Vec<Vec<u8>>) -> u32 {
    let mut result_1 = 0;

    let rows = grid.len();
    let cols = grid[0].len();

    let mut min_heap = BinaryHeap::new();

    for x in 0..rows {
        for y in 0..cols {
            if part_1 {
                if degree(&grid, x, y, rows, cols) == 0 {
                    result_1 += 1 + grid[x][y] as u32;
                }
            } else {
                if grid[x][y] != 9 {
                    let basin_size = dfs_basin_size(&mut grid, x, y, rows, cols);

                    min_heap.push(Reverse(basin_size));

                    while min_heap.len() > 3 {
                        min_heap.pop();
                    }
                }
            }
        }
    }

    if part_1 {
        result_1
    } else {
        min_heap.iter().map(|v| v.0).product()
    }
}

fn dfs_basin_size(grid: &mut Vec<Vec<u8>>, x: usize, y: usize, rows: usize, cols: usize) -> u32 {
    if x >= rows || y >= cols || grid[x][y] == 9 {
        return 0;
    }

    let mut size = 1;

    grid[x][y] = 9;

    if let Some(x) = x.checked_sub(1) {
        size += dfs_basin_size(grid, x, y, rows, cols);
    }
    if let Some(y) = y.checked_sub(1) {
        size += dfs_basin_size(grid, x, y, rows, cols);
    }
    size += dfs_basin_size(grid, x + 1, y, rows, cols);
    size += dfs_basin_size(grid, x, y + 1, rows, cols);

    size
}

fn degree(grid: &Vec<Vec<u8>>, x: usize, y: usize, rows: usize, cols: usize) -> u8 {
    let mut degree = 0;

    let v = grid[x][y];

    if x > 0 && v >= grid[x - 1][y] { degree += 1; }
    if x < rows - 1 && v >= grid[x + 1][y] { degree += 1; }
    if y > 0 && v >= grid[x][y - 1] { degree += 1; }
    if y < cols - 1 && v >= grid[x][y + 1] { degree += 1; }

    degree
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let grid = parser(path).unwrap();
        let res = solve(true, grid);
        assert_eq!(15, res);
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input.txt");
        let grid = parser(path).unwrap();
        let res = solve(false, grid);
        assert_eq!(1134, res);
    }
}

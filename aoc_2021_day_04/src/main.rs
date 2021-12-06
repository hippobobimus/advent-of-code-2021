use ansi_term::Colour::{Red};

use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let (nums, boards) = parser(path).unwrap();

    let res_1 = solve(Part::One, &nums, boards.clone());
    let res_2 = solve(Part::Two, &nums, boards);

    println!("*-*-*-*-*- Day 04 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}

#[derive(Debug, Clone)]
struct Board {
    grid: [[u32; Self::DIMENSION]; Self::DIMENSION],
    unmarked: HashMap<u32, Position>, // unmarked values and their (row, col) position.
    marked_totals: HashMap<usize, usize>, // total marked in each row and col.
    has_won: bool,
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.grid {
            for v in row {
                if self.unmarked.contains_key(&v) {
                    write!(f, "{:>2} ", v)?;
                } else {
                    write!(f, "{:>2} ", Red.bold().paint(format!("{}", v)))?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}


impl Board {
    const DIMENSION: usize = 5;

    fn new(lines: Vec<String>) -> Self {
        let mut marked_totals = HashMap::new();
        for i in 0..(Self::DIMENSION * 2) {
            marked_totals.insert(i, 0);
        }

        let mut unmarked = HashMap::new();
        let mut grid = [[0; Self::DIMENSION]; Self::DIMENSION];
        for (row_idx, row) in lines.iter().enumerate() {
            for (col_idx, val) in row.split_whitespace().enumerate() {
                let val = val.parse::<u32>().unwrap();
                unmarked.insert(val, Position::new(row_idx, col_idx));
                grid[row_idx][col_idx] = val;
            }
        }

        Self { grid, unmarked, marked_totals, has_won: false }
    }

    /// Returns the score if the board wins, else None.
    fn mark(&mut self, n: u32) -> Option<u32> {
        if let Some(pos) = self.unmarked.remove(&n) {
            if let Some(row_mrks) = self.marked_totals.get_mut(&pos.row) {
                *row_mrks += 1;
            }

            if let Some(col_mrks) = self.marked_totals.get_mut(&(pos.col + Self::DIMENSION)) {
                *col_mrks += 1;
            }

            if self.marked_totals[&pos.row] == Self::DIMENSION
                || self.marked_totals[&(pos.col + Self::DIMENSION)] == Self::DIMENSION
            {
                self.has_won = true;
                return Some(n * self.unmarked.keys().sum::<u32>());
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
struct Position {
    row: usize,
    col: usize,
}

impl Position {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

fn parser(path: &Path) -> io::Result<(Vec<u32>, Vec<Board>)> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let mut lines_iter = buf.lines();

    let drawn_nums = {
        let l = lines_iter.next().unwrap().unwrap();
        l.split_terminator(",")
            .map(|v| v.parse::<u32>().unwrap())
            .collect()
    };

    let mut boards = vec![];
    let mut board_lines = vec![];

    for line in lines_iter {
        if let Ok(l) = line {
            if l.is_empty() {
                if board_lines.is_empty() {
                    continue;
                }

                boards.push(Board::new(board_lines));

                board_lines = vec![];
            } else {
                board_lines.push(l);
            }
        }
    }
    boards.push(Board::new(board_lines));

    Ok((drawn_nums, boards))
}

#[derive(PartialEq, Debug)]
enum Part {
    One,
    Two,
}

fn solve(part: Part, nums: &Vec<u32>, mut boards: Vec<Board>) -> u32 {
    let mut result = 0;
    'outer: for n in nums.into_iter() {
        for b in boards.iter_mut() {
            if b.has_won { continue; }
            if let Some(score) = b.mark(*n) {
                result = score;
                if part == Part::One {
                    break 'outer;
                }
            }
        }
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let (nums, boards) = parser(path).unwrap();
        let res = solve(Part::One, &nums, boards);
        assert_eq!(4512, res);
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input.txt");
        let (nums, boards) = parser(path).unwrap();
        let res = solve(Part::Two, &nums, boards);
        assert_eq!(1924, res);
    }
}

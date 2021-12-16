use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let grid = parse(path).unwrap();
    let ext_grid = expand_grid(&grid, 5);

    let res_1 = dijkstra(&grid);
    let res_2 = dijkstra(&ext_grid);

    println!("*-*-*-*-*- Day 15 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}

fn parse(path: &Path) -> io::Result<Vec<Vec<usize>>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    // add an outer perimeter of 10s to the grid to ease indexing.
    let mut grid: Vec<Vec<usize>> = buf.lines()
        .map(|l| {
            let mut row = l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>();
            row.insert(0, 10);
            row.push(10);
            row
        })
        .collect();

    grid.insert(0, vec![10; grid[0].len()]);
    grid.push(vec![10; grid[0].len()]);

    Ok(grid)
}

#[derive(Debug, Eq, PartialEq)]
struct State {
    risk: usize,
    x: usize,
    y: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk.cmp(&self.risk)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn expand_grid(grid: &Vec<Vec<usize>>, factor: usize) -> Vec<Vec<usize>> {
    let mut grid = grid.clone();
    grid.remove(0);
    grid.truncate(grid.len() - 1);

    for row in grid.iter_mut() {
        row.remove(0);
        row.truncate(row.len() - 1);
    }

    let mut new_grid = vec![vec![0; grid.len() * factor]; grid.len() * factor];

    for (x, row) in new_grid.iter_mut().enumerate() {
        for (y, val) in row.iter_mut().enumerate() {
            *val = (grid[x % grid.len()][y % grid[0].len()] + x / grid.len() + y / grid[0].len() - 1) % 9 + 1;
        }
    }

    for row in new_grid.iter_mut() {
        row.insert(0, 10);
        row.push(10);
    }

    new_grid.insert(0, vec![10; new_grid[0].len()]);
    new_grid.push(vec![10; new_grid[0].len()]);

    new_grid
}

fn dijkstra(grid: &Vec<Vec<usize>>) -> usize {
    let mut fringe: BinaryHeap<State> = BinaryHeap::new();
    let mut risk_to = vec![vec![usize::MAX; grid[0].len()]; grid.len()];
    let destination = (grid.len() - 2, grid[0].len() - 2);

    // insert start
    fringe.push(State { risk: 0, x: 1, y: 1 });

    while let Some(State { risk, x, y }) = fringe.pop() {
        // check if reached destination.
        if (x, y) == destination {
            return risk;
        }

        // check if a better path has already been found.
        if risk > risk_to[x][y] {
            continue;
        }

        // add neighbours to fringe.
        for (nbr_x, nbr_y) in neighbours(x, y) {
            let additional_risk = grid[nbr_x][nbr_y];

            // out of bounds.
            if additional_risk == 10 {
                continue;
            }

            let nbr_risk = risk + additional_risk;

            if nbr_risk < risk_to[nbr_x][nbr_y] {
                fringe.push(State { risk: nbr_risk, x: nbr_x, y: nbr_y });
                risk_to[nbr_x][nbr_y] = nbr_risk;
            }
        }
    }

    unreachable!()
}

fn neighbours(x: usize, y: usize) -> Vec<(usize, usize)> {
    vec![
        (x, y + 1),
        (x, y - 1),
        (x + 1, y),
        (x - 1, y),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let grid = parse(path).unwrap();
        let res = dijkstra(&grid);
        assert_eq!(40, res);
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input.txt");
        let grid = expand_grid(&parse(path).unwrap(), 5);
        let res = dijkstra(&grid);
        assert_eq!(315, res);
    }
}

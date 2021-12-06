use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let initial_fish = parser(path).unwrap();

    let res_1 = solve(&initial_fish, 80);
    let res_2 = solve(&initial_fish, 256);

    println!("*-*-*-*-*- Day 06 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}

fn parser(path: &Path) -> io::Result<Vec<usize>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let lines = buf.lines().map(|l| l.unwrap()).collect::<Vec<String>>();

    let result = lines[0].split_terminator(",").map(|x| x.parse::<usize>().unwrap()).collect();

    Ok(result)
}

fn solve(initial_generation: &Vec<usize>, days: i64) -> i64 {
    // number of fish at each stage of the lifecycle.
    let mut fish = [0; 9];
    
    for stage in initial_generation {
        fish[*stage] += 1;
    }

    for _ in 0..days {
        fish.rotate_left(1);
        // spawn
        fish[6] += fish[8];
    }

    fish.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let initial_fish = parser(path).unwrap();
        let res = solve(&initial_fish, 80);
        assert_eq!(5934, res);
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input.txt");
        let initial_fish = parser(path).unwrap();
        let res = solve(&initial_fish, 256);
        assert_eq!(26984457539, res);
    }
}

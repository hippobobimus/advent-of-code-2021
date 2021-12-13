use phf::phf_map;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

static CHARACTER_MAP: phf::Map<char, char> = phf_map! {
    '(' => ')',
    '[' => ']',
    '{' => '}',
    '<' => '>',
};
static SCORE_MAP_1: phf::Map<char, u64> = phf_map! {
    ')' => 3,
    ']' => 57,
    '}' => 1197,
    '>' => 25137,
};
static SCORE_MAP_2: phf::Map<char, u64> = phf_map! {
    ')' => 1,
    ']' => 2,
    '}' => 3,
    '>' => 4,
};

fn main() {
    let path = Path::new("input.txt");
    let lines = parser(path).unwrap();

    let res_1 = solve(true, &lines);
    let res_2 = solve(false, &lines);

    println!("*-*-*-*-*- Day 08 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}

fn parser(path: &Path) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let result = buf.lines()
        .map(|l| l.unwrap().chars().collect::<Vec<char>>())
        .collect();

    Ok(result)
}


fn solve(part_1: bool, lines: &Vec<Vec<char>>) -> u64 {
    let mut result = 0;
    let mut scores = vec![];
    let mut stack = vec![];

    'outer_loop: for l in lines {
        stack.clear();

        // check all characters in line are valid
        for c in l {
            if let Some(closing_char) = CHARACTER_MAP.get(&c) {
                stack.push(*closing_char);
            } else {
                if let Some(expected_char) = stack.pop() {
                    // discard corrupt lines.
                    if expected_char != *c {
                        if part_1 {
                            result += SCORE_MAP_1.get(&c).unwrap();
                        }
                        continue 'outer_loop;
                    }
                }
            }
        }

        // auto-completion score incomplete lines
        if !part_1 {
            let score = stack.iter().rev().fold(0, |acc, c| {
                (5 * acc) + SCORE_MAP_2.get(c).unwrap()
            });

            scores.push(score);
        }
    }

    if part_1 {
        result
    } else {
        scores.sort();
        scores[scores.len() / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let lines = parser(path).unwrap();
        let res = solve(true, &lines);
        assert_eq!(26397, res);
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input.txt");
        let lines = parser(path).unwrap();
        let res = solve(false, &lines);
        assert_eq!(288957, res);
    }
}

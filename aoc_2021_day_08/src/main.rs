use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let lines = parser(path).unwrap();

    let res_1 = solve(true, &lines);
    let res_2 = solve(false, &lines);

    println!("*-*-*-*-*- Day 08 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}

fn solve(part_1: bool, lines: &Vec<(Vec<u8>, Vec<u8>)>) -> u32 {
    let mut result: u32 = 0;

    for line in lines {
        let (patterns, output) = line;

        let decoder = analyse_patterns(patterns);

        for (i, code) in output.iter().rev().enumerate() {
            let val = *decoder.get(&code).unwrap_or(&0);
            if part_1 {
                if val == 1 || val == 4 || val == 7 || val == 8 {
                    result += 1;
                }
            } else {
                result += val as u32 * 10_u32.pow(i as u32);
            }
        }
    }

    result
}

// the first 7 bits correspond to the presence of characters a-g.
fn convert_to_bits(input: &str) -> u8 {
    let mut result = 0;

    for c in input.chars() {
        result += 1 << (c as u8 - 'a' as u8);
    }

    result
}

fn parser(path: &Path) -> io::Result<Vec<(Vec<u8>, Vec<u8>)>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let mut result = vec![];

    for line in buf.lines() {
        let line = line.unwrap();
        let parts = line.split_terminator(" | ").collect::<Vec<&str>>();

        let patterns = parts[0].split_whitespace().map(|s| convert_to_bits(s)).collect::<Vec<u8>>();

        let output = parts[1].split_whitespace().map(|s| convert_to_bits(s)).collect::<Vec<u8>>();

        result.push((patterns, output));
    }

    Ok(result)
}

fn analyse_patterns(patterns: &Vec<u8>) -> HashMap<u8, u8> {
    let mut decoder: HashMap<u8, u8> = HashMap::new();
    let mut encoder: HashMap<u8, u8> = HashMap::new();
    let mut num_segments_map: HashMap<u32, Vec<u8>> = HashMap::new();

    for p in patterns {
        let num_segments = p.count_ones();

        match num_segments {
            2 => {
                decoder.insert(*p, 1);
                encoder.insert(1, *p);
            },
            3 => {
                decoder.insert(*p, 7);
                encoder.insert(7, *p);
            },
            4 => {
                decoder.insert(*p, 4);
                encoder.insert(4, *p);
            },
            7 => {
                decoder.insert(*p, 8);
                encoder.insert(8, *p);
            },
            _ => {
                if let Some(pattern_vec) = num_segments_map.get_mut(&num_segments) {
                    pattern_vec.push(*p);
                } else {
                    num_segments_map.insert(num_segments, vec![*p]);
                }
            },
        }
    }

    // 5 segments
    let mut five_segments = num_segments_map.remove(&5).unwrap();
    // find 2
    let p4 = *encoder.get(&4).unwrap();
    five_segments.retain(|p| {
        let is_p2 = p | p4 == 127;
        if is_p2 {
            decoder.insert(*p, 2);
            encoder.insert(2, *p);
        }
        !is_p2
    });
    // find 5
    let p2 = *encoder.get(&2).unwrap();
    five_segments.retain(|p| {
        let is_p5 = p | p2 == 127;
        if is_p5 {
            decoder.insert(*p, 5);
            encoder.insert(5, *p);
        }
        !is_p5
    });
    // leaves 3
    decoder.insert(five_segments.remove(0), 3);

    // 6 segments
    let mut six_segments = num_segments_map.remove(&6).unwrap();
    // find 0
    let p5 = *encoder.get(&5).unwrap();
    six_segments.retain(|p| {
        let is_p0 = p | p5 == 127;
        if is_p0 {
            decoder.insert(*p, 0);
            encoder.insert(0, *p);
        }
        !is_p0
    });
    // find 6
    let p1 = *encoder.get(&1).unwrap();
    six_segments.retain(|p| {
        let is_p6 = p | p1 == 127;
        if is_p6 {
            decoder.insert(*p, 6);
            encoder.insert(6, *p);
        }
        !is_p6
    });
    // leaves 9
    decoder.insert(six_segments.remove(0), 9);

    decoder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let lines = parser(path).unwrap();
        let res = solve(true, &lines);
        assert_eq!(26, res);
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input.txt");
        let lines = parser(path).unwrap();
        let res = solve(false, &lines);
        assert_eq!(61229, res);
    }
}

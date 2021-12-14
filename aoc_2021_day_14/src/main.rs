use std::collections::HashMap;

use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let (template, pairs, initial_pairs, descendents_map) = parse(path).unwrap();

    let res_1 = solve(&template, &pairs, initial_pairs.clone(), &descendents_map, 10);
    let res_2 = solve(&template, &pairs, initial_pairs, &descendents_map, 40);

    println!("*-*-*-*-*- Day 08 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}

/// Returns mappings from a pair to the index of the character it inserts and to the two descendent
/// pairs it becomes.
/// e.g. "CH" -> B = 1
///      "CH" -> ("CB", BH")
fn parse(path: &Path) -> io::Result<(String, Vec<String>, Vec<u64>, HashMap<usize, (usize, usize)>)> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let mut lines_iter = buf.lines().flatten();

    let template = lines_iter.next().unwrap();

    // skip blank line.
    lines_iter.next();

    let mut pair_index_map = HashMap::new();

    let mut pairs = vec![];
    let mut descendents = vec![];

    for (i, line) in lines_iter.enumerate() {
        let (pair, insert) = line.split_once(" -> ").unwrap();

        let pair_chars = pair.chars().collect::<Vec<char>>();
        let insert_char = insert.chars().next().unwrap();

        let left_descendent = [pair_chars[0], insert_char].iter().collect::<String>();
        let right_descendent = [insert_char, pair_chars[1]].iter().collect::<String>();

        pair_index_map.insert(pair.to_string(), i);
        pairs.push(pair.to_string());

        descendents.push((left_descendent, right_descendent));

    }

    let mut descendents_map: HashMap<usize, (usize, usize)> = HashMap::new();

    for (i, (left_str, right_str)) in descendents.iter().enumerate() {
        let left_idx = pair_index_map.get(left_str).unwrap();
        let right_idx = pair_index_map.get(right_str).unwrap();
        descendents_map.insert(i, (*left_idx, *right_idx));
    }

    let initial_freq = template.chars()
        .collect::<Vec<char>>()
        .windows(2)
        .fold(vec![0; pair_index_map.len()], |mut acc, pair| {
            let pair_str = pair.iter().collect::<String>();
            let pair_idx = pair_index_map.get(&pair_str).unwrap();
            acc[*pair_idx] += 1;
            acc
        });


    Ok((template, pairs, initial_freq, descendents_map))
}

fn solve(
    template: &str,
    pairs: &Vec<String>,
    initial_pair_freq: Vec<u64>,
    descendents_map: &HashMap<usize, (usize, usize)>,
    steps: u32,
) -> u64 {
    let mut pair_freq = initial_pair_freq;

    for _ in 0..steps {
        pair_freq = pair_freq.iter().enumerate().fold(vec![0; pair_freq.len()], |mut acc, (i, freq)| {
            if *freq == 0 { return acc; }
            let (left, right) = descendents_map.get(&i).unwrap();
            acc[*left] += freq;
            acc[*right] += freq;
            acc
        });
    }

    let mut char_freq = pair_freq.iter()
        .enumerate()
        .fold(vec![0; 26], |mut acc, (i, freq)| {
            let chars = pairs[i].chars().collect::<Vec<char>>();
            acc[chars[0] as usize - 'A' as usize] += freq;
            acc[chars[1] as usize - 'A' as usize] += freq;
            acc
        });

    for freq in char_freq.iter_mut() {
        *freq /= 2;
    }

    // add first and last char
    let template_chars = template.chars().collect::<Vec<char>>();
    char_freq[template_chars[0] as usize - 'A' as usize] += 1;
    char_freq[*template_chars.last().unwrap() as usize - 'A' as usize] += 1;

    *char_freq.iter().max().unwrap() - *char_freq.iter().filter(|&&n| n != 0).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let (template, pairs, initial_pairs, descendents_map) = parse(path).unwrap();
        let res = solve(&template, &pairs, initial_pairs, &descendents_map, 10);
        assert_eq!(1588, res);
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input.txt");
        let (template, pairs, initial_pairs, descendents_map) = parse(path).unwrap();
        let res = solve(&template, &pairs, initial_pairs, &descendents_map, 40);
        assert_eq!(2188189693529, res);
    }
}

use std::cmp;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let (input, bit_width) = parser(path).unwrap();

    let res_1 = solve_part_1(&input, bit_width as usize);
    let res_2 = solve_part_2(&input, bit_width);

    println!("*-*-*-*-*- Day 02 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}


fn parser(path: &Path) -> io::Result<(Vec<u32>, u8)> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let strings = buf.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let bit_width = strings[0].len() as u8;
    let values = strings.iter()
        .map(|s| {
            u32::from_str_radix(&s, 2).expect("Error parsing string into int")
        })
        .collect();


    Ok((values, bit_width))
}

fn solve_part_1(input: &Vec<u32>, bit_width: usize) -> u32 {
    let threshold = input.len() as u32 / 2;

    let mut freq_vec = vec![0; bit_width];

    for value in input.iter() {
        for i in 0..bit_width {
            if value & 1 << i != 0 {
                freq_vec[i] += 1;
            }
        }
    }

    let (mut gamma, mut epsilon) = (0, 0);

    for (i, freq) in freq_vec.into_iter().enumerate() {
        if freq > threshold {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }

    gamma * epsilon
}

fn solve_part_2(input: &Vec<u32>, bit_width: u8) -> u32 {
    // first split
    let (p0, p1): (Vec<u32>, Vec<u32>) = input.iter().partition(|&val| {
        val & 0b1 << bit_width - 1 == 0
    });

    let (mut gamma_vec, mut epsilon_vec) = if p0.len() > p1.len() {
        (p0, p1)
    } else {
        (p1, p0)
    };

    // find gamma
    for i in (0..bit_width - 1).rev() {
        if gamma_vec.len() == 1 {
            break;
        }

        let (p0, p1): (Vec<u32>, Vec<u32>) = gamma_vec.iter()
            .partition(|&val| { val & 0b1 << i == 0 });

        gamma_vec = cmp::max_by(p0, p1, |p0, p1| p0.len().cmp(&p1.len()));
    }
    let gamma = gamma_vec[0];

    // find epsilon
    for i in (0..bit_width - 1).rev() {
        if epsilon_vec.len() == 1 {
            break;
        }

        let (p0, p1): (Vec<u32>, Vec<u32>) = epsilon_vec.iter().partition(|&val| {
            val & 0b1 << i == 0
        });

        epsilon_vec = if p0.len() > p1.len() {
            if p1.is_empty() {
                vec![*p0.last().unwrap()]
            } else {
                p1
            }
        } else {
            if p0.is_empty() {
                vec![*p1.last().unwrap()]
            } else {
                p0
            }
        };
    }
    let epsilon = epsilon_vec[0];

    gamma * epsilon
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let (input, bit_width) = parser(path).unwrap();
        let res = solve_part_1(&input, bit_width as usize);
        assert_eq!(198, res);
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input.txt");
        let (input, bit_width) = parser(path).unwrap();
        let res = solve_part_2(&input, bit_width);
        assert_eq!(230, res);
    }
}

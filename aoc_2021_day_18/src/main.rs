use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::Sum;
use std::ops::Add;
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let input = parse(path).unwrap();

    let res_1 = solve_1(input.clone());
    let res_2 = solve_2(input);

    println!("*-*-*-*-*- Day 18 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}

fn parse(path: &Path) -> io::Result<Vec<VecTree>> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let result = buf.lines()
        .map(|l| VecTree::from_str(&l.unwrap()))
        .collect();

    Ok(result)
}

fn solve_1(input: Vec<VecTree>) -> u32 {
    let sum: VecTree = input.into_iter().sum();

    sum.magnitude()
}

fn solve_2(input: Vec<VecTree>) -> u32 {
    let mut result = 0;

    for a in input.iter() {
        for b in input.iter() {
            if a == b { continue; }

            let sum = a.clone() + b.clone();

            result = result.max(sum.magnitude());
        }
    }

    result
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct VecTree {
    vals: Vec<u32>,
    depths: Vec<u32>,
}

impl VecTree {
    fn from_str(s: &str) -> Self {
        let mut vals = vec![];
        let mut depths = vec![];
        let mut depth = 0;

        for c in s.chars() {
            match c {
                '[' => depth += 1,
                ']' => depth -= 1,
                ',' => (),
                _ => {
                    vals.push(c.to_digit(10).unwrap());
                    depths.push(depth);
                },
            }
        }

        Self { vals, depths }
    }

    fn reduce(&mut self) {
        for i in 0..self.vals.len() {
            if self.depths[i] == 5 {
                self.explode(i);
                self.reduce();
                break;
            }
        }
        for i in 0..self.vals.len() {
            if self.vals[i] > 9 {
                self.split(i);
                self.reduce();
                break;
            }
        }
    }

    fn split(&mut self, idx: usize) {
        let l = self.vals[idx] / 2;
        let r = self.vals[idx] - l;

        self.vals[idx] = l;
        self.depths[idx] += 1;

        self.vals.insert(idx + 1, r);
        self.depths.insert(idx + 1, self.depths[idx]);
    }

    fn explode(&mut self, idx: usize) {
        let (l, r) = (self.vals[idx], self.vals[idx + 1]);

        if idx > 0 {
            self.vals[idx - 1] += l;
        }
        if idx < self.vals.len() - 2 {
            self.vals[idx + 2] += r;
        }

        self.vals[idx + 1] = 0;
        self.depths[idx + 1] -= 1;

        self.vals.remove(idx);
        self.depths.remove(idx);
    }

    fn magnitude(&self) -> u32 {
        let mut vals = self.vals.clone();
        let mut depths = self.depths.clone();
        let mut merge_depth = 4;

        while vals.len() > 1 {
            let mut new_vals = vec![];
            let mut new_depths = vec![];
            let mut i = 0;

            while i < vals.len() {
                if depths[i] == merge_depth {
                    new_vals.push(vals[i] * 3 + vals[i + 1] * 2);
                    new_depths.push(depths[i] - 1);
                    i += 2;
                    continue;
                }
                new_vals.push(vals[i]);
                new_depths.push(depths[i]);
                i += 1;
            }

            vals = new_vals;
            depths = new_depths;
            merge_depth -= 1;
        }

        vals[0]
    }
}

impl Add for VecTree {
    type Output = Self;

    fn add(mut self, other: Self) -> Self {
        self.vals.extend(other.vals);
        self.depths.extend(other.depths);

        for d in self.depths.iter_mut() {
            *d += 1;
        }

        self.reduce();

        self
    }
}

impl Sum for VecTree {
    fn sum<I>(mut iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        let mut sum = iter.next().unwrap();

        for vt in iter {
            sum = sum + vt;
        }

        sum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnitude_calc() {
        let num_1 = VecTree::from_str("[[1,2],[[3,4],5]]");
        let num_2 = VecTree::from_str("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]");
        let num_3 = VecTree::from_str("[[[[1,1],[2,2]],[3,3]],[4,4]]");
        let num_4 = VecTree::from_str("[[[[3,0],[5,3]],[4,4]],[5,5]]");
        let num_5 = VecTree::from_str("[[[[5,0],[7,4]],[5,5]],[6,6]]");
        let num_6 = VecTree::from_str("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
        let num_7 = VecTree::from_str("[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]");
        let num_8 = VecTree::from_str("[[[5,[2,8]],4],[5,[[9,9],0]]]");

        assert_eq!(143, num_1.magnitude());
        assert_eq!(1384, num_2.magnitude());
        assert_eq!(445, num_3.magnitude());
        assert_eq!(791, num_4.magnitude());
        assert_eq!(1137, num_5.magnitude());
        assert_eq!(3488, num_6.magnitude());
        assert_eq!(1636, num_7.magnitude());
        assert_eq!(1125, num_8.magnitude());
    }

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let input = parse(path).unwrap();
        let res = solve_1(input);
        assert_eq!(4140, res);
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input.txt");
        let input = parse(path).unwrap();
        let res = solve_2(input);
        assert_eq!(3993, res);
    }
}

use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = Path::new("input.txt");
    let mut img = parse(path).unwrap();

    for _ in 0..2 {
        img.enhance();
    }

    println!("*-*-*-*-*- Day 20 -*-*-*-*-*\n");
    println!("Answer to part 1: Total lit pixels = {}", img.total_lit_pixels());

    for _ in 0..48 {
        img.enhance();
    }

    println!("Answer to part 2: Total lit pixels = {}", img.total_lit_pixels());
}

fn parse(path: &Path) -> io::Result<Image> {
    let file = File::open(path)?;
    let buf = BufReader::new(file);

    let mut lines_iter = buf.lines().flatten();

    let algo_string = lines_iter.next().unwrap();

    // skip blank line.
    lines_iter.next();

    let image_lines = lines_iter.collect::<Vec<String>>();

    let img = Image::new(image_lines, &algo_string);

    Ok(img)
}

struct Image {
    light_pixels: HashSet<[i32; 2]>,
    enhancement_algo: Vec<bool>,
    min: [i32; 2],
    max: [i32; 2],
    pass: usize,
}

impl Image {
    fn new(image_strings: Vec<String>, algo_string: &str) -> Self {
        let enhancement_algo = algo_string
            .chars()
            .map(|c| {
                match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("invalid input char"),
                }
            })
            .collect::<Vec<bool>>();

        let min = [0, 0];
        let max = [image_strings.len() as i32, image_strings[0].len() as i32];

        let mut light_pixels: HashSet<[i32; 2]> = HashSet::new();

        for (row, s) in image_strings.into_iter().enumerate() {
            for (col, c) in s.chars().enumerate() {
                if c == '#' {
                    light_pixels.insert([row as i32, col as i32]);
                }
            }
        }

        Self { light_pixels, enhancement_algo, min, max, pass: 0 }
    }

    fn total_lit_pixels(&self) -> usize {
        self.light_pixels.len()
    }

    fn enhance(&mut self) {
        // expand boundary.
        if self.pass % 2 == 0 {
            self.min[0] -= 3;
            self.min[1] -= 3;
            self.max[0] += 3;
            self.max[1] += 3;
        } else {
            self.min[0] += 1;
            self.min[1] += 1;
            self.max[0] -= 1;
            self.max[1] -= 1;
        }

        let mut update: Vec<([i32; 2], bool)> = vec![];

        for row in self.min[0]..self.max[0] {
            for col in self.min[1]..self.max[1] {
                update.push(([row, col], self.get_enhanced_pixel(row, col)));
            }
        }

        // apply update.
        for ([row, col], light) in update {
            if light {
                self.light_pixels.insert([row, col]);
            } else {
                self.light_pixels.remove(&[row, col]);
            }
        }

        if self.pass % 2 != 0 {
            // remove elements just outside border.
            for r in &[self.min[0] - 1, self.max[0]] {
                for c in (self.min[1] - 1)..(self.max[1] + 1) {
                    self.light_pixels.remove(&[*r, c]);
                }
            }
            for c in &[self.min[1] - 1, self.max[1]] {
                for r in (self.min[0] - 1)..(self.max[0] + 1) {
                    self.light_pixels.remove(&[r, *c]);
                }
            }
        }
        self.pass += 1;
    }

    fn get_enhanced_pixel(&self, row: i32, col: i32) -> bool {
        let idx = self.pixel_to_binary(row, col);

        self.enhancement_algo[idx]
    }

    fn pixel_to_binary(&self, row: i32, col: i32) -> usize {
        let mut result = 0;
        let mut idx = 8;

        for r in (row - 1)..=(row + 1) {
            for c in (col - 1)..=(col + 1) {
                if self.light_pixels.contains(&[r as i32, c as i32]) {
                    result += 1 << idx;
                }
                idx -= 1;
            }
        }

        result
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in (self.min[0] - 3)..(self.max[0] + 3) {
            for col in (self.min[1] - 3)..(self.max[1] + 3) {
                let pixel = if self.light_pixels.contains(&[row, col]) {
                    "#"
                } else {
                    "."
                };

                write!(f, "{}", pixel)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALGO: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..##\
                        #..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###\
                        .######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#.\
                        .#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#.....\
                        .#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#..\
                        ...####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.....\
                        ..##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#";

    #[test]
    fn test_pixel_enhancement() {
        let img_strings = vec![
            String::from("#..#."),
            String::from("#...."),
            String::from("##..#"),
            String::from("..#.."),
            String::from("..###"),
        ];

        let img = Image::new(img_strings, ALGO);

        assert_eq!(34, img.pixel_to_binary(2, 2));
        assert!(img.get_enhanced_pixel(2, 2));
    }

    #[test]
    fn test_part_1() {
        let path = Path::new("test_input.txt");
        let mut img = parse(path).unwrap();
        println!("ORIGINAL:\n{}", img);
        for i in 0..2 {
            img.enhance();
            println!("PASS {}:\n{}", i, img);
        }
        assert_eq!(35, img.total_lit_pixels());
    }

    #[test]
    fn test_part_2() {
        let path = Path::new("test_input.txt");
        let mut img = parse(path).unwrap();
        println!("ORIGINAL:\n{}", img);
        for i in 0..50 {
            img.enhance();
            println!("PASS {}:\n{}", i, img);
        }
        assert_eq!(3351, img.total_lit_pixels());
    }
}

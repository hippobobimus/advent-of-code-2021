use std::cmp;
use std::collections::HashMap;

fn main() {
    let res_1 = play_1(1, 2);
    let res_2 = play_2(1, 2);

    println!("*-*-*-*-*- Day 21 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);

}

fn play_2(p0_start: u8, p1_start: u8) -> u64 {
    fn helper(p0_score: u8, p1_score: u8, p0_pos: u8, p1_pos: u8, winners_cache: &mut HashMap<u32, (u64, u64)>) -> (u64, u64) {
        if p1_score >= 21 {
            return (0, 1);
        }

        let key = u32::from_be_bytes([p0_score, p1_score, p0_pos, p1_pos]);
        if let Some(result) = winners_cache.get(&key) {
            return *result;
        }

        let mut result = (0, 0);

        for (roll, freq) in (3..=9).zip(&[1, 3, 6, 7, 6, 3, 1]) {
            let new_p0_pos = (p0_pos + roll - 1) % 10 + 1;
            let new_p0_score = p0_score + new_p0_pos;

            let (p1_winners, p0_winners) =
                helper(p1_score, new_p0_score, p1_pos, new_p0_pos, winners_cache);

            result.0 += freq * p0_winners;
            result.1 += freq * p1_winners;
        }

        winners_cache.insert(key, result);

        result
    }

    let mut winners_cache: HashMap<u32, (u64, u64)> = HashMap::new();

    let (p0_wins, p1_wins) = helper(0, 0, p0_start, p1_start, &mut winners_cache);

    cmp::max(p0_wins, p1_wins)
}

fn play_1(p0_start: usize, p1_start: usize) -> usize {
    let mut score_0 = 0;
    let mut score_1 = 0;

    let mut pos_0 = p0_start;
    let mut pos_1 = p1_start;

    let mut d = Dice::new();

    loop {
        pos_0 = (pos_0 + d.roll() - 1) % 10 + 1;
        score_0 += pos_0;

        if score_0 >= 1000 {
            return score_1 * d.rolls;
        }

        pos_1 = (pos_1 + d.roll() - 1) % 10 + 1;
        score_1 += pos_1;

        if score_1 >= 1000 {
            return score_0 * d.rolls;
        }
    }
}

struct Dice {
    val: usize,
    rolls: usize,
}

impl Dice {
    fn new() -> Self {
        Self { val: 1, rolls: 0 }
    }

    fn roll(&mut self) -> usize {
        let mut score = 0;

        for _ in 0..3 {
            score += self.val;
            self.val = self.val % 100 + 1;
            self.rolls += 1;
        }

        score
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let res = play_1(4, 8);
        assert_eq!(739785, res);
    }

    #[test]
    fn test_part_2() {
        let res = play_2(4, 8);
        assert_eq!(444356092776315, res);
    }
}

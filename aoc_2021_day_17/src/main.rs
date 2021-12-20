fn main() {
    let target_area = ((185, 221), (-122, -74));

    let res_1 = highest_y_position(target_area);
    let res_2 = number_of_accurate_initial_velocities(target_area);

    println!("*-*-*-*-*- Day 17 -*-*-*-*-*\n");
    println!("Answer to part 1: {}", res_1);
    println!("Answer to part 2: {}", res_2);
}

fn highest_y_position(target_area: ((isize, isize), (isize, isize))) -> isize {
    let ((_, _), (min_y, _)) = target_area;

    (min_y.abs() - 1) * min_y.abs() / 2
}

fn number_of_accurate_initial_velocities(target_area: ((isize, isize), (isize, isize))) -> usize {
    let mut result = 0;

    let ((target_min_x, target_max_x), (target_min_y, _)) = target_area;

    let min_y_vel = target_min_y;
    let max_y_vel = target_min_y.abs() - 1;
    let min_x_vel = target_min_x.abs() / target_min_x;
    let max_x_vel = target_max_x;

    for initial_x_vel in min_x_vel..=max_x_vel {
        for initial_y_vel in min_y_vel..=max_y_vel {
            if is_hit(initial_x_vel, initial_y_vel, target_area) {
                result += 1;
            }
        }
    }

    result
}

fn is_hit(mut x_vel: isize, mut y_vel: isize, target_area: ((isize, isize), (isize, isize))) -> bool {
    let ((target_min_x, target_max_x), (target_min_y, target_max_y)) = target_area;

    let (mut x, mut y) = (0, 0);
    let target_range_x = target_min_x..=target_max_x;
    let target_range_y = target_min_y..=target_max_y;

    while x <= target_max_x && y >= target_min_y {
        if target_range_x.contains(&x) && target_range_y.contains(&y) {
            return true;
        }

        x += x_vel;
        y += y_vel;
        if x_vel != 0 {
            if x_vel < 0 {
                x_vel += 1;
            } else {
                x_vel -= 1;
            }
        }
        y_vel -= 1;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET_AREA: ((isize, isize), (isize, isize)) = ((20, 30), (-10, -5));

    #[test]
    fn test_part_1() {
        let res = highest_y_position(TARGET_AREA);
        assert_eq!(45, res);
    }

    #[test]
    fn test_part_2() {
        let res = number_of_accurate_initial_velocities(TARGET_AREA);
        assert_eq!(112, res);
    }
}

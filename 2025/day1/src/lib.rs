mod input;

pub mod part1 {
    pub fn solve(input: &str) -> u32 {
        let mut pswd: u32 = 0;
        let mut dial: i32 = 50;

        for line in input.lines() {
            let rotation = line[1..].parse::<i32>().unwrap();
            dial += rotation
                * match line.starts_with('R') {
                    true => 1,
                    false => -1,
                };
            dial %= 100;
            if dial == 0 {
                pswd += 1;
            }
        }
        pswd
    }
}

pub mod part2 {
    pub fn solve(input: &str) -> u32 {
        let mut pswd: u32 = 0;
        let mut dial: i32 = 50;

        for line in input.lines() {
            let rotation = line[1..].parse::<i32>().unwrap();

            let n_full_rotations = (rotation / 100) as u32;
            pswd += n_full_rotations;

            let eventual_rotation = rotation % 100;
            if eventual_rotation == 0 {
                continue;
            }

            let new_dial = dial
                + eventual_rotation
                    * match line.starts_with('R') {
                        true => 1,
                        false => -1,
                    };

            if dial != 0 && (new_dial <= 0 || new_dial >= 100) {
                pswd += 1
            }

            dial = ((new_dial % 100) + 100) % 100; // to ensure positive modulo of negative numbers
        }
        pswd
    }
}

#[cfg(test)]
mod tests {
    use super::{input, part1, part2};

    #[test]
    fn sample_test1() {
        assert_eq!(part1::solve(input::SAMPLE_CASE), 3 as u32)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1::solve(input::CASE1), 1089 as u32);
    }

    #[test]
    fn sample_test2() {
        assert_eq!(part2::solve(input::SAMPLE_CASE), 6 as u32)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2::solve(input::CASE2), 6530 as u32);
    }
}

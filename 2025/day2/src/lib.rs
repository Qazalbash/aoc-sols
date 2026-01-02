pub mod part1 {

    fn repeated_twice(n: usize) -> bool {
        let n_str = n.to_string();
        let length = n_str.len();
        let half_length = length >> 1;
        match length & 0b1 == 1 {
            true => false,
            false => n_str[0..half_length] == n_str[half_length..length],
        }
    }

    pub fn solve(input: &[(usize, usize)]) -> usize {
        let mut sum_of_invalid_ids: usize = 0_usize;
        for ab in input {
            let (a, b): (usize, usize) = *ab;
            for n in a..=b {
                if repeated_twice(n) {
                    sum_of_invalid_ids += n;
                }
            }
        }
        sum_of_invalid_ids
    }
}

pub mod part2 {

    fn next_chunk_size(chunk_size: usize, n: usize) -> Option<usize> {
        if chunk_size == 1 {
            return None;
        }
        let mut new_chunk_size = chunk_size - 1;
        while !n.is_multiple_of(new_chunk_size) {
            new_chunk_size -= 1;
        }
        Some(new_chunk_size)
    }

    fn is_repeating(n_str: &str, chunk_size: usize) -> bool {
        let length = n_str.len();
        let zero_chunk = &n_str[0..chunk_size];
        let mut idx: usize = chunk_size;
        while idx < length {
            if idx + chunk_size > length {
                return false;
            }
            if *zero_chunk != n_str[idx..(idx + chunk_size)] {
                return false;
            }
            idx += chunk_size;
        }
        true
    }

    fn repeated_at_least_twice(n: usize) -> bool {
        let n_str = n.to_string();
        let length = n_str.len();
        if length == 1 {
            return false;
        }
        let mut chunk_size = length >> 1;
        loop {
            if is_repeating(&n_str, chunk_size) {
                return true;
            }
            if let Some(maybe_next_chunk_size) = next_chunk_size(chunk_size, length) {
                chunk_size = maybe_next_chunk_size;
            } else {
                break;
            }
        }
        false
    }

    pub fn solve(input: &[(usize, usize)]) -> usize {
        let mut sum_of_invalid_ids: usize = 0_usize;
        for ab in input {
            let (a, b): (usize, usize) = *ab;
            for n in a..=b {
                if repeated_at_least_twice(n) {
                    sum_of_invalid_ids += n;
                }
            }
        }
        sum_of_invalid_ids
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};

    const SAMPLE_CASE: [(usize, usize); 11] = [
        (11, 22),
        (1188511880, 1188511890),
        (1698522, 1698528),
        (2121212118, 2121212124),
        (222220, 222224),
        (38593856, 38593862),
        (446443, 446449),
        (565653, 565659),
        (824824821, 824824827),
        (95, 115),
        (998, 1012),
    ];

    const TEST_CASE: [(usize, usize); 29] = [
        (1056, 1771),
        (1919108745, 1919268183),
        (1922, 9652),
        (197, 407),
        (2, 12),
        (20124, 44038),
        (206885, 246173),
        (25, 57),
        (262128, 339499),
        (38342224, 38444598),
        (408, 1000),
        (424942, 446151),
        (4429019, 4570680),
        (4603696, 4688732),
        (483824, 534754),
        (48414903, 48538379),
        (5261723647, 5261785283),
        (557930, 573266),
        (58, 134),
        (60035, 128980),
        (648613, 673853),
        (6812551522, 6812585188),
        (714164, 782292),
        (75712519, 75792205),
        (857821365, 857927915),
        (881574, 897488),
        (92856246, 93001520),
        (9648251, 9913729),
        (9944818, 10047126),
    ];

    #[test]
    fn sample_test1() {
        assert_eq!(part1::solve(&SAMPLE_CASE), 1227775554_usize)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1::solve(&TEST_CASE), 19128774598_usize);
    }

    #[test]
    fn sample_test2() {
        assert_eq!(part2::solve(&SAMPLE_CASE), 4174379265_usize)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2::solve(&TEST_CASE), 21932258645_usize);
    }
}

mod input;

pub mod part1 {
    use crate::input::DataBase;

    pub fn solve(database: DataBase) -> u64 {
        let ranges = database.ranges;
        let mut count_fresh: u64 = 0_u64;

        database.ingredients.iter().for_each(|ingredient| {
            if ranges
                .iter()
                .any(|range| (range.0 <= *ingredient) && (*ingredient <= range.1))
            {
                count_fresh += 1;
            }
        });

        count_fresh
    }
}

pub mod part2 {
    use crate::input::{DataBase, RangeT};

    fn merge_intervals(ranges: &[RangeT]) -> Vec<RangeT> {
        let mut sorted_ranges = ranges.to_owned();
        sorted_ranges.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut new_ranges: Vec<RangeT> = vec![sorted_ranges[0]];
        let mut n: usize = 1_usize;

        for range in sorted_ranges.iter() {
            let (start, end) = *range;
            let (last_start, last_end) = new_ranges[n - 1];
            if start <= last_end {
                new_ranges[n - 1] = (last_start, std::cmp::max(last_end, end));
            } else {
                new_ranges.push(*range);
                n += 1;
            }
        }

        new_ranges
    }

    pub fn solve(database: DataBase) -> u64 {
        let ranges = merge_intervals(&database.ranges);
        let mut count_fresh: u64 = 0_u64;
        ranges
            .iter()
            .for_each(|range| count_fresh += range.1 - range.0 + 1);
        count_fresh
    }
}

#[cfg(test)]
mod tests {
    use super::{input, part1, part2};

    #[test]
    fn sample_case1() {
        assert_eq!(part1::solve(input::sample_case()), 3_u64)
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1::solve(input::test_case()), 558_u64);
    }

    #[test]
    fn sample_test2() {
        assert_eq!(part2::solve(input::sample_case()), 14_u64)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2::solve(input::test_case()), 344813017450467_u64);
    }
}

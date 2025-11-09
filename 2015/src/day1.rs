use std::{hint::unreachable_unchecked, simd::Simd};

use aoc_utils::simd::simd_eq_diff_count;

pub const BENCH_GROUPS: [&str; 2] = ["2015d1p1", "2015d1p2"];
pub const INPUT: &str = include_str!("../../input/2015/day1.txt");
pub const ANSWER_1: i32 = 74;
pub const ANSWER_2: usize = 1795;

pub fn part1_baseline(input: &str) -> i32 {
    input.chars().fold(0, |acc, c| {
        acc + match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        }
    })
}

pub fn part1(input: &str) -> i32 {
    let (chunks, remainder) = input.as_bytes().as_chunks::<64>();
    let acc = chunks.iter().fold(0_i32, |acc, c| {
        let simd = Simd::from_array(*c);
        acc + simd_eq_diff_count(simd, b'(')
    });

    remainder.iter().fold(acc, |acc, b| unsafe {
        match b {
            b'(' => acc + 1,
            b')' => acc - 1,
            _ => unreachable_unchecked(),
        }
    })
}

pub fn part2_baseline(input: &str) -> usize {
    let mut acc = 0;
    for (i, c) in input.char_indices() {
        acc += match c {
            '(' => 1,
            ')' => -1,
            _ => 0,
        };

        if acc < 0 {
            return i + 1;
        }
    }

    0
}

pub fn part2(input: &str) -> usize {
    let mut acc = 0;
    let mut i = 0;
    let (chunks, remainder) = input.as_bytes().as_chunks::<64>();
    for chunk in chunks {
        let simd = Simd::from_array(*chunk);
        let step = simd_eq_diff_count(simd, b'(');
        acc += step;

        // If this chunk decremented acc below 0: Undo step and check single elements, then return early
        if acc < 0 {
            acc -= step;
            for b in chunk {
                acc += match b {
                    b'(' => 1,
                    b')' => -1,
                    _ => unsafe { unreachable_unchecked() },
                };

                i += 1;

                if acc < 0 {
                    return i;
                }
            }
        }

        i += 64;
    }

    // Check remainder
    for b in remainder {
        acc += match b {
            b'(' => 1,
            b')' => -1,
            _ => unsafe { unreachable_unchecked() },
        };

        i += 1;

        if acc < 0 {
            break;
        }
    }

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT_0: &str = r"(())";
        const INPUT_3: &str = r"))(((((";
        const INPUT_NEG_1: &str = r"())";
        const INPUT_NEG_3: &str = r")())())";

        assert_eq!(part1_baseline(INPUT_0), 0);
        assert_eq!(part1_baseline(INPUT_3), 3);
        assert_eq!(part1_baseline(INPUT_NEG_1), -1);
        assert_eq!(part1_baseline(INPUT_NEG_3), -3);
    }

    #[test]
    fn test_part2() {
        const INPUT_1: &str = r")";
        const INPUT_5: &str = r"()())";

        assert_eq!(part2_baseline(INPUT_1), 1);
        assert_eq!(part2_baseline(INPUT_5), 5);
    }

    #[test]
    fn answer_part1() {
        eprintln!("Answer: {}", part1_baseline(INPUT));
    }

    #[test]
    fn answer_part2() {
        eprintln!("Answer: {}", part2_baseline(INPUT));
    }

    #[test]
    fn equivalence() {
        assert_eq!(part1(INPUT), part1_baseline(INPUT));
        assert_eq!(part2(INPUT), part2_baseline(INPUT));
    }

    #[test]
    fn check_answers() {
        assert_eq!(part1_baseline(INPUT), ANSWER_1);
        assert_eq!(part2_baseline(INPUT), ANSWER_2);
    }
}

pub const BENCH_GROUPS: [&str; 2] = ["2015d2p1", "2015d2p2"];
pub const INPUT: &str = include_str!("../../input/2015/day2.txt");
pub const ANSWER_1: u32 = 1586300;
pub const ANSWER_2: u32 = 3737498;

use std::{
    hint::unreachable_unchecked,
    simd::{Simd, cmp::SimdOrd, num::SimdUint},
};

pub fn part1_baseline(input: &str) -> u32 {
    input.lines().fold(0_u32, |acc, line| {
        let mut dims = line
            .split_terminator('x')
            .map(|d| d.parse::<u32>().unwrap());
        let l = dims.next().unwrap();
        let w = dims.next().unwrap();
        let h = dims.next().unwrap();
        let smallest = (l * w).min(w * h).min(l * h);
        acc + 2 * l * w + 2 * w * h + 2 * h * l + smallest
    })
}

const TWO: Simd<u16, 32> = Simd::splat(2);

fn read_dims(input: &str) -> [[u16; 1024]; 3] {
    let mut dims = [[0; 1024]; 3];

    let mut acc = 0;
    let mut dim = 0;
    let mut line = 0;

    let bytes = input.as_bytes();

    for &b in bytes {
        match b {
            b @ b'0'..=b'9' => {
                let n = (b - 0x30) as u16;
                acc = acc * 10 + n;
            }
            b'x' => {
                dims[dim][line] = acc;
                dim += 1;
                acc = 0;
            }
            b'\n' => {
                dims[dim][line] = acc;
                dim = 0; // Reset dimension
                acc = 0;
                line += 1; // Increment line index
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }

    dims
}

pub fn part1(input: &str) -> u32 {
    let dims = read_dims(input);
    let [dim_0, dim_1, dim_2] = dims;

    let chunk_0 = unsafe { dim_0.as_chunks_unchecked::<32>() };
    let chunk_1 = unsafe { dim_1.as_chunks_unchecked::<32>() };
    let chunk_2 = unsafe { dim_2.as_chunks_unchecked::<32>() };

    let mut result: Simd<u32, 32> = Simd::splat(0);

    for i in 0..1024 / 32 {
        let simd_0: Simd<u16, 32> = Simd::from_array(chunk_0[i]);
        let simd_1: Simd<u16, 32> = Simd::from_array(chunk_1[i]);
        let simd_2: Simd<u16, 32> = Simd::from_array(chunk_2[i]);
        let a = simd_0 * simd_1;
        let b = simd_1 * simd_2;
        let c = simd_0 * simd_2;
        let min = a.simd_min(b).simd_min(c);
        let total = TWO * (a + b + c) + min;
        result += total.cast();
    }

    result.reduce_sum()
}

pub fn part2_baseline(input: &str) -> u32 {
    input.lines().fold(0_u32, |acc, line| {
        let mut dims = line
            .split_terminator('x')
            .map(|d| d.parse::<u32>().unwrap());
        let l = dims.next().unwrap();
        let w = dims.next().unwrap();
        let h = dims.next().unwrap();
        let shortest = (l + w).min(w + h).min(l + h);
        acc + l * w * h + 2 * shortest
    })
}

pub fn part2(input: &str) -> u32 {
    let dims = read_dims(input);
    let [dim_0, dim_1, dim_2] = dims;

    let chunk_0 = unsafe { dim_0.as_chunks_unchecked::<32>() };
    let chunk_1 = unsafe { dim_1.as_chunks_unchecked::<32>() };
    let chunk_2 = unsafe { dim_2.as_chunks_unchecked::<32>() };

    let mut result: Simd<u32, 32> = Simd::splat(0);

    for i in 0..1024 / 32 {
        let simd_0: Simd<u16, 32> = Simd::from_array(chunk_0[i]);
        let simd_1: Simd<u16, 32> = Simd::from_array(chunk_1[i]);
        let simd_2: Simd<u16, 32> = Simd::from_array(chunk_2[i]);
        let a = simd_0 + simd_1;
        let b = simd_1 + simd_2;
        let c = simd_0 + simd_2;

        let min = a.simd_min(b).simd_min(c);
        let total = (simd_0 * simd_1 * simd_2) + TWO * min;

        result += total.cast();
    }

    result.cast::<u32>().reduce_sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT_58: &str = r"2x3x4";
        const INPUT_43: &str = r"1x1x10";

        assert_eq!(part1_baseline(INPUT_58), 58);
        assert_eq!(part1_baseline(INPUT_43), 43);
    }

    #[test]
    fn test_part2() {
        const INPUT_34: &str = r"2x3x4";
        const INPUT_14: &str = r"1x1x10";

        assert_eq!(part2_baseline(INPUT_34), 34);
        assert_eq!(part2_baseline(INPUT_14), 14);
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

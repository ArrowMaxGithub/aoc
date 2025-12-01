use std::hint::unreachable_unchecked;

pub const BENCH_GROUPS: [&str; 2] = ["2025d1p1", "2025d1p2"];
pub const INPUT: &str = include_str!("../../input/2025/day1.txt");
pub const ANSWER_1: i32 = 1074;
pub const ANSWER_2: i32 = 6254;

pub fn part1_baseline(input: &str) -> i32 {
    let (zeros, _) = input
        .lines()
        .fold((0, 50), |(mut zeros, mut current), line| {
            let mut rotation: i32 = line[1..].parse().unwrap();
            if line.as_bytes()[0] == b'L' {
                rotation *= -1
            }

            current += rotation;

            if current % 100 == 0 {
                zeros += 1;
            }

            (zeros, current)
        });

    zeros
}

pub fn part1(input: &str) -> i32 {
    let bytes = input.as_bytes();
    let mut current = 50;
    let mut zeros = 0;
    let mut acc = 0;
    let mut sign = 0;

    for &b in bytes {
        match b {
            b @ b'0'..=b'9' => {
                let n = (b - 0x30) as i16;
                acc = acc * 10 + n;
            }
            b'L' => {
                sign = -1;
            }
            b'R' => {
                sign = 1;
            }
            b'\n' => {
                current += acc * sign;
                acc = 0;
                current %= 100;
                if current == 0 {
                    zeros += 1;
                }
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }

    zeros
}

pub fn part2_baseline(input: &str) -> i32 {
    let (zeros, _) = input
        .lines()
        .fold((0, 50), |(mut zeros, mut current), line| {
            let mut rotation: i32 = line[1..].parse().unwrap();
            if line.as_bytes()[0] == b'L' {
                rotation *= -1
            }

            let previous = current;
            current += rotation;

            let multiple_of_100 = (current / 100).abs();
            zeros += multiple_of_100;

            let flipped_sign = current.signum() * previous.signum();

            if current == 0 || flipped_sign == -1 {
                zeros += 1;
            }

            current %= 100;

            (zeros, current)
        });

    zeros
}

pub fn part2(input: &str) -> i32 {
    let bytes = input.as_bytes();
    let mut current = 50;
    let mut zeros = 0;
    let mut acc = 0;
    let mut sign = 0;

    for &b in bytes {
        match b {
            b @ b'0'..=b'9' => {
                let n = (b - 0x30) as i32;
                acc = acc * 10 + n;
            }
            b'L' => {
                sign = -1;
            }
            b'R' => {
                sign = 1;
            }
            b'\n' => {
                let previous = current;
                current += acc * sign;
                acc = 0;

                let flipped_sign = current.signum() * previous.signum();
                let multiple_of_100 = (current / 100).abs();

                zeros += multiple_of_100;

                if current == 0 || flipped_sign == -1 {
                    zeros += 1;
                }

                current %= 100;
            }
            _ => unsafe { unreachable_unchecked() },
        }
    }

    zeros
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const INPUT_3: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

        assert_eq!(part1_baseline(INPUT_3), 3);
    }

    #[test]
    fn test_part2() {
        const INPUT_6: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";

        assert_eq!(part2_baseline(INPUT_6), 6);
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

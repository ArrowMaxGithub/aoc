use std::simd::{
    Simd,
    cmp::{SimdOrd, SimdPartialEq},
    num::SimdUint,
    ptr::SimdConstPtr,
};

pub const BENCH_GROUPS: [&str; 2] = ["2015d2p1", "2015d2p2"];
pub const INPUT: &str = include_str!("../../input/2015/day2.txt");
pub const ANSWER_1: u32 = 1586300;
pub const ANSWER_2: u32 = 3737498;

const SIMD_0_GATHER: Simd<usize, 32> = {
    Simd::from_array([
        0, 8, 16, 24, 32, 40, 48, 56, 64, 72, 80, 88, 96, 104, 112, 120, 128, 136, 144, 152, 160,
        168, 176, 184, 192, 200, 208, 216, 224, 232, 240, 248,
    ])
};
const SIMD_1_GATHER: Simd<usize, 32> = {
    Simd::from_array([
        1, 9, 17, 25, 33, 41, 49, 57, 65, 73, 81, 89, 97, 105, 113, 121, 129, 137, 145, 153, 161,
        169, 177, 185, 193, 201, 209, 217, 225, 233, 241, 249,
    ])
};
const SIMD_2_GATHER: Simd<usize, 32> = {
    Simd::from_array([
        2, 10, 18, 26, 34, 42, 50, 58, 66, 74, 82, 90, 98, 106, 114, 122, 130, 138, 146, 154, 162,
        170, 178, 186, 194, 202, 210, 218, 226, 234, 242, 250,
    ])
};
const SIMD_3_GATHER: Simd<usize, 32> = {
    Simd::from_array([
        3, 11, 19, 27, 35, 43, 51, 59, 67, 75, 83, 91, 99, 107, 115, 123, 131, 139, 147, 155, 163,
        171, 179, 187, 195, 203, 211, 219, 227, 235, 243, 251,
    ])
};
const SIMD_4_GATHER: Simd<usize, 32> = {
    Simd::from_array([
        4, 12, 20, 28, 36, 44, 52, 60, 68, 76, 84, 92, 100, 108, 116, 124, 132, 140, 148, 156, 164,
        172, 180, 188, 196, 204, 212, 220, 228, 236, 244, 252,
    ])
};
const SIMD_5_GATHER: Simd<usize, 32> = {
    Simd::from_array([
        5, 13, 21, 29, 37, 45, 53, 61, 69, 77, 85, 93, 101, 109, 117, 125, 133, 141, 149, 157, 165,
        173, 181, 189, 197, 205, 213, 221, 229, 237, 245, 253,
    ])
};
const SIMD_6_GATHER: Simd<usize, 32> = {
    Simd::from_array([
        6, 14, 22, 30, 38, 46, 54, 62, 70, 78, 86, 94, 102, 110, 118, 126, 134, 142, 150, 158, 166,
        174, 182, 190, 198, 206, 214, 222, 230, 238, 246, 254,
    ])
};
const SIMD_7_GATHER: Simd<usize, 32> = {
    Simd::from_array([
        7, 15, 23, 31, 39, 47, 55, 63, 71, 79, 87, 95, 103, 111, 119, 127, 135, 143, 151, 159, 167,
        175, 183, 191, 199, 207, 215, 223, 231, 239, 247, 255,
    ])
};

const ZERO: Simd<u8, 32> = Simd::splat(0);
const ONE: Simd<u8, 32> = Simd::splat(1);
const TWO: Simd<u8, 32> = Simd::splat(2);
const TWO_U16: Simd<u16, 32> = Simd::splat(2);
const TEN: Simd<u8, 32> = Simd::splat(10);
const ASCII_X: Simd<u8, 32> = Simd::splat(b'x');
const ASCII_0X30: Simd<u8, 32> = Simd::splat(0x30);

fn read_dims_from_lines_chunk(chunk: &[&[u8]]) -> [Simd<u16, 32>; 3] {
    let mut bytes = [b'x'; 32 * 8];
    for (i, line) in chunk.iter().enumerate() {
        let start = i * 8 + (8 - line.len());
        bytes[start..start + line.len()].copy_from_slice(line);
    }

    // Read columns of 32 bytes
    let simd_0: Simd<u8, 32> =
        unsafe { Simd::gather_ptr(Simd::splat(bytes.as_ptr()).wrapping_add(SIMD_0_GATHER)) };
    let simd_1: Simd<u8, 32> =
        unsafe { Simd::gather_ptr(Simd::splat(bytes.as_ptr()).wrapping_add(SIMD_1_GATHER)) };
    let simd_2: Simd<u8, 32> =
        unsafe { Simd::gather_ptr(Simd::splat(bytes.as_ptr()).wrapping_add(SIMD_2_GATHER)) };
    let simd_3: Simd<u8, 32> =
        unsafe { Simd::gather_ptr(Simd::splat(bytes.as_ptr()).wrapping_add(SIMD_3_GATHER)) };
    let simd_4: Simd<u8, 32> =
        unsafe { Simd::gather_ptr(Simd::splat(bytes.as_ptr()).wrapping_add(SIMD_4_GATHER)) };
    let simd_5: Simd<u8, 32> =
        unsafe { Simd::gather_ptr(Simd::splat(bytes.as_ptr()).wrapping_add(SIMD_5_GATHER)) };
    let simd_6: Simd<u8, 32> =
        unsafe { Simd::gather_ptr(Simd::splat(bytes.as_ptr()).wrapping_add(SIMD_6_GATHER)) };
    let simd_7: Simd<u8, 32> =
        unsafe { Simd::gather_ptr(Simd::splat(bytes.as_ptr()).wrapping_add(SIMD_7_GATHER)) };

    let mut dim_0: Simd<u8, 32> = Simd::splat(0);
    let mut dim_1: Simd<u8, 32> = Simd::splat(0);
    let mut dim_2: Simd<u8, 32> = Simd::splat(0);
    let mut n: Simd<u8, 32> = Simd::splat(0);
    let mut exp: Simd<u8, 32> = Simd::splat(0);

    // Process columns of 32 bytes, starting from the rightmost byte
    for simd in [
        simd_7, simd_6, simd_5, simd_4, simd_3, simd_2, simd_1, simd_0,
    ] {
        // Extract any byte other than b'x'
        let valid_digit = simd.simd_ne(ASCII_X).cast();

        // Convert 0x30..=0x39 to 0..=9 and multiply by 10_pow(n) ('x' will produce garbage)
        let any_digits = (simd - ASCII_0X30).cast::<u8>();
        let digits = valid_digit.select(any_digits, ZERO);
        let digits10 = exp.simd_eq(ONE).select(digits * TEN, digits);

        // Set exp to 0 for any columns with 'x', increment for any valid digit
        exp = valid_digit.select(exp + ONE, ZERO);

        // Increment n for any column with 'x'
        n += valid_digit.select(ZERO, ONE);

        // Add digit to dim_0 where n is 0
        dim_0 += n.simd_eq(ZERO).select(digits10, ZERO);

        // Add digit to dim_1 where n is 1
        dim_1 += n.simd_eq(ONE).select(digits10, ZERO);

        // Add digit to dim_2 where n is 2
        dim_2 += n.simd_eq(TWO).select(digits10, ZERO);
    }

    [dim_0.cast(), dim_1.cast(), dim_2.cast()]
}

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

pub fn part1(input: &str) -> u32 {
    let mut acc = 0;
    let lines: Vec<&[u8]> = input.as_bytes().split(|&b| b == b'\n').collect();

    let (chunks, remainder) = lines.as_chunks::<32>();

    for chunk in chunks {
        part1_process_chunk(&mut acc, chunk);
    }

    // Process remainder, end will be masked off
    part1_process_chunk(&mut acc, remainder);

    acc
}

fn part1_process_chunk(acc: &mut u32, chunk: &[&[u8]]) {
    let [dim_0, dim_1, dim_2] = read_dims_from_lines_chunk(chunk);

    let a = dim_0 * dim_1;
    let b = dim_1 * dim_2;
    let c = dim_0 * dim_2;

    let min = a.simd_min(b).simd_min(c);
    let total = TWO_U16 * (a + b + c) + min;

    *acc += total.cast::<u32>().reduce_sum()
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
    let mut acc = 0;
    let lines: Vec<&[u8]> = input.as_bytes().split(|&b| b == b'\n').collect();
    let (chunks, remainder) = lines.as_chunks::<32>();

    for chunk in chunks {
        part2_process_chunk(&mut acc, chunk);
    }

    // Process remainder, end will be masked off
    part2_process_chunk(&mut acc, remainder);

    acc
}

fn part2_process_chunk(acc: &mut u32, chunk: &[&[u8]]) {
    let [dim_0, dim_1, dim_2] = read_dims_from_lines_chunk(chunk);

    let a = dim_0 + dim_1;
    let b = dim_1 + dim_2;
    let c = dim_0 + dim_2;

    let min = a.simd_min(b).simd_min(c);
    let total = dim_0 * dim_1 * dim_2 + TWO_U16 * min;

    *acc += total.cast::<u32>().reduce_sum()
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

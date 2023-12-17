struct ParsedData {
    number_pairs: Vec<(u32, u32)>,
}

fn parse(input: &str) -> ParsedData {
    let number_pairs = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let mut digits = line
                .chars()
                .filter(|c| c.is_ascii_digit())
                .map(|d| d.to_digit(10).unwrap());

            let first: u32 = digits
                .next()
                .unwrap_or_else(|| panic!("no digit found in line {index:?}: {line:?}"));
            let last = digits.last().unwrap_or(first);

            (first, last)
        })
        .collect();

    ParsedData { number_pairs }
}

fn solve(data: ParsedData) -> u32 {
    data.number_pairs
        .iter()
        .map(|(first, last)| first * 10 + last)
        .sum()
}

#[test]
fn test() {
    let input = include_str!("step_1.test");

    let data = parse(input);
    let sum = solve(data);

    assert_eq!(sum, 142);
}

#[test]
fn prod() {
    let input = include_str!("step_1.prod");

    let data = parse(input);
    let sum = solve(data);

    println!("sum: {sum}");
}

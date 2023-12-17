const NUMBER_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

struct ParsedData {
    number_pairs: Vec<(u32, u32)>,
}

fn parse(input: &str) -> ParsedData {
    let number_pairs = input
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let mut digits: Vec<Option<u32>> = line.chars().map(|d| d.to_digit(10)).collect();

            for (i, num) in NUMBER_WORDS.iter().enumerate(){
                if let Some(pos) = line.find(num) {
                    digits[pos] = Some(i as u32 + 1);
                }

                // we don't care about repeated number words in the middle
                // if the same word is repeated thrice, only the first and last instance matter

                if let Some(pos) = line.rfind(NUMBER_WORDS[i]) {
                    digits[pos] = Some(i as u32 + 1);
                }
            }

            let digits: Vec<u32> = digits.into_iter().flatten().collect();

            let first: u32 = *digits
                .first()
                .unwrap_or_else(|| panic!("no digit found in line {index:?}: {line:?}"));
            let last: u32 = *digits.last().unwrap_or(&first);

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
    let input = include_str!("step_2.test");

    let data = parse(input);
    let sum = solve(data);

    assert_eq!(sum, 281);
}

#[test]
fn prod() {
    let input = include_str!("step_2.prod");

    let data = parse(input);
    let sum = solve(data);

    println!("sum: {sum}");
}

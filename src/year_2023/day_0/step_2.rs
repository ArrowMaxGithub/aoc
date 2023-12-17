struct ParsedData {

}

fn parse(input: &str) -> ParsedData {
    let number_pairs = input
        .lines()
        .enumerate()
        .map(|(index, line)| {

        })
        .collect();

    ParsedData { }
}

fn solve(data: ParsedData) -> u32 {
    
}

#[test]
fn test() {
    let input = include_str!("step_2.test");

    let data = parse(input);
    let result = solve(data);

    assert_eq!(result, 42);
}

#[test]
fn prod() {
    let input = include_str!("step_2.prod");

    let data = parse(input);
    let result = solve(data);

    println!("result: {result}");
}

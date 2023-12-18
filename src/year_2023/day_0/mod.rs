const STEP_1_EXAMPLE_RESULT: u32 = 42;
const STEP_2_EXAMPLE_RESULT: u32 = 42;

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

fn solve_1(data: ParsedData) -> u32 {
    
}

fn solve_2(data: ParsedData) -> u32 {
    
}

#[test]
fn step_1() {
    let example_input = include_str!("step_1.test");
    let example_data = parse(example_input);
    let example_result = solve_1(example_data);
    assert_eq!(example_result, STEP_1_EXAMPLE_RESULT);

    let input = include_str!("step_1.prod");
    let data = parse(input);
    let result = solve_1(data);
    println!("result: {result}");
}

#[test]
fn step_2() {
    let example_input = include_str!("step_2.test");
    let example_data = parse(example_input);
    let example_result = solve_2(example_data);
    assert_eq!(example_result, STEP_2_EXAMPLE_RESULT);

    let input = include_str!("step_2.prod");
    let data = parse(input);
    let result = solve_2(data);
    println!("result: {result}");
}
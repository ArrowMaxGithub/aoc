const STEP_1_EXAMPLE_RESULT: u32 = 4361;
const STEP_2_EXAMPLE_RESULT: u32 = 42;

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}

struct NumberGroup{
    line: usize,
    start: usize,
    end: usize,
    val: u32,
}

struct Symbol{
    line: usize,
    index: usize,
    c: char,
}

struct ParsedData {
    number_groups: Vec<NumberGroup>,
    symbols: Vec<Symbol>,
}

fn parse(input: &str) -> ParsedData {
    let (number_groups, symbols) = input
        .lines()
        .enumerate()
        .map(|(line, content)| {
            let number_indices = content.match_indices(!is_symbol).collect();
            let symbol_indices = content.match_indices(is_symbol).collect();

            let number_groups = number_indices.iter()
                .map(|(char_index, text)|{
                    NumberGroup{
                        line,
                        start_index: char_index,
                        end_index: char_index + text.len(),
                        val: text.parse().unwrap(),
                    }
                })
                .collect();

            let symbols = symbol_indices.iter()
                .map(|(char_index, text)|{
                    Symbol{
                        line,
                        index: char_index,
                        c: text,
                    }
                })
                .collect();

            (number_groups, symbols)
        })
        .unzip()
        .flatten()
        .collect();

    ParsedData { 
        number_groups,
        symbols,
    }
}

fn solve_1(data: ParsedData) -> u32 {
    let active_coords = data.symbols.iter()
        .map(|symbol|{
            [
                // calc 3x3 activated coords by this symbol
                // simple 1-d index offsets are not possible due to unconstrained input line length

                (symbol.index.checked_sub(1), symbol.line.checked_sub(1)), // top-left
                (symbol.index.checked_sub(0), symbol.line.checked_sub(1)), // top-center
                (symbol.index.checked_add(1), symbol.line.checked_sub(1)), // top-right
                (symbol.index.checked_sub(1), symbol.line.checked_sub(0)), // same-left
                (symbol.index.checked_sub(0), symbol.line.checked_sub(0)), // same-center
                (symbol.index.checked_add(1), symbol.line.checked_sub(0)), // same-right
                (symbol.index.checked_sub(1), symbol.line.checked_add(1)), // bot-left
                (symbol.index.checked_sub(0), symbol.line.checked_add(1)), // bot-center
                (symbol.index.checked_add(1), symbol.line.checked_add(1)), // bot-right
            ]
        })
        .flatten()
        .filter(|(x, y)| x.is_some() && y.is_some())
        .collect();
    
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
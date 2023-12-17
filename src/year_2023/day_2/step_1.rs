use std::{num::ParseIntError, str::FromStr};

enum Color {
    Red(usize),
    Green(usize),
    Blue(usize),
}

impl FromStr for Color {
    type Err = ParseIntError;

    // example: ' 2 green'
    fn from_str(cube_draw: &str) -> Result<Self, Self::Err> {
        let mut split = cube_draw.split_whitespace();
        let number = split.next().expect("cube draw does not contain any number");
        let col = split.next().expect("cube draw does not contain any color");

        match col {
            "red" => Ok(Color::Red(number.parse()?)),
            "green" => Ok(Color::Green(number.parse()?)),
            "blue" => Ok(Color::Blue(number.parse()?)),
            _ => unreachable!(),
        }
    }
}

struct Set {
    draws: Vec<Color>,
}
impl Set {
    fn cube_count(&self) -> (usize, usize, usize) {
        self.draws
            .iter()
            .fold((0, 0, 0), |(mut r, mut g, mut b), draw| {
                match draw {
                    Color::Red(count) => r += count,
                    Color::Green(count) => g += count,
                    Color::Blue(count) => b += count,
                }

                (r, g, b)
            })
    }
}

impl FromStr for Set {
    type Err = ParseIntError;

    //example: '2 red, 2 green'
    fn from_str(set: &str) -> Result<Self, Self::Err> {
        let draws = set
            .split(',')
            .map(|draw| draw.parse())
            .collect::<Result<Vec<Color>, _>>()?;

        Ok(Self { draws })
    }
}

struct Game {
    sets: Vec<Set>,
}

impl Game {
    fn minimum_req_cubes(&self) -> (usize, usize, usize) {
        let set_cubes: Vec<(usize, usize, usize)> =
            self.sets.iter().map(|set| set.cube_count()).collect();

        let max_r = set_cubes.iter().max_by(|x, y| x.0.cmp(&y.0)).unwrap().0;
        let max_g = set_cubes.iter().max_by(|x, y| x.1.cmp(&y.1)).unwrap().1;
        let max_b = set_cubes.iter().max_by(|x, y| x.2.cmp(&y.2)).unwrap().2;

        (max_r, max_g, max_b)
    }
}

impl FromStr for Game {
    type Err = ParseIntError;

    // example: '3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green'
    fn from_str(game: &str) -> Result<Self, Self::Err> {
        let sets = game
            .split(';')
            .map(|set| set.parse())
            .collect::<Result<Vec<Set>, _>>()?;

        Ok(Self { sets })
    }
}

struct ParsedData {
    games: Vec<Game>,
}

fn parse(input: &str) -> ParsedData {
    let games = input
        .lines()
        .map(|line| line.split(':').last().unwrap().trim().parse()) // split Game ID from Game 'content' => trim ws and parse 'content'
        .collect::<Result<Vec<Game>, _>>()
        .unwrap();

    ParsedData { games }
}

fn solve(data: ParsedData) -> usize {
    let game_mins: Vec<(usize, usize, usize)> = data
        .games
        .iter()
        .map(|game| game.minimum_req_cubes())
        .collect();

    game_mins
        .iter()
        .enumerate()
        .map(|(index, (r, g, b))| {
            if *r <= 12 && *g <= 13 && *b <= 14 {
                index + 1
            } else {
                0
            }
        })
        .sum()
}

#[test]
fn test() {
    let input = include_str!("step_1.test");

    let data = parse(input);
    let sum = solve(data);

    assert_eq!(sum, 8);
}

#[test]
fn prod() {
    let input = include_str!("step_1.prod");

    let data = parse(input);
    let sum = solve(data);

    println!("sum: {sum}");
}

// https://adventofcode.com/2022/day/2

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

const WIN: u32 = 6;
const DRAW: u32 = 3;
const LOSS: u32 = 0;

fn main() {
    let input = std::fs::read_to_string("./input.txt").unwrap();
    let rounds = input.split("\n");

    let mut optimal_score: u32 = 0;
    let mut real_score: u32 = 0;

    rounds.for_each(|round| {
        let me = round.split_whitespace().next_back().unwrap();
        let opponent = round.split_whitespace().next().unwrap();

        optimal_score += match (me, opponent) {
            ("X", "A") => ROCK + DRAW,
            ("X", "B") => ROCK + LOSS,
            ("X", "C") => ROCK + WIN,

            ("Y", "A") => PAPER + WIN,
            ("Y", "B") => PAPER + DRAW,
            ("Y", "C") => PAPER + LOSS,

            ("Z", "A") => SCISSORS + LOSS,
            ("Z", "B") => SCISSORS + WIN,
            ("Z", "C") => SCISSORS + DRAW,
            _ => panic!("OTHER"),
        };

        real_score += match (me, opponent) {
            ("X", "A") => LOSS + SCISSORS,
            ("X", "B") => LOSS + ROCK,
            ("X", "C") => LOSS + PAPER,

            ("Y", "A") => DRAW + ROCK,
            ("Y", "B") => DRAW + PAPER,
            ("Y", "C") => DRAW + SCISSORS,

            ("Z", "A") => WIN + PAPER,
            ("Z", "B") => WIN + SCISSORS,
            ("Z", "C") => WIN + ROCK,
            _ => panic!("OTHER"),
        };
    });

    println!("Optimal score is: {}", optimal_score);

    println!("Real score is: {}", real_score);
}

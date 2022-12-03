// https://adventofcode.com/2022/day/3

use std::collections::HashSet;

fn evaluate_char_value(c: &char) -> u32 {
    // Do not look at this
    if c >= &'a' && c <= &'z' {
        return c.clone() as u32 - 97 + 1;
    } else if c >= &'A' && c <= &'Z' {
        return c.clone() as u32 - 65 + 27;
    }
    panic!("Woops");
}

fn part_1(input: &String) -> u32 {
    let lines = input.split("\n");

    let mut sum = 0u32;
    lines.for_each(|line| {
        let (first_part, second_part) = line.split_at(line.len() / 2);

        let first_chars: HashSet<char> = first_part.chars().collect();
        second_part.chars().any(|c| {
            if first_chars.contains(&c) {
                sum += evaluate_char_value(&c);
                return true;
            }
            false
        });
    });

    sum
}

fn part_2(input: &String) -> u32 {
    let groups = input.split("\n").collect::<Vec<&str>>();
    let chunks = groups.chunks(3);

    let mut sum = 0u32;
    chunks.for_each(|c| {
        let first_chars: HashSet<char> = c[0].chars().collect();
        let second_chars: HashSet<char> = c[1].chars().collect();

        c[2].chars().any(|c| {
            if first_chars.contains(&c) && second_chars.contains(&c) {
                sum += evaluate_char_value(&c);
                return true;
            }
            false
        });
    });

    sum
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Result of part1: {}", part_1(&input));
    println!("Result of part2: {}", part_2(&input));
}

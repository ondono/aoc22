use anyhow::Result;
use itertools::Itertools;

fn main() -> Result<()> {
    let test_input = include_str!("../input/day_3_test.txt");
    let input = include_str!("../input/day_3.txt");

    println!("Part test 1: {}", part1(test_input));
    println!("Part 1: {}", part1(input));

    println!("Part test 2: {}", part2(test_input));
    println!("Part 2: {}", part2(input));

    Ok(())
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| line.split_at(line.len() / 2))
        .map(|(left, right)| (left.chars().unique(), right.chars().unique()))
        .flat_map(|(left, right)| {
            left.filter(|c| right.clone().contains(c))
                .collect::<Vec<char>>()
        })
        .map(|c| match c {
            'a'..='z' => c as u64 - 'a' as u64 + 1,
            'A'..='Z' => c as u64 - 'A' as u64 + 27,
            _ => 0,
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|c| c.collect::<Vec<&str>>())
        .map(|v| {
            for c in v[0].chars() {
                if v[1].chars().contains(&c) && v[2].chars().contains(&c) {
                    return c;
                }
            }
            ' ' // dummy because we know there is always a match
        })
        .map(|c| match c {
            'a'..='z' => c as u64 - 'a' as u64 + 1,
            'A'..='Z' => c as u64 - 'A' as u64 + 27,
            _ => 0,
        })
        .sum::<u64>()
}

fn main() {
    let input = include_str!("../input/day_2.txt");
    println!("Part 1 total score is: {}", part1(input));
    println!("Part 2 total score is: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut l = line.split_whitespace();
            (l.next().unwrap(), l.next().unwrap())
        })
        .map(|(op, own)| match (op, own) {
            // first the ties
            ("A", "X") => 1 + 3,
            ("B", "Y") => 2 + 3,
            ("C", "Z") => 3 + 3,
            // then the wins
            ("A", "Y") => 2 + 6,
            ("B", "Z") => 3 + 6,
            ("C", "X") => 1 + 6,
            // then the losses
            ("A", "Z") => 3,
            ("B", "X") => 1,
            ("C", "Y") => 2,
            _ => 0,
        })
        .sum()
}

fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|line| {
            let mut l = line.split_whitespace();
            (l.next().unwrap(), l.next().unwrap())
        })
        .map(|(op, own)| match (op, own) {
            // first the losses
            ("A", "X") => 3,
            ("B", "X") => 1,
            ("C", "X") => 2,
            // then the draws
            ("A", "Y") => 1 + 3,
            ("B", "Y") => 2 + 3,
            ("C", "Y") => 3 + 3,
            // then the wins
            ("A", "Z") => 2 + 6,
            ("B", "Z") => 3 + 6,
            ("C", "Z") => 1 + 6,
            _ => 0,
        })
        .sum()
}

use itertools::Itertools;

fn main() {
    let input = include_str!("../input/day_1.txt");

    println!("The elf with the most calories carries: {}", part1(input));
    println!(
        "The three elves with the most calories carry: {}",
        part2(input)
    );
}

fn part1(input: &str) -> u64 {
    input
        .lines()
        .map(|x| x.parse::<u64>().ok())
        .batching(|iter| iter.map_while(|x| x).sum1::<u64>())
        .max()
        .unwrap()
}
fn part2(input: &str) -> u64 {
    input
        .lines()
        .map(|x| x.parse::<u64>().ok())
        .batching(|iter| iter.map_while(|x| x).sum1::<u64>())
        .sorted()
        .rev()
        .take(3)
        .sum()
}

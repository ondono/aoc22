fn main() {
    let input_test = std::fs::read_to_string("input/day-11-test.txt").unwrap();
    let input = std::fs::read_to_string("input/day-11.txt").unwrap();

    //part1(&input_test);
    //part1(&input);

    //part2(&input_test);
    part2(&input);
}

use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Default, Clone, PartialEq)]
struct Monkey {
    id: usize,
    items: VecDeque<i64>,
    items_inspected: usize,
    operation: MonkeyOperation,
    test: i32,
    next_if_true: usize,
    next_if_false: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MonkeyOperand {
    Old,
    Value(i64),
}

impl Default for MonkeyOperand {
    fn default() -> Self {
        MonkeyOperand::Old
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum MonkeyOperation {
    Add(MonkeyOperand, MonkeyOperand),
    Multiply(MonkeyOperand, MonkeyOperand),
}

impl MonkeyOperation {
    pub fn parse(list: Vec<&str>) -> Self {
        let op = list[1];
        let a = list[0];
        let b = list[2];

        let a = if a == "old" {
            MonkeyOperand::Old
        } else {
            MonkeyOperand::Value(a.parse().unwrap())
        };

        let b = if b == "old" {
            MonkeyOperand::Old
        } else {
            MonkeyOperand::Value(b.parse().unwrap())
        };

        match op {
            "+" => MonkeyOperation::Add(a, b),
            "*" => MonkeyOperation::Multiply(a, b),
            _ => MonkeyOperation::Add(a, b),
        }
    }
}

impl Default for MonkeyOperation {
    fn default() -> Self {
        MonkeyOperation::Multiply(MonkeyOperand::Old, MonkeyOperand::Old)
    }
}

fn part1(input: &str) {
    let mut monkeys = vec![];

    let groups = input
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .map(|(_, g)| g.collect::<Vec<&str>>().join("\n"))
        .collect::<Vec<String>>();

    for g in groups.iter().filter(|g| !g.is_empty()) {
        //println!("{:?}", g);
        let mut m: Monkey = Monkey::default();

        for (i, s) in g.lines().enumerate() {
            match i {
                0 => {
                    // Parse monkey id
                    m.id = s
                        .split_whitespace()
                        .nth(1)
                        .unwrap()
                        .trim_end_matches(':')
                        .parse::<usize>()
                        .unwrap();
                }
                1 => {
                    // Parse items list
                    m.items = s
                        .split_whitespace()
                        .skip(2)
                        .map(|s| s.trim_end_matches(',').parse().unwrap())
                        .collect();
                }
                2 => {
                    // parse operation
                    let list = s.split_whitespace().skip(3).collect::<Vec<&str>>();
                    m.operation = MonkeyOperation::parse(list);
                }
                3 => {
                    // Parse test
                    m.test = s.split_whitespace().last().unwrap().parse::<i32>().unwrap();
                }
                4 => {
                    // parse true
                    m.next_if_true = s
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                }
                5 => {
                    // parse false
                    m.next_if_false = s
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                }
                _ => (),
            }
        }
        monkeys.push(m);
    }
    let num_monkeys = monkeys.len();

    for round in 0..20 {
        for index in 0..num_monkeys {
            // create a copy of the relevant information
            let monkey = &monkeys[index].clone();
            //println!("Monkey: {}", monkey.id);
            // item slinging
            for _ in 0..monkey.items.len() {
                let mut item = monkeys[index].items.pop_front().unwrap();
                //println!("Monkey {} inspects item {}", monkey.id, item);
                monkeys[index].items_inspected += 1;
                // perform operation
                match monkey.operation {
                    MonkeyOperation::Add(a, b) => {
                        let a = match a {
                            MonkeyOperand::Old => item,
                            MonkeyOperand::Value(v) => v,
                        };
                        let b = match b {
                            MonkeyOperand::Old => item,
                            MonkeyOperand::Value(v) => v,
                        };
                        item = a + b;
                    }
                    MonkeyOperation::Multiply(a, b) => {
                        let a = match a {
                            MonkeyOperand::Old => item,
                            MonkeyOperand::Value(v) => v,
                        };
                        let b = match b {
                            MonkeyOperand::Old => item,
                            MonkeyOperand::Value(v) => v,
                        };
                        item = a * b;
                    }
                }
                //println!("Worry level is {}", item);
                // Monkey gets bored
                item /= 3;
                // println!(
                //     "Monkey {} gets bored. Worry level is now {}",
                //     monkey.id, item
                // );

                if item % (monkey.test as i64) == 0 {
                    //println!("Current worry level is divisible by {}", monkey.test);
                    monkeys[monkey.next_if_true].items.push_back(item);
                    // println!(
                    //     "Monkey {} throws item {} to Monkey {}",
                    //     monkey.id, item, monkey.next_if_true
                    // );
                } else {
                    //println!("Current worry level is not divisible by {}", monkey.test);
                    monkeys[monkey.next_if_false].items.push_back(item);
                    // println!(
                    //     "Monkey {} throws item {} to Monkey {}",
                    //     monkey.id, item, monkey.next_if_false
                    // );
                }
                //println!();
            }
        }
        println!("After round {}, the monkeys hold:", round);
        for m in &monkeys {
            println!("Monkey {}: {:?}", m.id, m.items);
        }
    }
    println!();
    let mut inspections = vec![];
    for m in &monkeys {
        println!("Monkey {} inspected {} items", m.id, m.items_inspected);
        inspections.push(m.items_inspected);
    }
    inspections.sort();
    println!(
        "The level of monkey business is: {:?}",
        inspections.pop().unwrap() * inspections.pop().unwrap()
    );
}
fn part2(input: &str) {
    let mut monkeys = vec![];

    let groups = input
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .map(|(_, g)| g.collect::<Vec<&str>>().join("\n"))
        .collect::<Vec<String>>();

    for g in groups.iter().filter(|g| !g.is_empty()) {
        //println!("{:?}", g);
        let mut m: Monkey = Monkey::default();

        for (i, s) in g.lines().enumerate() {
            match i {
                0 => {
                    // Parse monkey id
                    m.id = s
                        .split_whitespace()
                        .nth(1)
                        .unwrap()
                        .trim_end_matches(':')
                        .parse::<usize>()
                        .unwrap();
                }
                1 => {
                    // Parse items list
                    m.items = s
                        .split_whitespace()
                        .skip(2)
                        .map(|s| s.trim_end_matches(',').parse().unwrap())
                        .collect();
                }
                2 => {
                    // parse operation
                    let list = s.split_whitespace().skip(3).collect::<Vec<&str>>();
                    m.operation = MonkeyOperation::parse(list);
                }
                3 => {
                    // Parse test
                    m.test = s.split_whitespace().last().unwrap().parse::<i32>().unwrap();
                }
                4 => {
                    // parse true
                    m.next_if_true = s
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                }
                5 => {
                    // parse false
                    m.next_if_false = s
                        .split_whitespace()
                        .last()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap();
                }
                _ => (),
            }
        }
        monkeys.push(m);
    }
    let num_monkeys = monkeys.len();
    let common_multiple: i64 = monkeys.iter().map(|m| m.test).product::<i32>() as i64;

    for round in 0..10000 {
        for index in 0..num_monkeys {
            // create a copy of the relevant information
            let monkey = &monkeys[index].clone();
            //println!("Monkey: {}", monkey.id);
            // item slinging
            for _ in 0..monkey.items.len() {
                let mut item = monkeys[index].items.pop_front().unwrap();
                //println!("Monkey {} inspects item {}", monkey.id, item);
                monkeys[index].items_inspected += 1;
                // perform operation
                match monkey.operation {
                    MonkeyOperation::Add(a, b) => {
                        let a = match a {
                            MonkeyOperand::Old => item,
                            MonkeyOperand::Value(v) => v,
                        };
                        let b = match b {
                            MonkeyOperand::Old => item,
                            MonkeyOperand::Value(v) => v,
                        };
                        item = a + b;
                    }
                    MonkeyOperation::Multiply(a, b) => {
                        let a = match a {
                            MonkeyOperand::Old => item,
                            MonkeyOperand::Value(v) => v,
                        };
                        let b = match b {
                            MonkeyOperand::Old => item,
                            MonkeyOperand::Value(v) => v,
                        };
                        item = a * b;
                    }
                }
                item %= common_multiple;
                //println!("Worry level is {}", item);

                if item % (monkey.test as i64) == 0 {
                    //println!("Current worry level is divisible by {}", monkey.test);
                    monkeys[monkey.next_if_true].items.push_back(item);
                    // println!(
                    //     "Monkey {} throws item {} to Monkey {}",
                    //     monkey.id, item, monkey.next_if_true
                    // );
                } else {
                    //println!("Current worry level is not divisible by {}", monkey.test);
                    monkeys[monkey.next_if_false].items.push_back(item);
                    // println!(
                    //     "Monkey {} throws item {} to Monkey {}",
                    //     monkey.id, item, monkey.next_if_false
                    // );
                }
                //println!();
            }
        }
        println!("After round {}, the monkeys hold:", round);
        for m in &monkeys {
            println!("Monkey {}: {:?}", m.id, m.items);
        }
    }
    println!();
    let mut inspections = vec![];
    for m in &monkeys {
        println!("Monkey {} inspected {} items", m.id, m.items_inspected);
        inspections.push(m.items_inspected);
    }
    inspections.sort();
    println!(
        "The level of monkey business is: {:?}",
        inspections.pop().unwrap() * inspections.pop().unwrap()
    );
}

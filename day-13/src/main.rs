use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input_test = std::fs::read_to_string("input/day-13-test.txt").unwrap();
    let input = std::fs::read_to_string("input/day-13.txt").unwrap();

    println!("Test result: {}", part1(&input_test)?);
    println!("Result: {}", part1(&input)?);

    println!("Test result 2: {}", part2(&input_test)?);
    println!("Result 2: {}", part2(&input)?);

    Ok(())
}

use itertools::Itertools;

fn part1(input: &str) -> Result<usize> {
    let mut right_ones = vec![];

    let x = input
        .lines()
        .filter(|l| !l.is_empty())
        .batching(|it| match it.next() {
            None => None,
            Some(x) => it.next().map(|y| (x, y)),
        })
        .collect::<Vec<_>>();

    for (i, (a, b)) in x.iter().enumerate() {
        let obj_a = Object::parse(a)?;
        let obj_b = Object::parse(b)?;

        assert!(obj_a.to_string() == *a);
        assert!(obj_b.to_string() == *b);

        if obj_a < obj_b {
            right_ones.push(i + 1);
        }
    }

    Ok(right_ones.iter().sum())
}

fn part2(input: &str) -> Result<usize> {
    let mut packets = input
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|l| Object::parse(l).ok())
        .collect::<Vec<_>>();

    packets.push(Object::parse("[[2]]")?);
    packets.push(Object::parse("[[6]]")?);
    packets.sort();

    let divider1 = packets
        .iter()
        .position(|p| p.to_string() == "[[2]]")
        .ok_or_else(|| anyhow!("can't find divider1"))?;

    let divider2 = packets
        .iter()
        .position(|p| p.to_string() == "[[6]]")
        .ok_or_else(|| anyhow!("cant' find divider2"))?;

    Ok((divider1 + 1) * (divider2 + 1))
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Object {
    Int(i32),
    List(Vec<Object>),
}
use std::fmt::Display;
impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Object::Int(x) => write!(f, "{}", x),
            Object::List(x) => {
                write!(f, "[")?;
                let lenght = x.len();
                for (i, obj) in x.iter().enumerate() {
                    write!(f, "{}", obj)?;
                    if i != lenght - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")
            }
        }
    }
}

impl Object {
    fn parse(s: &str) -> Result<Object> {
        let mut obj: Object;
        //println!("Parsing string: \"{}\"", s);
        if &s[0..1] == "[" {
            let mut stack: i32 = 0;
            let v = s[1..s.len() - 1]
                .split(|c| {
                    if c == '[' {
                        stack += 1;
                    } else if c == ']' {
                        stack -= 1;
                    }
                    c == ',' && stack == 0
                })
                .filter_map(|s| (!s.is_empty()).then(|| Object::parse(s)))
                .filter_map(|x| x.ok())
                .collect();
            Ok(Object::List(v))
        } else {
            // if s.is_empty() {
            //     Ok(Object::List(vec![]))
            // } else {
            // In
            Ok(Object::Int(s.parse::<i32>()?))
            //}
        }
    }
}

use std::cmp::Ord;
use std::cmp::PartialOrd;

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Object::Int(a), Object::Int(b)) => a.partial_cmp(b),
            (Object::List(a), Object::List(b)) => a.partial_cmp(b),
            (Object::Int(a), Object::List(b)) => vec![Object::Int(*a)].partial_cmp(b),
            (Object::List(a), Object::Int(b)) => a.partial_cmp(&vec![Object::Int(*b)]),
        }
    }
}

impl Ord for Object {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Object::Int(a), Object::Int(b)) => a.cmp(b),
            (Object::List(a), Object::List(b)) => a.cmp(b),
            (Object::Int(a), Object::List(b)) => vec![Object::Int(*a)].cmp(b),
            (Object::List(a), Object::Int(b)) => a.cmp(&vec![Object::Int(*b)]),
        }
    }
}

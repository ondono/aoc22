fn main() {
    let test_input = std::fs::read_to_string("input/day-10-test.txt").unwrap();
    let input = std::fs::read_to_string("input/day-10.txt").unwrap();

    println!("Part 1 test result: {}", part1(&test_input));
    println!("Part 1 result: {}", part1(&input));

    part2(&test_input);
    part2(&input);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    Add(i32),
    Nop,
    Fake,
}
use Instruction::*;

fn part1(input: &str) -> i32 {
    let instructions = input
        .lines()
        .flat_map(|line| match line {
            "noop" => vec![Nop],
            _ => {
                let (_, val) = line.split_at(5);
                vec![Nop, Add(val.parse().unwrap())]
            }
        })
        .collect::<Vec<_>>();

    let mut x = 1;
    let mut relevant_strengths = vec![];

    for (cycle, ins) in instructions.iter().enumerate().map(|(i, ins)| (i + 1, ins)) {
        if cycle >= 20 && (cycle - 20) % 40 == 0 {
            //  println!("x: {}", x);
            //  println!("signal at {}: {}", cycle, x * (cycle as i32));
            relevant_strengths.push(x * (cycle as i32));
        }
        match ins {
            Add(val) => x += val,
            Nop => (),
            Fake => (),
        }
    }

    relevant_strengths.iter().sum()
}

fn print_sprite(position: usize) {
    assert!(position <= 40);
    print!("Sprite position: ");
    for i in 0..40 {
        if position == i + 1 || position == i || position + 1 == i {
            print!("#");
        } else {
            print!(".");
        }
    }
    println!();
}

fn part2(input: &str) {
    let instructions = input
        .lines()
        .flat_map(|line| match line {
            "noop" => vec![Nop],
            _ => {
                let (_, val) = line.split_at(5);
                vec![Nop, Add(val.parse().unwrap())]
            }
        })
        .collect::<Vec<_>>();

    let mut x = 1;

    let mut frame = ['.'; 240];

    for (cycle, ins) in instructions.iter().enumerate().map(|(i, ins)| (i + 1, ins)) {
        // Screen printing
        let horizontal = ((cycle - 1) % 40) as i32;
        if horizontal == x - 1 || horizontal == x || horizontal == x + 1 {
            frame[cycle - 1] = '#';
        } else {
            frame[cycle - 1] = '.';
        }

        print_sprite(horizontal as usize);
        let row = (cycle - 1) / 40;
        let column = (cycle - 1) % 40;
        println!("Cyle: {}, pixel in position: ({},{})", cycle, column, row);

        if ins == &Fake {
            println!("begining Add instruction");
        }
        if let Add(n) = ins {
            println!("Executing Add({})", n);
        }
        println!();
        print!("Current CRT row: ");
        for i in 0..=column {
            print!("{}", frame[i + row * 40]);
        }
        println!();
        println!();

        match ins {
            Add(val) => x += val,
            Nop => (),
            Fake => (),
        }
    }

    for y in 0..6 {
        for x in 0..40 {
            print!("{}", frame[y * 40 + x]);
        }
        println!();
    }
}

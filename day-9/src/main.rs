fn main() {
    let input_test = std::fs::read_to_string("input/day_9_test.txt").unwrap();
    let input = std::fs::read_to_string("input/day_9.txt").unwrap();

    part1(&input_test);
    part1(&input);
}

fn manhattan_distance(p: (i32, i32), q: (i32, i32)) -> i32 {
    std::cmp::max((p.0 - q.0).abs(), (p.1 - q.1).abs())
}

fn print_path(path: &Vec<(i32, i32)>, item: char) {
    let max_x = *path.iter().map(|(x, _)| x).max().unwrap() as usize;
    let max_y = *path.iter().map(|(_, y)| y).max().unwrap() as usize;

    let head = path.last().unwrap();
    let tail = path.first().unwrap();

    for y in (0..=max_y).rev() {
        for x in 0..=max_x {
            if (x as i32, y as i32) == *head {
                print!("{}", item);
            } else if (x as i32, y as i32) == *tail {
                print!("s");
            } else if path.contains(&(x as i32, y as i32)) {
                print!("#");
                //grid[y][x] = '#';
            } else {
                print!(".");
                //grid[y][x] = '.';
            }
        }
        println!();
    }
    println!();
}

fn print_head_and_tail(head: (i32, i32), tail: (i32, i32), max_x: i32, max_y: i32) {
    for y in (0..=max_y).rev() {
        for x in 0..=max_x {
            if (x as i32, y as i32) == (0, 0) {
                print!("s");
            } else if (x as i32, y as i32) == head {
                print!("H");
            } else if (x as i32, y as i32) == tail {
                print!("T");
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

fn add_point(p: (i32, i32), q: (i32, i32)) -> (i32, i32) {
    (p.0 + q.0, p.1 + q.1)
}

fn part1(input: &str) -> i32 {
    let instructions = input
        .lines()
        .map(|line| line.split_at(2))
        .map(|(op, num)| (op.trim(), num.parse::<i32>().unwrap()))
        .collect::<Vec<_>>();

    let num_segments = 10;

    let mut visited = vec![vec![(0, 0)]; num_segments];

    let mut segments = vec![(0, 0); num_segments];

    for (dir, num) in instructions.iter() {
        println!("== {} {} ==", dir, num);
        // First move the HEAD
        for i in 1..=*num {
            println!("-- {} {} --", dir, num - i);

            //let position = *head_path.last().unwrap();
            let op = dir.to_owned();
            let delta = match op {
                "U" => (0, 1),
                "R" => (1, 0),
                "D" => (0, -1),
                "L" => (-1, 0),
                _ => panic!("wrong instructions!"),
            };

            segments[0] = add_point(segments[0], delta);

            for i in 1..segments.len() {
                let head = segments[i - 1];
                let segment = segments[i];

                if manhattan_distance(head, segment) > 1 {
                    let dx = (head.0 - segment.0).signum();
                    let dy = (head.1 - segment.1).signum();
                    segments[i] = add_point(segment, (dx, dy));
                    visited[i].push(segments[i]);
                }
            }

            let h = segments[0];
            let t = segments[segments.len() - 1];

            println!("head: {:?}, tail: {:?}", h, t);
            print_head_and_tail(h, t, 10, 10);
        }
    }

    use itertools::Itertools;
    println!(
        "visited: {:?}",
        visited
            .iter()
            .map(|v| v.iter().unique().count())
            .collect::<Vec<usize>>(),
    );

    0
}

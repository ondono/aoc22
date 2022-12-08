fn main() {
    let test_input = include_str!("../input/day_8_test.txt");
    let input = include_str!("../input/day_8.txt");
    println!("Part 1 test: {}", part1(test_input));
    println!("Part 1: {}", part1(input));
    println!("Part 2 test: {}", part2(test_input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    // parse into a Vec<Vec<u32>>
    let field = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = field[0].len();
    let height = field.len();

    let mut visible: Vec<Vec<bool>> = vec![vec![false; width]; height];

    for y in 0..height {
        for x in 0..width {
            // for each tree
            // all trees in the border are visible
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                visible[y][x] = true;
                continue;
            }
            let visible_left = field[y][0..x].iter().all(|&c| c < field[y][x]);
            let visible_right = field[y][x + 1..width].iter().all(|&c| c < field[y][x]);
            let visible_down = field[y + 1..height]
                .iter()
                .map(|row| row[x])
                .all(|c| c < field[y][x]);
            let visible_up = field[0..y]
                .iter()
                .map(|row| row[x])
                .all(|c| c < field[y][x]);
            visible[y][x] = visible_right || visible_left || visible_up || visible_down;
        }
    }
    visible.iter().flatten().filter(|&&b| b).count() as u64
}
fn part2(input: &str) -> u64 {
    // parse into a Vec<Vec<u32>>
    let field = input
        .lines()
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let width = field[0].len();
    let height = field.len();

    let mut visible: Vec<Vec<bool>> = vec![vec![false; width]; height];

    for y in 0..height {
        for x in 0..width {
            // for each tree
            // all trees in the border are visible
            if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                visible[y][x] = true;
                continue;
            }
            // travel up
            let visible_left = field[y][0..x].iter().all(|&c| c < field[y][x]);
            let visible_right = field[y][x + 1..width].iter().all(|&c| c < field[y][x]);

            let visible_down = field[y + 1..height]
                .iter()
                .map(|row| row[x])
                .all(|c| c < field[y][x]);

            let visible_up = field[0..y]
                .iter()
                .map(|row| row[x])
                .all(|c| c < field[y][x]);
            visible[y][x] = visible_right || visible_left || visible_up || visible_down;
        }
    }

    // up to this point we are just repeating part 1
    let mut scores: Vec<Vec<usize>> = vec![vec![0; width]; height];

    for y in 0..height {
        for x in 0..width {
            // up and left need to be reversed!
            let mut distance_left = field[y][0..x]
                .iter()
                .rev()
                .take_while(|&&c| c < field[y][x])
                .count();
            let mut distance_right = field[y][x + 1..width]
                .iter()
                .take_while(|&&c| c < field[y][x])
                .count();
            let mut distance_up = field[0..y]
                .iter()
                .rev()
                .map(|row| row[x])
                .take_while(|&c| c < field[y][x])
                .count();
            let mut distance_down = field[y + 1..height]
                .iter()
                .map(|row| row[x])
                .take_while(|&c| c < field[y][x])
                .count();

            // correction to account for not counting the bigger tree
            // we need to add 1 unless we are seeing the border of the forest
            if distance_left < x {
                distance_left += 1;
            }
            if distance_right < width - x - 1 {
                distance_right += 1;
            }
            if distance_up < y {
                distance_up += 1;
            }
            if distance_down < height - y - 1 {
                distance_down += 1;
            }

            let score = distance_up * distance_down * distance_left * distance_right;
            scores[y][x] = score;
        }
    }

    scores.iter().flatten().map(|&s| s as u64).max().unwrap()
}

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input_test = std::fs::read_to_string("input/day-12-test.txt").unwrap();
    let input = std::fs::read_to_string("input/day-12.txt").unwrap();

    println!(
        "The shortest path of the test takes {} steps.",
        part1(&input_test)?
    );
    println!(
        "The shortest path of the input takes {} steps.",
        part1(&input)?
    );
    //part2(&input_test);
    part2(&input);
    Ok(())
}

use std::fmt::Display;

fn draw_map(map: &Vec<Vec<impl Display>>) {
    for row in map {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    println!();
}

fn draw_path(path: Vec<Point>, max_x: i32, max_y: i32) {
    // first create an empty map
    let mut map = vec![vec!['.'; max_x as usize]; max_y as usize];

    let start = path[0];
    let end = path[path.len() - 1];

    for x in 0..max_x {
        for y in 0..max_y {
            if Point::new(x, y) == start {
                map[y as usize][x as usize] = 'S';
            } else if Point::new(x, y) == end {
                map[y as usize][x as usize] = 'e';
            } else if path.contains(&Point::new(x, y)) {
                let position = path.iter().position(|p| p == &Point::new(x, y)).unwrap();
                let next = path[position + 1];
                if next.x > x {
                    map[y as usize][x as usize] = '>';
                } else if next.x < x {
                    map[y as usize][x as usize] = '<';
                } else if next.y > y {
                    map[y as usize][x as usize] = 'v';
                } else if next.y < y {
                    map[y as usize][x as usize] = '^';
                }
            }
        }
    }

    draw_map(&map);
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}
use std::ops::Add;

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }

    fn up(&self) -> Point {
        Point {
            x: self.x,
            y: self.y.saturating_sub(1),
        }
    }

    fn down(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }
    fn left(&self) -> Point {
        Point {
            x: self.x.saturating_sub(1),
            y: self.y,
        }
    }
}

fn part1(input: &str) -> Result<i32> {
    let mut map = vec![];

    for line in input.lines() {
        map.push(line.chars().collect::<Vec<char>>());
    }

    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);

    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let x = x as i32;
            let y = y as i32;
            match *col {
                'S' => start = Point::new(x, y),
                'E' => end = Point::new(x, y),
                _ => (),
            }
        }
    }

    find_best_path(start, end, &mut map)
}

fn find_best_path(start: Point, end: Point, map: &mut [Vec<char>]) -> Result<i32> {
    // correct the elevation map
    let map = map
        .iter()
        .map(|row| {
            row.iter()
                .map(|c| match *c {
                    'S' => b'a' as i32,
                    'E' => b'z' as i32,
                    c => c as i32,
                })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    let max_x = map[0].len() as i32;
    let max_y = map.len() as i32;

    let mut hop_map = vec![vec![None; max_x as usize]; max_y as usize];
    let mut unreachable_map = vec![vec![false; max_x as usize]; max_y as usize];

    // avoid visiting the first point!
    hop_map[start.y as usize][start.x as usize] = Some(0);

    // let's take some points out of the map by checking for unreachable points
    for x in 0..max_x {
        for y in 0..max_y {
            let point = Point::new(x, y);

            // the elevation of the current point
            let elevation = map[y as usize][x as usize];

            let next_points = [point.up(), point.down(), point.left(), point.right()]
                .iter()
                .filter(|p| p.x >= 0 && p.x < max_x && p.y >= 0 && p.y < max_y)
                .copied()
                .collect::<Vec<Point>>();

            next_points.iter().for_each(|p| {
                let next_elevation = map[p.y as usize][p.x as usize];
                if next_elevation + 1 < elevation {
                    unreachable_map[p.y as usize][p.x as usize] = true;
                }
            });
        }
    }

    let first_path = vec![start];
    let mut paths = vec![first_path];

    let mut all_paths_found = false;

    while !all_paths_found {
        let mut paths_found = 0;
        let path_count = paths.len();
        let mut open_paths;

        open_paths = 0;
        // go over each path on our list
        for path_index in 0..path_count {
            if paths[path_index].last().unwrap() == &end {
                // if the path gets to the end, count it and skip it
                paths_found += 1;
                continue;
            } else {
                // iterate on this path
                let current = *paths[path_index].clone().last().unwrap();
                let hops = paths[path_index].len() as i32;

                let next_points = [
                    current.up(),
                    current.down(),
                    current.left(),
                    current.right(),
                ]
                .iter()
                .filter(|p| p.x >= 0 && p.x < max_x && p.y >= 0 && p.y < max_y)
                .filter(|p| !paths[path_index].contains(p))
                .filter(|p| {
                    if let Some(hops_to_next) = hop_map[p.y as usize][p.x as usize] {
                        if hops < hops_to_next {
                            hop_map[p.y as usize][p.x as usize] = Some(hops);
                            true
                        } else {
                            false
                        }
                    } else {
                        true
                    }
                })
                // this is filtering points it should for some reason!
                //.filter(|p| {
                //    let elevation = map[p.y as usize][p.x as usize];
                //    let current_elevation = map[current.y as usize][current.x as usize];
                //    current_elevation - elevation <= 1
                //})
                .copied()
                .collect::<Vec<Point>>();

                let path_copy = paths[path_index].clone();

                for (i, next) in next_points.iter().enumerate() {
                    let x = next.x as usize;
                    let y = next.y as usize;

                    // check if we can hop to this point
                    if map[y][x] - map[current.y as usize][current.x as usize] <= 1 {
                        let hops = paths[path_index].len() as i32;
                        // if we had a better path here, skip this one
                        if let Some(min_hops) = hop_map[y][x] {
                            if hops < min_hops {
                                hop_map[y][x] = Some(hops);
                            } else {
                                continue;
                            }
                        } else {
                            hop_map[y][x] = Some(hops);
                        }

                        open_paths += 1;
                        // if it's the first, just add it to the current path
                        if i == 0 {
                            paths[path_index].push(*next);
                        } else {
                            // otherwise, clone the current path and add it to the list of paths
                            paths.push(path_copy.clone());
                            // remove the last point from the current path
                            paths.last_mut().unwrap().push(*next);
                        }
                    }
                }
            }
        }

        if paths_found > 0 {
            let lenghts = paths
                .iter()
                .filter(|p| p.last().unwrap() == &end)
                .map(|p| p.len());

            let min = lenghts.clone().min().unwrap();

            let point_count = map
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .filter(|(x, _)| unreachable_map[y][*x])
                        .map(|(_, v)| v)
                        .copied()
                        .collect::<Vec<i32>>()
                })
                .collect::<Vec<Vec<i32>>>()
                .iter()
                .flatten()
                .count();

            let explored = hop_map
                .iter()
                .enumerate()
                .map(|(y, row)| {
                    row.iter()
                        .enumerate()
                        .filter(|(x, _)| unreachable_map[y][*x])
                        .map(|(_, v)| v)
                        .copied()
                        .collect::<Vec<Option<i32>>>()
                })
                .collect::<Vec<Vec<Option<i32>>>>()
                .iter()
                .flatten()
                .filter(|p| p.is_some())
                .count();

            print!(
                "\rFound {} paths. Explored {} out of {} points ({}%) Current best is: {}",
                paths_found,
                explored,
                point_count,
                explored * 100 / point_count,
                min - 1
            );
            if explored == point_count {
                println!();
                all_paths_found = true;
            }
        } else if open_paths == 0 {
            // for path in paths {
            //     draw_path(path.to_vec(), max_x, max_y);
            // }
            return Err(anyhow!("No paths to End found!"));
        }
    }

    let best_path = paths
        .iter()
        .filter(|p| p.last().unwrap() == &end)
        .min_by_key(|p| p.len())
        .unwrap();

    //draw_path(best_path.to_vec(), max_x, max_y);

    Ok(best_path.len() as i32 - 1)
}

fn part2(input: &str) {
    let mut map = vec![];
    let mut start_positions = vec![];

    for line in input.lines() {
        map.push(line.chars().collect::<Vec<char>>());
    }

    let mut start = Point::new(0, 0);
    let mut end = Point::new(0, 0);

    for (y, row) in map.iter().enumerate() {
        for (x, col) in row.iter().enumerate() {
            let x = x as i32;
            let y = y as i32;
            match *col {
                'S' => start = Point::new(x, y),
                'E' => end = Point::new(x, y),
                'a' => {
                    start_positions.push(Point::new(x, y));
                }
                _ => (),
            }
        }
    }
    // add the original one
    start_positions.push(start);

    // debug the problematic case
    //let start_positions = vec![Point::new(15, 0)];

    let num_starts = start_positions.len();
    println!("found {} starting positions", num_starts);
    let mut lengths = vec![];

    let mut start_map = map.clone();
    for start in &start_positions {
        start_map[start.y as usize][start.x as usize] = 'S';
    }
    draw_map(&start_map);

    for (i, start) in start_positions.iter().enumerate() {
        println!("starting point {}/{}; {}", i, num_starts, start);
        let mut new_map = map.clone();
        if let Ok(length) = find_best_path(*start, end, &mut new_map) {
            lengths.push(length);
        }
    }
    println!("Best path is: {}", lengths.iter().min().unwrap());
}

#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Point {
    Empty,
    Wall,
    End,
}

#[derive(Debug, Clone, Copy)]
struct Reindeer {
    x: usize,
    y: usize,
    direction: Dir,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Dir {
    pub fn to_tuple(&self) -> (isize, isize) {
        match self {
            Dir::Up => (0, -1),
            Dir::Right => (1, 0),
            Dir::Down => (0, 1),
            Dir::Left => (-1, 0),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    data: Vec<Vec<Point>>,
}

impl Map {
    pub fn new(data: Vec<Vec<Point>>) -> Self {
        Map { data }
    }

    pub fn from_string(input: &str) -> (Self, Reindeer) {
        let mut reindeer = Reindeer {
            x: 0,
            y: 0,
            direction: Dir::Right,
        };

        let mut map = Vec::new();

        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                match c {
                    '.' => {
                        row.push(Point::Empty);
                    },
                    '#' => row.push(Point::Wall),
                    'S' => {
                        reindeer.x = x;
                        reindeer.y = y;
                        row.push(Point::Empty);
                    },
                    'E' => row.push(Point::End),
                    _ => panic!("Invalid character in input, {}", c),
                }
            }
            map.push(row);
        }

        (Map::new(map), reindeer)
    }

    fn print(&self) {
        for (_, row) in self.data.iter().enumerate() {
            for (_, point) in row.iter().enumerate() {
                match point {
                    Point::Empty => print!("."),
                    Point::Wall => print!("#"),
                    Point::End => print!("E"),
                }
            }
            println!();
        }
    }
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (map, reindeer) = Map::from_string(&input);

    // find the path
    let score = find_path(&map, reindeer);

    println!("The shortest path is: {}", score);
}

fn find_path(map: &Map, reindeer: Reindeer) -> i64 {
    let mut queue = std::collections::VecDeque::new();
    let mut visited = HashMap::new();

    queue.push_back((reindeer, 0, vec![ (reindeer.x, reindeer.y) ]));

    let mut paths = Vec::new();

    let mut all_used_tiles = HashSet::new();

    let mut shortest_path = std::i64::MAX;

    while let Some((reindeer, score, prev)) = queue.pop_front() {
        let x = reindeer.x;
        let y = reindeer.y;

        match map.data[y][x] {
            Point::Empty => {
                let reindeer = reindeer;

                let left = match reindeer.direction {
                    Dir::Up => Dir::Left,
                    Dir::Right => Dir::Up,
                    Dir::Down => Dir::Right,
                    Dir::Left => Dir::Down,
                };
                let right = match reindeer.direction {
                    Dir::Up => Dir::Right,
                    Dir::Right => Dir::Down,
                    Dir::Down => Dir::Left,
                    Dir::Left => Dir::Up,
                };

                for dir in [reindeer.direction, left, right] {
                    let (dx, dy) = if dir == reindeer.direction { dir.to_tuple() } else { (0, 0) };
                    let x = reindeer.x as isize + dx;
                    let y = reindeer.y as isize + dy;
                    let mut prev = prev.clone();

                    if x < 0 || y < 0 || x >= map.data[0].len() as isize || y >= map.data.len() as isize {
                        continue;
                    }

                    let x = x as usize;
                    let y = y as usize;

                    let new_score = if dir == reindeer.direction {
                        score+1
                    } else {
                        score + 1000
                    };

                    if !visited.contains_key(&(x, y, dir)) || *visited.get(&(x, y, dir)).unwrap() >= new_score {
                        visited.insert((x, y, dir), new_score);

                        prev.push((x,y));

                        queue.push_back((Reindeer { x, y, direction: dir }, new_score, prev));
                    }
                }

            },
            Point::End => {
                if score <= shortest_path {
                    paths.push((score, prev));
                }
                
                if score < shortest_path {
                    shortest_path = score;
                }
            }
            _ => (),
        }
    }

    let shortest_paths = paths.iter().filter(|(score, _)| *score == shortest_path).collect::<Vec<_>>();

    for (_, path) in shortest_paths {
        for (x, y) in path {
            all_used_tiles.insert((*x, *y));
        }
    }

    display_current_state(map, &all_used_tiles);

    return all_used_tiles.len() as i64;
}

fn display_current_state(map: &Map, used_tiles: &HashSet<(usize, usize)>) {
    for (y, row) in map.data.iter().enumerate() {
        for (x, point) in row.iter().enumerate() {
            if used_tiles.contains(&(x, y)) {
                print!("O");
            } else {
                match point {
                    Point::Empty => print!("."),
                    Point::Wall => print!("#"),
                    Point::End => print!("E"),
                }
            }
        }
        println!();
    }
    println!();
    
}
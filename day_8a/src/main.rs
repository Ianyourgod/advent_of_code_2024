#![allow(dead_code)]

use std::collections::HashSet;

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy)]
enum Item {
    Empty,
    Antenna(char),
}

#[derive(Debug, Clone)]
struct Map {
    map: Vec<Vec<Item>>,
    antennas: Vec<(Point, char)>,
}

fn find_antinodes(p1: Point, p2: Point, bounds: (usize, usize)) -> Vec<Point> {
    // the antinodes are like if you drew a line between the two points, found the distance, then went that same distance in the opposite direction (both sides)
    let x = p2.x - p1.x;
    let y = p2.y - p1.y;
    
    let mut out = Vec::new();

    let mut pos_1 = p1;

    out.push(p1);
    out.push(p2);
    
    while pos_1.x >= 0 && pos_1.y >= 0 && pos_1.x < bounds.0 as i32 && pos_1.y < bounds.1 as i32 {
        pos_1.x -= x;
        pos_1.y -= y;
        out.push(pos_1);
    }

    let mut pos_2 = p2;

    while pos_2.x >= 0 && pos_2.y >= 0 && pos_2.x < bounds.0 as i32 && pos_2.y < bounds.1 as i32 {
        pos_2.x += x;
        pos_2.y += y;
        out.push(pos_2);
    }

    out
}

fn create_map(input: &str) -> Map {
    let mut map = Vec::new();
    let mut antennas = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push(if c == '.' { Item::Empty } else { Item::Antenna(c) });
            if c != '.' {
                antennas.push((Point { x: (row.len()-1) as i32, y: map.len() as i32 }, c));
            }
        }
        map.push(row);
    }

    Map { map, antennas }
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    // create map
    let map = create_map(&input);

    // find antinodes
    let mut antinodes = HashSet::new();

    for i in 0..map.antennas.len() {
        for j in i+1..map.antennas.len() {
            let (p1, a1) = map.antennas[i];
            let (p2, a2) = map.antennas[j];

            if a1 != a2 { continue; }
            if p1 == p2 { continue; }

            let points = find_antinodes(p1, p2, (map.map[0].len(), map.map.len()));


            for point in points {
                if point.x >= 0 && point.y >= 0 && point.x < map.map[0].len() as i32 && point.y < map.map.len() as i32 {
                    antinodes.insert(point);
                }
            }
        }
    }

    println!("Antinodes: {:?}", antinodes.len());

    //display_map(&map, &antinodes);
}


fn display_map(map: &Map, antinodes: &HashSet<Point>) {
    for (y, row) in map.map.iter().enumerate() {
        for (x, item) in row.iter().enumerate() {
            if antinodes.contains(&Point { x: x as i32, y: y as i32 }) {
                print!("#");
            } else {
                match item {
                    Item::Empty => print!("."),
                    Item::Antenna(c) => print!("{}", c),
                }
            }
        }
        println!();
    }
}
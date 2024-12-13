#![allow(dead_code)]

use std::{collections::HashSet, hash::Hash};

#[derive(Debug, Clone)]
struct Grid {
    map: Vec<Vec<char>>,
    debug: Vec<Vec<char>>,
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

impl Grid {
    fn new(map: Vec<Vec<char>>) -> Self {
        Self { map: map.clone(), debug: map }
    }

    fn generate(input: &str) -> Self {
        let map = input
            .lines()
            .map(|line| line.chars().collect())
            .collect();
        Self::new(map)
    }

    pub fn print(&self, debug: bool) {
        if !debug {
            for row in &self.map {
                for cell in row {
                    print!("{}", cell);
                }
                println!();
            }
        } else {
            for row in &self.debug {
                for cell in row {
                    print!("{}", cell);
                }
                println!();
            }
        }
    }

    fn get_cell(&self, x: usize, y: usize) -> Option<char> {
        self.map.get(y).and_then(|row| row.get(x).copied())
    }

    fn get_debug_cell(&self, x: usize, y: usize) -> Option<char> {
        self.debug.get(y).and_then(|row| row.get(x).copied())
    }

    fn set_debug_cell(&mut self, x: usize, y: usize, value: char) {
        if self.get_cell(x, y).is_some() {
            self.debug[y][x] = value;
        }
    }

    fn find_area_of_region_recursive(&self, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> i64 {
        visited.insert((x, y));

        let self_cell = self.get_cell(x, y).unwrap();

        let mut area = 1;
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_x = (x as i64 + dx) as usize;
            let new_y = (y as i64 + dy) as usize;
            let cell = self.get_cell(new_x, new_y);
            if cell == Some(self_cell) && !visited.contains(&(new_x, new_y)) {
                area += self.find_area_of_region_recursive(new_x, new_y, visited);
            }
        }

        area
    }

    fn find_perimeter_of_region_recursive(&self, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> i64 {
        visited.insert((x, y));

        let self_cell = self.get_cell(x, y).unwrap();

        let mut perimeter = 0;
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let new_x = (x as i64 + dx) as usize;
            let new_y = (y as i64 + dy) as usize;
            let cell = self.get_cell(new_x, new_y);
            if cell != Some(self_cell) {
                perimeter += 1;
            } else if !visited.contains(&(new_x, new_y)) {
                perimeter += self.find_perimeter_of_region_recursive(new_x, new_y, visited);
            }
        }

        perimeter
    }

    fn find_sides_of_region_wrapper(&mut self, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> i64 {  
        let items = self.find_sides_of_region_recursive(x, y, visited);
        
        let mut sides = 0;

        for (x, y, dir) in items.iter() {
            match *dir {
                Dir::Up => {
                    if !items.contains(&(*x+1, *y, Dir::Up)) {
                        sides += 1;
                    }
                },
                Dir::Down => {
                    if !items.contains(&(*x+1, *y, Dir::Down)) {
                        sides += 1;
                    }
                },
                Dir::Left => {
                    if !items.contains(&(*x, *y+1, Dir::Left)) {
                        sides += 1;
                    }
                },
                Dir::Right => {
                    if !items.contains(&(*x, *y+1, Dir::Right)) {
                        sides += 1;
                    }
                },
            }
        }

        sides
    }

    fn find_sides_of_region_recursive(&mut self, x: usize, y: usize, visited: &mut HashSet<(usize, usize)>) -> HashSet<(usize, usize, Dir)> {
        visited.insert((x, y));

        let self_cell = self.get_cell(x, y).unwrap();

        let mut cells = HashSet::new();

        for direction in [Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            let (dx, dy) = match direction {
                Dir::Up => (0, 1),
                Dir::Down => (0, -1),
                Dir::Left => (-1, 0),
                Dir::Right => (1, 0),
            };

            let new_x = (x as i64 + dx) as usize;
            let new_y = (y as i64 + dy) as usize;
            let cell = self.get_cell(new_x, new_y);
            if cell != Some(self_cell) {
                cells.insert((x, y, direction));
            } else if !visited.contains(&(new_x, new_y)) {
                cells.extend(self.find_sides_of_region_recursive(new_x, new_y, visited));
            }
        }

        cells
    }
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut grid = Grid::generate(&input);

    let mut area_visited = HashSet::new();
    let mut perimeter_visited = HashSet::new();
    let mut total_cost = 0;
    for y in 0..grid.map.len() {
        for x in 0..grid.map[y].len() {
            if !area_visited.contains(&(x, y)) {
                let area = grid.find_area_of_region_recursive(x, y, &mut area_visited);
                let sides = grid.find_sides_of_region_wrapper(x, y, &mut perimeter_visited);
                println!("Area: {}, Sides: {}, Char: {}", area, sides, grid.get_cell(x, y).unwrap());
                total_cost += area * sides;
            }
        }
    }

    grid.print(true);

    println!("Total cost: {}", total_cost);
}

#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
struct Guard {
    x: i32,
    y: i32,
    facing: i8,
}

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<i8>>,
}

struct PreviousPosition {
    x: i32,
    y: i32,
    facing: i8,
}

fn generate_map(input: &str) -> (Map, Guard) {
    let mut grid = Vec::new();
    let mut guard = Guard { x: 0, y: 0, facing: 0 };

    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            match c {
                '#' => row.push(1),
                '.' => row.push(0),
                '^' => {
                    row.push(0);
                    guard.x = row.len() as i32 - 1;
                    guard.y = grid.len() as i32;
                },
                _ => panic!("Invalid character in input"),
            }
        }
        grid.push(row);
    }

    (Map { grid }, guard)
}

fn get_next_move(map: &Map, guard: &mut Guard, previous_positions: &mut Vec<PreviousPosition>) -> (bool, bool, bool) { // moved, out of range, loop
    // check position in front of guard
    let (f_x, f_y) = match guard.facing {
        0 => (guard.x, guard.y - 1),
        1 => (guard.x + 1, guard.y),
        2 => (guard.x, guard.y + 1),
        3 => (guard.x - 1, guard.y),
        _ => panic!("Invalid facing value"),
    };

    if f_x >= map.grid[0].len() as i32 || f_y >= map.grid.len() as i32 || f_x < 0 || f_y < 0 {
        return (false, false, false);
    }

    if previous_positions.iter().any(|p| p.x == f_x && p.y == f_y && p.facing == guard.facing) {
        return (false, false, true);
    }

    // check if position in front of guard is a wall
    let ret = if map.grid[f_y as usize][f_x as usize] == 1 {
        // turn right
        guard.facing = (guard.facing + 1) % 4;
        (false, true, false)
    } else {
        // move forward
        match guard.facing {
            0 => guard.y -= 1,
            1 => guard.x += 1,
            2 => guard.y += 1,
            3 => guard.x -= 1,
            _ => panic!("Invalid facing value"),
        }
        (true, true, false)
    };

    previous_positions.push(PreviousPosition { x: guard.x, y: guard.y, facing: guard.facing });

    ret
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (map, mut guard) = generate_map(&input);

    // move guard
    let mut loops = 0;
    for y in 0..map.grid.len() {
        for x in 0..map.grid[y].len() {
            let mut new_map = map.clone();
            let mut new_guard = guard.clone();

            if new_map.grid[y][x] == 1 {
                continue;
            }
            if x == guard.x as usize && y == guard.y as usize {
                continue;
            }
            new_map.grid[y][x] = 1;

            let mut previous_positions = Vec::new();

            loop {
                let (_, moved, in_loop) = get_next_move(&new_map, &mut new_guard, &mut previous_positions);

                if in_loop {
                    loops += 1;
                    break;
                }

                if !moved {
                    break;
                }
            }
        }
        println!("loops: {}", loops);
    }

    println!("loops: {}", loops);
}


fn display_map(map: &Map) {
    for row in &map.grid {
        for cell in row {
            print!("{}", if *cell == 0 { '.' } else if *cell == 1 { '#' } else { 'X' });
        }
        println!();
    }
}
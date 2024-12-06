#![allow(dead_code)]

use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Guard {
    x: i32,
    y: i32,
    facing: i8,
}

#[derive(Debug, Clone)]
struct Map {
    grid: Vec<Vec<i8>>,
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

fn get_next_move(map: &Map, guard: &mut Guard, previous_positions: &mut HashSet<Guard>) -> ((i32, i32), bool, bool, bool) { // moved, out of range, loop
    // check position in front of guard
    let (f_x, f_y) = match guard.facing {
        0 => (guard.x, guard.y - 1),
        1 => (guard.x + 1, guard.y),
        2 => (guard.x, guard.y + 1),
        3 => (guard.x - 1, guard.y),
        _ => panic!("Invalid facing value"),
    };

    if f_x >= map.grid[0].len() as i32 || f_y >= map.grid.len() as i32 || f_x < 0 || f_y < 0 {
        return ((-1,-1),false, false, false);
    }

    if previous_positions.contains(&guard) {
        return ((-1,-1),false, false, true);
    }

    previous_positions.insert(guard.clone());

    // check if position in front of guard is a wall
    let ret = if map.grid[f_y as usize][f_x as usize] == 1 {
        // turn right
        guard.facing = (guard.facing + 1) % 4;
        ((-1,-1),false, true, false)
    } else {
        // move forward
        match guard.facing {
            0 => guard.y -= 1,
            1 => guard.x += 1,
            2 => guard.y += 1,
            3 => guard.x -= 1,
            _ => panic!("Invalid facing value"),
        }
        ((guard.x, guard.y),true, true, false)
    };

    ret
}

fn round(map: &mut Map, guard: &mut Guard, x:i32, y:i32) -> bool {
    let prev_guard_info = (guard.x, guard.y, guard.facing);

    if map.grid[y as usize][x as usize] == 1 {
        return false;
    }
    if x == guard.x && y == guard.y {
        return false;
    }
    map.grid[y as usize][x as usize] = 1;

    let mut previous_positions = HashSet::new();

    loop {
        let (_, _, moved, in_loop) = get_next_move(&map, guard, &mut previous_positions);

        if in_loop {
            map.grid[y as usize][x as usize] = 0;
            guard.x = prev_guard_info.0;
            guard.y = prev_guard_info.1;
            guard.facing = prev_guard_info.2;
            return true;
        }

        if !moved {
            map.grid[y as usize][x as usize] = 0;
            guard.x = prev_guard_info.0;
            guard.y = prev_guard_info.1;
            guard.facing = prev_guard_info.2;
            return false;
        }
    }
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();
    let (mut map, mut guard) = generate_map(&input);

    // generate movement positions
    let mut positions = Vec::new();
    let mut guard_copy = guard.clone();
    let mut unused = HashSet::new();
    loop {
        let (pos, _, not_at_end, _) = get_next_move(&map, &mut guard_copy, &mut unused);

        if (pos.0, pos.1) != (-1, -1) {
            positions.push((pos.0, pos.1));
        }

        if !not_at_end {
            break;
        }
    }

    // move guard
    let mut loops = 0;
    //let mut iters = 0;
    //let pos_len = positions.len();
    for (x, y) in positions {
        if round(&mut map, &mut guard, x, y) {
            loops += 1;
        }

        //iters += 1;
        //println!("round {}/{}", iters, pos_len);
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    Box(bool), // false for left, true for right
    Robot,
}

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    moves: Vec<Move>,
    robot_pos: (usize, usize),
}

impl Map {
    pub fn new(tiles: Vec<Vec<Tile>>, moves: Vec<Move>, robot_pos: (usize, usize)) -> Self {
        Self { tiles, moves, robot_pos }
    }

    pub fn parse_input(input: &str) -> Self {
        let mut tiles = Vec::new();
        let mut moves = Vec::new();
        let mut robot_pos = (0, 0);

        let mut moves_mode = false;
        for line in input.lines() {
            let mut row = Vec::new();

            if line.is_empty() {
                moves_mode = true;
                continue;
            }

            for c in line.chars() {
                if !moves_mode {
                    match c {
                        '#' => {
                            row.push(Tile::Wall);
                            row.push(Tile::Wall);
                        },
                        '.' => {
                            row.push(Tile::Empty);
                            row.push(Tile::Empty);
                        }
                        'O' => {
                            row.push(Tile::Box(false));
                            row.push(Tile::Box(true));
                        }
                        '@' => {
                            robot_pos = (row.len(), tiles.len());
                            row.push(Tile::Robot);
                            row.push(Tile::Empty);
                        }
                        _ => panic!("Invalid character in input, '{}'", c),
                    }
                } else {
                    match c {
                        '^' => moves.push(Move::Up),
                        'v' => moves.push(Move::Down),
                        '<' => moves.push(Move::Left),
                        '>' => moves.push(Move::Right),
                        _ => panic!("Invalid character in input"),
                    }
                }
            }

            if !moves_mode { tiles.push(row); }
        }

        Self::new(tiles, moves, robot_pos)
    }

    pub fn print(&self) {
        for row in &self.tiles {
            for tile in row {
                match tile {
                    Tile::Wall => print!("#"),
                    Tile::Empty => print!("."),
                    Tile::Box(side) => {
                        if *side {
                            print!("]");
                        } else {
                            print!("[");
                        }
                    }
                    Tile::Robot => print!("@"),
                }
            }
            println!();
        }

        /*
        for m in &self.moves {
            match m {
                Move::Up => print!("^"),
                Move::Down => print!("v"),
                Move::Left => print!("<"),
                Move::Right => print!(">"),
            }
        }
        println!();
        */
    }

    fn move_recursive(&mut self, pos: (usize, usize), dir: (isize, isize), actually_move: bool) -> bool {
        let (dx, dy) = dir;
        let current_type = self.tiles[pos.1][pos.0];
        
        //println!("current_type: {:?}", current_type);

        if let Tile::Box(side) = current_type {
            let (left, right) = if side {
                (pos.0 - 1, pos.0)
            } else {
                (pos.0, pos.0 + 1)
            };

            let next_left = ((left as isize + dx) as usize, (pos.1 as isize + dy) as usize);
            let next_right = ((right as isize + dx) as usize, (pos.1 as isize + dy) as usize);

            let new_y = (pos.1 as isize + dy) as usize;

            let going_left = dir == (-1, 0);
            let going_right = dir == (1, 0);

            let left_success = if going_right { true } else {
                self.move_recursive(next_left, dir, false)
            };

            let right_success = if going_left { true } else {                
                self.move_recursive(next_right, dir, false)
            };

            if left_success && right_success {
                if actually_move {
                    if going_right {
                        self.tiles[next_left.1][next_left.0] = Tile::Box(false);
                        // recurse
                        self.move_recursive(next_right, dir, true);
                        self.tiles[next_right.1][next_right.0] = Tile::Box(true);

                        // reset
                        self.tiles[new_y][left] = Tile::Empty;
                    }
                    else if going_left {
                        self.tiles[next_right.1][next_right.0] = Tile::Box(true);
                        // recurse
                        self.move_recursive(next_left, dir, true);
                        self.tiles[next_left.1][next_left.0] = Tile::Box(false);

                        // reset
                        self.tiles[new_y][right] = Tile::Empty;
                    }
                    else {
                        // recurse
                        self.move_recursive(next_left, dir, true);
                        self.move_recursive(next_right, dir, true);

                        self.tiles[next_left.1][next_left.0] = Tile::Box(false);
                        self.tiles[next_right.1][next_right.0] = Tile::Box(true);

                        // reset
                        self.tiles[pos.1][left] = Tile::Empty;
                        self.tiles[pos.1][right] = Tile::Empty;
                    }
                }

                return true;
            }

            return false;
        }

        if current_type == Tile::Wall {
            return false;
        }

        if current_type == Tile::Empty {
            return true;
        }

        if current_type == Tile::Robot {
            // recurse
            let next_pos = ((pos.0 as isize + dx) as usize, (pos.1 as isize + dy) as usize);
            let can = self.move_recursive(next_pos, dir, actually_move);

            if can && actually_move {
                self.tiles[next_pos.1][next_pos.0] = Tile::Robot;
                self.tiles[pos.1][pos.0] = Tile::Empty;
            }

            return can;
        }

        false
    }

    pub fn run_turn(&mut self) -> bool {
        if self.moves.is_empty() {
            return false;
        }

        let next_move = self.moves.remove(0);

        //println!("Next move: {:?}", next_move);

        let (dx, dy) = match next_move {
            Move::Up => (0, -1),
            Move::Down => (0, 1),
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
        };

        if self.move_recursive(self.robot_pos, (dx, dy), true) {
            self.robot_pos = ((self.robot_pos.0 as isize + dx) as usize, (self.robot_pos.1 as isize + dy) as usize);
        }

        true
    }

    fn find_gps(&self, pos: (usize, usize)) -> usize {
        return pos.1 * 100 + pos.0;
    }

    pub fn gps_sum(&self) -> usize {
        let mut boxes = Vec::new();

        for (y, row) in self.tiles.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                if *tile == Tile::Box(false) {
                    boxes.push((x, y));
                }
            }
        }

        boxes.iter().map(|&pos| self.find_gps(pos)).sum()
    }
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    // parse input
    let mut map = Map::parse_input(&input);

    map.print();

    // run turns
    while map.run_turn() {  /*map.print();*/  }

    map.print();

    // print sum
    println!("{}", map.gps_sum());
}

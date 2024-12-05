#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn up(&self, amount: isize) -> Point {
        Point::new(self.x, self.y - amount)
    }

    fn down(&self, amount: isize) -> Point {
        Point::new(self.x, self.y + amount)
    }

    fn left(&self, amount: isize) -> Point {
        Point::new(self.x - amount, self.y)
    }

    fn right(&self, amount: isize) -> Point {
        Point::new(self.x + amount, self.y)
    }

    fn diag_up_left(&self, amount: isize) -> Point {
        Point::new(self.x - amount, self.y - amount)
    }

    fn diag_up_right(&self, amount: isize) -> Point {
        Point::new(self.x + amount, self.y - amount)
    }

    fn diag_down_left(&self, amount: isize) -> Point {
        Point::new(self.x - amount, self.y + amount)
    }

    fn diag_down_right(&self, amount: isize) -> Point {
        Point::new(self.x + amount, self.y + amount)
    }
}

#[derive(Debug)]
struct Map(Vec<Vec<(char, bool)>>);

impl Map {
    fn get(&self, idx: Point) -> char {
        if idx.x >= self.0[0].len() as isize || idx.y >= self.0.len() as isize || idx.x < 0 || idx.y < 0 {
            return ' ';
        }
        self.0[idx.y as usize][idx.x as usize].0
    }

    fn set(&mut self, idx: Point, value: bool, ch_val: char) {
        self.0[idx.y as usize][idx.x as usize] = (ch_val, value);
    }
}

fn check_letter(idx: Point, arr: &mut Map) -> bool {
    // look for
    /*
    2 MAS's crossing
    ex:
    M.S
    .A.
    M.S

    S.S
    .A.
    M.M
     */

    if arr.get(idx) != 'A' {
        return false;
    }

    let left_down = arr.get(idx.diag_up_left(1)) == 'M' && arr.get(idx.diag_down_right(1)) == 'S';
    let right_down = arr.get(idx.diag_up_right(1)) == 'M' && arr.get(idx.diag_down_left(1)) == 'S';
    let left_up = arr.get(idx.diag_down_left(1)) == 'M' && arr.get(idx.diag_up_right(1)) == 'S';
    let right_up = arr.get(idx.diag_down_right(1)) == 'M' && arr.get(idx.diag_up_left(1)) == 'S';

    if (right_down || left_up) && (left_down || right_up) {
        arr.set(idx, true, '@');
        // set all the other points
        arr.set(idx.diag_up_left(1), true, arr.get(idx.diag_up_left(1)));
        arr.set(idx.diag_down_right(1), true, arr.get(idx.diag_down_right(1)));
        arr.set(idx.diag_up_right(1), true, arr.get(idx.diag_up_right(1)));
        arr.set(idx.diag_down_left(1), true, arr.get(idx.diag_down_left(1)));
    }

    return (right_down || left_up) && (left_down || right_up);
}

fn generate_map(input: &str) -> Map {
    let mut map = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for c in line.chars() {
            row.push((c, false));
        }
        map.push(row);
    }
    Map(map)
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    // generate map
    let mut map = generate_map(&input);

    // check for XMAS
    let mut found = 0;
    for y in 0..map.0.len() {
        for x in 0..map.0[y].len() {
            found += if check_letter(Point::new(x as isize, y as isize), &mut map) { 1 } else { 0 };
        }
    }

    /*
    // loop through map
    for row in map.0.iter() {
        for (c, p) in row.iter() {
            if *p {
                print!("{}", *c);
            } else {
                print!(".");
            }
        }
        println!();
    }
    */

    println!("Found {} XMAS", found);
}

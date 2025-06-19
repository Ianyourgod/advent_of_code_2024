const MIN_SAVE: u64 = 100;

#[inline]
fn opb(s: Option<bool>) -> bool {
    match s {
        Some(b) => b,
        None => false,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    pub x: i64,
    pub y: i64,
}

impl Point {
    pub fn new(x: i64, y: i64) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn default() -> Self {
        Self::new(0, 0)
    }

    pub fn up(&self) -> Self {
        Self::new(self.x, self.y-1)
    }

    pub fn down(&self) -> Self {
        Self::new(self.x, self.y+1)
    }

    pub fn left(&self) -> Self {
        Self::new(self.x-1, self.y)
    }

    pub fn right(&self) -> Self {
        Self::new(self.x+1, self.y)
    }

    fn nearby_points<'a>(
        &'a self,
        max_x: i64,
        max_y: i64,
        range: i64,
    ) -> impl Iterator<Item = (Point, i64)> + 'a {
        (-range..=range).flat_map(move |dx: i64| {
            let dy_range = range - dx.abs();
            (-dy_range..=dy_range).filter_map(move |dy| {
                let new_x = self.x + dx;
                let new_y = self.y + dy;
                let dist = dy.abs() + dx.abs();

                if new_x >= 0 && new_x <= max_x && new_y >= 0 && new_y <= max_y {
                    Some((Point { x: new_x, y: new_y }, dist))
                } else {
                    None
                }
            })
        })
    }
}

#[derive(Debug)]
struct Map {
    map: [[Option<i64>; 141]; 141],
    start: Point,
    end: Point
}

impl Map {
    pub fn from_str(input: &str) -> Self {
        let mut map = Self {
            map: [[None;141];141],
            start: Point::default(),
            end: Point::default()
        };

        for (y, line) in input.lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                match c {
                    '#' => (),
                    '.' => {
                        map.map[y][x] = Some(0);
                    },
                    'S' => {
                        map.map[y][x] = Some(0);
                        map.start = Point::new(x as i64, y as i64);
                    },
                    'E' => {
                        map.map[y][x] = Some(0);
                        map.end = Point::new(x as i64, y as i64);
                    },
                    _ => unreachable!()
                }
            }
        }

        let mut cur_point = map.end;
        let mut cur_dist = 0;
        let mut prev_dir = -1;
        while cur_point != map.start {
            let up = cur_point.up();
            let down = cur_point.down();
            let left = cur_point.left();
            let right = cur_point.right();
            cur_point = 
                 if map.get_point(&up)   .is_some() && prev_dir != 0 { prev_dir = 1; up    }
            else if map.get_point(&down) .is_some() && prev_dir != 1 { prev_dir = 0; down  }
            else if map.get_point(&left) .is_some() && prev_dir != 2 { prev_dir = 3; left  }
            else if map.get_point(&right).is_some() && prev_dir != 3 { prev_dir = 2; right }
            else { unreachable!() };

            cur_dist += 1;
            map.set_point(&cur_point, Some(cur_dist));
        }

        map
    }

    #[inline]
    pub fn get_point(&self, p: &Point) -> Option<i64> {
        if self.map.len() as i64 <= p.y || self.map[0].len() as i64 <= p.x || p.y <= 0 || p.x <= 0 { return None; }
        self.map[p.y as usize][p.x as usize]
    }

    #[inline]
    pub fn set_point(&mut self, p: &Point, v: Option<i64>) {
        self.map[p.y as usize][p.x as usize] = v;
    }

    #[allow(unused)]
    pub fn display(&self) -> String {
        let mut output = String::new();
        
        for line in self.map {
            for c in line {
                output.push(match c {
                    None => '#',
                    Some(x) => {
                        (x % 10).to_string().chars().nth(0).unwrap()
                    }
                });
            }

            output.push('\n');
        }

        output
    }

    pub fn find_saves(&self, min: u64) -> u64 {
        let mut count = 0;
        let min = min as i64;

        let map_max = self.map.len();

        // i64 is needed because the next thing might be higher than current

        for (y, row) in self.map.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if let Some(item) = item {
                    let item = *item as i64;
                    let item = item;

                    let point = Point::new(x as i64, y as i64);

                    for (p, dist) in point.nearby_points(map_max as i64, map_max as i64, 20) {
                        if opb(self.get_point(&p).map(|a|item-dist-a >= min)) {
                            count += 1;
                        }
                    }
                }
            }
        }

        count
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let map = Map::from_str(&input);

    println!("{}", map.find_saves(MIN_SAVE));
}

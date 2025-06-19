use std::collections::{HashMap, HashSet};

const SIZE: (isize, isize) = (71, 71);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    pub x: isize,
    pub y: isize
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self {
            x,
            y
        }
    }

    pub fn from_str(line: &str) -> Self {
        let mut l = line.split(',');
        let x = l.next().unwrap().parse().unwrap();
        let y = l.next().unwrap().parse().unwrap();
        Self::new(x, y)
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
}

fn get_bytes(input: &str, n: usize) -> impl std::iter::Iterator<Item = Point> + '_ {
    input
        .lines()
        .take(n)
        .map(|line| Point::from_str(line))
}

struct Map {
    obstacles: HashSet<Point>
}

impl Map {
    pub fn new(o: HashSet<Point>) -> Self {
        Self {
            obstacles: o
        }
    }

    pub fn can_move_to_point(&self, p: &Point) -> bool {
        if p.x >= SIZE.0 || p.y >= SIZE.1 { return false; }

        !self.obstacles.contains(p)
    }
}

#[inline]
fn min(a: Option<u64>, b: u64) -> u64 {
    if let Some(a) = a {
        if a < b { a } else { b }
    } else { b }
}

fn find_easist_path<O: std::iter::Iterator<Item = Point>>(obstacles: O) -> Option<u64> {
    let map = Map::new(obstacles.collect::<HashSet<_>>());

    let mut queue = std::collections::VecDeque::new();
    let mut visited = HashMap::new();

    let start = Point::new(0, 0);
    queue.push_back((start, 0));
    visited.insert(start, 0);

    let mut shortest_path = None;

    while let Some((pos, score)) = queue.pop_front() {

        if pos == Point::new(SIZE.0-1, SIZE.1-1) {
            shortest_path = Some(min(shortest_path, score));
            continue;
        }
        
        let up = pos.up();
        let down = pos.down();
        let left = pos.left();
        let right = pos.right();

        let new_score = score+1;
        if can_do_point(&map, &visited, &up, new_score) {
            queue.push_back((up, new_score));
            visited.insert(up, new_score);
        }
        if can_do_point(&map, &visited, &down, new_score) {
            queue.push_back((down, new_score));
            visited.insert(down, new_score);
        }
        if can_do_point(&map, &visited, &left, new_score) {
            queue.push_back((left, new_score));
            visited.insert(left, new_score);
        }
        if can_do_point(&map, &visited, &right, new_score) {
            queue.push_back((right, new_score));
            visited.insert(right, new_score);
        }
    }

    shortest_path
}

fn can_do_point(map: &Map, visited: &HashMap<Point, u64>, point: &Point, score: u64) -> bool {
    if !map.can_move_to_point(point) {
        return false;
    }
    
    if let Some(&prev) = visited.get(point) {
        return prev > score;
    } else {
        return !(
            point.x < 0       ||
            point.x >= SIZE.0 ||
            point.y < 0       ||
            point.y >= SIZE.1 );
    }
}

fn main() {
    use std::time::Instant;
    let now = Instant::now();

    {
        let input = std::fs::read_to_string("input.txt").unwrap();

        let mut max = 3450;
        let mut min = 1;
        let mut prev_mid = 0;
        let breaker = loop {
            let mid = (max + min) / 2;

            if mid == prev_mid {
                let mut bytes = get_bytes(&input, mid+1);
                break bytes.nth(mid).unwrap();
            }
            
            let bytes = get_bytes(&input, mid);

            let easiest_path = find_easist_path(bytes);

            

            if easiest_path.is_none() {
                max = mid;
            } else {
                min = mid;
            }

            prev_mid = mid;
        };

        println!("{},{}", breaker.x, breaker.y);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
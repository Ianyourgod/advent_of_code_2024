use std::fmt::Display;

#[derive(Debug, Clone)]
struct Trail {
    map: Vec<Vec<u8>>,
    heads: Vec<(usize, usize)>,
}

impl Trail {
    fn new(input: &str) -> Self {
        // big block of numbers
        let map: Vec<Vec<u8>> = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        // find all heads (all 0)
        let heads = map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &cell)| cell == 0)
                    .map(move |(x, _)| (x, y))
            })
            .collect();

        Self { map, heads }
    }
}

impl Display for Trail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.map {
            for cell in row {
                write!(f, "{}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    // create a new Trail
    let trail = Trail::new(&input);

    // find all paths
    let paths = find_paths(&trail);

    println!("Paths: {}", paths);
}

fn find_paths(trail: &Trail) -> i32 {
    // loop over heads
    let mut count = 0;
    for head in &trail.heads {
        //println!("Head: {:?}", head);
        let scored = recurse(trail, head.clone());
        count += scored;
        //println!("Scored: {}", scored);
    }

    count
}

fn recurse(trail: &Trail, position: (usize, usize)) -> i32 {
    let current_value = trail.map[position.1][position.0];

    // if we hit a 9, we're done
    if current_value == 9 {
        return 1;
    }

    let mut count = 0;

    if position.1 > 0 {
        let up = (position.0, position.1 - 1);
        let up = (up, trail.map[up.1][up.0]);
        if up.1 == current_value + 1 {
            count += recurse(trail, up.0);
        }
    }

    if position.1 < trail.map.len() - 1 {
        let down = (position.0, position.1 + 1);
        let down = (down, trail.map[down.1][down.0]);
        if down.1 == current_value + 1 {
            count += recurse(trail, down.0);
        }
    }
    
    if position.0 > 0 {
        let left = (position.0 - 1, position.1);
        let left = (left, trail.map[left.1][left.0]);
        if left.1 == current_value + 1 {
            count += recurse(trail, left.0);
        }
    }

    if position.0 < trail.map[0].len() - 1 {
        let right = (position.0 + 1, position.1);
        let right = (right, trail.map[right.1][right.0]);
        if right.1 == current_value + 1 {
            count += recurse(trail, right.0);
        }
    }

    count
}
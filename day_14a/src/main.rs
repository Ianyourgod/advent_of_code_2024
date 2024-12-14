#![allow(dead_code)]

#[derive(Debug, Clone, Copy)]
struct Robot {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
}

impl Robot {
    pub fn new(x: i32, y: i32, dx: i32, dy: i32) -> Self {
        Self { x, y, dx, dy }
    }

    pub fn parse_input(input: &str) -> Self {
        // skip first 2 characters
        let input = &input[2..];
        
        let inputs = input.split(" v=").collect::<Vec<&str>>();

        let pos_input = inputs[0].split(",").collect::<Vec<&str>>();
        let dir_input = inputs[1].split(",").collect::<Vec<&str>>();

        let x = pos_input[0].parse::<i32>().unwrap();
        let y = pos_input[1].parse::<i32>().unwrap();
        let dx = dir_input[0].parse::<i32>().unwrap();
        let dy = dir_input[1].parse::<i32>().unwrap();

        Self::new(x, y, dx, dy)
    }
}

#[derive(Debug, Clone)]
struct Map {
    width: usize,
    height: usize,
    robots: Vec<Robot>,
}

impl Map {
    pub fn find_robots_in_quads(&self) -> (u32, u32, u32, u32) {
        let mut q1 = 0;
        let mut q2 = 0;
        let mut q3 = 0;
        let mut q4 = 0;

        // we dont count things directly on the axis
        let q1_bound = ((0., (self.width / 2) as f32), (0., (self.height / 2) as f32));
        let q2_bound = (((self.width as f32 / 2.), self.width as f32), (0., (self.height / 2) as f32));
        let q3_bound = ((0., (self.width / 2) as f32), ((self.height as f32 / 2.), self.height as f32));
        let q4_bound = (((self.width as f32 / 2.), self.width as f32), ((self.height as f32 / 2.), self.height as f32));

        let bounds = [q1_bound, q2_bound, q3_bound, q4_bound];

        for robot in &self.robots {
            for (i, bound) in bounds.iter().enumerate() {
                if (robot.x as f32) >= bound.0.0
                    && (robot.x as f32) < bound.0.1
                    && (robot.y as f32) >= bound.1.0
                    && (robot.y as f32) < bound.1.1
                {
                    match i {
                        0 => q1 += 1,
                        1 => q2 += 1,
                        2 => q3 += 1,
                        3 => q4 += 1,
                        _ => (),
                    }
                }
            }
        }

        (q1, q2, q3, q4)
    }

    pub fn run_second(&mut self) {
        for robot in &mut self.robots {
            robot.x = robot.x + robot.dx;
            robot.y = robot.y + robot.dy;

            if robot.x >= self.width as i32 {
                robot.x -= self.width as i32;
            }
            if robot.x < 0 {
                robot.x += self.width as i32;
            }
            if robot.y >= self.height as i32 {
                robot.y -= self.height as i32;
            }
            if robot.y < 0 {
                robot.y += self.height as i32;
            }
        }
    }

    pub fn parse_input(input: &str, bounds: (usize, usize)) -> Self {
        let robots = input.lines().map(|line| Robot::parse_input(line)).collect();

        Self {
            width: bounds.0,
            height: bounds.1,
            robots,
        }
    }

    pub fn print(&self) {
        let mut map = vec![vec!['.'; self.width]; self.height];

        for robot in &self.robots {
            if map[robot.y as usize][robot.x as usize] == '.' {
                map[robot.y as usize][robot.x as usize] = '1';
            } else {
                map[robot.y as usize][robot.x as usize] = map[robot.y as usize][robot.x as usize]
                    .to_digit(10)
                    .unwrap()
                    .wrapping_add(1)
                    .to_string()
                    .chars()
                    .next()
                    .unwrap();
            }
        }

        for row in map {
            for cell in row {
                print!("{}", cell);
            }
            println!();
        }
    }
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();
    let bounds = (101, 103);

    let mut map = Map::parse_input(&input, bounds);

    let seconds = 8000;
    let mut lowest = (std::u32::MAX, -1);

    for i in 0..seconds {
        map.run_second();
        let (q1, q2, q3, q4) = map.find_robots_in_quads();
        let safety = q1 * q2 * q3 * q4;
        if safety < lowest.0 {
            lowest = (safety, i);
            map.print();
            println!("seconds: {}", i+1);
        }
    }
}

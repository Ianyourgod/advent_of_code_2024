#![allow(dead_code)]

#[derive(Debug, Clone)]
struct Machine {
    button_a: (f64, f64),
    button_b: (f64, f64),
    prize: (f64, f64),
}

impl Machine {
    fn new() -> Machine {
        Machine {
            button_a: (0., 0.),
            button_b: (0., 0.),
            prize: (0., 0.),
        }
    }

    fn parse(input: &str) -> Self {
        // split by lines
        let lines: Vec<&str> = input.lines().collect();

        let line_1: &str = lines[0];
        let line_2: &str = lines[1];
        let line_3: &str = lines[2];

        // skip the first 12 characters
        let button_a: &str = &line_1[12..line_1.len()];
        let button_b: &str = &line_2[12..line_2.len()];

        // split by whitespace
        let button_a: Vec<&str> = button_a.split(", Y+").collect();

        let button_a_x: i64 = button_a[0].parse().unwrap();
        let button_a_y: i64 = button_a[1].parse().unwrap();

        let button_b: Vec<&str> = button_b.split(", Y+").collect();
        
        let button_b_x: i64 = button_b[0].parse().unwrap();
        let button_b_y: i64 = button_b[1].parse().unwrap();

        // skip the first 9 characters
        let prize: &str = &line_3[9..line_3.len()];
        // split by `, Y=`
        let prize: Vec<&str> = prize.split(", Y=").collect();

        let mut prize_x: i64 = prize[0].parse().unwrap();
        let mut prize_y: i64 = prize[1].parse().unwrap();

        prize_x += 10000000000000;
        prize_y += 10000000000000;

        Machine {
            button_a: (button_a_x as f64, button_a_y as f64),
            button_b: (button_b_x as f64, button_b_y as f64),
            prize: (prize_x as f64, prize_y as f64),
        }
    }

    fn find_amount(&self) -> Option<(i64, i64)> {
        let b = (self.button_a.1 * self.prize.0 - self.button_a.0 * self.prize.1) / (self.button_a.1 * self.button_b.0 - self.button_a.0 * self.button_b.1);
        let a = (self.prize.0 - b * self.button_b.0) / self.button_a.0;

        if (a - a.round()).abs() > 0.0001 || (b - b.round()).abs() > 0.0001 {
            return None;
        }

        Some((a as i64, b as i64))
    }
}

fn main() {
    let input: &str = include_str!("../input.txt");

    // split by lines
    let lines: Vec<&str> = input.lines().collect();

    let mut i = 0;
    let mut machines = Vec::new();
    while i < lines.len() {
        let machine = Machine::parse(&lines[i..i + 3].join("\n"));
        machines.push(machine);
        i += 4;
    }

    let mut total_tokens = 0;
    for machine in machines {
        println!("{:?}", machine);
        let (a, b) = match machine.find_amount() {
            Some((a, b)) => (a, b),
            None => {
                continue;
            }
        };
        total_tokens += a * 3 + b * 1;
    }

    println!("Total tokens: {}", total_tokens);
}

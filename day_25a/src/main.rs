#[derive(Debug, Clone)]
struct Schematic {
    pub heights: [u8; 5],
    pub is_lock: bool,
}

impl Schematic {
    pub fn new(heights: [u8; 5], is_lock: bool) -> Self {
        Schematic { heights, is_lock }
    }

    pub fn from_str(s: &str) -> Self {
        let mut lines = s.lines();

        let is_lock = lines.next().unwrap().starts_with('#');

        /*
        ####.
        ###..
        ##...
        #....
        heights:
        43210
        
         */

        let mut heights = [0; 5];
        
        for line in lines.take(5) {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    heights[i] += 1;
                }
            }
        }

        Schematic::new(heights, is_lock)
    }
}

struct Schematics {
    pub locks: Vec<Schematic>,
    pub keys: Vec<Schematic>,
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut schematics = Schematics {
        locks: Vec::new(),
        keys: Vec::new(),
    };

    for s in input.split("\n\n") {
        let schematic = Schematic::from_str(s);

        if schematic.is_lock {
            schematics.locks.push(schematic);
        } else {
            schematics.keys.push(schematic);
        }
    }

    let mut successes = 0;
    for lock in &schematics.locks {
        for key in &schematics.keys {
            let mut success = true;
            for (height_a, height_b) in lock.heights.iter().zip(key.heights.iter()) {
                if *height_a + *height_b > 5 {
                    success = false;
                    break;
                }
            }
            successes += success as u32;
        }
    }

    println!("Successes: {}", successes);
}

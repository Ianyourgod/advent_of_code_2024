use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl Color {
    fn from_char(s: char) -> Self {
        match s {
            'w' => Color::White,
            'u' => Color::Blue,
            'b' => Color::Black,
            'r' => Color::Red,
            'g' => Color::Green,
            _ => panic!("Invalid color"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Towel {
    colors: Vec<Color>,
}

impl Towel {
    fn from_str(s: &str) -> Self {
        let colors = s.chars().map(|c| Color::from_char(c)).collect();
        Towel { colors }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Pattern {
    pub colors: Vec<Color>,
}

impl Pattern {
    fn from_str(s: &str) -> Self {
        let colors = s.chars().map(|c| Color::from_char(c)).collect();
        Pattern { colors }
    }
}

struct Cache<T: Copy> {
    pub cache: HashMap<Pattern, T>,
}

impl<T: Copy> Cache<T> {
    fn new() -> Self {
        Cache {
            cache: HashMap::new(),
        }
    }

    fn get(&self, pattern: &Pattern) -> Option<T> {
        self.cache.get(pattern).copied()
    }

    fn insert(&mut self, pattern: Pattern, result: T) {
        self.cache.insert(pattern, result);
    }
}

fn try_wrapper(pattern: &Pattern, towels: &Vec<Towel>, level: u64, cache: &mut Cache<u64>) -> u64 {
    if cache.get(pattern).is_some() {
        return cache.get(pattern).unwrap();
    }

    let result = try_pattern(pattern, towels, level, cache);

    cache.insert(pattern.clone(), result);

    result
}

fn try_pattern(pattern: &Pattern, towels: &Vec<Towel>, level: u64, cache: &mut Cache<u64>) -> u64 {
    if pattern.colors.len() == 0 {
        return 1;
    }

    let mut total = 0;

    for towel in towels.iter() {
        // check if the towel equals the start of the pattern
        if towel.colors.len() > pattern.colors.len() {
            continue;
        }

        let mut found = true;
        for i in 0..towel.colors.len() {
            if towel.colors[i] != pattern.colors[i] {
                found = false;
                break;
            }
        }

        if found {
            // remove the first colors from the pattern
            let mut copy = pattern.clone();
            copy.colors = copy.colors.split_off(towel.colors.len());
            total += try_wrapper(&copy, towels, level+1, cache);
        }
    }

    total
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = input.lines();

    // first line is towels
    let towels = lines.next().unwrap();
    let towels = towels.split(", ").map(|t| Towel::from_str(t)).collect::<Vec<_>>();

    // skip empty line
    lines.next();

    // all the next lines are patterns
    let patterns = lines.map(|p| Pattern::from_str(p)).collect::<Vec<_>>();

    let mut total_possible = 0;
    let mut i = 0;
    let mut cache = Cache::new();
    for pattern in patterns {
        total_possible += try_pattern(&pattern, &towels, 0, &mut cache);
        println!("Pattern {} checked", i);
        i += 1;
    }

    println!("Successes: {}", total_possible);
}

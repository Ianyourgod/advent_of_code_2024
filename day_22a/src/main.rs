//use cached::proc_macro::cached;

use std::collections::HashMap;

//#[cached]
fn evolve_number(number: i64) -> i64 {
    let step_1 = prune(mix(number, number*64));
    let step_2 = prune(mix(step_1, step_1/32));
    let step_3 = prune(mix(step_2, step_2*2048));
    return step_3;
}

//#[cached]
fn mix(a: i64, b: i64) -> i64 {
    // bitwise xor
    a ^ b
}

//#[cached]
fn prune(number: i64) -> i64 {
    // mod 16777216
    number % 16777216
}

fn price(number: i64) -> i64 {
    return number % 10;
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    let numbers = input.lines().map(|line| line.parse::<i64>().unwrap()).collect::<Vec<i64>>();

    const ITERATIONS: usize = 2000;

    let mut sequences = Vec::new();

    for (monkey, &number) in numbers.iter().enumerate() {
        let mut final_number = number;
        let mut prev_price = price(final_number);
        let mut previous_changes = Vec::new();

        for i in 0..ITERATIONS {
            final_number = evolve_number(final_number);
            let price = price(final_number);
            //println!("{}: {} ({})", final_number, price, price-prev_price);
            previous_changes.push(price-prev_price);
            if previous_changes.len() > 4 {
                previous_changes.remove(0);
            }

            if i > 3 {
                sequences.push((monkey, price, previous_changes.clone()));
            }

            prev_price = price;
        }
    }

    println!("Sequence finding complete");

    let mut sequences_total: HashMap<Vec<i64>, HashMap<usize, i64>> = HashMap::new();

    for (monkey, price, sequence) in sequences {
        // we can only take one sequence per monkey
        // we need to find the sequence that gives the most total (get the sequence from all monkeys and sum the price)
        match sequences_total.get_mut(&sequence) {
            Some(monkeys) => {
                // monkeys is a hashmap
                if !monkeys.contains_key(&monkey) {
                    monkeys.insert(monkey, price);
                }
            }
            None => {
                let mut monkeys = HashMap::new();
                monkeys.insert(monkey, price);
                sequences_total.insert(sequence, monkeys);
            }
        }
    }

    let max_sequence = sequences_total.iter().max_by_key(|(_, monkeys)| {
        monkeys.iter().map(|(_, price)| price).sum::<i64>()
    });

    let max_sequence = match max_sequence {
        Some(max_sequence) => {
            max_sequence
        }
        None => {
            panic!("No max sequence found");
        }
    };

    println!("{:?}", max_sequence.0);

    let total_price = max_sequence.1.iter().map(|(_, price)| price).sum::<i64>();

    println!("{:?}", total_price);
}
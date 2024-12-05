#[derive(Debug)]
struct Rules(Vec<(i32, i32)>);

#[derive(Debug, Clone)]
struct Update(Vec<i32>); // page numbers

fn read_input(input: &str) -> (Rules, Vec<Update>) {
    /*
    layout:
    1|2
    3|4
    // end of rules section (this comment is not included in the input. instead its just an empty line)

    1,2,3,4
     */

    let mut rules = Vec::new();
    let mut updates = Vec::new();

    let mut is_rules = true;

    for line in input.lines() {
        if line.is_empty() {
            is_rules = false;
            continue;
        }

        if is_rules {
            let mut parts = line.split('|');
            let a = parts.next().unwrap().parse().unwrap();
            let b = parts.next().unwrap().parse().unwrap();
            rules.push((a, b));
        } else {
            let update = line.split(',').map(|x| x.parse().unwrap()).collect();
            updates.push(Update(update));
        }
    }

    (Rules(rules), updates)
}

fn is_in_order(rules: &Rules, update: &Update) -> bool {
    for rule in &rules.0 {
        let a = match update.0.iter().position(|&r| r == rule.0) {
            Some(x) => x,
            None => continue,
        };
        let b = match update.0.iter().position(|&r| r == rule.1) {
            Some(x) => x,
            None => continue,
        };

        if a > b {
            return false;
        }
    }

    true
}

fn fix_order(rules: &Rules, update: &mut Update) {
    while !is_in_order(rules, update) {
        for rule in &rules.0 {
            let a = match update.0.iter().position(|&r| r == rule.0) {
                Some(x) => x,
                None => continue,
            };
            let b = match update.0.iter().position(|&r| r == rule.1) {
                Some(x) => x,
                None => continue,
            };

            if a > b {
                update.0.swap(a, b);
            }
        }
    }
}

fn main() {
    // read from input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    let (rules, updates) = read_input(&input);

    let mut total = 0;
    for update in &updates {
        if is_in_order(&rules, update) {
            // pass
        } else {
            let mut new_update = update.clone();
            fix_order(&rules, &mut new_update);
            println!("Original: {:?}, Fixed: {:?}", update, new_update);
            // get middle number
            let middle = new_update.0[new_update.0.len() / 2];
            total += middle;
        }
    }

    println!("Total: {}", total);
}

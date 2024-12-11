use cached::proc_macro::cached;

fn create_vec(input: &str) -> Vec<i64> {
    input
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect()
}

/*
fn run_rules(vec: &mut Vec<i64>) {
    let mut i = 0;
    while i < vec.len() {
        let (a, b) = rules(vec[i]);
        vec[i] = a;
        if let Some(b) = b {
            vec.insert(i + 1, b);
            i += 1;
        }
        i += 1;
    }
}

fn rules(item: i64) -> (i64, Option<i64>) {
    if item == 0 { return (1, None); }

    // convert to string
    let s = item.to_string();
    if s.len() % 2 == 0 {
        // split in 2
        let (a, b) = s.split_at(s.len() / 2);
        let a = a.parse::<i64>().unwrap();
        let b = b.parse::<i64>().unwrap();

        return (a, Some(b));
    }

    return (item * 2024, None);
}
*/

#[cached]
fn find_eventual_length(item: i64, steps: i32) -> usize {
    if steps == 0 { return 1; }
    let as_string = item.to_string();
    if as_string.len() % 2 == 0 {
        return
            find_eventual_length(as_string[0..as_string.len() / 2].parse::<i64>().unwrap(), steps - 1) +
            find_eventual_length(as_string[as_string.len() / 2..].parse::<i64>().unwrap(), steps - 1);
    }

    match item {
        0 => find_eventual_length(1, steps - 1),
        _ => find_eventual_length(item * 2024, steps - 1),
    }
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    let vec = create_vec(&input);

    let mut length = 0;
    let mut i = 0;
    for item in vec {
        length += find_eventual_length(item, 75);
        i += 1;
        println!("{}", i);
    }
    println!("Length: {}", length);
}

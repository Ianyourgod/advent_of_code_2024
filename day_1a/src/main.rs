fn parse_line(line: &str) -> (i32, i32) {
    // int   int
    let mut iter = line.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().unwrap();
    let b: i32 = iter.next().unwrap().parse().unwrap();
    (a, b)
}

fn main() {
    // read from input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    // split by new line
    let lines = input.lines();

    // create 2 lists (1 for left, 1 for right)
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    // loop through each line
    for line in lines {
        // parse the line
        let (a, b) = parse_line(line);
        // push to the lists
        left.push(a);
        right.push(b);
    }

    // sort the lists
    left.sort();
    right.sort();
    
    // now we find the similarity score.
    /* to find this we need to
    1. loop over the first list
    2. find the amount of times the number is in the second list
    3. multiply the item by the amount of times it is in the second list
    4. add the result to the score
    */

    let mut score = 0;
    let mut right_index = 0;
    for item in left {
        let mut count = 0; 
        loop {
            let right_item = right[right_index];

            if item == right_item {
                count += 1;
            } else if item < right_item {
                break;
            }
            right_index += 1;
        }
        score += item * count;
    }

    // print the score
    println!("{}", score);
}

fn main() {
    // read text from input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;
    let mut cur_idx = 0;
    let mut enabled = true;
    while cur_idx < input.len() {
        let cur_char = input.chars().nth(cur_idx).unwrap();

        // look for mul(X,Y)


        if enabled && cur_char == 'm' &&
            input.chars().nth(cur_idx+1) == Some('u') &&
            input.chars().nth(cur_idx+2) == Some('l') &&
            input.chars().nth(cur_idx+3) == Some('(') {
            let mut idx = cur_idx + 4;
            let mut x = 0;
            let mut y = 0;
            // look for number, until , or non number
            while input.chars().nth(idx).unwrap().is_digit(10) {
                x = x * 10 + input.chars().nth(idx).unwrap().to_digit(10).unwrap();
                idx += 1;
            }

            // skip ,
            if input.chars().nth(idx).unwrap() != ',' {
                cur_idx = idx + 1;
                continue;
            }

            idx += 1;

            // look for number, until )
            while input.chars().nth(idx).unwrap().is_digit(10) {
                y = y * 10 + input.chars().nth(idx).unwrap().to_digit(10).unwrap();
                idx += 1;
            }

            // skip )
            if input.chars().nth(idx).unwrap() != ')' {
                cur_idx = idx + 1;
                continue;
            }

            sum += x * y;
            cur_idx = idx + 1;
        } else if cur_char == 'd' &&
            input.chars().nth(cur_idx+1) == Some('o') &&
            input.chars().nth(cur_idx+2) == Some('(') &&
            input.chars().nth(cur_idx+3) == Some(')')  {

            enabled = true;
            cur_idx += 3;
        } else if cur_char == 'd' &&
            input.chars().nth(cur_idx+1) == Some('o') &&
            input.chars().nth(cur_idx+2) == Some('n') &&
            input.chars().nth(cur_idx+3) == Some('\'') &&
            input.chars().nth(cur_idx+4) == Some('t') &&
            input.chars().nth(cur_idx+5) == Some('(') &&
            input.chars().nth(cur_idx+6) == Some(')') {

            enabled = false;

            cur_idx += 6;
        } else {
            cur_idx += 1;
        }
    }

    println!("Sum: {}", sum);
}

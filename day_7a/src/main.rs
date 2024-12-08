#![allow(dead_code)]

fn parse_equation(str: &str) -> (f64, Vec<f64>) {
    // split the string into two parts, split by ":" character

    let parts: Vec<&str> = str.split(":").collect();

    // first part is the rule
    let rule = parts[0];

    // second part is the list of numbers, split by " "
    let numbers: Vec<f64> = parts[1]
        .split(" ")
        .filter(|x| x.len() > 0)
        .map(|x| x.parse::<f64>().unwrap())
        .collect();

    // return the tuple
    (rule.parse::<f64>().unwrap(), numbers)
}

#[derive(Clone, Debug)]
enum Operator {
    Add,
    Multiply,
}

fn valid_eq(eq: &(f64, Vec<f64>)) -> bool {
    // get the rule
    let result = eq.0;

    if eq.1.len() == 1 {
        return eq.1[0] == result;
    }

    let mut new_eq = eq.clone();
    // remove the first number
    new_eq.1.remove(0);
    new_eq.1[0] = eq.1[0] + eq.1[1];

    if valid_eq(&new_eq) {
        return true;
    }

    new_eq.1[0] = eq.1[0] * eq.1[1];

    if valid_eq(&new_eq) {
        return true;
    }

    let mut temp = eq.1[0].to_string();
    temp.push_str(eq.1[1].to_string().as_str());
    new_eq.1[0] = temp.parse::<f64>().unwrap();

    if valid_eq(&new_eq) {
        return true;
    }

    false
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    // split the input by new line
    let lines: Vec<&str> = input.trim().split("\n").collect();

    // parse each line
    let equations: Vec<(f64, Vec<f64>)> = lines.iter().map(|x| parse_equation(x)).collect();

    // find valid equations
    let valid_eqs: Vec<(f64, Vec<f64>)> = equations.iter().filter(|x| valid_eq(x)).cloned().collect();

    // add together the first number of each valid equation
    let sum: f64 = valid_eqs.iter().map(|x| x.0).sum();

    println!("Sum: {}", sum);
}

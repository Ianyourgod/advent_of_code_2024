use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Gate {
    pub gate: GateType,
    pub left: String,
    pub right: String,
    pub output: String,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GateType {
    And,
    Xor,
    Or,
}

impl GateType {
    pub fn new(gate: &str) -> Self {
        match gate {
            "AND" => Self::And,
            "XOR" => Self::Xor,
            "OR" => Self::Or,
            _ => panic!("Invalid gate"),
        }
    }
}

fn follows_rule_1(gate: &Gate) -> bool {
    if gate.output.starts_with('z') && !gate.output.ends_with("45") {
        gate.gate == GateType::Xor
    } else {
        true
    }
}

fn follows_rule_2(gate: &Gate) -> bool {
    let left_is_xy = gate.left.starts_with('x') || gate.left.starts_with('y');
    let right_is_xy = gate.right.starts_with('x') || gate.right.starts_with('y');

    (gate.output.starts_with('z') || left_is_xy || right_is_xy) || gate.gate != GateType::Xor
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut vals = HashMap::new();

    let parts = input.split("\n\n").collect::<Vec<&str>>();

    let mut x_inputs = vec![];
    let mut y_inputs = vec![];

    for starting_val in parts[0].split('\n') {
        let mut parts = starting_val.split(": ");

        let name = parts.next().unwrap().to_string();
        let val = if parts.next().unwrap() == "1" { true } else { false };

        vals.insert(name.clone(), val);

        if name.starts_with("x") {
            x_inputs.push((name, val));
        } else if name.starts_with("y") {
            y_inputs.push((name, val));
        }
    }

    let mut x_str = String::new();
    for x in x_inputs.iter().rev() {
        x_str.push(if x.1 { '1' } else { '0' });
    }
    let mut y_str = String::new();
    for y in y_inputs.iter().rev() {
        y_str.push(if y.1 { '1' } else { '0' });
    }

    let x_num = u64::from_str_radix(&x_str, 2).unwrap();
    let y_num = u64::from_str_radix(&y_str, 2).unwrap();

    let z_num = x_num + y_num;
    let z_num_as_str = format!("{:b}", z_num);

    println!("{}", z_num_as_str);

    let mut gates = vec![];

    for gate in parts[1].split('\n') {
        let mut parts = gate.split(' ');

        let left = parts.next().unwrap().to_string();
        let gate = parts.next().unwrap().to_string();
        let right = parts.next().unwrap().to_string();
        let output = parts.last().unwrap().to_string();

        gates.push(Gate {
            gate: GateType::new(&gate),
            left,
            right,
            output,
        });
    }

    for (i, gate) in gates.iter().enumerate() {
        if !follows_rule_1(gate) || !follows_rule_2(gate) {
            println!("gate {} ({:?}) breaks rule {}", i, gate, if !follows_rule_1(gate) { '1' } else { '2' });
        }
    }

    while gates.len() > 0 {
        let mut removing = None;
        for (i, gate) in gates.iter().enumerate() {
            let left = match vals.get(&gate.left) {
                Some(val) => *val,
                None => continue,
            };

            let right = match vals.get(&gate.right) {
                Some(val) => *val,
                None => continue,
            };

            let result = match gate.gate {
                GateType::And => left & right,
                GateType::Xor => left ^ right,
                GateType::Or => left | right,
            };

            vals.insert(gate.output.clone(), result);
            removing = Some(i);
            break;
        }

        gates.remove(removing.unwrap());
    }

    let mut z_vals = Vec::new();

    for val in vals.iter() {
        if val.0.starts_with("z") {
            z_vals.push(val);
        }
    }

    z_vals.sort_by(|a, b| {
        let a = a.0.chars().skip(1).collect::<String>().parse::<u32>().unwrap();
        let b = b.0.chars().skip(1).collect::<String>().parse::<u32>().unwrap();

        b.cmp(&a)
    });

    let mut final_str = String::new();
    for val in z_vals {
        final_str.push(if *val.1 { '1' } else { '0' });
    }

    println!("{}", final_str);

    // convert the binary string to a number
    let final_num = u64::from_str_radix(&final_str, 2).unwrap();

    let xored = final_num ^ z_num;

    println!("{:046b}", xored);

    println!("Final number: {}", final_num);
}

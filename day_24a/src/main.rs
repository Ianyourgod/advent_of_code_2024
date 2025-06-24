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
    pub fn from_str(gate: &str) -> Self {
        match gate {
            "AND" => Self::And,
            "XOR" => Self::Xor,
            "OR" => Self::Or,
            _ => panic!("Invalid gate"),
        }
    }
}

fn main() {
    use std::time::Instant;

    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("PART 1:");

    let p1_start = Instant::now();

    {
        println!("{}", part_1(&input));
    }

    let time_taken = p1_start.elapsed();
    println!("\nPART 1 took: {:.2?}", time_taken);

    println!("\nPART 2:");

    let p2_start = Instant::now();

    {
        println!("{}", part_2(&input));
    }

    let time_taken = p2_start.elapsed();
    println!("\nPART 2 took: {:.2?}", time_taken);
}

fn part_1(input: &str) -> u64 {
    let mut vals = HashMap::new();

    let parts = input.split("\n\n").collect::<Vec<&str>>();

    for starting_val in parts[0].split('\n') {
        let mut parts = starting_val.split(": ");

        let name = parts.next().unwrap().to_string();
        let val = if parts.next().unwrap() == "1" { true } else { false };

        vals.insert(name.clone(), val);
    }

    let mut gates = vec![];

    for gate in parts[1].split('\n') {
        let mut parts = gate.split(' ');

        let left = parts.next().unwrap().to_string();
        let gate = parts.next().unwrap().to_string();
        let right = parts.next().unwrap().to_string();
        let output = parts.last().unwrap().to_string();

        gates.push(Gate {
            gate: GateType::from_str(&gate),
            left,
            right,
            output,
        });
    }

    get_true_output(&gates, vals)
}

fn get_true_output(gates: &Vec<Gate>, mut vals: HashMap<String, bool>) -> u64 {
    let mut completed_gates = 0;

    let mut current_gate = 0;
    while completed_gates < gates.len() {
        let gate = gates.get(current_gate).unwrap();

        if vals.contains_key(&gate.left) && vals.contains_key(&gate.right) && !vals.contains_key(&gate.output) {
            completed_gates += 1;
            let left = *vals.get(&gate.left).unwrap();
            let right = *vals.get(&gate.right).unwrap();

            let val = match gate.gate {
                GateType::And => left && right,
                GateType::Or => left || right,
                GateType::Xor => left ^ right,
            };

            vals.insert(gate.output.clone(), val);
        }

        current_gate += 1;
        if current_gate >= gates.len() {
            current_gate = 0;
        }
    }

    let mut zs = Vec::new();
    for v in vals {
        if v.0.starts_with('z') {
            zs.push(v);
        }
    }

    /*
    For example |a, b| (a - b).cmp(a) is a comparison function that is neither transitive nor reflexive nor total, a < b < c < a with a = 1, b = 2, c = 3. For more information and examples see the [Ord] documentation.
     */

    zs.sort_by(|a,b| {
        a.0.cmp(&b.0)
    });

    zs.reverse();

    let mut z_val = 0;
    for z in zs {
        z_val *= 2;
        z_val += if z.1 { 1 } else { 0 };
    }

    z_val
}

fn part_2(input: &str) -> String {
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

    println!("Z:\n{}\n{}", z_num_as_str, z_num);

    let mut gates = vec![];

    for gate in parts[1].split('\n') {
        let mut parts = gate.split(' ');

        let left = parts.next().unwrap().to_string();
        let gate = parts.next().unwrap().to_string();
        let right = parts.next().unwrap().to_string();
        let output = parts.last().unwrap().to_string();

        gates.push(Gate {
            gate: GateType::from_str(&gate),
            left,
            right,
            output,
        });
    }

    let true_z = get_true_output(&gates, vals.clone());

    println!("TRUE Z:\n{:b}\n{}", true_z, true_z);

    //let mut cur_bit = z_num_as_str.len()-1;
    String::from("sigma_goon")
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let input = std::fs::read_to_string("input.txt").unwrap();
        super::part_1(&input);
    }
}
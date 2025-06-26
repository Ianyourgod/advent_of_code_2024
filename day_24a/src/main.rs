use std::collections::HashMap;

use itertools::Itertools;
use rayon::prelude::*;
use rand::prelude::*;
use cached::proc_macro::cached;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Gate {
    pub gate: GateType,
    pub left: String,
    pub right: String,
    pub output: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

type Gates = HashMap<String, Gate>;
type Swap = (String, String);
type Swaps = [Swap; 4];

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
    let (gates, vals, _, _) = get_vals_and_gates(input);

    get_true_output(&gates, vals)
}

fn get_vals_and_gates(input: &str) -> (Gates, HashMap<String, bool>, u64, u64) {
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

    let mut gates = HashMap::new();

    for gate in parts[1].split('\n') {
        let mut parts = gate.split(' ');

        let left = parts.next().unwrap().to_string();
        let gate = parts.next().unwrap().to_string();
        let right = parts.next().unwrap().to_string();
        let output = parts.last().unwrap().to_string();

        gates.insert(output.clone(), Gate {
            gate: GateType::from_str(&gate),
            left,
            right,
            output,
        });
    }

    (gates, vals, x_num, y_num)
}

fn get_true_output(gates: &Gates, mut vals: HashMap<String, bool>) -> u64 {
    let mut completed_gates = 0;

    let mut current_gate = 0;
    let gs = gates.values().collect::<Vec<_>>();
    while completed_gates < gates.len() {
        let gate = gs.get(current_gate).unwrap();

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
    let (gates, vals, x_num, y_num) = get_vals_and_gates(input);

    let z_num = x_num + y_num;

    let wires = get_all_wires(&gates).into_iter().cloned().collect::<Vec<_>>();

    let swaps = blank_swaps();

    let pairs = generate_all_pairs(&wires);

    let scored_pairs = pairs.into_par_iter().map(|pair| {
        let mut swaps = swaps.clone();
        swaps[0] = pair;
        let score = check_bits_correct(&gates, &swaps);
        let pair = swaps.into_iter().nth(0).unwrap();
        (score, pair)
    })
    .collect::<Vec<_>>()
    .into_iter()
    .sorted_by(|(a, _), (b, _)| {
        b.cmp(a)
    })
    .take(200);

    let toups = all_pairs(scored_pairs).collect::<Vec<_>>();
    let scored_doubles = toups.into_par_iter().map(|((_, uno), (_, dos))| {
        let mut swaps = swaps.clone();

        let one = uno.clone();
        let two = dos.clone();

        swaps[0] = uno;
        swaps[1] = dos;
        let score = check_bits_correct(&gates, &swaps);
        (score, (one, two))
    })
    .collect::<Vec<_>>()
    .into_iter()
    .sorted_by(|(a, _), (b, _)| {
        b.cmp(a)
    })
    .take(200)
    .collect::<Vec<_>>();

    let swaps = all_pairs(scored_doubles).map(|((_, (a, b)), (_, (c, d)))| {
        [
            a,
            b,
            c,
            d
        ]
    }).collect::<Vec<_>>();

    let scoring_vals = gen_scoring_vals(20);

    let final_swap = swaps.into_par_iter().find_any(|swaps| {
        let score = check_bits_correct(&gates, &swaps);
        let actual = run_with_swaps(&gates, vals.clone(), swaps);
        let actual_is_correct = actual == z_num;
        let fuzzy_score = score_gene(swaps, &gates, &scoring_vals);

        score == 45 && actual_is_correct && fuzzy_score == 0
    });

    let mut f = final_swap.unwrap().into_iter().flat_map(|(a,b)|[a,b]).collect::<Vec<_>>();
    f.sort();

    f.join(",")
}

#[inline]
fn all_pairs<T: Clone>(iter: impl IntoIterator<Item = T>) -> impl Iterator<Item = (T, T)> {
    let items: Vec<T> = iter.into_iter().collect();
    let len = items.len();
    let mut out: Vec<(T, T)> = Vec::with_capacity(len * (len-1) >> 1);
    for i in 0..items.len() {
        let item = items.get(i).unwrap().clone();
        for j in i..items.len() {
            let inner_item = items.get(j).unwrap().clone();
            out.push((item.clone(), inner_item));
        }
    }
    out.into_iter()
}

fn generate_all_pairs(wires: &[String]) -> Vec<Swap> {
    let mut pairs = Vec::with_capacity((wires.len() * (wires.len() - 1)) >> 1);

    for i in 0..wires.len() {
        for j in (i + 1)..wires.len() {
            let a = &wires[i];
            let b = &wires[j];
            if a < b {
                pairs.push((a.clone(), b.clone()));
            } else {
                pairs.push((b.clone(), a.clone()));
            }
        }
    }

    pairs
}

#[inline]
fn blank_swaps() -> Swaps {
    [
        ("", ""),
        ("", ""),
        ("", ""),
        ("", "")
    ].map(|(a,b)|(a.to_string(),b.to_string()))
}

fn bits_diff(v1: u64, v2: u64) -> u8 {
    let mut diff = 0;

    for bit in 0..64 {
        let i = (2 << bit) >> 1;

        if v1 & i != v2 & i { diff += 1 }
    }

    diff
}

#[cached]
fn gen_vals(x: u64, y: u64) -> HashMap<String, bool> {
    let mut vals = HashMap::new();

    for i in 0..45 {
        let x_bit_val = (x & (1 << i)) > 0;
        let y_bit_val = (y & (1 << i)) > 0;
        let x = format!("x{:02}", i);
        let y = format!("y{:02}", i);
        vals.insert(x, x_bit_val);
        vals.insert(y, y_bit_val);
    }

    vals
}

fn gen_scoring_vals(gen_amount: usize) -> Vec<(HashMap<String, bool>, u64)> {

    let mut rng = rand::rng();
    let mut s = Vec::with_capacity(gen_amount);

    for _ in 0..gen_amount {
        let bits_45 = 0b111111111111111111111111111111111111111111111;
        let x = rng.random::<u64>() & bits_45;
        let y = rng.random::<u64>() & bits_45;
        let z = x + y;
        let vals = gen_vals(x, y);

        s.push((vals, z))
    }

    s
}

fn score_gene(swaps: &Swaps, gates: &Gates, scoring_vals: &Vec<(HashMap<String, bool>, u64)>) -> u64 {
       let mut total_score = 0;

    for (vals, wanted) in scoring_vals {
        let s = run_with_swaps(gates, vals.clone(), swaps);

        let this_score = bits_diff(*wanted, s) as u64;

        total_score += this_score.pow(2);
    }

    total_score
}

fn get_all_wires<'a>(gates: &'a Gates) -> Vec<&'a String> {
    let mut wires = Vec::new();
    for (_, gate) in gates {
        wires.push(&gate.output);
    }
    wires
}

#[inline]
fn get_actual_from_swaps<'a>(swaps: &'a Swaps, a: &'a str) -> &'a str {
    for s in swaps {
        if a == s.0 {
            return &s.1;
        }
        if a == s.1 {
            return &s.0;
        }
    }

    a
}

#[inline]
fn swap_vals_contains(vals: &HashMap<String, bool>, v: &str, swaps: &Swaps) -> bool {
    let v = get_actual_from_swaps(swaps, v);
    vals.contains_key(v)
}

fn run_with_swaps(gates: &Gates, mut vals: HashMap<String, bool>, swaps: &Swaps) -> u64 {
    let mut gates = gates.clone();
    let mut since_last_complete = 0;

    let mut current_gate = 0;
    while gates.len() > 0 {
        if since_last_complete > gates.len() {
            return 0;
        }
        since_last_complete += 1;

        if current_gate >= gates.len() {
            current_gate = 0;
        }
        let gate = gates.iter().nth(current_gate).unwrap().1;

        if vals.contains_key(&gate.left) && vals.contains_key(&gate.right) && !swap_vals_contains(&vals, &gate.output, swaps) {
            since_last_complete = 0;
            let left = *vals.get(&gate.left).unwrap();
            let right = *vals.get(&gate.right).unwrap();

            let val = match gate.gate {
                GateType::And => left && right,
                GateType::Or => left || right,
                GateType::Xor => left ^ right,
            };

            let out = get_actual_from_swaps(swaps, &gate.output).to_string();
            let actual_out = gate.output.clone();

            gates.remove(&actual_out);
            vals.insert(out, val);
            current_gate = 0;
        } else {
            current_gate += 1;
        }
    }

    let mut zs = Vec::new();
    for v in vals {
        if v.0.starts_with('z') {
            zs.push(v);
        }
    }

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

const POW_LIST: [u64; 45] = [
    1,
    2,
    4,
    8,
    16,
    32,
    64,
    128,
    256,
    512,
    1024,
    2048,
    4096,
    8192,
    16384,
    32768,
    65536,
    131072,
    262144,
    524288,
    1048576,
    2097152,
    4194304,
    8388608,
    16777216,
    33554432,
    67108864,
    134217728,
    268435456,
    536870912,
    1073741824,
    2147483648,
    4294967296,
    8589934592,
    17179869184,
    34359738368,
    68719476736,
    137438953472,
    274877906944,
    549755813888,
    1099511627776,
    2199023255552,
    4398046511104,
    8796093022208,
    17592186044416,
];

fn check_bits_correct(gates: &Gates, swaps: &Swaps) -> u64 {
    let mut total = 0;

    for i in POW_LIST {
        let n = i;
        let n2 = n*2;

        let vals = gen_vals(n, n);

        let result = run_with_swaps(gates, vals, swaps);

        let correct = result - n2 == 0;

        if correct { total += 1; }
    }

    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn part_1() {
        let input = std::fs::read_to_string("input.txt").unwrap();
        let a = super::part_1(&input);
        assert_eq!(a, 48806532300520);
    }
}
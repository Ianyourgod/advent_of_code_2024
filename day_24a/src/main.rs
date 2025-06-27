#![feature(exact_size_is_empty)]
#![feature(let_chains)]

use itertools::Itertools;
use rayon::prelude::*;
use rand::prelude::*;
//use cached::proc_macro::cached;
use array_init::from_iter;
use hashbrown::{HashMap, HashSet};

// NOTE: 100 for speed. 150-200 for reliability. (100 will not always return a solution)
const TOP_X_SEARCH: usize = 150;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct StrGate {
    pub gate: GateType,
    pub left: String,
    pub right: String,
    pub output: String,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Gate {
    pub gate: GateType,
    pub left: usize,
    pub right: usize,
    pub output: usize,
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

type WireConversions = HashMap<usize, String>;
type WCBack = HashMap<String, usize>;
type Gates = Vec<Option<Gate>>;
type Swap = (usize, usize);
type Swaps = [Swap; 4];
type Vals = Vec<Option<bool>>;

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

fn generate_wire_conversions(wires: Vec<String>) -> (WireConversions, WCBack) {
    let mut n = 0;
    let mut f = WireConversions::with_capacity(wires.len());
    let mut b = WCBack::with_capacity(wires.len());
    for wire in wires {
        f.insert(n, wire.clone());
        b.insert(wire, n);
        n += 1;
    }
    (f, b)
}

fn part_1(input: &str) -> u64 {
    let (gates, vals, _, _, wc_front, _) = get_vals_and_gates(input);

    run_with_swaps(&gates, &vals, &blank_swaps(), &wc_front)
}

fn get_vals_and_gates(input: &str) -> (Gates, Vals, u64, u64, WireConversions, WCBack) {
    let mut vals = HashMap::with_capacity(350);

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

    let gate_lines = parts[1].split('\n');

    let lines_len = match gate_lines.try_len() {
        Ok(l) => l,
        Err(e) => {
            if let Some(l) = e.1 { l } else { e.0 }
        }
    };

    let mut gates = HashMap::with_capacity(lines_len);

    let mut wires = HashSet::new();

    for gate in gate_lines {
        let mut parts = gate.split(' ');

        let left = parts.next().unwrap().to_string();
        let gate = parts.next().unwrap().to_string();
        let right = parts.next().unwrap().to_string();
        let output = parts.last().unwrap().to_string();

        wires.insert(left.clone());
        wires.insert(right.clone());
        wires.insert(output.clone());

        gates.insert(output.clone(), StrGate {
            gate: GateType::from_str(&gate),
            left,
            right,
            output,
        });
    }

    let (wc_front, wc_back) = generate_wire_conversions(wires.into_iter().collect());

    let mut max_out = 0;
    let gates_iter = gates.into_iter().map(|(i, gate)| {
        let idx = *wc_back.get(&i).unwrap();

        if idx > max_out {
            max_out = idx;
        }
        
        (idx, Gate {
            gate: gate.gate,
            left: *wc_back.get(&gate.left).unwrap(),
            right: *wc_back.get(&gate.right).unwrap(),
            output: *wc_back.get(&gate.output).unwrap()
        })
    }).collect::<Vec<_>>();

    let gates_len = max_out+1;
    let mut gates = Vec::with_capacity(gates_len);
    gates.resize(gates_len, None);
    gates_iter.into_iter().for_each(|g|gates[g.0] = Some(g.1));

    let vals_iter = vals.into_iter().map(|(i, val)| {
        let idx = *wc_back.get(&i).unwrap();
        (idx, val)
    }).collect::<Vec<_>>();

    let vals_len = 350;
    let mut vals = Vec::with_capacity(vals_len);
    vals.resize(vals_len, None);
    vals_iter.into_iter().for_each(|v|vals[v.0] = Some(v.1));

    (gates, vals, x_num, y_num, wc_front, wc_back)
}

fn part_2(input: &str) -> String {
    let (gates, vals, x_num, y_num, wc_front, wc_back) = get_vals_and_gates(input);

    let z_num = x_num + y_num;

    let testing_vals = gen_testing_vals(&wc_back);

    let scored_pairs = scored_pairs(&gates, &testing_vals, &wc_front);

    let scored_doubles = scored_doubles(scored_pairs, &gates, &testing_vals, &wc_front);

    let swaps = get_swaps(scored_doubles);
    let scoring_vals = gen_scoring_vals(20, &wc_back);

    let final_swap = final_swap(swaps, &scoring_vals, &gates, &vals, z_num, &testing_vals, &wc_front);

    final_swap.unwrap().into_iter()
    .flat_map(|(a,b)|[a,b])
    .map(|i|wc_front.get(&i).unwrap())
    .sorted()
    .join(",")
}

fn scored_pairs(gates: &Gates, testing_vals: &[(Vals, u64); POW_LIST.len()], wc: &WireConversions) -> impl Iterator<Item = (u64, Swap)> {
    let wires = get_all_wires(&gates).into_iter().collect::<Vec<_>>();

    let swaps = blank_swaps();

    let pairs = generate_all_pairs(&wires);

    pairs.into_par_iter().map(|pair| {
        let mut swaps = swaps.clone();
        swaps[0] = pair;
        let score = check_bits_correct(&gates, &swaps, testing_vals, wc);
        let pair = swaps.into_iter().nth(0).unwrap();
        (score, pair)
    })
    .collect::<Vec<_>>()
    .into_iter()
    .sorted_by(|(a, _), (b, _)| {
        b.cmp(a)
    })
    .take(TOP_X_SEARCH)
}

fn scored_doubles<T>(scored_pairs: T, gates: &Gates, testing_vals: &[(Vals, u64); POW_LIST.len()], wc: &WireConversions) -> impl Iterator<Item = (u64, (Swap, Swap))>
where
    T: Iterator<Item = (u64, Swap)>,
{
    let swaps = blank_swaps();
    let toups = all_pairs(scored_pairs).collect::<Vec<_>>();
    toups.into_par_iter().map(|((_, uno), (_, dos))| {
        let mut swaps = swaps.clone();

        let one = uno.clone();
        let two = dos.clone();

        swaps[0] = uno;
        swaps[1] = dos;
        let score = check_bits_correct(&gates, &swaps, testing_vals, wc);
        (score, (one, two))
    })
    .collect::<Vec<_>>()
    .into_iter()
    .sorted_by(|(a, _), (b, _)| {
        b.cmp(a)
    })
    .take(TOP_X_SEARCH)
}

fn get_swaps<T>(scored_doubles: T) -> Vec<Swaps>
where
    T: Iterator<Item = (u64, (Swap, Swap))>
{
    all_pairs(scored_doubles).map(|((_, (a, b)), (_, (c, d)))| {
        [
            a,
            b,
            c,
            d
        ]
    }).collect::<Vec<_>>()
}

fn final_swap(swaps: Vec<Swaps>, scoring_vals: &Vec<(Vals, u64)>, gates: &Gates, vals: &Vals, z_num: u64, testing_vals: &[(Vals, u64); POW_LIST.len()], wc: &WireConversions) -> Option<Swaps> {
    swaps.into_par_iter().find_any(|swaps| {
        let score = check_bits_correct(&gates, &swaps, testing_vals, wc);
        let actual = run_with_swaps(&gates, vals, swaps, wc);
        let actual_is_correct = actual == z_num;
        let fuzzy_score = score_gene(swaps, &gates, &scoring_vals, wc);

        score == 45 && actual_is_correct && fuzzy_score == 0
    })
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

fn generate_all_pairs(wires: &[usize]) -> Vec<Swap> {
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
    let m = usize::MAX;
    [
        (m, m),
        (m, m),
        (m, m),
        (m, m)
    ]
}

fn bits_diff(v1: u64, v2: u64) -> u8 {
    let mut diff = 0;

    for bit in 0..64 {
        let i = (2 << bit) >> 1;

        if v1 & i != v2 & i { diff += 1 }
    }

    diff
}

//#[cached]
fn gen_vals(x: u64, y: u64, wc: &WCBack) -> Vals {
    let mut vals = Vec::with_capacity(350);
    vals.resize(350, None);

    for i in 0..45 {
        let x_bit_val = (x & (1 << i)) > 0;
        let y_bit_val = (y & (1 << i)) > 0;
        let x = format!("x{:02}", i);
        let y = format!("y{:02}", i);
        let x = *wc.get(&x).unwrap();
        let y = *wc.get(&y).unwrap();
        if x > vals.len() { vals.resize(x+1, None); }
        if y > vals.len() { vals.resize(y+1, None); }
        vals[x] = Some(x_bit_val);
        vals[y] = Some(y_bit_val);
    }

    vals
}

fn gen_scoring_vals(gen_amount: usize, wc: &WCBack) -> Vec<(Vals, u64)> {
    let mut rng = rand::rng();
    let mut s = Vec::with_capacity(gen_amount);

    for _ in 0..gen_amount {
        let bits_45 = 0b111111111111111111111111111111111111111111111;
        let x = rng.random::<u64>() & bits_45;
        let y = rng.random::<u64>() & bits_45;
        let z = x + y;
        let vals = gen_vals(x, y, wc);

        s.push((vals, z))
    }

    s
}

fn score_gene(swaps: &Swaps, gates: &Gates, scoring_vals: &Vec<(Vals, u64)>, wc: &WireConversions) -> u64 {
       let mut total_score = 0;

    for (vals, wanted) in scoring_vals {
        let s = run_with_swaps(gates, vals, swaps, wc);

        let this_score = bits_diff(*wanted, s) as u64;

        total_score += this_score.pow(2);
    }

    total_score
}

fn get_all_wires(gates: &Gates) -> Vec<usize> {
    let mut wires = Vec::new();
    for gate in gates {
        if let Some(g) = gate { wires.push(g.output); }
    }
    wires
}

//#[inline]
fn check_swap(swap: &Swap, a: usize) -> Option<usize> {
    Some(if swap.0 == a { swap.1 }
    else if swap.1 == a { swap.0 }
    else { return None })
}

//#[inline]
fn get_actual_from_swaps(swaps: &Swaps, a: usize) -> usize {
    if let Some(s) = check_swap(&swaps[0], a) { return s }
    if let Some(s) = check_swap(&swaps[1], a) { return s }
    if let Some(s) = check_swap(&swaps[2], a) { return s }
    if let Some(s) = check_swap(&swaps[3], a) { return s }

    a
}

#[cfg_attr(not(debug_assertions), inline)]
fn swap_vals_contains(vals: &Vals, vals2: &Vals, v: usize, swaps: &Swaps) -> bool {
    let v = get_actual_from_swaps(swaps, v);
    let v = vals_contains(vals, vals2, v);
    v
}

#[cfg_attr(not(debug_assertions), inline)]
fn vals_contains(starting_vals: &Vals, our_vals: &Vals, s: usize) -> bool {
    iv_contains(starting_vals, s) || iv_contains(our_vals, s)
}

#[inline]
fn iv_contains(v: &Vals, s: usize) -> bool {
    v[s].is_some()
}

#[cfg_attr(not(debug_assertions), inline)]
fn vals_get(starting_vals: &Vals, our_vals: &Vals, s: usize) -> Option<bool> {
    return Some(if let Some(Some(b)) = our_vals.get(s) {
        *b
    } else if let Some(Some(b)) = starting_vals.get(s) {
        *b
    } else {
        return None;
    })
}

fn run_with_swaps(gates: &Gates, starting_vals: &Vals, swaps: &Swaps, wc: &WireConversions) -> u64 {
    let mut since_last_complete = 0;
    let len = gates.len();

    let sv = starting_vals;
    let ov_len = gates.len();
    let mut ov = Vec::with_capacity(ov_len);
    ov.resize(ov_len, None);
    let mut completed = 0;
    let mut current_gate = 0;

    let actual_len = gates.iter().fold(0, |a,g|{
        if g.is_some() { a + 1 } else { a }
    });

    loop {
        if since_last_complete > actual_len {
            return 0;
        }

        if current_gate >= len {
            current_gate = 0;
        }
        let gate = gates.get(current_gate).unwrap();

        let gate = 
        if let Some(gate) = gate { gate }
        else {
            current_gate += 1;
            continue;
        };
        since_last_complete += 1;

        let has_left = vals_contains(sv, &ov, gate.left);
        let has_right = vals_contains(sv, &ov, gate.right);
        let has_out = swap_vals_contains(sv, &ov, gate.output, swaps);

        if has_left && has_right && !has_out {
            since_last_complete = 0;
            let left = vals_get(sv, &ov, gate.left).unwrap();
            let right = vals_get(sv, &ov, gate.right).unwrap();

            let val = match gate.gate {
                GateType::And => left && right,
                GateType::Or => left || right,
                GateType::Xor => left ^ right,
            };

            let out = get_actual_from_swaps(swaps, gate.output);

            ov[out] = Some(val);
            completed += 1;
            if completed >= actual_len {
                break;
            }
        }

        current_gate += 1;
    }

    let mut zs = Vec::new();
    for v in ov.iter().enumerate() {
        let s = wc.get(&v.0);
        if let Some(s) = s && let Some(v) = v.1 {
            if s.starts_with('z') {
                zs.push((s, *v));
            }
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

fn gen_testing_vals(wc: &WCBack) -> [(Vals, u64); POW_LIST.len()] {
    let i = POW_LIST.into_iter().map(|i| {
        let n = i;
        let n2 = n*2;

        (gen_vals(n, n, wc), n2)        
    });

    from_iter(i).unwrap()
}

fn check_bits_correct(gates: &Gates, swaps: &Swaps, testing_vals: &[(Vals, u64); POW_LIST.len()], wc: &WireConversions) -> u64 {
    let mut total = 0;

    for (vals, n2) in testing_vals {
        let result = run_with_swaps(gates, vals, swaps, wc);

        let correct = result == *n2;

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
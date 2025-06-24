use std::collections::HashMap;

//use rayon::prelude::*;
use itertools::Itertools;

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    const DIR_PADS: usize = 25;

    let mut memo = HashMap::new();

    let num_pad = get_num_pad(&input);

    let move_lens = num_pad.into_iter().map(|num_pad| {
        find_count(num_pad, DIR_PADS, &mut memo)
    });

    /*const FIRST_HALF: usize = DIR_PADS / 2;
    const SECOND_HALF: usize = DIR_PADS - FIRST_HALF;

    let move_lens = num_pad.into_iter().enumerate().map(|(i, pad)| {
        println!("input {}", i);
        let first = (0..FIRST_HALF).fold(pad, |pad, n| {
            println!("dir pad {}", n+1);

            moves_to_moves(pad, &mut memo)
        });

        next_pad(first, SECOND_HALF, &mut memo)
    });
    */

    //let move_lens = move_lens.map(|m|m.len());

    /*
    let move_lens = (0..DIR_PADS).fold(num_pad, |moves,i| {
        println!("dir pad {i}");
        moves.into_iter().map(|m|moves_to_moves(m, &mut memo)).collect()
    }).into_iter().map(|m|m.len());
    */

    let s = input
        .split('\n')
        .zip(move_lens)
        .map(|(line, len)| {
            let num = (&line[0..3]).parse::<u16>().unwrap() as u64;
            num * len as u64
        })
        .sum::<u64>();

    println!("{}", s);
}

fn find_count(pad: Vec<Move>, pads: usize, memo: &mut HashMap<(Move, Move, usize), u64>) -> u64 {
    vec_to_rec(pad, pads+1, memo)
}

fn find_count_rec(start: Move, end: Move, remaining: usize, memo: &mut HashMap<(Move, Move, usize), u64>) -> u64 {
    if let Some(o) = memo.get(&(start, end, remaining)) {
        return *o;
    }
    
    if remaining == 0 {
        return 1;
    }

    let start_p = start.to_point();
    let end_p = end.to_point();
    let moves = gen_path_to(false, start_p, end_p);
    let o = vec_to_rec(moves, remaining, memo);

    memo.insert((start, end, remaining), o);

    o
}

fn vec_to_rec(moves: Vec<Move>, remaining: usize, memo: &mut HashMap<(Move, Move, usize), u64>) -> u64 {
    let o = std::iter::once(Move::Press).chain(moves).tuple_windows().map(|(start, end)| {
        find_count_rec(start, end, remaining-1, memo)
    }).sum();

    o
}

/*
fn split_into_two(mut p: Vec<Move>) -> (Vec<Move>, Vec<Move>) {
    let mid = p.len() / 2;
    let right = p.split_off(mid);
    let left = p;
    (left, right)
}

fn split_into_three(mut p: Vec<Move>) -> (Vec<Move>, Vec<Move>, Vec<Move>) {
    let first = p.len() / 3;
    let r = p.split_off(first);
    let left = p;
    let (mid, right) = split_into_two(r);
    (left, mid, right)
}
*/

/*
fn next_pad(prev: Vec<Move>, remaining: usize, memo: &mut HashMap<(Point, Point), Vec<Move>>) -> usize {
    //println!("{}, {}", remaining, prev.len());
    if remaining == 0 {
        return prev.len();
    }
    // split into 4
    let (one, two, three) = split_into_three(prev);

    let one = moves_to_moves(one, memo);
    let two = moves_to_moves(two, memo);
    let three = moves_to_moves(three, memo);

    let out =
        next_pad(one, remaining-1, memo) +
        next_pad(two, remaining-1, memo) +
        next_pad(three, remaining-1, memo);

    out
}
*/

fn get_num_pad(input: &str) -> Vec<Vec<Move>> {
    //let mut memo = HashMap::new();

    input
        .split('\n')
        .map(|item| {
            let moves = 
                vec![char_to_point('A')]
                .into_iter()
                .chain(
                    item
                    .chars()
                    .map(|c| {
                        char_to_point(c)
                    })
                )
                .tuple_windows()
                .map(|(start, end)| {
                    gen_path_to(true, start, end)
                })
                .concat();

            moves
        }).collect::<Vec<_>>()
}

#[allow(unused)]
fn moves_to_inner_moves(moves: Vec<Move>) -> Vec<Move> {
    let mut pos = Point::from_tuple((2, 0));

    let mut new_moves = vec![];

    for m in moves {
        match m {
            Move::Press => {
                new_moves.push(Move::from_point(pos))
            },
            Move::Up => pos.y -= 1,
            Move::Down => pos.y += 1,
            Move::Left => pos.x -= 1,
            Move::Right => pos.x += 1,
        }
    }

    new_moves
}

#[allow(unused)]
fn move_text_to_moves(mov_text: &str) -> Vec<Move> {
    mov_text.chars().map(|c| {
        match c {
            '<' => Move::Left,
            '>' => Move::Right,
            '^' => Move::Up,
            'v' => Move::Down,
            'A' => Move::Press,
            _ => unreachable!()
        }
    }).collect()
}

/*
fn moves_to_moves(moves: Vec<Move>) -> Vec<Move> {
    vec![Point::from_tuple((2, 0))]
        .into_iter()
        .chain(
            moves
                .into_iter()
                .map(|m| {
                    m.to_point()
                })
        )
        .tuple_windows()
        .map(|(start, end)| {
            gen_path_to(false, start, end)
        })
        .concat()
}
*/

fn char_to_point(c: char) -> Point {
    Point::from_tuple(
        match c {
            '7' => (0, 0),
            '8' => (1, 0),
            '9' => (2, 0),
            '4' => (0, 1),
            '5' => (1, 1),
            '6' => (2, 1),
            '1' => (0, 2),
            '2' => (1, 2),
            '3' => (2, 2),
            '0' => (1, 3),
            'A' => (2, 3),
            _ => unreachable!()
        }
    )
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Move {
    Up,
    Down,
    Left,
    Right,
    Press
}

impl Move {
    pub fn to_point(&self) -> Point {
        Point::from_tuple(match self {
            Move::Up => (1, 0),
            Move::Press => (2, 0),
            Move::Left => (0, 1),
            Move::Down => (1, 1),
            Move::Right => (2, 1),
        })
    }

    pub fn from_point(p: Point) -> Self {
        match (p.x, p.y) {
            (1, 0) => Move::Up,
            (2, 0) => Move::Press,
            (0, 1) => Move::Left,
            (1, 1) => Move::Down,
            (2, 1) => Move::Right,
            _ => unreachable!("{} {}", p.x, p.y)
        }
    }

    #[allow(unused)]
    pub fn display_vec(v: &Vec<Move>) -> String {
        let mut out = String::with_capacity(v.len());
        for m in v {
            out.push(match m {
                Move::Up => '^',
                Move::Down => 'v',
                Move::Left => '<',
                Move::Right => '>',
                Move::Press => 'A',
            })
        }
        out
    }
}

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: u8,
    y: u8
}

impl Point {
    pub fn from_tuple(p: (u8, u8)) -> Self {
        Self {
            x: p.0,
            y: p.1
        }
    }
}

fn gen_path_to(first: bool, cur_pos: Point, goto: Point) -> Vec<Move> {    
    let mut moves = Vec::new();

    let forward = cur_pos.x < goto.x;
    let (start, end) = if forward { (cur_pos.x, goto.x) } else { (goto.x, cur_pos.x) };
    let mut hor_moves = (start..end).map(|_| {
        if forward {
            Move::Right
        } else {
            Move::Left
        }
    }).collect();
    let downward = cur_pos.y < goto.y;
    let (start, end) = if downward { (cur_pos.y, goto.y) } else { (goto.y, cur_pos.y) };
    let mut vert_moves = (start..end).map(|_| {
        if downward {
            Move::Down
        } else {
            Move::Up
        }
    }).collect();

    if first && cur_pos.y == 3 && goto.x == 0 {
        moves.append(&mut vert_moves);
        moves.append(&mut hor_moves);
    } else if first && cur_pos.x == 0 && goto.y == 3 {
        moves.append(&mut hor_moves);
        moves.append(&mut vert_moves);
    } else if !first && cur_pos.x == 0 {
        moves.append(&mut hor_moves);
        moves.append(&mut vert_moves);
    } else if !first && goto.x == 0 {
        moves.append(&mut vert_moves);
        moves.append(&mut hor_moves);
    } else {
        if forward {
            moves.append(&mut vert_moves);
            moves.append(&mut hor_moves);
        } else {
            moves.append(&mut hor_moves);
            moves.append(&mut vert_moves);
        }
    }

    moves.push(Move::Press);

    moves
}
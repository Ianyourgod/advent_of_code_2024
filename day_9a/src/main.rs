#![allow(dead_code)]

fn parse_to_ints(input: &str) -> Vec<i8> {
    // every char is a number
    input.chars().map(|c| c.to_digit(10).unwrap() as i8).collect()
}

#[derive(Debug, Clone, Copy)]
enum Block {
    Free(i8), // size
    Used(i8, i64, bool), // size id
}

impl Block {
    fn is_free(&self) -> bool {
        if let Block::Free(_) = self {
            true
        } else {
            false
        }
    }

    fn has_been_moved(&self) -> bool {
        match self {
            Block::Free(_) => false,
            Block::Used(_, _, moved) => *moved,
        }
    }

    fn get_size(&self) -> i8 {
        match self {
            Block::Free(size) => *size,
            Block::Used(size, _, _) => *size,
        }
    }
}

fn split_to_blocks(numbers: &Vec<i8>) -> Vec<Block> {
    let mut blocks = Vec::new();
    let mut free = false;
    for (i, item) in numbers.iter().enumerate() {
        if free {
            blocks.push(Block::Free(*item));
        } else {
            blocks.push(Block::Used(*item, (i as i64) / 2, false));
        }

        free = !free;
    }
    blocks
}

fn run_step(blocks: &mut Vec<Block>) -> bool {
    let mut longest_free;
    let mut open_space;
    let mut last_used;
    loop {
        longest_free = (None, 0);
        for (i, block) in blocks.iter().enumerate() {
            if let Block::Free(size) = block {
                if *size > longest_free.1 {
                    longest_free = (Some(i), *size);
                }
            }
        }

        if longest_free.0.is_none() {
            return false;
        }

        let longest_free = (longest_free.0.unwrap(), longest_free.1);
        
        let mut used_vec = Vec::new();
        for (i, block) in blocks.iter().enumerate() {
            if block.is_free() {
                continue;
            }

            if block.has_been_moved() {
                continue;
            }

            let (size, id, _) = match block {
                Block::Used(size, id, _) => (*size, *id, i),
                _ => unreachable!(),
            };

            if size > longest_free.1 {
                continue;
            }

            used_vec.push((i, size, id));
        }

        if used_vec.is_empty() {
            return false;
        }

        used_vec.sort_by(|a, b| a.2.cmp(&b.2));

        last_used = used_vec.pop().unwrap();

        open_space = None;
        for (i, block) in blocks.iter().enumerate() {
            if i > last_used.0 {
                break;
            }

            if block.is_free() {
                let size = match block {
                    Block::Free(size) => *size,
                    _ => unreachable!(),
                };

                if size >= last_used.1 {
                    open_space = Some((i, size));
                    break;
                }
            }
        }

        if !open_space.is_none() {
            break;
        } else {
            // mark last_used as moved
            blocks[last_used.0] = Block::Used(last_used.1, last_used.2, true);
        }
    }

    let open_space = open_space.unwrap();

    if open_space.1 == last_used.1 {
        blocks.swap(last_used.0, open_space.0);
    } else {
        blocks[last_used.0] = Block::Free(last_used.1);
        blocks[open_space.0] = Block::Used(last_used.1, last_used.2, true);

        // make a new free block with the remaining space
        let new_size = open_space.1 - last_used.1;
        blocks.insert(open_space.0+1, Block::Free(new_size));
    }

    true
}

fn main() {
    // read input.txt
    let input = std::fs::read_to_string("input.txt").unwrap();

    // parse input to a vector of integers
    let numbers = parse_to_ints(&input);

    // split the vector into blocks
    let mut blocks = split_to_blocks(&numbers);

    //display_blocks(&blocks);
    while run_step(&mut blocks) {}//display_blocks(&blocks);}

    let checksum = generate_checksum(&blocks);

    println!("Checksum: {}", checksum);
}

fn generate_checksum(blocks: &Vec<Block>) -> i64 {
    let mut checksum = 0;
    let mut current_idx = 0;
    for block in blocks.iter() {
        if let Block::Used(size, id, _) = block {
            for _ in 0..*size {
                checksum += (current_idx) * (*id);
                current_idx += 1;
            }
        } else {
            current_idx += block.get_size() as i64;
        }
    }
    checksum
}

fn display_blocks(blocks: &Vec<Block>) {
    for block in blocks {
        match block {
            Block::Free(size) => {
                for _ in 0..*size {
                    print!(".");
                }
            }
            Block::Used(size, id, _) => {
                for _ in 0..*size {
                    print!("{}", *id);
                }
            }
        }
    }
    println!();
}
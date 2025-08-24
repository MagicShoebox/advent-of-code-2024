use std::{fmt, iter::repeat_n};

use crate::{Error, SolveError, SolveResult};

#[derive(Clone, Copy, Debug)]
enum BlockBlock {
    File { id: u32, size: usize },
    Free { size: usize },
}

#[derive(Clone, Copy)]
enum Block {
    File { id: u32 },
    Free,
}

impl fmt::Debug for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::File { id } => write!(f, "{}", id),
            Self::Free => write!(f, "."),
        }
    }
}

pub fn solve(input: &str) -> SolveResult {
    let blocks = parse(input)?;
    Ok((part1(blocks.iter()), part2(blocks)))
}

fn parse(input: &str) -> Result<Vec<BlockBlock>, SolveError> {
    let mut blocks = Vec::new();
    let mut is_file_block = true;
    let mut file_id = 0;
    for c in input.trim().chars() {
        let length = c.to_digit(10).ok_or(Error::InputError("Invalid digit"))? as usize;
        if is_file_block {
            blocks.push(BlockBlock::File {
                id: file_id,
                size: length,
            });
            file_id += 1;
        } else {
            blocks.push(BlockBlock::Free { size: length });
        }
        is_file_block = !is_file_block;
    }
    Ok(blocks)
}

fn part1<'a, I>(block_blocks: I) -> String
where
    I: Iterator<Item = &'a BlockBlock>,
{
    let mut blocks = flatten_block_blocks(block_blocks);
    let mut i = 0;
    while i < blocks.len() {
        if let Block::Free = blocks[i] {
            while let Some(Block::Free) = blocks.last() {
                blocks.pop();
            }
            if i >= blocks.len() {
                break;
            }
            if let Some(b @ Block::File { .. }) = blocks.pop() {
                blocks[i] = b;
            }
        }
        i += 1;
    }
    checksum(blocks.iter()).to_string()
}

fn part2(mut block_blocks: Vec<BlockBlock>) -> String {
    // This is a little inefficient with a Vec<>, but std:collections::LinkedList
    // didn't seem provide a stable API for inserting & removing from within the list,
    // which defeats the point.
    for source_index in (0..block_blocks.len()).rev() {
        if let file @ BlockBlock::File {
            size: file_size, ..
        } = block_blocks[source_index]
        {
            for target_index in 0..source_index {
                match block_blocks[target_index] {
                    BlockBlock::Free { size: free_size } if free_size == file_size => {
                        block_blocks[source_index] = BlockBlock::Free { size: file_size };
                        block_blocks[target_index] = file;
                        break;
                    }
                    BlockBlock::Free { size: free_size } if free_size > file_size => {
                        block_blocks[source_index] = BlockBlock::Free { size: file_size };
                        block_blocks.insert(target_index, file);
                        block_blocks[target_index + 1] = BlockBlock::Free {
                            size: free_size - file_size,
                        };
                        break;
                    }
                    _ => (),
                }
            }
        }
    }

    let blocks = flatten_block_blocks(block_blocks.iter());
    checksum(blocks.iter()).to_string()
}

fn flatten_block_blocks<'a, I>(block_blocks: I) -> Vec<Block>
where
    I: Iterator<Item = &'a BlockBlock>,
{
    let mut blocks = Vec::new();
    for block in block_blocks {
        match block {
            BlockBlock::File { id, size } => {
                blocks.extend(repeat_n(Block::File { id: *id }, *size))
            }
            BlockBlock::Free { size } => blocks.extend(repeat_n(Block::Free, *size)),
        }
    }
    blocks
}

fn checksum<'a, I>(blocks: I) -> usize
where
    I: Iterator<Item = &'a Block>,
{
    blocks
        .enumerate()
        .map(|(i, b)| match b {
            Block::File { id } => i * *id as usize,
            Block::Free => 0,
        })
        .sum()
}

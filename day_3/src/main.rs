use std::fs::File;
use std::io::{BufReader, Read};
use std::ops::Bound::{Excluded, Included, Unbounded};
use std::ops::RangeBounds;

#[derive(Debug)]
struct MulInstruction {
    x: i32,
    y: i32,
    index: usize

}

impl MulInstruction {

    fn mul(&self) -> i32{
        self.x * self.y
    }
}
const MAX_INSTRUCTION_LENGTH: usize = 12;
const MIN_INSTRUCTION_LENGTH: usize = 8;

fn main() {
    let input_file = std::env::args().nth(1).expect("please supply an input file");

    let input_path = std::path::Path::new(&input_file);

    println!("Loading input from {}", input_file);

    if input_path.exists() {
        println!("File exists: {}", input_path.display());
    }else {
        println!("File does not exist: {}", input_path.display());
        return;
    }

    match std::fs::File::open(input_path) {
        Ok(file) => compute_sum_one(file),
        // Ok(file) => compute_valid_levels_part_two(file),
        Err(error) => {
            println!("Error reading file: {}", error);
            return;
        }
    };

}

fn compute_sum_one(file: File){

    let sum = read_instructions::<i32>(file, |x, y| {

        match x {
            Some(accumulated_sum) =>
                accumulated_sum + y
                    .iter()
                    .map(|i| i.mul())
                    .sum::<i32>()
            ,
            None => y
                .iter()
                .map(|i| i.mul())
                .sum::<i32>()
        }

    });

    match sum {
        Some(sum) => println!("Sum: {}", sum),
        None => println!("No instructions found")
    };

}

fn read_instructions<R: std::fmt::Debug>(file: File, instruction_handler: fn(previous: Option<R>, &Vec<MulInstruction>) -> R) -> Option<R> {
    let mut reader = BufReader::new(file);
    let mut line = [0; MAX_INSTRUCTION_LENGTH * 2];

    let mut read_result = reader.read(&mut line);

    let mut result: Option<R> = None;

    loop {

        let read = match read_result {
            Ok(n) => n ,
            Err(_) => break,
        };

        if read < MIN_INSTRUCTION_LENGTH {
            break;
        }

        match parse_mul_instruction(&line, read){
            None => {
                // end of instructions
                if read < MAX_INSTRUCTION_LENGTH {
                    break;
                }
                // could not read any instructions
                // read the next chunk, make sure to offset back to read last instruction
                // which may be truncated due to the buffer size

                read_result = read_next_chunk(&mut line, (read - MAX_INSTRUCTION_LENGTH).., &mut reader);

            }
            Some(instructions) => {

                result = Some(instruction_handler(result, &instructions));
                // it is there
                let last_instruction_index = instructions.last().unwrap().index;

                if read >= MAX_INSTRUCTION_LENGTH {
                    // offset back to after the last instruction

                    read_result = read_next_chunk(&mut line, last_instruction_index.., &mut reader);

                }

            }
        };
    }

    result
}
fn read_next_chunk<R: RangeBounds<usize>>(line: &mut [u8], copy_from: R, reader: &mut BufReader<File>) -> Result<usize, std::io::Error>{

    let from = match copy_from.start_bound() {
        Included(b) => *b,
        Excluded(b) => *b+1,
        Unbounded => 0
    };

    let to = match copy_from.end_bound() {
        Included(b) => line.len().min(*b+1),
        Excluded(b) => line.len().min(*b),
        Unbounded => line.len()
    };
    line.copy_within(copy_from, 0);
    let range = to - from;
    let read_result = reader.read(&mut line[range..]);

    read_result
        .map(|x| {
            if x == 0 {
                return 0;
            }

            return  x + range;
        })
}

fn parse_mul_instruction(buff: &[u8], read: usize) -> Option<Vec<MulInstruction>> {

    // let mut index = 0;
    let mul_text: &[u8] = b"mul(";
    let mut mul_instructions = Vec::new();

    for i in MIN_INSTRUCTION_LENGTH..read {

        let current: &[u8] = &buff[i-MIN_INSTRUCTION_LENGTH..i-MIN_INSTRUCTION_LENGTH+mul_text.len()];

        if mul_text != current {
            continue;
        }

        // println!("parse: {}", String::from_utf8_lossy(&buff[i-8..read]));
        let index = i-MIN_INSTRUCTION_LENGTH+mul_text.len();

        match parse_input(&buff[i-MIN_INSTRUCTION_LENGTH+mul_text.len()..read]) {
            Some((x, y, read)) => {
                mul_instructions.push(MulInstruction{x, y, index: index + read});
                // println!("inputs: ({}, {}, {})", x, y, index);
            },
            None => {} // just move on when failed to parse input, invalid instruction
        }

    }

    if !mul_instructions.is_empty() {
        return Some(mul_instructions);
    }

    None
}

fn parse_input(buff: &[u8]) -> Option<(i32,i32,usize)> {

    let mut x = 0;
    let mut y = 0;

    let mut value_ref = &mut x;

    for i in 0..buff.len() {

        if buff[i] >= b'0' && buff[i] <= b'9' {
            *value_ref = *value_ref * 10 + (buff[i] - b'0') as i32;
            continue;
        }
        if b',' == buff[i]  {
            value_ref = &mut y;
            continue;
        }
        if b')' == buff[i] {
            return Some((x,y, i));
        }

        break;

    }
    None
}

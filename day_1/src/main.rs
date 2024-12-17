use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::zip;
use std::str::FromStr;

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
        // Ok(file) => compute_distance_part_one(file),
        Ok(file) => compute_similarity_part_two(file),
        Err(error) => {
            println!("Error reading file: {}", error);
            return;
        }
    };

}

fn compute_distance_part_one(file: File){
    // let left = []std::i;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut left = Vec::new();
    let mut right = Vec::new();

    while reader.read_line(&mut line).unwrap() > 0 {

        // read each line and split by white space
        let nums: Vec<i32> = line.split(" ")
            .filter(|s| !s.is_empty())
            // convert to int 32
            .map(|x| i32::from_str(x.trim()).unwrap())
            .collect();
        left.push(nums[0]);
        right.push(nums[1]);
        // clear buffer
        line.clear();
    }

    left.sort();
    right.sort();

    let diff: Vec<i32> = zip(&left, &right)
        .map(|x| (x.0 - x.1).abs())
        .collect();

    // println!("Difference: {:?}", diff);
    println!("sum: {}", diff.iter().sum::<i32>());

}

fn compute_similarity_part_two(file: File){
    // let left = []std::i;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut left = Vec::new();
    let mut right_map: HashMap<i32, i32> = HashMap::new();

    while reader.read_line(&mut line).unwrap() > 0 {

        // read each line and split by white space
        let nums: Vec<i32> = line.split(" ")
            .filter(|s| !s.is_empty())
            // convert to int 32
            .map(|x| i32::from_str(x.trim()).unwrap())
            .collect();

        left.push(nums[0]);
        // right_map.entry(nums[0])
        let value = right_map.entry(nums[1]).or_insert(0);
        *value += 1;

        // clear buffer
        line.clear();
    }

    let similarity: i32 = left.iter()
        .map(|x| match right_map.get(x){
            Some(val) => val * x,
            None => 0
        })
        .sum();

    // println!("Difference: {:?}", diff);
    println!("sum: {:?}", similarity);

}



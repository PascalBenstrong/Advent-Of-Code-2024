use std::fs::File;
use std::io::{BufRead, BufReader};
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
        // Ok(file) => compute_valid_levels_part_one(file),
        Ok(file) => compute_valid_levels_part_two(file),
        Err(error) => {
            println!("Error reading file: {}", error);
            return;
        }
    };

}

fn compute_valid_levels_part_one(file: File){

    let levels = read_levels(file, |x| is_valid_level(&diff(x)));

    println!("Levels: {}", levels.len());

}

fn compute_valid_levels_part_two(file: File){

    let levels = read_levels(file, is_valid_level_dampened);

    println!("Levels: {}", levels.len());

}

fn read_levels(file: File, is_valid_level: fn(&Vec<i32>) -> bool) -> Vec<Vec<i32>> {
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    let mut levels = Vec::new();

    while reader.read_line(&mut line).unwrap() > 0 {

        // read each line and split by white space
        let nums: Vec<i32> = line.split(" ")
            .filter(|s| !s.is_empty())
            // convert to int 32
            .map(|x| i32::from_str(x.trim()).unwrap())
            .collect();

        // clear buffer
        line.clear();

        if !is_valid_level(&nums) {
            continue;
        }
        // add level
        levels.push(nums);

    }

    levels

}

fn is_valid_level(diffs: &Vec<i32>) -> bool {

    let all_positive = diffs.iter().all(|x| *x > 0 && (*x).abs() <= 3);
    let all_negative = diffs.iter().all(|x| *x < 0 && (*x).abs() <= 3);

    if all_positive {
        return true;
    }

    all_negative
}

fn is_valid_level_dampened(levels: &Vec<i32>) -> bool {

    let mut diffs = diff(&levels);

    if is_valid_level(&diffs){
        return true;
    }

    for i in 0..levels.len() {
        let mut copy = levels.clone();
        copy.remove(i);

        diffs = diff(&copy);

        if is_valid_level(&diffs) {
            return true;
        }

    }

    false
}

fn diff(nums: &Vec<i32>) -> Vec<i32> {
    let mut diffs: Vec<i32> = Vec::with_capacity(nums.len() - 1);

    for i in 0..(nums.len() - 1) {
        diffs.push(nums[i] - nums[1 + i]);
    }

    diffs
}

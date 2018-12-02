use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
    println!("ArchSonix Advent of Code 2018");
    day1();
}

fn day1() {
    let file_handle = File::open("data/day1/input.txt").expect("Day 1: could not find file");
    let file = BufReader::new(&file_handle);

    let mut frequency_increments: Vec<i32> = vec![];
    for line in file.lines().enumerate() {
        frequency_increments.push(line.1.unwrap().parse().unwrap());
    }

    let mut frequencies: Vec<i32> = vec![];
    let mut first_run_frequency: i32 = 0;
    let mut first_run_complete: bool = false;
    let mut first_repeated_frequency: i32 = 0;
    let mut first_repeated_frequency_found: bool = false;

    let mut current_frequency: i32 = 0;
    while !first_repeated_frequency_found {
        for increment in frequency_increments.clone() {
            current_frequency += increment;

            if !first_repeated_frequency_found && frequencies.contains(&current_frequency) {
                first_repeated_frequency = current_frequency;
                first_repeated_frequency_found = true;
            }
            frequencies.push(current_frequency);
        }
        if !first_run_complete {
            first_run_frequency = current_frequency;
            first_run_complete = true;
        }
    }

    println!("Day 1.1: {}", first_run_frequency);
    println!("Day 1.2: {}", first_repeated_frequency);
}

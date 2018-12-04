use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file_handle = File::open("input.txt").expect("Couldn't find input.txt");
    let mut input = String::new();
    file_handle.read_to_string(&mut input).expect("Couldn't read input");

    day1(&input)
}

fn day1(input: &str) {
    let mut frequency_increments: Vec<i32> = vec!();
    for line in input.lines() {
        frequency_increments.push(line.parse().unwrap());
    }

    let mut frequencies: Vec<i32> = vec!();
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

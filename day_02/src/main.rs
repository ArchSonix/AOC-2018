use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't find input.txt");

    part_1(&input);
    part_2(&input)
}

fn part_1(input: &str) {
    let mut twice: u32 = 0;
    let mut thrice: u32 = 0;

    for line in input.lines() {
        let mut line_map: HashMap<char, u32> = HashMap::new();
        line.chars()
            .enumerate()
            .for_each(|(_i, line_char)| *line_map.entry(line_char).or_insert(0) += 1);

        if line_map.iter().any(|(_i, &val)| val == 2) {
            twice += 1;
        }
        if line_map.iter().any(|(_i, &val)| val == 3) {
            thrice += 1;
        }
    }

    println!("Day 2.1: {}", twice * thrice);
}

fn part_2(input: &str) {
    // println!("Day 2.2: {}", twice * thrice);
}

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("Couldn't find input.txt");

    part_1(&input.trim());
    part_2(&input.trim())
}

fn part_1(input: &str) {
    println!("Day 5.1: {}", react_polymer(input).len())
}

fn part_2(input: &str) {
    // Find all letters && make them unique
    let mut letters: Vec<char> = input.to_uppercase().chars().collect();
    letters.sort();
    letters.dedup();

    let smallest_len: usize = letters
        .into_iter()
        // Calculate length for variant
        .map(|ch| {
            let st = input
                .replace(ch, "")
                .replace(&ch.to_lowercase().collect::<String>(), "");
            react_polymer(&st).len()
        })
        .enumerate()
        // Find variant with lowest length
        .min_by(|(_i1, len1), (_i2, len2)| {
            len1.partial_cmp(len2).expect("Tried to compare to a nan")
        })
        .map(|(_i, len)| len)
        .unwrap();

    println!("Day 5.2: {}", smallest_len)
}

fn react_polymer(input: &str) -> String {
    let mut input_chars: Vec<char> = input.chars().collect();
    (0..input_chars.len()).rev().for_each(|i| {
        if input_chars.get(i + 1) == Option::None {
            return;
        }
        if unit_unstable(input_chars[i], input_chars[i + 1]) {
            input_chars.remove(i + 1);
            input_chars.remove(i);
        }
    });

    input_chars.into_iter().collect()
}

fn unit_unstable(left: char, right: char) -> bool {
    left.to_uppercase().collect::<String>() == right.to_uppercase().collect::<String>()
        && left != right
}

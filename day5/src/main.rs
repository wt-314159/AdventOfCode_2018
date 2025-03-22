#[allow(unused_imports)]
use std::{cmp::max, cmp::min, collections::HashMap, fs};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    println!("{:?}", input);
    println!("Input length: {}", input.len());

    part_two(input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let result = do_all_reactions(input.to_string());
    println!("Remaining units: {}", result.len());
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let mut min_length = input.len();
    for c in 'a'..='z' {
        let mut removed = input.to_string().replace(c, "");
        removed = removed.replace(c.to_ascii_uppercase(), "");
        let result = do_all_reactions(removed);
        if result.len() < min_length {
            min_length = result.len();
        }
    }
    println!("Shortest possible polymer is: {min_length}");
}

fn do_all_reactions(mut polymer: String) -> String {
    loop {
        if !react_polymer(&mut polymer) {
            break;
        }
        // eprintln!("{polymer}");
    }
    polymer
}

fn react_polymer(polymer: &mut String) -> bool {
    let mut prev_char = ' ';
    let mut prev_uppercase = false;
    let mut to_remove = Vec::new();
    for (i, c) in polymer.chars().enumerate() {
        if c.is_uppercase() != prev_uppercase && c.to_ascii_lowercase() == prev_char {
            to_remove.push(i - 1);
            to_remove.push(i);
            prev_char = ' ';
        } else {
            prev_uppercase = c.is_uppercase();
            prev_char = c.to_ascii_lowercase();
        }
    }
    for i in to_remove.iter().rev() {
        polymer.remove(*i);
    }
    !to_remove.is_empty()
}

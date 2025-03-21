#[allow(unused_imports)]
use std::{fs, collections::HashMap, cmp::min, cmp::max};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    println!("{:?}", input);
    println!("Input length: {}", input.len());
    println!("Hello world!");
    println!("Trying to change exe to see if windows will let it run");
    part_one(&input); 
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let result = do_all_reactions(input.to_string());
    println!("Remaining units: {}", result.len());
}

#[allow(dead_code)]
fn part_two(input: &str) {
    
}

fn do_all_reactions(mut polymer: String) -> String {
    loop {
        if !react_polymer(&mut polymer) { break; }
        // eprintln!("{polymer}");
    };
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
    to_remove.len() != 0
}
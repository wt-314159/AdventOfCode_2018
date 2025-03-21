#[allow(unused_imports)]
use std::{cmp::max, cmp::min, collections::HashMap, fs};
// use fancy_regex::Regex;
// use regex::Regex;
// use md5::{Digest, Md5};
// use priority_queue::PriorityQueue;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    // println!("{:?}", input);
    println!("Input length: {}", input.len());

    part_two(&input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let (twos, threes) = count_twos_and_threes(input);
    let checksum = twos * threes;
    println!("Checksum: {checksum}");
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let Some((match1, match2)) = find_almost_matching_ids(input) else {
        panic!("Failed to find matching strings");
    };
    let same = match1
        .chars()
        .enumerate()
        .filter(|(i, c)| match2.chars().nth(*i).unwrap() == *c)
        .map(|(_, c)| c)
        .collect::<String>();
    println!("Matching letters in answer: {same}");
}

fn count_twos_and_threes(input: &str) -> (usize, usize) {
    let (mut twos, mut threes) = (0, 0);
    for line in input.lines() {
        let mut char_map = HashMap::with_capacity(line.len());
        for c in line.chars() {
            *char_map.entry(c).or_insert(0) += 1;
        }
        if char_map.iter().any(|(_, &n)| n == 2) {
            twos += 1;
        }
        if char_map.iter().any(|(_, &n)| n == 3) {
            threes += 1;
        }
    }

    (twos, threes)
}

fn find_almost_matching_ids(input: &str) -> Option<(&str, &str)> {
    for (i, line) in input.lines().enumerate() {
        for pos_match in input.lines().take(i) {
            let mut diffs = line
                .chars()
                .enumerate()
                .filter(|(i, c)| pos_match.chars().nth(*i).unwrap() != *c)
                .count();
            if diffs == 1 {
                return Some((pos_match, line));
            }
        }
    }
    None
}

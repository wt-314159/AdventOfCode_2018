use std::collections::HashSet;
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

    part_two(input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let mut freq = 0;
    for shift in parse_input(input) {
        freq += shift;
    }
    println!("Frequency after shifting: {freq}");
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let mut freq = 0;
    let mut set = HashSet::new();
    let shifts = parse_input(input).collect::<Vec<_>>();
    let mut found = None;

    while found.is_none() {
        for shift in shifts.iter() {
            freq += shift;
            if set.contains(&freq) {
                found = Some(freq);
                break;
            }
            set.insert(freq);
        }
    }

    let Some(dupe) = found else {
        unreachable!("Because we only break when found is some.")
    };
    println!("First duplicate freq: {dupe}");
}

fn parse_input(input: &str) -> impl Iterator<Item = i32> + use<'_> {
    input.lines().map(|l| l.parse::<i32>().unwrap())
}

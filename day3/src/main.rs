#[allow(unused_imports)]
use std::{cmp::max, cmp::min, collections::HashMap, fs};
use std::{num::ParseIntError, str::FromStr};
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
    let claims = input
        .lines()
        .map(|l| l.parse::<Claim>().unwrap())
        .collect::<Vec<_>>();
    let overlap = find_overlap_size(&claims);
    println!("Overlapping area: {overlap}");
}

#[allow(dead_code)]
fn part_two(_input: &str) {
    let claims = _input
        .lines()
        .map(|l| l.parse::<Claim>().unwrap())
        .collect::<Vec<_>>();
    let non_overlapping = find_non_overlapping_claim(&claims).unwrap();
    println!("Non overlapping claim id: {}", non_overlapping.id);
}

struct Claim {
    id: usize,
    left: usize,
    top: usize,
    right: usize,
    bottom: usize,
}

impl FromStr for Claim {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let id = parts.next().unwrap().trim_matches('#').parse::<usize>()?;
        parts.next().unwrap();
        let position = parts.next().unwrap().trim_matches(':');
        let mut position = position.split(',');
        let (left, top) = (
            position.next().unwrap().parse::<usize>()?,
            position.next().unwrap().parse::<usize>()?,
        );
        let size = parts.next().unwrap();
        let mut size = size.split('x');
        let (right, bottom) = (
            left + size.next().unwrap().parse::<usize>()? - 1,
            top + size.next().unwrap().parse::<usize>()? - 1,
        );

        Ok(Self {
            id,
            top,
            left,
            right,
            bottom,
        })
    }
}

impl Claim {
    fn cells(&self) -> impl Iterator<Item = (usize, usize)> + use<'_> {
        (self.top..=self.bottom).flat_map(|r| (self.left..=self.right).map(move |c| (r, c)))
    }
}

struct Grid {
    grid: Box<[u8]>,
    stride: usize,
}

impl Grid {
    fn new(size: usize) -> Self {
        Self {
            grid: vec![0; size * size].into_boxed_slice(),
            stride: size,
        }
    }

    fn from_claims(claims: &Vec<Claim>) -> Self {
        let max_size = claims.iter().map(|c| max(c.bottom, c.right)).max().unwrap();
        let mut grid = Grid::new(max_size + 1);
        for claim in claims {
            for (r, c) in claim.cells() {
                *grid.cell(r, c) += 1;
            }
        }
        grid
    }

    #[allow(dead_code)]
    fn rows(&self) -> impl Iterator<Item = &[u8]> {
        self.grid.chunks_exact(self.stride)
    }

    fn cell(&mut self, row: usize, col: usize) -> &mut u8 {
        &mut self.grid[row * self.stride + col]
    }

    fn count_overlaps(&self) -> usize {
        self.grid.iter().filter(|c| **c > 1).count()
    }
}

fn find_overlap_size(claims: &Vec<Claim>) -> usize {
    let grid = Grid::from_claims(claims);
    grid.count_overlaps()
}

fn find_non_overlapping_claim(claims: &Vec<Claim>) -> Option<&Claim> {
    let mut grid = Grid::from_claims(claims);
    claims
        .iter()
        .find(|&claim| claim.cells().all(|(r, c)| *(grid.cell(r, c)) < 2))
}

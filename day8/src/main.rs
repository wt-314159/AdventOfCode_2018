use std::num::ParseIntError;

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    eprintln!("Input length: {}", input.len());

    part_one(input);
}

fn part_one(input: &str) {
    let (root, remain) = Node::parse(input).expect("Failed to parse root node");
    assert!(remain.is_empty());
    let metadata_sum = root.sum_metadata();

    println!("Total metadata: {metadata_sum}");

    println!("Root node value: {}", root.value());
}

struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn sum_metadata(&self) -> usize {
        let mut sum = self.metadata.iter().sum();
        for child in &self.children {
            sum += child.sum_metadata();
        }
        sum
    }

    fn value(&self) -> usize {
        if self.children.is_empty() {
            dbg!(self.metadata.iter().sum())
        } else {
            self.metadata
                .iter()
                .map(|&m| {
                    if m != 0 && m <= self.children.len() {
                        self.children[m - 1].value()
                    } else {
                        0
                    }
                })
                .sum()
        }
    }

    fn parse(s: &str) -> Result<(Self, &str), ParseIntError> {
        let (num_children, remainder) = s
            .split_once(' ')
            .expect("Failed to find number of children");
        let num_children = num_children.parse::<usize>()?;
        let (num_metadata, mut remainder) = remainder
            .split_once(' ')
            .expect("Failed to find number of metadata values");
        let num_metadata = num_metadata.parse::<usize>()?;

        let mut children = Vec::new();
        let mut child;
        for _ in 0..num_children {
            (child, remainder) = Self::parse(remainder).expect("Failed to parse child");
            children.push(child);
        }
        let metadata = (0..num_metadata)
            .map(|_| {
                let (metadata, remain) = remainder.split_once(' ').unwrap_or((remainder, ""));
                remainder = remain;
                metadata.parse::<usize>()
            })
            .collect::<Result<Vec<_>, ParseIntError>>()?;

        Ok((Self { children, metadata }, remainder))
    }
}

use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // let input = include_str!("../inputs/test_puzzle_input.txt");
    eprintln!("input length: {}", input.len());

    // part_one(input);
    part_two(input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let steps = parse_input(input);
    eprintln!("Parsed {} steps", steps.len());
    let mut instructions = Instructions::new(steps);
    let ordered_steps = instructions.order_steps();
    ordered_steps.iter().for_each(|id| print!("{id}"));
    println!();
}

fn part_two(input: &str) {
    let steps = parse_input(input);
    eprintln!("Parsed {} steps", steps.len());
    let mut instructions = Instructions::new(steps);
    let time_to_complete = instructions.time_to_complete();
    eprintln!("Time to complete: {time_to_complete} seconds");
}

fn parse_input(input: &str) -> HashMap<char, Step> {
    let mut map = HashMap::new();
    input.lines().for_each(|l| {
        let mut parts = l.split_whitespace();
        let prereq = parts.nth(1).unwrap().chars().next().unwrap();
        let id = parts.nth(5).unwrap().chars().next().unwrap();
        let entry = map.entry(id).or_insert(Step::default());
        entry.prerequisites.push(prereq);

        let prereq_entry = map.entry(prereq).or_insert(Step::default());
        prereq_entry.dependants.push(id);
    });
    map
}

#[derive(Debug, Default, Clone)]
struct Instructions {
    queued_steps: BinaryHeap<Reverse<char>>,
    completed_steps: Vec<char>,
    all_steps: HashMap<char, Step>,
    in_progress: Vec<ProgressStep>,
}

impl Instructions {
    fn new(all_steps: HashMap<char, Step>) -> Self {
        Self {
            all_steps,
            ..Default::default()
        }
    }

    fn time_to_complete(&mut self) -> usize {
        let mut total_time = 0;
        loop {
            self.queue_steps();
            self.start_steps();
            let Some(completed) = self.in_progress.pop() else {
                break;
            };
            total_time += completed.time;
            self.in_progress
                .iter_mut()
                .for_each(|s| s.time -= completed.time);
            self.complete_step(completed.id);
        }
        total_time
    }

    fn start_steps(&mut self) {
        while self.in_progress.len() < 5 {
            let Some(Reverse(next_step)) = self.queued_steps.pop() else {
                break;
            };
            self.in_progress.push(ProgressStep::new(next_step));
            self.in_progress.sort();
            self.in_progress.reverse();
        }
    }

    fn order_steps(&mut self) -> &Vec<char> {
        loop {
            self.queue_steps();
            let Some(Reverse(next_step)) = self.queued_steps.pop() else {
                break;
            };
            self.complete_step(next_step);
        }
        &self.completed_steps
    }

    fn complete_step(&mut self, step_id: char) {
        self.completed_steps.push(step_id);
        self.all_steps
            .iter_mut()
            .filter(|(_, s)| !s.queued)
            .for_each(|(_, s)| s.remove_prerequisite(step_id));
    }

    fn queue_steps(&mut self) {
        self.all_steps
            .iter_mut()
            .filter(|(_, s)| !s.queued && s.prerequisites.is_empty())
            .for_each(|(id, s)| {
                self.queued_steps.push(Reverse(*id));
                s.queued = true;
            });
    }
}

#[derive(Default, Debug, Clone)]
struct ProgressStep {
    id: char,
    time: usize,
}

impl ProgressStep {
    fn new(id: char) -> Self {
        let time = 61 + id as usize - 'A' as usize;
        Self { id, time }
    }
}

impl PartialEq for ProgressStep {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for ProgressStep {}

impl PartialOrd for ProgressStep {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.time.cmp(&other.time))
    }
}

impl Ord for ProgressStep {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.time.cmp(&other.time)
    }
}

#[derive(Default, Debug, Clone)]
struct Step {
    queued: bool,
    prerequisites: Vec<char>,
    dependants: Vec<char>,
}

impl Step {
    fn remove_prerequisite(&mut self, prereq_id: char) {
        self.prerequisites.retain(|p| *p != prereq_id);
    }
}

#[cfg(test)]
mod tests {
    use std::{cmp::Reverse, collections::BinaryHeap};

    use crate::*;

    #[test]
    fn test_binary_heap() {
        let mut heap = BinaryHeap::new();
        heap.push(Reverse('F'));
        heap.push(Reverse('A'));
        heap.push(Reverse('X'));
        heap.push(Reverse('D'));

        assert_eq!(heap.pop(), Some(Reverse('A')));
        assert_eq!(heap.pop(), Some(Reverse('D')));
        assert_eq!(heap.pop(), Some(Reverse('F')));
        assert_eq!(heap.pop(), Some(Reverse('X')));
    }

    #[test]
    fn test_step_time() {
        let step1 = ProgressStep::new('A');
        let step2 = ProgressStep::new('Z');

        assert_eq!(step1.time, 61);
        assert_eq!(step2.time, 86);
    }
}

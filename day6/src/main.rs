use std::{collections::HashSet, fmt::Display, num::ParseIntError, str::FromStr};

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    // let input = include_str!("../inputs/test_puzzle_input.txt");

    part_two(input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let coords = get_all_coords(input);
    let max_x = coords.iter().map(|c| c.x).max().unwrap();
    let max_y = coords.iter().map(|c| c.y).max().unwrap();
    let mut grid = Grid::new(max_x + 2, max_y + 2);
    grid.find_nearest_coords(&coords);
    eprintln!("Found all nearest coordinates");
    let infinite = dbg!(grid.find_infinite_areas());
    let areas = dbg!(grid.find_areas());

    // eprintln!("{grid}");
    let max_area = areas
        .iter()
        .enumerate()
        .filter(|(i, _)| !infinite.contains(&(*i as u8)))
        .max_by(|(_, a1), (_, a2)| a1.cmp(a2))
        .unwrap();

    println!(
        "Coord with max area has id: {}, and area: {}",
        max_area.0, max_area.1
    );
}

#[allow(dead_code)]
fn part_two(input: &str) {
    let coords = get_all_coords(input);
    let max_x = coords.iter().map(|c| c.x).max().unwrap();
    let max_y = coords.iter().map(|c| c.y).max().unwrap();
    let grid = Grid::new(max_x + 2, max_y + 2);

    let safe_area_size = grid.find_size_of_safe_region(&coords);
    println!("Size of safe region: {safe_area_size}");
}

fn get_all_coords(input: &str) -> Vec<Coord> {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.parse::<Coord>()
                .unwrap_or_else(|_| panic!("Failed to parse line: '{l}'"))
                .set_id(i + 1)
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Grid {
    grid: Box<[u8]>,
    stride: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.grid.chunks_exact(self.stride) {
            for id in row {
                if *id == 0 {
                    write!(f, ".")?;
                } else {
                    // write!(f, "{id:_>2},")?
                    write!(f, "{id}")?
                }
            }
            writeln!(f, "|")?;
        }
        Ok(())
    }
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let grid = vec![0u8; width * height].into_boxed_slice();
        Grid {
            grid,
            stride: width,
        }
    }

    fn find_size_of_safe_region(&self, coords: &[Coord]) -> usize {
        let mut size = 0;
        let (mut x, mut y) = (0, 0);
        self.grid.iter().for_each(|_| {
            let total_manhattan: usize = coords.iter().map(|c| c.manhattan_distance((x, y))).sum();
            if total_manhattan < 10_000 {
                size += 1;
            }
            x += 1;
            if x >= self.stride {
                y += 1;
                x = 0;
            }
        });
        size
    }

    fn find_infinite_areas(&self) -> HashSet<u8> {
        // All ids on the borders will extend infitely
        let mut infinite = HashSet::new();
        self.grid.iter().take(self.stride).for_each(|id| {
            infinite.insert(*id);
        });
        self.grid
            .iter()
            .skip(self.stride)
            .step_by(self.stride)
            .for_each(|id| {
                infinite.insert(*id);
            });
        self.grid
            .iter()
            .skip(self.stride - 1)
            .step_by(self.stride)
            .for_each(|id| {
                infinite.insert(*id);
            });
        self.grid.iter().rev().take(self.stride).for_each(|id| {
            infinite.insert(*id);
        });
        infinite
    }

    fn find_nearest_coords(&mut self, coords: &[Coord]) {
        let (mut x, mut y) = (0, 0);
        for n in self.grid.iter_mut() {
            let mut smallest = usize::MAX - 1;
            let mut second_smallest = usize::MAX;
            let mut smallest_id = 0;
            coords
                .iter()
                .map(|c| (c.manhattan_distance((x, y)), c))
                .for_each(|(d, c)| {
                    if d < smallest {
                        second_smallest = smallest;
                        smallest = d;
                        smallest_id = c.id;
                    } else if d < second_smallest {
                        second_smallest = d;
                    }
                });
            if smallest != second_smallest {
                *n = smallest_id;
            }
            x += 1;
            if x >= self.stride {
                y += 1;
                x = 0;
            }
        }
    }

    fn find_areas(&self) -> [i32; 51] {
        let mut areas = [0; 51];
        self.grid.iter().for_each(|id| areas[usize::from(*id)] += 1);
        areas
    }
}

struct Coord {
    id: u8,
    x: usize,
    y: usize,
}

impl Coord {
    fn set_id(mut self, id: usize) -> Self {
        self.id = id as u8;
        self
    }

    fn manhattan_distance(&self, (point_x, point_y): (usize, usize)) -> usize {
        self.x.abs_diff(point_x) + self.y.abs_diff(point_y)
    }
}

impl FromStr for Coord {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(", ");
        Ok(Self {
            id: 0,
            x: split.next().unwrap().parse::<usize>()?,
            y: split.next().unwrap().parse::<usize>()?,
        })
    }
}

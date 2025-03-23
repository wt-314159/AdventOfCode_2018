use std::{num::ParseIntError, str::FromStr};

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");

    part_one(input);
}

#[allow(dead_code)]
fn part_one(input: &str) {
    let coords = get_all_coords(input);
    let max_x = coords.iter().map(|c| c.x).max().unwrap();
    let max_y = coords.iter().map(|c| c.y).max().unwrap();
    let mut grid = Grid::new(max_x + 5, max_y + 5);
    grid.find_nearest_coords(&coords);
    eprintln!("Found all nearest coordinates");
}

#[allow(dead_code)]
fn part_two(_input: &str) {}

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

struct Grid {
    grid: Box<[u8]>,
    stride: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let grid = vec![0u8; width * height].into_boxed_slice();
        Grid {
            grid,
            stride: width,
        }
    }

    fn find_nearest_coords(&mut self, coords: &[Coord]) {
        let (mut row, mut col) = (0, 0);
        for n in self.grid.iter_mut() {
            *n = coords
                .iter()
                .map(|c| (c.manhattan_distance((row, col)), c))
                .min_by(|(d1, _), (d2, _)| d1.cmp(d2))
                .unwrap()
                .1
                .id;
            col += 1;
            if col >= self.stride {
                row += 1;
                col = 0;
            }
        }
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

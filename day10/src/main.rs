use std::{
    fmt::Display,
    num::ParseIntError,
    ops::{AddAssign, Mul, Sub},
    str::FromStr,
};

fn main() {
    let input = include_str!("../inputs/puzzle_input.txt");
    let input = include_str!("../inputs/test_puzzle_input.txt");

    part_one(input);
}

fn part_one(input: &str) {
    let mut lights = input
        .parse::<Space>()
        .expect("Failed to parse input to lights");
    eprintln!("{lights:?}");

    // Go through 2 steps at a time until the y values converge to under 100
    while lights.y_spread() > 100 {
        lights.step_by(2);
    }
    // Then go through one at a time until the y values range is less than 15
    while lights.y_spread() > 9 {
        lights.step_by(1);
    }
    // Then, until the lights diverge again, print each stepj
    while lights.y_spread() <= 9 {
        println!("{lights}");
        println!("============================================");
        lights.step_by(1);
    }
}

#[derive(Debug, Clone)]
struct Space {
    lights: Box<[Light]>,
}

impl Space {
    fn y_spread(&self) -> i32 {
        self.lights.iter().map(|l| l.position.1).max().unwrap()
            - self.lights.iter().map(|l| l.position.1).max().unwrap()
    }

    fn step_by(&mut self, step: i32) {
        self.lights.iter_mut().for_each(|l| l.step_by(step));
    }
}

impl FromStr for Space {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lights = s
            .lines()
            .map(|l| l.parse::<Light>())
            .collect::<Result<Vec<Light>, ParseIntError>>()?
            .into_boxed_slice();
        Ok(Space { lights })
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.lights.iter().map(|l| l.position.0).min().unwrap() - 3;
        let max_x = self.lights.iter().map(|l| l.position.0).max().unwrap() + 3;
        let min_y = self.lights.iter().map(|l| l.position.1).min().unwrap() - 3;
        let max_y = self.lights.iter().map(|l| l.position.1).max().unwrap() + 3;
        let y_range = (max_y - min_y) as usize;
        let x_range = (max_x - min_x) as usize;

        eprintln!("x: {min_x} -> {max_x}, y: {min_y} -> {max_y}");
        let buffer_start = Point(min_x, min_y);
        let mut buffer = vec![vec!['.'; x_range + 1]; y_range + 1];
        self.lights.iter().for_each(|l| {
            let relative_pos = l.position - buffer_start;
            buffer[relative_pos.1 as usize][relative_pos.0 as usize] = '#';
        });

        for row in buffer.iter().take(y_range + 1) {
            for col in row.iter().take(x_range + 1) {
                write!(f, "{0}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point(i32, i32);

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Mul<Point> for i32 {
    type Output = Point;

    fn mul(self, rhs: Point) -> Self::Output {
        Point(rhs.0 * self, rhs.1 * self)
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Light {
    position: Point,
    velocity: Point,
}

impl Light {
    fn step_by(&mut self, num_steps: i32) {
        self.position += num_steps * self.velocity;
    }

    fn y_pos(&self) -> i32 {
        self.position.1
    }
}

impl FromStr for Light {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, remainder) = s
            .split_once('<')
            .expect("Failed to find position start char '<'");
        let (position, remainder) = remainder
            .split_once('>')
            .expect("Failed to find position end char '>'");
        let (x, y) = position
            .split_once(", ")
            .expect("Position incorrectly formatted");
        let position = Point(x.trim().parse::<i32>()?, y.trim().parse::<i32>()?);

        let (_, remainder) = remainder
            .split_once('<')
            .expect("Failed to find velocity start char '<'");
        let (velocity, _) = remainder
            .split_once('>')
            .expect("Failed to find velocity end char '>'");
        let (vx, vy) = velocity
            .split_once(", ")
            .expect("Velocity incorrectly formatted");
        let velocity = Point(vx.trim().parse::<i32>()?, vy.trim().parse::<i32>()?);

        Ok(Self { position, velocity })
    }
}

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign};
use std::{fmt, fs, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq)]
struct ComplexNumber {
    x: i64,
    y: i64,
}

impl ComplexNumber {
    fn from_file(path: &str) -> Result<Self, &'static str> {
        let content = fs::read_to_string(path).map_err(|_| "Failed to read file")?;
        let content = content
            .trim()
            .strip_prefix("A=")
            .unwrap_or(content.as_str());
        content.parse::<ComplexNumber>()
    }
}

impl fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{},{}]", self.x, self.y)
    }
}

impl FromStr for ComplexNumber {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim();

        let s = s
            .strip_prefix('[')
            .and_then(|s| s.strip_suffix(']'))
            .unwrap_or(s)
            .trim();

        let s = s
            .strip_prefix('(')
            .and_then(|s| s.strip_suffix(')'))
            .unwrap_or(s)
            .trim();

        let (x_str, y_str) = s
            .split_once(',')
            .ok_or("Expected format: x,y, [x,y] or (x,y)")?;

        let x = x_str.trim().parse::<i64>().map_err(|_| "Invalid x value")?;
        let y = y_str.trim().parse::<i64>().map_err(|_| "Invalid y value")?;

        Ok(ComplexNumber { x, y })
    }
}

impl Add for ComplexNumber {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        ComplexNumber {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for ComplexNumber {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        ComplexNumber {
            x: self.x * other.x - self.y * other.y,
            y: self.x * other.y + self.y * other.x,
        }
    }
}

impl Div for ComplexNumber {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let x_res_mod = self.x % other.x;
        let y_res_mod = self.y % other.y;
        ComplexNumber {
            x: (self.x - x_res_mod) / other.x,
            y: (self.y - y_res_mod) / other.y,
        }
    }
}

impl AddAssign for ComplexNumber {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl MulAssign for ComplexNumber {
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl DivAssign for ComplexNumber {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self / rhs;
    }
}

fn main() {
    let test1 = ComplexNumber::from_file("input/test1.txt").unwrap();
    let input1 = ComplexNumber::from_file("input/input1.txt").unwrap();
    let test2 = ComplexNumber::from_file("input/test2.txt").unwrap();
    let input2 = ComplexNumber::from_file("input/input2.txt").unwrap();

    println!("Test 1: {}", part_1(test1));
    println!("Part 1: {}", part_1(input1));
    println!("Test 2: {}", part_2(test2));
    println!("Part 2: {}", part_2(input2));
    println!("Test 3: {}", part_3(test2));
    println!("Part 3: {}", part_3(input2));
}

fn part_1(complex_number_a: ComplexNumber) -> ComplexNumber {
    let mut result = ComplexNumber { x: 0, y: 0 };
    for _ in 0..3 {
        result *= result;
        result /= ComplexNumber { x: 10, y: 10 };
        result += complex_number_a;
    }

    result
}

fn part_2(corner: ComplexNumber) -> usize {
    let opposite_corner = corner + ComplexNumber { x: 1000, y: 1000 };

    let mut result = 0;

    let mut current_point = corner;

    while current_point.y <= opposite_corner.y {
        let mut current_point_copy = current_point;
        let mut should_be_engraved = true;
        for _ in 0..99 {
            current_point_copy *= current_point_copy;
            current_point_copy /= ComplexNumber {
                x: 100000,
                y: 100000,
            };
            current_point_copy += current_point;
            if current_point_copy.x > 1000000
                || current_point_copy.x < -1000000
                || current_point_copy.y > 1000000
                || current_point_copy.y < -1000000
            {
                should_be_engraved = false;
                break;
            }
        }
        if should_be_engraved {
            result += 1;
        }
        if current_point.x < opposite_corner.x {
            current_point.x += 10;
        } else {
            current_point.x = corner.x;
            current_point.y += 10;
        }
    }

    result
}

fn part_3(corner: ComplexNumber) -> usize {
    let opposite_corner = corner + ComplexNumber { x: 1000, y: 1000 };

    let mut result = 0;

    let mut current_point = corner;

    while current_point.y <= opposite_corner.y {
        let mut current_point_copy = current_point;
        let mut should_be_engraved = true;
        for _ in 0..99 {
            current_point_copy *= current_point_copy;
            current_point_copy /= ComplexNumber {
                x: 100000,
                y: 100000,
            };
            current_point_copy += current_point;
            if current_point_copy.x > 1000000
                || current_point_copy.x < -1000000
                || current_point_copy.y > 1000000
                || current_point_copy.y < -1000000
            {
                should_be_engraved = false;
                break;
            }
        }
        if should_be_engraved {
            result += 1;
        }
        if current_point.x < opposite_corner.x {
            current_point.x += 1;
        } else {
            current_point.x = corner.x;
            current_point.y += 1;
        }
    }

    result
}

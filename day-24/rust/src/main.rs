use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
struct Point {
    z: f64,
    y: f64,
    x: f64,
}

impl Point {
    fn parse(string: &str) -> Self {
        let mut coords = string.split(", ");
        let x = coords.next().unwrap().parse().unwrap();
        let y = coords.next().unwrap().parse().unwrap();
        let z = coords.next().unwrap().parse().unwrap();

        Self { z, y, x }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Velocity {
    z: f64,
    y: f64,
    x: f64,
}

impl Velocity {
    fn parse(string: &str) -> Self {
        let mut coords = string.split(", ");
        let x = coords.next().unwrap().parse().unwrap();
        let y = coords.next().unwrap().parse().unwrap();
        let z = coords.next().unwrap().parse().unwrap();

        Self { z, y, x }
    }

    fn xy_slope(&self) -> f64 {
        self.y / self.x
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Hailstone {
    position: Point,
    velocity: Velocity,
}

impl Hailstone {
    fn parse(line: &str) -> Self {
        let (position, velocity) = line.split_once(" @ ").unwrap();
        let position = Point::parse(position);
        let velocity = Velocity::parse(velocity);

        Self { position, velocity }
    }

    // y1 = m1 * x + b1
    // y2 = m2 * x + b2
    // m1 * x + b1 = m2 * x + b2
    // m1 * x = m2 * x + b2 - b1
    // m1 * x - m2 * x = b2 - b1
    // x * (m1 - m2) = b2 - b1
    // x = (b2 - b1) / (m1 - m2)
    fn xy_intersect(&self, other: &Self) -> Option<Point> {
        match self.xy_slope() == other.xy_slope() {
            true => None, // parallel
            false => {
                let x = (other.b() - self.b()) / (self.xy_slope() - other.xy_slope());
                Some(Point {
                    y: self.y(x),
                    x,
                    z: 0.0, // TODO
                })
            }
        }
    }

    fn y(&self, x: f64) -> f64 {
        self.xy_slope() * x + self.b()
    }

    fn xy_slope(&self) -> f64 {
        self.velocity.xy_slope()
    }

    fn b(&self) -> f64 {
        self.position.y - self.position.x * self.velocity.xy_slope()
    }

    fn is_in_future(&self, point: Point) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match (
            self.position.x.partial_cmp(&point.x),
            self.velocity.x.is_sign_negative(),
        ) {
            (Some(Ordering::Less), false) => true,
            (Some(Ordering::Greater), true) => true,
            _ => false,
        }
    }
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let hailstones: Vec<Hailstone> = input.lines().map(Hailstone::parse).collect();

    let mut part1 = 0;
    // let range = 7.0..=27.0;
    let range = 200000000000000.0..=400000000000000.0;

    for (i, a) in hailstones.iter().enumerate() {
        for b in hailstones.iter().skip(i) {
            if let Some(intersection) = a.xy_intersect(b) {
                if range.contains(&intersection.x)
                    && range.contains(&intersection.y)
                    && a.is_in_future(intersection)
                    && b.is_in_future(intersection)
                {
                    part1 += 1;
                }
            }
        }
    }

    println!("part1 = {part1}");
}

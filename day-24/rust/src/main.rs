use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::ops::RangeInclusive;

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

    fn is_integer(&self) -> bool {
        self.x.floor() == self.x && self.y.floor() == self.y
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

    fn adjust(self, offset: Self) -> Self {
        Self {
            x: self.x - offset.x,
            y: self.y - offset.y,
            z: self.z - offset.z,
        }
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
        // match dbg!(self.xy_slope()) == dbg!(other.xy_slope()) {
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

    //
    fn z_intersect(&self, other: &Self, y: f64, x: f64) -> Option<Point> {
        if self.velocity == other.velocity {
            None // parallel?
        } else {
            todo!()
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

    fn adjust_velocity(&self, offset: Velocity) -> Self {
        Self {
            velocity: self.velocity.adjust(offset),
            ..*self
        }
    }

    // Format as y = mx + b
    #[allow(dead_code)]
    fn debug(&self) -> String {
        format!("y = {}x + {}", self.xy_slope(), self.b())
    }

    // Format as a1 * y + a2 * x = 0
    #[allow(dead_code)]
    fn debug2(&self) -> String {
        format!("y - {}x = {}", self.xy_slope(), self.b())
    }

    // Format as x ~ y = 0
    #[allow(dead_code)]
    fn debug3(&self) -> String {
        format!(
            "((x - {})/{}) - ((y - {})/{}) = 0",
            self.position.x, self.velocity.x, self.position.y, self.velocity.y
        )
    }

    // Format as x ~ y ~ z = 0
    #[allow(dead_code)]
    fn debug4(&self) -> String {
        format!(
            "((x - {})/{}) - ((y - {})/{}) - ((z - {})/{}) = 0",
            self.position.x,
            self.velocity.x,
            self.position.y,
            self.velocity.y,
            self.position.z,
            self.velocity.z
        )
    }

    // Same as above but with isolated variables.
    #[allow(dead_code)]
    fn debug5(&self) -> String {
        format!(
            "({}x - {}) - ({}y - {}) - ({}z - {}) = 0",
            self.velocity.x.recip(),
            self.position.x / self.velocity.x,
            self.velocity.y.recip(),
            self.position.y / self.velocity.y,
            self.velocity.z.recip(),
            self.position.z / self.velocity.z
        )
    }

    // Same as above but with combined scalars.
    #[allow(dead_code)]
    fn debug6(&self) -> String {
        format!(
            "{}x - {}y - {}z - {} = 0",
            self.velocity.x.recip(),
            self.velocity.y.recip(),
            self.velocity.z.recip(),
            self.position.x / self.velocity.x
                - self.position.y / self.velocity.y
                - self.position.z / self.velocity.z
        )
    }
}

fn find_xy_intersections(
    hailstones: &[Hailstone],
    range: RangeInclusive<f64>,
) -> (Vec<Point>, usize) {
    let mut intersections = vec![];
    let mut parallel = 0;

    for (i, a) in hailstones.iter().enumerate() {
        for b in hailstones.iter().skip(i + 1) {
            // println!("{:?}", a.position);
            // println!("{:?}", a.velocity);
            // println!("{:?}", b.position);
            // println!("{:?}", b.velocity);
            if let Some(intersection) = a.xy_intersect(b) {
                if range.contains(&intersection.x)
                    && range.contains(&intersection.y)
                    && a.is_in_future(intersection)
                    && b.is_in_future(intersection)
                {
                    // println!("intersect at {intersection:?}");
                    intersections.push(intersection);
                } else {
                    // println!("not in range");
                }
            } else {
                // println!("no intersection");
                parallel += 1;
            }
            // println!();
        }
    }

    (intersections, parallel)
}

// fn find_xyz_intersections(hailstones: &[Hailstone], range: RangeInclusive<f64>) -> Vec<Point> {
//     let mut intersections = vec![];

//     for (i, a) in hailstones.iter().enumerate() {
//         for b in hailstones.iter().skip(i + 1) {
//             if let Some(intersection) = a.xy_intersect(b) {
//                 if range.contains(&intersection.x)
//                     && range.contains(&intersection.y)
//                     && a.is_in_future(intersection)
//                     && b.is_in_future(intersection)
//                 {
//                     intersections.push(intersection);
//                 }
//             }
//         }
//     }

//     intersections
// }

fn main() {
    // let (input, range) = (include_str!("../../test_input.txt"), 7.0..=27.0);
    let (input, range) = (
        include_str!("../../input.txt"),
        200000000000000.0..=400000000000000.0,
    );

    let hailstones: Vec<Hailstone> = input.lines().map(Hailstone::parse).collect();

    let part1 = find_xy_intersections(&hailstones, range.clone()).0.len();
    println!("part1 = {part1}");

    for hailstone in hailstones.iter() {
        // println!("# mx + b:");
        // println!("{}", hailstone.debug());
        // println!("# x & y => 0:");
        // println!("{}", hailstone.debug3());
        // println!("# with z:");
        // println!("{}", hailstone.debug4());
        // println!("# isolated variables:");
        // println!("{}", hailstone.debug5());
        // println!("# reduced scalars:");
        // println!("{}", hailstone.debug6());
        // println!();
    }

    // let offset = Velocity {
    //     x: -3.0,
    //     y: 1.0,
    //     z: 0.0,
    // };

    // let hailstones: Vec<Hailstone> = hailstones
    //     .iter()
    //     .copied()
    //     .map(|h| h.adjust_velocity(offset))
    //     .collect();

    // let intersections = find_xy_intersections(&hailstones, range.clone());
    // dbg!(intersections);

    let target = (hailstones.len() * (hailstones.len() - 1)) / 2;
    dbg!(target);
    let mut max_matches = 0;

    // for z in -10..10 {
    // for y in Spiral::default().take(100) {
    // for x in Spiral::default().take(100) {
    // for y in 0..50 {
    //     for x in 150..350 {
    //         let offset = Velocity {
    //             x: f64::from(x),
    //             y: f64::from(y),
    //             // z: f64::from(z),
    //             z: 0.0,
    //         };

    //         let hailstones: Vec<Hailstone> = hailstones
    //             .iter()
    //             .copied()
    //             .map(|h| h.adjust_velocity(offset))
    //             .collect();

    //         let a = hailstones.get(0).copied().unwrap();
    //         let b = hailstones.get(1).copied().unwrap();
    //         let Some(collision) = a.xy_intersect(&b) else {
    //             continue;
    //         };

    //         if range.contains(&collision.x)
    //             && range.contains(&collision.y)
    //             && a.is_in_future(collision)
    //             && b.is_in_future(collision) {
    //         } else {
    //             continue;
    //         }

    //         if !collision.is_integer() {
    //             continue;
    //         }
    //         println!("collision at {collision:?}");
    //         println!("maybe = ({y}, {x})");
    //         let collision = Hailstone {
    //             position: collision,
    //             velocity: Velocity {
    //                 y: 0.0,
    //                 x: 0.0,
    //                 z: 0.0,
    //             },
    //         };

    //         if hailstones.into_iter().all(|hailstone| {
    //             hailstone
    //                 .xy_intersect(&collision)
    //                 .is_some_and(|c| c.is_integer())
    //         }) {
    //             println!("answer = ({y}, {x})");
    //         }

    //         // solve system of equations for x & y
    //         // // let (intersections, parallel) = find_xy_intersections(&hailstones, range.clone());
    //         // if intersections.len() + parallel == target {
    //         //     println!("{offset:?} = {} ({parallel})", intersections.len());
    //         // }

    //         // // TODO: z?

    //         // if intersections.len() > max_matches {
    //         //     max_matches = intersections.len();
    //         //     println!(
    //         //         "{max_matches}: {offset:?} ({parallel})",
    //         //     );
    //         //     let mut t = tally(intersections).into_iter().collect::<Vec<_>>();
    //         //     t.sort_by_key(|(_, x)| *x);

    //         //     for t in t {
    //         //         println!("{t:?}");
    //         //     }
    //         //     std::process::exit(0);
    //         // for p in intersections {
    //         // println!("{p:?}");
    //         // std::thread::sleep(std::time::Duration::from_millis(1));
    //         // }
    //         // }
    //     }
    // }

    // for hailstone in hailstones.iter().skip(1).take(4) {
    for hailstone in hailstones.iter().take(4) {
        let Hailstone {
            position:
                Point {
                    z: hpz,
                    y: hpy,
                    x: hpx,
                },
            velocity:
                Velocity {
                    z: hvz,
                    y: hvy,
                    x: hvx,
                },
        } = hailstone;

        // println!("{hvx}y - {hpy}*{hvx} + {hpy}*vx - {hvy}x + {hpx}*{hvy} - {hpx}vy");
        // println!("{hvx}y - {hpy_hvx} + {hpy}*vx - {hvy}x + {hpx_hvy} - {hpx}vy", hpy_hvx = hpy * hvx, hpx_hvy = hpx * hvy);
        // with variable shorthands:
        // println!("{hvx}a - {hpy_hvx} + {hpy}b - {hvy}c + {hpx_hvy} - {hpx}d", hpy_hvx = hpy * hvx, hpx_hvy = hpx * hvy);
        // with constant term:
        println!("a*b - c*d = {hvx}a - {hpy_hvx} + {hpy}b - {hvy}c + {hpx_hvy} - {hpx}d", hpy_hvx = hpy * hvx, hpx_hvy = hpx * hvy);
        // a = y; b = vx; c = x; d = vy
        // And swapping in z for y:
        println!("a*b - c*d = {hvx}a - {hpz_hvx} + {hpz}b - {hvz}c + {hpx_hvz} - {hpx}d", hpz_hvx = hpz * hvx, hpx_hvz = hpx * hvz);
        // a = z; b = vx; c = x; d = vz

        // plug all that in to WolframAlpha and you get:
        // b = vx = 314
        // d1 = vy = 19
        // d2 = vz = 197
        //
        // c = x = 133619443970450
        // a1 = y = 263917577518425
        // a2 = z = 180640699244168

    }

    println!("part2 = {}", 133619443970450u64 + 263917577518425 + 180640699244168);

    // dbg!(max_matches);
    // }
}

/// An iterator that "spirals" out from the start
///
/// E.g., it yields 0, 1, -1, 2, -2, 3, -3, 4, etc.
#[derive(Debug, Copy, Clone, Default)]
struct Spiral {
    x: i32,
}

impl Spiral {
    fn new(start: i32) -> Self {
        Self { x: start }
    }
}

impl Iterator for Spiral {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.x;

        match self.x.signum() {
            -1 => self.x = -self.x + 1,
            0 => self.x += 1,
            1 => self.x = -self.x,
            _ => unreachable!(),
        };

        Some(ret)
    }
}

fn tally(v: Vec<Point>) -> HashMap<(i64, i64, i64), usize> {
    let mut map: HashMap<_, _> = [].into();

    for p in v {
        let item = (p.x as i64, p.y as i64, p.z as i64);
        map.entry(item).and_modify(|x| *x += 1).or_insert(1);
    }

    map
}

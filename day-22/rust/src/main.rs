use std::{
    collections::{HashMap, HashSet},
    ops::Range,
};

#[derive(Debug, Clone)]
struct Brick {
    x: Range<usize>,
    y: Range<usize>,
    z: Range<usize>,
}

impl Brick {
    fn parse(string: &str) -> Self {
        let (start, end) = string.split_once('~').unwrap();

        let mut pairs = start.split(',').zip(end.split(','));

        let x = pairs.next().unwrap();
        let y = pairs.next().unwrap();
        let z = pairs.next().unwrap();

        Self {
            x: pair_to_range(x),
            y: pair_to_range(y),
            z: pair_to_range(z),
        }
    }

    fn down(&self) -> Self {
        Self {
            x: self.x.clone(),
            y: self.y.clone(),
            z: range_down(&self.z),
        }
    }

    fn intersects(&self, other: &Self) -> bool {
        !(range_overlap(&self.x, &other.x).is_empty()
            || range_overlap(&self.y, &other.y).is_empty()
            || range_overlap(&self.z, &other.z).is_empty())
    }
}

fn pair_to_range((a, b): (&str, &str)) -> Range<usize> {
    a.parse().unwrap()..b.parse::<usize>().unwrap() + 1
}

fn range_down(range: &Range<usize>) -> Range<usize> {
    (range.start - 1)..(range.end - 1)
}

fn range_overlap(a: &Range<usize>, b: &Range<usize>) -> Range<usize> {
    a.start.max(b.start)..a.end.min(b.end)
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let mut bricks: Vec<Brick> = input.lines().map(Brick::parse).collect();
    bricks.sort_by_key(|brick| brick.z.start);
    // dbg!(&bricks);

    for i in 0..bricks.len() {
        loop {
            let this = &bricks[i];
            if this.z.contains(&1) {
                break;
            }

            let down = this.down();
            if bricks.iter().take(i).any(|brick| brick.intersects(&down)) {
                break;
            } else {
                bricks[i] = this.down();
            }
        }
    }

    // dbg!(&bricks);

    // Maps a brick to all the bricks supporting it
    let mut supports: HashMap<usize, Vec<usize>> = [].into();

    for i in (0..bricks.len()).rev() {
        let this = &bricks[i];
        let down = this.down();
        let entry = supports.entry(i).or_default();

        for (j, other) in bricks.iter().take(i).enumerate() {
            if down.intersects(other) {
                entry.push(j);
            }
        }
    }

    // dbg!(&supports);

    let cant_disintegrate: HashSet<usize> = supports
        .into_values()
        .filter(|v| v.len() == 1)
        .flatten()
        .collect();

    let part1 = bricks.len() - cant_disintegrate.len();
    println!("part1 = {part1}");
}

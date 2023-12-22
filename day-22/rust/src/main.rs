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

fn drop(bricks: &mut Vec<Brick>) -> usize {
    let mut which_bricks_moved = HashSet::new();

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
                which_bricks_moved.insert(i);
            }
        }
    }

    which_bricks_moved.len()
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let mut bricks: Vec<Brick> = input.lines().map(Brick::parse).collect();
    bricks.sort_by_key(|brick| brick.z.start);
    // dbg!(&bricks);

    drop(&mut bricks);

    let mut ugh = bricks.clone();
    ugh.sort_by_key(|brick| brick.z.start);
    for brick in ugh {
        println!("{brick:?}");
    }

    // Maps a brick to all the bricks supporting it
    let mut supports: HashMap<usize, HashSet<usize>> = [].into();

    for i in (0..bricks.len()).rev() {
        let this = &bricks[i];
        let down = this.down();
        let entry = supports.entry(i).or_default();

        for (j, other) in bricks.iter().take(i).enumerate() {
            if down.intersects(other) {
                entry.insert(j);
            }
        }
    }

    // dbg!(&supports);

    let cant_disintegrate: HashSet<usize> = supports
        .values()
        .filter(|v| v.len() == 1)
        .flatten()
        .copied()
        .collect();

    // Maps a brick to all the bricks that it supports
    // let mut supporting: HashMap<usize, Vec<usize>> = [].into();

    // for i in 0..bricks.len() {
    //     let this = &bricks[i];
    //     if this.z.contains(&1) {
    //         continue;
    //     }

    //     let down = this.down();
    //     let entry = supporting.entry(i).or_default();

    //     for (j, other) in bricks.iter().take(i).enumerate() {
    //         if down.intersects(other) {
    //             entry.push(j);
    //         }
    //     }
    // }

    let part1 = bricks.len() - cant_disintegrate.len();
    println!("part1 = {part1}");

    let mut total = 0;
    for candidate in cant_disintegrate {
        let mut bricks = bricks.clone();
        bricks.remove(candidate);
        total += drop(&mut bricks);
    }

    println!("total = {total}");

    // let mut total = 0;
    // for candidate in cant_disintegrate.iter() {
    //     // let this = &bricks[*candidate];
    //     let mut supports = supports.clone();
    //     let mut disintegrated: HashSet<usize> = [*candidate].into();

    //     for other in candidate + 1..bricks.len() {
    //         let supported_by = supports.get_mut(&other).unwrap();
    //         let diff = supported_by.difference(&disintegrated).copied();
    //         // *supported_by = diff.collect();

    //         // if supported_by.is_empty() {
    //         if diff.count() == 0 {
    //             // println!("dis {candidate} causes {other} to dis");
    //             disintegrated.insert(other);
    //         }
    //     }

    //     total += disintegrated.len() - 1;
    // }

    // 76039 = too high
    // println!("part2 = {total}");
}

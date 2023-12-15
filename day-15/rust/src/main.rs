#[derive(Debug, Copy, Clone)]
enum Step<'a> {
    Remove { label: &'a str },
    Set { label: &'a str, focal_length: u64 },
}

impl<'a> Step<'a> {
    fn parse(string: &'a str) -> Self {
        match string.split_once('=') {
            Some((label, focal_length)) => {
                let focal_length = focal_length.parse().unwrap();
                Self::Set {
                    label,
                    focal_length,
                }
            }
            None => match string.split_once('-') {
                Some((label, _)) => Self::Remove { label },
                None => unreachable!("bad input: {string}"),
            },
        }
    }
}

fn hash(string: &str) -> u64 {
    let mut start: u64 = 0;

    for byte in string.bytes() {
        start += byte as u64;
        start *= 17;
        start %= 256;
    }

    start
}

#[derive(Debug, Copy, Clone)]
struct BoxItem<'a> {
    label: &'a str,
    focal_length: u64,
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let part1: u64 = input.trim_end().split(',').map(hash).sum();
    println!("part1 = {part1}");

    let steps: Vec<Step> = input.trim_end().split(',').map(Step::parse).collect();
    let mut boxes: [Vec<BoxItem>; 256] = std::array::from_fn(|_| vec![]);

    for step in steps {
        match step {
            Step::Remove { label } => {
                let a_box = &mut boxes[hash(label) as usize];
                if let Some(pos) = a_box.iter().position(|item| item.label == label) {
                    a_box.remove(pos);
                }
            }
            Step::Set {
                label,
                focal_length,
            } => {
                let a_box = &mut boxes[hash(label) as usize];
                let item = BoxItem {
                    label,
                    focal_length,
                };
                match a_box.iter_mut().find(|item| item.label == label) {
                    Some(slot) => *slot = item,
                    None => a_box.push(item),
                }
            }
        }
    }

    let part2: u64 = boxes
        .into_iter()
        .enumerate()
        .map(|(box_number, a_box)| {
            a_box
                .into_iter()
                .enumerate()
                .map(|(slot, item)| (box_number as u64 + 1) * (slot as u64 + 1) * item.focal_length)
                .sum::<u64>()
        })
        .sum();
    println!("part2 = {part2}");
}

use std::collections::VecDeque;
use std::ops::Range;

#[derive(Debug)]
struct Mapping {
    source_start: u64,
    dest_start: u64,
    length: u64,
}

impl Mapping {
    fn source(&self) -> Range<u64> {
        self.source_start..(self.source_start + self.length)
    }

    fn lookup(&self, number: u64) -> u64 {
        number - self.source_start + self.dest_start
    }

    fn translate(&self, range: Range<u64>) -> Range<u64> {
        (range.start + self.dest_start - self.source_start)
            ..(range.end + self.dest_start - self.source_start)
    }
}

type Map = Vec<Mapping>;

fn lookup(number: u64, ranges: &[Mapping]) -> Option<u64> {
    ranges
        .iter()
        .find(|mapping| mapping.source().contains(&number))
        .map(|mapping| mapping.lookup(number))
}

fn parse_ranges(line: &str) -> Option<Mapping> {
    let (dest_range_start, line) = line.split_once(' ')?;
    let (source_range_start, range_length) = line.split_once(' ').unwrap();

    let dest_start = dest_range_start.parse().unwrap();
    let source_start = source_range_start.parse().unwrap();
    let length = range_length.parse().unwrap();

    Some(Mapping {
        source_start,
        dest_start,
        length,
    })
}

fn parse_map(string: &str) -> Map {
    string.lines().skip(1).map_while(parse_ranges).collect()
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let mut sections = input.split("\n\n");

    let seeds = sections.next().unwrap();
    let seeds: Vec<u64> = seeds[7..]
        .split(' ')
        .map(|num| num.parse().unwrap())
        .collect();

    let seed_ranges: Vec<Range<u64>> = seeds
        .chunks(2)
        .map(|window| window[0]..(window[0] + window[1]))
        .collect();

    let seed_to_soil = parse_map(sections.next().unwrap());
    let soil_to_fertilizer = parse_map(sections.next().unwrap());
    let fertilizer_to_water = parse_map(sections.next().unwrap());
    let water_to_light = parse_map(sections.next().unwrap());
    let light_to_temperature = parse_map(sections.next().unwrap());
    let temperature_to_humidity = parse_map(sections.next().unwrap());
    let humidity_to_location = parse_map(sections.next().unwrap());

    let part1 = seeds
        .into_iter()
        .map(|seed| {
            let soil = lookup(seed, &seed_to_soil).unwrap_or(seed);
            let fertilizer = lookup(soil, &soil_to_fertilizer).unwrap_or(soil);
            let water = lookup(fertilizer, &fertilizer_to_water).unwrap_or(fertilizer);
            let light = lookup(water, &water_to_light).unwrap_or(water);
            let temperature = lookup(light, &light_to_temperature).unwrap_or(light);
            let humidity = lookup(temperature, &temperature_to_humidity).unwrap_or(temperature);
            lookup(humidity, &humidity_to_location).unwrap_or(humidity)
        })
        .min()
        .unwrap();

    println!("part1 = {part1}");

    let soil = round(seed_ranges, &seed_to_soil);
    let fertilizer = round(soil, &soil_to_fertilizer);
    let water = round(fertilizer, &fertilizer_to_water);
    let light = round(water, &water_to_light);
    let temperature = round(light, &light_to_temperature);
    let humidity = round(temperature, &temperature_to_humidity);
    let location = round(humidity, &humidity_to_location);

    let part2 = location.into_iter().map(|range| range.start).min().unwrap();
    println!("part2 = {part2}");
}

fn round(coming_from: Vec<Range<u64>>, mappings: &[Mapping]) -> Vec<Range<u64>> {
    let mut done = vec![];
    let mut queue = VecDeque::from_iter(coming_from);

    'queue: while let Some(seed_range) = queue.pop_front() {
        for mapping in mappings {
            match smart_overlap(&seed_range, &mapping.source()) {
                OverlapResult::None => (),
                OverlapResult::Full(range) => {
                    done.push(mapping.translate(range));
                    continue 'queue;
                }
                OverlapResult::Partial { overlap, leftovers } => {
                    done.push(mapping.translate(overlap));
                    queue.push_back(leftovers);
                    continue 'queue;
                }
                OverlapResult::Split {
                    overlap,
                    left,
                    right,
                } => {
                    done.push(mapping.translate(overlap));
                    queue.extend([left, right]);
                    continue 'queue;
                }
            }
        }

        // None of the mappings overlapped so pass through unaltered.
        done.push(seed_range);
    }

    done
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum OverlapResult {
    None,
    Full(Range<u64>),
    Partial {
        overlap: Range<u64>,
        leftovers: Range<u64>,
    },
    Split {
        overlap: Range<u64>,
        left: Range<u64>,
        right: Range<u64>,
    },
}

// Figure out how much of a overlaps with b.
// If it's not fully contained, the leftovers (of a!) are returned.
fn smart_overlap(a: &Range<u64>, b: &Range<u64>) -> OverlapResult {
    use std::cmp::Ordering as O;
    match (
        a.start.cmp(&b.start),
        a.start.cmp(&b.end),
        a.end.cmp(&b.start),
        a.end.cmp(&b.end),
    ) {
        (_, _, O::Less | O::Equal, _) => OverlapResult::None,
        (_, O::Greater | O::Equal, _, _) => OverlapResult::None,
        (O::Equal | O::Greater, _, _, O::Equal | O::Less) => OverlapResult::Full(a.clone()),
        (O::Less, _, _, O::Greater) => OverlapResult::Split {
            overlap: b.clone(),
            left: a.start..b.start,
            right: b.end..a.end,
        },
        (O::Less, _, O::Greater, _) => OverlapResult::Partial {
            overlap: b.start..a.end,
            leftovers: a.start..b.start,
        },
        (O::Equal, _, _, _) => OverlapResult::Partial {
            overlap: a.start..b.end,
            leftovers: b.end..a.end,
        },
        (_, O::Less, _, _) => OverlapResult::Partial {
            overlap: a.start..b.end,
            leftovers: b.end..a.end,
        },
    }
}

#[cfg(test)]
mod tests;

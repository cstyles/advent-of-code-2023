fn main() {
    // let input = include_str!("../../input.txt");
    // let times = [41, 66, 72, 66];
    // let distances = [244, 1047, 1228, 1040];

    let times = [7, 15, 30];
    let distances = [9, 40, 200];

    let part1: usize = times
        .into_iter()
        .zip(distances)
        .map(|(round_length, record)| attempt(round_length, record))
        .product();
    println!("part1 = {part1}");

    // println!("part2 = {}", attempt(71530, 940200));
    println!("part2 = {}", attempt(41667266, 244104712281040));
}

fn attempt(round_length: u64, record: u64) -> usize {
    (1..round_length)
        .map(|speed| speed * (round_length - speed))
        .filter(|distance| *distance > record)
        .count()
}

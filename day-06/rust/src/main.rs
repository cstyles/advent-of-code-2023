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

    println!("part2 = {}", quadratic_formula(41667266.0, 244104712281040.0));
}

fn attempt(round_length: u64, record: u64) -> usize {
    (1..round_length)
        .map(|speed| speed * (round_length - speed))
        .filter(|distance| *distance > record)
        .count()
}

// record = time_held * (round_length - time_held)
// record = time_held * round_length - time_held^2
// 0 = time_held * round_length - time_held^2 - record
// 0 = time_held^2 - time_held * round_length + record
//
// a = 1, b = -round_length, c = record
// x = (round_length ± sqrt(round_length^2 - 4 * record)) / 2
fn quadratic_formula(round_length: f64, record: f64) -> f64 {
    let discriminant = round_length.powf(2.0) - 4.0 * record;
    let upper = round_length + discriminant.sqrt() / 2.0;
    let lower = round_length - discriminant.sqrt() / 2.0;

    upper.floor() - lower.floor()
}

fn hash(string: &str) -> u64 {
    let mut start: u64 = 0;

    for byte in string.bytes() {
        start += byte as u64;
        start *= 17;
        start %= 256;
    }

    start
}

fn main() {
    // let input = include_str!("../../test_input.txt");
    let input = include_str!("../../input.txt");

    let part1: u64 = input.trim_end().split(',').map(hash).sum();
    println!("part1 = {part1}");
}

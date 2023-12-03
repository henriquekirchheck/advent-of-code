fn main() {
    let input = include_str!(concat!(env!("OUT_DIR"), "/input.txt"));
    let output = day::part1::process(input);
    println!("{}", output);
}

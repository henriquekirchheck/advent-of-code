fn main() {
    let input = include_str!(concat!(env!("OUT_DIR"), "/input.txt"));
    let output = day::part2::process(input);
    println!("{}", output);
}

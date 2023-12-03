use day::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn part1() {
    part1::process(divan::black_box(include_str!(concat!(
        env!("OUT_DIR"),
        "/input.txt"
    ))));
}

#[divan::bench]
fn part2() {
    part2::process(divan::black_box(include_str!(concat!(
        env!("OUT_DIR"),
        "/input.txt"
    ))));
}

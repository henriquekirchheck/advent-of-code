#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    let input = include_str!(concat!(env!("OUT_DIR"), "/input.txt"));
    let output = day::part1::process(input);
    println!("{}", output);
}

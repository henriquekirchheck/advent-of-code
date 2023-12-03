use reqwest::header::COOKIE;
use std::{env, error::Error, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let client = reqwest::blocking::Client::new();
    let cookie = env::var("AOC_COOKIE").expect(
        "AOC_COOKIE environment variable needs to be set with the advent of code auth cookie",
    );
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let pwd = env::var("PWD").unwrap();
    let (year, day) = pwd
        .split_once("advent-of-code/")
        .unwrap()
        .1
        .split_once("/")
        .unwrap();

    let input_path = Path::new(&out_dir).join("input.txt");
    let input = client
        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
        .header(COOKIE, format!("session={cookie}"))
        .send()?
        .text()?;
    fs::write(&input_path, input)?;

    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}

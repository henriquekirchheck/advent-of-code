fn main() {
    let input = include_str!(concat!(env!("OUT_DIR"), "/input.txt"));
    let output = process(input);
    println!("{}", output);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let expected = "281";

        assert_eq!(process(input), expected.to_owned());
    }
}

fn process(input: &str) -> String {
    day::parser2(input).to_string()
}

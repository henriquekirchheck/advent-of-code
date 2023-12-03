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

pub fn process(input: &str) -> String {
    crate::parser::parser2(input).to_string()
}

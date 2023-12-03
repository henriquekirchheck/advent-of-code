#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let expected = "142";

        assert_eq!(process(input), expected.to_owned());
    }
}

pub fn process(input: &str) -> String {
    crate::parser::parser1(input).to_string()
}

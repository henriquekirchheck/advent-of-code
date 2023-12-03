#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "";
        let expected = "";

        assert_eq!(process(input), expected.to_owned());
    }
}

pub fn process(input: &str) -> String {
    input.to_owned()
}

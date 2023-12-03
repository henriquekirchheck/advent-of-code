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
        let input = "";
        let expected = "";

        assert_eq!(process(input), expected.to_owned());
    }
}

fn process(input: &str) -> String {
    input.to_owned()
}

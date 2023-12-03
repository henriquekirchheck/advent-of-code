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
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        let expected = "142";

        assert_eq!(process(input), expected.to_owned());
    }
}

fn process(input: &str) -> String {
    day::parser1(input).sum::<usize>().to_string()
}

use nom::{
    character::complete::{alpha0, line_ending, satisfy},
    multi::many1,
    sequence::{preceded, terminated},
    IResult,
};

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
treb7uchet
";
        let expected = "142";

        assert_eq!(process(input), expected.to_owned());
    }
}

fn main_parser(input: &str) -> IResult<&str, Vec<char>> {
    many1(preceded(
        alpha0,
        terminated(satisfy(|c| c.is_ascii_digit()), alpha0),
    ))(input)
}

fn mult_parser(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    many1(terminated(main_parser, line_ending))(input)
}

fn process(input: &str) -> String {
    mult_parser(input)
        .unwrap()
        .1
        .iter()
        .map(|ind| {
            let first = ind.iter().next().unwrap();
            let last = ind.iter().last().unwrap();
            format!("{first}{last}").parse::<usize>().unwrap()
        })
        .sum::<usize>()
        .to_string()
}

use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::line_ending,
    combinator::map,
    multi::many1,
    sequence::terminated,
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
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";
        let expected = "281";

        assert_eq!(process(input), expected.to_owned());
    }
}

fn number_parser(input: &str) -> IResult<&str, u8> {
    map(
        alt((
            tag("one"),
            tag("two"),
            tag("three"),
            tag("four"),
            tag("five"),
            tag("six"),
            tag("seven"),
            tag("eight"),
            tag("nine"),
            tag("1"),
            tag("2"),
            tag("3"),
            tag("4"),
            tag("5"),
            tag("6"),
            tag("7"),
            tag("8"),
            tag("9"),
        )),
        |res| match res {
            "one" => 1,
            "two" => 2,
            "three" => 3,
            "four" => 4,
            "five" => 5,
            "six" => 6,
            "seven" => 7,
            "eight" => 8,
            "nine" => 9,
            char if char
                .chars()
                .next()
                .map(|c| c.is_ascii_digit())
                .unwrap_or(false) =>
            {
                char.parse().unwrap()
            }
            _ => unreachable!("Parser should give only correct input"),
        },
    )(input)
}

fn main_parser(input: &str) -> IResult<&str, Vec<u8>> {
    let (rest, input) = take_until("\n")(input)?;

    Ok((
        rest,
        (0..input.len())
            .flat_map(|start| {
                let input = &input[start..];
                number_parser(input).map(|res| res.1)
            })
            .collect::<Vec<u8>>(),
    ))
}

fn mult_parser(input: &str) -> IResult<&str, Vec<Vec<u8>>> {
    many1(terminated(main_parser, line_ending))(input)
}

fn process(input: &str) -> String {
    dbg!(mult_parser(input))
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

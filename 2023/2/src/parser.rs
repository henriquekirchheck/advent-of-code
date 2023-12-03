use std::cmp::Ordering;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{satisfy, u8},
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult, Parser,
};

pub const MAX_GAME: Game = Game {
    red: 12,
    green: 13,
    blue: 14,
};

#[derive(Debug, PartialEq)]
pub struct Game {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl PartialOrd for Game {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let rgb = [
            self.red.partial_cmp(&other.red),
            self.green.partial_cmp(&other.green),
            self.blue.partial_cmp(&other.blue),
        ];
        let mut rgb_iter = rgb.iter().map(|color| color.unwrap());
        if rgb_iter
            .clone()
            .all(|cpm| cpm == Ordering::Less || cpm == Ordering::Equal)
        {
            Some(Ordering::Less)
        } else if rgb_iter.clone().all(|cpm| cpm == Ordering::Equal) {
            Some(Ordering::Equal)
        } else if rgb_iter.any(|cpm| cpm == Ordering::Greater) {
            Some(Ordering::Greater)
        } else {
            unreachable!("Should not work get here because all branches are accounted for")
        }
    }
}

fn from_parser_output(value: Vec<Vec<(u8, &str)>>) -> Game {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for group in value {
        for (num, color) in group {
            match color {
                "red" if num > red => red = num,
                "green" if num > green => green = num,
                "blue" if num > blue => blue = num,
                _ => {}
            }
        }
    }

    Game { red, green, blue }
}

pub fn parser(input: &str) -> (u8, Game) {
    let res: IResult<&str, _> = tuple((
        preceded(tag("Game "), terminated(u8, tag(": "))),
        separated_list1(
            tag("; "),
            separated_list1(
                tag(", "),
                separated_pair(
                    u8,
                    satisfy(|c| c.is_whitespace()),
                    alt((tag("red"), tag("green"), tag("blue"))),
                ),
            ),
        )
        .map(from_parser_output),
    ))(input);

    res.unwrap().1
}

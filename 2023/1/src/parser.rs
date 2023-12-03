use rayon::prelude::*;
use std::ops::RangeFrom;

type Lookup = &'static [(&'static str, u8)];

const LOOKUP: Lookup = &[
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
];

fn number_parser(input: &str, lookup: Lookup) -> Option<&u8> {
    lookup
        .iter()
        .flat_map(|(slice, num)| {
            input
                .get(..slice.len())
                .and_then(|input| match input == *slice {
                    true => Some(num),
                    false => None,
                })
        })
        .next()
}

fn line_parser(input: &str, lookup_range: RangeFrom<usize>) -> impl Iterator<Item = &u8> {
    (0..input.len())
        .flat_map(move |start| number_parser(&input[start..], &LOOKUP[lookup_range.clone()]))
}

fn parser_base(input: &str, named_numbers: bool) -> usize {
    input
        .lines()
        .par_bridge()
        .map(move |line| line_parser(line, (!named_numbers as usize * 9)..))
        .map(|mut digits| {
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            (first * 10 + last) as usize
        })
        .sum()
}

pub fn parser1(input: &str) -> usize {
    parser_base(input, false)
}

pub fn parser2(input: &str) -> usize {
    parser_base(input, true)
}

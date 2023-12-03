use crate::parser;
use rayon::iter::{ParallelBridge, ParallelIterator};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let expected = "2286";

        assert_eq!(process(input), expected.to_owned());
    }
}

pub fn process(input: &str) -> String {
    input
        .lines()
        .par_bridge()
        .map(parser::parser)
        .map(|(_, game)| game.red as usize * game.blue as usize * game.green as usize)
        .sum::<usize>()
        .to_string()
}

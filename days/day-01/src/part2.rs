use winnow::ascii::{dec_int, multispace0};
use winnow::combinator::separated_pair;
use winnow::prelude::*;

use crate::custom_error::AocError;

fn parse_line(input: &mut &str) -> PResult<(i32, i32)> {
    separated_pair(dec_int, multispace0, dec_int).parse_next(input)
}
pub fn solve(input: &str) -> miette::Result<String, AocError> {
    let parsed: Vec<(i32, i32)> = input
        .trim()
        .lines()
        .map(|line| parse_line.parse(line).map_err(|e| e.to_string()).unwrap())
        .collect();
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for (id1, id2) in parsed {
        list1.push(id1);
        list2.push(id2);
    }

    let result = list1
        .iter()
        .map(|id1| id1 * list2.iter().filter(|id2| id1 == id2.to_owned()).count() as i32)
        .fold(0, |acc, x| acc + x)
        .to_string();
    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn solve_example() -> miette::Result<()> {
        let input = include_str!("../example.txt");
        assert_eq!("31", solve(input)?);
        Ok(())
    }
}

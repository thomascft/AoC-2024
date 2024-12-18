use std::str::FromStr;

use crate::custom_error::AocError;
use winnow::ascii::{dec_int, space0};
use winnow::combinator::{repeat, terminated};
use winnow::{seq, Parser};

#[derive(Debug, PartialEq)]
struct Report {
    levels: Vec<i32>,
}

impl std::str::FromStr for Report {
    type Err = AocError;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        parse_report.parse(input).map_err(|e| e.into())
    }
}

fn parse_report(input: &mut &str) -> winnow::PResult<Report> {
    seq!(Report {
        levels: repeat(0.., terminated(dec_int::<_, i32, _>, space0))
    })
    .parse_next(input)
}

impl Report {
    fn is_safe(&self) -> bool {
        let mut safe = true;
        self.levels
            .iter()
            .zip(self.levels.iter().skip(1))
            .for_each(|(c, n)| {
                let n = n.to_owned();
                let diff = c.abs_diff(n);
                if diff > 3 || diff < 1 {
                    safe = false;
                }
            });
        if !self.levels.iter().is_sorted() && !self.levels.iter().rev().is_sorted() {
            safe = false;
        }
        safe
    }
}

pub fn solve(input: &str) -> miette::Result<String, AocError> {
    let mut total_safe = 0;
    for line in input.lines() {
        let report = Report::from_str(line)?;
        if report.is_safe() {
            total_safe += 1;
        }
    }

    Ok(total_safe.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_parse() -> miette::Result<()> {
        let input = "5 4 3 2 1";
        let expected = Report {
            levels: vec![5, 4, 3, 2, 1],
        };
        assert_eq!(expected, Report::from_str(input)?);
        Ok(())
    }

    #[test]
    fn solve_example() -> miette::Result<()> {
        let input = include_str!("../example.txt");
        assert_eq!("2", solve(input)?);
        Ok(())
    }
}

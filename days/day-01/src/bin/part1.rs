use day_01::{custom_error::AocError, part1::solve};

fn main() -> miette::Result<(), AocError> {
    let input = include_str!("../../input.txt");
    let result = solve(input)?;
    println!("Day1 Part1: {}", result);
    Ok(())
}

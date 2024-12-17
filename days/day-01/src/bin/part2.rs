use day_01::custom_error::AocError;
use day_01::part2::solve;

fn main() -> miette::Result<(), AocError> {
    let input = include_str!("../../input.txt");
    let result = solve(input)?;
    println!("Day 01 part2 solution: {}", result);
    Ok(())
}

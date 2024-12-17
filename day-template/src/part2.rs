use crate::custom_error::AocError;
pub fn solve(input: &str) -> miette::Result<String, AocError> {
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_example() -> miette::Result<()> {
        todo!();
        let input = include_str!("../example.txt");
        assert_eq!("", solve(input)?);
        Ok(())
    }
}

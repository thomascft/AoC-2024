use {{crate_name}}::part2::solve;
use miette::Context;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let input = include_str!("../../input.txt");
    let result = solve(input).context("Solve part2")?;
    println!("{{project-name}} part2 solution: {}", result);
    Ok(())
}

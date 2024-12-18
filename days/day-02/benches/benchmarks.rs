use day_02::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    todo!();
    part1::solve(divan::black_box(include_str!("../input.txt"))).unwrap();
}
#[divan::bench]
fn part2() {
    todo!();
    part2::solve(divan::black_box(include_str!("../input.txt"))).unwrap();
}

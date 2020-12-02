use anyhow::Result;
use aoc_next::{aoc_main, Aoc, solver, failable_parser, solution};

use aoc2020::{day1, day2};

const AOC: Aoc = Aoc {
    allow_download: true,
    year: 2020,
    solutions: &[
        solution!{1, failable_parser!{ day1::input_generator }, solver!{ day1::solve_part1 }},
        solution!{1, failable_parser!{ day1::input_generator }, solver!{ day1::solve_part2 }},
        solution!{2, failable_parser!{ day2::input_generator }, solver!{ day2::solve_part1 }},
        solution!{2, failable_parser!{ day2::input_generator }, solver!{ day2::solve_part2 }},
    ],
};

pub fn main() -> Result<()> {
    aoc_main(AOC)
}

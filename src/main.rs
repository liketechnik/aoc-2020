// SPDX-FileCopyrightText: 2020 Florian Warzecha <liketechnik@disroot.org>
//
// SPDX-License-Identifier: MPL-2.0

use anyhow::Result;
use aoc_next::{aoc_main, failable_parser, parser, solution, solver, Aoc};

use aoc2020::{day1, day2, day3, day4, day4_beautified, day5};

const AOC: Aoc = Aoc {
    allow_download: true,
    year: 2020,
    solutions: &[
        solution! {1, failable_parser!{ day1::input_generator }, solver!{ day1::solve_part1 }},
        solution! {1, failable_parser!{ day1::input_generator }, solver!{ day1::solve_part2 }},
        solution! {2, failable_parser!{ day2::input_generator }, solver!{ day2::solve_part1 }},
        solution! {2, failable_parser!{ day2::input_generator }, solver!{ day2::solve_part2 }},
        solution! {3, failable_parser!{ day3::input_generator }, solver!{ day3::solve_part1 }},
        solution! {3, failable_parser!{ day3::input_generator }, solver!{ day3::solve_part2 }},
        solution! {3, failable_parser!{ day3::input_generator }, solver!{ day3::solve_part1_iter_map }},
        solution! {3, failable_parser!{ day3::input_generator }, solver!{ day3::solve_part2_iter_map }},
        solution! {4, parser!{ day4::input_generator1 }, solver!{ day4::solve_part1 }},
        solution! {4, parser!{ day4::input_generator }, solver!{ day4::solve_part2 }},
        solution! {4, parser!{ day4_beautified::input_generator }, solver!{ day4_beautified::solve_part1 }},
        solution! {4, parser!{ day4_beautified::input_generator }, solver!{ day4_beautified::solve_part2 }},
        solution! {5, parser!{ day5::input_generator }, solver!{ day5::solve_part1 }},
        solution! {5, parser!{ day5::input_generator }, solver!{ day5::solve_part2 }},
    ],
};

pub fn main() -> Result<()> {
    aoc_main(AOC)
}

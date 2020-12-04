// SPDX-FileCopyrightText: 2020 Florian Warzecha <liketechnik@disroot.org>
//
// SPDX-License-Identifier: MPL-2.0

use anyhow::{anyhow, Result};

#[derive(PartialEq)]
pub enum Location {
    Tree,
    Open,
}

impl Location {
    fn new(c: char) -> Result<Location> {
        match c {
            '.' => Ok(Location::Open),
            '#' => Ok(Location::Tree),
            _ => Err(anyhow!("'{}' is not a valid location marker", c)),
        }
    }
}

type Position = (usize, usize);

pub struct Map {
    map: Vec<Vec<Location>>,
}

impl Map {
    fn new(lines: &str) -> Result<Map> {
        let map: Vec<Vec<Location>> = lines
            .lines()
            .map(|line| {
                line.chars()
                    .map(Location::new)
                    .collect::<Result<Vec<Location>>>()
            })
            .collect::<Result<Vec<Vec<Location>>>>()?;

        Ok(Map { map })
    }

    fn get_pos(&self, pos: Position) -> Option<&Location> {
        let row = self.map.get(pos.0)?; // get the specified row (has a limit)
        row.iter().cycle().nth(pos.1) // get the column (has no limit, e. g. cycle())
    }

    fn trees(&self, steps: Position) -> usize {
        self.map
            .iter()
            .step_by(steps.0)
            .zip((0..).step_by(steps.1))
            .map(|(row, col_nr)| row.iter().cycle().nth(col_nr))
            .filter_map(|loc| {
                if loc? == &Location::Tree {
                    Some(())
                } else {
                    None
                }
            })
            .count()
    }
}

pub fn input_generator(input: &str) -> Result<Map> {
    Map::new(input)
}

pub fn solve_part1(input: Map) -> usize {
    trees(&input, (1, 3))
}

pub fn trees(input: &Map, steps: Position) -> usize {
    (0..)
        .step_by(steps.0)
        .zip((0..).step_by(steps.1))
        .map(|pos| input.get_pos(pos))
        .take_while(|x| x.is_some())
        .filter_map(|loc| {
            if loc? == &Location::Tree {
                Some(())
            } else {
                None
            }
        })
        .count()
}

pub fn solve_part2(input: Map) -> usize {
    trees(&input, (1, 1))
        * trees(&input, (1, 3))
        * trees(&input, (1, 5))
        * trees(&input, (1, 7))
        * trees(&input, (2, 1))
}

pub fn solve_part1_iter_map(input: Map) -> usize {
    input.trees((1, 3))
}

pub fn solve_part2_iter_map(input: Map) -> usize {
    input.trees((1, 1))
        * input.trees((1, 3))
        * input.trees((1, 5))
        * input.trees((1, 7))
        * input.trees((2, 1))
}

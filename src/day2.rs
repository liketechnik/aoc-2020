// SPDX-FileCopyrightText: 2020 Florian Warzecha <liketechnik@disroot.org>
//
// SPDX-License-Identifier: MPL-2.0

use anyhow::{anyhow, Result};
use aoc_runner_derive::{aoc, aoc_generator};

pub struct Rule {
    about: char,
    min: usize,
    max: usize,
}

pub enum RuleType {
    SledRentalPlace,
    TobogganCorporatePolicy,
}

pub struct Password {
    content: String,
    rule: Rule,
}

impl Password {
    fn new(line: &str) -> Result<Password> {
        let values: Vec<&str> = line.splitn(4, |s| s == '-' || s == ' ' || s == ':' ).collect();

        if values.len() != 4 {
            return Err(anyhow!("Expected 4 parts, have: {:?}", values));
        }

        let min = values[0].parse()?;
        let max = values[1].parse()?;
        let about = values[2].chars().next().unwrap();
        let pw = values[3].chars().skip(1).collect();

        let rule = Rule { about, min, max };
        Ok(Password { rule, content: pw })
    }

    fn is_valid(&self, rule: RuleType) -> bool {
        match rule {
            RuleType::SledRentalPlace => {
                let actual_count = self.content.chars().filter(|c| c == &self.rule.about).count();
                actual_count <= self.rule.max && actual_count >= self.rule.min
            }
            RuleType::TobogganCorporatePolicy => {
                let count_at_pos = self.content.chars().enumerate().filter(|(i, c)| c == &self.rule.about && (i + 1 == self.rule.max || i + 1 == self.rule.min)).count();
                count_at_pos == 1
            }
        }
    }
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Result<Vec<Password>> {
    input.lines().map(Password::new).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[Password]) -> usize {
    input.iter().filter(|p| p.is_valid(RuleType::SledRentalPlace)).count()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[Password]) -> usize {
    input.iter().filter(|p| p.is_valid(RuleType::TobogganCorporatePolicy)).count()
}

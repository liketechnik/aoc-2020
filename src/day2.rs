// SPDX-FileCopyrightText: 2020 Florian Warzecha <liketechnik@disroot.org>
//
// SPDX-License-Identifier: MPL-2.0

use anyhow::Result;

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
        let mut values = line.splitn(4, |s| s == '-' || s == ' ' || s == ':');

        let min = values.next().unwrap().parse()?;
        let max = values.next().unwrap().parse()?;
        let about = values.next().unwrap().chars().next().unwrap();
        let pw = values.next().unwrap().chars().skip(1).collect();

        let rule = Rule { about, min, max };
        Ok(Password { rule, content: pw })
    }

    fn is_valid(&self, rule: RuleType) -> bool {
        match rule {
            RuleType::SledRentalPlace => {
                let actual_count = self
                    .content
                    .chars()
                    .try_fold(0, |acc, c| {
                        let mut acc = acc;
                        if c == self.rule.about {
                            acc += 1
                        }

                        if acc > self.rule.max {
                            Err(acc)
                        } else {
                            Ok(acc)
                        }
                    })
                    .unwrap_or(self.rule.max);
                actual_count < self.rule.min
            }
            RuleType::TobogganCorporatePolicy => {
                self.content
                    .chars()
                    .enumerate()
                    .try_fold(0, |acc, (i, c)| {
                        let mut acc = acc;
                        if (i + 1 == self.rule.min || i + 1 == self.rule.max)
                            && c == self.rule.about
                        {
                            acc += 1
                        }

                        if acc > 1 {
                            Err(acc)
                        } else {
                            Ok(acc)
                        }
                    })
                    .unwrap_or(0)
                    == 1
            }
        }
    }
}

pub fn input_generator(input: &str) -> Result<Vec<Password>> {
    input.lines().map(Password::new).collect()
}

pub fn solve_part1(input: Vec<Password>) -> usize {
    input
        .iter()
        .filter(|p| p.is_valid(RuleType::SledRentalPlace))
        .count()
}

pub fn solve_part2(input: Vec<Password>) -> usize {
    input
        .iter()
        .filter(|p| p.is_valid(RuleType::TobogganCorporatePolicy))
        .count()
}

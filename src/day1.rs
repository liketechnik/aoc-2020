// SPDX-FileCopyrightText: 2020 Florian Warzecha <liketechnik@disroot.org>
//
// SPDX-License-Identifier: MPL-2.0

use anyhow::{anyhow, Result};

pub fn input_generator(input: &str) -> Result<Vec<usize>> {
    input
        .lines()
        .map(|line| {
            line.parse()
                .map_err(|e| anyhow!("Failed to convert '{}': {:?}", line, e))
        })
        .collect()
}

pub fn solve_part1(input: Vec<usize>) -> String {
    input
        .iter()
        .enumerate()
        .flat_map(|input1| {
            input
                .split_at(input1.0)
                .1
                .iter()
                .map(move |input2| (input1.1, input2))
        })
        .filter_map(|input| {
            if input.0 + input.1 == 2020 {
                Some(format!(
                    "Found {} + {} = {}, product is {}",
                    input.0,
                    input.1,
                    input.0 + input.1,
                    input.0 * input.1
                ))
            } else {
                None
            }
        })
        .collect()
}

pub fn solve_part2(input: Vec<usize>) -> String {
    input
        .iter()
        .enumerate()
        .flat_map(|input1| {
            ((input1.0 + 1)..)
                .zip(input.split_at(input1.0).1.iter())
                .map(move |input2| (input1.1, input2))
        })
        .flat_map(|input12| {
            input
                .split_at(input12.1 .0)
                .1
                .iter()
                .map(move |input3| (input12.0, input12.1 .1, input3))
        })
        .filter_map(|input123| {
            if input123.0 + input123.1 + input123.2 == 2020 {
                Some(format!(
                    "Found {} + {} + {} = {}, product is {}",
                    input123.0,
                    input123.1,
                    input123.2,
                    input123.0 + input123.1 + input123.2,
                    input123.0 * input123.1 * input123.2
                ))
            } else {
                None
            }
        })
        .collect()
}

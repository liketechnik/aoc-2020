// SPDX-FileCopyrightText: 2020 Florian Warzecha <liketechnik@disroot.org>
//
// SPDX-License-Identifier: MPL-2.0

use anyhow::{anyhow, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    let expenses: Vec<usize> = include_str!("../input").lines().map(|line| line.parse::<usize>().map_err(|e| anyhow!("Failed to parse '{}': {:?}", line, e))).collect::<Result<Vec<usize>>>()?;

    Ok(expenses
        .iter()
        .enumerate()
        // create unique tuples of expense to expense to expense mapping
        .flat_map(|input1| {
            ((input1.0 + 1)..) // enumerate would start at 0, but we need the 'real' index below
                .zip(
                    expenses
                        // skip the first 'n' elements, because they were already included before
                        // (if the index of input1's iteration is 5, we already have a all possible
                        // mappings for index 1, because addition doesn't care about the order of operators)
                        .split_at(input1.0)
                        .1
                        .iter(),
                )
                .map(move |input2| (input1.1, input2))
        })
        .flat_map(|input12| {
            expenses
                .split_at(input12.1 .0)
                .1
                .iter()
                .map(move |input3| (input12.0, input12.1 .1, input3))
        })
        // filter out mappings of exactly the same value (determined by index)
        // and keep only the ones which sum to 2020
        .filter_map(|input| {
            if input.0 + input.1 + input.2 == 2020 {
                Some((input.0, input.1, Some(input.2)))
            } else if input.0 + input.1 == 2020 {
                Some((input.0, input.1, None))    
            } else {
                None
            }
        })
        .unique()
        .map(|sum| {
            println!(
                "Found {} + {} + {:?} = {}, product is {}",
                sum.0,
                sum.1,
                sum.2,
                sum.0 + sum.1 + sum.2.unwrap_or(&0usize),
                sum.0 * sum.1 * sum.2.unwrap_or(&1usize)
            );
        }).collect())
}

// SPDX-FileCopyrightText: 2020 Florian Warzecha <liketechnik@disroot.org>
//
// SPDX-License-Identifier: MPL-2.0

use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<()> {
    let f = File::open("input").map_err(|e| anyhow!("Failed to open 'input': {:?}", e))?;
    let f = BufReader::new(f);

    let expenses: Result<Vec<usize>> = f.lines().map(parse_line).collect();
    let expenses = expenses?;

    let sums = expenses
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
                .map(move |input2| (input1, input2))
        })
        .flat_map(|input12| {
            expenses
                .split_at(std::cmp::max(input12.0 .0, input12.1 .0))
                .1
                .iter()
                .map(move |input3| (input12.0 .1, input12.1 .1, input3))
        })
        // filter out mappings of exactly the same value (determined by index)
        // and keep only the ones which sum to 2020
        .filter_map(|input| {
            if input.0 + input.1 + input.2 == 2020 {
                Some((input.0, input.1, input.2))
            } else {
                None
            }
        });

    for sum in sums {
        println!(
            "Found {} + {} + {} = {}, product is {}",
            sum.0,
            sum.1,
            sum.2,
            sum.0 + sum.1 + sum.2,
            sum.0 * sum.1 * sum.2
        );
    }

    Ok(())
}

fn parse_line(line: Result<String, std::io::Error>) -> Result<usize> {
    let line = line?;
    line.parse()
        .map_err(|e| anyhow!("Failed to parse {} into usize: {:?}", line, e))
}

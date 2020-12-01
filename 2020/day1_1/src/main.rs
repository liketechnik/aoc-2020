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
        // create unique tuples of expense to expense mapping
        .map(|input1| {
            expenses
                // skip the first 'n' elements, because they were already included before
                // (if the index of input1's iteration is 5, we already have a all possible
                // mappings for index 1, because addition doesn't care about the order of operators)
                .split_at(input1.0)
                .1
                .iter()
                .map(move |input2| (input1.1, input2))
        })
        .flatten()
        .filter_map(|input| {
            if input.0 + input.1 == 2020 {
                Some((input.0, input.1))
            } else {
                None
            }
        });

    for sum in sums {
        println!(
            "Found {} + {} = {}, product is {}",
            sum.0,
            sum.1,
            sum.0 + sum.1,
            sum.0 * sum.1
        );
    }

    Ok(())
}

fn parse_line(line: Result<String, std::io::Error>) -> Result<usize> {
    let line = line?;
    line.parse()
        .map_err(|e| anyhow!("Failed to parse {} into usize: {:?}", line, e))
}

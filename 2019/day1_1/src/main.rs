// SPDX-FileCopyrightText: 2020 Florian Warzecha <liketechnik@disroot.org>
//
// SPDX-License-Identifier: MPL-2.0

use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use itertools::process_results;

use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let f = File::open("input").map_err(|e| anyhow!("could not open 'input': {:?}", e))?;
    let f = BufReader::new(f);

    let sum = process_results(f.lines().map(parse_line), |iter| iter.map(|mass| mass / 3 - 2).try_fold(0u64, |sum, requirement| sum.checked_add(requirement)))?;
    println!("Sum of fuel requirements: {}", sum.ok_or_else(|| anyhow!("Overflow!"))?);

    Ok(())
}

fn parse_line(line: Result<String, std::io::Error>) -> Result<u64> {
    let line = line?;
    line.parse().map_err(|e| anyhow!("Failed to parse '{}' into u64: {:?}", line, e))
}


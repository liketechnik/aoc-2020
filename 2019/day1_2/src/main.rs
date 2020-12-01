// SPDX-FileCopyrightText: 2020 Florian Warzecha <liketechnik@disroot.org>
//
// SPDX-License-Identifier: MPL-2.0

use std::convert::TryInto;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;
use itertools::process_results;

use anyhow::{Result, anyhow};

fn main() -> Result<()> {
    let f = File::open("input").map_err(|e| anyhow!("could not open 'input': {:?}", e))?;
    let f = BufReader::new(f);

    let sum = process_results(f.lines().map(parse_line), |iter| iter
        .map(|mass| mass / 3 - 2)
        .try_fold(0i128, |sum, requirement| {
            let fuel = fuel_requirements(requirement).ok()?;
            let with_fuel = requirement.checked_add(fuel).ok_or_else(|| anyhow!("Overflow")).ok()?;
            sum.checked_add(with_fuel)
        })
    )?.ok_or_else(|| anyhow!("Overflow!"))?;
    println!("Sum of fuel requirements: {}", sum);

    Ok(())
}

fn parse_line(line: Result<String, std::io::Error>) -> Result<i128> {
    let line = line?;
    line.parse().map_err(|e| anyhow!("Failed to parse '{}' into u64: {:?}", line, e))
}

fn fuel_requirements(fuel_mass: i128) -> Result<i128> {
    let mut required = fuel_mass / 3 - 2;

    if required > 0 {
        let fuel_requirement = fuel_requirements(required)?;
        required = required.checked_add(fuel_requirement).ok_or_else(|| anyhow!("overflow!"))?;
    } else {
        required = 0;
    }

    return Ok(required);
}

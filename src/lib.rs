// SPDX-FileCopyrightText: 2020 Florian Warzecha <liketechnik@disroot.org>
//
// SPDX-License-Identifier: MPL-2.0

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day4_beautified;

#[macro_export]
macro_rules! parse_number {
    ($container:tt) => {
        $container
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse '{}': {}", $container, e))
    };

    ($container:tt.$container2:tt) => {
        $container
            .$container2
            .parse()
            .map_err(|e| anyhow::anyhow!("Failed to parse '{}': {}", $container.$container2, e))
    };
}

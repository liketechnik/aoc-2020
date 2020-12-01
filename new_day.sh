#!/bin/sh

# SPDX-FileCopyrightText: 2020 Florian Warzecha <liketechnik@disroot.org>
#
# SPDX-License-Identifier: MPL-2.0

cargo new aoc_day$1 --vcs none --edition 2018 --bin
echo "anyhow = \"1.0\"\nitertools = \"0.9\"" >> aoc_day$1/Cargo.toml
reuse addheader --copyright "Florian Warzecha <liketechnik@disroot.org>" --license "MPL-2.0" --year 2020 aoc_day$1/Cargo.toml aoc_day$1/src/main.rs
cargo member include aoc_day$1

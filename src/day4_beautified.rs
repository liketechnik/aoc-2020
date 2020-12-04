// SPDX-FileCopyrightText: 2020 Florian Warzecha <liketechnik@disroot.org>
//
// SPDX-License-Identifier: MPL-2.0

use crate::parse_number;
use anyhow::Result;

#[derive(Clone, Copy, PartialEq)]
pub enum Unit {
    Cm,
    In,
}

pub struct Passport {
    byr: Option<Result<u64>>,
    iyr: Option<Result<u64>>,
    eyr: Option<Result<u64>>,
    hgt: Option<(Result<u64>, Unit)>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    pub fn new(input: &str) -> Passport {
        let fields = input.split(char::is_whitespace);

        let byr = fields
            .clone()
            .filter(|part| part.contains("byr:"))
            .map(|part| part.strip_prefix("byr:"))
            .collect::<Vec<Option<&str>>>()
            .get(0)
            .and_then(|content| {
                if let Some(content) = content {
                    Some(parse_number!(content))
                } else {
                    None
                }
            });
        let iyr = fields
            .clone()
            .filter(|part| part.contains("iyr:"))
            .map(|part| part.strip_prefix("iyr:"))
            .collect::<Vec<Option<&str>>>()
            .get(0)
            .and_then(|content| {
                if let Some(content) = content {
                    Some(parse_number!(content))
                } else {
                    None
                }
            });
        let eyr = fields
            .clone()
            .filter(|part| part.contains("eyr:"))
            .map(|part| part.strip_prefix("eyr:"))
            .collect::<Vec<Option<&str>>>()
            .get(0)
            .and_then(|content| {
                if let Some(content) = content {
                    Some(parse_number!(content))
                } else {
                    None
                }
            });
        let hgt = fields
            .clone()
            .filter(|part| part.contains("hgt:"))
            .map(|part| {
                if let Some(part) = part.strip_prefix("hgt:")?.strip_suffix("cm") {
                    Some((part, Unit::Cm))
                } else if let Some(part) = part.strip_prefix("hgt:")?.strip_suffix("in") {
                    Some((part, Unit::In))
                } else {
                    Some(("abc", Unit::In)) // yeah, it would be cleaner to have another option inside here,
                                            // instead of forcing the parsing to fail below
                }
            })
            .collect::<Vec<Option<(&str, Unit)>>>()
            .get(0)
            .and_then(|content| {
                if let Some(content) = content {
                    Some((parse_number!(content.0), content.1))
                } else {
                    None
                }
            });
        let hcl = fields
            .clone()
            .filter(|part| part.contains("hcl:"))
            .map(|part| part.strip_prefix("hcl:"))
            .collect::<Vec<Option<&str>>>()
            .get(0)
            .and_then(|content| {
                if let Some(content) = content {
                    Some(content.to_string())
                } else {
                    None
                }
            });
        let ecl = fields
            .clone()
            .filter(|part| part.contains("ecl:"))
            .map(|part| part.strip_prefix("ecl:"))
            .collect::<Vec<Option<&str>>>()
            .get(0)
            .and_then(|content| {
                if let Some(content) = content {
                    Some(content.to_string())
                } else {
                    None
                }
            });
        let pid = fields
            .clone()
            .filter(|part| part.contains("pid:"))
            .map(|part| part.strip_prefix("pid:"))
            .collect::<Vec<Option<&str>>>()
            .get(0)
            .and_then(|content| {
                if let Some(content) = content {
                    Some(content.to_string())
                } else {
                    None
                }
            });

        Passport {
            byr,
            iyr,
            eyr,
            hgt,
            hcl,
            ecl,
            pid,
        }
    }

    fn has_req_fields(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid(&self) -> bool {
        if let Some(byr) = &self.byr {
            if let Ok(byr) = byr {
                if byr < &1920 || byr > &2002 {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        if let Some(iyr) = &self.iyr {
            if let Ok(iyr) = iyr {
                if iyr < &2010 || iyr > &2020 {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        if let Some(eyr) = &self.eyr {
            if let Ok(eyr) = eyr {
                if eyr < &2020 || eyr > &2030 {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        if let Some((hgt, unit)) = &self.hgt {
            if let Ok(hgt) = hgt {
                match unit {
                    Unit::Cm => {
                        if hgt < &150 || hgt > &193 {
                            return false;
                        }
                    }
                    Unit::In => {
                        if hgt < &59 || hgt > &76 {
                            return false;
                        }
                    }
                }
            } else {
                return false;
            }
        } else {
            return false;
        }
        if let Some(hcl) = &self.hcl {
            if !hcl.starts_with('#') && hcl.len() != 7 {
                return false;
            }
            if hcl.chars().skip(1).any(|c| !"0123456789abcdef".contains(c)) {
                return false;
            }
        } else {
            return false;
        }
        if let Some(ecl) = &self.ecl {
            if ecl != "amb"
                && ecl != "blu"
                && ecl != "brn"
                && ecl != "gry"
                && ecl != "grn"
                && ecl != "hzl"
                && ecl != "oth"
            {
                return false;
            }
        } else {
            return false;
        }
        if let Some(pid) = &self.pid {
            if pid.len() != 9 || pid.parse::<u64>().is_err() {
                return false;
            }
        } else {
            return false;
        }

        true
    }
}

pub fn input_generator(input: &str) -> Vec<Passport> {
    input.split("\n\n").map(|inp| Passport::new(inp)).collect()
}

pub fn solve_part2(input: Vec<Passport>) -> usize {
    input.iter().filter(|port| port.is_valid()).count()
}

pub fn solve_part1(input: Vec<Passport>) -> usize {
    input.iter().filter(|port| port.has_req_fields()).count()
}

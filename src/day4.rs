// SPDX-FileCopyrightText: 2020 Florian Warzecha <liketechnik@disroot.org>
//
// SPDX-License-Identifier: MPL-2.0

#[derive(Clone, Copy, PartialEq)]
pub enum Unit {
    Cm,
    In,
}

pub struct Passport {
    byr: Option<u64>,
    iyr: Option<u64>,
    eyr: Option<u64>,
    hgt: Option<(Option<u64>, Unit)>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
}

impl Passport {
    fn new(input: &str) -> Passport {
        let fields = input.split(char::is_whitespace);
        let byr: Option<u64> = fields
            .clone()
            .filter(|part| part.contains("byr:"))
            .map(|part| part.strip_prefix("byr:"))
            .collect::<Vec<Option<&str>>>()
            .get(0)
            .and_then(|content| {
                if let Some(content) = content {
                    content.parse().ok()
                } else {
                    None
                }
            });
        let iyr: Option<u64> = fields
            .clone()
            .filter(|part| part.contains("iyr:"))
            .map(|part| part.strip_prefix("iyr:"))
            .collect::<Vec<Option<&str>>>()
            .get(0)
            .and_then(|content| {
                if let Some(content) = content {
                    content.parse().ok()
                } else {
                    None
                }
            });
        let eyr: Option<u64> = fields
            .clone()
            .filter(|part| part.contains("eyr:"))
            .map(|part| part.strip_prefix("eyr:"))
            .collect::<Vec<Option<&str>>>()
            .get(0)
            .and_then(|content| {
                if let Some(content) = content {
                    content.parse().ok()
                } else {
                    None
                }
            });
        let hgt: Option<(Option<u64>, Unit)> = fields
            .clone()
            .filter(|part| part.contains("hgt:"))
            .map(|part| {
                if let Some(part) = part.strip_prefix("hgt:")?.strip_suffix("cm") {
                    Some((part, Unit::Cm))
                } else if let Some(part) = part.strip_prefix("hgt:")?.strip_suffix("in") {
                    Some((part, Unit::In))
                } else {
                    None
                }
            })
            .collect::<Vec<Option<(&str, Unit)>>>()
            .get(0)
            .and_then(|content| {
                if let Some(content) = content {
                    Some((content.0.parse().ok(), content.1))
                } else {
                    None
                }
            });
        let hcl: Option<String> = fields
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
        let ecl: Option<String> = fields
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
        let pid: Option<String> = fields
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

    fn is_valid(&self) -> bool {
        if let Some(byr) = self.byr {
            if byr < 1920 || byr > 2002 {
                return false;
            }
        } else {
            return false;
        }
        if let Some(iyr) = self.iyr {
            if iyr < 2010 || iyr > 2020 {
                return false;
            }
        } else {
            return false;
        }
        if let Some(eyr) = self.eyr {
            if eyr < 2020 || eyr > 2030 {
                return false;
            }
        } else {
            return false;
        }
        if let Some((hgt, unit)) = self.hgt {
            if let Some(hgt) = hgt {
                match unit {
                    Unit::Cm => {
                        if hgt < 150 || hgt > 193 {
                            return false;
                        }
                    }
                    Unit::In => {
                        if hgt < 59 || hgt > 76 {
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

pub fn input_generator1(input: &str) -> String {
    input.to_string()
}

pub fn solve_part1(input: String) -> usize {
    input
        .split("\n\n")
        .map(|port| port.split(char::is_whitespace))
        .filter(|port| port.clone().any(|field| field.contains("byr:")))
        .filter(|port| port.clone().any(|field| field.contains("iyr:")))
        .filter(|port| port.clone().any(|field| field.contains("eyr:")))
        .filter(|port| port.clone().any(|field| field.contains("hgt:")))
        .filter(|port| port.clone().any(|field| field.contains("hcl:")))
        .filter(|port| port.clone().any(|field| field.contains("ecl:")))
        .filter(|port| port.clone().any(|field| field.contains("pid:")))
        .count()
}

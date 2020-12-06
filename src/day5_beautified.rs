// SPDX-FileCopyrightText: 2020 Florian Warzecha <liketechnik@disroot.org>
//
// SPDX-License-Identifier: MPL-2.0

pub struct Seat {
    row: u128,
    column: u128,
}

pub struct BinarySpacePartition {
    lower_bound: u128,
    upper_bound: u128,
}

impl BinarySpacePartition {
    pub fn new(lower_bound: u128, upper_bound: u128) -> Self {
        BinarySpacePartition {
            lower_bound,
            upper_bound,
        }
    }

    #[inline]
    pub fn upper(&mut self) {
        if self.upper_bound == self.lower_bound {
            panic!("Already determined the final value!");
        }

        self.lower_bound = self.lower_bound + (self.upper_bound - self.lower_bound) / 2;
        if self.upper_bound % 2 != 0 {
            self.lower_bound += 1;
        }
    }

    #[inline]
    pub fn lower(&mut self) {
        if self.upper_bound == self.lower_bound {
            panic!("Already determined the final value!");
        }

        self.upper_bound = self.lower_bound + (self.upper_bound - self.lower_bound) / 2;
    }

    #[inline]
    pub fn value(&self) -> u128 {
        if self.lower_bound != self.upper_bound {
            panic!("Did not determine the final value yet!"); 
        }

        self.upper_bound
    }
}

impl Seat {
    fn new(input: &str) -> Seat {
        let mut row = BinarySpacePartition::new(0, 127);
        let mut column = BinarySpacePartition::new(0, 7);

        for c in input.chars() {
            match c {
                'B' => row.upper(),
                'F' => row.lower(),
                'R' => column.upper(),
                'L' => column.lower(),
                _ => panic!("Unsupported seat position specififer: {}", c)
            }
        }

        Seat {
            row: row.value(),
            column: column.value(),
        }
    }

    #[inline]
    fn id(&self) -> u128 {
        self.row * 8 + self.column 
    }
}

pub fn input_generator(input: &str) -> Vec<Seat> {
    let mut input: Vec<Seat> = input.lines().map(Seat::new).collect();
    input.sort_by_key(Seat::id);
    input
}

pub fn solve_part1(input: Vec<Seat>) -> u128 {
    input.iter().map(Seat::id).max().expect("Expecting to have a max value")
}

pub fn solve_part2(input: Vec<Seat>) -> u128 {
    let seat = input.iter()
        .filter(|seat| {
            let last_first_row = Seat { row: 0, column: 7 };
            let first_last_row = Seat { row: 127, column: 0};
            seat.id() > last_first_row.id() && seat.id() < first_last_row.id()
        })
        .try_fold(None, |prev: Option<&Seat>, seat| {
            match prev {
                None => Ok(Some(seat)),
                Some(prev) => {
                    if prev.id() + 2 == seat.id() {
                        Err(prev)
                    } else {
                        Ok(Some(seat))
                    }
                }
            }
        });
    seat.err().expect("Got no seat that has an id 1 smaller than mine").id() + 1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_seat() {
        let seat = Seat::new("FBFBBFFRLR");
        assert_eq!(seat.row, 44);
        assert_eq!(seat.column, 5);
        assert_eq!(seat.id(), 357);
    }
}

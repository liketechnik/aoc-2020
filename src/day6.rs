// SPDX-FileCopyrightText: 2020 Florian Warzecha <liketechnik@disroot.org>
//
// SPDX-License-Identifier: MPL-2.0

pub struct Answers {
    yes: [bool; 26],
}

impl Answers {
    fn new() -> Answers {
        Answers { yes: [false; 26] }
    }

    fn count_yes(&self) -> usize {
        self.yes.iter().filter(|a| **a).count()
    }

    fn answer_yes(&mut self, q: usize) {
        self.yes[q] = true;
    }
}

pub fn input_generator1(input: &str) -> Vec<Answers> {
    input
        .split("\n\n")
        .map(|group| {
            group.lines().fold(Answers::new(), |mut answers, person| {
                for c in person.bytes() {
                    answers.answer_yes((c - (b'a' - b'0') - b'0').into());
                }
                answers
            })
        })
        .collect()
}

pub fn solve_part1(input: Vec<Answers>) -> usize {
    input.iter().map(|answers| answers.count_yes()).sum()
}

pub struct EveryoneAnswers {
    yes: [usize; 26],
    group_size: usize,
}

impl EveryoneAnswers {
    fn new() -> EveryoneAnswers {
        EveryoneAnswers {
            yes: [0; 26],
            group_size: 0,
        }
    }

    fn count_yes(&self) -> usize {
        self.yes.iter().filter(|a| a == &&self.group_size).count()
    }

    fn answer_yes(&mut self, q: usize) {
        self.yes[q] += 1;
    }

    fn add_member(&mut self) {
        self.group_size += 1;
    }
}

pub fn input_generator2(input: &str) -> Vec<EveryoneAnswers> {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .fold(EveryoneAnswers::new(), |mut answers, person| {
                    answers.add_member();
                    for c in person.bytes() {
                        answers.answer_yes((c - (b'a' - b'0') - b'0').into());
                    }
                    answers
                })
        })
        .collect()
}

pub fn solve_part2(input: Vec<EveryoneAnswers>) -> usize {
    input.iter().map(|answers| answers.count_yes()).sum()
}

// SPDX-FileCopyrightText: 2020 Florian Warzecha <liketechnik@disroot.org>
//
// SPDX-License-Identifier: MPL-2.0

use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let f = File::open("input").map_err(|e| anyhow!("could not open 'input': {:?}", e))?;
    let mut f = BufReader::new(f);
    let mut input = String::new();
    f.read_line(&mut input)?;

    let mut program = Program::new(input)?;
    program.input(12, 2)?;

    println!("value at pos 0: {}", program.eval()?);
    Ok(())
}

pub use program::*;
pub mod program {

    use anyhow::{anyhow, Result};
    use num_derive::FromPrimitive;
    use num_traits::FromPrimitive;

    pub struct Program {
        memory: Vec<usize>,
        instruction_pointer: usize,
    }

    #[derive(Eq, PartialEq, FromPrimitive)]
    enum OpCode {
        HALT = 99,
        SUM = 1,
        MULTIPLY = 2,
    }

    impl Program {
        pub fn new(state: String) -> Result<Program> {
            let mut memory = Vec::new();

            for int in state.split(",") {
                let int = int.trim();
                memory.push(int.parse()?);            
            }

            Ok(Program {
                memory,
                instruction_pointer: 0,
            })
        }

        pub fn input(&mut self, input1: usize, input2: usize) -> Result<()> {
            self.write(1, input1)?;
            self.write(2, input2)?;
        
            Ok(())
        }

        fn op_code(&self) -> Result<OpCode> {
            let code = self.read(self.instruction_pointer)?;
            OpCode::from_usize(code).ok_or_else(|| anyhow!("Invalid op code: {}", code))
        }

        fn parameters(&self) -> Result<(usize, usize, usize)> {
            Ok((self.read(self.instruction_pointer + 1)?, self.read(self.instruction_pointer + 2)?, self.read(self.instruction_pointer + 3)?))
        }

        fn read(&self, address: usize) -> Result<usize> {
            Ok(*(self.memory.get(address).ok_or_else(|| anyhow!("Could not read position {}", address))?))
        }

        fn write(&mut self, address: usize, value: usize) -> Result<()> {
            if let Some(elem) = self.memory.get_mut(address) {
                *elem = value;
                Ok(())
            } else {
                Err(anyhow!("Could not write to position {}", address))
            }
        }

        pub fn eval(mut self) -> Result<usize> {
            while self.op_code()? != OpCode::HALT {
                match self.op_code()? {
                    OpCode::MULTIPLY => {
                        let parameters = self.parameters()?;
                        self.write(parameters.2, self.read(parameters.0)? * self.read(parameters.1)?)?;
                    }
                    OpCode::SUM => {
                        let parameters = self.parameters()?;
                        self.write(parameters.2, self.read(parameters.0)? + self.read(parameters.1)?)?;
                    }
                    OpCode::HALT => () // handled by while loop
                }

                self.instruction_pointer += 4;
            }

            self.read(0)
        }
    }
}

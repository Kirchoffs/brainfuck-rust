use super::opcode;

#[derive(Debug, PartialEq)]
pub enum IR {
    SHR(u32),
    SHL(u32),
    ADD(u8),
    SUB(u8),
    PUTCHAR,
    GETCHAR,
    JIZ(u32), // Jump if zero
    JNZ(u32)  // Jumo if not zero
}

pub struct Code {
    pub instructions: Vec<IR>
}

impl Code {
    pub fn from(data: Vec<opcode::Opcode>) -> Result<Self, Box<dyn std::error::Error>> {
        let mut instructions: Vec<IR> = Vec::new();

        let mut jump_stack: Vec<u32> = Vec::new();
        for e in data {
            match e {
                opcode::Opcode::SHR => {
                    match instructions.last_mut() {
                        Some(IR::SHR(cnt)) => {
                            *cnt += 1;
                        },
                        _ => {
                            instructions.push(IR::SHR(1));
                        }
                    }
                },
                opcode::Opcode::SHL => {
                    match instructions.last_mut() {
                        Some(IR::SHL(cnt)) => {
                            *cnt += 1;
                        },
                        _ => {
                            instructions.push(IR::SHL(1));
                        }
                    }
                },
                opcode::Opcode::ADD => {
                    match instructions.last_mut() {
                        Some(IR::ADD(cnt)) => {
                            *cnt = cnt.overflowing_add(1).0;
                        },
                        _ => {
                            instructions.push(IR::ADD(1));
                        }
                    }
                },
                opcode::Opcode::SUB => {
                    match instructions.last_mut() {
                        Some(IR::ADD(cnt)) => {
                            *cnt = cnt.overflowing_add(1).0;
                        },
                        _ => {
                            instructions.push(IR::SUB(1));
                        }
                    }
                },
                opcode::Opcode::PUTCHAR => {
                    instructions.push(IR::PUTCHAR);
                },
                opcode::Opcode::GETCHAR => {
                    instructions.push(IR::GETCHAR);
                },
                opcode::Opcode::LB => {
                    instructions.push(IR::JIZ(0));
                    jump_stack.push((instructions.len() - 1) as u32);
                },
                opcode::Opcode::RB => {
                    let pair_idx = jump_stack.pop().ok_or("pop from empty list")?;
                    instructions.push(IR::JNZ(pair_idx));
                    let instruction_len = instructions.len();
                    match &mut instructions[pair_idx as usize] {
                        IR::JIZ(value) => {
                            *value = (instruction_len - 1) as u32;
                        },
                        _ => unreachable!()
                    }

                }
            }
        }

        Ok(Code {instructions})
    }

    pub fn len(&self) -> usize {
        return self.instructions.len();
    }
}
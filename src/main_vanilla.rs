mod opcode;

use std::io::{ Write, Read };

struct Interpreter {
    stack: Vec<u8>
}

impl Interpreter {
    fn new() -> Self {
        Self {
            stack: vec![0; 1]
        }
    }

    fn run(&mut self, data: Vec<u8>) -> Result<(), Box<dyn std::error::Error>> {
        let code = opcode::Code::from(data)?;
        let code_len = code.len();
        let mut pc = 0; // Program Counter
        let mut sp = 0; // Stack Pointer

        loop {
            if pc >= code_len {
                break;
            }

            match code.instructions[pc] {
                opcode::Opcode::SHR => {
                    sp += 1;
                    if sp == self.stack.len() {
                        self.stack.push(0);
                    }
                },
                opcode::Opcode::SHL => {
                    if sp != 0 {
                        sp -= 1;
                    }
                },
                opcode::Opcode::ADD => {
                    self.stack[sp] = self.stack[sp].overflowing_add(1).0;
                },
                opcode::Opcode::SUB => {
                    self.stack[sp] = self.stack[sp].overflowing_sub(1).0;
                },
                opcode::Opcode::PUTCHAR => {
                    std::io::stdout().write_all(&[self.stack[sp]])?;
                },
                opcode::Opcode::GETCHAR => {
                    let mut buf: Vec<u8> = vec![0; 1];
                    std::io::stdin().read_exact(&mut buf)?;
                    self.stack[sp] = buf[0];
                },
                opcode::Opcode::LB => {
                    if self.stack[sp] == 0x00 {
                        pc = code.jump_table[&pc];
                    }
                },
                opcode::Opcode::RB => {
                    if self.stack[sp] != 0x00 {
                        pc = code.jump_table[&pc];
                    }
                }
            }

            pc += 1;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let data = std::fs::read(&args[1])?;
    
    let mut interpreter = Interpreter::new();
    interpreter.run(data)?;

    Ok(())
}
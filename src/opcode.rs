#[derive(Debug, PartialEq)]
pub enum Opcode {   // Operation code
    SHR = 0x3E,     // >
    SHL = 0x3C,     // <
    ADD = 0x2B,     // +
    SUB = 0x2D,     // -
    PUTCHAR = 0x2E, // .
    GETCHAR = 0x2C, // ,
    LB = 0x5B,      // [
    RB = 0x5D       // ]
}

impl From<u8> for Opcode {
    fn from(u: u8) -> Self {
        match u {
            0x3E => Opcode::SHR,
            0x3C => Opcode::SHL,
            0x2B => Opcode::ADD,
            0x2D => Opcode::SUB,
            0x2E => Opcode::PUTCHAR,
            0x2C => Opcode::GETCHAR,
            0x5B => Opcode::LB,
            0x5D => Opcode::RB,
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
pub struct Code {
    pub instructions: Vec<Opcode>,
    pub jump_table: std::collections::HashMap<usize, usize>,
}

impl Code {
    pub fn from(data: Vec<u8>) -> Result<Self, Box<dyn std::error::Error>> {
        let dict: Vec<u8> = vec![
            Opcode::SHR as u8,
            Opcode::SHL as u8,
            Opcode::ADD as u8,
            Opcode::SUB as u8,
            Opcode::PUTCHAR as u8,
            Opcode::GETCHAR as u8,
            Opcode::LB as u8,
            Opcode::RB as u8
        ];

        let instructions: Vec<Opcode> = 
            data.iter().filter(|x| dict.contains(x)).map(|x| Opcode::from(*x)).collect();

        let mut jump_stack: Vec<usize> = Vec::new();
        let mut jump_table: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for (idx, instruction) in instructions.iter().enumerate() {
            if *instruction == Opcode::LB {
                jump_stack.push(idx);
            }

            if *instruction == Opcode::RB {
                let pair_idx = jump_stack.pop().ok_or("pop from empty list")?;
                jump_table.insert(idx, pair_idx);
                jump_table.insert(pair_idx, idx);
            }
        }
        
        Ok(Code { instructions, jump_table })
    }

    pub fn len(&self) -> usize {
        return self.instructions.len();
    }
}
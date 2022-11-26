#[derive(Debug, PartialEq)]
pub enum Opcode {
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
    pub jumpTable: std::collections::HashMap<usize, usize>,
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

        let mut jumpStack: Vec<usize> = Vec::new();
        let mut jumpTable: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for (idx, instruction) in instructions.iter().enumerate() {
            if *instruction == Opcode::LB {
                jumpStack.push(idx);
            }

            if *instruction == Opcode::RB {
                let pair_idx = jumpStack.pop().ok_or("pop from empty list")?;
                jumpTable.insert(idx, pair_idx);
                jumpTable.insert(pair_idx, idx);
            }
        }
        
        Ok(Code { instructions, jumpTable })
    }
}

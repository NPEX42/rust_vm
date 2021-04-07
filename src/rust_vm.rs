use core::fmt;

pub static INSTRUCTION_BYTES:usize = 4;

#[derive(Clone, PartialEq, Eq)]
struct VMstate {
    ip:usize,
    stack:Vec<u16>,

    ram:[u16; 65536]
}

impl VMstate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({:#06X}, {:?})", self.ip, self.stack)
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct RustVM {
    state:VMstate
}

#[repr(C)]
pub struct Instruction {
    opcode:u16,
    data:u16
}

impl Instruction {
    pub fn size() -> usize {
        let ins:&Instruction =  &Instruction{data:0, opcode:0};
        std::mem::size_of_val(ins)
    }


}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(OC: {:#06X}, DATA: {:#06X})", self.opcode, self.data)
    }
}
 
impl RustVM {
    pub fn new() -> RustVM {
        RustVM {state: VMstate {
            ip: 0x0000,
            stack: Vec::new(),
            ram: [19; 65536]
        }}
    }

    pub fn clock(&mut self) {
        let mut ins = self.next_instruction();
        self.state.ip += 2;
        let opcode = ins.opcode;
        let data = ins.data;
        match opcode {
            _ => { println!("Illegal Instruction At {:#06X}: {}", self.state.ip - 1, ins)}
        }
    }

    fn next_instruction(&mut self) -> Instruction {
        Instruction {
            opcode: self.state.ram[self.state.ip],
            data: self.state.ram[self.state.ip + 1]
        }
    }

    pub fn get_ip(&mut self) -> usize {
        return self.state.ip;
    }

    pub fn get_instruction_length() -> usize {
        Instruction::size()
    }

}


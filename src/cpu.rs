use crate::memory::Memory;

#[derive(Debug, Clone)]
pub struct Registers {
    pub zero: u32,
    pub at: u32,

    pub v0: u32,
    pub v1: u32,

    pub a0: u32,
    pub a1: u32,
    pub a2: u32,
    pub a3: u32,

    pub t0: u32,
    pub t1: u32,
    pub t2: u32,
    pub t3: u32,
    pub t4: u32,
    pub t5: u32,
    pub t6: u32,
    pub t7: u32,

    pub s0: u32,
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
    pub s4: u32,
    pub s5: u32,
    pub s6: u32,
    pub s7: u32,

    pub t8: u32,
    pub t9: u32,

    pub k0: u32,
    pub k1: u32,

    pub gp: u32,
    pub sp: u32,
    pub fp: u32,
    pub ra: u32,

    pub pc: u32,
    pub hi: u32,
    pub lo: u32,

    position: usize,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            zero: 0,
            at: 0,

            v0: 0,
            v1: 0,

            a0: 0,
            a1: 0,
            a2: 0,
            a3: 0,

            t0: 0,
            t1: 0,
            t2: 0,
            t3: 0,
            t4: 0,
            t5: 0,
            t6: 0,
            t7: 0,

            s0: 0,
            s1: 0,
            s2: 0,
            s3: 0,
            s4: 0,
            s5: 0,
            s6: 0,
            s7: 0,

            t8: 0,
            t9: 0,

            k0: 0,
            k1: 0,

            gp: 0x10008000,
            sp: 0x7ffffffc,
            fp: 0,
            ra: 0,

            pc: 0x00400000,
            hi: 0,
            lo: 0,

            position: 0,
        }
    }
}

impl Iterator for Registers {
    type Item = (String, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.position {
            0 => Some(("zero".to_string(), self.zero)),
            1 => Some(("at".to_string(), self.at)),
            2 => Some(("v0".to_string(), self.v0)),
            3 => Some(("v1".to_string(), self.v1)),
            4 => Some(("a0".to_string(), self.a0)),
            5 => Some(("a1".to_string(), self.a1)),
            6 => Some(("a2".to_string(), self.a2)),
            7 => Some(("a3".to_string(), self.a3)),
            8 => Some(("t0".to_string(), self.t0)),
            9 => Some(("t1".to_string(), self.t1)),
            10 => Some(("t2".to_string(), self.t2)),
            11 => Some(("t3".to_string(), self.t3)),
            12 => Some(("t4".to_string(), self.t4)),
            13 => Some(("t5".to_string(), self.t5)),
            14 => Some(("t6".to_string(), self.t6)),
            15 => Some(("t7".to_string(), self.t7)),
            16 => Some(("s0".to_string(), self.s0)),
            17 => Some(("s1".to_string(), self.s1)),
            18 => Some(("s2".to_string(), self.s2)),
            19 => Some(("s3".to_string(), self.s3)),
            20 => Some(("s4".to_string(), self.s4)),
            21 => Some(("s5".to_string(), self.s5)),
            22 => Some(("s6".to_string(), self.s6)),
            23 => Some(("s7".to_string(), self.s7)),
            24 => Some(("t8".to_string(), self.t8)),
            25 => Some(("t9".to_string(), self.t9)),
            26 => Some(("k0".to_string(), self.k0)),
            27 => Some(("k1".to_string(), self.k1)),
            28 => Some(("gp".to_string(), self.gp)),
            29 => Some(("sp".to_string(), self.sp)),
            30 => Some(("fp".to_string(), self.fp)),
            31 => Some(("ra".to_string(), self.ra)),
            32 => Some(("pc".to_string(), self.pc)),
            33 => Some(("hi".to_string(), self.hi)),
            34 => Some(("lo".to_string(), self.lo)),
            _ => None,
        };
        self.position += 1;
        result
    }
}

impl Registers {
    fn read_register(&mut self, number: u8) -> u32 {
        match number {
            0 => self.zero as u32,
            1 => self.at as u32,
            2 => self.v0 as u32,
            3 => self.v1 as u32,
            4 => self.a0 as u32,
            5 => self.a1 as u32,
            6 => self.a2 as u32,
            7 => self.a3 as u32,
            8 => self.t0 as u32,
            9 => self.t1 as u32,
            10 => self.t2 as u32,
            11 => self.t3 as u32,
            12 => self.t4 as u32,
            13 => self.t5 as u32,
            14 => self.t6 as u32,
            15 => self.t7 as u32,
            16 => self.s0 as u32,
            17 => self.s1 as u32,
            18 => self.s2 as u32,
            19 => self.s3 as u32,
            20 => self.s4 as u32,
            21 => self.s5 as u32,
            22 => self.s6 as u32,
            23 => self.s7 as u32,
            24 => self.t8 as u32,
            25 => self.t9 as u32,
            26 => self.k0 as u32,
            27 => self.k1 as u32,
            28 => self.gp as u32,
            29 => self.sp as u32,
            30 => self.fp as u32,
            31 => self.ra as u32,
            _ => panic!("Invalid register number: {}", self.position),
        }
    }

    fn write_register(&mut self, number: u8, value: u32) {
        match number {
            0 => (),
            1 => self.at = value,
            2 => self.v0 = value,
            3 => self.v1 = value,
            4 => self.a0 = value,
            5 => self.a1 = value,
            6 => self.a2 = value,
            7 => self.a3 = value,
            8 => self.t0 = value,
            9 => self.t1 = value,
            10 => self.t2 = value,
            11 => self.t3 = value,
            12 => self.t4 = value,
            13 => self.t5 = value,
            14 => self.t6 = value,
            15 => self.t7 = value,
            16 => self.s0 = value,
            17 => self.s1 = value,
            18 => self.s2 = value,
            19 => self.s3 = value,
            20 => self.s4 = value,
            21 => self.s5 = value,
            22 => self.s6 = value,
            23 => self.s7 = value,
            24 => self.t8 = value,
            25 => self.t9 = value,
            26 => self.k0 = value,
            27 => self.k1 = value,
            28 => self.gp = value,
            29 => self.sp = value,
            30 => self.fp = value,
            31 => self.ra = value,
            _ => panic!("Invalid register number: {}", self.position),
        }
    }

    fn write_register_high(&mut self, number: u8, value: u16) {
        let value = (value as u32) << 16;
        match number {
            0 => (),
            1 => self.at = self.at & 0xffff | value,
            2 => self.v0 = self.v0 & 0xffff | value,
            3 => self.v1 = self.v1 & 0xffff | value,
            4 => self.a0 = self.a0 & 0xffff | value,
            5 => self.a1 = self.a1 & 0xffff | value,
            6 => self.a2 = self.a2 & 0xffff | value,
            7 => self.a3 = self.a3 & 0xffff | value,
            8 => self.t0 = self.t0 & 0xffff | value,
            9 => self.t1 = self.t1 & 0xffff | value,
            10 => self.t2 = self.t2 & 0xffff | value,
            11 => self.t3 = self.t3 & 0xffff | value,
            12 => self.t4 = self.t4 & 0xffff | value,
            13 => self.t5 = self.t5 & 0xffff | value,
            14 => self.t6 = self.t6 & 0xffff | value,
            15 => self.t7 = self.t7 & 0xffff | value,
            16 => self.s0 = self.s0 & 0xffff | value,
            17 => self.s1 = self.s1 & 0xffff | value,
            18 => self.s2 = self.s2 & 0xffff | value,
            19 => self.s3 = self.s3 & 0xffff | value,
            20 => self.s4 = self.s4 & 0xffff | value,
            21 => self.s5 = self.s5 & 0xffff | value,
            22 => self.s6 = self.s6 & 0xffff | value,
            23 => self.s7 = self.s7 & 0xffff | value,
            24 => self.t8 = self.t8 & 0xffff | value,
            25 => self.t9 = self.t9 & 0xffff | value,
            26 => self.k0 = self.k0 & 0xffff | value,
            27 => self.k1 = self.k1 & 0xffff | value,
            28 => self.gp = self.gp & 0xffff | value,
            29 => self.sp = self.sp & 0xffff | value,
            30 => self.fp = self.fp & 0xffff | value,
            31 => self.ra = self.ra & 0xffff | value,
            _ => panic!("Invalid register number: {}", self.position),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct CPU {
    pub registers: Registers,
    pub halted: bool,
    pub status: Option<String>,
    jump: bool,
}

#[derive(Debug)]
pub struct Instruction {
    opcode: u8,
    rs: u8,
    rt: u8,
    rd: u8,
    shamt: u8,
    funct: u8,
    imm: u16,
    address: u32,
    kind: InstructionKind,
}

#[derive(Debug)]
enum InstructionKind {
    RType,
    IType,
    JType,
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.kind {
            InstructionKind::RType => write!(
                f,
                "opcode: 0b{:06b}, rs: 0b{:05b}, rt: 0b{:05b}, rd: 0b{:05b}, shamt: 0b{:05b}, funct: 0b{:06b}",
                self.opcode, self.rs, self.rt, self.rd, self.shamt, self.funct
            ),
            InstructionKind::IType => write!(
                f,
                "opcode: 0b{:06b}, rs: 0b{:05b}, rt: 0b{:05b}, imm: 0x{:04x}",
                self.opcode, self.rs, self.rt, self.imm
            ),
            InstructionKind::JType => write!(f, "opcode: 0b{:06b}, address: 0x{:08x}", self.opcode, self.address),
        }
    }
}

#[derive(Debug)]
enum Exception {
    IntegerOverflow,
}

impl CPU {
    pub fn fetch_instruction(&mut self, memory: &Memory) -> Instruction {
        let instruction = memory.read_word(self.registers.pc);
        let opcode = (instruction >> 26) as u8;
        let rs = ((instruction >> 21) & 0x1f) as u8;
        let rt = ((instruction >> 16) & 0x1f) as u8;
        let rd = ((instruction >> 11) & 0x1f) as u8;
        let shamt = ((instruction >> 6) & 0x1f) as u8;
        let funct = (instruction & 0x3f) as u8;
        let imm = instruction as u16;
        let address = (instruction & 0x3ffffff) as u32;

        let kind = match opcode {
            0b000000 => InstructionKind::RType,
            0b000010 => InstructionKind::JType, // j
            0b000101 => InstructionKind::IType, // bne
            0b001001 => InstructionKind::IType, // addiu
            0b001111 => InstructionKind::IType, // lui
            0b100011 => InstructionKind::IType, // lw
            0b101011 => InstructionKind::IType, // sw
            _ => panic!("Unknown opcode: {:06b}", opcode),
        };
        Instruction {
            opcode,
            rs,
            rt,
            rd,
            shamt,
            funct,
            imm,
            address,
            kind,
        }
    }

    fn trigger_exception(&mut self, exception: Exception) {
        match exception {
            Exception::IntegerOverflow => {
                self.status = Some("Integer overflow".to_string());
                self.halted = true;
            }
        }
    }

    pub fn run(&mut self, memory: &mut Memory) {
        while !self.halted {
            self.step(memory);
        }
    }

    pub fn step(&mut self, memory: &mut Memory) {
        let instruction = self.fetch_instruction(memory);
        match instruction.opcode {
            0b000000 => self.run_r_type(instruction),
            0b000010 => self.j(&instruction),
            0b000101 => self.bne(&instruction),
            0b001001 => self.addiu(&instruction),
            0b001111 => self.lui(&instruction),
            0b100011 => self.lw(&instruction, memory),
            0b101011 => self.sw(&instruction, memory),
            _ => panic!("Unknown instruction: {}", instruction),
        }
        if !self.jump {
            self.registers.pc += 4;
        }
        self.jump = false;
    }

    /**
     * r-type
     * opcode: 0b000000
     */
    fn run_r_type(&mut self, instruction: Instruction) {
        match instruction.funct {
            0b001100 => self.syscall(),
            0b010010 => self.mflo(&instruction),
            0b011001 => self.multu(&instruction),
            0b100000 => self.add(&instruction),
            0b100010 => self.sub(&instruction),
            _ => panic!("Unknown R-Type instruction: {}", instruction),
        }
    }

    /**
     * System call
     * opcode: 0b000000
     * funct: 0b001100
     */
    fn syscall(&mut self) {
        match self.registers.v0 {
            10 => self.halted = true,
            _ => panic!("Unknown system call: {}", self.registers.v0),
        }
    }

    /**
     * Move from LO
     * opcode: 0b000000
     * funct: 0b010010
     */
    fn mflo(&mut self, instruction: &Instruction) {
        self.registers
            .write_register(instruction.rd, self.registers.lo);
    }

    /**
     * Multiply unsigned
     * opcode: 0b000000
     * funct: 0b011001
     */
    fn multu(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        let result = rs as u64 * rt as u64;
        self.registers.hi = (result >> 32) as u32;
        self.registers.lo = result as u32;
    }

    /**
     * Add
     * opcode: 0b000000
     * funct: 0b100000
     */
    fn add(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        if rs.checked_add(rt).is_none() {
            self.trigger_exception(Exception::IntegerOverflow);
        }
        self.registers
            .write_register(instruction.rd, rs.wrapping_add(rt));
    }

    /**
     * Add
     * opcode: 0b000000
     * funct: 0b100010
     */
    fn sub(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        if rs.checked_sub(rt).is_none() {
            self.trigger_exception(Exception::IntegerOverflow);
        }
        self.registers
            .write_register(instruction.rd, rs.wrapping_sub(rt));
    }

    /**
     * Jump
     * opcode: 0b000010
     */
    fn j(&mut self, instruction: &Instruction) {
        self.jump = true;
        let target = instruction.address << 2;
        // Keep the upper 4 bits of the PC
        self.registers.pc = (self.registers.pc & 0xf0000000) | target;
    }

    /**
     * Branch not equal
     * opcode: 0b000101
     */
    fn bne(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        if rs != rt {
            let target = (instruction.imm as i16 as i32) * 4;
            self.registers.pc = (self.registers.pc as i32 + target) as u32;
        }
    }

    /**
     * Add immediate unsigned
     * opcode: 0b001001
     */
    fn addiu(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let imm = instruction.imm as u32;
        self.registers
            .write_register(instruction.rt, rs.wrapping_add(imm));
    }

    /**
     * Load upper immediate
     * opcode: 0b001111
     */
    fn lui(&mut self, instruction: &Instruction) {
        self.registers
            .write_register_high(instruction.rt, instruction.imm);
    }

    /**
     * Load word
     * opcode: 0b100011
     */
    fn lw(&mut self, instruction: &Instruction, memory: &Memory) {
        let base = self.registers.read_register(instruction.rs);
        let offset = instruction.imm as i16 as i32;
        let address = base.wrapping_add(offset as u32);
        let value = memory.read_word(address);
        self.registers.write_register(instruction.rt, value);
    }

    /**
     * Store word
     * opcode: 0b101011
     */
    fn sw(&mut self, instruction: &Instruction, memory: &mut Memory) {
        let base = self.registers.read_register(instruction.rs);
        let offset = instruction.imm as i16 as i32;
        let address = base.wrapping_add(offset as u32);
        let value = self.registers.read_register(instruction.rt);
        memory.write_word(address, value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_instruction() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x16aaaaaa);
        let instruction = cpu.fetch_instruction(&memory);
        assert_eq!(instruction.opcode, 0b000101);
        assert_eq!(instruction.rs, 0b10101);
        assert_eq!(instruction.rt, 0b01010);
        assert_eq!(instruction.rd, 0b10101);
        assert_eq!(instruction.shamt, 0b01010);
        assert_eq!(instruction.funct, 0b101010);
        assert_eq!(instruction.imm, 0xaaaa);
        assert_eq!(instruction.address, 0x2aaaaaa);
    }

    #[test]
    fn test_run_add() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 1;
        cpu.registers.t2 = 2;
        memory.write_word(0x00400000, 0x012a4820); // add $t1, $t1, $t2
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 3);
        assert_eq!(cpu.registers.t2, 2);
    }

    #[test]
    fn test_run_addiu() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 1;
        memory.write_word(0x00400000, 0x25290003); // addiu $t1, $t1, 0x4821
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 4);
    }

    #[test]
    fn test_run_sub() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 1;
        cpu.registers.t2 = 2;
        memory.write_word(0x00400000, 0x012a4822); // sub $t1, $t1, $t2
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1 as i32, -1);
        assert_eq!(cpu.registers.t2, 2);
    }

    #[test]
    fn test_run_multu() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 0xaaaaaaaa;
        cpu.registers.t2 = 0x33333333;
        memory.write_word(0x00400000, 0x012a0019); // multu $t1, $t2
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.hi, 0x22222221);
        assert_eq!(cpu.registers.lo, 0xDDDDDDDE);
    }

    #[test]
    fn test_run_mflo() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.hi = 0x22222221;
        cpu.registers.lo = 0xDDDDDDDE;
        memory.write_word(0x00400000, 0x00005012); // mflo $t2
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t2, 0xDDDDDDDE);
    }

    #[test]
    fn test_run_bne() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 1;
        cpu.registers.t2 = 2;
        memory.write_word(0x00400000, 0x012a4822); // sub $t1, $t1, $t2
        memory.write_word(0x00400004, 0x152afffe); // bne $t1, $t2, -2
        cpu.step(&mut memory);
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.pc, 0x00400000);
    }

    #[test]
    fn test_run_j() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x08100000);
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.pc, 0x00400000);
    }

    #[test]
    fn test_run_lui() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x3C090001); // lui $t1, 0x0001
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0x00010000);
    }

    #[test]
    fn test_run_lw() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.data_address = 0x10000000;
        cpu.registers.t1 = 0x10000000;
        memory.write_word(0x00400000, 0x8d2a0004); // lw $t2, 4($t1)
        memory.write_word(0x10000004, 0x12345678);
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t2, 0x12345678);
    }

    #[test]
    fn test_run_sw() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.data_address = 0x10000000;
        cpu.registers.t1 = 0x10000000;
        cpu.registers.t2 = 0x12345678;
        memory.write_word(0x00400000, 0xad2a0004); // sw $t2, 4($t1)
        cpu.step(&mut memory);
        assert_eq!(memory.read_word(0x10000004), 0x12345678);
    }
}

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
    pub exception: Exception,
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

#[derive(Debug, Clone, Default, PartialEq)]
pub enum Exception {
    #[default]
    None,
    Breakpoint,
    IntegerOverflow,
    Trap,
}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Exception::None => write!(f, "No exception"),
            Exception::Breakpoint => write!(f, "Breakpoint"),
            Exception::IntegerOverflow => write!(f, "Integer overflow"),
            Exception::Trap => write!(f, "Trap"),
        }
    }
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
            0b000000 => InstructionKind::RType, // Special
            0b000010 => InstructionKind::JType, // j
            0b000100 => InstructionKind::IType, // beq
            0b000101 => InstructionKind::IType, // bne
            0b001000 => InstructionKind::IType, // addi
            0b001001 => InstructionKind::IType, // addiu
            0b001100 => InstructionKind::IType, // andi
            0b001111 => InstructionKind::IType, // lui
            0b010100 => InstructionKind::IType, // beql
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
        self.exception = exception.clone();
        self.halted = true;
    }

    pub fn run(&mut self, memory: &mut Memory) {
        while !self.halted {
            self.step(memory);
        }
    }

    pub fn step(&mut self, memory: &mut Memory) {
        let instruction = self.fetch_instruction(memory);
        match instruction.opcode {
            0b000000 => self.special(&instruction),
            0b000010 => self.j(&instruction),
            0b000100 => self.beq(&instruction),
            0b000101 => self.bne(&instruction),
            0b001000 => self.addi(&instruction),
            0b001001 => self.addiu(&instruction),
            0b001100 => self.andi(&instruction),
            0b001111 => self.lui(&instruction),
            0b010100 => self.beql(&instruction),
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
    fn special(&mut self, instruction: &Instruction) {
        match instruction.funct {
            0b000000 => self.sll(instruction),
            0b000010 => self.srl(instruction),
            0b000011 => self.sra(instruction),
            0b000100 => self.sllv(instruction),
            0b000110 => self.srlv(instruction),
            0b000111 => self.srav(instruction),
            0b001000 => self.jr(instruction),
            0b001001 => self.jalr(instruction),
            0b001010 => self.movz(instruction),
            0b001011 => self.movn(instruction),
            0b001100 => self.syscall(),
            0b001101 => self.breakpoint(),
            0b010000 => self.mfhi(instruction),
            0b010001 => self.mthi(instruction),
            0b010010 => self.mflo(instruction),
            0b010011 => self.mtlo(instruction),
            0b011000 => self.mult(instruction),
            0b011001 => self.multu(instruction),
            0b011010 => self.div(instruction),
            0b011011 => self.divu(instruction),
            0b100000 => self.add(instruction),
            0b100001 => self.addu(instruction),
            0b100010 => self.sub(instruction),
            0b100011 => self.subu(instruction),
            0b100100 => self.and(instruction),
            0b100101 => self.or(instruction),
            0b100110 => self.xor(instruction),
            0b100111 => self.nor(instruction),
            0b101010 => self.slt(instruction),
            0b101011 => self.sltu(instruction),
            0b110000 => self.tge(instruction),
            0b110001 => self.tgeu(instruction),
            0b110010 => self.tlt(instruction),
            0b110011 => self.tltu(instruction),
            0b110100 => self.teq(instruction),
            0b110110 => self.tne(instruction),
            _ => panic!("Unknown SPECIAL instruction: {}", instruction),
        }
    }

    /**
     * Shift left logical
     * opcode: 0b000000
     * funct: 0b000000
     */
    fn sll(&mut self, instruction: &Instruction) {
        let rt = self.registers.read_register(instruction.rt);
        let shamt = instruction.shamt;
        self.registers.write_register(instruction.rd, rt << shamt);
    }

    /**
     * Shift right logical
     * opcode: 0b000000
     * funct: 0b000010
     */
    fn srl(&mut self, instruction: &Instruction) {
        let rt = self.registers.read_register(instruction.rt);
        let shamt = instruction.shamt;
        let result = if instruction.rs == 0 {
            // Sign makes the shift logical
            rt >> shamt
        } else {
            rt.rotate_right(shamt as u32)
        };
        self.registers.write_register(instruction.rd, result);
    }

    /**
     * Shift right arithmetic
     * opcode: 0b000000
     * funct: 0b000011
     */
    fn sra(&mut self, instruction: &Instruction) {
        let rt = self.registers.read_register(instruction.rt) as i32;
        let shamt = instruction.shamt;
        // Sign makes the shift arithmetic
        self.registers
            .write_register(instruction.rd, (rt >> shamt) as u32);
    }

    /**
     * Shift left logical variable
     * opcode: 0b000000
     * funct: 0b000110
     */
    fn sllv(&mut self, instruction: &Instruction) {
        let rt = self.registers.read_register(instruction.rt);
        let rs = self.registers.read_register(instruction.rs);
        let shamt = rs & 0x1f;
        self.registers.write_register(instruction.rd, rt << shamt);
    }

    /**
     * Shift right logical variable
     * opcode: 0b000000
     * funct: 0b000110
     */
    fn srlv(&mut self, instruction: &Instruction) {
        let rt = self.registers.read_register(instruction.rt);
        let rs = self.registers.read_register(instruction.rs);
        let shamt = rs & 0x1f;
        let result = if instruction.shamt == 1 {
            rt.rotate_right(shamt)
        } else {
            rt >> shamt
        };
        self.registers.write_register(instruction.rd, result);
    }

    /**
     * Shift right arithmetic variable
     * opcode: 0b000000
     * funct: 0b000111
     */
    fn srav(&mut self, instruction: &Instruction) {
        let rt = self.registers.read_register(instruction.rt) as i32;
        let rs = self.registers.read_register(instruction.rs);
        let shamt = rs & 0x1f;
        self.registers
            .write_register(instruction.rd, (rt >> shamt) as u32);
    }

    /**
     * Jump register
     * opcode: 0b000000
     * funct: 0b001000
     */
    fn jr(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        self.registers.pc = rs;
        self.jump = true;
    }

    /**
     * Jump and link register
     * opcode: 0b000000
     * funct: 0b001001
     */
    fn jalr(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        self.registers
            .write_register(instruction.rd, self.registers.pc + 4);
        self.registers.pc = rs;
        self.jump = true;
    }

    /**
     * Move conditional on zero
     * opcode: 0b000000
     * funct: 0b001010
     */
    fn movz(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        if rt == 0 {
            self.registers.write_register(instruction.rd, rs);
        }
    }

    /**
     * Move conditional on not zero
     * opcode: 0b000000
     * funct: 0b001011
     */
    fn movn(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        if rt != 0 {
            self.registers.write_register(instruction.rd, rs);
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
     * Breakpoint
     * opcode: 0b000000
     * funct: 0b001101
     */
    fn breakpoint(&mut self) {
        self.halted = true;
        self.trigger_exception(Exception::Breakpoint);
    }

    /**
     * Move from HI
     * opcode: 0b000000
     * funct: 0b010000
     */
    fn mfhi(&mut self, instruction: &Instruction) {
        self.registers
            .write_register(instruction.rd, self.registers.hi);
    }

    /**
     * Move to HI
     * opcode: 0b000000
     * funct: 0b010001
     */
    fn mthi(&mut self, instruction: &Instruction) {
        self.registers.hi = self.registers.read_register(instruction.rs);
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
     * Move to LO
     * opcode: 0b000000
     * funct: 0b010011
     */
    fn mtlo(&mut self, instruction: &Instruction) {
        self.registers.lo = self.registers.read_register(instruction.rs);
    }

    /**
     * Multiply
     * opcode: 0b000000
     * funct: 0b011000
     */
    fn mult(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs) as i64;
        let rt = self.registers.read_register(instruction.rt) as i64;
        let result = rs * rt;
        self.registers.hi = (result >> 32) as u32;
        self.registers.lo = result as u32;
    }

    /**
     * Multiply unsigned
     * opcode: 0b000000
     * funct: 0b011001
     */
    fn multu(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs) as u64;
        let rt = self.registers.read_register(instruction.rt) as u64;
        let result = rs * rt;
        self.registers.hi = (result >> 32) as u32;
        self.registers.lo = result as u32;
    }

    /**
     * Divide
     * opcode: 0b000000
     * funct: 0b011010
     */
    fn div(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs) as i32;
        let rt = self.registers.read_register(instruction.rt) as i32;
        if rt == 0 {
            self.registers.hi = 0;
            self.registers.lo = 0;
        } else {
            self.registers.hi = (rs % rt) as u32;
            self.registers.lo = (rs / rt) as u32;
        }
    }

    /**
     * Divide unsigned
     * opcode: 0b000000
     * funct: 0b011011
     */
    fn divu(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        if rt == 0 {
            self.registers.hi = 0;
            self.registers.lo = 0;
        } else {
            self.registers.hi = (rs % rt) as u32;
            self.registers.lo = (rs / rt) as u32;
        }
    }

    /**
     * Add
     * opcode: 0b000000
     * funct: 0b100000
     */
    fn add(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs) as i32;
        let rt = self.registers.read_register(instruction.rt) as i32;
        if rs.checked_add(rt).is_none() {
            self.trigger_exception(Exception::IntegerOverflow);
        }
        self.registers
            .write_register(instruction.rd, (rs + rt) as u32);
    }

    /**
     * Add unsigned
     * opcode: 0b000000
     * funct: 0b100001
     */
    fn addu(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        self.registers
            .write_register(instruction.rd, rs.wrapping_add(rt));
    }

    /**
     * Subtract
     * opcode: 0b000000
     * funct: 0b100010
     */
    fn sub(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs) as i32;
        let rt = self.registers.read_register(instruction.rt) as i32;
        if rs.checked_sub(rt).is_none() {
            self.trigger_exception(Exception::IntegerOverflow);
        }
        self.registers
            .write_register(instruction.rd, (rs - rt) as u32);
    }

    /**
     * Subtract unsigned
     * opcode: 0b000000
     * funct: 0b100011
     */
    fn subu(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        self.registers
            .write_register(instruction.rd, rs.wrapping_sub(rt));
    }

    /**
     * And
     * opcode: 0b000000
     * funct: 0b100100
     */
    fn and(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        self.registers.write_register(instruction.rd, rs & rt);
    }

    /**
     * Or
     * opcode: 0b000000
     * funct: 0b100101
     */
    fn or(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        self.registers.write_register(instruction.rd, rs | rt);
    }

    /**
     * Xor
     * opcode: 0b000000
     * funct: 0b100110
     */
    fn xor(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        self.registers.write_register(instruction.rd, rs ^ rt);
    }

    /**
     * Nor
     * opcode: 0b000000
     * funct: 0b100111
     */
    fn nor(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        self.registers.write_register(instruction.rd, !(rs | rt));
    }

    /**
     * Set on less than
     * opcode: 0b000000
     * funct: 0b101010
     */
    fn slt(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs) as i32;
        let rt = self.registers.read_register(instruction.rt) as i32;
        self.registers
            .write_register(instruction.rd, (rs < rt) as u32);
    }

    /**
     * Set on less than unsigned
     * opcode: 0b000000
     * funct: 0b101011
     */
    fn sltu(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        self.registers
            .write_register(instruction.rd, (rs < rt) as u32);
    }

    /**
     * Trap if greater or equal
     * opcode: 0b000000
     * funct: 0b110000
     */
    fn tge(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs) as i32;
        let rt = self.registers.read_register(instruction.rt) as i32;
        if rs >= rt {
            self.trigger_exception(Exception::Trap);
        }
    }

    /**
     * Trap if greater or equal unsigned
     * opcode: 0b000000
     * funct: 0b110001
     */
    fn tgeu(&mut self, instruction: &Instruction) {
        println!("tgeu");
        println!("{}", instruction);
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        println!("rs: {}, rt: {}", rs, rt);
        if rs >= rt {
            self.trigger_exception(Exception::Trap);
        }
    }

    /**
     * Trap if less than
     * opcode: 0b000000
     * funct: 0b110010
     */
    fn tlt(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs) as i32;
        let rt = self.registers.read_register(instruction.rt) as i32;
        if rs < rt {
            self.trigger_exception(Exception::Trap);
        }
    }

    /**
     * Trap if less than unsigned
     * opcode: 0b000000
     * funct: 0b110011
     */
    fn tltu(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        if rs < rt {
            self.trigger_exception(Exception::Trap);
        }
    }

    /**
     * Trap if equal
     * opcode: 0b000000
     * funct: 0b110100
     */
    fn teq(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        if rs == rt {
            self.trigger_exception(Exception::Trap);
        }
    }

    /**
     * Trap if not equal
     * opcode: 0b000000
     * funct: 0b110110
     */
    fn tne(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        if rs != rt {
            self.trigger_exception(Exception::Trap);
        }
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
     * Branch equal
     * opcode: 0b000100
     */
    fn beq(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        if rs == rt {
            let target = (instruction.imm as i16 as i32) * 4;
            self.registers.pc = (self.registers.pc as i32 + target) as u32;
        }
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
     * Add immediate
     * opcode: 0b001000
     */
    fn addi(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs) as i32;
        let imm = instruction.imm as i16 as i32;
        if rs.checked_add(imm).is_none() {
            self.trigger_exception(Exception::IntegerOverflow);
        }
        self.registers
            .write_register(instruction.rt, rs.wrapping_add(imm) as u32);
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
     * And immediate
     * opcode: 0b001100
     */
    fn andi(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let imm = instruction.imm as u32;
        self.registers.write_register(instruction.rt, rs & imm);
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
     * Branch equal likely
     * opcode: 0b010100
     */
    fn beql(&mut self, instruction: &Instruction) {
        let rs = self.registers.read_register(instruction.rs);
        let rt = self.registers.read_register(instruction.rt);
        if rs == rt {
            let target = (instruction.imm as i16 as i32) * 4;
            self.registers.pc = (self.registers.pc as i32 + target) as u32;
        }
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

    // opcode: 0b000000
    // funct: 0b000000
    #[test]
    fn test_run_sll() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x000A4880); // sll $t1, $t2, 2
        cpu.registers.t2 = 0b0000_0000_0000_0000_0000_0000_0000_1010;
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0b0000_0000_0000_0000_0000_0000_0010_1000);
    }

    // opcode: 0b000000
    // funct: 0b000010
    #[test]
    fn test_run_srl() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x000A4882); // srl $t1, $t2, 2
        cpu.registers.t2 = 0b0000_0000_0000_0000_0000_0000_0000_1010;
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0b0000_0000_0000_0000_0000_0000_0000_0010);
    }

    // opcode: 0b000000
    // funct: 0b000011
    #[test]
    fn test_run_sra() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x000A4883); // sra $t1, $t2, 2
        cpu.registers.t2 = 0b1000_0000_0000_0000_0000_0000_0000_1010;
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0b1110_0000_0000_0000_0000_0000_0000_0010);
    }

    // opcode: 0b000000
    // funct: 0b000010
    #[test]
    fn test_run_rotr() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x002A4882); // rotr $t1, $t2, 2
        cpu.registers.t2 = 0b0000_0000_0000_0000_0000_0000_0000_1010;
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0b1000_0000_0000_0000_0000_0000_0000_0010);
    }

    // opcode: 0b000000
    // funct: 0b000100
    #[test]
    fn test_run_sllv() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x016A4804); // sllv $t1, $t2, $t3
        cpu.registers.t2 = 0b0000_0000_0000_0000_0000_0000_0000_1010;
        cpu.registers.t3 = 2;
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0b0000_0000_0000_0000_0000_0000_0010_1000);
    }

    // opcode: 0b000000
    // funct: 0b000110
    #[test]
    fn test_run_srlv() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x016A4806); // srlv $t1, $t2, $t3
        cpu.registers.t2 = 0b0000_0000_0000_0000_0000_0000_0000_1010;
        cpu.registers.t3 = 2;
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0b0000_0000_0000_0000_0000_0000_0000_0010);
    }

    // opcode: 0b000000
    // funct: 0b000111
    #[test]
    fn test_run_srav() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x016A4807); // srav $t1, $t2, $t3
        cpu.registers.t2 = 0b1000_0000_0000_0000_0000_0000_0000_1010;
        cpu.registers.t3 = 2;
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0b1110_0000_0000_0000_0000_0000_0000_0010);
    }

    // opcode: 0b000000
    // funct: 0b000110
    #[test]
    fn test_run_rotrv() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x014B4846); // rotrv $t1, $t2, $t3
        cpu.registers.t2 = 2;
        cpu.registers.t3 = 0b0000_0000_0000_0000_0000_0000_0000_1010;
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0b1000_0000_0000_0000_0000_0000_0000_0010);
    }

    // opcode: 0b000000
    // funct: 0b001000
    #[test]
    fn test_run_jr() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 0xDEADBEEF;
        memory.write_word(0x00400000, 0x01200008); // jr $t1
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.pc, 0xDEADBEEF);
    }

    // opcode: 0b000000
    // funct: 0b001001
    #[test]
    fn test_run_jalr() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 0xDEADBEEF;
        memory.write_word(0x00400000, 0x01205009); // jalr $t1, $t2
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.pc, 0xDEADBEEF);
    }

    // opcode: 0b000000
    // funct: 0b001010
    #[test]
    fn test_run_movz() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 0;
        cpu.registers.t2 = 0xDEADBEEF;
        cpu.registers.t3 = 0;
        memory.write_word(0x00400000, 0x014b480a); // movz $t1, $t2, $t3
        memory.write_word(0x00400004, 0x014b480a); // movz $t1, $t2, $t3
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0xDEADBEEF);
        assert_eq!(cpu.registers.t2, 0xDEADBEEF);
        assert_eq!(cpu.registers.t3, 0);
        cpu.registers.t1 = 0;
        cpu.registers.t3 = 1;
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0);
        assert_eq!(cpu.registers.t2, 0xDEADBEEF);
        assert_eq!(cpu.registers.t3, 1);
    }

    // opcode: 0b000000
    // funct: 0b001011
    #[test]
    fn test_run_movn() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 0;
        cpu.registers.t2 = 0xDEADBEEF;
        cpu.registers.t3 = 0;
        memory.write_word(0x00400000, 0x014b480b); // movn $t1, $t2, $t3
        memory.write_word(0x00400004, 0x014b480b); // movn $t1, $t2, $t3
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0);
        assert_eq!(cpu.registers.t2, 0xDEADBEEF);
        assert_eq!(cpu.registers.t3, 0);
        cpu.registers.t3 = 1;
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0xDEADBEEF);
        assert_eq!(cpu.registers.t2, 0xDEADBEEF);
        assert_eq!(cpu.registers.t3, 1);
    }

    // opcode: 0b000000
    // funct: 0b001100
    #[test]
    fn test_run_syscall() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.v0 = 10;
        memory.write_word(0x00400000, 0x0000000c); // syscall
        cpu.step(&mut memory);
        assert_eq!(cpu.halted, true);
    }

    // opcode: 0b000000
    // funct: 0b001101
    #[test]
    fn test_run_breakpoint() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x0000000d); // breakpoint
        cpu.step(&mut memory);
        assert_eq!(cpu.halted, true);
        assert_eq!(cpu.exception, Exception::Breakpoint);
    }

    // opcode: 0b000000
    // funct: 0b010000
    #[test]
    fn test_run_mfhi() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.hi = 0xDEADBEEF;
        memory.write_word(0x00400000, 0x00004810); // mfhi $t1
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0xDEADBEEF);
    }

    // opcode: 0b000000
    // funct: 0b010001
    #[test]
    fn test_run_mthi() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 0xDEADBEEF;
        memory.write_word(0x00400000, 0x01200011); // mthi $t1
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.hi, 0xDEADBEEF);
    }

    // opcode: 0b000000
    // funct: 0b010010
    #[test]
    fn test_run_mflo() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.lo = 0xDEADBEEF;
        memory.write_word(0x00400000, 0x00004812); // mflo $t1
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0xDEADBEEF);
    }

    // opcode: 0b000000
    // funct: 0b010011
    #[test]
    fn test_run_mtlo() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 0xDEADBEEF;
        memory.write_word(0x00400000, 0x01200013); // mthi $t1
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.lo, 0xDEADBEEF);
    }

    // opcode: 0b000000
    // funct: 0b011000
    #[test]
    fn test_run_mult() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 0xaaaaaaaa;
        cpu.registers.t2 = 0x33333333;
        memory.write_word(0x00400000, 0x012a0018); // mult $t1, $t2
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.hi, 0x22222221);
        assert_eq!(cpu.registers.lo, 0xDDDDDDDE);
    }

    // opcode: 0b000000
    // funct: 0b011001
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

    // opcode: 0b000000
    // funct: 0b011010
    #[test]
    fn test_run_div() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 10;
        cpu.registers.t2 = 3;
        memory.write_word(0x00400000, 0x012a001a); // div $t1, $t2
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.hi, 1);
        assert_eq!(cpu.registers.lo, 3);
    }

    // opcode: 0b000000
    // funct: 0b011011
    #[test]
    fn test_run_divu() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 10;
        cpu.registers.t2 = 3;
        memory.write_word(0x00400000, 0x012a001b); // div $t1, $t2
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.hi, 1);
        assert_eq!(cpu.registers.lo, 3);
    }

    // opcode: 0b000000
    // funct: 0b100000
    #[test]
    fn test_run_add() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t2 = 1;
        cpu.registers.t3 = 2;
        memory.write_word(0x00400000, 0x014b4820); // add $t1, $t2, $t3
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 3);
    }

    // opcode: 0b000000
    // funct: 0b100001
    #[test]
    fn test_run_addu() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t2 = 1;
        cpu.registers.t3 = 2;
        memory.write_word(0x00400000, 0x014b4821); // addu $t1, $t2, $t3
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 3);
    }

    // opcode: 0b000000
    // funct: 0b100010
    #[test]
    fn test_run_sub() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t2 = u32::MAX;
        cpu.registers.t3 = u32::MAX - 1;
        memory.write_word(0x00400000, 0x014b4822); // sub $t1, $t2, $t3
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1 as i32, 1);
    }

    // opcode: 0b000000
    // funct: 0b100011
    #[test]
    fn test_run_subu() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t2 = u32::MAX;
        cpu.registers.t3 = u32::MAX - 1;
        memory.write_word(0x00400000, 0x014b4823); // subu $t1, $t2, $t3
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 1);
    }

    // opcode: 0b000000
    // funct: 0b100100
    #[test]
    fn test_run_and() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t2 = 0b10101010;
        cpu.registers.t3 = 0b11100011;
        memory.write_word(0x00400000, 0x014b4824); // and $t1, $t2, $t3
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0b10100010);
    }

    // opcode: 0b000000
    // funct: 0b100101
    #[test]
    fn test_run_or() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t2 = 0b10101010;
        cpu.registers.t3 = 0b11100011;
        memory.write_word(0x00400000, 0x014b4825); // or $t1, $t2, $t3
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0b11101011);
    }

    // opcode: 0b000000
    // funct: 0b100110
    #[test]
    fn test_run_xor() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t2 = 0b10101010;
        cpu.registers.t3 = 0b11100011;
        memory.write_word(0x00400000, 0x014b4826); // xor $t1, $t2, $t3
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0b01001001);
    }

    // opcode: 0b000000
    // funct: 0b100111
    #[test]
    fn test_run_nor() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t2 = 0x0000FAFB;
        cpu.registers.t3 = 0x0000F000;
        memory.write_word(0x00400000, 0x014b4827); // nor $t1, $t2, $t3
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0xFFFF0504);
    }

    // opcode: 0b000000
    // funct: 0b101010
    #[test]
    fn test_run_slt() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t2 = u32::MAX - 1;
        cpu.registers.t3 = u32::MAX;
        memory.write_word(0x00400000, 0x014b482a); // slt $t1, $t2, $t3
        memory.write_word(0x00400004, 0x014b482a); // slt $t1, $t2, $t3
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 1);
        cpu.registers.t2 = u32::MAX;
        cpu.registers.t3 = u32::MAX - 1;
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0);
    }

    // opcode: 0b000000
    // funct: 0b101011
    #[test]
    fn test_run_sltu() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t2 = u32::MAX - 1;
        cpu.registers.t3 = u32::MAX;
        memory.write_word(0x00400000, 0x014b482b); // sltu $t1, $t2, $t3
        memory.write_word(0x00400004, 0x014b482b); // sltu $t1, $t2, $t3
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 1);
        cpu.registers.t2 = u32::MAX;
        cpu.registers.t3 = u32::MAX - 1;
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0);
    }

    // opcode: 0b000000
    // funct: 0b110000
    #[test]
    fn test_run_tge() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x014b4830); // tge $t2, $t3
        memory.write_word(0x00400004, 0x014b4830); // tge $t2, $t3
        memory.write_word(0x00400008, 0x014b4830); // tge $t2, $t3
        memory.write_word(0x0040000c, 0x014b4830); // tge $t2, $t3
        cpu.registers.t2 = 1;
        cpu.registers.t3 = 2;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::None);
        cpu.registers.t2 = 1;
        cpu.registers.t3 = 1;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::Trap);
        cpu.registers.t2 = u32::MAX;
        cpu.registers.t3 = 1;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::None);
        cpu.registers.t2 = 2;
        cpu.registers.t3 = 1;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::Trap);
    }

    // opcode: 0b000000
    // funct: 0b110001
    #[test]
    fn test_run_tgeu() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x014b4831); // tgeu $t2, $t3
        memory.write_word(0x00400004, 0x014b4831); // tgeu $t2, $t3
        memory.write_word(0x00400008, 0x014b4831); // tgeu $t2, $t3
        memory.write_word(0x0040000c, 0x014b4831); // tgeu $t2, $t3
        cpu.registers.t2 = 1;
        cpu.registers.t3 = 2;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::None);
        cpu.registers.t2 = 1;
        cpu.registers.t3 = 1;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::Trap);
        cpu.registers.t2 = u32::MAX;
        cpu.registers.t3 = 1;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::Trap);
        cpu.registers.t2 = 2;
        cpu.registers.t3 = 1;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::Trap);
    }

    // opcode: 0b000000
    // funct: 0b110010
    #[test]
    fn test_run_tlt() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x014b4832); // tge $t2, $t3
        memory.write_word(0x00400004, 0x014b4832); // tge $t2, $t3
        memory.write_word(0x00400008, 0x014b4832); // tge $t2, $t3
        memory.write_word(0x0040000c, 0x014b4832); // tge $t2, $t3
        cpu.registers.t2 = 1;
        cpu.registers.t3 = 2;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::Trap);
        cpu.registers.t2 = 1;
        cpu.registers.t3 = 1;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::None);
        cpu.registers.t2 = u32::MAX;
        cpu.registers.t3 = 1;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::Trap);
        cpu.registers.t2 = 2;
        cpu.registers.t3 = 1;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::None);
    }

    // opcode: 0b000000
    // funct: 0b110011
    #[test]
    fn test_run_tltu() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x014b4833); // tgeu $t2, $t3
        memory.write_word(0x00400004, 0x014b4833); // tgeu $t2, $t3
        memory.write_word(0x00400008, 0x014b4833); // tgeu $t2, $t3
        memory.write_word(0x0040000c, 0x014b4833); // tgeu $t2, $t3
        cpu.registers.t2 = 1;
        cpu.registers.t3 = 2;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::Trap);
        cpu.registers.t2 = 1;
        cpu.registers.t3 = 1;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::None);
        cpu.registers.t2 = u32::MAX;
        cpu.registers.t3 = 1;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::None);
        cpu.registers.t2 = 2;
        cpu.registers.t3 = 1;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::None);
    }

    // opcode: 0b000000
    // funct: 0b110100
    #[test]
    fn test_run_teq() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x014b4834); // teq $t2, $t3
        memory.write_word(0x00400004, 0x014b4834); // teq $t2, $t3
        cpu.registers.t2 = 1;
        cpu.registers.t3 = 2;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::None);
        cpu.registers.t3 = 1;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::Trap);
    }

    // opcode: 0b000000
    // funct: 0b110110
    #[test]
    fn test_run_tne() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x014b4836); // tne $t2, $t3
        memory.write_word(0x00400004, 0x014b4836); // tne $t2, $t3
        cpu.registers.t2 = 1;
        cpu.registers.t3 = 2;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::Trap);
        cpu.registers.t3 = 1;
        cpu.exception = Exception::None;
        cpu.step(&mut memory);
        assert_eq!(cpu.exception, Exception::None);
    }

    // opcode: 0b000010
    #[test]
    fn test_run_j() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x08100000); // j 0x00400000
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.pc, 0x00400000);
    }

    // opcode: 0b000100
    #[test]
    fn test_run_beq() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 1;
        cpu.registers.t2 = 2;
        cpu.registers.t3 = 3;
        memory.write_word(0x00400000, 0x012a4820); // add $t1, $t1, $t2
        memory.write_word(0x00400004, 0x112bfffe); // beq $t1, $t3, -2
        cpu.step(&mut memory);
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 3);
        assert_eq!(cpu.registers.t2, 2);
        assert_eq!(cpu.registers.t3, 3);
        assert_eq!(cpu.registers.pc, 0x00400000);
        cpu.step(&mut memory);
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 5);
        assert_eq!(cpu.registers.t2, 2);
        assert_eq!(cpu.registers.t3, 3);
        assert_eq!(cpu.registers.pc, 0x00400008);
    }

    // opcode: 0b000101
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

    // opcode: 0b001000
    #[test]
    fn test_run_addi() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 1;
        memory.write_word(0x00400000, 0x21290003); // addi $t1, $t1, 0x4821
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 4);
    }

    // opcode: 0b001001
    #[test]
    fn test_run_addiu() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 1;
        memory.write_word(0x00400000, 0x25290003); // addiu $t1, $t1, 3
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 4);
    }

    // opcode: 0b001100
    #[test]
    fn test_run_andi() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 0xDEADBEEF;
        memory.write_word(0x00400000, 0x3129BABE); // andi $t1, $t1, 3
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0x0000BAAE);
    }

    // opcode: 0b001111
    #[test]
    fn test_run_lui() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        memory.write_word(0x00400000, 0x3C090001); // lui $t1, 0x0001
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 0x00010000);
    }

    // opcode: 0b010100
    #[test]
    fn test_run_beql() {
        let mut cpu = CPU::default();
        let mut memory = Memory::default();
        memory.text_address = 0x00400000;
        cpu.registers.t1 = 1;
        cpu.registers.t2 = 2;
        cpu.registers.t3 = 3;
        memory.write_word(0x00400000, 0x012a4820); // add $t1, $t1, $t2
        memory.write_word(0x00400004, 0x512BFFFE); // beql $t1, $t2, -2
        cpu.step(&mut memory);
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 3);
        assert_eq!(cpu.registers.t2, 2);
        assert_eq!(cpu.registers.t3, 3);
        assert_eq!(cpu.registers.pc, 0x00400000);
        cpu.step(&mut memory);
        cpu.step(&mut memory);
        assert_eq!(cpu.registers.t1, 5);
        assert_eq!(cpu.registers.t2, 2);
        assert_eq!(cpu.registers.t3, 3);
        assert_eq!(cpu.registers.pc, 0x00400008);
    }

    // opcode: 0b100011
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
        assert_eq!(memory.data.len(), 8);
    }

    // opcode: 0b101011
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
        assert_eq!(memory.data.len(), 8);
    }
}

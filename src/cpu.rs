use crate::memory::Memory;

#[derive(Debug, Clone)]
pub struct Registers {
    pub zero: i32,
    pub at: i32,

    pub v0: i32,
    pub v1: i32,

    pub a0: i32,
    pub a1: i32,
    pub a2: i32,
    pub a3: i32,

    pub t0: i32,
    pub t1: i32,
    pub t2: i32,
    pub t3: i32,
    pub t4: i32,
    pub t5: i32,
    pub t6: i32,
    pub t7: i32,

    pub s0: i32,
    pub s1: i32,
    pub s2: i32,
    pub s3: i32,
    pub s4: i32,
    pub s5: i32,
    pub s6: i32,
    pub s7: i32,

    pub t8: i32,
    pub t9: i32,

    pub k0: i32,
    pub k1: i32,

    pub gp: i32,
    pub sp: i32,
    pub fp: i32,
    pub ra: i32,

    pub pc: u32,
    pub hi: i32,
    pub lo: i32,

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
    type Item = (String, i32);

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
            32 => Some(("pc".to_string(), self.pc as i32)),
            33 => Some(("hi".to_string(), self.hi)),
            34 => Some(("lo".to_string(), self.lo)),
            _ => None,
        };
        self.position += 1;
        result
    }
}

#[derive(Debug, Default, Clone)]
pub struct CPU {
    pub registers: Registers,
}

impl CPU {
    pub fn load_program(&mut self, memory: &mut Memory, program: Vec<u8>) {
        let start = 0x00400000;
        for (i, byte) in program.iter().enumerate() {
            memory.write_byte(i as u32 + start, *byte);
        }
    }

    pub fn fetch_instruction(&mut self, memory: &Memory) -> i32 {
        let instruction = memory.read_word(self.registers.pc);
        self.registers.pc += 4;
        instruction
    }

    pub fn run(&mut self, memory: &Memory) {
        loop {
            let instruction = self.fetch_instruction(memory);
            match instruction {
                0 => break,
                _ => println!("Unknown instruction: {:08x}", instruction),
            }
        }
    }
}

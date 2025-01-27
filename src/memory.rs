#[derive(Debug, Default)]
pub struct Memory {
    pub text: Vec<u8>,
    pub data: Vec<u8>,
    pub heap: Vec<u8>,
    pub stack: Vec<u8>,
}

#[derive(Debug, Clone, Copy)]
enum Section {
    Text,
    Data,
    Heap,
    Stack,
}

impl Memory {
    fn get_section(address: u32) -> Section {
        match address {
            0x00400000..=0x007FFFFF => Section::Text,
            0x10000000..=0x103FFFFF => Section::Data,
            0x10400000..=0x3FFFFFFF => Section::Heap,
            0x40000000..=0x7FFFFFFC => Section::Stack,
            _ => panic!("Invalid address: 0x{:08x}", address),
        }
    }

    fn get_location(section: Section, address: u32) -> usize {
        match section {
            Section::Text => (address - 0x00400000) as usize,
            Section::Data => (address - 0x10000000) as usize,
            Section::Heap => (address - 0x10400000) as usize,
            Section::Stack => (0x7FFFFFF - (address - 0x40000000)) as usize, // Stack grows downwards
        }
    }

    fn read_byte(&self, address: u32) -> i8 {
        let section = Self::get_section(address);
        let location = Self::get_location(section, address);
        match section {
            Section::Text => self.text[location] as i8,
            Section::Data => self.data[location] as i8,
            Section::Heap => self.heap[location] as i8,
            Section::Stack => self.stack[location] as i8,
        }
    }

    pub fn read_halfword(&self, address: u32) -> i16 {
        self.read_byte(address) as i16 | (self.read_byte(address + 1) as i16) << 8
    }

    pub fn read_word(&self, address: u32) -> i32 {
        self.read_byte(address) as i32
            | (self.read_byte(address + 1) as i32) << 8
            | (self.read_byte(address + 2) as i32) << 16
            | (self.read_byte(address + 3) as i32) << 24
    }

    fn write_to_memory(memory: &mut Vec<u8>, location: usize, value: u8) {
        if location < memory.len() {
            memory[location] = value;
        } else {
            memory.resize(location + 1, 0);
            memory[location] = value;
        }
    }

    pub fn write_byte(&mut self, address: u32, value: u8) {
        let section = Self::get_section(address);
        let location = Self::get_location(section, address);
        match section {
            Section::Text => Self::write_to_memory(&mut self.text, location, value),
            Section::Data => Self::write_to_memory(&mut self.data, location, value),
            Section::Heap => Self::write_to_memory(&mut self.heap, location, value),
            Section::Stack => Self::write_to_memory(&mut self.stack, location, value),
        }
    }

    pub fn write_halfword(&mut self, address: u32, value: u16) {
        self.write_byte(address, value as u8);
        self.write_byte(address + 1, (value >> 8) as u8);
    }

    pub fn write_word(&mut self, address: u32, value: u32) {
        self.write_byte(address, value as u8);
        self.write_byte(address + 1, (value >> 8) as u8);
        self.write_byte(address + 2, (value >> 16) as u8);
        self.write_byte(address + 3, (value >> 24) as u8);
    }
}

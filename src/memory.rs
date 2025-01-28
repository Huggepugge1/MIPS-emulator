use crate::elf::ELF;

#[derive(Debug, Default)]
pub struct Memory {
    pub text: Vec<u8>,
    pub data: Vec<u8>,
    pub heap: Vec<u8>,
    pub stack: Vec<u8>,

    pub text_address: u32,
    pub data_address: u32,
    pub heap_address: u32,
    pub stack_address: u32,
}

#[derive(Debug, Clone, Copy)]
enum Section {
    Text,
    Data,
    Heap,
    Stack,
}

impl Memory {
    fn set_sections(&mut self, elf: &ELF) {
        self.text_address = 0x00400000;

        for section_header in &elf.section_headers {
            let header_names = &elf.section_headers[elf.elf_header.shstrndx as usize];
            let section_name = &elf.elf
                [header_names.sh_offset as usize + section_header.sh_name as usize..]
                .split(|&x| x == 0)
                .next()
                .unwrap();

            let section_name = std::str::from_utf8(section_name).unwrap();

            if section_name == ".data" {
                self.data_address = section_header.sh_addr;
            }
        }
    }

    fn get_section(&self, address: u32) -> Section {
        let mut sections = [
            (Section::Text, self.text_address),
            (Section::Data, self.data_address),
            (Section::Heap, self.heap_address),
            (Section::Stack, self.stack_address),
        ];
        sections.sort_by(|a, b| a.1.cmp(&b.1));

        if address >= sections[0].1 && address < sections[1].1 {
            sections[0].0
        } else if address >= sections[1].1 && address < sections[2].1 {
            sections[1].0
        } else if address >= sections[2].1 && address < sections[3].1 {
            sections[2].0
        } else if address >= sections[3].1 {
            sections[3].0
        } else {
            panic!("Invalid address: 0x{:08x}", address);
        }
    }

    fn get_location(&self, address: u32) -> usize {
        match self.get_section(address) {
            Section::Text => (address - self.text_address) as usize,
            Section::Data => (address - self.data_address) as usize,
            Section::Heap => (address - self.heap_address) as usize,
            Section::Stack => todo!("Finish get_location for stack"),
        }
    }

    fn read_byte(&self, address: u32) -> u8 {
        let section = self.get_section(address);
        let location = self.get_location(address);
        match section {
            Section::Text => self.text[location],
            Section::Data => self.data[location],
            Section::Heap => self.heap[location],
            Section::Stack => self.stack[location],
        }
    }

    pub fn read_halfword(&self, address: u32) -> u16 {
        (self.read_byte(address) as u16) << 8 | self.read_byte(address + 1) as u16
    }

    pub fn read_word(&self, address: u32) -> u32 {
        (self.read_byte(address) as u32) << 24
            | (self.read_byte(address + 1) as u32) << 16
            | (self.read_byte(address + 2) as u32) << 8
            | self.read_byte(address + 3) as u32
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
        let section = self.get_section(address);
        let location = self.get_location(address);
        match section {
            Section::Text => Self::write_to_memory(&mut self.text, location, value),
            Section::Data => Self::write_to_memory(&mut self.data, location, value),
            Section::Heap => Self::write_to_memory(&mut self.heap, location, value),
            Section::Stack => Self::write_to_memory(&mut self.stack, location, value),
        }
    }

    pub fn write_halfword(&mut self, address: u32, value: u16) {
        self.write_byte(address, (value >> 8) as u8);
        self.write_byte(address + 1, value as u8);
    }

    pub fn write_word(&mut self, address: u32, value: u32) {
        self.write_byte(address, (value >> 24) as u8);
        self.write_byte(address + 1, (value >> 16) as u8);
        self.write_byte(address + 2, (value >> 8) as u8);
        self.write_byte(address + 3, value as u8);
    }

    pub fn load_elf(&mut self, binary: &[u8]) -> u32 {
        let elf = ELF::parse_elf(binary);

        self.set_sections(&elf);

        for program_header in elf.program_headers {
            if program_header.p_type == 1 {
                let section = self.get_section(program_header.p_paddr);
                let location = self.get_location(program_header.p_paddr);
                match section {
                    Section::Text => {
                        self.text
                            .resize(location + program_header.p_memsz as usize, 0);
                        self.text[location..location + program_header.p_filesz as usize]
                            .copy_from_slice(
                                &binary[program_header.p_offset as usize
                                    ..program_header.p_offset as usize
                                        + program_header.p_filesz as usize],
                            );
                    }
                    Section::Data => {
                        self.data
                            .resize(location + program_header.p_memsz as usize, 0);
                        self.data[location..location + program_header.p_filesz as usize]
                            .copy_from_slice(
                                &binary[program_header.p_offset as usize
                                    ..program_header.p_offset as usize
                                        + program_header.p_filesz as usize],
                            );
                    }
                    _ => panic!("Unsupported section"),
                }
            }
        }

        elf.elf_header.entry
    }
}

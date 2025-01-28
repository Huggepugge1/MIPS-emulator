#[derive(Debug)]
pub struct ELF {
    pub elf_header: ELFHeader,
    pub program_headers: Vec<ProgramHeader>,
    pub section_headers: Vec<SectionHeader>,
    pub elf: Vec<u8>,
}

#[derive(Debug)]
pub struct ELFHeader {
    ident: [u8; 4],
    bit: u8,
    endianness: u8,
    elf_header_version: u8,
    os_abi: u8,
    os_abi_version: u8,
    e_type: u16,
    instruction_set: u16,
    elf_version: u32,
    pub entry: u32,
    phoff: u32,
    shoff: u32,
    flags: u32,
    pub ehsize: u16,
    phentsize: u16,
    phnum: u16,
    shentsize: u16,
    shnum: u16,
    pub shstrndx: u16,
}

#[derive(Debug, Clone)]
pub struct ProgramHeader {
    pub p_type: u32,
    pub p_offset: u32,
    pub _p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub _p_flags: u32,
    pub _p_align: u32,
}

#[derive(Debug, Clone)]
pub struct SectionHeader {
    pub sh_name: u32,
    _sh_type: u32,
    _sh_flags: u32,
    pub sh_addr: u32,
    pub sh_offset: u32,
    _sh_size: u32,
    _sh_link: u32,
    _sh_info: u32,
    _sh_addralign: u32,
    _sh_entsize: u32,
}

impl ELF {
    pub fn parse_elf(elf: &[u8]) -> Self {
        let elf_header = ELFHeader::parse_elf_header(elf);
        if !elf_header.check() {
            panic!("ELF file is not valid!");
        }

        let mut program_headers = Vec::new();
        for i in 0..elf_header.phnum {
            let offset = elf_header.phoff as usize + i as usize * elf_header.phentsize as usize;
            let program_header = ProgramHeader::parse_program_header(elf, offset);
            program_headers.push(program_header);
        }

        let mut section_headers = Vec::new();
        for i in 0..elf_header.shnum {
            let offset = elf_header.shoff as usize + i as usize * elf_header.shentsize as usize;
            let section_header = SectionHeader::parse_section_header(elf, offset);
            section_headers.push(section_header);
        }

        ELF {
            elf: elf.to_vec(),
            elf_header,
            program_headers,
            section_headers,
        }
    }
}

impl ELFHeader {
    fn parse_elf_header(elf: &[u8]) -> Self {
        let ident = &elf[0..4];
        let bit = elf[4];
        let endianness = elf[5];
        let elf_header_version = elf[6];
        let os_abi = elf[7];
        let os_abi_version = elf[8];
        let _pad = &elf[9..15];

        let e_type = u16::from_be_bytes([elf[16], elf[17]]);
        let instruction_set = u16::from_be_bytes([elf[18], elf[19]]);
        let elf_version = u32::from_be_bytes([elf[20], elf[21], elf[22], elf[23]]);
        let entry = u32::from_be_bytes([elf[24], elf[25], elf[26], elf[27]]);
        let phoff = u32::from_be_bytes([elf[28], elf[29], elf[30], elf[31]]);
        let shoff = u32::from_be_bytes([elf[32], elf[33], elf[34], elf[35]]);
        let flags = u32::from_be_bytes([elf[36], elf[37], elf[38], elf[39]]);
        let ehsize = u16::from_be_bytes([elf[40], elf[41]]);
        let phentsize = u16::from_be_bytes([elf[42], elf[43]]);
        let phnum = u16::from_be_bytes([elf[44], elf[45]]);
        let shentsize = u16::from_be_bytes([elf[46], elf[47]]);
        let shnum = u16::from_be_bytes([elf[48], elf[49]]);
        let shstrndx = u16::from_be_bytes([elf[50], elf[51]]);

        ELFHeader {
            ident: ident.try_into().unwrap(),
            bit,
            endianness,
            elf_header_version,
            os_abi,
            os_abi_version,
            e_type,
            instruction_set,
            elf_version,
            entry,
            phoff,
            shoff,
            flags,
            ehsize,
            phentsize,
            phnum,
            shentsize,
            shnum,
            shstrndx,
        }
    }

    fn check(&self) -> bool {
        self.ident == [0x7f, 0x45, 0x4c, 0x46]
            && self.bit == 1
            && self.endianness == 2
            && self.elf_header_version == 0x01
            && self.os_abi == 0x00
            && self.os_abi_version == 0x00
            && self.e_type == 0x02
            && self.instruction_set == 0x0008
            && self.elf_version == 0x00000001
            && self.phoff == 0x00000034
            && self.ehsize == 52
            && self.phentsize == 0x0020
            && self.shentsize == 0x0028
    }
}

impl ProgramHeader {
    fn parse_program_header(elf: &[u8], offset: usize) -> Self {
        let p_type = u32::from_be_bytes([
            elf[offset],
            elf[offset + 1],
            elf[offset + 2],
            elf[offset + 3],
        ]);
        let p_offset = u32::from_be_bytes([
            elf[offset + 4],
            elf[offset + 5],
            elf[offset + 6],
            elf[offset + 7],
        ]);
        let _p_vaddr = u32::from_be_bytes([
            elf[offset + 8],
            elf[offset + 9],
            elf[offset + 10],
            elf[offset + 11],
        ]);
        let p_paddr = u32::from_be_bytes([
            elf[offset + 12],
            elf[offset + 13],
            elf[offset + 14],
            elf[offset + 15],
        ]);
        let p_filesz = u32::from_be_bytes([
            elf[offset + 16],
            elf[offset + 17],
            elf[offset + 18],
            elf[offset + 19],
        ]);
        let p_memsz = u32::from_be_bytes([
            elf[offset + 20],
            elf[offset + 21],
            elf[offset + 22],
            elf[offset + 23],
        ]);
        let _p_flags = u32::from_be_bytes([
            elf[offset + 24],
            elf[offset + 25],
            elf[offset + 26],
            elf[offset + 27],
        ]);
        let _p_align = u32::from_be_bytes([
            elf[offset + 28],
            elf[offset + 29],
            elf[offset + 30],
            elf[offset + 31],
        ]);

        ProgramHeader {
            p_type,
            p_offset,
            _p_vaddr,
            p_paddr,
            p_filesz,
            p_memsz,
            _p_flags,
            _p_align,
        }
    }
}

impl SectionHeader {
    fn parse_section_header(elf: &[u8], offset: usize) -> Self {
        let sh_name = u32::from_be_bytes([
            elf[offset],
            elf[offset + 1],
            elf[offset + 2],
            elf[offset + 3],
        ]);
        let _sh_type = u32::from_be_bytes([
            elf[offset + 4],
            elf[offset + 5],
            elf[offset + 6],
            elf[offset + 7],
        ]);
        let _sh_flags = u32::from_be_bytes([
            elf[offset + 8],
            elf[offset + 9],
            elf[offset + 10],
            elf[offset + 11],
        ]);
        let sh_addr = u32::from_be_bytes([
            elf[offset + 12],
            elf[offset + 13],
            elf[offset + 14],
            elf[offset + 15],
        ]);
        let sh_offset = u32::from_be_bytes([
            elf[offset + 16],
            elf[offset + 17],
            elf[offset + 18],
            elf[offset + 19],
        ]);
        let _sh_size = u32::from_be_bytes([
            elf[offset + 20],
            elf[offset + 21],
            elf[offset + 22],
            elf[offset + 23],
        ]);
        let _sh_link = u32::from_be_bytes([
            elf[offset + 24],
            elf[offset + 25],
            elf[offset + 26],
            elf[offset + 27],
        ]);
        let _sh_info = u32::from_be_bytes([
            elf[offset + 28],
            elf[offset + 29],
            elf[offset + 30],
            elf[offset + 31],
        ]);
        let _sh_addralign = u32::from_be_bytes([
            elf[offset + 32],
            elf[offset + 33],
            elf[offset + 34],
            elf[offset + 35],
        ]);
        let _sh_entsize = u32::from_be_bytes([
            elf[offset + 36],
            elf[offset + 37],
            elf[offset + 38],
            elf[offset + 39],
        ]);

        SectionHeader {
            sh_name,
            _sh_type,
            _sh_flags,
            sh_addr,
            sh_offset,
            _sh_size,
            _sh_link,
            _sh_info,
            _sh_addralign,
            _sh_entsize,
        }
    }
}

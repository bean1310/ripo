use std::fmt::{Display, Write, Result, Formatter};

const X86_STR: &str = "x86";
const X86_64_STR: &str = "x86_64";
const ARM_STR: &str = "arm";
const ARM64_STR: &str = "arm64";

pub enum CpuType {
    X86 = 7,
    ARM = 12,
    X86_64 = 7 | 0x01000000,
    ARM64  = 12 | 0x01000000,
}

pub struct Arch {
    cpu_type    : CpuType,
    cpu_sub_type: u32,
    offset      : u32,
    size        : u32,
    align       : u32,
}

impl Arch {
    pub fn new(cpu_type: CpuType, cpu_sub_type: u32, offset: u32, size: u32, align: u32) -> Self {
        Arch {
            cpu_type    : cpu_type,
            cpu_sub_type: cpu_sub_type,
            offset      : offset,
            size        : size,
            align       : align,
        }
    }

    pub fn new_from_byte(byte: &[u8;20]) -> Self {
        if byte.len() != 20 {
            panic!("hoge");
        }       

        // not good...
        let _cpu_type       = CpuType::from_be_bytes([byte[0], byte[1], byte[2], byte[3]]);
        let cpu_sub_type    = u32::from_be_bytes([byte[4], byte[5], byte[6], byte[7]]);
        let _offset         = u32::from_be_bytes([byte[8], byte[9], byte[10], byte[11]]);
        let _size           = u32::from_be_bytes([byte[12], byte[13], byte[14], byte[15]]);
        let _align          = u32::from_be_bytes([byte[16], byte[17], byte[18], byte[19]]);

        Arch::new(_cpu_type, cpu_sub_type, _offset, _size, _align)

    }

    fn cpu_arch_string(&self) -> &str {
        match self.cpu_type {
            CpuType::X86    => X86_STR,
            CpuType::X86_64 => X86_64_STR,
            CpuType::ARM    => ARM_STR,
            CpuType::ARM64  => ARM64_STR,
        }
    }

}

impl CpuType {
    pub fn from_be_bytes(byte_array: [u8; 4]) -> Self {

        let target = i32::from_be_bytes(byte_array);
        let _x86 = CpuType::X86 as i32;
        let _arm = CpuType::ARM as i32;
        let _x86_64 = CpuType::X86_64 as i32;
        let _arm64 = CpuType::ARM64 as i32;
        
        if target == _x86 {
            CpuType::X86
        } else if target == _x86_64 {
            CpuType::X86_64
        } else if target == _arm {
            CpuType::ARM
        } else if target == _arm64 {
            CpuType::ARM64
        } else {
            panic!("Unknown arch type")
        }
    }
}

impl Display for Arch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "
            architecture\t: {}
            CPU sub type\t: {}
            Offset\t\t: 0x{:x}
            Size\t\t: 0x{:x}
            Align\t\t: 0x{:x}
            ",
            self.cpu_arch_string(),
            self.cpu_sub_type,
            self.offset,
            self.size,
            self.align
        )
    }
}

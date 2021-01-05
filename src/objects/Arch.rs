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
    pub fn new(cpu_type: CpuType, cpu_sub_type: u32, offset: u32, size: u32, align: u32) -> Arch {
        Arch {
            cpu_type    : cpu_type,
            cpu_sub_type: cpu_sub_type,
            offset      : offset,
            size        : size,
            align       : align,
        }
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



impl Display for Arch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "
            architecture: {}\n
            \tCPU sub type: {}\n
            \tOffset: 0x{:x}\n
            \tSize: 0x{:x}\n
            \tAlign: 0x{:x}
            ",
            self.cpu_arch_string(),
            self.cpu_sub_type,
            self.offset,
            self.size,
            self.align
        )
    }
}


// architecture arm64
//     cputype CPU_TYPE_ARM64
//     cpusubtype CPU_SUBTYPE_ARM64_ALL
//     capabilities 0x0
//     offset 65536
//     size 49956
//     align 2^14 (16384)
use std::fmt::{Display, Formatter, Result};

/// CpuType represents CPU architecture. (e.g. x86, arm64)
pub enum CpuType {
    X86 = 7,
    ARM = 12,
    X86_64 = 7 | 0x01000000,
    ARM64  = 12 | 0x01000000,
}

impl CpuType {
    /// CpuType has from_be_byte method.
    /// This returns CpuType value from a byte array.
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

/// CpuType has to_string method
impl Display for CpuType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Self::X86    => write!(f, "x86"),
            Self::X86_64 => write!(f, "x86_64"),
            Self::ARM    => write!(f, "arm"),
            Self::ARM64  => write!(f, "arm64")
        }
    }
}

/// Arch represent each architecture info inside Fat binary.
pub struct Arch {
    cpu_type    : CpuType,
    cpu_sub_type: u32,
    offset      : u32,
    size        : u32,
    align       : u32,
}

impl Arch {
    /// Arch can be instantiated by new() method.
    pub fn new(cpu_type: CpuType, cpu_sub_type: u32, offset: u32, size: u32, align: u32) -> Self {
        Self {
            cpu_type    : cpu_type,
            cpu_sub_type: cpu_sub_type,
            offset      : offset,
            size        : size,
            align       : align,
        }
    }

    /// Arch can be instantiated by new_from_byte() method from a byte array.
    pub fn new_from_byte(byte: &[u8; 20]) -> Self {
        if byte.len() != 20 {
            panic!("hoge");
        }

        // not good...
        let _cpu_type = CpuType::from_be_bytes([byte[0], byte[1], byte[2], byte[3]]);
        let cpu_sub_type = u32::from_be_bytes([byte[4], byte[5], byte[6], byte[7]]);
        let _offset = u32::from_be_bytes([byte[8], byte[9], byte[10], byte[11]]);
        let _size = u32::from_be_bytes([byte[12], byte[13], byte[14], byte[15]]);
        let _align = u32::from_be_bytes([byte[16], byte[17], byte[18], byte[19]]);

        Self::new(_cpu_type, cpu_sub_type, _offset, _size, _align)
    }

    pub fn offset(&self) -> u32 {
        self.offset
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    /// This method returns human readable cpu architecture name. (e.g. "x86" "arm64")
    fn cpu_arch_string(&self) -> String {
        self.cpu_type.to_string()
    }
}

impl Display for Arch {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "
            architecture\t: {}
            CPU sub type\t: {}
            Offset\t\t: {}
            Size\t\t: {}
            Align\t\t: 2^{}\n",
            self.cpu_arch_string(),
            self.cpu_sub_type,
            self.offset,
            self.size,
            self.align
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arch_new_test() {
        let created_by_method = Arch::new(
            CpuType::X86_64,
            0x00000003,
            0x00004000,
            0x000080f0,
            0x0000000e,
        );

        let x86_64 = Arch {
            cpu_type: CpuType::X86_64,
            cpu_sub_type: 0x00000003,
            offset: 0x00004000,
            size: 0x000080f0,
            align: 0x0000000e,
        };

        assert_eq!(
            created_by_method.cpu_type as i32, x86_64.cpu_type as i32,
            "Cpu type is not equal"
        );
        assert_eq!(
            created_by_method.cpu_sub_type, x86_64.cpu_sub_type,
            "Cpu sub-type is not equal"
        );
        assert_eq!(
            created_by_method.offset, x86_64.offset,
            "Offset is not equal"
        );
        assert_eq!(created_by_method.size, x86_64.size, "Size is not equal");
        assert_eq!(created_by_method.align, x86_64.align, "Align is not equal");
    }

    #[test]
    fn arch_new_from_byte_test() {
        let arch_header: [u8; 20] = [
            1, 0, 0, 7, 0, 0, 0, 3, 0, 0, 64, 0, 0, 0, 128, 240, 0, 0, 0, 14,
        ];
        let created_by_method = Arch::new_from_byte(&arch_header);
        let x86_64 = Arch {
            cpu_type: CpuType::X86_64,
            cpu_sub_type: 0x00000003,
            offset: 0x00004000,
            size: 0x000080f0,
            align: 0x0000000e,
        };

        assert_eq!(
            created_by_method.cpu_type as i32, x86_64.cpu_type as i32,
            "Cpu type is not equal"
        );
        assert_eq!(
            created_by_method.cpu_sub_type, x86_64.cpu_sub_type,
            "Cpu sub-type is not equal"
        );
        assert_eq!(
            created_by_method.offset, x86_64.offset,
            "Offset is not equal"
        );
        assert_eq!(created_by_method.size, x86_64.size, "Size is not equal");
        assert_eq!(created_by_method.align, x86_64.align, "Align is not equal");
    }

    #[test]
    fn cpu_type_to_string_test() {
        let x86_64 = Arch {
            cpu_type: CpuType::X86_64,
            cpu_sub_type: 0x00000003,
            offset: 0x00004000,
            size: 0x000080f0,
            align: 0x0000000e,
        };

        let arm64 = Arch {
            cpu_type: CpuType::ARM64,
            cpu_sub_type: 0x00000003,
            offset: 0x00004000,
            size: 0x000080f0,
            align: 0x0000000e,
        };

        assert_eq!(x86_64.cpu_type.to_string(), "x86_64");
        assert_eq!(arm64.cpu_type.to_string(), "arm64");
    }

    #[test]
    fn cpu_type() {
        let target_x86_64 = CpuType::from_be_bytes([1, 0, 0, 7]);
        assert_eq!(target_x86_64 as usize, CpuType::X86_64 as usize);
        let target_arm_64 = CpuType::from_be_bytes([1, 0, 0, 0xc]);
        assert_eq!(target_arm_64 as usize, CpuType::ARM64 as usize);
    }
}

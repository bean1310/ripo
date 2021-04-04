use super::arch::Arch;
use std::convert::TryInto;
use std::io::prelude::*;
use std::io::SeekFrom;
use crate::ripo_error::RipoError;

pub struct File {
    magic: [u8; 4],
    arch_info: Vec<Arch>,
    arch_binary: Vec<Vec<u8>>,
}

impl File {
    pub fn from_binary_file(mut file: &std::fs::File) -> Result<Self, Box<dyn std::error::Error>> {
        let mut magic = [0; 4];
        let mut arch_count = [0; 4];

        file.read(&mut magic)?;
        file.read(&mut arch_count)?;

        let _fat_magic = [0xca, 0xfe, 0xba, 0xbe];
        if magic != _fat_magic {
            return Err(Box::new(RipoError::InvalidFileError));
        }

        let _arch_count = u32::from_be_bytes(arch_count);

        let mut archs: Vec<Arch> = Vec::new();

        for _i in 0.._arch_count {
            let mut arch = [0; 20];
            file.read(&mut arch)?;
            archs.push(Arch::new_from_byte(&arch));
        }

        let mut binaries: Vec<Vec<u8>> = Vec::new();
        for arch in archs.iter() {
            let mut binary: Vec<u8> = vec![0; arch.size().try_into().unwrap()];
            file.seek(SeekFrom::Start(arch.offset().into()))?;
            file.read_exact(&mut binary)?;
            binaries.push(binary);
        }

        Ok(Self {
            magic: magic,
            arch_info: archs,
            arch_binary: binaries,
        })
    }

    pub fn print_arch_info(&self) {
        for arch in self.arch_info.iter() {
            print!("{}", arch);
        }
    }
}

mod tests {
    use super::*;
    use std::prelude::*;
    use std::path::{Path, PathBuf};

    #[test]
    fn from_binary_file_pos_test() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("bin");
        path.push("universal_app");
        let file = match std::fs::File::open(path.as_path()) {
            Ok(file) => file,
            Err(why) => panic!("could not open {}: {}", path.display(), why),
        };

        let fat_binary = File::from_binary_file(&file).unwrap();

        assert_eq!(fat_binary.magic, [0xca, 0xfe, 0xba, 0xbe]);
        assert_eq!(fat_binary.arch_info.len(), 2); // not good

    }

    #[test]
    fn from_binary_file_neg_test() {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.push("bin");
        path.push("arm_app");
        let file = match std::fs::File::open(path.as_path()) {
            Ok(file) => file,
            Err(why) => panic!("could not open {}: {}", path.display(), why),
        };

        let fat_binary = File::from_binary_file(&file).unwrap();
        // TODO: This uncomplete unit test coding
        assert_eq!(fat_binary.magic, [0xca, 0xfe, 0xba, 0xbe]);
        assert_eq!(fat_binary.arch_info.len(), 1); 
    }
}
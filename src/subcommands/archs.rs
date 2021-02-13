use crate::objects::arch::{Arch, CpuType};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn cli(archs: &&clap::ArgMatches) -> bool {
    let input_file_path = archs.value_of("input_file").unwrap();
    dbg!(input_file_path);

    let path = Path::new(input_file_path);
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("could not open {}: {}", path.display(), why),
    };

    let mut magic = [0; 4];
    let mut arch_count = [0; 4];

    file.read(&mut magic);
    file.read(&mut arch_count);

    let _fat_magic = [0xca, 0xfe, 0xba, 0xbe];
    if magic != _fat_magic {
        panic!("is not universal binary");
    }

    let _arch_count = u32::from_be_bytes(arch_count);

    let mut archs: Vec<Arch> = Vec::new();
    for _i in 0.._arch_count {
        let mut arch = [0; 20];
        file.read(&mut arch);
        archs.push(Arch::new_from_byte(&arch));
    }

    for arch in archs {
        println!("{}", arch);
    }
    
    return true;
}

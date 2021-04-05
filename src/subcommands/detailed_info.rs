use crate::objects::arch::Arch;
use super::super::objects::fat_binary;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn cli(args: &&clap::ArgMatches) -> Result<(), ()> {
    let input_file_path = args.value_of("input_file").unwrap();
    dbg!(input_file_path);

    let path = Path::new(input_file_path);
    let file = match File::open(path) {
        Ok(file) => file,
        Err(why) => panic!("could not open {}: {}", path.display(), why),
    };

    let fat_binary_file = match fat_binary::File::from_binary_file(&file) {
        Ok(file) => file,
        Err(why) => panic!("Failed to instantiate {} as fat binary file: {}", path.display(), why)
    };

    fat_binary_file.print_arch_info();
    
    Ok(())
}

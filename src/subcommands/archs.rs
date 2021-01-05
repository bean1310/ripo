use crate::objects::arch::{Arch, CpuType};

pub fn cli(archs: &&clap::ArgMatches) -> bool
{
    let input_file_path = archs.value_of("input_file").unwrap().to_string();
    dbg!(input_file_path);
    let arch = Arch::new(CpuType::X86, 12, 1, 1, 1);
    println!("{}", arch);
    return true;
}
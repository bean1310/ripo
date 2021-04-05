pub fn cli(detailed_info: &&clap::ArgMatches) {
    let input_file_name = detailed_info.value_of("input_file").unwrap();
    dbg!(input_file_name);
}

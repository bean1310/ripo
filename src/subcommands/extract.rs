pub fn cli(extract: &&clap::ArgMatches) -> bool {
    let input_file_path = extract.value_of("input_file").unwrap().to_string();
    dbg!(input_file_path);
    return true;
}
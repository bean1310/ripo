pub fn cli(create: &&clap::ArgMatches) {
    if let Some(input_files) = create.values_of("input_file_path") {
        for input_file in input_files {
            dbg!(input_file);
        }
    }
    let output_file = create.value_of("output_file_path").unwrap().to_string();
    dbg!(output_file);
    println!("call create");
}
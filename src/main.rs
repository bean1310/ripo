#[macro_use]
extern crate clap;
use clap::App;

mod subcommands;
use subcommands::*;

mod objects;

mod ripo_error;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let _matches = App::from_yaml(yaml).get_matches();

    if let Some(ref archs) = _matches.subcommand_matches("archs") {
        archs::cli(archs);
    } else if let Some(ref detailed_info) = _matches.subcommand_matches("detailed_info") {
        detailed_info::cli(detailed_info).unwrap();
    } else if let Some(ref create) = _matches.subcommand_matches("create") {
        create::cli(create);
    } else if let Some(ref extract) =  _matches.subcommand_matches("extract") {
        extract::cli(extract);
    } else {
        panic!("Err");
    }
}

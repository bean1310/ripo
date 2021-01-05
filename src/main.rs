#[macro_use]
extern crate clap;
use clap::App;
mod subcommands;
mod objects;

fn main() {
    // The YAML file is found relative to the current file, similar to how modules are found
    let yaml = load_yaml!("cli.yml");
    let _matches = App::from_yaml(yaml).get_matches();

    if let Some(ref archs) = _matches.subcommand_matches("archs") {
        subcommands::archs::cli(archs);
    } else if let Some(ref detailed_info) = _matches.subcommand_matches("detailed_info") {
        subcommands::detailed_info::cli(detailed_info);
    } else if let Some(ref create) = _matches.subcommand_matches("create") {
        subcommands::create::cli(create);
    } else if let Some(ref extract) =  _matches.subcommand_matches("extract") {
        subcommands::extract::cli(extract);
    } else {
        panic!("Err");
    }

    // match _matches.subcommand_name() {
    //     Some("archs") => archs::cli(),
    //     Some("create") => create::cli(),
    //     Some("detailed_info") => {

    //         detailed_info::cli(_matches)
    //     },
    //     Some("extract") => extract::cli(),
    //     None => println!("none"),
    //     _ => println!("def"),
    // }
}

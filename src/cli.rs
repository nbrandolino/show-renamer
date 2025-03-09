use crate::constants::NAME;
use crate::constants;

// cli arguments
pub fn build_cli() -> clap::Command {
    clap::Command::new(NAME)
        .version(constants::VERSION)
        .author(constants::AUTHOR)
        .about(format!("A helper tool for renaming the seasons and episodes of a tv show"))
        // rename
        .arg(
            clap::Arg::new("rename")
                .long("rename")
                .short('r')
                .help("Renames a TV show's seasons and episodes at the specified path")
                .value_name("PATH")
                .value_parser(clap::value_parser!(String))
                .required(true),
        )
}

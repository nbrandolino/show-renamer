use crate::constants::NAME;
use crate::constants;

// cli arguments
pub fn build_cli() -> clap::Command {
    clap::Command::new(NAME)
        .version(constants::VERSION)
        .author(constants::AUTHOR)
        .about(format!("A helper tool for renaming the seasons and episodes of a tv show"))
        // test flag
        .arg(
            clap::Arg::new("rename")
                .long("rename")
                .short('r')
                .help("Renames a TV shows seasons and episodes")
                .value_parser(clap::value_parser!(String)),
        )
}

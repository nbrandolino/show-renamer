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
            clap::Arg::new("test")
                .long("test")
                .short('t')
                .help("test flag")
                .action(clap::ArgAction::SetTrue),
        )
}

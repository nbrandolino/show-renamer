use show_renamer::cli;
use show_renamer::utils;

// main function
fn main() {
    // check os
    utils::check_os();

    let matches = cli::build_cli().get_matches();
    if matches.get_flag("test") {
        println!("Hello, world!");
    }
    // if no flags are passed display message
    else {
        println!("No action specified. Use --help for usage.");
    }
}

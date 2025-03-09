use show_renamer::actions::*;
use show_renamer::cli;
use show_renamer::utils;

// main function
fn main() {
    // check os
    utils::check_os();

    let matches = cli::build_cli().get_matches();
    if matches.get_flag("rename") {
        if let Some(show_path) = matches.get_one::<String>("rename") {
            rename::main(show_path);
        }
        else {
            eprintln!("Error: No path provided. Use: show-renamer --rename <SHOW_PATH>");
        }
    }
    // if no flags are passed display message
    else {
        eprintln!("No action specified. Use --help for usage.");
    }
}

use show_renamer::actions::*;
use show_renamer::cli;
use show_renamer::utils;

// main function
fn main() {
    utils::check_os();

    let matches = cli::build_cli().get_matches();
    if let Some(mut values) = matches.get_many::<String>("rename") {
        let show_path = values.next().unwrap();
        let new_name = values.next().map(|s| s.as_str());

        rename::main(show_path, new_name);
    }
    else {
        eprintln!("No action specified. Use --help for usage.");
    }
}

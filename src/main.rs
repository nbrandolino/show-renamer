use show_renamer::cli;

// main function
fn main() {
    let matches = cli::build_cli().get_matches();
    if matches.get_flag("test") {
        println!("Hello, world!");
    }
}

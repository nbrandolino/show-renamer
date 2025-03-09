use std::process;

// verify if program is being run on macos or linux
pub fn check_os() {
    let target_os = std::env::consts::OS;
    if target_os == "macos" || target_os == "linux" {
        // do nothing
    }
    else {
        eprintln!("Error: Unsupported operating system: {}", target_os);
        process::exit(1);
    }
}

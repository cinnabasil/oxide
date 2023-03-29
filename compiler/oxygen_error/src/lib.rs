use std::result;

const RED: &str = "\u{001b}[31m";
const BOLD: &str = "\u{001b}[1m";
const WHITE: &str = "\u{001b}[37m";
const RESET: &str = "\u{001b}[0;0m";

// This structure represents an error that
// has already been outputted to the user,
// and therefore doesn't need to be handled but
// just propogated
pub struct ErrorEmitted(());

pub type Result<T> = result::Result<T, ErrorEmitted>;

pub fn early_error(message: String) -> ErrorEmitted {
    eprintln!("{RED}{BOLD}error: {WHITE}{message}{RESET}");    

    ErrorEmitted(())
}

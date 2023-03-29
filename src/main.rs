use std::env;

use oxygen::run_compiler;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    run_compiler(arguments).unwrap_or_else(|_err| {
        // oxygen_error::Result is an `ErrorEmitted` type,
        // so we don't actually care too much about the error,
        // since it has already been emitted to the user.
        std::process::exit(1);
    });
}

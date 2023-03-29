use oxygen_error::{ early_error, Result };
use oxygen_options::Options;
use oxygen_lexer::tokenize;

fn usage() {
    println!("Usage: oxygen [OPTIONS] <input>");
    println!("\t-h, --help            Print this message and exit.");
}

fn handle_arguments(arguments: &[String]) -> Result<Option<Options>> {
    let arguments = &arguments[1..];

    if arguments.is_empty() {
        usage();
        return Ok(None);
    }

    let mut idx = 0;
    let mut options = Options::default();

    while idx < arguments.len() {
        let argument = &arguments[idx];
        idx += 1;

        if argument.starts_with("-") {
            match argument.as_str() {
                "-h" | "--help" => {
                    usage();
                    return Ok(None);
                },
                _ => {
                    Err(early_error(format!("No such argument: {argument}")))?;
                }
            }
        } else {
            if !options.input_path.is_empty() {
                Err(early_error(format!(
                    "More than one input file was provided (Found {} and {})",
                    options.input_path,
                    argument
                )))?;
            }

            options.input_path = argument.to_string();
        }
    }

    Ok(Some(options))
}

pub fn run_compiler(arguments: Vec<String>) -> Result<()> {
    let Some(options) = handle_arguments(&arguments)? else { return Ok(()) };

    let src = std::fs::read_to_string(options.input_path).unwrap();
    for token in tokenize(&src) {
        println!("{token:?}");
    }

    Ok(())
}

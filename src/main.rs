use rusty_scissors::{process_directory, AppError};
use std::env;
use std::path::PathBuf;
use std::process;

fn parse_args() -> Result<(PathBuf, bool), AppError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 3 {
        return Err(AppError {
            message: format!("Usage: {} <input-path> [--override]", args[0]),
        });
    }
    let override_flag = args.len() == 3 && args[2] == "--override";
    Ok((PathBuf::from(&args[1]), override_flag))
}

fn main() {
    match parse_args().and_then(|(path, override_flag)| process_directory(&path, override_flag)) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

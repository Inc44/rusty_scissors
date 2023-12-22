use rusty_scissors::{process_directory, AppError};
use std::env;
use std::path::PathBuf;
use std::process;

fn parse_args() -> Result<(PathBuf, bool, bool), AppError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 4 {
        return Err(AppError {
            message: format!("Usage: {} <input-path> [--override] [--keep]", args[0]),
        });
    }
    let override_flag = args.contains(&"--override".to_string());
    let keep_flag = args.contains(&"--keep".to_string());

    Ok((PathBuf::from(&args[1]), override_flag, keep_flag))
}

fn main() {
    match parse_args() {
        Ok((path, override_flag, keep_flag)) => {
            if let Err(e) = process_directory(&path, override_flag, keep_flag) {
                eprintln!("{}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1);
        }
    }
}

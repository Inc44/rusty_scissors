use rusty_scissors::{process_directory, AppError};
use std::env;
use std::path::PathBuf;
use std::process;

fn parse_args() -> Result<(PathBuf, bool, bool, f32), AppError> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 || args.len() > 5 {
        return Err(AppError {
            message: format!(
                "Usage: {} <input-path> [--override] [--keep] [--tolerance=<percentage>]",
                args[0]
            ),
        });
    }
    let override_flag = args.contains(&"--override".to_string());
    let keep_flag = args.contains(&"--keep".to_string());

    let tolerance_percent =
        if let Some(tolerance_arg) = args.iter().find(|arg| arg.starts_with("--tolerance=")) {
            tolerance_arg["--tolerance=".len()..]
                .parse()
                .map_err(|_| AppError {
                    message: format!("Invalid tolerance value: {}", tolerance_arg),
                })?
        } else {
            0.0
        };

    Ok((
        PathBuf::from(&args[1]),
        override_flag,
        keep_flag,
        tolerance_percent,
    ))
}

fn main() {
    match parse_args() {
        Ok((path, override_flag, keep_flag, tolerance_percent)) => {
            if let Err(e) = process_directory(&path, override_flag, keep_flag, tolerance_percent) {
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

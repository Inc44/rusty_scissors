use clap::Parser;
use rusty_scissors::{process_images, DeltaMethod};
use std::path::PathBuf;
use std::process;
#[derive(Parser)]
#[command(
    version,
    about = "A command-line tool for trimming images.",
    long_about = "Rusty Scissors is a useful tool created with ❤️ using Rust. It quickly trims extra space around images like smart scissors. It's fast, efficient, and precise.",
    override_usage = "rusty_scissors <input_paths>... [options]"
)]
struct Cli {
    /// Paths to the input images or directories (required)
    #[arg(value_name = "input_paths", required = true)]
    input_paths: Vec<PathBuf>,
    /// Override the input image instead of creating a new one
    #[arg(short, long = "override")]
    override_flag: bool,
    /// Keep modification time
    #[arg(short, long = "keep")]
    keep_flag: bool,
    /// Set pixel similarity tolerance (default: 0)
    #[arg(
        short,
        long,
        value_name = "value",
        default_value_t = 0.0,
        hide_default_value = true
    )]
    tolerance: f32,
    /// Process alpha channel
    #[arg(short, long = "alpha", conflicts_with = "delta")]
    alpha_flag: bool,
    /// Use Delta E instead of RGB difference (E76 or E2000)
    #[arg(short, long = "delta", value_name = "method")]
    delta: Option<String>,
}
fn main() {
    let cli = Cli::parse();
    let delta_method = match cli.delta.as_deref() {
        Some("E76") => Some(DeltaMethod::E76),
        Some("E2000") => Some(DeltaMethod::E2000),
        Some(other) => {
            eprintln!("Invalid delta method: {}. Expected 'E76' or 'E2000'", other);
            process::exit(1);
        }
        _ => None,
    };
    let mut has_errors = false;
    for path in &cli.input_paths {
        if process_images(
            path,
            cli.override_flag,
            cli.keep_flag,
            cli.tolerance,
            cli.alpha_flag,
            delta_method,
        )
        .is_err()
        {
            has_errors = true;
        }
    }
    if has_errors {
        process::exit(1);
    }
}
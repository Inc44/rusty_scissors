use clap::Parser;
use rusty_scissors::process_images;
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
        long = "tolerance",
        default_value_t = 0.0,
        value_name = "percentage"
    )]
    tolerance_percent: f32,
}
fn main() {
    let cli = Cli::parse();
    let mut has_errors = false;
    for path in &cli.input_paths {
        if let Err(error) = process_images(
            path,
            cli.override_flag,
            cli.keep_flag,
            cli.tolerance_percent,
        ) {
            eprintln!("{}", error);
            has_errors = true;
        }
    }
    if has_errors {
        process::exit(1);
    }
}
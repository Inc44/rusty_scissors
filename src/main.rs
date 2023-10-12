use image::{DynamicImage, GenericImageView};
use rayon::prelude::*;
use std::env;
use std::path::Path;
use walkdir::WalkDir;

fn all_same(image: &DynamicImage, row: u32, col: u32, is_row: bool) -> bool {
    let value = image.get_pixel(col, row);
    if is_row {
        (0..image.width()).all(|x| image.get_pixel(x, row) == value)
    } else {
        (0..image.height()).all(|y| image.get_pixel(col, y) == value)
    }
}

fn process_image(input_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut img = image::open(input_path)?;

    let (mut left, mut right, mut top, mut bottom) = (0, img.width() - 1, 0, img.height() - 1);

    while all_same(&img, top, 0, true) {
        top += 1;
    }
    while all_same(&img, bottom, 0, true) {
        bottom -= 1;
    }
    while all_same(&img, 0, left, false) {
        left += 1;
    }
    while all_same(&img, 0, right, false) {
        right -= 1;
    }

    let sub_img = img.crop(left, top, right - left + 1, bottom - top + 1);

    let parent_dir = input_path.parent().unwrap_or_else(|| Path::new(""));
    let file_stem = input_path
        .file_stem()
        .ok_or("Failed to get file stem")?
        .to_string_lossy();
    let extension = input_path
        .extension()
        .ok_or("Failed to get file extension")?
        .to_string_lossy();

    let output_path = (0..)
        .map(|counter| {
            parent_dir.join(format!(
                "trim_{}{}{}",
                file_stem,
                if counter > 0 {
                    format!("_{}", counter)
                } else {
                    "".to_string()
                },
                if extension.is_empty() {
                    "".to_string()
                } else {
                    format!(".{}", extension)
                }
            ))
        })
        .find(|path| !path.exists())
        .ok_or("Failed to generate a unique output path")?;

    sub_img.save(&output_path)?;

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input-path>", args[0]);
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);

    if input_path.is_dir() {
        let paths: Vec<_> = WalkDir::new(input_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.path().to_owned())
            .collect();

        paths.par_iter().for_each(|path| {
            if let Err(e) = process_image(path) {
                eprintln!("Failed to process {}: {}", path.display(), e);
            }
        });
    } else if input_path.is_file() {
        if let Err(e) = process_image(input_path) {
            eprintln!("Failed to process {}: {}", input_path.display(), e);
        }
    } else {
        eprintln!("Invalid input path: {}", input_path.display());
        std::process::exit(1);
    }
}

use image::{DynamicImage, GenericImageView, Rgba};
use rayon::prelude::*;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::Path;
use std::time::SystemTime;
use walkdir::WalkDir;

#[derive(Debug)]
pub struct AppError {
    pub message: String,
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for AppError {}

pub fn all_same(
    image: &DynamicImage,
    row: u32,
    col: u32,
    is_row: bool,
    tolerance_percent: f32,
) -> bool {
    let value = image.get_pixel(col, row);
    let tolerance = (255.0 * tolerance_percent / 100.0) as u8; // Convert tolerance percentage to u8

    if is_row {
        (0..image.width()).all(|x| {
            let pixel = image.get_pixel(x, row);
            pixel_similarity(value, pixel, tolerance)
        })
    } else {
        (0..image.height()).all(|y| {
            let pixel = image.get_pixel(col, y);
            pixel_similarity(value, pixel, tolerance)
        })
    }
}

fn pixel_similarity(pixel1: Rgba<u8>, pixel2: Rgba<u8>, tolerance: u8) -> bool {
    (pixel1[0] as i16 - pixel2[0] as i16).abs() as u8 <= tolerance
        && (pixel1[1] as i16 - pixel2[1] as i16).abs() as u8 <= tolerance
        && (pixel1[2] as i16 - pixel2[2] as i16).abs() as u8 <= tolerance
}

pub fn process_image(
    input_path: &Path,
    override_flag: bool,
    keep_flag: bool,
    tolerance_percent: f32,
) -> Result<(), Box<dyn Error>> {
    let mut img = image::open(input_path)?;

    let mut modified_date = SystemTime::now();

    if keep_flag {
        let metadata = fs::metadata(input_path)?;
        modified_date = metadata.modified()?;
    }

    let (mut left, mut right, mut top, mut bottom) = (0, img.width() - 1, 0, img.height() - 1);

    while all_same(&img, top, 0, true, tolerance_percent) {
        top += 1;
    }
    while all_same(&img, bottom, 0, true, tolerance_percent) {
        bottom -= 1;
    }
    while all_same(&img, 0, left, false, tolerance_percent) {
        left += 1;
    }
    while all_same(&img, 0, right, false, tolerance_percent) {
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

    let output_path = if override_flag {
        input_path.to_path_buf()
    } else {
        (0..)
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
            .ok_or("Failed to generate a unique output path")?
    };

    sub_img.save(&output_path)?;

    if keep_flag {
        filetime::set_file_times(&output_path, modified_date.into(), modified_date.into())?;
    }

    Ok(())
}

pub fn process_directory(
    input_path: &Path,
    override_flag: bool,
    keep_flag: bool,
    tolerance_percent: f32,
) -> Result<(), AppError> {
    if input_path.is_dir() {
        let paths: Vec<_> = WalkDir::new(input_path)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.path().is_file())
            .map(|e| e.path().to_owned())
            .collect();
        paths.par_iter().for_each(|path| {
            if let Err(e) = process_image(path, override_flag, keep_flag, tolerance_percent) {
                eprintln!("Failed to process {}: {}", path.display(), e);
            }
        });
    } else if input_path.is_file() {
        if let Err(e) = process_image(input_path, override_flag, keep_flag, tolerance_percent) {
            eprintln!("Failed to process {}: {}", input_path.display(), e);
        }
    } else {
        return Err(AppError {
            message: format!("Invalid input path: {}", input_path.display()),
        });
    }
    Ok(())
}

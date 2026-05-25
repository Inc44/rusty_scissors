use image::{DynamicImage, GenericImageView, Rgba};
use rayon::prelude::*;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use walkdir::WalkDir;
fn all_same(
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
        && (pixel1[3] as i16 - pixel2[3] as i16).abs() as u8 <= tolerance
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
    while top <= bottom && all_same(&img, top, 0, true, tolerance_percent) {
        top += 1;
    }
    while bottom >= top && all_same(&img, bottom, 0, true, tolerance_percent) {
        bottom -= 1;
    }
    while left <= right && all_same(&img, 0, left, false, tolerance_percent) {
        left += 1;
    }
    while right >= left && all_same(&img, 0, right, false, tolerance_percent) {
        right -= 1;
    }
    let output_path = build_output_path(input_path, override_flag)?;
    if left > right || top > bottom {
        fs::copy(input_path, &output_path)?;
    } else {
        let sub_img = img.crop(left, top, right - left + 1, bottom - top + 1);
        sub_img.save(&output_path)?;
    }
    if keep_flag {
        filetime::set_file_times(&output_path, modified_date.into(), modified_date.into())?;
    }
    Ok(())
}
fn build_output_path(input_path: &Path, override_flag: bool) -> Result<PathBuf, Box<dyn Error>> {
    if override_flag {
        return Ok(input_path.to_path_buf());
    }
    let file_stem = input_path
        .file_stem()
        .ok_or("Failed to get file stem")?
        .to_string_lossy();
    let extension = input_path
        .extension()
        .ok_or("Failed to get file extension")?
        .to_string_lossy();
    let parent_dir = input_path.parent().unwrap_or_else(|| Path::new(""));
    let output_path = (0..)
        .map(|counter| {
            parent_dir.join(format!(
                "trimmed_{}{}{}",
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
    Ok(output_path)
}
fn collect_paths(input_path: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    if !input_path.is_dir() {
        return Err(format!("Invalid input path: {}", input_path.display()).into());
    }
    let paths = WalkDir::new(input_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .map(|entry| entry.path().to_owned())
        .collect();
    Ok(paths)
}
pub fn process_images(
    input_path: &Path,
    override_flag: bool,
    keep_flag: bool,
    tolerance_percent: f32,
) -> Result<(), Box<dyn Error>> {
    if input_path.is_file() {
        if let Err(error) = process_image(input_path, override_flag, keep_flag, tolerance_percent) {
            eprintln!("Failed to process {}: {}", input_path.display(), error);
        }
        return Ok(());
    }
    let paths = collect_paths(input_path)?;
    paths.par_iter().for_each(|path| {
        if let Err(error) = process_image(path, override_flag, keep_flag, tolerance_percent) {
            eprintln!("Failed to process {}: {}", path.display(), error);
        }
    });
    Ok(())
}
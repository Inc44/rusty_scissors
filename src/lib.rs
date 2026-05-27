use clap::ValueEnum;
use image::{DynamicImage, GenericImageView, Rgba};
use palette::color_difference::{Ciede2000, EuclideanDistance};
use palette::{IntoColor, Lab, Srgb};
use rayon::prelude::*;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::SystemTime;
use walkdir::WalkDir;
#[derive(Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum DeltaMethod {
    #[value(name = "E76")]
    E76,
    #[value(name = "E2000")]
    E2000,
}
fn to_lab(pixel: Rgba<u8>) -> Lab {
    Srgb::new(pixel[0], pixel[1], pixel[2])
        .into_format::<f32>()
        .into_color()
}
fn pixel_similarity(
    pixel1: Rgba<u8>,
    pixel2: Rgba<u8>,
    tolerance: f32,
    alpha_flag: bool,
    delta: Option<DeltaMethod>,
) -> bool {
    match delta {
        Some(DeltaMethod::E76) => to_lab(pixel1).distance(to_lab(pixel2)) <= tolerance,
        Some(DeltaMethod::E2000) => to_lab(pixel1).difference(to_lab(pixel2)) <= tolerance,
        _ => {
            let r_diff = (pixel1[0] as f32 - pixel2[0] as f32).abs() <= tolerance;
            let g_diff = (pixel1[1] as f32 - pixel2[1] as f32).abs() <= tolerance;
            let b_diff = (pixel1[2] as f32 - pixel2[2] as f32).abs() <= tolerance;
            let a_diff = !alpha_flag || (pixel1[3] as f32 - pixel2[3] as f32).abs() <= tolerance;
            r_diff && g_diff && b_diff && a_diff
        }
    }
}
fn row_is_uniform(
    image: &DynamicImage,
    row: u32,
    tolerance: f32,
    alpha_flag: bool,
    delta: Option<DeltaMethod>,
) -> bool {
    let reference_pixel = image.get_pixel(0, row);
    (0..image.width()).all(|x| {
        let pixel = image.get_pixel(x, row);
        pixel_similarity(reference_pixel, pixel, tolerance, alpha_flag, delta)
    })
}
fn column_is_uniform(
    image: &DynamicImage,
    column: u32,
    tolerance: f32,
    alpha_flag: bool,
    delta: Option<DeltaMethod>,
) -> bool {
    let reference_pixel = image.get_pixel(column, 0);
    (0..image.height()).all(|y| {
        let pixel = image.get_pixel(column, y);
        pixel_similarity(reference_pixel, pixel, tolerance, alpha_flag, delta)
    })
}
fn calculate_trimmed_bounds(
    image: &DynamicImage,
    tolerance: f32,
    alpha_flag: bool,
    delta: Option<DeltaMethod>,
) -> (u32, u32, u32, u32) {
    let (mut left, mut right, mut top, mut bottom) = (0, image.width() - 1, 0, image.height() - 1);
    while top <= bottom && row_is_uniform(image, top, tolerance, alpha_flag, delta) {
        top += 1;
    }
    while bottom >= top && row_is_uniform(image, bottom, tolerance, alpha_flag, delta) {
        bottom -= 1;
    }
    while left <= right && column_is_uniform(image, left, tolerance, alpha_flag, delta) {
        left += 1;
    }
    while right >= left && column_is_uniform(image, right, tolerance, alpha_flag, delta) {
        right -= 1;
    }
    (left, right, top, bottom)
}
fn format_file_name(file_stem: &str, extension: &str, counter: u32) -> String {
    let counter_str = if counter > 0 {
        format!("_{:03}", counter)
    } else {
        String::new()
    };
    let extension_str = if extension.is_empty() {
        String::new()
    } else {
        format!(".{}", extension)
    };
    format!("trimmed_{}{}{}", file_stem, counter_str, extension_str)
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
    let output_path = (0..1000)
        .map(|counter| {
            let file_name = format_file_name(&file_stem, &extension, counter);
            parent_dir.join(file_name)
        })
        .find(|path| !path.exists())
        .ok_or("Failed to generate a unique output path")?;
    Ok(output_path)
}
pub fn process_image(
    input_path: &Path,
    override_flag: bool,
    keep_flag: bool,
    tolerance: f32,
    alpha_flag: bool,
    delta: Option<DeltaMethod>,
) -> Result<(), Box<dyn Error>> {
    let mut image = image::open(input_path)?;
    if image.width() == 0 || image.height() == 0 {
        return Ok(());
    }
    let mut modified_date = SystemTime::now();
    if keep_flag {
        let metadata = fs::metadata(input_path)?;
        modified_date = metadata.modified()?;
    }
    let (left, right, top, bottom) = calculate_trimmed_bounds(&image, tolerance, alpha_flag, delta);
    let output_path = build_output_path(input_path, override_flag)?;
    if left > right || top > bottom {
        fs::copy(input_path, &output_path)?;
    } else {
        let trimmed_image = image.crop(left, top, right - left + 1, bottom - top + 1);
        trimmed_image.save(&output_path)?;
    }
    if keep_flag {
        filetime::set_file_times(&output_path, modified_date.into(), modified_date.into())?;
    }
    Ok(())
}
fn collect_paths(input_path: &Path) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    if !input_path.is_dir() {
        eprintln!("Invalid input path: {}", input_path.display());
        return Err("".into());
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
    tolerance: f32,
    alpha_flag: bool,
    delta: Option<DeltaMethod>,
) -> Result<(), Box<dyn Error>> {
    if input_path.is_file() {
        if let Err(error) = process_image(
            input_path,
            override_flag,
            keep_flag,
            tolerance,
            alpha_flag,
            delta,
        ) {
            eprintln!("Failed to process {}: {}", input_path.display(), error);
            return Err("".into());
        }
        return Ok(());
    }
    let paths = collect_paths(input_path)?;
    let has_errors = AtomicBool::new(false);
    paths.par_iter().for_each(|path| {
        if let Err(error) =
            process_image(path, override_flag, keep_flag, tolerance, alpha_flag, delta)
        {
            eprintln!("Failed to process {}: {}", path.display(), error);
            has_errors.store(true, Ordering::Relaxed);
        }
    });
    if has_errors.load(Ordering::Relaxed) {
        return Err("".into());
    }
    Ok(())
}
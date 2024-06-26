# 🖼️ Rusty Scissors ✂️

## Introduction
Rusty Scissors is a useful tool created with ❤️ using Rust. It quickly trims extra space around images like smart scissors. It's fast, efficient, and precise.

## Features 🌟
- 🚀 Fast image trimming powered by [Rayon](https://crates.io/crates/rayon).
- 🖼️ Supports many image formats (JPEG, PNG, BMP, GIF, ICO, QOI, and more).
- 🗂️ Batch process multiple images at once.
- 📁 Organized output to keep trimmed images together.
- 🛠️ Easy-to-use command-line interface.

## How to Use 💼

If you are on Windows, simply download the .exe file. For all other operating systems, refer to the "Build the Project" section for compilation instructions.

## Build the Project 🚀

1. Clone the repository:
```bash
$ git clone https://github.com/Inc44/rusty-scissors.git
```
2. Go to the project directory:
```bash
$ cd rusty-scissors
```
3. Build and run the project:
```bash
$ cargo run --release <input-path>
```

## How Does It Work? 🔎

Rusty Scissors scans each row and column of pixels, checking for similarities within a specified tolerance.

If you're processing noisy images with slightly varying pixel values, you can set a tolerance value of `13.725` to account for this noise. If the default behavior (a tolerance value of `0`) works well for your images, there's no need to adjust the tolerance.

Currently, Rusty Scissors does not use the Delta E 2000 metric for color similarity, as it would significantly impact performance due to the complex calculations involved. Instead, a simpler pixel difference approach is used to prioritize speed.

## Contribution 🤝
We welcome contributions! For significant changes, please open an issue for discussion before making a pull request.

## License 📜
This software is licensed under the MIT  Massachusetts Institute of Technology (MIT). For more details, refer to [LICENSE](LICENSE.md).
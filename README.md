# 🖼️ Rusty Scissors ✂️

Rusty Scissors is a nifty little tool built with ❤️ in Rust, designed to trim the extra space around images in a jiffy, like a pair of smart scissors. It's fast, efficient, and does the job with precision! 🎯

![Rusty Scissors](screenshot.png)

## Features 🌟
- 🚀 Super fast image trimming thanks to the power of [Rayon](https://crates.io/crates/rayon)
- 🖼 Supports a variety of image formats (JPEG, PNG, BMP, GIF, ICO, QOI, and many more)
- 🗂 Batch processing for handling multiple images in one go
- 📁 Organized output to keep your trimmed images neatly together
- 🛠 Easy to use with straightforward command-line interface

## Usage 💼

Compile and run the program with Rust. Use the following command to trim your images:

```bash
$ cargo run --release -- <input-path>
```

Replace `<input-path>` with the path to the image file or a directory containing image files you want to trim.

## Example 🎬

```bash
$ cargo run --release -- ./images
```

This will process all the images in the `./images` directory, trimming the extra space around them.

## Getting Started 🚀

1. Clone the repository:

```bash
$ git clone https://github.com/Inc44/rusty-scissors.git
```

2. Navigate to the project directory:

```bash
$ cd rusty-scissors
```

3. Build and run the project:

```bash
$ cargo run --release -- <input-path>
```

## 🤝 Contribution

Contributions are heartily welcomed! If you're considering significant modifications, please initiate an issue for discussions before submitting a pull request.

## 📜 License

This software is under the GNU General Public License v3.0 (GPL-3.0). For comprehensive details, refer to [LICENSE.md](LICENSE.md).
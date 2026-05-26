# 🖼️ Rusty Scissors ✂️

![Stars](https://img.shields.io/github/stars/Inc44/rusty_scissors?style=social)
![Forks](https://img.shields.io/github/forks/Inc44/rusty_scissors?style=social)
![Watchers](https://img.shields.io/github/watchers/Inc44/rusty_scissors?style=social)
![Repo Size](https://img.shields.io/github/repo-size/Inc44/rusty_scissors)
![Language Count](https://img.shields.io/github/languages/count/Inc44/rusty_scissors)
![Top Language](https://img.shields.io/github/languages/top/Inc44/rusty_scissors)
[![Issues](https://img.shields.io/github/issues/Inc44/rusty_scissors)](https://github.com/Inc44/rusty_scissors/issues?q=is%3Aopen+is%3Aissue)
![Last Commit](https://img.shields.io/github/last-commit/Inc44/rusty_scissors?color=red)
[![Release](https://img.shields.io/github/release/Inc44/rusty_scissors.svg)](https://github.com/Inc44/rusty_scissors/releases)
[![Sponsor](https://img.shields.io/static/v1?label=Sponsor&message=%E2%9D%A4&logo=GitHub&color=%23fe8e86)](https://github.com/sponsors/Inc44)

Rusty Scissors is a useful tool created with ❤️ using Rust. It quickly trims extra space around images like smart scissors. It's fast, efficient, and precise.

## ⚙️ Features

- 🚀 Fast image trimming powered by [Rayon](https://crates.io/crates/rayon).
- 🖼️ Supports many image formats (BMP, GIF, JPEG, PNG, QOI, TGA, and WEBP).
- 🗂️ Batch process multiple images at once.
- 📁 Organized output to keep trimmed images together.
- 🛠️ Easy-to-use command-line interface.

## ⚠️ Disclaimers

Rusty Scissors scans each row and column of pixels, checking for similarities within a specified tolerance.

If you're processing noisy images with slightly varying pixel values, you can set a tolerance value of `35` to account for this noise. If the default behavior (a tolerance value of `0`) works well for your images, there's no need to adjust the tolerance.

When using the `--delta` flag, you may have to adjust the tolerance to `2`. Using `--delta` with default tolerance may be considered a waste of resources, as the Delta E 2000 metric for color similarity would significantly impact performance due to the complex calculations involved. Instead, a simpler pixel difference approach should be used to prioritize speed.

Due to the algorithm's edge-inward scanning design, some images may not be trimmed at all even when trimming is visually possible. For example, images that consist of bars of different colors.

If an image is completely uniform and trimmed down to nothing, the original file is copied to the output path to prevent data loss and avoid creating invalid empty images.

Input images with zero width or height (0x0) are skipped entirely, as according to image standards supported by this tool, they are not valid images by definition. They are not copied and the `--keep` option does not apply to them since no file operations are performed.

## 🚀 Installation from crates.io

```bash
cargo install rusty_scissors
```

## 🛠️ Build from Source

```bash
git clone https://github.com/Inc44/rusty_scissors.git
cd rusty_scissors
cargo build --release
```

## 📦 Publish

```bash
cargo publish
```

## 📖 Usage Example

```bash
cargo run --release <input_paths>... [options]
```

Or

```bash
rusty_scissors <input_paths>... [options]
```

## 🎨 Command-Line Arguments

| Argument                  | Description                                            |
|---------------------------|--------------------------------------------------------|
| `<input_paths>`           | Paths to the input images or directories (required)    |
| `-o, --override`          | Override the input image instead of creating a new one |
| `-k, --keep`              | Keep modification time                                 |
| `-t, --tolerance <value>` | Set pixel similarity tolerance (default: 0)            |
| `-a, --alpha`             | Process alpha channel                                  |
| `-d, --delta <method>`    | Use Delta E instead of RGB difference (E76 or E2000)   |

## 🐛 Bugs

Not yet found.

## ⛔ Known Limitations

Not yet known.

## 🙏 Thanks

Creators of:

- [Rust](https://www.rust-lang.org)
- [clap](https://github.com/clap-rs/clap)
- [filetime](https://github.com/alexcrichton/filetime)
- [Image](https://github.com/image-rs/image)
- [palette](https://github.com/Ogeon/palette)
- [Rayon](https://github.com/rayon-rs/rayon)
- [walkdir](https://github.com/BurntSushi/walkdir)

## 🤝 Contribution

Contributions, suggestions, and new ideas are heartily welcomed. If you're considering significant modifications, please initiate an issue for discussion before submitting a pull request.

## 📜 License

[![MIT](https://img.shields.io/badge/License-MIT-lightgrey.svg)](https://opensource.org/licenses/MIT)

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## 💖 Support

[![BuyMeACoffee](https://img.shields.io/badge/Buy%20Me%20a%20Coffee-ffdd00?style=for-the-badge&logo=buy-me-a-coffee&logoColor=black)](https://buymeacoffee.com/xamituchido)
[![Ko-Fi](https://img.shields.io/badge/Ko--fi-F16061?style=for-the-badge&logo=ko-fi&logoColor=white)](https://ko-fi.com/inc44)
[![Patreon](https://img.shields.io/badge/Patreon-F96854?style=for-the-badge&logo=patreon&logoColor=white)](https://www.patreon.com/Inc44)
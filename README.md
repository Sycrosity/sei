`SEI`
==================
<!-- ![CI](https://github.com/sycrosity/sei/actions/workflows/ci.yml/badge.svg) -->

A simple parser for .SEI (stacking ePaper image) files.
-------
**NOTE - this project is in a VERY early stage, with .sei files being extremely new.**

## Compat

| crate version | sei version | 
| --- | --- |
| 0.0.* | 0 |

---

<!-- ## Download & Test

#### From source

1. Install rust at [rustup.rs](https://rustup.rs)
2. Install espup at [esp-rs/espup](https://github.com/esp-rs/espup)
3. Clone the repo `git clone https://github.com/Sycrosity/sei.git`
4. `cd sei`
5. Build with `cargo ` -->


## File format

offset | length | field | description |
| -------- | ------- | ------- | ------- |
0 | 2 | magic bytes | 'S' 'E' |
2 | 1 | version | format version |
3 | 1 | offset | offset where the image data starts - allows for header size to increase with latter versions |
4 | 2 | width | image width in px |
6 | 2 | height | image height in px |
8 | 1 | settings | a more complicated bitwise type (described below) |
9 | 1 | z index | layer order for stacking - higher values appear above lower ones
10 | x | image data | raw pixel data |

---

### Settings format

bit(s) | name | description |
| -------- | ------- | ------- |
0-1 | bit depth | 00 = 1-bit, 01 = 2-bit | 10 = 4 bit | 11 = not used |
2 | white mode | 0 (all 0 bits is white, all 1 bits is black), 1 (all 1 bits is white, all 0 bits is black) |
3 | padding | 0 - padding is enabled (all rows are padded up to the nearest byte), 1 - padding is disabled |
4-5 | stacking mode | 00 = full colour (pixels are coloured, and overwrite pixels they are on top of), 01 (white is transparent), 10 (black is transparent), 11 = unused |
6-7 | reserved | reserved for later versions |

-------

## Contributing

Any and all contributions are welcome! Pull requests are checked for `cargo test`, `cargo clippy` and `cargo fmt`.

Before submitting a PR or issue, please run the following commands and follow their instructions:
1. `cargo clippy`
2. `cargo fmt`

-------

## License

The source code for this project is licensed under either of:

 - Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
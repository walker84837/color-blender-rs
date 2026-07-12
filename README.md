# color-blender-rs

A color blender, written in Rust. The color blender takes two hexadecimal colors as input, along with the number of midpoints to generate between those two colors.

![License](https://img.shields.io/badge/license-0BSD-blue.svg)
![Project Status](https://img.shields.io/badge/status-Not%20actively%20developed-lightgrey.svg)
[![No Maintenance Intended](https://unmaintained.tech/badge.svg)](https://unmaintained.tech/)

## Table of Contents

1. [Installation](#installation)
2. [Usage](#usage)
3. [Contributing](#contributing)
4. [License](#license)
5. [Project status](#project-status)

## Installation

To build this application from source, you need [rustup](https://rustup.rs/) to use Cargo. Otherwise, you can download the precompiled binaries.

Once you have successfully installed Rust and Cargo, you can proceed to build and install this program by executing the following command:

```console
$ cargo build --release
```

## Usage

```console
$ ./color-blender-rs <first-color> <second-color> [-m <midpoints> -o <output-file> --benchmark>]
```

  - `<start color>`: The starting hexadecimal color (e.g., "\#ff0000" for red).
  - `<end color>`: The ending hexadecimal color (e.g., "\#00ff00" for green).
  - `<midpoints>`: The number of midpoints to generate between the start and end colors. The default is 10.
  - `<output-file>`: The output file to write the blended colors. (default: print colors to console).
  - `<benchmark>`: Gets the time it takes to blend the colors in microseconds.

### Examples

Blend 5 midpoints between red (\#ff0000) and green (\#00ff00) and display the results in the console:

```console
$ ./color-blender-rs "#ff0000" "#00ff00" 5
```

Blend 10 midpoints between blue (\#0000ff) and yellow (\#ffff00) and write the output to default (output.txt):

```console
$ ./color-blender-rs "#0000ff" "#ffff00" 10 -o output.txt
```

## Contributing

Contributions to the `color-blender-rs` project are always welcome\! If you want to contribute:
  - Follow the same code style. Format your code using:
  ```console
  $ cargo fmt
  ```
  - Follow the code of conduct (see [CODE_OF_CONDUCT.md](CODE_OF_CONDUCT.md)).
  - For big changes (e.g., a new feature), open an issue, and describe the following points:
    - Why should it be added?
    - What's the difference between using it and not using it in practice? What does it enable?

If you need help or guidance with this project, open an issue.

## License

This project is licensed under the [Zero-Clause BSD License](LICENSE).

## Project status

I think this project has achieved its goal, so it isn't being actively developed, especially when it comes to adding new features. However, contributions are still welcome.

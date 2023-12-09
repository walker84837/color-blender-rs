# color-blender-rs

A color blender, written in Rust. The color blender takes two hexadecimal colors as input, along with the number of midpoints to generate between those two colors.

![License](https://img.shields.io/badge/license-GPLv3-blue.svg)
![Project Status](https://img.shields.io/badge/status-Not%20actively%20developed-lightgrey.svg)
![Contributions Welcome](https://img.shields.io/badge/contributions-welcome-brightgreen.svg)

## Table of Contents

1. [Installation](#installation)
2. [Usage](#usage)
3. [Contributing](#contributing)
4. [License](#license)
5. [Project status](#project-status)

## Installation

To build this application from source, you need [rustup](https://rustup.rs/) to use Cargo. Otherwise, you can download the precompile binaries.

Once you have successfully installed Rust and Cargo, you can proceed to build and install this program by executing the following command:

```console
$ cargo build --release
```

## Usage

```console
$ ./color-blender-rs <first-color> <second-color> [-m <midpoints> --write <should-write> -o <output-file>]
```

  - `<start color>`: The starting hexadecimal color (e.g., "\#ff0000" for red).
  - `<end color>`: The ending hexadecimal color (e.g., "\#00ff00" for green).
  - `<midpoints>`: The number of midpoints to generate between the start and end colors. The default is 10.
  - `<should-write>`: Specify if you want to write the blended colors to a file. If you won't, it will display the colors in the console.
  - `<output-file>`: Self-explanatory.

### Examples

Blend 5 midpoints between red (\#ff0000) and green (\#00ff00) and display the results in the console:

```console
$ ./color-blender-rs "#ff0000" "#00ff00" 5
```

Blend 10 midpoints between blue (\#0000ff) and yellow (\#ffff00) and write the output to default (output.txt):

```console
$ ./color-blender-rs "#0000ff" "#ffff00" 10 --write
```

## Contributing

Contributions to the `color-blender-rs` project are always welcome\! If you want to contribute:
  - Follow the same code style. Format your code using:
  ```console
  $ rustfmt --edition 2021 src/*
  ```
  - Obviously, follow the [code of conduct](CODE_OF_CONDUCT.md).
  - I recommend using Rust stable rather than Rust nightly, for compatibility purposes.
  If you notice my code has Rust nightly features, feel free to replace them with stable equivalents.
  - If you have to use an external library, please use lightweight ones (eg. `ureq` over `reqwest`, `async-std` over `tokio`)
  - Prefer using the standard library over reinventing the wheel.
  - For big changes (let's take a new feature), open an issue, and describe the following points:
    - Why should it be added? What does it add and why should it even be considered?
    - What's the difference between using it and not using it?

If you need help or guidance with this project, open an issue.

## License

This project is licensed under the [GNU General Public License, version 3](LICENSE.md).

## Project status

Development is not particularly active as [I](https://github.com/walker84837) work on it when I have time. Contributions are welcome.
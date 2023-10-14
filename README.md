# color-blender-rs

This is a color blender written in Rust. The color blender takes two hexadecimal colors as input, along with the number of midpoints to generate between those two colors.

## Table of Contents

1. [Installation](#installation)
2. [How to Use](#how-to-use)
3. [Contributing](#contributing)
4. [License](#license)
5. [Example](#example)

## Installation

To utilize this application, it is necessary to have Rust and its package manager, Cargo, installed. You can install them by following the official [Rust installation guide](https://www.rust-lang.org/tools/install).

Once you have successfully installed Rust and Cargo, you can proceed to build and install this program by executing the following command:

```bash
cargo install --path .
```

## How to Use

To use the color blender, you need to provide the following command-line arguments:

```./color-blender-rs <first-color> <second-color> <midpoints> --write <should-write>```

  - `<start color>`: The starting hexadecimal color (e.g., "\#FF0000" for red).
  - `<end color>`: The ending hexadecimal color (e.g., "\#00FF00" for green).
  - `<midpoints>`: The number of midpoints to generate between the start and end colors. Must be a positive integer.
  - `write <should-write>`: Specify "yes" if you want to write the blended colors to "output.txt", or "no" to display the colors in the console.

## Example

Blend 5 midpoints between red (\#FF0000) and green (\#00FF00) and display the results in the console:

```./color-blender-rs "#FF0000" "#00FF00" 5 no```

Blend 10 midpoints between blue (\#0000FF) and yellow (\#FFFF00) and write the output to a file:

```./color-blender-rs "#0000FF" "#FFFF00" 10 yes```

## Contributing

Contributions to the `color-blender-rs` project are always welcome\! If you want to contribute, follow these steps:

1.  Fork the repository.
2.  Clone the forked repository: ```git clone https://github.com/your-username/color-blender-rs.git```
3.  Make your changes and improvements and commit them.
4.  Create a pull request with a detailed description of your changes.

## License

This project is licensed under the GNU GPLv3. You can find the full text of the license in the [LICENSE](LICENSE) file or visit the [GNU website](https://www.gnu.org/licenses/gpl-3.0.html).


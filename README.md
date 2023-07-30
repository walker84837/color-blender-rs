# color-blender-rs

This is a simple color blender written in Rust. The color blender takes two hexadecimal colors as input, along with the number of midpoints to generate between those two colors. It then blends the colors and produces a palette of colors that smoothly transition from the start color to the end color through the specified number of midpoints.

## Features

- Blends two hexadecimal colors with a specified number of midpoints.
- Generates a smooth color palette between the start and end colors.
- Supports writing the blended colors to a file.

## How to Use

To use the color blender, you need to provide the following command-line arguments:

  ```$ ./color-blender-rs <start color> <end color> <midpoints> <write to file? yes|no>```

- `<start color>`: The starting hexadecimal color (e.g., "#FF0000" for red).
- `<end color>`: The ending hexadecimal color (e.g., "#00FF00" for green).
- `<midpoints>`: The number of midpoints to generate between the start and end colors. Must be a positive integer.
- `<write to file? yes|no>`: Specify "yes" if you want to write the blended colors to a file named "output.txt", or "no" to display the colors in the console.

## Example

Blend 5 midpoints between red (#FF0000) and green (#00FF00) and display the results in the console:

  ```$ ./color-blender-rs "#FF0000" "#00FF00" 5 no```

Blend 10 midpoints between blue (#0000FF) and yellow (#FFFF00) and write the output to a file:

  ```$ ./color-blender-rs "#0000FF" "#FFFF00" 10 yes```

## Contributing

Contributions to the `color-blender-rs` project are always welcome! If you want to contribute, follow these steps:

1. Clone the repository: `git clone https://github.com/walker84837/color-blender-rs.git`
2. Make your changes and improvements.
3. Create a pull request with a detailed description of your changes.

## License

This project is licensed under the GNU GPLv3. You can find the full text of the license in the [LICENSE](LICENSE) file or visit the [GNU GPL website](https://www.gnu.org/licenses/gpl-3.0.html).

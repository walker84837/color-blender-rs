# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/),
and this project adheres to [Semantic Versioning](https://semver.org/).

## \[Unreleased\]

### Added

  - Color converting to HSL, LCh, and RGB.
  - TOML configuration (optional):
    - Color distance
    - Benchmark
    - Midpoints

## \[0.3.0\] - 2023-12-24

### Added

  - Added the option to benchmark the color blending.
  - Added the possibility to calculate the difference between two hexadecimal
    colors.

### Changed

  - Updated dependencies in `Cargo.lock` (`anyhow`, `libc`, `proc-macro2`, `rayon`).
  - The GitHub Actions workflows now check, test and build on Windows and Linux.
  - Bumped version in `Cargo.toml` (0.2.0 -\> 0.3.0).

### Removed

  - Removed the `-w, --write` flag, as the `-o, --output` flag is more concise.

## \[0.2.0\] - 2023-12-09

### Fixed

  - Removed trailing empty line in [code of conduct](CODE_OF_CONDUCT.md) file.
  - Updated various sections in the [readme](README.md) file, and added project status section:
    - Installation
    - Usage & examples
    - Contributing

### Changed

  - Switched error handling to `anyhow`.
  - Refactored the majority of the code:
    - Simplified `Blending` struct and its `write` and `print` functions in [main.rs](src/main.rs).
    - Improved readability when calling the `blend_colors` function in in [color_blender.rs](src/color_blender.rs).

## \[0.1.0\] - 2023-11-08

### Changed

  - Made the midpoints and "should write" bools optional.
  - Changed command-line argument option for midpoints.

## \[0.0.2\] - 2023-10-14

### Added

  - Added a CHANGELOG.md, CODE\_OF\_CONDUCT.md file.
  - Command-line arguments are now handled by StructOpt.

### Fixed

  - Renamed the LICENSE file to LICENSE.md for formatting purposes.
  - Made the README.md simpler to read.

## \[0.0.1\] - 2023-07-30

### Added

  - Initial release of `color-blender-rs`.

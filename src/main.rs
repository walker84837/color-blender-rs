use std::env;
use std::fs::File;
use std::io::Write;
use std::process;
use std::str::FromStr;

mod color_blender;

fn main() {
    let args: Vec<String> = env::args().collect();
    let blender = color_blender::ColorBlender::new();

    let midpoints: usize;
    let should_write: bool;
    let start_color: String;
    let end_color: String;
    let should_write_inp: String;

    if args.get(1).is_none() {
        eprintln!("No arguments are provided.");
        process::exit(1);
    } else if args[1] == "--help" || args[1] == "-h" {
        println!(
            "{} <first color> <second color> <midpoints> <write to file? yes|no>",
            args[0]
        );
        println!("Color blender - Blends 2 hex colors with a user-given amount of midpoints.");
        process::exit(0);
    } else if args.len() == 5 {
        start_color = args[1].clone();
        end_color = args[2].clone();
        midpoints = usize::from_str(&args[3]).unwrap_or_else(|_| {
            eprintln!("Invalid input for midpoints. Expected a positive integer.");
            process::exit(1);
        });
        should_write_inp = args[4].clone();
    } else {
        eprintln!("Invalid input. Try again.");
        process::exit(1);
    }

    if &should_write_inp.trim().to_lowercase() != "yes" {
        should_write = false;
    } else {
        should_write = true;
    }

    let blended_colors = blender.blend_colors(&start_color, &end_color, midpoints);

    if !should_write {
        for color in &blended_colors {
            println!("{}", color);
        }
    } else {
        let mut output = File::create("output.txt").unwrap_or_else(|_| {
            eprintln!("Error creating the file 'output.txt' for writing.");
            process::exit(1);
        });

        for color in &blended_colors {
            writeln!(output, "{}", color).unwrap_or_else(|_| {
                eprintln!("Error writing to the file 'output.txt'.");
                process::exit(1);
            });
        }

        println!("File 'output.txt' written successfully.");
    }
}

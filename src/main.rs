use std::{fs::File, io::Write, process};

use structopt::StructOpt;

mod color_blender;

#[derive(StructOpt, Debug)]
struct Opt {
    /// First color (hex format)
    first_color: String,

    /// Second color (hex format)
    second_color: String,

    /// Number of midpoints
    midpoints: usize,

    /// Write to a file (yes|no)
    #[structopt(long = "write")]
    should_write: String,
}

fn main() {
    let opt = Opt::from_args();
    let blender = color_blender::ColorBlender::new();

    if opt.should_write.trim().to_lowercase() != "yes" {
        blend_and_print(&opt, &blender);
    } else {
        blend_and_write(&opt, &blender);
    }
}

fn blend_and_print(opt: &Opt, blender: &color_blender::ColorBlender) {
    let blended_colors = blender.blend_colors(&opt.first_color, &opt.second_color, opt.midpoints);

    for color in &blended_colors {
        println!("{}", color);
    }
}

fn blend_and_write(opt: &Opt, blender: &color_blender::ColorBlender) {
    let blended_colors = blender.blend_colors(&opt.first_color, &opt.second_color, opt.midpoints);

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

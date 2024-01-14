use anyhow::{Context, Result};
use clap::Parser;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
    time::Instant,
};

pub struct Blending;
mod color_blender;
use color_blender::{ColorBlender, ColorConverter};

#[derive(Parser)]
#[structopt(name = "color-blender-rs", about = "A color blender, written in Rust.")]
struct Opt {
    #[arg(help = "The first color in hex format")]
    first_color: String,

    #[arg(help = "Second color in hex format")]
    second_color: String,

    #[arg(short, long, default_value = "10", help = "Number of midpoints")]
    midpoints: usize,

    #[arg(short, long, help = "Output file path")]
    output: Option<PathBuf>,

    #[arg(short, long, help = "Calculates the sRGB distance between two colors")]
    distance: bool,

    #[arg(short, long, help = "Prints the time it took to blend colors")]
    benchmark: bool,
}


fn main() -> Result<()> {
    let opt = Opt::parse();

    let first_color = opt.first_color.to_string();
    let second_color = opt.second_color.to_string();

    let blender = ColorBlender::new(first_color, second_color, opt.midpoints);

    if opt.benchmark {
        let start_time = Instant::now();
        let colors: Vec<String> = blender.blend_colors();
        let end_time = Instant::now();

        let elapsed_time = end_time - start_time;
        let avg_time_per_iteration = (elapsed_time / opt.midpoints as u32).as_nanos();

        for color in &colors {
            println!("{}", color);
        }

        println!("Elapsed time: {}μs", elapsed_time.as_micros());
        println!("Average time per iteration: {}ns", avg_time_per_iteration);
        return Ok(());
    }

    if opt.distance {
        let firstcolors = ColorConverter::hex_to_rgb(&opt.first_color)?;
        let lastcolors = ColorConverter::hex_to_rgb(&opt.second_color)?;

        let (r, g, b) = firstcolors;
        let first_colors = (r as f32, g as f32, b as f32);

        let (r, g, b) = lastcolors;
        let last_colors = (r as f32, g as f32, b as f32);

        let distance = color_difference(first_colors, last_colors);

        println!("Distance: {distance}");

        if distance == 0.0 {
            println!("The colors are identical.");
        } else if 0.0 < distance && distance < 0.1 {
            println!("There is a difference in color, but it's not noticeable.");
        } else if 0.1 < distance && distance < 0.5 {
            println!("There is a noticeable but not significant difference in color");
        } else if 0.5 < distance && distance < 1.0 {
            println!("There is a potentially significant and very noticeable color difference.");
        } else if distance > 1.0 {
            println!("There is a significant difference in the colors.");
        }

        return Ok(());
    }


    let colors: Vec<String> = blender.blend_colors();

    match opt.output {
        Some(path) => {
            let file = File::create(&path)?;
            let writer = BufWriter::new(file);

            return write_colors(colors, writer)
                .with_context(|| format!("Error while writing file to '{}'", path.display()));
        }
        None => {
            for color in colors {
                println!("{}", color);
            }
        }
    };

    Ok(())
}

fn write_colors<W: Write>(blended_colors: Vec<String>, mut writer: W) -> Result<()> {
    let mut buffered_writer = BufWriter::new(&mut writer);

    for color in &blended_colors {
        writeln!(buffered_writer, "{}", color)?;
    }

    buffered_writer.flush()?;
    Ok(())
}

fn color_difference(first_color: (f32, f32, f32), second_color: (f32, f32, f32)) -> f32 {
    let difference = ((second_color.0 - first_color.0) / 255.0).powf(2.0)
        + ((second_color.1 - first_color.1) / 255.0).powf(2.0)
        + ((second_color.2 - first_color.2) / 255.0).powf(2.0);

    difference.sqrt()
}

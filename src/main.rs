use anyhow::Result;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};
use structopt::StructOpt;

pub struct Blending;
mod color_blender;
use color_blender::ColorBlender;

#[derive(StructOpt, Debug)]
#[structopt(name = "color-blender-rs", about = "A color blender, written in Rust.")]
struct Opt {
    #[structopt(help = "The first color in hex format")]
    first_color: String,

    #[structopt(help = "Second color in hex format")]
    second_color: String,

    #[structopt(short, long, default_value = "10", help = "Number of midpoints")]
    midpoints: usize,

    #[structopt(short, long, help = "Output file path", default_value = "output.txt")]
    output: PathBuf,

    #[structopt(
        short = "w",
        long = "write",
        help = "Write the blended colors to a file"
    )]
    should_write: bool,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();

    let blender = ColorBlender::new(opt.first_color, opt.second_color, opt.midpoints);
    let colors = blender.blend_colors();

    let file = File::create(opt.output)?;
    let writer = BufWriter::new(file);

    match opt.should_write {
        true => match Blending::write(colors, writer) {
            Ok(_) => {
                return Ok(());
            }
            Err(err) => {
                let error_msg = format!("Error: {}", err);
                return Err(anyhow::Error::msg(error_msg));
            }
        },
        false => {
            Blending::print(colors);
        }
    };

    Ok(())
}

impl Blending {
    pub fn write<W: Write>(blended_colors: Vec<String>, mut writer: W) -> Result<()> {
        let mut buffered_writer = BufWriter::new(&mut writer);

        for color in &blended_colors {
            writeln!(buffered_writer, "{}", color)?;
        }

        buffered_writer.flush()?;
        println!("Data written successfully.");

        Ok(())
    }
    pub fn print(blended_colors: Vec<String>) {
        for color in blended_colors {
            println!("{}", color);
        }
    }
}

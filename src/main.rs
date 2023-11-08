use structopt::StructOpt;
use std::{fs::File, io::Write};

mod color_blender;

pub struct Blending;

impl Blending {
    pub fn bprint(opt: &Opt, blender: &color_blender::ColorBlender) {
        let midpoint = match opt.midpoints {
            Some(val) => val,
            None => 10,
        };
        let blended_colors = blender.blend_colors(&opt.first_color, &opt.second_color, midpoint);

        for color in &blended_colors {
            println!("{}", color);
        }
    }

    pub fn bwrite(opt: &Opt, blender: &color_blender::ColorBlender) -> Result<(), Box<dyn std::error::Error>> {
        let midpoint = match opt.midpoints {
            Some(val) => val,
            None => 10,
        };
        let blended_colors = blender.blend_colors(&opt.first_color, &opt.second_color, midpoint);

        let mut output = File::create("output.txt")?;

        for color in &blended_colors {
            writeln!(output, "{}", color)?
        }

        println!("File 'output.txt' written successfully.");

        Ok(())
    }
}

#[derive(StructOpt, Debug)]
pub struct Opt {
    /// First color (hex format)
    first_color: String,

    /// Second color (hex format)
    second_color: String,

    /// Number of midpoints
    #[structopt(short, long)]
    midpoints: Option<usize>,

    /// Should the blended colors be written to a file?
    #[structopt(short = "w", long = "write")]
    should_write: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    let blender = color_blender::ColorBlender::new();

    let _ = match opt.should_write {
        true => Blending::bwrite(&opt, &blender),
        false => Ok(Blending::bprint(&opt, &blender)),
    };
    Ok(())
}


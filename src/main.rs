use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;
use structopt::StructOpt;
use image::{ImageOutputFormat};

#[derive(StructOpt)]
#[structopt(name = "image-converter", about = "Converts images to different formats")]
struct Args {
    input: PathBuf,
    output: PathBuf,
    output_format: Option<String>,
}

fn main() -> std::io::Result<()> {
    let args = Args::from_args();

    let input = File::open(&args.input)?;
    let input_image = image::io::Reader::new(BufReader::new(input)).with_guessed_format()?.decode().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;



    let output_format = args.output_format.unwrap_or_else(|| {
        args.output.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("png")
            .to_owned()
    });

    let output_image_format = match output_format.as_str() {
        "png" => ImageOutputFormat::Png,
        "jpeg" | "jpg" => ImageOutputFormat::Jpeg(90),
        "gif" => ImageOutputFormat::Gif,
        "bmp" => ImageOutputFormat::Bmp,
        _ => return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "Unsupported output format")),
    };

    let mut output = File::create(&args.output)?;
    // input_image.write_to(&mut output, output_image_format)?;
    input_image.write_to(&mut output, output_image_format).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
    Ok(())
}

use clap::{ArgGroup, Parser};
use image::GenericImageView;
use std::{path::PathBuf, process};

#[derive(Parser)]
#[command(version, about = "Prints the dimensions of an image in 'width x height' format.", long_about = None)]
#[command(group(ArgGroup::new("dimension").args(&["width", "height"]).multiple(false)))]
struct Args {
  /// Target image
  file: Option<PathBuf>,
  /// Output only the width of the image
  #[arg(short, long)]
  width: bool,
  /// Output only the height of the image
  #[arg(short, long)]
  height: bool,
}

fn main() {
  let args = Args::parse();

  let file_path = match args.file {
    Some(path) => path,
    None => {
      eprintln!("\x1b[31mERROR\x1b[0m No file path is provided.");
      process::exit(1)
    },
  };

  let img = match image::open(&file_path) {
    Ok(image) => image,
    Err(_) => {
      eprintln!("\x1b[31mERROR\x1b[0m Could not open the image file or the file is not a valid image.");
      process::exit(1)
    },
  };

  let dimension = img.dimensions();

  if args.width {
    println!("{}", dimension.0);
  } else if args.height {
    println!("{}", dimension.1);
  } else {
    println!("{} x {}", dimension.0, dimension.1);
  }
}

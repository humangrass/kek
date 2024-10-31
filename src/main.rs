mod converter;
mod cli;

use std::fs;
use clap::Parser;
use crate::converter::Converter;

fn main() {
    let args = cli::Args::parse();
    let converter = Converter {
        image_path: args.input,
        width: args.width,
        should_display: args.should_display,
        with_background: args.background,
    };

    let ascii_art = converter.image_to_ascii();
    fs::write(&args.output, ascii_art).expect("Failed to write ASCII image")
}

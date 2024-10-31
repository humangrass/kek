use clap::Parser;

#[derive(Parser)]
#[command(name = "Image Converter", version, about)]
pub(crate) struct Args {
    /// Path to image
    #[arg(short, long)]
    pub(crate) input: String,

    /// Path to output file
    #[arg(short, long, default_value = "output.txt")]
    pub(crate) output: String,

    /// Image width in characters
    #[arg(short, long, default_value_t = 100)]
    pub(crate) width: u32,

    /// Should output be displayed
    #[arg(short, long, default_value_t = true)]
    pub(crate) should_display: bool,

    /// Fill background
    #[arg(short, long, default_value_t = false)]
    pub(crate) background: bool,
}

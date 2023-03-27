use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "dta-palette-parser")]
#[command(about = "An image parser for pallete + raw images", long_about = None)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Sets a custom palette to get
    #[arg(long, value_name = "FILE", default_value = "freedom.pal")]
    pub palette: Option<PathBuf>,

    /// The raw image file
    #[arg(long, value_name = "FILE", default_value = "Bullets.dta")]
    pub rawimg: Option<PathBuf>,

    #[arg(long, value_name = "width")]
    pub width: u32,

    #[arg(long, value_name = "height")]
    pub height: u32,

    /// Sets a custom palette to get
    #[arg(long, value_name = "FILE", default_value = "output.png")]
    pub output: Option<PathBuf>,
}

pub fn parse_cli() -> Cli {
    let cli = Cli::parse();

    // // You can check the value provided by positional arguments, or option arguments

    // if let Some(config_path) = cli.config.as_deref() {
    //     println!("Value for config: {}", config_path.display());
    // }
    // if let Some(log_input) = cli.logcfg.as_deref() {
    //     println!("Value for logcfg: {}", log_input.display());
    // }

    // println!("{:?}", cli);

    cli
}

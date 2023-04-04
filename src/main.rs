use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use fern::colors::{Color, ColoredLevelConfig};
use vastatrix::{vastatrix::Vastatrix, loading};
use zip::ZipArchive;

#[derive(Parser)]
struct Cli {
    #[arg(long = "jar", value_name = "FILE")]
    jar: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let file = File::open(cli.jar.as_deref().unwrap()).unwrap();
    logging_setup();
    let archive = ZipArchive::new(file).unwrap();
    let mut vtx = Vastatrix::new(archive); 
    vtx.run();
}

fn logging_setup() -> () {
    let colors_line =
        ColoredLevelConfig::new().error(Color::Red).warn(Color::Yellow).info(Color::White).debug(Color::White).trace(Color::BrightBlack);

    let colors_level = colors_line.clone().info(Color::Green);
    fern::Dispatch::new().format(move |out, message, record| {
                             out.finish(format_args!("{color_line}[{date}][{target}][{level}{color_line}] {message}\x1B[0m",
                                                     color_line = format_args!("\x1B[{}m", colors_line.get_color(&record.level()).to_fg_str()),
                                                     date = chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                                                     target = record.target(),
                                                     level = colors_level.color(record.level()),
                                                     message = message,));
                         })
                         .level(log::LevelFilter::Warn)
                         .level_for("vastatrix", log::LevelFilter::Trace)
                         .chain(std::io::stdout())
                         .apply()
                         .unwrap();
}

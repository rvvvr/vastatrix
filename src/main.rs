use std::fs::File;
use std::path::PathBuf;

use clap::Parser;
use vastatrix::vastatrix::Vastatrix;
use zip::ZipArchive;

#[derive(Parser)]
struct Cli {
    #[arg(long = "jar", value_name = "FILE")]
    jar: Option<PathBuf>,
}

fn main() {
    let cli = Cli::parse();
    let file = File::open(cli.jar.as_deref().unwrap()).unwrap();

    let archive = ZipArchive::new(file).unwrap();
    let mut vtx = Vastatrix::new(archive);
    vtx.run();
}

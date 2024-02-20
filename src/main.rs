use clap::Parser;
use vastatrix::{Vastatrix, LaunchOptions, HowLaunch};
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(long = "jar", value_name = "FILE")]
    jar: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let mut vtx = Vastatrix::new();
    let launch_options = if let Some(path) = cli.jar {
	LaunchOptions::new(HowLaunch::JarFile(path))
    } else {
	unimplemented!("Other launch options that aren't jar");
    };
    vtx.go(launch_options)?;
    Ok(())
}

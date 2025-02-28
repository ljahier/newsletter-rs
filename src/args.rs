use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "newsletter", about = "Application Newsletter", version = "1.0")]
pub struct Args {
    #[arg(
        short = 'c',
        long = "config",
        value_name = "FILE_PATH",
        default_value = "./config.cfg"
    )]
    pub file_path: PathBuf,

    #[arg(short = None, long = "init-db", action = clap::ArgAction::SetTrue, )]
    pub init_db: bool,
}

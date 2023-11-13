use std::path::PathBuf;

use clap::{CommandFactory, FromArgMatches, Parser};
use concat_with::concat_line;
use terminal_size::terminal_size;

const APP_NAME: &str = "s2tw";
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

const AFTER_HELP: &str = "Enjoy it! https://magiclen.org";

const APP_ABOUT: &str = concat!(
    "A simple tool for converting Simple Chinese to Traditional Chinese(TW).\n\nEXAMPLES:\n",
    concat_line!(prefix "s2tw ",
        "                  # Convert each of input lines from Simple Chinese to Traditional Chinese",
        "chs.txt cht.txt   # Convert chs.txt (in Simple Chinese) to cht.txt (in Traditional Chinese)",
        "a.chs.txt         # Convert a.chs.txt (in Simple Chinese) to a.cht.txt (in Traditional Chinese)"
    )
);

#[derive(Debug, Parser)]
#[command(name = APP_NAME)]
#[command(term_width = terminal_size().map(|(width, _)| width.0 as usize).unwrap_or(0))]
#[command(version = CARGO_PKG_VERSION)]
#[command(author = CARGO_PKG_AUTHORS)]
#[command(after_help = AFTER_HELP)]
pub struct CLIArgs {
    #[arg(short, long)]
    #[arg(help = "Force to output if the output file exists")]
    pub force: bool,

    #[arg(value_hint = clap::ValueHint::FilePath)]
    #[arg(help = "Assign the path of your Simple Chinese document. It should be a file path")]
    pub s_path: Option<PathBuf>,

    #[arg(value_hint = clap::ValueHint::FilePath)]
    #[arg(help = "Assign the path of your Traditional Chinese document. It should be a file path")]
    pub tw_path: Option<PathBuf>,
}
pub fn get_args() -> CLIArgs {
    let args = CLIArgs::command();

    let about = format!("{APP_NAME} {CARGO_PKG_VERSION}\n{CARGO_PKG_AUTHORS}\n{APP_ABOUT}");

    let args = args.about(about);

    let matches = args.get_matches();

    match CLIArgs::from_arg_matches(&matches) {
        Ok(args) => args,
        Err(err) => {
            err.exit();
        },
    }
}

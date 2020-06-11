#[macro_use]
extern crate concat_with;
extern crate clap;
extern crate terminal_size;

extern crate opencc_rust;

extern crate path_absolutize;

extern crate s2tw;

use std::borrow::Cow;
use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

use clap::{App, Arg};
use terminal_size::terminal_size;

use opencc_rust::{generate_static_dictionary, DefaultConfig, OpenCC};

use path_absolutize::Absolutize;

use s2tw::*;

const APP_NAME: &str = "s2tw";
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

fn main() -> Result<(), String> {
    let matches = App::new(APP_NAME)
        .set_term_width(terminal_size().map(|(width, _)| width.0 as usize).unwrap_or(0))
        .version(CARGO_PKG_VERSION)
        .author(CARGO_PKG_AUTHORS)
        .about(concat!("A simple tool for converting Simple Chinese to Traditional Chinese(TW).\n\nEXAMPLES:\n", concat_line!(prefix "s2tw ",
            "                               # Convert each of input lines from Simple Chinese to Traditional Chinese",
            "chs.txt cht.txt                # Convert chs.txt (in Simple Chinese) to cht.txt (in Traditional Chinese)",
            "a.chs.txt                      # Convert a.chs.txt (in Simple Chinese) to a.cht.txt (in Traditional Chinese)"
        )))
        .arg(Arg::with_name("FORCE")
            .long("force")
            .short("f")
            .help("Forces to output if the output file exists.")
        )
        .arg(Arg::with_name("S_PATH")
            .help("Assigns the path of your Simple Chinese document. It should be a file path.")
            .takes_value(true)
            .index(1)
        )
        .arg(Arg::with_name("TW_PATH")
            .help("Assigns the path of your Traditional Chinese document. It should be a file path.")
            .takes_value(true)
            .index(2)
        )
        .after_help("Enjoy it! https://magiclen.org")
        .get_matches();

    let s_path = matches.value_of("S_PATH");
    let tw_path = matches.value_of("TW_PATH");

    let force = matches.is_present("FORCE");

    let temporary_path = env::temp_dir();

    generate_static_dictionary(&temporary_path, DefaultConfig::S2TWP)
        .map_err(|err| err.to_string())?;

    let opencc = OpenCC::new(Path::join(&temporary_path, DefaultConfig::S2TWP))
        .map_err(|err| err.to_string())?;
    assert_eq!("測試字串", opencc.convert("测试字符串"));

    match s_path {
        Some(s_path) => {
            let s_path = Path::new(s_path);

            if s_path.is_dir() {
                return Err(format!(
                    "`{}` is a directory!",
                    s_path.absolutize().map_err(|err| err.to_string())?.to_string_lossy()
                ));
            }

            let s_file = File::open(&s_path).map_err(|err| err.to_string())?;

            let tw_path = match tw_path {
                Some(tw_path) => Cow::from(Path::new(tw_path)),
                None => {
                    let parent = s_path.parent().unwrap();

                    let file_stem = match s_path.file_stem() {
                        Some(file_stem) => {
                            let file_stem = file_stem
                                .to_str()
                                .ok_or_else(|| String::from("Unsupported path."))?;

                            if file_stem.ends_with(".chs") {
                                &file_stem[..file_stem.len() - 4]
                            } else {
                                file_stem
                            }
                        }
                        None => "",
                    };

                    let file_stem = opencc.convert(&file_stem);

                    let file_name = match s_path.extension() {
                        Some(extension) => {
                            format!("{}.cht.{}", file_stem, extension.to_string_lossy())
                        }
                        None => format!("{}.cht", file_stem),
                    };

                    let tw_path = Path::join(parent, file_name);

                    Cow::from(tw_path)
                }
            };

            if tw_path.exists() && (tw_path.is_dir() || !force) {
                return Err(format!(
                    "`{}` exists!",
                    tw_path.absolutize().map_err(|err| err.to_string())?.to_string_lossy()
                ));
            }

            let mut tw_file = File::create(tw_path.as_ref()).map_err(|err| err.to_string())?;

            let mut s_file = BufReader::new(s_file);

            let mut line = String::new();

            loop {
                line.clear();

                let c = s_file.read_line(&mut line).map_err(|err| {
                    try_delete(&tw_path);
                    err.to_string()
                })?;

                if c == 0 {
                    break;
                }

                tw_file.write(&opencc.convert(&line[0..c]).into_bytes()).map_err(|err| {
                    try_delete(&tw_path);
                    err.to_string()
                })?;
            }
        }
        None => {
            let mut line = String::new();
            loop {
                line.clear();

                let c = io::stdin().read_line(&mut line).map_err(|err| err.to_string())?;

                if c == 0 {
                    break;
                }

                println!("{}", opencc.convert(&line[0..(c - 1)]));
            }
        }
    }

    Ok(())
}

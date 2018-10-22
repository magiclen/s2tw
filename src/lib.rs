//! # s2tw
//! A simple tool for converting Simple Chinese to Traditional Chinese(TW).

extern crate clap;
extern crate opencc_rust;
extern crate path_absolutize;

use path_absolutize::*;

use std::env;
use std::path::Path;
use std::io::{self, Write, BufReader, BufRead};
use std::fs::{self, File};

use clap::{App, Arg};

use opencc_rust::*;

// TODO -----Config START-----

const APP_NAME: &str = "s2tw";
const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");

#[derive(Debug)]
pub struct Config {
    pub s_path: Option<String>,
    pub tw_path: Option<String>,
    pub force: bool,
}

impl Config {
    pub fn from_cli() -> Result<Config, String> {
        let arg0 = env::args().next().unwrap();
        let arg0 = Path::new(&arg0).file_stem().unwrap().to_str().unwrap();

        let examples = vec![
            "                               # Convert each of input lines from Simple Chinese to Traditional Chinese",
            "chs.txt cht.txt                # Convert chs.txt (in Simple Chinese) to cht.txt (in Traditional Chinese)",
            "a.chs.txt                      # Convert a.chs.txt (in Simple Chinese) to a.cht.txt (in Traditional Chinese)"
        ];

        let matches = App::new(APP_NAME)
            .version(CARGO_PKG_VERSION)
            .author(CARGO_PKG_AUTHORS)
            .about(format!("A simple tool for converting Simple Chinese to Traditional Chinese(TW).\n\nEXAMPLES:\n{}", examples.iter()
                .map(|e| format!("  {} {}\n", arg0, e))
                .collect::<Vec<String>>()
                .concat()
            ).as_str()
            )
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

        let s_path = matches.value_of("S_PATH").map(|s| s.to_string());

        let tw_path = matches.value_of("TW_PATH").map(|s| s.to_string());

        let force = matches.is_present("FORCE");

        Ok(Config {
            s_path,
            tw_path,
            force,
        })
    }
}

// TODO -----Config END-----

pub fn run(config: Config) -> Result<i32, String> {
    let temporary_path = env::temp_dir();

    generate_static_dictionary(&temporary_path, DefaultConfig::S2TWP).unwrap();

    let opencc = OpenCC::new(Path::join(&temporary_path, DefaultConfig::S2TWP)).unwrap();
    assert_eq!("測試字串", opencc.convert("测试字符串"));

    match config.s_path {
        Some(s_path) => {
            let s_path = Path::new(&s_path).absolutize().unwrap();

            if !s_path.exists() {
                return Err(format!("`{}` does not exist!", s_path.to_str().unwrap()));
            }

            if !s_path.is_file() {
                return Err(format!("`{}` is not a file!", s_path.to_str().unwrap()));
            }

            let tw_path = match config.tw_path {
                Some(tw_path) => {
                    let tw_path = Path::new(&tw_path).absolutize().unwrap();

                    if tw_path.exists() {
                        if config.force {
                            if !s_path.is_file() {
                                return Err(format!("`{}` is not a file!", tw_path.to_str().unwrap()));
                            }
                        } else {
                            return Err(format!("`{}` exists!", tw_path.to_str().unwrap()));
                        }
                    }

                    tw_path
                }
                None => {
                    let parent = s_path.parent().unwrap();

                    let file_stem = s_path.file_stem().unwrap().to_str().unwrap();
                    let extension = s_path.extension().unwrap().to_str().unwrap();

                    let file_stem = if file_stem.ends_with(".chs") {
                        &file_stem[..file_stem.len() - 4]
                    } else {
                        file_stem
                    };

                    let file_stem = opencc.convert(&file_stem);

                    let file_name = format!("{}.cht.{}", file_stem, extension);

                    Path::join(parent, file_name)
                }
            };

            let s_file = File::open(&s_path).map_err(|_| format!("Cannot open {}.", s_path.to_str().unwrap()))?;

            let mut s_file = BufReader::new(s_file);

            let mut tw_file = File::create(&tw_path).map_err(|_| format!("Cannot create {}.", tw_path.to_str().unwrap()))?;

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

    Ok(0)
}

fn try_delete<P: AsRef<Path>>(path: P) {
    if let Err(_) = fs::remove_file(path.as_ref()) {}
}
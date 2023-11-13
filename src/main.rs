mod cli;

use std::{
    env, fs,
    fs::File,
    io,
    io::{BufRead, BufReader, Write},
};

use anyhow::{anyhow, Context};
use cli::*;
use opencc_rust::{generate_static_dictionary, DefaultConfig, OpenCC};

fn main() -> anyhow::Result<()> {
    let args = get_args();

    let temporary_path = env::temp_dir();

    generate_static_dictionary(&temporary_path, DefaultConfig::S2TWP).unwrap();

    let opencc = OpenCC::new(temporary_path.join(DefaultConfig::S2TWP)).unwrap();
    debug_assert_eq!("測試字串", opencc.convert("测试字符串"));

    match args.s_path {
        Some(s_path) => {
            if s_path.is_dir() {
                return Err(anyhow!("{s_path:?} is a directory!"));
            }

            let s_file = File::open(s_path.as_path()).with_context(|| anyhow!("{s_path:?}"))?;

            let tw_path = match args.tw_path {
                Some(tw_path) => tw_path,
                None => {
                    let parent = s_path.parent().unwrap();

                    let file_stem = match s_path.file_stem() {
                        Some(file_stem) => {
                            let file_stem = file_stem
                                .to_str()
                                .ok_or_else(|| anyhow!("{s_path:?} is an unsupported path."))?;

                            file_stem.strip_suffix(".chs").unwrap_or(file_stem.as_ref())
                        },
                        None => "",
                    };

                    let file_stem = opencc.convert(file_stem);

                    let file_name = match s_path.extension() {
                        Some(extension) => {
                            format!("{file_stem}.cht.{}", extension.to_string_lossy())
                        },
                        None => format!("{file_stem}.cht"),
                    };

                    parent.join(file_name)
                },
            };

            match tw_path.metadata() {
                Ok(metadata) => {
                    if metadata.is_dir() {
                        return Err(anyhow!("{tw_path:?} is a directory!"));
                    } else if !args.force {
                        return Err(anyhow!("{tw_path:?} exists!"));
                    }
                },
                Err(error) if error.kind() == io::ErrorKind::NotFound => (),
                Err(error) => {
                    return Err(error).with_context(|| anyhow!("{tw_path:?}"));
                },
            }

            let mut tw_file =
                File::create(tw_path.as_path()).with_context(|| anyhow!("{tw_path:?}"))?;

            let mut s_file = BufReader::new(s_file);

            let mut line = String::new();

            loop {
                line.clear();

                let c = s_file
                    .read_line(&mut line)
                    .map_err(|error| {
                        let _ = fs::remove_file(tw_path.as_path());

                        error
                    })
                    .with_context(|| anyhow!("{s_path:?}"))?;

                if c == 0 {
                    break;
                }

                tw_file.write(&opencc.convert(&line[0..c]).into_bytes()).map_err(|error| {
                    let _ = fs::remove_file(tw_path.as_path());

                    error
                })?;
            }
        },
        None => {
            let mut line = String::new();

            loop {
                line.clear();

                let c = io::stdin().read_line(&mut line).with_context(|| anyhow!("stdin"))?;

                if c == 0 {
                    break;
                }

                println!("{}", opencc.convert(&line[0..(c - 1)]));
            }
        },
    }

    Ok(())
}

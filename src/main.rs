mod cli;

use std::{
    env, fs,
    fs::File,
    io,
    io::{BufRead, BufReader, BufWriter, Write},
    path::Path,
};

use anyhow::{Context, anyhow};
use cli::*;
use opencc_rust::{DefaultConfig, OpenCC, generate_static_dictionary};

fn main() -> anyhow::Result<()> {
    let args = get_args();

    // A dedicated directory keeps the dictionaries away from files created by other programs.
    let temporary_path =
        env::temp_dir().join(concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION")));

    generate_static_dictionary(temporary_path.as_path(), DefaultConfig::S2TWP)
        .map_err(|error| anyhow!(error))
        .with_context(|| anyhow!("{temporary_path:?}"))?;

    let opencc = OpenCC::new(temporary_path.join(DefaultConfig::S2TWP))
        .with_context(|| anyhow!("{temporary_path:?}"))?;
    debug_assert_eq!(Ok("測試字串"), opencc.convert("测试字符串").as_ref().map(|s| s.as_str()));

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

                            file_stem.strip_suffix(".chs").unwrap_or(file_stem)
                        },
                        None => "",
                    };

                    let file_stem = opencc.convert(file_stem)?;

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

                    // Creating the output file would truncate the input file before it is read.
                    let s_path_canonical = fs::canonicalize(s_path.as_path())
                        .with_context(|| anyhow!("{s_path:?}"))?;
                    let tw_path_canonical = fs::canonicalize(tw_path.as_path())
                        .with_context(|| anyhow!("{tw_path:?}"))?;

                    if s_path_canonical == tw_path_canonical {
                        return Err(anyhow!("{tw_path:?} is the same file as {s_path:?}!"));
                    }
                },
                Err(error) if error.kind() == io::ErrorKind::NotFound => (),
                Err(error) => {
                    return Err(error).with_context(|| anyhow!("{tw_path:?}"));
                },
            }

            let tw_file =
                File::create(tw_path.as_path()).with_context(|| anyhow!("{tw_path:?}"))?;

            if let Err(error) =
                convert_file(&opencc, s_file, s_path.as_path(), tw_file, tw_path.as_path())
            {
                let _ = fs::remove_file(tw_path.as_path());

                return Err(error);
            }
        },
        None => {
            let mut stdin = io::stdin().lock();
            let mut stdout = io::stdout().lock();

            let mut line = String::new();

            loop {
                line.clear();

                let c = stdin.read_line(&mut line).with_context(|| anyhow!("stdin"))?;

                if c == 0 {
                    break;
                }

                // The last line of the input may not end with a newline.
                let content = line.strip_suffix('\n').unwrap_or(line.as_str());

                writeln!(stdout, "{}", opencc.convert(content)?)
                    .with_context(|| anyhow!("stdout"))?;
            }
        },
    }

    Ok(())
}

fn convert_file(
    opencc: &OpenCC,
    s_file: File,
    s_path: &Path,
    tw_file: File,
    tw_path: &Path,
) -> anyhow::Result<()> {
    let mut s_file = BufReader::new(s_file);
    let mut tw_file = BufWriter::new(tw_file);

    let mut line = String::new();

    loop {
        line.clear();

        let c = s_file.read_line(&mut line).with_context(|| anyhow!("{s_path:?}"))?;

        if c == 0 {
            break;
        }

        tw_file
            .write_all(opencc.convert(line.as_str())?.as_bytes())
            .with_context(|| anyhow!("{tw_path:?}"))?;
    }

    tw_file.flush().with_context(|| anyhow!("{tw_path:?}"))
}

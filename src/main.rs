mod logs;
mod parser;
mod structs;
mod utils;

use colored::Colorize;
use logs::{error, help, version, warn};
use parser::{commands::command_parser, flags::flag_parser, options::option_parser};

use std::{
    env,
    process::{exit, Command},
};
use utils::sts;

fn main() {
    let pico_parser = pico_args::Arguments::from_env();

    let mut data = structs::Data::init(pico_parser);

    if data.args.help {
        help();
        exit(0)
    }

    if data.args.version {
        version();
        exit(0)
    }

    if data.args.command.is_none() {
        error("Missing command.".into(), None);
        exit(1)
    }

    command_parser(&mut data);
    option_parser(&mut data);
    flag_parser(&mut data);

    if data.args.output.is_none() {
        error("Missing output".into(), None);
        exit(1)
    }

    if data.args.output.as_ref().unwrap().exists() && !data.args.overwrite {
        logs::error(
            format!(
                "File {:#?} already exists.",
                data.args.output.unwrap().file_name().unwrap()
            ),
            Some(" Use `--overwrite` or `-w` flag for overwrite.".into()),
        );
        exit(1)
    }

    if data.af_args.len() != 0 {
        data.ffmpeg_args
            .extend(sts(&["-af", data.af_args.join(";").as_str()]));
    }

    if data.vf_args.len() != 0 {
        data.ffmpeg_args
            .extend(sts(&["-vf", data.vf_args.join(";").as_str()]));
    }

    data.ffmpeg_args.push("-y".into());
    data.ffmpeg_args
        .push(data.args.output.unwrap().to_string_lossy().to_string());

    logs::info(format!(
        "Spawning FFmpeg command\n{} {}",
        "ffmpeg".italic(),
        data.ffmpeg_args.join(" ").italic()
    ));

    match Command::new("ffmpeg")
        .current_dir(env::current_dir().unwrap())
        .args(data.ffmpeg_args)
        .output()
    {
        Ok(e) => {
            if !e.status.success() {
                logs::error(
                    "An error has ocurred while running FFmpeg.".into(),
                    if data.args.verbose {
                        Some("".into())
                    } else {
                        Some(
                            String::from_utf8_lossy(&e.stderr)
                                .lines()
                                .last()
                                .unwrap()
                                .to_string(),
                        )
                    },
                );

                if data.args.verbose {
                    eprintln!("{}", String::from_utf8_lossy(&e.stderr));
                }
            } else {
                logs::ok("Completed.".into());

                if data.args.verbose {
                    println!("{}", String::from_utf8_lossy(&e.stdout));
                }
            }
        }
        Err(err) => eprintln!("{}", err),
    };
}

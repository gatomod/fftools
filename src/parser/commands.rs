use std::process::exit;

use crate::{
    logs::{self, error},
    structs::Data,
    utils::sts,
};

mod fps;
mod gif;
mod merge;
mod optimize;
mod scale;
mod trim;
mod volume;

// TODO Rewrite command handler (with better IO requirements and other stuff)
pub fn command_parser(data: &mut Data) {
    let args = data.args.clone();

    if data.args.input.len() == 0 {
        error("Missing `-i/--input` argument.".into(), None);
        exit(1)
    }

    for input in data.args.input.iter() {
        data.ffmpeg_args
            .extend(sts(&["-i", input.to_str().unwrap()]));
    }

    let unknown_command = || {
        logs::error(
            format!(
                "Unknown `{}` command.",
                args.command.clone().unwrap_or("null".into())
            ),
            None,
        );
        exit(1)
    };

    match &args.command {
        Some(cmd) => match cmd.as_str() {
            "free" => {}

            "o" | "optimize" => optimize::optimize(data),
            "t" | "trim" => trim::trim(data),
            "g" | "gif" => gif::gif(data),
            "s" | "scale" => scale::scale(data),
            "v" | "volume" => volume::volume(data),
            "fps" => fps::fps(data),
            "m" | "merge" => merge::merge(data),

            "merge" => {
                if data.args.input.len() < 2 {
                    error(
                        format!("`{}` command requires at least two inputs.", cmd),
                        None,
                    );
                    exit(1)
                } else {
                    match cmd.as_str() {
                        "merge" => merge::merge(data),
                        _ => unknown_command(),
                    }
                }
            }
            _ => unknown_command(),
        },
        _ => {
            logs::error("Missing command.".into(), None);
            exit(1)
        }
    };
}

mod logs;
mod parser;
mod structs;

use colored::Colorize;
use pico_args::Error;
use std::{
    env,
    process::{exit, Command},
    vec,
};

fn main() {
    // Parse flags and custom errors

    let mut args = match parser::parser() {
        Ok(v) => v,
        Err(e) => {
            logs::error(
                match e {
                    Error::MissingArgument => "Missing argument.".to_string(),
                    Error::MissingOption(_) => format!("Missing `-i / --input` option"),
                    Error::OptionWithoutAValue(opt) => {
                        format!("`{}` needs a value.", opt)
                    }
                    Error::NonUtf8Argument => "Detected a non UTF-8 character".to_string(),
                    Error::Utf8ArgumentParsingFailed { value, cause } => {
                        format!("UTF-8 parsing failed: Value `{}`, cause `{}`", value, cause)
                    }
                    Error::ArgumentParsingFailed { cause } => {
                        format!("Parsing failed: `{}`", cause)
                    }
                },
                None,
            );
            exit(1);
        }
    };

    // FFmpeg args collector

    let mut ffmpeg_args: Vec<&str> = vec![];
    let mut vf_args: Vec<&str> = vec![];
    let mut af_args: Vec<&str> = vec![];

    // Input

    ffmpeg_args.push("-i");
    ffmpeg_args.push(args.input.to_str().unwrap());

    // Command args builder
    // Checks the required flags for each command and adds it to the args collector

    // Command checker
    match args.command.as_str() {
        "free" => {}
        "trim" => {
            if args.from.is_none() && args.to.is_none() {
                logs::ferror("trim".into(), vec!["--from", "--to"], Some(1));
                exit(1);
            }
        }
        "gif" => {
            if let Some(value) = args.output.extension() {
                if value != "gif" {
                    logs::error("GIF files must end in `.gif`".into(), None);
                    exit(1);
                }
            } else {
                args.output.set_extension("gif");
            }

            vf_args.push("[0:v]split[a][b]");
            vf_args.push("[a]palettegen[p]");
            vf_args.push("[b][p]paletteuse");
        }
        "scale" => {
            if args.scale.is_none() {
                logs::ferror("scale".into(), vec!["--scale"], None);
                exit(1);
            }
        }
        _ => {
            logs::error(format!("Unknown `{}` command.", args.command), None);
            exit(1)
        }
    }

    // Args builder
    // This args are not in command checker in order to keep user choose arguments
    // (for free command or some additional flags in commands)

    // From
    if let Some(from) = args.from.as_ref() {
        ffmpeg_args.push("-ss");
        ffmpeg_args.push(from.as_str());
    }

    // To
    if let Some(to) = args.to.as_ref() {
        ffmpeg_args.push("-to");
        ffmpeg_args.push(to.as_str());
    }

    // FPS
    if let Some(fps) = args.fps.as_ref() {
        ffmpeg_args.push("-r");
        ffmpeg_args.push(fps.as_str());
    }

    // Scale
    let scale_str: String;
    if let Some(scale) = args.scale.as_ref() {
        scale_str = format!(
            "scale=iw*{p}:ih*{p}",
            p = scale.parse::<f64>().unwrap_or_else(|_| {
                logs::error("Please insert a valid number.".into(), None);
                exit(1);
            }) / 100.0
        );
        vf_args.push(&scale_str)
    }

    // Volume
    // TODO add dB and percentage support (100 as 1, https://trac.ffmpeg.org/wiki/AudioVolume)
    let vol_str: String;
    if let Some(volume) = args.volume.as_ref() {
        vol_str = format!("volume={}", volume);
    }

    // Media filters

    // Video
    let vf_args = vf_args.join(";");
    if vf_args.len() > 0 {
        ffmpeg_args.push("-vf");
        ffmpeg_args.push(&vf_args);
    }

    // Audio
    let af_args = af_args.join(";");
    if af_args.len() > 0 {
        ffmpeg_args.push("-af");
        ffmpeg_args.push(&af_args);
    }

    // Output
    ffmpeg_args.push("-y");
    ffmpeg_args.push(args.output.to_str().unwrap());

    // Check if file exists
    // This is checked after parsing args in case of output modification
    if args.output.exists() && !args.overwrite {
        logs::error(
            format!(
                "File {:#?} already exists.",
                args.output.file_name().unwrap()
            ),
            Some(" Use `--overwrite` flag for overwrite.".into()),
        );
        exit(1)
    }

    // FFmpeg spawner
    // Creates a child process with the args collector

    logs::info(format!(
        "Spawning FFmpeg command\n{} {}",
        "ffmpeg".italic(),
        ffmpeg_args.join(" ").italic()
    ));

    match Command::new("ffmpeg")
        .current_dir(env::current_dir().unwrap())
        .args(ffmpeg_args)
        .output()
    {
        Ok(e) => {
            if !e.status.success() {
                logs::error(
                    "An error has ocurred while running FFMPEG.".into(),
                    if args.verbose {
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

                if args.verbose {
                    eprintln!("{}", String::from_utf8_lossy(&e.stderr));
                }
            } else {
                logs::ok("Completed.".into());

                if args.verbose {
                    println!("{}", String::from_utf8_lossy(&e.stdout));
                }
            }
        }
        Err(err) => eprintln!("{}", err),
    };
}

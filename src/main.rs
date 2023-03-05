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
use structs::Args;

use crate::logs::ferror;

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

	// FFmpeg args collector

	let mut ffmpeg_args: Vec<&str> = vec![];

	// Input

	ffmpeg_args.push("-i");
	ffmpeg_args.push(args.input.to_str().unwrap());

	// Command args builder
	// Checks the required flags for each command and adds it to the args collector

	// Command checker
	match args.command.as_str() {
		"trim" => {
			if args.from.is_none() && args.to.is_none() {
				ferror("trim".into(), vec!["--from", "--to"], Some(1));
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

			// TODO add scale options in filter_complex
			ffmpeg_args.push("-filter_complex");
			ffmpeg_args.push("[0:v]split[a][b];[a]palettegen[p];[b][p]paletteuse");
		}
		"free" => {}
		_ => {
			logs::error(format!("Unknown `{}` command.", args.command), None);
			exit(1)
		}
	}

	// Args builder
	// This args are not in command checker in order to keep user choose arguments
	// (for free command or some additional flags in commands)

	// From
	if args.from.is_some() {
		ffmpeg_args.push("-ss");
		ffmpeg_args.push(args.from.as_ref().unwrap().as_str());
	}

	// To
	if args.to.is_some() {
		ffmpeg_args.push("-to");
		ffmpeg_args.push(args.to.as_ref().unwrap().as_str());
	}

	// Output
	ffmpeg_args.push("-y");
	ffmpeg_args.push(args.output.to_str().unwrap());

	// Spawn FFmpeg
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

pub mod commands;
pub mod flags;
pub mod options;

use std::process::exit;

use pico_args::Error;

use crate::{logs::error, structs};

pub fn parser(args: &mut pico_args::Arguments) -> Result<structs::Args, pico_args::Error> {
    #[rustfmt::skip]
	let app_args = structs::Args {
		command: args.subcommand()?,

		input: args.values_from_os_str(["-i", "--input"], parse_path)?,

		version:        args.contains(["-v", 	"--version"		]),
		help:           args.contains(["-h", 	"--help"		]),
		verbose:        args.contains(["-V", 	"--verbose"		]),
		overwrite:      args.contains(["-w", 	"--overwrite"	]),
		no_audio:       args.contains(["-n", 	"--noaudio"		]),
		no_trim:        args.contains("--notrim"),
		
		encode:         args.contains(["-e", 	"--encode"]),
		audio_encode:   args.contains("--aencode"),
		video_encode:   args.contains("--vencode"),
		
		copy:           args.contains(["-c", 	"--copy"]),
		audio_copy:     args.contains("--acopy"),
		video_copy:     args.contains("--vcopy"),

		optimize: 	args.opt_value_from_str(["-o", "--optimize"	])?,
		from:   	args.opt_value_from_str(["-f", "--from"		])?,
		to:     	args.opt_value_from_str(["-t", "--to"			])?,
		scale:  	args.opt_value_from_str(["-s", "--scale"		])?,
		volume: 	args.opt_value_from_str(["-l", "--volume"		])?,
		fps:    	args.opt_value_from_str("--fps")?,

		output: args.opt_free_from_os_str(parse_path)?,
	};

    Ok(app_args)
}

pub fn pico_args_error_handler(err: Error) {
    #[rustfmt::skip]
	let msg = match err {
		Error::MissingArgument
			=> format!("Missing argument."),

		Error::MissingOption(opt)
			=> format!("Missing {:?} option.", opt),

		Error::NonUtf8Argument
			=> format!("Detected a non UTF-8 character."),

		Error::OptionWithoutAValue(opt) 
			=> {
			format!("`{}` needs a value.", opt)
		}

		Error::Utf8ArgumentParsingFailed { value, cause }
			=> {
			format!("UTF-8 parsing failed: Value `{}`, cause `{}`", value, cause)
		}

		Error::ArgumentParsingFailed { cause }
			=> {
			format!("Parsing failed: `{}`", cause)
		}
	};

    error(msg, None);

    exit(1)
}

fn parse_path(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    Ok(s.into())
}

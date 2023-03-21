use std::{path::PathBuf, process::exit};

use crate::parser::{parser, pico_args_error_handler};

/// ### Args
/// List of arguments to be parsed by `pico_args`
///
/// By default, all arguments are options, except booleans (flags) and input (vector)
///
#[rustfmt::skip]
#[derive(Clone)]
pub struct Args {
	pub command: Option<String>,

	pub input: Vec<PathBuf>,

	pub version:        bool,
	pub help:           bool,
	pub verbose:        bool,
	pub overwrite:      bool,
	pub no_audio:       bool,
	pub no_trim:        bool,

	pub encode:         bool,
	pub audio_encode:   bool,
	pub video_encode:   bool,

	pub copy:           bool,
	pub audio_copy:     bool,
	pub video_copy:     bool,

	pub optimize: 	Option<String>,
	pub from:   	Option<String>,
	pub to:     	Option<String>,
	pub scale:  	Option<u64>,
	pub volume: 	Option<String>,
	pub fps:    	Option<u64>,

	pub output: Option<PathBuf>,
}

/// ### Data
/// Main struct, with FFmpeg argument vectors and parser
///
#[rustfmt::skip]
pub struct Data {
	pub ffmpeg_args:    Vec<String>,
	pub af_args:        Vec<String>,
	pub vf_args:        Vec<String>,
	
	pub args:           Args,
	pub pico_parser:    pico_args::Arguments,
}

impl Data {
    pub fn init(mut pico_parser: pico_args::Arguments) -> Self {
        let args = parser(&mut pico_parser).unwrap_or_else(|err| {
            pico_args_error_handler(err);
            exit(1)
        });

        Self {
            ffmpeg_args: vec![],
            af_args: vec![],
            vf_args: vec![],
            args,
            pico_parser,
        }
    }
}

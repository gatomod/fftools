use std::process::exit;

use crate::{logs, structs};

pub fn parser() -> Result<structs::Args, pico_args::Error> {
    // Pico args parser

    let mut args = pico_args::Arguments::from_env();

    // Some flags

    if args.contains(["-h", "--help"]) {
        logs::help();
        exit(0);
    }

    if args.contains(["-v", "--version"]) {
        logs::version();
        exit(0);
    }

    // Args parser

    let app_args = structs::Args {
        // Command
        command: match args.subcommand() {
            Err(_) | Ok(None) => {
                logs::error("Missing command.".into(), None);
                exit(1)
            }
            Ok(p) => p.unwrap(),
        },

        // Input
        input: args.value_from_os_str(["-i", "--input"], parse_path)?,

        // From - To
        from: args.opt_value_from_str(["-f", "--from"])?,
        to: args.opt_value_from_str(["-t", "--to"])?,

        // Verbosity
        verbose: if args.contains(["-V", "--verbose"]) {
            true
        } else {
            false
        },

        // Overwrite
        overwrite: if args.contains(["-w", "--overwrite"]) {
            true
        } else {
            false
        },

        // FPS
        fps: args.opt_value_from_str("--fps")?,

        // Scale
        scale: args.opt_value_from_str(["-s", "--scale"])?,

        // Volume
        volume: args.opt_value_from_str(["-vv", "--volume"])?,

        // Output
        output: match args.free_from_os_str(parse_path) {
            Err(_) => {
                logs::error("Missing output.".into(), None);
                exit(1)
            }
            Ok(p) => p,
        },
    };

    Ok(app_args)
}

fn parse_path(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    Ok(s.into())
}

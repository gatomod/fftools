use colored::*;
use indoc::printdoc;

const HELP: &str = "Use `--help` for more information.";

/// ## Help
/// Help string with command list.
pub fn help() {
    printdoc! {
        "{title}
		FFmpeg-based toolkit for manipulate multimedia easily.
		
		Licensed under the Apache License, Version 2.0.
		< https://www.apache.org/licenses/LICENSE-2.0 >

		{usage}
			fftools <SUBCOMMAND> -i <INPUT> [OPTIONS] <OUTPUT>

		{commands}
			trim	Trims video between a range [ at least `--from` or `--to` ]
			gif	Converts video into lossless GIF [ no options ]
			free	Dynamic subcommand [ any option ]


		{options}
		Stream management:
			-i, --input <file>	Input file
			-w, --overwrite		Overwrite output
		
		Media manipulation:
			-f, --from <time>  Start timestamp
			-t, --to <time>    End timestamp

		Miscellaneous:
			-V, --verbose   Add verbosity
			-h, --help      Show this message
			-v, --version   Show FFtools version
		\n",
        title="<< FFtools >>".cyan().bold(),
        usage="USAGE".bold(),
        commands="COMMANDS".bold(),
        options="OPTIONS/FLAGS".bold(),
    };
    /* println!(
        r#"{}
        {}
        FFmpeg-based toolkit for manipulate multimedia easily.
        Licensed under the Apache License, Version 2.0.

        < https://www.apache.org/licenses/LICENSE-2.0 >"#,
        "<< FFtools >>".cyan().bold(),
        format!(
            "Version {} - Copyright 2023 Gátomo",
            env!("CARGO_PKG_VERSION")
        )
        .italic()
    ) */
}

pub fn version() {
    println!(
        "{title}\n{header}",
        title = "<< FFtools >>".cyan().bold(),
        header = format!(
            "Version {} - Copyright 2023 Gátomo",
            env!("CARGO_PKG_VERSION")
        )
        .italic()
    )
}

/// ## Error
/// Prints an error.
pub fn error(msg: String, tip: Option<String>) {
    eprintln!(
        "{} {}\n{}",
        "Error >".red().bold(),
        msg,
        if tip.is_some() {
            tip.unwrap().italic()
        } else {
            HELP.italic()
        }
    )
}

/// ## ferror
/// Prints a list of missing flags.
pub fn ferror(cmd: String, flags: Vec<&str>, min: Option<u8>) {
    eprintln!(
        "{} `{}` command requires{} the next flags:\n{}\n\n{}",
        "Error >".red().bold(),
        cmd,
        if min.is_some() {
            format!(" at least {} of", min.unwrap())
        } else {
            "".into()
        },
        flags.join("\n"),
        HELP.italic()
    )
}

/// ## Warn
/// Prints a warn.
pub fn warn(msg: String) {
    println!("{} {}", " Warn >".yellow().bold(), msg)
}

/// ## Info
/// Prints info.
pub fn info(msg: String) {
    println!("{} {}", "Info >".blue().bold(), msg)
}

/// ## Ok
/// Prints ok.
pub fn ok(msg: String) {
    println!("{} {}", "  OK >".green().bold(), msg)
}

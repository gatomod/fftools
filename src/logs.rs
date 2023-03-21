use colored::*;
use tabled::Style;
use tabled::TableIteratorExt;
use tabled::{format::Format, object::Columns, Modify};

const HELP: &str = "Use `--help` for more information.";

fn parse_table(input: &[[&str; 2]]) -> String {
    input
        .table()
        .with(Style::blank())
        .with(Modify::new(Columns::single(0)).with(Format::new(|s| s.bold().yellow().to_string())))
        .to_string()
        .lines()
        .skip(1)
        .map(|e| e)
        .collect::<Vec<&str>>()
        .join("\n    ")
}

/// ## Help
/// Help string with command list.
// TODO add help generator
pub fn help() {
    #[rustfmt::skip]
    let cmd_table = [
        ["o, optimize", "Reduce input TBN to reduce file size with not much loss of quality"],
        ["t, trim",     "Trims media between a range"],
        ["g, gif",      "Converts video into lossless GIF"],
        ["s, scale",    "Scale a file by percentage"],
        ["v, volume",   "Set audio volume"],
        ["m, merge",    "Merge audio and video"],
        ["   fps",      "Set framerate"],
        ["f, free",     "Dynamic subcommand"],
    ];

    #[rustfmt::skip]
    let io_table = [
        ["-i, --input <file>",  "Input file"],
        ["-w, --overwrite",     "Overwrite output"],
    ];

    #[rustfmt::skip]
    let manipulation_table = [
        ["-f, --from <time>",       "Start timestamp"],
        ["-t, --to <time>",         "End timestamp"],
        ["-s, --scale <percent>",   "Scale percentage"],
        ["    --fps <fps>",         "Set framerate"],
        ["-l, --volume <level>",    "Set volume (dB or percentage)"],
        ["-n, --noaudio",           "Silent media"],
        ["    --notrim",            "Avoid media trimming (useful for `trim` command)",],
    ];

    #[rustfmt::skip]
    let encode_table = [
        ["-o, --optimize",  "Reduces TBN"],
        ["-e, --encode",    "Encode input stream to output format"],
        ["    --vencode",       "Encode video"],
        ["    --aencode",       "Encode audio"],
        ["-c, --copy",      "Copy input stream to output (no encode)"],
        ["    --vcopy",         "Copy video"],
        ["    --acopy",         "Copy audio"],
    ];

    #[rustfmt::skip]
    let misc_table = [
        ["-V, --verbose",   "Add verbosity"],
        ["-h, --help",      "Show this message"],
        ["-v, --version",   "Show FFtools version"],
    ];

    println!(
        r#"{title}
FFMPEG-based toolkit for easy media manipulation

Copyright 2023 Gátomo
Licensed under the Apache License, Version 2.0
{apache}

{usage}
    {usage_struct}

{commands}
    {commands_table}

{options}
{io}
    {io_table}

{manipulation}
    {manipulation_table}

{encode}
    {encode_table}

{misc}
    {misc_table}

{doc_title}
See GitHub Wiki for detailed description of commands and options.
{wiki}
"#,
        title = "<< FFtools >>".cyan().bold(),
        apache = "< https://www.apache.org/licenses/LICENSE-2.0 >".italic(),
        usage = "USAGE".green().bold(),
        usage_struct = "fftools <SUBCOMMAND> -i <INPUT> [OPTIONS] <OUTPUT> [SUB_VALUES]".bold(),
        commands = "COMMANDS".green().bold(),
        commands_table = parse_table(&cmd_table),
        options = "OPTIONS/FLAGS".green().bold(),
        io = "IO management:".bold(),
        io_table = parse_table(&io_table),
        manipulation = "Media manipulation:".bold(),
        manipulation_table = parse_table(&manipulation_table),
        encode = "Media codification:".bold(),
        encode_table = parse_table(&encode_table),
        misc = "Miscellaneous:".bold(),
        misc_table = parse_table(&misc_table),
        doc_title = "Something is missing?".cyan().bold(),
        wiki = "< https://github.com/gatomo-oficial/fftools/wiki >".italic()
    );
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

/// ## flag_error
/// Prints a list of missing flags.
pub fn flag_error(cmd: &str, flags: Vec<&str>, min: Option<&str>) {
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

/// ## args_error
/// Prints a list of missing args.
pub fn args_error(cmd: &str) {
    eprintln!(
        "{} `{}` command requires a value\n{}",
        "Error >".red().bold(),
        cmd,
        HELP.italic()
    )
}

/// ## nan_error
/// Prints a NaN error
pub fn nan_error() {
    error("Please insert a valid number.".into(), None);
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

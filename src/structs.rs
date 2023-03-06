use std::path::PathBuf;

#[derive(Debug)]
pub struct Args {
    /// ## Command
    /// Specify the command to use
    ///
    /// - Required
    ///
    /// ### FFmpeg equivalent
    /// This is a FFtools feature
    pub command: String,

    /// ## Input
    /// Set the input file.
    ///
    /// - Required
    ///
    /// ### FFmpeg equivalent
    /// `-i`
    pub input: PathBuf,

    /// ## Output
    /// Set the output file.
    ///
    /// - Required
    ///
    /// ### FFmpeg equivalent
    /// *No flags.*
    pub output: PathBuf,

    /// ## From
    /// Seeks to start timestamp
    ///
    /// ### FFmpeg equivalent
    /// `-ss`
    pub from: Option<String>,

    /// ## To
    /// Seeks to end timestamp
    ///
    /// ### FFmpeg equivalent
    /// `-to`
    pub to: Option<String>,

    /// ## Verbose
    /// Prints the FFmpeg stdout/stderr
    ///
    /// ### FFmpeg equivalent
    /// This is a FFtools feature
    pub verbose: bool,

    /// ## Overwrite
    /// Overwrite the output if exists
    ///
    /// ### FFmpeg equivalent
    /// `-y`
    pub overwrite: bool,

    /// ## Overwrite
    /// Overwrite the output if exists
    ///
    /// ### FFmpeg equivalent
    /// `-r`
    // FIXME this should be u32. See `FIXME (1)` in main.rs
    pub fps: Option<String>,
}

use colored::*;

pub fn help() -> String {
    format!(
        "{}\n{}",
        "FFtools".cyan().bold(),
        "FFMPEG-based toolkit for manipulate multimedia easily"
    )
}

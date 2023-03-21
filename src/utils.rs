use std::{io, path::Path, process::Command};

// TODO add duration command
pub fn length(input: &Path) -> io::Result<String> {
    match Command::new("ffprobe")
        .args([
            "-i",
            &input.to_string_lossy(),
            "-show_entries",
            "format=duration",
            "-sexagesimal",
            "-v quiet",
            "-of",
            "csv=\"p=0\"",
        ])
        .output()
    {
        Err(err) => Err(err),
        Ok(data) => Ok(String::from_utf8_lossy(&data.stdout).to_string()),
    }
}

/// ### sts
/// Get string slice array and returns a string vector
pub fn sts(input: &[&str]) -> Vec<String> {
    let mut out: Vec<String> = vec![];

    for element in input.iter() {
        out.push(element.to_string());
    }

    out
}

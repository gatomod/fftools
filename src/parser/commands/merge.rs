use std::process::exit;

use crate::{logs, structs::Data};

pub fn merge(data: &mut Data) {
    if data.args.input.len() < 2 {
        logs::error("`merge` command requires at least two inputs.".into(), None);
        exit(1)
    }

    if !data.args.no_trim {
        data.ffmpeg_args.push("-shortest".into());
    }

    data.args.copy = true;
}

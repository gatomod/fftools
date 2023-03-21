use std::process::exit;

use crate::{logs, structs::Data};

pub fn volume(data: &mut Data) {
    match data
        .pico_parser
        .opt_free_from_str::<String>()
        .unwrap_or(None)
    {
        None => {
            logs::args_error("volume");
            exit(1)
        }
        Some(e) => {
            data.args.volume = Some(e);
        }
    }
}

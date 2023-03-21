use std::process::exit;

use crate::{logs, structs::Data};

pub fn fps(data: &mut Data) {
    match data
        .pico_parser
        .opt_free_from_str::<String>()
        .unwrap_or(None)
    {
        None => {
            logs::args_error("fps");
            exit(1)
        }
        Some(e) => {
            data.args.fps = Some(e.parse::<u64>().unwrap_or_else(|_| {
                logs::nan_error();
                exit(1)
            }));
        }
    }
}

use std::process::exit;

use crate::{logs, structs::Data};

pub fn scale(data: &mut Data) {
    match data
        .pico_parser
        .opt_free_from_str::<String>()
        .unwrap_or(None)
    {
        None => {
            logs::args_error("scale");
            exit(1)
        }
        Some(e) => {
            data.args.scale = Some(e.parse::<u64>().unwrap_or_else(|_| {
                logs::nan_error();
                exit(1)
            }));
        }
    }
}

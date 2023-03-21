use std::process::exit;

use crate::{logs, structs::Data};

pub fn optimize(data: &mut Data) {
    match data
        .pico_parser
        .opt_free_from_str::<String>()
        .unwrap_or(None)
    {
        None => {
            logs::args_error("optimize");
            exit(1)
        }
        Some(e) => {
            data.args.optimize = Some(e);
        }
    }
}

use std::process::exit;

use crate::{logs, structs::Data};

pub fn trim(data: &mut Data) {
    if data.args.from.is_none() && data.args.from.is_none() {
        logs::flag_error("trim", vec!["--from", "--to"], Some("one"));
        exit(1)
    }
}

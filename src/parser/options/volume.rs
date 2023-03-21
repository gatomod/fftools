use std::process::exit;

use crate::{logs, structs::Data};

pub fn volume(data: &mut Data, value: String) {
    let value = if value.ends_with("dB") {
        value.to_string()
    } else {
        (value.parse::<f64>().unwrap_or_else(|_| {
            logs::nan_error();
            exit(1);
        }) / 100.0)
            .to_string()
    };

    data.af_args.push(format!("volume={}", value));

    data.args.audio_encode = true;
}

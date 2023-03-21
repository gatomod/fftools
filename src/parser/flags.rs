mod copy;
mod encode;
mod no_audio;
mod no_trim;
mod overwrite;

use crate::structs::Data;

pub fn flag_parser(data: &mut Data) {
    let args = data.args.clone();

    if !args.overwrite {
        overwrite::overwrite(&args);
    }

    if args.no_audio {
        no_audio::no_audio(data);
    }

    if args.no_trim {
        no_trim::trim(data);
    }

    if args.encode {
        encode::encode(data);
    }

    if args.copy {
        copy::copy(data);
    }

    copy::audio_copy(data);
    copy::video_copy(data);
}

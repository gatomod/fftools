use crate::{structs::Data, utils::sts};

pub fn copy(data: &mut Data) {
    data.args.audio_copy = true;
    data.args.video_copy = true;
}

pub fn audio_copy(data: &mut Data) {
    if !data.args.audio_encode {
        data.ffmpeg_args.extend(sts(&["-c:a", "copy"]));
    }
}

pub fn video_copy(data: &mut Data) {
    if !data.args.video_encode {
        data.ffmpeg_args.extend(sts(&["-c:v", "copy"]));
    }
}

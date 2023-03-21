use crate::structs::Data;

pub fn fps(data: &mut Data, value: u64) {
    data.ffmpeg_args
        .extend_from_slice(&["-r".into(), value.to_string()]);

    data.args.video_encode = true;
}

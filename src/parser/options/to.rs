use crate::structs::Data;

pub fn to(data: &mut Data, value: String) {
    data.ffmpeg_args.extend_from_slice(&["-to".into(), value]);
}

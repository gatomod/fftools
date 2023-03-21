use crate::structs::Data;

pub fn from(data: &mut Data, value: String) {
    data.ffmpeg_args.extend_from_slice(&["-ss".into(), value]);
}

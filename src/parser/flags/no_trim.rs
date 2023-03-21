use crate::structs::Data;

pub fn trim(data: &mut Data) {
    data.ffmpeg_args.push("-shortest".into());
}

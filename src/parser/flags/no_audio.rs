use crate::structs::Data;

pub fn no_audio(data: &mut Data) {
    data.ffmpeg_args.push("-an".into());
}

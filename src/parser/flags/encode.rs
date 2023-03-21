use crate::structs::Data;

pub fn encode(data: &mut Data) {
    data.args.audio_encode = true;
    data.args.video_encode = true;
}

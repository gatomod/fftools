use crate::{structs::Data, utils::sts};

pub fn optimize(data: &mut Data, value: String) {
    data.ffmpeg_args
        .extend(sts(&["-video_track_timescale", value.as_str()]));

    data.args.video_encode = true;
}

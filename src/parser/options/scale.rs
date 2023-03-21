use crate::structs::Data;

pub fn scale(data: &mut Data, value: u64) {
    data.vf_args
        .push(format!("scale=iw*{0}:ih*{0}", value as f64 / 100_f64));

    data.args.video_encode = true;
}

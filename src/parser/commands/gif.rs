use crate::{structs::Data, utils::sts};

pub fn gif(data: &mut Data) {
    data.args.output.as_mut().unwrap().set_extension("gif");

    data.vf_args.extend(sts(&[
        "[0:v]split[a][b]",
        "[a]palettegen[p]",
        "[b][p]paletteuse",
    ]));

    data.args.encode = true;
}

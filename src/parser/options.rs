use crate::structs::Data;

mod fps;
mod from;
mod optimize;
mod scale;
mod to;
mod volume;

pub fn option_parser(data: &mut Data) {
    let args = data.args.clone();

    if let Some(v) = args.optimize {
        optimize::optimize(data, v);
    }

    if let Some(v) = args.from {
        from::from(data, v);
    }

    if let Some(v) = args.to {
        to::to(data, v);
    }

    if let Some(v) = args.scale {
        scale::scale(data, v);
    }

    if let Some(v) = args.scale {
        scale::scale(data, v);
    }

    if let Some(v) = args.volume {
        volume::volume(data, v);
    }

    if let Some(v) = args.fps {
        fps::fps(data, v)
    }
}

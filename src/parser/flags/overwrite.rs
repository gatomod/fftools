use std::process::exit;

use crate::{logs, structs::Args};

pub fn overwrite(args: &Args) {
    if args.output.is_some() {
        if args.output.as_ref().unwrap().exists() {
            logs::error(
                format!(
                    "File {:#?} already exists.",
                    args.output.as_ref().unwrap().file_name().unwrap()
                ),
                Some(" Use `--overwrite` or `-w` flag for overwrite.".into()),
            );
            exit(1)
        }
    }
}

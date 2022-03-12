use clap::Parser;
use std::thread;

use barr::barrel_write;

/// Barrel your JavaScript files! 🛢
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    /// The file extension to create
    #[clap(short, long)]
    extension: Option<String>,

    /// Files to process
    files: Vec<String>,
}

fn main() {
    let args = Args::parse();

    for f in args.files {
        let ext = args.extension.as_ref().cloned();

        thread::spawn(move || {
            barrel_write(&f, ext);
        });
    }
}

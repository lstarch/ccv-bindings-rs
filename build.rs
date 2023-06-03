extern crate pkg_config;

use std::process::Command;
use std::env;
use std::path::Path;

use std::fs::File;
use std::io::Write;

fn main() {
    let cur_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    let mut status = File::create("/tmp/cargo.log").unwrap();
    status.write_fmt(format_args!("Starting\n")).unwrap();

    let lib_dir = Path::new(&cur_dir)
        .join("ccv")
        .join("lib");
    status.write_fmt(format_args!("lib: {:?}\n", lib_dir)).unwrap();

    let configure_cmd = lib_dir.join("configure");
    status.write_fmt(format_args!("configure: {:?}\n", configure_cmd)).unwrap();
    Command::new(configure_cmd)
        .current_dir(lib_dir.clone())
        .status()
        .expect("Error in lib/configure");

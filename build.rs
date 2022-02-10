extern crate pkg_config;

use std::process::Command;
use std::env;
use std::path::Path;

use std::fs::File;
use std::io::Write;

fn main() {
    let cur_dir = env::var("CARGO_MANIFEST_DIR").unwrap
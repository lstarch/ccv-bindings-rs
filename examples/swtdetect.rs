
extern crate ccv;
// Trivial port of example swtdetect.c, as provided with ccv.

use std::env::args;
use std::time::*;

use ccv::*;
use ccv::swt::*;

fn main() {
    let mut args = std::env::args();
    let _ = args.next().unwrap();        // Unused: executable name.
    let file_in = args.next().expect("Expected an input file");
    let file_out = args.next().expect("Expected an output file");
    let mut pix = Matrix::read(file_in, OpenAs::ToGray).expect("Could not read image");

    let start = Instant::now();
    let words = pix.detect_words(Default::default());
    let duration = Instant::now() - start;
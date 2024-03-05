
extern crate ccv;
// Trivial port of example swtdetect.c, as provided with ccv.

use std::env::args;
use std::time::*;

use ccv::*;
use ccv::swt::*;

fn main() {
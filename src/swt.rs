
use ffi;
use matrix::*;

pub use ffi::Rect;
pub use ffi::SwtParams;

use std::ptr::null_mut;

impl Default for SwtParams {
    fn default() -> Self {
        SwtParams {
            direction: 1,
            interval: 1,
            same_word_thresh: [ 0.1, 0.8 ],
            min_neighbors: 1,
            scale_invariant: 0,
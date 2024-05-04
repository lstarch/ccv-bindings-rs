
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
            size: 3,
            low_thresh: 124,
            high_thresh: 204,
            max_height: 300,
            min_height: 8,
            min_area: 38,
            letter_occlude_thresh: 3,
            aspect_ratio: 8.,
            std_ratio: 0.83,
            thickness_ratio: 1.5,
            height_ratio: 1.7,
            intensity_thresh: 31,
            distance_ratio: 2.9,
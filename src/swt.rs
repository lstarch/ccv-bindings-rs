
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
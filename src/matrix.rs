
use std::ffi::CString;
use std::path::Path;
use std::ptr::null_mut;

use libc::*;

use ffi;

pub struct Matrix(*mut ffi::DenseMatrix);

use std::ffi::CString;
use std::path::Path;
use std::ptr::null_mut;

use libc::*;

use ffi;

pub struct Matrix(*mut ffi::DenseMatrix);

pub use ffi::FileFormat;

pub enum OpenAs {
    Any,
    ToGray,
    ToColor
}

// Don't expose this to the public.
pub trait TPrivate {
    fn from_c(ptr: *mut ffi::DenseMatrix) -> Matrix;
    fn as_c(&mut self) -> *mut ffi::DenseMatrix;
}

impl TPrivate for Matrix {
    fn from_c(ptr: *mut ffi::DenseMatrix) -> Matrix {
       assert!(!ptr.is_null());
       Matrix(ptr)
   }
   fn as_c(&mut self) -> *mut ffi::DenseMatrix {
       self.0
   }
}

impl Drop for Matrix {
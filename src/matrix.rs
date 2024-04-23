
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
    fn drop(&mut self) {
        unsafe {
            ffi::ccv_matrix_free(self.0);
        }
    }
}

impl Matrix {
    pub fn read<P: AsRef<Path>>(path: P, params: OpenAs) -> Option<Matrix> {
        let path : &str = path.as_ref().to_str().unwrap(); // FIXME: Better error reporting.
        let c_path = CString::new(path).unwrap().as_ptr(); // FIXME: Better error reporting.
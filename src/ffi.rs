
use libc::*;

#[repr(C)]
pub struct DenseMatrix {
    #[allow(dead_code)]
    placeholder: c_int,
}


#[repr(C)]
pub enum FileType {
    AnyFile = 0x020,
    Gray = 0x100,
    Color = 0x300
}

#[repr(C)]
pub enum FileFormat {
    BMP = 0x021,
    JPEG = 0x022,
    PNG = 0x023,
}

#[repr(C)]
pub enum Depth {
    U8 = 0x01000,
    S32 = 0x02000,
    F32 = 0x04000,
    S64 = 0x08000,
    F64 = 0x10000,
}
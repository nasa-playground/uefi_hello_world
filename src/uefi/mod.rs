use core::ffi::c_void;

pub struct Handle(*mut c_void);

// NOTE
// UINT64 Signature;
// UINT32 Revision;
// UINT32 HeaderSize;
// UINT32 CRC32;
// UINT32 Reserved;
#[repr(C)]
struct Header {
    signature: u64,
    revision: u32,
    header_size: u32,
    crc32: u32,
    reserved: u32,
}
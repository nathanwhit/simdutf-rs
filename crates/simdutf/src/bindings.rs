// Auto generated by codegen/src/bindings.rs
extern "C" {

    pub fn simdutf_validate_ascii(buf: *const u8, len: usize) -> bool;

    pub fn simdutf_validate_utf8(buf: *const u8, len: usize) -> bool;

    pub fn simdutf_validate_utf16(buf: *const u16, len: usize) -> bool;

    pub fn simdutf_validate_utf16be(buf: *const u16, len: usize) -> bool;

    pub fn simdutf_validate_utf16le(buf: *const u16, len: usize) -> bool;

    pub fn simdutf_validate_utf32(buf: *const u32, len: usize) -> bool;

}

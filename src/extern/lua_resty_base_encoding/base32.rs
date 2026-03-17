extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub const CHARPAD: ::core::ffi::c_int = '=' as i32;
pub const BADCHAR: ::core::ffi::c_int = 0xff as ::core::ffi::c_int;
static mut std_encode_table: *const ::core::ffi::c_char =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567=\0".as_ptr() as *const ::core::ffi::c_char;
static mut std_decode_table: [::core::ffi::c_char; 256] = [
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    0x1a as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1b as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1c as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1d as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1e as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x4 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x5 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x8 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_char,
    0xa as ::core::ffi::c_int as ::core::ffi::c_char,
    0xb as ::core::ffi::c_int as ::core::ffi::c_char,
    0xc as ::core::ffi::c_int as ::core::ffi::c_char,
    0xd as ::core::ffi::c_int as ::core::ffi::c_char,
    0xe as ::core::ffi::c_int as ::core::ffi::c_char,
    0xf as ::core::ffi::c_int as ::core::ffi::c_char,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x12 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x13 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x14 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x15 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x16 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x17 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x18 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x19 as ::core::ffi::c_int as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x4 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x5 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x8 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_char,
    0xa as ::core::ffi::c_int as ::core::ffi::c_char,
    0xb as ::core::ffi::c_int as ::core::ffi::c_char,
    0xc as ::core::ffi::c_int as ::core::ffi::c_char,
    0xd as ::core::ffi::c_int as ::core::ffi::c_char,
    0xe as ::core::ffi::c_int as ::core::ffi::c_char,
    0xf as ::core::ffi::c_int as ::core::ffi::c_char,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x12 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x13 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x14 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x15 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x16 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x17 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x18 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x19 as ::core::ffi::c_int as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x4 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x5 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x8 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_char,
    0xa as ::core::ffi::c_int as ::core::ffi::c_char,
    0xb as ::core::ffi::c_int as ::core::ffi::c_char,
    0xc as ::core::ffi::c_int as ::core::ffi::c_char,
    0xd as ::core::ffi::c_int as ::core::ffi::c_char,
    0xe as ::core::ffi::c_int as ::core::ffi::c_char,
    0xf as ::core::ffi::c_int as ::core::ffi::c_char,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x12 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x13 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x14 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x15 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x16 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x17 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x18 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x19 as ::core::ffi::c_int as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
static mut hex_encode_table: *const ::core::ffi::c_char =
    b"0123456789ABCDEFGHIJKLMNOPQRSTUV=\0".as_ptr() as *const ::core::ffi::c_char;
static mut hex_decode_table: [::core::ffi::c_char; 256] = [
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    0 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x2 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x3 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x4 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x5 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x6 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x7 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x8 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x9 as ::core::ffi::c_int as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    0xa as ::core::ffi::c_int as ::core::ffi::c_char,
    0xb as ::core::ffi::c_int as ::core::ffi::c_char,
    0xc as ::core::ffi::c_int as ::core::ffi::c_char,
    0xd as ::core::ffi::c_int as ::core::ffi::c_char,
    0xe as ::core::ffi::c_int as ::core::ffi::c_char,
    0xf as ::core::ffi::c_int as ::core::ffi::c_char,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x12 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x13 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x14 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x15 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x16 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x17 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x18 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x19 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1a as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1b as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1c as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1d as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1e as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    0xa as ::core::ffi::c_int as ::core::ffi::c_char,
    0xb as ::core::ffi::c_int as ::core::ffi::c_char,
    0xc as ::core::ffi::c_int as ::core::ffi::c_char,
    0xd as ::core::ffi::c_int as ::core::ffi::c_char,
    0xe as ::core::ffi::c_int as ::core::ffi::c_char,
    0xf as ::core::ffi::c_int as ::core::ffi::c_char,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x12 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x13 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x14 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x15 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x16 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x17 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x18 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x19 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1a as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1b as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1c as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1d as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1e as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    0xa as ::core::ffi::c_int as ::core::ffi::c_char,
    0xb as ::core::ffi::c_int as ::core::ffi::c_char,
    0xc as ::core::ffi::c_int as ::core::ffi::c_char,
    0xd as ::core::ffi::c_int as ::core::ffi::c_char,
    0xe as ::core::ffi::c_int as ::core::ffi::c_char,
    0xf as ::core::ffi::c_int as ::core::ffi::c_char,
    0x10 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x11 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x12 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x13 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x14 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x15 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x16 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x17 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x18 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x19 as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1a as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1b as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1c as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1d as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1e as ::core::ffi::c_int as ::core::ffi::c_char,
    0x1f as ::core::ffi::c_int as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    BADCHAR as ::core::ffi::c_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
pub const IN_BLOCK_LEN: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const OUT_BLOCK_LEN: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn b32_encode(
    mut dest: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut len: size_t,
    mut no_padding: uint32_t,
    mut hex: uint32_t,
) -> size_t {
    let mut n1: ::core::ffi::c_uchar = 0;
    let mut n2: ::core::ffi::c_uchar = 0;
    let mut n3: ::core::ffi::c_uchar = 0;
    let mut n4: ::core::ffi::c_uchar = 0;
    let mut n5: ::core::ffi::c_uchar = 0;
    let mut n6: ::core::ffi::c_uchar = 0;
    let mut n7: ::core::ffi::c_uchar = 0;
    let mut n8: ::core::ffi::c_uchar = 0;
    let mut s: *const uint8_t = src as *const uint8_t;
    let mut p: *mut uint8_t = dest as *mut uint8_t;
    let mut encode_table: *const ::core::ffi::c_char = if hex == 0 as uint32_t {
        std_encode_table
    } else {
        hex_encode_table
    };
    while (len >= 5 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        n8 = (*s.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x1f as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        n7 = ((*s.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xe0 as ::core::ffi::c_int)
            >> 5 as ::core::ffi::c_int
            | (*s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x3 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        n6 = ((*s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x7c as ::core::ffi::c_int)
            >> 2 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        n5 = ((*s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int)
            >> 7 as ::core::ffi::c_int
            | (*s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        n4 = ((*s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xf0 as ::core::ffi::c_int)
            >> 4 as ::core::ffi::c_int
            | (*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x1 as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        n3 = ((*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x3e as ::core::ffi::c_int)
            >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        n2 = ((*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xc0 as ::core::ffi::c_int)
            >> 6 as ::core::ffi::c_int
            | (*s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x7 as ::core::ffi::c_int)
                << 2 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        n1 = ((*s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0xf8 as ::core::ffi::c_int)
            >> 3 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
        let c2rust_fresh0 = p;
        p = p.offset(1);
        *c2rust_fresh0 = *encode_table.offset(n1 as isize) as uint8_t;
        let c2rust_fresh1 = p;
        p = p.offset(1);
        *c2rust_fresh1 = *encode_table.offset(n2 as isize) as uint8_t;
        let c2rust_fresh2 = p;
        p = p.offset(1);
        *c2rust_fresh2 = *encode_table.offset(n3 as isize) as uint8_t;
        let c2rust_fresh3 = p;
        p = p.offset(1);
        *c2rust_fresh3 = *encode_table.offset(n4 as isize) as uint8_t;
        let c2rust_fresh4 = p;
        p = p.offset(1);
        *c2rust_fresh4 = *encode_table.offset(n5 as isize) as uint8_t;
        let c2rust_fresh5 = p;
        p = p.offset(1);
        *c2rust_fresh5 = *encode_table.offset(n6 as isize) as uint8_t;
        let c2rust_fresh6 = p;
        p = p.offset(1);
        *c2rust_fresh6 = *encode_table.offset(n7 as isize) as uint8_t;
        let c2rust_fresh7 = p;
        p = p.offset(1);
        *c2rust_fresh7 = *encode_table.offset(n8 as isize) as uint8_t;
        s = s.offset(IN_BLOCK_LEN as isize);
        len = len.wrapping_sub(IN_BLOCK_LEN as size_t);
    }
    n8 = 0 as ::core::ffi::c_uchar;
    n7 = n8;
    n6 = n7;
    n5 = n6;
    n4 = n5;
    n3 = n4;
    n2 = n3;
    n1 = n2;
    let mut step: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut padding_start: *mut uint8_t = p;
    let mut c2rust_current_block_42: u64;
    match len {
        4 => {
            n7 = ((*s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x3 as ::core::ffi::c_int)
                << 3 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            n6 = ((*s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x7c as ::core::ffi::c_int)
                >> 2 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            n5 = ((*s.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int)
                >> 7 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *p.offset(6 as ::core::ffi::c_int as isize) =
                *encode_table.offset(n7 as isize) as uint8_t;
            *p.offset(5 as ::core::ffi::c_int as isize) =
                *encode_table.offset(n6 as isize) as uint8_t;
            step += 2 as ::core::ffi::c_int;
            c2rust_current_block_42 = 15654777605002825172;
        }
        3 => {
            c2rust_current_block_42 = 15654777605002825172;
        }
        2 => {
            c2rust_current_block_42 = 9832539432606915957;
        }
        1 => {
            c2rust_current_block_42 = 16063830846081352862;
        }
        0 => return p.offset_from(dest as *mut uint8_t) as ::core::ffi::c_long as size_t,
        _ => {
            c2rust_current_block_42 = 5494826135382683477;
        }
    }
    match c2rust_current_block_42 {
        15654777605002825172 => {
            n5 = (n5 as ::core::ffi::c_int
                | (*s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0xf as ::core::ffi::c_int)
                    << 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            n4 = ((*s.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf0 as ::core::ffi::c_int)
                >> 4 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *p.offset(4 as ::core::ffi::c_int as isize) =
                *encode_table.offset(n5 as isize) as uint8_t;
            step += 1 as ::core::ffi::c_int;
            c2rust_current_block_42 = 9832539432606915957;
        }
        _ => {}
    }
    match c2rust_current_block_42 {
        9832539432606915957 => {
            n4 = (n4 as ::core::ffi::c_int
                | (*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x1 as ::core::ffi::c_int)
                    << 4 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            n3 = ((*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x3e as ::core::ffi::c_int)
                >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            n2 = ((*s.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xc0 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *p.offset(3 as ::core::ffi::c_int as isize) =
                *encode_table.offset(n4 as isize) as uint8_t;
            *p.offset(2 as ::core::ffi::c_int as isize) =
                *encode_table.offset(n3 as isize) as uint8_t;
            step += 2 as ::core::ffi::c_int;
            c2rust_current_block_42 = 16063830846081352862;
        }
        _ => {}
    }
    match c2rust_current_block_42 {
        16063830846081352862 => {
            n2 = (n2 as ::core::ffi::c_int
                | (*s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    & 0x7 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            n1 = ((*s.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xf8 as ::core::ffi::c_int)
                >> 3 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            *p.offset(1 as ::core::ffi::c_int as isize) =
                *encode_table.offset(n2 as isize) as uint8_t;
            *p.offset(0 as ::core::ffi::c_int as isize) =
                *encode_table.offset(n1 as isize) as uint8_t;
            step += 2 as ::core::ffi::c_int;
        }
        _ => {}
    }
    p = p.offset(step as isize);
    if no_padding == 0 {
        memset(
            p as *mut ::core::ffi::c_void,
            CHARPAD,
            padding_start
                .offset(8 as ::core::ffi::c_int as isize)
                .offset_from(p) as ::core::ffi::c_long as size_t,
        );
        p = padding_start.offset(8 as ::core::ffi::c_int as isize);
    }
    return p.offset_from(dest as *mut uint8_t) as ::core::ffi::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn b32_decode(
    mut dest: *mut ::core::ffi::c_char,
    mut src: *const ::core::ffi::c_char,
    mut len: size_t,
    mut hex: uint32_t,
) -> size_t {
    let mut in1: ::core::ffi::c_uchar = 0;
    let mut in2: ::core::ffi::c_uchar = 0;
    let mut in3: ::core::ffi::c_uchar = 0;
    let mut in4: ::core::ffi::c_uchar = 0;
    let mut in5: ::core::ffi::c_uchar = 0;
    let mut in6: ::core::ffi::c_uchar = 0;
    let mut in7: ::core::ffi::c_uchar = 0;
    let mut in8: ::core::ffi::c_uchar = 0;
    let mut s: *const ::core::ffi::c_uchar = src as *const ::core::ffi::c_uchar;
    let mut p: *mut ::core::ffi::c_uchar = dest as *mut ::core::ffi::c_uchar;
    let mut decode_table: *const ::core::ffi::c_char = if hex == 0 as uint32_t {
        &raw const std_decode_table as *const ::core::ffi::c_char
    } else {
        &raw const hex_decode_table as *const ::core::ffi::c_char
    };
    if *src.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int == CHARPAD {
        if len < 8 as size_t || len.wrapping_rem(8 as size_t) != 0 as size_t {
            return -(1 as ::core::ffi::c_int) as size_t;
        }
        len = len.wrapping_sub(1);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        i = 0 as ::core::ffi::c_int;
        while *src.offset(len.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int == CHARPAD {
            if i >= 6 as ::core::ffi::c_int {
                return -(1 as ::core::ffi::c_int) as size_t;
            }
            len = len.wrapping_sub(1);
            i += 1;
        }
    }
    while (len >= 8 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        let c2rust_fresh8 = s;
        s = s.offset(1);
        in1 = *decode_table.offset(*c2rust_fresh8 as isize) as ::core::ffi::c_uchar;
        let c2rust_fresh9 = s;
        s = s.offset(1);
        in2 = *decode_table.offset(*c2rust_fresh9 as isize) as ::core::ffi::c_uchar;
        let c2rust_fresh10 = s;
        s = s.offset(1);
        in3 = *decode_table.offset(*c2rust_fresh10 as isize) as ::core::ffi::c_uchar;
        let c2rust_fresh11 = s;
        s = s.offset(1);
        in4 = *decode_table.offset(*c2rust_fresh11 as isize) as ::core::ffi::c_uchar;
        let c2rust_fresh12 = s;
        s = s.offset(1);
        in5 = *decode_table.offset(*c2rust_fresh12 as isize) as ::core::ffi::c_uchar;
        let c2rust_fresh13 = s;
        s = s.offset(1);
        in6 = *decode_table.offset(*c2rust_fresh13 as isize) as ::core::ffi::c_uchar;
        let c2rust_fresh14 = s;
        s = s.offset(1);
        in7 = *decode_table.offset(*c2rust_fresh14 as isize) as ::core::ffi::c_uchar;
        let c2rust_fresh15 = s;
        s = s.offset(1);
        in8 = *decode_table.offset(*c2rust_fresh15 as isize) as ::core::ffi::c_uchar;
        if (in1 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
            || in2 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
            || in3 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
            || in4 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
            || in5 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
            || in6 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
            || in7 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int
            || in8 as ::core::ffi::c_int == 0xff as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            return -(1 as ::core::ffi::c_int) as size_t;
        }
        let c2rust_fresh16 = p;
        p = p.offset(1);
        *c2rust_fresh16 = ((in1 as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
            << 3 as ::core::ffi::c_int
            | (in2 as ::core::ffi::c_int & 0x1c as ::core::ffi::c_int) >> 2 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        let c2rust_fresh17 = p;
        p = p.offset(1);
        *c2rust_fresh17 = ((in2 as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int)
            << 6 as ::core::ffi::c_int
            | (in3 as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) << 1 as ::core::ffi::c_int
            | (in4 as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int) >> 4 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        let c2rust_fresh18 = p;
        p = p.offset(1);
        *c2rust_fresh18 = ((in4 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int)
            << 4 as ::core::ffi::c_int
            | (in5 as ::core::ffi::c_int & 0x1e as ::core::ffi::c_int) >> 1 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        let c2rust_fresh19 = p;
        p = p.offset(1);
        *c2rust_fresh19 = ((in5 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int)
            << 7 as ::core::ffi::c_int
            | (in6 as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
            | (in7 as ::core::ffi::c_int & 0x18 as ::core::ffi::c_int) >> 3 as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        let c2rust_fresh20 = p;
        p = p.offset(1);
        *c2rust_fresh20 = ((in7 as ::core::ffi::c_int & 0x7 as ::core::ffi::c_int)
            << 5 as ::core::ffi::c_int
            | in8 as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
            as ::core::ffi::c_uchar;
        len = len.wrapping_sub(OUT_BLOCK_LEN as size_t);
    }
    let mut step: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i_0: ::core::ffi::c_uint = 0;
    i_0 = 0 as ::core::ffi::c_uint;
    while (i_0 as size_t) < len {
        if *decode_table.offset(*s.offset(i_0 as isize) as isize) as ::core::ffi::c_uchar
            as ::core::ffi::c_int
            == BADCHAR
        {
            return -(1 as ::core::ffi::c_int) as size_t;
        }
        i_0 = i_0.wrapping_add(1);
    }
    let mut c2rust_current_block_56: u64;
    match len {
        7 => {
            in5 = *decode_table.offset(*s.offset(4 as ::core::ffi::c_int as isize) as isize)
                as ::core::ffi::c_uchar;
            in6 = *decode_table.offset(*s.offset(5 as ::core::ffi::c_int as isize) as isize)
                as ::core::ffi::c_uchar;
            in7 = *decode_table.offset(*s.offset(6 as ::core::ffi::c_int as isize) as isize)
                as ::core::ffi::c_uchar;
            *p.offset(3 as ::core::ffi::c_int as isize) =
                ((in5 as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int) << 7 as ::core::ffi::c_int
                    | (in6 as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
                        << 2 as ::core::ffi::c_int
                    | (in7 as ::core::ffi::c_int & 0x18 as ::core::ffi::c_int)
                        >> 3 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            step += 1;
            c2rust_current_block_56 = 1701664287637484839;
        }
        5 => {
            c2rust_current_block_56 = 1701664287637484839;
        }
        4 => {
            c2rust_current_block_56 = 16190753031682270451;
        }
        2 => {
            c2rust_current_block_56 = 10061427371901828084;
        }
        0 => {
            c2rust_current_block_56 = 1622411330066726685;
        }
        _ => return -(1 as ::core::ffi::c_int) as size_t,
    }
    match c2rust_current_block_56 {
        1701664287637484839 => {
            in5 = *decode_table.offset(*s.offset(4 as ::core::ffi::c_int as isize) as isize)
                as ::core::ffi::c_uchar;
            in4 = *decode_table.offset(*s.offset(3 as ::core::ffi::c_int as isize) as isize)
                as ::core::ffi::c_uchar;
            *p.offset(2 as ::core::ffi::c_int as isize) =
                ((in4 as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | (in5 as ::core::ffi::c_int & 0x1e as ::core::ffi::c_int)
                        >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            step += 1;
            c2rust_current_block_56 = 16190753031682270451;
        }
        _ => {}
    }
    match c2rust_current_block_56 {
        16190753031682270451 => {
            in4 = *decode_table.offset(*s.offset(3 as ::core::ffi::c_int as isize) as isize)
                as ::core::ffi::c_uchar;
            in3 = *decode_table.offset(*s.offset(2 as ::core::ffi::c_int as isize) as isize)
                as ::core::ffi::c_uchar;
            in2 = *decode_table.offset(*s.offset(1 as ::core::ffi::c_int as isize) as isize)
                as ::core::ffi::c_uchar;
            *p.offset(1 as ::core::ffi::c_int as isize) =
                ((in2 as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (in3 as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
                        << 1 as ::core::ffi::c_int
                    | (in4 as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int)
                        >> 4 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            step += 1;
            c2rust_current_block_56 = 10061427371901828084;
        }
        _ => {}
    }
    match c2rust_current_block_56 {
        10061427371901828084 => {
            in2 = *decode_table.offset(*s.offset(1 as ::core::ffi::c_int as isize) as isize)
                as ::core::ffi::c_uchar;
            in1 = *decode_table.offset(*s.offset(0 as ::core::ffi::c_int as isize) as isize)
                as ::core::ffi::c_uchar;
            *p.offset(0 as ::core::ffi::c_int as isize) =
                ((in1 as ::core::ffi::c_int & 0x1f as ::core::ffi::c_int)
                    << 3 as ::core::ffi::c_int
                    | (in2 as ::core::ffi::c_int & 0x1c as ::core::ffi::c_int)
                        >> 2 as ::core::ffi::c_int) as ::core::ffi::c_uchar;
            step += 1;
        }
        _ => {}
    }
    return (p.offset_from(dest as *mut uint8_t) as ::core::ffi::c_long as size_t)
        .wrapping_add(step as size_t);
}

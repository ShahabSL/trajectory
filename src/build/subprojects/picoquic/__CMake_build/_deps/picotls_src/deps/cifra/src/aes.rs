extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn abort() -> !;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type cf_prp_block =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const uint8_t, *mut uint8_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_prp {
    pub blocksz: size_t,
    pub encrypt: cf_prp_block,
    pub decrypt: cf_prp_block,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_aes_context {
    pub rounds: uint32_t,
    pub ks: [uint32_t; 60],
}
pub const AES_BLOCKSZ: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const AES128_ROUNDS: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AES192_ROUNDS: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const AES256_ROUNDS: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn mem_clean(mut v: *mut ::core::ffi::c_void, mut len: size_t) {
    if len != 0 {
        memset(v as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, len);
        *(v as *mut uint8_t);
    }
}
#[inline]
unsafe extern "C" fn rotr32(mut x: uint32_t, mut n: ::core::ffi::c_uint) -> uint32_t {
    return x >> n | x << (32 as ::core::ffi::c_uint).wrapping_sub(n);
}
#[inline]
unsafe extern "C" fn rotl32(mut x: uint32_t, mut n: ::core::ffi::c_uint) -> uint32_t {
    return x << n | x >> (32 as ::core::ffi::c_uint).wrapping_sub(n);
}
#[inline]
unsafe extern "C" fn read32_be(mut buf: *const uint8_t) -> uint32_t {
    return (*buf.offset(0 as ::core::ffi::c_int as isize) as uint32_t) << 24 as ::core::ffi::c_int
        | (*buf.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 16 as ::core::ffi::c_int
        | (*buf.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int
        | *buf.offset(3 as ::core::ffi::c_int as isize) as uint32_t;
}
#[inline]
unsafe extern "C" fn write32_be(mut v: uint32_t, mut buf: *mut uint8_t) {
    let c2rust_fresh0 = buf;
    buf = buf.offset(1);
    *c2rust_fresh0 = (v >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    let c2rust_fresh1 = buf;
    buf = buf.offset(1);
    *c2rust_fresh1 = (v >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    let c2rust_fresh2 = buf;
    buf = buf.offset(1);
    *c2rust_fresh2 = (v >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    *buf = (v & 0xff as uint32_t) as uint8_t;
}
#[inline]
unsafe extern "C" fn mask_u8(mut x: uint32_t, mut y: uint32_t) -> uint8_t {
    let mut diff: uint32_t = x ^ y;
    let mut diff_is_zero: uint8_t = (!diff & diff.wrapping_sub(1 as uint32_t)) as uint8_t;
    return -(diff_is_zero as ::core::ffi::c_int >> 7 as ::core::ffi::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn select_u8x4(
    mut a: *mut uint8_t,
    mut b: *mut uint8_t,
    mut c: *mut uint8_t,
    mut d: *mut uint8_t,
    mut tab: *const uint8_t,
    mut n: uint32_t,
) {
    let mut ra: uint8_t = 0 as uint8_t;
    let mut rb: uint8_t = 0 as uint8_t;
    let mut rc: uint8_t = 0 as uint8_t;
    let mut rd: uint8_t = 0 as uint8_t;
    let mut mask: uint8_t = 0;
    let mut i: uint32_t = 0;
    i = 0 as uint32_t;
    while i < n {
        let mut item: uint8_t = *tab.offset(i as isize);
        mask = mask_u8(*a as uint32_t, i);
        ra = (ra as ::core::ffi::c_int & !(mask as ::core::ffi::c_int)
            | item as ::core::ffi::c_int & mask as ::core::ffi::c_int) as uint8_t;
        mask = mask_u8(*b as uint32_t, i);
        rb = (rb as ::core::ffi::c_int & !(mask as ::core::ffi::c_int)
            | item as ::core::ffi::c_int & mask as ::core::ffi::c_int) as uint8_t;
        mask = mask_u8(*c as uint32_t, i);
        rc = (rc as ::core::ffi::c_int & !(mask as ::core::ffi::c_int)
            | item as ::core::ffi::c_int & mask as ::core::ffi::c_int) as uint8_t;
        mask = mask_u8(*d as uint32_t, i);
        rd = (rd as ::core::ffi::c_int & !(mask as ::core::ffi::c_int)
            | item as ::core::ffi::c_int & mask as ::core::ffi::c_int) as uint8_t;
        i = i.wrapping_add(1);
    }
    *a = ra;
    *b = rb;
    *c = rc;
    *d = rd;
}
static mut S: [uint8_t; 256] = [
    0x63 as ::core::ffi::c_int as uint8_t,
    0x7c as ::core::ffi::c_int as uint8_t,
    0x77 as ::core::ffi::c_int as uint8_t,
    0x7b as ::core::ffi::c_int as uint8_t,
    0xf2 as ::core::ffi::c_int as uint8_t,
    0x6b as ::core::ffi::c_int as uint8_t,
    0x6f as ::core::ffi::c_int as uint8_t,
    0xc5 as ::core::ffi::c_int as uint8_t,
    0x30 as ::core::ffi::c_int as uint8_t,
    0x1 as ::core::ffi::c_int as uint8_t,
    0x67 as ::core::ffi::c_int as uint8_t,
    0x2b as ::core::ffi::c_int as uint8_t,
    0xfe as ::core::ffi::c_int as uint8_t,
    0xd7 as ::core::ffi::c_int as uint8_t,
    0xab as ::core::ffi::c_int as uint8_t,
    0x76 as ::core::ffi::c_int as uint8_t,
    0xca as ::core::ffi::c_int as uint8_t,
    0x82 as ::core::ffi::c_int as uint8_t,
    0xc9 as ::core::ffi::c_int as uint8_t,
    0x7d as ::core::ffi::c_int as uint8_t,
    0xfa as ::core::ffi::c_int as uint8_t,
    0x59 as ::core::ffi::c_int as uint8_t,
    0x47 as ::core::ffi::c_int as uint8_t,
    0xf0 as ::core::ffi::c_int as uint8_t,
    0xad as ::core::ffi::c_int as uint8_t,
    0xd4 as ::core::ffi::c_int as uint8_t,
    0xa2 as ::core::ffi::c_int as uint8_t,
    0xaf as ::core::ffi::c_int as uint8_t,
    0x9c as ::core::ffi::c_int as uint8_t,
    0xa4 as ::core::ffi::c_int as uint8_t,
    0x72 as ::core::ffi::c_int as uint8_t,
    0xc0 as ::core::ffi::c_int as uint8_t,
    0xb7 as ::core::ffi::c_int as uint8_t,
    0xfd as ::core::ffi::c_int as uint8_t,
    0x93 as ::core::ffi::c_int as uint8_t,
    0x26 as ::core::ffi::c_int as uint8_t,
    0x36 as ::core::ffi::c_int as uint8_t,
    0x3f as ::core::ffi::c_int as uint8_t,
    0xf7 as ::core::ffi::c_int as uint8_t,
    0xcc as ::core::ffi::c_int as uint8_t,
    0x34 as ::core::ffi::c_int as uint8_t,
    0xa5 as ::core::ffi::c_int as uint8_t,
    0xe5 as ::core::ffi::c_int as uint8_t,
    0xf1 as ::core::ffi::c_int as uint8_t,
    0x71 as ::core::ffi::c_int as uint8_t,
    0xd8 as ::core::ffi::c_int as uint8_t,
    0x31 as ::core::ffi::c_int as uint8_t,
    0x15 as ::core::ffi::c_int as uint8_t,
    0x4 as ::core::ffi::c_int as uint8_t,
    0xc7 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0xc3 as ::core::ffi::c_int as uint8_t,
    0x18 as ::core::ffi::c_int as uint8_t,
    0x96 as ::core::ffi::c_int as uint8_t,
    0x5 as ::core::ffi::c_int as uint8_t,
    0x9a as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x12 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0xe2 as ::core::ffi::c_int as uint8_t,
    0xeb as ::core::ffi::c_int as uint8_t,
    0x27 as ::core::ffi::c_int as uint8_t,
    0xb2 as ::core::ffi::c_int as uint8_t,
    0x75 as ::core::ffi::c_int as uint8_t,
    0x9 as ::core::ffi::c_int as uint8_t,
    0x83 as ::core::ffi::c_int as uint8_t,
    0x2c as ::core::ffi::c_int as uint8_t,
    0x1a as ::core::ffi::c_int as uint8_t,
    0x1b as ::core::ffi::c_int as uint8_t,
    0x6e as ::core::ffi::c_int as uint8_t,
    0x5a as ::core::ffi::c_int as uint8_t,
    0xa0 as ::core::ffi::c_int as uint8_t,
    0x52 as ::core::ffi::c_int as uint8_t,
    0x3b as ::core::ffi::c_int as uint8_t,
    0xd6 as ::core::ffi::c_int as uint8_t,
    0xb3 as ::core::ffi::c_int as uint8_t,
    0x29 as ::core::ffi::c_int as uint8_t,
    0xe3 as ::core::ffi::c_int as uint8_t,
    0x2f as ::core::ffi::c_int as uint8_t,
    0x84 as ::core::ffi::c_int as uint8_t,
    0x53 as ::core::ffi::c_int as uint8_t,
    0xd1 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0xed as ::core::ffi::c_int as uint8_t,
    0x20 as ::core::ffi::c_int as uint8_t,
    0xfc as ::core::ffi::c_int as uint8_t,
    0xb1 as ::core::ffi::c_int as uint8_t,
    0x5b as ::core::ffi::c_int as uint8_t,
    0x6a as ::core::ffi::c_int as uint8_t,
    0xcb as ::core::ffi::c_int as uint8_t,
    0xbe as ::core::ffi::c_int as uint8_t,
    0x39 as ::core::ffi::c_int as uint8_t,
    0x4a as ::core::ffi::c_int as uint8_t,
    0x4c as ::core::ffi::c_int as uint8_t,
    0x58 as ::core::ffi::c_int as uint8_t,
    0xcf as ::core::ffi::c_int as uint8_t,
    0xd0 as ::core::ffi::c_int as uint8_t,
    0xef as ::core::ffi::c_int as uint8_t,
    0xaa as ::core::ffi::c_int as uint8_t,
    0xfb as ::core::ffi::c_int as uint8_t,
    0x43 as ::core::ffi::c_int as uint8_t,
    0x4d as ::core::ffi::c_int as uint8_t,
    0x33 as ::core::ffi::c_int as uint8_t,
    0x85 as ::core::ffi::c_int as uint8_t,
    0x45 as ::core::ffi::c_int as uint8_t,
    0xf9 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x7f as ::core::ffi::c_int as uint8_t,
    0x50 as ::core::ffi::c_int as uint8_t,
    0x3c as ::core::ffi::c_int as uint8_t,
    0x9f as ::core::ffi::c_int as uint8_t,
    0xa8 as ::core::ffi::c_int as uint8_t,
    0x51 as ::core::ffi::c_int as uint8_t,
    0xa3 as ::core::ffi::c_int as uint8_t,
    0x40 as ::core::ffi::c_int as uint8_t,
    0x8f as ::core::ffi::c_int as uint8_t,
    0x92 as ::core::ffi::c_int as uint8_t,
    0x9d as ::core::ffi::c_int as uint8_t,
    0x38 as ::core::ffi::c_int as uint8_t,
    0xf5 as ::core::ffi::c_int as uint8_t,
    0xbc as ::core::ffi::c_int as uint8_t,
    0xb6 as ::core::ffi::c_int as uint8_t,
    0xda as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x10 as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xf3 as ::core::ffi::c_int as uint8_t,
    0xd2 as ::core::ffi::c_int as uint8_t,
    0xcd as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0x13 as ::core::ffi::c_int as uint8_t,
    0xec as ::core::ffi::c_int as uint8_t,
    0x5f as ::core::ffi::c_int as uint8_t,
    0x97 as ::core::ffi::c_int as uint8_t,
    0x44 as ::core::ffi::c_int as uint8_t,
    0x17 as ::core::ffi::c_int as uint8_t,
    0xc4 as ::core::ffi::c_int as uint8_t,
    0xa7 as ::core::ffi::c_int as uint8_t,
    0x7e as ::core::ffi::c_int as uint8_t,
    0x3d as ::core::ffi::c_int as uint8_t,
    0x64 as ::core::ffi::c_int as uint8_t,
    0x5d as ::core::ffi::c_int as uint8_t,
    0x19 as ::core::ffi::c_int as uint8_t,
    0x73 as ::core::ffi::c_int as uint8_t,
    0x60 as ::core::ffi::c_int as uint8_t,
    0x81 as ::core::ffi::c_int as uint8_t,
    0x4f as ::core::ffi::c_int as uint8_t,
    0xdc as ::core::ffi::c_int as uint8_t,
    0x22 as ::core::ffi::c_int as uint8_t,
    0x2a as ::core::ffi::c_int as uint8_t,
    0x90 as ::core::ffi::c_int as uint8_t,
    0x88 as ::core::ffi::c_int as uint8_t,
    0x46 as ::core::ffi::c_int as uint8_t,
    0xee as ::core::ffi::c_int as uint8_t,
    0xb8 as ::core::ffi::c_int as uint8_t,
    0x14 as ::core::ffi::c_int as uint8_t,
    0xde as ::core::ffi::c_int as uint8_t,
    0x5e as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0xdb as ::core::ffi::c_int as uint8_t,
    0xe0 as ::core::ffi::c_int as uint8_t,
    0x32 as ::core::ffi::c_int as uint8_t,
    0x3a as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0x49 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x24 as ::core::ffi::c_int as uint8_t,
    0x5c as ::core::ffi::c_int as uint8_t,
    0xc2 as ::core::ffi::c_int as uint8_t,
    0xd3 as ::core::ffi::c_int as uint8_t,
    0xac as ::core::ffi::c_int as uint8_t,
    0x62 as ::core::ffi::c_int as uint8_t,
    0x91 as ::core::ffi::c_int as uint8_t,
    0x95 as ::core::ffi::c_int as uint8_t,
    0xe4 as ::core::ffi::c_int as uint8_t,
    0x79 as ::core::ffi::c_int as uint8_t,
    0xe7 as ::core::ffi::c_int as uint8_t,
    0xc8 as ::core::ffi::c_int as uint8_t,
    0x37 as ::core::ffi::c_int as uint8_t,
    0x6d as ::core::ffi::c_int as uint8_t,
    0x8d as ::core::ffi::c_int as uint8_t,
    0xd5 as ::core::ffi::c_int as uint8_t,
    0x4e as ::core::ffi::c_int as uint8_t,
    0xa9 as ::core::ffi::c_int as uint8_t,
    0x6c as ::core::ffi::c_int as uint8_t,
    0x56 as ::core::ffi::c_int as uint8_t,
    0xf4 as ::core::ffi::c_int as uint8_t,
    0xea as ::core::ffi::c_int as uint8_t,
    0x65 as ::core::ffi::c_int as uint8_t,
    0x7a as ::core::ffi::c_int as uint8_t,
    0xae as ::core::ffi::c_int as uint8_t,
    0x8 as ::core::ffi::c_int as uint8_t,
    0xba as ::core::ffi::c_int as uint8_t,
    0x78 as ::core::ffi::c_int as uint8_t,
    0x25 as ::core::ffi::c_int as uint8_t,
    0x2e as ::core::ffi::c_int as uint8_t,
    0x1c as ::core::ffi::c_int as uint8_t,
    0xa6 as ::core::ffi::c_int as uint8_t,
    0xb4 as ::core::ffi::c_int as uint8_t,
    0xc6 as ::core::ffi::c_int as uint8_t,
    0xe8 as ::core::ffi::c_int as uint8_t,
    0xdd as ::core::ffi::c_int as uint8_t,
    0x74 as ::core::ffi::c_int as uint8_t,
    0x1f as ::core::ffi::c_int as uint8_t,
    0x4b as ::core::ffi::c_int as uint8_t,
    0xbd as ::core::ffi::c_int as uint8_t,
    0x8b as ::core::ffi::c_int as uint8_t,
    0x8a as ::core::ffi::c_int as uint8_t,
    0x70 as ::core::ffi::c_int as uint8_t,
    0x3e as ::core::ffi::c_int as uint8_t,
    0xb5 as ::core::ffi::c_int as uint8_t,
    0x66 as ::core::ffi::c_int as uint8_t,
    0x48 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0xf6 as ::core::ffi::c_int as uint8_t,
    0xe as ::core::ffi::c_int as uint8_t,
    0x61 as ::core::ffi::c_int as uint8_t,
    0x35 as ::core::ffi::c_int as uint8_t,
    0x57 as ::core::ffi::c_int as uint8_t,
    0xb9 as ::core::ffi::c_int as uint8_t,
    0x86 as ::core::ffi::c_int as uint8_t,
    0xc1 as ::core::ffi::c_int as uint8_t,
    0x1d as ::core::ffi::c_int as uint8_t,
    0x9e as ::core::ffi::c_int as uint8_t,
    0xe1 as ::core::ffi::c_int as uint8_t,
    0xf8 as ::core::ffi::c_int as uint8_t,
    0x98 as ::core::ffi::c_int as uint8_t,
    0x11 as ::core::ffi::c_int as uint8_t,
    0x69 as ::core::ffi::c_int as uint8_t,
    0xd9 as ::core::ffi::c_int as uint8_t,
    0x8e as ::core::ffi::c_int as uint8_t,
    0x94 as ::core::ffi::c_int as uint8_t,
    0x9b as ::core::ffi::c_int as uint8_t,
    0x1e as ::core::ffi::c_int as uint8_t,
    0x87 as ::core::ffi::c_int as uint8_t,
    0xe9 as ::core::ffi::c_int as uint8_t,
    0xce as ::core::ffi::c_int as uint8_t,
    0x55 as ::core::ffi::c_int as uint8_t,
    0x28 as ::core::ffi::c_int as uint8_t,
    0xdf as ::core::ffi::c_int as uint8_t,
    0x8c as ::core::ffi::c_int as uint8_t,
    0xa1 as ::core::ffi::c_int as uint8_t,
    0x89 as ::core::ffi::c_int as uint8_t,
    0xd as ::core::ffi::c_int as uint8_t,
    0xbf as ::core::ffi::c_int as uint8_t,
    0xe6 as ::core::ffi::c_int as uint8_t,
    0x42 as ::core::ffi::c_int as uint8_t,
    0x68 as ::core::ffi::c_int as uint8_t,
    0x41 as ::core::ffi::c_int as uint8_t,
    0x99 as ::core::ffi::c_int as uint8_t,
    0x2d as ::core::ffi::c_int as uint8_t,
    0xf as ::core::ffi::c_int as uint8_t,
    0xb0 as ::core::ffi::c_int as uint8_t,
    0x54 as ::core::ffi::c_int as uint8_t,
    0xbb as ::core::ffi::c_int as uint8_t,
    0x16 as ::core::ffi::c_int as uint8_t,
];
static mut Rcon: [uint8_t; 11] = [
    0x8d as ::core::ffi::c_int as uint8_t,
    0x1 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x4 as ::core::ffi::c_int as uint8_t,
    0x8 as ::core::ffi::c_int as uint8_t,
    0x10 as ::core::ffi::c_int as uint8_t,
    0x20 as ::core::ffi::c_int as uint8_t,
    0x40 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0x1b as ::core::ffi::c_int as uint8_t,
    0x36 as ::core::ffi::c_int as uint8_t,
];
unsafe extern "C" fn sub_word(mut w: uint32_t, mut sbox: *const uint8_t) -> uint32_t {
    let mut a: uint8_t = (w
        >> ((3 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        & 0xff as uint32_t) as uint8_t;
    let mut b: uint8_t = (w
        >> ((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        & 0xff as uint32_t) as uint8_t;
    let mut c: uint8_t = (w
        >> ((3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        & 0xff as uint32_t) as uint8_t;
    let mut d: uint8_t = (w
        >> ((3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        & 0xff as uint32_t) as uint8_t;
    select_u8x4(
        &raw mut a,
        &raw mut b,
        &raw mut c,
        &raw mut d,
        sbox,
        256 as uint32_t,
    );
    return (a as uint32_t) << 24 as ::core::ffi::c_int
        | (b as uint32_t) << 16 as ::core::ffi::c_int
        | (c as uint32_t) << 8 as ::core::ffi::c_int
        | d as uint32_t;
}
unsafe extern "C" fn aes_schedule(
    mut ctx: *mut cf_aes_context,
    mut key: *const uint8_t,
    mut nkey: size_t,
) {
    let mut i: size_t = 0;
    let mut nb: size_t = (AES_BLOCKSZ / 4 as ::core::ffi::c_int) as size_t;
    let mut nk: size_t = nkey.wrapping_div(4 as size_t);
    let mut n: size_t = nb.wrapping_mul((*ctx).rounds.wrapping_add(1 as uint32_t) as size_t);
    let mut w: *mut uint32_t = &raw mut (*ctx).ks as *mut uint32_t;
    i = 0 as size_t;
    while i < nk {
        *w.offset(i as isize) = read32_be(key.offset(i.wrapping_mul(4 as size_t) as isize));
        i = i.wrapping_add(1);
    }
    let mut i_div_nk: uint32_t = 1 as uint32_t;
    let mut i_mod_nk: uint32_t = 0 as uint32_t;
    while i < n {
        let mut temp: uint32_t = *w.offset(i.wrapping_sub(1 as size_t) as isize);
        if i_mod_nk as size_t == nk {
            i_div_nk = i_div_nk.wrapping_add(1);
            i_mod_nk = 0 as uint32_t;
        }
        if i_mod_nk == 0 as uint32_t {
            temp = sub_word(
                rotl32(temp, 8 as ::core::ffi::c_uint),
                &raw const S as *const uint8_t,
            ) ^ (Rcon[i_div_nk as usize] as uint32_t) << 24 as ::core::ffi::c_int;
        } else if nk > 6 as size_t && i_mod_nk == 4 as uint32_t {
            temp = sub_word(temp, &raw const S as *const uint8_t);
        }
        *w.offset(i as isize) = *w.offset(i.wrapping_sub(nk) as isize) ^ temp;
        i = i.wrapping_add(1);
        i_mod_nk = i_mod_nk.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cf_aes_init(
    mut ctx: *mut cf_aes_context,
    mut key: *const uint8_t,
    mut nkey: size_t,
) {
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cf_aes_context>() as size_t,
    );
    match nkey {
        16 => {
            (*ctx).rounds = AES128_ROUNDS as uint32_t;
            aes_schedule(ctx, key, nkey);
        }
        24 => {
            (*ctx).rounds = AES192_ROUNDS as uint32_t;
            aes_schedule(ctx, key, nkey);
        }
        32 => {
            (*ctx).rounds = AES256_ROUNDS as uint32_t;
            aes_schedule(ctx, key, nkey);
        }
        _ => {
            abort();
        }
    };
}
unsafe extern "C" fn add_round_key(mut state: *mut uint32_t, mut rk: *const uint32_t) {
    *state.offset(0 as ::core::ffi::c_int as isize) ^= *rk.offset(0 as ::core::ffi::c_int as isize);
    *state.offset(1 as ::core::ffi::c_int as isize) ^= *rk.offset(1 as ::core::ffi::c_int as isize);
    *state.offset(2 as ::core::ffi::c_int as isize) ^= *rk.offset(2 as ::core::ffi::c_int as isize);
    *state.offset(3 as ::core::ffi::c_int as isize) ^= *rk.offset(3 as ::core::ffi::c_int as isize);
}
unsafe extern "C" fn sub_block(mut state: *mut uint32_t) {
    *state.offset(0 as ::core::ffi::c_int as isize) = sub_word(
        *state.offset(0 as ::core::ffi::c_int as isize),
        &raw const S as *const uint8_t,
    );
    *state.offset(1 as ::core::ffi::c_int as isize) = sub_word(
        *state.offset(1 as ::core::ffi::c_int as isize),
        &raw const S as *const uint8_t,
    );
    *state.offset(2 as ::core::ffi::c_int as isize) = sub_word(
        *state.offset(2 as ::core::ffi::c_int as isize),
        &raw const S as *const uint8_t,
    );
    *state.offset(3 as ::core::ffi::c_int as isize) = sub_word(
        *state.offset(3 as ::core::ffi::c_int as isize),
        &raw const S as *const uint8_t,
    );
}
unsafe extern "C" fn shift_rows(mut state: *mut uint32_t) {
    let mut u: uint32_t = 0;
    let mut v: uint32_t = 0;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    u = (*state.offset(0 as ::core::ffi::c_int as isize)
        >> ((3 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*state.offset(1 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t)
            << 16 as ::core::ffi::c_int
        | (*state.offset(2 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t)
            << 8 as ::core::ffi::c_int
        | *state.offset(3 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t;
    v = (*state.offset(1 as ::core::ffi::c_int as isize)
        >> ((3 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*state.offset(2 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t)
            << 16 as ::core::ffi::c_int
        | (*state.offset(3 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t)
            << 8 as ::core::ffi::c_int
        | *state.offset(0 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t;
    x = (*state.offset(2 as ::core::ffi::c_int as isize)
        >> ((3 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*state.offset(3 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t)
            << 16 as ::core::ffi::c_int
        | (*state.offset(0 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t)
            << 8 as ::core::ffi::c_int
        | *state.offset(1 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t;
    y = (*state.offset(3 as ::core::ffi::c_int as isize)
        >> ((3 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*state.offset(0 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t)
            << 16 as ::core::ffi::c_int
        | (*state.offset(1 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t)
            << 8 as ::core::ffi::c_int
        | *state.offset(2 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t;
    *state.offset(0 as ::core::ffi::c_int as isize) = u;
    *state.offset(1 as ::core::ffi::c_int as isize) = v;
    *state.offset(2 as ::core::ffi::c_int as isize) = x;
    *state.offset(3 as ::core::ffi::c_int as isize) = y;
}
unsafe extern "C" fn gf_poly_mul2(mut x: uint32_t) -> uint32_t {
    return (x & 0x7f7f7f7f as uint32_t) << 1 as ::core::ffi::c_int
        ^ ((x & 0x80808080 as uint32_t) >> 7 as ::core::ffi::c_int).wrapping_mul(0x1b as uint32_t);
}
unsafe extern "C" fn mix_column(mut x: uint32_t) -> uint32_t {
    let mut x2: uint32_t = gf_poly_mul2(x);
    return x2
        ^ rotr32(x ^ x2, 24 as ::core::ffi::c_uint)
        ^ rotr32(x, 16 as ::core::ffi::c_uint)
        ^ rotr32(x, 8 as ::core::ffi::c_uint);
}
unsafe extern "C" fn mix_columns(mut state: *mut uint32_t) {
    *state.offset(0 as ::core::ffi::c_int as isize) =
        mix_column(*state.offset(0 as ::core::ffi::c_int as isize));
    *state.offset(1 as ::core::ffi::c_int as isize) =
        mix_column(*state.offset(1 as ::core::ffi::c_int as isize));
    *state.offset(2 as ::core::ffi::c_int as isize) =
        mix_column(*state.offset(2 as ::core::ffi::c_int as isize));
    *state.offset(3 as ::core::ffi::c_int as isize) =
        mix_column(*state.offset(3 as ::core::ffi::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn cf_aes_encrypt(
    mut ctx: *const cf_aes_context,
    mut in_0: *const uint8_t,
    mut out: *mut uint8_t,
) {
    if !((*ctx).rounds == 10 as uint32_t
        || (*ctx).rounds == 12 as uint32_t
        || (*ctx).rounds == 14 as uint32_t)
    {
        abort();
    }
    let mut state: [uint32_t; 4] = [
        read32_be(in_0.offset(0 as ::core::ffi::c_int as isize)),
        read32_be(in_0.offset(4 as ::core::ffi::c_int as isize)),
        read32_be(in_0.offset(8 as ::core::ffi::c_int as isize)),
        read32_be(in_0.offset(12 as ::core::ffi::c_int as isize)),
    ];
    let mut round_keys: *const uint32_t = &raw const (*ctx).ks as *const uint32_t;
    add_round_key(
        &raw mut state as *mut uint32_t,
        round_keys as *const uint32_t,
    );
    round_keys = round_keys.offset(4 as ::core::ffi::c_int as isize);
    let mut round: uint32_t = 0;
    round = 1 as uint32_t;
    while round < (*ctx).rounds {
        sub_block(&raw mut state as *mut uint32_t);
        shift_rows(&raw mut state as *mut uint32_t);
        mix_columns(&raw mut state as *mut uint32_t);
        add_round_key(
            &raw mut state as *mut uint32_t,
            round_keys as *const uint32_t,
        );
        round_keys = round_keys.offset(4 as ::core::ffi::c_int as isize);
        round = round.wrapping_add(1);
    }
    sub_block(&raw mut state as *mut uint32_t);
    shift_rows(&raw mut state as *mut uint32_t);
    add_round_key(
        &raw mut state as *mut uint32_t,
        round_keys as *const uint32_t,
    );
    write32_be(
        state[0 as ::core::ffi::c_int as usize],
        out.offset(0 as ::core::ffi::c_int as isize),
    );
    write32_be(
        state[1 as ::core::ffi::c_int as usize],
        out.offset(4 as ::core::ffi::c_int as isize),
    );
    write32_be(
        state[2 as ::core::ffi::c_int as usize],
        out.offset(8 as ::core::ffi::c_int as isize),
    );
    write32_be(
        state[3 as ::core::ffi::c_int as usize],
        out.offset(12 as ::core::ffi::c_int as isize),
    );
}
static mut S_inv: [uint8_t; 256] = [
    0x52 as ::core::ffi::c_int as uint8_t,
    0x9 as ::core::ffi::c_int as uint8_t,
    0x6a as ::core::ffi::c_int as uint8_t,
    0xd5 as ::core::ffi::c_int as uint8_t,
    0x30 as ::core::ffi::c_int as uint8_t,
    0x36 as ::core::ffi::c_int as uint8_t,
    0xa5 as ::core::ffi::c_int as uint8_t,
    0x38 as ::core::ffi::c_int as uint8_t,
    0xbf as ::core::ffi::c_int as uint8_t,
    0x40 as ::core::ffi::c_int as uint8_t,
    0xa3 as ::core::ffi::c_int as uint8_t,
    0x9e as ::core::ffi::c_int as uint8_t,
    0x81 as ::core::ffi::c_int as uint8_t,
    0xf3 as ::core::ffi::c_int as uint8_t,
    0xd7 as ::core::ffi::c_int as uint8_t,
    0xfb as ::core::ffi::c_int as uint8_t,
    0x7c as ::core::ffi::c_int as uint8_t,
    0xe3 as ::core::ffi::c_int as uint8_t,
    0x39 as ::core::ffi::c_int as uint8_t,
    0x82 as ::core::ffi::c_int as uint8_t,
    0x9b as ::core::ffi::c_int as uint8_t,
    0x2f as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0x87 as ::core::ffi::c_int as uint8_t,
    0x34 as ::core::ffi::c_int as uint8_t,
    0x8e as ::core::ffi::c_int as uint8_t,
    0x43 as ::core::ffi::c_int as uint8_t,
    0x44 as ::core::ffi::c_int as uint8_t,
    0xc4 as ::core::ffi::c_int as uint8_t,
    0xde as ::core::ffi::c_int as uint8_t,
    0xe9 as ::core::ffi::c_int as uint8_t,
    0xcb as ::core::ffi::c_int as uint8_t,
    0x54 as ::core::ffi::c_int as uint8_t,
    0x7b as ::core::ffi::c_int as uint8_t,
    0x94 as ::core::ffi::c_int as uint8_t,
    0x32 as ::core::ffi::c_int as uint8_t,
    0xa6 as ::core::ffi::c_int as uint8_t,
    0xc2 as ::core::ffi::c_int as uint8_t,
    0x23 as ::core::ffi::c_int as uint8_t,
    0x3d as ::core::ffi::c_int as uint8_t,
    0xee as ::core::ffi::c_int as uint8_t,
    0x4c as ::core::ffi::c_int as uint8_t,
    0x95 as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0x42 as ::core::ffi::c_int as uint8_t,
    0xfa as ::core::ffi::c_int as uint8_t,
    0xc3 as ::core::ffi::c_int as uint8_t,
    0x4e as ::core::ffi::c_int as uint8_t,
    0x8 as ::core::ffi::c_int as uint8_t,
    0x2e as ::core::ffi::c_int as uint8_t,
    0xa1 as ::core::ffi::c_int as uint8_t,
    0x66 as ::core::ffi::c_int as uint8_t,
    0x28 as ::core::ffi::c_int as uint8_t,
    0xd9 as ::core::ffi::c_int as uint8_t,
    0x24 as ::core::ffi::c_int as uint8_t,
    0xb2 as ::core::ffi::c_int as uint8_t,
    0x76 as ::core::ffi::c_int as uint8_t,
    0x5b as ::core::ffi::c_int as uint8_t,
    0xa2 as ::core::ffi::c_int as uint8_t,
    0x49 as ::core::ffi::c_int as uint8_t,
    0x6d as ::core::ffi::c_int as uint8_t,
    0x8b as ::core::ffi::c_int as uint8_t,
    0xd1 as ::core::ffi::c_int as uint8_t,
    0x25 as ::core::ffi::c_int as uint8_t,
    0x72 as ::core::ffi::c_int as uint8_t,
    0xf8 as ::core::ffi::c_int as uint8_t,
    0xf6 as ::core::ffi::c_int as uint8_t,
    0x64 as ::core::ffi::c_int as uint8_t,
    0x86 as ::core::ffi::c_int as uint8_t,
    0x68 as ::core::ffi::c_int as uint8_t,
    0x98 as ::core::ffi::c_int as uint8_t,
    0x16 as ::core::ffi::c_int as uint8_t,
    0xd4 as ::core::ffi::c_int as uint8_t,
    0xa4 as ::core::ffi::c_int as uint8_t,
    0x5c as ::core::ffi::c_int as uint8_t,
    0xcc as ::core::ffi::c_int as uint8_t,
    0x5d as ::core::ffi::c_int as uint8_t,
    0x65 as ::core::ffi::c_int as uint8_t,
    0xb6 as ::core::ffi::c_int as uint8_t,
    0x92 as ::core::ffi::c_int as uint8_t,
    0x6c as ::core::ffi::c_int as uint8_t,
    0x70 as ::core::ffi::c_int as uint8_t,
    0x48 as ::core::ffi::c_int as uint8_t,
    0x50 as ::core::ffi::c_int as uint8_t,
    0xfd as ::core::ffi::c_int as uint8_t,
    0xed as ::core::ffi::c_int as uint8_t,
    0xb9 as ::core::ffi::c_int as uint8_t,
    0xda as ::core::ffi::c_int as uint8_t,
    0x5e as ::core::ffi::c_int as uint8_t,
    0x15 as ::core::ffi::c_int as uint8_t,
    0x46 as ::core::ffi::c_int as uint8_t,
    0x57 as ::core::ffi::c_int as uint8_t,
    0xa7 as ::core::ffi::c_int as uint8_t,
    0x8d as ::core::ffi::c_int as uint8_t,
    0x9d as ::core::ffi::c_int as uint8_t,
    0x84 as ::core::ffi::c_int as uint8_t,
    0x90 as ::core::ffi::c_int as uint8_t,
    0xd8 as ::core::ffi::c_int as uint8_t,
    0xab as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0x8c as ::core::ffi::c_int as uint8_t,
    0xbc as ::core::ffi::c_int as uint8_t,
    0xd3 as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0xf7 as ::core::ffi::c_int as uint8_t,
    0xe4 as ::core::ffi::c_int as uint8_t,
    0x58 as ::core::ffi::c_int as uint8_t,
    0x5 as ::core::ffi::c_int as uint8_t,
    0xb8 as ::core::ffi::c_int as uint8_t,
    0xb3 as ::core::ffi::c_int as uint8_t,
    0x45 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0xd0 as ::core::ffi::c_int as uint8_t,
    0x2c as ::core::ffi::c_int as uint8_t,
    0x1e as ::core::ffi::c_int as uint8_t,
    0x8f as ::core::ffi::c_int as uint8_t,
    0xca as ::core::ffi::c_int as uint8_t,
    0x3f as ::core::ffi::c_int as uint8_t,
    0xf as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0xc1 as ::core::ffi::c_int as uint8_t,
    0xaf as ::core::ffi::c_int as uint8_t,
    0xbd as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x1 as ::core::ffi::c_int as uint8_t,
    0x13 as ::core::ffi::c_int as uint8_t,
    0x8a as ::core::ffi::c_int as uint8_t,
    0x6b as ::core::ffi::c_int as uint8_t,
    0x3a as ::core::ffi::c_int as uint8_t,
    0x91 as ::core::ffi::c_int as uint8_t,
    0x11 as ::core::ffi::c_int as uint8_t,
    0x41 as ::core::ffi::c_int as uint8_t,
    0x4f as ::core::ffi::c_int as uint8_t,
    0x67 as ::core::ffi::c_int as uint8_t,
    0xdc as ::core::ffi::c_int as uint8_t,
    0xea as ::core::ffi::c_int as uint8_t,
    0x97 as ::core::ffi::c_int as uint8_t,
    0xf2 as ::core::ffi::c_int as uint8_t,
    0xcf as ::core::ffi::c_int as uint8_t,
    0xce as ::core::ffi::c_int as uint8_t,
    0xf0 as ::core::ffi::c_int as uint8_t,
    0xb4 as ::core::ffi::c_int as uint8_t,
    0xe6 as ::core::ffi::c_int as uint8_t,
    0x73 as ::core::ffi::c_int as uint8_t,
    0x96 as ::core::ffi::c_int as uint8_t,
    0xac as ::core::ffi::c_int as uint8_t,
    0x74 as ::core::ffi::c_int as uint8_t,
    0x22 as ::core::ffi::c_int as uint8_t,
    0xe7 as ::core::ffi::c_int as uint8_t,
    0xad as ::core::ffi::c_int as uint8_t,
    0x35 as ::core::ffi::c_int as uint8_t,
    0x85 as ::core::ffi::c_int as uint8_t,
    0xe2 as ::core::ffi::c_int as uint8_t,
    0xf9 as ::core::ffi::c_int as uint8_t,
    0x37 as ::core::ffi::c_int as uint8_t,
    0xe8 as ::core::ffi::c_int as uint8_t,
    0x1c as ::core::ffi::c_int as uint8_t,
    0x75 as ::core::ffi::c_int as uint8_t,
    0xdf as ::core::ffi::c_int as uint8_t,
    0x6e as ::core::ffi::c_int as uint8_t,
    0x47 as ::core::ffi::c_int as uint8_t,
    0xf1 as ::core::ffi::c_int as uint8_t,
    0x1a as ::core::ffi::c_int as uint8_t,
    0x71 as ::core::ffi::c_int as uint8_t,
    0x1d as ::core::ffi::c_int as uint8_t,
    0x29 as ::core::ffi::c_int as uint8_t,
    0xc5 as ::core::ffi::c_int as uint8_t,
    0x89 as ::core::ffi::c_int as uint8_t,
    0x6f as ::core::ffi::c_int as uint8_t,
    0xb7 as ::core::ffi::c_int as uint8_t,
    0x62 as ::core::ffi::c_int as uint8_t,
    0xe as ::core::ffi::c_int as uint8_t,
    0xaa as ::core::ffi::c_int as uint8_t,
    0x18 as ::core::ffi::c_int as uint8_t,
    0xbe as ::core::ffi::c_int as uint8_t,
    0x1b as ::core::ffi::c_int as uint8_t,
    0xfc as ::core::ffi::c_int as uint8_t,
    0x56 as ::core::ffi::c_int as uint8_t,
    0x3e as ::core::ffi::c_int as uint8_t,
    0x4b as ::core::ffi::c_int as uint8_t,
    0xc6 as ::core::ffi::c_int as uint8_t,
    0xd2 as ::core::ffi::c_int as uint8_t,
    0x79 as ::core::ffi::c_int as uint8_t,
    0x20 as ::core::ffi::c_int as uint8_t,
    0x9a as ::core::ffi::c_int as uint8_t,
    0xdb as ::core::ffi::c_int as uint8_t,
    0xc0 as ::core::ffi::c_int as uint8_t,
    0xfe as ::core::ffi::c_int as uint8_t,
    0x78 as ::core::ffi::c_int as uint8_t,
    0xcd as ::core::ffi::c_int as uint8_t,
    0x5a as ::core::ffi::c_int as uint8_t,
    0xf4 as ::core::ffi::c_int as uint8_t,
    0x1f as ::core::ffi::c_int as uint8_t,
    0xdd as ::core::ffi::c_int as uint8_t,
    0xa8 as ::core::ffi::c_int as uint8_t,
    0x33 as ::core::ffi::c_int as uint8_t,
    0x88 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0xc7 as ::core::ffi::c_int as uint8_t,
    0x31 as ::core::ffi::c_int as uint8_t,
    0xb1 as ::core::ffi::c_int as uint8_t,
    0x12 as ::core::ffi::c_int as uint8_t,
    0x10 as ::core::ffi::c_int as uint8_t,
    0x59 as ::core::ffi::c_int as uint8_t,
    0x27 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0xec as ::core::ffi::c_int as uint8_t,
    0x5f as ::core::ffi::c_int as uint8_t,
    0x60 as ::core::ffi::c_int as uint8_t,
    0x51 as ::core::ffi::c_int as uint8_t,
    0x7f as ::core::ffi::c_int as uint8_t,
    0xa9 as ::core::ffi::c_int as uint8_t,
    0x19 as ::core::ffi::c_int as uint8_t,
    0xb5 as ::core::ffi::c_int as uint8_t,
    0x4a as ::core::ffi::c_int as uint8_t,
    0xd as ::core::ffi::c_int as uint8_t,
    0x2d as ::core::ffi::c_int as uint8_t,
    0xe5 as ::core::ffi::c_int as uint8_t,
    0x7a as ::core::ffi::c_int as uint8_t,
    0x9f as ::core::ffi::c_int as uint8_t,
    0x93 as ::core::ffi::c_int as uint8_t,
    0xc9 as ::core::ffi::c_int as uint8_t,
    0x9c as ::core::ffi::c_int as uint8_t,
    0xef as ::core::ffi::c_int as uint8_t,
    0xa0 as ::core::ffi::c_int as uint8_t,
    0xe0 as ::core::ffi::c_int as uint8_t,
    0x3b as ::core::ffi::c_int as uint8_t,
    0x4d as ::core::ffi::c_int as uint8_t,
    0xae as ::core::ffi::c_int as uint8_t,
    0x2a as ::core::ffi::c_int as uint8_t,
    0xf5 as ::core::ffi::c_int as uint8_t,
    0xb0 as ::core::ffi::c_int as uint8_t,
    0xc8 as ::core::ffi::c_int as uint8_t,
    0xeb as ::core::ffi::c_int as uint8_t,
    0xbb as ::core::ffi::c_int as uint8_t,
    0x3c as ::core::ffi::c_int as uint8_t,
    0x83 as ::core::ffi::c_int as uint8_t,
    0x53 as ::core::ffi::c_int as uint8_t,
    0x99 as ::core::ffi::c_int as uint8_t,
    0x61 as ::core::ffi::c_int as uint8_t,
    0x17 as ::core::ffi::c_int as uint8_t,
    0x2b as ::core::ffi::c_int as uint8_t,
    0x4 as ::core::ffi::c_int as uint8_t,
    0x7e as ::core::ffi::c_int as uint8_t,
    0xba as ::core::ffi::c_int as uint8_t,
    0x77 as ::core::ffi::c_int as uint8_t,
    0xd6 as ::core::ffi::c_int as uint8_t,
    0x26 as ::core::ffi::c_int as uint8_t,
    0xe1 as ::core::ffi::c_int as uint8_t,
    0x69 as ::core::ffi::c_int as uint8_t,
    0x14 as ::core::ffi::c_int as uint8_t,
    0x63 as ::core::ffi::c_int as uint8_t,
    0x55 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0x7d as ::core::ffi::c_int as uint8_t,
];
unsafe extern "C" fn inv_sub_block(mut state: *mut uint32_t) {
    *state.offset(0 as ::core::ffi::c_int as isize) = sub_word(
        *state.offset(0 as ::core::ffi::c_int as isize),
        &raw const S_inv as *const uint8_t,
    );
    *state.offset(1 as ::core::ffi::c_int as isize) = sub_word(
        *state.offset(1 as ::core::ffi::c_int as isize),
        &raw const S_inv as *const uint8_t,
    );
    *state.offset(2 as ::core::ffi::c_int as isize) = sub_word(
        *state.offset(2 as ::core::ffi::c_int as isize),
        &raw const S_inv as *const uint8_t,
    );
    *state.offset(3 as ::core::ffi::c_int as isize) = sub_word(
        *state.offset(3 as ::core::ffi::c_int as isize),
        &raw const S_inv as *const uint8_t,
    );
}
unsafe extern "C" fn inv_shift_rows(mut state: *mut uint32_t) {
    let mut u: uint32_t = 0;
    let mut v: uint32_t = 0;
    let mut x: uint32_t = 0;
    let mut y: uint32_t = 0;
    u = (*state.offset(0 as ::core::ffi::c_int as isize)
        >> ((3 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*state.offset(3 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t)
            << 16 as ::core::ffi::c_int
        | (*state.offset(2 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t)
            << 8 as ::core::ffi::c_int
        | *state.offset(1 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t;
    v = (*state.offset(1 as ::core::ffi::c_int as isize)
        >> ((3 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*state.offset(0 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t)
            << 16 as ::core::ffi::c_int
        | (*state.offset(3 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t)
            << 8 as ::core::ffi::c_int
        | *state.offset(2 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t;
    x = (*state.offset(2 as ::core::ffi::c_int as isize)
        >> ((3 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*state.offset(1 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t)
            << 16 as ::core::ffi::c_int
        | (*state.offset(0 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t)
            << 8 as ::core::ffi::c_int
        | *state.offset(3 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t;
    y = (*state.offset(3 as ::core::ffi::c_int as isize)
        >> ((3 as ::core::ffi::c_int - 0 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
        & 0xff as uint32_t)
        << 24 as ::core::ffi::c_int
        | (*state.offset(2 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 1 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t)
            << 16 as ::core::ffi::c_int
        | (*state.offset(1 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 2 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t)
            << 8 as ::core::ffi::c_int
        | *state.offset(0 as ::core::ffi::c_int as isize)
            >> ((3 as ::core::ffi::c_int - 3 as ::core::ffi::c_int) << 3 as ::core::ffi::c_int)
            & 0xff as uint32_t;
    *state.offset(0 as ::core::ffi::c_int as isize) = u;
    *state.offset(1 as ::core::ffi::c_int as isize) = v;
    *state.offset(2 as ::core::ffi::c_int as isize) = x;
    *state.offset(3 as ::core::ffi::c_int as isize) = y;
}
unsafe extern "C" fn inv_mix_column(mut x: uint32_t) -> uint32_t {
    let mut x2: uint32_t = gf_poly_mul2(x);
    let mut x4: uint32_t = gf_poly_mul2(x2);
    let mut x9: uint32_t = x ^ gf_poly_mul2(x4);
    let mut x11: uint32_t = x2 ^ x9;
    let mut x13: uint32_t = x4 ^ x9;
    return x
        ^ x2
        ^ x13
        ^ rotr32(x11, 24 as ::core::ffi::c_uint)
        ^ rotr32(x13, 16 as ::core::ffi::c_uint)
        ^ rotr32(x9, 8 as ::core::ffi::c_uint);
}
unsafe extern "C" fn inv_mix_columns(mut state: *mut uint32_t) {
    *state.offset(0 as ::core::ffi::c_int as isize) =
        inv_mix_column(*state.offset(0 as ::core::ffi::c_int as isize));
    *state.offset(1 as ::core::ffi::c_int as isize) =
        inv_mix_column(*state.offset(1 as ::core::ffi::c_int as isize));
    *state.offset(2 as ::core::ffi::c_int as isize) =
        inv_mix_column(*state.offset(2 as ::core::ffi::c_int as isize));
    *state.offset(3 as ::core::ffi::c_int as isize) =
        inv_mix_column(*state.offset(3 as ::core::ffi::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn cf_aes_decrypt(
    mut ctx: *const cf_aes_context,
    mut in_0: *const uint8_t,
    mut out: *mut uint8_t,
) {
    if !((*ctx).rounds == 10 as uint32_t
        || (*ctx).rounds == 12 as uint32_t
        || (*ctx).rounds == 14 as uint32_t)
    {
        abort();
    }
    let mut state: [uint32_t; 4] = [
        read32_be(in_0.offset(0 as ::core::ffi::c_int as isize)),
        read32_be(in_0.offset(4 as ::core::ffi::c_int as isize)),
        read32_be(in_0.offset(8 as ::core::ffi::c_int as isize)),
        read32_be(in_0.offset(12 as ::core::ffi::c_int as isize)),
    ];
    let mut round_keys: *const uint32_t = (&raw const (*ctx).ks as *const uint32_t)
        .offset(((*ctx).rounds << 2 as ::core::ffi::c_int) as isize)
        as *const uint32_t;
    add_round_key(
        &raw mut state as *mut uint32_t,
        round_keys as *const uint32_t,
    );
    round_keys = round_keys.offset(-(4 as ::core::ffi::c_int as isize));
    let mut round: uint32_t = 0;
    round = (*ctx).rounds.wrapping_sub(1 as uint32_t);
    while round != 0 as uint32_t {
        inv_shift_rows(&raw mut state as *mut uint32_t);
        inv_sub_block(&raw mut state as *mut uint32_t);
        add_round_key(
            &raw mut state as *mut uint32_t,
            round_keys as *const uint32_t,
        );
        inv_mix_columns(&raw mut state as *mut uint32_t);
        round_keys = round_keys.offset(-(4 as ::core::ffi::c_int as isize));
        round = round.wrapping_sub(1);
    }
    inv_shift_rows(&raw mut state as *mut uint32_t);
    inv_sub_block(&raw mut state as *mut uint32_t);
    add_round_key(
        &raw mut state as *mut uint32_t,
        round_keys as *const uint32_t,
    );
    write32_be(
        state[0 as ::core::ffi::c_int as usize],
        out.offset(0 as ::core::ffi::c_int as isize),
    );
    write32_be(
        state[1 as ::core::ffi::c_int as usize],
        out.offset(4 as ::core::ffi::c_int as isize),
    );
    write32_be(
        state[2 as ::core::ffi::c_int as usize],
        out.offset(8 as ::core::ffi::c_int as isize),
    );
    write32_be(
        state[3 as ::core::ffi::c_int as usize],
        out.offset(12 as ::core::ffi::c_int as isize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_aes_finish(mut ctx: *mut cf_aes_context) {
    mem_clean(
        ctx as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<cf_aes_context>() as size_t,
    );
}
#[no_mangle]
pub static mut cf_aes: cf_prp = unsafe {
    cf_prp {
        blocksz: AES_BLOCKSZ as size_t,
        encrypt: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const cf_aes_context, *const uint8_t, *mut uint8_t) -> ()>,
            cf_prp_block,
        >(Some(
            cf_aes_encrypt
                as unsafe extern "C" fn(*const cf_aes_context, *const uint8_t, *mut uint8_t) -> (),
        )),
        decrypt: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const cf_aes_context, *const uint8_t, *mut uint8_t) -> ()>,
            cf_prp_block,
        >(Some(
            cf_aes_decrypt
                as unsafe extern "C" fn(*const cf_aes_context, *const uint8_t, *mut uint8_t) -> (),
        )),
    }
};

pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn picoformat_16(mut bytes: *mut uint8_t, mut n16: uint16_t) {
    *bytes.offset(0 as ::core::ffi::c_int as isize) =
        (n16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uint8_t;
    *bytes.offset(1 as ::core::ffi::c_int as isize) = n16 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoformat_24(mut bytes: *mut uint8_t, mut n24: uint32_t) {
    *bytes.offset(0 as ::core::ffi::c_int as isize) = (n24 >> 16 as ::core::ffi::c_int) as uint8_t;
    *bytes.offset(1 as ::core::ffi::c_int as isize) = (n24 >> 8 as ::core::ffi::c_int) as uint8_t;
    *bytes.offset(2 as ::core::ffi::c_int as isize) = n24 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoformat_32(mut bytes: *mut uint8_t, mut n32: uint32_t) {
    *bytes.offset(0 as ::core::ffi::c_int as isize) = (n32 >> 24 as ::core::ffi::c_int) as uint8_t;
    *bytes.offset(1 as ::core::ffi::c_int as isize) = (n32 >> 16 as ::core::ffi::c_int) as uint8_t;
    *bytes.offset(2 as ::core::ffi::c_int as isize) = (n32 >> 8 as ::core::ffi::c_int) as uint8_t;
    *bytes.offset(3 as ::core::ffi::c_int as isize) = n32 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoformat_64(mut bytes: *mut uint8_t, mut n64: uint64_t) {
    *bytes.offset(0 as ::core::ffi::c_int as isize) = (n64 >> 56 as ::core::ffi::c_int) as uint8_t;
    *bytes.offset(1 as ::core::ffi::c_int as isize) = (n64 >> 48 as ::core::ffi::c_int) as uint8_t;
    *bytes.offset(2 as ::core::ffi::c_int as isize) = (n64 >> 40 as ::core::ffi::c_int) as uint8_t;
    *bytes.offset(3 as ::core::ffi::c_int as isize) = (n64 >> 32 as ::core::ffi::c_int) as uint8_t;
    *bytes.offset(4 as ::core::ffi::c_int as isize) = (n64 >> 24 as ::core::ffi::c_int) as uint8_t;
    *bytes.offset(5 as ::core::ffi::c_int as isize) = (n64 >> 16 as ::core::ffi::c_int) as uint8_t;
    *bytes.offset(6 as ::core::ffi::c_int as isize) = (n64 >> 8 as ::core::ffi::c_int) as uint8_t;
    *bytes.offset(7 as ::core::ffi::c_int as isize) = n64 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_encode_varint_length(mut n64: uint64_t) -> size_t {
    if n64 < 16384 as uint64_t {
        if n64 < 64 as uint64_t {
            return 1 as size_t;
        } else {
            return 2 as size_t;
        }
    } else if n64 < 1073741824 as uint64_t {
        return 4 as size_t;
    } else {
        return 8 as size_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_varint_length(mut byte: uint8_t) -> size_t {
    return (1 as ::core::ffi::c_uint as size_t)
        << ((byte as ::core::ffi::c_int & 0xc0 as ::core::ffi::c_int) >> 6 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_varint_encode(
    mut bytes: *mut uint8_t,
    mut max_bytes: size_t,
    mut n64: uint64_t,
) -> size_t {
    let mut x: *mut uint8_t = bytes;
    if n64 < 16384 as uint64_t {
        if n64 < 64 as uint64_t {
            if max_bytes > 0 as size_t {
                let c2rust_fresh0 = x;
                x = x.offset(1);
                *c2rust_fresh0 = n64 as uint8_t;
            }
        } else if max_bytes >= 2 as size_t {
            let c2rust_fresh1 = x;
            x = x.offset(1);
            *c2rust_fresh1 = (n64 >> 8 as ::core::ffi::c_int | 0x40 as uint64_t) as uint8_t;
            let c2rust_fresh2 = x;
            x = x.offset(1);
            *c2rust_fresh2 = n64 as uint8_t;
        }
    } else if n64 < 1073741824 as uint64_t {
        if max_bytes >= 4 as size_t {
            let c2rust_fresh3 = x;
            x = x.offset(1);
            *c2rust_fresh3 = (n64 >> 24 as ::core::ffi::c_int | 0x80 as uint64_t) as uint8_t;
            let c2rust_fresh4 = x;
            x = x.offset(1);
            *c2rust_fresh4 = (n64 >> 16 as ::core::ffi::c_int) as uint8_t;
            let c2rust_fresh5 = x;
            x = x.offset(1);
            *c2rust_fresh5 = (n64 >> 8 as ::core::ffi::c_int) as uint8_t;
            let c2rust_fresh6 = x;
            x = x.offset(1);
            *c2rust_fresh6 = n64 as uint8_t;
        }
    } else if max_bytes >= 8 as size_t {
        let c2rust_fresh7 = x;
        x = x.offset(1);
        *c2rust_fresh7 = (n64 >> 56 as ::core::ffi::c_int | 0xc0 as uint64_t) as uint8_t;
        let c2rust_fresh8 = x;
        x = x.offset(1);
        *c2rust_fresh8 = (n64 >> 48 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh9 = x;
        x = x.offset(1);
        *c2rust_fresh9 = (n64 >> 40 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh10 = x;
        x = x.offset(1);
        *c2rust_fresh10 = (n64 >> 32 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh11 = x;
        x = x.offset(1);
        *c2rust_fresh11 = (n64 >> 24 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh12 = x;
        x = x.offset(1);
        *c2rust_fresh12 = (n64 >> 16 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh13 = x;
        x = x.offset(1);
        *c2rust_fresh13 = (n64 >> 8 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh14 = x;
        x = x.offset(1);
        *c2rust_fresh14 = n64 as uint8_t;
    }
    return x.offset_from(bytes) as ::core::ffi::c_long as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_varint_encode_16(mut bytes: *mut uint8_t, mut n16: uint16_t) {
    let mut x: *mut uint8_t = bytes;
    let c2rust_fresh15 = x;
    x = x.offset(1);
    *c2rust_fresh15 = ((n16 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int
        | 0x40 as ::core::ffi::c_int)
        & 0x7f as ::core::ffi::c_int) as uint8_t;
    let c2rust_fresh16 = x;
    x = x.offset(1);
    *c2rust_fresh16 = n16 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_varint_decode(
    mut bytes: *const uint8_t,
    mut max_bytes: size_t,
    mut n64: *mut uint64_t,
) -> size_t {
    let mut length: size_t = 0 as size_t;
    if max_bytes < 1 as size_t {
        *n64 = 0 as uint64_t;
    } else {
        length = (1 as ::core::ffi::c_int as size_t)
            << ((*bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0xc0 as ::core::ffi::c_int)
                >> 6 as ::core::ffi::c_int);
        if length > max_bytes {
            *n64 = 0 as uint64_t;
            length = 0 as size_t;
        } else {
            let c2rust_fresh17 = bytes;
            bytes = bytes.offset(1);
            let mut v: uint64_t =
                (*c2rust_fresh17 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as uint64_t;
            let mut i: size_t = 1 as size_t;
            while i < length {
                v <<= 8 as ::core::ffi::c_int;
                let c2rust_fresh18 = bytes;
                bytes = bytes.offset(1);
                v = v.wrapping_add(*c2rust_fresh18 as uint64_t);
                i = i.wrapping_add(1);
            }
            *n64 = v;
        }
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_varint_skip(mut bytes: *const uint8_t) -> size_t {
    return picoquic_decode_varint_length(*bytes.offset(0 as ::core::ffi::c_int as isize));
}

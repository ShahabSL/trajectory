pub type size_t = usize;
#[no_mangle]
pub unsafe extern "C" fn slipstream_inline_dotify(
    mut buf: *mut ::core::ffi::c_char,
    mut buflen: size_t,
    mut len: size_t,
) -> size_t {
    if len == 0 as size_t {
        if buflen > 0 as size_t {
            *buf.offset(0 as ::core::ffi::c_int as isize) = '\0' as i32 as ::core::ffi::c_char;
        }
        return 0 as size_t;
    }
    let mut dots: size_t = len.wrapping_div(57 as size_t);
    let mut new_len: size_t = len.wrapping_add(dots);
    if new_len.wrapping_add(1 as size_t) > buflen {
        return -(1 as ::core::ffi::c_int) as size_t;
    }
    *buf.offset(new_len as isize) = '\0' as i32 as ::core::ffi::c_char;
    let mut src: *mut ::core::ffi::c_char = buf
        .offset(len as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    let mut dst: *mut ::core::ffi::c_char = buf
        .offset(new_len as isize)
        .offset(-(1 as ::core::ffi::c_int as isize));
    let mut next_dot: size_t = len.wrapping_sub(len.wrapping_rem(57 as size_t));
    let mut current_pos: size_t = len;
    while current_pos > 0 as size_t {
        if current_pos == next_dot {
            let c2rust_fresh0 = dst;
            dst = dst.offset(-1);
            *c2rust_fresh0 = '.' as i32 as ::core::ffi::c_char;
            next_dot = next_dot.wrapping_sub(57 as size_t);
            current_pos = current_pos.wrapping_sub(1);
        } else {
            let c2rust_fresh1 = src;
            src = src.offset(-1);
            let c2rust_fresh2 = dst;
            dst = dst.offset(-1);
            *c2rust_fresh2 = *c2rust_fresh1;
            current_pos = current_pos.wrapping_sub(1);
        }
    }
    return new_len;
}
#[no_mangle]
pub unsafe extern "C" fn slipstream_inline_undotify(
    mut buf: *mut ::core::ffi::c_char,
    mut len: size_t,
) -> size_t {
    let mut reader: *mut ::core::ffi::c_char = buf;
    let mut writer: *mut ::core::ffi::c_char = buf;
    let mut i: size_t = 0 as size_t;
    while i < len {
        let c2rust_fresh3 = reader;
        reader = reader.offset(1);
        let mut c: ::core::ffi::c_char = *c2rust_fresh3;
        if c as ::core::ffi::c_int != '.' as i32 {
            let c2rust_fresh4 = writer;
            writer = writer.offset(1);
            *c2rust_fresh4 = c;
        }
        i = i.wrapping_add(1);
    }
    *writer = '\0' as i32 as ::core::ffi::c_char;
    return writer.offset_from(buf) as ::core::ffi::c_long as size_t;
}

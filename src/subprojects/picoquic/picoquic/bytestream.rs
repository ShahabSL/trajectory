extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn picoformat_16(bytes: *mut uint8_t, n16: uint16_t);
    fn picoformat_32(bytes: *mut uint8_t, n32: uint32_t);
    fn picoformat_64(bytes: *mut uint8_t, n64: uint64_t);
    fn picoquic_varint_encode(bytes: *mut uint8_t, max_bytes: size_t, n64: uint64_t) -> size_t;
    fn picoquic_varint_decode(
        bytes: *const uint8_t,
        max_bytes: size_t,
        n64: *mut uint64_t,
    ) -> size_t;
    fn picoquic_encode_varint_length(n64: uint64_t) -> size_t;
    fn picoquic_decode_varint_length(byte: uint8_t) -> size_t;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [::core::ffi::c_char; 118],
    pub __ss_align: ::core::ffi::c_ulong,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::core::ffi::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_connection_id_t {
    pub id: [uint8_t; 20],
    pub id_len: uint8_t,
}
pub type picoquic_connection_id_t = st_picoquic_connection_id_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bytestream {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub ptr: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bytestream_buf {
    pub s: bytestream,
    pub buf: [uint8_t; 2560],
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const PICOQUIC_CONNECTION_ID_MAX_SIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const BYTESTREAM_MAX_BUFFER_SIZE: ::core::ffi::c_int = 2560 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn bytestream_ref_init(
    mut s: *mut bytestream,
    mut bytes: *const ::core::ffi::c_void,
    mut nb_bytes: size_t,
) -> *mut bytestream {
    (*s).data = bytes as *mut uint8_t;
    (*s).size = nb_bytes;
    (*s).ptr = 0 as size_t;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn bytestream_buf_init(
    mut s: *mut bytestream_buf,
    mut nb_bytes: size_t,
) -> *mut bytestream {
    if nb_bytes > BYTESTREAM_MAX_BUFFER_SIZE as size_t {
        return ::core::ptr::null_mut::<bytestream>();
    }
    (*s).s.data = &raw mut (*s).buf as *mut uint8_t;
    (*s).s.size = nb_bytes;
    (*s).s.ptr = 0 as size_t;
    return &raw mut (*s).s;
}
#[no_mangle]
pub unsafe extern "C" fn bytestream_alloc(
    mut s: *mut bytestream,
    mut nb_bytes: size_t,
) -> *mut bytestream {
    (*s).data = malloc(nb_bytes) as *mut uint8_t;
    if (*s).data.is_null() {
        free(s as *mut ::core::ffi::c_void);
        return ::core::ptr::null_mut::<bytestream>();
    }
    (*s).size = nb_bytes;
    (*s).ptr = 0 as size_t;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn bytestream_delete(mut s: *mut bytestream) {
    if !(*s).data.is_null() {
        free((*s).data as *mut ::core::ffi::c_void);
        (*s).data = ::core::ptr::null_mut::<uint8_t>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn bytestream_data(mut s: *mut bytestream) -> *const uint8_t {
    return (*s).data;
}
#[no_mangle]
pub unsafe extern "C" fn bytestream_ptr(mut s: *mut bytestream) -> *const uint8_t {
    return (*s).data.offset((*s).ptr as isize);
}
#[no_mangle]
pub unsafe extern "C" fn bytestream_size(mut s: *mut bytestream) -> size_t {
    return (*s).size;
}
#[no_mangle]
pub unsafe extern "C" fn bytestream_length(mut s: *mut bytestream) -> size_t {
    return (*s).ptr;
}
#[no_mangle]
pub unsafe extern "C" fn bytestream_remain(mut s: *mut bytestream) -> size_t {
    return (*s).size.wrapping_sub((*s).ptr);
}
#[no_mangle]
pub unsafe extern "C" fn bytestream_reset(mut s: *mut bytestream) {
    (*s).ptr = 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn bytestream_clear(mut s: *mut bytestream) {
    (*s).ptr = 0 as size_t;
    memset(
        (*s).data as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (*s).size,
    );
}
#[no_mangle]
pub unsafe extern "C" fn bytestream_finished(mut s: *mut bytestream) -> ::core::ffi::c_int {
    return ((*s).ptr >= (*s).size) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bytestream_skip(
    mut s: *mut bytestream,
    mut nb_bytes: size_t,
) -> ::core::ffi::c_int {
    let mut max_bytes: size_t = (*s).size.wrapping_sub((*s).ptr);
    if max_bytes < nb_bytes {
        return bytestream_error(s);
    } else {
        (*s).ptr = (*s).ptr.wrapping_add(nb_bytes);
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn byteread_skip_vint(mut s: *mut bytestream) -> ::core::ffi::c_int {
    let mut max_bytes: size_t = (*s).size.wrapping_sub((*s).ptr);
    if max_bytes < 1 as size_t {
        return bytestream_error(s);
    }
    let mut len: size_t = picoquic_decode_varint_length(*(*s).data.offset((*s).ptr as isize));
    return bytestream_skip(s, len);
}
#[no_mangle]
pub unsafe extern "C" fn bytewrite_vint(
    mut s: *mut bytestream,
    mut value: uint64_t,
) -> ::core::ffi::c_int {
    let mut len: size_t = picoquic_varint_encode(
        (*s).data.offset((*s).ptr as isize),
        (*s).size.wrapping_sub((*s).ptr),
        value,
    );
    if len == 0 as size_t {
        return bytestream_error(s);
    } else {
        (*s).ptr = (*s).ptr.wrapping_add(len);
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn byteread_vint(
    mut s: *mut bytestream,
    mut value: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut max_bytes: size_t = (*s).size.wrapping_sub((*s).ptr);
    if max_bytes < 1 as size_t {
        return bytestream_error(s);
    }
    let mut len: size_t = picoquic_varint_decode(
        (*s).data.offset((*s).ptr as isize),
        (*s).size.wrapping_sub((*s).ptr),
        value,
    );
    if len == 0 as size_t {
        return bytestream_error(s);
    } else {
        (*s).ptr = (*s).ptr.wrapping_add(len);
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bytestream_vint_len(mut value: uint64_t) -> size_t {
    return picoquic_encode_varint_length(value);
}
#[no_mangle]
pub unsafe extern "C" fn byteread_vlen(
    mut s: *mut bytestream,
    mut value: *mut size_t,
) -> ::core::ffi::c_int {
    let mut val_read: uint64_t = 0 as uint64_t;
    let mut ret: ::core::ffi::c_int = byteread_vint(s, &raw mut val_read);
    *value = val_read as size_t;
    return if *value != val_read as size_t {
        -(1 as ::core::ffi::c_int)
    } else {
        ret
    };
}
#[no_mangle]
pub unsafe extern "C" fn bytewrite_int8(
    mut s: *mut bytestream,
    mut value: uint8_t,
) -> ::core::ffi::c_int {
    let mut max_bytes: size_t = (*s).size.wrapping_sub((*s).ptr);
    if max_bytes < 1 as size_t {
        return bytestream_error(s);
    } else {
        let c2rust_fresh0 = (*s).ptr;
        (*s).ptr = (*s).ptr.wrapping_add(1);
        *(*s).data.offset(c2rust_fresh0 as isize) = value;
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn byteread_int8(
    mut s: *mut bytestream,
    mut value: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut max_bytes: size_t = (*s).size.wrapping_sub((*s).ptr);
    if max_bytes < 1 as size_t {
        return bytestream_error(s);
    } else {
        let c2rust_fresh1 = (*s).ptr;
        (*s).ptr = (*s).ptr.wrapping_add(1);
        *value = *(*s).data.offset(c2rust_fresh1 as isize);
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn byteshow_int8(
    mut s: *mut bytestream,
    mut value: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut max_bytes: size_t = (*s).size.wrapping_sub((*s).ptr);
    if max_bytes < 1 as size_t {
        return -(1 as ::core::ffi::c_int);
    } else {
        *value = *(*s).data.offset((*s).ptr as isize);
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bytewrite_int16(
    mut s: *mut bytestream,
    mut value: uint16_t,
) -> ::core::ffi::c_int {
    let mut max_bytes: size_t = (*s).size.wrapping_sub((*s).ptr);
    if max_bytes < 2 as size_t {
        return bytestream_error(s);
    } else {
        picoformat_16((*s).data.offset((*s).ptr as isize), value);
        (*s).ptr = (*s).ptr.wrapping_add(2 as size_t);
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn byteread_int16(
    mut s: *mut bytestream,
    mut value: *mut uint16_t,
) -> ::core::ffi::c_int {
    let mut max_bytes: size_t = (*s).size.wrapping_sub((*s).ptr);
    if max_bytes < 2 as size_t {
        return bytestream_error(s);
    } else {
        let mut ptr: *const uint8_t = (*s).data.offset((*s).ptr as isize);
        *value = ((*ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            as uint16_t;
        (*s).ptr = (*s).ptr.wrapping_add(2 as size_t);
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bytewrite_int32(
    mut s: *mut bytestream,
    mut value: uint32_t,
) -> ::core::ffi::c_int {
    let mut max_bytes: size_t = (*s).size.wrapping_sub((*s).ptr);
    if max_bytes < 4 as size_t {
        return bytestream_error(s);
    } else {
        picoformat_32((*s).data.offset((*s).ptr as isize), value);
        (*s).ptr = (*s).ptr.wrapping_add(4 as size_t);
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn byteread_int32(
    mut s: *mut bytestream,
    mut value: *mut uint32_t,
) -> ::core::ffi::c_int {
    let mut max_bytes: size_t = (*s).size.wrapping_sub((*s).ptr);
    if max_bytes < 4 as size_t {
        return bytestream_error(s);
    } else {
        let mut ptr: *const uint8_t = (*s).data.offset((*s).ptr as isize);
        *value = (*ptr.offset(0 as ::core::ffi::c_int as isize) as uint32_t)
            << 24 as ::core::ffi::c_int
            | (*ptr.offset(1 as ::core::ffi::c_int as isize) as uint32_t)
                << 16 as ::core::ffi::c_int
            | (*ptr.offset(2 as ::core::ffi::c_int as isize) as uint32_t)
                << 8 as ::core::ffi::c_int
            | *ptr.offset(3 as ::core::ffi::c_int as isize) as uint32_t;
        (*s).ptr = (*s).ptr.wrapping_add(4 as size_t);
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bytewrite_int64(
    mut s: *mut bytestream,
    mut value: uint64_t,
) -> ::core::ffi::c_int {
    let mut max_bytes: size_t = (*s).size.wrapping_sub((*s).ptr);
    if max_bytes < 8 as size_t {
        return bytestream_error(s);
    } else {
        picoformat_64((*s).data.offset((*s).ptr as isize), value);
        (*s).ptr = (*s).ptr.wrapping_add(8 as size_t);
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn byteread_int64(
    mut s: *mut bytestream,
    mut value: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut max_bytes: size_t = (*s).size.wrapping_sub((*s).ptr);
    if max_bytes < 8 as size_t {
        return bytestream_error(s);
    } else {
        let mut v: uint64_t = 0 as uint64_t;
        let mut i: size_t = 0 as size_t;
        while i < 8 as size_t {
            v <<= 8 as ::core::ffi::c_int;
            let c2rust_fresh2 = (*s).ptr;
            (*s).ptr = (*s).ptr.wrapping_add(1);
            v = v.wrapping_add(*(*s).data.offset(c2rust_fresh2 as isize) as uint64_t);
            i = i.wrapping_add(1);
        }
        *value = v;
        return 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn bytewrite_buffer(
    mut s: *mut bytestream,
    mut buffer: *const ::core::ffi::c_void,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let mut max_bytes: size_t = (*s).size.wrapping_sub((*s).ptr);
    if max_bytes < length {
        return bytestream_error(s);
    }
    memcpy(
        (*s).data.offset((*s).ptr as isize) as *mut ::core::ffi::c_void,
        buffer,
        length,
    );
    (*s).ptr = (*s).ptr.wrapping_add(length);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn byteread_buffer(
    mut s: *mut bytestream,
    mut buffer: *mut ::core::ffi::c_void,
    mut length: size_t,
) -> ::core::ffi::c_int {
    let mut max_bytes: size_t = (*s).size.wrapping_sub((*s).ptr);
    if max_bytes < length {
        return bytestream_error(s);
    }
    memcpy(
        buffer,
        (*s).data.offset((*s).ptr as isize) as *const ::core::ffi::c_void,
        length,
    );
    (*s).ptr = (*s).ptr.wrapping_add(length);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bytewrite_cid(
    mut s: *mut bytestream,
    mut cid: *const picoquic_connection_id_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = bytewrite_int8(s, (*cid).id_len);
    ret |= bytewrite_buffer(
        s,
        &raw const (*cid).id as *const uint8_t as *const ::core::ffi::c_void,
        (*cid).id_len as size_t,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn byteread_cid(
    mut s: *mut bytestream,
    mut cid: *mut picoquic_connection_id_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = byteread_int8(s, &raw mut (*cid).id_len);
    if (*cid).id_len as ::core::ffi::c_int > PICOQUIC_CONNECTION_ID_MAX_SIZE {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        memset(
            &raw mut (*cid).id as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
        );
        ret |= byteread_buffer(
            s,
            &raw mut (*cid).id as *mut uint8_t as *mut ::core::ffi::c_void,
            (*cid).id_len as size_t,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn byteskip_cid(mut s: *mut bytestream) -> ::core::ffi::c_int {
    let mut id_len: uint8_t = 0 as uint8_t;
    let mut ret: ::core::ffi::c_int = byteread_int8(s, &raw mut id_len);
    ret |= bytestream_skip(s, id_len as size_t);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn bytewrite_cstr(
    mut s: *mut bytestream,
    mut cstr: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut l_cstr: size_t = strlen(cstr);
    let mut ret: ::core::ffi::c_int = bytewrite_vint(s, l_cstr as uint64_t);
    ret |= bytewrite_buffer(s, cstr as *const ::core::ffi::c_void, l_cstr);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn byteread_cstr(
    mut s: *mut bytestream,
    mut cstr: *mut ::core::ffi::c_char,
    mut max_len: size_t,
) -> ::core::ffi::c_int {
    let mut l_read: uint64_t = 0 as uint64_t;
    let mut ret: ::core::ffi::c_int = byteread_vint(s, &raw mut l_read);
    let mut l_cstr: size_t = l_read as size_t;
    if ret != 0 as ::core::ffi::c_int
        || l_cstr != l_read as size_t
        || l_cstr.wrapping_add(1 as size_t) > max_len
    {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        ret |= byteread_buffer(s, cstr as *mut ::core::ffi::c_void, l_cstr);
        *cstr.offset(l_cstr as isize) = 0 as ::core::ffi::c_char;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn byteskip_cstr(mut s: *mut bytestream) -> ::core::ffi::c_int {
    let mut l_read: uint64_t = 0 as uint64_t;
    let mut ret: ::core::ffi::c_int = byteread_vint(s, &raw mut l_read);
    let mut l_cstr: size_t = l_read as size_t;
    if ret != 0 as ::core::ffi::c_int || l_cstr != l_read as size_t {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        ret = bytestream_skip(s, l_cstr);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn bytewrite_addr(
    mut s: *mut bytestream,
    mut addr: *const sockaddr,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = bytewrite_vint(s, (*addr).sa_family as uint64_t);
    if (*addr).sa_family as ::core::ffi::c_int == AF_INET {
        let mut s4: *mut sockaddr_in = addr as *mut sockaddr_in;
        ret |= bytewrite_buffer(
            s,
            &raw mut (*s4).sin_addr as *const ::core::ffi::c_void,
            4 as size_t,
        );
        ret |= bytewrite_int16(s, (*s4).sin_port as uint16_t);
    } else {
        let mut s6: *mut sockaddr_in6 = addr as *mut sockaddr_in6;
        ret |= bytewrite_buffer(
            s,
            &raw mut (*s6).sin6_addr as *const ::core::ffi::c_void,
            16 as size_t,
        );
        ret |= bytewrite_int16(s, (*s6).sin6_port as uint16_t);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn byteread_addr(
    mut s: *mut bytestream,
    mut addr: *mut sockaddr_storage,
) -> ::core::ffi::c_int {
    let mut family: uint64_t = 0 as uint64_t;
    let mut ret: ::core::ffi::c_int = byteread_vint(s, &raw mut family);
    if ret == 0 as ::core::ffi::c_int && family == AF_INET as uint64_t {
        let mut s4: *mut sockaddr_in = addr as *mut sockaddr_in;
        (*s4).sin_family = AF_INET as sa_family_t;
        ret |= byteread_buffer(
            s,
            &raw mut (*s4).sin_addr as *mut ::core::ffi::c_void,
            4 as size_t,
        );
        ret |= byteread_int16(s, &raw mut (*s4).sin_port);
    } else {
        let mut s6: *mut sockaddr_in6 = addr as *mut sockaddr_in6;
        (*s6).sin6_family = AF_INET6 as sa_family_t;
        ret |= byteread_buffer(
            s,
            &raw mut (*s6).sin6_addr as *mut ::core::ffi::c_void,
            16 as size_t,
        );
        ret |= byteread_int16(s, &raw mut (*s6).sin6_port);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn byteskip_addr(mut s: *mut bytestream) -> ::core::ffi::c_int {
    let mut family: uint64_t = 0 as uint64_t;
    let mut ret: ::core::ffi::c_int = byteread_vint(s, &raw mut family);
    if ret == 0 as ::core::ffi::c_int && family == AF_INET as uint64_t {
        ret |= bytestream_skip(
            s,
            (4 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as size_t,
        );
    } else {
        ret |= bytestream_skip(
            s,
            (16 as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as size_t,
        );
    }
    return ret;
}
unsafe extern "C" fn bytestream_error(mut s: *mut bytestream) -> ::core::ffi::c_int {
    (*s).ptr = (*s).size;
    return -(1 as ::core::ffi::c_int);
}

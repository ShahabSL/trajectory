extern "C" {
    fn inet_pton(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn inet_ntop(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_void,
        __buf: *mut ::core::ffi::c_char,
        __len: socklen_t,
    ) -> *const ::core::ffi::c_char;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn debug_printf(fmt: *const ::core::ffi::c_char, ...);
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __socklen_t = ::core::ffi::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
pub type socklen_t = __socklen_t;
pub type sa_family_t = ::core::ffi::c_ushort;
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
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const INET6_ADDRSTRLEN: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const DBG_PRINTF_FILENAME_MAX: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn picoquic_connection_id_to_string(
    mut cid: *const picoquic_connection_id_t,
) -> *mut ::core::ffi::c_char {
    let mut str: *mut ::core::ffi::c_char = malloc(
        (((*cid).id_len as ::core::ffi::c_int * 2 as ::core::ffi::c_int + 1 as ::core::ffi::c_int)
            as size_t)
            .wrapping_mul(::core::mem::size_of::<::core::ffi::c_char>() as size_t),
    ) as *mut ::core::ffi::c_char;
    if str.is_null() {
        return ::core::ptr::null_mut::<::core::ffi::c_char>();
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*cid).id_len as ::core::ffi::c_int {
        sprintf(
            str.offset((i * 2 as ::core::ffi::c_int) as isize) as *mut ::core::ffi::c_char,
            b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
            (*cid).id[i as usize] as ::core::ffi::c_int,
        );
        i += 1;
    }
    *str.offset(((*cid).id_len as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as isize) =
        '\0' as i32 as ::core::ffi::c_char;
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn sockaddr_dummy(mut addr_storage: *mut sockaddr_storage) {
    memset(
        addr_storage as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_storage>() as size_t,
    );
    let mut addr4: *mut sockaddr_in = addr_storage as *mut sockaddr_in;
    (*addr4).sin_family = AF_INET as sa_family_t;
    inet_pton(
        AF_INET,
        b"192.0.2.1\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*addr4).sin_addr as *mut ::core::ffi::c_void,
    );
    (*addr4).sin_port = __bswap_16(12345 as __uint16_t) as in_port_t;
}
#[no_mangle]
pub unsafe extern "C" fn print_sockaddr_ip_and_port(mut addr_storage: *mut sockaddr_storage) {
    let mut ip_str: [::core::ffi::c_char; 46] = [0; 46];
    let mut port: ::core::ffi::c_int = 0;
    if (*addr_storage).ss_family as ::core::ffi::c_int == AF_INET {
        let mut addr4: *mut sockaddr_in = addr_storage as *mut sockaddr_in;
        inet_ntop(
            AF_INET,
            &raw mut (*addr4).sin_addr as *const ::core::ffi::c_void,
            &raw mut ip_str as *mut ::core::ffi::c_char,
            INET6_ADDRSTRLEN as socklen_t,
        );
        port = __bswap_16((*addr4).sin_port as __uint16_t) as ::core::ffi::c_int;
    } else if (*addr_storage).ss_family as ::core::ffi::c_int == AF_INET6 {
        let mut addr6: *mut sockaddr_in6 = addr_storage as *mut sockaddr_in6;
        inet_ntop(
            AF_INET6,
            &raw mut (*addr6).sin6_addr as *const ::core::ffi::c_void,
            &raw mut ip_str as *mut ::core::ffi::c_char,
            INET6_ADDRSTRLEN as socklen_t,
        );
        port = __bswap_16((*addr6).sin6_port as __uint16_t) as ::core::ffi::c_int;
    } else {
        debug_printf(
            b"%s:%u [%s]: Unknown address family\n\0".as_ptr() as *const ::core::ffi::c_char,
            (b"../src/slipstream_utils.c\0".as_ptr() as *const ::core::ffi::c_char).offset(
                (if 24 as usize > ::core::mem::size_of::<[::core::ffi::c_char; 26]>() as usize {
                    24 as usize
                } else {
                    ::core::mem::size_of::<[::core::ffi::c_char; 26]>() as usize
                })
                .wrapping_sub(DBG_PRINTF_FILENAME_MAX as usize) as isize,
            ) as *const ::core::ffi::c_char,
            62 as ::core::ffi::c_int,
            b"print_sockaddr_ip_and_port\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        return;
    }
    debug_printf(
        b"%s:%u [%s]: %s:%d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (b"../src/slipstream_utils.c\0".as_ptr() as *const ::core::ffi::c_char).offset(
            (if 24 as usize > ::core::mem::size_of::<[::core::ffi::c_char; 26]>() as usize {
                24 as usize
            } else {
                ::core::mem::size_of::<[::core::ffi::c_char; 26]>() as usize
            })
            .wrapping_sub(DBG_PRINTF_FILENAME_MAX as usize) as isize,
        ) as *const ::core::ffi::c_char,
        66 as ::core::ffi::c_int,
        b"print_sockaddr_ip_and_port\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut ip_str as *mut ::core::ffi::c_char,
        port,
    );
}

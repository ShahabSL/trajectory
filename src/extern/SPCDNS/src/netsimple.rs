extern "C" {
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn sendto(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
        __addr: *const sockaddr,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn recvfrom(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn inet_pton(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type nfds_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __ssize_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type ssize_t = __ssize_t;
pub type socklen_t = __socklen_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uintptr_t = usize;
pub type __socket_type = ::core::ffi::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
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
pub type dns_packet_t = uintptr_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sockaddr_all {
    pub sa: sockaddr,
    pub sin: sockaddr_in,
    pub sin6: sockaddr_in6,
}
pub const EPROTOTYPE: ::core::ffi::c_int = 91 as ::core::ffi::c_int;
pub const ETIMEDOUT: ::core::ffi::c_int = 110 as ::core::ffi::c_int;
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
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
#[no_mangle]
pub unsafe extern "C" fn net_server(
    mut addr: *mut sockaddr_all,
    mut host: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if !addr.is_null() {
        } else {
            __assert_fail(
                b"addr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/netsimple.c\0".as_ptr() as *const ::core::ffi::c_char,
                63 as ::core::ffi::c_uint,
                b"int net_server(sockaddr_all *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !host.is_null() {
        } else {
            __assert_fail(
                b"host != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/netsimple.c\0".as_ptr() as *const ::core::ffi::c_char,
                64 as ::core::ffi::c_uint,
                b"int net_server(sockaddr_all *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    memset(
        addr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_all>() as size_t,
    );
    if inet_pton(
        AF_INET,
        host,
        &raw mut (*addr).sin.sin_addr.s_addr as *mut ::core::ffi::c_void,
    ) < 0 as ::core::ffi::c_int
    {
        if inet_pton(
            AF_INET6,
            host,
            &raw mut (*addr).sin6.sin6_addr.__in6_u.__u6_addr8 as *mut ::core::ffi::c_void,
        ) < 0 as ::core::ffi::c_int
        {
            return *__errno_location();
        }
        (*addr).sin6.sin6_family = AF_INET6 as sa_family_t;
        (*addr).sin6.sin6_port = __bswap_16(53 as __uint16_t) as in_port_t;
    } else {
        (*addr).sin.sin_family = AF_INET as sa_family_t;
        (*addr).sin.sin_port = __bswap_16(53 as __uint16_t) as in_port_t;
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn net_request(
    mut srvaddr: *mut sockaddr_all,
    mut dest: *mut dns_packet_t,
    mut dsize: *mut size_t,
    mut src: *const dns_packet_t,
    mut ssize: size_t,
) -> ::core::ffi::c_int {
    let mut polldat: pollfd = pollfd {
        fd: 0,
        events: 0,
        revents: 0,
    };
    let mut asize: socklen_t = 0;
    let mut bytes: ssize_t = 0;
    let mut sock: ::core::ffi::c_int = 0;
    let mut rc: ::core::ffi::c_int = 0;
    let mut err: ::core::ffi::c_int = 0;
    match (*srvaddr).sa.sa_family as ::core::ffi::c_int {
        AF_INET => {
            asize = ::core::mem::size_of::<sockaddr_in>() as socklen_t;
        }
        AF_INET6 => {
            asize = ::core::mem::size_of::<sockaddr_in6>() as socklen_t;
        }
        _ => {
            '_c2rust_label: {
                __assert_fail(
                    b"0\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../extern/SPCDNS/src/netsimple.c\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    105 as ::core::ffi::c_uint,
                    b"int net_request(sockaddr_all *, dns_packet_t *, size_t *, const dns_packet_t *, size_t)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            };
            return EPROTOTYPE;
        }
    }
    sock = socket(
        (*srvaddr).sa.sa_family as ::core::ffi::c_int,
        SOCK_DGRAM as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if sock < 0 as ::core::ffi::c_int {
        return *__errno_location();
    }
    bytes = sendto(
        sock,
        src as *const ::core::ffi::c_void,
        ssize,
        0 as ::core::ffi::c_int,
        &raw mut (*srvaddr).sa,
        asize,
    );
    if bytes < 0 as ssize_t {
        err = *__errno_location();
        close(sock);
        return err;
    }
    polldat.fd = sock;
    polldat.events = POLLIN as ::core::ffi::c_short;
    rc = poll(&raw mut polldat, 1 as nfds_t, 15000 as ::core::ffi::c_int);
    if rc < 0 as ::core::ffi::c_int {
        err = *__errno_location();
        close(sock);
        return err;
    }
    if rc == 0 as ::core::ffi::c_int {
        close(sock);
        return ETIMEDOUT;
    }
    bytes = recvfrom(
        sock,
        dest as *mut ::core::ffi::c_void,
        *dsize,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<sockaddr>(),
        ::core::ptr::null_mut::<socklen_t>(),
    );
    if bytes < 0 as ssize_t {
        let mut err_0: ::core::ffi::c_int = *__errno_location();
        close(sock);
        return err_0;
    }
    *dsize = bytes as size_t;
    close(sock);
    return 0 as ::core::ffi::c_int;
}

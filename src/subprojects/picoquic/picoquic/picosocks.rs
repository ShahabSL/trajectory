extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type st_picoquic_quic_t;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn select(
        __nfds: ::core::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::core::ffi::c_int;
    fn getenv(__name: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
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
    fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn bind(
        __fd: ::core::ffi::c_int,
        __addr: *const sockaddr,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    fn getsockname(
        __fd: ::core::ffi::c_int,
        __addr: *mut sockaddr,
        __len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn sendmsg(
        __fd: ::core::ffi::c_int,
        __message: *const msghdr,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn recvmsg(
        __fd: ::core::ffi::c_int,
        __message: *mut msghdr,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn setsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *const ::core::ffi::c_void,
        __optlen: socklen_t,
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn inet_pton(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn getaddrinfo(
        __name: *const ::core::ffi::c_char,
        __service: *const ::core::ffi::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> ::core::ffi::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn picoquic_current_time() -> uint64_t;
    fn picoquic_is_sslkeylog_enabled(quic: *mut picoquic_quic_t) -> ::core::ffi::c_int;
    fn picoquic_set_key_log_file(
        quic: *mut picoquic_quic_t,
        keylog_filename: *const ::core::ffi::c_char,
    );
    fn picoquic_addr_length(addr: *const sockaddr) -> ::core::ffi::c_int;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub __fds_bits: [__fd_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut ::core::ffi::c_void,
    pub iov_len: size_t,
}
pub type socklen_t = __socklen_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [::core::ffi::c_char; 118],
    pub __ss_align: ::core::ffi::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut ::core::ffi::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut ::core::ffi::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: ::core::ffi::c_int,
    pub cmsg_type: ::core::ffi::c_int,
    pub __cmsg_data: [::core::ffi::c_uchar; 0],
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_pktinfo {
    pub ipi_ifindex: ::core::ffi::c_int,
    pub ipi_spec_dst: in_addr,
    pub ipi_addr: in_addr,
}
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const IPPROTO_MAX: C2Rust_Unnamed = 263;
pub const IPPROTO_MPTCP: C2Rust_Unnamed = 262;
pub const IPPROTO_RAW: C2Rust_Unnamed = 255;
pub const IPPROTO_ETHERNET: C2Rust_Unnamed = 143;
pub const IPPROTO_MPLS: C2Rust_Unnamed = 137;
pub const IPPROTO_UDPLITE: C2Rust_Unnamed = 136;
pub const IPPROTO_SCTP: C2Rust_Unnamed = 132;
pub const IPPROTO_L2TP: C2Rust_Unnamed = 115;
pub const IPPROTO_COMP: C2Rust_Unnamed = 108;
pub const IPPROTO_PIM: C2Rust_Unnamed = 103;
pub const IPPROTO_ENCAP: C2Rust_Unnamed = 98;
pub const IPPROTO_BEETPH: C2Rust_Unnamed = 94;
pub const IPPROTO_MTP: C2Rust_Unnamed = 92;
pub const IPPROTO_AH: C2Rust_Unnamed = 51;
pub const IPPROTO_ESP: C2Rust_Unnamed = 50;
pub const IPPROTO_GRE: C2Rust_Unnamed = 47;
pub const IPPROTO_RSVP: C2Rust_Unnamed = 46;
pub const IPPROTO_IPV6: C2Rust_Unnamed = 41;
pub const IPPROTO_DCCP: C2Rust_Unnamed = 33;
pub const IPPROTO_TP: C2Rust_Unnamed = 29;
pub const IPPROTO_IDP: C2Rust_Unnamed = 22;
pub const IPPROTO_UDP: C2Rust_Unnamed = 17;
pub const IPPROTO_PUP: C2Rust_Unnamed = 12;
pub const IPPROTO_EGP: C2Rust_Unnamed = 8;
pub const IPPROTO_TCP: C2Rust_Unnamed = 6;
pub const IPPROTO_IPIP: C2Rust_Unnamed = 4;
pub const IPPROTO_IGMP: C2Rust_Unnamed = 2;
pub const IPPROTO_ICMP: C2Rust_Unnamed = 1;
pub const IPPROTO_IP: C2Rust_Unnamed = 0;
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
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
pub struct in6_pktinfo {
    pub ipi6_addr: in6_addr,
    pub ipi6_ifindex: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: ::core::ffi::c_int,
    pub ai_family: ::core::ffi::c_int,
    pub ai_socktype: ::core::ffi::c_int,
    pub ai_protocol: ::core::ffi::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut ::core::ffi::c_char,
    pub ai_next: *mut addrinfo,
}
pub type picoquic_quic_t = st_picoquic_quic_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_server_sockets_t {
    pub s_socket: [::core::ffi::c_int; 2],
}
pub type picoquic_server_sockets_t = st_picoquic_server_sockets_t;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
pub const __NFDBITS: ::core::ffi::c_int =
    8 as ::core::ffi::c_int * ::core::mem::size_of::<__fd_mask>() as ::core::ffi::c_int;
pub const PF_UNSPEC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_UNSPEC: ::core::ffi::c_int = PF_UNSPEC;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
#[inline]
unsafe extern "C" fn __cmsg_nxthdr(
    mut __mhdr: *mut msghdr,
    mut __cmsg: *mut cmsghdr,
) -> *mut cmsghdr {
    let mut __msg_control_ptr: *mut ::core::ffi::c_uchar =
        (*__mhdr).msg_control as *mut ::core::ffi::c_uchar;
    let mut __cmsg_ptr: *mut ::core::ffi::c_uchar = __cmsg as *mut ::core::ffi::c_uchar;
    let cmsg_hdr = ::core::ptr::read_unaligned(__cmsg);
    let mut __size_needed: size_t = (::core::mem::size_of::<cmsghdr>() as size_t).wrapping_add(
        (::core::mem::size_of::<size_t>() as size_t).wrapping_sub(
            cmsg_hdr.cmsg_len
                & (::core::mem::size_of::<size_t>() as size_t).wrapping_sub(1 as size_t),
        ) & (::core::mem::size_of::<size_t>() as size_t).wrapping_sub(1 as size_t),
    );
    if cmsg_hdr.cmsg_len < ::core::mem::size_of::<cmsghdr>() as usize {
        return ::core::ptr::null_mut::<cmsghdr>();
    }
    if (__msg_control_ptr
        .offset((*__mhdr).msg_controllen as isize)
        .offset_from(__cmsg_ptr) as ::core::ffi::c_long as size_t)
        < __size_needed
        || (__msg_control_ptr
            .offset((*__mhdr).msg_controllen as isize)
            .offset_from(__cmsg_ptr) as ::core::ffi::c_long as size_t)
            .wrapping_sub(__size_needed)
            < cmsg_hdr.cmsg_len
    {
        return ::core::ptr::null_mut::<cmsghdr>();
    }
    __cmsg = (__cmsg as *mut ::core::ffi::c_uchar).offset(
        (cmsg_hdr
            .cmsg_len
            .wrapping_add(::core::mem::size_of::<size_t>() as size_t)
            .wrapping_sub(1 as size_t)
            & !(::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize))
            as isize,
    ) as *mut cmsghdr;
    return __cmsg;
}
pub const EAFNOSUPPORT: ::core::ffi::c_int = 97 as ::core::ffi::c_int;
pub const ENETDOWN: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const ENETUNREACH: ::core::ffi::c_int = 101 as ::core::ffi::c_int;
pub const ECONNRESET: ::core::ffi::c_int = 104 as ::core::ffi::c_int;
pub const EHOSTUNREACH: ::core::ffi::c_int = 113 as ::core::ffi::c_int;
pub const IP_TOS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const IP_PKTINFO: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const IP_MTU_DISCOVER: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const IP_RECVTOS: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const IP_PMTUDISC_PROBE: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const IPV6_MTU_DISCOVER: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const IPV6_V6ONLY: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const IPV6_RECVPKTINFO: ::core::ffi::c_int = 49 as ::core::ffi::c_int;
pub const IPV6_PKTINFO: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const IPV6_DONTFRAG: ::core::ffi::c_int = 62 as ::core::ffi::c_int;
pub const IPV6_RECVTCLASS: ::core::ffi::c_int = 66 as ::core::ffi::c_int;
pub const IPV6_TCLASS: ::core::ffi::c_int = 67 as ::core::ffi::c_int;
pub const SOL_IPV6: ::core::ffi::c_int = 41 as ::core::ffi::c_int;
pub const UDP_SEGMENT: ::core::ffi::c_int = 103 as ::core::ffi::c_int;
pub const SOL_UDP: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const INVALID_SOCKET: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
pub const PICOQUIC_ECN_ECT_1: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PICOQUIC_NB_SERVER_SOCKETS: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn picoquic_bind_to_port(
    mut fd: ::core::ffi::c_int,
    mut af: ::core::ffi::c_int,
    mut port: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sa: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut addr_length: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    memset(
        &raw mut sa as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_storage>() as size_t,
    );
    if af == AF_INET {
        let mut s4: *mut sockaddr_in = &raw mut sa as *mut sockaddr_in;
        (*s4).sin_family = af as sa_family_t;
        (*s4).sin_port = __bswap_16(port as __uint16_t) as in_port_t;
        addr_length = ::core::mem::size_of::<sockaddr_in>() as ::core::ffi::c_int;
    } else {
        let mut s6: *mut sockaddr_in6 = &raw mut sa as *mut sockaddr_in6;
        (*s6).sin6_family = AF_INET6 as sa_family_t;
        (*s6).sin6_port = __bswap_16(port as __uint16_t) as in_port_t;
        addr_length = ::core::mem::size_of::<sockaddr_in6>() as ::core::ffi::c_int;
    }
    return bind(fd, &raw mut sa as *mut sockaddr, addr_length as socklen_t);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_local_address(
    mut sd: ::core::ffi::c_int,
    mut addr: *mut sockaddr_storage,
) -> ::core::ffi::c_int {
    let mut name_len: socklen_t = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
    return getsockname(sd, addr as *mut sockaddr, &raw mut name_len);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_socket_set_pkt_info(
    mut sd: ::core::ffi::c_int,
    mut af: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if af == AF_INET6 {
        let mut val: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        ret = setsockopt(
            sd,
            IPPROTO_IPV6 as ::core::ffi::c_int,
            IPV6_V6ONLY,
            &raw mut val as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        );
        if ret == 0 as ::core::ffi::c_int {
            val = 1 as ::core::ffi::c_int;
            ret = setsockopt(
                sd,
                IPPROTO_IPV6 as ::core::ffi::c_int,
                IPV6_RECVPKTINFO,
                &raw mut val as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
                ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
            );
        }
    } else {
        let mut val_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        ret = setsockopt(
            sd,
            IPPROTO_IP as ::core::ffi::c_int,
            IP_PKTINFO,
            &raw mut val_0 as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_socket_set_ecn_options(
    mut sd: ::core::ffi::c_int,
    mut af: ::core::ffi::c_int,
    mut recv_set: *mut ::core::ffi::c_int,
    mut send_set: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if af == AF_INET6 {
        let mut ecn: ::core::ffi::c_uint = PICOQUIC_ECN_ECT_1 as ::core::ffi::c_uint;
        if setsockopt(
            sd,
            IPPROTO_IPV6 as ::core::ffi::c_int,
            IPV6_TCLASS,
            &raw mut ecn as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_uint>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            *send_set = 0 as ::core::ffi::c_int;
        } else {
            *send_set = 1 as ::core::ffi::c_int;
        }
        let mut set: ::core::ffi::c_uint = 0x1 as ::core::ffi::c_uint;
        if setsockopt(
            sd,
            IPPROTO_IPV6 as ::core::ffi::c_int,
            IPV6_RECVTCLASS,
            &raw mut set as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_uint>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            ret = -(1 as ::core::ffi::c_int);
            *recv_set = 0 as ::core::ffi::c_int;
        } else {
            *recv_set = 1 as ::core::ffi::c_int;
            ret = 0 as ::core::ffi::c_int;
        }
    } else {
        let mut ecn_0: ::core::ffi::c_uint = PICOQUIC_ECN_ECT_1 as ::core::ffi::c_uint;
        if setsockopt(
            sd,
            IPPROTO_IP as ::core::ffi::c_int,
            IP_TOS,
            &raw mut ecn_0 as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_uint>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            *send_set = 0 as ::core::ffi::c_int;
        } else {
            *send_set = 1 as ::core::ffi::c_int;
        }
        let mut set_0: ::core::ffi::c_uint = 1 as ::core::ffi::c_uint;
        if setsockopt(
            sd,
            IPPROTO_IP as ::core::ffi::c_int,
            IP_RECVTOS,
            &raw mut set_0 as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_uint>() as socklen_t,
        ) < 0 as ::core::ffi::c_int
        {
            ret = -(1 as ::core::ffi::c_int);
            *recv_set = 0 as ::core::ffi::c_int;
        } else {
            *recv_set = 1 as ::core::ffi::c_int;
            ret = 0 as ::core::ffi::c_int;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_socket_set_pmtud_options(
    mut sd: ::core::ffi::c_int,
    mut af: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut val: ::core::ffi::c_int = IP_PMTUDISC_PROBE;
    if af == AF_INET6 {
        ret = setsockopt(
            sd,
            IPPROTO_IPV6 as ::core::ffi::c_int,
            IPV6_MTU_DISCOVER,
            &raw mut val as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        );
    } else {
        ret = setsockopt(
            sd,
            IPPROTO_IP as ::core::ffi::c_int,
            IP_MTU_DISCOVER,
            &raw mut val as *const ::core::ffi::c_void,
            ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_open_client_socket(
    mut af: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sd: ::core::ffi::c_int = socket(
        af,
        SOCK_DGRAM as ::core::ffi::c_int,
        IPPROTO_UDP as ::core::ffi::c_int,
    );
    if sd != INVALID_SOCKET {
        let mut send_set: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut recv_set: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        picoquic_socket_set_pkt_info(sd, af) != 0 as ::core::ffi::c_int;
        picoquic_socket_set_ecn_options(sd, af, &raw mut recv_set, &raw mut send_set)
            != 0 as ::core::ffi::c_int;
        picoquic_socket_set_pmtud_options(sd, af) != 0 as ::core::ffi::c_int;
    }
    return sd;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_open_server_sockets(
    mut sockets: *mut picoquic_server_sockets_t,
    mut port: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let sock_af: [::core::ffi::c_int; 2] = [AF_INET6, AF_INET];
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < PICOQUIC_NB_SERVER_SOCKETS {
        if ret == 0 as ::core::ffi::c_int {
            (*sockets).s_socket[i as usize] = socket(
                sock_af[i as usize],
                SOCK_DGRAM as ::core::ffi::c_int,
                IPPROTO_UDP as ::core::ffi::c_int,
            );
        } else {
            (*sockets).s_socket[i as usize] = INVALID_SOCKET;
        }
        if (*sockets).s_socket[i as usize] == INVALID_SOCKET {
            ret = -(1 as ::core::ffi::c_int);
        } else {
            let mut recv_set: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut send_set: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            picoquic_socket_set_ecn_options(
                (*sockets).s_socket[i as usize],
                sock_af[i as usize],
                &raw mut recv_set,
                &raw mut send_set,
            ) != 0 as ::core::ffi::c_int;
            ret =
                picoquic_socket_set_pkt_info((*sockets).s_socket[i as usize], sock_af[i as usize]);
            if ret == 0 as ::core::ffi::c_int {
                ret = picoquic_bind_to_port(
                    (*sockets).s_socket[i as usize],
                    sock_af[i as usize],
                    port,
                );
            }
            if ret == 0 as ::core::ffi::c_int {
                ret = picoquic_socket_set_pmtud_options(
                    (*sockets).s_socket[i as usize],
                    sock_af[i as usize],
                );
            }
        }
        i += 1;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_close_server_sockets(
    mut sockets: *mut picoquic_server_sockets_t,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < PICOQUIC_NB_SERVER_SOCKETS {
        if (*sockets).s_socket[i as usize] != INVALID_SOCKET {
            close((*sockets).s_socket[i as usize]);
            (*sockets).s_socket[i as usize] = INVALID_SOCKET;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_socks_cmsg_parse(
    mut vmsg: *mut ::core::ffi::c_void,
    mut addr_dest: *mut sockaddr_storage,
    mut dest_if: *mut ::core::ffi::c_int,
    mut received_ecn: *mut ::core::ffi::c_uchar,
    mut udp_coalesced_size: *mut size_t,
) {
    let mut msg: *mut msghdr = vmsg as *mut msghdr;
    let mut cmsg: *mut cmsghdr = ::core::ptr::null_mut::<cmsghdr>();
    let cmsg_data_offset = (::core::mem::size_of::<cmsghdr>() as size_t)
        .wrapping_add(::core::mem::size_of::<size_t>() as size_t)
        .wrapping_sub(1 as size_t)
        & !(::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize);
    cmsg = if (*msg).msg_controllen >= ::core::mem::size_of::<cmsghdr>() as usize {
        (*msg).msg_control as *mut cmsghdr
    } else {
        ::core::ptr::null_mut::<cmsghdr>()
    };
    while !cmsg.is_null() {
        let cmsg_hdr = ::core::ptr::read_unaligned(cmsg);
        let cmsg_data = (cmsg as *mut ::core::ffi::c_uchar).offset(cmsg_data_offset as isize);
        if cmsg_hdr.cmsg_level == IPPROTO_IP as ::core::ffi::c_int {
            if cmsg_hdr.cmsg_type == IP_PKTINFO {
                if !addr_dest.is_null() {
                    let p_pkt_info = ::core::ptr::read_unaligned(
                        cmsg_data as *const in_pktinfo,
                    );
                    (*(addr_dest as *mut sockaddr_in)).sin_family = AF_INET as sa_family_t;
                    (*(addr_dest as *mut sockaddr_in)).sin_port = 0 as in_port_t;
                    (*(addr_dest as *mut sockaddr_in)).sin_addr.s_addr =
                        p_pkt_info.ipi_addr.s_addr;
                    if !dest_if.is_null() {
                        *dest_if = p_pkt_info.ipi_ifindex;
                    }
                }
            } else if (cmsg_hdr.cmsg_type == IP_TOS || cmsg_hdr.cmsg_type == IP_RECVTOS)
                && cmsg_hdr.cmsg_len > 0 as size_t
            {
                if !received_ecn.is_null() {
                    *received_ecn = *cmsg_data;
                }
            }
        } else if cmsg_hdr.cmsg_level == IPPROTO_IPV6 as ::core::ffi::c_int {
            if cmsg_hdr.cmsg_type == IPV6_PKTINFO {
                if !addr_dest.is_null() {
                    let p_pkt_info6 = ::core::ptr::read_unaligned(
                        cmsg_data as *const in6_pktinfo,
                    );
                    (*(addr_dest as *mut sockaddr_in6)).sin6_family = AF_INET6 as sa_family_t;
                    (*(addr_dest as *mut sockaddr_in6)).sin6_port = 0 as in_port_t;
                    memcpy(
                        &raw mut (*(addr_dest as *mut sockaddr_in6)).sin6_addr
                            as *mut ::core::ffi::c_void,
                        &p_pkt_info6.ipi6_addr as *const _ as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<in6_addr>() as size_t,
                    );
                    if !dest_if.is_null() {
                        *dest_if = p_pkt_info6.ipi6_ifindex as ::core::ffi::c_int;
                    }
                }
            } else if cmsg_hdr.cmsg_type == IPV6_TCLASS {
                if cmsg_hdr.cmsg_len > 0 as size_t && !received_ecn.is_null() {
                    *received_ecn = *cmsg_data;
                }
            }
        }
        cmsg = __cmsg_nxthdr(msg, cmsg);
    }
}
unsafe extern "C" fn cmsg_format_header_return_data_ptr(
    mut msg: *mut msghdr,
    mut last_cmsg: *mut *mut cmsghdr,
    mut control_length: *mut ::core::ffi::c_int,
    mut cmsg_level: ::core::ffi::c_int,
    mut cmsg_type: ::core::ffi::c_int,
    mut cmsg_data_len: size_t,
) -> *mut ::core::ffi::c_void {
    let mut cmsg_data_ptr: *mut ::core::ffi::c_void = NULL;
    let mut cmsg: *mut cmsghdr = if (*last_cmsg).is_null() {
        if (*msg).msg_controllen >= ::core::mem::size_of::<cmsghdr>() as usize {
            (*msg).msg_control as *mut cmsghdr
        } else {
            ::core::ptr::null_mut::<cmsghdr>()
        }
    } else {
        (*last_cmsg as *mut ::core::ffi::c_uchar).offset(
            ((**last_cmsg)
                .cmsg_len
                .wrapping_add(::core::mem::size_of::<size_t>() as size_t)
                .wrapping_sub(1 as size_t)
                & !(::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize))
                as isize,
        ) as *mut cmsghdr
    };
    if !cmsg.is_null() {
        let mut cmsg_required_space: size_t = (cmsg_data_len
            .wrapping_add(::core::mem::size_of::<size_t>() as size_t)
            .wrapping_sub(1 as size_t)
            & !(::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize))
        .wrapping_add(
            (::core::mem::size_of::<cmsghdr>() as size_t)
                .wrapping_add(::core::mem::size_of::<size_t>() as size_t)
                .wrapping_sub(1 as size_t)
                & !(::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize),
        );
        *control_length += cmsg_required_space as ::core::ffi::c_int;
        memset(
            cmsg as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            cmsg_required_space,
        );
        (*cmsg).cmsg_level = cmsg_level;
        (*cmsg).cmsg_type = cmsg_type;
        (*cmsg).cmsg_len = ((::core::mem::size_of::<cmsghdr>() as usize)
            .wrapping_add(::core::mem::size_of::<size_t>() as usize)
            .wrapping_sub(1 as usize)
            & !(::core::mem::size_of::<size_t>() as usize).wrapping_sub(1 as usize))
        .wrapping_add(cmsg_data_len as usize) as size_t;
        cmsg_data_ptr =
            &raw mut (*cmsg).__cmsg_data as *mut ::core::ffi::c_uchar as *mut ::core::ffi::c_void;
        *last_cmsg = cmsg;
    }
    return cmsg_data_ptr;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_socks_cmsg_format(
    mut vmsg: *mut ::core::ffi::c_void,
    mut message_length: size_t,
    mut send_msg_size: size_t,
    mut addr_from: *mut sockaddr,
    mut dest_if: ::core::ffi::c_int,
) {
    let mut msg: *mut msghdr = vmsg as *mut msghdr;
    let mut control_length: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut last_cmsg: *mut cmsghdr = ::core::ptr::null_mut::<cmsghdr>();
    let mut is_null: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !addr_from.is_null()
        && (*addr_from).sa_family as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        if (*addr_from).sa_family as ::core::ffi::c_int == AF_INET {
            let mut pktinfo: *mut in_pktinfo = cmsg_format_header_return_data_ptr(
                msg,
                &raw mut last_cmsg,
                &raw mut control_length,
                IPPROTO_IP as ::core::ffi::c_int,
                IP_PKTINFO,
                ::core::mem::size_of::<in_pktinfo>() as size_t,
            ) as *mut in_pktinfo;
            if !pktinfo.is_null() {
                (*pktinfo).ipi_spec_dst.s_addr = (*(addr_from as *mut sockaddr_in)).sin_addr.s_addr;
                (*pktinfo).ipi_ifindex = dest_if as ::core::ffi::c_ulong as ::core::ffi::c_int;
            } else {
                is_null = 1 as ::core::ffi::c_int;
            }
        } else {
            let mut pktinfo6: *mut in6_pktinfo = cmsg_format_header_return_data_ptr(
                msg,
                &raw mut last_cmsg,
                &raw mut control_length,
                IPPROTO_IPV6 as ::core::ffi::c_int,
                IPV6_PKTINFO,
                ::core::mem::size_of::<in6_pktinfo>() as size_t,
            ) as *mut in6_pktinfo;
            if !pktinfo6.is_null() {
                memcpy(
                    &raw mut (*pktinfo6).ipi6_addr as *mut ::core::ffi::c_void,
                    &raw mut (*(addr_from as *mut sockaddr_in6)).sin6_addr
                        as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<in6_addr>() as size_t,
                );
                (*pktinfo6).ipi6_ifindex = dest_if as ::core::ffi::c_ulong as ::core::ffi::c_uint;
            } else {
                is_null = 1 as ::core::ffi::c_int;
            }
            if is_null == 0 {
                let mut pval: *mut ::core::ffi::c_int = cmsg_format_header_return_data_ptr(
                    msg,
                    &raw mut last_cmsg,
                    &raw mut control_length,
                    SOL_IPV6,
                    IPV6_DONTFRAG,
                    ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
                )
                    as *mut ::core::ffi::c_int;
                if !pval.is_null() {
                    *pval = 1 as ::core::ffi::c_int;
                } else {
                    is_null = 1 as ::core::ffi::c_int;
                }
            }
        }
    }
    if is_null == 0 && send_msg_size > 0 as size_t && send_msg_size < message_length {
        let mut pval_0: *mut uint16_t = cmsg_format_header_return_data_ptr(
            msg,
            &raw mut last_cmsg,
            &raw mut control_length,
            SOL_UDP,
            UDP_SEGMENT,
            ::core::mem::size_of::<uint16_t>() as size_t,
        ) as *mut uint16_t;
        if !pval_0.is_null() {
            *pval_0 = send_msg_size as uint16_t;
        } else {
            is_null = 1 as ::core::ffi::c_int;
        }
    }
    (*msg).msg_controllen = control_length as size_t;
    if control_length == 0 as ::core::ffi::c_int {
        (*msg).msg_control = NULL;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_recvmsg(
    mut fd: ::core::ffi::c_int,
    mut addr_from: *mut sockaddr_storage,
    mut addr_dest: *mut sockaddr_storage,
    mut dest_if: *mut ::core::ffi::c_int,
    mut received_ecn: *mut ::core::ffi::c_uchar,
    mut buffer: *mut uint8_t,
    mut buffer_max: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bytes_recv: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut msg: msghdr = msghdr {
        msg_name: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_namelen: 0,
        msg_iov: ::core::ptr::null_mut::<iovec>(),
        msg_iovlen: 0,
        msg_control: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut dataBuf: iovec = iovec {
        iov_base: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        iov_len: 0,
    };
    let mut cmsg_buffer: [::core::ffi::c_char; 1024] = [0; 1024];
    if !dest_if.is_null() {
        *dest_if = 0 as ::core::ffi::c_int;
    }
    dataBuf.iov_base = buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
    dataBuf.iov_len = buffer_max as size_t;
    msg.msg_name = addr_from as *mut sockaddr as *mut ::core::ffi::c_void;
    msg.msg_namelen = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
    msg.msg_iov = &raw mut dataBuf;
    msg.msg_iovlen = 1 as size_t;
    msg.msg_flags = 0 as ::core::ffi::c_int;
    msg.msg_control = &raw mut cmsg_buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
    msg.msg_controllen = ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize as size_t;
    bytes_recv = recvmsg(fd, &raw mut msg, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if bytes_recv <= 0 as ::core::ffi::c_int {
        (*addr_from).ss_family = 0 as sa_family_t;
    } else {
        picoquic_socks_cmsg_parse(
            &raw mut msg as *mut ::core::ffi::c_void,
            addr_dest,
            dest_if,
            received_ecn,
            ::core::ptr::null_mut::<size_t>(),
        );
    }
    return bytes_recv;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_sendmsg(
    mut fd: ::core::ffi::c_int,
    mut addr_dest: *mut sockaddr,
    mut addr_from: *mut sockaddr,
    mut dest_if: ::core::ffi::c_int,
    mut bytes: *const ::core::ffi::c_char,
    mut length: ::core::ffi::c_int,
    mut send_msg_size: ::core::ffi::c_int,
    mut sock_err: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut msg: msghdr = msghdr {
        msg_name: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_namelen: 0,
        msg_iov: ::core::ptr::null_mut::<iovec>(),
        msg_iovlen: 0,
        msg_control: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut dataBuf: iovec = iovec {
        iov_base: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        iov_len: 0,
    };
    let mut cmsg_buffer: [::core::ffi::c_char; 1024] = [0; 1024];
    let mut bytes_sent: ::core::ffi::c_int = 0;
    if send_msg_size > 0 as ::core::ffi::c_int {
        let segment_count: ::core::ffi::c_int = length / send_msg_size;
        if segment_count > 128 as ::core::ffi::c_int {
            return -(1 as ::core::ffi::c_int);
        }
    }
    dataBuf.iov_base = bytes as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
    dataBuf.iov_len = length as size_t;
    memset(
        &raw mut msg as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<msghdr>() as size_t,
    );
    msg.msg_name = addr_dest as *mut ::core::ffi::c_void;
    msg.msg_namelen = picoquic_addr_length(addr_dest) as socklen_t;
    msg.msg_iov = &raw mut dataBuf;
    msg.msg_iovlen = 1 as size_t;
    msg.msg_control = &raw mut cmsg_buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void;
    msg.msg_controllen = ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as usize as size_t;
    picoquic_socks_cmsg_format(
        &raw mut msg as *mut ::core::ffi::c_void,
        length as size_t,
        send_msg_size as size_t,
        addr_from,
        dest_if,
    );
    bytes_sent = sendmsg(fd, &raw mut msg, 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if bytes_sent <= 0 as ::core::ffi::c_int {
        let mut last_error: ::core::ffi::c_int = *__errno_location();
        if !sock_err.is_null() {
            *sock_err = last_error;
        }
    }
    return bytes_sent;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_select_ex(
    mut sockets: *mut ::core::ffi::c_int,
    mut nb_sockets: ::core::ffi::c_int,
    mut addr_from: *mut sockaddr_storage,
    mut addr_dest: *mut sockaddr_storage,
    mut dest_if: *mut ::core::ffi::c_int,
    mut received_ecn: *mut ::core::ffi::c_uchar,
    mut buffer: *mut uint8_t,
    mut buffer_max: ::core::ffi::c_int,
    mut delta_t: int64_t,
    mut socket_rank: *mut ::core::ffi::c_int,
    mut current_time: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut readfds: fd_set = fd_set {
        __fds_bits: [0; 16],
    };
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ret_select: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bytes_recv: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sockmax: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !received_ecn.is_null() {
        *received_ecn = 0 as ::core::ffi::c_uchar;
    }
    let mut __i: ::core::ffi::c_uint = 0;
    let mut __arr: *mut fd_set = &raw mut readfds;
    __i = 0 as ::core::ffi::c_uint;
    while (__i as usize)
        < (::core::mem::size_of::<fd_set>() as usize)
            .wrapping_div(::core::mem::size_of::<__fd_mask>() as usize)
    {
        (*__arr).__fds_bits[__i as usize] = 0 as __fd_mask;
        __i = __i.wrapping_add(1);
    }
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < nb_sockets {
        if sockmax < *sockets.offset(i as isize) {
            sockmax = *sockets.offset(i as isize);
        }
        readfds.__fds_bits[(*sockets.offset(i as isize) / __NFDBITS) as usize] |=
            ((1 as ::core::ffi::c_ulong) << *sockets.offset(i as isize) % __NFDBITS) as __fd_mask;
        i += 1;
    }
    if delta_t <= 0 as int64_t {
        tv.tv_sec = 0 as __time_t;
        tv.tv_usec = 0 as __suseconds_t;
    } else if delta_t > 10000000 as int64_t {
        tv.tv_sec = 10 as ::core::ffi::c_int as ::core::ffi::c_long as __time_t;
        tv.tv_usec = 0 as __suseconds_t;
    } else {
        tv.tv_sec = (delta_t / 1000000 as int64_t) as ::core::ffi::c_long as __time_t;
        tv.tv_usec = (delta_t % 1000000 as int64_t) as ::core::ffi::c_long as __suseconds_t;
    }
    ret_select = select(
        sockmax + 1 as ::core::ffi::c_int,
        &raw mut readfds,
        ::core::ptr::null_mut::<fd_set>(),
        ::core::ptr::null_mut::<fd_set>(),
        &raw mut tv,
    );
    if ret_select < 0 as ::core::ffi::c_int {
        bytes_recv = -(1 as ::core::ffi::c_int);
    } else if ret_select > 0 as ::core::ffi::c_int {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < nb_sockets {
            if readfds.__fds_bits[(*sockets.offset(i_0 as isize) / __NFDBITS) as usize]
                & ((1 as ::core::ffi::c_ulong) << *sockets.offset(i_0 as isize) % __NFDBITS)
                    as __fd_mask
                != 0 as __fd_mask
            {
                *socket_rank = i_0;
                bytes_recv = picoquic_recvmsg(
                    *sockets.offset(i_0 as isize),
                    addr_from,
                    addr_dest,
                    dest_if,
                    received_ecn,
                    buffer,
                    buffer_max,
                );
                if bytes_recv <= 0 as ::core::ffi::c_int {
                    break;
                } else {
                    break;
                }
            } else {
                i_0 += 1;
            }
        }
    }
    *current_time = picoquic_current_time();
    return bytes_recv;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_select(
    mut sockets: *mut ::core::ffi::c_int,
    mut nb_sockets: ::core::ffi::c_int,
    mut addr_from: *mut sockaddr_storage,
    mut addr_dest: *mut sockaddr_storage,
    mut dest_if: *mut ::core::ffi::c_int,
    mut received_ecn: *mut ::core::ffi::c_uchar,
    mut buffer: *mut uint8_t,
    mut buffer_max: ::core::ffi::c_int,
    mut delta_t: int64_t,
    mut current_time: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut socket_rank: ::core::ffi::c_int = 0;
    return picoquic_select_ex(
        sockets,
        nb_sockets,
        addr_from,
        addr_dest,
        dest_if,
        received_ecn,
        buffer,
        buffer_max,
        delta_t,
        &raw mut socket_rank,
        current_time,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_send_through_socket(
    mut fd: ::core::ffi::c_int,
    mut addr_dest: *mut sockaddr,
    mut addr_from: *mut sockaddr,
    mut from_if: ::core::ffi::c_int,
    mut bytes: *const ::core::ffi::c_char,
    mut length: ::core::ffi::c_int,
    mut sock_err: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut sent: ::core::ffi::c_int = picoquic_sendmsg(
        fd,
        addr_dest,
        addr_from,
        from_if,
        bytes,
        length,
        0 as ::core::ffi::c_int,
        sock_err,
    );
    return sent;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_send_through_server_sockets(
    mut sockets: *mut picoquic_server_sockets_t,
    mut addr_dest: *mut sockaddr,
    mut addr_from: *mut sockaddr,
    mut from_if: ::core::ffi::c_int,
    mut bytes: *const ::core::ffi::c_char,
    mut length: ::core::ffi::c_int,
    mut sock_err: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut socket_index: ::core::ffi::c_int =
        if (*addr_dest).sa_family as ::core::ffi::c_int == AF_INET {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        };
    return picoquic_send_through_socket(
        (*sockets).s_socket[socket_index as usize],
        addr_dest,
        addr_from,
        from_if,
        bytes,
        length,
        sock_err,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_server_address(
    mut ip_address_text: *const ::core::ffi::c_char,
    mut server_port: ::core::ffi::c_int,
    mut server_address: *mut sockaddr_storage,
    mut is_name: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ipv4_dest: *mut sockaddr_in = server_address as *mut sockaddr_in;
    let mut ipv6_dest: *mut sockaddr_in6 = server_address as *mut sockaddr_in6;
    memset(
        server_address as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_storage>() as size_t,
    );
    *is_name = 0 as ::core::ffi::c_int;
    if inet_pton(
        AF_INET,
        ip_address_text,
        &raw mut (*ipv4_dest).sin_addr as *mut ::core::ffi::c_void,
    ) == 1 as ::core::ffi::c_int
    {
        (*ipv4_dest).sin_family = AF_INET as sa_family_t;
        (*ipv4_dest).sin_port = __bswap_16(server_port as __uint16_t) as in_port_t;
    } else if inet_pton(
        AF_INET6,
        ip_address_text,
        &raw mut (*ipv6_dest).sin6_addr as *mut ::core::ffi::c_void,
    ) == 1 as ::core::ffi::c_int
    {
        (*ipv6_dest).sin6_family = AF_INET6 as sa_family_t;
        (*ipv6_dest).sin6_port = __bswap_16(server_port as __uint16_t) as in_port_t;
    } else {
        let mut result: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
        let mut hints: addrinfo = addrinfo {
            ai_flags: 0,
            ai_family: 0,
            ai_socktype: 0,
            ai_protocol: 0,
            ai_addrlen: 0,
            ai_addr: ::core::ptr::null_mut::<sockaddr>(),
            ai_canonname: ::core::ptr::null_mut::<::core::ffi::c_char>(),
            ai_next: ::core::ptr::null_mut::<addrinfo>(),
        };
        memset(
            &raw mut hints as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<addrinfo>() as size_t,
        );
        hints.ai_family = AF_UNSPEC;
        hints.ai_socktype = SOCK_DGRAM as ::core::ffi::c_int;
        hints.ai_protocol = IPPROTO_UDP as ::core::ffi::c_int;
        ret = getaddrinfo(
            ip_address_text,
            ::core::ptr::null::<::core::ffi::c_char>(),
            &raw mut hints,
            &raw mut result,
        );
        if ret != 0 as ::core::ffi::c_int {
            let mut err: ::core::ffi::c_int = ret;
            fprintf(
                stderr,
                b"Cannot get IP address for %s, err = %d (0x%x)\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                ip_address_text,
                err,
                err,
            );
            ret = -(1 as ::core::ffi::c_int);
        } else {
            *is_name = 1 as ::core::ffi::c_int;
            match (*result).ai_family {
                AF_INET => {
                    (*ipv4_dest).sin_family = AF_INET as sa_family_t;
                    (*ipv4_dest).sin_port = __bswap_16(server_port as __uint16_t) as in_port_t;
                    (*ipv4_dest).sin_addr.s_addr =
                        (*((*result).ai_addr as *mut sockaddr_in)).sin_addr.s_addr;
                }
                AF_INET6 => {
                    (*ipv6_dest).sin6_family = AF_INET6 as sa_family_t;
                    (*ipv6_dest).sin6_port = __bswap_16(server_port as __uint16_t) as in_port_t;
                    memcpy(
                        &raw mut (*ipv6_dest).sin6_addr as *mut ::core::ffi::c_void,
                        &raw mut (*((*result).ai_addr as *mut sockaddr_in6)).sin6_addr
                            as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<in6_addr>() as size_t,
                    );
                }
                _ => {
                    fprintf(
                        stderr,
                        b"Error getting IPv6 address for %s, family = %d\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ip_address_text,
                        (*result).ai_family,
                    );
                    ret = -(1 as ::core::ffi::c_int);
                }
            }
            freeaddrinfo(result);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_key_log_file_from_env(mut quic: *mut picoquic_quic_t) {
    if picoquic_is_sslkeylog_enabled(quic) != 0 {
        let mut keylog_filename: *mut ::core::ffi::c_char =
            ::core::ptr::null_mut::<::core::ffi::c_char>();
        keylog_filename = getenv(b"SSLKEYLOGFILE\0".as_ptr() as *const ::core::ffi::c_char);
        if keylog_filename.is_null() {
            return;
        }
        picoquic_set_key_log_file(quic, keylog_filename);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_socket_error_implies_unreachable(
    mut sock_err: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    static mut unreachable_errors: [::core::ffi::c_int; 6] = [
        EAFNOSUPPORT,
        ECONNRESET,
        EHOSTUNREACH,
        ENETDOWN,
        ENETUNREACH,
        -(1 as ::core::ffi::c_int),
    ];
    let mut nb_errors: size_t = (::core::mem::size_of::<[::core::ffi::c_int; 6]>() as size_t)
        .wrapping_div(::core::mem::size_of::<::core::ffi::c_int>() as size_t);
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0 as size_t;
    while ret == 0 as ::core::ffi::c_int && i < nb_errors {
        ret = (sock_err == unreachable_errors[i as usize]) as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
    return ret;
}

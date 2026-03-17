use ::c2rust_bitfields;
extern "C" {
    pub type sockaddr_x25;
    pub type sockaddr_un;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type evp_md_st;
    pub type evp_pkey_st;
    pub type x509_st;
    pub type x509_store_st;
    pub type x509_lookup_st;
    pub type x509_lookup_method_st;
    pub type ossl_init_settings_st;
    pub type stack_st_X509;
    pub type st_ptls_t;
    pub type st_ptls_key_schedule_t;
    pub type st_ptls_traffic_protection_t;
    fn select(
        __nfds: ::core::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::core::ffi::c_int;
    fn socket(
        __domain: ::core::ffi::c_int,
        __type: ::core::ffi::c_int,
        __protocol: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn bind(
        __fd: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    fn connect(
        __fd: ::core::ffi::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> ::core::ffi::c_int;
    fn setsockopt(
        __fd: ::core::ffi::c_int,
        __level: ::core::ffi::c_int,
        __optname: ::core::ffi::c_int,
        __optval: *const ::core::ffi::c_void,
        __optlen: socklen_t,
    ) -> ::core::ffi::c_int;
    fn listen(__fd: ::core::ffi::c_int, __n: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn accept(
        __fd: ::core::ffi::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn shutdown(__fd: ::core::ffi::c_int, __how: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn write(__fd: ::core::ffi::c_int, __buf: *const ::core::ffi::c_void, __n: size_t) -> ssize_t;
    static mut optarg: *mut ::core::ffi::c_char;
    static mut optind: ::core::ffi::c_int;
    fn getopt(
        ___argc: ::core::ffi::c_int,
        ___argv: *const *mut ::core::ffi::c_char,
        __shortopts: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn fread(
        __ptr: *mut ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn feof(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn perror(__s: *const ::core::ffi::c_char);
    fn __res_init() -> ::core::ffi::c_int;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn fcntl(__fd: ::core::ffi::c_int, __cmd: ::core::ffi::c_int, ...) -> ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memmove(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn strcasecmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn exit(__status: ::core::ffi::c_int) -> !;
    fn OPENSSL_init_crypto(
        opts: uint64_t,
        settings: *const OPENSSL_INIT_SETTINGS,
    ) -> ::core::ffi::c_int;
    fn EVP_PKEY_free(pkey: *mut EVP_PKEY);
    fn X509_STORE_new() -> *mut X509_STORE;
    fn X509_STORE_free(v: *mut X509_STORE);
    fn X509_STORE_add_lookup(v: *mut X509_STORE, m: *mut X509_LOOKUP_METHOD) -> *mut X509_LOOKUP;
    fn X509_LOOKUP_file() -> *mut X509_LOOKUP_METHOD;
    fn X509_LOOKUP_ctrl(
        ctx: *mut X509_LOOKUP,
        cmd: ::core::ffi::c_int,
        argc: *const ::core::ffi::c_char,
        argl: ::core::ffi::c_long,
        ret: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn d2i_PUBKEY(
        a: *mut *mut EVP_PKEY,
        in_0: *mut *const ::core::ffi::c_uchar,
        len: ::core::ffi::c_long,
    ) -> *mut EVP_PKEY;
    fn PEM_read_PrivateKey(
        out: *mut FILE,
        x: *mut *mut EVP_PKEY,
        cb: Option<pem_password_cb>,
        u: *mut ::core::ffi::c_void,
    ) -> *mut EVP_PKEY;
    fn ENGINE_load_builtin_engines();
    fn ENGINE_register_all_ciphers();
    fn ENGINE_register_all_digests();
    fn ptls_buffer__release_memory(buf: *mut ptls_buffer_t);
    fn ptls_buffer_reserve(buf: *mut ptls_buffer_t, delta: size_t) -> ::core::ffi::c_int;
    fn ptls_decode16(
        value: *mut uint16_t,
        src: *mut *const uint8_t,
        end: *const uint8_t,
    ) -> ::core::ffi::c_int;
    fn ptls_decode_quicint(src: *mut *const uint8_t, end: *const uint8_t) -> uint64_t;
    fn ptls_log_add_fd(fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ptls_client_new(ctx: *mut ptls_context_t) -> *mut ptls_t;
    fn ptls_server_new(ctx: *mut ptls_context_t) -> *mut ptls_t;
    fn ptls_free(tls: *mut ptls_t);
    fn ptls_get_context(tls: *mut ptls_t) -> *mut ptls_context_t;
    fn ptls_get_client_random(tls: *mut ptls_t) -> ptls_iovec_t;
    fn ptls_get_cipher(tls: *mut ptls_t) -> *const ptls_cipher_suite_t;
    fn ptls_set_server_name(
        tls: *mut ptls_t,
        server_name: *const ::core::ffi::c_char,
        server_name_len: size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_handshake(
        tls: *mut ptls_t,
        sendbuf: *mut ptls_buffer_t,
        input: *const ::core::ffi::c_void,
        inlen: *mut size_t,
        args: *mut ptls_handshake_properties_t,
    ) -> ::core::ffi::c_int;
    fn ptls_receive(
        tls: *mut ptls_t,
        plaintextbuf: *mut ptls_buffer_t,
        input: *const ::core::ffi::c_void,
        len: *mut size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_send(
        tls: *mut ptls_t,
        sendbuf: *mut ptls_buffer_t,
        input: *const ::core::ffi::c_void,
        inlen: size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_update_key(tls: *mut ptls_t, request_update: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ptls_is_server(tls: *mut ptls_t) -> ::core::ffi::c_int;
    fn ptls_send_alert(
        tls: *mut ptls_t,
        sendbuf: *mut ptls_buffer_t,
        level: uint8_t,
        description: uint8_t,
    ) -> ::core::ffi::c_int;
    fn ptls_load_certificates(
        ctx: *mut ptls_context_t,
        cert_pem_file: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn ptls_hpke_setup_base_r(
        kem: *const ptls_hpke_kem_t,
        cipher: *const ptls_hpke_cipher_suite_t,
        keyex: *mut ptls_key_exchange_context_t,
        ctx: *mut *mut ptls_aead_context_t,
        pk_s: ptls_iovec_t,
        info: ptls_iovec_t,
    ) -> ::core::ffi::c_int;
    fn ptls_hexdump(
        dst: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_void,
        len: size_t,
    ) -> *mut ::core::ffi::c_char;
    static mut ptls_get_time: ptls_get_time_t;
    static ptls_openssl_secp256r1: ptls_key_exchange_algorithm_t;
    static ptls_openssl_secp384r1: ptls_key_exchange_algorithm_t;
    static ptls_openssl_secp521r1: ptls_key_exchange_algorithm_t;
    static ptls_openssl_x25519: ptls_key_exchange_algorithm_t;
    static mut ptls_openssl_cipher_suites: [*const ptls_cipher_suite_t; 0];
    static mut ptls_openssl_cipher_suites_all: [*const ptls_cipher_suite_t; 0];
    static mut ptls_openssl_hpke_kems: [*const ptls_hpke_kem_t; 0];
    static mut ptls_openssl_hpke_cipher_suites: [*const ptls_hpke_cipher_suite_t; 0];
    fn ptls_openssl_random_bytes(buf: *mut ::core::ffi::c_void, len: size_t);
    fn ptls_openssl_create_key_exchange(
        ctx: *mut *mut ptls_key_exchange_context_t,
        pkey: *mut EVP_PKEY,
    ) -> ::core::ffi::c_int;
    fn ptls_openssl_init_sign_certificate(
        self_0: *mut ptls_openssl_sign_certificate_t,
        key: *mut EVP_PKEY,
    ) -> ::core::ffi::c_int;
    fn ptls_openssl_init_verify_certificate(
        self_0: *mut ptls_openssl_verify_certificate_t,
        store: *mut X509_STORE,
    ) -> ::core::ffi::c_int;
    fn ptls_openssl_raw_pubkey_init_verify_certificate(
        self_0: *mut ptls_openssl_raw_pubkey_verify_certificate_t,
        pubkey: *mut EVP_PKEY,
    ) -> ::core::ffi::c_int;
    static mut ptls_decompress_certificate: ptls_decompress_certificate_t;
    fn ptls_init_compressed_certificate(
        ecc: *mut ptls_emit_compressed_certificate_t,
        certificates: *mut ptls_iovec_t,
        num_certificates: size_t,
        ocsp_status: ptls_iovec_t,
    ) -> ::core::ffi::c_int;
    fn getaddrinfo(
        __name: *const ::core::ffi::c_char,
        __service: *const ::core::ffi::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> ::core::ffi::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
    fn ptls_load_pem_objects(
        pem_fname: *const ::core::ffi::c_char,
        label: *const ::core::ffi::c_char,
        list: *mut ptls_iovec_t,
        list_max: size_t,
        nb_objects: *mut size_t,
    ) -> ::core::ffi::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __intptr_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type ssize_t = isize;
pub type size_t = usize;
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
    pub fds_bits: [__fd_mask; 16],
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const SHUT_RDWR: C2Rust_Unnamed = 2;
pub const SHUT_WR: C2Rust_Unnamed = 1;
pub const SHUT_RD: C2Rust_Unnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
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
pub type in_port_t = uint16_t;
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
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
pub type C2Rust_Unnamed_1 = ::core::ffi::c_uint;
pub const IPPROTO_MAX: C2Rust_Unnamed_1 = 263;
pub const IPPROTO_MPTCP: C2Rust_Unnamed_1 = 262;
pub const IPPROTO_RAW: C2Rust_Unnamed_1 = 255;
pub const IPPROTO_ETHERNET: C2Rust_Unnamed_1 = 143;
pub const IPPROTO_MPLS: C2Rust_Unnamed_1 = 137;
pub const IPPROTO_UDPLITE: C2Rust_Unnamed_1 = 136;
pub const IPPROTO_SCTP: C2Rust_Unnamed_1 = 132;
pub const IPPROTO_L2TP: C2Rust_Unnamed_1 = 115;
pub const IPPROTO_COMP: C2Rust_Unnamed_1 = 108;
pub const IPPROTO_PIM: C2Rust_Unnamed_1 = 103;
pub const IPPROTO_ENCAP: C2Rust_Unnamed_1 = 98;
pub const IPPROTO_BEETPH: C2Rust_Unnamed_1 = 94;
pub const IPPROTO_MTP: C2Rust_Unnamed_1 = 92;
pub const IPPROTO_AH: C2Rust_Unnamed_1 = 51;
pub const IPPROTO_ESP: C2Rust_Unnamed_1 = 50;
pub const IPPROTO_GRE: C2Rust_Unnamed_1 = 47;
pub const IPPROTO_RSVP: C2Rust_Unnamed_1 = 46;
pub const IPPROTO_IPV6: C2Rust_Unnamed_1 = 41;
pub const IPPROTO_DCCP: C2Rust_Unnamed_1 = 33;
pub const IPPROTO_TP: C2Rust_Unnamed_1 = 29;
pub const IPPROTO_IDP: C2Rust_Unnamed_1 = 22;
pub const IPPROTO_UDP: C2Rust_Unnamed_1 = 17;
pub const IPPROTO_PUP: C2Rust_Unnamed_1 = 12;
pub const IPPROTO_EGP: C2Rust_Unnamed_1 = 8;
pub const IPPROTO_TCP: C2Rust_Unnamed_1 = 6;
pub const IPPROTO_IPIP: C2Rust_Unnamed_1 = 4;
pub const IPPROTO_IGMP: C2Rust_Unnamed_1 = 2;
pub const IPPROTO_ICMP: C2Rust_Unnamed_1 = 1;
pub const IPPROTO_IP: C2Rust_Unnamed_1 = 0;
pub type intptr_t = __intptr_t;
pub type __gnuc_va_list = __builtin_va_list;
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
pub type va_list = __gnuc_va_list;
pub type EVP_MD = evp_md_st;
pub type EVP_PKEY = evp_pkey_st;
pub type X509 = x509_st;
pub type X509_STORE = x509_store_st;
pub type X509_LOOKUP = x509_lookup_st;
pub type X509_LOOKUP_METHOD = x509_lookup_method_st;
pub type OPENSSL_INIT_SETTINGS = ossl_init_settings_st;
pub type pem_password_cb = unsafe extern "C" fn(
    *mut ::core::ffi::c_char,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;
pub type ptls_t = st_ptls_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_ptls_context_t {
    pub random_bytes: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>,
    pub get_time: *mut ptls_get_time_t,
    pub key_exchanges: *mut *const ptls_key_exchange_algorithm_t,
    pub cipher_suites: *mut *const ptls_cipher_suite_t,
    pub certificates: C2Rust_Unnamed_14,
    pub pre_shared_key: C2Rust_Unnamed_13,
    pub ech: C2Rust_Unnamed_10,
    pub on_client_hello: *mut ptls_on_client_hello_t,
    pub emit_certificate: *mut ptls_emit_certificate_t,
    pub sign_certificate: *mut ptls_sign_certificate_t,
    pub verify_certificate: *mut ptls_verify_certificate_t,
    pub ticket_lifetime: uint32_t,
    pub max_early_data_size: uint32_t,
    pub max_buffer_size: size_t,
    pub hkdf_label_prefix__obsolete: *const ::core::ffi::c_char,
    #[bitfield(
        name = "require_dhe_on_psk",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    #[bitfield(name = "use_exporter", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(
        name = "send_change_cipher_spec",
        ty = "::core::ffi::c_uint",
        bits = "2..=2"
    )]
    #[bitfield(
        name = "require_client_authentication",
        ty = "::core::ffi::c_uint",
        bits = "3..=3"
    )]
    #[bitfield(
        name = "omit_end_of_early_data",
        ty = "::core::ffi::c_uint",
        bits = "4..=4"
    )]
    #[bitfield(
        name = "use_raw_public_keys",
        ty = "::core::ffi::c_uint",
        bits = "5..=5"
    )]
    #[bitfield(
        name = "server_cipher_preference",
        ty = "::core::ffi::c_uint",
        bits = "6..=6"
    )]
    #[bitfield(
        name = "server_cipher_chacha_priority",
        ty = "::core::ffi::c_uint",
        bits = "7..=7"
    )]
    pub require_dhe_on_psk_use_exporter_send_change_cipher_spec_require_client_authentication_omit_end_of_early_data_use_raw_public_keys_server_cipher_preference_server_cipher_chacha_priority:
        [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub encrypt_ticket: *mut ptls_encrypt_ticket_t,
    pub save_ticket: *mut ptls_save_ticket_t,
    pub log_event: *mut ptls_log_event_t,
    pub update_open_count: *mut ptls_update_open_count_t,
    pub update_traffic_key: *mut ptls_update_traffic_key_t,
    pub decompress_certificate: *mut ptls_decompress_certificate_t,
    pub on_extension: *mut ptls_on_extension_t,
    pub tls12_cipher_suites: *mut *const ptls_cipher_suite_t,
    pub ticket_context: C2Rust_Unnamed_3,
    pub client_ca_names: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub list: *const ptls_iovec_t,
    pub count: size_t,
}
pub type ptls_iovec_t = st_ptls_iovec_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_iovec_t {
    pub base: *mut uint8_t,
    pub len: size_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub bytes: [uint8_t; 32],
    #[bitfield(name = "is_set", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub is_set: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type ptls_cipher_suite_t = st_ptls_cipher_suite_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_cipher_suite_t {
    pub id: uint16_t,
    pub aead: *const ptls_aead_algorithm_t,
    pub hash: *const ptls_hash_algorithm_t,
    pub name: *const ::core::ffi::c_char,
}
pub type ptls_hash_algorithm_t = st_ptls_hash_algorithm_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_hash_algorithm_t {
    pub name: *const ::core::ffi::c_char,
    pub block_size: size_t,
    pub digest_size: size_t,
    pub create: Option<unsafe extern "C" fn() -> *mut ptls_hash_context_t>,
    pub empty_digest: [uint8_t; 64],
}
pub type ptls_hash_context_t = st_ptls_hash_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_hash_context_t {
    pub update: Option<
        unsafe extern "C" fn(*mut st_ptls_hash_context_t, *const ::core::ffi::c_void, size_t) -> (),
    >,
    pub final_0: Option<
        unsafe extern "C" fn(
            *mut st_ptls_hash_context_t,
            *mut ::core::ffi::c_void,
            ptls_hash_final_mode_t,
        ) -> (),
    >,
    pub clone_:
        Option<unsafe extern "C" fn(*mut st_ptls_hash_context_t) -> *mut st_ptls_hash_context_t>,
}
pub type ptls_hash_final_mode_t = en_ptls_hash_final_mode_t;
pub type en_ptls_hash_final_mode_t = ::core::ffi::c_uint;
pub const PTLS_HASH_FINAL_MODE_SNAPSHOT: en_ptls_hash_final_mode_t = 2;
pub const PTLS_HASH_FINAL_MODE_RESET: en_ptls_hash_final_mode_t = 1;
pub const PTLS_HASH_FINAL_MODE_FREE: en_ptls_hash_final_mode_t = 0;
pub type ptls_aead_algorithm_t = st_ptls_aead_algorithm_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_ptls_aead_algorithm_t {
    pub name: *const ::core::ffi::c_char,
    pub confidentiality_limit: uint64_t,
    pub integrity_limit: uint64_t,
    pub ctr_cipher: *const ptls_cipher_algorithm_t,
    pub ecb_cipher: *const ptls_cipher_algorithm_t,
    pub key_size: size_t,
    pub iv_size: size_t,
    pub tag_size: size_t,
    pub tls12: C2Rust_Unnamed_4,
    #[bitfield(name = "non_temporal", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub non_temporal: [u8; 1],
    pub align_bits: uint8_t,
    pub context_size: size_t,
    pub setup_crypto: Option<
        unsafe extern "C" fn(
            *mut ptls_aead_context_t,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_aead_context_t = st_ptls_aead_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_aead_context_t {
    pub algo: *const st_ptls_aead_algorithm_t,
    pub dispose_crypto: Option<unsafe extern "C" fn(*mut st_ptls_aead_context_t) -> ()>,
    pub do_get_iv:
        Option<unsafe extern "C" fn(*mut st_ptls_aead_context_t, *mut ::core::ffi::c_void) -> ()>,
    pub do_set_iv:
        Option<unsafe extern "C" fn(*mut st_ptls_aead_context_t, *const ::core::ffi::c_void) -> ()>,
    pub do_encrypt_init: Option<
        unsafe extern "C" fn(
            *mut st_ptls_aead_context_t,
            uint64_t,
            *const ::core::ffi::c_void,
            size_t,
        ) -> (),
    >,
    pub do_encrypt_update: Option<
        unsafe extern "C" fn(
            *mut st_ptls_aead_context_t,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            size_t,
        ) -> size_t,
    >,
    pub do_encrypt_final: Option<
        unsafe extern "C" fn(*mut st_ptls_aead_context_t, *mut ::core::ffi::c_void) -> size_t,
    >,
    pub do_encrypt: Option<
        unsafe extern "C" fn(
            *mut st_ptls_aead_context_t,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            size_t,
            uint64_t,
            *const ::core::ffi::c_void,
            size_t,
            *mut ptls_aead_supplementary_encryption_t,
        ) -> (),
    >,
    pub do_encrypt_v: Option<
        unsafe extern "C" fn(
            *mut st_ptls_aead_context_t,
            *mut ::core::ffi::c_void,
            *mut ptls_iovec_t,
            size_t,
            uint64_t,
            *const ::core::ffi::c_void,
            size_t,
        ) -> (),
    >,
    pub do_decrypt: Option<
        unsafe extern "C" fn(
            *mut st_ptls_aead_context_t,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            size_t,
            uint64_t,
            *const ::core::ffi::c_void,
            size_t,
        ) -> size_t,
    >,
}
pub type ptls_aead_supplementary_encryption_t = st_ptls_aead_supplementary_encryption_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_aead_supplementary_encryption_t {
    pub ctx: *mut ptls_cipher_context_t,
    pub input: *const ::core::ffi::c_void,
    pub output: [uint8_t; 16],
}
pub type ptls_cipher_context_t = st_ptls_cipher_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_cipher_context_t {
    pub algo: *const st_ptls_cipher_algorithm_t,
    pub do_dispose: Option<unsafe extern "C" fn(*mut st_ptls_cipher_context_t) -> ()>,
    pub do_init: Option<
        unsafe extern "C" fn(*mut st_ptls_cipher_context_t, *const ::core::ffi::c_void) -> (),
    >,
    pub do_transform: Option<
        unsafe extern "C" fn(
            *mut st_ptls_cipher_context_t,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            size_t,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_cipher_algorithm_t {
    pub name: *const ::core::ffi::c_char,
    pub key_size: size_t,
    pub block_size: size_t,
    pub iv_size: size_t,
    pub context_size: size_t,
    pub setup_crypto: Option<
        unsafe extern "C" fn(
            *mut ptls_cipher_context_t,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub fixed_iv_size: size_t,
    pub record_iv_size: size_t,
}
pub type ptls_cipher_algorithm_t = st_ptls_cipher_algorithm_t;
pub type ptls_on_extension_t = st_ptls_on_extension_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_on_extension_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_on_extension_t,
            *mut ptls_t,
            uint8_t,
            uint16_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_decompress_certificate_t = st_ptls_decompress_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_decompress_certificate_t {
    pub supported_algorithms: *const uint16_t,
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_decompress_certificate_t,
            *mut ptls_t,
            uint16_t,
            ptls_iovec_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_update_traffic_key_t = st_ptls_update_traffic_key_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_update_traffic_key_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_update_traffic_key_t,
            *mut ptls_t,
            ::core::ffi::c_int,
            size_t,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_update_open_count_t = st_ptls_update_open_count_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_update_open_count_t {
    pub cb: Option<unsafe extern "C" fn(*mut st_ptls_update_open_count_t, ssize_t) -> ()>,
}
pub type ptls_log_event_t = st_ptls_log_event_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_log_event_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_log_event_t,
            *mut ptls_t,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            ...
        ) -> (),
    >,
}
pub type ptls_save_ticket_t = st_ptls_save_ticket_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_save_ticket_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_save_ticket_t,
            *mut ptls_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_encrypt_ticket_t = st_ptls_encrypt_ticket_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_encrypt_ticket_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_encrypt_ticket_t,
            *mut ptls_t,
            ::core::ffi::c_int,
            *mut ptls_buffer_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_buffer_t = st_ptls_buffer_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_buffer_t {
    pub base: *mut uint8_t,
    pub capacity: size_t,
    pub off: size_t,
    pub is_allocated: uint8_t,
    pub align_bits: uint8_t,
}
pub type ptls_verify_certificate_t = st_ptls_verify_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_verify_certificate_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_verify_certificate_t,
            *mut ptls_t,
            *const ::core::ffi::c_char,
            *mut Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    uint16_t,
                    ptls_iovec_t,
                    ptls_iovec_t,
                ) -> ::core::ffi::c_int,
            >,
            *mut *mut ::core::ffi::c_void,
            *mut ptls_iovec_t,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub algos: *const uint16_t,
}
pub type ptls_sign_certificate_t = st_ptls_sign_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_sign_certificate_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_sign_certificate_t,
            *mut ptls_t,
            *mut *mut ptls_async_job_t,
            *mut uint16_t,
            *mut ptls_buffer_t,
            ptls_iovec_t,
            *const uint16_t,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_async_job_t = st_ptls_async_job_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_async_job_t {
    pub destroy_: Option<unsafe extern "C" fn(*mut st_ptls_async_job_t) -> ()>,
    pub get_fd: Option<unsafe extern "C" fn(*mut st_ptls_async_job_t) -> ::core::ffi::c_int>,
    pub set_completion_callback: Option<
        unsafe extern "C" fn(
            *mut st_ptls_async_job_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
}
pub type ptls_emit_certificate_t = st_ptls_emit_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_emit_certificate_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_emit_certificate_t,
            *mut ptls_t,
            *mut ptls_message_emitter_t,
            *mut ptls_key_schedule_t,
            ptls_iovec_t,
            ::core::ffi::c_int,
            *const uint16_t,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_key_schedule_t = st_ptls_key_schedule_t;
pub type ptls_message_emitter_t = st_ptls_message_emitter_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_message_emitter_t {
    pub buf: *mut ptls_buffer_t,
    pub enc: *mut st_ptls_traffic_protection_t,
    pub record_header_length: size_t,
    pub begin_message:
        Option<unsafe extern "C" fn(*mut st_ptls_message_emitter_t) -> ::core::ffi::c_int>,
    pub commit_message:
        Option<unsafe extern "C" fn(*mut st_ptls_message_emitter_t) -> ::core::ffi::c_int>,
}
pub type ptls_on_client_hello_t = st_ptls_on_client_hello_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_on_client_hello_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_on_client_hello_t,
            *mut ptls_t,
            *mut ptls_on_client_hello_parameters_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_on_client_hello_parameters_t = st_ptls_on_client_hello_parameters_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_ptls_on_client_hello_parameters_t {
    pub server_name: ptls_iovec_t,
    pub raw_message: ptls_iovec_t,
    pub cipher_suites: ptls_iovec_t,
    pub negotiated_protocols: C2Rust_Unnamed_9,
    pub signature_algorithms: C2Rust_Unnamed_8,
    pub certificate_compression_algorithms: C2Rust_Unnamed_7,
    pub server_certificate_types: C2Rust_Unnamed_6,
    pub psk_identities: C2Rust_Unnamed_5,
    #[bitfield(
        name = "incompatible_version",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    pub incompatible_version: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub list: *const ptls_client_hello_psk_identity_t,
    pub count: size_t,
}
pub type ptls_client_hello_psk_identity_t = st_ptls_client_hello_psk_identity_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_client_hello_psk_identity_t {
    pub identity: ptls_iovec_t,
    pub obfuscated_ticket_age: uint32_t,
    pub binder: ptls_iovec_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub list: *const uint8_t,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub list: *const uint16_t,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub list: *const uint16_t,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
    pub list: *mut ptls_iovec_t,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub client: C2Rust_Unnamed_12,
    pub server: C2Rust_Unnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub create_opener: *mut ptls_ech_create_opener_t,
    pub retry_configs: ptls_iovec_t,
}
pub type ptls_ech_create_opener_t = st_ptls_ech_create_opener_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_ech_create_opener_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_ech_create_opener_t,
            *mut *const ptls_hpke_kem_t,
            *mut *const ptls_hpke_cipher_suite_t,
            *mut ptls_t,
            uint8_t,
            ptls_hpke_cipher_suite_id_t,
            ptls_iovec_t,
            ptls_iovec_t,
        ) -> *mut ptls_aead_context_t,
    >,
}
pub type ptls_hpke_cipher_suite_id_t = st_ptls_hpke_cipher_suite_id_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_hpke_cipher_suite_id_t {
    pub kdf: uint16_t,
    pub aead: uint16_t,
}
pub type ptls_hpke_cipher_suite_t = st_ptls_hpke_cipher_suite_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_hpke_cipher_suite_t {
    pub id: ptls_hpke_cipher_suite_id_t,
    pub name: *const ::core::ffi::c_char,
    pub hash: *const ptls_hash_algorithm_t,
    pub aead: *const ptls_aead_algorithm_t,
}
pub type ptls_hpke_kem_t = st_ptls_hpke_kem_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_hpke_kem_t {
    pub id: uint16_t,
    pub keyex: *const ptls_key_exchange_algorithm_t,
    pub hash: *const ptls_hash_algorithm_t,
}
pub type ptls_key_exchange_algorithm_t = st_ptls_key_exchange_algorithm_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_key_exchange_algorithm_t {
    pub id: uint16_t,
    pub create: Option<
        unsafe extern "C" fn(
            *const st_ptls_key_exchange_algorithm_t,
            *mut *mut ptls_key_exchange_context_t,
        ) -> ::core::ffi::c_int,
    >,
    pub exchange: Option<
        unsafe extern "C" fn(
            *const st_ptls_key_exchange_algorithm_t,
            *mut ptls_iovec_t,
            *mut ptls_iovec_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
    pub data: intptr_t,
    pub name: *const ::core::ffi::c_char,
}
pub type ptls_key_exchange_context_t = st_ptls_key_exchange_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_key_exchange_context_t {
    pub algo: *const st_ptls_key_exchange_algorithm_t,
    pub pubkey: ptls_iovec_t,
    pub on_exchange: Option<
        unsafe extern "C" fn(
            *mut *mut st_ptls_key_exchange_context_t,
            ::core::ffi::c_int,
            *mut ptls_iovec_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
    pub ciphers: *mut *const ptls_hpke_cipher_suite_t,
    pub kems: *mut *const ptls_hpke_kem_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub identity: ptls_iovec_t,
    pub secret: ptls_iovec_t,
    pub hash: *const ptls_hash_algorithm_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub list: *mut ptls_iovec_t,
    pub count: size_t,
}
pub type ptls_get_time_t = st_ptls_get_time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_get_time_t {
    pub cb: Option<unsafe extern "C" fn(*mut st_ptls_get_time_t) -> uint64_t>,
}
pub type ptls_context_t = st_ptls_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_raw_extension_t {
    pub type_0: uint16_t,
    pub data: ptls_iovec_t,
}
pub type ptls_raw_extension_t = st_ptls_raw_extension_t;
pub type en_ptls_early_data_acceptance_t = ::core::ffi::c_uint;
pub const PTLS_EARLY_DATA_ACCEPTED: en_ptls_early_data_acceptance_t = 2;
pub const PTLS_EARLY_DATA_REJECTED: en_ptls_early_data_acceptance_t = 1;
pub const PTLS_EARLY_DATA_ACCEPTANCE_UNKNOWN: en_ptls_early_data_acceptance_t = 0;
pub type ptls_early_data_acceptance_t = en_ptls_early_data_acceptance_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_handshake_properties_t {
    pub c2rust_unnamed: C2Rust_Unnamed_15,
    pub additional_extensions: *mut ptls_raw_extension_t,
    pub collect_extension: Option<
        unsafe extern "C" fn(
            *mut ptls_t,
            *mut st_ptls_handshake_properties_t,
            uint16_t,
        ) -> ::core::ffi::c_int,
    >,
    pub collected_extensions: Option<
        unsafe extern "C" fn(
            *mut ptls_t,
            *mut st_ptls_handshake_properties_t,
            *mut ptls_raw_extension_t,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_15 {
    pub client: C2Rust_Unnamed_19,
    pub server: C2Rust_Unnamed_16,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub selected_psk_binder: C2Rust_Unnamed_18,
    pub cookie: C2Rust_Unnamed_17,
    #[bitfield(name = "enforce_retry", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "retry_uses_cookie", ty = "::core::ffi::c_uint", bits = "1..=1")]
    pub enforce_retry_retry_uses_cookie: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub key: *const ::core::ffi::c_void,
    pub additional_data: ptls_iovec_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub base: [uint8_t; 64],
    pub len: size_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub negotiated_protocols: C2Rust_Unnamed_21,
    pub session_ticket: ptls_iovec_t,
    pub max_early_data_size: *mut size_t,
    pub early_data_acceptance: ptls_early_data_acceptance_t,
    #[bitfield(
        name = "negotiate_before_key_exchange",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    pub negotiate_before_key_exchange: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub ech: C2Rust_Unnamed_20,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub configs: ptls_iovec_t,
    pub retry_configs: *mut ptls_iovec_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub list: *const ptls_iovec_t,
    pub count: size_t,
}
pub type ptls_handshake_properties_t = st_ptls_handshake_properties_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_openssl_signature_scheme_t {
    pub scheme_id: uint16_t,
    pub scheme_md: Option<unsafe extern "C" fn() -> *const EVP_MD>,
}
pub type ptls_openssl_signature_scheme_t = st_ptls_openssl_signature_scheme_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_ptls_openssl_sign_certificate_t {
    pub super_0: ptls_sign_certificate_t,
    pub key: *mut EVP_PKEY,
    pub schemes: *const ptls_openssl_signature_scheme_t,
    #[bitfield(name = "async_0", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub async_0: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type ptls_openssl_sign_certificate_t = st_ptls_openssl_sign_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_openssl_raw_pubkey_verify_certificate_t {
    pub super_0: ptls_verify_certificate_t,
    pub expected_pubkey: *mut EVP_PKEY,
}
pub type ptls_openssl_raw_pubkey_verify_certificate_t =
    st_ptls_openssl_raw_pubkey_verify_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_openssl_override_verify_certificate_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_openssl_override_verify_certificate_t,
            *mut ptls_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut X509,
            *mut stack_st_X509,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_openssl_override_verify_certificate_t = st_ptls_openssl_override_verify_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_openssl_verify_certificate_t {
    pub super_0: ptls_verify_certificate_t,
    pub cert_store: *mut X509_STORE,
    pub override_callback: *mut ptls_openssl_override_verify_certificate_t,
}
pub type ptls_openssl_verify_certificate_t = st_ptls_openssl_verify_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_emit_compressed_certificate_t {
    pub super_0: ptls_emit_certificate_t,
    pub algo: uint16_t,
    pub with_ocsp_status: st_ptls_compressed_certificate_entry_t,
    pub without_ocsp_status: st_ptls_compressed_certificate_entry_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_compressed_certificate_entry_t {
    pub uncompressed_length: uint32_t,
    pub bytes: ptls_iovec_t,
}
pub type ptls_emit_compressed_certificate_t = st_ptls_emit_compressed_certificate_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_util_save_ticket_t {
    pub super_0: ptls_save_ticket_t,
    pub fn_0: [::core::ffi::c_char; 4096],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_util_log_event_t {
    pub super_0: ptls_log_event_t,
    pub fp: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_util_session_cache_t {
    pub super_0: ptls_encrypt_ticket_t,
    pub id: [uint8_t; 32],
    pub data: ptls_iovec_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub config_list: ptls_iovec_t,
    pub keyex: C2Rust_Unnamed_24,
    pub retry: C2Rust_Unnamed_23,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_23 {
    pub configs: ptls_iovec_t,
    pub fn_0: *mut ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_24 {
    pub list: [C2Rust_Unnamed_25; 16],
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_25 {
    pub kem: *const ptls_hpke_kem_t,
    pub ctx: *mut ptls_key_exchange_context_t,
}
pub type C2Rust_Unnamed_26 = ::core::ffi::c_uint;
pub const IN_SHUTDOWN: C2Rust_Unnamed_26 = 2;
pub const IN_1RTT: C2Rust_Unnamed_26 = 1;
pub const IN_HANDSHAKE: C2Rust_Unnamed_26 = 0;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
pub const __NFDBITS: ::core::ffi::c_int =
    8 as ::core::ffi::c_int * ::core::mem::size_of::<__fd_mask>() as ::core::ffi::c_int;
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_REUSEADDR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const SOMAXCONN: ::core::ffi::c_int = 4096 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const O_WRONLY: ::core::ffi::c_int = 0o1 as ::core::ffi::c_int;
pub const O_CREAT: ::core::ffi::c_int = 0o100 as ::core::ffi::c_int;
pub const O_APPEND: ::core::ffi::c_int = 0o2000 as ::core::ffi::c_int;
pub const O_NONBLOCK: ::core::ffi::c_int = 0o4000 as ::core::ffi::c_int;
pub const F_SETFL: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const OPENSSL_INIT_LOAD_CRYPTO_STRINGS: ::core::ffi::c_long = 0x2 as ::core::ffi::c_long;
pub const OPENSSL_INIT_ADD_ALL_CIPHERS: ::core::ffi::c_long = 0x4 as ::core::ffi::c_long;
pub const OPENSSL_INIT_ADD_ALL_DIGESTS: ::core::ffi::c_long = 0x8 as ::core::ffi::c_long;
pub const X509_L_FILE_LOAD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PTLS_HELLO_RANDOM_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const PTLS_ERROR_CLASS_INTERNAL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const PTLS_ALERT_LEVEL_WARNING: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PTLS_ALERT_CLOSE_NOTIFY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PTLS_ALERT_DECODE_ERROR: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const PTLS_ALERT_ECH_REQUIRED: ::core::ffi::c_int = 121 as ::core::ffi::c_int;
pub const PTLS_ERROR_NO_MEMORY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 1 as ::core::ffi::c_int;
pub const PTLS_ERROR_IN_PROGRESS: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 2 as ::core::ffi::c_int;
pub const PTLS_ERROR_LIBRARY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 3 as ::core::ffi::c_int;
pub const PTLS_ERROR_SESSION_NOT_FOUND: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 5 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn ptls_new(
    mut ctx: *mut ptls_context_t,
    mut is_server: ::core::ffi::c_int,
) -> *mut ptls_t {
    return if is_server != 0 {
        ptls_server_new(ctx)
    } else {
        ptls_client_new(ctx)
    };
}
#[inline]
unsafe extern "C" fn ptls_iovec_init(
    mut p: *const ::core::ffi::c_void,
    mut len: size_t,
) -> ptls_iovec_t {
    let mut r: ptls_iovec_t = st_ptls_iovec_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    };
    r.base = p as *mut uint8_t;
    r.len = len;
    return r;
}
#[inline]
unsafe extern "C" fn ptls_buffer_init(
    mut buf: *mut ptls_buffer_t,
    mut smallbuf: *mut ::core::ffi::c_void,
    mut smallbuf_size: size_t,
) {
    (*buf).base = smallbuf as *mut uint8_t;
    (*buf).off = 0 as size_t;
    (*buf).capacity = smallbuf_size;
    (*buf).is_allocated = 0 as uint8_t;
    (*buf).align_bits = 0 as uint8_t;
}
#[inline]
unsafe extern "C" fn ptls_buffer_dispose(mut buf: *mut ptls_buffer_t) {
    ptls_buffer__release_memory(buf);
    *buf = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0 as size_t,
        off: 0 as size_t,
        is_allocated: 0 as uint8_t,
        align_bits: 0 as uint8_t,
    };
}
pub const AI_PASSIVE: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const AI_ADDRCONFIG: ::core::ffi::c_int = 0x20 as ::core::ffi::c_int;
pub const AI_NUMERICSERV: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn load_certificate_chain(
    mut ctx: *mut ptls_context_t,
    mut fn_0: *const ::core::ffi::c_char,
) {
    if ptls_load_certificates(ctx, fn_0 as *mut ::core::ffi::c_char) != 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"failed to load certificate:%s:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            fn_0,
            strerror(*__errno_location()),
        );
        exit(1 as ::core::ffi::c_int);
    }
}
#[inline]
unsafe extern "C" fn load_raw_public_key(
    mut raw_public_key: *mut ptls_iovec_t,
    mut cert_pem_file: *const ::core::ffi::c_char,
) {
    let mut count: size_t = 0;
    if ptls_load_pem_objects(
        cert_pem_file,
        b"PUBLIC KEY\0".as_ptr() as *const ::core::ffi::c_char,
        raw_public_key,
        1 as size_t,
        &raw mut count,
    ) != 0 as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"failed to load public key:%s:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            cert_pem_file,
            strerror(*__errno_location()),
        );
        exit(1 as ::core::ffi::c_int);
    }
}
#[inline]
unsafe extern "C" fn load_private_key(
    mut ctx: *mut ptls_context_t,
    mut fn_0: *const ::core::ffi::c_char,
) {
    static mut sc: ptls_openssl_sign_certificate_t = st_ptls_openssl_sign_certificate_t {
        super_0: st_ptls_sign_certificate_t { cb: None },
        key: ::core::ptr::null::<EVP_PKEY>() as *mut EVP_PKEY,
        schemes: ::core::ptr::null::<ptls_openssl_signature_scheme_t>(),
        async_0: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut pkey: *mut EVP_PKEY = ::core::ptr::null_mut::<EVP_PKEY>();
    fp = fopen(fn_0, b"rb\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
    if fp.is_null() {
        fprintf(
            stderr,
            b"failed to open file:%s:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            fn_0,
            strerror(*__errno_location()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    pkey = PEM_read_PrivateKey(
        fp,
        ::core::ptr::null_mut::<*mut EVP_PKEY>(),
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    fclose(fp);
    if pkey.is_null() {
        fprintf(
            stderr,
            b"failed to read private key from file:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            fn_0,
        );
        exit(1 as ::core::ffi::c_int);
    }
    ptls_openssl_init_sign_certificate(&raw mut sc, pkey);
    EVP_PKEY_free(pkey);
    (*ctx).sign_certificate = &raw mut sc.super_0;
}
unsafe extern "C" fn util_save_ticket_cb(
    mut _self: *mut ptls_save_ticket_t,
    mut tls: *mut ptls_t,
    mut src: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut self_0: *mut st_util_save_ticket_t = _self as *mut st_util_save_ticket_t;
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fp = fopen(
        &raw mut (*self_0).fn_0 as *mut ::core::ffi::c_char,
        b"wb\0".as_ptr() as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if fp.is_null() {
        fprintf(
            stderr,
            b"failed to open file:%s:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut (*self_0).fn_0 as *mut ::core::ffi::c_char,
            strerror(*__errno_location()),
        );
        return PTLS_ERROR_LIBRARY;
    }
    fwrite(
        src.base as *const ::core::ffi::c_void,
        1 as size_t,
        src.len,
        fp,
    );
    fclose(fp);
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn setup_session_file(
    mut ctx: *mut ptls_context_t,
    mut hsprop: *mut ptls_handshake_properties_t,
    mut fn_0: *const ::core::ffi::c_char,
) {
    static mut st: st_util_save_ticket_t = st_util_save_ticket_t {
        super_0: st_ptls_save_ticket_t { cb: None },
        fn_0: [0; 4096],
    };
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    strcpy(&raw mut st.fn_0 as *mut ::core::ffi::c_char, fn_0);
    st.super_0.cb = Some(
        util_save_ticket_cb
            as unsafe extern "C" fn(
                *mut ptls_save_ticket_t,
                *mut ptls_t,
                ptls_iovec_t,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut st_ptls_save_ticket_t,
                *mut ptls_t,
                ptls_iovec_t,
            ) -> ::core::ffi::c_int,
        >;
    (*ctx).save_ticket = &raw mut st.super_0;
    fp = fopen(fn_0, b"rb\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
    if !fp.is_null() {
        static mut ticket: [uint8_t; 16384] = [0; 16384];
        let mut ticket_size: size_t = fread(
            &raw mut ticket as *mut uint8_t as *mut ::core::ffi::c_void,
            1 as size_t,
            ::core::mem::size_of::<[uint8_t; 16384]>() as size_t,
            fp,
        ) as size_t;
        if ticket_size == 0 as size_t || feof(fp) == 0 {
            fprintf(
                stderr,
                b"failed to load ticket from file:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                fn_0,
            );
            exit(1 as ::core::ffi::c_int);
        }
        fclose(fp);
        (*hsprop).c2rust_unnamed.client.session_ticket = ptls_iovec_init(
            &raw mut ticket as *mut uint8_t as *const ::core::ffi::c_void,
            ticket_size,
        );
    }
}
#[inline]
unsafe extern "C" fn init_cert_store(mut crt_file: *const ::core::ffi::c_char) -> *mut X509_STORE {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut store: *mut X509_STORE = X509_STORE_new();
    if !store.is_null() {
        let mut lookup: *mut X509_LOOKUP = X509_STORE_add_lookup(store, X509_LOOKUP_file());
        ret = X509_LOOKUP_ctrl(
            lookup,
            X509_L_FILE_LOAD,
            crt_file,
            1 as ::core::ffi::c_int as ::core::ffi::c_long,
            ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
        );
        if ret != 1 as ::core::ffi::c_int {
            fprintf(
                stderr,
                b"Cannot load store (%s), ret = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                crt_file,
                ret,
            );
            X509_STORE_free(store);
            exit(1 as ::core::ffi::c_int);
        }
    } else {
        fprintf(
            stderr,
            b"Cannot get a new X509 store\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    return store;
}
#[inline]
unsafe extern "C" fn setup_verify_certificate(
    mut ctx: *mut ptls_context_t,
    mut ca_file: *const ::core::ffi::c_char,
) {
    static mut vc: ptls_openssl_verify_certificate_t = st_ptls_openssl_verify_certificate_t {
        super_0: st_ptls_verify_certificate_t {
            cb: None,
            algos: ::core::ptr::null::<uint16_t>(),
        },
        cert_store: ::core::ptr::null::<X509_STORE>() as *mut X509_STORE,
        override_callback: ::core::ptr::null::<ptls_openssl_override_verify_certificate_t>()
            as *mut ptls_openssl_override_verify_certificate_t,
    };
    ptls_openssl_init_verify_certificate(
        &raw mut vc,
        if !ca_file.is_null() {
            init_cert_store(ca_file)
        } else {
            ::core::ptr::null_mut::<X509_STORE>()
        },
    );
    (*ctx).verify_certificate = &raw mut vc.super_0;
}
#[inline]
unsafe extern "C" fn setup_raw_pubkey_verify_certificate(
    mut ctx: *mut ptls_context_t,
    mut pubkey: *mut EVP_PKEY,
) {
    static mut vc: ptls_openssl_raw_pubkey_verify_certificate_t =
        st_ptls_openssl_raw_pubkey_verify_certificate_t {
            super_0: st_ptls_verify_certificate_t {
                cb: None,
                algos: ::core::ptr::null::<uint16_t>(),
            },
            expected_pubkey: ::core::ptr::null::<EVP_PKEY>() as *mut EVP_PKEY,
        };
    ptls_openssl_raw_pubkey_init_verify_certificate(&raw mut vc, pubkey);
    (*ctx).verify_certificate = &raw mut vc.super_0;
}
unsafe extern "C" fn log_event_cb(
    mut _self: *mut ptls_log_event_t,
    mut tls: *mut ptls_t,
    mut type_0: *const ::core::ffi::c_char,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut self_0: *mut st_util_log_event_t = _self as *mut st_util_log_event_t;
    let mut randomhex: [::core::ffi::c_char; 65] = [0; 65];
    let mut args: ::core::ffi::VaListImpl;
    ptls_hexdump(
        &raw mut randomhex as *mut ::core::ffi::c_char,
        ptls_get_client_random(tls).base as *const ::core::ffi::c_void,
        PTLS_HELLO_RANDOM_SIZE as size_t,
    );
    fprintf(
        (*self_0).fp,
        b"%s %s \0".as_ptr() as *const ::core::ffi::c_char,
        type_0,
        &raw mut randomhex as *mut ::core::ffi::c_char,
    );
    args = c2rust_args.clone();
    vfprintf((*self_0).fp, fmt, args.as_va_list());
    fprintf((*self_0).fp, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    fflush((*self_0).fp);
}
#[inline]
unsafe extern "C" fn setup_log_event(
    mut ctx: *mut ptls_context_t,
    mut fn_0: *const ::core::ffi::c_char,
) {
    static mut ls: st_util_log_event_t = st_util_log_event_t {
        super_0: st_ptls_log_event_t { cb: None },
        fp: ::core::ptr::null::<FILE>() as *mut FILE,
    };
    ls.fp = fopen(fn_0, b"at\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
    if ls.fp.is_null() {
        fprintf(
            stderr,
            b"failed to open file:%s:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            fn_0,
            strerror(*__errno_location()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    ls.super_0.cb = Some(
        log_event_cb
            as unsafe extern "C" fn(
                *mut ptls_log_event_t,
                *mut ptls_t,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                ...
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut st_ptls_log_event_t,
                *mut ptls_t,
                *const ::core::ffi::c_char,
                *const ::core::ffi::c_char,
                ...
            ) -> (),
        >;
    (*ctx).log_event = &raw mut ls.super_0;
}
unsafe extern "C" fn encrypt_ticket_cb(
    mut _self: *mut ptls_encrypt_ticket_t,
    mut tls: *mut ptls_t,
    mut is_encrypt: ::core::ffi::c_int,
    mut dst: *mut ptls_buffer_t,
    mut src: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut self_0: *mut st_util_session_cache_t = _self as *mut st_util_session_cache_t;
    let mut ret: ::core::ffi::c_int = 0;
    if is_encrypt != 0 {
        free((*self_0).data.base as *mut ::core::ffi::c_void);
        (*self_0).data.base = malloc(src.len) as *mut uint8_t;
        if (*self_0).data.base.is_null() {
            return PTLS_ERROR_NO_MEMORY;
        }
        (*ptls_get_context(tls))
            .random_bytes
            .expect("non-null function pointer")(
            &raw mut (*self_0).id as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        );
        memcpy(
            (*self_0).data.base as *mut ::core::ffi::c_void,
            src.base as *const ::core::ffi::c_void,
            src.len,
        );
        (*self_0).data.len = src.len;
        ret = ptls_buffer_reserve(dst, ::core::mem::size_of::<[uint8_t; 32]>() as size_t);
        if ret != 0 as ::core::ffi::c_int {
            return ret;
        }
        memcpy(
            (*dst).base.offset((*dst).off as isize) as *mut ::core::ffi::c_void,
            &raw mut (*self_0).id as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        );
        (*dst).off = ((*dst).off as ::core::ffi::c_ulong)
            .wrapping_add(::core::mem::size_of::<[uint8_t; 32]>() as usize as ::core::ffi::c_ulong)
            as size_t as size_t;
    } else {
        if src.len != ::core::mem::size_of::<[uint8_t; 32]>() as usize {
            return PTLS_ERROR_SESSION_NOT_FOUND;
        }
        if memcmp(
            &raw mut (*self_0).id as *mut uint8_t as *const ::core::ffi::c_void,
            src.base as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        ) != 0 as ::core::ffi::c_int
        {
            return PTLS_ERROR_SESSION_NOT_FOUND;
        }
        ret = ptls_buffer_reserve(dst, (*self_0).data.len);
        if ret != 0 as ::core::ffi::c_int {
            return ret;
        }
        memcpy(
            (*dst).base.offset((*dst).off as isize) as *mut ::core::ffi::c_void,
            (*self_0).data.base as *const ::core::ffi::c_void,
            (*self_0).data.len,
        );
        (*dst).off = (*dst).off.wrapping_add((*self_0).data.len);
    }
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn setup_session_cache(mut ctx: *mut ptls_context_t) {
    static mut sc: st_util_session_cache_t = st_util_session_cache_t {
        super_0: st_ptls_encrypt_ticket_t { cb: None },
        id: [0; 32],
        data: st_ptls_iovec_t {
            base: ::core::ptr::null_mut::<uint8_t>(),
            len: 0,
        },
    };
    sc.super_0.cb = Some(
        encrypt_ticket_cb
            as unsafe extern "C" fn(
                *mut ptls_encrypt_ticket_t,
                *mut ptls_t,
                ::core::ffi::c_int,
                *mut ptls_buffer_t,
                ptls_iovec_t,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut st_ptls_encrypt_ticket_t,
                *mut ptls_t,
                ::core::ffi::c_int,
                *mut ptls_buffer_t,
                ptls_iovec_t,
            ) -> ::core::ffi::c_int,
        >;
    (*ctx).ticket_lifetime = 86400 as uint32_t;
    (*ctx).max_early_data_size = 8192 as uint32_t;
    (*ctx).encrypt_ticket = &raw mut sc.super_0;
}
static mut ech: C2Rust_Unnamed_22 = C2Rust_Unnamed_22 {
    config_list: st_ptls_iovec_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    },
    keyex: C2Rust_Unnamed_24 {
        list: [C2Rust_Unnamed_25 {
            kem: ::core::ptr::null::<ptls_hpke_kem_t>(),
            ctx: ::core::ptr::null::<ptls_key_exchange_context_t>()
                as *mut ptls_key_exchange_context_t,
        }; 16],
        count: 0,
    },
    retry: C2Rust_Unnamed_23 {
        configs: st_ptls_iovec_t {
            base: ::core::ptr::null_mut::<uint8_t>(),
            len: 0,
        },
        fn_0: ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char,
    },
};
unsafe extern "C" fn ech_create_opener(
    mut self_0: *mut ptls_ech_create_opener_t,
    mut kem: *mut *const ptls_hpke_kem_t,
    mut cipher: *mut *const ptls_hpke_cipher_suite_t,
    mut tls: *mut ptls_t,
    mut config_id: uint8_t,
    mut cipher_id: ptls_hpke_cipher_suite_id_t,
    mut enc: ptls_iovec_t,
    mut info_prefix: ptls_iovec_t,
) -> *mut ptls_aead_context_t {
    let mut c2rust_current_block: u64;
    let mut src: *const uint8_t = ech.config_list.base;
    let end: *const uint8_t = src.offset(ech.config_list.len as isize);
    let mut index: size_t = 0 as size_t;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    *cipher = ::core::ptr::null::<ptls_hpke_cipher_suite_t>();
    let mut i: size_t = 0 as size_t;
    while !(*(&raw mut ptls_openssl_hpke_cipher_suites as *mut *const ptls_hpke_cipher_suite_t)
        .offset(i as isize))
    .is_null()
    {
        if (**(&raw mut ptls_openssl_hpke_cipher_suites as *mut *const ptls_hpke_cipher_suite_t)
            .offset(i as isize))
        .id
        .kdf as ::core::ffi::c_int
            == cipher_id.kdf as ::core::ffi::c_int
            && (**(&raw mut ptls_openssl_hpke_cipher_suites
                as *mut *const ptls_hpke_cipher_suite_t)
                .offset(i as isize))
            .id
            .aead as ::core::ffi::c_int
                == cipher_id.aead as ::core::ffi::c_int
        {
            *cipher = *(&raw mut ptls_openssl_hpke_cipher_suites
                as *mut *const ptls_hpke_cipher_suite_t)
                .offset(i as isize);
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if !(*cipher).is_null() {
        let mut _capacity: size_t = 2 as size_t;
        let mut _block_size: size_t = 0;
        if _capacity == -(1 as ::core::ffi::c_int) as size_t {
            let mut _block_size64: uint64_t = 0;
            let mut _src: *const uint8_t = src;
            _block_size64 = ptls_decode_quicint(&raw mut _src, end);
            if _block_size64 == UINT64_MAX as uint64_t
                || (::core::mem::size_of::<size_t>() as usize) < 8 as usize
                    && _block_size64
                        >> (8 as usize).wrapping_mul(::core::mem::size_of::<size_t>() as usize)
                        != 0 as uint64_t
            {
                ret = PTLS_ALERT_DECODE_ERROR;
                c2rust_current_block = 10042394784345274924;
            } else {
                src = _src;
                _block_size = _block_size64 as size_t;
                c2rust_current_block = 4808432441040389987;
            }
        } else if _capacity > end.offset_from(src) as ::core::ffi::c_long as size_t {
            ret = PTLS_ALERT_DECODE_ERROR;
            c2rust_current_block = 10042394784345274924;
        } else {
            _block_size = 0 as size_t;
            loop {
                let c2rust_fresh0 = src;
                src = src.offset(1);
                _block_size = _block_size << 8 as ::core::ffi::c_int | *c2rust_fresh0 as size_t;
                _capacity = _capacity.wrapping_sub(1);
                if !(_capacity != 0 as size_t) {
                    break;
                }
            }
            c2rust_current_block = 4808432441040389987;
        }
        match c2rust_current_block {
            10042394784345274924 => {}
            _ => {
                if _block_size > end.offset_from(src) as ::core::ffi::c_long as size_t {
                    ret = PTLS_ALERT_DECODE_ERROR;
                } else {
                    let end_0: *const uint8_t = src.offset(_block_size as isize);
                    let mut version: uint16_t = 0;
                    ret = ptls_decode16(&raw mut version, &raw mut src, end_0);
                    if !(ret != 0 as ::core::ffi::c_int) {
                        loop {
                            let mut _capacity_0: size_t = 2 as size_t;
                            let mut _block_size_0: size_t = 0;
                            if _capacity_0 == -(1 as ::core::ffi::c_int) as size_t {
                                let mut _block_size64_0: uint64_t = 0;
                                let mut _src_0: *const uint8_t = src;
                                _block_size64_0 = ptls_decode_quicint(&raw mut _src_0, end_0);
                                if _block_size64_0 == 18446744073709551615 as uint64_t
                                    || (::core::mem::size_of::<size_t>() as usize) < 8 as usize
                                        && _block_size64_0
                                            >> (8 as usize).wrapping_mul(::core::mem::size_of::<
                                                size_t,
                                            >(
                                            )
                                                as usize)
                                            != 0 as uint64_t
                                {
                                    ret = 50 as ::core::ffi::c_int;
                                    c2rust_current_block = 10042394784345274924;
                                    break;
                                } else {
                                    src = _src_0;
                                    _block_size_0 = _block_size64_0 as size_t;
                                }
                            } else if _capacity_0
                                > end_0.offset_from(src) as ::core::ffi::c_long as size_t
                            {
                                ret = 50 as ::core::ffi::c_int;
                                c2rust_current_block = 10042394784345274924;
                                break;
                            } else {
                                _block_size_0 = 0 as size_t;
                                loop {
                                    let c2rust_fresh1 = src;
                                    src = src.offset(1);
                                    _block_size_0 = _block_size_0 << 8 as ::core::ffi::c_int
                                        | *c2rust_fresh1 as size_t;
                                    _capacity_0 = _capacity_0.wrapping_sub(1);
                                    if !(_capacity_0 != 0 as size_t) {
                                        break;
                                    }
                                }
                            }
                            if _block_size_0
                                > end_0.offset_from(src) as ::core::ffi::c_long as size_t
                            {
                                ret = 50 as ::core::ffi::c_int;
                                c2rust_current_block = 10042394784345274924;
                                break;
                            } else {
                                let end_1: *const uint8_t = src.offset(_block_size_0 as isize);
                                if src == end_1 {
                                    ret = 50 as ::core::ffi::c_int;
                                    c2rust_current_block = 10042394784345274924;
                                    break;
                                } else {
                                    if *src as ::core::ffi::c_int == config_id as ::core::ffi::c_int
                                    {
                                        if index >= ech.keyex.count {
                                            fprintf(
                                                stderr,
                                                b"ECH key missing for config %zu\n\0".as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                index,
                                            );
                                            return ::core::ptr::null_mut::<ptls_aead_context_t>();
                                        }
                                        let mut info: *mut uint8_t = malloc(
                                            end_1.offset(info_prefix.len as isize).offset_from(
                                                src.offset(-(4 as ::core::ffi::c_int as isize)),
                                            )
                                                as ::core::ffi::c_long
                                                as size_t,
                                        )
                                            as *mut uint8_t;
                                        memcpy(
                                            info as *mut ::core::ffi::c_void,
                                            info_prefix.base as *const ::core::ffi::c_void,
                                            info_prefix.len,
                                        );
                                        memcpy(
                                            info.offset(info_prefix.len as isize)
                                                as *mut ::core::ffi::c_void,
                                            src.offset(-(4 as ::core::ffi::c_int as isize))
                                                as *const ::core::ffi::c_void,
                                            end_1.offset_from(
                                                src.offset(-(4 as ::core::ffi::c_int as isize)),
                                            )
                                                as ::core::ffi::c_long
                                                as size_t,
                                        );
                                        let mut aead: *mut ptls_aead_context_t =
                                            ::core::ptr::null_mut::<ptls_aead_context_t>();
                                        ptls_hpke_setup_base_r(
                                            ech.keyex.list[index as usize].kem,
                                            *cipher,
                                            ech.keyex.list[index as usize].ctx,
                                            &raw mut aead,
                                            enc,
                                            ptls_iovec_init(
                                                info as *const ::core::ffi::c_void,
                                                end_1.offset(info_prefix.len as isize).offset_from(
                                                    src.offset(-(4 as ::core::ffi::c_int as isize)),
                                                )
                                                    as ::core::ffi::c_long
                                                    as size_t,
                                            ),
                                        );
                                        free(info as *mut ::core::ffi::c_void);
                                        *kem = ech.keyex.list[index as usize].kem;
                                        return aead;
                                    }
                                    index = index.wrapping_add(1);
                                    src = end_1;
                                    if src != end_1 {
                                        ret = 50 as ::core::ffi::c_int;
                                        c2rust_current_block = 10042394784345274924;
                                        break;
                                    } else if !(src != end_0) {
                                        c2rust_current_block = 3392087639489470149;
                                        break;
                                    }
                                }
                            }
                        }
                        match c2rust_current_block {
                            10042394784345274924 => {}
                            _ => {
                                if src != end_0 {
                                    ret = PTLS_ALERT_DECODE_ERROR;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if ret != 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"ECH decode error:%d\n\0".as_ptr() as *const ::core::ffi::c_char,
            ret,
        );
    }
    return ::core::ptr::null_mut::<ptls_aead_context_t>();
}
unsafe extern "C" fn ech_save_retry_configs() {
    if ech.retry.configs.base.is_null() {
        return;
    }
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    fp = fopen(
        ech.retry.fn_0,
        b"wt\0".as_ptr() as *const ::core::ffi::c_char,
    ) as *mut FILE;
    if fp.is_null() {
        fprintf(
            stderr,
            b"failed to write to ECH config file:%s:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            ech.retry.fn_0,
            strerror(*__errno_location()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    fwrite(
        ech.retry.configs.base as *const ::core::ffi::c_void,
        1 as size_t,
        ech.retry.configs.len,
        fp,
    );
    fclose(fp);
}
unsafe extern "C" fn load_file(mut fn_0: *const ::core::ffi::c_char) -> ptls_iovec_t {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut buf: ptls_iovec_t = st_ptls_iovec_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    };
    fp = fopen(fn_0, b"rt\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
    if fp.is_null() {
        fprintf(
            stderr,
            b"failed to open file:%s:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            fn_0,
            strerror(*__errno_location()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    buf.len = 65536 as ::core::ffi::c_int as size_t;
    buf.base = malloc(buf.len) as *mut uint8_t;
    if buf.base.is_null() {
        fprintf(
            stderr,
            b"no memory\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        abort();
    }
    buf.len = fread(
        buf.base as *mut ::core::ffi::c_void,
        1 as size_t,
        buf.len,
        fp,
    ) as size_t;
    fclose(fp);
    return buf;
}
unsafe extern "C" fn ech_setup_configs(mut fn_0: *const ::core::ffi::c_char) {
    ech.config_list = load_file(fn_0);
    ech.retry.fn_0 = strdup(fn_0);
}
unsafe extern "C" fn ech_setup_key(
    mut ctx: *mut ptls_context_t,
    mut fn_0: *const ::core::ffi::c_char,
) {
    let mut fp: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut pkey: *mut EVP_PKEY = ::core::ptr::null_mut::<EVP_PKEY>();
    let mut ret: ::core::ffi::c_int = 0;
    fp = fopen(fn_0, b"rt\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
    if fp.is_null() {
        fprintf(
            stderr,
            b"failed to open ECH private key file:%s:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            fn_0,
            strerror(*__errno_location()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    pkey = PEM_read_PrivateKey(
        fp,
        ::core::ptr::null_mut::<*mut EVP_PKEY>(),
        None,
        ::core::ptr::null_mut::<::core::ffi::c_void>(),
    );
    if pkey.is_null() {
        fprintf(
            stderr,
            b"failed to load private key from file:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            fn_0,
        );
        exit(1 as ::core::ffi::c_int);
    }
    ret = ptls_openssl_create_key_exchange(
        &raw mut (*(&raw mut ech.keyex.list as *mut C2Rust_Unnamed_25)
            .offset(ech.keyex.count as isize))
        .ctx,
        pkey,
    );
    if ret != 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"failed to load private key from file:%s:picotls-error:%d\0".as_ptr()
                as *const ::core::ffi::c_char,
            fn_0,
            ret,
        );
        exit(1 as ::core::ffi::c_int);
    }
    EVP_PKEY_free(pkey);
    fclose(fp);
    let mut i: size_t = 0 as size_t;
    while !(*(&raw mut ptls_openssl_hpke_kems as *mut *const ptls_hpke_kem_t).offset(i as isize))
        .is_null()
    {
        if (**(&raw mut ptls_openssl_hpke_kems as *mut *const ptls_hpke_kem_t).offset(i as isize))
            .keyex
            == (*ech.keyex.list[ech.keyex.count as usize].ctx).algo
        {
            ech.keyex.list[ech.keyex.count as usize].kem = *(&raw mut ptls_openssl_hpke_kems
                as *mut *const ptls_hpke_kem_t)
                .offset(i as isize);
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if ech.keyex.list[ech.keyex.count as usize].kem.is_null() {
        fprintf(
            stderr,
            b"kem unknown for private key:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            fn_0,
        );
        exit(1 as ::core::ffi::c_int);
    }
    ech.keyex.count = ech.keyex.count.wrapping_add(1);
    static mut opener: ptls_ech_create_opener_t = unsafe {
        st_ptls_ech_create_opener_t {
            cb: Some(
                ech_create_opener
                    as unsafe extern "C" fn(
                        *mut ptls_ech_create_opener_t,
                        *mut *const ptls_hpke_kem_t,
                        *mut *const ptls_hpke_cipher_suite_t,
                        *mut ptls_t,
                        uint8_t,
                        ptls_hpke_cipher_suite_id_t,
                        ptls_iovec_t,
                        ptls_iovec_t,
                    ) -> *mut ptls_aead_context_t,
            ),
        }
    };
    (*ctx).ech.server.create_opener = &raw mut opener;
}
#[inline]
unsafe extern "C" fn resolve_address(
    mut sa: *mut sockaddr,
    mut salen: *mut socklen_t,
    mut host: *const ::core::ffi::c_char,
    mut port: *const ::core::ffi::c_char,
    mut family: ::core::ffi::c_int,
    mut type_0: ::core::ffi::c_int,
    mut proto: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
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
    let mut res: *mut addrinfo = ::core::ptr::null_mut::<addrinfo>();
    let mut err: ::core::ffi::c_int = 0;
    memset(
        &raw mut hints as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<addrinfo>() as size_t,
    );
    hints.ai_family = family;
    hints.ai_socktype = type_0;
    hints.ai_protocol = proto;
    hints.ai_flags = AI_ADDRCONFIG | AI_NUMERICSERV | AI_PASSIVE;
    err = getaddrinfo(host, port, &raw mut hints, &raw mut res);
    if err != 0 as ::core::ffi::c_int || res.is_null() {
        fprintf(
            stderr,
            b"failed to resolve address:%s:%s:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            host,
            port,
            if err != 0 as ::core::ffi::c_int {
                gai_strerror(err)
            } else {
                b"getaddrinfo returned NULL\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        return -(1 as ::core::ffi::c_int);
    }
    memcpy(
        sa as *mut ::core::ffi::c_void,
        (*res).ai_addr as *const ::core::ffi::c_void,
        (*res).ai_addrlen as size_t,
    );
    *salen = (*res).ai_addrlen;
    freeaddrinfo(res);
    return 0 as ::core::ffi::c_int;
}
static mut input_file_is_benchmark: [::core::ffi::c_char; 13] =
    unsafe { ::core::mem::transmute::<[u8; 13], [::core::ffi::c_char; 13]>(*b"is:benchmark\0") };
unsafe extern "C" fn shift_buffer(mut buf: *mut ptls_buffer_t, mut delta: size_t) {
    if delta != 0 as size_t {
        if delta != (*buf).off {
            memmove(
                (*buf).base as *mut ::core::ffi::c_void,
                (*buf).base.offset(delta as isize) as *const ::core::ffi::c_void,
                (*buf).off.wrapping_sub(delta),
            );
        }
        (*buf).off = (*buf).off.wrapping_sub(delta);
    }
}
unsafe extern "C" fn setup_ptlslog(mut fn_0: *const ::core::ffi::c_char) {
    let mut fd: ::core::ffi::c_int = 0;
    fd = open(
        fn_0,
        O_WRONLY | O_CREAT | O_APPEND,
        0o666 as ::core::ffi::c_int,
    );
    if fd == -(1 as ::core::ffi::c_int) {
        fprintf(
            stderr,
            b"failed to open file:%s:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            fn_0,
            strerror(*__errno_location()),
        );
        exit(1 as ::core::ffi::c_int);
    }
    ptls_log_add_fd(fd);
}
unsafe extern "C" fn handle_connection(
    mut sockfd: ::core::ffi::c_int,
    mut ctx: *mut ptls_context_t,
    mut server_name: *const ::core::ffi::c_char,
    mut input_file: *const ::core::ffi::c_char,
    mut hsprop: *mut ptls_handshake_properties_t,
    mut request_key_update: ::core::ffi::c_int,
    mut keep_sender_open: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut c2rust_current_block: u64;
    static mut inputfd_is_benchmark: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
    let mut tls: *mut ptls_t = ptls_new(
        ctx,
        (server_name
            == ::core::ptr::null_mut::<::core::ffi::c_void>() as *const ::core::ffi::c_char)
            as ::core::ffi::c_int,
    );
    let mut rbuf: ptls_buffer_t = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0,
        off: 0,
        is_allocated: 0,
        align_bits: 0,
    };
    let mut encbuf: ptls_buffer_t = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0,
        off: 0,
        is_allocated: 0,
        align_bits: 0,
    };
    let mut ptbuf: ptls_buffer_t = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0,
        off: 0,
        is_allocated: 0,
        align_bits: 0,
    };
    let mut state: C2Rust_Unnamed_26 = IN_HANDSHAKE;
    let mut inputfd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut early_bytes_sent: size_t = 0 as size_t;
    let mut data_received: uint64_t = 0 as uint64_t;
    let mut ioret: ssize_t = 0;
    let mut start_at: uint64_t = (*(*ctx).get_time).cb.expect("non-null function pointer")(
        (*ctx).get_time as *mut st_ptls_get_time_t,
    );
    ptls_buffer_init(
        &raw mut rbuf,
        b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as size_t,
    );
    ptls_buffer_init(
        &raw mut encbuf,
        b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as size_t,
    );
    ptls_buffer_init(
        &raw mut ptbuf,
        b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as size_t,
    );
    fcntl(sockfd, F_SETFL, O_NONBLOCK);
    if input_file == &raw const input_file_is_benchmark as *const ::core::ffi::c_char {
        if ptls_is_server(tls) == 0 {
            inputfd = inputfd_is_benchmark;
        }
        c2rust_current_block = 13586036798005543211;
    } else if !input_file.is_null() {
        inputfd = open(input_file, O_RDONLY);
        if inputfd == -(1 as ::core::ffi::c_int) {
            fprintf(
                stderr,
                b"failed to open file:%s:%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                input_file,
                strerror(*__errno_location()),
            );
            ret = 1 as ::core::ffi::c_int;
            c2rust_current_block = 8021516759781826851;
        } else {
            c2rust_current_block = 13586036798005543211;
        }
    } else {
        c2rust_current_block = 13586036798005543211;
    }
    match c2rust_current_block {
        13586036798005543211 => {
            if !server_name.is_null() {
                ptls_set_server_name(tls, server_name, 0 as size_t);
                ret = ptls_handshake(
                    tls,
                    &raw mut encbuf,
                    ::core::ptr::null::<::core::ffi::c_void>(),
                    ::core::ptr::null_mut::<size_t>(),
                    hsprop,
                );
                if ret != PTLS_ERROR_IN_PROGRESS {
                    fprintf(
                        stderr,
                        b"ptls_handshake:%d\n\0".as_ptr() as *const ::core::ffi::c_char,
                        ret,
                    );
                    ret = 1 as ::core::ffi::c_int;
                    c2rust_current_block = 8021516759781826851;
                } else {
                    c2rust_current_block = 15904375183555213903;
                }
            } else {
                c2rust_current_block = 15904375183555213903;
            }
            match c2rust_current_block {
                8021516759781826851 => {}
                _ => 's_94: loop {
                    let mut readfds: fd_set = fd_set { fds_bits: [0; 16] };
                    let mut writefds: fd_set = fd_set { fds_bits: [0; 16] };
                    let mut exceptfds: fd_set = fd_set { fds_bits: [0; 16] };
                    let mut maxfd: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut timeout: timeval = timeval {
                        tv_sec: 0,
                        tv_usec: 0,
                    };
                    loop {
                        let mut __i: ::core::ffi::c_uint = 0;
                        let mut __arr: *mut fd_set = &raw mut readfds;
                        __i = 0 as ::core::ffi::c_uint;
                        while (__i as usize)
                            < (::core::mem::size_of::<fd_set>() as usize)
                                .wrapping_div(::core::mem::size_of::<__fd_mask>() as usize)
                        {
                            (*__arr).fds_bits[__i as usize] = 0 as __fd_mask;
                            __i = __i.wrapping_add(1);
                        }
                        let mut __i_0: ::core::ffi::c_uint = 0;
                        let mut __arr_0: *mut fd_set = &raw mut writefds;
                        __i_0 = 0 as ::core::ffi::c_uint;
                        while (__i_0 as usize)
                            < (::core::mem::size_of::<fd_set>() as usize)
                                .wrapping_div(::core::mem::size_of::<__fd_mask>() as usize)
                        {
                            (*__arr_0).fds_bits[__i_0 as usize] = 0 as __fd_mask;
                            __i_0 = __i_0.wrapping_add(1);
                        }
                        let mut __i_1: ::core::ffi::c_uint = 0;
                        let mut __arr_1: *mut fd_set = &raw mut exceptfds;
                        __i_1 = 0 as ::core::ffi::c_uint;
                        while (__i_1 as usize)
                            < (::core::mem::size_of::<fd_set>() as usize)
                                .wrapping_div(::core::mem::size_of::<__fd_mask>() as usize)
                        {
                            (*__arr_1).fds_bits[__i_1 as usize] = 0 as __fd_mask;
                            __i_1 = __i_1.wrapping_add(1);
                        }
                        readfds.fds_bits[(sockfd / __NFDBITS) as usize] |=
                            ((1 as ::core::ffi::c_ulong) << sockfd % __NFDBITS) as __fd_mask;
                        if encbuf.off != 0 as size_t || inputfd == inputfd_is_benchmark {
                            writefds.fds_bits[(sockfd / __NFDBITS) as usize] |=
                                ((1 as ::core::ffi::c_ulong) << sockfd % __NFDBITS) as __fd_mask;
                        }
                        exceptfds.fds_bits[(sockfd / __NFDBITS) as usize] |=
                            ((1 as ::core::ffi::c_ulong) << sockfd % __NFDBITS) as __fd_mask;
                        maxfd = sockfd + 1 as ::core::ffi::c_int;
                        if inputfd >= 0 as ::core::ffi::c_int {
                            readfds.fds_bits[(inputfd / __NFDBITS) as usize] |=
                                ((1 as ::core::ffi::c_ulong) << inputfd % __NFDBITS) as __fd_mask;
                            exceptfds.fds_bits[(inputfd / __NFDBITS) as usize] |=
                                ((1 as ::core::ffi::c_ulong) << inputfd % __NFDBITS) as __fd_mask;
                            if maxfd <= inputfd {
                                maxfd = inputfd + 1 as ::core::ffi::c_int;
                            }
                        }
                        timeout.tv_sec = (if encbuf.off != 0 as size_t {
                            0 as ::core::ffi::c_int
                        } else {
                            3600 as ::core::ffi::c_int
                        }) as __time_t;
                        timeout.tv_usec = 0 as __suseconds_t;
                        if !(select(
                            maxfd,
                            &raw mut readfds,
                            &raw mut writefds,
                            &raw mut exceptfds,
                            &raw mut timeout,
                        ) == -(1 as ::core::ffi::c_int))
                        {
                            break;
                        }
                    }
                    if readfds.fds_bits[(sockfd / __NFDBITS) as usize]
                        & ((1 as ::core::ffi::c_ulong) << sockfd % __NFDBITS) as __fd_mask
                        != 0 as __fd_mask
                        || exceptfds.fds_bits[(sockfd / __NFDBITS) as usize]
                            & ((1 as ::core::ffi::c_ulong) << sockfd % __NFDBITS) as __fd_mask
                            != 0 as __fd_mask
                    {
                        let mut bytebuf: [::core::ffi::c_char; 16384] = [0; 16384];
                        let mut off: size_t = 0 as size_t;
                        let mut leftlen: size_t = 0;
                        loop {
                            ioret = read(
                                sockfd,
                                &raw mut bytebuf as *mut ::core::ffi::c_char
                                    as *mut ::core::ffi::c_void,
                                ::core::mem::size_of::<[::core::ffi::c_char; 16384]>() as size_t,
                            );
                            if !(ioret == -(1 as ::core::ffi::c_int) as ssize_t
                                && *__errno_location() == EINTR)
                            {
                                break;
                            }
                        }
                        if ioret == -(1 as ::core::ffi::c_int) as ssize_t
                            && (*__errno_location() == EWOULDBLOCK || *__errno_location() == EAGAIN)
                        {
                            ioret = 0 as ssize_t;
                        } else if ioret <= 0 as ssize_t {
                            break;
                        }
                        loop {
                            leftlen = (ioret as size_t).wrapping_sub(off);
                            if !(leftlen != 0 as size_t) {
                                break;
                            }
                            if state as ::core::ffi::c_uint
                                == IN_HANDSHAKE as ::core::ffi::c_int as ::core::ffi::c_uint
                            {
                                ret = ptls_handshake(
                                    tls,
                                    &raw mut encbuf,
                                    (&raw mut bytebuf as *mut ::core::ffi::c_char)
                                        .offset(off as isize)
                                        as *const ::core::ffi::c_void,
                                    &raw mut leftlen,
                                    hsprop,
                                );
                                if ret == 0 as ::core::ffi::c_int {
                                    state = IN_1RTT;
                                    ech_save_retry_configs();
                                    if (*hsprop).c2rust_unnamed.client.early_data_acceptance
                                        as ::core::ffi::c_uint
                                        == PTLS_EARLY_DATA_ACCEPTED as ::core::ffi::c_int
                                            as ::core::ffi::c_uint
                                    {
                                        shift_buffer(&raw mut ptbuf, early_bytes_sent);
                                    }
                                    if request_key_update != 0 {
                                        ptls_update_key(tls, 1 as ::core::ffi::c_int);
                                    }
                                } else if !(ret == PTLS_ERROR_IN_PROGRESS) {
                                    if ret == PTLS_ALERT_ECH_REQUIRED {
                                        ech_save_retry_configs();
                                    }
                                    if encbuf.off != 0 as size_t {
                                        while write(
                                            sockfd,
                                            encbuf.base as *const ::core::ffi::c_void,
                                            encbuf.off,
                                        ) < 0 as ssize_t
                                        {
                                            if !(*__errno_location() == EINTR) {
                                                break;
                                            }
                                        }
                                    }
                                    fprintf(
                                        stderr,
                                        b"ptls_handshake:%d\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        ret,
                                    );
                                    break 's_94;
                                }
                            } else {
                                ret = ptls_receive(
                                    tls,
                                    &raw mut rbuf,
                                    (&raw mut bytebuf as *mut ::core::ffi::c_char)
                                        .offset(off as isize)
                                        as *const ::core::ffi::c_void,
                                    &raw mut leftlen,
                                );
                                if ret == 0 as ::core::ffi::c_int {
                                    if rbuf.off != 0 as size_t {
                                        data_received = (data_received as ::core::ffi::c_ulong)
                                            .wrapping_add(rbuf.off as ::core::ffi::c_ulong)
                                            as uint64_t
                                            as uint64_t;
                                        if input_file
                                            != &raw const input_file_is_benchmark
                                                as *const ::core::ffi::c_char
                                        {
                                            while write(
                                                1 as ::core::ffi::c_int,
                                                rbuf.base as *const ::core::ffi::c_void,
                                                rbuf.off,
                                            ) < 0 as ssize_t
                                            {
                                                if !(*__errno_location() == EINTR) {
                                                    break 's_94;
                                                }
                                            }
                                        }
                                        rbuf.off = 0 as size_t;
                                    }
                                } else if !(ret == PTLS_ERROR_IN_PROGRESS) {
                                    fprintf(
                                        stderr,
                                        b"ptls_receive:%d\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        ret,
                                    );
                                    break 's_94;
                                }
                            }
                            off = off.wrapping_add(leftlen);
                        }
                    }
                    if encbuf.off == 0 as size_t
                        || state as ::core::ffi::c_uint
                            == IN_HANDSHAKE as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        static mut block_size: size_t = 16384 as size_t;
                        if inputfd >= 0 as ::core::ffi::c_int
                            && (readfds.fds_bits[(inputfd / __NFDBITS) as usize]
                                & ((1 as ::core::ffi::c_ulong) << inputfd % __NFDBITS) as __fd_mask
                                != 0 as __fd_mask
                                || exceptfds.fds_bits[(inputfd / __NFDBITS) as usize]
                                    & ((1 as ::core::ffi::c_ulong) << inputfd % __NFDBITS)
                                        as __fd_mask
                                    != 0 as __fd_mask)
                        {
                            ret = ptls_buffer_reserve(&raw mut ptbuf, block_size);
                            if ret != 0 as ::core::ffi::c_int {
                                break;
                            }
                            loop {
                                ioret = read(
                                    inputfd,
                                    ptbuf.base.offset(ptbuf.off as isize)
                                        as *mut ::core::ffi::c_void,
                                    block_size,
                                );
                                if !(ioret == -(1 as ::core::ffi::c_int) as ssize_t
                                    && *__errno_location() == EINTR)
                                {
                                    break;
                                }
                            }
                            if ioret > 0 as ssize_t {
                                ptbuf.off = ptbuf.off.wrapping_add(ioret as size_t);
                            } else if ioret == 0 as ssize_t {
                                if !input_file.is_null() {
                                    close(inputfd);
                                }
                                inputfd = -(1 as ::core::ffi::c_int);
                            }
                        } else if inputfd == inputfd_is_benchmark {
                            if ptbuf.capacity < block_size {
                                ret = ptls_buffer_reserve(
                                    &raw mut ptbuf,
                                    block_size.wrapping_sub(ptbuf.capacity),
                                );
                                if ret != 0 as ::core::ffi::c_int {
                                    break;
                                }
                                memset(
                                    ptbuf.base.offset(ptbuf.capacity as isize)
                                        as *mut ::core::ffi::c_void,
                                    0 as ::core::ffi::c_int,
                                    block_size.wrapping_sub(ptbuf.capacity),
                                );
                            }
                            ptbuf.off = block_size;
                        }
                    }
                    if ptbuf.off != 0 as size_t {
                        if state as ::core::ffi::c_uint
                            == IN_HANDSHAKE as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            let mut send_amount: size_t = 0 as size_t;
                            if !server_name.is_null()
                                && !(*hsprop)
                                    .c2rust_unnamed
                                    .client
                                    .max_early_data_size
                                    .is_null()
                            {
                                let mut max_can_be_sent: size_t =
                                    *(*hsprop).c2rust_unnamed.client.max_early_data_size;
                                if max_can_be_sent > ptbuf.off {
                                    max_can_be_sent = ptbuf.off;
                                }
                                send_amount = max_can_be_sent.wrapping_sub(early_bytes_sent);
                            }
                            if send_amount != 0 as size_t {
                                ret = ptls_send(
                                    tls,
                                    &raw mut encbuf,
                                    ptbuf.base as *const ::core::ffi::c_void,
                                    send_amount,
                                );
                                if ret != 0 as ::core::ffi::c_int {
                                    fprintf(
                                        stderr,
                                        b"ptls_send(early_data):%d\n\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        ret,
                                    );
                                    break;
                                } else {
                                    early_bytes_sent = early_bytes_sent.wrapping_add(send_amount);
                                }
                            }
                        } else {
                            ret = ptls_send(
                                tls,
                                &raw mut encbuf,
                                ptbuf.base as *const ::core::ffi::c_void,
                                ptbuf.off,
                            );
                            if ret != 0 as ::core::ffi::c_int {
                                fprintf(
                                    stderr,
                                    b"ptls_send(1rtt):%d\n\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    ret,
                                );
                                break;
                            } else {
                                ptbuf.off = 0 as size_t;
                            }
                        }
                    }
                    if encbuf.off != 0 as size_t {
                        loop {
                            ioret = write(
                                sockfd,
                                encbuf.base as *const ::core::ffi::c_void,
                                encbuf.off,
                            );
                            if !(ioret == -(1 as ::core::ffi::c_int) as ssize_t
                                && *__errno_location() == EINTR)
                            {
                                break;
                            }
                        }
                        if !(ioret == -(1 as ::core::ffi::c_int) as ssize_t
                            && (*__errno_location() == EWOULDBLOCK
                                || *__errno_location() == EAGAIN))
                        {
                            if ioret <= 0 as ssize_t {
                                break;
                            }
                            shift_buffer(&raw mut encbuf, ioret as size_t);
                        }
                    }
                    if !(state as ::core::ffi::c_uint
                        == IN_1RTT as ::core::ffi::c_int as ::core::ffi::c_uint
                        && inputfd == -(1 as ::core::ffi::c_int))
                    {
                        continue;
                    }
                    if keep_sender_open == 0 {
                        let mut wbuf: ptls_buffer_t = st_ptls_buffer_t {
                            base: ::core::ptr::null_mut::<uint8_t>(),
                            capacity: 0,
                            off: 0,
                            is_allocated: 0,
                            align_bits: 0,
                        };
                        let mut wbuf_small: [uint8_t; 32] = [0; 32];
                        ptls_buffer_init(
                            &raw mut wbuf,
                            &raw mut wbuf_small as *mut uint8_t as *mut ::core::ffi::c_void,
                            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
                        );
                        ret = ptls_send_alert(
                            tls,
                            &raw mut wbuf,
                            PTLS_ALERT_LEVEL_WARNING as uint8_t,
                            PTLS_ALERT_CLOSE_NOTIFY as uint8_t,
                        );
                        if ret != 0 as ::core::ffi::c_int {
                            fprintf(
                                stderr,
                                b"ptls_send_alert:%d\n\0".as_ptr() as *const ::core::ffi::c_char,
                                ret,
                            );
                        }
                        if wbuf.off != 0 as size_t {
                            while write(sockfd, wbuf.base as *const ::core::ffi::c_void, wbuf.off)
                                < 0 as ssize_t
                            {
                                if *__errno_location() == EINTR {
                                    continue;
                                }
                                ptls_buffer_dispose(&raw mut wbuf);
                                break 's_94;
                            }
                        }
                        ptls_buffer_dispose(&raw mut wbuf);
                        shutdown(sockfd, SHUT_WR as ::core::ffi::c_int);
                    }
                    state = IN_SHUTDOWN;
                },
            }
        }
        _ => {}
    }
    if input_file == &raw const input_file_is_benchmark as *const ::core::ffi::c_char {
        let mut elapsed: ::core::ffi::c_double =
            (*(*ctx).get_time).cb.expect("non-null function pointer")(
                (*ctx).get_time as *mut st_ptls_get_time_t,
            )
            .wrapping_sub(start_at) as ::core::ffi::c_double
                / 1000.0f64;
        let mut cipher_suite: *const ptls_cipher_suite_t = ptls_get_cipher(tls);
        fprintf(
            stderr,
            b"received %lu bytes in %.3f seconds (%f.3Mbps); %s\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            data_received,
            elapsed,
            data_received.wrapping_mul(8 as uint64_t) as ::core::ffi::c_double
                / elapsed
                / 1000 as ::core::ffi::c_int as ::core::ffi::c_double
                / 1000 as ::core::ffi::c_int as ::core::ffi::c_double,
            if !cipher_suite.is_null() {
                (*(*cipher_suite).aead).name
            } else {
                b"unknown cipher\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
    }
    if sockfd != -(1 as ::core::ffi::c_int) {
        close(sockfd);
    }
    if !input_file.is_null()
        && input_file != &raw const input_file_is_benchmark as *const ::core::ffi::c_char
        && inputfd >= 0 as ::core::ffi::c_int
    {
        close(inputfd);
    }
    ptls_buffer_dispose(&raw mut rbuf);
    ptls_buffer_dispose(&raw mut encbuf);
    ptls_buffer_dispose(&raw mut ptbuf);
    ptls_free(tls);
    return (ret != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
}
unsafe extern "C" fn run_server(
    mut sa: *mut sockaddr,
    mut salen: socklen_t,
    mut ctx: *mut ptls_context_t,
    mut input_file: *const ::core::ffi::c_char,
    mut hsprop: *mut ptls_handshake_properties_t,
    mut request_key_update: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut listen_fd: ::core::ffi::c_int = 0;
    let mut conn_fd: ::core::ffi::c_int = 0;
    let mut on: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    listen_fd = socket(
        (*sa).sa_family as ::core::ffi::c_int,
        SOCK_STREAM as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if listen_fd == -(1 as ::core::ffi::c_int) {
        perror(b"socket(2) failed\0".as_ptr() as *const ::core::ffi::c_char);
        return 1 as ::core::ffi::c_int;
    }
    if setsockopt(
        listen_fd,
        SOL_SOCKET,
        SO_REUSEADDR,
        &raw mut on as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    ) != 0 as ::core::ffi::c_int
    {
        perror(b"setsockopt(SO_REUSEADDR) failed\0".as_ptr() as *const ::core::ffi::c_char);
        return 1 as ::core::ffi::c_int;
    }
    if bind(listen_fd, __CONST_SOCKADDR_ARG { __sockaddr__: sa }, salen) != 0 as ::core::ffi::c_int
    {
        perror(b"bind(2) failed\0".as_ptr() as *const ::core::ffi::c_char);
        return 1 as ::core::ffi::c_int;
    }
    if listen(listen_fd, SOMAXCONN) != 0 as ::core::ffi::c_int {
        perror(b"listen(2) failed\0".as_ptr() as *const ::core::ffi::c_char);
        return 1 as ::core::ffi::c_int;
    }
    fprintf(
        stderr,
        b"server started on port %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        __bswap_16((*(sa as *mut sockaddr_in)).sin_port as __uint16_t) as ::core::ffi::c_int,
    );
    loop {
        fprintf(
            stderr,
            b"waiting for connections\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        conn_fd = accept(
            listen_fd,
            __SOCKADDR_ARG {
                __sockaddr__: ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut sockaddr,
            },
            ::core::ptr::null_mut::<socklen_t>(),
        );
        if conn_fd != -(1 as ::core::ffi::c_int) {
            handle_connection(
                conn_fd,
                ctx,
                ::core::ptr::null::<::core::ffi::c_char>(),
                input_file,
                hsprop,
                request_key_update,
                0 as ::core::ffi::c_int,
            );
        }
    }
}
unsafe extern "C" fn run_client(
    mut sa: *mut sockaddr,
    mut salen: socklen_t,
    mut ctx: *mut ptls_context_t,
    mut server_name: *const ::core::ffi::c_char,
    mut input_file: *const ::core::ffi::c_char,
    mut hsprop: *mut ptls_handshake_properties_t,
    mut request_key_update: ::core::ffi::c_int,
    mut keep_sender_open: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = 0;
    fd = socket(
        (*sa).sa_family as ::core::ffi::c_int,
        SOCK_STREAM as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if fd == 1 as ::core::ffi::c_int {
        perror(b"socket(2) failed\0".as_ptr() as *const ::core::ffi::c_char);
        return 1 as ::core::ffi::c_int;
    }
    if connect(fd, __CONST_SOCKADDR_ARG { __sockaddr__: sa }, salen) != 0 as ::core::ffi::c_int {
        perror(b"connect(2) failed\0".as_ptr() as *const ::core::ffi::c_char);
        return 1 as ::core::ffi::c_int;
    }
    let mut ret: ::core::ffi::c_int = handle_connection(
        fd,
        ctx,
        server_name,
        input_file,
        hsprop,
        request_key_update,
        keep_sender_open,
    );
    return ret;
}
unsafe extern "C" fn usage(mut cmd: *const ::core::ffi::c_char) {
    printf(
        b"Usage: %s [options] host port\n\nOptions:\n  -4                   force IPv4\n  -6                   force IPv6\n  -a                   require client authentication\n  -b                   enable brotli compression\n  -B                   benchmark mode for measuring sustained bandwidth. Run\n                       both endpoints with this option for some time, then kill\n                       the client. Server will report the ingress bandwidth.\n  -C certificate-file  certificate chain used for client authentication\n  -c certificate-file  certificate chain used for server authentication\n  -i file              a file to read from and send to the peer (default: stdin)\n  -I                   keep send side open after sending all data (client-only)\n  -j log-file          file to log probe events in JSON-Lines\n  -k key-file          specifies the credentials for signing the certificate\n  -K key-file          ECH private key for each ECH config provided by -E\n  -l log-file          file to log events (incl. traffic secrets)\n  -n                   negotiates the key exchange method (i.e. wait for HRR)\n  -N named-group       named group to be used (default: secp256r1); if \"null\"\n                       is specified alongside `-p`, external PSK handshake with\n                       no ECDHE is performed\n  -s session-file      file to read/write the session ticket\n  -S                   require public key exchange when resuming a session\n  -E echconfiglist     file that contains ECHConfigList or an empty file to\n                       grease ECH; will be overwritten when receiving\n                       retry_configs from the server\n  -e                   when resuming a session, send first 8,192 bytes of input\n                       as early data\n  -r public-key-file   use raw public keys (RFC 7250). When set and running as a\n                       client, the argument specifies the public keys that the\n                       server is expected to use. When running as a server, the\n                       argument is ignored.\n  -p psk-identity      name of the PSK key; if set, -c and -C specify the\n                       pre-shared secret\n  -P psk-hash          hash function associated to the PSK (default: sha256)\n  -u                   update the traffic key when handshake is complete\n  -v                   verify peer using the default certificates\n  -V CA-root-file      verify peer using the CA Root File\n  -y cipher-suite      cipher-suite to be used\n  -h                   print this help\n\nSupported named groups: secp256r1, secp384r1, secp521r1, X25519\nSupported signature algorithms: rsa, secp256r1, secp384r1, secp521r1, ed25519\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        cmd,
    );
    printf(b"Supported cipher suites:\0".as_ptr() as *const ::core::ffi::c_char);
    let mut i: size_t = 0 as size_t;
    while !(*(&raw mut ptls_openssl_cipher_suites_all as *mut *const ptls_cipher_suite_t)
        .offset(i as isize))
    .is_null()
    {
        if i != 0 as size_t {
            printf(b",\0".as_ptr() as *const ::core::ffi::c_char);
        }
        printf(
            b" %s\0".as_ptr() as *const ::core::ffi::c_char,
            (**(&raw mut ptls_openssl_cipher_suites_all as *mut *const ptls_cipher_suite_t)
                .offset(i as isize))
            .name,
        );
        i = i.wrapping_add(1);
    }
    printf(b"\n\n\0".as_ptr() as *const ::core::ffi::c_char);
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    OPENSSL_init_crypto(
        OPENSSL_INIT_LOAD_CRYPTO_STRINGS as uint64_t,
        ::core::ptr::null::<OPENSSL_INIT_SETTINGS>(),
    );
    OPENSSL_init_crypto(
        (OPENSSL_INIT_ADD_ALL_CIPHERS | OPENSSL_INIT_ADD_ALL_DIGESTS) as uint64_t,
        ::core::ptr::null::<OPENSSL_INIT_SETTINGS>(),
    );
    ENGINE_load_builtin_engines();
    ENGINE_register_all_ciphers();
    ENGINE_register_all_digests();
    __res_init();
    let mut key_exchanges: [*const ptls_key_exchange_algorithm_t; 128] = [
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
    ];
    let mut cipher_suites: [*const ptls_cipher_suite_t; 128] = [
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
        ::core::ptr::null::<ptls_cipher_suite_t>(),
    ];
    let mut ctx: ptls_context_t = {
        let mut init = st_ptls_context_t {
            require_dhe_on_psk_use_exporter_send_change_cipher_spec_require_client_authentication_omit_end_of_early_data_use_raw_public_keys_server_cipher_preference_server_cipher_chacha_priority: [0; 1],
            c2rust_padding: [0; 7],
            random_bytes: Some(
                ptls_openssl_random_bytes
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> (),
            ),
            get_time: &raw mut ptls_get_time,
            key_exchanges: &raw mut key_exchanges
                as *mut *const ptls_key_exchange_algorithm_t,
            cipher_suites: &raw mut cipher_suites as *mut *const ptls_cipher_suite_t,
            certificates: C2Rust_Unnamed_14 {
                list: ::core::ptr::null_mut::<ptls_iovec_t>(),
                count: 0,
            },
            pre_shared_key: C2Rust_Unnamed_13 {
                identity: st_ptls_iovec_t {
                    base: ::core::ptr::null_mut::<uint8_t>(),
                    len: 0,
                },
                secret: st_ptls_iovec_t {
                    base: ::core::ptr::null_mut::<uint8_t>(),
                    len: 0,
                },
                hash: ::core::ptr::null::<ptls_hash_algorithm_t>(),
            },
            ech: C2Rust_Unnamed_10 {
                client: C2Rust_Unnamed_12 {
                    ciphers: &raw mut ptls_openssl_hpke_cipher_suites
                        as *mut *const ptls_hpke_cipher_suite_t,
                    kems: &raw mut ptls_openssl_hpke_kems as *mut *const ptls_hpke_kem_t,
                },
                server: C2Rust_Unnamed_11 {
                    create_opener: ::core::ptr::null_mut::<ptls_ech_create_opener_t>(),
                    retry_configs: st_ptls_iovec_t {
                        base: ::core::ptr::null_mut::<uint8_t>(),
                        len: 0,
                    },
                },
            },
            on_client_hello: ::core::ptr::null_mut::<ptls_on_client_hello_t>(),
            emit_certificate: ::core::ptr::null_mut::<ptls_emit_certificate_t>(),
            sign_certificate: ::core::ptr::null_mut::<ptls_sign_certificate_t>(),
            verify_certificate: ::core::ptr::null_mut::<ptls_verify_certificate_t>(),
            ticket_lifetime: 0,
            max_early_data_size: 0,
            max_buffer_size: 0,
            hkdf_label_prefix__obsolete: ::core::ptr::null::<::core::ffi::c_char>(),
            encrypt_ticket: ::core::ptr::null_mut::<ptls_encrypt_ticket_t>(),
            save_ticket: ::core::ptr::null_mut::<ptls_save_ticket_t>(),
            log_event: ::core::ptr::null_mut::<ptls_log_event_t>(),
            update_open_count: ::core::ptr::null_mut::<ptls_update_open_count_t>(),
            update_traffic_key: ::core::ptr::null_mut::<ptls_update_traffic_key_t>(),
            decompress_certificate: ::core::ptr::null_mut::<
                ptls_decompress_certificate_t,
            >(),
            on_extension: ::core::ptr::null_mut::<ptls_on_extension_t>(),
            tls12_cipher_suites: ::core::ptr::null_mut::<*const ptls_cipher_suite_t>(),
            ticket_context: C2Rust_Unnamed_3 {
                bytes: [0; 32],
                is_set: [0; 1],
                c2rust_padding: [0; 3],
            },
            client_ca_names: C2Rust_Unnamed_2 {
                list: ::core::ptr::null::<ptls_iovec_t>(),
                count: 0,
            },
        };
        init.set_require_dhe_on_psk(0);
        init.set_use_exporter(0);
        init.set_send_change_cipher_spec(0);
        init.set_require_client_authentication(0);
        init.set_omit_end_of_early_data(0);
        init.set_use_raw_public_keys(0);
        init.set_server_cipher_preference(0);
        init.set_server_cipher_chacha_priority(0);
        init
    };
    let mut hsprop: ptls_handshake_properties_t = st_ptls_handshake_properties_t {
        c2rust_unnamed: C2Rust_Unnamed_15 {
            client: {
                let mut init = C2Rust_Unnamed_19 {
                    negotiate_before_key_exchange: [0; 1],
                    c2rust_padding: [0; 3],
                    negotiated_protocols: C2Rust_Unnamed_21 {
                        list: ::core::ptr::null::<ptls_iovec_t>(),
                        count: 0,
                    },
                    session_ticket: st_ptls_iovec_t {
                        base: ::core::ptr::null_mut::<uint8_t>(),
                        len: 0,
                    },
                    max_early_data_size: ::core::ptr::null_mut::<size_t>(),
                    early_data_acceptance: PTLS_EARLY_DATA_ACCEPTANCE_UNKNOWN,
                    ech: C2Rust_Unnamed_20 {
                        configs: st_ptls_iovec_t {
                            base: ::core::ptr::null_mut::<uint8_t>(),
                            len: 0,
                        },
                        retry_configs: ::core::ptr::null_mut::<ptls_iovec_t>(),
                    },
                };
                init.set_negotiate_before_key_exchange(0);
                init
            },
        },
        additional_extensions: ::core::ptr::null_mut::<ptls_raw_extension_t>(),
        collect_extension: None,
        collected_extensions: None,
    };
    let mut host: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut port: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut input_file: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut psk_hash: *const ::core::ffi::c_char =
        b"sha256\0".as_ptr() as *const ::core::ffi::c_char;
    let mut is_server: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut use_early_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut request_key_update: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut keep_sender_open: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ch: ::core::ffi::c_int = 0;
    let mut sa: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut salen: socklen_t = 0;
    let mut family: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut raw_pub_key_file: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    let mut cert_location: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    loop {
        ch = getopt(
            argc,
            argv,
            b"46abBC:c:i:Ij:k:nN:es:Sr:p:P:E:K:l:y:vV:h\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if !(ch != -(1 as ::core::ffi::c_int)) {
            break;
        }
        match ch {
            52 => {
                family = AF_INET;
            }
            54 => {
                family = AF_INET6;
            }
            97 => {
                ctx.set_require_client_authentication(
                    1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                );
            }
            98 => {
                ctx.decompress_certificate = &raw mut ptls_decompress_certificate;
            }
            66 => {
                input_file = &raw const input_file_is_benchmark as *const ::core::ffi::c_char;
            }
            67 | 99 => {
                if !cert_location.is_null() {
                    fprintf(
                        stderr,
                        b"-C/-c can only be specified once\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    return 1 as ::core::ffi::c_int;
                }
                cert_location = optarg;
                is_server = (ch == 'c' as i32) as ::core::ffi::c_int;
            }
            105 => {
                input_file = optarg;
            }
            73 => {
                keep_sender_open = 1 as ::core::ffi::c_int;
            }
            106 => {
                setup_ptlslog(optarg);
            }
            107 => {
                load_private_key(&raw mut ctx, optarg);
            }
            110 => {
                hsprop
                    .c2rust_unnamed
                    .client
                    .set_negotiate_before_key_exchange(
                        1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                    );
            }
            101 => {
                use_early_data = 1 as ::core::ffi::c_int;
            }
            114 => {
                raw_pub_key_file = optarg;
            }
            112 => {
                ctx.pre_shared_key.identity =
                    ptls_iovec_init(optarg as *const ::core::ffi::c_void, strlen(optarg));
            }
            80 => {
                psk_hash = optarg;
            }
            115 => {
                setup_session_file(&raw mut ctx, &raw mut hsprop, optarg);
            }
            83 => {
                ctx.set_require_dhe_on_psk(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            69 => {
                ech_setup_configs(optarg);
            }
            75 => {
                ech_setup_key(&raw mut ctx, optarg);
            }
            108 => {
                setup_log_event(&raw mut ctx, optarg);
            }
            118 => {
                setup_verify_certificate(&raw mut ctx, ::core::ptr::null::<::core::ffi::c_char>());
            }
            86 => {
                setup_verify_certificate(&raw mut ctx, optarg);
            }
            78 => {
                if strcasecmp(optarg, b"null\0".as_ptr() as *const ::core::ffi::c_char)
                    == 0 as ::core::ffi::c_int
                {
                    ctx.key_exchanges =
                        ::core::ptr::null_mut::<*const ptls_key_exchange_algorithm_t>();
                } else {
                    let mut algo: *const ptls_key_exchange_algorithm_t =
                        ::core::ptr::null::<ptls_key_exchange_algorithm_t>();
                    if algo.is_null()
                        && strcasecmp(
                            optarg,
                            b"secp256r1\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        algo = &raw const ptls_openssl_secp256r1;
                    }
                    if algo.is_null()
                        && strcasecmp(
                            optarg,
                            b"secp384r1\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        algo = &raw const ptls_openssl_secp384r1;
                    }
                    if algo.is_null()
                        && strcasecmp(
                            optarg,
                            b"secp521r1\0".as_ptr() as *const ::core::ffi::c_char,
                        ) == 0 as ::core::ffi::c_int
                    {
                        algo = &raw const ptls_openssl_secp521r1;
                    }
                    if algo.is_null()
                        && strcasecmp(optarg, b"x25519\0".as_ptr() as *const ::core::ffi::c_char)
                            == 0 as ::core::ffi::c_int
                    {
                        algo = &raw const ptls_openssl_x25519;
                    }
                    if algo.is_null() {
                        fprintf(
                            stderr,
                            b"could not find key exchange: %s\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            optarg,
                        );
                        return 1 as ::core::ffi::c_int;
                    }
                    let mut i: size_t = 0;
                    i = 0 as size_t;
                    while !key_exchanges[i as usize].is_null() {
                        i = i.wrapping_add(1);
                    }
                    let c2rust_fresh2 = i;
                    i = i.wrapping_add(1);
                    key_exchanges[c2rust_fresh2 as usize] = algo;
                }
            }
            117 => {
                request_key_update = 1 as ::core::ffi::c_int;
            }
            121 => {
                let mut added: *const ptls_cipher_suite_t =
                    ::core::ptr::null::<ptls_cipher_suite_t>();
                let mut i_0: size_t = 0 as size_t;
                while !(*(&raw mut ptls_openssl_cipher_suites_all
                    as *mut *const ptls_cipher_suite_t)
                    .offset(i_0 as isize))
                .is_null()
                {
                    if strcasecmp(
                        (**(&raw mut ptls_openssl_cipher_suites_all
                            as *mut *const ptls_cipher_suite_t)
                            .offset(i_0 as isize))
                        .name,
                        optarg,
                    ) == 0 as ::core::ffi::c_int
                    {
                        added = *(&raw mut ptls_openssl_cipher_suites_all
                            as *mut *const ptls_cipher_suite_t)
                            .offset(i_0 as isize);
                        break;
                    } else {
                        i_0 = i_0.wrapping_add(1);
                    }
                }
                if added.is_null() {
                    fprintf(
                        stderr,
                        b"unknown cipher-suite: %s, see -h for list of cipher-suites supported\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        optarg,
                    );
                    exit(1 as ::core::ffi::c_int);
                }
                let mut slot: size_t = 0;
                slot = 0 as size_t;
                while !cipher_suites[slot as usize].is_null() {
                    if (*cipher_suites[slot as usize]).id as ::core::ffi::c_int
                        == (*added).id as ::core::ffi::c_int
                    {
                        fprintf(
                            stderr,
                            b"cipher-suite %s is already in list\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*added).name,
                        );
                        exit(1 as ::core::ffi::c_int);
                    }
                    slot = slot.wrapping_add(1);
                }
                cipher_suites[slot as usize] = added;
            }
            104 => {
                usage(*argv.offset(0 as ::core::ffi::c_int as isize));
                exit(0 as ::core::ffi::c_int);
            }
            _ => {
                exit(1 as ::core::ffi::c_int);
            }
        }
    }
    argc -= optind;
    argv = argv.offset(optind as isize);
    if !raw_pub_key_file.is_null() {
        let mut is_dash: ::core::ffi::c_int = (strcmp(
            raw_pub_key_file,
            b"-\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0) as ::core::ffi::c_int;
        if is_server != 0 {
            ctx.certificates.list =
                malloc(::core::mem::size_of::<ptls_iovec_t>() as size_t) as *mut ptls_iovec_t;
            load_raw_public_key(ctx.certificates.list, cert_location);
            ctx.certificates.count = 1 as size_t;
        } else if is_dash == 0 {
            let mut raw_pub_key: ptls_iovec_t = st_ptls_iovec_t {
                base: ::core::ptr::null_mut::<uint8_t>(),
                len: 0,
            };
            let mut pubkey: *mut EVP_PKEY = ::core::ptr::null_mut::<EVP_PKEY>();
            load_raw_public_key(&raw mut raw_pub_key, raw_pub_key_file);
            pubkey = d2i_PUBKEY(
                ::core::ptr::null_mut::<*mut EVP_PKEY>(),
                &raw mut raw_pub_key.base as *mut *const ::core::ffi::c_uchar,
                raw_pub_key.len as ::core::ffi::c_long,
            );
            if pubkey.is_null() {
                fprintf(
                    stderr,
                    b"Failed to create an EVP_PKEY from the key found in %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    raw_pub_key_file,
                );
                return 1 as ::core::ffi::c_int;
            }
            setup_raw_pubkey_verify_certificate(&raw mut ctx, pubkey);
            EVP_PKEY_free(pubkey);
        }
        ctx.set_use_raw_public_keys(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    } else if !ctx.pre_shared_key.identity.base.is_null() {
        if cert_location.is_null() {
            fprintf(
                stderr,
                b"-p must be used with -C or -c\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            return 1 as ::core::ffi::c_int;
        }
        ctx.pre_shared_key.secret = load_file(cert_location);
    } else if !cert_location.is_null() {
        load_certificate_chain(&raw mut ctx, cert_location);
    }
    if (ctx.certificates.count == 0 as size_t) as ::core::ffi::c_int
        != (ctx.sign_certificate
            == ::core::ptr::null_mut::<::core::ffi::c_void>() as *mut ptls_sign_certificate_t)
            as ::core::ffi::c_int
    {
        fprintf(
            stderr,
            b"-C/-c and -k options must be used together\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    if is_server != 0 {
        if ctx.certificates.count != 0 as size_t && !ctx.decompress_certificate.is_null() {
            static mut ecc: ptls_emit_compressed_certificate_t =
                st_ptls_emit_compressed_certificate_t {
                    super_0: st_ptls_emit_certificate_t { cb: None },
                    algo: 0,
                    with_ocsp_status: st_ptls_compressed_certificate_entry_t {
                        uncompressed_length: 0,
                        bytes: st_ptls_iovec_t {
                            base: ::core::ptr::null_mut::<uint8_t>(),
                            len: 0,
                        },
                    },
                    without_ocsp_status: st_ptls_compressed_certificate_entry_t {
                        uncompressed_length: 0,
                        bytes: st_ptls_iovec_t {
                            base: ::core::ptr::null_mut::<uint8_t>(),
                            len: 0,
                        },
                    },
                };
            if ptls_init_compressed_certificate(
                &raw mut ecc,
                ctx.certificates.list,
                ctx.certificates.count,
                ptls_iovec_init(::core::ptr::null::<::core::ffi::c_void>(), 0 as size_t),
            ) != 0 as ::core::ffi::c_int
            {
                fprintf(
                    stderr,
                    b"failed to create a brotli-compressed version of the certificate chain.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
                exit(1 as ::core::ffi::c_int);
            }
            ctx.emit_certificate = &raw mut ecc.super_0;
        }
        setup_session_cache(&raw mut ctx);
    } else {
        if use_early_data != 0 {
            static mut max_early_data_size: size_t = 0;
            hsprop.c2rust_unnamed.client.max_early_data_size = &raw mut max_early_data_size;
        }
        ctx.set_send_change_cipher_spec(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        hsprop.c2rust_unnamed.client.ech.configs = ech.config_list;
        hsprop.c2rust_unnamed.client.ech.retry_configs = &raw mut ech.retry.configs;
    }
    if key_exchanges[0 as ::core::ffi::c_int as usize].is_null() {
        key_exchanges[0 as ::core::ffi::c_int as usize] = &raw const ptls_openssl_secp256r1;
    }
    if cipher_suites[0 as ::core::ffi::c_int as usize].is_null() {
        let mut i_1: size_t = 0 as size_t;
        while !(*(&raw mut ptls_openssl_cipher_suites as *mut *const ptls_cipher_suite_t)
            .offset(i_1 as isize))
        .is_null()
        {
            cipher_suites[i_1 as usize] = *(&raw mut ptls_openssl_cipher_suites
                as *mut *const ptls_cipher_suite_t)
                .offset(i_1 as isize);
            i_1 = i_1.wrapping_add(1);
        }
    }
    if !ctx.pre_shared_key.identity.base.is_null() {
        let mut i_2: size_t = 0;
        i_2 = 0 as size_t;
        while !cipher_suites[i_2 as usize].is_null() {
            if strcmp((*(*cipher_suites[i_2 as usize]).hash).name, psk_hash)
                == 0 as ::core::ffi::c_int
            {
                break;
            }
            i_2 = i_2.wrapping_add(1);
        }
        if cipher_suites[i_2 as usize].is_null() {
            fprintf(
                stderr,
                b"no compatible cipher-suite for psk hash: %s\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                psk_hash,
            );
            exit(1 as ::core::ffi::c_int);
        }
        ctx.pre_shared_key.hash = (*cipher_suites[i_2 as usize]).hash;
    }
    if argc != 2 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"missing host and port\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return 1 as ::core::ffi::c_int;
    }
    argc -= 1;
    let c2rust_fresh3 = argv;
    argv = argv.offset(1);
    host = *c2rust_fresh3;
    argc -= 1;
    let c2rust_fresh4 = argv;
    argv = argv.offset(1);
    port = *c2rust_fresh4;
    if resolve_address(
        &raw mut sa as *mut sockaddr,
        &raw mut salen,
        host,
        port,
        family,
        SOCK_STREAM as ::core::ffi::c_int,
        IPPROTO_TCP as ::core::ffi::c_int,
    ) != 0 as ::core::ffi::c_int
    {
        exit(1 as ::core::ffi::c_int);
    }
    if is_server != 0 {
        return run_server(
            &raw mut sa as *mut sockaddr,
            salen,
            &raw mut ctx,
            input_file,
            &raw mut hsprop,
            request_key_update,
        );
    } else {
        return run_client(
            &raw mut sa as *mut sockaddr,
            salen,
            &raw mut ctx,
            host,
            input_file,
            &raw mut hsprop,
            request_key_update,
            keep_sender_open,
        );
    };
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}

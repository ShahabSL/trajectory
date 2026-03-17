use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type st_ptls_iovec_t;
    pub type st_ptls_buffer_t;
    pub type st_picoquic_unified_logging_t;
    pub type st_ptls_verify_certificate_t;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn perror(__s: *const ::core::ffi::c_char);
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
    fn send(
        __fd: ::core::ffi::c_int,
        __buf: *const ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
    fn recv(
        __fd: ::core::ffi::c_int,
        __buf: *mut ::core::ffi::c_void,
        __n: size_t,
        __flags: ::core::ffi::c_int,
    ) -> ssize_t;
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
        __addr: *mut sockaddr,
        __addr_len: *mut socklen_t,
    ) -> ::core::ffi::c_int;
    fn inet_ntop(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_void,
        __buf: *mut ::core::ffi::c_char,
        __len: socklen_t,
    ) -> *const ::core::ffi::c_char;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn picoquic_current_time() -> uint64_t;
    fn picoquic_free(quic: *mut picoquic_quic_t);
    fn picoquic_set_cookie_mode(quic: *mut picoquic_quic_t, cookie_mode: ::core::ffi::c_int);
    fn picoquic_create_cnx(
        quic: *mut picoquic_quic_t,
        initial_cnx_id: picoquic_connection_id_t,
        remote_cnx_id: picoquic_connection_id_t,
        addr_to: *const sockaddr,
        start_time: uint64_t,
        preferred_version: uint32_t,
        sni: *const ::core::ffi::c_char,
        alpn: *const ::core::ffi::c_char,
        client_mode: ::core::ffi::c_char,
    ) -> *mut picoquic_cnx_t;
    fn picoquic_start_client_cnx(cnx: *mut picoquic_cnx_t) -> ::core::ffi::c_int;
    fn picoquic_close(
        cnx: *mut picoquic_cnx_t,
        application_reason_code: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_probe_new_path_ex(
        cnx: *mut picoquic_cnx_t,
        addr_peer: *const sockaddr,
        addr_local: *const sockaddr,
        if_index: ::core::ffi::c_int,
        current_time: uint64_t,
        to_preferred_address: ::core::ffi::c_int,
        path_id_p: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_get_first_cnx(quic: *mut picoquic_quic_t) -> *mut picoquic_cnx_t;
    fn picoquic_get_next_cnx(cnx: *mut picoquic_cnx_t) -> *mut picoquic_cnx_t;
    fn picoquic_get_initial_cnxid(cnx: *mut picoquic_cnx_t) -> picoquic_connection_id_t;
    fn picoquic_set_callback(
        cnx: *mut picoquic_cnx_t,
        callback_fn: picoquic_stream_data_cb_fn,
        callback_ctx: *mut ::core::ffi::c_void,
    );
    fn picoquic_unlink_app_stream_ctx(cnx: *mut picoquic_cnx_t, stream_id: uint64_t);
    fn picoquic_mark_active_stream(
        cnx: *mut picoquic_cnx_t,
        stream_id: uint64_t,
        is_active: ::core::ffi::c_int,
        v_stream_ctx: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn picoquic_set_default_priority(quic: *mut picoquic_quic_t, default_stream_priority: uint8_t);
    fn picoquic_provide_stream_data_buffer(
        context: *mut ::core::ffi::c_void,
        nb_bytes: size_t,
        is_fin: ::core::ffi::c_int,
        is_still_active: ::core::ffi::c_int,
    ) -> *mut uint8_t;
    fn picoquic_reset_stream(
        cnx: *mut picoquic_cnx_t,
        stream_id: uint64_t,
        local_stream_error: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_get_next_local_stream_id(
        cnx: *mut picoquic_cnx_t,
        is_unidir: ::core::ffi::c_int,
    ) -> uint64_t;
    fn picoquic_enable_keep_alive(cnx: *mut picoquic_cnx_t, interval: uint64_t);
    fn picoquic_disable_keep_alive(cnx: *mut picoquic_cnx_t);
    fn rand() -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
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
    fn strdup(__s: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strerror(__errnum: ::core::ffi::c_int) -> *mut ::core::ffi::c_char;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn getnameinfo(
        __sa: *const sockaddr,
        __salen: socklen_t,
        __host: *mut ::core::ffi::c_char,
        __hostlen: socklen_t,
        __serv: *mut ::core::ffi::c_char,
        __servlen: socklen_t,
        __flags: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_set_key_log_file_from_env(quic: *mut picoquic_quic_t);
    fn debug_printf(fmt: *const ::core::ffi::c_char, ...);
    static picoquic_null_connection_id: picoquic_connection_id_t;
    fn picoquic_wake_up_network_thread(
        thread_ctx: *mut picoquic_network_thread_ctx_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_open_network_wake_up(
        thread_ctx: *mut picoquic_network_thread_ctx_t,
        ret: *mut ::core::ffi::c_int,
    );
    fn picoquic_connection_disconnect(cnx: *mut picoquic_cnx_t);
    fn picoquic_reinsert_by_wake_time(
        quic: *mut picoquic_quic_t,
        cnx: *mut picoquic_cnx_t,
        next_time: uint64_t,
    );
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_exit(__retval: *mut ::core::ffi::c_void) -> !;
    fn pthread_detach(__th: pthread_t) -> ::core::ffi::c_int;
    fn pthread_setname_np(
        __target_thread: pthread_t,
        __name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn slipstream_packet_loop(
        thread_ctx: *mut picoquic_network_thread_ctx_t,
    ) -> *mut ::core::ffi::c_void;
    fn signal(__sig: ::core::ffi::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn ioctl(__fd: ::core::ffi::c_int, __request: ::core::ffi::c_ulong, ...) -> ::core::ffi::c_int;
    fn poll(
        __fds: *mut pollfd,
        __nfds: nfds_t,
        __timeout: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn b32_encode(
        dest: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_char,
        len: size_t,
        no_padding: uint32_t,
        hex: uint32_t,
    ) -> size_t;
    fn picoquic_create_and_configure(
        config: *mut picoquic_quic_config_t,
        default_callback_fn: picoquic_stream_data_cb_fn,
        default_callback_ctx: *mut ::core::ffi::c_void,
        current_time: uint64_t,
        p_simulated_time: *mut uint64_t,
    ) -> *mut picoquic_quic_t;
    fn picoquic_config_init(config: *mut picoquic_quic_config_t);
    fn slipstream_inline_dotify(
        buf: *mut ::core::ffi::c_char,
        buflen: size_t,
        len: size_t,
    ) -> size_t;
    fn print_sockaddr_ip_and_port(addr_storage: *mut sockaddr_storage);
    fn dns_encode(_: *mut dns_packet_t, _: *mut size_t, _: *const dns_query_t) -> dns_rcode_t;
    fn dns_decode(
        _: *mut dns_decoded_t,
        _: *mut size_t,
        _: *const dns_packet_t,
        _: size_t,
    ) -> dns_rcode_t;
    fn dns_rcode_text(_: dns_rcode_t) -> *const ::core::ffi::c_char;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type __sig_atomic_t = ::core::ffi::c_int;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = usize;
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
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
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
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const MSG_CMSG_CLOEXEC: C2Rust_Unnamed = 1073741824;
pub const MSG_FASTOPEN: C2Rust_Unnamed = 536870912;
pub const MSG_ZEROCOPY: C2Rust_Unnamed = 67108864;
pub const MSG_BATCH: C2Rust_Unnamed = 262144;
pub const MSG_WAITFORONE: C2Rust_Unnamed = 65536;
pub const MSG_MORE: C2Rust_Unnamed = 32768;
pub const MSG_NOSIGNAL: C2Rust_Unnamed = 16384;
pub const MSG_ERRQUEUE: C2Rust_Unnamed = 8192;
pub const MSG_RST: C2Rust_Unnamed = 4096;
pub const MSG_CONFIRM: C2Rust_Unnamed = 2048;
pub const MSG_SYN: C2Rust_Unnamed = 1024;
pub const MSG_FIN: C2Rust_Unnamed = 512;
pub const MSG_WAITALL: C2Rust_Unnamed = 256;
pub const MSG_EOR: C2Rust_Unnamed = 128;
pub const MSG_DONTWAIT: C2Rust_Unnamed = 64;
pub const MSG_TRUNC: C2Rust_Unnamed = 32;
pub const MSG_PROXY: C2Rust_Unnamed = 16;
pub const MSG_CTRUNC: C2Rust_Unnamed = 8;
pub const MSG_DONTROUTE: C2Rust_Unnamed = 4;
pub const MSG_PEEK: C2Rust_Unnamed = 2;
pub const MSG_OOB: C2Rust_Unnamed = 1;
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
pub type picoquic_state_enum = ::core::ffi::c_uint;
pub const picoquic_state_disconnected: picoquic_state_enum = 19;
pub const picoquic_state_draining: picoquic_state_enum = 18;
pub const picoquic_state_closing: picoquic_state_enum = 17;
pub const picoquic_state_closing_received: picoquic_state_enum = 16;
pub const picoquic_state_disconnecting: picoquic_state_enum = 15;
pub const picoquic_state_ready: picoquic_state_enum = 14;
pub const picoquic_state_client_ready_start: picoquic_state_enum = 13;
pub const picoquic_state_server_almost_ready: picoquic_state_enum = 12;
pub const picoquic_state_server_false_start: picoquic_state_enum = 11;
pub const picoquic_state_client_almost_ready: picoquic_state_enum = 10;
pub const picoquic_state_handshake_failure_resend: picoquic_state_enum = 9;
pub const picoquic_state_handshake_failure: picoquic_state_enum = 8;
pub const picoquic_state_client_handshake_start: picoquic_state_enum = 7;
pub const picoquic_state_server_handshake: picoquic_state_enum = 6;
pub const picoquic_state_server_init: picoquic_state_enum = 5;
pub const picoquic_state_client_init_resent: picoquic_state_enum = 4;
pub const picoquic_state_client_retry_received: picoquic_state_enum = 3;
pub const picoquic_state_client_renegotiate: picoquic_state_enum = 2;
pub const picoquic_state_client_init_sent: picoquic_state_enum = 1;
pub const picoquic_state_client_init: picoquic_state_enum = 0;
pub type picoquic_packet_context_enum = ::core::ffi::c_uint;
pub const picoquic_nb_packet_context: picoquic_packet_context_enum = 3;
pub const picoquic_packet_context_initial: picoquic_packet_context_enum = 2;
pub const picoquic_packet_context_handshake: picoquic_packet_context_enum = 1;
pub const picoquic_packet_context_application: picoquic_packet_context_enum = 0;
pub type picoquic_pmtud_policy_enum = ::core::ffi::c_uint;
pub const picoquic_pmtud_blocked: picoquic_pmtud_policy_enum = 3;
pub const picoquic_pmtud_delayed: picoquic_pmtud_policy_enum = 2;
pub const picoquic_pmtud_required: picoquic_pmtud_policy_enum = 1;
pub const picoquic_pmtud_basic: picoquic_pmtud_policy_enum = 0;
pub type picoquic_spinbit_version_enum = ::core::ffi::c_uint;
pub const picoquic_spinbit_on: picoquic_spinbit_version_enum = 3;
pub const picoquic_spinbit_null: picoquic_spinbit_version_enum = 2;
pub const picoquic_spinbit_random: picoquic_spinbit_version_enum = 1;
pub const picoquic_spinbit_basic: picoquic_spinbit_version_enum = 0;
pub type picoquic_lossbit_version_enum = ::core::ffi::c_uint;
pub const picoquic_lossbit_send_receive: picoquic_lossbit_version_enum = 2;
pub const picoquic_lossbit_send_only: picoquic_lossbit_version_enum = 1;
pub const picoquic_lossbit_none: picoquic_lossbit_version_enum = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_connection_id_t {
    pub id: [uint8_t; 20],
    pub id_len: uint8_t,
}
pub type picoquic_connection_id_t = st_picoquic_connection_id_t;
pub type ptls_iovec_t = st_ptls_iovec_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_quic_t {
    pub tls_master_ctx: *mut ::core::ffi::c_void,
    pub default_callback_fn: picoquic_stream_data_cb_fn,
    pub default_callback_ctx: *mut ::core::ffi::c_void,
    pub default_alpn: *const ::core::ffi::c_char,
    pub alpn_select_fn: picoquic_alpn_select_fn,
    pub reset_seed: [uint8_t; 16],
    pub retry_seed: [uint8_t; 64],
    pub p_simulated_time: *mut uint64_t,
    pub ticket_file_name: *const ::core::ffi::c_char,
    pub token_file_name: *const ::core::ffi::c_char,
    pub p_first_ticket: *mut picoquic_stored_ticket_t,
    pub p_first_token: *mut picoquic_stored_token_t,
    pub token_reuse_tree: picosplay_tree_t,
    pub local_cnxid_length: uint8_t,
    pub default_stream_priority: uint8_t,
    pub default_datagram_priority: uint8_t,
    pub local_cnxid_ttl: uint64_t,
    pub mtu_max: uint32_t,
    pub initial_send_mtu_ipv4: uint32_t,
    pub initial_send_mtu_ipv6: uint32_t,
    pub padding_multiple_default: uint32_t,
    pub padding_minsize_default: uint32_t,
    pub sequence_hole_pseudo_period: uint32_t,
    pub default_pmtud_policy: picoquic_pmtud_policy_enum,
    pub default_spin_policy: picoquic_spinbit_version_enum,
    pub default_lossbit_policy: picoquic_lossbit_version_enum,
    pub default_multipath_option: uint32_t,
    pub default_handshake_timeout: uint64_t,
    pub crypto_epoch_length_max: uint64_t,
    pub max_simultaneous_logs: uint32_t,
    pub current_number_of_open_logs: uint32_t,
    pub max_half_open_before_retry: uint32_t,
    pub current_number_half_open: uint32_t,
    pub current_number_connections: uint32_t,
    pub tentative_max_number_connections: uint32_t,
    pub max_number_connections: uint32_t,
    pub stateless_reset_next_time: uint64_t,
    pub stateless_reset_min_interval: uint64_t,
    pub cwin_max: uint64_t,
    #[bitfield(name = "check_token", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "force_check_token", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "provide_token", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(
        name = "unconditional_cnx_id",
        ty = "::core::ffi::c_uint",
        bits = "3..=3"
    )]
    #[bitfield(name = "client_zero_share", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "server_busy", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(
        name = "is_cert_store_not_empty",
        ty = "::core::ffi::c_uint",
        bits = "6..=6"
    )]
    #[bitfield(name = "use_long_log", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(name = "should_close_log", ty = "::core::ffi::c_uint", bits = "8..=8")]
    #[bitfield(name = "enable_sslkeylog", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(
        name = "use_unique_log_names",
        ty = "::core::ffi::c_uint",
        bits = "10..=10"
    )]
    #[bitfield(
        name = "dont_coalesce_init",
        ty = "::core::ffi::c_uint",
        bits = "11..=11"
    )]
    #[bitfield(
        name = "one_way_grease_quic_bit",
        ty = "::core::ffi::c_uint",
        bits = "12..=12"
    )]
    #[bitfield(name = "log_pn_dec", ty = "::core::ffi::c_uint", bits = "13..=13")]
    #[bitfield(name = "random_initial", ty = "::core::ffi::c_uint", bits = "14..=15")]
    #[bitfield(
        name = "packet_train_mode",
        ty = "::core::ffi::c_uint",
        bits = "16..=16"
    )]
    #[bitfield(
        name = "use_constant_challenges",
        ty = "::core::ffi::c_uint",
        bits = "17..=17"
    )]
    #[bitfield(name = "use_low_memory", ty = "::core::ffi::c_uint", bits = "18..=18")]
    #[bitfield(
        name = "is_preemptive_repeat_enabled",
        ty = "::core::ffi::c_uint",
        bits = "19..=19"
    )]
    #[bitfield(
        name = "default_send_receive_bdp_frame",
        ty = "::core::ffi::c_uint",
        bits = "20..=20"
    )]
    #[bitfield(
        name = "enforce_client_only",
        ty = "::core::ffi::c_uint",
        bits = "21..=21"
    )]
    #[bitfield(
        name = "test_large_server_flight",
        ty = "::core::ffi::c_uint",
        bits = "22..=22"
    )]
    #[bitfield(
        name = "is_port_blocking_disabled",
        ty = "::core::ffi::c_uint",
        bits = "23..=23"
    )]
    #[bitfield(
        name = "are_path_callbacks_enabled",
        ty = "::core::ffi::c_uint",
        bits = "24..=24"
    )]
    #[bitfield(
        name = "use_predictable_random",
        ty = "::core::ffi::c_uint",
        bits = "25..=25"
    )]
    pub check_token_force_check_token_provide_token_unconditional_cnx_id_client_zero_share_server_busy_is_cert_store_not_empty_use_long_log_should_close_log_enable_sslkeylog_use_unique_log_names_dont_coalesce_init_one_way_grease_quic_bit_log_pn_dec_random_initial_packet_train_mode_use_constant_challenges_use_low_memory_is_preemptive_repeat_enabled_default_send_receive_bdp_frame_enforce_client_only_test_large_server_flight_is_port_blocking_disabled_are_path_callbacks_enabled_use_predictable_random:
        [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
    pub pending_stateless_packet: *mut picoquic_stateless_packet_t,
    pub default_congestion_alg: *const picoquic_congestion_algorithm_t,
    pub wifi_shadow_rtt: uint64_t,
    pub bbr_quantum_ratio: ::core::ffi::c_double,
    pub cnx_list: *mut st_picoquic_cnx_t,
    pub cnx_last: *mut st_picoquic_cnx_t,
    pub cnx_wake_tree: picosplay_tree_t,
    pub cnx_in_progress: *mut st_picoquic_cnx_t,
    pub table_cnx_by_id: *mut picohash_table,
    pub table_cnx_by_net: *mut picohash_table,
    pub table_cnx_by_icid: *mut picohash_table,
    pub table_cnx_by_secret: *mut picohash_table,
    pub table_issued_tickets: *mut picohash_table,
    pub table_issued_tickets_first: *mut picoquic_issued_ticket_t,
    pub table_issued_tickets_last: *mut picoquic_issued_ticket_t,
    pub table_issued_tickets_nb: size_t,
    pub p_first_packet: *mut picoquic_packet_t,
    pub nb_packets_in_pool: ::core::ffi::c_int,
    pub nb_packets_allocated: ::core::ffi::c_int,
    pub nb_packets_allocated_max: ::core::ffi::c_int,
    pub p_first_data_node: *mut picoquic_stream_data_node_t,
    pub nb_data_nodes_in_pool: ::core::ffi::c_int,
    pub nb_data_nodes_allocated: ::core::ffi::c_int,
    pub nb_data_nodes_allocated_max: ::core::ffi::c_int,
    pub cnx_id_callback_fn: picoquic_connection_id_cb_fn,
    pub cnx_id_callback_ctx: *mut ::core::ffi::c_void,
    pub aead_encrypt_ticket_ctx: *mut ::core::ffi::c_void,
    pub aead_decrypt_ticket_ctx: *mut ::core::ffi::c_void,
    pub retry_integrity_sign_ctx: *mut *mut ::core::ffi::c_void,
    pub retry_integrity_verify_ctx: *mut *mut ::core::ffi::c_void,
    pub verify_certificate_callback: *mut st_ptls_verify_certificate_t,
    pub free_verify_certificate_callback_fn: picoquic_free_verify_certificate_ctx,
    pub default_tp: picoquic_tp_t,
    pub fuzz_fn: picoquic_fuzz_fn,
    pub fuzz_ctx: *mut ::core::ffi::c_void,
    pub wake_file: ::core::ffi::c_int,
    pub wake_line: ::core::ffi::c_int,
    pub max_data_limit: uint64_t,
    pub rtt_update_delta: uint64_t,
    pub pacing_rate_update_delta: uint64_t,
    pub F_log: *mut ::core::ffi::c_void,
    pub binlog_dir: *mut ::core::ffi::c_char,
    pub qlog_dir: *mut ::core::ffi::c_char,
    pub autoqlog_fn: picoquic_autoqlog_fn,
    pub text_log_fns: *mut st_picoquic_unified_logging_t,
    pub bin_log_fns: *mut st_picoquic_unified_logging_t,
    pub qlog_fns: *mut st_picoquic_unified_logging_t,
    pub perflog_fn: picoquic_performance_log_fn,
    pub v_perflog_ctx: *mut ::core::ffi::c_void,
    pub bbr_exp_flags: bbr_exp,
}
pub type bbr_exp = st_bbr_exp;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_bbr_exp {
    #[bitfield(name = "do_early_exit", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "do_rapid_start", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(
        name = "do_handle_suspension",
        ty = "::core::ffi::c_uint",
        bits = "2..=2"
    )]
    #[bitfield(name = "do_control_lost", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(
        name = "do_exit_probeBW_up_on_delay",
        ty = "::core::ffi::c_uint",
        bits = "4..=4"
    )]
    #[bitfield(
        name = "do_enter_probeBW_after_limited",
        ty = "::core::ffi::c_uint",
        bits = "5..=5"
    )]
    pub do_early_exit_do_rapid_start_do_handle_suspension_do_control_lost_do_exit_probeBW_up_on_delay_do_enter_probeBW_after_limited:
        [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type picoquic_performance_log_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_quic_t,
        *mut picoquic_cnx_t,
        ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type picoquic_cnx_t = st_picoquic_cnx_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_cnx_t {
    pub quic: *mut picoquic_quic_t,
    pub next_in_table: *mut st_picoquic_cnx_t,
    pub previous_in_table: *mut st_picoquic_cnx_t,
    pub proposed_version: uint32_t,
    pub rejected_version: uint32_t,
    pub desired_version: uint32_t,
    pub version_index: ::core::ffi::c_int,
    #[bitfield(name = "is_0RTT_accepted", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(
        name = "remote_parameters_received",
        ty = "::core::ffi::c_uint",
        bits = "1..=1"
    )]
    #[bitfield(name = "client_mode", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "key_phase_enc", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "key_phase_dec", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(
        name = "zero_rtt_data_accepted",
        ty = "::core::ffi::c_uint",
        bits = "5..=5"
    )]
    #[bitfield(name = "sending_ecn_ack", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(
        name = "sent_blocked_frame",
        ty = "::core::ffi::c_uint",
        bits = "7..=7"
    )]
    #[bitfield(
        name = "stream_blocked_bidir_sent",
        ty = "::core::ffi::c_uint",
        bits = "8..=8"
    )]
    #[bitfield(
        name = "stream_blocked_unidir_sent",
        ty = "::core::ffi::c_uint",
        bits = "9..=9"
    )]
    #[bitfield(
        name = "max_stream_data_needed",
        ty = "::core::ffi::c_uint",
        bits = "10..=10"
    )]
    #[bitfield(
        name = "path_demotion_needed",
        ty = "::core::ffi::c_uint",
        bits = "11..=11"
    )]
    #[bitfield(
        name = "alt_path_challenge_needed",
        ty = "::core::ffi::c_uint",
        bits = "12..=12"
    )]
    #[bitfield(
        name = "is_handshake_finished",
        ty = "::core::ffi::c_uint",
        bits = "13..=13"
    )]
    #[bitfield(
        name = "is_handshake_done_acked",
        ty = "::core::ffi::c_uint",
        bits = "14..=14"
    )]
    #[bitfield(
        name = "is_new_token_acked",
        ty = "::core::ffi::c_uint",
        bits = "15..=15"
    )]
    #[bitfield(
        name = "is_1rtt_received",
        ty = "::core::ffi::c_uint",
        bits = "16..=16"
    )]
    #[bitfield(name = "is_1rtt_acked", ty = "::core::ffi::c_uint", bits = "17..=17")]
    #[bitfield(
        name = "has_successful_probe",
        ty = "::core::ffi::c_uint",
        bits = "18..=18"
    )]
    #[bitfield(
        name = "grease_transport_parameters",
        ty = "::core::ffi::c_uint",
        bits = "19..=19"
    )]
    #[bitfield(
        name = "test_large_chello",
        ty = "::core::ffi::c_uint",
        bits = "20..=20"
    )]
    #[bitfield(
        name = "initial_validated",
        ty = "::core::ffi::c_uint",
        bits = "21..=21"
    )]
    #[bitfield(
        name = "initial_repeat_needed",
        ty = "::core::ffi::c_uint",
        bits = "22..=22"
    )]
    #[bitfield(
        name = "is_loss_bit_enabled_incoming",
        ty = "::core::ffi::c_uint",
        bits = "23..=23"
    )]
    #[bitfield(
        name = "is_loss_bit_enabled_outgoing",
        ty = "::core::ffi::c_uint",
        bits = "24..=24"
    )]
    #[bitfield(
        name = "is_ack_frequency_negotiated",
        ty = "::core::ffi::c_uint",
        bits = "25..=25"
    )]
    #[bitfield(
        name = "is_ack_frequency_updated",
        ty = "::core::ffi::c_uint",
        bits = "26..=26"
    )]
    #[bitfield(
        name = "recycle_sooner_needed",
        ty = "::core::ffi::c_uint",
        bits = "27..=27"
    )]
    #[bitfield(
        name = "is_time_stamp_enabled",
        ty = "::core::ffi::c_uint",
        bits = "28..=28"
    )]
    #[bitfield(
        name = "is_time_stamp_sent",
        ty = "::core::ffi::c_uint",
        bits = "29..=29"
    )]
    #[bitfield(
        name = "is_pacing_update_requested",
        ty = "::core::ffi::c_uint",
        bits = "30..=30"
    )]
    #[bitfield(
        name = "is_path_quality_update_requested",
        ty = "::core::ffi::c_uint",
        bits = "31..=31"
    )]
    #[bitfield(
        name = "is_hcid_verified",
        ty = "::core::ffi::c_uint",
        bits = "32..=32"
    )]
    #[bitfield(
        name = "do_grease_quic_bit",
        ty = "::core::ffi::c_uint",
        bits = "33..=33"
    )]
    #[bitfield(
        name = "quic_bit_greased",
        ty = "::core::ffi::c_uint",
        bits = "34..=34"
    )]
    #[bitfield(
        name = "quic_bit_received_0",
        ty = "::core::ffi::c_uint",
        bits = "35..=35"
    )]
    #[bitfield(name = "is_half_open", ty = "::core::ffi::c_uint", bits = "36..=36")]
    #[bitfield(
        name = "did_receive_short_initial",
        ty = "::core::ffi::c_uint",
        bits = "37..=37"
    )]
    #[bitfield(
        name = "ack_ignore_order_local",
        ty = "::core::ffi::c_uint",
        bits = "38..=38"
    )]
    #[bitfield(
        name = "ack_ignore_order_remote",
        ty = "::core::ffi::c_uint",
        bits = "39..=39"
    )]
    #[bitfield(
        name = "are_path_callbacks_enabled",
        ty = "::core::ffi::c_uint",
        bits = "40..=40"
    )]
    #[bitfield(
        name = "is_sending_large_buffer",
        ty = "::core::ffi::c_uint",
        bits = "41..=41"
    )]
    #[bitfield(
        name = "is_preemptive_repeat_enabled",
        ty = "::core::ffi::c_uint",
        bits = "42..=42"
    )]
    #[bitfield(
        name = "do_version_negotiation",
        ty = "::core::ffi::c_uint",
        bits = "43..=43"
    )]
    #[bitfield(
        name = "send_receive_bdp_frame",
        ty = "::core::ffi::c_uint",
        bits = "44..=44"
    )]
    #[bitfield(
        name = "cwin_notified_from_seed",
        ty = "::core::ffi::c_uint",
        bits = "45..=45"
    )]
    #[bitfield(
        name = "is_datagram_ready",
        ty = "::core::ffi::c_uint",
        bits = "46..=46"
    )]
    #[bitfield(
        name = "is_immediate_ack_required",
        ty = "::core::ffi::c_uint",
        bits = "47..=47"
    )]
    #[bitfield(
        name = "is_multipath_enabled",
        ty = "::core::ffi::c_uint",
        bits = "48..=48"
    )]
    #[bitfield(
        name = "is_lost_feedback_notification_required",
        ty = "::core::ffi::c_uint",
        bits = "49..=49"
    )]
    #[bitfield(
        name = "is_forced_probe_up_required",
        ty = "::core::ffi::c_uint",
        bits = "50..=50"
    )]
    #[bitfield(
        name = "is_address_discovery_provider",
        ty = "::core::ffi::c_uint",
        bits = "51..=51"
    )]
    #[bitfield(
        name = "is_address_discovery_receiver",
        ty = "::core::ffi::c_uint",
        bits = "52..=52"
    )]
    #[bitfield(
        name = "is_poll_requested",
        ty = "::core::ffi::c_uint",
        bits = "53..=53"
    )]
    #[bitfield(name = "no_ack_delay", ty = "::core::ffi::c_uint", bits = "54..=54")]
    pub is_0RTT_accepted_remote_parameters_received_client_mode_key_phase_enc_key_phase_dec_zero_rtt_data_accepted_sending_ecn_ack_sent_blocked_frame_stream_blocked_bidir_sent_stream_blocked_unidir_sent_max_stream_data_needed_path_demotion_needed_alt_path_challenge_needed_is_handshake_finished_is_handshake_done_acked_is_new_token_acked_is_1rtt_received_is_1rtt_acked_has_successful_probe_grease_transport_parameters_test_large_chello_initial_validated_initial_repeat_needed_is_loss_bit_enabled_incoming_is_loss_bit_enabled_outgoing_is_ack_frequency_negotiated_is_ack_frequency_updated_recycle_sooner_needed_is_time_stamp_enabled_is_time_stamp_sent_is_pacing_update_requested_is_path_quality_update_requested_is_hcid_verified_do_grease_quic_bit_quic_bit_greased_quic_bit_received_0_is_half_open_did_receive_short_initial_ack_ignore_order_local_ack_ignore_order_remote_are_path_callbacks_enabled_is_sending_large_buffer_is_preemptive_repeat_enabled_do_version_negotiation_send_receive_bdp_frame_cwin_notified_from_seed_is_datagram_ready_is_immediate_ack_required_is_multipath_enabled_is_lost_feedback_notification_required_is_forced_probe_up_required_is_address_discovery_provider_is_address_discovery_receiver_is_poll_requested_no_ack_delay:
        [u8; 7],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 1],
    pub pmtud_policy: picoquic_pmtud_policy_enum,
    pub spin_policy: picoquic_spinbit_version_enum,
    pub idle_timeout: uint64_t,
    pub local_parameters: picoquic_tp_t,
    pub remote_parameters: picoquic_tp_t,
    pub padding_multiple: uint32_t,
    pub padding_minsize: uint32_t,
    pub seed_ip_addr: [uint8_t; 16],
    pub seed_ip_addr_length: uint8_t,
    pub seed_rtt_min: uint64_t,
    pub seed_cwin: uint64_t,
    pub issued_ticket_id: uint64_t,
    pub resumed_ticket_id: uint64_t,
    pub sni: *const ::core::ffi::c_char,
    pub alpn: *const ::core::ffi::c_char,
    pub max_early_data_size: size_t,
    pub callback_fn: picoquic_stream_data_cb_fn,
    pub callback_ctx: *mut ::core::ffi::c_void,
    pub cnx_state: picoquic_state_enum,
    pub initial_cnxid: picoquic_connection_id_t,
    pub original_cnxid: picoquic_connection_id_t,
    pub registered_icid_addr: sockaddr_storage,
    pub registered_icid_item: picohash_item,
    pub registered_secret_addr: sockaddr_storage,
    pub registered_reset_secret: [uint8_t; 16],
    pub registered_reset_secret_item: picohash_item,
    pub start_time: uint64_t,
    pub phase_delay: int64_t,
    pub application_error: uint64_t,
    pub local_error: uint64_t,
    pub local_error_reason: *const ::core::ffi::c_char,
    pub remote_application_error: uint64_t,
    pub remote_error: uint64_t,
    pub offending_frame_type: uint64_t,
    pub retry_token_length: uint16_t,
    pub retry_token: *mut uint8_t,
    pub next_wake_time: uint64_t,
    pub cnx_wake_node: picosplay_node_t,
    pub app_wake_time: uint64_t,
    pub tls_ctx: *mut ::core::ffi::c_void,
    pub crypto_epoch_length_max: uint64_t,
    pub crypto_epoch_sequence: uint64_t,
    pub crypto_rotation_time_guard: uint64_t,
    pub tls_sendbuf: *mut st_ptls_buffer_t,
    pub psk_cipher_suite_id: uint16_t,
    pub tls_stream: [picoquic_stream_head_t; 4],
    pub crypto_context: [picoquic_crypto_context_t; 4],
    pub crypto_context_old: picoquic_crypto_context_t,
    pub crypto_context_new: picoquic_crypto_context_t,
    pub crypto_failure_count: uint64_t,
    pub latest_progress_time: uint64_t,
    pub latest_receive_time: uint64_t,
    pub last_close_sent: uint64_t,
    pub pkt_ctx: [picoquic_packet_context_t; 3],
    pub ack_ctx: [picoquic_ack_context_t; 3],
    pub observed_number: uint64_t,
    pub nb_bytes_queued: uint64_t,
    pub nb_zero_rtt_sent: uint32_t,
    pub nb_zero_rtt_acked: uint32_t,
    pub nb_zero_rtt_received: uint32_t,
    pub max_mtu_sent: size_t,
    pub max_mtu_received: size_t,
    pub nb_packets_received: uint64_t,
    pub nb_trains_sent: uint64_t,
    pub nb_trains_short: uint64_t,
    pub nb_trains_blocked_cwin: uint64_t,
    pub nb_trains_blocked_pacing: uint64_t,
    pub nb_trains_blocked_others: uint64_t,
    pub nb_packets_sent: uint64_t,
    pub nb_packets_logged: uint64_t,
    pub nb_retransmission_total: uint64_t,
    pub nb_preemptive_repeat: uint64_t,
    pub nb_spurious: uint64_t,
    pub nb_crypto_key_rotations: uint64_t,
    pub nb_packet_holes_inserted: uint64_t,
    pub max_ack_delay_remote: uint64_t,
    pub max_ack_gap_remote: uint64_t,
    pub max_ack_delay_local: uint64_t,
    pub max_ack_gap_local: uint64_t,
    pub min_ack_delay_remote: uint64_t,
    pub min_ack_delay_local: uint64_t,
    #[bitfield(name = "cwin_blocked", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "flow_blocked", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "stream_blocked", ty = "::core::ffi::c_uint", bits = "2..=2")]
    pub cwin_blocked_flow_blocked_stream_blocked: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
    pub congestion_alg: *const picoquic_congestion_algorithm_t,
    pub rtt_update_delta: uint64_t,
    pub pacing_rate_update_delta: uint64_t,
    pub pacing_rate_signalled: uint64_t,
    pub pacing_increase_threshold: uint64_t,
    pub pacing_decrease_threshold: uint64_t,
    pub pacing_change_threshold: uint64_t,
    pub initial_data_received: uint64_t,
    pub initial_data_sent: uint64_t,
    pub data_sent: uint64_t,
    pub data_received: uint64_t,
    pub maxdata_local: uint64_t,
    pub maxdata_local_acked: uint64_t,
    pub maxdata_remote: uint64_t,
    pub max_stream_data_local: uint64_t,
    pub max_stream_data_remote: uint64_t,
    pub max_stream_id_bidir_local: uint64_t,
    pub max_stream_id_bidir_rank_acked: uint64_t,
    pub max_stream_id_bidir_local_computed: uint64_t,
    pub max_stream_id_bidir_remote: uint64_t,
    pub max_stream_id_unidir_local: uint64_t,
    pub max_stream_id_unidir_rank_acked: uint64_t,
    pub max_stream_id_unidir_local_computed: uint64_t,
    pub max_stream_id_unidir_remote: uint64_t,
    pub first_misc_frame: *mut picoquic_misc_frame_header_t,
    pub last_misc_frame: *mut picoquic_misc_frame_header_t,
    pub stream_tree: picosplay_tree_t,
    pub first_output_stream: *mut picoquic_stream_head_t,
    pub last_output_stream: *mut picoquic_stream_head_t,
    pub high_priority_stream_id: uint64_t,
    pub next_stream_id: [uint64_t; 4],
    pub priority_limit_for_bypass: uint64_t,
    pub priority_bypass_pacing: picoquic_pacing_t,
    pub queue_data_repeat_tree: picosplay_tree_t,
    pub first_datagram: *mut picoquic_misc_frame_header_t,
    pub last_datagram: *mut picoquic_misc_frame_header_t,
    pub datagram_priority: uint64_t,
    pub datagram_conflicts_count: ::core::ffi::c_int,
    pub datagram_conflicts_max: ::core::ffi::c_int,
    pub keep_alive_interval: uint64_t,
    pub path: *mut *mut picoquic_path_t,
    pub nb_paths: ::core::ffi::c_int,
    pub nb_path_alloc: ::core::ffi::c_int,
    pub last_path_polled: ::core::ffi::c_int,
    pub unique_path_id_next: uint64_t,
    pub nominal_path_for_ack: *mut picoquic_path_t,
    pub status_sequence_to_send_next: uint64_t,
    pub max_path_id_local: uint64_t,
    pub max_path_id_acknowledged: uint64_t,
    pub max_path_id_remote: uint64_t,
    pub path_blocked_acknowledged: uint64_t,
    pub first_remote_cnxid_stash: *mut picoquic_remote_cnxid_stash_t,
    pub nb_local_cnxid_lists: uint64_t,
    pub next_path_id_in_lists: uint64_t,
    pub first_local_cnxid_list: *mut picoquic_local_cnxid_list_t,
    pub ack_frequency_sequence_local: uint64_t,
    pub ack_gap_local: uint64_t,
    pub ack_frequency_delay_local: uint64_t,
    pub ack_frequency_sequence_remote: uint64_t,
    pub ack_gap_remote: uint64_t,
    pub ack_delay_remote: uint64_t,
    pub ack_reordering_threshold_remote: uint64_t,
    pub first_sooner: *mut picoquic_stateless_packet_t,
    pub last_sooner: *mut picoquic_stateless_packet_t,
    pub log_unique: uint16_t,
    pub f_binlog: *mut FILE,
    pub binlog_file_name: *mut ::core::ffi::c_char,
}
pub type picoquic_stateless_packet_t = st_picoquic_stateless_packet_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_stateless_packet_t {
    pub next_packet: *mut st_picoquic_stateless_packet_t,
    pub addr_to: sockaddr_storage,
    pub addr_local: sockaddr_storage,
    pub if_index_local: ::core::ffi::c_int,
    pub received_ecn: ::core::ffi::c_uchar,
    pub length: size_t,
    pub receive_time: uint64_t,
    pub cnxid_log64: uint64_t,
    pub initial_cid: picoquic_connection_id_t,
    pub ptype: picoquic_packet_type_enum,
    pub bytes: [uint8_t; 1536],
}
pub type picoquic_packet_type_enum = ::core::ffi::c_uint;
pub const picoquic_packet_type_max: picoquic_packet_type_enum = 7;
pub const picoquic_packet_1rtt_protected: picoquic_packet_type_enum = 6;
pub const picoquic_packet_0rtt_protected: picoquic_packet_type_enum = 5;
pub const picoquic_packet_handshake: picoquic_packet_type_enum = 4;
pub const picoquic_packet_retry: picoquic_packet_type_enum = 3;
pub const picoquic_packet_initial: picoquic_packet_type_enum = 2;
pub const picoquic_packet_version_negotiation: picoquic_packet_type_enum = 1;
pub const picoquic_packet_error: picoquic_packet_type_enum = 0;
pub type picoquic_local_cnxid_list_t = st_picoquic_local_cnxid_list_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_local_cnxid_list_t {
    pub next_list: *mut st_picoquic_local_cnxid_list_t,
    pub unique_path_id: uint64_t,
    pub local_cnxid_sequence_next: uint64_t,
    pub local_cnxid_retire_before: uint64_t,
    pub local_cnxid_oldest_created: uint64_t,
    pub nb_local_cnxid: ::core::ffi::c_int,
    pub nb_local_cnxid_expired: ::core::ffi::c_int,
    #[bitfield(name = "is_demoted", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub is_demoted: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub demotion_time: uint64_t,
    pub local_cnxid_first: *mut picoquic_local_cnxid_t,
}
pub type picoquic_local_cnxid_t = st_picoquic_local_cnxid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_local_cnxid_t {
    pub next: *mut st_picoquic_local_cnxid_t,
    pub registered_cnx: *mut picoquic_cnx_t,
    pub hash_item: picohash_item,
    pub path_id: uint64_t,
    pub sequence: uint64_t,
    pub create_time: uint64_t,
    pub cnx_id: picoquic_connection_id_t,
    pub is_acked: ::core::ffi::c_uint,
}
pub type picohash_item = _picohash_item;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _picohash_item {
    pub hash: uint64_t,
    pub next_in_bin: *mut _picohash_item,
    pub key: *const ::core::ffi::c_void,
}
pub type picoquic_remote_cnxid_stash_t = st_picoquic_remote_cnxid_stash_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_remote_cnxid_stash_t {
    pub next_stash: *mut st_picoquic_remote_cnxid_stash_t,
    pub unique_path_id: uint64_t,
    pub retire_cnxid_before: uint64_t,
    pub cnxid_stash_first: *mut picoquic_remote_cnxid_t,
    #[bitfield(name = "is_in_use", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub is_in_use: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type picoquic_remote_cnxid_t = st_picoquic_remote_cnxid_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_remote_cnxid_t {
    pub next: *mut st_picoquic_remote_cnxid_t,
    pub sequence: uint64_t,
    pub cnx_id: picoquic_connection_id_t,
    pub reset_secret: [uint8_t; 16],
    pub nb_path_references: ::core::ffi::c_int,
    #[bitfield(name = "needs_removal", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "retire_sent", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "retire_acked", ty = "::core::ffi::c_uint", bits = "2..=2")]
    pub needs_removal_retire_sent_retire_acked: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub pkt_ctx: picoquic_packet_context_t,
}
pub type picoquic_packet_context_t = st_picoquic_packet_context_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_packet_context_t {
    pub send_sequence: uint64_t,
    pub next_sequence_hole: uint64_t,
    pub retransmit_sequence: uint64_t,
    pub highest_acknowledged: uint64_t,
    pub latest_time_acknowledged: uint64_t,
    pub highest_acknowledged_time: uint64_t,
    pub pending_last: *mut picoquic_packet_t,
    pub pending_first: *mut picoquic_packet_t,
    pub retransmitted_newest: *mut picoquic_packet_t,
    pub retransmitted_oldest: *mut picoquic_packet_t,
    pub preemptive_repeat_ptr: *mut picoquic_packet_t,
    pub retransmitted_queue_size: uint64_t,
    pub ecn_ect0_total_remote: uint64_t,
    pub ecn_ect1_total_remote: uint64_t,
    pub ecn_ce_total_remote: uint64_t,
    #[bitfield(
        name = "ack_of_ack_requested",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    pub ack_of_ack_requested: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type picoquic_packet_t = st_picoquic_packet_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_packet_t {
    pub packet_next: *mut st_picoquic_packet_t,
    pub packet_previous: *mut st_picoquic_packet_t,
    pub send_path: *mut st_picoquic_path_t,
    pub queue_data_repeat_node: picosplay_node_t,
    pub sequence_number: uint64_t,
    pub send_time: uint64_t,
    pub delivered_prior: uint64_t,
    pub delivered_time_prior: uint64_t,
    pub delivered_sent_prior: uint64_t,
    pub lost_prior: uint64_t,
    pub inflight_prior: uint64_t,
    pub data_repeat_frame: size_t,
    pub data_repeat_index: size_t,
    pub data_repeat_priority: uint64_t,
    pub data_repeat_stream_id: uint64_t,
    pub data_repeat_stream_offset: uint64_t,
    pub data_repeat_stream_data_length: size_t,
    pub length: size_t,
    pub checksum_overhead: size_t,
    pub offset: size_t,
    pub ptype: picoquic_packet_type_enum,
    pub pc: picoquic_packet_context_enum,
    #[bitfield(name = "is_evaluated", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "is_ack_eliciting", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "is_mtu_probe", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(
        name = "is_multipath_probe",
        ty = "::core::ffi::c_uint",
        bits = "3..=3"
    )]
    #[bitfield(name = "is_ack_trap", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(
        name = "delivered_app_limited",
        ty = "::core::ffi::c_uint",
        bits = "5..=5"
    )]
    #[bitfield(name = "sent_cwin_limited", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(
        name = "is_preemptive_repeat",
        ty = "::core::ffi::c_uint",
        bits = "7..=7"
    )]
    #[bitfield(
        name = "was_preemptively_repeated",
        ty = "::core::ffi::c_uint",
        bits = "8..=8"
    )]
    #[bitfield(name = "is_queued_to_path", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(
        name = "is_queued_for_retransmit",
        ty = "::core::ffi::c_uint",
        bits = "10..=10"
    )]
    #[bitfield(
        name = "is_queued_for_spurious_detection",
        ty = "::core::ffi::c_uint",
        bits = "11..=11"
    )]
    #[bitfield(
        name = "is_queued_for_data_repeat",
        ty = "::core::ffi::c_uint",
        bits = "12..=12"
    )]
    pub is_evaluated_is_ack_eliciting_is_mtu_probe_is_multipath_probe_is_ack_trap_delivered_app_limited_sent_cwin_limited_is_preemptive_repeat_was_preemptively_repeated_is_queued_to_path_is_queued_for_retransmit_is_queued_for_spurious_detection_is_queued_for_data_repeat:
        [u8; 2],
    pub bytes: [uint8_t; 1536],
}
pub type picosplay_node_t = st_picosplay_node_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picosplay_node_t {
    pub parent: *mut st_picosplay_node_t,
    pub left: *mut st_picosplay_node_t,
    pub right: *mut st_picosplay_node_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_path_t {
    pub p_local_cnxid: *mut picoquic_local_cnxid_t,
    pub p_remote_cnxid: *mut picoquic_remote_cnxid_t,
    pub registered_peer_addr: sockaddr_storage,
    pub net_id_hash_item: picohash_item,
    pub cnx: *mut st_picoquic_cnx_t,
    pub unique_path_id: uint64_t,
    pub app_path_ctx: *mut ::core::ffi::c_void,
    pub ack_ctx: picoquic_ack_context_t,
    pub pkt_ctx: picoquic_packet_context_t,
    pub peer_addr: sockaddr_storage,
    pub local_addr: sockaddr_storage,
    pub if_index_dest: ::core::ffi::c_ulong,
    pub observed_addr: sockaddr_storage,
    pub observed_address_received: uint64_t,
    #[bitfield(
        name = "observed_addr_acked",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    pub observed_addr_acked: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub nb_observed_repeat: ::core::ffi::c_int,
    pub observed_sequence_sent: uint64_t,
    pub observed_time: uint64_t,
    pub last_non_path_probing_pn: uint64_t,
    pub challenge_response: uint64_t,
    pub challenge: [uint64_t; 3],
    pub challenge_time: uint64_t,
    pub demotion_time: uint64_t,
    pub challenge_time_first: uint64_t,
    pub challenge_repeat_count: uint8_t,
    pub nat_challenge: [uint64_t; 3],
    pub nat_challenge_time: uint64_t,
    pub nat_challenge_repeat_count: uint64_t,
    pub p_remote_nat_cnxid: *mut picoquic_remote_cnxid_t,
    pub if_index_nat_dest: ::core::ffi::c_ulong,
    pub nat_peer_addr: sockaddr_storage,
    pub nat_local_addr: sockaddr_storage,
    pub last_sent_time: uint64_t,
    pub status_sequence_to_receive_next: uint64_t,
    pub status_sequence_sent_last: uint64_t,
    #[bitfield(name = "mtu_probe_sent", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "path_is_published", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(
        name = "challenge_required",
        ty = "::core::ffi::c_uint",
        bits = "2..=2"
    )]
    #[bitfield(
        name = "challenge_verified",
        ty = "::core::ffi::c_uint",
        bits = "3..=3"
    )]
    #[bitfield(name = "challenge_failed", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "response_required", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(
        name = "nat_challenge_required",
        ty = "::core::ffi::c_uint",
        bits = "6..=6"
    )]
    #[bitfield(name = "path_is_standby", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(name = "path_is_demoted", ty = "::core::ffi::c_uint", bits = "8..=8")]
    #[bitfield(
        name = "path_abandon_received",
        ty = "::core::ffi::c_uint",
        bits = "9..=9"
    )]
    #[bitfield(
        name = "path_abandon_sent",
        ty = "::core::ffi::c_uint",
        bits = "10..=10"
    )]
    #[bitfield(name = "current_spin", ty = "::core::ffi::c_uint", bits = "11..=11")]
    #[bitfield(
        name = "last_bw_estimate_path_limited",
        ty = "::core::ffi::c_uint",
        bits = "12..=12"
    )]
    #[bitfield(
        name = "path_cid_rotated",
        ty = "::core::ffi::c_uint",
        bits = "13..=13"
    )]
    #[bitfield(
        name = "path_is_preferred_path",
        ty = "::core::ffi::c_uint",
        bits = "14..=14"
    )]
    #[bitfield(
        name = "is_nat_challenge",
        ty = "::core::ffi::c_uint",
        bits = "15..=15"
    )]
    #[bitfield(
        name = "is_cc_data_updated",
        ty = "::core::ffi::c_uint",
        bits = "16..=16"
    )]
    #[bitfield(
        name = "is_multipath_probe_needed",
        ty = "::core::ffi::c_uint",
        bits = "17..=17"
    )]
    #[bitfield(
        name = "was_local_cnxid_retired",
        ty = "::core::ffi::c_uint",
        bits = "18..=18"
    )]
    #[bitfield(
        name = "is_ssthresh_initialized",
        ty = "::core::ffi::c_uint",
        bits = "19..=19"
    )]
    #[bitfield(
        name = "is_token_published",
        ty = "::core::ffi::c_uint",
        bits = "20..=20"
    )]
    #[bitfield(
        name = "is_ticket_seeded",
        ty = "::core::ffi::c_uint",
        bits = "21..=21"
    )]
    #[bitfield(name = "is_bdp_sent", ty = "::core::ffi::c_uint", bits = "22..=22")]
    #[bitfield(
        name = "is_nominal_ack_path",
        ty = "::core::ffi::c_uint",
        bits = "23..=23"
    )]
    #[bitfield(name = "is_ack_lost", ty = "::core::ffi::c_uint", bits = "24..=24")]
    #[bitfield(name = "is_ack_expected", ty = "::core::ffi::c_uint", bits = "25..=25")]
    #[bitfield(
        name = "is_datagram_ready",
        ty = "::core::ffi::c_uint",
        bits = "26..=26"
    )]
    #[bitfield(name = "is_pto_required", ty = "::core::ffi::c_uint", bits = "27..=27")]
    #[bitfield(name = "is_probing_nat", ty = "::core::ffi::c_uint", bits = "28..=28")]
    #[bitfield(
        name = "is_lost_feedback_notified",
        ty = "::core::ffi::c_uint",
        bits = "29..=29"
    )]
    #[bitfield(
        name = "is_cca_probing_up",
        ty = "::core::ffi::c_uint",
        bits = "30..=30"
    )]
    #[bitfield(
        name = "rtt_is_initialized",
        ty = "::core::ffi::c_uint",
        bits = "31..=31"
    )]
    pub mtu_probe_sent_path_is_published_challenge_required_challenge_verified_challenge_failed_response_required_nat_challenge_required_path_is_standby_path_is_demoted_path_abandon_received_path_abandon_sent_current_spin_last_bw_estimate_path_limited_path_cid_rotated_path_is_preferred_path_is_nat_challenge_is_cc_data_updated_is_multipath_probe_needed_was_local_cnxid_retired_is_ssthresh_initialized_is_token_published_is_ticket_seeded_is_bdp_sent_is_nominal_ack_path_is_ack_lost_is_ack_expected_is_datagram_ready_is_pto_required_is_probing_nat_is_lost_feedback_notified_is_cca_probing_up_rtt_is_initialized:
        [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 4],
    pub last_packet_received_at: uint64_t,
    pub last_loss_event_detected: uint64_t,
    pub nb_retransmit: uint64_t,
    pub total_bytes_lost: uint64_t,
    pub nb_losses_found: uint64_t,
    pub nb_timer_losses: uint64_t,
    pub nb_spurious: uint64_t,
    pub nb_losses_reported: uint64_t,
    pub q_square: uint64_t,
    pub max_ack_delay: uint64_t,
    pub rtt_sample: uint64_t,
    pub one_way_delay_sample: uint64_t,
    pub smoothed_rtt: uint64_t,
    pub rtt_variant: uint64_t,
    pub retransmit_timer: uint64_t,
    pub rtt_min: uint64_t,
    pub rtt_max: uint64_t,
    pub max_spurious_rtt: uint64_t,
    pub max_reorder_delay: uint64_t,
    pub max_reorder_gap: uint64_t,
    pub latest_sent_time: uint64_t,
    pub rtt_packet_previous_period: uint64_t,
    pub rtt_time_previous_period: uint64_t,
    pub nb_rtt_estimate_in_period: uint64_t,
    pub sum_rtt_estimate_in_period: uint64_t,
    pub max_rtt_estimate_in_period: uint64_t,
    pub min_rtt_estimate_in_period: uint64_t,
    pub send_mtu: size_t,
    pub send_mtu_max_tried: size_t,
    pub delivered: uint64_t,
    pub delivered_last: uint64_t,
    pub delivered_time_last: uint64_t,
    pub delivered_sent_last: uint64_t,
    pub delivered_limited_index: uint64_t,
    pub delivered_last_packet: uint64_t,
    pub bandwidth_estimate: uint64_t,
    pub bandwidth_estimate_max: uint64_t,
    pub max_sample_acked_time: uint64_t,
    pub max_sample_sent_time: uint64_t,
    pub max_sample_delivered: uint64_t,
    pub peak_bandwidth_estimate: uint64_t,
    pub bytes_sent: uint64_t,
    pub received: uint64_t,
    pub receive_rate_epoch: uint64_t,
    pub received_prior: uint64_t,
    pub receive_rate_estimate: uint64_t,
    pub receive_rate_max: uint64_t,
    pub cwin: uint64_t,
    pub bytes_in_transit: uint64_t,
    pub last_sender_limited_time: uint64_t,
    pub last_cwin_blocked_time: uint64_t,
    pub last_time_acked_data_frame_sent: uint64_t,
    pub congestion_alg_state: *mut ::core::ffi::c_void,
    pub pacing: picoquic_pacing_t,
    pub nb_mtu_losses: uint64_t,
    pub lost_after_delivered: ::core::ffi::c_int,
    pub responder: ::core::ffi::c_int,
    pub challenger: ::core::ffi::c_int,
    pub polled: ::core::ffi::c_int,
    pub paced: ::core::ffi::c_int,
    pub congested: ::core::ffi::c_int,
    pub selected: ::core::ffi::c_int,
    pub nb_delay_outliers: ::core::ffi::c_int,
    pub rtt_update_delta: uint64_t,
    pub pacing_rate_update_delta: uint64_t,
    pub rtt_threshold_low: uint64_t,
    pub rtt_threshold_high: uint64_t,
    pub pacing_rate_threshold_low: uint64_t,
    pub pacing_rate_threshold_high: uint64_t,
    pub receive_rate_threshold_low: uint64_t,
    pub receive_rate_threshold_high: uint64_t,
    pub rtt_min_remote: uint64_t,
    pub cwin_remote: uint64_t,
    pub ip_client_remote: [uint8_t; 16],
    pub ip_client_remote_length: uint8_t,
}
pub type picoquic_pacing_t = st_picoquic_pacing_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_pacing_t {
    pub rate: uint64_t,
    pub evaluation_time: uint64_t,
    pub bucket_max: int64_t,
    pub packet_time_microsec: uint64_t,
    pub quantum_max: uint64_t,
    pub rate_max: uint64_t,
    pub bandwidth_pause: ::core::ffi::c_int,
    pub bucket_nanosec: int64_t,
    pub packet_time_nanosec: int64_t,
}
pub type picoquic_ack_context_t = st_picoquic_ack_context_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_ack_context_t {
    pub sack_list: picoquic_sack_list_t,
    pub time_stamp_largest_received: uint64_t,
    pub act: [picoquic_ack_context_track_t; 2],
    pub crypto_rotation_sequence: uint64_t,
    pub ecn_ect0_total_local: uint64_t,
    pub ecn_ect1_total_local: uint64_t,
    pub ecn_ce_total_local: uint64_t,
    #[bitfield(name = "sending_ecn_ack", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub sending_ecn_ack: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type picoquic_ack_context_track_t = st_picoquic_ack_context_track_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_ack_context_track_t {
    pub highest_ack_sent: uint64_t,
    pub highest_ack_sent_time: uint64_t,
    pub time_oldest_unack_packet_received: uint64_t,
    #[bitfield(name = "ack_needed", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "ack_after_fin", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(
        name = "out_of_order_received",
        ty = "::core::ffi::c_uint",
        bits = "2..=2"
    )]
    #[bitfield(
        name = "is_immediate_ack_required",
        ty = "::core::ffi::c_uint",
        bits = "3..=3"
    )]
    pub ack_needed_ack_after_fin_out_of_order_received_is_immediate_ack_required: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type picoquic_sack_list_t = st_picoquic_sack_list_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_sack_list_t {
    pub ack_tree: picosplay_tree_t,
    pub ack_horizon: uint64_t,
    pub horizon_delay: int64_t,
    pub rc: [picoquic_sack_range_count_t; 2],
}
pub type picoquic_sack_range_count_t = st_picoquic_sack_range_count_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_sack_range_count_t {
    pub range_counts: [::core::ffi::c_int; 4],
}
pub type picosplay_tree_t = st_picosplay_tree_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picosplay_tree_t {
    pub root: *mut picosplay_node_t,
    pub comp: picosplay_comparator,
    pub create: picosplay_create,
    pub delete_node: picosplay_delete_node,
    pub node_value: picosplay_node_value,
    pub size: ::core::ffi::c_int,
}
pub type picosplay_node_value =
    Option<unsafe extern "C" fn(*mut picosplay_node_t) -> *mut ::core::ffi::c_void>;
pub type picosplay_delete_node =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut picosplay_node_t) -> ()>;
pub type picosplay_create =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut picosplay_node_t>;
pub type picosplay_comparator =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> int64_t>;
pub type picoquic_path_t = st_picoquic_path_t;
pub type picoquic_misc_frame_header_t = st_picoquic_misc_frame_header_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_misc_frame_header_t {
    pub next_misc_frame: *mut st_picoquic_misc_frame_header_t,
    pub previous_misc_frame: *mut st_picoquic_misc_frame_header_t,
    pub length: size_t,
    pub pc: picoquic_packet_context_enum,
    pub is_pure_ack: ::core::ffi::c_int,
}
pub type picoquic_stream_head_t = st_picoquic_stream_head_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_stream_head_t {
    pub stream_node: picosplay_node_t,
    pub next_output_stream: *mut st_picoquic_stream_head_t,
    pub previous_output_stream: *mut st_picoquic_stream_head_t,
    pub cnx: *mut picoquic_cnx_t,
    pub stream_id: uint64_t,
    pub affinity_path: *mut st_picoquic_path_t,
    pub consumed_offset: uint64_t,
    pub fin_offset: uint64_t,
    pub maxdata_local: uint64_t,
    pub maxdata_local_acked: uint64_t,
    pub maxdata_remote: uint64_t,
    pub local_error: uint64_t,
    pub remote_error: uint64_t,
    pub local_stop_error: uint64_t,
    pub remote_stop_error: uint64_t,
    pub last_time_data_sent: uint64_t,
    pub stream_data_tree: picosplay_tree_t,
    pub sent_offset: uint64_t,
    pub send_queue: *mut picoquic_stream_queue_node_t,
    pub app_stream_ctx: *mut ::core::ffi::c_void,
    pub direct_receive_fn: picoquic_stream_direct_receive_fn,
    pub direct_receive_ctx: *mut ::core::ffi::c_void,
    pub sack_list: picoquic_sack_list_t,
    pub stream_priority: uint8_t,
    #[bitfield(name = "is_active", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "fin_requested", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "fin_sent", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(name = "fin_received", ty = "::core::ffi::c_uint", bits = "3..=3")]
    #[bitfield(name = "fin_signalled", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "reset_requested", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "reset_sent", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "reset_acked", ty = "::core::ffi::c_uint", bits = "7..=7")]
    #[bitfield(name = "reset_received", ty = "::core::ffi::c_uint", bits = "8..=8")]
    #[bitfield(name = "reset_signalled", ty = "::core::ffi::c_uint", bits = "9..=9")]
    #[bitfield(
        name = "stop_sending_requested",
        ty = "::core::ffi::c_uint",
        bits = "10..=10"
    )]
    #[bitfield(
        name = "stop_sending_sent",
        ty = "::core::ffi::c_uint",
        bits = "11..=11"
    )]
    #[bitfield(
        name = "stop_sending_received",
        ty = "::core::ffi::c_uint",
        bits = "12..=12"
    )]
    #[bitfield(
        name = "stop_sending_signalled",
        ty = "::core::ffi::c_uint",
        bits = "13..=13"
    )]
    #[bitfield(
        name = "max_stream_updated",
        ty = "::core::ffi::c_uint",
        bits = "14..=14"
    )]
    #[bitfield(
        name = "stream_data_blocked_sent",
        ty = "::core::ffi::c_uint",
        bits = "15..=15"
    )]
    #[bitfield(
        name = "is_output_stream",
        ty = "::core::ffi::c_uint",
        bits = "16..=16"
    )]
    #[bitfield(name = "is_closed", ty = "::core::ffi::c_uint", bits = "17..=17")]
    #[bitfield(name = "is_discarded", ty = "::core::ffi::c_uint", bits = "18..=18")]
    pub is_active_fin_requested_fin_sent_fin_received_fin_signalled_reset_requested_reset_sent_reset_acked_reset_received_reset_signalled_stop_sending_requested_stop_sending_sent_stop_sending_received_stop_sending_signalled_max_stream_updated_stream_data_blocked_sent_is_output_stream_is_closed_is_discarded:
        [u8; 3],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type picoquic_stream_direct_receive_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_cnx_t,
        uint64_t,
        ::core::ffi::c_int,
        *const uint8_t,
        uint64_t,
        size_t,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type picoquic_stream_queue_node_t = st_picoquic_stream_queue_node_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_stream_queue_node_t {
    pub quic: *mut picoquic_quic_t,
    pub next_stream_data: *mut st_picoquic_stream_queue_node_t,
    pub offset: uint64_t,
    pub length: size_t,
    pub bytes: *mut uint8_t,
}
pub type picoquic_quic_t = st_picoquic_quic_t;
pub type picoquic_congestion_algorithm_t = st_picoquic_congestion_algorithm_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_congestion_algorithm_t {
    pub congestion_algorithm_id: *const ::core::ffi::c_char,
    pub congestion_algorithm_number: uint8_t,
    pub alg_init: picoquic_congestion_algorithm_init,
    pub alg_notify: picoquic_congestion_algorithm_notify,
    pub alg_delete: picoquic_congestion_algorithm_delete,
    pub alg_observe: picoquic_congestion_algorithm_observe,
}
pub type picoquic_congestion_algorithm_observe =
    Option<unsafe extern "C" fn(*mut picoquic_path_t, *mut uint64_t, *mut uint64_t) -> ()>;
pub type picoquic_congestion_algorithm_delete =
    Option<unsafe extern "C" fn(*mut picoquic_path_t) -> ()>;
pub type picoquic_congestion_algorithm_notify = Option<
    unsafe extern "C" fn(
        *mut picoquic_cnx_t,
        *mut picoquic_path_t,
        picoquic_congestion_notification_t,
        *mut picoquic_per_ack_state_t,
        uint64_t,
    ) -> (),
>;
pub type picoquic_per_ack_state_t = st_picoquic_per_ack_state_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_per_ack_state_t {
    pub rtt_measurement: uint64_t,
    pub one_way_delay: uint64_t,
    pub nb_bytes_acknowledged: uint64_t,
    pub nb_bytes_newly_lost: uint64_t,
    pub nb_bytes_lost_since_packet_sent: uint64_t,
    pub nb_bytes_delivered_since_packet_sent: uint64_t,
    pub inflight_prior: uint64_t,
    pub lost_packet_number: uint64_t,
    pub lost_packet_sent_time: uint64_t,
    #[bitfield(name = "is_app_limited", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "is_cwnd_limited", ty = "::core::ffi::c_uint", bits = "1..=1")]
    pub is_app_limited_is_cwnd_limited: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type picoquic_congestion_notification_t = ::core::ffi::c_uint;
pub const picoquic_congestion_notification_lost_feedback: picoquic_congestion_notification_t = 9;
pub const picoquic_congestion_notification_reset: picoquic_congestion_notification_t = 8;
pub const picoquic_congestion_notification_seed_cwin: picoquic_congestion_notification_t = 7;
pub const picoquic_congestion_notification_cwin_blocked: picoquic_congestion_notification_t = 6;
pub const picoquic_congestion_notification_ecn_ec: picoquic_congestion_notification_t = 5;
pub const picoquic_congestion_notification_rtt_measurement: picoquic_congestion_notification_t = 4;
pub const picoquic_congestion_notification_spurious_repeat: picoquic_congestion_notification_t = 3;
pub const picoquic_congestion_notification_timeout: picoquic_congestion_notification_t = 2;
pub const picoquic_congestion_notification_repeat: picoquic_congestion_notification_t = 1;
pub const picoquic_congestion_notification_acknowledgement: picoquic_congestion_notification_t = 0;
pub type picoquic_congestion_algorithm_init =
    Option<unsafe extern "C" fn(*mut picoquic_cnx_t, *mut picoquic_path_t, uint64_t) -> ()>;
pub type picoquic_crypto_context_t = st_picoquic_crypto_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_crypto_context_t {
    pub aead_encrypt: *mut ::core::ffi::c_void,
    pub aead_decrypt: *mut ::core::ffi::c_void,
    pub pn_enc: *mut ::core::ffi::c_void,
    pub pn_dec: *mut ::core::ffi::c_void,
}
pub type picoquic_stream_data_cb_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_cnx_t,
        uint64_t,
        *mut uint8_t,
        size_t,
        picoquic_call_back_event_t,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type picoquic_call_back_event_t = ::core::ffi::c_uint;
pub const picoquic_callback_app_wakeup: picoquic_call_back_event_t = 25;
pub const picoquic_callback_path_address_observed: picoquic_call_back_event_t = 24;
pub const picoquic_callback_path_quality_changed: picoquic_call_back_event_t = 23;
pub const picoquic_callback_path_deleted: picoquic_call_back_event_t = 22;
pub const picoquic_callback_path_suspended: picoquic_call_back_event_t = 21;
pub const picoquic_callback_path_available: picoquic_call_back_event_t = 20;
pub const picoquic_callback_datagram_spurious: picoquic_call_back_event_t = 19;
pub const picoquic_callback_datagram_lost: picoquic_call_back_event_t = 18;
pub const picoquic_callback_datagram_acked: picoquic_call_back_event_t = 17;
pub const picoquic_callback_prepare_datagram: picoquic_call_back_event_t = 16;
pub const picoquic_callback_pacing_changed: picoquic_call_back_event_t = 15;
pub const picoquic_callback_set_alpn: picoquic_call_back_event_t = 14;
pub const picoquic_callback_request_alpn_list: picoquic_call_back_event_t = 13;
pub const picoquic_callback_version_negotiation: picoquic_call_back_event_t = 12;
pub const picoquic_callback_datagram: picoquic_call_back_event_t = 11;
pub const picoquic_callback_ready: picoquic_call_back_event_t = 10;
pub const picoquic_callback_almost_ready: picoquic_call_back_event_t = 9;
pub const picoquic_callback_prepare_to_send: picoquic_call_back_event_t = 8;
pub const picoquic_callback_stream_gap: picoquic_call_back_event_t = 7;
pub const picoquic_callback_application_close: picoquic_call_back_event_t = 6;
pub const picoquic_callback_close: picoquic_call_back_event_t = 5;
pub const picoquic_callback_stateless_reset: picoquic_call_back_event_t = 4;
pub const picoquic_callback_stop_sending: picoquic_call_back_event_t = 3;
pub const picoquic_callback_stream_reset: picoquic_call_back_event_t = 2;
pub const picoquic_callback_stream_fin: picoquic_call_back_event_t = 1;
pub const picoquic_callback_stream_data: picoquic_call_back_event_t = 0;
pub type picoquic_tp_t = st_picoquic_tp_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_tp_t {
    pub initial_max_stream_data_bidi_local: uint64_t,
    pub initial_max_stream_data_bidi_remote: uint64_t,
    pub initial_max_stream_data_uni: uint64_t,
    pub initial_max_data: uint64_t,
    pub initial_max_stream_id_bidir: uint64_t,
    pub initial_max_stream_id_unidir: uint64_t,
    pub max_idle_timeout: uint64_t,
    pub max_packet_size: uint32_t,
    pub max_ack_delay: uint32_t,
    pub active_connection_id_limit: uint32_t,
    pub ack_delay_exponent: uint8_t,
    pub migration_disabled: ::core::ffi::c_uint,
    pub prefered_address: picoquic_tp_prefered_address_t,
    pub max_datagram_frame_size: uint32_t,
    pub enable_loss_bit: ::core::ffi::c_int,
    pub enable_time_stamp: ::core::ffi::c_int,
    pub min_ack_delay: uint64_t,
    pub do_grease_quic_bit: ::core::ffi::c_int,
    pub version_negotiation: picoquic_tp_version_negotiation_t,
    pub enable_bdp_frame: ::core::ffi::c_int,
    pub is_multipath_enabled: ::core::ffi::c_int,
    pub initial_max_path_id: uint64_t,
    pub address_discovery_mode: ::core::ffi::c_int,
}
pub type picoquic_tp_version_negotiation_t = st_picoquic_tp_version_negotiation_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_tp_version_negotiation_t {
    pub current: uint32_t,
    pub previous: uint32_t,
    pub nb_received: size_t,
    pub received: *mut uint32_t,
    pub nb_supported: size_t,
    pub supported: *mut uint32_t,
}
pub type picoquic_tp_prefered_address_t = st_picoquic_tp_prefered_address_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_tp_prefered_address_t {
    pub is_defined: ::core::ffi::c_int,
    pub ipv4Address: [uint8_t; 4],
    pub ipv4Port: uint16_t,
    pub ipv6Address: [uint8_t; 16],
    pub ipv6Port: uint16_t,
    pub connection_id: picoquic_connection_id_t,
    pub statelessResetToken: [uint8_t; 16],
}
pub type picoquic_autoqlog_fn =
    Option<unsafe extern "C" fn(*mut picoquic_cnx_t) -> ::core::ffi::c_int>;
pub type picoquic_fuzz_fn = Option<
    unsafe extern "C" fn(
        *mut ::core::ffi::c_void,
        *mut picoquic_cnx_t,
        *mut uint8_t,
        size_t,
        size_t,
        size_t,
    ) -> uint32_t,
>;
pub type picoquic_free_verify_certificate_ctx =
    Option<unsafe extern "C" fn(*mut ptls_verify_certificate_t) -> ()>;
pub type ptls_verify_certificate_t = st_ptls_verify_certificate_t;
pub type picoquic_connection_id_cb_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_quic_t,
        picoquic_connection_id_t,
        picoquic_connection_id_t,
        *mut ::core::ffi::c_void,
        *mut picoquic_connection_id_t,
    ) -> (),
>;
pub type picoquic_stream_data_node_t = st_picoquic_stream_data_node_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_stream_data_node_t {
    pub stream_data_node: picosplay_node_t,
    pub quic: *mut picoquic_quic_t,
    pub next_stream_data: *mut st_picoquic_stream_data_node_t,
    pub offset: uint64_t,
    pub length: size_t,
    pub bytes: *const uint8_t,
    pub data: [uint8_t; 1536],
}
pub type picoquic_issued_ticket_t = st_picoquic_issued_ticket_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_issued_ticket_t {
    pub next_ticket: *mut st_picoquic_issued_ticket_t,
    pub previous_ticket: *mut st_picoquic_issued_ticket_t,
    pub hash_item: picohash_item,
    pub ticket_id: uint64_t,
    pub creation_time: uint64_t,
    pub rtt: uint64_t,
    pub cwin: uint64_t,
    pub ip_addr: [uint8_t; 16],
    pub ip_addr_length: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct picohash_table {
    pub hash_bin: *mut *mut picohash_item,
    pub nb_bin: size_t,
    pub count: size_t,
    pub picohash_hash: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> uint64_t>,
    pub picohash_compare: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub picohash_key_to_item:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> *mut picohash_item>,
}
pub type picoquic_stored_token_t = st_picoquic_stored_token_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_stored_token_t {
    pub next_token: *mut st_picoquic_stored_token_t,
    pub sni: *const ::core::ffi::c_char,
    pub token: *const uint8_t,
    pub ip_addr: *const uint8_t,
    pub time_valid_until: uint64_t,
    pub sni_length: uint16_t,
    pub token_length: uint16_t,
    pub ip_addr_length: uint8_t,
    #[bitfield(name = "was_used", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub was_used: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 2],
}
pub type picoquic_stored_ticket_t = st_picoquic_stored_ticket_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_stored_ticket_t {
    pub next_ticket: *mut st_picoquic_stored_ticket_t,
    pub sni: *mut ::core::ffi::c_char,
    pub alpn: *mut ::core::ffi::c_char,
    pub ip_addr: *mut uint8_t,
    pub tp_0rtt: [uint64_t; 10],
    pub ticket: *mut uint8_t,
    pub time_valid_until: uint64_t,
    pub sni_length: uint16_t,
    pub alpn_length: uint16_t,
    pub version: uint32_t,
    pub ticket_length: uint16_t,
    pub ip_addr_length: uint8_t,
    pub ip_addr_client_length: uint8_t,
    pub ip_addr_client: *mut uint8_t,
    #[bitfield(name = "was_used", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub was_used: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type picoquic_alpn_select_fn =
    Option<unsafe extern "C" fn(*mut picoquic_quic_t, *mut ptls_iovec_t, size_t) -> size_t>;
pub type picoquic_packet_loop_cb_enum = ::core::ffi::c_uint;
pub const picoquic_packet_loop_alt_port: picoquic_packet_loop_cb_enum = 8;
pub const picoquic_packet_loop_wake_up: picoquic_packet_loop_cb_enum = 7;
pub const picoquic_packet_loop_system_call_duration: picoquic_packet_loop_cb_enum = 6;
pub const picoquic_packet_loop_time_check: picoquic_packet_loop_cb_enum = 5;
pub const picoquic_packet_loop_port_update: picoquic_packet_loop_cb_enum = 4;
pub const picoquic_packet_loop_after_send: picoquic_packet_loop_cb_enum = 3;
pub const picoquic_packet_loop_after_receive: picoquic_packet_loop_cb_enum = 2;
pub const picoquic_packet_loop_before_select: picoquic_packet_loop_cb_enum = 1;
pub const picoquic_packet_loop_ready: picoquic_packet_loop_cb_enum = 0;
pub type picoquic_packet_loop_cb_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_quic_t,
        picoquic_packet_loop_cb_enum,
        *mut ::core::ffi::c_void,
        *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_packet_loop_param_t {
    pub local_port: uint16_t,
    pub local_af: ::core::ffi::c_int,
    pub dest_if: ::core::ffi::c_int,
    pub socket_buffer_size: ::core::ffi::c_int,
    pub do_not_use_gso: ::core::ffi::c_int,
    pub extra_socket_required: ::core::ffi::c_int,
    pub simulate_eio: ::core::ffi::c_int,
    pub send_length_max: size_t,
    pub is_client: ::core::ffi::c_int,
    pub decode: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut *mut ::core::ffi::c_uchar,
            *const ::core::ffi::c_uchar,
            size_t,
            *mut sockaddr_storage,
            *mut sockaddr_storage,
        ) -> ssize_t,
    >,
    pub encode: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            *mut ::core::ffi::c_void,
            *mut *mut ::core::ffi::c_uchar,
            *const ::core::ffi::c_uchar,
            size_t,
            *mut size_t,
            *mut sockaddr_storage,
            *mut sockaddr_storage,
        ) -> ssize_t,
    >,
    pub delay_max: int64_t,
}
pub type picoquic_packet_loop_param_t = st_picoquic_packet_loop_param_t;
pub type picoquic_custom_thread_setname_fn =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char) -> ()>;
pub type picoquic_custom_thread_delete_fn =
    Option<unsafe extern "C" fn(*mut *mut ::core::ffi::c_void) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_network_thread_ctx_t {
    pub quic: *mut picoquic_quic_t,
    pub param: *mut picoquic_packet_loop_param_t,
    pub loop_callback: picoquic_packet_loop_cb_fn,
    pub thread_delete_fn: picoquic_custom_thread_delete_fn,
    pub thread_setname_fn: picoquic_custom_thread_setname_fn,
    pub thread_name: *const ::core::ffi::c_char,
    pub pthread: *mut ::core::ffi::c_void,
    pub loop_callback_ctx: *mut ::core::ffi::c_void,
    pub wake_up_pipe_fd: [::core::ffi::c_int; 2],
    pub is_threaded: ::core::ffi::c_int,
    pub wake_up_defined: ::core::ffi::c_int,
    pub thread_is_ready: ::core::ffi::c_int,
    pub thread_should_close: ::core::ffi::c_int,
    pub thread_is_closed: ::core::ffi::c_int,
    pub return_code: ::core::ffi::c_int,
}
pub type picoquic_network_thread_ctx_t = st_picoquic_network_thread_ctx_t;
pub type sig_atomic_t = __sig_atomic_t;
pub type __sighandler_t = Option<unsafe extern "C" fn(::core::ffi::c_int) -> ()>;
pub type nfds_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pollfd {
    pub fd: ::core::ffi::c_int,
    pub events: ::core::ffi::c_short,
    pub revents: ::core::ffi::c_short,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_quic_config_t {
    pub nb_connections: uint32_t,
    pub solution_dir: *const ::core::ffi::c_char,
    pub server_cert_file: *const ::core::ffi::c_char,
    pub server_key_file: *const ::core::ffi::c_char,
    pub log_file: *const ::core::ffi::c_char,
    pub bin_dir: *const ::core::ffi::c_char,
    pub qlog_dir: *const ::core::ffi::c_char,
    pub performance_log: *const ::core::ffi::c_char,
    pub server_port: ::core::ffi::c_int,
    pub dest_if: ::core::ffi::c_int,
    pub mtu_max: ::core::ffi::c_int,
    pub initial_send_mtu_ipv4: ::core::ffi::c_int,
    pub initial_send_mtu_ipv6: ::core::ffi::c_int,
    pub cnx_id_length: ::core::ffi::c_int,
    pub idle_timeout: ::core::ffi::c_int,
    pub socket_buffer_size: ::core::ffi::c_int,
    pub cc_algo_id: *const ::core::ffi::c_char,
    pub cnx_id_cbdata: *const ::core::ffi::c_char,
    pub spinbit_policy: picoquic_spinbit_version_enum,
    pub lossbit_policy: picoquic_lossbit_version_enum,
    pub multipath_option: ::core::ffi::c_int,
    pub multipath_alt_config: *mut ::core::ffi::c_char,
    pub bdp_frame_option: ::core::ffi::c_int,
    pub cwin_max: uint64_t,
    pub address_discovery_mode: ::core::ffi::c_int,
    pub initial_random: ::core::ffi::c_uint,
    #[bitfield(name = "use_long_log", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(
        name = "do_preemptive_repeat",
        ty = "::core::ffi::c_uint",
        bits = "1..=1"
    )]
    #[bitfield(name = "do_not_use_gso", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(
        name = "disable_port_blocking",
        ty = "::core::ffi::c_uint",
        bits = "3..=3"
    )]
    #[bitfield(name = "enable_sslkeylog", ty = "::core::ffi::c_uint", bits = "4..=4")]
    pub use_long_log_do_preemptive_repeat_do_not_use_gso_disable_port_blocking_enable_sslkeylog:
        [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub www_dir: *const ::core::ffi::c_char,
    pub reset_seed: [uint8_t; 16],
    pub ticket_encryption_key: *const uint8_t,
    pub ticket_encryption_key_length: size_t,
    #[bitfield(name = "do_retry", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "has_reset_seed", ty = "::core::ffi::c_uint", bits = "1..=1")]
    pub do_retry_has_reset_seed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
    pub ticket_file_name: *const ::core::ffi::c_char,
    pub token_file_name: *const ::core::ffi::c_char,
    pub sni: *const ::core::ffi::c_char,
    pub alpn: *const ::core::ffi::c_char,
    pub out_dir: *const ::core::ffi::c_char,
    pub root_trust_file: *const ::core::ffi::c_char,
    pub cipher_suite_id: ::core::ffi::c_int,
    pub proposed_version: uint32_t,
    pub desired_version: uint32_t,
    #[bitfield(name = "force_zero_share", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "no_disk", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(
        name = "large_client_hello",
        ty = "::core::ffi::c_uint",
        bits = "2..=2"
    )]
    pub force_zero_share_no_disk_large_client_hello: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 3],
}
pub type picoquic_quic_config_t = st_picoquic_quic_config_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_address_t {
    pub server_address: sockaddr_storage,
    pub added: bool,
}
pub type address_t = st_address_t;
pub type slipstream_client_ctx_t = st_slipstream_client_ctx_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_slipstream_client_ctx_t {
    pub cnx: *mut picoquic_cnx_t,
    pub first_stream: *mut slipstream_client_stream_ctx_t,
    pub thread_ctx: *mut picoquic_network_thread_ctx_t,
    pub server_addresses: *mut st_address_t,
    pub server_address_count: size_t,
    pub ready: bool,
    pub closed: bool,
    pub listen_sock: ::core::ffi::c_int,
}
pub type slipstream_client_stream_ctx_t = st_slipstream_client_stream_ctx_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_slipstream_client_stream_ctx_t {
    pub next_stream: *mut st_slipstream_client_stream_ctx_t,
    pub previous_stream: *mut st_slipstream_client_stream_ctx_t,
    pub fd: ::core::ffi::c_int,
    pub stream_id: uint64_t,
    pub set_active: sig_atomic_t,
}
pub type slipstream_client_poller_args = st_slipstream_client_poller_args;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_slipstream_client_poller_args {
    pub fd: ::core::ffi::c_int,
    pub cnx: *mut picoquic_cnx_t,
    pub client_ctx: *mut slipstream_client_ctx_t,
    pub stream_ctx: *mut slipstream_client_stream_ctx_t,
}
pub type slipstream_client_accepter_args = st_slipstream_client_accepter_args;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_slipstream_client_accepter_args {
    pub fd: ::core::ffi::c_int,
    pub cnx: *mut picoquic_cnx_t,
    pub client_ctx: *mut slipstream_client_ctx_t,
    pub stream_ctx: *mut slipstream_client_stream_ctx_t,
    pub thread_ctx: *mut picoquic_network_thread_ctx_t,
}
pub type dns_packet_t = uintptr_t;
pub type dns_rcode_t = dns_rcode;
pub type dns_rcode = ::core::ffi::c_uint;
pub const RCODE_BAD_STRING: dns_rcode = 3843;
pub const RCODE_NO_MEMORY: dns_rcode = 3842;
pub const RCODE_PRIVATE: dns_rcode = 3841;
pub const RCODE_BADCOOKIE: dns_rcode = 23;
pub const RCODE_BADTRUC: dns_rcode = 22;
pub const RCODE_BADALG: dns_rcode = 21;
pub const RCODE_BADNAME: dns_rcode = 20;
pub const RCODE_BADMODE: dns_rcode = 19;
pub const RCODE_BADTIME: dns_rcode = 18;
pub const RCODE_BADKEY: dns_rcode = 17;
pub const RCODE_BADSIG: dns_rcode = 16;
pub const RCODE_BADVERS: dns_rcode = 16;
pub const RCODE_NOTZONE: dns_rcode = 10;
pub const RCODE_NOTAUTH: dns_rcode = 9;
pub const RCODE_NXRRSET: dns_rcode = 8;
pub const RCODE_YXRRSET: dns_rcode = 7;
pub const RCODE_YXDOMAIN: dns_rcode = 6;
pub const RCODE_REFUSED: dns_rcode = 5;
pub const RCODE_NOT_IMPLEMENTED: dns_rcode = 4;
pub const RCODE_NAME_ERROR: dns_rcode = 3;
pub const RCODE_SERVER_FAILURE: dns_rcode = 2;
pub const RCODE_FORMAT_ERROR: dns_rcode = 1;
pub const RCODE_OKAY: dns_rcode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_query_t {
    pub id: ::core::ffi::c_int,
    pub query: bool,
    pub opcode: dns_op_t,
    pub aa: bool,
    pub tc: bool,
    pub rd: bool,
    pub ra: bool,
    pub z: bool,
    pub ad: bool,
    pub cd: bool,
    pub rcode: dns_rcode_t,
    pub qdcount: size_t,
    pub ancount: size_t,
    pub nscount: size_t,
    pub arcount: size_t,
    pub questions: *mut dns_question_t,
    pub answers: *mut dns_answer_t,
    pub nameservers: *mut dns_answer_t,
    pub additional: *mut dns_answer_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union dns_answer_t {
    pub generic: dns_generic_t,
    pub x: dns_x_t,
    pub a: dns_a_t,
    pub ns: dns_ns_t,
    pub md: dns_md_t,
    pub mf: dns_mf_t,
    pub cname: dns_cname_t,
    pub soa: dns_soa_t,
    pub mb: dns_mb_t,
    pub mg: dns_mg_t,
    pub mr: dns_mr_t,
    pub null: dns_null_t,
    pub wks: dns_wks_t,
    pub ptr: dns_ptr_t,
    pub hinfo: dns_hinfo_t,
    pub minfo: dns_minfo_t,
    pub mx: dns_mx_t,
    pub txt: dns_txt_t,
    pub rp: dns_rp_t,
    pub afsdb: dns_afsdb_t,
    pub x25: dns_x25_t,
    pub isdn: dns_isdn_t,
    pub rt: dns_rt_t,
    pub nsap: dns_nsap_t,
    pub nsap_ptr: dns_nsap_ptr_t,
    pub sig: dns_sig_t,
    pub key: dns_key_t,
    pub px: dns_px_t,
    pub gpos: dns_gpos_t,
    pub aaaa: dns_aaaa_t,
    pub loc: dns_loc_t,
    pub nxt: dns_nxt_t,
    pub eid: dns_eid_t,
    pub nimloc: dns_nimloc_t,
    pub srv: dns_srv_t,
    pub atm: dns_atm_t,
    pub naptr: dns_naptr_t,
    pub kx: dns_kx_t,
    pub cert: dns_cert_t,
    pub a6: dns_a6_t,
    pub dname: dns_dname_t,
    pub sink: dns_sink_t,
    pub opt: dns_edns0opt_t,
    pub apl: dns_apl_t,
    pub ds: dns_ds_t,
    pub rrsig: dns_rrsig_t,
    pub nsec: dns_nsec_t,
    pub dnskey: dns_dnskey_t,
    pub spf: dns_spf_t,
    pub tsig: dns_tsig_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_tsig_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub algorithm: *const ::core::ffi::c_char,
    pub timesigned: uint64_t,
    pub fudge: ::core::ffi::c_uint,
    pub MACsize: size_t,
    pub MAC: *mut uint8_t,
    pub id: ::core::ffi::c_int,
    pub error: ::core::ffi::c_int,
    pub lenother: size_t,
    pub other: *mut uint8_t,
}
pub type TTL = uint32_t;
pub type dns_class_t = dns_class;
pub type dns_class = ::core::ffi::c_uint;
pub const CLASS_UNKNOWN: dns_class = 65535;
pub const CLASS_PRIVATE: dns_class = 65280;
pub const CLASS_ANY: dns_class = 255;
pub const CLASS_NONE: dns_class = 254;
pub const CLASS_HS: dns_class = 4;
pub const CLASS_CH: dns_class = 3;
pub const CLASS_CS: dns_class = 2;
pub const CLASS_IN: dns_class = 1;
pub type dns_type_t = dns_type;
pub type dns_type = ::core::ffi::c_uint;
pub const RR_UNKNOWN: dns_type = 65535;
pub const RR_PRIVATE: dns_type = 65280;
pub const RR_DLV: dns_type = 32769;
pub const RR_TA: dns_type = 32768;
pub const RR_AMTRELAY: dns_type = 260;
pub const RR_DOA: dns_type = 259;
pub const RR_AVC: dns_type = 258;
pub const RR_CAA: dns_type = 257;
pub const RR_URI: dns_type = 256;
pub const RR_ANY: dns_type = 255;
pub const RR_MAILA: dns_type = 254;
pub const RR_MAILB: dns_type = 253;
pub const RR_AXFR: dns_type = 252;
pub const RR_IXFR: dns_type = 251;
pub const RR_TSIG: dns_type = 250;
pub const RR_TKEY: dns_type = 249;
pub const RR_EUI64: dns_type = 109;
pub const RR_EUI48: dns_type = 108;
pub const RR_LP: dns_type = 107;
pub const RR_L64: dns_type = 106;
pub const RR_L32: dns_type = 105;
pub const RR_NID: dns_type = 104;
pub const RR_UNSPEC: dns_type = 103;
pub const RR_GID: dns_type = 102;
pub const RR_UID: dns_type = 101;
pub const RR_UINFO: dns_type = 100;
pub const RR_SPF: dns_type = 99;
pub const RR_HTTPS: dns_type = 65;
pub const RR_SVCB: dns_type = 64;
pub const RR_ZONEMD: dns_type = 63;
pub const RR_CSYNC: dns_type = 62;
pub const RR_OPENPGPKEY: dns_type = 61;
pub const RR_CDNSKEY: dns_type = 60;
pub const RR_CDS: dns_type = 59;
pub const RR_TALINK: dns_type = 58;
pub const RR_RKEY: dns_type = 57;
pub const RR_NINFO: dns_type = 56;
pub const RR_HIP: dns_type = 55;
pub const RR_SMIMEA: dns_type = 53;
pub const RR_TLSA: dns_type = 52;
pub const RR_NSEC3PARAM: dns_type = 51;
pub const RR_NSEC3: dns_type = 50;
pub const RR_DHCID: dns_type = 49;
pub const RR_DNSKEY: dns_type = 48;
pub const RR_NSEC: dns_type = 47;
pub const RR_RRSIG: dns_type = 46;
pub const RR_ISECKEY: dns_type = 45;
pub const RR_SSHFP: dns_type = 44;
pub const RR_DS: dns_type = 43;
pub const RR_APL: dns_type = 42;
pub const RR_OPT: dns_type = 41;
pub const RR_SINK: dns_type = 40;
pub const RR_DNAME: dns_type = 39;
pub const RR_A6: dns_type = 38;
pub const RR_CERT: dns_type = 37;
pub const RR_KX: dns_type = 36;
pub const RR_NAPTR: dns_type = 35;
pub const RR_ATMA: dns_type = 34;
pub const RR_SRV: dns_type = 33;
pub const RR_NIMLOC: dns_type = 32;
pub const RR_EID: dns_type = 31;
pub const RR_NXT: dns_type = 30;
pub const RR_LOC: dns_type = 29;
pub const RR_AAAA: dns_type = 28;
pub const RR_GPOS: dns_type = 27;
pub const RR_PX: dns_type = 26;
pub const RR_KEY: dns_type = 25;
pub const RR_SIG: dns_type = 24;
pub const RR_NSAP_PTR: dns_type = 23;
pub const RR_NSAP: dns_type = 22;
pub const RR_RT: dns_type = 21;
pub const RR_ISDN: dns_type = 20;
pub const RR_X25: dns_type = 19;
pub const RR_AFSDB: dns_type = 18;
pub const RR_RP: dns_type = 17;
pub const RR_TXT: dns_type = 16;
pub const RR_MX: dns_type = 15;
pub const RR_MINFO: dns_type = 14;
pub const RR_HINFO: dns_type = 13;
pub const RR_PTR: dns_type = 12;
pub const RR_WKS: dns_type = 11;
pub const RR_NULL: dns_type = 10;
pub const RR_MR: dns_type = 9;
pub const RR_MG: dns_type = 8;
pub const RR_MB: dns_type = 7;
pub const RR_SOA: dns_type = 6;
pub const RR_CNAME: dns_type = 5;
pub const RR_MF: dns_type = 4;
pub const RR_MD: dns_type = 3;
pub const RR_NS: dns_type = 2;
pub const RR_A: dns_type = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_spf_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub len: size_t,
    pub text: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_dnskey_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub zonekey: bool,
    pub sep: bool,
    pub protocol: dnskey_protocol,
    pub algorithm: dnskey_algorithm,
    pub keysize: size_t,
    pub key: *mut uint8_t,
}
pub type dnskey_algorithm = ::core::ffi::c_uint;
pub const DNSKEYA_RSVP: dnskey_algorithm = 255;
pub const DNSKEYA_PRIVATEOID: dnskey_algorithm = 254;
pub const DNSKEYA_PRIVATEDNS: dnskey_algorithm = 253;
pub const DNSKEYA_INDIRECT: dnskey_algorithm = 252;
pub const DNSKEYA_RSASHA1: dnskey_algorithm = 5;
pub const DNSKEYA_ECC: dnskey_algorithm = 4;
pub const DNSKEYA_DSA: dnskey_algorithm = 3;
pub const DNSKEYA_DH: dnskey_algorithm = 2;
pub const DNSKEYA_RSAMD5: dnskey_algorithm = 1;
pub type dnskey_protocol = ::core::ffi::c_uint;
pub const DNSKEYP_ALL: dnskey_protocol = 255;
pub const DNSKEYP_IPSEC: dnskey_protocol = 4;
pub const DNSKEYP_DNSSEC: dnskey_protocol = 3;
pub const DNSKEYP_EMAIL: dnskey_protocol = 2;
pub const DNSKEYP_TLS: dnskey_protocol = 1;
pub const DNSKEYP_NONE: dnskey_protocol = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_nsec_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub next: *const ::core::ffi::c_char,
    pub numbits: size_t,
    pub bitmap: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_rrsig_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub covered: dns_type_t,
    pub algorithm: dnskey_algorithm,
    pub labels: ::core::ffi::c_int,
    pub originttl: TTL,
    pub sigexpire: ::core::ffi::c_ulong,
    pub timesigned: ::core::ffi::c_ulong,
    pub keyfootprint: uint16_t,
    pub signer: *const ::core::ffi::c_char,
    pub sigsize: size_t,
    pub signature: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_ds_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub keytag: dnskey_protocol,
    pub algorithm: dnskey_algorithm,
    pub digest: dnsds_digest,
    pub digestlen: size_t,
    pub digestdata: *mut uint8_t,
}
pub type dnsds_digest = ::core::ffi::c_uint;
pub const DNSDS_SHA1: dnsds_digest = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_apl_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub numrecs: size_t,
    pub recs: *mut dnsapl_record,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dnsapl_record {
    pub addressfamily: ::core::ffi::c_int,
    pub prefix: ::core::ffi::c_int,
    pub afdlength: size_t,
    pub afdpart: *mut uint8_t,
    pub negate: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_edns0opt_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub udp_payload: size_t,
    pub version: ::core::ffi::c_int,
    pub fdo: bool,
    pub fug: ::core::ffi::c_int,
    pub z: ::core::ffi::c_uint,
    pub numopts: size_t,
    pub opts: *mut edns0_opt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edns0_opt_t {
    pub code: edns0_type_t,
    pub len: size_t,
    pub data: *mut uint8_t,
}
pub type edns0_type_t = edns0_type;
pub type edns0_type = ::core::ffi::c_uint;
pub const EDNS0RR_NSID: edns0_type = 3;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_sink_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub rawdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_dname_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub rawdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_a6_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub mask: size_t,
    pub address: in6_addr,
    pub prefixname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_cert_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub rawdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_kx_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub rawdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_naptr_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub order: ::core::ffi::c_int,
    pub preference: ::core::ffi::c_int,
    pub flags: *const ::core::ffi::c_char,
    pub services: *const ::core::ffi::c_char,
    pub regexp: *const ::core::ffi::c_char,
    pub replacement: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_atm_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub rawdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_srv_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub priority: ::core::ffi::c_int,
    pub weight: ::core::ffi::c_int,
    pub port: ::core::ffi::c_int,
    pub target: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_nimloc_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub rawdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_eid_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub rawdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_nxt_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub next: *const ::core::ffi::c_char,
    pub numbits: size_t,
    pub bitmap: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_loc_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub version: ::core::ffi::c_int,
    pub size: ::core::ffi::c_ulonglong,
    pub horiz_pre: ::core::ffi::c_ulonglong,
    pub vert_pre: ::core::ffi::c_ulonglong,
    pub latitude: dnsgpos_angle,
    pub longitude: dnsgpos_angle,
    pub altitude: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dnsgpos_angle {
    pub deg: ::core::ffi::c_int,
    pub min: ::core::ffi::c_int,
    pub sec: ::core::ffi::c_int,
    pub frac: ::core::ffi::c_int,
    pub nw: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_aaaa_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub address: in6_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_gpos_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub longitude: dnsgpos_angle,
    pub latitude: dnsgpos_angle,
    pub altitude: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_px_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub map822: *const ::core::ffi::c_char,
    pub mapx400: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_key_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub flags: C2Rust_Unnamed_3,
    pub signatory: ::core::ffi::c_int,
    pub protocol: dnskey_protocol,
    pub algorithm: dnskey_algorithm,
    pub key: dnskey_key,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union dnskey_key {
    pub md5: C2Rust_Unnamed_2,
    pub unknown: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub size: size_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub expsize: size_t,
    pub exponent: *mut uint8_t,
    pub modsize: size_t,
    pub modulus: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub authentication: bool,
    pub confidential: bool,
    pub experimental: bool,
    pub user: bool,
    pub zone: bool,
    pub host: bool,
    pub ipsec: bool,
    pub email: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_sig_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub covered: dns_type_t,
    pub algorithm: dnskey_algorithm,
    pub labels: ::core::ffi::c_int,
    pub originttl: TTL,
    pub sigexpire: ::core::ffi::c_ulong,
    pub timesigned: ::core::ffi::c_ulong,
    pub keyfootprint: uint16_t,
    pub signer: *const ::core::ffi::c_char,
    pub sigsize: size_t,
    pub signature: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_nsap_ptr_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub owner: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_nsap_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub length: *const ::core::ffi::c_char,
    pub nsapaddress: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_rt_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub preference: ::core::ffi::c_int,
    pub host: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_isdn_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub isdnaddress: *const ::core::ffi::c_char,
    pub sa: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_x25_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub psdnaddress: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_afsdb_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub subtype: ::core::ffi::c_int,
    pub hostname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_rp_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub mbox: *const ::core::ffi::c_char,
    pub domain: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_txt_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub len: size_t,
    pub text: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_mx_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub preference: ::core::ffi::c_int,
    pub exchange: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_minfo_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub rmailbx: *const ::core::ffi::c_char,
    pub emailbx: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_hinfo_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub cpu: *const ::core::ffi::c_char,
    pub os: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_ptr_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub ptr: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_wks_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub address: in_addr_t,
    pub protocol: ::core::ffi::c_int,
    pub numbits: size_t,
    pub bits: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_null_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_mr_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub newname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_mg_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub mgmname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_mb_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub madname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_soa_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub mname: *const ::core::ffi::c_char,
    pub rname: *const ::core::ffi::c_char,
    pub serial: uint32_t,
    pub refresh: uint32_t,
    pub retry: uint32_t,
    pub expire: uint32_t,
    pub minimum: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_cname_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub cname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_mf_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub madname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_md_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub madname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_ns_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub nsdname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_a_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub address: in_addr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_x_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub rawdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_generic_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_question_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
}
pub type dns_op_t = dns_op;
pub type dns_op = ::core::ffi::c_uint;
pub const OP_UNKNOWN: dns_op = 1;
pub const OP_UPDATE: dns_op = 5;
pub const OP_NOTIFY: dns_op = 4;
pub const OP_STATUS: dns_op = 2;
pub const OP_IQUERY: dns_op = 1;
pub const OP_QUERY: dns_op = 0;
pub type dns_decoded_t = uintptr_t;
pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const SOL_SOCKET: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const SO_REUSEADDR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const INADDR_ANY: in_addr_t = 0 as ::core::ffi::c_int as in_addr_t;
pub const PICOQUIC_ERROR_CLASS: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const PICOQUIC_NO_ERROR_TERMINATE_PACKET_LOOP: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 47 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const EXIT_FAILURE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const EAGAIN: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const EPIPE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const EWOULDBLOCK: ::core::ffi::c_int = EAGAIN;
pub const NI_NUMERICHOST: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const NI_NUMERICSERV: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const DBG_PRINTF_FILENAME_MAX: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const FIONREAD: ::core::ffi::c_int = 0x541b as ::core::ffi::c_int;
pub const POLLIN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const SLIPSTREAM_ALPN: [::core::ffi::c_char; 16] =
    unsafe { ::core::mem::transmute::<[u8; 16], [::core::ffi::c_char; 16]>(*b"picoquic_sample\0") };
pub const SLIPSTREAM_SNI: [::core::ffi::c_char; 17] = unsafe {
    ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"test.example.com\0")
};
pub const SLIPSTREAM_INTERNAL_ERROR: ::core::ffi::c_int = 0x101 as ::core::ffi::c_int;
pub const SLIPSTREAM_FILE_CANCEL_ERROR: ::core::ffi::c_int = 0x105 as ::core::ffi::c_int;
pub const DNS_DECODEBUF_4K: usize =
    (4096 as usize).wrapping_div(::core::mem::size_of::<dns_decoded_t>() as usize);
pub const MAX_DNS_QUERY_SIZE: ::core::ffi::c_int = 512 as ::core::ffi::c_int;
#[no_mangle]
pub static mut should_shutdown: sig_atomic_t = 0 as sig_atomic_t;
#[no_mangle]
pub unsafe extern "C" fn client_sighandler(mut signum: ::core::ffi::c_int) {
    debug_printf(
        b"%s:%u [%s]: Signal %d received\n\0".as_ptr() as *const ::core::ffi::c_char,
        (b"../src/slipstream_client.c\0".as_ptr() as *const ::core::ffi::c_char).offset(
            (if 24 as usize > ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize {
                24 as usize
            } else {
                ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize
            })
            .wrapping_sub(DBG_PRINTF_FILENAME_MAX as usize) as isize,
        ) as *const ::core::ffi::c_char,
        31 as ::core::ffi::c_int,
        b"client_sighandler\0".as_ptr() as *const ::core::ffi::c_char,
        signum,
    );
    ::core::ptr::write_volatile(
        &mut should_shutdown as *mut sig_atomic_t,
        1 as ::core::ffi::c_int as sig_atomic_t,
    );
}
#[no_mangle]
pub static mut client_domain_name: *mut ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>() as *mut ::core::ffi::c_char;
#[no_mangle]
pub static mut client_domain_name_len: size_t = 0 as size_t;
#[no_mangle]
pub unsafe extern "C" fn client_encode_segment(
    mut packet: *mut dns_packet_t,
    mut packet_len: *mut size_t,
    mut src_buf: *const ::core::ffi::c_uchar,
    mut src_buf_len: size_t,
) -> ssize_t {
    let mut name: [::core::ffi::c_char; 255] = [0; 255];
    let len: size_t = b32_encode(
        (&raw mut name as *mut ::core::ffi::c_char).offset(0 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_char,
        src_buf as *const ::core::ffi::c_char,
        src_buf_len,
        true_0 as uint32_t,
        false_0 as uint32_t,
    ) as size_t;
    let encoded_len: size_t = slipstream_inline_dotify(
        &raw mut name as *mut ::core::ffi::c_char,
        255 as size_t,
        len,
    ) as size_t;
    name[encoded_len as usize] = '.' as i32 as ::core::ffi::c_char;
    memcpy(
        (&raw mut name as *mut ::core::ffi::c_char)
            .offset(encoded_len.wrapping_add(1 as size_t) as isize)
            as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        client_domain_name as *const ::core::ffi::c_void,
        client_domain_name_len,
    );
    name[encoded_len
        .wrapping_add(1 as size_t)
        .wrapping_add(client_domain_name_len) as usize] = '.' as i32 as ::core::ffi::c_char;
    name[encoded_len
        .wrapping_add(1 as size_t)
        .wrapping_add(client_domain_name_len)
        .wrapping_add(1 as size_t) as usize] = '\0' as i32 as ::core::ffi::c_char;
    let mut question: dns_question_t = dns_question_t {
        name: ::core::ptr::null::<::core::ffi::c_char>(),
        type_0: 0 as dns_type_t,
        class: 0 as dns_class_t,
    };
    question.name = &raw mut name as *mut ::core::ffi::c_char;
    question.type_0 = RR_TXT;
    question.class = CLASS_IN;
    let mut edns: dns_answer_t = ::core::mem::zeroed();
    edns.opt.name = b".\0".as_ptr() as *const ::core::ffi::c_char;
    edns.opt.type_0 = RR_OPT;
    edns.opt.class = 1232 as dns_class_t;
    edns.opt.ttl = 0 as TTL;
    edns.opt.udp_payload = 1232 as size_t;
    edns.opt.version = 0 as ::core::ffi::c_int;
    let mut query: dns_query_t = dns_query_t {
        id: 0 as ::core::ffi::c_int,
        query: false,
        opcode: OP_QUERY,
        aa: false,
        tc: false,
        rd: false,
        ra: false,
        z: false,
        ad: false,
        cd: false,
        rcode: RCODE_OKAY,
        qdcount: 0,
        ancount: 0,
        nscount: 0,
        arcount: 0,
        questions: ::core::ptr::null_mut::<dns_question_t>(),
        answers: ::core::ptr::null_mut::<dns_answer_t>(),
        nameservers: ::core::ptr::null_mut::<dns_answer_t>(),
        additional: ::core::ptr::null_mut::<dns_answer_t>(),
    };
    query.id = rand() % UINT16_MAX;
    query.query = true_0 != 0;
    query.opcode = OP_QUERY;
    query.rd = true_0 != 0;
    query.rcode = RCODE_OKAY;
    query.qdcount = 1 as size_t;
    query.questions = &raw mut question;
    query.arcount = 1 as size_t;
    query.additional = &raw mut edns;
    let rc: dns_rcode_t = dns_encode(packet, packet_len, &raw mut query) as dns_rcode_t;
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        debug_printf(
            b"%s:%u [%s]: dns_encode() = (%d) %s: %s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
            (b"../src/slipstream_client.c\0".as_ptr() as *const ::core::ffi::c_char).offset(
                (if 24 as usize > ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize {
                    24 as usize
                } else {
                    ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize
                })
                .wrapping_sub(DBG_PRINTF_FILENAME_MAX as usize) as isize,
            ) as *const ::core::ffi::c_char,
            93 as ::core::ffi::c_int,
            b"client_encode_segment\0".as_ptr() as *const ::core::ffi::c_char,
            rc as ::core::ffi::c_uint,
            dns_rcode_text(rc),
            &raw mut name as *mut ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int) as ssize_t;
    }
    return 0 as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn client_encode(
    mut slot_p: *mut ::core::ffi::c_void,
    mut callback_ctx: *mut ::core::ffi::c_void,
    mut dest_buf: *mut *mut ::core::ffi::c_uchar,
    mut src_buf: *const ::core::ffi::c_uchar,
    mut src_buf_len: size_t,
    mut segment_len: *mut size_t,
    mut peer_addr: *mut sockaddr_storage,
    mut local_addr: *mut sockaddr_storage,
) -> ssize_t {
    *dest_buf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    if src_buf_len <= *segment_len {
        let mut packet_len: size_t = MAX_DNS_QUERY_SIZE as size_t;
        let mut packet: *mut ::core::ffi::c_uchar = malloc(packet_len) as *mut ::core::ffi::c_uchar;
        let ret: ssize_t = client_encode_segment(
            packet as *mut dns_packet_t,
            &raw mut packet_len,
            src_buf,
            src_buf_len,
        ) as ssize_t;
        if ret < 0 as ssize_t {
            free(packet as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int) as ssize_t;
        }
        *dest_buf = packet;
        *segment_len = packet_len;
        return packet_len as ssize_t;
    }
    let mut num_segments: size_t = src_buf_len.wrapping_div(*segment_len);
    let mut packets: *mut ::core::ffi::c_uchar =
        malloc((MAX_DNS_QUERY_SIZE as size_t).wrapping_mul(num_segments))
            as *mut ::core::ffi::c_uchar;
    let mut current_packet: *mut ::core::ffi::c_uchar = packets;
    let mut segment: *const ::core::ffi::c_uchar = src_buf;
    let mut first_packet_len: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < num_segments {
        let mut packet_len_0: size_t = MAX_DNS_QUERY_SIZE as size_t;
        let ret_0: ssize_t = client_encode_segment(
            current_packet as *mut dns_packet_t,
            &raw mut packet_len_0,
            segment,
            *segment_len,
        ) as ssize_t;
        if ret_0 < 0 as ssize_t {
            free(packets as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int) as ssize_t;
        }
        if first_packet_len == 0 as size_t {
            first_packet_len = packet_len_0;
        } else if packet_len_0 > first_packet_len {
            debug_printf(
                b"%s:%u [%s]: current encoded segment length %d > %d than first segment\n\n\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                (b"../src/slipstream_client.c\0".as_ptr() as *const ::core::ffi::c_char).offset(
                    (if 24 as usize > ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize {
                        24 as usize
                    } else {
                        ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize
                    })
                    .wrapping_sub(DBG_PRINTF_FILENAME_MAX as usize) as isize,
                ) as *const ::core::ffi::c_char,
                147 as ::core::ffi::c_int,
                b"client_encode\0".as_ptr() as *const ::core::ffi::c_char,
                packet_len_0,
                first_packet_len,
            );
            free(packets as *mut ::core::ffi::c_void);
            return -(1 as ::core::ffi::c_int) as ssize_t;
        }
        current_packet = current_packet.offset(packet_len_0 as isize);
        segment = segment.offset(*segment_len as isize);
        i = i.wrapping_add(1);
    }
    *dest_buf = packets;
    *segment_len = first_packet_len;
    return current_packet.offset_from(packets) as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn client_decode(
    mut slot_p: *mut ::core::ffi::c_void,
    mut callback_ctx: *mut ::core::ffi::c_void,
    mut dest_buf: *mut *mut ::core::ffi::c_uchar,
    mut src_buf: *const ::core::ffi::c_uchar,
    mut src_buf_len: size_t,
    mut peer_addr: *mut sockaddr_storage,
    mut local_addr: *mut sockaddr_storage,
) -> ssize_t {
    *dest_buf = ::core::ptr::null_mut::<::core::ffi::c_uchar>();
    let mut bufsize: size_t =
        DNS_DECODEBUF_4K.wrapping_mul(::core::mem::size_of::<dns_decoded_t>() as size_t);
    let mut decoded: [dns_decoded_t; 512] = [
        0 as ::core::ffi::c_int as dns_decoded_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let rc: dns_rcode_t = dns_decode(
        &raw mut decoded as *mut dns_decoded_t,
        &raw mut bufsize,
        src_buf as *const dns_packet_t,
        src_buf_len,
    ) as dns_rcode_t;
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        debug_printf(
            b"%s:%u [%s]: dns_decode() = (%d) %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            (b"../src/slipstream_client.c\0".as_ptr() as *const ::core::ffi::c_char).offset(
                (if 24 as usize > ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize {
                    24 as usize
                } else {
                    ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize
                })
                .wrapping_sub(DBG_PRINTF_FILENAME_MAX as usize) as isize,
            ) as *const ::core::ffi::c_char,
            177 as ::core::ffi::c_int,
            b"client_decode\0".as_ptr() as *const ::core::ffi::c_char,
            rc as ::core::ffi::c_uint,
            dns_rcode_text(rc),
        );
        return -(1 as ::core::ffi::c_int) as ssize_t;
    }
    let mut query: *const dns_query_t = &raw mut decoded as *mut dns_decoded_t as *mut dns_query_t;
    if (*query).query as ::core::ffi::c_int == 1 as ::core::ffi::c_int {
        debug_printf(
            b"%s:%u [%s]: [%d] dns record is not a response\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            (b"../src/slipstream_client.c\0".as_ptr() as *const ::core::ffi::c_char).offset(
                (if 24 as usize > ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize {
                    24 as usize
                } else {
                    ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize
                })
                .wrapping_sub(DBG_PRINTF_FILENAME_MAX as usize) as isize,
            ) as *const ::core::ffi::c_char,
            184 as ::core::ffi::c_int,
            b"client_decode\0".as_ptr() as *const ::core::ffi::c_char,
            (*query).id,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        return 0 as ssize_t;
    }
    if (*query).rcode as ::core::ffi::c_uint
        == RCODE_NAME_ERROR as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return 0 as ssize_t;
    }
    if (*query).rcode as ::core::ffi::c_uint
        != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debug_printf(
            b"%s:%u [%s]: [%d] dns record rcode not okay: %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            (b"../src/slipstream_client.c\0".as_ptr() as *const ::core::ffi::c_char).offset(
                (if 24 as usize > ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize {
                    24 as usize
                } else {
                    ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize
                })
                .wrapping_sub(DBG_PRINTF_FILENAME_MAX as usize) as isize,
            ) as *const ::core::ffi::c_char,
            194 as ::core::ffi::c_int,
            b"client_decode\0".as_ptr() as *const ::core::ffi::c_char,
            (*query).id,
            (*query).rcode as ::core::ffi::c_uint,
        );
        return 0 as ssize_t;
    }
    if (*query).ancount != 1 as size_t {
        return 0 as ssize_t;
    }
    let mut answer_txt: *mut dns_txt_t = (*query).answers.offset(0 as ::core::ffi::c_int as isize)
        as *mut dns_answer_t as *mut dns_txt_t;
    if (*answer_txt).type_0 as ::core::ffi::c_uint
        != RR_TXT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        debug_printf(
            b"%s:%u [%s]: [%d] answer type is not TXT\n\0".as_ptr() as *const ::core::ffi::c_char,
            (b"../src/slipstream_client.c\0".as_ptr() as *const ::core::ffi::c_char).offset(
                (if 24 as usize > ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize {
                    24 as usize
                } else {
                    ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize
                })
                .wrapping_sub(DBG_PRINTF_FILENAME_MAX as usize) as isize,
            ) as *const ::core::ffi::c_char,
            205 as ::core::ffi::c_int,
            b"client_decode\0".as_ptr() as *const ::core::ffi::c_char,
            (*query).id,
            ::core::ptr::null_mut::<::core::ffi::c_void>(),
        );
        return 0 as ssize_t;
    }
    *dest_buf = malloc((*answer_txt).len) as *mut ::core::ffi::c_uchar;
    memcpy(
        *dest_buf as *mut ::core::ffi::c_void,
        (*answer_txt).text as *const ::core::ffi::c_void,
        (*answer_txt).len,
    );
    return (*answer_txt).len as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn slipstream_client_create_stream_ctx(
    mut cnx: *mut picoquic_cnx_t,
    mut client_ctx: *mut slipstream_client_ctx_t,
    mut sock_fd: ::core::ffi::c_int,
) -> *mut slipstream_client_stream_ctx_t {
    let mut stream_ctx: *mut slipstream_client_stream_ctx_t =
        malloc(::core::mem::size_of::<slipstream_client_stream_ctx_t>() as size_t)
            as *mut slipstream_client_stream_ctx_t;
    if stream_ctx.is_null() {
        fprintf(
            stdout,
            b"Memory Error, cannot create stream for sock %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            sock_fd,
        );
        return ::core::ptr::null_mut::<slipstream_client_stream_ctx_t>();
    }
    memset(
        stream_ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<slipstream_client_stream_ctx_t>() as size_t,
    );
    if (*client_ctx).first_stream.is_null() {
        (*client_ctx).first_stream = stream_ctx;
    } else {
        (*stream_ctx).next_stream =
            (*client_ctx).first_stream as *mut st_slipstream_client_stream_ctx_t;
        (*(*stream_ctx).next_stream).previous_stream =
            stream_ctx as *mut st_slipstream_client_stream_ctx_t;
        (*client_ctx).first_stream = stream_ctx;
    }
    (*stream_ctx).fd = sock_fd;
    (*stream_ctx).stream_id = -(1 as ::core::ffi::c_int) as uint64_t;
    return stream_ctx;
}
unsafe extern "C" fn slipstream_client_free_stream_ctx(
    mut client_ctx: *mut slipstream_client_ctx_t,
    mut stream_ctx: *mut slipstream_client_stream_ctx_t,
) {
    if !(*stream_ctx).previous_stream.is_null() {
        (*(*stream_ctx).previous_stream).next_stream = (*stream_ctx).next_stream;
    }
    if !(*stream_ctx).next_stream.is_null() {
        (*(*stream_ctx).next_stream).previous_stream = (*stream_ctx).previous_stream;
    }
    if (*client_ctx).first_stream == stream_ctx {
        (*client_ctx).first_stream =
            (*stream_ctx).next_stream as *mut slipstream_client_stream_ctx_t;
    }
    (*stream_ctx).fd = close((*stream_ctx).fd);
    free(stream_ctx as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn slipstream_client_mark_active_pass(
    mut client_ctx: *mut slipstream_client_ctx_t,
) {
    let mut stream_ctx: *mut slipstream_client_stream_ctx_t = (*client_ctx).first_stream;
    while !stream_ctx.is_null() {
        if (*stream_ctx).set_active != 0 {
            if (*stream_ctx).stream_id == -(1 as ::core::ffi::c_int) as uint64_t {
                (*stream_ctx).stream_id = picoquic_get_next_local_stream_id(
                    (*client_ctx).cnx as *mut picoquic_cnx_t,
                    0 as ::core::ffi::c_int,
                );
                printf(
                    b"[%lu:%d] assigned stream id\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*stream_ctx).stream_id,
                    (*stream_ctx).fd,
                );
            }
            ::core::ptr::write_volatile(
                &mut (*stream_ctx).set_active as *mut sig_atomic_t,
                0 as ::core::ffi::c_int as sig_atomic_t,
            );
            printf(
                b"[%lu:%d] activate: stream\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*stream_ctx).stream_id,
                (*stream_ctx).fd,
            );
            picoquic_mark_active_stream(
                (*client_ctx).cnx as *mut picoquic_cnx_t,
                (*stream_ctx).stream_id,
                1 as ::core::ffi::c_int,
                stream_ctx as *mut ::core::ffi::c_void,
            );
        }
        stream_ctx = (*stream_ctx).next_stream as *mut slipstream_client_stream_ctx_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn slipstream_add_paths(mut client_ctx: *mut slipstream_client_ctx_t) {
    let mut cnx: *mut picoquic_cnx_t = (*client_ctx).cnx;
    let mut i: size_t = 1 as size_t;
    while i < (*client_ctx).server_address_count {
        let mut slipstream_path: *mut address_t =
            (*client_ctx).server_addresses.offset(i as isize) as *mut address_t;
        if !(*slipstream_path).added {
            let mut current_time: uint64_t = picoquic_current_time();
            print_sockaddr_ip_and_port(&raw mut (*slipstream_path).server_address);
            let mut path_id: ::core::ffi::c_int = -(2 as ::core::ffi::c_int);
            picoquic_probe_new_path_ex(
                cnx as *mut picoquic_cnx_t,
                &raw mut (*slipstream_path).server_address as *mut sockaddr,
                &raw mut (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).local_addr
                    as *mut sockaddr,
                0 as ::core::ffi::c_int,
                current_time,
                0 as ::core::ffi::c_int,
                &raw mut path_id,
            );
            if path_id < 0 as ::core::ffi::c_int {
                debug_printf(
                    b"%s:%u [%s]: Failed adding path\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (b"../src/slipstream_client.c\0".as_ptr() as *const ::core::ffi::c_char).offset(
                        (if 24 as usize
                            > ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize
                        {
                            24 as usize
                        } else {
                            ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize
                        })
                        .wrapping_sub(DBG_PRINTF_FILENAME_MAX as usize)
                            as isize,
                    ) as *const ::core::ffi::c_char,
                    303 as ::core::ffi::c_int,
                    b"slipstream_add_paths\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
            } else {
                debug_printf(
                    b"%s:%u [%s]: Added path\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (b"../src/slipstream_client.c\0".as_ptr() as *const ::core::ffi::c_char).offset(
                        (if 24 as usize
                            > ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize
                        {
                            24 as usize
                        } else {
                            ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize
                        })
                        .wrapping_sub(DBG_PRINTF_FILENAME_MAX as usize)
                            as isize,
                    ) as *const ::core::ffi::c_char,
                    306 as ::core::ffi::c_int,
                    b"slipstream_add_paths\0".as_ptr() as *const ::core::ffi::c_char,
                    ::core::ptr::null_mut::<::core::ffi::c_void>(),
                );
                picoquic_reinsert_by_wake_time((*cnx).quic, cnx, current_time);
                (*slipstream_path).added = true_0 != 0;
            }
        }
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn slipstream_client_sockloop_callback(
    mut quic: *mut picoquic_quic_t,
    mut cb_mode: picoquic_packet_loop_cb_enum,
    mut callback_ctx: *mut ::core::ffi::c_void,
    mut callback_arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut client_ctx: *mut slipstream_client_ctx_t = callback_ctx as *mut slipstream_client_ctx_t;
    if (*client_ctx).closed {
        return 0 as ::core::ffi::c_int;
    }
    let mut c2rust_current_block_33: u64;
    match cb_mode as ::core::ffi::c_uint {
        1 => {
            if (*client_ctx).ready {
                slipstream_add_paths(client_ctx);
            }
            if should_shutdown != 0 {
                let mut cnx: *mut picoquic_cnx_t =
                    picoquic_get_first_cnx(quic as *mut picoquic_quic_t) as *mut picoquic_cnx_t;
                let mut has_unclosed: bool = false_0 != 0;
                while !cnx.is_null() {
                    debug_printf(
                        b"%s:%u [%s]: CNX state: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                        (b"../src/slipstream_client.c\0".as_ptr() as *const ::core::ffi::c_char)
                            .offset(
                                (if 24 as usize
                                    > ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize
                                {
                                    24 as usize
                                } else {
                                    ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize
                                })
                                .wrapping_sub(DBG_PRINTF_FILENAME_MAX as usize)
                                    as isize,
                            ) as *const ::core::ffi::c_char,
                        331 as ::core::ffi::c_int,
                        b"slipstream_client_sockloop_callback\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*cnx).cnx_state as ::core::ffi::c_uint,
                    );
                    if (*cnx).cnx_state as ::core::ffi::c_uint
                        != picoquic_state_disconnected as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        has_unclosed = true_0 != 0;
                    }
                    picoquic_close(cnx as *mut picoquic_cnx_t, 0 as uint64_t);
                    if (*cnx).cnx_state as ::core::ffi::c_uint
                        == picoquic_state_draining as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        picoquic_connection_disconnect(cnx);
                    }
                    cnx = picoquic_get_next_cnx(cnx as *mut picoquic_cnx_t) as *mut picoquic_cnx_t;
                }
                if !has_unclosed {
                    debug_printf(
                        b"%s:%u [%s]: All connections closed, shutting down.\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (b"../src/slipstream_client.c\0".as_ptr() as *const ::core::ffi::c_char)
                            .offset(
                                (if 24 as usize
                                    > ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize
                                {
                                    24 as usize
                                } else {
                                    ::core::mem::size_of::<[::core::ffi::c_char; 27]>() as usize
                                })
                                .wrapping_sub(DBG_PRINTF_FILENAME_MAX as usize)
                                    as isize,
                            ) as *const ::core::ffi::c_char,
                        346 as ::core::ffi::c_int,
                        b"slipstream_client_sockloop_callback\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ::core::ptr::null_mut::<::core::ffi::c_void>(),
                    );
                    return -(1 as ::core::ffi::c_int);
                }
            }
            c2rust_current_block_33 = 6800848482211366931;
        }
        7 => {
            c2rust_current_block_33 = 6800848482211366931;
        }
        3 => {
            if callback_ctx.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            if (*(*client_ctx).cnx).cnx_state as ::core::ffi::c_uint
                == picoquic_state_disconnected as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                printf(b"Terminate packet loop\n\0".as_ptr() as *const ::core::ffi::c_char);
                return PICOQUIC_NO_ERROR_TERMINATE_PACKET_LOOP;
            }
            c2rust_current_block_33 = 1608152415753874203;
        }
        _ => {
            c2rust_current_block_33 = 1608152415753874203;
        }
    }
    match c2rust_current_block_33 {
        6800848482211366931 => {
            if callback_ctx.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            slipstream_client_mark_active_pass(client_ctx);
        }
        _ => {}
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn slipstream_client_poller(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut args: *mut slipstream_client_poller_args = arg as *mut slipstream_client_poller_args;
    loop {
        let mut fds: pollfd = pollfd {
            fd: 0,
            events: 0,
            revents: 0,
        };
        fds.fd = (*args).fd;
        fds.events = POLLIN as ::core::ffi::c_short;
        fds.revents = 0 as ::core::ffi::c_short;
        let mut ret: ::core::ffi::c_int =
            poll(&raw mut fds, 1 as nfds_t, 1000 as ::core::ffi::c_int);
        if ret < 0 as ::core::ffi::c_int {
            perror(b"poll() failed\0".as_ptr() as *const ::core::ffi::c_char);
            break;
        } else {
            if ret == 0 as ::core::ffi::c_int {
                continue;
            }
            ::core::ptr::write_volatile(
                &mut (*(*args).stream_ctx).set_active as *mut sig_atomic_t,
                1 as ::core::ffi::c_int as sig_atomic_t,
            );
            ret = picoquic_wake_up_network_thread((*(*args).client_ctx).thread_ctx);
            if ret != 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"poll: could not wake up network thread, ret = %d\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ret,
                );
            }
            printf(
                b"[%lu:%d] wakeup\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*(*args).stream_ctx).stream_id,
                (*args).fd,
            );
            break;
        }
    }
    free(args as *mut ::core::ffi::c_void);
    pthread_exit(NULL);
}
#[no_mangle]
pub unsafe extern "C" fn slipstream_client_accepter(
    mut arg: *mut ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut args: *mut slipstream_client_accepter_args =
        arg as *mut slipstream_client_accepter_args;
    loop {
        let mut client_addr: sockaddr_in = sockaddr_in {
            sin_family: 0,
            sin_port: 0,
            sin_addr: in_addr { s_addr: 0 },
            sin_zero: [0; 8],
        };
        let mut client_len: socklen_t = ::core::mem::size_of::<sockaddr_in>() as socklen_t;
        let mut client_sock: ::core::ffi::c_int = accept(
            (*args).fd,
            &raw mut client_addr as *mut sockaddr,
            &raw mut client_len,
        );
        if client_sock < 0 as ::core::ffi::c_int {
            if *__errno_location() == EINTR {
                fprintf(stderr, b"my ass?\0".as_ptr() as *const ::core::ffi::c_char);
            } else {
                perror(b"accept() failed\0".as_ptr() as *const ::core::ffi::c_char);
                break;
            }
        } else {
            let mut client_ip_str: [::core::ffi::c_char; 16] = [0; 16];
            if inet_ntop(
                AF_INET,
                &raw mut client_addr.sin_addr as *const ::core::ffi::c_void,
                &raw mut client_ip_str as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as socklen_t,
            )
            .is_null()
            {
                perror(b"inet_ntop failed\0".as_ptr() as *const ::core::ffi::c_char);
                close(client_sock);
            } else {
                let mut client_port: uint16_t =
                    __bswap_16(client_addr.sin_port as __uint16_t) as uint16_t;
                fprintf(
                    stderr,
                    b"Accepted connection from %s:%u on socket %d\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    &raw mut client_ip_str as *mut ::core::ffi::c_char,
                    client_port as ::core::ffi::c_int,
                    client_sock,
                );
                let mut stream_ctx: *mut slipstream_client_stream_ctx_t =
                    slipstream_client_create_stream_ctx(
                        (*args).cnx,
                        (*args).client_ctx,
                        client_sock,
                    );
                if stream_ctx.is_null() {
                    fprintf(
                        stderr,
                        b"Could not initiate stream for %d\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        client_sock,
                    );
                    break;
                } else {
                    ::core::ptr::write_volatile(
                        &mut (*stream_ctx).set_active as *mut sig_atomic_t,
                        1 as ::core::ffi::c_int as sig_atomic_t,
                    );
                    let mut ret: ::core::ffi::c_int =
                        picoquic_wake_up_network_thread((*args).thread_ctx);
                    if ret != 0 as ::core::ffi::c_int {
                        fprintf(
                            stderr,
                            b"accept: could not wake up network thread, ret = %d\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            ret,
                        );
                        pthread_exit(NULL);
                    }
                    printf(
                        b"[%lu:%d] accept: connection\n[%lu:%d] wakeup\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (*stream_ctx).stream_id,
                        client_sock,
                        (*stream_ctx).stream_id,
                        client_sock,
                    );
                }
            }
        }
    }
    free(args as *mut ::core::ffi::c_void);
    pthread_exit(NULL);
}
#[no_mangle]
pub unsafe extern "C" fn slipstream_client_callback(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut bytes: *mut uint8_t,
    mut length: size_t,
    mut fin_or_event: picoquic_call_back_event_t,
    mut callback_ctx: *mut ::core::ffi::c_void,
    mut v_stream_ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut client_ctx: *mut slipstream_client_ctx_t = callback_ctx as *mut slipstream_client_ctx_t;
    let mut stream_ctx: *mut slipstream_client_stream_ctx_t =
        v_stream_ctx as *mut slipstream_client_stream_ctx_t;
    if client_ctx.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    let mut c2rust_current_block_74: u64;
    match fin_or_event as ::core::ffi::c_uint {
        0 | 1 => {
            if stream_ctx.is_null() {
                return 0 as ::core::ffi::c_int;
            }
            if length > 0 as size_t {
                let mut bytes_sent: ssize_t = send(
                    (*stream_ctx).fd,
                    bytes as *const ::core::ffi::c_void,
                    length,
                    MSG_NOSIGNAL as ::core::ffi::c_int,
                );
                if bytes_sent < 0 as ssize_t {
                    if *__errno_location() == EPIPE {
                        printf(
                            b"[%lu:%d] send: closed stream\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            stream_id,
                            (*stream_ctx).fd,
                        );
                        picoquic_reset_stream(
                            cnx as *mut picoquic_cnx_t,
                            stream_id,
                            SLIPSTREAM_FILE_CANCEL_ERROR as uint64_t,
                        );
                        return 0 as ::core::ffi::c_int;
                    }
                    *__errno_location() == EAGAIN;
                    printf(
                        b"[%lu:%d] send: error: %s (%d)\n\0".as_ptr() as *const ::core::ffi::c_char,
                        stream_id,
                        (*stream_ctx).fd,
                        strerror(*__errno_location()),
                        *__errno_location(),
                    );
                    picoquic_reset_stream(
                        cnx as *mut picoquic_cnx_t,
                        stream_id,
                        SLIPSTREAM_INTERNAL_ERROR as uint64_t,
                    );
                    return 0 as ::core::ffi::c_int;
                }
            }
            if fin_or_event as ::core::ffi::c_uint
                == picoquic_callback_stream_fin as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                printf(
                    b"[%lu:%d] fin\n\0".as_ptr() as *const ::core::ffi::c_char,
                    stream_id,
                    (*stream_ctx).fd,
                );
                close((*stream_ctx).fd);
                (*stream_ctx).fd = -(1 as ::core::ffi::c_int);
                picoquic_unlink_app_stream_ctx(cnx as *mut picoquic_cnx_t, stream_id);
            }
            c2rust_current_block_74 = 7189308829251266000;
        }
        3 => {
            picoquic_reset_stream(cnx as *mut picoquic_cnx_t, stream_id, 0 as uint64_t);
            c2rust_current_block_74 = 7391479593097540975;
        }
        2 => {
            c2rust_current_block_74 = 7391479593097540975;
        }
        4 | 5 | 6 => {
            printf(b"Connection closed.\n\0".as_ptr() as *const ::core::ffi::c_char);
            ::core::ptr::write_volatile(
                &mut should_shutdown as *mut sig_atomic_t,
                true_0 as sig_atomic_t,
            );
            c2rust_current_block_74 = 7189308829251266000;
        }
        8 => {
            if stream_ctx.is_null() {
                c2rust_current_block_74 = 7189308829251266000;
            } else {
                let mut length_available: ::core::ffi::c_int = 0;
                ret = ioctl(
                    (*stream_ctx).fd,
                    FIONREAD as ::core::ffi::c_ulong,
                    &raw mut length_available,
                );
                if ret < 0 as ::core::ffi::c_int {
                    printf(
                        b"[%lu:%d] ioctl error: %s (%d)\n\0".as_ptr() as *const ::core::ffi::c_char,
                        stream_id,
                        (*stream_ctx).fd,
                        strerror(*__errno_location()),
                        *__errno_location(),
                    );
                    picoquic_reset_stream(
                        cnx as *mut picoquic_cnx_t,
                        stream_id,
                        SLIPSTREAM_INTERNAL_ERROR as uint64_t,
                    );
                } else {
                    ret = 0 as ::core::ffi::c_int;
                    let mut length_to_read: ::core::ffi::c_int =
                        (if length < length_available as size_t {
                            length
                        } else {
                            length_available as size_t
                        }) as ::core::ffi::c_int;
                    if length_to_read == 0 as ::core::ffi::c_int {
                        let mut a: ::core::ffi::c_char = 0;
                        let mut bytes_read: ssize_t = recv(
                            (*stream_ctx).fd,
                            &raw mut a as *mut ::core::ffi::c_void,
                            1 as size_t,
                            MSG_PEEK as ::core::ffi::c_int | MSG_DONTWAIT as ::core::ffi::c_int,
                        );
                        if *__errno_location() == EAGAIN || *__errno_location() == EWOULDBLOCK {
                            picoquic_provide_stream_data_buffer(
                                bytes as *mut ::core::ffi::c_void,
                                0 as size_t,
                                0 as ::core::ffi::c_int,
                                0 as ::core::ffi::c_int,
                            );
                            printf(
                                b"[%lu:%d] recv->quic_send: empty, disactivate\n\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                stream_id,
                                (*stream_ctx).fd,
                            );
                            let mut args: *mut slipstream_client_poller_args =
                                malloc(::core::mem::size_of::<slipstream_client_poller_args>()
                                    as size_t)
                                    as *mut slipstream_client_poller_args;
                            (*args).fd = (*stream_ctx).fd;
                            (*args).cnx = cnx;
                            (*args).client_ctx = client_ctx;
                            (*args).stream_ctx = stream_ctx;
                            let mut thread: pthread_t = 0;
                            if pthread_create(
                                &raw mut thread,
                                ::core::ptr::null::<pthread_attr_t>(),
                                Some(
                                    slipstream_client_poller
                                        as unsafe extern "C" fn(
                                            *mut ::core::ffi::c_void,
                                        )
                                            -> *mut ::core::ffi::c_void,
                                ),
                                args as *mut ::core::ffi::c_void,
                            ) != 0 as ::core::ffi::c_int
                            {
                                perror(b"pthread_create() failed for thread1\0".as_ptr()
                                    as *const ::core::ffi::c_char);
                                free(args as *mut ::core::ffi::c_void);
                            }
                            pthread_setname_np(
                                thread,
                                b"slipstream_server_poller\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                            );
                            pthread_detach(thread);
                        }
                        if bytes_read == 0 as ssize_t {
                            printf(
                                b"[%lu:%d] recv: closed stream\n\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                stream_id,
                                (*stream_ctx).fd,
                            );
                            picoquic_reset_stream(
                                cnx as *mut picoquic_cnx_t,
                                stream_id,
                                SLIPSTREAM_FILE_CANCEL_ERROR as uint64_t,
                            );
                            return 0 as ::core::ffi::c_int;
                        }
                        if bytes_read > 0 as ssize_t {
                            picoquic_provide_stream_data_buffer(
                                bytes as *mut ::core::ffi::c_void,
                                0 as size_t,
                                0 as ::core::ffi::c_int,
                                1 as ::core::ffi::c_int,
                            );
                        } else {
                            return 0 as ::core::ffi::c_int;
                        }
                    } else {
                        let mut buffer: *mut uint8_t = picoquic_provide_stream_data_buffer(
                            bytes as *mut ::core::ffi::c_void,
                            length_to_read as size_t,
                            0 as ::core::ffi::c_int,
                            1 as ::core::ffi::c_int,
                        );
                        if !buffer.is_null() {
                            let mut bytes_read_0: ssize_t = recv(
                                (*stream_ctx).fd,
                                buffer as *mut ::core::ffi::c_void,
                                length_to_read as size_t,
                                MSG_DONTWAIT as ::core::ffi::c_int,
                            );
                            if bytes_read_0 == 0 as ssize_t {
                                printf(
                                    b"Closed connection on sock %d on recv\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                    (*stream_ctx).fd,
                                );
                                picoquic_reset_stream(
                                    cnx as *mut picoquic_cnx_t,
                                    stream_id,
                                    SLIPSTREAM_FILE_CANCEL_ERROR as uint64_t,
                                );
                                return 0 as ::core::ffi::c_int;
                            }
                            if bytes_read_0 < 0 as ssize_t {
                                fprintf(
                                    stderr,
                                    b"recv: %s (%d)\n\0".as_ptr() as *const ::core::ffi::c_char,
                                    strerror(*__errno_location()),
                                    *__errno_location(),
                                );
                                picoquic_reset_stream(
                                    cnx as *mut picoquic_cnx_t,
                                    stream_id,
                                    SLIPSTREAM_INTERNAL_ERROR as uint64_t,
                                );
                                return 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
                c2rust_current_block_74 = 7189308829251266000;
            }
        }
        9 => {
            fprintf(
                stdout,
                b"Connection completed, almost ready.\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            c2rust_current_block_74 = 7189308829251266000;
        }
        10 => {
            fprintf(
                stdout,
                b"Connection confirmed.\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
            (*client_ctx).ready = true_0 != 0;
            slipstream_add_paths(client_ctx);
            c2rust_current_block_74 = 7189308829251266000;
        }
        _ => {
            c2rust_current_block_74 = 7189308829251266000;
        }
    }
    match c2rust_current_block_74 {
        7391479593097540975 => {
            if !stream_ctx.is_null() {
                printf(
                    b"[%lu:%d] stream reset\n\0".as_ptr() as *const ::core::ffi::c_char,
                    stream_id,
                    (*stream_ctx).fd,
                );
                slipstream_client_free_stream_ctx(client_ctx, stream_ctx);
                picoquic_reset_stream(
                    cnx as *mut picoquic_cnx_t,
                    stream_id,
                    SLIPSTREAM_FILE_CANCEL_ERROR as uint64_t,
                );
            }
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn slipstream_connect(
    mut server_address: *mut sockaddr_storage,
    mut quic: *mut picoquic_quic_t,
    mut cnx: *mut *mut picoquic_cnx_t,
    mut client_ctx: *mut slipstream_client_ctx_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sni: *const ::core::ffi::c_char = SLIPSTREAM_SNI.as_ptr();
    let mut current_time: uint64_t = picoquic_current_time();
    *cnx = ::core::ptr::null_mut::<picoquic_cnx_t>();
    let mut host: [::core::ffi::c_char; 1025] = [0; 1025];
    let mut addrlen: socklen_t = ::core::mem::size_of::<sockaddr_storage>() as socklen_t;
    ret = getnameinfo(
        server_address as *mut sockaddr,
        addrlen,
        &raw mut host as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 1025]>() as socklen_t,
        ::core::ptr::null_mut::<::core::ffi::c_char>(),
        0 as socklen_t,
        NI_NUMERICHOST | NI_NUMERICSERV,
    );
    if ret != 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"Could not get name info for server address\n\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    printf(
        b"Starting connection to %s\n\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut host as *mut ::core::ffi::c_char,
    );
    *cnx = picoquic_create_cnx(
        quic as *mut picoquic_quic_t,
        picoquic_null_connection_id,
        picoquic_null_connection_id,
        server_address as *mut sockaddr,
        current_time,
        0 as uint32_t,
        sni,
        SLIPSTREAM_ALPN.as_ptr(),
        1 as ::core::ffi::c_char,
    ) as *mut picoquic_cnx_t;
    if (*cnx).is_null() {
        fprintf(
            stderr,
            b"Could not create connection context\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    (*client_ctx).cnx = *cnx;
    picoquic_set_callback(
        *cnx,
        Some(
            slipstream_client_callback
                as unsafe extern "C" fn(
                    *mut picoquic_cnx_t,
                    uint64_t,
                    *mut uint8_t,
                    size_t,
                    picoquic_call_back_event_t,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        client_ctx as *mut ::core::ffi::c_void,
    );
    ret = picoquic_start_client_cnx(*cnx);
    if ret < 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"Could not activate connection\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    let mut icid: picoquic_connection_id_t = picoquic_get_initial_cnxid(*cnx);
    printf(b"Initial connection ID: \0".as_ptr() as *const ::core::ffi::c_char);
    let mut i: uint8_t = 0 as uint8_t;
    while (i as ::core::ffi::c_int) < icid.id_len as ::core::ffi::c_int {
        printf(
            b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
            icid.id[i as usize] as ::core::ffi::c_int,
        );
        i = i.wrapping_add(1);
    }
    printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_slipstream_client(
    mut listen_port: ::core::ffi::c_int,
    mut server_addresses: *mut st_address_t,
    mut server_address_count: size_t,
    mut domain_name: *const ::core::ffi::c_char,
    mut cc_algo_id: *const ::core::ffi::c_char,
    mut gso: bool,
    keep_alive_interval: size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current_time: uint64_t = 0 as uint64_t;
    client_domain_name = strdup(domain_name);
    client_domain_name_len = strlen(domain_name);
    let mut mtu_d: ::core::ffi::c_double = 240 as ::core::ffi::c_int as ::core::ffi::c_double
        - client_domain_name_len as ::core::ffi::c_double;
    mtu_d = mtu_d / 1.6f64;
    let mut mtu: ::core::ffi::c_int = mtu_d as ::core::ffi::c_int;
    let mut config: picoquic_quic_config_t = st_picoquic_quic_config_t {
        nb_connections: 0,
        solution_dir: ::core::ptr::null::<::core::ffi::c_char>(),
        server_cert_file: ::core::ptr::null::<::core::ffi::c_char>(),
        server_key_file: ::core::ptr::null::<::core::ffi::c_char>(),
        log_file: ::core::ptr::null::<::core::ffi::c_char>(),
        bin_dir: ::core::ptr::null::<::core::ffi::c_char>(),
        qlog_dir: ::core::ptr::null::<::core::ffi::c_char>(),
        performance_log: ::core::ptr::null::<::core::ffi::c_char>(),
        server_port: 0,
        dest_if: 0,
        mtu_max: 0,
        initial_send_mtu_ipv4: 0,
        initial_send_mtu_ipv6: 0,
        cnx_id_length: 0,
        idle_timeout: 0,
        socket_buffer_size: 0,
        cc_algo_id: ::core::ptr::null::<::core::ffi::c_char>(),
        cnx_id_cbdata: ::core::ptr::null::<::core::ffi::c_char>(),
        spinbit_policy: picoquic_spinbit_basic,
        lossbit_policy: picoquic_lossbit_none,
        multipath_option: 0,
        multipath_alt_config: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        bdp_frame_option: 0,
        cwin_max: 0,
        address_discovery_mode: 0,
        initial_random: 0,
        use_long_log_do_preemptive_repeat_do_not_use_gso_disable_port_blocking_enable_sslkeylog: [0;
            1],
        c2rust_padding: [0; 7],
        www_dir: ::core::ptr::null::<::core::ffi::c_char>(),
        reset_seed: [0; 16],
        ticket_encryption_key: ::core::ptr::null::<uint8_t>(),
        ticket_encryption_key_length: 0,
        do_retry_has_reset_seed: [0; 1],
        c2rust_padding_0: [0; 7],
        ticket_file_name: ::core::ptr::null::<::core::ffi::c_char>(),
        token_file_name: ::core::ptr::null::<::core::ffi::c_char>(),
        sni: ::core::ptr::null::<::core::ffi::c_char>(),
        alpn: ::core::ptr::null::<::core::ffi::c_char>(),
        out_dir: ::core::ptr::null::<::core::ffi::c_char>(),
        root_trust_file: ::core::ptr::null::<::core::ffi::c_char>(),
        cipher_suite_id: 0,
        proposed_version: 0,
        desired_version: 0,
        force_zero_share_no_disk_large_client_hello: [0; 1],
        c2rust_padding_1: [0; 3],
    };
    picoquic_config_init(&raw mut config);
    config.nb_connections = 8 as uint32_t;
    config.mtu_max = mtu;
    config.initial_send_mtu_ipv4 = mtu;
    config.initial_send_mtu_ipv6 = mtu;
    config.cc_algo_id = cc_algo_id;
    config.multipath_option = 1 as ::core::ffi::c_int;
    config.set_use_long_log(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    config.set_do_preemptive_repeat(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    config.set_disable_port_blocking(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    config.set_enable_sslkeylog(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    config.alpn = SLIPSTREAM_ALPN.as_ptr();
    current_time = picoquic_current_time();
    let mut client_ctx: slipstream_client_ctx_t = st_slipstream_client_ctx_t {
        cnx: ::core::ptr::null_mut::<picoquic_cnx_t>(),
        first_stream: ::core::ptr::null_mut::<slipstream_client_stream_ctx_t>(),
        thread_ctx: ::core::ptr::null_mut::<picoquic_network_thread_ctx_t>(),
        server_addresses: ::core::ptr::null_mut::<st_address_t>(),
        server_address_count: 0,
        ready: false,
        closed: false,
        listen_sock: 0,
    };
    let mut quic: *mut picoquic_quic_t = picoquic_create_and_configure(
        &raw mut config,
        Some(
            slipstream_client_callback
                as unsafe extern "C" fn(
                    *mut picoquic_cnx_t,
                    uint64_t,
                    *mut uint8_t,
                    size_t,
                    picoquic_call_back_event_t,
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        &raw mut client_ctx as *mut ::core::ffi::c_void,
        current_time,
        ::core::ptr::null_mut::<uint64_t>(),
    );
    if quic.is_null() {
        fprintf(
            stderr,
            b"Could not create server context\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    picoquic_set_cookie_mode(quic as *mut picoquic_quic_t, 0 as ::core::ffi::c_int);
    picoquic_set_default_priority(quic as *mut picoquic_quic_t, 2 as uint8_t);
    picoquic_set_key_log_file_from_env(quic as *mut picoquic_quic_t);
    client_ctx.server_addresses = server_addresses;
    client_ctx.server_address_count = server_address_count;
    let mut cnx: *mut picoquic_cnx_t = ::core::ptr::null_mut::<picoquic_cnx_t>();
    ret = slipstream_connect(
        &raw mut (*client_ctx
            .server_addresses
            .offset(0 as ::core::ffi::c_int as isize))
        .server_address,
        quic,
        &raw mut cnx,
        &raw mut client_ctx,
    );
    if ret != 0 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"Could not connect to server\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        return -(1 as ::core::ffi::c_int);
    }
    if keep_alive_interval != 0 as size_t {
        picoquic_enable_keep_alive(
            cnx as *mut picoquic_cnx_t,
            (keep_alive_interval as uint64_t).wrapping_mul(1000 as uint64_t),
        );
    } else {
        picoquic_disable_keep_alive(cnx as *mut picoquic_cnx_t);
    }
    client_ctx.listen_sock = socket(
        AF_INET,
        SOCK_STREAM as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
    if client_ctx.listen_sock < 0 as ::core::ffi::c_int {
        perror(b"socket() failed\0".as_ptr() as *const ::core::ffi::c_char);
        exit(EXIT_FAILURE);
    }
    let mut optval: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    setsockopt(
        client_ctx.listen_sock,
        SOL_SOCKET,
        SO_REUSEADDR,
        &raw mut optval as *const ::core::ffi::c_void,
        ::core::mem::size_of::<::core::ffi::c_int>() as socklen_t,
    );
    let mut listen_addr: sockaddr_in = sockaddr_in {
        sin_family: 0 as sa_family_t,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    listen_addr.sin_family = AF_INET as sa_family_t;
    listen_addr.sin_addr.s_addr = INADDR_ANY;
    listen_addr.sin_port = __bswap_16(listen_port as __uint16_t) as in_port_t;
    if bind(
        client_ctx.listen_sock,
        &raw mut listen_addr as *mut sockaddr,
        ::core::mem::size_of::<sockaddr_in>() as socklen_t,
    ) < 0 as ::core::ffi::c_int
    {
        perror(b"bind() failed\0".as_ptr() as *const ::core::ffi::c_char);
        close(client_ctx.listen_sock);
        exit(EXIT_FAILURE);
    }
    if listen(client_ctx.listen_sock, 5 as ::core::ffi::c_int) < 0 as ::core::ffi::c_int {
        perror(b"listen() failed\0".as_ptr() as *const ::core::ffi::c_char);
        close(client_ctx.listen_sock);
        exit(EXIT_FAILURE);
    }
    printf(
        b"Listening on port %d...\n\0".as_ptr() as *const ::core::ffi::c_char,
        listen_port,
    );
    let mut param: picoquic_packet_loop_param_t = st_picoquic_packet_loop_param_t {
        local_port: 0 as uint16_t,
        local_af: 0,
        dest_if: 0,
        socket_buffer_size: 0,
        do_not_use_gso: 0,
        extra_socket_required: 0,
        simulate_eio: 0,
        send_length_max: 0,
        is_client: 0,
        decode: None,
        encode: None,
        delay_max: 0,
    };
    param.local_af = (*client_ctx
        .server_addresses
        .offset(0 as ::core::ffi::c_int as isize))
    .server_address
    .ss_family as ::core::ffi::c_int;
    param.do_not_use_gso = !gso as ::core::ffi::c_int;
    param.is_client = 1 as ::core::ffi::c_int;
    param.decode = Some(
        client_decode
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut *mut ::core::ffi::c_uchar,
                *const ::core::ffi::c_uchar,
                size_t,
                *mut sockaddr_storage,
                *mut sockaddr_storage,
            ) -> ssize_t,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut *mut ::core::ffi::c_uchar,
                *const ::core::ffi::c_uchar,
                size_t,
                *mut sockaddr_storage,
                *mut sockaddr_storage,
            ) -> ssize_t,
        >;
    param.encode = Some(
        client_encode
            as unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut *mut ::core::ffi::c_uchar,
                *const ::core::ffi::c_uchar,
                size_t,
                *mut size_t,
                *mut sockaddr_storage,
                *mut sockaddr_storage,
            ) -> ssize_t,
    )
        as Option<
            unsafe extern "C" fn(
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
                *mut *mut ::core::ffi::c_uchar,
                *const ::core::ffi::c_uchar,
                size_t,
                *mut size_t,
                *mut sockaddr_storage,
                *mut sockaddr_storage,
            ) -> ssize_t,
        >;
    let mut thread_ctx: picoquic_network_thread_ctx_t = st_picoquic_network_thread_ctx_t {
        quic: ::core::ptr::null_mut::<picoquic_quic_t>(),
        param: ::core::ptr::null_mut::<picoquic_packet_loop_param_t>(),
        loop_callback: None,
        thread_delete_fn: None,
        thread_setname_fn: None,
        thread_name: ::core::ptr::null::<::core::ffi::c_char>(),
        pthread: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        loop_callback_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        wake_up_pipe_fd: [0; 2],
        is_threaded: 0,
        wake_up_defined: 0,
        thread_is_ready: 0,
        thread_should_close: 0,
        thread_is_closed: 0,
        return_code: 0,
    };
    thread_ctx.quic = quic as *mut picoquic_quic_t;
    thread_ctx.param = &raw mut param;
    thread_ctx.loop_callback = Some(
        slipstream_client_sockloop_callback
            as unsafe extern "C" fn(
                *mut picoquic_quic_t,
                picoquic_packet_loop_cb_enum,
                *mut ::core::ffi::c_void,
                *mut ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
    ) as picoquic_packet_loop_cb_fn;
    thread_ctx.loop_callback_ctx = &raw mut client_ctx as *mut ::core::ffi::c_void;
    picoquic_open_network_wake_up(&raw mut thread_ctx, &raw mut ret);
    client_ctx.thread_ctx = &raw mut thread_ctx;
    let mut args: *mut slipstream_client_accepter_args =
        malloc(::core::mem::size_of::<slipstream_client_accepter_args>() as size_t)
            as *mut slipstream_client_accepter_args;
    (*args).fd = client_ctx.listen_sock;
    (*args).cnx = cnx;
    (*args).client_ctx = &raw mut client_ctx;
    (*args).thread_ctx = &raw mut thread_ctx;
    let mut thread: pthread_t = 0;
    if pthread_create(
        &raw mut thread,
        ::core::ptr::null::<pthread_attr_t>(),
        Some(
            slipstream_client_accepter
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        ),
        args as *mut ::core::ffi::c_void,
    ) != 0 as ::core::ffi::c_int
    {
        perror(b"pthread_create() failed for thread\0".as_ptr() as *const ::core::ffi::c_char);
        free(args as *mut ::core::ffi::c_void);
    }
    signal(
        SIGTERM,
        Some(client_sighandler as unsafe extern "C" fn(::core::ffi::c_int) -> ()),
    );
    slipstream_packet_loop(&raw mut thread_ctx);
    ret = thread_ctx.return_code;
    printf(
        b"Client exit, ret = %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        ret,
    );
    picoquic_free(quic as *mut picoquic_quic_t);
    return ret;
}

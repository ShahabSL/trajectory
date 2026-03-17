use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type st_ptls_t;
    static mut stdout: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn fputc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
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
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn picoquic_get_logging_cnxid(cnx: *mut picoquic_cnx_t) -> picoquic_connection_id_t;
    fn picoquic_cnx_is_still_logging(cnx: *mut picoquic_cnx_t) -> ::core::ffi::c_int;
    fn debug_printf_reset(suspended: ::core::ffi::c_int) -> ::core::ffi::c_int;
    static picoquic_null_connection_id: picoquic_connection_id_t;
    fn picoquic_parse_connection_id(
        bytes: *const uint8_t,
        len: uint8_t,
        cnx_id: *mut picoquic_connection_id_t,
    ) -> uint8_t;
    fn picoquic_val64_connection_id(cnx_id: picoquic_connection_id_t) -> uint64_t;
    fn picoquic_file_open(
        file_name: *const ::core::ffi::c_char,
        flags: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn picoquic_file_close(F: *mut FILE) -> *mut FILE;
    fn picoquic_frames_fixed_skip(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
        size: uint64_t,
    ) -> *const uint8_t;
    fn picoquic_frames_varint_skip(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
    ) -> *const uint8_t;
    fn picoquic_frames_varint_decode(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
        n64: *mut uint64_t,
    ) -> *const uint8_t;
    fn picoquic_varint_decode(
        bytes: *const uint8_t,
        max_bytes: size_t,
        n64: *mut uint64_t,
    ) -> size_t;
    fn picoquic_varint_skip(bytes: *const uint8_t) -> size_t;
    fn picoquic_parse_packet_header(
        quic: *mut picoquic_quic_t,
        bytes: *const uint8_t,
        length: size_t,
        addr_from: *const sockaddr,
        ph: *mut picoquic_packet_header,
        pcnx: *mut *mut picoquic_cnx_t,
        receiving: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_get_checksum_length(
        cnx: *mut picoquic_cnx_t,
        is_cleartext_mode: picoquic_epoch_enum,
    ) -> size_t;
    fn picoquic_parse_stream_header(
        bytes: *const uint8_t,
        bytes_max: size_t,
        stream_id: *mut uint64_t,
        offset: *mut uint64_t,
        data_length: *mut size_t,
        fin: *mut ::core::ffi::c_int,
        consumed: *mut size_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_parse_ack_header(
        bytes: *const uint8_t,
        bytes_max: size_t,
        num_block: *mut uint64_t,
        path_id: *mut uint64_t,
        largest: *mut uint64_t,
        ack_delay: *mut uint64_t,
        consumed: *mut size_t,
        ack_delay_exponent: uint8_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_parse_ack_frequency_frame(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
        seq: *mut uint64_t,
        packets: *mut uint64_t,
        microsec: *mut uint64_t,
        ignore_order: *mut uint8_t,
        reordering_threshold: *mut uint64_t,
    ) -> *const uint8_t;
    fn picoquic_parse_observed_address_frame(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
        ftype: uint64_t,
        sequence: *mut uint64_t,
        addr: *mut *const uint8_t,
        port: *mut uint16_t,
    ) -> *const uint8_t;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type ptls_t = st_ptls_t;
pub type ptls_iovec_t = st_ptls_iovec_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_iovec_t {
    pub base: *mut uint8_t,
    pub len: size_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _picohash_item {
    pub hash: uint64_t,
    pub next_in_bin: *mut _picohash_item,
    pub key: *const ::core::ffi::c_void,
}
pub type picohash_item = _picohash_item;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picosplay_node_t {
    pub parent: *mut st_picosplay_node_t,
    pub left: *mut st_picosplay_node_t,
    pub right: *mut st_picosplay_node_t,
}
pub type picosplay_node_t = st_picosplay_node_t;
pub type picosplay_comparator =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut ::core::ffi::c_void) -> int64_t>;
pub type picosplay_create =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut picosplay_node_t>;
pub type picosplay_delete_node =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut picosplay_node_t) -> ()>;
pub type picosplay_node_value =
    Option<unsafe extern "C" fn(*mut picosplay_node_t) -> *mut ::core::ffi::c_void>;
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
pub type picosplay_tree_t = st_picosplay_tree_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_unified_logging_t {
    pub log_quic_app_message: picoquic_log_quic_app_message_fn,
    pub log_quic_pdu: picoquic_log_quic_pdu_fn,
    pub log_quic_close: picoquic_log_quic_close,
    pub log_app_message: picoquic_log_app_message_fn,
    pub log_pdu: picoquic_log_pdu_fn,
    pub log_packet: picoquic_log_packet_fn,
    pub log_dropped_packet: picoquic_log_dropped_packet_fn,
    pub log_buffered_packet: picoquic_log_buffered_packet_fn,
    pub log_outgoing_packet: picoquic_log_outgoing_packet_fn,
    pub log_packet_lost: picoquic_log_packet_lost_fn,
    pub log_negotiated_alpn: picoquic_log_negotiated_alpn_fn,
    pub log_transport_extension: picoquic_log_transport_extension_fn,
    pub log_picotls_ticket: picoquic_log_tls_ticket_fn,
    pub log_new_connection: picoquic_log_new_connection_fn,
    pub log_close_connection: picoquic_log_close_connection_fn,
    pub log_cc_dump: picoquic_log_cc_dump_fn,
}
pub type picoquic_log_cc_dump_fn =
    Option<unsafe extern "C" fn(*mut picoquic_cnx_t, uint64_t) -> ()>;
pub type picoquic_log_close_connection_fn = Option<unsafe extern "C" fn(*mut picoquic_cnx_t) -> ()>;
pub type picoquic_log_new_connection_fn = Option<unsafe extern "C" fn(*mut picoquic_cnx_t) -> ()>;
pub type picoquic_log_tls_ticket_fn =
    Option<unsafe extern "C" fn(*mut picoquic_cnx_t, *mut uint8_t, uint16_t) -> ()>;
pub type picoquic_log_transport_extension_fn = Option<
    unsafe extern "C" fn(*mut picoquic_cnx_t, ::core::ffi::c_int, size_t, *mut uint8_t) -> (),
>;
pub type picoquic_log_negotiated_alpn_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_cnx_t,
        ::core::ffi::c_int,
        *const uint8_t,
        size_t,
        *const uint8_t,
        size_t,
        *const ptls_iovec_t,
        size_t,
    ) -> (),
>;
pub type picoquic_log_packet_lost_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_cnx_t,
        *mut picoquic_path_t,
        picoquic_packet_type_enum,
        uint64_t,
        *const ::core::ffi::c_char,
        *mut picoquic_connection_id_t,
        size_t,
        uint64_t,
    ) -> (),
>;
pub type picoquic_log_outgoing_packet_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_cnx_t,
        *mut picoquic_path_t,
        *mut uint8_t,
        uint64_t,
        size_t,
        size_t,
        *mut uint8_t,
        size_t,
        uint64_t,
    ) -> (),
>;
pub type picoquic_log_buffered_packet_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_cnx_t,
        *mut picoquic_path_t,
        picoquic_packet_type_enum,
        uint64_t,
    ) -> (),
>;
pub type picoquic_log_dropped_packet_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_cnx_t,
        *mut picoquic_path_t,
        *mut st_picoquic_packet_header_t,
        size_t,
        ::core::ffi::c_int,
        *mut uint8_t,
        uint64_t,
    ) -> (),
>;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_packet_header_t {
    pub dest_cnx_id: picoquic_connection_id_t,
    pub srce_cnx_id: picoquic_connection_id_t,
    pub pn: uint32_t,
    pub vn: uint32_t,
    pub offset: size_t,
    pub pn_offset: size_t,
    pub ptype: picoquic_packet_type_enum,
    pub pnmask: uint64_t,
    pub pn64: uint64_t,
    pub payload_length: size_t,
    pub version_index: ::core::ffi::c_int,
    pub epoch: picoquic_epoch_enum,
    pub pc: picoquic_packet_context_enum,
    #[bitfield(name = "key_phase", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "spin", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "has_spin_bit", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(
        name = "has_reserved_bit_set",
        ty = "::core::ffi::c_uint",
        bits = "3..=3"
    )]
    #[bitfield(name = "has_loss_bits", ty = "::core::ffi::c_uint", bits = "4..=4")]
    #[bitfield(name = "loss_bit_Q", ty = "::core::ffi::c_uint", bits = "5..=5")]
    #[bitfield(name = "loss_bit_L", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "quic_bit_is_zero", ty = "::core::ffi::c_uint", bits = "7..=7")]
    pub key_phase_spin_has_spin_bit_has_reserved_bit_set_has_loss_bits_loss_bit_Q_loss_bit_L_quic_bit_is_zero:
        [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub token_length: size_t,
    pub token_bytes: *const uint8_t,
    pub pl_val: size_t,
    pub l_cid: *mut st_picoquic_local_cnxid_t,
}
pub type picoquic_epoch_enum = ::core::ffi::c_uint;
pub const picoquic_epoch_1rtt: picoquic_epoch_enum = 3;
pub const picoquic_epoch_handshake: picoquic_epoch_enum = 2;
pub const picoquic_epoch_0rtt: picoquic_epoch_enum = 1;
pub const picoquic_epoch_initial: picoquic_epoch_enum = 0;
pub type picoquic_log_packet_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_cnx_t,
        *mut picoquic_path_t,
        ::core::ffi::c_int,
        uint64_t,
        *mut st_picoquic_packet_header_t,
        *const uint8_t,
        size_t,
    ) -> (),
>;
pub type picoquic_log_pdu_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_cnx_t,
        ::core::ffi::c_int,
        uint64_t,
        *const sockaddr,
        *const sockaddr,
        size_t,
    ) -> (),
>;
pub type picoquic_log_app_message_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_cnx_t,
        *const ::core::ffi::c_char,
        ::core::ffi::VaList,
    ) -> (),
>;
pub type picoquic_log_quic_close = Option<unsafe extern "C" fn(*mut picoquic_quic_t) -> ()>;
pub type picoquic_log_quic_pdu_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_quic_t,
        ::core::ffi::c_int,
        uint64_t,
        uint64_t,
        *const sockaddr,
        *const sockaddr,
        size_t,
    ) -> (),
>;
pub type picoquic_log_quic_app_message_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_quic_t,
        *const picoquic_connection_id_t,
        *const ::core::ffi::c_char,
        ::core::ffi::VaList,
    ) -> (),
>;
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
pub type picoquic_frame_type_enum_t = ::core::ffi::c_uint;
pub const picoquic_frame_type_observed_address_v6: picoquic_frame_type_enum_t = 10453415;
pub const picoquic_frame_type_observed_address_v4: picoquic_frame_type_enum_t = 10453414;
pub const picoquic_frame_type_path_blocked: picoquic_frame_type_enum_t = 354585613;
pub const picoquic_frame_type_max_path_id: picoquic_frame_type_enum_t = 354585612;
pub const picoquic_frame_type_bdp: picoquic_frame_type_enum_t = 60377;
pub const picoquic_frame_type_path_available: picoquic_frame_type_enum_t = 354585608;
pub const picoquic_frame_type_path_backup: picoquic_frame_type_enum_t = 354585607;
pub const picoquic_frame_type_path_abandon: picoquic_frame_type_enum_t = 354585605;
pub const picoquic_frame_type_path_ack_ecn: picoquic_frame_type_enum_t = 354585601;
pub const picoquic_frame_type_path_ack: picoquic_frame_type_enum_t = 354585600;
pub const picoquic_frame_type_time_stamp: picoquic_frame_type_enum_t = 757;
pub const picoquic_frame_type_immediate_ack: picoquic_frame_type_enum_t = 31;
pub const picoquic_frame_type_ack_frequency: picoquic_frame_type_enum_t = 175;
pub const picoquic_frame_type_datagram_l: picoquic_frame_type_enum_t = 49;
pub const picoquic_frame_type_datagram: picoquic_frame_type_enum_t = 48;
pub const picoquic_frame_type_handshake_done: picoquic_frame_type_enum_t = 30;
pub const picoquic_frame_type_application_close: picoquic_frame_type_enum_t = 29;
pub const picoquic_frame_type_connection_close: picoquic_frame_type_enum_t = 28;
pub const picoquic_frame_type_path_response: picoquic_frame_type_enum_t = 27;
pub const picoquic_frame_type_path_challenge: picoquic_frame_type_enum_t = 26;
pub const picoquic_frame_type_path_retire_connection_id: picoquic_frame_type_enum_t = 354585610;
pub const picoquic_frame_type_retire_connection_id: picoquic_frame_type_enum_t = 25;
pub const picoquic_frame_type_path_new_connection_id: picoquic_frame_type_enum_t = 354585609;
pub const picoquic_frame_type_new_connection_id: picoquic_frame_type_enum_t = 24;
pub const picoquic_frame_type_streams_blocked_unidir: picoquic_frame_type_enum_t = 23;
pub const picoquic_frame_type_streams_blocked_bidir: picoquic_frame_type_enum_t = 22;
pub const picoquic_frame_type_stream_data_blocked: picoquic_frame_type_enum_t = 21;
pub const picoquic_frame_type_data_blocked: picoquic_frame_type_enum_t = 20;
pub const picoquic_frame_type_max_streams_unidir: picoquic_frame_type_enum_t = 19;
pub const picoquic_frame_type_max_streams_bidir: picoquic_frame_type_enum_t = 18;
pub const picoquic_frame_type_max_stream_data: picoquic_frame_type_enum_t = 17;
pub const picoquic_frame_type_max_data: picoquic_frame_type_enum_t = 16;
pub const picoquic_frame_type_stream_range_max: picoquic_frame_type_enum_t = 15;
pub const picoquic_frame_type_stream_range_min: picoquic_frame_type_enum_t = 8;
pub const picoquic_frame_type_new_token: picoquic_frame_type_enum_t = 7;
pub const picoquic_frame_type_crypto_hs: picoquic_frame_type_enum_t = 6;
pub const picoquic_frame_type_stop_sending: picoquic_frame_type_enum_t = 5;
pub const picoquic_frame_type_reset_stream: picoquic_frame_type_enum_t = 4;
pub const picoquic_frame_type_ack_ecn: picoquic_frame_type_enum_t = 3;
pub const picoquic_frame_type_ack: picoquic_frame_type_enum_t = 2;
pub const picoquic_frame_type_poll: picoquic_frame_type_enum_t = 32;
pub const picoquic_frame_type_ping: picoquic_frame_type_enum_t = 1;
pub const picoquic_frame_type_padding: picoquic_frame_type_enum_t = 0;
pub type picoquic_packet_header = st_picoquic_packet_header_t;
pub type picoquic_tp_enum = uint64_t;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const PICOQUIC_ERROR_CLASS: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_STATELESS_RESET: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 30 as ::core::ffi::c_int;
pub const picoquic_tp_original_connection_id: picoquic_tp_enum = 0 as picoquic_tp_enum;
pub const picoquic_tp_idle_timeout: picoquic_tp_enum = 1 as picoquic_tp_enum;
pub const picoquic_tp_stateless_reset_token: picoquic_tp_enum = 2 as picoquic_tp_enum;
pub const picoquic_tp_max_packet_size: picoquic_tp_enum = 3 as picoquic_tp_enum;
pub const picoquic_tp_initial_max_data: picoquic_tp_enum = 4 as picoquic_tp_enum;
pub const picoquic_tp_initial_max_stream_data_bidi_local: picoquic_tp_enum = 5 as picoquic_tp_enum;
pub const picoquic_tp_initial_max_stream_data_bidi_remote: picoquic_tp_enum = 6 as picoquic_tp_enum;
pub const picoquic_tp_initial_max_stream_data_uni: picoquic_tp_enum = 7 as picoquic_tp_enum;
pub const picoquic_tp_initial_max_streams_bidi: picoquic_tp_enum = 8 as picoquic_tp_enum;
pub const picoquic_tp_initial_max_streams_uni: picoquic_tp_enum = 9 as picoquic_tp_enum;
pub const picoquic_tp_ack_delay_exponent: picoquic_tp_enum = 10 as picoquic_tp_enum;
pub const picoquic_tp_max_ack_delay: picoquic_tp_enum = 11 as picoquic_tp_enum;
pub const picoquic_tp_disable_migration: picoquic_tp_enum = 12 as picoquic_tp_enum;
pub const picoquic_tp_server_preferred_address: picoquic_tp_enum = 13 as picoquic_tp_enum;
pub const picoquic_tp_active_connection_id_limit: picoquic_tp_enum = 14 as picoquic_tp_enum;
pub const picoquic_tp_handshake_connection_id: picoquic_tp_enum = 15 as picoquic_tp_enum;
pub const picoquic_tp_retry_connection_id: picoquic_tp_enum = 16 as picoquic_tp_enum;
pub const picoquic_tp_max_datagram_frame_size: picoquic_tp_enum = 32 as picoquic_tp_enum;
pub const picoquic_tp_test_large_chello: picoquic_tp_enum = 3127 as picoquic_tp_enum;
pub const picoquic_tp_enable_loss_bit: picoquic_tp_enum = 4183 as picoquic_tp_enum;
pub const picoquic_tp_min_ack_delay: picoquic_tp_enum = 4278509083 as picoquic_tp_enum;
pub const picoquic_tp_enable_time_stamp: picoquic_tp_enum = 29016 as picoquic_tp_enum;
pub const picoquic_tp_grease_quic_bit: picoquic_tp_enum = 10930 as picoquic_tp_enum;
pub const picoquic_tp_version_negotiation: picoquic_tp_enum = 17 as picoquic_tp_enum;
pub const picoquic_tp_enable_bdp_frame: picoquic_tp_enum = 60377 as picoquic_tp_enum;
pub const picoquic_tp_initial_max_path_id: picoquic_tp_enum =
    1113404765106498833 as picoquic_tp_enum;
pub const picoquic_tp_address_discovery: picoquic_tp_enum = 2676072822 as picoquic_tp_enum;
unsafe extern "C" fn textlog_time(
    mut F: *mut FILE,
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
    mut label1: *const ::core::ffi::c_char,
    mut label2: *const ::core::ffi::c_char,
) {
    let mut delta_t: uint64_t = if cnx.is_null() {
        current_time
    } else {
        current_time.wrapping_sub((*cnx).start_time)
    };
    let mut time_sec: uint64_t = delta_t.wrapping_div(1000000 as uint64_t);
    let mut time_usec: uint32_t = delta_t.wrapping_rem(1000000 as uint64_t) as uint32_t;
    fprintf(
        F,
        b"%s%llu.%06d%s\0".as_ptr() as *const ::core::ffi::c_char,
        label1,
        time_sec as ::core::ffi::c_ulonglong,
        time_usec,
        label2,
    );
}
unsafe extern "C" fn textlog_prefix_initial_cid64(mut F: *mut FILE, mut log_cnxid64: uint64_t) {
    if log_cnxid64 != 0 as uint64_t {
        fprintf(
            F,
            b"%016llx: \0".as_ptr() as *const ::core::ffi::c_char,
            log_cnxid64 as ::core::ffi::c_ulonglong,
        );
    }
}
unsafe extern "C" fn textlog_address(mut F: *mut FILE, mut addr_peer: *const sockaddr) {
    if (*addr_peer).sa_family as ::core::ffi::c_int == AF_INET {
        let mut s4: *mut sockaddr_in = addr_peer as *mut sockaddr_in;
        let mut addr: *mut uint8_t = &raw mut (*s4).sin_addr as *mut uint8_t;
        fprintf(
            F,
            b"%d.%d.%d.%d:%d\0".as_ptr() as *const ::core::ffi::c_char,
            *addr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *addr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *addr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *addr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            __bswap_16((*s4).sin_port as __uint16_t) as ::core::ffi::c_int,
        );
    } else {
        let mut s6: *mut sockaddr_in6 = addr_peer as *mut sockaddr_in6;
        let mut addr_0: *mut uint8_t = &raw mut (*s6).sin6_addr as *mut uint8_t;
        fprintf(F, b"[\0".as_ptr() as *const ::core::ffi::c_char);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            if i != 0 as ::core::ffi::c_int {
                fprintf(F, b":\0".as_ptr() as *const ::core::ffi::c_char);
            }
            if *addr_0.offset((2 as ::core::ffi::c_int * i) as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                fprintf(
                    F,
                    b"%x%02x\0".as_ptr() as *const ::core::ffi::c_char,
                    *addr_0.offset((2 as ::core::ffi::c_int * i) as isize) as ::core::ffi::c_int,
                    *addr_0.offset((2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int,
                );
            } else {
                fprintf(
                    F,
                    b"%x\0".as_ptr() as *const ::core::ffi::c_char,
                    *addr_0.offset((2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int,
                );
            }
            i += 1;
        }
        fprintf(
            F,
            b"]:%d\0".as_ptr() as *const ::core::ffi::c_char,
            __bswap_16((*s6).sin6_port as __uint16_t) as ::core::ffi::c_int,
        );
    };
}
unsafe extern "C" fn textlog_packet_address(
    mut F: *mut FILE,
    mut log_cnxid64: uint64_t,
    mut cnx: *mut picoquic_cnx_t,
    mut addr_peer: *const sockaddr,
    mut receiving: ::core::ffi::c_int,
    mut length: size_t,
    mut current_time: uint64_t,
) {
    let mut delta_t: uint64_t = 0 as uint64_t;
    let mut time_sec: uint64_t = 0 as uint64_t;
    let mut time_usec: uint32_t = 0 as uint32_t;
    textlog_prefix_initial_cid64(F, log_cnxid64);
    fprintf(
        F,
        if receiving != 0 {
            b"Receiving %d bytes from \0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"Sending %d bytes to \0".as_ptr() as *const ::core::ffi::c_char
        },
        length as ::core::ffi::c_int,
    );
    textlog_address(F, addr_peer);
    if !cnx.is_null() {
        delta_t = current_time.wrapping_sub((*cnx).start_time);
        time_sec = delta_t.wrapping_div(1000000 as uint64_t);
        time_usec = delta_t.wrapping_rem(1000000 as uint64_t) as uint32_t;
    } else {
        time_sec = current_time.wrapping_div(1000000 as uint64_t);
        time_usec = current_time.wrapping_rem(1000000 as uint64_t) as uint32_t;
    }
    fprintf(
        F,
        b" at T=%llu.%06d (%llx)\n\0".as_ptr() as *const ::core::ffi::c_char,
        time_sec as ::core::ffi::c_ulonglong,
        time_usec,
        current_time as ::core::ffi::c_ulonglong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn textlog_ptype_name(
    mut ptype: picoquic_packet_type_enum,
) -> *const ::core::ffi::c_char {
    let mut ptype_name: *const ::core::ffi::c_char =
        b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
    match ptype as ::core::ffi::c_uint {
        0 => {
            ptype_name = b"error\0".as_ptr() as *const ::core::ffi::c_char;
        }
        1 => {
            ptype_name = b"version negotiation\0".as_ptr() as *const ::core::ffi::c_char;
        }
        2 => {
            ptype_name = b"initial\0".as_ptr() as *const ::core::ffi::c_char;
        }
        3 => {
            ptype_name = b"retry\0".as_ptr() as *const ::core::ffi::c_char;
        }
        4 => {
            ptype_name = b"handshake\0".as_ptr() as *const ::core::ffi::c_char;
        }
        5 => {
            ptype_name = b"0rtt protected\0".as_ptr() as *const ::core::ffi::c_char;
        }
        6 => {
            ptype_name = b"1rtt protected\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return ptype_name;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_frame_names(
    mut frame_type: uint64_t,
) -> *const ::core::ffi::c_char {
    let mut frame_name: *const ::core::ffi::c_char =
        b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
    match frame_type as picoquic_frame_type_enum_t as ::core::ffi::c_uint {
        0 => {
            frame_name = b"padding\0".as_ptr() as *const ::core::ffi::c_char;
        }
        4 => {
            frame_name = b"reset_stream\0".as_ptr() as *const ::core::ffi::c_char;
        }
        28 => {
            frame_name = b"connection_close\0".as_ptr() as *const ::core::ffi::c_char;
        }
        29 => {
            frame_name = b"application_close\0".as_ptr() as *const ::core::ffi::c_char;
        }
        16 => {
            frame_name = b"max_data\0".as_ptr() as *const ::core::ffi::c_char;
        }
        17 => {
            frame_name = b"max_stream_data\0".as_ptr() as *const ::core::ffi::c_char;
        }
        18 => {
            frame_name = b"max_streams_bidir\0".as_ptr() as *const ::core::ffi::c_char;
        }
        19 => {
            frame_name = b"max_streams_unidir\0".as_ptr() as *const ::core::ffi::c_char;
        }
        1 => {
            frame_name = b"ping\0".as_ptr() as *const ::core::ffi::c_char;
        }
        32 => {
            frame_name = b"poll\0".as_ptr() as *const ::core::ffi::c_char;
        }
        20 => {
            frame_name = b"data_blocked\0".as_ptr() as *const ::core::ffi::c_char;
        }
        21 => {
            frame_name = b"stream_data_blocked\0".as_ptr() as *const ::core::ffi::c_char;
        }
        22 => {
            frame_name = b"streams_blocked_bidir\0".as_ptr() as *const ::core::ffi::c_char;
        }
        23 => {
            frame_name = b"streams_blocked_unidir\0".as_ptr() as *const ::core::ffi::c_char;
        }
        24 => {
            frame_name = b"new_connection_id\0".as_ptr() as *const ::core::ffi::c_char;
        }
        5 => {
            frame_name = b"stop_sending\0".as_ptr() as *const ::core::ffi::c_char;
        }
        2 => {
            frame_name = b"ack\0".as_ptr() as *const ::core::ffi::c_char;
        }
        354585600 => {
            frame_name = b"path_ack\0".as_ptr() as *const ::core::ffi::c_char;
        }
        26 => {
            frame_name = b"path_challenge\0".as_ptr() as *const ::core::ffi::c_char;
        }
        27 => {
            frame_name = b"path_response\0".as_ptr() as *const ::core::ffi::c_char;
        }
        6 => {
            frame_name = b"crypto_hs\0".as_ptr() as *const ::core::ffi::c_char;
        }
        7 => {
            frame_name = b"new_token\0".as_ptr() as *const ::core::ffi::c_char;
        }
        3 => {
            frame_name = b"ack_ecn\0".as_ptr() as *const ::core::ffi::c_char;
        }
        354585601 => {
            frame_name = b"path_ack_ecn\0".as_ptr() as *const ::core::ffi::c_char;
        }
        25 => {
            frame_name = b"retire_connection_id\0".as_ptr() as *const ::core::ffi::c_char;
        }
        30 => {
            frame_name = b"handshake_done\0".as_ptr() as *const ::core::ffi::c_char;
        }
        48 | 49 => {
            frame_name = b"datagram\0".as_ptr() as *const ::core::ffi::c_char;
        }
        175 => {
            frame_name = b"ack_frequency\0".as_ptr() as *const ::core::ffi::c_char;
        }
        31 => {
            frame_name = b"immediate_ack\0".as_ptr() as *const ::core::ffi::c_char;
        }
        757 => {
            frame_name = b"time_stamp\0".as_ptr() as *const ::core::ffi::c_char;
        }
        354585605 => {
            frame_name = b"path_abandon\0".as_ptr() as *const ::core::ffi::c_char;
        }
        354585607 => {
            frame_name = b"path_backup\0".as_ptr() as *const ::core::ffi::c_char;
        }
        354585608 => {
            frame_name = b"path_available\0".as_ptr() as *const ::core::ffi::c_char;
        }
        354585612 => {
            frame_name = b"max_path_id\0".as_ptr() as *const ::core::ffi::c_char;
        }
        354585609 => {
            frame_name = b"path_new_connection_id\0".as_ptr() as *const ::core::ffi::c_char;
        }
        354585610 => {
            frame_name = b"path_retire_connection_id\0".as_ptr() as *const ::core::ffi::c_char;
        }
        354585613 => {
            frame_name = b"path_blocked\0".as_ptr() as *const ::core::ffi::c_char;
        }
        60377 => {
            frame_name = b"bdp_frame\0".as_ptr() as *const ::core::ffi::c_char;
        }
        10453414 => {
            frame_name = b"observed_address_v4\0".as_ptr() as *const ::core::ffi::c_char;
        }
        10453415 => {
            frame_name = b"observed_address_v6\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {
            if frame_type
                & !(picoquic_frame_type_stream_range_min as ::core::ffi::c_int
                    ^ picoquic_frame_type_stream_range_max as ::core::ffi::c_int)
                    as uint64_t
                == picoquic_frame_type_stream_range_min as ::core::ffi::c_int as uint64_t
            {
                frame_name = b"stream\0".as_ptr() as *const ::core::ffi::c_char;
            }
        }
    }
    return frame_name;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_tp_name(
    mut tp_number: picoquic_tp_enum,
) -> *const ::core::ffi::c_char {
    let mut tp_name: *const ::core::ffi::c_char =
        b"unknown\0".as_ptr() as *const ::core::ffi::c_char;
    match tp_number {
        0 => {
            tp_name = b"ocid\0".as_ptr() as *const ::core::ffi::c_char;
        }
        1 => {
            tp_name = b"idle_timeout\0".as_ptr() as *const ::core::ffi::c_char;
        }
        2 => {
            tp_name = b"stateless_reset_token\0".as_ptr() as *const ::core::ffi::c_char;
        }
        3 => {
            tp_name = b"max_packet_size\0".as_ptr() as *const ::core::ffi::c_char;
        }
        4 => {
            tp_name = b"initial_max_data\0".as_ptr() as *const ::core::ffi::c_char;
        }
        5 => {
            tp_name = b"max_stream_data_bidi_local\0".as_ptr() as *const ::core::ffi::c_char;
        }
        6 => {
            tp_name = b"max_stream_data_bidi_remote\0".as_ptr() as *const ::core::ffi::c_char;
        }
        7 => {
            tp_name = b"max_stream_data_uni\0".as_ptr() as *const ::core::ffi::c_char;
        }
        8 => {
            tp_name = b"max_streams_bidi\0".as_ptr() as *const ::core::ffi::c_char;
        }
        9 => {
            tp_name = b"max_streams_uni\0".as_ptr() as *const ::core::ffi::c_char;
        }
        10 => {
            tp_name = b"ack_delay_exponent\0".as_ptr() as *const ::core::ffi::c_char;
        }
        11 => {
            tp_name = b"max_ack_delay\0".as_ptr() as *const ::core::ffi::c_char;
        }
        12 => {
            tp_name = b"disable_migration\0".as_ptr() as *const ::core::ffi::c_char;
        }
        13 => {
            tp_name = b"server_preferred_address\0".as_ptr() as *const ::core::ffi::c_char;
        }
        14 => {
            tp_name = b"active_connection_id_limit\0".as_ptr() as *const ::core::ffi::c_char;
        }
        16 => {
            tp_name = b"rcid\0".as_ptr() as *const ::core::ffi::c_char;
        }
        15 => {
            tp_name = b"hcid\0".as_ptr() as *const ::core::ffi::c_char;
        }
        32 => {
            tp_name = b"max_datagram_frame_size\0".as_ptr() as *const ::core::ffi::c_char;
        }
        3127 => {
            tp_name = b"large_chello\0".as_ptr() as *const ::core::ffi::c_char;
        }
        4183 => {
            tp_name = b"enable_loss_bit\0".as_ptr() as *const ::core::ffi::c_char;
        }
        4278509083 => {
            tp_name = b"min_ack_delay\0".as_ptr() as *const ::core::ffi::c_char;
        }
        29016 => {
            tp_name = b"enable_time_stamp\0".as_ptr() as *const ::core::ffi::c_char;
        }
        10930 => {
            tp_name = b"grease_quic_bit\0".as_ptr() as *const ::core::ffi::c_char;
        }
        17 => {
            tp_name = b"version_negotiation\0".as_ptr() as *const ::core::ffi::c_char;
        }
        60377 => {
            tp_name = b"enable_bdp_frame\0".as_ptr() as *const ::core::ffi::c_char;
        }
        1113404765106498833 => {
            tp_name = b"initial_max_path_id\0".as_ptr() as *const ::core::ffi::c_char;
        }
        2676072822 => {
            tp_name = b"address_discovery\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {}
    }
    return tp_name;
}
unsafe extern "C" fn textlog_connection_id(
    mut F: *mut FILE,
    mut cid: *mut picoquic_connection_id_t,
) {
    fprintf(F, b"<\0".as_ptr() as *const ::core::ffi::c_char);
    let mut i: uint8_t = 0 as uint8_t;
    while (i as ::core::ffi::c_int) < (*cid).id_len as ::core::ffi::c_int {
        fprintf(
            F,
            b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
            (*cid).id[i as usize] as ::core::ffi::c_int,
        );
        i = i.wrapping_add(1);
    }
    fprintf(F, b">\0".as_ptr() as *const ::core::ffi::c_char);
}
unsafe extern "C" fn textlog_packet_header(
    mut F: *mut FILE,
    mut log_cnxid64: uint64_t,
    mut ph: *mut picoquic_packet_header,
    mut receiving: ::core::ffi::c_int,
) {
    textlog_prefix_initial_cid64(F, log_cnxid64);
    fprintf(
        F,
        b"%s packet type: %d (%s), \0".as_ptr() as *const ::core::ffi::c_char,
        if receiving != 0 as ::core::ffi::c_int {
            b"Receiving\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"Sending\0".as_ptr() as *const ::core::ffi::c_char
        },
        (*ph).ptype as ::core::ffi::c_uint,
        textlog_ptype_name((*ph).ptype),
    );
    fprintf(
        F,
        b"S%d,\0".as_ptr() as *const ::core::ffi::c_char,
        (*ph).spin() as ::core::ffi::c_int,
    );
    fprintf(
        F,
        b" Q%d,\0".as_ptr() as *const ::core::ffi::c_char,
        ((*ph).quic_bit_is_zero() == 0) as ::core::ffi::c_int,
    );
    match (*ph).ptype as ::core::ffi::c_uint {
        6 => {
            fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
            textlog_prefix_initial_cid64(F, log_cnxid64);
            fprintf(F, b"    \0".as_ptr() as *const ::core::ffi::c_char);
            textlog_connection_id(F, &raw mut (*ph).dest_cnx_id);
            fprintf(
                F,
                b", Seq: %d (%llu), Phi: %d,\0".as_ptr() as *const ::core::ffi::c_char,
                (*ph).pn,
                (*ph).pn64 as ::core::ffi::c_ulonglong,
                (*ph).key_phase() as ::core::ffi::c_int,
            );
            if (*ph).has_loss_bits() != 0 {
                fprintf(
                    F,
                    b" Q(%d), L(%d),\0".as_ptr() as *const ::core::ffi::c_char,
                    (*ph).loss_bit_Q() as ::core::ffi::c_int,
                    (*ph).loss_bit_L() as ::core::ffi::c_int,
                );
            }
            fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
        1 => {
            fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
            textlog_prefix_initial_cid64(F, log_cnxid64);
            fprintf(F, b"    \0".as_ptr() as *const ::core::ffi::c_char);
            textlog_connection_id(F, &raw mut (*ph).dest_cnx_id);
            fprintf(F, b", \0".as_ptr() as *const ::core::ffi::c_char);
            textlog_connection_id(F, &raw mut (*ph).srce_cnx_id);
            fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
        _ => {
            fprintf(
                F,
                b" Version %x,\0".as_ptr() as *const ::core::ffi::c_char,
                (*ph).vn,
            );
            fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
            textlog_prefix_initial_cid64(F, log_cnxid64);
            fprintf(F, b"    \0".as_ptr() as *const ::core::ffi::c_char);
            textlog_connection_id(F, &raw mut (*ph).dest_cnx_id);
            fprintf(F, b", \0".as_ptr() as *const ::core::ffi::c_char);
            textlog_connection_id(F, &raw mut (*ph).srce_cnx_id);
            fprintf(
                F,
                b", Seq: %d, pl: %zd\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*ph).pn,
                (*ph).pl_val,
            );
            if (*ph).ptype as ::core::ffi::c_uint
                == picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                textlog_prefix_initial_cid64(F, log_cnxid64);
                fprintf(
                    F,
                    b"    Token length: %zd\0".as_ptr() as *const ::core::ffi::c_char,
                    (*ph).token_length,
                );
                if (*ph).token_length > 0 as size_t {
                    let mut printed_length: size_t = if (*ph).token_length > 16 as size_t {
                        16 as size_t
                    } else {
                        (*ph).token_length
                    };
                    fprintf(F, b", Token: \0".as_ptr() as *const ::core::ffi::c_char);
                    let mut i: size_t = 0 as size_t;
                    while i < printed_length {
                        fprintf(
                            F,
                            b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                            *(*ph).token_bytes.offset(i as isize) as ::core::ffi::c_int,
                        );
                        i = i.wrapping_add(1);
                    }
                    if printed_length < (*ph).token_length {
                        fprintf(F, b"...\0".as_ptr() as *const ::core::ffi::c_char);
                    }
                }
                fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
            }
        }
    };
}
unsafe extern "C" fn textlog_negotiation_packet(
    mut F: *mut FILE,
    mut log_cnxid64: uint64_t,
    mut bytes: *const uint8_t,
    mut length: size_t,
    mut ph: *mut picoquic_packet_header,
) {
    let mut byte_index: size_t = (*ph).offset;
    let mut vn: uint32_t = 0 as uint32_t;
    textlog_prefix_initial_cid64(F, log_cnxid64);
    fprintf(
        F,
        b"    versions: \0".as_ptr() as *const ::core::ffi::c_char,
    );
    while byte_index.wrapping_add(4 as size_t) <= length {
        vn = (((*bytes
            .offset(byte_index as isize)
            .offset(0 as ::core::ffi::c_int as isize) as uint16_t
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *bytes
                .offset(byte_index as isize)
                .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int) as uint32_t)
            << 16 as ::core::ffi::c_int
            | ((*bytes
                .offset(byte_index as isize)
                .offset(2 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *bytes
                    .offset(byte_index as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t;
        byte_index = byte_index.wrapping_add(4 as size_t);
        fprintf(F, b"%02x, \0".as_ptr() as *const ::core::ffi::c_char, vn);
    }
    fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
}
unsafe extern "C" fn textlog_retry_packet(
    mut F: *mut FILE,
    mut log_cnxid64: uint64_t,
    mut bytes: *const uint8_t,
    mut ph: *mut picoquic_packet_header,
) {
    let mut byte_index: size_t = (*ph).offset;
    let mut token_length: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut payload_length: ::core::ffi::c_int = (*ph).payload_length as ::core::ffi::c_int;
    let mut is_err: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut checksum_length: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
    if checksum_length >= payload_length {
        textlog_prefix_initial_cid64(F, log_cnxid64);
        fprintf(
            F,
            b"    packet too short, checksum: %d bytes, only %d bytes available.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            checksum_length,
            payload_length,
        );
        is_err = 1 as ::core::ffi::c_int;
    } else {
        token_length = payload_length - checksum_length;
        textlog_prefix_initial_cid64(F, log_cnxid64);
        fprintf(
            F,
            b"    Token length: %d, Checksum length: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            token_length,
            checksum_length,
        );
    }
    if token_length > 0 as ::core::ffi::c_int && is_err == 0 {
        let mut printed_length: ::core::ffi::c_int = if token_length > 16 as ::core::ffi::c_int {
            16 as ::core::ffi::c_int
        } else {
            token_length
        };
        textlog_prefix_initial_cid64(F, log_cnxid64);
        fprintf(F, b"    Token: \0".as_ptr() as *const ::core::ffi::c_char);
        let mut i: uint8_t = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < printed_length {
            let c2rust_fresh0 = byte_index;
            byte_index = byte_index.wrapping_add(1);
            fprintf(
                F,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *bytes.offset(c2rust_fresh0 as isize) as ::core::ffi::c_int,
            );
            i = i.wrapping_add(1);
        }
        if printed_length < token_length {
            fprintf(F, b"...\0".as_ptr() as *const ::core::ffi::c_char);
        }
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn textlog_stream_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut byte_index: size_t = 0;
    let mut stream_id: uint64_t = 0;
    let mut data_length: size_t = 0;
    let mut offset: uint64_t = 0;
    let mut fin: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut suspended: ::core::ffi::c_int = debug_printf_reset(1 as ::core::ffi::c_int);
    ret = picoquic_parse_stream_header(
        bytes,
        bytes_max,
        &raw mut stream_id,
        &raw mut offset,
        &raw mut data_length,
        &raw mut fin,
        &raw mut byte_index,
    );
    debug_printf_reset(suspended);
    if ret != 0 as ::core::ffi::c_int {
        return bytes_max;
    }
    fprintf(
        F,
        b"    %s %lu, offset %lu, length %d, fin = %d\0".as_ptr() as *const ::core::ffi::c_char,
        textlog_frame_names(*bytes.offset(0 as ::core::ffi::c_int as isize) as uint64_t),
        stream_id,
        offset,
        data_length as ::core::ffi::c_int,
        fin,
    );
    fprintf(F, b": \0".as_ptr() as *const ::core::ffi::c_char);
    let mut i: size_t = 0 as size_t;
    while i < 8 as size_t && i < data_length {
        fprintf(
            F,
            b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
            *bytes.offset(byte_index.wrapping_add(i) as isize) as ::core::ffi::c_int,
        );
        i = i.wrapping_add(1);
    }
    fprintf(
        F,
        b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
        if data_length > 8 as size_t {
            b"...\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    return byte_index.wrapping_add(data_length);
}
#[no_mangle]
pub unsafe extern "C" fn textlog_ack_frame(
    mut F: *mut FILE,
    mut cnx_id64: uint64_t,
    mut frame_id: uint64_t,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut is_ecn: ::core::ffi::c_int,
    mut has_path_id: ::core::ffi::c_int,
) -> size_t {
    let mut byte_index: size_t = 0;
    let mut path_id: uint64_t = 0 as uint64_t;
    let mut num_block: uint64_t = 0;
    let mut largest: uint64_t = 0;
    let mut ack_delay: uint64_t = 0;
    let mut ecnx3: [uint64_t; 3] = [0; 3];
    let mut suspended: ::core::ffi::c_int = debug_printf_reset(1 as ::core::ffi::c_int);
    let mut error_detected: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = 0;
    ret = picoquic_parse_ack_header(
        bytes,
        bytes_max,
        &raw mut num_block,
        if has_path_id != 0 {
            &raw mut path_id
        } else {
            ::core::ptr::null_mut::<uint64_t>()
        },
        &raw mut largest,
        &raw mut ack_delay,
        &raw mut byte_index,
        0 as uint8_t,
    );
    debug_printf_reset(suspended);
    if ret != 0 as ::core::ffi::c_int {
        return bytes_max;
    }
    fprintf(
        F,
        b"    %s (nb=%u)\0".as_ptr() as *const ::core::ffi::c_char,
        textlog_frame_names(frame_id),
        num_block as ::core::ffi::c_int,
    );
    if has_path_id != 0 {
        fprintf(
            F,
            b", path=%lu\0".as_ptr() as *const ::core::ffi::c_char,
            path_id,
        );
    }
    while error_detected == 0 {
        let mut range: uint64_t = 0;
        let mut block_to_block: uint64_t = 0;
        if byte_index >= bytes_max {
            fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
            if cnx_id64 != 0 as uint64_t {
                fprintf(
                    F,
                    b"%lx: \0".as_ptr() as *const ::core::ffi::c_char,
                    cnx_id64,
                );
            }
            fprintf(
                F,
                b"        Malformed ACK RANGE, %d blocks remain.\0".as_ptr()
                    as *const ::core::ffi::c_char,
                num_block as ::core::ffi::c_int,
            );
            error_detected = 1 as ::core::ffi::c_int;
            break;
        } else {
            let mut l_range: size_t = picoquic_varint_decode(
                bytes.offset(byte_index as isize),
                bytes_max.wrapping_sub(byte_index),
                &raw mut range,
            );
            if l_range == 0 as size_t {
                byte_index = bytes_max;
                fprintf(
                    F,
                    b", Malformed ACK RANGE, requires %d bytes out of %d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    picoquic_varint_skip(bytes) as ::core::ffi::c_int,
                    bytes_max.wrapping_sub(byte_index) as ::core::ffi::c_int,
                );
                break;
            } else {
                byte_index = byte_index.wrapping_add(l_range);
                range = range.wrapping_add(1);
                if largest.wrapping_add(1 as uint64_t) < range {
                    fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                    if cnx_id64 != 0 as uint64_t {
                        fprintf(
                            F,
                            b"%lx: \0".as_ptr() as *const ::core::ffi::c_char,
                            cnx_id64,
                        );
                    }
                    fprintf(
                        F,
                        b"        ack range error: largest=%lu, range=%lu\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        largest,
                        range,
                    );
                    byte_index = bytes_max;
                    break;
                } else {
                    if range <= 1 as uint64_t {
                        fprintf(
                            F,
                            b", %lu\0".as_ptr() as *const ::core::ffi::c_char,
                            largest,
                        );
                    } else {
                        fprintf(
                            F,
                            b", %lu-%lu\0".as_ptr() as *const ::core::ffi::c_char,
                            largest.wrapping_sub(range).wrapping_add(1 as uint64_t),
                            largest,
                        );
                    }
                    let c2rust_fresh1 = num_block;
                    num_block = num_block.wrapping_sub(1);
                    if c2rust_fresh1 == 0 as uint64_t {
                        break;
                    }
                    if byte_index >= bytes_max {
                        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                        if cnx_id64 != 0 as uint64_t {
                            fprintf(
                                F,
                                b"%lx: \0".as_ptr() as *const ::core::ffi::c_char,
                                cnx_id64,
                            );
                        }
                        fprintf(
                            F,
                            b"        Malformed ACK GAP, %d blocks remain.\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            num_block as ::core::ffi::c_int,
                        );
                        byte_index = bytes_max;
                        error_detected = 1 as ::core::ffi::c_int;
                        break;
                    } else {
                        let mut l_gap: size_t = picoquic_varint_decode(
                            bytes.offset(byte_index as isize),
                            bytes_max.wrapping_sub(byte_index),
                            &raw mut block_to_block,
                        );
                        if l_gap == 0 as size_t {
                            byte_index = bytes_max;
                            fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                            if cnx_id64 != 0 as uint64_t {
                                fprintf(
                                    F,
                                    b"%lx: \0".as_ptr() as *const ::core::ffi::c_char,
                                    cnx_id64,
                                );
                            }
                            fprintf(
                                F,
                                b"        Malformed ACK GAP, requires %d bytes out of %d\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                picoquic_varint_skip(bytes) as ::core::ffi::c_int,
                                bytes_max.wrapping_sub(byte_index) as ::core::ffi::c_int,
                            );
                            error_detected = 1 as ::core::ffi::c_int;
                            break;
                        } else {
                            byte_index = byte_index.wrapping_add(l_gap);
                            block_to_block = block_to_block.wrapping_add(1 as uint64_t);
                            block_to_block = block_to_block.wrapping_add(range);
                            if largest < block_to_block {
                                fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                                if cnx_id64 != 0 as uint64_t {
                                    fprintf(
                                        F,
                                        b"%lx: \0".as_ptr() as *const ::core::ffi::c_char,
                                        cnx_id64,
                                    );
                                }
                                fprintf(
                                    F,
                                    b"        ack gap error: largest=%lu, range=%lu, gap=%lu\0"
                                        .as_ptr()
                                        as *const ::core::ffi::c_char,
                                    largest,
                                    range,
                                    block_to_block.wrapping_sub(range),
                                );
                                byte_index = bytes_max;
                                break;
                            } else {
                                largest = largest.wrapping_sub(block_to_block);
                            }
                        }
                    }
                }
            }
        }
    }
    if is_ecn != 0 {
        let mut ecnx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while ecnx < 3 as ::core::ffi::c_int && error_detected == 0 {
            let mut l_ecnx: size_t = picoquic_varint_decode(
                bytes.offset(byte_index as isize),
                bytes_max.wrapping_sub(byte_index),
                (&raw mut ecnx3 as *mut uint64_t).offset(ecnx as isize) as *mut uint64_t,
            );
            if l_ecnx == 0 as size_t {
                fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                if cnx_id64 != 0 as uint64_t {
                    fprintf(
                        F,
                        b"%lx: \0".as_ptr() as *const ::core::ffi::c_char,
                        cnx_id64,
                    );
                }
                fprintf(
                    F,
                    b"        incorrect ECN encoding\0".as_ptr() as *const ::core::ffi::c_char,
                );
                byte_index = bytes_max;
                error_detected = 1 as ::core::ffi::c_int;
                break;
            } else {
                byte_index = byte_index.wrapping_add(l_ecnx);
                ecnx += 1;
            }
        }
        if error_detected == 0 {
            fprintf(
                F,
                b", ect0=%llu, ect1=%llu, ce=%llu\n\0".as_ptr() as *const ::core::ffi::c_char,
                ecnx3[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_ulonglong,
                ecnx3[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_ulonglong,
                ecnx3[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_ulonglong,
            );
        } else {
            fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
    } else {
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_reset_stream_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut byte_index: size_t = 1 as size_t;
    let mut stream_id: uint64_t = 0 as uint64_t;
    let mut error_code: uint64_t = 0 as uint64_t;
    let mut offset: uint64_t = 0 as uint64_t;
    let mut l1: size_t = 0 as size_t;
    let mut l2: size_t = 0 as size_t;
    let mut l3: size_t = 0 as size_t;
    if bytes_max > 2 as size_t {
        l1 = picoquic_varint_decode(
            bytes.offset(byte_index as isize),
            bytes_max.wrapping_sub(byte_index),
            &raw mut stream_id,
        );
        byte_index = byte_index.wrapping_add(l1);
        if l1 > 0 as size_t {
            l2 = picoquic_varint_decode(
                bytes.offset(byte_index as isize),
                bytes_max.wrapping_sub(byte_index),
                &raw mut error_code,
            );
            byte_index = byte_index.wrapping_add(l2);
        }
        if l2 > 0 as size_t {
            l3 = picoquic_varint_decode(
                bytes.offset(byte_index as isize),
                bytes_max.wrapping_sub(byte_index),
                &raw mut offset,
            );
            byte_index = byte_index.wrapping_add(l3);
        }
    }
    if l1 == 0 as size_t || l2 == 0 as size_t || l3 == 0 as size_t {
        fprintf(
            F,
            b"    Malformed RESET STREAM, requires %d bytes out of %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            byte_index.wrapping_add(
                (if l1 == 0 as size_t {
                    picoquic_varint_skip(bytes.offset(1 as ::core::ffi::c_int as isize))
                        .wrapping_add(3 as size_t)
                } else {
                    picoquic_varint_skip(bytes.offset(byte_index as isize))
                }),
            ) as ::core::ffi::c_int,
            bytes_max as ::core::ffi::c_int,
        );
        byte_index = bytes_max;
    } else {
        fprintf(
            F,
            b"    %s %llu, Error 0x%08x, Offset 0x%llx.\n\0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(picoquic_frame_type_reset_stream as ::core::ffi::c_int as uint64_t),
            stream_id as ::core::ffi::c_ulonglong,
            error_code as uint32_t,
            offset as ::core::ffi::c_ulonglong,
        );
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_stop_sending_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut byte_index: size_t = 1 as size_t;
    let mut l1: size_t = 0 as size_t;
    let mut l2: size_t = 0 as size_t;
    let mut stream_id: uint64_t = 0;
    let mut error_code: uint64_t = 0 as uint64_t;
    l1 = picoquic_varint_decode(
        bytes.offset(byte_index as isize),
        bytes_max.wrapping_sub(byte_index),
        &raw mut stream_id,
    );
    if l1 != 0 as size_t {
        byte_index = byte_index.wrapping_add(l1);
        l2 = picoquic_varint_decode(
            bytes.offset(byte_index as isize),
            bytes_max.wrapping_sub(byte_index),
            &raw mut error_code,
        );
        byte_index = byte_index.wrapping_add(l2);
    }
    if l1 == 0 as size_t || l2 == 0 as size_t || byte_index > bytes_max {
        fprintf(
            F,
            b"    Malformed STOP SENDING, requires more than %d bytes\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            bytes_max as ::core::ffi::c_int,
        );
        return bytes_max;
    }
    fprintf(
        F,
        b"    %s: Stream %lld (0x%llx), Error 0x%llx.\n\0".as_ptr() as *const ::core::ffi::c_char,
        textlog_frame_names(picoquic_frame_type_stop_sending as ::core::ffi::c_int as uint64_t),
        stream_id as ::core::ffi::c_ulonglong,
        stream_id as ::core::ffi::c_ulonglong,
        error_code as ::core::ffi::c_ulonglong,
    );
    return byte_index;
}
unsafe extern "C" fn textlog_reason_text(
    mut F: *mut FILE,
    mut string_length: size_t,
    mut text_bytes: *const uint8_t,
) {
    let mut reason_string: [::core::ffi::c_char; 49] = [0; 49];
    let mut printed_length: uint64_t = if string_length > 48 as size_t {
        48 as uint64_t
    } else {
        string_length as uint64_t
    };
    let mut i: uint32_t = 0 as uint32_t;
    while (i as uint64_t) < printed_length {
        let mut c: ::core::ffi::c_int = *text_bytes.offset(i as isize) as ::core::ffi::c_int;
        if c < 0x20 as ::core::ffi::c_int || c > 0x7e as ::core::ffi::c_int {
            c = '.' as i32;
        }
        reason_string[i as usize] = c as ::core::ffi::c_char;
        i = i.wrapping_add(1);
    }
    reason_string[printed_length as usize] = 0 as ::core::ffi::c_char;
    fprintf(
        F,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut reason_string as *mut ::core::ffi::c_char,
    );
    if string_length > printed_length as size_t {
        fprintf(F, b"...\0".as_ptr() as *const ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn textlog_generic_close_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut ftype: uint8_t,
    mut cnx_id64: uint64_t,
) -> size_t {
    let mut byte_index: size_t = 1 as size_t;
    let mut error_code: uint64_t = 0 as uint64_t;
    let mut string_length: uint64_t = 0 as uint64_t;
    let mut offending_frame_type: uint64_t = 0 as uint64_t;
    let mut lf: size_t = 0 as size_t;
    let mut l1: size_t = 0 as size_t;
    let mut l0: size_t = 0 as size_t;
    if bytes_max >= 2 as size_t {
        l0 = picoquic_varint_decode(
            bytes.offset(byte_index as isize),
            bytes_max.wrapping_sub(byte_index),
            &raw mut error_code,
        );
        byte_index = byte_index.wrapping_add(l0);
        if ftype as ::core::ffi::c_int == picoquic_frame_type_connection_close as ::core::ffi::c_int
            && l0 != 0 as size_t
        {
            lf = picoquic_varint_decode(
                bytes.offset(byte_index as isize),
                bytes_max.wrapping_sub(byte_index),
                &raw mut offending_frame_type,
            );
            if lf == 0 as size_t {
                byte_index = bytes_max;
            } else {
                byte_index = byte_index.wrapping_add(lf);
            }
        }
        if ftype as ::core::ffi::c_int != picoquic_frame_type_connection_close as ::core::ffi::c_int
            || lf != 0 as size_t
        {
            l1 = picoquic_varint_decode(
                bytes.offset(byte_index as isize),
                bytes_max.wrapping_sub(byte_index),
                &raw mut string_length,
            );
        }
    }
    if l1 == 0 as size_t || l0 == 0 as size_t {
        fprintf(
            F,
            b"    Malformed %s, requires %d bytes out of %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            textlog_frame_names(ftype as uint64_t),
            byte_index.wrapping_add(picoquic_varint_skip(
                bytes.offset(3 as ::core::ffi::c_int as isize),
            )) as ::core::ffi::c_int,
            bytes_max as ::core::ffi::c_int,
        );
        byte_index = bytes_max;
    } else {
        byte_index = byte_index.wrapping_add(l1);
        fprintf(
            F,
            b"    %s, Error 0x%04x, \0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(ftype as uint64_t),
            error_code as uint16_t as ::core::ffi::c_int,
        );
        if ftype as ::core::ffi::c_int == picoquic_frame_type_connection_close as ::core::ffi::c_int
            && offending_frame_type != 0 as uint64_t
        {
            fprintf(
                F,
                b"Offending frame 0x%llx, \0".as_ptr() as *const ::core::ffi::c_char,
                offending_frame_type as ::core::ffi::c_ulonglong,
            );
        }
        fprintf(
            F,
            b"Reason length %llu\n\0".as_ptr() as *const ::core::ffi::c_char,
            string_length as ::core::ffi::c_ulonglong,
        );
        if byte_index.wrapping_add(string_length as size_t) > bytes_max {
            fprintf(
                F,
                b"    Malformed %s, requires %llu bytes out of %llu\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                textlog_frame_names(ftype as uint64_t),
                byte_index.wrapping_add(string_length as size_t) as ::core::ffi::c_ulonglong,
                bytes_max as ::core::ffi::c_ulonglong,
            );
            byte_index = bytes_max;
        } else if string_length > 0 as uint64_t {
            if cnx_id64 != 0 as uint64_t {
                fprintf(
                    F,
                    b"%lx: \0".as_ptr() as *const ::core::ffi::c_char,
                    cnx_id64,
                );
            }
            fprintf(
                F,
                b"        Reason: \0".as_ptr() as *const ::core::ffi::c_char,
            );
            textlog_reason_text(
                F,
                string_length as size_t,
                bytes.offset(byte_index as isize),
            );
            fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
            byte_index = byte_index.wrapping_add(string_length as size_t);
        }
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_max_data_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut byte_index: size_t = 1 as size_t;
    let mut max_data: uint64_t = 0;
    let mut l1: size_t = picoquic_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max.wrapping_sub(1 as size_t),
        &raw mut max_data,
    );
    if (1 as size_t).wrapping_add(l1) > bytes_max {
        fprintf(
            F,
            b"    Malformed MAX DATA, requires %d bytes out of %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            (1 as size_t).wrapping_add(l1) as ::core::ffi::c_int,
            bytes_max as ::core::ffi::c_int,
        );
        return bytes_max;
    } else {
        byte_index = (1 as size_t).wrapping_add(l1);
    }
    fprintf(
        F,
        b"    %s: 0x%llx.\n\0".as_ptr() as *const ::core::ffi::c_char,
        textlog_frame_names(picoquic_frame_type_max_data as ::core::ffi::c_int as uint64_t),
        max_data as ::core::ffi::c_ulonglong,
    );
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_max_stream_data_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut byte_index: size_t = 1 as size_t;
    let mut stream_id: uint64_t = 0;
    let mut max_data: uint64_t = 0;
    let mut l1: size_t = picoquic_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max.wrapping_sub(1 as size_t),
        &raw mut stream_id,
    );
    let mut l2: size_t = picoquic_varint_decode(
        bytes
            .offset(1 as ::core::ffi::c_int as isize)
            .offset(l1 as isize),
        bytes_max.wrapping_sub(1 as size_t).wrapping_sub(l1),
        &raw mut max_data,
    );
    if l1 == 0 as size_t || l2 == 0 as size_t {
        fprintf(
            F,
            b"    Malformed MAX STREAM DATA, requires %d bytes out of %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            (1 as size_t).wrapping_add(l1).wrapping_add(l2) as ::core::ffi::c_int,
            bytes_max as ::core::ffi::c_int,
        );
        return bytes_max;
    } else {
        byte_index = (1 as size_t).wrapping_add(l1).wrapping_add(l2);
    }
    fprintf(
        F,
        b"    %s, Stream: %lu, max data: 0x%llx.\n\0".as_ptr() as *const ::core::ffi::c_char,
        textlog_frame_names(picoquic_frame_type_max_stream_data as ::core::ffi::c_int as uint64_t),
        stream_id,
        max_data as ::core::ffi::c_ulonglong,
    );
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_max_stream_id_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut frame_id: uint64_t,
) -> size_t {
    let mut byte_index: size_t = 1 as size_t;
    let min_size: size_t = (1 as size_t).wrapping_add(picoquic_varint_skip(
        bytes.offset(1 as ::core::ffi::c_int as isize),
    ) as size_t);
    let mut rank: uint64_t = 0;
    if min_size > bytes_max {
        fprintf(
            F,
            b"    Malformed %s, requires %d bytes out of %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            textlog_frame_names(frame_id),
            min_size as ::core::ffi::c_int,
            bytes_max as ::core::ffi::c_int,
        );
        return bytes_max;
    }
    byte_index = byte_index.wrapping_add(picoquic_varint_decode(
        bytes.offset(byte_index as isize),
        bytes_max.wrapping_sub(byte_index),
        &raw mut rank,
    ));
    fprintf(
        F,
        b"    %s: max rank %lu.\n\0".as_ptr() as *const ::core::ffi::c_char,
        textlog_frame_names(frame_id),
        rank,
    );
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_blocked_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut byte_index: size_t = 1 as size_t;
    let min_size: size_t = (1 as size_t).wrapping_add(picoquic_varint_skip(
        bytes.offset(1 as ::core::ffi::c_int as isize),
    ) as size_t);
    let mut blocked_offset: uint64_t = 0 as uint64_t;
    if min_size > bytes_max {
        fprintf(
            F,
            b"    Malformed %s, requires %d bytes out of %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            textlog_frame_names(picoquic_frame_type_data_blocked as ::core::ffi::c_int as uint64_t),
            min_size as ::core::ffi::c_int,
            bytes_max as ::core::ffi::c_int,
        );
        return bytes_max;
    }
    byte_index = byte_index.wrapping_add(picoquic_varint_decode(
        bytes.offset(byte_index as isize),
        bytes_max.wrapping_sub(byte_index),
        &raw mut blocked_offset,
    ));
    fprintf(
        F,
        b"    %s: offset %lu.\n\0".as_ptr() as *const ::core::ffi::c_char,
        textlog_frame_names(picoquic_frame_type_data_blocked as ::core::ffi::c_int as uint64_t),
        blocked_offset,
    );
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_stream_blocked_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut byte_index: size_t = 1 as size_t;
    let min_size: size_t = (1 as size_t).wrapping_add(picoquic_varint_skip(
        bytes.offset(1 as ::core::ffi::c_int as isize),
    ) as size_t);
    let mut blocked_stream_id: uint64_t = 0;
    if min_size > bytes_max {
        fprintf(
            F,
            b"    Malformed STREAM BLOCKED, requires %d bytes out of %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            min_size as ::core::ffi::c_int,
            bytes_max as ::core::ffi::c_int,
        );
        return bytes_max;
    }
    byte_index = byte_index.wrapping_add(picoquic_varint_decode(
        bytes.offset(byte_index as isize),
        bytes_max.wrapping_sub(byte_index),
        &raw mut blocked_stream_id,
    ));
    byte_index = byte_index.wrapping_add(picoquic_varint_skip(
        bytes.offset(byte_index as isize) as *const uint8_t
    ));
    fprintf(
        F,
        b"    %s: %lu.\n\0".as_ptr() as *const ::core::ffi::c_char,
        textlog_frame_names(
            picoquic_frame_type_stream_data_blocked as ::core::ffi::c_int as uint64_t,
        ),
        blocked_stream_id,
    );
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_streams_blocked_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut frame_id: uint64_t,
) -> size_t {
    let mut byte_index: size_t = 1 as size_t;
    let min_size: size_t = (1 as size_t).wrapping_add(picoquic_varint_skip(
        bytes.offset(1 as ::core::ffi::c_int as isize),
    ) as size_t);
    let mut blocked_stream_rank: uint64_t = 0;
    if min_size > bytes_max {
        fprintf(
            F,
            b"    Malformed %s frame, requires %d bytes out of %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            textlog_frame_names(frame_id),
            min_size as ::core::ffi::c_int,
            bytes_max as ::core::ffi::c_int,
        );
        byte_index = bytes_max;
    } else {
        byte_index = byte_index.wrapping_add(picoquic_varint_decode(
            bytes.offset(byte_index as isize),
            bytes_max.wrapping_sub(byte_index),
            &raw mut blocked_stream_rank,
        ));
        fprintf(
            F,
            b"    %s: %lld\n\0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(frame_id),
            blocked_stream_rank as ::core::ffi::c_ulonglong,
        );
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_new_connection_id_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut is_mpath: ::core::ffi::c_int,
) -> size_t {
    let mut byte_index: size_t = 0;
    let mut min_size: size_t =
        (2 as ::core::ffi::c_uint).wrapping_add(16 as ::core::ffi::c_uint) as size_t;
    let mut sequence: uint64_t = 0;
    let mut retire_before: uint64_t = 0 as uint64_t;
    let mut path_id: uint64_t = 0 as uint64_t;
    let mut new_cnx_id: picoquic_connection_id_t = picoquic_null_connection_id;
    let mut l_cid: uint8_t = 0 as uint8_t;
    let mut l_seq: size_t = 0 as size_t;
    let mut l_ret: size_t = 1 as size_t;
    let mut l_path_id: size_t = 0 as size_t;
    byte_index = picoquic_varint_skip(bytes);
    if is_mpath != 0 {
        l_path_id = picoquic_varint_decode(
            bytes.offset(byte_index as isize) as *const uint8_t,
            bytes_max,
            &raw mut path_id,
        );
        min_size = min_size.wrapping_add(l_path_id);
        byte_index = byte_index.wrapping_add(l_path_id);
    }
    l_seq = picoquic_varint_decode(
        bytes.offset(byte_index as isize) as *const uint8_t,
        bytes_max,
        &raw mut sequence,
    );
    min_size = min_size.wrapping_add(l_seq);
    byte_index = byte_index.wrapping_add(l_seq);
    l_ret = picoquic_varint_decode(
        bytes.offset(byte_index as isize) as *const uint8_t,
        bytes_max,
        &raw mut retire_before,
    );
    min_size = min_size.wrapping_add(l_ret);
    byte_index = byte_index.wrapping_add(l_ret);
    if byte_index < bytes_max {
        let c2rust_fresh2 = byte_index;
        byte_index = byte_index.wrapping_add(1);
        l_cid = *bytes.offset(c2rust_fresh2 as isize);
    }
    min_size = min_size.wrapping_add(l_cid as size_t);
    if l_seq == 0 as size_t || l_ret == 0 as size_t || min_size > bytes_max {
        fprintf(
            F,
            b"    Malformed %sNEW CONNECTION ID, requires %d bytes out of %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            if is_mpath != 0 {
                b"PATH \0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            min_size as ::core::ffi::c_int,
            bytes_max as ::core::ffi::c_int,
        );
        byte_index = bytes_max;
    } else {
        byte_index = byte_index.wrapping_add(picoquic_parse_connection_id(
            bytes.offset(byte_index as isize),
            l_cid,
            &raw mut new_cnx_id,
        ) as size_t);
        if is_mpath != 0 {
            fprintf(
                F,
                b"    %s[%lu, %lu]: 0x\0".as_ptr() as *const ::core::ffi::c_char,
                textlog_frame_names(
                    picoquic_frame_type_path_retire_connection_id as ::core::ffi::c_int as uint64_t,
                ),
                path_id,
                sequence,
            );
        } else {
            fprintf(
                F,
                b"    %s[%lu]: 0x\0".as_ptr() as *const ::core::ffi::c_char,
                textlog_frame_names(
                    picoquic_frame_type_retire_connection_id as ::core::ffi::c_int as uint64_t,
                ),
                sequence,
            );
        }
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < new_cnx_id.id_len as ::core::ffi::c_int {
            fprintf(
                F,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                new_cnx_id.id[x as usize] as ::core::ffi::c_int,
            );
            x += 1;
        }
        fprintf(F, b", \0".as_ptr() as *const ::core::ffi::c_char);
        let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_0 < 16 as ::core::ffi::c_int {
            let c2rust_fresh3 = byte_index;
            byte_index = byte_index.wrapping_add(1);
            fprintf(
                F,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *bytes.offset(c2rust_fresh3 as isize) as ::core::ffi::c_int,
            );
            x_0 += 1;
        }
        if retire_before != 0 as uint64_t {
            fprintf(
                F,
                b", retire before: %lu\0".as_ptr() as *const ::core::ffi::c_char,
                retire_before,
            );
        }
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_retire_connection_id_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut is_mpath: ::core::ffi::c_int,
) -> size_t {
    let mut byte_index: size_t = 0;
    let mut sequence: uint64_t = 0 as uint64_t;
    let mut path_id: uint64_t = 0 as uint64_t;
    let mut l_seq: size_t = 0 as size_t;
    let mut l_path_id: size_t = 0 as size_t;
    byte_index = picoquic_varint_skip(bytes);
    if is_mpath != 0 {
        l_path_id = picoquic_varint_decode(
            bytes.offset(byte_index as isize),
            bytes_max,
            &raw mut path_id,
        );
        byte_index = byte_index.wrapping_add(l_path_id);
    }
    if bytes_max > byte_index {
        l_seq = picoquic_varint_decode(
            bytes.offset(byte_index as isize),
            bytes_max.wrapping_sub(byte_index),
            &raw mut sequence,
        );
        byte_index = byte_index.wrapping_add(l_seq);
    }
    if l_seq == 0 as size_t || byte_index > bytes_max {
        fprintf(
            F,
            b"    Malformed %sRETIRE CONNECTION ID, requires %d bytes out of %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            if is_mpath != 0 {
                b"PATH \0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
            byte_index.wrapping_add(
                (if l_seq == 0 as size_t {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as size_t,
            ) as ::core::ffi::c_int,
            bytes_max as ::core::ffi::c_int,
        );
        byte_index = bytes_max;
    } else if is_mpath != 0 {
        fprintf(
            F,
            b"    %s[%lu, %lu]\n\0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(
                picoquic_frame_type_path_retire_connection_id as ::core::ffi::c_int as uint64_t,
            ),
            path_id,
            sequence,
        );
    } else {
        fprintf(
            F,
            b"    %s[%lu]\n\0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(
                picoquic_frame_type_retire_connection_id as ::core::ffi::c_int as uint64_t,
            ),
            sequence,
        );
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_new_token_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut byte_index: size_t = 1 as size_t;
    let mut min_size: uint64_t = 1 as uint64_t;
    let mut l_toklen: size_t = 0 as size_t;
    let mut toklen: uint64_t = 0 as uint64_t;
    l_toklen = picoquic_varint_decode(
        bytes.offset(byte_index as isize) as *const uint8_t,
        bytes_max,
        &raw mut toklen,
    );
    min_size = (min_size as ::core::ffi::c_ulong)
        .wrapping_add(l_toklen.wrapping_add(toklen as size_t) as ::core::ffi::c_ulong)
        as uint64_t as uint64_t;
    if l_toklen == 0 as size_t || min_size > bytes_max as uint64_t {
        fprintf(
            F,
            b"    Malformed %s, requires %d bytes out of %d\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            textlog_frame_names(picoquic_frame_type_new_token as ::core::ffi::c_int as uint64_t),
            min_size as ::core::ffi::c_int,
            bytes_max as ::core::ffi::c_int,
        );
        return bytes_max;
    } else {
        byte_index = byte_index.wrapping_add(l_toklen);
        fprintf(
            F,
            b"    %s[%d]: 0x\0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(picoquic_frame_type_new_token as ::core::ffi::c_int as uint64_t),
            toklen as ::core::ffi::c_int,
        );
        let mut x: uint64_t = 0 as uint64_t;
        while x < toklen && x < 16 as uint64_t {
            fprintf(
                F,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *bytes.offset(byte_index.wrapping_add(x as size_t) as isize) as ::core::ffi::c_int,
            );
            x = x.wrapping_add(1);
        }
        byte_index = byte_index.wrapping_add(toklen as size_t);
        if toklen > 16 as uint64_t {
            fprintf(F, b"...\0".as_ptr() as *const ::core::ffi::c_char);
        }
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_path_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut byte_index: size_t = 1 as size_t;
    let mut challenge_length: size_t = 8 as size_t;
    if byte_index.wrapping_add(challenge_length) > bytes_max {
        fprintf(
            F,
            b"    Malformed %s frame, %d bytes needed, %d available\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            textlog_frame_names(*bytes.offset(0 as ::core::ffi::c_int as isize) as uint64_t),
            challenge_length.wrapping_add(1 as size_t) as ::core::ffi::c_int,
            bytes_max as ::core::ffi::c_int,
        );
        byte_index = bytes_max;
    } else {
        fprintf(
            F,
            b"    %s: \0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(*bytes.offset(0 as ::core::ffi::c_int as isize) as uint64_t),
        );
        let mut i: size_t = 0 as size_t;
        while i < challenge_length {
            fprintf(
                F,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *bytes.offset(byte_index.wrapping_add(i) as isize) as ::core::ffi::c_int,
            );
            i = i.wrapping_add(1);
        }
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        byte_index = byte_index.wrapping_add(challenge_length);
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_crypto_hs_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut offset: uint64_t = 0 as uint64_t;
    let mut data_length: uint64_t = 0 as uint64_t;
    let mut byte_index: size_t = 1 as size_t;
    let mut l_off: size_t = 0 as size_t;
    let mut l_len: size_t = 0 as size_t;
    if bytes_max > byte_index {
        l_off = picoquic_varint_decode(
            bytes.offset(byte_index as isize),
            bytes_max.wrapping_sub(byte_index),
            &raw mut offset,
        );
        byte_index = byte_index.wrapping_add(l_off);
    }
    if bytes_max > byte_index {
        l_len = picoquic_varint_decode(
            bytes.offset(byte_index as isize),
            bytes_max.wrapping_sub(byte_index),
            &raw mut data_length,
        );
        byte_index = byte_index.wrapping_add(l_len);
    }
    if l_off == 0 as size_t
        || l_len == 0 as size_t
        || byte_index.wrapping_add(data_length as size_t) > bytes_max
    {
        fprintf(
            F,
            b"    Malformed %s frame.\n\0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(picoquic_frame_type_crypto_hs as ::core::ffi::c_int as uint64_t),
        );
        byte_index = bytes_max;
    } else {
        fprintf(
            F,
            b"    %s, offset %lu, length %d\0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(picoquic_frame_type_crypto_hs as ::core::ffi::c_int as uint64_t),
            offset,
            data_length as ::core::ffi::c_int,
        );
        fprintf(F, b": \0".as_ptr() as *const ::core::ffi::c_char);
        let mut i: size_t = 0 as size_t;
        while i < 8 as size_t && i < data_length as size_t {
            fprintf(
                F,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *bytes.offset(byte_index.wrapping_add(i) as isize) as ::core::ffi::c_int,
            );
            i = i.wrapping_add(1);
        }
        fprintf(
            F,
            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            if data_length > 8 as uint64_t {
                b"...\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        byte_index = byte_index.wrapping_add(data_length as size_t);
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_datagram_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut frame_id: uint64_t,
) -> size_t {
    let mut has_length: ::core::ffi::c_uint = (frame_id & 1 as uint64_t) as ::core::ffi::c_uint;
    let mut l_l: size_t = 0 as size_t;
    let mut length: uint64_t = 0 as uint64_t;
    let mut byte_index: size_t = 1 as size_t;
    if has_length != 0 {
        if bytes_max > byte_index {
            l_l = picoquic_varint_decode(
                bytes.offset(byte_index as isize),
                bytes_max.wrapping_sub(byte_index),
                &raw mut length,
            );
            byte_index = byte_index.wrapping_add(l_l);
        }
    } else {
        length = bytes_max.wrapping_sub(byte_index) as uint64_t;
    }
    if has_length != 0 && l_l == 0 as size_t
        || byte_index.wrapping_add(length as size_t) > bytes_max
    {
        fprintf(
            F,
            b"    Malformed %s: \0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(frame_id),
        );
        let mut i: size_t = 0 as size_t;
        while i < bytes_max && i < 8 as size_t {
            fprintf(
                F,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *bytes.offset(i as isize) as ::core::ffi::c_int,
            );
            i = i.wrapping_add(1);
        }
        if bytes_max > 8 as size_t {
            fprintf(F, b"...\0".as_ptr() as *const ::core::ffi::c_char);
        }
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        byte_index = bytes_max;
    } else {
        fprintf(
            F,
            b"    %s\0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(frame_id),
        );
        fprintf(
            F,
            b", length: %d: \0".as_ptr() as *const ::core::ffi::c_char,
            length as ::core::ffi::c_int,
        );
        let mut i_0: size_t = 0 as size_t;
        while i_0 < 8 as size_t && i_0 < length as size_t {
            fprintf(
                F,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *bytes.offset(byte_index.wrapping_add(i_0) as isize) as ::core::ffi::c_int,
            );
            i_0 = i_0.wrapping_add(1);
        }
        fprintf(
            F,
            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            if length > 8 as uint64_t {
                b"...\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        byte_index = byte_index.wrapping_add(length as size_t);
    }
    return byte_index;
}
unsafe extern "C" fn textlog_ack_frequency_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut sequence: uint64_t = 0 as uint64_t;
    let mut packets: uint64_t = 0 as uint64_t;
    let mut microsecs: uint64_t = 0 as uint64_t;
    let mut ignore_order: uint8_t = 0 as uint8_t;
    let mut bytes_end: *const uint8_t = bytes.offset(bytes_max as isize);
    let mut bytes0: *const uint8_t = bytes;
    let mut byte_index: size_t = 0 as size_t;
    let mut reordering_threshold: uint64_t = 0 as uint64_t;
    bytes = picoquic_frames_varint_skip(bytes, bytes_end);
    if bytes.is_null() || {
        bytes = picoquic_parse_ack_frequency_frame(
            bytes,
            bytes_end,
            &raw mut sequence,
            &raw mut packets,
            &raw mut microsecs,
            &raw mut ignore_order,
            &raw mut reordering_threshold,
        );
        bytes.is_null()
    } {
        fprintf(
            F,
            b"    Malformed %s frame: \0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(
                picoquic_frame_type_ack_frequency as ::core::ffi::c_int as uint64_t,
            ),
        );
        let mut i: size_t = 0 as size_t;
        while i < bytes_max && i < 8 as size_t {
            fprintf(
                F,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *bytes0.offset(i as isize) as ::core::ffi::c_int,
            );
            i = i.wrapping_add(1);
        }
        if bytes_max > 8 as size_t {
            fprintf(F, b"...\0".as_ptr() as *const ::core::ffi::c_char);
        }
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        byte_index = bytes_max;
    } else {
        fprintf(
            F,
            b"    %s: S=%lu, P=%lu, uS=%lu, Reordering threshold: %lu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            textlog_frame_names(
                picoquic_frame_type_ack_frequency as ::core::ffi::c_int as uint64_t,
            ),
            sequence,
            packets,
            microsecs,
            reordering_threshold,
        );
        byte_index = bytes.offset_from(bytes0) as ::core::ffi::c_long as size_t;
    }
    return byte_index;
}
unsafe extern "C" fn textlog_immediate_ack_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut bytes0: *const uint8_t = bytes;
    let mut byte_index: size_t = 0 as size_t;
    bytes = picoquic_frames_varint_skip(bytes, bytes.offset(bytes_max as isize));
    if !bytes.is_null() {
        fprintf(
            F,
            b"    %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(
                picoquic_frame_type_immediate_ack as ::core::ffi::c_int as uint64_t,
            ),
        );
        byte_index = bytes.offset_from(bytes0) as ::core::ffi::c_long as size_t;
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_time_stamp_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut time_stamp: uint64_t = 0 as uint64_t;
    let mut bytes_end: *const uint8_t = bytes.offset(bytes_max as isize);
    let mut bytes0: *const uint8_t = bytes;
    let mut byte_index: size_t = 0 as size_t;
    bytes = picoquic_frames_varint_skip(bytes, bytes_end);
    if bytes.is_null() || {
        bytes = picoquic_frames_varint_decode(bytes, bytes_end, &raw mut time_stamp);
        bytes.is_null()
    } {
        fprintf(
            F,
            b"    Malformed %s frame: \0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(picoquic_frame_type_time_stamp as ::core::ffi::c_int as uint64_t),
        );
        let mut i: size_t = 0 as size_t;
        while i < bytes_max && i < 8 as size_t {
            fprintf(
                F,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *bytes0.offset(i as isize) as ::core::ffi::c_int,
            );
            i = i.wrapping_add(1);
        }
        if bytes_max > 8 as size_t {
            fprintf(F, b"...\0".as_ptr() as *const ::core::ffi::c_char);
        }
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        byte_index = bytes_max;
    } else {
        fprintf(
            F,
            b"    %s: %lu\n\0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(picoquic_frame_type_time_stamp as ::core::ffi::c_int as uint64_t),
            time_stamp,
        );
        byte_index = bytes.offset_from(bytes0) as ::core::ffi::c_long as size_t;
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_path_abandon_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut bytes_end: *const uint8_t = bytes.offset(bytes_max as isize);
    let mut bytes0: *const uint8_t = bytes;
    let mut path_id: uint64_t = 0;
    let mut reason: uint64_t = 0;
    let mut byte_index: size_t = 0 as size_t;
    bytes = picoquic_frames_varint_skip(bytes, bytes_end);
    if bytes.is_null()
        || {
            bytes = picoquic_frames_varint_decode(bytes, bytes_end, &raw mut path_id);
            bytes.is_null()
        }
        || {
            bytes = picoquic_frames_varint_decode(bytes, bytes_end, &raw mut reason);
            bytes.is_null()
        }
    {
        fprintf(
            F,
            b"    Malformed %s frame: \0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(picoquic_frame_type_path_abandon as ::core::ffi::c_int as uint64_t),
        );
        let mut i: size_t = 0 as size_t;
        while i < bytes_max && i < 8 as size_t {
            fprintf(
                F,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *bytes0.offset(i as isize) as ::core::ffi::c_int,
            );
            i = i.wrapping_add(1);
        }
        if bytes_max > 8 as size_t {
            fprintf(F, b"...\0".as_ptr() as *const ::core::ffi::c_char);
        }
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        byte_index = bytes_max;
    } else {
        fprintf(
            F,
            b"    %s, path_id: %lu\0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(picoquic_frame_type_path_abandon as ::core::ffi::c_int as uint64_t),
            path_id,
        );
        fprintf(
            F,
            b", reason: %lu\0".as_ptr() as *const ::core::ffi::c_char,
            reason,
        );
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        byte_index = bytes.offset_from(bytes0) as ::core::ffi::c_long as size_t;
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_path_available_or_standby_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut bytes_end: *const uint8_t = bytes.offset(bytes_max as isize);
    let mut bytes0: *const uint8_t = bytes;
    let mut frame_id: uint64_t = 0 as uint64_t;
    let mut path_id: uint64_t = 0;
    let mut sequence: uint64_t = 0;
    let mut byte_index: size_t = 0 as size_t;
    bytes = picoquic_frames_varint_decode(bytes, bytes_end, &raw mut frame_id);
    if bytes.is_null()
        || {
            bytes = picoquic_frames_varint_decode(bytes, bytes_end, &raw mut path_id);
            bytes.is_null()
        }
        || {
            bytes = picoquic_frames_varint_decode(bytes, bytes_end, &raw mut sequence);
            bytes.is_null()
        }
    {
        fprintf(
            F,
            b"    Malformed %s frame: \0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(frame_id),
        );
        let mut i: size_t = 0 as size_t;
        while i < bytes_max && i < 8 as size_t {
            fprintf(
                F,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *bytes0.offset(i as isize) as ::core::ffi::c_int,
            );
            i = i.wrapping_add(1);
        }
        if bytes_max > 8 as size_t {
            fprintf(F, b"...\0".as_ptr() as *const ::core::ffi::c_char);
        }
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        byte_index = bytes_max;
    } else {
        fprintf(
            F,
            b"    %s, path_id: %lu\0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(frame_id),
            path_id,
        );
        fprintf(
            F,
            b", sequence: %lu\0".as_ptr() as *const ::core::ffi::c_char,
            sequence,
        );
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        byte_index = bytes.offset_from(bytes0) as ::core::ffi::c_long as size_t;
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_max_path_id_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut bytes_end: *const uint8_t = bytes.offset(bytes_max as isize);
    let mut bytes0: *const uint8_t = bytes;
    let mut frame_id: uint64_t = 0 as uint64_t;
    let mut max_path_id: uint64_t = 0;
    let mut byte_index: size_t = 0 as size_t;
    bytes = picoquic_frames_varint_decode(bytes, bytes_end, &raw mut frame_id);
    if bytes.is_null() || {
        bytes = picoquic_frames_varint_decode(bytes, bytes_end, &raw mut max_path_id);
        bytes.is_null()
    } {
        fprintf(
            F,
            b"    Malformed %s frame: \0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(frame_id),
        );
        let mut i: size_t = 0 as size_t;
        while i < bytes_max && i < 8 as size_t {
            fprintf(
                F,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *bytes0.offset(i as isize) as ::core::ffi::c_int,
            );
            i = i.wrapping_add(1);
        }
        if bytes_max > 8 as size_t {
            fprintf(F, b"...\0".as_ptr() as *const ::core::ffi::c_char);
        }
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        byte_index = bytes_max;
    } else {
        fprintf(
            F,
            b"    %s, max_path_id: %lu\n\0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(picoquic_frame_type_max_path_id as ::core::ffi::c_int as uint64_t),
            max_path_id,
        );
        byte_index = bytes.offset_from(bytes0) as ::core::ffi::c_long as size_t;
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_path_blocked_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut bytes_end: *const uint8_t = bytes.offset(bytes_max as isize);
    let mut bytes0: *const uint8_t = bytes;
    let mut frame_id: uint64_t = 0 as uint64_t;
    let mut max_path_id: uint64_t = 0;
    let mut byte_index: size_t = 0 as size_t;
    bytes = picoquic_frames_varint_decode(bytes, bytes_end, &raw mut frame_id);
    if bytes.is_null() || {
        bytes = picoquic_frames_varint_decode(bytes, bytes_end, &raw mut max_path_id);
        bytes.is_null()
    } {
        fprintf(
            F,
            b"    Malformed %s frame: \0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(frame_id),
        );
        let mut i: size_t = 0 as size_t;
        while i < bytes_max && i < 8 as size_t {
            fprintf(
                F,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *bytes0.offset(i as isize) as ::core::ffi::c_int,
            );
            i = i.wrapping_add(1);
        }
        if bytes_max > 8 as size_t {
            fprintf(F, b"...\0".as_ptr() as *const ::core::ffi::c_char);
        }
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        byte_index = bytes_max;
    } else {
        fprintf(
            F,
            b"    %s, max_path_id: %lu\n\0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(picoquic_frame_type_path_blocked as ::core::ffi::c_int as uint64_t),
            max_path_id,
        );
        byte_index = bytes.offset_from(bytes0) as ::core::ffi::c_long as size_t;
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_bdp_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut bytes_end: *const uint8_t = bytes.offset(bytes_max as isize);
    let mut bytes0: *const uint8_t = bytes;
    let mut lifetime: uint64_t = 0;
    let mut recon_bytes_in_flight: uint64_t = 0;
    let mut recon_min_rtt: uint64_t = 0;
    let mut ip_length: uint64_t = 0;
    let mut byte_index: size_t = 0 as size_t;
    bytes = picoquic_frames_varint_skip(bytes, bytes_end);
    if bytes.is_null()
        || {
            bytes = picoquic_frames_varint_decode(bytes, bytes_end, &raw mut lifetime);
            bytes.is_null()
        }
        || {
            bytes = picoquic_frames_varint_decode(bytes, bytes_end, &raw mut recon_bytes_in_flight);
            bytes.is_null()
        }
        || {
            bytes = picoquic_frames_varint_decode(bytes, bytes_end, &raw mut recon_min_rtt);
            bytes.is_null()
        }
        || {
            bytes = picoquic_frames_varint_decode(bytes, bytes_end, &raw mut ip_length);
            bytes.is_null()
        }
        || ip_length != 4 as uint64_t && ip_length != 16 as uint64_t
        || {
            bytes = picoquic_frames_fixed_skip(bytes, bytes_end, ip_length);
            bytes.is_null()
        }
    {
        fprintf(
            F,
            b"    Malformed %s frame: \0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(picoquic_frame_type_bdp as ::core::ffi::c_int as uint64_t),
        );
        let mut i: size_t = 0 as size_t;
        while i < bytes_max && i < 8 as size_t {
            fprintf(
                F,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *bytes0.offset(i as isize) as ::core::ffi::c_int,
            );
            i = i.wrapping_add(1);
        }
        if bytes_max > 8 as size_t {
            fprintf(F, b"...\0".as_ptr() as *const ::core::ffi::c_char);
        }
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        byte_index = bytes_max;
    } else {
        fprintf(
            F,
            b"    %s, lifetime: %lu, bytes_in_flight: %lu, min_rtt: %lu, ip: \0".as_ptr()
                as *const ::core::ffi::c_char,
            textlog_frame_names(picoquic_frame_type_bdp as ::core::ffi::c_int as uint64_t),
            lifetime,
            recon_bytes_in_flight,
            recon_min_rtt,
        );
        let mut i_0: uint64_t = 0 as uint64_t;
        while i_0 < ip_length {
            fprintf(
                F,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *bytes.offset(-(ip_length as isize)).offset(i_0 as isize) as ::core::ffi::c_int,
            );
            i_0 = i_0.wrapping_add(1);
        }
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        byte_index = bytes.offset_from(bytes0) as ::core::ffi::c_long as size_t;
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn textlog_observed_address_frame(
    mut F: *mut FILE,
    mut bytes: *const uint8_t,
    mut byte_size: size_t,
    mut frame_id: uint64_t,
) -> size_t {
    let mut bytes_index: size_t = byte_size;
    let mut bytes_max: *const uint8_t = bytes.offset(byte_size as isize);
    let mut addr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut port: uint16_t = 0 as uint16_t;
    let mut sequence: uint64_t = 0 as uint64_t;
    let mut bytes_next: *const uint8_t = ::core::ptr::null::<uint8_t>();
    bytes_next = picoquic_frames_varint_skip(bytes, bytes_max);
    if bytes_next.is_null() || {
        bytes_next = picoquic_parse_observed_address_frame(
            bytes_next,
            bytes_max,
            frame_id,
            &raw mut sequence,
            &raw mut addr,
            &raw mut port,
        );
        bytes_next.is_null()
    } {
        fprintf(
            F,
            b"    Malformed %s frame.\n\0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(frame_id),
        );
    } else {
        bytes_index = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
        fprintf(
            F,
            b"    %s, sequence: %lu, \0".as_ptr() as *const ::core::ffi::c_char,
            textlog_frame_names(frame_id),
            sequence,
        );
        if frame_id & 1 as uint64_t == 0 as uint64_t {
            fprintf(
                F,
                b"addr: %u.%u.%u.%u, \0".as_ptr() as *const ::core::ffi::c_char,
                *addr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                *addr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                *addr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                *addr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            );
        } else {
            fprintf(
                F,
                b"addr: %x:%x:%x:%x:%x:%x:%x:%x, \0".as_ptr() as *const ::core::ffi::c_char,
                256 as ::core::ffi::c_int
                    * *addr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *addr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                256 as ::core::ffi::c_int
                    * *addr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *addr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                256 as ::core::ffi::c_int
                    * *addr.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *addr.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                256 as ::core::ffi::c_int
                    * *addr.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *addr.offset(7 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                256 as ::core::ffi::c_int
                    * *addr.offset(8 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *addr.offset(9 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                256 as ::core::ffi::c_int
                    * *addr.offset(10 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *addr.offset(11 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                256 as ::core::ffi::c_int
                    * *addr.offset(12 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *addr.offset(13 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
                256 as ::core::ffi::c_int
                    * *addr.offset(14 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    + *addr.offset(15 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            );
        }
        fprintf(
            F,
            b"port: %u\n\0".as_ptr() as *const ::core::ffi::c_char,
            port as ::core::ffi::c_int,
        );
    }
    return bytes_index;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_textlog_frames(
    mut F: *mut FILE,
    mut cnx_id64: uint64_t,
    mut bytes: *const uint8_t,
    mut length: size_t,
) {
    let mut byte_index: size_t = 0 as size_t;
    while byte_index < length {
        let mut frame_id: uint64_t = 0 as uint64_t;
        let mut frame_id_ll: size_t = picoquic_varint_decode(
            bytes.offset(byte_index as isize),
            length.wrapping_sub(byte_index),
            &raw mut frame_id,
        );
        textlog_prefix_initial_cid64(F, cnx_id64);
        if frame_id_ll == 0 as size_t || frame_id < 64 as uint64_t && frame_id_ll != 1 as size_t {
            let mut id_length: size_t = length.wrapping_sub(byte_index);
            let mut id_more: *const ::core::ffi::c_char =
                b"\0".as_ptr() as *const ::core::ffi::c_char;
            if id_length > 8 as size_t {
                id_length = 8 as size_t;
                id_more = b"...\0".as_ptr() as *const ::core::ffi::c_char;
            }
            fprintf(
                F,
                b"    Incorrect frame id: \0".as_ptr() as *const ::core::ffi::c_char,
            );
            let mut x: size_t = 0 as size_t;
            while x < id_length {
                let c2rust_fresh4 = byte_index;
                byte_index = byte_index.wrapping_add(1);
                fprintf(
                    F,
                    b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                    *bytes.offset(c2rust_fresh4 as isize) as ::core::ffi::c_int,
                );
                x = x.wrapping_add(1);
            }
            fprintf(F, b"%s\n\0".as_ptr() as *const ::core::ffi::c_char, id_more);
            byte_index = length;
        } else if frame_id
            & !(picoquic_frame_type_stream_range_min as ::core::ffi::c_int
                ^ picoquic_frame_type_stream_range_max as ::core::ffi::c_int)
                as uint64_t
            == picoquic_frame_type_stream_range_min as ::core::ffi::c_int as uint64_t
        {
            byte_index = byte_index.wrapping_add(textlog_stream_frame(
                F,
                bytes.offset(byte_index as isize),
                length.wrapping_sub(byte_index),
            ));
        } else {
            match frame_id {
                2 => {
                    byte_index = byte_index.wrapping_add(textlog_ack_frame(
                        F,
                        cnx_id64,
                        frame_id,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                        0 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    ));
                }
                3 => {
                    byte_index = byte_index.wrapping_add(textlog_ack_frame(
                        F,
                        cnx_id64,
                        frame_id,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    ));
                }
                354585600 => {
                    byte_index = byte_index.wrapping_add(textlog_ack_frame(
                        F,
                        cnx_id64,
                        frame_id,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                        0 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    ));
                }
                354585601 => {
                    byte_index = byte_index.wrapping_add(textlog_ack_frame(
                        F,
                        cnx_id64,
                        frame_id,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                        1 as ::core::ffi::c_int,
                        1 as ::core::ffi::c_int,
                    ));
                }
                25 => {
                    byte_index = byte_index.wrapping_add(textlog_retire_connection_id_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                        0 as ::core::ffi::c_int,
                    ));
                }
                354585610 => {
                    byte_index = byte_index.wrapping_add(textlog_retire_connection_id_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                        1 as ::core::ffi::c_int,
                    ));
                }
                0 | 1 | 32 => {
                    let mut nb: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while byte_index < length
                        && *bytes.offset(byte_index as isize) as uint64_t == frame_id
                    {
                        byte_index = byte_index.wrapping_add(1);
                        nb += 1;
                    }
                    fprintf(
                        F,
                        b"    %s, %d bytes\n\0".as_ptr() as *const ::core::ffi::c_char,
                        textlog_frame_names(frame_id),
                        nb,
                    );
                }
                4 => {
                    byte_index = byte_index.wrapping_add(textlog_reset_stream_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                28 => {
                    byte_index = byte_index.wrapping_add(textlog_generic_close_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                        picoquic_frame_type_connection_close as ::core::ffi::c_int as uint8_t,
                        cnx_id64,
                    ));
                }
                29 => {
                    byte_index = byte_index.wrapping_add(textlog_generic_close_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                        picoquic_frame_type_application_close as ::core::ffi::c_int as uint8_t,
                        cnx_id64,
                    ));
                }
                16 => {
                    byte_index = byte_index.wrapping_add(textlog_max_data_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                17 => {
                    byte_index = byte_index.wrapping_add(textlog_max_stream_data_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                18 | 19 => {
                    byte_index = byte_index.wrapping_add(textlog_max_stream_id_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                        frame_id,
                    ));
                }
                20 => {
                    byte_index = byte_index.wrapping_add(textlog_blocked_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                21 => {
                    byte_index = byte_index.wrapping_add(textlog_stream_blocked_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                22 | 23 => {
                    byte_index = byte_index.wrapping_add(textlog_streams_blocked_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                        frame_id,
                    ));
                }
                24 => {
                    byte_index = byte_index.wrapping_add(textlog_new_connection_id_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                        0 as ::core::ffi::c_int,
                    ));
                }
                354585609 => {
                    byte_index = byte_index.wrapping_add(textlog_new_connection_id_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                        1 as ::core::ffi::c_int,
                    ));
                }
                5 => {
                    byte_index = byte_index.wrapping_add(textlog_stop_sending_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                26 => {
                    byte_index = byte_index.wrapping_add(textlog_path_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                27 => {
                    byte_index = byte_index.wrapping_add(textlog_path_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                6 => {
                    byte_index = byte_index.wrapping_add(textlog_crypto_hs_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                7 => {
                    byte_index = byte_index.wrapping_add(textlog_new_token_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                30 => {
                    fprintf(
                        F,
                        b"    %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                        textlog_frame_names(frame_id),
                    );
                    byte_index = byte_index.wrapping_add(1);
                }
                48 | 49 => {
                    byte_index = byte_index.wrapping_add(textlog_datagram_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                        frame_id,
                    ));
                }
                175 => {
                    byte_index = byte_index.wrapping_add(textlog_ack_frequency_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                31 => {
                    byte_index = byte_index.wrapping_add(textlog_immediate_ack_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                757 => {
                    byte_index = byte_index.wrapping_add(textlog_time_stamp_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                354585605 => {
                    byte_index = byte_index.wrapping_add(textlog_path_abandon_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                354585607 | 354585608 => {
                    byte_index = byte_index.wrapping_add(textlog_path_available_or_standby_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                354585612 => {
                    byte_index = byte_index.wrapping_add(textlog_max_path_id_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                354585613 => {
                    byte_index = byte_index.wrapping_add(textlog_path_blocked_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                60377 => {
                    byte_index = byte_index.wrapping_add(textlog_bdp_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                    ));
                }
                10453414 | 10453415 => {
                    byte_index = byte_index.wrapping_add(textlog_observed_address_frame(
                        F,
                        bytes.offset(byte_index as isize),
                        length.wrapping_sub(byte_index),
                        frame_id,
                    ));
                }
                _ => {
                    fprintf(
                        F,
                        b"    Unknown frame, type: %lu (0x\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        frame_id,
                    );
                    let mut i: size_t = 0 as size_t;
                    while i < 8 as size_t && byte_index.wrapping_add(i) < length {
                        fprintf(
                            F,
                            b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                            *bytes.offset(byte_index.wrapping_add(i) as isize)
                                as ::core::ffi::c_int,
                        );
                        i = i.wrapping_add(1);
                    }
                    if byte_index.wrapping_add(8 as size_t) < length {
                        fprintf(
                            F,
                            b"... + %zu bytes)\n\0".as_ptr() as *const ::core::ffi::c_char,
                            length.wrapping_sub(byte_index).wrapping_sub(8 as size_t),
                        );
                    } else {
                        fprintf(F, b")\n\0".as_ptr() as *const ::core::ffi::c_char);
                    }
                    byte_index = length;
                }
            }
        }
    }
}
unsafe extern "C" fn textlog_decrypted_segment(
    mut F_log: *mut ::core::ffi::c_void,
    mut log_cnxid: ::core::ffi::c_int,
    mut cnx: *mut picoquic_cnx_t,
    mut receiving: ::core::ffi::c_int,
    mut ph: *mut picoquic_packet_header,
    mut bytes: *const uint8_t,
    mut length: size_t,
    mut ret: ::core::ffi::c_int,
) {
    let mut log_cnxid64: uint64_t = 0 as uint64_t;
    let mut F: *mut FILE = F_log as *mut FILE;
    if F.is_null() {
        return;
    }
    if log_cnxid != 0 as ::core::ffi::c_int {
        if cnx.is_null() {
            (*ph).pn64 = (*ph).pn as uint64_t;
            if ret == 0 as ::core::ffi::c_int {
                if (*ph).ptype as ::core::ffi::c_uint
                    == picoquic_packet_version_negotiation as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                {
                    log_cnxid64 = picoquic_val64_connection_id((*ph).srce_cnx_id);
                } else {
                    log_cnxid64 = picoquic_val64_connection_id((*ph).dest_cnx_id);
                }
            }
        } else {
            log_cnxid64 = picoquic_val64_connection_id(picoquic_get_logging_cnxid(
                cnx as *mut picoquic_cnx_t,
            ));
        }
    }
    textlog_packet_header(F, log_cnxid64, ph, receiving);
    if ret != 0 as ::core::ffi::c_int {
        textlog_prefix_initial_cid64(F, log_cnxid64);
        if ret == PICOQUIC_ERROR_STATELESS_RESET {
            fprintf(
                F,
                b"   Stateless reset.\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            fprintf(
                F,
                b"   Header or encryption error: %x.\n\0".as_ptr() as *const ::core::ffi::c_char,
                ret,
            );
        }
    } else if (*ph).ptype as ::core::ffi::c_uint
        == picoquic_packet_version_negotiation as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        textlog_negotiation_packet(F, log_cnxid64, bytes, length, ph);
    } else if (*ph).ptype as ::core::ffi::c_uint
        == picoquic_packet_retry as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        textlog_retry_packet(F, log_cnxid64, bytes, ph);
    } else if (*ph).ptype as ::core::ffi::c_uint
        != picoquic_packet_error as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        textlog_prefix_initial_cid64(F, log_cnxid64);
        fprintf(
            F,
            b"    %s %d bytes\n\0".as_ptr() as *const ::core::ffi::c_char,
            if receiving != 0 {
                b"Decrypted\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"Prepared\0".as_ptr() as *const ::core::ffi::c_char
            },
            (*ph).payload_length as ::core::ffi::c_int,
        );
        picoquic_textlog_frames(
            F,
            log_cnxid64,
            bytes.offset((*ph).offset as isize),
            (*ph).payload_length,
        );
    }
    fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
}
unsafe extern "C" fn textlog_outgoing_segment(
    mut F_log: *mut ::core::ffi::c_void,
    mut log_cnxid: ::core::ffi::c_int,
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut sequence_number: uint64_t,
    mut length: size_t,
    mut send_buffer: *mut uint8_t,
    mut send_length: size_t,
    mut pn_length: size_t,
) {
    let mut pcnx: *mut picoquic_cnx_t = cnx;
    let mut ph: picoquic_packet_header = st_picoquic_packet_header_t {
        dest_cnx_id: st_picoquic_connection_id_t {
            id: [0; 20],
            id_len: 0,
        },
        srce_cnx_id: st_picoquic_connection_id_t {
            id: [0; 20],
            id_len: 0,
        },
        pn: 0,
        vn: 0,
        offset: 0,
        pn_offset: 0,
        ptype: picoquic_packet_error,
        pnmask: 0,
        pn64: 0,
        payload_length: 0,
        version_index: 0,
        epoch: picoquic_epoch_initial,
        pc: picoquic_packet_context_application,
        key_phase_spin_has_spin_bit_has_reserved_bit_set_has_loss_bits_loss_bit_Q_loss_bit_L_quic_bit_is_zero: [0; 1],
        c2rust_padding: [0; 3],
        token_length: 0,
        token_bytes: ::core::ptr::null::<uint8_t>(),
        pl_val: 0,
        l_cid: ::core::ptr::null_mut::<st_picoquic_local_cnxid_t>(),
    };
    let mut default_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut checksum_length: size_t = 16 as size_t;
    let mut ret: ::core::ffi::c_int = 0;
    if F_log.is_null() {
        return;
    }
    memset(
        &raw mut default_addr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_in>() as size_t,
    );
    default_addr.sin_family = AF_INET as sa_family_t;
    ret = picoquic_parse_packet_header(
        if cnx.is_null() {
            ::core::ptr::null_mut::<picoquic_quic_t>()
        } else {
            (*cnx).quic
        },
        send_buffer,
        send_length,
        if cnx.is_null() || (*(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).is_null() {
            &raw mut default_addr as *mut sockaddr
        } else {
            &raw mut (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).local_addr
                as *mut sockaddr
        },
        &raw mut ph,
        &raw mut pcnx,
        0 as ::core::ffi::c_int,
    );
    ph.pn64 = sequence_number;
    ph.pn = ph.pn64 as uint32_t;
    if ph.ptype as ::core::ffi::c_uint
        != picoquic_packet_retry as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if !cnx.is_null() {
            let mut epoch: picoquic_epoch_enum = (if ph.ptype as ::core::ffi::c_uint
                == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                picoquic_epoch_1rtt as ::core::ffi::c_int
            } else if ph.ptype as ::core::ffi::c_uint
                == picoquic_packet_0rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                picoquic_epoch_0rtt as ::core::ffi::c_int
            } else if ph.ptype as ::core::ffi::c_uint
                == picoquic_packet_handshake as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                picoquic_epoch_handshake as ::core::ffi::c_int
            } else {
                picoquic_epoch_initial as ::core::ffi::c_int
            }) as picoquic_epoch_enum;
            if !(*cnx).crypto_context[epoch as usize].aead_encrypt.is_null() {
                checksum_length = picoquic_get_checksum_length(cnx, epoch);
            }
        }
        if ph.pn_offset != 0 as size_t {
            ph.offset = ph.pn_offset.wrapping_add(pn_length);
            ph.payload_length = ph.payload_length.wrapping_sub(pn_length);
        }
    }
    if ph.ptype as ::core::ffi::c_uint
        != picoquic_packet_version_negotiation as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if ph.payload_length > checksum_length {
            ph.payload_length = ph
                .payload_length
                .wrapping_sub(checksum_length as uint16_t as size_t);
        } else {
            ph.payload_length = 0 as size_t;
        }
    }
    textlog_decrypted_segment(
        F_log,
        log_cnxid,
        cnx,
        0 as ::core::ffi::c_int,
        &raw mut ph,
        bytes,
        length,
        ret,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_textlog_transport_extension_content(
    mut F: *mut FILE,
    mut log_cnxid: ::core::ffi::c_int,
    mut cnx_id_64: uint64_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: size_t,
) {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut byte_index: size_t = 0 as size_t;
    if bytes_max < 256 as size_t {
        if ret == 0 as ::core::ffi::c_int {
            let mut extensions_size: size_t = bytes_max;
            let mut extensions_end: size_t = 0;
            extensions_end = byte_index.wrapping_add(extensions_size);
            if log_cnxid != 0 as ::core::ffi::c_int {
                textlog_prefix_initial_cid64(F, cnx_id_64);
            }
            fprintf(
                F,
                b"    Extension list (%d bytes):\n\0".as_ptr() as *const ::core::ffi::c_char,
                extensions_size as uint32_t,
            );
            while ret == 0 as ::core::ffi::c_int && byte_index < extensions_end {
                let mut extension_type: uint64_t = 0 as uint64_t;
                let mut extension_length: uint64_t = 0 as uint64_t;
                let mut ll_type: size_t = 0 as size_t;
                let mut ll_length: size_t = 0 as size_t;
                ll_type = picoquic_varint_decode(
                    bytes.offset(byte_index as isize),
                    extensions_end.wrapping_sub(byte_index),
                    &raw mut extension_type,
                );
                byte_index = byte_index.wrapping_add(ll_type);
                ll_length = picoquic_varint_decode(
                    bytes.offset(byte_index as isize),
                    extensions_end.wrapping_sub(byte_index),
                    &raw mut extension_length,
                );
                byte_index = byte_index.wrapping_add(ll_length);
                if ll_type == 0 as size_t
                    || ll_length == 0 as size_t
                    || byte_index.wrapping_add(extension_length as size_t) > extensions_end
                {
                    if log_cnxid != 0 as ::core::ffi::c_int {
                        textlog_prefix_initial_cid64(F, cnx_id_64);
                    }
                    fprintf(
                        F,
                        b"        Malformed extension -- only %d bytes avaliable for type and length.\n\0"
                            .as_ptr() as *const ::core::ffi::c_char,
                        extensions_end.wrapping_sub(byte_index) as ::core::ffi::c_int,
                    );
                    ret = -(1 as ::core::ffi::c_int);
                } else {
                    if log_cnxid != 0 as ::core::ffi::c_int {
                        textlog_prefix_initial_cid64(F, cnx_id_64);
                    }
                    fprintf(
                        F,
                        b"        Extension type: %lu (%s), length %d%s\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        extension_type,
                        textlog_tp_name(extension_type),
                        extension_length as ::core::ffi::c_int,
                        if extension_length == 0 as uint64_t {
                            b"\0".as_ptr() as *const ::core::ffi::c_char
                        } else {
                            b", \0".as_ptr() as *const ::core::ffi::c_char
                        },
                    );
                    if byte_index.wrapping_add(extension_length as size_t) > extensions_end {
                        if log_cnxid != 0 as ::core::ffi::c_int {
                            textlog_prefix_initial_cid64(F, cnx_id_64);
                        }
                        fprintf(
                            F,
                            b"Malformed extension, only %d bytes available.\n\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            extensions_end.wrapping_sub(byte_index) as ::core::ffi::c_int,
                        );
                        ret = -(1 as ::core::ffi::c_int);
                    } else {
                        let mut i: uint64_t = 0 as uint64_t;
                        while i < extension_length {
                            let c2rust_fresh5 = byte_index;
                            byte_index = byte_index.wrapping_add(1);
                            fprintf(
                                F,
                                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                                *bytes.offset(c2rust_fresh5 as isize) as ::core::ffi::c_int,
                            );
                            i = i.wrapping_add(1);
                        }
                        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                    }
                }
            }
        }
        if ret == 0 as ::core::ffi::c_int && byte_index < bytes_max {
            if log_cnxid != 0 as ::core::ffi::c_int {
                textlog_prefix_initial_cid64(F, cnx_id_64);
            }
            fprintf(
                F,
                b"    Remaining bytes (%d)\n\0".as_ptr() as *const ::core::ffi::c_char,
                bytes_max.wrapping_sub(byte_index) as uint32_t,
            );
        }
    } else {
        if log_cnxid != 0 as ::core::ffi::c_int {
            textlog_prefix_initial_cid64(F, cnx_id_64);
        }
        fprintf(
            F,
            b"Received transport parameter TLS extension (%d bytes):\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            bytes_max as uint32_t,
        );
        if log_cnxid != 0 as ::core::ffi::c_int {
            textlog_prefix_initial_cid64(F, cnx_id_64);
        }
        fprintf(
            F,
            b"    First bytes (%d):\n\0".as_ptr() as *const ::core::ffi::c_char,
            bytes_max.wrapping_sub(byte_index) as uint32_t,
        );
    }
    if ret == 0 as ::core::ffi::c_int {
        while byte_index < bytes_max && byte_index < 128 as size_t {
            if log_cnxid != 0 as ::core::ffi::c_int {
                fprintf(
                    F,
                    b"%lx: \0".as_ptr() as *const ::core::ffi::c_char,
                    cnx_id_64,
                );
            }
            fprintf(F, b"        \0".as_ptr() as *const ::core::ffi::c_char);
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < 32 as ::core::ffi::c_int
                && byte_index < bytes_max
                && byte_index < 128 as size_t
            {
                let c2rust_fresh6 = byte_index;
                byte_index = byte_index.wrapping_add(1);
                fprintf(
                    F,
                    b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                    *bytes.offset(c2rust_fresh6 as isize) as ::core::ffi::c_int,
                );
                i_0 += 1;
            }
            fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_textlog_transport_extension(
    mut F: *mut FILE,
    mut cnx: *mut picoquic_cnx_t,
    mut received: ::core::ffi::c_int,
    mut log_cnxid: ::core::ffi::c_int,
    mut bytes: *mut uint8_t,
    mut bytes_max: size_t,
) {
    let mut cnx_id_64: uint64_t = if log_cnxid != 0 {
        picoquic_val64_connection_id(picoquic_get_logging_cnxid(cnx as *mut picoquic_cnx_t))
    } else {
        0 as uint64_t
    };
    textlog_prefix_initial_cid64(F, cnx_id_64);
    fprintf(
        F,
        b"%s transport parameter TLS extension (%d bytes):\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        if received != 0 {
            b"Received\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"Sending\0".as_ptr() as *const ::core::ffi::c_char
        },
        bytes_max as uint32_t,
    );
    picoquic_textlog_transport_extension_content(F, log_cnxid, cnx_id_64, bytes, bytes_max);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_textlog_negotiated_alpn(
    mut F: *mut FILE,
    mut cnx: *mut picoquic_cnx_t,
    mut received: ::core::ffi::c_int,
    mut log_cnxid: ::core::ffi::c_int,
    mut list: *const ptls_iovec_t,
    mut count: size_t,
) {
    let mut cnx_id_64: uint64_t = if log_cnxid != 0 {
        picoquic_val64_connection_id(picoquic_get_logging_cnxid(cnx as *mut picoquic_cnx_t))
    } else {
        0 as uint64_t
    };
    textlog_prefix_initial_cid64(F, cnx_id_64);
    fprintf(
        F,
        b"%s ALPN list (%d): \0".as_ptr() as *const ::core::ffi::c_char,
        if received != 0 {
            b"Received\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"Sending\0".as_ptr() as *const ::core::ffi::c_char
        },
        count as uint32_t,
    );
    let mut i: size_t = 0 as size_t;
    while i < count {
        let mut alpn_target: [::core::ffi::c_char; 64] = [0; 64];
        if (*list.offset(i as isize)).len < 64 as size_t {
            memcpy(
                &raw mut alpn_target as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                (*list.offset(i as isize)).base as *const ::core::ffi::c_void,
                (*list.offset(i as isize)).len,
            );
            alpn_target[(*list.offset(i as isize)).len as usize] = 0 as ::core::ffi::c_char;
        } else {
            memcpy(
                &raw mut alpn_target as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
                (*list.offset(i as isize)).base as *const ::core::ffi::c_void,
                60 as size_t,
            );
            alpn_target[60 as ::core::ffi::c_int as usize] = '.' as i32 as ::core::ffi::c_char;
            alpn_target[61 as ::core::ffi::c_int as usize] = '.' as i32 as ::core::ffi::c_char;
            alpn_target[62 as ::core::ffi::c_int as usize] = '.' as i32 as ::core::ffi::c_char;
            alpn_target[63 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
        }
        fprintf(
            F,
            b"%s%s\0".as_ptr() as *const ::core::ffi::c_char,
            if i == 0 as size_t {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b", \0".as_ptr() as *const ::core::ffi::c_char
            },
            &raw mut alpn_target as *mut ::core::ffi::c_char,
        );
        i = i.wrapping_add(1);
    }
    fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
}
unsafe extern "C" fn textlog_congestion_state(
    mut F: *mut FILE,
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
) {
    let mut path_x: *mut picoquic_path_t = *(*cnx).path.offset(0 as ::core::ffi::c_int as isize);
    fprintf(
        F,
        b"%lx: \0".as_ptr() as *const ::core::ffi::c_char,
        picoquic_val64_connection_id(picoquic_get_logging_cnxid(cnx as *mut picoquic_cnx_t)),
    );
    textlog_time(
        F,
        cnx,
        current_time,
        b"T= \0".as_ptr() as *const ::core::ffi::c_char,
        b", \0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        F,
        b"cwin: %d,\0".as_ptr() as *const ::core::ffi::c_char,
        (*path_x).cwin as ::core::ffi::c_int,
    );
    fprintf(
        F,
        b"flight: %d,\0".as_ptr() as *const ::core::ffi::c_char,
        (*path_x).bytes_in_transit as ::core::ffi::c_int,
    );
    fprintf(
        F,
        b"nb_ret: %d,\0".as_ptr() as *const ::core::ffi::c_char,
        (*cnx).nb_retransmission_total as ::core::ffi::c_int,
    );
    fprintf(
        F,
        b"rtt_min: %d,\0".as_ptr() as *const ::core::ffi::c_char,
        (*path_x).rtt_min as ::core::ffi::c_int,
    );
    fprintf(
        F,
        b"rtt: %d,\0".as_ptr() as *const ::core::ffi::c_char,
        (*path_x).smoothed_rtt as ::core::ffi::c_int,
    );
    fprintf(
        F,
        b"rtt_var: %d,\0".as_ptr() as *const ::core::ffi::c_char,
        (*path_x).rtt_variant as ::core::ffi::c_int,
    );
    fprintf(
        F,
        b"max_ack_delay: %d,\0".as_ptr() as *const ::core::ffi::c_char,
        (*path_x).max_ack_delay as ::core::ffi::c_int,
    );
    fprintf(
        F,
        b"state: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
        (*cnx).cnx_state as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn picoquic_textlog_tls_ticket(
    mut F: *mut FILE,
    mut cnx_id: picoquic_connection_id_t,
    mut ticket: *mut uint8_t,
    mut ticket_length: uint16_t,
) {
    let mut cnx_id64: uint64_t = picoquic_val64_connection_id(cnx_id);
    let mut lifetime: uint32_t = 0 as uint32_t;
    let mut age_add: uint32_t = 0 as uint32_t;
    let mut nonce_length: uint8_t = 0 as uint8_t;
    let mut ticket_val_length: uint16_t = 0 as uint16_t;
    let mut extension_length: uint16_t = 0 as uint16_t;
    let mut extension_ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut byte_index: uint16_t = 0 as uint16_t;
    let mut min_length: uint16_t = (4 as ::core::ffi::c_int
        + 4 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint16_t;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (ticket_length as ::core::ffi::c_int) < min_length as ::core::ffi::c_int {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        lifetime = (((*ticket.offset(0 as ::core::ffi::c_int as isize) as uint16_t
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *ticket.offset(1 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
            as uint32_t)
            << 16 as ::core::ffi::c_int
            | ((*ticket
                .offset(2 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *ticket
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t;
        byte_index = (byte_index as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint16_t;
        age_add = (((*ticket
            .offset(byte_index as ::core::ffi::c_int as isize)
            .offset(0 as ::core::ffi::c_int as isize) as uint16_t
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *ticket
                .offset(byte_index as ::core::ffi::c_int as isize)
                .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int) as uint32_t)
            << 16 as ::core::ffi::c_int
            | ((*ticket
                .offset(byte_index as ::core::ffi::c_int as isize)
                .offset(2 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *ticket
                    .offset(byte_index as ::core::ffi::c_int as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t;
        byte_index = (byte_index as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint16_t;
        let c2rust_fresh7 = byte_index;
        byte_index = byte_index.wrapping_add(1);
        nonce_length = *ticket.offset(c2rust_fresh7 as isize);
        min_length =
            (min_length as ::core::ffi::c_int + nonce_length as ::core::ffi::c_int) as uint16_t;
        if (ticket_length as ::core::ffi::c_int) < min_length as ::core::ffi::c_int {
            ret = -(1 as ::core::ffi::c_int);
        } else {
            byte_index =
                (byte_index as ::core::ffi::c_int + nonce_length as ::core::ffi::c_int) as uint16_t;
            ticket_val_length = ((*ticket
                .offset(byte_index as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize)
                as uint16_t as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *ticket
                    .offset(byte_index as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint16_t;
            byte_index = (byte_index as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint16_t;
            min_length = (min_length as ::core::ffi::c_int
                + ticket_val_length as ::core::ffi::c_int) as uint16_t;
            if (ticket_length as ::core::ffi::c_int) < min_length as ::core::ffi::c_int {
                ret = -(1 as ::core::ffi::c_int);
            } else {
                byte_index = (byte_index as ::core::ffi::c_int
                    + ticket_val_length as ::core::ffi::c_int)
                    as uint16_t;
                extension_length = ((*ticket
                    .offset(byte_index as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as uint16_t as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *ticket
                        .offset(byte_index as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int) as uint16_t;
                byte_index =
                    (byte_index as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint16_t;
                if extension_length as ::core::ffi::c_int
                    > ticket_length as ::core::ffi::c_int - min_length as ::core::ffi::c_int
                {
                    ret = -(2 as ::core::ffi::c_int);
                } else {
                    extension_ptr = ticket.offset(byte_index as isize) as *mut uint8_t;
                    min_length = (min_length as ::core::ffi::c_int
                        + extension_length as ::core::ffi::c_int)
                        as uint16_t;
                }
            }
        }
    }
    if ret == -(1 as ::core::ffi::c_int) {
        textlog_prefix_initial_cid64(F, cnx_id64);
        fprintf(
            F,
            b"Malformed ticket, length = %d, at least %d required.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            ticket_length as ::core::ffi::c_int,
            min_length as ::core::ffi::c_int,
        );
    }
    textlog_prefix_initial_cid64(F, cnx_id64);
    fprintf(
        F,
        b"lifetime = %d, age_add = %x, %d nonce, %d ticket, %d extensions.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        lifetime,
        age_add,
        nonce_length as ::core::ffi::c_int,
        ticket_val_length as ::core::ffi::c_int,
        extension_length as ::core::ffi::c_int,
    );
    if !extension_ptr.is_null() {
        let mut x_index: uint16_t = 0 as uint16_t;
        textlog_prefix_initial_cid64(F, cnx_id64);
        fprintf(
            F,
            b"ticket extensions: \0".as_ptr() as *const ::core::ffi::c_char,
        );
        while (x_index as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            < extension_length as ::core::ffi::c_int
        {
            let mut x_type: uint16_t = ((*extension_ptr
                .offset(x_index as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize)
                as uint16_t as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *extension_ptr
                    .offset(x_index as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint16_t;
            let mut x_len: uint16_t = ((*extension_ptr
                .offset(x_index as ::core::ffi::c_int as isize)
                .offset(2 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize)
                as uint16_t as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *extension_ptr
                    .offset(x_index as ::core::ffi::c_int as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint16_t;
            x_index = (x_index as ::core::ffi::c_int
                + (4 as ::core::ffi::c_int + x_len as ::core::ffi::c_int))
                as uint16_t;
            if x_type as ::core::ffi::c_int == 42 as ::core::ffi::c_int
                && x_len as ::core::ffi::c_int == 4 as ::core::ffi::c_int
            {
                let mut ed_len: uint32_t = (((*extension_ptr
                    .offset(x_index as ::core::ffi::c_int as isize)
                    .offset(-(4 as ::core::ffi::c_int as isize))
                    .offset(0 as ::core::ffi::c_int as isize)
                    as uint16_t
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *extension_ptr
                        .offset(x_index as ::core::ffi::c_int as isize)
                        .offset(-(4 as ::core::ffi::c_int as isize))
                        .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int)
                    as uint32_t)
                    << 16 as ::core::ffi::c_int
                    | ((*extension_ptr
                        .offset(x_index as ::core::ffi::c_int as isize)
                        .offset(-(4 as ::core::ffi::c_int as isize))
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as uint16_t as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *extension_ptr
                            .offset(x_index as ::core::ffi::c_int as isize)
                            .offset(-(4 as ::core::ffi::c_int as isize))
                            .offset(2 as ::core::ffi::c_int as isize)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as uint16_t as ::core::ffi::c_int) as uint32_t;
                fprintf(
                    F,
                    b"%d(ED: %x),\0".as_ptr() as *const ::core::ffi::c_char,
                    x_type as ::core::ffi::c_int,
                    ed_len,
                );
            } else {
                fprintf(
                    F,
                    b"%d (%d bytes),\0".as_ptr() as *const ::core::ffi::c_char,
                    x_type as ::core::ffi::c_int,
                    x_len as ::core::ffi::c_int,
                );
            }
            if x_index as ::core::ffi::c_int > extension_length as ::core::ffi::c_int {
                fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
                textlog_prefix_initial_cid64(F, cnx_id64);
                fprintf(
                    F,
                    b"malformed extensions, require %d bytes, not just %d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    x_index as ::core::ffi::c_int,
                    extension_length as ::core::ffi::c_int,
                );
            }
        }
        fprintf(F, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        if (x_index as ::core::ffi::c_int) < extension_length as ::core::ffi::c_int {
            textlog_prefix_initial_cid64(F, cnx_id64);
            fprintf(
                F,
                b"%d extra bytes at the end of the extensions\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                extension_length as ::core::ffi::c_int - x_index as ::core::ffi::c_int,
            );
        }
    }
    if ret == -(2 as ::core::ffi::c_int) {
        textlog_prefix_initial_cid64(F, cnx_id64);
        fprintf(
            F,
            b"Malformed TLS ticket, %d extra bytes.\n\0".as_ptr() as *const ::core::ffi::c_char,
            ticket_length as ::core::ffi::c_int - min_length as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_textlog_picotls_ticket(
    mut F: *mut FILE,
    mut cnx_id: picoquic_connection_id_t,
    mut ticket: *mut uint8_t,
    mut ticket_length: uint16_t,
) {
    let mut cnx_id64: uint64_t = picoquic_val64_connection_id(cnx_id);
    let mut ticket_time: uint64_t = 0 as uint64_t;
    let mut kx_id: uint16_t = 0 as uint16_t;
    let mut suite_id: uint16_t = 0 as uint16_t;
    let mut tls_ticket_length: uint32_t = 0 as uint32_t;
    let mut tls_ticket_ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut secret_length: uint16_t = 0 as uint16_t;
    let mut byte_index: uint16_t = 0 as uint16_t;
    let mut min_length: uint32_t = (8 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 3 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint32_t;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (ticket_length as uint32_t) < min_length {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        ticket_time = (((((*ticket.offset(0 as ::core::ffi::c_int as isize) as uint16_t
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *ticket.offset(1 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
            as uint32_t)
            << 16 as ::core::ffi::c_int
            | ((*ticket
                .offset(2 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *ticket
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t) as uint64_t)
            << 32 as ::core::ffi::c_int
            | ((((*ticket
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *ticket
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t)
                << 16 as ::core::ffi::c_int
                | ((*ticket
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *ticket
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int) as uint32_t) as uint64_t;
        byte_index = (byte_index as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as uint16_t;
        kx_id = ((*ticket
            .offset(byte_index as ::core::ffi::c_int as isize)
            .offset(0 as ::core::ffi::c_int as isize) as uint16_t
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *ticket
                .offset(byte_index as ::core::ffi::c_int as isize)
                .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int) as uint16_t;
        byte_index = (byte_index as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint16_t;
        suite_id = ((*ticket
            .offset(byte_index as ::core::ffi::c_int as isize)
            .offset(0 as ::core::ffi::c_int as isize) as uint16_t
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *ticket
                .offset(byte_index as ::core::ffi::c_int as isize)
                .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int) as uint16_t;
        byte_index = (byte_index as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint16_t;
        tls_ticket_length = (((*ticket
            .offset(byte_index as ::core::ffi::c_int as isize)
            .offset(0 as ::core::ffi::c_int as isize) as uint16_t
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *ticket
                .offset(byte_index as ::core::ffi::c_int as isize)
                .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int) as uint32_t)
            << 8 as ::core::ffi::c_int
            | *ticket
                .offset(byte_index as ::core::ffi::c_int as isize)
                .offset(2 as ::core::ffi::c_int as isize) as uint32_t;
        byte_index = (byte_index as ::core::ffi::c_int + 3 as ::core::ffi::c_int) as uint16_t;
        min_length = min_length.wrapping_add(tls_ticket_length);
        if (ticket_length as uint32_t) < min_length {
            ret = -(1 as ::core::ffi::c_int);
        } else {
            tls_ticket_ptr = ticket.offset(byte_index as isize) as *mut uint8_t;
            byte_index = (byte_index as ::core::ffi::c_int
                + tls_ticket_length as uint16_t as ::core::ffi::c_int)
                as uint16_t;
            secret_length = ((*ticket
                .offset(byte_index as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *ticket
                    .offset(byte_index as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint16_t;
            min_length = min_length.wrapping_add(
                (secret_length as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint32_t,
            );
            if (ticket_length as uint32_t) < min_length {
                ret = -(1 as ::core::ffi::c_int);
            } else if ticket_length as uint32_t > min_length {
                ret = -(2 as ::core::ffi::c_int);
            }
        }
    }
    textlog_prefix_initial_cid64(F, cnx_id64);
    fprintf(
        F,
        b"ticket time = %llu, kx = %x, suite = %x, %d ticket, %d secret.\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        ticket_time as ::core::ffi::c_ulonglong,
        kx_id as ::core::ffi::c_int,
        suite_id as ::core::ffi::c_int,
        tls_ticket_length,
        secret_length as ::core::ffi::c_int,
    );
    if ret == -(1 as ::core::ffi::c_int) {
        textlog_prefix_initial_cid64(F, cnx_id64);
        fprintf(
            F,
            b"Malformed PTLS ticket, length = %d, at least %d required.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            ticket_length as ::core::ffi::c_int,
            min_length,
        );
    } else if tls_ticket_length > 0 as uint32_t && !tls_ticket_ptr.is_null() {
        picoquic_textlog_tls_ticket(F, cnx_id, tls_ticket_ptr, tls_ticket_length as uint16_t);
    }
    if ret == -(2 as ::core::ffi::c_int) {
        textlog_prefix_initial_cid64(F, cnx_id64);
        fprintf(
            F,
            b"Malformed PTLS ticket, %d extra bytes.\n\0".as_ptr() as *const ::core::ffi::c_char,
            (ticket_length as uint32_t).wrapping_sub(min_length),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_txtlog_message_v(
    mut quic: *mut picoquic_quic_t,
    mut cid: *const picoquic_connection_id_t,
    mut fmt: *const ::core::ffi::c_char,
    mut vargs: ::core::ffi::VaList,
) {
    let mut F: *mut FILE = (*quic).F_log as *mut FILE;
    textlog_prefix_initial_cid64(F, picoquic_val64_connection_id(*cid));
    vfprintf(F, fmt, vargs.as_va_list());
    fputc('\n' as i32, F);
}
#[no_mangle]
pub unsafe extern "C" fn txtlog_context_free_app_message(
    mut quic: *mut picoquic_quic_t,
    mut cid: *const picoquic_connection_id_t,
    mut fmt: *const ::core::ffi::c_char,
    mut vargs: ::core::ffi::VaList,
) {
    if !(*quic).F_log.is_null() {
        picoquic_txtlog_message_v(quic, cid, fmt, vargs.as_va_list());
    }
}
unsafe extern "C" fn textlog_app_message(
    mut cnx: *mut picoquic_cnx_t,
    mut fmt: *const ::core::ffi::c_char,
    mut vargs: ::core::ffi::VaList,
) {
    if !(*(*cnx).quic).F_log.is_null() {
        picoquic_txtlog_message_v(
            (*cnx).quic,
            &raw mut (*cnx).initial_cnxid,
            fmt,
            vargs.as_va_list(),
        );
    }
}
unsafe extern "C" fn textlog_quic_pdu(
    mut quic: *mut picoquic_quic_t,
    mut receiving: ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut cid64: uint64_t,
    mut addr_peer: *const sockaddr,
    mut addr_local: *const sockaddr,
    mut packet_length: size_t,
) {
    if !(*quic).F_log.is_null() {
        textlog_packet_address(
            (*quic).F_log as *mut FILE,
            cid64,
            ::core::ptr::null_mut::<picoquic_cnx_t>(),
            addr_peer,
            receiving,
            packet_length,
            current_time,
        );
    }
}
unsafe extern "C" fn textlog_pdu_ex(
    mut cnx: *mut picoquic_cnx_t,
    mut receiving: ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut addr_peer: *const sockaddr,
    mut addr_local: *const sockaddr,
    mut packet_length: size_t,
) {
    if !(*(*cnx).quic).F_log.is_null()
        && picoquic_cnx_is_still_logging(cnx as *mut picoquic_cnx_t) != 0
    {
        textlog_packet_address(
            (*(*cnx).quic).F_log as *mut FILE,
            picoquic_val64_connection_id(picoquic_get_logging_cnxid(cnx as *mut picoquic_cnx_t)),
            cnx,
            addr_peer,
            receiving,
            packet_length,
            current_time,
        );
    }
}
unsafe extern "C" fn textlog_packet(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut receiving: ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut ph: *mut picoquic_packet_header,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) {
    if !(*(*cnx).quic).F_log.is_null()
        && picoquic_cnx_is_still_logging(cnx as *mut picoquic_cnx_t) != 0
    {
        textlog_decrypted_segment(
            (*(*cnx).quic).F_log,
            1 as ::core::ffi::c_int,
            cnx,
            receiving,
            ph,
            bytes,
            bytes_max,
            0 as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn textlog_dropped_packet(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut ph: *mut picoquic_packet_header,
    mut packet_size: size_t,
    mut ret: ::core::ffi::c_int,
    mut raw_data: *mut uint8_t,
    mut current_time: uint64_t,
) {
    if !(*(*cnx).quic).F_log.is_null()
        && picoquic_cnx_is_still_logging(cnx as *mut picoquic_cnx_t) != 0
    {
        textlog_decrypted_segment(
            (*(*cnx).quic).F_log,
            1 as ::core::ffi::c_int,
            cnx,
            1 as ::core::ffi::c_int,
            ph,
            raw_data,
            packet_size,
            ret,
        );
    }
}
unsafe extern "C" fn textlog_buffered_packet(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut ptype: picoquic_packet_type_enum,
    mut current_time: uint64_t,
) {
    if !(*(*cnx).quic).F_log.is_null()
        && picoquic_cnx_is_still_logging(cnx as *mut picoquic_cnx_t) != 0
    {
        let mut F: *mut FILE = (*(*cnx).quic).F_log as *mut FILE;
        fprintf(
            F,
            b"%lx: \0".as_ptr() as *const ::core::ffi::c_char,
            picoquic_val64_connection_id(picoquic_get_logging_cnxid(cnx as *mut picoquic_cnx_t)),
        );
        textlog_time(
            F,
            cnx,
            current_time,
            b"T= \0".as_ptr() as *const ::core::ffi::c_char,
            b", \0".as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            F,
            b"Keys unavailable, buffered packet type %d.\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            ptype as ::core::ffi::c_uint,
        );
    }
}
unsafe extern "C" fn textlog_outgoing_packet(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut bytes: *mut uint8_t,
    mut sequence_number: uint64_t,
    mut pn_length: size_t,
    mut length: size_t,
    mut send_buffer: *mut uint8_t,
    mut send_length: size_t,
    mut current_time: uint64_t,
) {
    if !(*(*cnx).quic).F_log.is_null()
        && picoquic_cnx_is_still_logging(cnx as *mut picoquic_cnx_t) != 0
    {
        textlog_outgoing_segment(
            (*(*cnx).quic).F_log,
            1 as ::core::ffi::c_int,
            cnx,
            bytes,
            sequence_number,
            length,
            send_buffer,
            send_length,
            pn_length,
        );
    }
}
unsafe extern "C" fn textlog_packet_lost(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut ptype: picoquic_packet_type_enum,
    mut sequence_number: uint64_t,
    mut trigger: *const ::core::ffi::c_char,
    mut dcid: *mut picoquic_connection_id_t,
    mut packet_size: size_t,
    mut current_time: uint64_t,
) {
    if !(*(*cnx).quic).F_log.is_null()
        && picoquic_cnx_is_still_logging(cnx as *mut picoquic_cnx_t) != 0
    {
        let mut F: *mut FILE = (*(*cnx).quic).F_log as *mut FILE;
        fprintf(
            F,
            b"%lx: \0".as_ptr() as *const ::core::ffi::c_char,
            picoquic_val64_connection_id(picoquic_get_logging_cnxid(cnx as *mut picoquic_cnx_t)),
        );
        textlog_time(
            F,
            cnx,
            current_time,
            b"T= \0".as_ptr() as *const ::core::ffi::c_char,
            b", \0".as_ptr() as *const ::core::ffi::c_char,
        );
        fprintf(
            F,
            b"Lost packet type %d, number %lu, size %zu\0".as_ptr() as *const ::core::ffi::c_char,
            ptype as ::core::ffi::c_uint,
            sequence_number,
            packet_size,
        );
        if !dcid.is_null() {
            fprintf(F, b", DCID \0".as_ptr() as *const ::core::ffi::c_char);
            textlog_connection_id(F, dcid);
        }
        fprintf(
            F,
            b", reason: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            trigger,
        );
    }
}
unsafe extern "C" fn textlog_negotiated_alpn(
    mut cnx: *mut picoquic_cnx_t,
    mut is_local: ::core::ffi::c_int,
    mut sni: *const uint8_t,
    mut sni_len: size_t,
    mut alpn: *const uint8_t,
    mut alpn_len: size_t,
    mut alpn_list: *const ptls_iovec_t,
    mut alpn_count: size_t,
) {
    if !(*(*cnx).quic).F_log.is_null()
        && picoquic_cnx_is_still_logging(cnx as *mut picoquic_cnx_t) != 0
    {
        picoquic_textlog_negotiated_alpn(
            (*(*cnx).quic).F_log as *mut FILE,
            cnx,
            if is_local != 0 {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            },
            1 as ::core::ffi::c_int,
            alpn_list,
            alpn_count,
        );
    }
}
unsafe extern "C" fn textlog_transport_extension(
    mut cnx: *mut picoquic_cnx_t,
    mut is_local: ::core::ffi::c_int,
    mut param_length: size_t,
    mut params: *mut uint8_t,
) {
    if !(*(*cnx).quic).F_log.is_null()
        && picoquic_cnx_is_still_logging(cnx as *mut picoquic_cnx_t) != 0
    {
        picoquic_textlog_transport_extension(
            (*(*cnx).quic).F_log as *mut FILE,
            cnx,
            if is_local != 0 {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            },
            1 as ::core::ffi::c_int,
            params,
            param_length,
        );
    }
}
unsafe extern "C" fn textlog_tls_ticket(
    mut cnx: *mut picoquic_cnx_t,
    mut ticket: *mut uint8_t,
    mut ticket_length: uint16_t,
) {
    if !(*(*cnx).quic).F_log.is_null()
        && picoquic_cnx_is_still_logging(cnx as *mut picoquic_cnx_t) != 0
    {
        picoquic_textlog_picotls_ticket(
            (*(*cnx).quic).F_log as *mut FILE,
            picoquic_get_logging_cnxid(cnx as *mut picoquic_cnx_t),
            ticket,
            ticket_length,
        );
    }
}
unsafe extern "C" fn textlog_new_connection(mut cnx: *mut picoquic_cnx_t) {}
unsafe extern "C" fn textlog_close_connection(mut cnx: *mut picoquic_cnx_t) {}
unsafe extern "C" fn textlog_cc_dump(mut cnx: *mut picoquic_cnx_t, mut current_time: uint64_t) {
    if !(*(*cnx).quic).F_log.is_null()
        && picoquic_cnx_is_still_logging(cnx as *mut picoquic_cnx_t) != 0
    {
        textlog_congestion_state((*(*cnx).quic).F_log as *mut FILE, cnx, current_time);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_textlog_close(mut quic: *mut picoquic_quic_t) {
    if !(*quic).F_log.is_null() && (*quic).should_close_log() as ::core::ffi::c_int != 0 {
        picoquic_file_close((*quic).F_log as *mut FILE);
    }
    (*quic).F_log = NULL;
    (*quic).set_should_close_log(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub static mut textlog_functions: st_picoquic_unified_logging_t = unsafe {
    st_picoquic_unified_logging_t {
        log_quic_app_message: Some(
            txtlog_context_free_app_message
                as unsafe extern "C" fn(
                    *mut picoquic_quic_t,
                    *const picoquic_connection_id_t,
                    *const ::core::ffi::c_char,
                    ::core::ffi::VaList,
                ) -> (),
        ),
        log_quic_pdu: Some(
            textlog_quic_pdu
                as unsafe extern "C" fn(
                    *mut picoquic_quic_t,
                    ::core::ffi::c_int,
                    uint64_t,
                    uint64_t,
                    *const sockaddr,
                    *const sockaddr,
                    size_t,
                ) -> (),
        ),
        log_quic_close: Some(
            picoquic_textlog_close as unsafe extern "C" fn(*mut picoquic_quic_t) -> (),
        ),
        log_app_message: Some(
            textlog_app_message
                as unsafe extern "C" fn(
                    *mut picoquic_cnx_t,
                    *const ::core::ffi::c_char,
                    ::core::ffi::VaList,
                ) -> (),
        ),
        log_pdu: Some(
            textlog_pdu_ex
                as unsafe extern "C" fn(
                    *mut picoquic_cnx_t,
                    ::core::ffi::c_int,
                    uint64_t,
                    *const sockaddr,
                    *const sockaddr,
                    size_t,
                ) -> (),
        ),
        log_packet: Some(
            textlog_packet
                as unsafe extern "C" fn(
                    *mut picoquic_cnx_t,
                    *mut picoquic_path_t,
                    ::core::ffi::c_int,
                    uint64_t,
                    *mut picoquic_packet_header,
                    *const uint8_t,
                    size_t,
                ) -> (),
        ),
        log_dropped_packet: Some(
            textlog_dropped_packet
                as unsafe extern "C" fn(
                    *mut picoquic_cnx_t,
                    *mut picoquic_path_t,
                    *mut picoquic_packet_header,
                    size_t,
                    ::core::ffi::c_int,
                    *mut uint8_t,
                    uint64_t,
                ) -> (),
        ),
        log_buffered_packet: Some(
            textlog_buffered_packet
                as unsafe extern "C" fn(
                    *mut picoquic_cnx_t,
                    *mut picoquic_path_t,
                    picoquic_packet_type_enum,
                    uint64_t,
                ) -> (),
        ),
        log_outgoing_packet: Some(
            textlog_outgoing_packet
                as unsafe extern "C" fn(
                    *mut picoquic_cnx_t,
                    *mut picoquic_path_t,
                    *mut uint8_t,
                    uint64_t,
                    size_t,
                    size_t,
                    *mut uint8_t,
                    size_t,
                    uint64_t,
                ) -> (),
        ),
        log_packet_lost: Some(
            textlog_packet_lost
                as unsafe extern "C" fn(
                    *mut picoquic_cnx_t,
                    *mut picoquic_path_t,
                    picoquic_packet_type_enum,
                    uint64_t,
                    *const ::core::ffi::c_char,
                    *mut picoquic_connection_id_t,
                    size_t,
                    uint64_t,
                ) -> (),
        ),
        log_negotiated_alpn: Some(
            textlog_negotiated_alpn
                as unsafe extern "C" fn(
                    *mut picoquic_cnx_t,
                    ::core::ffi::c_int,
                    *const uint8_t,
                    size_t,
                    *const uint8_t,
                    size_t,
                    *const ptls_iovec_t,
                    size_t,
                ) -> (),
        ),
        log_transport_extension: Some(
            textlog_transport_extension
                as unsafe extern "C" fn(
                    *mut picoquic_cnx_t,
                    ::core::ffi::c_int,
                    size_t,
                    *mut uint8_t,
                ) -> (),
        ),
        log_picotls_ticket: Some(
            textlog_tls_ticket
                as unsafe extern "C" fn(*mut picoquic_cnx_t, *mut uint8_t, uint16_t) -> (),
        ),
        log_new_connection: Some(
            textlog_new_connection as unsafe extern "C" fn(*mut picoquic_cnx_t) -> (),
        ),
        log_close_connection: Some(
            textlog_close_connection as unsafe extern "C" fn(*mut picoquic_cnx_t) -> (),
        ),
        log_cc_dump: Some(
            textlog_cc_dump as unsafe extern "C" fn(*mut picoquic_cnx_t, uint64_t) -> (),
        ),
    }
};
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_textlog(
    mut quic: *mut picoquic_quic_t,
    mut textlog_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut F_log: *mut FILE = ::core::ptr::null_mut::<FILE>();
    picoquic_textlog_close(quic);
    if !textlog_file.is_null() {
        if strcmp(textlog_file, b"-\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            (*quic).F_log = stdout as *mut ::core::ffi::c_void;
            (*quic).set_should_close_log(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        } else {
            F_log = picoquic_file_open(textlog_file, b"w\0".as_ptr() as *const ::core::ffi::c_char);
            if F_log.is_null() {
                ret = -(1 as ::core::ffi::c_int);
            } else {
                (*quic).F_log = F_log as *mut ::core::ffi::c_void;
                (*quic).set_should_close_log(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
        (*quic).text_log_fns = &raw mut textlog_functions as *mut st_picoquic_unified_logging_t;
    }
    return ret;
}

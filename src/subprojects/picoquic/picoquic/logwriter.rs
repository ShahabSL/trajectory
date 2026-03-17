use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type st_ptls_t;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
    fn fseek(
        __stream: *mut FILE,
        __off: ::core::ffi::c_long,
        __whence: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ftell(__stream: *mut FILE) -> ::core::ffi::c_long;
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
    fn picoquic_get_quic_time(quic: *mut picoquic_quic_t) -> uint64_t;
    fn picoquic_cnx_is_still_logging(cnx: *mut picoquic_cnx_t) -> ::core::ffi::c_int;
    fn picoquic_string_duplicate(original: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn picoquic_string_free(str: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn picoquic_sprintf(
        buf: *mut ::core::ffi::c_char,
        buf_len: size_t,
        nb_chars: *mut size_t,
        fmt: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    static picoquic_null_connection_id: picoquic_connection_id_t;
    fn picoquic_print_connection_id_hexa(
        buf: *mut ::core::ffi::c_char,
        buf_len: size_t,
        cnxid: *const picoquic_connection_id_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_file_open(
        file_name: *const ::core::ffi::c_char,
        flags: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn picoquic_file_close(F: *mut FILE) -> *mut FILE;
    fn picoquic_frames_varint_encode(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        n64: uint64_t,
    ) -> *mut uint8_t;
    fn picoformat_32(bytes: *mut uint8_t, n32: uint32_t);
    fn picoquic_varint_encode(bytes: *mut uint8_t, max_bytes: size_t, n64: uint64_t) -> size_t;
    fn picoquic_varint_decode(
        bytes: *const uint8_t,
        max_bytes: size_t,
        n64: *mut uint64_t,
    ) -> size_t;
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
    fn picoquic_skip_path_abandon_frame(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
    ) -> *const uint8_t;
    fn picoquic_skip_path_available_or_standby_frame(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
    ) -> *const uint8_t;
    fn bytestream_buf_init(s: *mut bytestream_buf, nb_bytes: size_t) -> *mut bytestream;
    fn bytestream_data(s: *mut bytestream) -> *const uint8_t;
    fn bytestream_length(s: *mut bytestream) -> size_t;
    fn bytewrite_int8(s: *mut bytestream, value: uint8_t) -> ::core::ffi::c_int;
    fn bytewrite_int16(s: *mut bytestream, value: uint16_t) -> ::core::ffi::c_int;
    fn bytewrite_int32(s: *mut bytestream, value: uint32_t) -> ::core::ffi::c_int;
    fn bytewrite_int64(s: *mut bytestream, value: uint64_t) -> ::core::ffi::c_int;
    fn bytewrite_vint(s: *mut bytestream, value: uint64_t) -> ::core::ffi::c_int;
    fn bytewrite_buffer(
        s: *mut bytestream,
        buffer: *const ::core::ffi::c_void,
        length: size_t,
    ) -> ::core::ffi::c_int;
    fn bytewrite_cid(
        s: *mut bytestream,
        cid: *const picoquic_connection_id_t,
    ) -> ::core::ffi::c_int;
    fn bytewrite_cstr(s: *mut bytestream, cstr: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn bytewrite_addr(s: *mut bytestream, addr: *const sockaddr) -> ::core::ffi::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_iovec_t {
    pub base: *mut uint8_t,
    pub len: size_t,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_buffer_t {
    pub base: *mut uint8_t,
    pub capacity: size_t,
    pub off: size_t,
    pub is_allocated: uint8_t,
    pub align_bits: uint8_t,
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
pub type ptls_t = st_ptls_t;
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
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const picoquic_frame_type_observed_address_v6: C2Rust_Unnamed = 10453415;
pub const picoquic_frame_type_observed_address_v4: C2Rust_Unnamed = 10453414;
pub const picoquic_frame_type_path_blocked: C2Rust_Unnamed = 354585613;
pub const picoquic_frame_type_max_path_id: C2Rust_Unnamed = 354585612;
pub const picoquic_frame_type_bdp: C2Rust_Unnamed = 60377;
pub const picoquic_frame_type_path_available: C2Rust_Unnamed = 354585608;
pub const picoquic_frame_type_path_backup: C2Rust_Unnamed = 354585607;
pub const picoquic_frame_type_path_abandon: C2Rust_Unnamed = 354585605;
pub const picoquic_frame_type_path_ack_ecn: C2Rust_Unnamed = 354585601;
pub const picoquic_frame_type_path_ack: C2Rust_Unnamed = 354585600;
pub const picoquic_frame_type_time_stamp: C2Rust_Unnamed = 757;
pub const picoquic_frame_type_immediate_ack: C2Rust_Unnamed = 31;
pub const picoquic_frame_type_ack_frequency: C2Rust_Unnamed = 175;
pub const picoquic_frame_type_datagram_l: C2Rust_Unnamed = 49;
pub const picoquic_frame_type_datagram: C2Rust_Unnamed = 48;
pub const picoquic_frame_type_handshake_done: C2Rust_Unnamed = 30;
pub const picoquic_frame_type_application_close: C2Rust_Unnamed = 29;
pub const picoquic_frame_type_connection_close: C2Rust_Unnamed = 28;
pub const picoquic_frame_type_path_response: C2Rust_Unnamed = 27;
pub const picoquic_frame_type_path_challenge: C2Rust_Unnamed = 26;
pub const picoquic_frame_type_path_retire_connection_id: C2Rust_Unnamed = 354585610;
pub const picoquic_frame_type_retire_connection_id: C2Rust_Unnamed = 25;
pub const picoquic_frame_type_path_new_connection_id: C2Rust_Unnamed = 354585609;
pub const picoquic_frame_type_new_connection_id: C2Rust_Unnamed = 24;
pub const picoquic_frame_type_streams_blocked_unidir: C2Rust_Unnamed = 23;
pub const picoquic_frame_type_streams_blocked_bidir: C2Rust_Unnamed = 22;
pub const picoquic_frame_type_stream_data_blocked: C2Rust_Unnamed = 21;
pub const picoquic_frame_type_data_blocked: C2Rust_Unnamed = 20;
pub const picoquic_frame_type_max_streams_unidir: C2Rust_Unnamed = 19;
pub const picoquic_frame_type_max_streams_bidir: C2Rust_Unnamed = 18;
pub const picoquic_frame_type_max_stream_data: C2Rust_Unnamed = 17;
pub const picoquic_frame_type_max_data: C2Rust_Unnamed = 16;
pub const picoquic_frame_type_stream_range_max: C2Rust_Unnamed = 15;
pub const picoquic_frame_type_stream_range_min: C2Rust_Unnamed = 8;
pub const picoquic_frame_type_new_token: C2Rust_Unnamed = 7;
pub const picoquic_frame_type_crypto_hs: C2Rust_Unnamed = 6;
pub const picoquic_frame_type_stop_sending: C2Rust_Unnamed = 5;
pub const picoquic_frame_type_reset_stream: C2Rust_Unnamed = 4;
pub const picoquic_frame_type_ack_ecn: C2Rust_Unnamed = 3;
pub const picoquic_frame_type_ack: C2Rust_Unnamed = 2;
pub const picoquic_frame_type_poll: C2Rust_Unnamed = 32;
pub const picoquic_frame_type_ping: C2Rust_Unnamed = 1;
pub const picoquic_frame_type_padding: C2Rust_Unnamed = 0;
pub type picoquic_packet_header = st_picoquic_packet_header_t;
pub type picoquic_log_event_type = ::core::ffi::c_uint;
pub const picoquic_log_event_frame_recv: picoquic_log_event_type = 131;
pub const picoquic_log_event_frame_sent: picoquic_log_event_type = 130;
pub const picoquic_log_event_info_message: picoquic_log_event_type = 58;
pub const picoquic_log_event_stream_update: picoquic_log_event_type = 57;
pub const picoquic_log_event_cc_update: picoquic_log_event_type = 56;
pub const picoquic_log_event_alpn_update: picoquic_log_event_type = 55;
pub const picoquic_log_event_param_update: picoquic_log_event_type = 54;
pub const picoquic_log_event_version_update: picoquic_log_event_type = 53;
pub const picoquic_log_event_tls_key_retired: picoquic_log_event_type = 33;
pub const picoquic_log_event_tls_key_update: picoquic_log_event_type = 32;
pub const picoquic_log_event_packet_buffered: picoquic_log_event_type = 21;
pub const picoquic_log_event_packet_dropped: picoquic_log_event_type = 20;
pub const picoquic_log_event_packet_lost: picoquic_log_event_type = 19;
pub const picoquic_log_event_connection_id_update: picoquic_log_event_type = 18;
pub const picoquic_log_event_connection_close: picoquic_log_event_type = 17;
pub const picoquic_log_event_new_connection: picoquic_log_event_type = 16;
pub const picoquic_log_event_packet_recv: picoquic_log_event_type = 9;
pub const picoquic_log_event_packet_sent: picoquic_log_event_type = 8;
pub const picoquic_log_event_pdu_recv: picoquic_log_event_type = 3;
pub const picoquic_log_event_pdu_sent: picoquic_log_event_type = 2;
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
pub const SEEK_SET: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SEEK_END: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const PICOQUIC_ERROR_CLASS: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_AEAD_CHECK: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 3 as ::core::ffi::c_int;
pub const PICOQUIC_RESET_SECRET_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PICOQUIC_FILE_SEPARATOR: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"/\0") };
pub const BYTESTREAM_MAX_BUFFER_SIZE: ::core::ffi::c_int = 2560 as ::core::ffi::c_int;
unsafe extern "C" fn picoquic_log_fixed_skip(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut size: size_t,
) -> *const uint8_t {
    return if bytes.is_null() {
        ::core::ptr::null::<uint8_t>()
    } else {
        bytes = bytes.offset(size as isize);
        if bytes <= bytes_max {
            bytes
        } else {
            ::core::ptr::null::<uint8_t>()
        }
    };
}
unsafe extern "C" fn picoquic_log_varint_skip(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    return if bytes.is_null() {
        ::core::ptr::null::<uint8_t>()
    } else if bytes < bytes_max {
        picoquic_log_fixed_skip(
            bytes,
            bytes_max,
            (1 as ::core::ffi::c_int as size_t)
                << (*bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    >> 6 as ::core::ffi::c_int
                    & 3 as ::core::ffi::c_int),
        )
    } else {
        ::core::ptr::null::<uint8_t>()
    };
}
unsafe extern "C" fn picoquic_log_varint(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut n64: *mut uint64_t,
) -> *const uint8_t {
    let mut len: size_t = if bytes.is_null() {
        0 as size_t
    } else {
        picoquic_varint_decode(
            bytes,
            bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t,
            n64,
        )
    };
    return if len == 0 as size_t {
        ::core::ptr::null::<uint8_t>()
    } else {
        bytes.offset(len as isize)
    };
}
unsafe extern "C" fn picoquic_log_length(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut nsz: *mut size_t,
) -> *const uint8_t {
    let mut n64: uint64_t = 0 as uint64_t;
    let mut len: size_t = 0 as size_t;
    if !bytes.is_null() {
        len = picoquic_varint_decode(
            bytes,
            bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t,
            &raw mut n64,
        );
    }
    *nsz = n64 as size_t;
    return if len == 0 as size_t || *nsz != n64 as size_t {
        ::core::ptr::null::<uint8_t>()
    } else {
        bytes.offset(len as isize)
    };
}
unsafe extern "C" fn picoquic_binlog_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) {
    if !bytes.is_null() && !bytes_max.is_null() {
        let mut len: size_t = bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t;
        let mut varlen: [uint8_t; 8] = [0; 8];
        let mut l_varlen: size_t = picoquic_varint_encode(
            &raw mut varlen as *mut uint8_t,
            8 as size_t,
            len as uint64_t,
        );
        fwrite(
            &raw mut varlen as *mut uint8_t as *const ::core::ffi::c_void,
            1 as size_t,
            l_varlen,
            f,
        );
        fwrite(bytes as *const ::core::ffi::c_void, 1 as size_t, len, f);
    }
}
unsafe extern "C" fn picoquic_log_stream_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    let mut ftype: uint8_t = *bytes.offset(0 as ::core::ffi::c_int as isize);
    let mut length: size_t = 0 as size_t;
    let mut log_buffer: [uint8_t; 256] = [0; 256];
    let mut has_length: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut extra_bytes: size_t = 8 as size_t;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    if ftype as ::core::ffi::c_int & 4 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        bytes = picoquic_log_varint_skip(bytes, bytes_max);
    }
    if !bytes.is_null() {
        if ftype as ::core::ffi::c_int & 2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            bytes = picoquic_log_length(bytes, bytes_max, &raw mut length);
            has_length = 1 as ::core::ffi::c_int;
        } else {
            length = bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t;
        }
    }
    if !bytes.is_null() {
        if length < extra_bytes {
            extra_bytes = length;
        }
        if has_length != 0 {
            picoquic_binlog_frame(f, bytes_begin, bytes.offset(extra_bytes as isize));
        } else {
            let mut log_next: *mut uint8_t = &raw mut log_buffer as *mut uint8_t;
            let mut l_head: size_t =
                bytes.offset_from(bytes_begin) as ::core::ffi::c_long as size_t;
            memcpy(
                &raw mut log_buffer as *mut uint8_t as *mut ::core::ffi::c_void,
                bytes_begin as *const ::core::ffi::c_void,
                l_head,
            );
            log_next = log_next.offset(l_head as isize);
            log_next = picoquic_frames_varint_encode(
                log_next,
                (&raw mut log_buffer as *mut uint8_t).offset(256 as ::core::ffi::c_int as isize),
                length as uint64_t,
            );
            if !log_next.is_null() {
                memcpy(
                    log_next as *mut ::core::ffi::c_void,
                    bytes as *const ::core::ffi::c_void,
                    extra_bytes,
                );
                log_next = log_next.offset(extra_bytes as isize);
                picoquic_binlog_frame(f, &raw mut log_buffer as *mut uint8_t, log_next);
            } else {
                picoquic_binlog_frame(
                    f,
                    &raw mut log_buffer as *mut uint8_t,
                    (&raw mut log_buffer as *mut uint8_t).offset(l_head as isize),
                );
            }
        }
        bytes = picoquic_log_fixed_skip(bytes, bytes_max, length);
    } else {
        length = bytes_max.offset_from(bytes_begin) as ::core::ffi::c_long as size_t;
        if length > 26 as size_t {
            length = 26 as size_t;
        }
        picoquic_binlog_frame(f, bytes_begin, bytes_begin.offset(length as isize));
    }
    return bytes;
}
unsafe extern "C" fn picoquic_log_ack_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    let mut ftype: uint64_t = 0 as uint64_t;
    let mut nb_blocks: uint64_t = 0;
    picoquic_varint_decode(
        bytes,
        bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t,
        &raw mut ftype,
    );
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    if ftype == picoquic_frame_type_path_ack as ::core::ffi::c_int as uint64_t
        || ftype == picoquic_frame_type_path_ack_ecn as ::core::ffi::c_int as uint64_t
    {
        bytes = picoquic_log_varint_skip(bytes, bytes_max);
    }
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint(bytes, bytes_max, &raw mut nb_blocks);
    let mut i: uint64_t = 0 as uint64_t;
    while !bytes.is_null() && i <= nb_blocks {
        if i != 0 as uint64_t {
            bytes = picoquic_log_varint_skip(bytes, bytes_max);
        }
        bytes = picoquic_log_varint_skip(bytes, bytes_max);
        i = i.wrapping_add(1);
    }
    if ftype == picoquic_frame_type_ack_ecn as ::core::ffi::c_int as uint64_t
        || ftype == picoquic_frame_type_path_ack_ecn as ::core::ffi::c_int as uint64_t
    {
        bytes = picoquic_log_varint_skip(bytes, bytes_max);
        bytes = picoquic_log_varint_skip(bytes, bytes_max);
        bytes = picoquic_log_varint_skip(bytes, bytes_max);
    }
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_reset_stream_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_stop_sending_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_close_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    let mut length: size_t = 0 as size_t;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_length(bytes, bytes_max, &raw mut length);
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, length);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_app_close_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    let mut length: size_t = 0 as size_t;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_length(bytes, bytes_max, &raw mut length);
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, length);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_max_data_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_max_stream_data_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_max_stream_id_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_blocked_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_stream_blocked_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_streams_blocked_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_new_connection_id_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    if !bytes.is_null() {
        bytes = picoquic_log_fixed_skip(
            bytes,
            bytes_max,
            (1 as ::core::ffi::c_int as size_t)
                .wrapping_add(*bytes.offset(0 as ::core::ffi::c_int as isize) as size_t),
        );
    }
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, PICOQUIC_RESET_SECRET_SIZE as size_t);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_path_new_connection_id_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    if !bytes.is_null() {
        bytes = picoquic_log_fixed_skip(
            bytes,
            bytes_max,
            (1 as ::core::ffi::c_int as size_t)
                .wrapping_add(*bytes.offset(0 as ::core::ffi::c_int as isize) as size_t),
        );
    }
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, PICOQUIC_RESET_SECRET_SIZE as size_t);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_retire_connection_id_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_path_retire_connection_id_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_new_token_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    let mut length: size_t = 0 as size_t;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    bytes = picoquic_log_length(bytes, bytes_max, &raw mut length);
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, length);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_path_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_fixed_skip(
        bytes,
        bytes_max,
        (1 as ::core::ffi::c_int + 8 as ::core::ffi::c_int) as size_t,
    );
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_crypto_hs_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    let mut length: size_t = 0 as size_t;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_length(bytes, bytes_max, &raw mut length);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, length);
    return bytes;
}
unsafe extern "C" fn picoquic_log_handshake_done_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_datagram_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    let mut ftype: uint8_t = *bytes.offset(0 as ::core::ffi::c_int as isize);
    let mut length: size_t = 0 as size_t;
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, 1 as size_t);
    if ftype as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
        bytes = picoquic_log_length(bytes, bytes_max, &raw mut length);
    } else {
        length = bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t;
    }
    picoquic_binlog_frame(f, bytes_begin, bytes);
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, length);
    return bytes;
}
unsafe extern "C" fn picoquic_log_time_stamp_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_path_abandon_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_skip_path_abandon_frame(bytes, bytes_max);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_path_available_or_backup_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_skip_path_available_or_standby_frame(bytes, bytes_max);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_ack_frequency_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_immediate_ack_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_erroring_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut frame_size: size_t = bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t;
    let mut copied: size_t = if frame_size > 8 as size_t {
        8 as size_t
    } else {
        frame_size
    };
    picoquic_binlog_frame(f, bytes, bytes.offset(copied as isize));
    return ::core::ptr::null::<uint8_t>();
}
unsafe extern "C" fn picoquic_log_padding(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    picoquic_binlog_frame(f, bytes, bytes.offset(1 as ::core::ffi::c_int as isize));
    let mut ftype: uint8_t = *bytes.offset(0 as ::core::ffi::c_int as isize);
    while bytes < bytes_max
        && *bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == ftype as ::core::ffi::c_int
    {
        bytes = bytes.offset(1);
    }
    return bytes;
}
unsafe extern "C" fn picoquic_log_bdp_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    let mut ip_len: size_t = 0 as size_t;
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_length(bytes, bytes_max, &raw mut ip_len);
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, ip_len);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
unsafe extern "C" fn picoquic_log_observed_address_frame(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut ftype: uint64_t,
) -> *const uint8_t {
    let mut bytes_begin: *const uint8_t = bytes;
    let mut ip_len: size_t = (if ftype & 1 as uint64_t == 0 as uint64_t {
        4 as ::core::ffi::c_int
    } else {
        16 as ::core::ffi::c_int
    }) as size_t;
    let mut data_len: size_t = ip_len.wrapping_add(2 as size_t);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_varint_skip(bytes, bytes_max);
    bytes = picoquic_log_fixed_skip(bytes, bytes_max, data_len);
    picoquic_binlog_frame(f, bytes_begin, bytes);
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_binlog_frames(
    mut f: *mut FILE,
    mut bytes: *const uint8_t,
    mut length: size_t,
) {
    let mut bytes_max: *const uint8_t = bytes.offset(length as isize);
    while !bytes.is_null() && bytes < bytes_max {
        let mut ftype: uint64_t = 0 as uint64_t;
        let mut ftype_ll: size_t = picoquic_varint_decode(bytes, length, &raw mut ftype);
        if ftype_ll == 0 as size_t {
            bytes = ::core::ptr::null::<uint8_t>();
            break;
        } else if ftype < 64 as uint64_t && ftype_ll != 1 as size_t {
            bytes = ::core::ptr::null::<uint8_t>();
            break;
        } else if ftype
            & !(picoquic_frame_type_stream_range_min as ::core::ffi::c_int
                ^ picoquic_frame_type_stream_range_max as ::core::ffi::c_int)
                as uint64_t
            == picoquic_frame_type_stream_range_min as ::core::ffi::c_int as uint64_t
        {
            bytes = picoquic_log_stream_frame(f, bytes, bytes_max);
        } else {
            match ftype {
                2 | 3 | 354585600 | 354585601 => {
                    bytes = picoquic_log_ack_frame(f, bytes, bytes_max);
                }
                25 => {
                    bytes = picoquic_log_retire_connection_id_frame(f, bytes, bytes_max);
                }
                354585610 => {
                    bytes = picoquic_log_path_retire_connection_id_frame(f, bytes, bytes_max);
                }
                0 | 1 | 32 => {
                    bytes = picoquic_log_padding(f, bytes, bytes_max);
                }
                4 => {
                    bytes = picoquic_log_reset_stream_frame(f, bytes, bytes_max);
                }
                28 => {
                    bytes = picoquic_log_close_frame(f, bytes, bytes_max);
                }
                29 => {
                    bytes = picoquic_log_app_close_frame(f, bytes, bytes_max);
                }
                16 => {
                    bytes = picoquic_log_max_data_frame(f, bytes, bytes_max);
                }
                17 => {
                    bytes = picoquic_log_max_stream_data_frame(f, bytes, bytes_max);
                }
                18 | 19 => {
                    bytes = picoquic_log_max_stream_id_frame(f, bytes, bytes_max);
                }
                20 => {
                    bytes = picoquic_log_blocked_frame(f, bytes, bytes_max);
                }
                21 => {
                    bytes = picoquic_log_stream_blocked_frame(f, bytes, bytes_max);
                }
                22 | 23 => {
                    bytes = picoquic_log_streams_blocked_frame(f, bytes, bytes_max);
                }
                24 => {
                    bytes = picoquic_log_new_connection_id_frame(f, bytes, bytes_max);
                }
                354585609 => {
                    bytes = picoquic_log_path_new_connection_id_frame(f, bytes, bytes_max);
                }
                5 => {
                    bytes = picoquic_log_stop_sending_frame(f, bytes, bytes_max);
                }
                26 | 27 => {
                    bytes = picoquic_log_path_frame(f, bytes, bytes_max);
                }
                6 => {
                    bytes = picoquic_log_crypto_hs_frame(f, bytes, bytes_max);
                }
                7 => {
                    bytes = picoquic_log_new_token_frame(f, bytes, bytes_max);
                }
                30 => {
                    bytes = picoquic_log_handshake_done_frame(f, bytes, bytes_max);
                }
                48 | 49 => {
                    bytes = picoquic_log_datagram_frame(f, bytes, bytes_max);
                }
                175 => {
                    bytes = picoquic_log_ack_frequency_frame(f, bytes, bytes_max);
                }
                31 => {
                    bytes = picoquic_log_immediate_ack_frame(f, bytes, bytes_max);
                }
                757 => {
                    bytes = picoquic_log_time_stamp_frame(f, bytes, bytes_max);
                }
                354585605 => {
                    bytes = picoquic_log_path_abandon_frame(f, bytes, bytes_max);
                }
                354585607 | 354585608 => {
                    bytes = picoquic_log_path_available_or_backup_frame(f, bytes, bytes_max);
                }
                60377 => {
                    bytes = picoquic_log_bdp_frame(f, bytes, bytes_max);
                }
                10453414 | 10453415 => {
                    bytes = picoquic_log_observed_address_frame(f, bytes, bytes_max, ftype);
                }
                _ => {
                    bytes = picoquic_log_erroring_frame(f, bytes, bytes_max);
                }
            }
        }
    }
}
unsafe extern "C" fn binlog_compose_event_header(
    mut msg: *mut bytestream,
    mut cid: *const picoquic_connection_id_t,
    mut current_time: uint64_t,
    mut path_id: uint64_t,
    mut event_type: picoquic_log_event_type,
) {
    bytewrite_cid(msg, cid);
    bytewrite_vint(msg, current_time);
    bytewrite_vint(msg, path_id);
    bytewrite_vint(msg, event_type as uint64_t);
}
unsafe extern "C" fn binlog_get_path_id(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
) -> uint64_t {
    let mut path_id: uint64_t = 0 as uint64_t;
    if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0 && !path_x.is_null() {
        path_id = (*path_x).unique_path_id;
    }
    return path_id;
}
#[no_mangle]
pub unsafe extern "C" fn binlog_pdu(
    mut f: *mut FILE,
    mut cid: *const picoquic_connection_id_t,
    mut receiving: ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut addr_peer: *const sockaddr,
    mut addr_local: *const sockaddr,
    mut packet_length: size_t,
) {
    let mut stream_msg: bytestream_buf = bytestream_buf {
        s: bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        },
        buf: [0; 2560],
    };
    let mut msg: *mut bytestream =
        bytestream_buf_init(&raw mut stream_msg, BYTESTREAM_MAX_BUFFER_SIZE as size_t);
    binlog_compose_event_header(
        msg,
        cid,
        current_time,
        0 as uint64_t,
        (picoquic_log_event_pdu_sent as ::core::ffi::c_int + receiving) as picoquic_log_event_type,
    );
    bytewrite_addr(msg, addr_peer);
    bytewrite_vint(msg, packet_length as uint64_t);
    bytewrite_addr(msg, addr_local);
    let mut head: [uint8_t; 4] = [0 as ::core::ffi::c_int as uint8_t, 0, 0, 0];
    picoformat_32(
        &raw mut head as *mut uint8_t,
        bytestream_length(msg) as uint32_t,
    );
    fwrite(
        &raw mut head as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
        1 as size_t,
        f,
    );
    fwrite(
        bytestream_data(msg) as *const ::core::ffi::c_void,
        bytestream_length(msg),
        1 as size_t,
        f,
    );
}
unsafe extern "C" fn binlog_pdu_ex(
    mut cnx: *mut picoquic_cnx_t,
    mut receiving: ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut addr_peer: *const sockaddr,
    mut addr_local: *const sockaddr,
    mut packet_length: size_t,
) {
    if !cnx.is_null()
        && !(*cnx).f_binlog.is_null()
        && picoquic_cnx_is_still_logging(cnx as *mut picoquic_cnx_t) != 0
    {
        binlog_pdu(
            (*cnx).f_binlog,
            &raw mut (*cnx).initial_cnxid,
            receiving,
            current_time,
            addr_peer,
            addr_local,
            packet_length,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn binlog_packet(
    mut f: *mut FILE,
    mut cid: *const picoquic_connection_id_t,
    mut path_id: uint64_t,
    mut receiving: ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut ph: *const picoquic_packet_header,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) {
    let mut fpos0: ::core::ffi::c_long = ftell(f);
    let mut head: [uint8_t; 4] = [0 as ::core::ffi::c_int as uint8_t, 0, 0, 0];
    fwrite(
        &raw mut head as *mut uint8_t as *const ::core::ffi::c_void,
        4 as size_t,
        1 as size_t,
        f,
    );
    let mut stream_msg: bytestream_buf = bytestream_buf {
        s: bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        },
        buf: [0; 2560],
    };
    let mut msg: *mut bytestream =
        bytestream_buf_init(&raw mut stream_msg, BYTESTREAM_MAX_BUFFER_SIZE as size_t);
    binlog_compose_event_header(
        msg,
        cid,
        current_time,
        path_id,
        (picoquic_log_event_packet_sent as ::core::ffi::c_int + receiving)
            as picoquic_log_event_type,
    );
    bytewrite_vint(msg, bytes_max as uint64_t);
    bytewrite_int8(
        msg,
        (64 as ::core::ffi::c_int * (*ph).quic_bit_is_zero() as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int * (*ph).spin() as ::core::ffi::c_int
            + (*ph).key_phase() as ::core::ffi::c_int) as uint8_t,
    );
    bytewrite_vint(msg, (*ph).payload_length as uint64_t);
    bytewrite_vint(msg, (*ph).ptype as uint64_t);
    bytewrite_vint(msg, (*ph).pn64);
    bytewrite_cid(msg, &raw const (*ph).dest_cnx_id);
    bytewrite_cid(msg, &raw const (*ph).srce_cnx_id);
    if (*ph).ptype as ::core::ffi::c_uint
        != picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*ph).ptype as ::core::ffi::c_uint
            != picoquic_packet_version_negotiation as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        bytewrite_int32(msg, (*ph).vn);
    }
    if (*ph).ptype as ::core::ffi::c_uint
        == picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        bytewrite_vint(msg, (*ph).token_length as uint64_t);
        bytewrite_buffer(
            msg,
            (*ph).token_bytes as *const ::core::ffi::c_void,
            (*ph).token_length,
        );
    }
    fwrite(
        bytestream_data(msg) as *const ::core::ffi::c_void,
        bytestream_length(msg),
        1 as size_t,
        f,
    );
    if (*ph).ptype as ::core::ffi::c_uint
        == picoquic_packet_version_negotiation as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*ph).ptype as ::core::ffi::c_uint
            == picoquic_packet_retry as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        picoquic_binlog_frame(
            f,
            bytes.offset((*ph).offset as isize),
            bytes.offset(bytes_max as isize),
        );
    } else if (*ph).ptype as ::core::ffi::c_uint
        != picoquic_packet_error as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        picoquic_binlog_frames(f, bytes.offset((*ph).offset as isize), (*ph).payload_length);
    }
    let mut fpos1: ::core::ffi::c_long = ftell(f);
    picoformat_32(
        &raw mut head as *mut uint8_t,
        (fpos1 - fpos0 - 4 as ::core::ffi::c_long) as uint32_t,
    );
    fseek(f, fpos0, SEEK_SET);
    fwrite(
        &raw mut head as *mut uint8_t as *const ::core::ffi::c_void,
        4 as size_t,
        1 as size_t,
        f,
    );
    fseek(f, 0 as ::core::ffi::c_long, SEEK_END);
}
unsafe extern "C" fn binlog_packet_ex(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut receiving: ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut ph: *mut picoquic_packet_header,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) {
    if !cnx.is_null()
        && !(*cnx).f_binlog.is_null()
        && picoquic_cnx_is_still_logging(cnx as *mut picoquic_cnx_t) != 0
    {
        binlog_packet(
            (*cnx).f_binlog,
            &raw mut (*cnx).initial_cnxid,
            binlog_get_path_id(cnx, path_x),
            receiving,
            current_time,
            ph,
            bytes,
            bytes_max,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn binlog_dropped_packet(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut ph: *mut picoquic_packet_header,
    mut packet_size: size_t,
    mut err: ::core::ffi::c_int,
    mut raw_data: *mut uint8_t,
    mut current_time: uint64_t,
) {
    let mut f: *mut FILE = (*cnx).f_binlog;
    let mut raw_size: size_t = packet_size;
    let mut stream_msg: bytestream_buf = bytestream_buf {
        s: bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        },
        buf: [0; 2560],
    };
    let mut msg: *mut bytestream =
        bytestream_buf_init(&raw mut stream_msg, BYTESTREAM_MAX_BUFFER_SIZE as size_t);
    if err == PICOQUIC_ERROR_AEAD_CHECK {
        raw_size = 0 as size_t;
    } else if raw_size > 32 as size_t {
        raw_size = 32 as size_t;
    }
    bytewrite_int32(msg, 0 as uint32_t);
    binlog_compose_event_header(
        msg,
        &raw mut (*cnx).initial_cnxid,
        current_time,
        binlog_get_path_id(cnx, path_x),
        picoquic_log_event_packet_dropped,
    );
    bytewrite_vint(msg, (*ph).ptype as uint64_t);
    bytewrite_vint(msg, packet_size as uint64_t);
    bytewrite_vint(msg, err as uint64_t);
    bytewrite_vint(msg, raw_size as uint64_t);
    bytewrite_buffer(msg, raw_data as *const ::core::ffi::c_void, raw_size);
    picoformat_32(
        (*msg).data,
        (*msg).ptr.wrapping_sub(4 as size_t) as uint32_t,
    );
    fwrite(
        bytestream_data(msg) as *const ::core::ffi::c_void,
        bytestream_length(msg),
        1 as size_t,
        f,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binlog_buffered_packet(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut ptype: picoquic_packet_type_enum,
    mut current_time: uint64_t,
) {
    let mut f: *mut FILE = (*cnx).f_binlog;
    let mut stream_msg: bytestream_buf = bytestream_buf {
        s: bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        },
        buf: [0; 2560],
    };
    let mut msg: *mut bytestream =
        bytestream_buf_init(&raw mut stream_msg, BYTESTREAM_MAX_BUFFER_SIZE as size_t);
    bytewrite_int32(msg, 0 as uint32_t);
    binlog_compose_event_header(
        msg,
        &raw mut (*cnx).initial_cnxid,
        current_time,
        binlog_get_path_id(cnx, path_x),
        picoquic_log_event_packet_buffered,
    );
    bytewrite_vint(msg, ptype as uint64_t);
    bytewrite_cstr(
        msg,
        b"keys_unavailable\0".as_ptr() as *const ::core::ffi::c_char,
    );
    picoformat_32(
        (*msg).data,
        (*msg).ptr.wrapping_sub(4 as size_t) as uint32_t,
    );
    fwrite(
        bytestream_data(msg) as *const ::core::ffi::c_void,
        bytestream_length(msg),
        1 as size_t,
        f,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binlog_outgoing_packet(
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
    let mut f: *mut FILE = (*cnx).f_binlog;
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
    let mut checksum_length: size_t = 16 as size_t;
    let mut default_addr: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut cnxid: *const picoquic_connection_id_t = if !cnx.is_null() {
        &raw mut (*cnx).initial_cnxid as *const picoquic_connection_id_t
    } else {
        &raw const picoquic_null_connection_id
    };
    memset(
        &raw mut default_addr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_in>() as size_t,
    );
    default_addr.sin_family = AF_INET as sa_family_t;
    picoquic_parse_packet_header(
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
    ph.pn64 = sequence_number;
    ph.pn = ph.pn64 as uint32_t;
    if ph.ptype as ::core::ffi::c_uint
        != picoquic_packet_retry as ::core::ffi::c_int as ::core::ffi::c_uint
    {
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
    binlog_packet(
        f,
        cnxid,
        binlog_get_path_id(cnx, path_x),
        0 as ::core::ffi::c_int,
        current_time,
        &raw mut ph,
        bytes,
        length,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binlog_packet_lost(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut ptype: picoquic_packet_type_enum,
    mut sequence_number: uint64_t,
    mut trigger: *const ::core::ffi::c_char,
    mut dcid: *mut picoquic_connection_id_t,
    mut packet_size: size_t,
    mut current_time: uint64_t,
) {
    let mut f: *mut FILE = (*cnx).f_binlog;
    let mut stream_msg: bytestream_buf = bytestream_buf {
        s: bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        },
        buf: [0; 2560],
    };
    let mut msg: *mut bytestream =
        bytestream_buf_init(&raw mut stream_msg, BYTESTREAM_MAX_BUFFER_SIZE as size_t);
    bytewrite_int32(msg, 0 as uint32_t);
    binlog_compose_event_header(
        msg,
        &raw mut (*cnx).initial_cnxid,
        current_time,
        binlog_get_path_id(cnx, path_x),
        picoquic_log_event_packet_lost,
    );
    bytewrite_vint(msg, ptype as uint64_t);
    bytewrite_vint(msg, sequence_number);
    bytewrite_cstr(msg, trigger);
    if !dcid.is_null() {
        bytewrite_cid(msg, dcid);
    } else {
        bytewrite_int8(msg, 0 as uint8_t);
    }
    bytewrite_vint(msg, packet_size as uint64_t);
    picoformat_32(
        (*msg).data,
        (*msg).ptr.wrapping_sub(4 as size_t) as uint32_t,
    );
    fwrite(
        bytestream_data(msg) as *const ::core::ffi::c_void,
        bytestream_length(msg),
        1 as size_t,
        f,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binlog_negotiated_alpn(
    mut cnx: *mut picoquic_cnx_t,
    mut is_local: ::core::ffi::c_int,
    mut sni: *const uint8_t,
    mut sni_len: size_t,
    mut alpn: *const uint8_t,
    mut alpn_len: size_t,
    mut alpn_list: *const ptls_iovec_t,
    mut alpn_count: size_t,
) {
    let mut f: *mut FILE = (*cnx).f_binlog;
    let mut stream_msg: bytestream_buf = bytestream_buf {
        s: bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        },
        buf: [0; 2560],
    };
    let mut msg: *mut bytestream =
        bytestream_buf_init(&raw mut stream_msg, BYTESTREAM_MAX_BUFFER_SIZE as size_t);
    binlog_compose_event_header(
        msg,
        &raw mut (*cnx).initial_cnxid,
        picoquic_get_quic_time((*cnx).quic as *mut picoquic_quic_t),
        0 as uint64_t,
        picoquic_log_event_alpn_update,
    );
    bytewrite_vint(msg, is_local as uint64_t);
    bytewrite_vint(msg, sni_len as uint64_t);
    if sni_len > 0 as size_t {
        bytewrite_buffer(msg, sni as *const ::core::ffi::c_void, sni_len);
    }
    bytewrite_vint(msg, alpn_count as uint64_t);
    if alpn_count > 0 as size_t {
        let mut i: size_t = 0 as size_t;
        while i < alpn_count {
            bytewrite_vint(msg, (*alpn_list.offset(i as isize)).len as uint64_t);
            bytewrite_buffer(
                msg,
                (*alpn_list.offset(i as isize)).base as *const ::core::ffi::c_void,
                (*alpn_list.offset(i as isize)).len,
            );
            i = i.wrapping_add(1);
        }
    }
    bytewrite_vint(msg, alpn_len as uint64_t);
    if alpn_len > 0 as size_t {
        bytewrite_buffer(msg, alpn as *const ::core::ffi::c_void, alpn_len);
    }
    let mut stream_head: bytestream_buf = bytestream_buf {
        s: bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        },
        buf: [0; 2560],
    };
    let mut head: *mut bytestream = bytestream_buf_init(&raw mut stream_head, 4 as size_t);
    bytewrite_int32(head, bytestream_length(msg) as uint32_t);
    fwrite(
        bytestream_data(head) as *const ::core::ffi::c_void,
        bytestream_length(head),
        1 as size_t,
        f,
    );
    fwrite(
        bytestream_data(msg) as *const ::core::ffi::c_void,
        bytestream_length(msg),
        1 as size_t,
        f,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binlog_transport_extension(
    mut cnx: *mut picoquic_cnx_t,
    mut is_local: ::core::ffi::c_int,
    mut param_length: size_t,
    mut params: *mut uint8_t,
) {
    let mut f: *mut FILE = (*cnx).f_binlog;
    let mut stream_msg: bytestream_buf = bytestream_buf {
        s: bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        },
        buf: [0; 2560],
    };
    let mut msg: *mut bytestream =
        bytestream_buf_init(&raw mut stream_msg, BYTESTREAM_MAX_BUFFER_SIZE as size_t);
    binlog_compose_event_header(
        msg,
        &raw mut (*cnx).initial_cnxid,
        picoquic_get_quic_time((*cnx).quic as *mut picoquic_quic_t),
        0 as uint64_t,
        picoquic_log_event_param_update,
    );
    bytewrite_vint(msg, is_local as uint64_t);
    bytewrite_vint(msg, param_length as uint64_t);
    if param_length > 0 as size_t {
        bytewrite_buffer(msg, params as *const ::core::ffi::c_void, param_length);
    }
    let mut stream_head: bytestream_buf = bytestream_buf {
        s: bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        },
        buf: [0; 2560],
    };
    let mut head: *mut bytestream = bytestream_buf_init(&raw mut stream_head, 4 as size_t);
    bytewrite_int32(head, bytestream_length(msg) as uint32_t);
    fwrite(
        bytestream_data(head) as *const ::core::ffi::c_void,
        bytestream_length(head),
        1 as size_t,
        f,
    );
    fwrite(
        bytestream_data(msg) as *const ::core::ffi::c_void,
        bytestream_length(msg),
        1 as size_t,
        f,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binlog_picotls_ticket(
    mut f: *mut FILE,
    mut cnx_id: picoquic_connection_id_t,
    mut ticket: *mut uint8_t,
    mut ticket_length: uint16_t,
) {
    let mut stream_msg: bytestream_buf = bytestream_buf {
        s: bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        },
        buf: [0; 2560],
    };
    let mut msg: *mut bytestream =
        bytestream_buf_init(&raw mut stream_msg, BYTESTREAM_MAX_BUFFER_SIZE as size_t);
    binlog_compose_event_header(
        msg,
        &raw mut cnx_id,
        0 as uint64_t,
        0 as uint64_t,
        picoquic_log_event_tls_key_update,
    );
    bytewrite_vint(msg, ticket_length as uint64_t);
    bytewrite_buffer(
        msg,
        ticket as *const ::core::ffi::c_void,
        ticket_length as size_t,
    );
    let mut stream_head: bytestream_buf = bytestream_buf {
        s: bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        },
        buf: [0; 2560],
    };
    let mut head: *mut bytestream = bytestream_buf_init(&raw mut stream_head, 8 as size_t);
    bytewrite_int32(head, bytestream_length(msg) as uint32_t);
    fwrite(
        bytestream_data(head) as *const ::core::ffi::c_void,
        bytestream_length(head),
        1 as size_t,
        f,
    );
    fwrite(
        bytestream_data(msg) as *const ::core::ffi::c_void,
        bytestream_length(msg),
        1 as size_t,
        f,
    );
}
unsafe extern "C" fn binlog_picotls_ticket_ex(
    mut cnx: *mut picoquic_cnx_t,
    mut ticket: *mut uint8_t,
    mut ticket_length: uint16_t,
) {
    if !cnx.is_null()
        && !(*cnx).f_binlog.is_null()
        && picoquic_cnx_is_still_logging(cnx as *mut picoquic_cnx_t) != 0
    {
        binlog_picotls_ticket((*cnx).f_binlog, (*cnx).initial_cnxid, ticket, ticket_length);
    }
}
#[no_mangle]
pub unsafe extern "C" fn binlog_new_connection(mut cnx: *mut picoquic_cnx_t) {
    let mut bin_dir: *const ::core::ffi::c_char = if (*(*cnx).quic).binlog_dir.is_null() {
        (*(*cnx).quic).qlog_dir
    } else {
        (*(*cnx).quic).binlog_dir
    };
    if bin_dir.is_null() {
        return;
    }
    if (*(*cnx).quic).current_number_of_open_logs >= (*(*cnx).quic).max_simultaneous_logs {
        return;
    }
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*cnx).f_binlog = picoquic_file_close((*cnx).f_binlog);
    let mut cid_name: [::core::ffi::c_char; 41] = [0; 41];
    if picoquic_print_connection_id_hexa(
        &raw mut cid_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 41]>() as size_t,
        &raw mut (*cnx).initial_cnxid,
    ) != 0 as ::core::ffi::c_int
    {
        ret = -(1 as ::core::ffi::c_int);
    }
    let mut log_filename: [::core::ffi::c_char; 512] = [0; 512];
    if ret == 0 as ::core::ffi::c_int {
        let mut sprintf_ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        if (*(*cnx).quic).use_unique_log_names() != 0 {
            sprintf_ret = picoquic_sprintf(
                &raw mut log_filename as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
                ::core::ptr::null_mut::<size_t>(),
                b"%s%s%s.%x.%s.log\0".as_ptr() as *const ::core::ffi::c_char,
                bin_dir,
                PICOQUIC_FILE_SEPARATOR.as_ptr(),
                &raw mut cid_name as *mut ::core::ffi::c_char,
                (*cnx).log_unique as ::core::ffi::c_int,
                if (*cnx).client_mode() as ::core::ffi::c_int != 0 {
                    b"client\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"server\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        } else {
            sprintf_ret = picoquic_sprintf(
                &raw mut log_filename as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 512]>() as size_t,
                ::core::ptr::null_mut::<size_t>(),
                b"%s%s%s.%s.log\0".as_ptr() as *const ::core::ffi::c_char,
                bin_dir,
                PICOQUIC_FILE_SEPARATOR.as_ptr(),
                &raw mut cid_name as *mut ::core::ffi::c_char,
                if (*cnx).client_mode() as ::core::ffi::c_int != 0 {
                    b"client\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"server\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        if sprintf_ret != 0 as ::core::ffi::c_int {
            ret = -(1 as ::core::ffi::c_int);
        } else {
            picoquic_string_free((*cnx).binlog_file_name);
            (*cnx).binlog_file_name =
                picoquic_string_duplicate(&raw mut log_filename as *mut ::core::ffi::c_char);
        }
    }
    if ret == 0 as ::core::ffi::c_int {
        (*cnx).f_binlog = create_binlog(
            &raw mut log_filename as *mut ::core::ffi::c_char,
            picoquic_get_quic_time((*cnx).quic as *mut picoquic_quic_t),
            (*cnx).local_parameters.is_multipath_enabled as ::core::ffi::c_uint,
        );
        if (*cnx).f_binlog.is_null() {
            (*cnx).binlog_file_name = picoquic_string_free((*cnx).binlog_file_name);
            ret = -(1 as ::core::ffi::c_int);
        } else {
            (*(*cnx).quic).current_number_of_open_logs =
                (*(*cnx).quic).current_number_of_open_logs.wrapping_add(1);
        }
    }
    if ret == 0 as ::core::ffi::c_int {
        let mut stream_msg: bytestream_buf = bytestream_buf {
            s: bytestream {
                data: ::core::ptr::null_mut::<uint8_t>(),
                size: 0,
                ptr: 0,
            },
            buf: [0; 2560],
        };
        let mut msg: *mut bytestream =
            bytestream_buf_init(&raw mut stream_msg, BYTESTREAM_MAX_BUFFER_SIZE as size_t);
        binlog_compose_event_header(
            msg,
            &raw mut (*cnx).initial_cnxid,
            (*cnx).start_time,
            0 as uint64_t,
            picoquic_log_event_new_connection,
        );
        bytewrite_int8(
            msg,
            ((*cnx).client_mode() as ::core::ffi::c_int != 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int as uint8_t,
        );
        bytewrite_int32(msg, (*cnx).proposed_version);
        bytewrite_cid(
            msg,
            &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid)
                .cnx_id,
        );
        bytewrite_cstr(msg, (*(*cnx).congestion_alg).congestion_algorithm_id);
        bytewrite_vint(msg, (*cnx).spin_policy as uint64_t);
        let mut stream_head: bytestream_buf = bytestream_buf {
            s: bytestream {
                data: ::core::ptr::null_mut::<uint8_t>(),
                size: 0,
                ptr: 0,
            },
            buf: [0; 2560],
        };
        let mut head: *mut bytestream = bytestream_buf_init(&raw mut stream_head, 8 as size_t);
        bytewrite_int32(head, bytestream_length(msg) as uint32_t);
        fwrite(
            bytestream_data(head) as *const ::core::ffi::c_void,
            bytestream_length(head),
            1 as size_t,
            (*cnx).f_binlog,
        );
        fwrite(
            bytestream_data(msg) as *const ::core::ffi::c_void,
            bytestream_length(msg),
            1 as size_t,
            (*cnx).f_binlog,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn binlog_close_connection(mut cnx: *mut picoquic_cnx_t) {
    let mut f: *mut FILE = (*cnx).f_binlog;
    if f.is_null() {
        return;
    }
    let mut stream_msg: bytestream_buf = bytestream_buf {
        s: bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        },
        buf: [0; 2560],
    };
    let mut msg: *mut bytestream =
        bytestream_buf_init(&raw mut stream_msg, BYTESTREAM_MAX_BUFFER_SIZE as size_t);
    binlog_compose_event_header(
        msg,
        &raw mut (*cnx).initial_cnxid,
        picoquic_get_quic_time((*cnx).quic as *mut picoquic_quic_t),
        0 as uint64_t,
        picoquic_log_event_connection_close,
    );
    let mut stream_head: bytestream_buf = bytestream_buf {
        s: bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        },
        buf: [0; 2560],
    };
    let mut head: *mut bytestream = bytestream_buf_init(&raw mut stream_head, 8 as size_t);
    bytewrite_int32(head, bytestream_length(msg) as uint32_t);
    fwrite(
        bytestream_data(head) as *const ::core::ffi::c_void,
        bytestream_length(head),
        1 as size_t,
        f,
    );
    fwrite(
        bytestream_data(msg) as *const ::core::ffi::c_void,
        bytestream_length(msg),
        1 as size_t,
        f,
    );
    fflush(f);
    (*cnx).f_binlog = picoquic_file_close((*cnx).f_binlog);
    if !(*(*cnx).quic).qlog_dir.is_null() && (*(*cnx).quic).autoqlog_fn.is_some() {
        (*(*cnx).quic)
            .autoqlog_fn
            .expect("non-null function pointer")(cnx as *mut picoquic_cnx_t);
    }
    (*cnx).binlog_file_name = picoquic_string_free((*cnx).binlog_file_name);
    if (*(*cnx).quic).current_number_of_open_logs > 0 as uint32_t {
        (*(*cnx).quic).current_number_of_open_logs =
            (*(*cnx).quic).current_number_of_open_logs.wrapping_sub(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn create_binlog(
    mut binlog_file: *const ::core::ffi::c_char,
    mut creation_time: uint64_t,
    mut is_multipath_supported: ::core::ffi::c_uint,
) -> *mut FILE {
    let mut f_binlog: *mut FILE =
        picoquic_file_open(binlog_file, b"wb\0".as_ptr() as *const ::core::ffi::c_char);
    if !f_binlog.is_null() {
        let mut stream: bytestream_buf = bytestream_buf {
            s: bytestream {
                data: ::core::ptr::null_mut::<uint8_t>(),
                size: 0,
                ptr: 0,
            },
            buf: [0; 2560],
        };
        let mut ps: *mut bytestream = bytestream_buf_init(&raw mut stream, 16 as size_t);
        bytewrite_int32(
            ps,
            ('g' as i32 as uint32_t) << 24 as ::core::ffi::c_int
                | (('o' as i32) << 16 as ::core::ffi::c_int) as uint32_t
                | (('l' as i32) << 8 as ::core::ffi::c_int) as uint32_t
                | 'q' as i32 as uint32_t,
        );
        bytewrite_int16(
            ps,
            (if is_multipath_supported != 0 {
                0x1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint16_t,
        );
        bytewrite_int16(ps, 0x1 as uint16_t);
        bytewrite_int64(ps, creation_time);
        if fwrite(
            bytestream_data(ps) as *const ::core::ffi::c_void,
            bytestream_length(ps),
            1 as size_t,
            f_binlog,
        ) <= 0 as ::core::ffi::c_ulong
        {
            f_binlog = picoquic_file_close(f_binlog);
        }
    }
    return f_binlog;
}
#[no_mangle]
pub unsafe extern "C" fn binlog_cc_dump(mut cnx: *mut picoquic_cnx_t, mut current_time: uint64_t) {
    if (*cnx).f_binlog.is_null() {
        return;
    }
    let mut stream_msg: bytestream_buf = bytestream_buf {
        s: bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        },
        buf: [0; 2560],
    };
    let mut ps_msg: *mut bytestream =
        bytestream_buf_init(&raw mut stream_msg, BYTESTREAM_MAX_BUFFER_SIZE as size_t);
    let mut path_max: ::core::ffi::c_int =
        if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0 {
            (*cnx).nb_paths
        } else {
            1 as ::core::ffi::c_int
        };
    let mut path_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while path_id < path_max {
        let mut path: *mut picoquic_path_t = *(*cnx).path.offset(path_id as isize);
        let mut pkt_ctx: *mut picoquic_packet_context_t = (&raw mut (*cnx).pkt_ctx
            as *mut picoquic_packet_context_t)
            .offset(picoquic_packet_context_application as ::core::ffi::c_int as isize)
            as *mut picoquic_packet_context_t;
        if (*cnx).is_multipath_enabled() != 0 {
            pkt_ctx = &raw mut (**(*cnx).path.offset(path_id as isize)).pkt_ctx;
        }
        if !((*path).is_cc_data_updated() == 0) {
            (*path).set_is_cc_data_updated(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            binlog_compose_event_header(
                ps_msg,
                &raw mut (*cnx).initial_cnxid,
                current_time,
                binlog_get_path_id(cnx, path),
                picoquic_log_event_cc_update,
            );
            bytewrite_vint(ps_msg, (*pkt_ctx).send_sequence);
            if (*pkt_ctx).highest_acknowledged != UINT64_MAX as uint64_t {
                bytewrite_vint(ps_msg, 1 as uint64_t);
                bytewrite_vint(ps_msg, (*pkt_ctx).highest_acknowledged);
                bytewrite_vint(
                    ps_msg,
                    (*pkt_ctx)
                        .highest_acknowledged_time
                        .wrapping_sub((*cnx).start_time),
                );
                bytewrite_vint(
                    ps_msg,
                    (*pkt_ctx)
                        .latest_time_acknowledged
                        .wrapping_sub((*cnx).start_time),
                );
            } else {
                bytewrite_vint(ps_msg, 0 as uint64_t);
            }
            bytewrite_vint(ps_msg, (*path).cwin);
            bytewrite_vint(ps_msg, (*path).one_way_delay_sample);
            bytewrite_vint(ps_msg, (*path).rtt_sample);
            bytewrite_vint(ps_msg, (*path).smoothed_rtt);
            bytewrite_vint(ps_msg, (*path).rtt_min);
            bytewrite_vint(ps_msg, (*path).bandwidth_estimate);
            bytewrite_vint(ps_msg, (*path).receive_rate_estimate);
            bytewrite_vint(ps_msg, (*path).send_mtu as uint64_t);
            bytewrite_vint(ps_msg, (*path).pacing.packet_time_microsec);
            if (*cnx).is_multipath_enabled() != 0 {
                bytewrite_vint(ps_msg, (*path).nb_losses_found);
                bytewrite_vint(ps_msg, (*path).nb_spurious);
            } else {
                bytewrite_vint(ps_msg, (*cnx).nb_retransmission_total);
                bytewrite_vint(ps_msg, (*cnx).nb_spurious);
            }
            bytewrite_vint(ps_msg, (*cnx).cwin_blocked() as uint64_t);
            bytewrite_vint(ps_msg, (*cnx).flow_blocked() as uint64_t);
            bytewrite_vint(ps_msg, (*cnx).stream_blocked() as uint64_t);
            if (*cnx).congestion_alg.is_null() {
                bytewrite_vint(ps_msg, 0 as uint64_t);
                bytewrite_vint(ps_msg, 0 as uint64_t);
            } else {
                let mut cc_state: uint64_t = 0 as uint64_t;
                let mut cc_param: uint64_t = 0 as uint64_t;
                if !(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                    .congestion_alg_state
                    .is_null()
                {
                    (*(*cnx).congestion_alg)
                        .alg_observe
                        .expect("non-null function pointer")(
                        *(*cnx).path.offset(0 as ::core::ffi::c_int as isize)
                            as *mut picoquic_path_t,
                        &raw mut cc_state,
                        &raw mut cc_param,
                    );
                }
                bytewrite_vint(ps_msg, cc_state);
                bytewrite_vint(ps_msg, cc_param);
            }
            bytewrite_vint(ps_msg, (*path).peak_bandwidth_estimate);
            bytewrite_vint(ps_msg, (*path).bytes_in_transit);
            bytewrite_vint(ps_msg, (*path).last_bw_estimate_path_limited() as uint64_t);
            let mut stream_head: bytestream_buf = bytestream_buf {
                s: bytestream {
                    data: ::core::ptr::null_mut::<uint8_t>(),
                    size: 0,
                    ptr: 0,
                },
                buf: [0; 2560],
            };
            let mut ps_head: *mut bytestream =
                bytestream_buf_init(&raw mut stream_head, BYTESTREAM_MAX_BUFFER_SIZE as size_t);
            bytewrite_int32(ps_head, bytestream_length(ps_msg) as uint32_t);
            fwrite(
                bytestream_data(ps_head) as *const ::core::ffi::c_void,
                bytestream_length(ps_head),
                1 as size_t,
                (*cnx).f_binlog,
            );
            fwrite(
                bytestream_data(ps_msg) as *const ::core::ffi::c_void,
                bytestream_length(ps_msg),
                1 as size_t,
                (*cnx).f_binlog,
            );
        }
        path_id += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_binlog_message_v(
    mut cnx: *mut picoquic_cnx_t,
    mut fmt: *const ::core::ffi::c_char,
    mut vargs: ::core::ffi::VaList,
) {
    if (*cnx).f_binlog.is_null() {
        return;
    }
    let mut stream_msg: bytestream_buf = bytestream_buf {
        s: bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        },
        buf: [0; 2560],
    };
    let mut ps_msg: *mut bytestream =
        bytestream_buf_init(&raw mut stream_msg, BYTESTREAM_MAX_BUFFER_SIZE as size_t);
    let mut message_len: size_t = 0;
    let mut message_text: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    let mut written: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    binlog_compose_event_header(
        ps_msg,
        &raw mut (*cnx).initial_cnxid,
        picoquic_get_quic_time((*cnx).quic as *mut picoquic_quic_t),
        0 as uint64_t,
        picoquic_log_event_info_message,
    );
    message_text = (*ps_msg).data.offset((*ps_msg).ptr as isize) as *mut ::core::ffi::c_char;
    written = vsnprintf(
        message_text,
        (*ps_msg).size.wrapping_sub((*ps_msg).ptr),
        fmt,
        vargs.as_va_list(),
    );
    if written < 0 as ::core::ffi::c_int
        || written as size_t >= (*ps_msg).size.wrapping_sub((*ps_msg).ptr)
    {
        message_len = (*ps_msg)
            .size
            .wrapping_sub((*ps_msg).ptr)
            .wrapping_sub(1 as size_t);
    } else {
        message_len = written as size_t;
    }
    (*ps_msg).ptr = (*ps_msg).ptr.wrapping_add(message_len);
    let mut stream_head: bytestream_buf = bytestream_buf {
        s: bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        },
        buf: [0; 2560],
    };
    let mut ps_head: *mut bytestream =
        bytestream_buf_init(&raw mut stream_head, BYTESTREAM_MAX_BUFFER_SIZE as size_t);
    bytewrite_int32(ps_head, bytestream_length(ps_msg) as uint32_t);
    fwrite(
        bytestream_data(ps_head) as *const ::core::ffi::c_void,
        bytestream_length(ps_head),
        1 as size_t,
        (*cnx).f_binlog,
    );
    fwrite(
        bytestream_data(ps_msg) as *const ::core::ffi::c_void,
        bytestream_length(ps_msg),
        1 as size_t,
        (*cnx).f_binlog,
    );
}
#[no_mangle]
pub unsafe extern "C" fn binlog_ignore_quic_app_message(
    mut quic: *mut picoquic_quic_t,
    mut cid: *const picoquic_connection_id_t,
    mut fmt: *const ::core::ffi::c_char,
    mut vargs: ::core::ffi::VaList,
) {
}
#[no_mangle]
pub unsafe extern "C" fn binlog_ignore_quic_pdu(
    mut quic: *mut picoquic_quic_t,
    mut receiving: ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut cid64: uint64_t,
    mut addr_peer: *const sockaddr,
    mut addr_local: *const sockaddr,
    mut packet_length: size_t,
) {
}
unsafe extern "C" fn binlog_app_message(
    mut cnx: *mut picoquic_cnx_t,
    mut fmt: *const ::core::ffi::c_char,
    mut vargs: ::core::ffi::VaList,
) {
    if !(*cnx).f_binlog.is_null() {
        picoquic_binlog_message_v(cnx, fmt, vargs.as_va_list());
    }
}
#[no_mangle]
pub unsafe extern "C" fn binlog_close(mut quic: *mut picoquic_quic_t) {}
#[no_mangle]
pub static mut binlog_functions: st_picoquic_unified_logging_t = unsafe {
    st_picoquic_unified_logging_t {
        log_quic_app_message: Some(
            binlog_ignore_quic_app_message
                as unsafe extern "C" fn(
                    *mut picoquic_quic_t,
                    *const picoquic_connection_id_t,
                    *const ::core::ffi::c_char,
                    ::core::ffi::VaList,
                ) -> (),
        ),
        log_quic_pdu: Some(
            binlog_ignore_quic_pdu
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
        log_quic_close: Some(binlog_close as unsafe extern "C" fn(*mut picoquic_quic_t) -> ()),
        log_app_message: Some(
            binlog_app_message
                as unsafe extern "C" fn(
                    *mut picoquic_cnx_t,
                    *const ::core::ffi::c_char,
                    ::core::ffi::VaList,
                ) -> (),
        ),
        log_pdu: Some(
            binlog_pdu_ex
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
            binlog_packet_ex
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
            binlog_dropped_packet
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
            binlog_buffered_packet
                as unsafe extern "C" fn(
                    *mut picoquic_cnx_t,
                    *mut picoquic_path_t,
                    picoquic_packet_type_enum,
                    uint64_t,
                ) -> (),
        ),
        log_outgoing_packet: Some(
            binlog_outgoing_packet
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
            binlog_packet_lost
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
            binlog_negotiated_alpn
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
            binlog_transport_extension
                as unsafe extern "C" fn(
                    *mut picoquic_cnx_t,
                    ::core::ffi::c_int,
                    size_t,
                    *mut uint8_t,
                ) -> (),
        ),
        log_picotls_ticket: Some(
            binlog_picotls_ticket_ex
                as unsafe extern "C" fn(*mut picoquic_cnx_t, *mut uint8_t, uint16_t) -> (),
        ),
        log_new_connection: Some(
            binlog_new_connection as unsafe extern "C" fn(*mut picoquic_cnx_t) -> (),
        ),
        log_close_connection: Some(
            binlog_close_connection as unsafe extern "C" fn(*mut picoquic_cnx_t) -> (),
        ),
        log_cc_dump: Some(
            binlog_cc_dump as unsafe extern "C" fn(*mut picoquic_cnx_t, uint64_t) -> (),
        ),
    }
};
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_binlog(
    mut quic: *mut picoquic_quic_t,
    mut binlog_dir: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    (*quic).binlog_dir = picoquic_string_free((*quic).binlog_dir);
    (*quic).binlog_dir = picoquic_string_duplicate(binlog_dir);
    (*quic).bin_log_fns = &raw mut binlog_functions;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_enable_binlog(mut quic: *mut picoquic_quic_t) {
    (*quic).bin_log_fns = &raw mut binlog_functions;
}

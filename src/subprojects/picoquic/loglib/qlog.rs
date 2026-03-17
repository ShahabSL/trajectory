use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type st_ptls_iovec_t;
    pub type st_ptls_buffer_t;
    pub type st_picoquic_unified_logging_t;
    pub type st_ptls_verify_certificate_t;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn fwrite(
        __ptr: *const ::core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __s: *mut FILE,
    ) -> ::core::ffi::c_ulong;
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
    fn picoquic_print_connection_id_hexa(
        buf: *mut ::core::ffi::c_char,
        buf_len: size_t,
        cnxid: *const picoquic_connection_id_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_compare_addr(
        expected: *const sockaddr,
        actual: *const sockaddr,
    ) -> ::core::ffi::c_int;
    fn picoquic_store_addr(stored_addr: *mut sockaddr_storage, addr: *const sockaddr);
    fn picoquic_file_open(
        file_name: *const ::core::ffi::c_char,
        flags: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn picoquic_file_close(F: *mut FILE) -> *mut FILE;
    fn bytestream_ref_init(
        s: *mut bytestream,
        bytes: *const ::core::ffi::c_void,
        nb_bytes: size_t,
    ) -> *mut bytestream;
    fn bytestream_ptr(s: *mut bytestream) -> *const uint8_t;
    fn bytestream_remain(s: *mut bytestream) -> size_t;
    fn byteread_int8(s: *mut bytestream, value: *mut uint8_t) -> ::core::ffi::c_int;
    fn byteread_int16(s: *mut bytestream, value: *mut uint16_t) -> ::core::ffi::c_int;
    fn byteread_vint(s: *mut bytestream, value: *mut uint64_t) -> ::core::ffi::c_int;
    fn byteread_addr(s: *mut bytestream, addr: *mut sockaddr_storage) -> ::core::ffi::c_int;
    fn binlog_convert(
        f_binlog: *mut FILE,
        cid: *const picoquic_connection_id_t,
        callbacks: *mut binlog_convert_cb_t,
    ) -> ::core::ffi::c_int;
    fn open_outfile(
        cid_name: *const ::core::ffi::c_char,
        binlog_name: *const ::core::ffi::c_char,
        out_dir: *const ::core::ffi::c_char,
        out_ext: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn ptype2str(ptype: picoquic_packet_type_enum) -> *const ::core::ffi::c_char;
    fn ftype2str(ftype: picoquic_frame_type_enum_t) -> *const ::core::ffi::c_char;
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
pub type picoquic_epoch_enum = ::core::ffi::c_uint;
pub const picoquic_epoch_1rtt: picoquic_epoch_enum = 3;
pub const picoquic_epoch_handshake: picoquic_epoch_enum = 2;
pub const picoquic_epoch_0rtt: picoquic_epoch_enum = 1;
pub const picoquic_epoch_initial: picoquic_epoch_enum = 0;
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
pub type picoquic_packet_header = st_picoquic_packet_header_t;
pub type picoquic_tp_enum = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bytestream {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub ptr: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct binlog_convert_cb_st {
    pub connection_start: Option<
        unsafe extern "C" fn(
            uint64_t,
            *const picoquic_connection_id_t,
            ::core::ffi::c_int,
            uint32_t,
            *const picoquic_connection_id_t,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub alpn_update: Option<
        unsafe extern "C" fn(
            uint64_t,
            *mut bytestream,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub param_update: Option<
        unsafe extern "C" fn(
            uint64_t,
            *mut bytestream,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub pdu: Option<
        unsafe extern "C" fn(
            uint64_t,
            ::core::ffi::c_int,
            *mut bytestream,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub packet_start: Option<
        unsafe extern "C" fn(
            uint64_t,
            uint64_t,
            uint64_t,
            *const picoquic_packet_header,
            ::core::ffi::c_int,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub packet_frame: Option<
        unsafe extern "C" fn(*mut bytestream, *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
    >,
    pub packet_end: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub packet_lost: Option<
        unsafe extern "C" fn(
            uint64_t,
            uint64_t,
            *mut bytestream,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub packet_dropped: Option<
        unsafe extern "C" fn(
            uint64_t,
            uint64_t,
            *mut bytestream,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub packet_buffered: Option<
        unsafe extern "C" fn(
            uint64_t,
            uint64_t,
            *mut bytestream,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub cc_update: Option<
        unsafe extern "C" fn(
            uint64_t,
            uint64_t,
            *mut bytestream,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub info_message: Option<
        unsafe extern "C" fn(
            uint64_t,
            *mut bytestream,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub connection_end:
        Option<unsafe extern "C" fn(uint64_t, *mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
    pub ptr: *mut ::core::ffi::c_void,
}
pub type binlog_convert_cb_t = binlog_convert_cb_st;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct qlog_context_st {
    pub f_txtlog: *mut FILE,
    pub version_number: uint32_t,
    pub cid_name: *const ::core::ffi::c_char,
    pub addr_peer: sockaddr_storage,
    pub addr_local: sockaddr_storage,
    pub start_time: uint64_t,
    pub event_count: ::core::ffi::c_int,
    pub packet_count: ::core::ffi::c_int,
    pub frame_count: ::core::ffi::c_int,
    pub packet_type: picoquic_packet_type_enum,
    pub cwin: uint64_t,
    pub rtt_sample: uint64_t,
    pub SRTT: uint64_t,
    pub RTT_min: uint64_t,
    pub bytes_in_transit: uint64_t,
    pub pacing_packet_time: uint64_t,
    #[bitfield(name = "trace_flow_id", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(
        name = "key_phase_sent_last",
        ty = "::core::ffi::c_uint",
        bits = "1..=1"
    )]
    #[bitfield(name = "key_phase_sent", ty = "::core::ffi::c_uint", bits = "2..=2")]
    #[bitfield(
        name = "key_phase_received_last",
        ty = "::core::ffi::c_uint",
        bits = "3..=3"
    )]
    #[bitfield(
        name = "key_phase_received",
        ty = "::core::ffi::c_uint",
        bits = "4..=4"
    )]
    #[bitfield(
        name = "spin_bit_sent_last",
        ty = "::core::ffi::c_uint",
        bits = "5..=5"
    )]
    #[bitfield(name = "spin_bit_sent", ty = "::core::ffi::c_uint", bits = "6..=6")]
    #[bitfield(name = "app_limited", ty = "::core::ffi::c_uint", bits = "7..=7")]
    pub trace_flow_id_key_phase_sent_last_key_phase_sent_key_phase_received_last_key_phase_received_spin_bit_sent_last_spin_bit_sent_app_limited:
        [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub state: ::core::ffi::c_int,
}
pub type qlog_context_t = qlog_context_st;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const PICOQUIC_ERROR_CLASS: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_DUPLICATE: uint64_t = 1025 as uint64_t;
pub const PICOQUIC_ERROR_AEAD_CHECK: uint64_t = 1027 as uint64_t;
pub const PICOQUIC_ERROR_CNXID_CHECK: uint64_t = 1031 as uint64_t;
pub const PICOQUIC_ERROR_INITIAL_TOO_SHORT: uint64_t = 1032 as uint64_t;
pub const PICOQUIC_ERROR_STATELESS_RESET: uint64_t = 1054 as uint64_t;
pub const PICOQUIC_ERROR_CNXID_NOT_AVAILABLE: uint64_t = 1057 as uint64_t;
pub const PICOQUIC_ERROR_KEY_ROTATION_NOT_READY: uint64_t = 1064 as uint64_t;
pub const PICOQUIC_ERROR_AEAD_NOT_READY: uint64_t = 1065 as uint64_t;
pub const PICOQUIC_INITIAL_RTT: ::core::ffi::c_ulonglong = 250000 as ::core::ffi::c_ulonglong;
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
pub const picoquic_tp_enable_loss_bit: picoquic_tp_enum = 4183 as picoquic_tp_enum;
pub const picoquic_tp_min_ack_delay: picoquic_tp_enum = 4278509083 as picoquic_tp_enum;
pub const picoquic_tp_enable_time_stamp: picoquic_tp_enum = 29016 as picoquic_tp_enum;
pub const picoquic_tp_grease_quic_bit: picoquic_tp_enum = 10930 as picoquic_tp_enum;
pub const picoquic_tp_version_negotiation: picoquic_tp_enum = 17 as picoquic_tp_enum;
pub const picoquic_tp_enable_bdp_frame: picoquic_tp_enum = 60377 as picoquic_tp_enum;
pub const picoquic_tp_initial_max_path_id: picoquic_tp_enum =
    1113404765106498833 as picoquic_tp_enum;
pub const picoquic_tp_address_discovery: picoquic_tp_enum = 2676072822 as picoquic_tp_enum;
#[no_mangle]
pub unsafe extern "C" fn qlog_string(
    mut f: *mut FILE,
    mut s: *mut bytestream,
    mut l: uint64_t,
) -> ::core::ffi::c_int {
    let mut x: uint64_t = 0;
    let mut error_found: ::core::ffi::c_int =
        ((*s).ptr.wrapping_add(l as size_t) > (*s).size) as ::core::ffi::c_int;
    fprintf(f, b"\"\0".as_ptr() as *const ::core::ffi::c_char);
    x = 0 as uint64_t;
    while x < l && (*s).ptr < (*s).size {
        let c2rust_fresh0 = (*s).ptr;
        (*s).ptr = (*s).ptr.wrapping_add(1);
        fprintf(
            f,
            b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
            *(*s).data.offset(c2rust_fresh0 as isize) as ::core::ffi::c_int,
        );
        x = x.wrapping_add(1);
    }
    if error_found != 0 {
        fprintf(
            f,
            b"... coding error!\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    fprintf(f, b"\"\0".as_ptr() as *const ::core::ffi::c_char);
    return if error_found != 0 {
        -(1 as ::core::ffi::c_int)
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn qlog_chars(
    mut f: *mut FILE,
    mut s: *mut bytestream,
    mut l: uint64_t,
) -> ::core::ffi::c_int {
    let mut x: uint64_t = 0;
    let mut error_found: ::core::ffi::c_int =
        ((*s).ptr.wrapping_add(l as size_t) > (*s).size) as ::core::ffi::c_int;
    fprintf(f, b"\"\0".as_ptr() as *const ::core::ffi::c_char);
    x = 0 as uint64_t;
    while x < l && (*s).ptr < (*s).size {
        let c2rust_fresh1 = (*s).ptr;
        (*s).ptr = (*s).ptr.wrapping_add(1);
        let mut c: ::core::ffi::c_int =
            *(*s).data.offset(c2rust_fresh1 as isize) as ::core::ffi::c_int;
        if c == '"' as i32 || c == '\\' as i32 {
            fprintf(f, b"\\%c\0".as_ptr() as *const ::core::ffi::c_char, c);
        } else if c >= ' ' as i32 && c < 127 as ::core::ffi::c_int {
            fprintf(f, b"%c\0".as_ptr() as *const ::core::ffi::c_char, c);
        } else {
            fprintf(f, b"\\%02x\0".as_ptr() as *const ::core::ffi::c_char, c);
        }
        x = x.wrapping_add(1);
    }
    if error_found != 0 {
        fprintf(
            f,
            b"... coding error!\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    fprintf(f, b"\"\0".as_ptr() as *const ::core::ffi::c_char);
    return if error_found != 0 {
        -(1 as ::core::ffi::c_int)
    } else {
        0 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn qlog_log_addr(mut f: *mut FILE, mut addr_peer: *mut sockaddr) {
    if (*addr_peer).sa_family as ::core::ffi::c_int == AF_INET {
        let mut s4: *mut sockaddr_in = addr_peer as *mut sockaddr_in;
        let mut addr: *mut uint8_t = &raw mut (*s4).sin_addr as *mut uint8_t;
        fprintf(
            f,
            b"\"ip_v4\": \"%d.%d.%d.%d\", \"port_v4\":%d\0".as_ptr() as *const ::core::ffi::c_char,
            *addr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *addr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *addr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            *addr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            __bswap_16((*s4).sin_port as __uint16_t) as ::core::ffi::c_int,
        );
    } else {
        let mut s6: *mut sockaddr_in6 = addr_peer as *mut sockaddr_in6;
        let mut addr_0: *mut uint8_t = &raw mut (*s6).sin6_addr as *mut uint8_t;
        fprintf(
            f,
            b" \"ip_v6\": \"\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int {
            if i != 0 as ::core::ffi::c_int {
                fprintf(f, b":\0".as_ptr() as *const ::core::ffi::c_char);
            }
            if *addr_0.offset((2 as ::core::ffi::c_int * i) as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            {
                fprintf(
                    f,
                    b"%x%02x\0".as_ptr() as *const ::core::ffi::c_char,
                    *addr_0.offset((2 as ::core::ffi::c_int * i) as isize) as ::core::ffi::c_int,
                    *addr_0.offset((2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int,
                );
            } else {
                fprintf(
                    f,
                    b"%x\0".as_ptr() as *const ::core::ffi::c_char,
                    *addr_0.offset((2 as ::core::ffi::c_int * i + 1 as ::core::ffi::c_int) as isize)
                        as ::core::ffi::c_int,
                );
            }
            i += 1;
        }
        fprintf(
            f,
            b"\", \"port_v6\" :%d\0".as_ptr() as *const ::core::ffi::c_char,
            __bswap_16((*s6).sin6_port as __uint16_t) as ::core::ffi::c_int,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn qlog_event_header(
    mut f: *mut FILE,
    mut ctx: *mut qlog_context_t,
    mut delta_time: int64_t,
    mut path_id: uint64_t,
    mut event_class: *const ::core::ffi::c_char,
    mut event_name: *const ::core::ffi::c_char,
) {
    fprintf(
        f,
        b"[%ld, \0".as_ptr() as *const ::core::ffi::c_char,
        delta_time,
    );
    if (*ctx).trace_flow_id() != 0 {
        fprintf(
            f,
            b"%ld, \0".as_ptr() as *const ::core::ffi::c_char,
            path_id,
        );
    }
    fprintf(
        f,
        b"\"%s\", \"%s\", {\0".as_ptr() as *const ::core::ffi::c_char,
        event_class,
        event_name,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_vint_transport_extension(
    mut f: *mut FILE,
    mut ext_name: *const ::core::ffi::c_char,
    mut s: *mut bytestream,
    mut len: uint64_t,
) {
    let mut val: uint64_t = 0;
    let mut current_ptr: size_t = (*s).ptr;
    let mut ret: ::core::ffi::c_int = byteread_vint(s, &raw mut val);
    fprintf(
        f,
        b"\"%s\" : \0".as_ptr() as *const ::core::ffi::c_char,
        ext_name,
    );
    if ret != 0 as ::core::ffi::c_int || current_ptr.wrapping_add(len as size_t) != (*s).ptr {
        (*s).ptr = current_ptr;
        qlog_string(f, s, len);
    } else {
        fprintf(f, b"%lu\0".as_ptr() as *const ::core::ffi::c_char, val);
    };
}
#[no_mangle]
pub unsafe extern "C" fn qlog_boolean_transport_extension(
    mut f: *mut FILE,
    mut ext_name: *const ::core::ffi::c_char,
    mut s: *mut bytestream,
    mut len: uint64_t,
) {
    fprintf(
        f,
        b"\"%s\" : \0".as_ptr() as *const ::core::ffi::c_char,
        ext_name,
    );
    if len != 0 as uint64_t {
        qlog_string(f, s, len);
    } else {
        fprintf(f, b"\"\"\0".as_ptr() as *const ::core::ffi::c_char);
    };
}
#[no_mangle]
pub unsafe extern "C" fn qlog_preferred_address(
    mut f: *mut FILE,
    mut s: *mut bytestream,
    mut len: uint64_t,
) {
    let mut port4: uint16_t = 0 as uint16_t;
    let mut port6: uint16_t = 0 as uint16_t;
    let mut cid_len: uint8_t = 0;
    let mut old_size: size_t = (*s).size;
    (*s).size = (*s).ptr.wrapping_add(len as size_t);
    fprintf(f, b"\"ip_v4\": \"\0".as_ptr() as *const ::core::ffi::c_char);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int && (*s).ptr < (*s).size {
        fprintf(
            f,
            b"%s%d\0".as_ptr() as *const ::core::ffi::c_char,
            if i == 0 as ::core::ffi::c_int {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b".\0".as_ptr() as *const ::core::ffi::c_char
            },
            *(*s).data.offset((*s).ptr as isize) as ::core::ffi::c_int,
        );
        i += 1;
        (*s).ptr = (*s).ptr.wrapping_add(1);
    }
    byteread_int16(s, &raw mut port4);
    fprintf(
        f,
        b"\", \"port_v4\":%d\0".as_ptr() as *const ::core::ffi::c_char,
        port4 as ::core::ffi::c_int,
    );
    fprintf(
        f,
        b", \"ip_v6\": \"\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 8 as ::core::ffi::c_int {
        let mut chunk: uint16_t = 0 as uint16_t;
        byteread_int16(s, &raw mut chunk);
        fprintf(
            f,
            b"%s%x\0".as_ptr() as *const ::core::ffi::c_char,
            if i_0 == 0 as ::core::ffi::c_int {
                b"\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b":\0".as_ptr() as *const ::core::ffi::c_char
            },
            chunk as ::core::ffi::c_int,
        );
        i_0 += 1;
    }
    byteread_int16(s, &raw mut port6);
    fprintf(
        f,
        b"\", \"port_v6\" : %d\0".as_ptr() as *const ::core::ffi::c_char,
        port6 as ::core::ffi::c_int,
    );
    byteread_int8(s, &raw mut cid_len);
    fprintf(
        f,
        b", \"connection_id\": \0".as_ptr() as *const ::core::ffi::c_char,
    );
    qlog_string(f, s, cid_len as uint64_t);
    fprintf(
        f,
        b", \"stateless_reset_token\": \0".as_ptr() as *const ::core::ffi::c_char,
    );
    qlog_string(f, s, 16 as uint64_t);
    if (*s).ptr < (*s).size {
        fprintf(
            f,
            b"\", \"extra_bytes\": \0".as_ptr() as *const ::core::ffi::c_char,
        );
        qlog_string(f, s, bytestream_remain(s) as uint64_t);
    }
    (*s).size = old_size;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_tp_version_negotiation(
    mut f: *mut FILE,
    mut s: *mut bytestream,
    mut len: uint64_t,
) {
    let mut old_size: size_t = (*s).size;
    (*s).size = (*s).ptr.wrapping_add(len as size_t);
    fprintf(f, b"{ \0".as_ptr() as *const ::core::ffi::c_char);
    if len & 3 as uint64_t != 0 as uint64_t || len == 0 as uint64_t {
        fprintf(
            f,
            b"\"bad_length\": \"%lu\0".as_ptr() as *const ::core::ffi::c_char,
            len,
        );
    } else {
        fprintf(
            f,
            b"\"chosen\": \"\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 4 as ::core::ffi::c_int && (*s).ptr < (*s).size {
            fprintf(
                f,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *(*s).data.offset((*s).ptr as isize) as ::core::ffi::c_int,
            );
            i += 1;
            (*s).ptr = (*s).ptr.wrapping_add(1);
        }
        fprintf(f, b"\"\0".as_ptr() as *const ::core::ffi::c_char);
        if (*s).ptr < (*s).size {
            let mut is_first: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            fprintf(
                f,
                b", \"others\": [\0".as_ptr() as *const ::core::ffi::c_char,
            );
            loop {
                fprintf(
                    f,
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    if is_first != 0 {
                        b"\"\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b", \"\0".as_ptr() as *const ::core::ffi::c_char
                    },
                );
                is_first = 0 as ::core::ffi::c_int;
                let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i_0 < 4 as ::core::ffi::c_int && (*s).ptr < (*s).size {
                    fprintf(
                        f,
                        b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                        *(*s).data.offset((*s).ptr as isize) as ::core::ffi::c_int,
                    );
                    i_0 += 1;
                    (*s).ptr = (*s).ptr.wrapping_add(1);
                }
                fprintf(f, b"\"\0".as_ptr() as *const ::core::ffi::c_char);
                if !((*s).ptr < (*s).size) {
                    break;
                }
            }
            fprintf(f, b"]\0".as_ptr() as *const ::core::ffi::c_char);
        }
    }
    fprintf(f, b"}\0".as_ptr() as *const ::core::ffi::c_char);
    (*s).size = old_size;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_transport_extensions(
    mut f: *mut FILE,
    mut s: *mut bytestream,
    mut tp_length: size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ptr_max: size_t = (*s).ptr.wrapping_add(tp_length);
    if ptr_max < (*s).size {
        fprintf(
            f,
            b",\n    \"transport_parameter_length\": %zu\0".as_ptr() as *const ::core::ffi::c_char,
            tp_length,
        );
        fprintf(
            f,
            b",\n    \"bytes_available\": %zulu\0".as_ptr() as *const ::core::ffi::c_char,
            (*s).size.wrapping_sub((*s).ptr),
        );
    } else {
        while ret == 0 as ::core::ffi::c_int && (*s).ptr < ptr_max {
            let mut extension_type: uint64_t = UINT64_MAX as uint64_t;
            let mut extension_length: uint64_t = 0 as uint64_t;
            let mut current_ptr: size_t = (*s).ptr;
            ret |= byteread_vint(s, &raw mut extension_type);
            ret |= byteread_vint(s, &raw mut extension_length);
            fprintf(f, b",\n    \0".as_ptr() as *const ::core::ffi::c_char);
            if ret != 0 as ::core::ffi::c_int || bytestream_remain(s) < extension_length as size_t {
                let mut len: size_t = bytestream_remain(s);
                ret = -(1 as ::core::ffi::c_int);
                (*s).ptr = current_ptr;
                fprintf(
                    f,
                    b"\"Parameter_coding_error\": \0".as_ptr() as *const ::core::ffi::c_char,
                );
                qlog_string(f, s, len as uint64_t);
                break;
            } else {
                match extension_type {
                    5 => {
                        qlog_vint_transport_extension(
                            f,
                            b"initial_max_stream_data_bidi_local\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    6 => {
                        qlog_vint_transport_extension(
                            f,
                            b"initial_max_stream_data_bidi_remote\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    7 => {
                        qlog_vint_transport_extension(
                            f,
                            b"initial_max_stream_data_uni\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    4 => {
                        qlog_vint_transport_extension(
                            f,
                            b"initial_max_data\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    8 => {
                        qlog_vint_transport_extension(
                            f,
                            b"initial_max_streams_bidi\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    1 => {
                        qlog_vint_transport_extension(
                            f,
                            b"idle_timeout\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    3 => {
                        qlog_vint_transport_extension(
                            f,
                            b"max_packet_size\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    2 => {
                        fprintf(
                            f,
                            b"\"stateless_reset_token\": \0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        qlog_string(f, s, extension_length);
                    }
                    10 => {
                        qlog_vint_transport_extension(
                            f,
                            b"ack_delay_exponent\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    9 => {
                        qlog_vint_transport_extension(
                            f,
                            b"initial_max_streams_uni\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    13 => {
                        fprintf(
                            f,
                            b"\"server_preferred_address\": {\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        qlog_preferred_address(f, s, extension_length);
                        fprintf(f, b"}\0".as_ptr() as *const ::core::ffi::c_char);
                    }
                    12 => {
                        qlog_boolean_transport_extension(
                            f,
                            b"disable_migration\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    11 => {
                        qlog_vint_transport_extension(
                            f,
                            b"max_ack_delay\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    0 => {
                        fprintf(
                            f,
                            b"\"original_connection_id\": \0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        qlog_string(f, s, extension_length);
                    }
                    16 => {
                        fprintf(
                            f,
                            b"\"retry_connection_id\": \0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        qlog_string(f, s, extension_length);
                    }
                    15 => {
                        fprintf(
                            f,
                            b"\"handshake_connection_id\": \0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        qlog_string(f, s, extension_length);
                    }
                    14 => {
                        qlog_vint_transport_extension(
                            f,
                            b"active_connection_id_limit\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    32 => {
                        qlog_vint_transport_extension(
                            f,
                            b"max_datagram_frame_size\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    4183 => {
                        qlog_vint_transport_extension(
                            f,
                            b"enable_loss_bit\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    4278509083 => {
                        qlog_vint_transport_extension(
                            f,
                            b"min_ack_delay\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    29016 => {
                        qlog_boolean_transport_extension(
                            f,
                            b"enable_time_stamp\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    10930 => {
                        qlog_boolean_transport_extension(
                            f,
                            b"grease_quic_bit\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    17 => {
                        fprintf(
                            f,
                            b"\"version_negotiation\": \0".as_ptr() as *const ::core::ffi::c_char,
                        );
                        qlog_tp_version_negotiation(f, s, extension_length);
                    }
                    60377 => {
                        qlog_vint_transport_extension(
                            f,
                            b"enable_bdp_frame\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    1113404765106498833 => {
                        qlog_vint_transport_extension(
                            f,
                            b"initial_max_path_id\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    2676072822 => {
                        qlog_vint_transport_extension(
                            f,
                            b"address_discovery\0".as_ptr() as *const ::core::ffi::c_char,
                            s,
                            extension_length,
                        );
                    }
                    _ => {
                        fprintf(
                            f,
                            b"\"%lx\": \0".as_ptr() as *const ::core::ffi::c_char,
                            extension_type,
                        );
                        qlog_string(f, s, extension_length);
                    }
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_alpn_update(
    mut time: uint64_t,
    mut s: *mut bytestream,
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut qlog_context_t = ptr as *mut qlog_context_t;
    let mut delta_time: int64_t = time.wrapping_sub((*ctx).start_time) as int64_t;
    let mut f: *mut FILE = (*ctx).f_txtlog;
    let mut owner: uint64_t = 0 as uint64_t;
    let mut sni_length: uint64_t = 0 as uint64_t;
    let mut alpn_length: uint64_t = 0 as uint64_t;
    let mut alpn_count: uint64_t = 0 as uint64_t;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ret |= byteread_vint(s, &raw mut owner);
    if (*ctx).event_count != 0 as ::core::ffi::c_int {
        fprintf(f, b",\n\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        fprintf(f, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
    ret |= byteread_vint(s, &raw mut sni_length);
    qlog_event_header(
        f,
        ctx,
        delta_time,
        0 as uint64_t,
        b"transport\0".as_ptr() as *const ::core::ffi::c_char,
        b"parameters_set\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        f,
        b"\n    \"owner\": \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
        if owner != 0 {
            b"local\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"remote\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    if sni_length > 0 as uint64_t {
        fprintf(
            f,
            b",\n    \"sni\": \0".as_ptr() as *const ::core::ffi::c_char,
        );
        ret |= qlog_chars(f, s, sni_length);
    }
    ret |= byteread_vint(s, &raw mut alpn_count);
    if ret == 0 as ::core::ffi::c_int && alpn_count > 0 as uint64_t {
        fprintf(
            f,
            b",\n    \"proposed_alpn\": [\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut i: size_t = 0 as size_t;
        while i < alpn_count as size_t {
            let mut len: uint64_t = 0;
            if i != 0 as size_t {
                fprintf(f, b", \0".as_ptr() as *const ::core::ffi::c_char);
            }
            ret |= byteread_vint(s, &raw mut len);
            ret |= qlog_chars(f, s, len);
            i = i.wrapping_add(1);
        }
        fprintf(f, b"]\0".as_ptr() as *const ::core::ffi::c_char);
    }
    ret |= byteread_vint(s, &raw mut alpn_length);
    if ret == 0 as ::core::ffi::c_int && alpn_length > 0 as uint64_t {
        fprintf(
            f,
            b",\n    \"alpn\": \0".as_ptr() as *const ::core::ffi::c_char,
        );
        qlog_chars(f, s, alpn_length);
    }
    fprintf(f, b"}]\0".as_ptr() as *const ::core::ffi::c_char);
    (*ctx).event_count += 1;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_param_update(
    mut time: uint64_t,
    mut s: *mut bytestream,
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut qlog_context_t = ptr as *mut qlog_context_t;
    let mut delta_time: int64_t = time.wrapping_sub((*ctx).start_time) as int64_t;
    let mut f: *mut FILE = (*ctx).f_txtlog;
    let mut owner: uint64_t = 0 as uint64_t;
    let mut tp_length: uint64_t = 0 as uint64_t;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ret |= byteread_vint(s, &raw mut owner);
    if (*ctx).event_count != 0 as ::core::ffi::c_int {
        fprintf(f, b",\n\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        fprintf(f, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
    qlog_event_header(
        f,
        ctx,
        delta_time,
        0 as uint64_t,
        b"transport\0".as_ptr() as *const ::core::ffi::c_char,
        b"parameters_set\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        f,
        b"\n    \"owner\": \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
        if owner != 0 {
            b"local\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"remote\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    ret |= byteread_vint(s, &raw mut tp_length);
    if ret == 0 as ::core::ffi::c_int && tp_length > 0 as uint64_t {
        qlog_transport_extensions(f, s, tp_length as size_t);
    }
    fprintf(f, b"}]\0".as_ptr() as *const ::core::ffi::c_char);
    (*ctx).event_count += 1;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_packet_lost(
    mut time: uint64_t,
    mut path_id: uint64_t,
    mut s: *mut bytestream,
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut qlog_context_t = ptr as *mut qlog_context_t;
    let mut delta_time: int64_t = time.wrapping_sub((*ctx).start_time) as int64_t;
    let mut f: *mut FILE = (*ctx).f_txtlog;
    let mut packet_type: uint64_t = 0 as uint64_t;
    let mut sequence: uint64_t = 0 as uint64_t;
    let mut trigger_length: uint64_t = 0;
    let mut packet_size: uint64_t = 0 as uint64_t;
    let mut cid_len: uint8_t = 0 as uint8_t;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ret |= byteread_vint(s, &raw mut packet_type);
    ret |= byteread_vint(s, &raw mut sequence);
    ret |= byteread_vint(s, &raw mut trigger_length);
    if (*ctx).event_count != 0 as ::core::ffi::c_int {
        fprintf(f, b",\n\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        fprintf(f, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
    qlog_event_header(
        f,
        ctx,
        delta_time,
        path_id,
        b"recovery\0".as_ptr() as *const ::core::ffi::c_char,
        b"packet_lost\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        f,
        b"\n    \"packet_type\" : \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
        ptype2str(packet_type as picoquic_packet_type_enum),
    );
    fprintf(
        f,
        b",\n    \"packet_number\" : %lu\0".as_ptr() as *const ::core::ffi::c_char,
        sequence,
    );
    if trigger_length > 0 as uint64_t {
        fprintf(
            f,
            b",\n    \"trigger\": \0".as_ptr() as *const ::core::ffi::c_char,
        );
        ret |= qlog_chars(f, s, trigger_length);
    }
    fprintf(
        f,
        b",\n    \"header\": {\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        f,
        b"\n        \"packet_type\" : \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
        ptype2str(packet_type as picoquic_packet_type_enum),
    );
    fprintf(
        f,
        b",\n        \"packet_number\" : %lu\0".as_ptr() as *const ::core::ffi::c_char,
        sequence,
    );
    ret |= byteread_int8(s, &raw mut cid_len);
    if ret == 0 as ::core::ffi::c_int && cid_len as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        fprintf(
            f,
            b",\n        \"dcid\" : \0".as_ptr() as *const ::core::ffi::c_char,
        );
        qlog_string(f, s, cid_len as uint64_t);
    }
    ret |= byteread_vint(s, &raw mut packet_size);
    if ret == 0 as ::core::ffi::c_int {
        fprintf(
            f,
            b",\n        \"packet_size\" : %lu\0".as_ptr() as *const ::core::ffi::c_char,
            packet_size,
        );
    }
    fprintf(f, b"}}]\0".as_ptr() as *const ::core::ffi::c_char);
    (*ctx).event_count += 1;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_packet_dropped(
    mut time: uint64_t,
    mut path_id: uint64_t,
    mut s: *mut bytestream,
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut qlog_context_t = ptr as *mut qlog_context_t;
    let mut delta_time: int64_t = time.wrapping_sub((*ctx).start_time) as int64_t;
    let mut f: *mut FILE = (*ctx).f_txtlog;
    let mut packet_type: uint64_t = 0 as uint64_t;
    let mut err_code: uint64_t = 0;
    let mut packet_size: uint64_t = 0 as uint64_t;
    let mut raw_len: uint64_t = 0 as uint64_t;
    let mut str: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ret |= byteread_vint(s, &raw mut packet_type);
    ret |= byteread_vint(s, &raw mut packet_size);
    ret |= byteread_vint(s, &raw mut err_code);
    ret |= byteread_vint(s, &raw mut raw_len);
    if (*ctx).event_count != 0 as ::core::ffi::c_int {
        fprintf(f, b",\n\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        fprintf(f, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
    qlog_event_header(
        f,
        ctx,
        delta_time,
        path_id,
        b"transport\0".as_ptr() as *const ::core::ffi::c_char,
        b"packet_dropped\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        f,
        b"\n    \"packet_type\" : \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
        ptype2str(packet_type as picoquic_packet_type_enum),
    );
    fprintf(
        f,
        b",\n    \"packet_size\" : %lu\0".as_ptr() as *const ::core::ffi::c_char,
        packet_size,
    );
    match err_code {
        1025 => {
            str = b"dos_prevention\0".as_ptr() as *const ::core::ffi::c_char;
        }
        1027 => {
            str = b"payload_decrypt_error\0".as_ptr() as *const ::core::ffi::c_char;
        }
        1031 => {
            str = b"unknown_connection_id\0".as_ptr() as *const ::core::ffi::c_char;
        }
        1032 => {
            str = b"dos_prevention\0".as_ptr() as *const ::core::ffi::c_char;
        }
        1057 => {
            str = b"unknown_connection_id\0".as_ptr() as *const ::core::ffi::c_char;
        }
        1064 => {
            str = b"key_unavailable\0".as_ptr() as *const ::core::ffi::c_char;
        }
        1065 => {
            str = b"key_unavailable\0".as_ptr() as *const ::core::ffi::c_char;
        }
        1054 => {
            str = b"stateless_reset\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => {
            str = b"protocol_violation\0".as_ptr() as *const ::core::ffi::c_char;
        }
    }
    fprintf(
        f,
        b",\n    \"trigger\": \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
        str,
    );
    if ret == 0 as ::core::ffi::c_int && raw_len > 0 as uint64_t {
        fprintf(
            f,
            b",\n    \"raw\": \0".as_ptr() as *const ::core::ffi::c_char,
        );
        qlog_string(f, s, raw_len);
    }
    fprintf(f, b"}]\0".as_ptr() as *const ::core::ffi::c_char);
    (*ctx).event_count += 1;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_packet_buffered(
    mut time: uint64_t,
    mut path_id: uint64_t,
    mut s: *mut bytestream,
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut qlog_context_t = ptr as *mut qlog_context_t;
    let mut delta_time: int64_t = time.wrapping_sub((*ctx).start_time) as int64_t;
    let mut f: *mut FILE = (*ctx).f_txtlog;
    let mut packet_type: uint64_t = 0 as uint64_t;
    let mut trigger_length: uint64_t = 0 as uint64_t;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ret |= byteread_vint(s, &raw mut packet_type);
    ret |= byteread_vint(s, &raw mut trigger_length);
    if (*ctx).event_count != 0 as ::core::ffi::c_int {
        fprintf(f, b",\n\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        fprintf(f, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
    qlog_event_header(
        f,
        ctx,
        delta_time,
        path_id,
        b"transport\0".as_ptr() as *const ::core::ffi::c_char,
        b"packet_buffered\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        f,
        b"\n    \"type\" : \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
        ptype2str(packet_type as picoquic_packet_type_enum),
    );
    fprintf(
        f,
        b",\n    \"trigger\": \0".as_ptr() as *const ::core::ffi::c_char,
    );
    qlog_chars(f, s, trigger_length);
    fprintf(f, b"}]\0".as_ptr() as *const ::core::ffi::c_char);
    (*ctx).event_count += 1;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_pdu(
    mut time: uint64_t,
    mut rxtx: ::core::ffi::c_int,
    mut s: *mut bytestream,
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut qlog_context_t = ptr as *mut qlog_context_t;
    let mut delta_time: int64_t = time.wrapping_sub((*ctx).start_time) as int64_t;
    let mut f: *mut FILE = (*ctx).f_txtlog;
    let mut addr_peer: sockaddr_storage = sockaddr_storage {
        ss_family: 0 as sa_family_t,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut addr_local: sockaddr_storage = sockaddr_storage {
        ss_family: 0 as sa_family_t,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    let mut byte_length: uint64_t = 0 as uint64_t;
    let mut ret_local: ::core::ffi::c_int = 0;
    byteread_addr(s, &raw mut addr_peer);
    byteread_vint(s, &raw mut byte_length);
    ret_local = byteread_addr(s, &raw mut addr_local);
    if (*ctx).event_count != 0 as ::core::ffi::c_int {
        fprintf(f, b",\n\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        fprintf(f, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
    qlog_event_header(
        f,
        ctx,
        delta_time,
        0 as uint64_t,
        b"transport\0".as_ptr() as *const ::core::ffi::c_char,
        if rxtx == 0 as ::core::ffi::c_int {
            b"datagram_sent\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"datagram_received\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        f,
        b" \"byte_length\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        byte_length,
    );
    if addr_peer.ss_family as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && picoquic_compare_addr(
            &raw mut addr_peer as *mut sockaddr,
            &raw mut (*ctx).addr_peer as *mut sockaddr,
        ) != 0 as ::core::ffi::c_int
    {
        fprintf(
            f,
            b", \"%s\" : {\0".as_ptr() as *const ::core::ffi::c_char,
            if rxtx == 0 as ::core::ffi::c_int {
                b"addr_to\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"addr_from\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        qlog_log_addr(f, &raw mut addr_peer as *mut sockaddr);
        fprintf(f, b"}\0".as_ptr() as *const ::core::ffi::c_char);
        picoquic_store_addr(
            &raw mut (*ctx).addr_peer,
            &raw mut addr_peer as *mut sockaddr,
        );
    }
    if ret_local == 0 as ::core::ffi::c_int
        && addr_local.ss_family as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && picoquic_compare_addr(
            &raw mut addr_local as *mut sockaddr,
            &raw mut (*ctx).addr_local as *mut sockaddr,
        ) != 0 as ::core::ffi::c_int
    {
        fprintf(
            f,
            b", \"%s\" : {\0".as_ptr() as *const ::core::ffi::c_char,
            if rxtx != 0 as ::core::ffi::c_int {
                b"addr_to\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"addr_from\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        qlog_log_addr(f, &raw mut addr_local as *mut sockaddr);
        fprintf(f, b"}\0".as_ptr() as *const ::core::ffi::c_char);
        picoquic_store_addr(
            &raw mut (*ctx).addr_local,
            &raw mut addr_local as *mut sockaddr,
        );
    }
    fprintf(f, b"}]\0".as_ptr() as *const ::core::ffi::c_char);
    (*ctx).event_count += 1;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_packet_start(
    mut time: uint64_t,
    mut path_id: uint64_t,
    mut size: uint64_t,
    mut ph: *const picoquic_packet_header,
    mut rxtx: ::core::ffi::c_int,
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut qlog_context_t = ptr as *mut qlog_context_t;
    let mut f: *mut FILE = (*ctx).f_txtlog;
    let mut delta_time: int64_t = time.wrapping_sub((*ctx).start_time) as int64_t;
    if (*ctx).event_count != 0 as ::core::ffi::c_int {
        fprintf(f, b",\n\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        fprintf(f, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if (*ph).ptype as ::core::ffi::c_uint
        == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
        && rxtx == 0 as ::core::ffi::c_int
    {
        if (*ctx).spin_bit_sent() as ::core::ffi::c_int != 0
            && (*ctx).spin_bit_sent_last() as ::core::ffi::c_int
                != (*ph).spin() as ::core::ffi::c_int
        {
            qlog_event_header(
                f,
                ctx,
                delta_time,
                path_id,
                b"transport\0".as_ptr() as *const ::core::ffi::c_char,
                b"spin_bit_updated\0".as_ptr() as *const ::core::ffi::c_char,
            );
            fprintf(
                f,
                b" \"state\": %s }],\n\0".as_ptr() as *const ::core::ffi::c_char,
                if (*ph).spin() as ::core::ffi::c_int != 0 {
                    b"true\0".as_ptr() as *const ::core::ffi::c_char
                } else {
                    b"false\0".as_ptr() as *const ::core::ffi::c_char
                },
            );
        }
        (*ctx).set_spin_bit_sent(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*ctx).set_spin_bit_sent_last((*ph).spin() as ::core::ffi::c_uint);
    }
    qlog_event_header(
        f,
        ctx,
        delta_time,
        path_id,
        b"transport\0".as_ptr() as *const ::core::ffi::c_char,
        if rxtx == 0 as ::core::ffi::c_int {
            b"packet_sent\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"packet_received\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        f,
        b" \"packet_type\": \"%s\", \"header\": { \"packet_size\": %lu\0".as_ptr()
            as *const ::core::ffi::c_char,
        ptype2str((*ph).ptype),
        size,
    );
    if (*ph).ptype as ::core::ffi::c_uint
        != picoquic_packet_version_negotiation as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*ph).ptype as ::core::ffi::c_uint
            != picoquic_packet_retry as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fprintf(
            f,
            b", \"packet_number\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
            (*ph).pn64,
        );
    }
    if (*ph).ptype as ::core::ffi::c_uint
        != picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*ctx).version_number != (*ph).vn {
            fprintf(
                f,
                b", \"version\": \"%08x\"\0".as_ptr() as *const ::core::ffi::c_char,
                (*ph).vn,
            );
            (*ctx).version_number = (*ph).vn;
        }
        if (*ph).ptype as ::core::ffi::c_uint
            != picoquic_packet_version_negotiation as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*ph).ptype as ::core::ffi::c_uint
                != picoquic_packet_retry as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*ph).ptype as ::core::ffi::c_uint
                != picoquic_packet_error as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            fprintf(
                f,
                b", \"payload_length\": %zu\0".as_ptr() as *const ::core::ffi::c_char,
                (*ph).payload_length,
            );
        }
    }
    if (*ph).ptype as ::core::ffi::c_uint
        != picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*ph).srce_cnx_id.id_len as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        let mut scid_name: [::core::ffi::c_char; 41] = [0; 41];
        picoquic_print_connection_id_hexa(
            &raw mut scid_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 41]>() as size_t,
            &raw const (*ph).srce_cnx_id,
        );
        fprintf(
            f,
            b", \"scid\": \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut scid_name as *mut ::core::ffi::c_char,
        );
    }
    if (*ph).dest_cnx_id.id_len as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        let mut dcid_name: [::core::ffi::c_char; 41] = [0; 41];
        picoquic_print_connection_id_hexa(
            &raw mut dcid_name as *mut ::core::ffi::c_char,
            ::core::mem::size_of::<[::core::ffi::c_char; 41]>() as size_t,
            &raw const (*ph).dest_cnx_id,
        );
        fprintf(
            f,
            b", \"dcid\": \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut dcid_name as *mut ::core::ffi::c_char,
        );
    }
    if (*ph).ptype as ::core::ffi::c_uint
        == picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*ph).token_length > 0 as size_t
    {
        let mut token: bytestream = bytestream {
            data: ::core::ptr::null_mut::<uint8_t>(),
            size: 0,
            ptr: 0,
        };
        bytestream_ref_init(
            &raw mut token,
            (*ph).token_bytes as *const ::core::ffi::c_void,
            (*ph).token_length,
        );
        fprintf(f, b", \"token\": \0".as_ptr() as *const ::core::ffi::c_char);
        qlog_string(f, &raw mut token, (*ph).token_length as uint64_t);
    }
    if (*ph).ptype as ::core::ffi::c_uint
        == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut need_key_phase: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if rxtx == 0 as ::core::ffi::c_int {
            need_key_phase = ((*ctx).key_phase_sent() == 0
                || (*ctx).key_phase_sent_last() as ::core::ffi::c_int
                    != (*ph).key_phase() as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            (*ctx).set_key_phase_sent(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*ctx).set_key_phase_sent_last((*ph).key_phase() as ::core::ffi::c_uint);
        } else {
            need_key_phase = ((*ctx).key_phase_received() == 0
                || (*ctx).key_phase_received_last() as ::core::ffi::c_int
                    != (*ph).key_phase() as ::core::ffi::c_int)
                as ::core::ffi::c_int;
            (*ctx).set_key_phase_received_last((*ph).key_phase() as ::core::ffi::c_uint);
            (*ctx).set_key_phase_received(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        if need_key_phase != 0 {
            fprintf(
                f,
                b", \"key_phase\": %d\0".as_ptr() as *const ::core::ffi::c_char,
                (*ph).key_phase() as ::core::ffi::c_int,
            );
        }
    }
    if (*ph).quic_bit_is_zero() != 0 {
        fprintf(
            f,
            b", \"quic_bit\": 0\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    (*ctx).packet_type = (*ph).ptype;
    if (*ctx).packet_type as ::core::ffi::c_uint
        == picoquic_packet_version_negotiation as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*ctx).packet_type as ::core::ffi::c_uint
            == picoquic_packet_retry as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fprintf(f, b" }\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        fprintf(
            f,
            b" }, \"frames\": [\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    (*ctx).frame_count = 0 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_time_stamp_frame(mut f: *mut FILE, mut s: *mut bytestream) {
    let mut time_stamp: uint64_t = 0 as uint64_t;
    byteread_vint(s, &raw mut time_stamp);
    fprintf(
        f,
        b", \"time_stamp\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        time_stamp,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_path_abandon_frame(mut f: *mut FILE, mut s: *mut bytestream) {
    let mut path_id: uint64_t = 0;
    let mut reason: uint64_t = 0;
    byteread_vint(s, &raw mut path_id);
    byteread_vint(s, &raw mut reason);
    fprintf(
        f,
        b", \"path_id\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        path_id,
    );
    fprintf(
        f,
        b", \"reason\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        reason,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_path_backup_frame(mut f: *mut FILE, mut s: *mut bytestream) {
    let mut path_id: uint64_t = 0 as uint64_t;
    let mut sequence: uint64_t = 0;
    byteread_vint(s, &raw mut path_id);
    byteread_vint(s, &raw mut sequence);
    fprintf(
        f,
        b", \"path_id\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        path_id,
    );
    fprintf(
        f,
        b", \"sequence\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        sequence,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_path_available_frame(mut f: *mut FILE, mut s: *mut bytestream) {
    let mut path_id: uint64_t = 0 as uint64_t;
    let mut sequence: uint64_t = 0;
    byteread_vint(s, &raw mut path_id);
    byteread_vint(s, &raw mut sequence);
    fprintf(
        f,
        b", \"path_id\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        path_id,
    );
    fprintf(
        f,
        b", \"sequence\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        sequence,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_max_path_id_frame(mut f: *mut FILE, mut s: *mut bytestream) {
    let mut max_path_id: uint64_t = 0 as uint64_t;
    byteread_vint(s, &raw mut max_path_id);
    fprintf(
        f,
        b", \"max_path_id\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        max_path_id,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_path_blocked_frame(mut f: *mut FILE, mut s: *mut bytestream) {
    let mut max_path_id: uint64_t = 0 as uint64_t;
    byteread_vint(s, &raw mut max_path_id);
    fprintf(
        f,
        b", \"max_path_id\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        max_path_id,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_reset_stream_frame(mut f: *mut FILE, mut s: *mut bytestream) {
    let mut stream_id: uint64_t = 0 as uint64_t;
    let mut error_code: uint64_t = 0 as uint64_t;
    let mut final_size: uint64_t = 0 as uint64_t;
    byteread_vint(s, &raw mut stream_id);
    fprintf(
        f,
        b", \"stream_id\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        stream_id,
    );
    byteread_vint(s, &raw mut error_code);
    fprintf(
        f,
        b", \"error_code\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        error_code,
    );
    byteread_vint(s, &raw mut final_size);
    fprintf(
        f,
        b", \"final_size\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        final_size,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_stop_sending_frame(mut f: *mut FILE, mut s: *mut bytestream) {
    let mut stream_id: uint64_t = 0 as uint64_t;
    let mut error_code: uint64_t = 0 as uint64_t;
    byteread_vint(s, &raw mut stream_id);
    fprintf(
        f,
        b", \"stream_id\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        stream_id,
    );
    byteread_vint(s, &raw mut error_code);
    fprintf(
        f,
        b", \"error_code\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        error_code,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_closing_frame(
    mut ftype: uint64_t,
    mut f: *mut FILE,
    mut s: *mut bytestream,
) {
    let mut error_code: uint64_t = 0 as uint64_t;
    let mut offending_frame_type: uint64_t = 0 as uint64_t;
    let mut reason_length: uint64_t = 0 as uint64_t;
    let mut offensive_type_name: *const ::core::ffi::c_char =
        ::core::ptr::null::<::core::ffi::c_char>();
    fprintf(
        f,
        b", \"error_space\": \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
        if ftype == picoquic_frame_type_connection_close as ::core::ffi::c_int as uint64_t {
            b"transport\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"application\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    byteread_vint(s, &raw mut error_code);
    fprintf(
        f,
        b", \"error_code\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        error_code,
    );
    if ftype == picoquic_frame_type_connection_close as ::core::ffi::c_int as uint64_t
        && error_code != 0 as uint64_t
    {
        byteread_vint(s, &raw mut offending_frame_type);
        offensive_type_name = ftype2str(offending_frame_type as picoquic_frame_type_enum_t);
        if strcmp(
            offensive_type_name,
            b"unknown\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
        {
            fprintf(
                f,
                b", \"trigger_frame_type\": \"%lx\"\0".as_ptr() as *const ::core::ffi::c_char,
                offending_frame_type,
            );
        } else {
            fprintf(
                f,
                b", \"trigger_frame_type\": \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                offensive_type_name,
            );
        }
    }
    byteread_vint(s, &raw mut reason_length);
    if reason_length > 0 as uint64_t {
        fprintf(
            f,
            b", \"reason\": \"\0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut i: uint64_t = 0 as uint64_t;
        while i < reason_length && (*s).ptr < (*s).size {
            let c2rust_fresh2 = (*s).ptr;
            (*s).ptr = (*s).ptr.wrapping_add(1);
            let mut c: ::core::ffi::c_int =
                *(*s).data.offset(c2rust_fresh2 as isize) as ::core::ffi::c_int;
            if c < 0x20 as ::core::ffi::c_int || c > 0x7e as ::core::ffi::c_int {
                c = '.' as i32;
            }
            fprintf(f, b"%c\0".as_ptr() as *const ::core::ffi::c_char, c);
            i = i.wrapping_add(1);
        }
        fprintf(f, b"\"\0".as_ptr() as *const ::core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C" fn qlog_max_data_frame(mut f: *mut FILE, mut s: *mut bytestream) {
    let mut maximum: uint64_t = 0 as uint64_t;
    byteread_vint(s, &raw mut maximum);
    fprintf(
        f,
        b", \"maximum\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        maximum,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_max_stream_data_frame(mut f: *mut FILE, mut s: *mut bytestream) {
    let mut stream_id: uint64_t = 0 as uint64_t;
    let mut maximum: uint64_t = 0 as uint64_t;
    byteread_vint(s, &raw mut stream_id);
    fprintf(
        f,
        b", \"stream_id\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        stream_id,
    );
    byteread_vint(s, &raw mut maximum);
    fprintf(
        f,
        b", \"maximum\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        maximum,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_max_streams_frame(
    mut ftype: uint64_t,
    mut f: *mut FILE,
    mut s: *mut bytestream,
) {
    let mut maximum: uint64_t = 0;
    fprintf(
        f,
        b", \"stream_type\": \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
        if ftype == picoquic_frame_type_max_streams_bidir as ::core::ffi::c_int as uint64_t {
            b"bidirectional\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"unidirectional\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    byteread_vint(s, &raw mut maximum);
    fprintf(
        f,
        b", \"maximum\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        maximum,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_blocked_frame(mut f: *mut FILE, mut s: *mut bytestream) {
    let mut limit: uint64_t = 0 as uint64_t;
    byteread_vint(s, &raw mut limit);
    fprintf(
        f,
        b", \"limit\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        limit,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_stream_blocked_frame(mut f: *mut FILE, mut s: *mut bytestream) {
    let mut stream_id: uint64_t = 0 as uint64_t;
    let mut limit: uint64_t = 0 as uint64_t;
    byteread_vint(s, &raw mut stream_id);
    fprintf(
        f,
        b", \"stream_id\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        stream_id,
    );
    byteread_vint(s, &raw mut limit);
    fprintf(
        f,
        b", \"limit\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        limit,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_streams_blocked_frame(
    mut ftype: uint64_t,
    mut f: *mut FILE,
    mut s: *mut bytestream,
) {
    let mut limit: uint64_t = 0;
    fprintf(
        f,
        b", \"stream_type\": \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
        if ftype == picoquic_frame_type_streams_blocked_bidir as ::core::ffi::c_int as uint64_t {
            b"bidirectional\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"unidirectional\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    byteread_vint(s, &raw mut limit);
    fprintf(
        f,
        b", \"limit\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        limit,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_new_connection_id_frame(
    mut ftype: uint64_t,
    mut f: *mut FILE,
    mut s: *mut bytestream,
) {
    let mut sequence_number: uint64_t = 0 as uint64_t;
    let mut retire_before: uint64_t = 0 as uint64_t;
    let mut cid_length: uint64_t = 0 as uint64_t;
    if ftype == picoquic_frame_type_path_new_connection_id as ::core::ffi::c_int as uint64_t {
        let mut path_id: uint64_t = 0;
        byteread_vint(s, &raw mut path_id);
        fprintf(
            f,
            b", \"path_id\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
            path_id,
        );
    }
    byteread_vint(s, &raw mut sequence_number);
    fprintf(
        f,
        b", \"sequence_number\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        sequence_number,
    );
    byteread_vint(s, &raw mut retire_before);
    fprintf(
        f,
        b", \"retire_before\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        retire_before,
    );
    byteread_vint(s, &raw mut cid_length);
    fprintf(
        f,
        b", \"connection_id\": \0".as_ptr() as *const ::core::ffi::c_char,
    );
    qlog_string(f, s, cid_length);
    fprintf(
        f,
        b", \"reset_token\": \0".as_ptr() as *const ::core::ffi::c_char,
    );
    qlog_string(f, s, 16 as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn qlog_retire_connection_id_frame(
    mut ftype: uint64_t,
    mut f: *mut FILE,
    mut s: *mut bytestream,
) {
    let mut sequence_number: uint64_t = 0 as uint64_t;
    if ftype == picoquic_frame_type_path_retire_connection_id as ::core::ffi::c_int as uint64_t {
        let mut path_id: uint64_t = 0 as uint64_t;
        byteread_vint(s, &raw mut path_id);
        fprintf(
            f,
            b", \"path_id\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
            path_id,
        );
    }
    byteread_vint(s, &raw mut sequence_number);
    fprintf(
        f,
        b", \"sequence_number\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        sequence_number,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_new_token_frame(mut f: *mut FILE, mut s: *mut bytestream) {
    let mut toklen: uint64_t = 0 as uint64_t;
    fprintf(
        f,
        b", \"new_token\": \0".as_ptr() as *const ::core::ffi::c_char,
    );
    byteread_vint(s, &raw mut toklen);
    qlog_string(f, s, toklen);
}
#[no_mangle]
pub unsafe extern "C" fn qlog_path_frame(
    mut ftype: uint64_t,
    mut f: *mut FILE,
    mut s: *mut bytestream,
) {
    if ftype == picoquic_frame_type_path_challenge as ::core::ffi::c_int as uint64_t {
        fprintf(
            f,
            b", \"path_challenge\": \0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        fprintf(
            f,
            b", \"path_response\": \0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    qlog_string(f, s, 8 as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn qlog_crypto_hs_frame(mut f: *mut FILE, mut s: *mut bytestream) {
    let mut offset: uint64_t = 0 as uint64_t;
    let mut data_length: uint64_t = 0 as uint64_t;
    byteread_vint(s, &raw mut offset);
    fprintf(
        f,
        b", \"offset\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        offset,
    );
    byteread_vint(s, &raw mut data_length);
    fprintf(
        f,
        b", \"length\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        data_length,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_datagram_frame(
    mut ftype: uint64_t,
    mut f: *mut FILE,
    mut s: *mut bytestream,
) {
    let mut has_length: ::core::ffi::c_uint = (ftype & 1 as uint64_t) as ::core::ffi::c_uint;
    let mut length: uint64_t = 0 as uint64_t;
    if has_length != 0 {
        byteread_vint(s, &raw mut length);
        fprintf(
            f,
            b", \"length\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
            length,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn qlog_ack_frequency_frame(mut f: *mut FILE, mut s: *mut bytestream) {
    let mut sequence_number: uint64_t = 0 as uint64_t;
    let mut packet_tolerance: uint64_t = 0 as uint64_t;
    let mut max_ack_delay: uint64_t = 0 as uint64_t;
    let mut reordering_threshold: uint64_t = 0 as uint64_t;
    byteread_vint(s, &raw mut sequence_number);
    fprintf(
        f,
        b", \"sequence_number\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        sequence_number,
    );
    byteread_vint(s, &raw mut packet_tolerance);
    fprintf(
        f,
        b", \"packet_tolerance\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        packet_tolerance,
    );
    byteread_vint(s, &raw mut max_ack_delay);
    fprintf(
        f,
        b", \"max_ack_delay\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        max_ack_delay,
    );
    byteread_vint(s, &raw mut reordering_threshold);
    fprintf(
        f,
        b", \"reordering_threshold\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        reordering_threshold,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_ack_frame(
    mut ftype: uint64_t,
    mut f: *mut FILE,
    mut s: *mut bytestream,
) {
    let mut largest: uint64_t = 0 as uint64_t;
    let mut ack_delay: uint64_t = 0 as uint64_t;
    let mut num: uint64_t = 0 as uint64_t;
    let mut path_id: uint64_t = 0 as uint64_t;
    if ftype == picoquic_frame_type_path_ack as ::core::ffi::c_int as uint64_t
        || ftype == picoquic_frame_type_path_ack_ecn as ::core::ffi::c_int as uint64_t
    {
        byteread_vint(s, &raw mut path_id);
        fprintf(
            f,
            b", \"path_id\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
            path_id,
        );
    }
    byteread_vint(s, &raw mut largest);
    byteread_vint(s, &raw mut ack_delay);
    fprintf(
        f,
        b", \"ack_delay\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
        ack_delay,
    );
    byteread_vint(s, &raw mut num);
    fprintf(
        f,
        b", \"acked_ranges\": [\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i: uint64_t = 0 as uint64_t;
    while i <= num {
        let mut skip: uint64_t = 0 as uint64_t;
        let mut start_range: int64_t = 0;
        let mut end_range: int64_t = 0;
        if i != 0 as uint64_t {
            byteread_vint(s, &raw mut skip);
            skip = skip.wrapping_add(1);
            largest = largest.wrapping_sub(skip);
            fprintf(f, b", \0".as_ptr() as *const ::core::ffi::c_char);
        }
        let mut range: uint64_t = 0 as uint64_t;
        byteread_vint(s, &raw mut range);
        start_range = largest.wrapping_sub(range) as int64_t;
        end_range = largest as int64_t;
        fprintf(
            f,
            b"[%ld, %ld]\0".as_ptr() as *const ::core::ffi::c_char,
            start_range,
            end_range,
        );
        largest = largest.wrapping_sub(range.wrapping_add(1 as uint64_t));
        i = i.wrapping_add(1);
    }
    fprintf(f, b"]\0".as_ptr() as *const ::core::ffi::c_char);
    if ftype == picoquic_frame_type_ack_ecn as ::core::ffi::c_int as uint64_t
        || ftype == picoquic_frame_type_path_ack_ecn as ::core::ffi::c_int as uint64_t
    {
        let mut ecn_name: [*const ::core::ffi::c_char; 3] = [
            b"ect0\0".as_ptr() as *const ::core::ffi::c_char,
            b"ect1\0".as_ptr() as *const ::core::ffi::c_char,
            b"ce\0".as_ptr() as *const ::core::ffi::c_char,
        ];
        let mut ecnx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while ecnx < 3 as ::core::ffi::c_int {
            let mut ecn_v: uint64_t = 0 as uint64_t;
            byteread_vint(s, &raw mut ecn_v);
            fprintf(
                f,
                b", \"%s\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
                ecn_name[ecnx as usize],
                ecn_v,
            );
            ecnx += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn qlog_erroring_frame(
    mut f: *mut FILE,
    mut s: *mut bytestream,
    mut ftype: uint64_t,
) {
    let mut extra_bytes: size_t = (*s).size.wrapping_sub((*s).ptr);
    fprintf(
        f,
        b",\"unknown_type\": %lu,\0".as_ptr() as *const ::core::ffi::c_char,
        ftype,
    );
    fprintf(
        f,
        b"\"begins_with\": \0".as_ptr() as *const ::core::ffi::c_char,
    );
    qlog_string(
        f,
        s,
        if extra_bytes > 8 as size_t {
            8 as uint64_t
        } else {
            extra_bytes as uint64_t
        },
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_proposed_versions(
    mut f: *mut FILE,
    mut s: *mut bytestream,
) -> ::core::ffi::c_int {
    let mut nb_versions: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    fprintf(
        f,
        b",\n    \"proposed_versions\": [\0".as_ptr() as *const ::core::ffi::c_char,
    );
    while bytestream_remain(s) > 0 as size_t {
        if nb_versions > 0 as ::core::ffi::c_int {
            fprintf(f, b", \0".as_ptr() as *const ::core::ffi::c_char);
        }
        qlog_string(f, s, 4 as uint64_t);
        nb_versions += 1;
    }
    fprintf(f, b"]\0".as_ptr() as *const ::core::ffi::c_char);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_retry_token(
    mut f: *mut FILE,
    mut s: *mut bytestream,
) -> ::core::ffi::c_int {
    let mut l: size_t = bytestream_remain(s);
    if l > 0 as size_t {
        fprintf(
            f,
            b",\n    \"retry_token\": \0".as_ptr() as *const ::core::ffi::c_char,
        );
        qlog_string(f, s, l as uint64_t);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_bdp_frame(mut f: *mut FILE, mut s: *mut bytestream) {
    let mut lifetime: uint64_t = 0 as uint64_t;
    let mut recon_bytes_in_flight: uint64_t = 0 as uint64_t;
    let mut recon_min_rtt: uint64_t = 0 as uint64_t;
    let mut ip_len: uint64_t = 0 as uint64_t;
    byteread_vint(s, &raw mut lifetime);
    byteread_vint(s, &raw mut recon_bytes_in_flight);
    byteread_vint(s, &raw mut recon_min_rtt);
    byteread_vint(s, &raw mut ip_len);
    fprintf(
        f,
        b", \"lifetime\": %lu, \"bytes_in_flight\": %lu, \"min_rtt\": %lu, \"ip\": \0".as_ptr()
            as *const ::core::ffi::c_char,
        lifetime,
        recon_bytes_in_flight,
        recon_min_rtt,
    );
    qlog_string(f, s, ip_len);
}
#[no_mangle]
pub unsafe extern "C" fn qlog_observed_address_frame(
    mut ftype: uint64_t,
    mut f: *mut FILE,
    mut s: *mut bytestream,
) {
    let mut port: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    let mut sequence: uint64_t = 0 as uint64_t;
    byteread_vint(s, &raw mut sequence);
    fprintf(
        f,
        b", \"sequence\": %lu, \"address\": \"\0".as_ptr() as *const ::core::ffi::c_char,
        sequence,
    );
    if ftype & 1 as uint64_t == 0 as uint64_t {
        let mut x: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x < 4 as ::core::ffi::c_int && (*s).ptr < (*s).size {
            if x != 0 as ::core::ffi::c_int {
                fprintf(f, b".\0".as_ptr() as *const ::core::ffi::c_char);
            }
            let c2rust_fresh3 = (*s).ptr;
            (*s).ptr = (*s).ptr.wrapping_add(1);
            fprintf(
                f,
                b"%d\0".as_ptr() as *const ::core::ffi::c_char,
                *(*s).data.offset(c2rust_fresh3 as isize) as ::core::ffi::c_int,
            );
            x += 1;
        }
    } else {
        let mut x_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while x_0 < 8 as ::core::ffi::c_int && (*s).ptr < (*s).size {
            let mut w: uint16_t = 0 as uint16_t;
            let mut y: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while y < 2 as ::core::ffi::c_int && (*s).ptr < (*s).size {
                w = ((w as ::core::ffi::c_int) << 8 as ::core::ffi::c_int) as uint16_t;
                let c2rust_fresh4 = (*s).ptr;
                (*s).ptr = (*s).ptr.wrapping_add(1);
                w = (w as ::core::ffi::c_int
                    + *(*s).data.offset(c2rust_fresh4 as isize) as ::core::ffi::c_int)
                    as uint16_t;
                y += 1;
            }
            if x_0 != 0 as ::core::ffi::c_int {
                fprintf(f, b":\0".as_ptr() as *const ::core::ffi::c_char);
            }
            fprintf(
                f,
                b"%x\0".as_ptr() as *const ::core::ffi::c_char,
                w as ::core::ffi::c_int,
            );
            x_0 += 1;
        }
    }
    let mut y_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while y_0 < 2 as ::core::ffi::c_int && (*s).ptr < (*s).size {
        port <<= 8 as ::core::ffi::c_int;
        let c2rust_fresh5 = (*s).ptr;
        (*s).ptr = (*s).ptr.wrapping_add(1);
        port = port.wrapping_add(*(*s).data.offset(c2rust_fresh5 as isize) as ::core::ffi::c_uint);
        y_0 += 1;
    }
    fprintf(
        f,
        b"\", \"port\": %u\0".as_ptr() as *const ::core::ffi::c_char,
        port,
    );
}
#[no_mangle]
pub unsafe extern "C" fn qlog_packet_frame(
    mut s: *mut bytestream,
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut qlog_context_t = ptr as *mut qlog_context_t;
    let mut f: *mut FILE = (*ctx).f_txtlog;
    if (*ctx).packet_type as ::core::ffi::c_uint
        == picoquic_packet_version_negotiation as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return qlog_proposed_versions(f, s);
    } else if (*ctx).packet_type as ::core::ffi::c_uint
        == picoquic_packet_retry as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return qlog_retry_token(f, s);
    }
    if (*ctx).frame_count != 0 as ::core::ffi::c_int {
        fprintf(f, b", \0".as_ptr() as *const ::core::ffi::c_char);
    }
    fprintf(f, b"{ \0".as_ptr() as *const ::core::ffi::c_char);
    let mut ftype: uint64_t = 0 as uint64_t;
    let mut ptr_before_type: size_t = (*s).ptr;
    byteread_vint(s, &raw mut ftype);
    fprintf(
        f,
        b"\n    \"frame_type\": \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
        ftype2str(ftype as picoquic_frame_type_enum_t),
    );
    if ftype >= picoquic_frame_type_stream_range_min as ::core::ffi::c_int as uint64_t
        && ftype <= picoquic_frame_type_stream_range_max as ::core::ffi::c_int as uint64_t
    {
        let mut stream_id: uint64_t = 0 as uint64_t;
        byteread_vint(s, &raw mut stream_id);
        let mut offset: uint64_t = 0 as uint64_t;
        if ftype & 4 as uint64_t != 0 as uint64_t {
            byteread_vint(s, &raw mut offset);
        }
        let mut length: uint64_t = 0 as uint64_t;
        byteread_vint(s, &raw mut length);
        fprintf(
            f,
            b", \"id\": %lu, \"offset\": %lu, \"length\": %lu, \"fin\": %s \0".as_ptr()
                as *const ::core::ffi::c_char,
            stream_id,
            offset,
            length,
            if ftype & 1 as uint64_t != 0 {
                b"true\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                b"false\0".as_ptr() as *const ::core::ffi::c_char
            },
        );
        if ftype & 2 as uint64_t == 0 as uint64_t {
            fprintf(
                f,
                b", \"has_length\": false\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        let mut extra_bytes: uint64_t = bytestream_remain(s) as uint64_t;
        if extra_bytes > 0 as uint64_t {
            fprintf(
                f,
                b", \"begins_with\": \0".as_ptr() as *const ::core::ffi::c_char,
            );
            qlog_string(f, s, extra_bytes);
        }
    } else {
        match ftype {
            2 | 3 | 354585600 | 354585601 => {
                qlog_ack_frame(ftype, f, s);
            }
            4 => {
                qlog_reset_stream_frame(f, s);
            }
            5 => {
                qlog_stop_sending_frame(f, s);
            }
            6 => {
                qlog_crypto_hs_frame(f, s);
            }
            7 => {
                qlog_new_token_frame(f, s);
            }
            16 => {
                qlog_max_data_frame(f, s);
            }
            17 => {
                qlog_max_stream_data_frame(f, s);
            }
            18 | 19 => {
                qlog_max_streams_frame(ftype, f, s);
            }
            20 => {
                qlog_blocked_frame(f, s);
            }
            21 => {
                qlog_stream_blocked_frame(f, s);
            }
            22 | 23 => {
                qlog_streams_blocked_frame(ftype, f, s);
            }
            24 | 354585609 => {
                qlog_new_connection_id_frame(ftype, f, s);
            }
            25 | 354585610 => {
                qlog_retire_connection_id_frame(ftype, f, s);
            }
            26 | 27 => {
                qlog_path_frame(ftype, f, s);
            }
            28 | 29 => {
                qlog_closing_frame(ftype, f, s);
            }
            48 | 49 => {
                qlog_datagram_frame(ftype, f, s);
            }
            175 => {
                qlog_ack_frequency_frame(f, s);
            }
            0 | 1 | 32 | 30 | 31 => {}
            757 => {
                qlog_time_stamp_frame(f, s);
            }
            354585605 => {
                qlog_path_abandon_frame(f, s);
            }
            354585607 => {
                qlog_path_backup_frame(f, s);
            }
            354585608 => {
                qlog_path_available_frame(f, s);
            }
            60377 => {
                qlog_bdp_frame(f, s);
            }
            354585612 => {
                qlog_max_path_id_frame(f, s);
            }
            354585613 => {
                qlog_path_blocked_frame(f, s);
            }
            10453414 | 10453415 => {
                qlog_observed_address_frame(ftype, f, s);
            }
            _ => {
                (*s).ptr = ptr_before_type;
                qlog_erroring_frame(f, s, ftype);
            }
        }
    }
    fprintf(f, b"}\0".as_ptr() as *const ::core::ffi::c_char);
    (*ctx).frame_count += 1;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_packet_end(mut ptr: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut ctx: *mut qlog_context_t = ptr as *mut qlog_context_t;
    let mut f: *mut FILE = (*ctx).f_txtlog;
    if (*ctx).packet_type as ::core::ffi::c_uint
        == picoquic_packet_version_negotiation as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*ctx).packet_type as ::core::ffi::c_uint
            == picoquic_packet_retry as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        fprintf(f, b"}]\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        fprintf(f, b"]}]\0".as_ptr() as *const ::core::ffi::c_char);
    }
    (*ctx).packet_count += 1;
    (*ctx).event_count += 1;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_cc_update(
    mut time: uint64_t,
    mut path_id: uint64_t,
    mut s: *mut bytestream,
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sequence: uint64_t = 0 as uint64_t;
    let mut packet_rcvd: uint64_t = 0 as uint64_t;
    let mut highest_ack: uint64_t = UINT64_MAX as uint64_t;
    let mut high_ack_time: uint64_t = 0 as uint64_t;
    let mut last_time_ack: uint64_t = 0 as uint64_t;
    let mut cwin: uint64_t = 0 as uint64_t;
    let mut one_way_delay: uint64_t = 0 as uint64_t;
    let mut rtt_sample: uint64_t = 0 as uint64_t;
    let mut SRTT: uint64_t = 0 as uint64_t;
    let mut RTT_min: uint64_t = 0 as uint64_t;
    let mut bandwidth_estimate: uint64_t = 0 as uint64_t;
    let mut receive_rate_estimate: uint64_t = 0 as uint64_t;
    let mut Send_MTU: uint64_t = 0 as uint64_t;
    let mut pacing_packet_time: uint64_t = 0 as uint64_t;
    let mut nb_retrans: uint64_t = 0 as uint64_t;
    let mut nb_spurious: uint64_t = 0 as uint64_t;
    let mut cwin_blkd: uint64_t = 0 as uint64_t;
    let mut flow_blkd: uint64_t = 0 as uint64_t;
    let mut stream_blkd: uint64_t = 0 as uint64_t;
    let mut cc_state: uint64_t = 0 as uint64_t;
    let mut cc_param: uint64_t = 0 as uint64_t;
    let mut bw_max: uint64_t = 0 as uint64_t;
    let mut bytes_in_transit: uint64_t = 0 as uint64_t;
    let mut app_limited: uint64_t = 0 as uint64_t;
    let mut ctx: *mut qlog_context_t = ptr as *mut qlog_context_t;
    let mut f: *mut FILE = (*ctx).f_txtlog;
    ret |= byteread_vint(s, &raw mut sequence);
    ret |= byteread_vint(s, &raw mut packet_rcvd);
    if packet_rcvd != 0 as uint64_t {
        ret |= byteread_vint(s, &raw mut highest_ack);
        ret |= byteread_vint(s, &raw mut high_ack_time);
        ret |= byteread_vint(s, &raw mut last_time_ack);
    }
    ret |= byteread_vint(s, &raw mut cwin);
    ret |= byteread_vint(s, &raw mut one_way_delay);
    ret |= byteread_vint(s, &raw mut rtt_sample);
    ret |= byteread_vint(s, &raw mut SRTT);
    ret |= byteread_vint(s, &raw mut RTT_min);
    ret |= byteread_vint(s, &raw mut bandwidth_estimate);
    ret |= byteread_vint(s, &raw mut receive_rate_estimate);
    ret |= byteread_vint(s, &raw mut Send_MTU);
    ret |= byteread_vint(s, &raw mut pacing_packet_time);
    ret |= byteread_vint(s, &raw mut nb_retrans);
    ret |= byteread_vint(s, &raw mut nb_spurious);
    ret |= byteread_vint(s, &raw mut cwin_blkd);
    ret |= byteread_vint(s, &raw mut flow_blkd);
    ret |= byteread_vint(s, &raw mut stream_blkd);
    ret |= byteread_vint(s, &raw mut cc_state);
    ret |= byteread_vint(s, &raw mut cc_param);
    ret |= byteread_vint(s, &raw mut bw_max);
    ret |= byteread_vint(s, &raw mut bytes_in_transit);
    byteread_vint(s, &raw mut app_limited);
    if ret == 0 as ::core::ffi::c_int
        && (cwin != (*ctx).cwin
            || rtt_sample != (*ctx).rtt_sample
            || SRTT != (*ctx).SRTT
            || RTT_min != (*ctx).RTT_min
            || bytes_in_transit != (*ctx).bytes_in_transit
            || pacing_packet_time != (*ctx).pacing_packet_time)
    {
        let mut delta_time: int64_t = time.wrapping_sub((*ctx).start_time) as int64_t;
        let mut comma: *mut ::core::ffi::c_char =
            b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        if (*ctx).event_count != 0 as ::core::ffi::c_int {
            fprintf(f, b",\n\0".as_ptr() as *const ::core::ffi::c_char);
        } else {
            fprintf(f, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        }
        qlog_event_header(
            f,
            ctx,
            delta_time,
            path_id,
            b"recovery\0".as_ptr() as *const ::core::ffi::c_char,
            b"metrics_updated\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if cwin != (*ctx).cwin {
            fprintf(
                f,
                b"%s\"cwnd\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
                comma,
                cwin,
            );
            (*ctx).cwin = cwin;
            comma = b",\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        if pacing_packet_time != (*ctx).pacing_packet_time && pacing_packet_time > 0 as uint64_t {
            let mut bps: ::core::ffi::c_double = Send_MTU as ::core::ffi::c_double
                * 8 as ::core::ffi::c_int as ::core::ffi::c_double
                * 1000000.0f64
                / pacing_packet_time as ::core::ffi::c_double;
            let mut bits_per_second: uint64_t = bps as uint64_t;
            fprintf(
                f,
                b"%s\"pacing_rate\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
                comma,
                bits_per_second,
            );
            (*ctx).pacing_packet_time = pacing_packet_time;
            comma = b",\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        if bytes_in_transit != (*ctx).bytes_in_transit {
            fprintf(
                f,
                b"%s\"bytes_in_flight\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
                comma,
                bytes_in_transit,
            );
            (*ctx).bytes_in_transit = bytes_in_transit;
            comma = b",\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        if SRTT != (*ctx).SRTT {
            fprintf(
                f,
                b"%s\"smoothed_rtt\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
                comma,
                SRTT,
            );
            comma = b",\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        if RTT_min != (*ctx).RTT_min {
            fprintf(
                f,
                b"%s\"min_rtt\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
                comma,
                RTT_min,
            );
            (*ctx).RTT_min = RTT_min;
            comma = b",\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        if rtt_sample != (*ctx).rtt_sample {
            fprintf(
                f,
                b"%s\"latest_rtt\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
                comma,
                rtt_sample,
            );
            (*ctx).rtt_sample = rtt_sample;
            comma = b",\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_char;
        }
        if app_limited != (*ctx).app_limited() as uint64_t {
            fprintf(
                f,
                b"%s\"app_limited\": %lu\0".as_ptr() as *const ::core::ffi::c_char,
                comma,
                app_limited,
            );
            (*ctx).set_app_limited(
                (app_limited != 0 as uint64_t) as ::core::ffi::c_int as ::core::ffi::c_uint
                    as ::core::ffi::c_uint,
            );
        }
        fprintf(f, b"}]\0".as_ptr() as *const ::core::ffi::c_char);
        (*ctx).event_count += 1;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_info_message(
    mut time: uint64_t,
    mut s: *mut bytestream,
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ctx: *mut qlog_context_t = ptr as *mut qlog_context_t;
    let mut f: *mut FILE = (*ctx).f_txtlog;
    let mut delta_time: int64_t = time.wrapping_sub((*ctx).start_time) as int64_t;
    let mut message: [uint8_t; 2560] = [0; 2560];
    let mut message_length: size_t = 0 as size_t;
    if (*ctx).event_count != 0 as ::core::ffi::c_int {
        fprintf(f, b",\n\0".as_ptr() as *const ::core::ffi::c_char);
    } else {
        fprintf(f, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
    qlog_event_header(
        f,
        ctx,
        delta_time,
        0 as uint64_t,
        b"info\0".as_ptr() as *const ::core::ffi::c_char,
        b"message\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        f,
        b" \"message\": \"\0".as_ptr() as *const ::core::ffi::c_char,
    );
    message_length = bytestream_remain(s);
    if message_length > ::core::mem::size_of::<[uint8_t; 2560]>() as usize {
        message_length = ::core::mem::size_of::<[uint8_t; 2560]>() as usize as size_t;
    }
    memcpy(
        &raw mut message as *mut uint8_t as *mut ::core::ffi::c_void,
        bytestream_ptr(s) as *const ::core::ffi::c_void,
        message_length,
    );
    let mut i: size_t = 0 as size_t;
    while i < message_length {
        let mut c: ::core::ffi::c_int = message[i as usize] as ::core::ffi::c_int;
        if c < 0x20 as ::core::ffi::c_int || c > 0x7e as ::core::ffi::c_int {
            message[i as usize] = '?' as i32 as uint8_t;
        }
        i = i.wrapping_add(1);
    }
    fwrite(
        &raw mut message as *mut uint8_t as *const ::core::ffi::c_void,
        message_length,
        1 as size_t,
        f,
    );
    fprintf(f, b"\"}]\0".as_ptr() as *const ::core::ffi::c_char);
    (*ctx).event_count += 1;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_connection_start(
    mut time: uint64_t,
    mut cid: *const picoquic_connection_id_t,
    mut client_mode: ::core::ffi::c_int,
    mut proposed_version: uint32_t,
    mut remote_cnxid: *const picoquic_connection_id_t,
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut qlog_context_t = ptr as *mut qlog_context_t;
    let mut f: *mut FILE = (*ctx).f_txtlog;
    (*ctx).start_time = time;
    (*ctx).packet_count = 0 as ::core::ffi::c_int;
    (*ctx).event_count = 0 as ::core::ffi::c_int;
    (*ctx).version_number = 0 as uint32_t;
    memset(
        &raw mut (*ctx).addr_peer as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_storage>() as size_t,
    );
    memset(
        &raw mut (*ctx).addr_local as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_storage>() as size_t,
    );
    (*ctx).cwin = 0 as uint64_t;
    (*ctx).bytes_in_transit = 0 as uint64_t;
    (*ctx).SRTT = PICOQUIC_INITIAL_RTT as uint64_t;
    (*ctx).RTT_min = 0 as uint64_t;
    (*ctx).rtt_sample = 0 as uint64_t;
    (*ctx).pacing_packet_time = 1 as uint64_t;
    (*ctx).set_key_phase_sent_last(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*ctx).set_key_phase_sent(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*ctx).set_key_phase_received_last(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*ctx).set_key_phase_received(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*ctx).set_spin_bit_sent_last(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*ctx).set_spin_bit_sent(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    fprintf(
        f,
        b"{ \"qlog_version\": \"draft-00\", \"title\": \"picoquic\", \"traces\": [\n\0".as_ptr()
            as *const ::core::ffi::c_char,
    );
    fprintf(
        f,
        b"{ \"vantage_point\": { \"name\": \"backend-67\", \"type\": \"%s\" },\n\0".as_ptr()
            as *const ::core::ffi::c_char,
        if client_mode != 0 {
            b"client\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"server\0".as_ptr() as *const ::core::ffi::c_char
        },
    );
    fprintf(
        f,
        b"\"title\": \"picoquic\", \"description\": \"%s\",\0".as_ptr()
            as *const ::core::ffi::c_char,
        (*ctx).cid_name,
    );
    if (*ctx).trace_flow_id() != 0 {
        fprintf(
            f,
            b"\"event_fields\": [\"relative_time\", \"path_id\", \"category\", \"event\", \"data\"],\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        fprintf(
            f,
            b"\"event_fields\": [\"relative_time\", \"category\", \"event\", \"data\"],\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
        );
    }
    fprintf(
        f,
        b"\"configuration\": {\"time_units\": \"us\"},\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    fprintf(
        f,
        b"\"common_fields\": { \"protocol_type\": \"QUIC_HTTP3\", \"reference_time\": \"%lu\"},\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        (*ctx).start_time,
    );
    fprintf(f, b"\"events\": [\0".as_ptr() as *const ::core::ffi::c_char);
    (*ctx).state = 1 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_connection_end(
    mut time: uint64_t,
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut qlog_context_t = ptr as *mut qlog_context_t;
    let mut f: *mut FILE = (*ctx).f_txtlog;
    fprintf(f, b"]}]}\n\0".as_ptr() as *const ::core::ffi::c_char);
    (*ctx).state = 2 as ::core::ffi::c_int;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn qlog_convert(
    mut cid: *const picoquic_connection_id_t,
    mut f_binlog: *mut FILE,
    mut binlog_name: *const ::core::ffi::c_char,
    mut txt_name: *const ::core::ffi::c_char,
    mut out_dir: *const ::core::ffi::c_char,
    mut flags: uint16_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut f_txtlog: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut cid_name: [::core::ffi::c_char; 41] = [0; 41];
    if picoquic_print_connection_id_hexa(
        &raw mut cid_name as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 41]>() as size_t,
        cid,
    ) != 0 as ::core::ffi::c_int
    {
        ret = -(1 as ::core::ffi::c_int);
    } else if txt_name.is_null() {
        f_txtlog = open_outfile(
            &raw mut cid_name as *mut ::core::ffi::c_char,
            binlog_name,
            out_dir,
            b"qlog\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        f_txtlog = picoquic_file_open(txt_name, b"w\0".as_ptr() as *const ::core::ffi::c_char);
    }
    if f_txtlog.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else if ret == 0 as ::core::ffi::c_int {
        let mut qlog: qlog_context_t = qlog_context_st {
            f_txtlog: ::core::ptr::null_mut::<FILE>(),
            version_number: 0,
            cid_name: ::core::ptr::null::<::core::ffi::c_char>(),
            addr_peer: sockaddr_storage {
                ss_family: 0,
                __ss_padding: [0; 118],
                __ss_align: 0,
            },
            addr_local: sockaddr_storage {
                ss_family: 0,
                __ss_padding: [0; 118],
                __ss_align: 0,
            },
            start_time: 0,
            event_count: 0,
            packet_count: 0,
            frame_count: 0,
            packet_type: picoquic_packet_error,
            cwin: 0,
            rtt_sample: 0,
            SRTT: 0,
            RTT_min: 0,
            bytes_in_transit: 0,
            pacing_packet_time: 0,
            trace_flow_id_key_phase_sent_last_key_phase_sent_key_phase_received_last_key_phase_received_spin_bit_sent_last_spin_bit_sent_app_limited: [0; 1],
            c2rust_padding: [0; 3],
            state: 0,
        };
        memset(
            &raw mut qlog as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<qlog_context_t>() as size_t,
        );
        qlog.f_txtlog = f_txtlog;
        qlog.cid_name = &raw mut cid_name as *mut ::core::ffi::c_char;
        qlog.start_time = 0 as uint64_t;
        qlog.packet_count = 0 as ::core::ffi::c_int;
        qlog.state = 0 as ::core::ffi::c_int;
        qlog.set_trace_flow_id(
            (if flags as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as ::core::ffi::c_uint as ::core::ffi::c_uint,
        );
        let mut ctx: binlog_convert_cb_t = binlog_convert_cb_st {
            connection_start: None,
            alpn_update: None,
            param_update: None,
            pdu: None,
            packet_start: None,
            packet_frame: None,
            packet_end: None,
            packet_lost: None,
            packet_dropped: None,
            packet_buffered: None,
            cc_update: None,
            info_message: None,
            connection_end: None,
            ptr: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        };
        ctx.connection_start = Some(
            qlog_connection_start
                as unsafe extern "C" fn(
                    uint64_t,
                    *const picoquic_connection_id_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    *const picoquic_connection_id_t,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    uint64_t,
                    *const picoquic_connection_id_t,
                    ::core::ffi::c_int,
                    uint32_t,
                    *const picoquic_connection_id_t,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
        ctx.connection_end = Some(
            qlog_connection_end
                as unsafe extern "C" fn(uint64_t, *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(uint64_t, *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
            >;
        ctx.alpn_update = Some(
            qlog_alpn_update
                as unsafe extern "C" fn(
                    uint64_t,
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    uint64_t,
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
        ctx.param_update = Some(
            qlog_param_update
                as unsafe extern "C" fn(
                    uint64_t,
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    uint64_t,
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
        ctx.pdu = Some(
            qlog_pdu
                as unsafe extern "C" fn(
                    uint64_t,
                    ::core::ffi::c_int,
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    uint64_t,
                    ::core::ffi::c_int,
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
        ctx.packet_start = Some(
            qlog_packet_start
                as unsafe extern "C" fn(
                    uint64_t,
                    uint64_t,
                    uint64_t,
                    *const picoquic_packet_header,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    uint64_t,
                    uint64_t,
                    uint64_t,
                    *const picoquic_packet_header,
                    ::core::ffi::c_int,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
        ctx.packet_frame = Some(
            qlog_packet_frame
                as unsafe extern "C" fn(
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
        ctx.packet_end = Some(
            qlog_packet_end as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        )
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>;
        ctx.packet_lost = Some(
            qlog_packet_lost
                as unsafe extern "C" fn(
                    uint64_t,
                    uint64_t,
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    uint64_t,
                    uint64_t,
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
        ctx.packet_dropped = Some(
            qlog_packet_dropped
                as unsafe extern "C" fn(
                    uint64_t,
                    uint64_t,
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    uint64_t,
                    uint64_t,
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
        ctx.packet_buffered = Some(
            qlog_packet_buffered
                as unsafe extern "C" fn(
                    uint64_t,
                    uint64_t,
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    uint64_t,
                    uint64_t,
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
        ctx.cc_update = Some(
            qlog_cc_update
                as unsafe extern "C" fn(
                    uint64_t,
                    uint64_t,
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    uint64_t,
                    uint64_t,
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
        ctx.info_message = Some(
            qlog_info_message
                as unsafe extern "C" fn(
                    uint64_t,
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    uint64_t,
                    *mut bytestream,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
        ctx.ptr = &raw mut qlog as *mut ::core::ffi::c_void;
        ret = binlog_convert(f_binlog, cid, &raw mut ctx);
        if qlog.state == 1 as ::core::ffi::c_int {
            qlog_connection_end(0 as uint64_t, &raw mut qlog as *mut ::core::ffi::c_void);
        }
        picoquic_file_close(f_txtlog);
    }
    return ret;
}

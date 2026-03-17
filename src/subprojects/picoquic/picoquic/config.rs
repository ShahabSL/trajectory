use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type st_ptls_iovec_t;
    pub type st_ptls_buffer_t;
    pub type st_ptls_verify_certificate_t;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn putc(__c: ::core::ffi::c_int, __stream: *mut FILE) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn picoquic_set_log_level(quic: *mut picoquic_quic_t, log_level: ::core::ffi::c_int);
    fn picoquic_enable_sslkeylog(quic: *mut picoquic_quic_t, enable_sslkeylog: ::core::ffi::c_int);
    fn picoquic_set_random_initial(quic: *mut picoquic_quic_t, random_initial: ::core::ffi::c_int);
    fn picoquic_set_padding_policy(
        quic: *mut picoquic_quic_t,
        padding_min_size: uint32_t,
        padding_multiple: uint32_t,
    );
    fn picoquic_disable_port_blocking(
        quic: *mut picoquic_quic_t,
        is_port_blocking_disabled: ::core::ffi::c_int,
    );
    fn picoquic_create(
        max_nb_connections: uint32_t,
        cert_file_name: *const ::core::ffi::c_char,
        key_file_name: *const ::core::ffi::c_char,
        cert_root_file_name: *const ::core::ffi::c_char,
        default_alpn: *const ::core::ffi::c_char,
        default_callback_fn: picoquic_stream_data_cb_fn,
        default_callback_ctx: *mut ::core::ffi::c_void,
        cnx_id_callback: picoquic_connection_id_cb_fn,
        cnx_id_callback_data: *mut ::core::ffi::c_void,
        reset_seed: *mut uint8_t,
        current_time: uint64_t,
        p_simulated_time: *mut uint64_t,
        ticket_file_name: *const ::core::ffi::c_char,
        ticket_encryption_key: *const uint8_t,
        ticket_encryption_key_length: size_t,
    ) -> *mut picoquic_quic_t;
    fn picoquic_free(quic: *mut picoquic_quic_t);
    fn picoquic_set_cookie_mode(quic: *mut picoquic_quic_t, cookie_mode: ::core::ffi::c_int);
    fn picoquic_set_cipher_suite(
        quic: *mut picoquic_quic_t,
        cipher_suite_id: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_set_default_spinbit_policy(
        quic: *mut picoquic_quic_t,
        default_spinbit_policy: picoquic_spinbit_version_enum,
    ) -> ::core::ffi::c_int;
    fn picoquic_set_default_lossbit_policy(
        quic: *mut picoquic_quic_t,
        default_lossbit_policy: picoquic_lossbit_version_enum,
    );
    fn picoquic_set_default_multipath_option(
        quic: *mut picoquic_quic_t,
        multipath_option: ::core::ffi::c_int,
    );
    fn picoquic_set_default_address_discovery_mode(
        quic: *mut picoquic_quic_t,
        mode: ::core::ffi::c_int,
    );
    fn picoquic_set_cwin_max(quic: *mut picoquic_quic_t, cwin_max: uint64_t);
    fn picoquic_set_default_idle_timeout(quic: *mut picoquic_quic_t, idle_timeout_ms: uint64_t);
    fn picoquic_load_retry_tokens(
        quic: *mut picoquic_quic_t,
        token_store_filename: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn picoquic_set_default_bdp_frame_option(
        quic: *mut picoquic_quic_t,
        enable_bdp_frame: ::core::ffi::c_int,
    );
    fn picoquic_set_default_connection_id_length(
        quic: *mut picoquic_quic_t,
        cid_length: uint8_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_set_mtu_max(quic: *mut picoquic_quic_t, mtu_max: uint32_t);
    fn picoquic_set_initial_send_mtu(
        quic: *mut picoquic_quic_t,
        intitial_mtu_ipv4: uint32_t,
        intitial_mtu_ipv6: uint32_t,
    );
    fn picoquic_set_preemptive_repeat_policy(
        quic: *mut picoquic_quic_t,
        do_repeat: ::core::ffi::c_int,
    );
    static mut picoquic_bbr_algorithm: *mut picoquic_congestion_algorithm_t;
    fn picoquic_get_congestion_algorithm(
        alg_name: *const ::core::ffi::c_char,
    ) -> *const picoquic_congestion_algorithm_t;
    fn picoquic_set_default_congestion_algorithm(
        quic: *mut picoquic_quic_t,
        algo: *const picoquic_congestion_algorithm_t,
    );
    fn picoquic_parse_hexa(
        hex_input: *const ::core::ffi::c_char,
        input_length: size_t,
        bin_output: *mut uint8_t,
        output_max: size_t,
    ) -> size_t;
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
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn picoquic_set_binlog(
        quic: *mut picoquic_quic_t,
        binlog_dir: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn picoquic_set_textlog(
        quic: *mut picoquic_quic_t,
        textlog_file: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
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
pub type picoquic_option_enum_t = ::core::ffi::c_uint;
pub const picoquic_option_HELP: picoquic_option_enum_t = 45;
pub const picoquic_option_AddressDiscovery: picoquic_option_enum_t = 44;
pub const picoquic_option_SSLKEYLOG: picoquic_option_enum_t = 43;
pub const picoquic_option_CWIN_MAX: picoquic_option_enum_t = 42;
pub const picoquic_option_BDP_frame: picoquic_option_enum_t = 41;
pub const picoquic_option_No_GSO: picoquic_option_enum_t = 40;
pub const picoquic_option_Version_Upgrade: picoquic_option_enum_t = 39;
pub const picoquic_option_Preemptive_Repeat: picoquic_option_enum_t = 38;
pub const picoquic_option_Performance_Log: picoquic_option_enum_t = 37;
pub const picoquic_option_Socket_buffer_size: picoquic_option_enum_t = 36;
pub const picoquic_option_Token_File_Name: picoquic_option_enum_t = 35;
pub const picoquic_option_Ticket_File_Name: picoquic_option_enum_t = 34;
pub const picoquic_option_LARGE_CLIENT_HELLO: picoquic_option_enum_t = 33;
pub const picoquic_option_Idle_Timeout: picoquic_option_enum_t = 32;
pub const picoquic_option_NO_DISK: picoquic_option_enum_t = 31;
pub const picoquic_option_CNXID_LENGTH: picoquic_option_enum_t = 30;
pub const picoquic_option_FORCE_ZERO_SHARE: picoquic_option_enum_t = 29;
pub const picoquic_option_ROOT_TRUST_FILE: picoquic_option_enum_t = 28;
pub const picoquic_option_ALPN: picoquic_option_enum_t = 27;
pub const picoquic_option_SNI: picoquic_option_enum_t = 26;
pub const picoquic_option_INITIAL_SEND_MTU_IPV6: picoquic_option_enum_t = 25;
pub const picoquic_option_INITIAL_SEND_MTU_IPV4: picoquic_option_enum_t = 24;
pub const picoquic_option_MTU_MAX: picoquic_option_enum_t = 23;
pub const picoquic_option_QLOG_DIR: picoquic_option_enum_t = 22;
pub const picoquic_option_BINLOG_DIR: picoquic_option_enum_t = 21;
pub const picoquic_option_LONG_LOG: picoquic_option_enum_t = 20;
pub const picoquic_option_LOG_FILE: picoquic_option_enum_t = 19;
pub const picoquic_option_INIT_CNXID: picoquic_option_enum_t = 18;
pub const picoquic_option_CIPHER_SUITE: picoquic_option_enum_t = 17;
pub const picoquic_option_DEST_IF: picoquic_option_enum_t = 16;
pub const picoquic_option_MULTIPATH: picoquic_option_enum_t = 15;
pub const picoquic_option_LOSSBIT: picoquic_option_enum_t = 14;
pub const picoquic_option_SPINBIT: picoquic_option_enum_t = 13;
pub const picoquic_option_CC_ALGO: picoquic_option_enum_t = 12;
pub const picoquic_option_SOLUTION_DIR: picoquic_option_enum_t = 11;
pub const picoquic_option_DisablePortBlocking: picoquic_option_enum_t = 10;
pub const picoquic_option_RESET_SEED: picoquic_option_enum_t = 9;
pub const picoquic_option_INITIAL_RANDOM: picoquic_option_enum_t = 8;
pub const picoquic_option_DO_RETRY: picoquic_option_enum_t = 7;
pub const picoquic_option_MAX_CONNECTIONS: picoquic_option_enum_t = 6;
pub const picoquic_option_WWWDIR: picoquic_option_enum_t = 5;
pub const picoquic_option_OUTDIR: picoquic_option_enum_t = 4;
pub const picoquic_option_PROPOSED_VERSION: picoquic_option_enum_t = 3;
pub const picoquic_option_SERVER_PORT: picoquic_option_enum_t = 2;
pub const picoquic_option_KEY: picoquic_option_enum_t = 1;
pub const picoquic_option_CERT: picoquic_option_enum_t = 0;
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
pub type option_table_line_t = st_option_table_line_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_option_table_line_t {
    pub option_num: picoquic_option_enum_t,
    pub option_letter: ::core::ffi::c_char,
    pub option_name: *const ::core::ffi::c_char,
    pub nb_params_required: ::core::ffi::c_int,
    pub param_sample: *const ::core::ffi::c_char,
    pub option_help: *const ::core::ffi::c_char,
}
pub type option_param_t = st_option_param_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_option_param_t {
    pub param: *const ::core::ffi::c_char,
    pub length: size_t,
}
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const PICOQUIC_MAX_PACKET_SIZE: ::core::ffi::c_int = 1536 as ::core::ffi::c_int;
pub const PICOQUIC_AES_128_GCM_SHA256: ::core::ffi::c_int = 0x1301 as ::core::ffi::c_int;
pub const PICOQUIC_AES_256_GCM_SHA384: ::core::ffi::c_int = 0x1302 as ::core::ffi::c_int;
pub const PICOQUIC_CHACHA20_POLY1305_SHA256: ::core::ffi::c_int = 0x1303 as ::core::ffi::c_int;
pub const PICOQUIC_CONNECTION_ID_MAX_SIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
static mut option_table: [option_table_line_t; 44] = [
    st_option_table_line_t {
        option_num: picoquic_option_CERT,
        option_letter: 'c' as i32 as ::core::ffi::c_char,
        option_name: b"cert\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"file\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"cert file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_KEY,
        option_letter: 'k' as i32 as ::core::ffi::c_char,
        option_name: b"key\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"file\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"key file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_SERVER_PORT,
        option_letter: 'p' as i32 as ::core::ffi::c_char,
        option_name: b"port\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"number\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"server port\0".as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_PROPOSED_VERSION,
        option_letter: 'v' as i32 as ::core::ffi::c_char,
        option_name: b"proposed_version\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Version proposed by client, e.g. -v ff000012\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_OUTDIR,
        option_letter: 'o' as i32 as ::core::ffi::c_char,
        option_name: b"outdir\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"folder\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Folder where client writes downloaded files, defaults to current directory.\0"
            .as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_WWWDIR,
        option_letter: 'w' as i32 as ::core::ffi::c_char,
        option_name: b"wwwdir\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"folder\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Folder containing web pages served by server\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_MAX_CONNECTIONS,
        option_letter: 'x' as i32 as ::core::ffi::c_char,
        option_name: b"max_connections\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"number\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Maximum number of concurrent connections, default 256\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_DO_RETRY,
        option_letter: 'r' as i32 as ::core::ffi::c_char,
        option_name: b"do_retry\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 0 as ::core::ffi::c_int,
        param_sample: b"\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Do Retry Request\0".as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_INITIAL_RANDOM,
        option_letter: 'R' as i32 as ::core::ffi::c_char,
        option_name: b"initial_random\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"option\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Randomize packet number spaces: none(0), initial(1, default), all(2).\0"
            .as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_RESET_SEED,
        option_letter: 's' as i32 as ::core::ffi::c_char,
        option_name: b"reset_seed\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"<32 hex chars>\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Reset seed\0".as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_DisablePortBlocking,
        option_letter: 'X' as i32 as ::core::ffi::c_char,
        option_name: b"disable_block\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 0 as ::core::ffi::c_int,
        param_sample: b"\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Disable the check for blocked ports\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_SOLUTION_DIR,
        option_letter: 'S' as i32 as ::core::ffi::c_char,
        option_name: b"solution_dir\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"folder\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Set the path to the source files to find the default files\0"
            .as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_CC_ALGO,
        option_letter: 'G' as i32 as ::core::ffi::c_char,
        option_name: b"cc_algo\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"cc_algorithm\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Use the specified congestion control algorithm: reno, cubic, bbr or fast. Defaults to bbr.\0"
            .as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_SPINBIT,
        option_letter: 'P' as i32 as ::core::ffi::c_char,
        option_name: b"spinbit\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"number\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Set the default spinbit policy\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_LOSSBIT,
        option_letter: 'O' as i32 as ::core::ffi::c_char,
        option_name: b"lossbit\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"number\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Set the default lossbit policy\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_MULTIPATH,
        option_letter: 'M' as i32 as ::core::ffi::c_char,
        option_name: b"multipath\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 0 as ::core::ffi::c_int,
        param_sample: b"\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Enable QUIC multipath extension\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_DEST_IF,
        option_letter: 'e' as i32 as ::core::ffi::c_char,
        option_name: b"dest_if\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"if\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Send on interface (default: -1)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_CIPHER_SUITE,
        option_letter: 'C' as i32 as ::core::ffi::c_char,
        option_name: b"cipher_suite\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"cipher_suite_id\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"specify cipher suite (e.g. -C 20 = chacha20)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_INIT_CNXID,
        option_letter: 'i' as i32 as ::core::ffi::c_char,
        option_name: b"cnxid_params\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"per-text-lb-spec\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"See documentation for LB compatible CID configuration\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_LOG_FILE,
        option_letter: 'l' as i32 as ::core::ffi::c_char,
        option_name: b"text_log\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"file\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Log file, Log to stdout if file = \"-\". No text logging if absent.\0"
            .as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_LONG_LOG,
        option_letter: 'L' as i32 as ::core::ffi::c_char,
        option_name: b"long_log\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 0 as ::core::ffi::c_int,
        param_sample: b"\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Log all packets. If absent, log stops after 100 packets.\0"
            .as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_BINLOG_DIR,
        option_letter: 'b' as i32 as ::core::ffi::c_char,
        option_name: b"binlog_dir\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"folder\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Binary logging to this directory. No binary logging if absent.\0"
            .as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_QLOG_DIR,
        option_letter: 'q' as i32 as ::core::ffi::c_char,
        option_name: b"qlog_dir\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"folder\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Qlog logging to this directory. No qlog logging if absent, but qlogs could be produced using picolog if binary logs are available.\0"
            .as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_MTU_MAX,
        option_letter: 'm' as i32 as ::core::ffi::c_char,
        option_name: b"mtu_max\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"mtu_max\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Largest mtu value that can be tried for discovery.\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_SNI,
        option_letter: 'n' as i32 as ::core::ffi::c_char,
        option_name: b"sni\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"sni\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"sni (default: server name)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_ALPN,
        option_letter: 'a' as i32 as ::core::ffi::c_char,
        option_name: b"alpn\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"alpn\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"alpn (default function of version)\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_ROOT_TRUST_FILE,
        option_letter: 't' as i32 as ::core::ffi::c_char,
        option_name: b"root_trust_file\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"file\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"root trust file\0".as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_FORCE_ZERO_SHARE,
        option_letter: 'z' as i32 as ::core::ffi::c_char,
        option_name: b"force_zero_share\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 0 as ::core::ffi::c_int,
        param_sample: b"\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Set TLS zero share behavior on client, to force HRR\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_CNXID_LENGTH,
        option_letter: 'I' as i32 as ::core::ffi::c_char,
        option_name: b"cnxid_length\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"length\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Length of CNX_ID used by the client, default=8\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_Idle_Timeout,
        option_letter: 'd' as i32 as ::core::ffi::c_char,
        option_name: b"idle_timeout\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"ms\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Duration of idle timeout in milliseconds\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_NO_DISK,
        option_letter: 'D' as i32 as ::core::ffi::c_char,
        option_name: b"no_disk\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 0 as ::core::ffi::c_int,
        param_sample: b"\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"no disk: do not save received files on disk\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_LARGE_CLIENT_HELLO,
        option_letter: 'Q' as i32 as ::core::ffi::c_char,
        option_name: b"large_client_hello\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 0 as ::core::ffi::c_int,
        param_sample: b"\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"send a large client hello in order to test post quantum readiness\0"
            .as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_Ticket_File_Name,
        option_letter: 'T' as i32 as ::core::ffi::c_char,
        option_name: b"ticket_file\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"file\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"File storing the session tickets\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_Token_File_Name,
        option_letter: 'N' as i32 as ::core::ffi::c_char,
        option_name: b"token_file\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"file\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"File storing the new tokens\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_Socket_buffer_size,
        option_letter: 'B' as i32 as ::core::ffi::c_char,
        option_name: b"so_buf_size\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"number\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Set buffer size with SO_SNDBUF SO_RCVBUF\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_Performance_Log,
        option_letter: 'F' as i32 as ::core::ffi::c_char,
        option_name: b"log_file_name\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"file\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Append performance reports to performance log\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_Preemptive_Repeat,
        option_letter: 'V' as i32 as ::core::ffi::c_char,
        option_name: b"preemptive_repeat\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 0 as ::core::ffi::c_int,
        param_sample: b"\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"enable preemptive repeat\0".as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_Version_Upgrade,
        option_letter: 'U' as i32 as ::core::ffi::c_char,
        option_name: b"version_upgrade\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Version upgrade if server agrees, e.g. -U 6b3343cf\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_No_GSO,
        option_letter: '0' as i32 as ::core::ffi::c_char,
        option_name: b"no_gso\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 0 as ::core::ffi::c_int,
        param_sample: b"\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Do not use UDP GSO or equivalent\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_BDP_frame,
        option_letter: 'j' as i32 as ::core::ffi::c_char,
        option_name: b"bdp\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"number\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"use bdp extension frame(1) or don't (0). Default=0\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_CWIN_MAX,
        option_letter: 'W' as i32 as ::core::ffi::c_char,
        option_name: b"cwin_max\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"bytes\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Max value for CWIN. Default=UINT64_MAX\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_SSLKEYLOG,
        option_letter: '8' as i32 as ::core::ffi::c_char,
        option_name: b"sslkeylog\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 0 as ::core::ffi::c_int,
        param_sample: b"\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"Enable SSLKEYLOG\0".as_ptr() as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_AddressDiscovery,
        option_letter: 'J' as i32 as ::core::ffi::c_char,
        option_name: b"addr_disc\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 1 as ::core::ffi::c_int,
        param_sample: b"mode\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"provider (0), receiver (1) or both (2).\0".as_ptr()
            as *const ::core::ffi::c_char,
    },
    st_option_table_line_t {
        option_num: picoquic_option_HELP,
        option_letter: 'h' as i32 as ::core::ffi::c_char,
        option_name: b"help\0".as_ptr() as *const ::core::ffi::c_char,
        nb_params_required: 0 as ::core::ffi::c_int,
        param_sample: b"\0".as_ptr() as *const ::core::ffi::c_char,
        option_help: b"This help message\0".as_ptr() as *const ::core::ffi::c_char,
    },
];
static mut option_table_size: size_t = 0;
unsafe extern "C" fn config_parse_target_version(
    mut v_arg: *const ::core::ffi::c_char,
) -> uint32_t {
    let mut v: uint32_t = 0 as uint32_t;
    let mut x: *const ::core::ffi::c_char = v_arg;
    while *x as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        let mut c: ::core::ffi::c_int = *x as ::core::ffi::c_int;
        if c >= '0' as i32 && c <= '9' as i32 {
            c -= '0' as i32;
        } else if c >= 'a' as i32 && c <= 'f' as i32 {
            c -= 'a' as i32;
            c += 10 as ::core::ffi::c_int;
        } else if c >= 'A' as i32 && c <= 'F' as i32 {
            c -= 'A' as i32;
            c += 10 as ::core::ffi::c_int;
        } else {
            v = 0 as uint32_t;
            break;
        }
        v = v.wrapping_mul(16 as uint32_t);
        v = v.wrapping_add(c as uint32_t);
        x = x.offset(1);
    }
    return v;
}
unsafe extern "C" fn config_set_string_param(
    mut v: *mut *const ::core::ffi::c_char,
    mut params: *const option_param_t,
    mut nb_param: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut p_dup: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !(*v).is_null() {
        free(*v as *mut ::core::ffi::c_void);
        *v = ::core::ptr::null::<::core::ffi::c_char>();
    }
    if !params.is_null()
        && x >= 0 as ::core::ffi::c_int
        && x < nb_param
        && !(*params.offset(x as isize)).param.is_null()
    {
        let mut alloc_length: size_t = (*params.offset(x as isize))
            .length
            .wrapping_add(1 as size_t);
        if (*params.offset(x as isize)).length > 0 as size_t
            && alloc_length > (*params.offset(x as isize)).length
        {
            p_dup = malloc(alloc_length) as *mut ::core::ffi::c_char;
        }
        if !p_dup.is_null() {
            memcpy(
                p_dup as *mut ::core::ffi::c_void,
                (*params.offset(x as isize)).param as *const ::core::ffi::c_void,
                (*params.offset(x as isize)).length,
            );
            *p_dup.offset((*params.offset(x as isize)).length as isize) = 0 as ::core::ffi::c_char;
            *v = p_dup as *const ::core::ffi::c_char;
        } else {
            fprintf(
                stderr,
                b"Cannot allocate %zu characters\n\0".as_ptr() as *const ::core::ffi::c_char,
                (*params.offset(x as isize)).length,
            );
            ret = -(1 as ::core::ffi::c_int);
        }
    } else {
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
unsafe extern "C" fn config_optval_string(
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_max: size_t,
    mut p: *const ::core::ffi::c_char,
    mut p_length: size_t,
) -> *mut ::core::ffi::c_char {
    if p_length.wrapping_add(1 as size_t) > buffer_max {
        p_length = buffer_max.wrapping_sub(1 as size_t);
    }
    memcpy(
        buffer as *mut ::core::ffi::c_void,
        p as *const ::core::ffi::c_void,
        p_length,
    );
    *buffer.offset(p_length as isize) = 0 as ::core::ffi::c_char;
    return buffer;
}
unsafe extern "C" fn config_optval_param_string(
    mut buffer: *mut ::core::ffi::c_char,
    mut buffer_max: size_t,
    mut params: *const option_param_t,
    mut nb_param: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_char {
    if params.is_null() || x < 0 as ::core::ffi::c_int || x >= nb_param {
        *buffer.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
        return buffer;
    } else {
        return config_optval_string(
            buffer,
            buffer_max,
            (*params.offset(x as isize)).param,
            (*params.offset(x as isize)).length,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn config_atoi(
    mut params: *const option_param_t,
    mut nb_param: ::core::ffi::c_int,
    mut x: ::core::ffi::c_int,
    mut ret: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut v: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if params.is_null() || x < 0 as ::core::ffi::c_int || x >= nb_param {
        *ret = -(1 as ::core::ffi::c_int);
    } else {
        let mut i: size_t = 0 as size_t;
        while i < (*params.offset(x as isize)).length {
            let mut c: ::core::ffi::c_int = *(*params.offset(x as isize)).param.offset(i as isize)
                as ::core::ffi::c_int
                - '0' as i32;
            if c < 0 as ::core::ffi::c_int || c > 9 as ::core::ffi::c_int {
                v = -(1 as ::core::ffi::c_int);
                *ret = -(1 as ::core::ffi::c_int);
                break;
            } else {
                v *= 10 as ::core::ffi::c_int;
                v += c;
                i = i.wrapping_add(1);
            }
        }
    }
    return v;
}
unsafe extern "C" fn config_set_option(
    mut option_desc: *mut option_table_line_t,
    mut params: *mut option_param_t,
    mut nb_params: ::core::ffi::c_int,
    mut config: *mut picoquic_quic_config_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut opval_buffer: [::core::ffi::c_char; 256] = [0; 256];
    match (*option_desc).option_num as ::core::ffi::c_uint {
        0 => {
            ret = config_set_string_param(
                &raw mut (*config).server_cert_file,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            );
        }
        1 => {
            ret = config_set_string_param(
                &raw mut (*config).server_key_file,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            );
        }
        2 => {
            (*config).server_port =
                config_atoi(params, nb_params, 0 as ::core::ffi::c_int, &raw mut ret);
            if ret != 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"Invalid port: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
            }
        }
        3 => {
            (*config).proposed_version = config_parse_target_version(config_optval_param_string(
                &raw mut opval_buffer as *mut ::core::ffi::c_char,
                256 as size_t,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            ));
            if (*config).proposed_version <= 0 as uint32_t {
                fprintf(
                    stderr,
                    b"Invalid version: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
                ret = -(1 as ::core::ffi::c_int);
            }
        }
        4 => {
            ret = config_set_string_param(
                &raw mut (*config).out_dir,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            );
        }
        5 => {
            ret = config_set_string_param(
                &raw mut (*config).www_dir,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            );
        }
        6 => {
            let mut v: ::core::ffi::c_int =
                config_atoi(params, nb_params, 0 as ::core::ffi::c_int, &raw mut ret);
            if ret != 0 as ::core::ffi::c_int || v <= 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"Invalid max connections: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
                ret = if ret == 0 as ::core::ffi::c_int {
                    -(1 as ::core::ffi::c_int)
                } else {
                    ret
                };
            } else {
                (*config).nb_connections = v as uint32_t;
            }
        }
        7 => {
            (*config).set_do_retry(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        8 => {
            let mut v_0: ::core::ffi::c_int =
                config_atoi(params, nb_params, 0 as ::core::ffi::c_int, &raw mut ret);
            if ret != 0 as ::core::ffi::c_int
                || v_0 < 0 as ::core::ffi::c_int
                || v_0 > 2 as ::core::ffi::c_int
            {
                fprintf(
                    stderr,
                    b"Invalid initial random value: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
                ret = if ret == 0 as ::core::ffi::c_int {
                    -(1 as ::core::ffi::c_int)
                } else {
                    ret
                };
            } else {
                (*config).initial_random = v_0 as ::core::ffi::c_uint;
            }
        }
        9 => {
            (*config).set_has_reset_seed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            ret = if picoquic_parse_hexa(
                (*params.offset(0 as ::core::ffi::c_int as isize)).param,
                (*params.offset(0 as ::core::ffi::c_int as isize)).length,
                &raw mut (*config).reset_seed as *mut uint8_t,
                ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
            ) == ::core::mem::size_of::<[uint8_t; 16]>() as usize
            {
                0 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            };
            if ret != 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"Invalid reset seed: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
            }
        }
        10 => {
            (*config).set_disable_port_blocking(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        43 => {
            (*config).set_enable_sslkeylog(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        11 => {
            ret = config_set_string_param(
                &raw mut (*config).solution_dir,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            );
        }
        12 => {
            ret = config_set_string_param(
                &raw mut (*config).cc_algo_id,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            );
        }
        13 => {
            let mut v_1: ::core::ffi::c_int =
                config_atoi(params, nb_params, 0 as ::core::ffi::c_int, &raw mut ret);
            if ret != 0 as ::core::ffi::c_int
                || v_1 < 0 as ::core::ffi::c_int
                || v_1 > picoquic_spinbit_on as ::core::ffi::c_int
            {
                fprintf(
                    stderr,
                    b"Invalid spinbit policy: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
                ret = if ret == 0 as ::core::ffi::c_int {
                    -(1 as ::core::ffi::c_int)
                } else {
                    ret
                };
            } else {
                (*config).spinbit_policy = v_1 as picoquic_spinbit_version_enum;
            }
        }
        14 => {
            let mut v_2: ::core::ffi::c_int =
                config_atoi(params, nb_params, 0 as ::core::ffi::c_int, &raw mut ret);
            if ret != 0 as ::core::ffi::c_int
                || v_2 < 0 as ::core::ffi::c_int
                || v_2 > picoquic_lossbit_send_receive as ::core::ffi::c_int
            {
                fprintf(
                    stderr,
                    b"Invalid lossbit policy: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
                ret = if ret == 0 as ::core::ffi::c_int {
                    -(1 as ::core::ffi::c_int)
                } else {
                    ret
                };
            } else {
                (*config).lossbit_policy = v_2 as picoquic_lossbit_version_enum;
            }
        }
        15 => {
            (*config).multipath_option = 1 as ::core::ffi::c_int;
        }
        16 => {
            (*config).dest_if =
                config_atoi(params, nb_params, 0 as ::core::ffi::c_int, &raw mut ret);
        }
        17 => {
            (*config).cipher_suite_id =
                config_atoi(params, nb_params, 0 as ::core::ffi::c_int, &raw mut ret);
        }
        18 => {
            ret = config_set_string_param(
                &raw mut (*config).cnx_id_cbdata,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            );
        }
        19 => {
            ret = config_set_string_param(
                &raw mut (*config).log_file,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            );
        }
        20 => {
            (*config).set_use_long_log(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        21 => {
            ret = config_set_string_param(
                &raw mut (*config).bin_dir,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            );
        }
        22 => {
            ret = config_set_string_param(
                &raw mut (*config).qlog_dir,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            );
        }
        23 => {
            (*config).mtu_max =
                config_atoi(params, nb_params, 0 as ::core::ffi::c_int, &raw mut ret);
            if (*config).mtu_max <= 0 as ::core::ffi::c_int
                || (*config).mtu_max > PICOQUIC_MAX_PACKET_SIZE
            {
                fprintf(
                    stderr,
                    b"Invalid max mtu: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
                ret = -(1 as ::core::ffi::c_int);
            }
        }
        24 => {
            (*config).initial_send_mtu_ipv4 =
                config_atoi(params, nb_params, 0 as ::core::ffi::c_int, &raw mut ret);
            if (*config).initial_send_mtu_ipv4 <= 0 as ::core::ffi::c_int
                || (*config).initial_send_mtu_ipv4 > PICOQUIC_MAX_PACKET_SIZE
            {
                fprintf(
                    stderr,
                    b"Invalid initial send mtu ipv4: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
                ret = -(1 as ::core::ffi::c_int);
            }
        }
        25 => {
            (*config).initial_send_mtu_ipv6 =
                config_atoi(params, nb_params, 0 as ::core::ffi::c_int, &raw mut ret);
            if (*config).initial_send_mtu_ipv6 <= 0 as ::core::ffi::c_int
                || (*config).initial_send_mtu_ipv6 > PICOQUIC_MAX_PACKET_SIZE
            {
                fprintf(
                    stderr,
                    b"Invalid initial send mtu ipv6: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
                ret = -(1 as ::core::ffi::c_int);
            }
        }
        26 => {
            ret = config_set_string_param(
                &raw mut (*config).sni,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            );
        }
        27 => {
            ret = config_set_string_param(
                &raw mut (*config).alpn,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            );
        }
        28 => {
            ret = config_set_string_param(
                &raw mut (*config).root_trust_file,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            );
        }
        29 => {
            (*config).set_force_zero_share(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        30 => {
            (*config).cnx_id_length =
                config_atoi(params, nb_params, 0 as ::core::ffi::c_int, &raw mut ret);
            if (*config).cnx_id_length < 0 as ::core::ffi::c_int
                || (*config).cnx_id_length > PICOQUIC_CONNECTION_ID_MAX_SIZE
            {
                fprintf(
                    stderr,
                    b"Invalid connection id length: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
                ret = -(1 as ::core::ffi::c_int);
            }
        }
        32 => {
            (*config).idle_timeout =
                config_atoi(params, nb_params, 0 as ::core::ffi::c_int, &raw mut ret);
            if (*config).idle_timeout < 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"Invalid idle timer: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
                ret = -(1 as ::core::ffi::c_int);
            }
        }
        31 => {
            (*config).set_no_disk(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        33 => {
            (*config).set_large_client_hello(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        34 => {
            ret = config_set_string_param(
                &raw mut (*config).ticket_file_name,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            );
        }
        35 => {
            ret = config_set_string_param(
                &raw mut (*config).token_file_name,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            );
        }
        36 => {
            (*config).socket_buffer_size =
                config_atoi(params, nb_params, 0 as ::core::ffi::c_int, &raw mut ret);
            if (*config).socket_buffer_size < 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"Invalid socket_buffer_size: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
                ret = -(1 as ::core::ffi::c_int);
            }
        }
        37 => {
            ret = config_set_string_param(
                &raw mut (*config).performance_log,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            );
        }
        38 => {
            (*config).set_do_preemptive_repeat(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        39 => {
            (*config).desired_version = config_parse_target_version(config_optval_param_string(
                &raw mut opval_buffer as *mut ::core::ffi::c_char,
                256 as size_t,
                params,
                nb_params,
                0 as ::core::ffi::c_int,
            ));
            if (*config).desired_version <= 0 as uint32_t {
                fprintf(
                    stderr,
                    b"Invalid version: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
                ret = -(1 as ::core::ffi::c_int);
            }
        }
        40 => {
            (*config).set_do_not_use_gso(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        41 => {
            let mut v_3: ::core::ffi::c_int =
                config_atoi(params, nb_params, 0 as ::core::ffi::c_int, &raw mut ret);
            if ret != 0 as ::core::ffi::c_int
                || v_3 < 0 as ::core::ffi::c_int
                || v_3 > 1 as ::core::ffi::c_int
            {
                fprintf(
                    stderr,
                    b"Invalid bdp option: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
                ret = if ret == 0 as ::core::ffi::c_int {
                    -(1 as ::core::ffi::c_int)
                } else {
                    ret
                };
            } else {
                (*config).bdp_frame_option = v_3;
            }
        }
        42 => {
            let mut v_4: ::core::ffi::c_int =
                config_atoi(params, nb_params, 0 as ::core::ffi::c_int, &raw mut ret);
            if ret != 0 as ::core::ffi::c_int || v_4 < 0 as ::core::ffi::c_int {
                fprintf(
                    stderr,
                    b"Invalid cwin max option: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
                ret = if ret == 0 as ::core::ffi::c_int {
                    -(1 as ::core::ffi::c_int)
                } else {
                    ret
                };
            } else {
                (*config).cwin_max = (if v_4 == 0 as ::core::ffi::c_int {
                    UINT64_MAX
                } else {
                    v_4 as ::core::ffi::c_ulong
                }) as uint64_t;
            }
        }
        44 => {
            let mut v_5: ::core::ffi::c_int =
                config_atoi(params, nb_params, 0 as ::core::ffi::c_int, &raw mut ret);
            if ret != 0 as ::core::ffi::c_int
                || v_5 < 0 as ::core::ffi::c_int
                || v_5 > 2 as ::core::ffi::c_int
            {
                fprintf(
                    stderr,
                    b"Invalid address discovery option: %s\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    config_optval_param_string(
                        &raw mut opval_buffer as *mut ::core::ffi::c_char,
                        256 as size_t,
                        params,
                        nb_params,
                        0 as ::core::ffi::c_int,
                    ),
                );
                ret = if ret == 0 as ::core::ffi::c_int {
                    -(1 as ::core::ffi::c_int)
                } else {
                    ret
                };
            } else {
                (*config).address_discovery_mode = v_5 + 1 as ::core::ffi::c_int;
            }
        }
        45 => {
            ret = -(1 as ::core::ffi::c_int);
        }
        _ => {
            ret = -(1 as ::core::ffi::c_int);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_config_option_letters(
    mut option_string: *mut ::core::ffi::c_char,
    mut string_max: size_t,
    mut string_length: *mut size_t,
) -> ::core::ffi::c_int {
    let mut l: size_t = 0 as size_t;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0 as size_t;
    while l.wrapping_add(1 as size_t) < string_max && i < option_table_size {
        let c2rust_fresh0 = l;
        l = l.wrapping_add(1);
        *option_string.offset(c2rust_fresh0 as isize) = option_table[i as usize].option_letter;
        if option_table[i as usize].nb_params_required > 0 as ::core::ffi::c_int {
            if l.wrapping_add(1 as size_t) < string_max {
                let c2rust_fresh1 = l;
                l = l.wrapping_add(1);
                *option_string.offset(c2rust_fresh1 as isize) = ':' as i32 as ::core::ffi::c_char;
            } else {
                l = l.wrapping_sub(1);
                ret = -(1 as ::core::ffi::c_int);
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    *option_string.offset(l as isize) = 0 as ::core::ffi::c_char;
    if !string_length.is_null() {
        *string_length = l;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_config_usage_file(mut F: *mut FILE) {
    fprintf(
        F,
        b"Picoquic options:\n\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i: size_t = 0 as size_t;
    while i < option_table_size {
        let mut spacer: size_t = strlen(option_table[i as usize].param_sample);
        fprintf(
            F,
            b"  -%c %s\0".as_ptr() as *const ::core::ffi::c_char,
            option_table[i as usize].option_letter as ::core::ffi::c_int,
            option_table[i as usize].param_sample,
        );
        loop {
            let c2rust_fresh2 = spacer;
            spacer = spacer.wrapping_add(1);
            if !(c2rust_fresh2 < 12 as size_t) {
                break;
            }
            putc(' ' as i32, F);
        }
        fprintf(
            F,
            b" %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            option_table[i as usize].option_help,
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_config_usage() {
    picoquic_config_usage_file(stderr);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_config_set_option(
    mut config: *mut picoquic_quic_config_t,
    mut option_num: picoquic_option_enum_t,
    mut opt_val: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut option_desc: *mut option_table_line_t = ::core::ptr::null_mut::<option_table_line_t>();
    let mut params: [option_param_t; 1] = [st_option_param_t {
        param: ::core::ptr::null::<::core::ffi::c_char>(),
        length: 0,
    }; 1];
    let mut nb_params: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0 as size_t;
    while i < option_table_size {
        if option_table[i as usize].option_num as ::core::ffi::c_uint
            == option_num as ::core::ffi::c_uint
        {
            option_desc = (&raw mut option_table as *mut option_table_line_t).offset(i as isize)
                as *mut option_table_line_t;
        }
        i = i.wrapping_add(1);
    }
    if option_desc.is_null() {
        fprintf(
            stderr,
            b"Unknow option number: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            option_num as ::core::ffi::c_uint,
        );
        ret = -(1 as ::core::ffi::c_int);
    } else {
        if !opt_val.is_null() {
            params[0 as ::core::ffi::c_int as usize].param = opt_val;
            params[0 as ::core::ffi::c_int as usize].length = strlen(opt_val);
            nb_params = 1 as ::core::ffi::c_int;
        }
        ret = config_set_option(
            option_desc,
            &raw mut params as *mut option_param_t,
            nb_params,
            config,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_config_get_option_char_index(
    mut opt: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut option_index: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: size_t = 0 as size_t;
    while i < option_table_size {
        if option_table[i as usize].option_letter as ::core::ffi::c_int == opt {
            option_index = i as ::core::ffi::c_int;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    return option_index;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_config_get_option_name_index(
    mut s: *const ::core::ffi::c_char,
    mut l: size_t,
) -> ::core::ffi::c_int {
    let mut option_index: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: size_t = 0 as size_t;
    while i < option_table_size {
        if strncmp(s, option_table[i as usize].option_name, l) == 0 as ::core::ffi::c_int {
            option_index = i as ::core::ffi::c_int;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    return option_index;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_config_get_command_line_option_index(
    mut opt_string: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut option_index: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if *opt_string.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == '-' as i32
        && *opt_string.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
    {
        if *opt_string.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            option_index = picoquic_config_get_option_char_index(
                *opt_string.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
            );
        } else if *opt_string.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == '-' as i32
            && *opt_string.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            let mut opt_name: *const ::core::ffi::c_char =
                opt_string.offset(2 as ::core::ffi::c_int as isize);
            option_index = picoquic_config_get_option_name_index(opt_name, strlen(opt_name));
        }
    }
    return option_index;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_command_line_option_value(
    mut option_index: ::core::ffi::c_int,
    mut opt_string: *const ::core::ffi::c_char,
    mut p_optind: *mut ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
    mut argc: ::core::ffi::c_int,
    mut optarg: *const ::core::ffi::c_char,
    mut config: *mut picoquic_quic_config_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut params: [option_param_t; 5] = [st_option_param_t {
        param: ::core::ptr::null::<::core::ffi::c_char>(),
        length: 0,
    }; 5];
    let mut nb_params: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if option_table[option_index as usize].nb_params_required > 0 as ::core::ffi::c_int {
        params[0 as ::core::ffi::c_int as usize].param = optarg;
        if optarg.is_null() {
            fprintf(
                stderr,
                b"option %s requires %d arguments\n\0".as_ptr() as *const ::core::ffi::c_char,
                opt_string,
                option_table[option_index as usize].nb_params_required,
            );
            ret = -(1 as ::core::ffi::c_int);
        } else {
            params[0 as ::core::ffi::c_int as usize].length = strlen(optarg);
            nb_params += 1;
            while nb_params < option_table[option_index as usize].nb_params_required {
                if *p_optind + 1 as ::core::ffi::c_int > argc {
                    fprintf(
                        stderr,
                        b"option %s requires %d arguments\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        opt_string,
                        option_table[option_index as usize].nb_params_required,
                    );
                    ret = -(1 as ::core::ffi::c_int);
                    break;
                } else {
                    params[nb_params as usize].param = *argv.offset(*p_optind as isize);
                    params[nb_params as usize].length =
                        strlen(*argv.offset(*p_optind as isize)) as ::core::ffi::c_int as size_t;
                    nb_params += 1;
                    *p_optind += 1 as ::core::ffi::c_int;
                }
            }
        }
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = config_set_option(
            (&raw mut option_table as *mut option_table_line_t).offset(option_index as isize)
                as *mut option_table_line_t,
            &raw mut params as *mut option_param_t,
            nb_params,
            config,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_config_command_line(
    mut opt: ::core::ffi::c_int,
    mut p_optind: *mut ::core::ffi::c_int,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
    mut optarg: *const ::core::ffi::c_char,
    mut config: *mut picoquic_quic_config_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut option_index: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut opt_string: [::core::ffi::c_char; 3] = [
        '-' as i32 as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
        0 as ::core::ffi::c_int as ::core::ffi::c_char,
    ];
    opt_string[1 as ::core::ffi::c_int as usize] = opt as ::core::ffi::c_char;
    option_index = picoquic_config_get_option_char_index(opt);
    if option_index == -(1 as ::core::ffi::c_int) {
        fprintf(
            stderr,
            b"Unknown option: -%c\n\0".as_ptr() as *const ::core::ffi::c_char,
            opt,
        );
        ret = -(1 as ::core::ffi::c_int);
    } else {
        ret = picoquic_get_command_line_option_value(
            option_index,
            &raw mut opt_string as *mut ::core::ffi::c_char,
            p_optind,
            argv,
            argc,
            optarg,
            config,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_config_command_line_ex(
    mut opt_string: *const ::core::ffi::c_char,
    mut p_optind: *mut ::core::ffi::c_int,
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *const ::core::ffi::c_char,
    mut optarg: *const ::core::ffi::c_char,
    mut config: *mut picoquic_quic_config_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut option_index: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    option_index = picoquic_config_get_command_line_option_index(opt_string);
    if option_index == -(1 as ::core::ffi::c_int) {
        fprintf(
            stderr,
            b"Unknown option: %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            opt_string,
        );
    } else {
        ret = picoquic_get_command_line_option_value(
            option_index,
            opt_string,
            p_optind,
            argv,
            argc,
            optarg,
            config,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_and_configure(
    mut config: *mut picoquic_quic_config_t,
    mut default_callback_fn: picoquic_stream_data_cb_fn,
    mut default_callback_ctx: *mut ::core::ffi::c_void,
    mut current_time: uint64_t,
    mut p_simulated_time: *mut uint64_t,
) -> *mut picoquic_quic_t {
    let mut quic: *mut picoquic_quic_t = picoquic_create(
        (*config).nb_connections,
        (*config).server_cert_file,
        (*config).server_key_file,
        (*config).root_trust_file,
        (*config).alpn,
        default_callback_fn,
        default_callback_ctx,
        None,
        NULL,
        if (*config).has_reset_seed() as ::core::ffi::c_int != 0 {
            &raw mut (*config).reset_seed as *mut uint8_t
        } else {
            ::core::ptr::null_mut::<uint8_t>()
        },
        current_time,
        p_simulated_time,
        (*config).ticket_file_name,
        (*config).ticket_encryption_key,
        (*config).ticket_encryption_key_length,
    ) as *mut picoquic_quic_t;
    if !quic.is_null() {
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut cc_algo: *const picoquic_congestion_algorithm_t =
            ::core::ptr::null::<picoquic_congestion_algorithm_t>();
        if (*config).do_retry() != 0 {
            picoquic_set_cookie_mode(quic as *mut picoquic_quic_t, 1 as ::core::ffi::c_int);
        } else {
            picoquic_set_cookie_mode(quic as *mut picoquic_quic_t, 2 as ::core::ffi::c_int);
        }
        if !(*config).cc_algo_id.is_null() {
            cc_algo = picoquic_get_congestion_algorithm((*config).cc_algo_id);
            if cc_algo.is_null() {
                fprintf(
                    stderr,
                    b"Unrecognized congestion algorithm: %s. Using BBR isntead.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*config).cc_algo_id,
                );
            }
        }
        if cc_algo.is_null() {
            cc_algo = picoquic_bbr_algorithm;
        }
        picoquic_set_default_congestion_algorithm(quic as *mut picoquic_quic_t, cc_algo);
        picoquic_set_default_spinbit_policy(quic as *mut picoquic_quic_t, (*config).spinbit_policy);
        picoquic_set_default_lossbit_policy(quic as *mut picoquic_quic_t, (*config).lossbit_policy);
        picoquic_set_default_multipath_option(
            quic as *mut picoquic_quic_t,
            (*config).multipath_option,
        );
        picoquic_set_default_idle_timeout(
            quic as *mut picoquic_quic_t,
            (*config).idle_timeout as uint64_t,
        );
        picoquic_set_cwin_max(quic as *mut picoquic_quic_t, (*config).cwin_max);
        picoquic_set_default_address_discovery_mode(
            quic as *mut picoquic_quic_t,
            (*config).address_discovery_mode,
        );
        if !(*config).token_file_name.is_null() {
            if picoquic_load_retry_tokens(quic as *mut picoquic_quic_t, (*config).token_file_name)
                != 0 as ::core::ffi::c_int
            {
                fprintf(
                    stderr,
                    b"No token file present. Will create one as <%s>.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*config).token_file_name,
                );
            }
        }
        if (*config).force_zero_share() != 0 {
            (*quic).set_client_zero_share(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        if (*config).mtu_max > 0 as ::core::ffi::c_int {
            picoquic_set_mtu_max(quic as *mut picoquic_quic_t, (*config).mtu_max as uint32_t);
        }
        if (*config).initial_send_mtu_ipv4 > 0 as ::core::ffi::c_int
            || (*config).initial_send_mtu_ipv6 > 0 as ::core::ffi::c_int
        {
            picoquic_set_initial_send_mtu(
                quic as *mut picoquic_quic_t,
                (*config).initial_send_mtu_ipv4 as uint32_t,
                (*config).initial_send_mtu_ipv6 as uint32_t,
            );
        }
        if (*config).cnx_id_length != -(1 as ::core::ffi::c_int) {
            if picoquic_set_default_connection_id_length(
                quic as *mut picoquic_quic_t,
                (*config).cnx_id_length as uint8_t,
            ) != 0 as ::core::ffi::c_int
            {
                fprintf(
                    stderr,
                    b"Could not set CNX-ID length #%d.\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*config).cnx_id_length,
                );
            }
        }
        picoquic_set_padding_policy(
            quic as *mut picoquic_quic_t,
            39 as uint32_t,
            128 as uint32_t,
        );
        picoquic_set_binlog(quic, (*config).bin_dir);
        picoquic_set_textlog(quic, (*config).log_file);
        picoquic_set_log_level(
            quic as *mut picoquic_quic_t,
            (*config).use_long_log() as ::core::ffi::c_int,
        );
        picoquic_set_preemptive_repeat_policy(
            quic as *mut picoquic_quic_t,
            (*config).do_preemptive_repeat() as ::core::ffi::c_int,
        );
        picoquic_disable_port_blocking(
            quic as *mut picoquic_quic_t,
            (*config).disable_port_blocking() as ::core::ffi::c_int,
        );
        picoquic_enable_sslkeylog(
            quic as *mut picoquic_quic_t,
            (*config).enable_sslkeylog() as ::core::ffi::c_int,
        );
        if (*config).initial_random >= 0 as ::core::ffi::c_uint
            && (*config).initial_random <= 2 as ::core::ffi::c_uint
        {
            picoquic_set_random_initial(
                quic as *mut picoquic_quic_t,
                (*config).initial_random as ::core::ffi::c_int,
            );
        }
        if (*config).cipher_suite_id != 0 as ::core::ffi::c_int {
            let mut iana_cipher_suite_code: ::core::ffi::c_int = (*config).cipher_suite_id;
            if (*config).cipher_suite_id == 20 as ::core::ffi::c_int {
                iana_cipher_suite_code = PICOQUIC_CHACHA20_POLY1305_SHA256;
            } else if (*config).cipher_suite_id == 128 as ::core::ffi::c_int {
                iana_cipher_suite_code = PICOQUIC_AES_128_GCM_SHA256;
            } else if (*config).cipher_suite_id == 256 as ::core::ffi::c_int {
                iana_cipher_suite_code = PICOQUIC_AES_256_GCM_SHA384;
            }
            if picoquic_set_cipher_suite(quic as *mut picoquic_quic_t, iana_cipher_suite_code)
                != 0 as ::core::ffi::c_int
            {
                fprintf(
                    stderr,
                    b"Could not set cipher suite #%d.\n\0".as_ptr() as *const ::core::ffi::c_char,
                    (*config).cipher_suite_id,
                );
            }
        }
        if (*config).do_retry() as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            picoquic_set_cookie_mode(quic as *mut picoquic_quic_t, 1 as ::core::ffi::c_int);
        } else {
            picoquic_set_cookie_mode(quic as *mut picoquic_quic_t, 2 as ::core::ffi::c_int);
        }
        picoquic_set_default_bdp_frame_option(
            quic as *mut picoquic_quic_t,
            (*config).bdp_frame_option,
        );
        if ret != 0 as ::core::ffi::c_int {
            picoquic_free(quic as *mut picoquic_quic_t);
            quic = ::core::ptr::null_mut::<picoquic_quic_t>();
        }
    }
    return quic;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_config_init(mut config: *mut picoquic_quic_config_t) {
    memset(
        config as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<picoquic_quic_config_t>() as size_t,
    );
    (*config).cnx_id_length = -(1 as ::core::ffi::c_int);
    (*config).nb_connections = 256 as uint32_t;
    (*config).initial_random = 3 as ::core::ffi::c_uint;
    (*config).cwin_max = UINT64_MAX as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_config_clear(mut config: *mut picoquic_quic_config_t) {
    if !(*config).solution_dir.is_null() {
        free((*config).solution_dir as *mut ::core::ffi::c_void);
    }
    if !(*config).server_cert_file.is_null() {
        free((*config).server_cert_file as *mut ::core::ffi::c_void);
    }
    if !(*config).server_key_file.is_null() {
        free((*config).server_key_file as *mut ::core::ffi::c_void);
    }
    if !(*config).log_file.is_null() {
        free((*config).log_file as *mut ::core::ffi::c_void);
    }
    if !(*config).bin_dir.is_null() {
        free((*config).bin_dir as *mut ::core::ffi::c_void);
    }
    if !(*config).qlog_dir.is_null() {
        free((*config).qlog_dir as *mut ::core::ffi::c_void);
    }
    if !(*config).performance_log.is_null() {
        free((*config).performance_log as *mut ::core::ffi::c_void);
    }
    if !(*config).cc_algo_id.is_null() {
        free((*config).cc_algo_id as *mut ::core::ffi::c_void);
    }
    if !(*config).cnx_id_cbdata.is_null() {
        free((*config).cnx_id_cbdata as *mut ::core::ffi::c_void);
    }
    if !(*config).multipath_alt_config.is_null() {
        free((*config).multipath_alt_config as *mut ::core::ffi::c_void);
    }
    if !(*config).www_dir.is_null() {
        free((*config).www_dir as *mut ::core::ffi::c_void);
    }
    if !(*config).ticket_file_name.is_null() {
        free((*config).ticket_file_name as *mut ::core::ffi::c_void);
    }
    if !(*config).token_file_name.is_null() {
        free((*config).token_file_name as *mut ::core::ffi::c_void);
    }
    if !(*config).sni.is_null() {
        free((*config).sni as *mut ::core::ffi::c_void);
    }
    if !(*config).alpn.is_null() {
        free((*config).alpn as *mut ::core::ffi::c_void);
    }
    if !(*config).out_dir.is_null() {
        free((*config).out_dir as *mut ::core::ffi::c_void);
    }
    if !(*config).root_trust_file.is_null() {
        free((*config).root_trust_file as *mut ::core::ffi::c_void);
    }
    picoquic_config_init(config);
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    option_table_size = (::core::mem::size_of::<[option_table_line_t; 44]>() as size_t)
        .wrapping_div(::core::mem::size_of::<option_table_line_t>() as size_t);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];

use ::c2rust_bitfields;
extern "C" {
    pub type st_ptls_iovec_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type st_ptls_buffer_t;
    pub type st_ptls_verify_certificate_t;
    static picoquic_null_connection_id: picoquic_connection_id_t;
    fn picoquic_format_connection_id(
        bytes: *mut uint8_t,
        bytes_max: size_t,
        cnx_id: picoquic_connection_id_t,
    ) -> uint8_t;
    fn picoquic_parse_connection_id(
        bytes: *const uint8_t,
        len: uint8_t,
        cnx_id: *mut picoquic_connection_id_t,
    ) -> uint8_t;
    fn picoquic_compare_connection_id(
        cnx_id1: *const picoquic_connection_id_t,
        cnx_id2: *const picoquic_connection_id_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_frames_uint32_decode(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
        n: *mut uint32_t,
    ) -> *const uint8_t;
    fn picoquic_frames_varint_encode(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        n64: uint64_t,
    ) -> *mut uint8_t;
    fn picoquic_frames_uint16_encode(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        n: uint16_t,
    ) -> *mut uint8_t;
    fn picoquic_frames_uint32_encode(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        n: uint32_t,
    ) -> *mut uint8_t;
    fn picoquic_frames_cid_encode(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        cid: *const picoquic_connection_id_t,
    ) -> *mut uint8_t;
    static picoquic_supported_versions: [picoquic_version_parameters_t; 0];
    static picoquic_nb_supported_versions: size_t;
    fn picoquic_get_version_index(proposed_version: uint32_t) -> ::core::ffi::c_int;
    fn picoquic_connection_error(
        cnx: *mut picoquic_cnx_t,
        local_error: uint64_t,
        frame_type: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_connection_error_ex(
        cnx: *mut picoquic_cnx_t,
        local_error: uint64_t,
        frame_type: uint64_t,
        local_reason: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn picoformat_16(bytes: *mut uint8_t, n16: uint16_t);
    fn picoquic_varint_encode(bytes: *mut uint8_t, max_bytes: size_t, n64: uint64_t) -> size_t;
    fn picoquic_varint_decode(
        bytes: *const uint8_t,
        max_bytes: size_t,
        n64: *mut uint64_t,
    ) -> size_t;
    fn picoquic_update_stream_initial_remote(cnx: *mut picoquic_cnx_t);
    fn picoquic_add_output_streams(
        cnx: *mut picoquic_cnx_t,
        old_limit: uint64_t,
        new_limit: uint64_t,
        is_bidir: ::core::ffi::c_uint,
    );
    fn picoquic_process_version_upgrade(
        cnx: *mut picoquic_cnx_t,
        old_version_index: ::core::ffi::c_int,
        new_version_index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_log_transport_extension(
        cnx: *mut picoquic_cnx_t,
        is_local: ::core::ffi::c_int,
        param_length: size_t,
        params: *mut uint8_t,
    );
    fn picoquic_create_cnxid_reset_secret(
        quic: *mut picoquic_quic_t,
        cnx_id: *mut picoquic_connection_id_t,
        reset_secret: *mut uint8_t,
    ) -> ::core::ffi::c_int;
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
pub type FILE = _IO_FILE;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_version_parameters_t {
    pub version: uint32_t,
    pub version_aead_key_length: size_t,
    pub version_aead_key: *mut uint8_t,
    pub version_retry_key_length: size_t,
    pub version_retry_key: *mut uint8_t,
    pub tls_prefix_label: *mut ::core::ffi::c_char,
    pub tls_traffic_update_label: *mut ::core::ffi::c_char,
    pub packet_type_version: uint32_t,
    pub upgrade_from: *mut uint32_t,
}
pub type picoquic_version_parameters_t = st_picoquic_version_parameters_t;
pub type picoquic_tp_enum = uint64_t;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const PICOQUIC_ERROR_CLASS: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_EXTENSION_BUFFER_TOO_SMALL: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 11 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_PARAMETER_ERROR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION: ::core::ffi::c_int = 0xa as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_VERSION_NEGOTIATION_ERROR: ::core::ffi::c_int =
    0x11 as ::core::ffi::c_int;
pub const PICOQUIC_MAX_PACKET_SIZE: ::core::ffi::c_int = 1536 as ::core::ffi::c_int;
pub const PICOQUIC_RESET_SECRET_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PICOQUIC_CONNECTION_ID_MAX_SIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const PICOQUIC_ENFORCED_INITIAL_MTU: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const PICOQUIC_NB_PATH_DEFAULT: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PICOQUIC_ACK_DELAY_MAX_DEFAULT: ::core::ffi::c_ulonglong =
    25000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_ACK_DELAY_MIN_MAX_VALUE: ::core::ffi::c_ulonglong =
    0xffffff as ::core::ffi::c_ulonglong;
pub const PICOQUIC_MAX_ACK_DELAY_MAX_MS: ::core::ffi::c_ulonglong =
    0x4000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_SEVENTEENTH_INTEROP_VERSION: ::core::ffi::c_uint =
    0xff00001b as ::core::ffi::c_uint;
pub const picoquic_tp_original_connection_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const picoquic_tp_idle_timeout: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const picoquic_tp_stateless_reset_token: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const picoquic_tp_max_packet_size: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const picoquic_tp_initial_max_data: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const picoquic_tp_initial_max_stream_data_bidi_local: ::core::ffi::c_int =
    5 as ::core::ffi::c_int;
pub const picoquic_tp_initial_max_stream_data_bidi_remote: ::core::ffi::c_int =
    6 as ::core::ffi::c_int;
pub const picoquic_tp_initial_max_stream_data_uni: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const picoquic_tp_initial_max_streams_bidi: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const picoquic_tp_initial_max_streams_uni: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const picoquic_tp_ack_delay_exponent: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const picoquic_tp_max_ack_delay: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const picoquic_tp_disable_migration: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const picoquic_tp_server_preferred_address: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const picoquic_tp_active_connection_id_limit: ::core::ffi::c_int = 14 as ::core::ffi::c_int;
pub const picoquic_tp_handshake_connection_id: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const picoquic_tp_retry_connection_id: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const picoquic_tp_max_datagram_frame_size: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const picoquic_tp_test_large_chello: ::core::ffi::c_int = 3127 as ::core::ffi::c_int;
pub const picoquic_tp_enable_loss_bit: ::core::ffi::c_int = 0x1057 as ::core::ffi::c_int;
pub const picoquic_tp_min_ack_delay: ::core::ffi::c_ulonglong =
    0xff04de1b as ::core::ffi::c_ulonglong;
pub const picoquic_tp_enable_time_stamp: ::core::ffi::c_int = 0x7158 as ::core::ffi::c_int;
pub const picoquic_tp_grease_quic_bit: ::core::ffi::c_int = 0x2ab2 as ::core::ffi::c_int;
pub const picoquic_tp_version_negotiation: ::core::ffi::c_int = 0x11 as ::core::ffi::c_int;
pub const picoquic_tp_enable_bdp_frame: ::core::ffi::c_int = 0xebd9 as ::core::ffi::c_int;
pub const picoquic_tp_initial_max_path_id: ::core::ffi::c_ulonglong =
    0xf739bbc1b666d11 as ::core::ffi::c_ulonglong;
pub const picoquic_tp_address_discovery: ::core::ffi::c_uint = 0x9f81a176 as ::core::ffi::c_uint;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn picoquic_transport_param_varint_encode_old(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut n64: uint64_t,
) -> *mut uint8_t {
    if bytes.offset(2 as ::core::ffi::c_int as isize) > bytes_max as *mut uint8_t {
        bytes = ::core::ptr::null_mut::<uint8_t>();
    } else {
        let mut byte_l: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut l: size_t = 0;
        let c2rust_fresh3 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh3 = 0 as uint8_t;
        byte_l = bytes;
        let c2rust_fresh4 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh4 = 0 as uint8_t;
        l = picoquic_varint_encode(
            bytes,
            bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t,
            n64,
        );
        if l == 0 as size_t {
            bytes = ::core::ptr::null_mut::<uint8_t>();
        } else {
            *byte_l = l as uint8_t;
            bytes = bytes.offset(l as isize);
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_transport_param_varint_decode(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut extension_length: uint64_t,
    mut ret: *mut ::core::ffi::c_int,
) -> uint64_t {
    let mut n64: uint64_t = 0 as uint64_t;
    let mut l_v: uint64_t =
        picoquic_varint_decode(bytes, extension_length as size_t, &raw mut n64) as uint64_t;
    if l_v == 0 as uint64_t || l_v != extension_length {
        *ret = picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
            0 as uint64_t,
        );
    }
    return n64;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_transport_param_type_varint_encode_old(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut tp_type: picoquic_tp_enum,
    mut n64: uint64_t,
) -> *mut uint8_t {
    if !bytes.is_null()
        && bytes.offset(2 as ::core::ffi::c_int as isize) <= bytes_max as *mut uint8_t
    {
        picoformat_16(bytes, tp_type as uint16_t);
        bytes = picoquic_transport_param_varint_encode_old(
            bytes.offset(2 as ::core::ffi::c_int as isize),
            bytes_max,
            n64,
        );
    } else {
        bytes = ::core::ptr::null_mut::<uint8_t>();
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_transport_param_varint_encode(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut n64: uint64_t,
) -> *mut uint8_t {
    if bytes.offset(1 as ::core::ffi::c_int as isize) > bytes_max as *mut uint8_t {
        bytes = ::core::ptr::null_mut::<uint8_t>();
    } else {
        let c2rust_fresh0 = bytes;
        bytes = bytes.offset(1);
        let mut byte_l: *mut uint8_t = c2rust_fresh0;
        bytes = picoquic_frames_varint_encode(bytes, bytes_max, n64);
        if !bytes.is_null() {
            *byte_l = (bytes.offset_from(byte_l) as ::core::ffi::c_long - 1 as ::core::ffi::c_long)
                as uint8_t;
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_transport_param_type_varint_encode(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut tp_type: picoquic_tp_enum,
    mut n64: uint64_t,
) -> *mut uint8_t {
    if !bytes.is_null() && {
        bytes = picoquic_frames_varint_encode(bytes, bytes_max, tp_type as uint64_t);
        !bytes.is_null()
    } {
        bytes = picoquic_transport_param_varint_encode(bytes, bytes_max, n64);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_transport_param_type_flag_encode(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut tp_type: picoquic_tp_enum,
) -> *mut uint8_t {
    if !bytes.is_null() && {
        bytes = picoquic_frames_varint_encode(bytes, bytes_max, tp_type as uint64_t);
        !bytes.is_null()
    } {
        bytes = picoquic_frames_varint_encode(bytes, bytes_max, 0 as uint64_t);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_transport_param_cid_encode(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut tp_type: picoquic_tp_enum,
    mut cid: *mut picoquic_connection_id_t,
) -> *mut uint8_t {
    if !bytes.is_null() && {
        bytes = picoquic_frames_varint_encode(bytes, bytes_max, tp_type as uint64_t);
        !bytes.is_null()
    } {
        bytes = picoquic_frames_cid_encode(bytes, bytes_max, cid);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_transport_param_cid_decode(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut extension_length: uint64_t,
    mut cid: *mut picoquic_connection_id_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*cid).id_len = picoquic_parse_connection_id(bytes, extension_length as uint8_t, cid);
    if (*cid).id_len as size_t != extension_length as size_t {
        ret = picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
            0 as uint64_t,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_encode_transport_param_prefered_address_old(
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut prefered_address: *mut picoquic_tp_prefered_address_t,
) -> *mut uint8_t {
    let mut coded_length: uint16_t = (4 as ::core::ffi::c_uint)
        .wrapping_add(2 as ::core::ffi::c_uint)
        .wrapping_add(16 as ::core::ffi::c_uint)
        .wrapping_add(2 as ::core::ffi::c_uint)
        .wrapping_add(1 as ::core::ffi::c_uint)
        .wrapping_add((*prefered_address).connection_id.id_len as ::core::ffi::c_uint)
        .wrapping_add(16 as ::core::ffi::c_uint) as uint16_t;
    if bytes.is_null() || bytes.offset(coded_length as ::core::ffi::c_int as isize) > bytes_max {
        bytes = ::core::ptr::null_mut::<uint8_t>();
    } else {
        picoformat_16(bytes, picoquic_tp_server_preferred_address as uint16_t);
        bytes = bytes.offset(2 as ::core::ffi::c_int as isize);
        picoformat_16(bytes, coded_length);
        bytes = bytes.offset(2 as ::core::ffi::c_int as isize);
        memcpy(
            bytes as *mut ::core::ffi::c_void,
            &raw mut (*prefered_address).ipv4Address as *mut uint8_t as *const ::core::ffi::c_void,
            4 as size_t,
        );
        bytes = bytes.offset(4 as ::core::ffi::c_int as isize);
        picoformat_16(bytes, (*prefered_address).ipv4Port);
        bytes = bytes.offset(2 as ::core::ffi::c_int as isize);
        memcpy(
            bytes as *mut ::core::ffi::c_void,
            &raw mut (*prefered_address).ipv6Address as *mut uint8_t as *const ::core::ffi::c_void,
            16 as size_t,
        );
        bytes = bytes.offset(16 as ::core::ffi::c_int as isize);
        picoformat_16(bytes, (*prefered_address).ipv4Port);
        bytes = bytes.offset(2 as ::core::ffi::c_int as isize);
        let c2rust_fresh5 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh5 = (*prefered_address).connection_id.id_len;
        bytes = bytes.offset(picoquic_format_connection_id(
            bytes,
            bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t,
            (*prefered_address).connection_id,
        ) as ::core::ffi::c_int as isize);
        memcpy(
            bytes as *mut ::core::ffi::c_void,
            &raw mut (*prefered_address).statelessResetToken as *mut uint8_t
                as *const ::core::ffi::c_void,
            16 as size_t,
        );
        bytes = bytes.offset(16 as ::core::ffi::c_int as isize);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_encode_transport_param_prefered_address(
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut prefered_address: *mut picoquic_tp_prefered_address_t,
) -> *mut uint8_t {
    let mut coded_length: uint64_t = ((4 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 16 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int) as uint64_t)
        .wrapping_add((*prefered_address).connection_id.id_len as uint64_t)
        .wrapping_add(16 as ::core::ffi::c_int as uint64_t);
    if !bytes.is_null()
        && {
            bytes = picoquic_frames_varint_encode(
                bytes,
                bytes_max,
                picoquic_tp_server_preferred_address as uint64_t,
            );
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, coded_length);
            !bytes.is_null()
        }
    {
        if bytes.offset(coded_length as isize) > bytes_max {
            bytes = ::core::ptr::null_mut::<uint8_t>();
        } else {
            memcpy(
                bytes as *mut ::core::ffi::c_void,
                &raw mut (*prefered_address).ipv4Address as *mut uint8_t
                    as *const ::core::ffi::c_void,
                4 as size_t,
            );
            bytes = bytes.offset(4 as ::core::ffi::c_int as isize);
            picoformat_16(bytes, (*prefered_address).ipv4Port);
            bytes = bytes.offset(2 as ::core::ffi::c_int as isize);
            memcpy(
                bytes as *mut ::core::ffi::c_void,
                &raw mut (*prefered_address).ipv6Address as *mut uint8_t
                    as *const ::core::ffi::c_void,
                16 as size_t,
            );
            bytes = bytes.offset(16 as ::core::ffi::c_int as isize);
            picoformat_16(bytes, (*prefered_address).ipv4Port);
            bytes = bytes.offset(2 as ::core::ffi::c_int as isize);
            let c2rust_fresh1 = bytes;
            bytes = bytes.offset(1);
            *c2rust_fresh1 = (*prefered_address).connection_id.id_len;
            bytes = bytes.offset(picoquic_format_connection_id(
                bytes,
                bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t,
                (*prefered_address).connection_id,
            ) as ::core::ffi::c_int as isize);
            memcpy(
                bytes as *mut ::core::ffi::c_void,
                &raw mut (*prefered_address).statelessResetToken as *mut uint8_t
                    as *const ::core::ffi::c_void,
                16 as size_t,
            );
            bytes = bytes.offset(16 as ::core::ffi::c_int as isize);
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_transport_param_prefered_address(
    mut bytes: *mut uint8_t,
    mut bytes_max: size_t,
    mut prefered_address: *mut picoquic_tp_prefered_address_t,
) -> size_t {
    let mut byte_index: size_t = 0 as size_t;
    let mut cnx_id_length: uint8_t = 0 as uint8_t;
    let mut minimal_length: size_t = (4 as ::core::ffi::c_uint)
        .wrapping_add(2 as ::core::ffi::c_uint)
        .wrapping_add(16 as ::core::ffi::c_uint)
        .wrapping_add(2 as ::core::ffi::c_uint)
        .wrapping_add(1 as ::core::ffi::c_uint)
        .wrapping_add(16 as ::core::ffi::c_uint) as size_t;
    let mut ret: size_t = 0 as size_t;
    if bytes_max >= minimal_length {
        memcpy(
            &raw mut (*prefered_address).ipv4Address as *mut uint8_t as *mut ::core::ffi::c_void,
            bytes.offset(byte_index as isize) as *const ::core::ffi::c_void,
            4 as size_t,
        );
        byte_index = byte_index.wrapping_add(4 as size_t);
        (*prefered_address).ipv4Port = ((*bytes
            .offset(byte_index as isize)
            .offset(0 as ::core::ffi::c_int as isize)
            as uint16_t as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *bytes
                .offset(byte_index as isize)
                .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int) as uint16_t;
        byte_index = byte_index.wrapping_add(2 as size_t);
        memcpy(
            &raw mut (*prefered_address).ipv6Address as *mut uint8_t as *mut ::core::ffi::c_void,
            bytes.offset(byte_index as isize) as *const ::core::ffi::c_void,
            16 as size_t,
        );
        byte_index = byte_index.wrapping_add(16 as size_t);
        (*prefered_address).ipv6Port = ((*bytes
            .offset(byte_index as isize)
            .offset(0 as ::core::ffi::c_int as isize)
            as uint16_t as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *bytes
                .offset(byte_index as isize)
                .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int) as uint16_t;
        byte_index = byte_index.wrapping_add(2 as size_t);
        let c2rust_fresh2 = byte_index;
        byte_index = byte_index.wrapping_add(1);
        cnx_id_length = *bytes.offset(c2rust_fresh2 as isize);
        if cnx_id_length as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && cnx_id_length as ::core::ffi::c_int <= PICOQUIC_CONNECTION_ID_MAX_SIZE
            && byte_index
                .wrapping_add(cnx_id_length as size_t)
                .wrapping_add(16 as size_t)
                <= bytes_max
            && cnx_id_length as ::core::ffi::c_int
                == picoquic_parse_connection_id(
                    bytes.offset(byte_index as isize),
                    cnx_id_length,
                    &raw mut (*prefered_address).connection_id,
                ) as ::core::ffi::c_int
        {
            byte_index = byte_index.wrapping_add(cnx_id_length as size_t);
            memcpy(
                &raw mut (*prefered_address).statelessResetToken as *mut uint8_t
                    as *mut ::core::ffi::c_void,
                bytes.offset(byte_index as isize) as *const ::core::ffi::c_void,
                16 as size_t,
            );
            byte_index = byte_index.wrapping_add(16 as size_t);
            ret = byte_index;
            (*prefered_address).is_defined = 1 as ::core::ffi::c_int;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_encode_transport_param_version_negotiation(
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut extension_mode: ::core::ffi::c_int,
    mut cnx: *mut picoquic_cnx_t,
) -> *mut uint8_t {
    let mut bytes_len: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    bytes = picoquic_frames_varint_encode(
        bytes,
        bytes_max,
        picoquic_tp_version_negotiation as uint64_t,
    );
    bytes_len = bytes;
    if !bytes.is_null()
        && {
            bytes = picoquic_frames_uint16_encode(bytes, bytes_max, 0 as uint16_t);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_uint32_encode(
                bytes,
                bytes_max,
                (*(&raw const picoquic_supported_versions as *const picoquic_version_parameters_t)
                    .offset((*cnx).version_index as isize))
                .version,
            );
            !bytes.is_null()
        }
    {
        if extension_mode == 0 as ::core::ffi::c_int {
            if (*cnx).desired_version != 0 as uint32_t
                && (*cnx).desired_version
                    != (*(&raw const picoquic_supported_versions
                        as *const picoquic_version_parameters_t)
                        .offset((*cnx).version_index as isize))
                    .version
            {
                bytes = picoquic_frames_uint32_encode(bytes, bytes_max, (*cnx).desired_version);
            }
            if !bytes.is_null() {
                bytes = picoquic_frames_uint32_encode(
                    bytes,
                    bytes_max,
                    (*(&raw const picoquic_supported_versions
                        as *const picoquic_version_parameters_t)
                        .offset((*cnx).version_index as isize))
                    .version,
                );
            }
        } else {
            let mut i: size_t = 0 as size_t;
            while i < picoquic_nb_supported_versions {
                bytes = picoquic_frames_uint32_encode(
                    bytes,
                    bytes_max,
                    (*(&raw const picoquic_supported_versions
                        as *const picoquic_version_parameters_t)
                        .offset(i as isize))
                    .version,
                );
                if bytes.is_null() {
                    break;
                }
                i = i.wrapping_add(1);
            }
        }
    }
    if !bytes.is_null() {
        let mut len: size_t = bytes.offset_from(bytes_len.offset(2 as ::core::ffi::c_int as isize))
            as ::core::ffi::c_long as size_t;
        if len > 0x3fff as size_t {
            bytes = ::core::ptr::null_mut::<uint8_t>();
        } else {
            *bytes_len.offset(0 as ::core::ffi::c_int as isize) =
                ((len >> 8 as ::core::ffi::c_int & 0x3f as size_t) as uint8_t as ::core::ffi::c_int
                    | 0x40 as ::core::ffi::c_int) as uint8_t;
            *bytes_len.offset(1 as ::core::ffi::c_int as isize) = (len & 0xff as size_t) as uint8_t;
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_tp_version_negotiation(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut extension_mode: ::core::ffi::c_int,
    mut envelop_vn: uint32_t,
    mut negotiated_vn: *mut uint32_t,
    mut negotiated_index: *mut ::core::ffi::c_int,
    mut vn_error: *mut uint64_t,
) -> *const uint8_t {
    let mut current: uint32_t = 0;
    *negotiated_vn = 0 as uint32_t;
    *negotiated_index = -(1 as ::core::ffi::c_int);
    *vn_error = 0 as uint64_t;
    bytes = picoquic_frames_uint32_decode(bytes, bytes_max, &raw mut current);
    if bytes.is_null() {
        *vn_error = PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t;
    } else if current != envelop_vn {
        *vn_error = PICOQUIC_TRANSPORT_VERSION_NEGOTIATION_ERROR as uint64_t;
        bytes = ::core::ptr::null::<uint8_t>();
    } else if extension_mode == 0 as ::core::ffi::c_int {
        while bytes < bytes_max {
            let mut proposed: uint32_t = 0;
            bytes = picoquic_frames_uint32_decode(bytes, bytes_max, &raw mut proposed);
            if bytes.is_null() {
                *vn_error = PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t;
                break;
            } else {
                let mut this_rank: ::core::ffi::c_int = picoquic_get_version_index(proposed);
                if !(this_rank >= 0 as ::core::ffi::c_int) {
                    continue;
                }
                *negotiated_vn = proposed;
                *negotiated_index = this_rank;
                break;
            }
        }
    } else {
        while bytes < bytes_max {
            let mut proposed_0: uint32_t = 0;
            bytes = picoquic_frames_uint32_decode(bytes, bytes_max, &raw mut proposed_0);
            if !bytes.is_null() {
                continue;
            }
            *vn_error = PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t;
            break;
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_negotiate_multipath_option(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*cnx).set_is_multipath_enabled(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if (*cnx).remote_parameters.is_multipath_enabled != 0
        && (*cnx).local_parameters.is_multipath_enabled != 0
    {
        (*cnx).set_is_multipath_enabled(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*cnx).max_path_id_acknowledged = (*cnx).local_parameters.initial_max_path_id;
        (*cnx).max_path_id_remote = (*cnx).remote_parameters.initial_max_path_id;
        (*cnx).max_path_id_local = (*cnx).local_parameters.initial_max_path_id;
    } else if (*cnx).client_mode() == 0 {
        (*cnx).local_parameters.is_multipath_enabled = 0 as ::core::ffi::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_transport_extensions(
    mut cnx: *mut picoquic_cnx_t,
    mut extension_mode: ::core::ffi::c_int,
    mut bytes: *mut uint8_t,
    mut bytes_length: size_t,
    mut consumed: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bytes_zero: *mut uint8_t = bytes;
    let mut bytes_max: *mut uint8_t = bytes.offset(bytes_length as isize);
    bytes = picoquic_transport_param_type_varint_encode(
        bytes,
        bytes_max,
        picoquic_tp_initial_max_stream_data_bidi_local as picoquic_tp_enum,
        (*cnx).local_parameters.initial_max_stream_data_bidi_local,
    );
    bytes = picoquic_transport_param_type_varint_encode(
        bytes,
        bytes_max,
        picoquic_tp_initial_max_data as picoquic_tp_enum,
        (*cnx).local_parameters.initial_max_data,
    );
    if (*cnx).local_parameters.initial_max_stream_id_bidir > 0 as uint64_t {
        bytes = picoquic_transport_param_type_varint_encode(
            bytes,
            bytes_max,
            picoquic_tp_initial_max_streams_bidi as picoquic_tp_enum,
            (*cnx).local_parameters.initial_max_stream_id_bidir,
        );
    }
    if (*cnx).local_parameters.max_idle_timeout > 0 as uint64_t {
        bytes = picoquic_transport_param_type_varint_encode(
            bytes,
            bytes_max,
            picoquic_tp_idle_timeout as picoquic_tp_enum,
            (*cnx).local_parameters.max_idle_timeout,
        );
    }
    bytes = picoquic_transport_param_type_varint_encode(
        bytes,
        bytes_max,
        picoquic_tp_max_packet_size as picoquic_tp_enum,
        (*cnx).local_parameters.max_packet_size as uint64_t,
    );
    if (*cnx).local_parameters.ack_delay_exponent as ::core::ffi::c_int != 3 as ::core::ffi::c_int {
        bytes = picoquic_transport_param_type_varint_encode(
            bytes,
            bytes_max,
            picoquic_tp_ack_delay_exponent as picoquic_tp_enum,
            (*cnx).local_parameters.ack_delay_exponent as uint64_t,
        );
    }
    if (*cnx).local_parameters.initial_max_stream_id_unidir > 0 as uint64_t {
        bytes = picoquic_transport_param_type_varint_encode(
            bytes,
            bytes_max,
            picoquic_tp_initial_max_streams_uni as picoquic_tp_enum,
            (*cnx).local_parameters.initial_max_stream_id_unidir,
        );
    }
    if (*cnx).local_parameters.prefered_address.is_defined != 0 {
        bytes = picoquic_encode_transport_param_prefered_address(
            bytes,
            bytes_max,
            &raw mut (*cnx).local_parameters.prefered_address,
        );
    }
    if (*cnx).local_parameters.migration_disabled != 0 as ::core::ffi::c_uint && !bytes.is_null() {
        bytes = picoquic_transport_param_type_flag_encode(
            bytes,
            bytes_max,
            picoquic_tp_disable_migration as picoquic_tp_enum,
        );
    }
    if (*cnx).local_parameters.initial_max_stream_data_bidi_remote > 0 as uint64_t {
        bytes = picoquic_transport_param_type_varint_encode(
            bytes,
            bytes_max,
            picoquic_tp_initial_max_stream_data_bidi_remote as picoquic_tp_enum,
            (*cnx).local_parameters.initial_max_stream_data_bidi_remote,
        );
    }
    if (*cnx).local_parameters.initial_max_stream_data_uni > 0 as uint64_t {
        bytes = picoquic_transport_param_type_varint_encode(
            bytes,
            bytes_max,
            picoquic_tp_initial_max_stream_data_uni as picoquic_tp_enum,
            (*cnx).local_parameters.initial_max_stream_data_uni,
        );
    }
    if (*cnx).local_parameters.active_connection_id_limit > 0 as uint32_t {
        bytes = picoquic_transport_param_type_varint_encode(
            bytes,
            bytes_max,
            picoquic_tp_active_connection_id_limit as picoquic_tp_enum,
            (*cnx).local_parameters.active_connection_id_limit as uint64_t,
        );
    }
    if (*cnx).local_parameters.max_ack_delay as ::core::ffi::c_ulonglong
        != PICOQUIC_ACK_DELAY_MAX_DEFAULT
    {
        bytes = picoquic_transport_param_type_varint_encode(
            bytes,
            bytes_max,
            picoquic_tp_max_ack_delay as picoquic_tp_enum,
            (*cnx)
                .local_parameters
                .max_ack_delay
                .wrapping_add(999 as uint32_t)
                .wrapping_div(1000 as uint32_t) as uint64_t,
        );
    }
    bytes = picoquic_transport_param_cid_encode(
        bytes,
        bytes_max,
        picoquic_tp_handshake_connection_id as picoquic_tp_enum,
        &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_local_cnxid).cnx_id,
    );
    if extension_mode == 1 as ::core::ffi::c_int {
        if (*cnx).original_cnxid.id_len as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            bytes = picoquic_transport_param_cid_encode(
                bytes,
                bytes_max,
                picoquic_tp_original_connection_id as picoquic_tp_enum,
                &raw mut (*cnx).original_cnxid,
            );
            bytes = picoquic_transport_param_cid_encode(
                bytes,
                bytes_max,
                picoquic_tp_retry_connection_id as picoquic_tp_enum,
                &raw mut (*cnx).initial_cnxid,
            );
        } else if (*cnx).is_hcid_verified() != 0 {
            bytes = picoquic_transport_param_cid_encode(
                bytes,
                bytes_max,
                picoquic_tp_original_connection_id as picoquic_tp_enum,
                &raw mut (*cnx).initial_cnxid,
            );
        }
    }
    if extension_mode == 1 as ::core::ffi::c_int {
        if !bytes.is_null()
            && {
                bytes = picoquic_frames_varint_encode(
                    bytes,
                    bytes_max,
                    picoquic_tp_stateless_reset_token as uint64_t,
                );
                !bytes.is_null()
            }
            && {
                bytes = picoquic_frames_varint_encode(
                    bytes,
                    bytes_max,
                    PICOQUIC_RESET_SECRET_SIZE as uint64_t,
                );
                !bytes.is_null()
            }
        {
            if bytes.offset(PICOQUIC_RESET_SECRET_SIZE as isize) < bytes_max {
                picoquic_create_cnxid_reset_secret(
                    (*cnx).quic,
                    &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                        .p_local_cnxid)
                        .cnx_id,
                    bytes as *mut uint8_t,
                );
                bytes = bytes.offset(PICOQUIC_RESET_SECRET_SIZE as isize);
            } else {
                bytes = ::core::ptr::null_mut::<uint8_t>();
            }
        }
    }
    if (*cnx).client_mode() == 0
        && (*cnx).local_parameters.max_datagram_frame_size == 0 as uint32_t
        && (*cnx).remote_parameters.max_datagram_frame_size > 0 as uint32_t
    {
        (*cnx).local_parameters.max_datagram_frame_size = PICOQUIC_MAX_PACKET_SIZE as uint32_t;
    }
    if (*cnx).local_parameters.max_datagram_frame_size > 0 as uint32_t && !bytes.is_null() {
        bytes = picoquic_transport_param_type_varint_encode(
            bytes,
            bytes_max,
            picoquic_tp_max_datagram_frame_size as picoquic_tp_enum,
            (*cnx).local_parameters.max_datagram_frame_size as uint64_t,
        );
    }
    if (*cnx).grease_transport_parameters() != 0 {
        let mut n: ::core::ffi::c_int = 31 as ::core::ffi::c_int
            * ((*cnx).initial_cnxid.id[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                + (*cnx).client_mode() as ::core::ffi::c_int)
            + 27 as ::core::ffi::c_int;
        let mut v: uint64_t = (*cnx).initial_cnxid.id[1 as ::core::ffi::c_int as usize] as uint64_t;
        while n == picoquic_tp_test_large_chello {
            n += 31 as ::core::ffi::c_int;
        }
        v = (v << 8 as ::core::ffi::c_int)
            .wrapping_add((*cnx).initial_cnxid.id[2 as ::core::ffi::c_int as usize] as uint64_t);
        bytes =
            picoquic_transport_param_type_varint_encode(bytes, bytes_max, n as picoquic_tp_enum, v);
    }
    if (*cnx).test_large_chello() as ::core::ffi::c_int != 0
        && !bytes.is_null()
        && {
            bytes = picoquic_frames_varint_encode(
                bytes,
                bytes_max,
                picoquic_tp_test_large_chello as uint64_t,
            );
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, 1200 as uint64_t);
            !bytes.is_null()
        }
    {
        if bytes.offset(1200 as ::core::ffi::c_int as isize) > bytes_max {
            bytes = ::core::ptr::null_mut::<uint8_t>();
        } else {
            memset(
                bytes as *mut ::core::ffi::c_void,
                'Q' as i32,
                1200 as size_t,
            );
            bytes = bytes.offset(1200 as ::core::ffi::c_int as isize);
        }
    }
    if (*cnx).local_parameters.enable_loss_bit > 0 as ::core::ffi::c_int && !bytes.is_null() {
        bytes = picoquic_transport_param_type_varint_encode(
            bytes,
            bytes_max,
            picoquic_tp_enable_loss_bit as picoquic_tp_enum,
            (if (*cnx).local_parameters.enable_loss_bit > 1 as ::core::ffi::c_int {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint64_t,
        );
    }
    if !bytes.is_null() && (*cnx).local_parameters.min_ack_delay > 0 as uint64_t {
        bytes = picoquic_transport_param_type_varint_encode(
            bytes,
            bytes_max,
            picoquic_tp_min_ack_delay as picoquic_tp_enum,
            (*cnx).local_parameters.min_ack_delay,
        );
    }
    if (*cnx).local_parameters.enable_time_stamp > 0 as ::core::ffi::c_int && !bytes.is_null() {
        bytes = picoquic_transport_param_type_varint_encode(
            bytes,
            bytes_max,
            picoquic_tp_enable_time_stamp as picoquic_tp_enum,
            (*cnx).local_parameters.enable_time_stamp as uint64_t,
        );
    }
    if (*cnx).local_parameters.do_grease_quic_bit != 0 && !bytes.is_null() {
        bytes = picoquic_transport_param_type_flag_encode(
            bytes,
            bytes_max,
            picoquic_tp_grease_quic_bit as picoquic_tp_enum,
        );
    }
    if (*cnx).do_version_negotiation() as ::core::ffi::c_int != 0 && !bytes.is_null() {
        bytes = picoquic_encode_transport_param_version_negotiation(
            bytes,
            bytes_max,
            extension_mode,
            cnx,
        );
    }
    if (*cnx).local_parameters.enable_bdp_frame > 0 as ::core::ffi::c_int && !bytes.is_null() {
        bytes = picoquic_transport_param_type_varint_encode(
            bytes,
            bytes_max,
            picoquic_tp_enable_bdp_frame as picoquic_tp_enum,
            (*cnx).local_parameters.enable_bdp_frame as uint64_t,
        );
    }
    if (*cnx).local_parameters.is_multipath_enabled > 0 as ::core::ffi::c_int && !bytes.is_null() {
        bytes = picoquic_transport_param_type_varint_encode(
            bytes,
            bytes_max,
            picoquic_tp_initial_max_path_id as picoquic_tp_enum,
            (*cnx).local_parameters.initial_max_path_id,
        );
    }
    if (*cnx).local_parameters.address_discovery_mode > 0 as ::core::ffi::c_int && !bytes.is_null()
    {
        bytes = picoquic_transport_param_type_varint_encode(
            bytes,
            bytes_max,
            picoquic_tp_address_discovery as picoquic_tp_enum,
            ((*cnx).local_parameters.address_discovery_mode - 1 as ::core::ffi::c_int) as uint64_t,
        );
    }
    if extension_mode == 1 as ::core::ffi::c_int
        && (*cnx).test_large_chello() == 0
        && (*(*cnx).quic).test_large_server_flight() as ::core::ffi::c_int != 0
        && !bytes.is_null()
    {
        let mut available: size_t = bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t;
        let mut pad_length: size_t = if available > 24 as size_t {
            available.wrapping_sub(24 as size_t)
        } else {
            1 as size_t
        };
        bytes = picoquic_frames_varint_encode(
            bytes,
            bytes_max,
            picoquic_tp_test_large_chello as uint64_t,
        );
        if !bytes.is_null() && {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, pad_length as uint64_t);
            !bytes.is_null()
        } {
            if bytes.offset(pad_length as isize) > bytes_max {
                bytes = ::core::ptr::null_mut::<uint8_t>();
            } else {
                memset(bytes as *mut ::core::ffi::c_void, 'Q' as i32, pad_length);
                bytes = bytes.offset(pad_length as isize);
            }
        }
    }
    if bytes.is_null() {
        *consumed = 0 as size_t;
        ret = PICOQUIC_ERROR_EXTENSION_BUFFER_TOO_SMALL;
    } else {
        *consumed = bytes.offset_from(bytes_zero) as ::core::ffi::c_long as size_t;
        picoquic_log_transport_extension(cnx, 1 as ::core::ffi::c_int, *consumed, bytes_zero);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_clear_transport_extensions(mut cnx: *mut picoquic_cnx_t) {
    (*cnx).remote_parameters.initial_max_stream_data_bidi_local = 0 as uint64_t;
    picoquic_update_stream_initial_remote(cnx);
    (*cnx).remote_parameters.initial_max_stream_data_bidi_remote = 0 as uint64_t;
    picoquic_update_stream_initial_remote(cnx);
    (*cnx).remote_parameters.initial_max_stream_data_uni = 0 as uint64_t;
    picoquic_update_stream_initial_remote(cnx);
    (*cnx).remote_parameters.initial_max_data = 0 as uint64_t;
    (*cnx).maxdata_remote = (*cnx).remote_parameters.initial_max_data;
    (*cnx).remote_parameters.initial_max_stream_id_bidir = 0 as uint64_t;
    (*cnx).max_stream_id_bidir_remote = 0 as uint64_t;
    (*cnx).remote_parameters.max_idle_timeout = 0 as uint64_t;
    (*cnx).remote_parameters.max_packet_size = 1500 as uint32_t;
    (*cnx).remote_parameters.ack_delay_exponent = 3 as uint8_t;
    (*cnx).remote_parameters.initial_max_stream_id_unidir = 0 as uint64_t;
    (*cnx).max_stream_id_unidir_remote = 0 as uint64_t;
    (*cnx).remote_parameters.migration_disabled = 0 as ::core::ffi::c_uint;
    (*cnx).remote_parameters.max_ack_delay = PICOQUIC_ACK_DELAY_MAX_DEFAULT as uint32_t;
    (*cnx).remote_parameters.max_datagram_frame_size = 0 as uint32_t;
    (*cnx).remote_parameters.active_connection_id_limit = 0 as uint32_t;
    (*cnx).remote_parameters.enable_loss_bit = 0 as ::core::ffi::c_int;
    (*cnx).remote_parameters.enable_time_stamp = 0 as ::core::ffi::c_int;
    (*cnx).remote_parameters.min_ack_delay = 0 as uint64_t;
    (*cnx).remote_parameters.do_grease_quic_bit = 0 as ::core::ffi::c_int;
    (*cnx).remote_parameters.enable_bdp_frame = 0 as ::core::ffi::c_int;
    (*cnx).remote_parameters.initial_max_path_id = 0 as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_receive_transport_extensions(
    mut cnx: *mut picoquic_cnx_t,
    mut extension_mode: ::core::ffi::c_int,
    mut bytes: *mut uint8_t,
    mut bytes_max: size_t,
    mut consumed: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut byte_index: size_t = 0 as size_t;
    let mut present_flag: uint64_t = 0 as uint64_t;
    let mut original_connection_id: picoquic_connection_id_t = picoquic_null_connection_id;
    let mut handshake_connection_id: picoquic_connection_id_t = picoquic_null_connection_id;
    let mut retry_connection_id: picoquic_connection_id_t = picoquic_null_connection_id;
    (*cnx).set_remote_parameters_received(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    picoquic_clear_transport_extensions(cnx);
    picoquic_log_transport_extension(cnx, 0 as ::core::ffi::c_int, bytes_max, bytes);
    memset(
        &raw mut (*cnx).remote_parameters as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<picoquic_tp_t>() as size_t,
    );
    (*cnx).remote_parameters.ack_delay_exponent = 3 as uint8_t;
    while ret == 0 as ::core::ffi::c_int && byte_index < bytes_max {
        let mut ll_type: size_t = 0 as size_t;
        let mut ll_length: size_t = 0 as size_t;
        let mut extension_type: uint64_t = UINT64_MAX as uint64_t;
        let mut extension_length: uint64_t = 0 as uint64_t;
        if byte_index.wrapping_add(2 as size_t) > bytes_max {
            ret = picoquic_connection_error_ex(
                cnx,
                PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                0 as uint64_t,
                b"TP length\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            ll_type = picoquic_varint_decode(
                bytes.offset(byte_index as isize),
                bytes_max.wrapping_sub(byte_index),
                &raw mut extension_type,
            );
            byte_index = byte_index.wrapping_add(ll_type);
            ll_length = picoquic_varint_decode(
                bytes.offset(byte_index as isize),
                bytes_max.wrapping_sub(byte_index),
                &raw mut extension_length,
            );
            byte_index = byte_index.wrapping_add(ll_length);
            if ll_type == 0 as size_t
                || ll_length == 0 as size_t
                || byte_index.wrapping_add(extension_length as size_t) > bytes_max
            {
                ret = picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                    0 as uint64_t,
                );
            } else {
                if extension_type < 64 as uint64_t {
                    if present_flag as ::core::ffi::c_ulonglong
                        & (1 as ::core::ffi::c_ulonglong) << extension_type
                        != 0 as ::core::ffi::c_ulonglong
                    {
                        ret = picoquic_connection_error_ex(
                            cnx,
                            PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                            0 as uint64_t,
                            b"Malformed TP\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    } else {
                        present_flag = (present_flag as ::core::ffi::c_ulonglong
                            | (1 as ::core::ffi::c_ulonglong) << extension_type)
                            as uint64_t;
                    }
                }
                match extension_type {
                    5 => {
                        (*cnx).remote_parameters.initial_max_stream_data_bidi_local =
                            picoquic_transport_param_varint_decode(
                                cnx,
                                bytes.offset(byte_index as isize),
                                extension_length,
                                &raw mut ret,
                            );
                        picoquic_update_stream_initial_remote(cnx);
                    }
                    6 => {
                        (*cnx).remote_parameters.initial_max_stream_data_bidi_remote =
                            picoquic_transport_param_varint_decode(
                                cnx,
                                bytes.offset(byte_index as isize),
                                extension_length,
                                &raw mut ret,
                            );
                        picoquic_update_stream_initial_remote(cnx);
                    }
                    7 => {
                        (*cnx).remote_parameters.initial_max_stream_data_uni =
                            picoquic_transport_param_varint_decode(
                                cnx,
                                bytes.offset(byte_index as isize),
                                extension_length,
                                &raw mut ret,
                            );
                        picoquic_update_stream_initial_remote(cnx);
                    }
                    4 => {
                        (*cnx).remote_parameters.initial_max_data =
                            picoquic_transport_param_varint_decode(
                                cnx,
                                bytes.offset(byte_index as isize),
                                extension_length,
                                &raw mut ret,
                            );
                        (*cnx).maxdata_remote = (*cnx).remote_parameters.initial_max_data;
                    }
                    8 => {
                        let mut old_limit: uint64_t = (*cnx).max_stream_id_bidir_remote;
                        (*cnx).remote_parameters.initial_max_stream_id_bidir =
                            picoquic_transport_param_varint_decode(
                                cnx,
                                bytes.offset(byte_index as isize),
                                extension_length,
                                &raw mut ret,
                            );
                        (*cnx).max_stream_id_bidir_remote = (if (*cnx)
                            .remote_parameters
                            .initial_max_stream_id_bidir
                            == 0xffffffff as uint64_t
                        {
                            0 as uint64_t
                        } else {
                            (*cnx).remote_parameters.initial_max_stream_id_bidir
                        })
                        .wrapping_sub(1 as ::core::ffi::c_int as uint64_t)
                            << 2 as ::core::ffi::c_int
                            | (0 as ::core::ffi::c_int as uint64_t) << 1 as ::core::ffi::c_int
                            | ((*cnx).client_mode() as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int)
                                as uint64_t;
                        (*cnx).max_stream_data_remote =
                            (*cnx).remote_parameters.initial_max_stream_data_bidi_remote;
                        picoquic_add_output_streams(
                            cnx,
                            old_limit,
                            (*cnx).max_stream_id_bidir_remote,
                            1 as ::core::ffi::c_uint,
                        );
                    }
                    1 => {
                        (*cnx).remote_parameters.max_idle_timeout =
                            picoquic_transport_param_varint_decode(
                                cnx,
                                bytes.offset(byte_index as isize),
                                extension_length,
                                &raw mut ret,
                            );
                    }
                    3 => {
                        let mut max_packet_size: uint64_t = picoquic_transport_param_varint_decode(
                            cnx,
                            bytes.offset(byte_index as isize),
                            extension_length,
                            &raw mut ret,
                        );
                        if ret == 0 as ::core::ffi::c_int {
                            if max_packet_size < PICOQUIC_ENFORCED_INITIAL_MTU as uint64_t
                                || max_packet_size > 65527 as uint64_t
                            {
                                ret = picoquic_connection_error_ex(
                                    cnx,
                                    PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                                    0 as uint64_t,
                                    b"Max packet size TP\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                            } else {
                                (*cnx).remote_parameters.max_packet_size =
                                    max_packet_size as uint32_t;
                            }
                        }
                    }
                    2 => {
                        if extension_mode != 1 as ::core::ffi::c_int {
                            ret = picoquic_connection_error_ex(
                                cnx,
                                PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                                0 as uint64_t,
                                b"Reset token from client\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        } else if extension_length != PICOQUIC_RESET_SECRET_SIZE as uint64_t {
                            ret = picoquic_connection_error_ex(
                                cnx,
                                PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                                0 as uint64_t,
                                b"Reset token TP\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        } else {
                            memcpy(
                                &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                                    .p_remote_cnxid)
                                    .reset_secret as *mut uint8_t
                                    as *mut ::core::ffi::c_void,
                                bytes.offset(byte_index as isize) as *const ::core::ffi::c_void,
                                PICOQUIC_RESET_SECRET_SIZE as size_t,
                            );
                        }
                    }
                    10 => {
                        (*cnx).remote_parameters.ack_delay_exponent =
                            picoquic_transport_param_varint_decode(
                                cnx,
                                bytes.offset(byte_index as isize),
                                extension_length,
                                &raw mut ret,
                            ) as uint8_t;
                    }
                    9 => {
                        let mut old_limit_0: uint64_t = (*cnx).max_stream_id_unidir_remote;
                        (*cnx).remote_parameters.initial_max_stream_id_unidir =
                            picoquic_transport_param_varint_decode(
                                cnx,
                                bytes.offset(byte_index as isize),
                                extension_length,
                                &raw mut ret,
                            );
                        (*cnx).max_stream_id_unidir_remote = (if (*cnx)
                            .remote_parameters
                            .initial_max_stream_id_unidir
                            == 0xffffffff as uint64_t
                        {
                            0 as uint64_t
                        } else {
                            (*cnx).remote_parameters.initial_max_stream_id_unidir
                        })
                        .wrapping_sub(1 as ::core::ffi::c_int as uint64_t)
                            << 2 as ::core::ffi::c_int
                            | (1 as ::core::ffi::c_int as uint64_t) << 1 as ::core::ffi::c_int
                            | ((*cnx).client_mode() as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int)
                                as uint64_t;
                        picoquic_add_output_streams(
                            cnx,
                            old_limit_0,
                            (*cnx).max_stream_id_unidir_remote,
                            0 as ::core::ffi::c_uint,
                        );
                    }
                    13 => {
                        let mut coded_length: uint64_t =
                            picoquic_decode_transport_param_prefered_address(
                                bytes.offset(byte_index as isize),
                                extension_length as size_t,
                                &raw mut (*cnx).remote_parameters.prefered_address,
                            ) as uint64_t;
                        if coded_length != extension_length {
                            ret = picoquic_connection_error_ex(
                                cnx,
                                PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                                0 as uint64_t,
                                b"Preferred address TP\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    12 => {
                        if extension_length != 0 as uint64_t {
                            ret = picoquic_connection_error_ex(
                                cnx,
                                PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                                0 as uint64_t,
                                b"Disable migration TP\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        } else {
                            (*cnx).remote_parameters.migration_disabled = 1 as ::core::ffi::c_uint;
                        }
                    }
                    11 => {
                        (*cnx).remote_parameters.max_ack_delay =
                            (picoquic_transport_param_varint_decode(
                                cnx,
                                bytes.offset(byte_index as isize),
                                extension_length,
                                &raw mut ret,
                            ) as uint32_t)
                                .wrapping_mul(1000 as uint32_t);
                        if (*cnx).remote_parameters.max_ack_delay as ::core::ffi::c_ulonglong
                            > PICOQUIC_MAX_ACK_DELAY_MAX_MS
                                .wrapping_mul(1000 as ::core::ffi::c_ulonglong)
                        {
                            ret = picoquic_connection_error_ex(
                                cnx,
                                PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                                0 as uint64_t,
                                b"Max ack delay TP\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                    }
                    0 => {
                        ret = picoquic_transport_param_cid_decode(
                            cnx,
                            bytes.offset(byte_index as isize),
                            extension_length,
                            &raw mut original_connection_id,
                        );
                    }
                    16 => {
                        ret = picoquic_transport_param_cid_decode(
                            cnx,
                            bytes.offset(byte_index as isize),
                            extension_length,
                            &raw mut retry_connection_id,
                        );
                    }
                    15 => {
                        ret = picoquic_transport_param_cid_decode(
                            cnx,
                            bytes.offset(byte_index as isize),
                            extension_length,
                            &raw mut handshake_connection_id,
                        );
                        if ret == 0 as ::core::ffi::c_int {
                            if picoquic_compare_connection_id(
                                &raw mut (*(**(*cnx)
                                    .path
                                    .offset(0 as ::core::ffi::c_int as isize))
                                .p_remote_cnxid)
                                    .cnx_id,
                                &raw mut handshake_connection_id,
                            ) != 0 as ::core::ffi::c_int
                            {
                                ret = picoquic_connection_error_ex(
                                    cnx,
                                    PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                                    0 as uint64_t,
                                    b"HCID check\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                            } else {
                                (*cnx).set_is_hcid_verified(
                                    1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                );
                            }
                        }
                    }
                    14 => {
                        (*cnx).remote_parameters.active_connection_id_limit =
                            picoquic_transport_param_varint_decode(
                                cnx,
                                bytes.offset(byte_index as isize),
                                extension_length,
                                &raw mut ret,
                            ) as uint32_t;
                    }
                    32 => {
                        (*cnx).remote_parameters.max_datagram_frame_size =
                            picoquic_transport_param_varint_decode(
                                cnx,
                                bytes.offset(byte_index as isize),
                                extension_length,
                                &raw mut ret,
                            ) as uint32_t;
                    }
                    4183 => {
                        let mut enabled: uint64_t = picoquic_transport_param_varint_decode(
                            cnx,
                            bytes.offset(byte_index as isize),
                            extension_length,
                            &raw mut ret,
                        );
                        if ret == 0 as ::core::ffi::c_int {
                            if enabled == 0 as uint64_t {
                                (*cnx).remote_parameters.enable_loss_bit = 1 as ::core::ffi::c_int;
                            } else if enabled == 1 as uint64_t {
                                (*cnx).remote_parameters.enable_loss_bit = 2 as ::core::ffi::c_int;
                            } else {
                                ret = picoquic_connection_error_ex(
                                    cnx,
                                    PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                                    0 as uint64_t,
                                    b"Loss bit TP\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                            }
                        }
                    }
                    4278509083 => {
                        (*cnx).remote_parameters.min_ack_delay =
                            picoquic_transport_param_varint_decode(
                                cnx,
                                bytes.offset(byte_index as isize),
                                extension_length,
                                &raw mut ret,
                            );
                        if ret == 0 as ::core::ffi::c_int
                            && ((*cnx).remote_parameters.min_ack_delay == 0 as uint64_t
                                || (*cnx).remote_parameters.min_ack_delay
                                    as ::core::ffi::c_ulonglong
                                    > PICOQUIC_ACK_DELAY_MIN_MAX_VALUE)
                        {
                            ret = picoquic_connection_error_ex(
                                cnx,
                                PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                                0 as uint64_t,
                                b"Min ack delay TP\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        } else if (*cnx).local_parameters.min_ack_delay > 0 as uint64_t {
                            (*cnx).set_is_ack_frequency_negotiated(
                                1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                            );
                        }
                    }
                    29016 => {
                        let mut tp_time_stamp: uint64_t = picoquic_transport_param_varint_decode(
                            cnx,
                            bytes.offset(byte_index as isize),
                            extension_length,
                            &raw mut ret,
                        );
                        if ret == 0 as ::core::ffi::c_int {
                            if tp_time_stamp < 1 as uint64_t || tp_time_stamp > 3 as uint64_t {
                                ret = picoquic_connection_error(
                                    cnx,
                                    PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                                    0 as uint64_t,
                                );
                            } else {
                                (*cnx).remote_parameters.enable_time_stamp =
                                    tp_time_stamp as ::core::ffi::c_int;
                            }
                        }
                    }
                    10930 => {
                        if extension_length != 0 as uint64_t {
                            ret = picoquic_connection_error_ex(
                                cnx,
                                PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                                0 as uint64_t,
                                b"Grease TP\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        } else {
                            (*cnx).remote_parameters.do_grease_quic_bit = 1 as ::core::ffi::c_int;
                        }
                    }
                    1113404765106498833 => {
                        (*cnx).remote_parameters.is_multipath_enabled = 1 as ::core::ffi::c_int;
                        (*cnx).remote_parameters.initial_max_path_id =
                            picoquic_transport_param_varint_decode(
                                cnx,
                                bytes.offset(byte_index as isize),
                                extension_length,
                                &raw mut ret,
                            );
                    }
                    17 => {
                        let mut error_found: uint64_t = 0;
                        let mut negotiated_vn: uint32_t = 0;
                        let mut negotiated_index: ::core::ffi::c_int = 0;
                        let mut final_0: *const uint8_t = picoquic_process_tp_version_negotiation(
                            bytes.offset(byte_index as isize),
                            bytes
                                .offset(byte_index as isize)
                                .offset(extension_length as isize),
                            extension_mode,
                            (*(&raw const picoquic_supported_versions
                                as *const picoquic_version_parameters_t)
                                .offset((*cnx).version_index as isize))
                            .version,
                            &raw mut negotiated_vn,
                            &raw mut negotiated_index,
                            &raw mut error_found,
                        );
                        if final_0.is_null() {
                            ret = picoquic_connection_error_ex(
                                cnx,
                                error_found,
                                0 as uint64_t,
                                b"V. Negotiation TP\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        } else {
                            (*cnx).set_do_version_negotiation(
                                1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                            );
                            if negotiated_vn != 0 as uint32_t
                                && (*cnx).version_index != negotiated_index
                            {
                                ret = picoquic_process_version_upgrade(
                                    cnx,
                                    (*cnx).version_index,
                                    negotiated_index,
                                );
                            }
                        }
                    }
                    60377 => {
                        let mut enable_bdp: uint64_t = picoquic_transport_param_varint_decode(
                            cnx,
                            bytes.offset(byte_index as isize),
                            extension_length,
                            &raw mut ret,
                        );
                        if ret == 0 as ::core::ffi::c_int {
                            if enable_bdp > 1 as uint64_t {
                                ret = picoquic_connection_error_ex(
                                    cnx,
                                    PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                                    0 as uint64_t,
                                    b"BDP parameter\0".as_ptr() as *const ::core::ffi::c_char,
                                );
                            } else {
                                (*cnx).remote_parameters.enable_bdp_frame =
                                    enable_bdp as ::core::ffi::c_int;
                            }
                        }
                    }
                    2676072822 => {
                        let mut address_discovery_mode: uint64_t =
                            picoquic_transport_param_varint_decode(
                                cnx,
                                bytes.offset(byte_index as isize),
                                extension_length,
                                &raw mut ret,
                            );
                        if ret == 0 as ::core::ffi::c_int {
                            if address_discovery_mode > 2 as uint64_t {
                                ret = picoquic_connection_error_ex(
                                    cnx,
                                    PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                                    0 as uint64_t,
                                    b"Address discovery parameter\0".as_ptr()
                                        as *const ::core::ffi::c_char,
                                );
                            } else {
                                (*cnx).remote_parameters.address_discovery_mode =
                                    address_discovery_mode.wrapping_add(1 as uint64_t)
                                        as ::core::ffi::c_int;
                                (*cnx).set_is_address_discovery_provider(
                                    ((*cnx).remote_parameters.address_discovery_mode
                                        & 2 as ::core::ffi::c_int
                                        != 0 as ::core::ffi::c_int
                                        && (*cnx).local_parameters.address_discovery_mode
                                            & 1 as ::core::ffi::c_int
                                            != 0 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                        as ::core::ffi::c_uint,
                                );
                                (*cnx).set_is_address_discovery_receiver(
                                    ((*cnx).remote_parameters.address_discovery_mode
                                        & 1 as ::core::ffi::c_int
                                        != 0 as ::core::ffi::c_int
                                        && (*cnx).local_parameters.address_discovery_mode
                                            & 2 as ::core::ffi::c_int
                                            != 0 as ::core::ffi::c_int)
                                        as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                        as ::core::ffi::c_uint,
                                );
                            }
                        }
                    }
                    _ => {}
                }
                if ret == 0 as ::core::ffi::c_int {
                    byte_index = byte_index.wrapping_add(extension_length as size_t);
                }
            }
        }
    }
    (*cnx).idle_timeout = ((*cnx).local_parameters.max_idle_timeout as ::core::ffi::c_ulonglong)
        .wrapping_mul(1000 as ::core::ffi::c_ulonglong) as uint64_t;
    if (*cnx).local_parameters.max_idle_timeout == 0 as uint64_t
        || (*cnx).remote_parameters.max_idle_timeout > 0 as uint64_t
            && (*cnx).remote_parameters.max_idle_timeout < (*cnx).local_parameters.max_idle_timeout
    {
        (*cnx).idle_timeout =
            ((*cnx).remote_parameters.max_idle_timeout as ::core::ffi::c_ulonglong)
                .wrapping_mul(1000 as ::core::ffi::c_ulonglong) as uint64_t;
    }
    if (*cnx).idle_timeout == 0 as uint64_t {
        (*cnx).idle_timeout = UINT64_MAX as uint64_t;
    } else if (*cnx).keep_alive_interval != 0 as uint64_t
        && (*cnx).keep_alive_interval > (*cnx).idle_timeout.wrapping_div(2 as uint64_t)
    {
        (*cnx).keep_alive_interval = (*cnx).idle_timeout.wrapping_div(2 as uint64_t);
    }
    if ret == 0 as ::core::ffi::c_int
        && present_flag as ::core::ffi::c_ulonglong
            & (1 as ::core::ffi::c_ulonglong) << picoquic_tp_max_ack_delay
            == 0 as ::core::ffi::c_ulonglong
    {
        (*cnx).remote_parameters.max_ack_delay = PICOQUIC_ACK_DELAY_MAX_DEFAULT as uint32_t;
    }
    if ret == 0 as ::core::ffi::c_int
        && present_flag as ::core::ffi::c_ulonglong
            & (1 as ::core::ffi::c_ulonglong) << picoquic_tp_active_connection_id_limit
            == 0 as ::core::ffi::c_ulonglong
    {
        if (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_local_cnxid)
            .cnx_id
            .id_len as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            (*cnx).remote_parameters.active_connection_id_limit = 0 as uint32_t;
        } else {
            (*cnx).remote_parameters.active_connection_id_limit =
                PICOQUIC_NB_PATH_DEFAULT as uint32_t;
        }
    }
    if ret == 0 as ::core::ffi::c_int
        && extension_mode == 0 as ::core::ffi::c_int
        && (present_flag as ::core::ffi::c_ulonglong
            & (1 as ::core::ffi::c_ulonglong) << picoquic_tp_stateless_reset_token
            != 0 as ::core::ffi::c_ulonglong
            || present_flag as ::core::ffi::c_ulonglong
                & (1 as ::core::ffi::c_ulonglong) << picoquic_tp_server_preferred_address
                != 0 as ::core::ffi::c_ulonglong
            || present_flag as ::core::ffi::c_ulonglong
                & (1 as ::core::ffi::c_ulonglong) << picoquic_tp_original_connection_id
                != 0 as ::core::ffi::c_ulonglong
            || present_flag as ::core::ffi::c_ulonglong
                & (1 as ::core::ffi::c_ulonglong) << picoquic_tp_retry_connection_id
                != 0 as ::core::ffi::c_ulonglong)
    {
        ret = picoquic_connection_error_ex(
            cnx,
            PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
            0 as uint64_t,
            b"T. Param. unexpected on client\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if ret == 0 as ::core::ffi::c_int
        && (*(&raw const picoquic_supported_versions as *const picoquic_version_parameters_t)
            .offset((*cnx).version_index as isize))
        .version
            != PICOQUIC_SEVENTEENTH_INTEROP_VERSION as uint32_t
        && present_flag as ::core::ffi::c_ulonglong
            & (1 as ::core::ffi::c_ulonglong) << picoquic_tp_handshake_connection_id
            == 0 as ::core::ffi::c_ulonglong
    {
        ret = picoquic_connection_error_ex(
            cnx,
            PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
            0 as uint64_t,
            b"HCID missing\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    if ret == 0 as ::core::ffi::c_int && extension_mode == 1 as ::core::ffi::c_int {
        if present_flag as ::core::ffi::c_ulonglong
            & (1 as ::core::ffi::c_ulonglong) << picoquic_tp_handshake_connection_id
            != 0 as ::core::ffi::c_ulonglong
        {
            if (*cnx).original_cnxid.id_len as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
                if present_flag as ::core::ffi::c_ulonglong
                    & (1 as ::core::ffi::c_ulonglong) << picoquic_tp_retry_connection_id
                    == 0 as ::core::ffi::c_ulonglong
                    || present_flag as ::core::ffi::c_ulonglong
                        & (1 as ::core::ffi::c_ulonglong) << picoquic_tp_original_connection_id
                        == 0 as ::core::ffi::c_ulonglong
                    || picoquic_compare_connection_id(
                        &raw mut (*cnx).original_cnxid,
                        &raw mut original_connection_id,
                    ) != 0 as ::core::ffi::c_int
                    || picoquic_compare_connection_id(
                        &raw mut (*cnx).initial_cnxid,
                        &raw mut retry_connection_id,
                    ) != 0 as ::core::ffi::c_int
                {
                    ret = picoquic_connection_error_ex(
                        cnx,
                        PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                        0 as uint64_t,
                        b"OCID verification\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                }
            } else if present_flag as ::core::ffi::c_ulonglong
                & (1 as ::core::ffi::c_ulonglong) << picoquic_tp_retry_connection_id
                != 0 as ::core::ffi::c_ulonglong
                || present_flag as ::core::ffi::c_ulonglong
                    & (1 as ::core::ffi::c_ulonglong) << picoquic_tp_original_connection_id
                    == 0 as ::core::ffi::c_ulonglong
                || picoquic_compare_connection_id(
                    &raw mut (*cnx).initial_cnxid,
                    &raw mut original_connection_id,
                ) != 0 as ::core::ffi::c_int
            {
                ret = picoquic_connection_error_ex(
                    cnx,
                    PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                    0 as uint64_t,
                    b"HCID or no OCID\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        } else if (*(&raw const picoquic_supported_versions
            as *const picoquic_version_parameters_t)
            .offset((*cnx).version_index as isize))
        .version
            == PICOQUIC_SEVENTEENTH_INTEROP_VERSION as uint32_t
        {
            if (*cnx).original_cnxid.id_len as ::core::ffi::c_int != 0 as ::core::ffi::c_int
                && (present_flag as ::core::ffi::c_ulonglong
                    & (1 as ::core::ffi::c_ulonglong) << picoquic_tp_original_connection_id
                    == 0 as ::core::ffi::c_ulonglong
                    || picoquic_compare_connection_id(
                        &raw mut (*cnx).original_cnxid,
                        &raw mut original_connection_id,
                    ) != 0 as ::core::ffi::c_int)
            {
                ret = picoquic_connection_error_ex(
                    cnx,
                    PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                    0 as uint64_t,
                    b"old draft version\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        }
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = picoquic_negotiate_multipath_option(cnx);
    }
    (*cnx).set_is_loss_bit_enabled_outgoing(
        ((*cnx).local_parameters.enable_loss_bit > 1 as ::core::ffi::c_int
            && (*cnx).remote_parameters.enable_loss_bit > 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
    (*cnx).set_is_loss_bit_enabled_incoming(
        ((*cnx).local_parameters.enable_loss_bit > 0 as ::core::ffi::c_int
            && (*cnx).remote_parameters.enable_loss_bit > 1 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
    (*cnx).set_send_receive_bdp_frame(
        ((*cnx).local_parameters.enable_bdp_frame > 0 as ::core::ffi::c_int
            && (*cnx).remote_parameters.enable_bdp_frame > 0 as ::core::ffi::c_int)
            as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
    if (*cnx).client_mode() != 0 {
        (*cnx).set_is_time_stamp_enabled(
            ((*cnx).local_parameters.enable_time_stamp & 1 as ::core::ffi::c_int != 0
                && (*cnx).remote_parameters.enable_time_stamp & 2 as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_uint,
        );
        (*cnx).set_is_time_stamp_sent(
            ((*cnx).local_parameters.enable_time_stamp & 2 as ::core::ffi::c_int != 0
                && (*cnx).remote_parameters.enable_time_stamp & 1 as ::core::ffi::c_int != 0)
                as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_uint,
        );
        (*cnx).set_do_grease_quic_bit(
            ((*cnx).local_parameters.do_grease_quic_bit != 0
                && (*cnx).remote_parameters.do_grease_quic_bit != 0)
                as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_uint,
        );
    } else {
        if (*cnx).remote_parameters.enable_time_stamp != 0 {
            let mut v_local: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if (*cnx).remote_parameters.enable_time_stamp & 1 as ::core::ffi::c_int != 0 {
                v_local |= 2 as ::core::ffi::c_int;
                (*cnx).set_is_time_stamp_sent(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            if (*cnx).remote_parameters.enable_time_stamp & 2 as ::core::ffi::c_int != 0 {
                v_local |= 1 as ::core::ffi::c_int;
                (*cnx).set_is_time_stamp_enabled(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            (*cnx).local_parameters.enable_time_stamp = v_local;
        }
        (*cnx).local_parameters.do_grease_quic_bit =
            ((*cnx).remote_parameters.do_grease_quic_bit != 0
                && (*(*cnx).quic).one_way_grease_quic_bit() == 0) as ::core::ffi::c_int;
        (*cnx).set_do_grease_quic_bit(
            (*cnx).remote_parameters.do_grease_quic_bit as ::core::ffi::c_uint
                as ::core::ffi::c_uint,
        );
    }
    if (*cnx).client_mode() == 0 && (*cnx).is_ack_frequency_negotiated() == 0 {
        (*cnx).local_parameters.min_ack_delay = 0 as uint64_t;
    }
    *consumed = byte_index;
    return ret;
}

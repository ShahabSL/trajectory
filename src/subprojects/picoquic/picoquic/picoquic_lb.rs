use ::c2rust_bitfields;
extern "C" {
    pub type st_ptls_iovec_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type st_ptls_buffer_t;
    pub type st_picoquic_unified_logging_t;
    pub type st_ptls_verify_certificate_t;
    fn picoquic_parse_hexa(
        hex_input: *const ::core::ffi::c_char,
        input_length: size_t,
        bin_output: *mut uint8_t,
        output_max: size_t,
    ) -> size_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
    fn picoquic_aes128_ecb_create(
        is_enc: ::core::ffi::c_int,
        ecb_key: *const ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
    fn picoquic_aes128_ecb_free(v_aesecb: *mut ::core::ffi::c_void);
    fn picoquic_aes128_ecb_encrypt(
        v_aesecb: *mut ::core::ffi::c_void,
        output: *mut uint8_t,
        input: *const uint8_t,
        len: size_t,
    );
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
pub type picoquic_load_balancer_cid_method_enum = ::core::ffi::c_uint;
pub const picoquic_load_balancer_cid_block_cipher: picoquic_load_balancer_cid_method_enum = 2;
pub const picoquic_load_balancer_cid_stream_cipher: picoquic_load_balancer_cid_method_enum = 1;
pub const picoquic_load_balancer_cid_clear: picoquic_load_balancer_cid_method_enum = 0;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_load_balancer_config_t {
    pub method: picoquic_load_balancer_cid_method_enum,
    #[bitfield(name = "rotation_bits", ty = "::core::ffi::c_uint", bits = "0..=1")]
    #[bitfield(
        name = "first_byte_encodes_length",
        ty = "::core::ffi::c_uint",
        bits = "2..=2"
    )]
    pub rotation_bits_first_byte_encodes_length: [u8; 1],
    pub server_id_length: uint8_t,
    pub nonce_length: uint8_t,
    pub connection_id_length: uint8_t,
    pub server_id64: uint64_t,
    pub cid_encryption_key: [uint8_t; 16],
}
pub type picoquic_load_balancer_config_t = st_picoquic_load_balancer_config_t;
pub type picoquic_load_balancer_cid_context_t = st_picoquic_load_balancer_cid_context_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_load_balancer_cid_context_t {
    pub method: picoquic_load_balancer_cid_method_enum,
    #[bitfield(name = "rotation_bits", ty = "::core::ffi::c_uint", bits = "0..=1")]
    #[bitfield(
        name = "first_byte_encodes_length",
        ty = "::core::ffi::c_uint",
        bits = "2..=2"
    )]
    pub rotation_bits_first_byte_encodes_length: [u8; 1],
    pub server_id_length: uint8_t,
    pub nonce_length: uint8_t,
    pub connection_id_length: uint8_t,
    pub server_id64: uint64_t,
    pub server_id: [uint8_t; 16],
    pub cid_encryption_context: *mut ::core::ffi::c_void,
    pub cid_decryption_context: *mut ::core::ffi::c_void,
}
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const PICOQUIC_CONNECTION_ID_MAX_SIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
unsafe extern "C" fn picoquic_lb_compat_cid_generate_first_byte(
    mut quic: *mut picoquic_quic_t,
    mut lb_ctx: *mut picoquic_load_balancer_cid_context_t,
    mut cnx_id_returned: *mut picoquic_connection_id_t,
) {
    if (*lb_ctx).first_byte_encodes_length() != 0 {
        (*cnx_id_returned).id[0 as ::core::ffi::c_int as usize] =
            (((*lb_ctx).rotation_bits() as uint8_t as ::core::ffi::c_int)
                << 6 as ::core::ffi::c_int
                | (*quic).local_cnxid_length as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
                as uint8_t;
    } else {
        (*cnx_id_returned).id[0 as ::core::ffi::c_int as usize] =
            ((*cnx_id_returned).id[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                & 0x3f as ::core::ffi::c_int) as uint8_t;
        (*cnx_id_returned).id[0 as ::core::ffi::c_int as usize] =
            ((*cnx_id_returned).id[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                | ((*lb_ctx).rotation_bits() as uint8_t as ::core::ffi::c_int)
                    << 6 as ::core::ffi::c_int) as uint8_t;
    };
}
unsafe extern "C" fn picoquic_lb_compat_cid_generate_clear(
    mut quic: *mut picoquic_quic_t,
    mut lb_ctx: *mut picoquic_load_balancer_cid_context_t,
    mut cnx_id_returned: *mut picoquic_connection_id_t,
) {
    picoquic_lb_compat_cid_generate_first_byte(quic, lb_ctx, cnx_id_returned);
    memcpy(
        (&raw mut (*cnx_id_returned).id as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        &raw mut (*lb_ctx).server_id as *mut uint8_t as *const ::core::ffi::c_void,
        (*lb_ctx).server_id_length as size_t,
    );
}
unsafe extern "C" fn picoquic_lb_compat_cid_one_pass_stream(
    mut enc_ctx: *mut ::core::ffi::c_void,
    mut nonce: *mut uint8_t,
    mut nonce_length: size_t,
    mut target: *mut uint8_t,
    mut target_length: size_t,
) {
    let mut mask: [uint8_t; 16] = [0; 16];
    memset(
        &raw mut mask as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    memcpy(
        &raw mut mask as *mut uint8_t as *mut ::core::ffi::c_void,
        nonce as *const ::core::ffi::c_void,
        nonce_length,
    );
    picoquic_aes128_ecb_encrypt(
        enc_ctx,
        &raw mut mask as *mut uint8_t,
        &raw mut mask as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    let mut i: size_t = 0 as size_t;
    while i < target_length {
        let ref mut c2rust_fresh0 = *target.offset(i as isize);
        *c2rust_fresh0 = (*c2rust_fresh0 as ::core::ffi::c_int
            ^ mask[i as usize] as ::core::ffi::c_int) as uint8_t;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn picoquic_lb_compat_cid_generate_stream_cipher(
    mut quic: *mut picoquic_quic_t,
    mut lb_ctx: *mut picoquic_load_balancer_cid_context_t,
    mut cnx_id_returned: *mut picoquic_connection_id_t,
) {
    let mut id_offset: size_t =
        (1 as ::core::ffi::c_int as size_t).wrapping_add((*lb_ctx).nonce_length as size_t);
    picoquic_lb_compat_cid_generate_first_byte(quic, lb_ctx, cnx_id_returned);
    memcpy(
        (&raw mut (*cnx_id_returned).id as *mut uint8_t).offset(id_offset as isize)
            as *mut ::core::ffi::c_void,
        &raw mut (*lb_ctx).server_id as *mut uint8_t as *const ::core::ffi::c_void,
        (*lb_ctx).server_id_length as size_t,
    );
    picoquic_lb_compat_cid_one_pass_stream(
        (*lb_ctx).cid_encryption_context,
        (&raw mut (*cnx_id_returned).id as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize),
        (*lb_ctx).nonce_length as size_t,
        (&raw mut (*cnx_id_returned).id as *mut uint8_t).offset(id_offset as isize),
        (*lb_ctx).server_id_length as size_t,
    );
    picoquic_lb_compat_cid_one_pass_stream(
        (*lb_ctx).cid_encryption_context,
        (&raw mut (*cnx_id_returned).id as *mut uint8_t).offset(id_offset as isize),
        (*lb_ctx).server_id_length as size_t,
        (&raw mut (*cnx_id_returned).id as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize),
        (*lb_ctx).nonce_length as size_t,
    );
    picoquic_lb_compat_cid_one_pass_stream(
        (*lb_ctx).cid_encryption_context,
        (&raw mut (*cnx_id_returned).id as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize),
        (*lb_ctx).nonce_length as size_t,
        (&raw mut (*cnx_id_returned).id as *mut uint8_t).offset(id_offset as isize),
        (*lb_ctx).server_id_length as size_t,
    );
}
unsafe extern "C" fn picoquic_lb_compat_cid_generate_block_cipher(
    mut quic: *mut picoquic_quic_t,
    mut lb_ctx: *mut picoquic_load_balancer_cid_context_t,
    mut cnx_id_returned: *mut picoquic_connection_id_t,
) {
    picoquic_lb_compat_cid_generate_first_byte(quic, lb_ctx, cnx_id_returned);
    memcpy(
        (&raw mut (*cnx_id_returned).id as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        &raw mut (*lb_ctx).server_id as *mut uint8_t as *const ::core::ffi::c_void,
        (*lb_ctx).server_id_length as size_t,
    );
    picoquic_aes128_ecb_encrypt(
        (*lb_ctx).cid_encryption_context,
        (&raw mut (*cnx_id_returned).id as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize),
        (&raw mut (*cnx_id_returned).id as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize),
        16 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_lb_compat_cid_generate(
    mut quic: *mut picoquic_quic_t,
    mut cnx_id_local: picoquic_connection_id_t,
    mut cnx_id_remote: picoquic_connection_id_t,
    mut cnx_id_cb_data: *mut ::core::ffi::c_void,
    mut cnx_id_returned: *mut picoquic_connection_id_t,
) {
    let mut lb_ctx: *mut picoquic_load_balancer_cid_context_t =
        cnx_id_cb_data as *mut picoquic_load_balancer_cid_context_t;
    match (*lb_ctx).method as ::core::ffi::c_uint {
        0 => {
            picoquic_lb_compat_cid_generate_clear(quic, lb_ctx, cnx_id_returned);
        }
        1 => {
            picoquic_lb_compat_cid_generate_stream_cipher(quic, lb_ctx, cnx_id_returned);
        }
        2 => {
            picoquic_lb_compat_cid_generate_block_cipher(quic, lb_ctx, cnx_id_returned);
        }
        _ => {}
    };
}
unsafe extern "C" fn picoquic_lb_compat_cid_verify_clear(
    mut quic: *mut picoquic_quic_t,
    mut lb_ctx: *mut picoquic_load_balancer_cid_context_t,
    mut cnx_id: *const picoquic_connection_id_t,
) -> uint64_t {
    let mut s_id64: uint64_t = 0 as uint64_t;
    let mut i: size_t = 0 as size_t;
    while i < (*lb_ctx).server_id_length as size_t {
        s_id64 <<= 8 as ::core::ffi::c_int;
        s_id64 =
            s_id64.wrapping_add((*cnx_id).id[i.wrapping_add(1 as size_t) as usize] as uint64_t);
        i = i.wrapping_add(1);
    }
    return s_id64;
}
unsafe extern "C" fn picoquic_lb_compat_cid_verify_stream_cipher(
    mut quic: *mut picoquic_quic_t,
    mut lb_ctx: *mut picoquic_load_balancer_cid_context_t,
    mut cnx_id: *const picoquic_connection_id_t,
) -> uint64_t {
    let mut id_offset: size_t =
        (1 as ::core::ffi::c_int as size_t).wrapping_add((*lb_ctx).nonce_length as size_t);
    let mut s_id64: uint64_t = 0 as uint64_t;
    let mut target: picoquic_connection_id_t = *cnx_id;
    picoquic_lb_compat_cid_one_pass_stream(
        (*lb_ctx).cid_encryption_context,
        (&raw mut target.id as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize),
        (*lb_ctx).nonce_length as size_t,
        (&raw mut target.id as *mut uint8_t).offset(id_offset as isize),
        (*lb_ctx).server_id_length as size_t,
    );
    picoquic_lb_compat_cid_one_pass_stream(
        (*lb_ctx).cid_encryption_context,
        (&raw mut target.id as *mut uint8_t).offset(id_offset as isize),
        (*lb_ctx).server_id_length as size_t,
        (&raw mut target.id as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize),
        (*lb_ctx).nonce_length as size_t,
    );
    picoquic_lb_compat_cid_one_pass_stream(
        (*lb_ctx).cid_encryption_context,
        (&raw mut target.id as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize),
        (*lb_ctx).nonce_length as size_t,
        (&raw mut target.id as *mut uint8_t).offset(id_offset as isize),
        (*lb_ctx).server_id_length as size_t,
    );
    let mut i: size_t = 0 as size_t;
    while i < (*lb_ctx).server_id_length as size_t {
        s_id64 <<= 8 as ::core::ffi::c_int;
        s_id64 = s_id64.wrapping_add(target.id[id_offset.wrapping_add(i) as usize] as uint64_t);
        i = i.wrapping_add(1);
    }
    return s_id64;
}
unsafe extern "C" fn picoquic_lb_compat_cid_verify_block_cipher(
    mut quic: *mut picoquic_quic_t,
    mut lb_ctx: *mut picoquic_load_balancer_cid_context_t,
    mut cnx_id: *const picoquic_connection_id_t,
) -> uint64_t {
    let mut decoded: [uint8_t; 16] = [0; 16];
    let mut s_id64: uint64_t = 0 as uint64_t;
    picoquic_aes128_ecb_encrypt(
        (*lb_ctx).cid_decryption_context,
        &raw mut decoded as *mut uint8_t,
        (&raw const (*cnx_id).id as *const uint8_t).offset(1 as ::core::ffi::c_int as isize),
        16 as size_t,
    );
    if s_id64 == 0 as uint64_t {
        let mut i: size_t = 0 as size_t;
        while i < (*lb_ctx).server_id_length as size_t {
            s_id64 <<= 8 as ::core::ffi::c_int;
            s_id64 = s_id64.wrapping_add(decoded[i as usize] as uint64_t);
            i = i.wrapping_add(1);
        }
    }
    return s_id64;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_lb_compat_cid_verify(
    mut quic: *mut picoquic_quic_t,
    mut cnx_id_cb_data: *mut ::core::ffi::c_void,
    mut cnx_id: *const picoquic_connection_id_t,
) -> uint64_t {
    let mut lb_ctx: *mut picoquic_load_balancer_cid_context_t =
        cnx_id_cb_data as *mut picoquic_load_balancer_cid_context_t;
    let mut server_id64: uint64_t = 0;
    if (*cnx_id).id_len as ::core::ffi::c_int
        != (*lb_ctx).connection_id_length as ::core::ffi::c_int
    {
        server_id64 = UINT64_MAX as uint64_t;
    } else {
        match (*lb_ctx).method as ::core::ffi::c_uint {
            0 => {
                server_id64 = picoquic_lb_compat_cid_verify_clear(quic, lb_ctx, cnx_id);
            }
            1 => {
                server_id64 = picoquic_lb_compat_cid_verify_stream_cipher(quic, lb_ctx, cnx_id);
            }
            2 => {
                server_id64 = picoquic_lb_compat_cid_verify_block_cipher(quic, lb_ctx, cnx_id);
            }
            _ => {
                server_id64 = UINT64_MAX as uint64_t;
            }
        }
    }
    return server_id64;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_lb_compat_cid_config_parse(
    mut lb_config: *mut picoquic_load_balancer_config_t,
    mut txt: *const ::core::ffi::c_char,
    mut txt_length: size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut parsed: size_t = 0 as size_t;
    let mut s_id_len: size_t = 0;
    let mut cid_len: size_t = 0 as size_t;
    let mut nonce_len: size_t = 0 as size_t;
    memset(
        lb_config as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<picoquic_load_balancer_config_t>() as size_t,
    );
    if txt_length < 4 as size_t {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        if *txt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int >= '0' as i32
            && *txt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int <= '3' as i32
        {
            (*lb_config).set_rotation_bits(
                (*txt.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int - '0' as i32)
                    as ::core::ffi::c_uint as ::core::ffi::c_uint,
            );
        } else {
            ret = -(1 as ::core::ffi::c_int);
        }
        if *txt.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'Y' as i32
            || *txt.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int == 'y' as i32
        {
            (*lb_config)
                .set_first_byte_encodes_length(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        } else if *txt.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 'N' as i32
            && *txt.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int != 'n' as i32
        {
            ret = -(1 as ::core::ffi::c_int);
        }
        parsed = 2 as size_t;
        while parsed < txt_length
            && *txt.offset(parsed as isize) as ::core::ffi::c_int >= '0' as i32
            && *txt.offset(parsed as isize) as ::core::ffi::c_int <= '9' as i32
        {
            cid_len = cid_len.wrapping_mul(10 as size_t);
            cid_len = cid_len.wrapping_add(
                (*txt.offset(parsed as isize) as ::core::ffi::c_int - '0' as i32) as size_t,
            );
            parsed = parsed.wrapping_add(1);
            if cid_len < 256 as size_t {
                (*lb_config).connection_id_length = cid_len as uint8_t;
            } else {
                ret = -(1 as ::core::ffi::c_int);
                break;
            }
        }
        if parsed >= txt_length {
            ret = -(1 as ::core::ffi::c_int);
        } else if ret == 0 as ::core::ffi::c_int {
            let mut c: ::core::ffi::c_char = *txt.offset(parsed as isize);
            parsed = parsed.wrapping_add(1);
            match c as ::core::ffi::c_int {
                99 | 67 => {
                    (*lb_config).method = picoquic_load_balancer_cid_clear;
                }
                115 | 83 => {
                    (*lb_config).method = picoquic_load_balancer_cid_stream_cipher;
                    while parsed < txt_length
                        && *txt.offset(parsed as isize) as ::core::ffi::c_int >= '0' as i32
                        && *txt.offset(parsed as isize) as ::core::ffi::c_int <= '9' as i32
                    {
                        nonce_len = nonce_len.wrapping_mul(10 as size_t);
                        nonce_len = nonce_len.wrapping_add(
                            (*txt.offset(parsed as isize) as ::core::ffi::c_int - '0' as i32)
                                as size_t,
                        );
                        parsed = parsed.wrapping_add(1);
                        if nonce_len < 256 as size_t {
                            (*lb_config).nonce_length = nonce_len as uint8_t;
                        } else {
                            ret = -(1 as ::core::ffi::c_int);
                            break;
                        }
                    }
                }
                98 | 66 => {
                    (*lb_config).method = picoquic_load_balancer_cid_block_cipher;
                }
                _ => {
                    ret = -(1 as ::core::ffi::c_int);
                }
            }
        }
        if *txt.offset(parsed as isize) as ::core::ffi::c_int == '-' as i32 {
            parsed = parsed.wrapping_add(1);
        } else {
            ret = -(1 as ::core::ffi::c_int);
        }
    }
    if txt_length <= parsed {
        ret = -(1 as ::core::ffi::c_int);
    } else if ret == 0 as ::core::ffi::c_int {
        let mut s_id_bin: [uint8_t; 8] = [0; 8];
        let mut hex_length: size_t = 0 as size_t;
        while parsed.wrapping_add(hex_length) < txt_length
            && *txt.offset(parsed.wrapping_add(hex_length) as isize) as ::core::ffi::c_int
                != '-' as i32
        {
            hex_length = hex_length.wrapping_add(1);
        }
        s_id_len = picoquic_parse_hexa(
            txt.offset(parsed as isize),
            hex_length,
            &raw mut s_id_bin as *mut uint8_t,
            8 as size_t,
        );
        if s_id_len == 0 as size_t || s_id_len > 255 as size_t {
            ret = 1 as ::core::ffi::c_int;
        } else {
            (*lb_config).server_id_length = s_id_len as uint8_t;
            let mut i: size_t = 0 as size_t;
            while i < s_id_len {
                (*lb_config).server_id64 <<= 8 as ::core::ffi::c_int;
                (*lb_config).server_id64 |= s_id_bin[i as usize] as uint64_t;
                i = i.wrapping_add(1);
            }
        }
        parsed = parsed.wrapping_add((2 as size_t).wrapping_mul(s_id_len));
    }
    if ret == 0 as ::core::ffi::c_int
        && ((*lb_config).method as ::core::ffi::c_uint
            == picoquic_load_balancer_cid_stream_cipher as ::core::ffi::c_int
                as ::core::ffi::c_uint
            || (*lb_config).method as ::core::ffi::c_uint
                == picoquic_load_balancer_cid_block_cipher as ::core::ffi::c_int
                    as ::core::ffi::c_uint)
    {
        if *txt.offset(parsed as isize) as ::core::ffi::c_int == '-' as i32 {
            parsed = parsed.wrapping_add(1);
        } else {
            ret = -(1 as ::core::ffi::c_int);
        }
        if ret == 0 as ::core::ffi::c_int {
            if txt_length < parsed.wrapping_add(32 as size_t) {
                ret = -(1 as ::core::ffi::c_int);
            } else {
                let mut key_length: size_t = picoquic_parse_hexa(
                    txt.offset(parsed as isize),
                    txt_length.wrapping_sub(parsed),
                    &raw mut (*lb_config).cid_encryption_key as *mut uint8_t,
                    16 as size_t,
                );
                if key_length != 16 as size_t {
                    ret = -(1 as ::core::ffi::c_int);
                }
                parsed = parsed.wrapping_add((2 as size_t).wrapping_mul(key_length));
            }
        }
    }
    if ret == 0 as ::core::ffi::c_int && parsed != txt_length {
        ret = -(1 as ::core::ffi::c_int);
    }
    if ret == 0 as ::core::ffi::c_int
        && (*lb_config).connection_id_length as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        let mut min_length: size_t = (1 as ::core::ffi::c_int
            + (*lb_config).server_id_length as ::core::ffi::c_int
            + (*lb_config).nonce_length as ::core::ffi::c_int)
            as size_t;
        if ((*lb_config).connection_id_length as size_t) < min_length
            || (*lb_config).method as ::core::ffi::c_uint
                == picoquic_load_balancer_cid_block_cipher as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                && ((*lb_config).connection_id_length as ::core::ffi::c_int)
                    < 17 as ::core::ffi::c_int
        {
            ret = -(1 as ::core::ffi::c_int);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_lb_compat_cid_config(
    mut quic: *mut picoquic_quic_t,
    mut lb_config: *mut picoquic_load_balancer_config_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(*quic).cnx_list.is_null()
        && (*quic).local_cnxid_length as ::core::ffi::c_int
            != (*lb_config).connection_id_length as ::core::ffi::c_int
    {
        ret = -(1 as ::core::ffi::c_int);
    } else if (*quic).cnx_id_callback_fn.is_some() && !(*quic).cnx_id_callback_ctx.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        if (*lb_config).connection_id_length as ::core::ffi::c_int > PICOQUIC_CONNECTION_ID_MAX_SIZE
        {
            ret = -(1 as ::core::ffi::c_int);
        } else {
            match (*lb_config).method as ::core::ffi::c_uint {
                0 => {
                    if (*lb_config).server_id_length as ::core::ffi::c_int + 1 as ::core::ffi::c_int
                        > (*lb_config).connection_id_length as ::core::ffi::c_int
                    {
                        ret = -(1 as ::core::ffi::c_int);
                    }
                }
                1 => {
                    if ((*lb_config).nonce_length as ::core::ffi::c_int) < 8 as ::core::ffi::c_int
                        || (*lb_config).nonce_length as ::core::ffi::c_int
                            > 16 as ::core::ffi::c_int
                        || (*lb_config).nonce_length as ::core::ffi::c_int
                            + (*lb_config).server_id_length as ::core::ffi::c_int
                            + 1 as ::core::ffi::c_int
                            > (*lb_config).connection_id_length as ::core::ffi::c_int
                    {
                        ret = -(1 as ::core::ffi::c_int);
                    }
                }
                2 => {
                    if ((*lb_config).connection_id_length as ::core::ffi::c_int)
                        < 17 as ::core::ffi::c_int
                        || (*lb_config).server_id_length as ::core::ffi::c_int
                            > 15 as ::core::ffi::c_int
                    {
                        ret = -(1 as ::core::ffi::c_int);
                    }
                }
                _ => {
                    ret = -(1 as ::core::ffi::c_int);
                }
            }
        }
        if ret == 0 as ::core::ffi::c_int {
            let mut lb_ctx: *mut picoquic_load_balancer_cid_context_t =
                malloc(::core::mem::size_of::<picoquic_load_balancer_cid_context_t>() as size_t)
                    as *mut picoquic_load_balancer_cid_context_t;
            if lb_ctx.is_null() {
                ret = -(1 as ::core::ffi::c_int);
            } else {
                let mut s_id64: uint64_t = (*lb_config).server_id64;
                memset(
                    lb_ctx as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<picoquic_load_balancer_cid_context_t>() as size_t,
                );
                (*lb_ctx).method = (*lb_config).method;
                (*lb_ctx).set_rotation_bits((*lb_config).rotation_bits() as ::core::ffi::c_uint);
                (*lb_ctx).set_first_byte_encodes_length(
                    (*lb_config).first_byte_encodes_length() as ::core::ffi::c_uint
                );
                (*lb_ctx).server_id_length = (*lb_config).server_id_length;
                (*lb_ctx).nonce_length = (*lb_config).nonce_length;
                (*lb_ctx).connection_id_length = (*lb_config).connection_id_length;
                (*lb_ctx).server_id64 = (*lb_config).server_id64;
                (*lb_ctx).cid_encryption_context = NULL;
                (*lb_ctx).cid_decryption_context = NULL;
                let mut i: size_t = 0 as size_t;
                while i < (*lb_ctx).server_id_length as size_t {
                    let mut j: size_t = ((*lb_ctx).server_id_length as size_t)
                        .wrapping_sub(i)
                        .wrapping_sub(1 as size_t);
                    (*lb_ctx).server_id[j as usize] = s_id64 as uint8_t;
                    s_id64 >>= 8 as ::core::ffi::c_int;
                    i = i.wrapping_add(1);
                }
                if s_id64 != 0 as uint64_t {
                    ret = -(1 as ::core::ffi::c_int);
                } else if (*lb_config).method as ::core::ffi::c_uint
                    == picoquic_load_balancer_cid_stream_cipher as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                    || (*lb_config).method as ::core::ffi::c_uint
                        == picoquic_load_balancer_cid_block_cipher as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                {
                    (*lb_ctx).cid_encryption_context = picoquic_aes128_ecb_create(
                        1 as ::core::ffi::c_int,
                        &raw mut (*lb_config).cid_encryption_key as *mut uint8_t
                            as *const ::core::ffi::c_void,
                    );
                    if (*lb_ctx).cid_encryption_context.is_null() {
                        ret = -(1 as ::core::ffi::c_int);
                    } else if (*lb_config).method as ::core::ffi::c_uint
                        == picoquic_load_balancer_cid_block_cipher as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                    {
                        (*lb_ctx).cid_decryption_context = picoquic_aes128_ecb_create(
                            0 as ::core::ffi::c_int,
                            &raw mut (*lb_config).cid_encryption_key as *mut uint8_t
                                as *const ::core::ffi::c_void,
                        );
                        if (*lb_ctx).cid_decryption_context.is_null() {
                            picoquic_aes128_ecb_free((*lb_ctx).cid_encryption_context);
                            (*lb_ctx).cid_encryption_context = NULL;
                            ret = -(1 as ::core::ffi::c_int);
                        }
                    }
                }
                if ret != 0 as ::core::ffi::c_int {
                    free(lb_ctx as *mut ::core::ffi::c_void);
                    lb_ctx = ::core::ptr::null_mut::<picoquic_load_balancer_cid_context_t>();
                } else {
                    (*quic).local_cnxid_length = (*lb_ctx).connection_id_length;
                    (*quic).cnx_id_callback_fn = Some(
                        picoquic_lb_compat_cid_generate
                            as unsafe extern "C" fn(
                                *mut picoquic_quic_t,
                                picoquic_connection_id_t,
                                picoquic_connection_id_t,
                                *mut ::core::ffi::c_void,
                                *mut picoquic_connection_id_t,
                            ) -> (),
                    )
                        as picoquic_connection_id_cb_fn;
                    (*quic).cnx_id_callback_ctx = lb_ctx as *mut ::core::ffi::c_void;
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_lb_compat_cid_config_free(mut quic: *mut picoquic_quic_t) {
    if (*quic).cnx_id_callback_fn
        == Some(
            picoquic_lb_compat_cid_generate
                as unsafe extern "C" fn(
                    *mut picoquic_quic_t,
                    picoquic_connection_id_t,
                    picoquic_connection_id_t,
                    *mut ::core::ffi::c_void,
                    *mut picoquic_connection_id_t,
                ) -> (),
        )
        && !(*quic).cnx_id_callback_ctx.is_null()
    {
        let mut lb_ctx: *mut picoquic_load_balancer_cid_context_t =
            (*quic).cnx_id_callback_ctx as *mut picoquic_load_balancer_cid_context_t;
        if !(*lb_ctx).cid_encryption_context.is_null() {
            picoquic_aes128_ecb_free((*lb_ctx).cid_encryption_context);
        }
        if !(*lb_ctx).cid_decryption_context.is_null() {
            picoquic_aes128_ecb_free((*lb_ctx).cid_decryption_context);
        }
        free(lb_ctx as *mut ::core::ffi::c_void);
        (*quic).cnx_id_callback_fn = None;
        (*quic).cnx_id_callback_ctx = NULL;
    }
}

use ::c2rust_bitfields;
extern "C" {
    pub type st_ptls_iovec_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type st_ptls_buffer_t;
    pub type st_picoquic_unified_logging_t;
    pub type st_ptls_verify_certificate_t;
    fn picoquic_test_uniform_random(random_context: *mut uint64_t, rnd_max: uint64_t) -> uint64_t;
    fn picoquic_update_pacing_data(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
        slow_start: ::core::ffi::c_int,
    );
    fn picoquic_update_pacing_rate(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
        pacing_rate: ::core::ffi::c_double,
        quantum: uint64_t,
    );
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn picoquic_cc_get_sequence_number(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
    ) -> uint64_t;
    fn picoquic_cc_get_ack_number(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
    ) -> uint64_t;
    fn picoquic_hystart_loss_volume_test(
        rtt_track: *mut picoquic_min_max_rtt_t,
        event: picoquic_congestion_notification_t,
        nb_bytes_newly_acked: uint64_t,
        nb_bytes_newly_lost: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_hystart_test(
        rtt_track: *mut picoquic_min_max_rtt_t,
        rtt_measurement: uint64_t,
        packet_time: uint64_t,
        current_time: uint64_t,
        is_one_way_delay_enabled: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_hystart_increase(
        path_x: *mut picoquic_path_t,
        rtt_filter: *mut picoquic_min_max_rtt_t,
        nb_delivered: uint64_t,
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
pub type picoquic_bbr_state_t = st_picoquic_bbr_state_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_picoquic_bbr_state_t {
    pub state: picoquic_bbr_alg_state_t,
    pub round_start_pn: uint64_t,
    pub round_count: ::core::ffi::c_int,
    pub rounds_since_probe: ::core::ffi::c_int,
    #[bitfield(name = "round_start", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub round_start: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub next_round_delivered: uint64_t,
    pub pacing_rate: ::core::ffi::c_double,
    pub send_quantum: uint64_t,
    pub prior_cwnd: uint64_t,
    pub pacing_gain: ::core::ffi::c_double,
    pub next_departure_time: uint64_t,
    pub cwnd_gain: ::core::ffi::c_double,
    #[bitfield(
        name = "packet_conservation",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    pub packet_conservation: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_0: [u8; 7],
    pub max_bw: uint64_t,
    pub bw_hi: uint64_t,
    pub bw_lo: uint64_t,
    pub bw: uint64_t,
    pub min_rtt: uint64_t,
    pub rtt_jitter_buffer: [uint64_t; 7],
    pub rtt_jitter_cycle: uint64_t,
    pub rtt_short_term_min: uint64_t,
    pub rtt_short_term_max: uint64_t,
    pub last_rtt_sample_stamp: uint64_t,
    pub nb_rtt_excess: ::core::ffi::c_int,
    pub bdp: uint64_t,
    pub extra_acked: uint64_t,
    pub offload_budget: uint64_t,
    pub max_inflight: uint64_t,
    pub inflight_hi: uint64_t,
    pub inflight_lo: uint64_t,
    pub bw_latest: uint64_t,
    pub inflight_latest: uint64_t,
    pub MaxBwFilter: [uint64_t; 2],
    pub cycle_count: ::core::ffi::c_uint,
    pub extra_acked_interval_start: uint64_t,
    pub extra_acked_delivered: uint64_t,
    pub ExtraACKedFilter: [uint64_t; 10],
    #[bitfield(name = "filled_pipe", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub filled_pipe: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_1: [u8; 7],
    pub full_bw: uint64_t,
    pub full_bw_count: ::core::ffi::c_int,
    pub min_rtt_stamp: uint64_t,
    pub probe_rtt_min_delay: uint64_t,
    pub probe_rtt_min_stamp: uint64_t,
    pub probe_rtt_done_stamp: uint64_t,
    pub min_rtt_margin: uint64_t,
    pub probe_rtt_expired: ::core::ffi::c_uint,
    pub probe_rtt_round_done: ::core::ffi::c_uint,
    #[bitfield(name = "idle_restart", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(
        name = "path_is_app_limited",
        ty = "::core::ffi::c_uint",
        bits = "1..=1"
    )]
    #[bitfield(
        name = "probe_probe_bw_quickly",
        ty = "::core::ffi::c_uint",
        bits = "2..=2"
    )]
    pub idle_restart_path_is_app_limited_probe_probe_bw_quickly: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_2: [u8; 7],
    pub bw_probe_wait: uint64_t,
    pub bw_probe_ceiling: uint64_t,
    pub cycle_stamp: uint64_t,
    pub rounds_since_bw_probe: uint32_t,
    pub bw_probe_up_cnt: uint32_t,
    pub bw_probe_up_rounds: uint32_t,
    pub bw_probe_samples: uint32_t,
    pub bw_probe_up_acks: uint64_t,
    pub ack_phase: picoquic_bbr_ack_phase_t,
    #[bitfield(
        name = "rtt_too_high_in_round",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    #[bitfield(name = "loss_in_round", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(name = "loss_round_start", ty = "::core::ffi::c_uint", bits = "2..=2")]
    pub rtt_too_high_in_round_loss_in_round_loss_round_start: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_3: [u8; 3],
    pub loss_round_delivered: uint64_t,
    pub is_in_recovery: ::core::ffi::c_uint,
    pub is_pto_recovery: ::core::ffi::c_uint,
    pub recovery_packet_number: uint64_t,
    pub recovery_delivered: uint64_t,
    #[bitfield(
        name = "is_handling_lost_feedback",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    pub is_handling_lost_feedback: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding_4: [u8; 7],
    pub cwin_before_lost_feedback: uint64_t,
    pub app_limited_round_count: ::core::ffi::c_int,
    pub app_limited_this_round: ::core::ffi::c_int,
    pub ecn_ect1_last_round: uint64_t,
    pub ecn_ce_last_round: uint64_t,
    pub ecn_alpha: ::core::ffi::c_double,
    pub random_context: uint64_t,
    pub rtt_filter: picoquic_min_max_rtt_t,
    pub bdp_seed: uint64_t,
    pub probe_bdp_seed: ::core::ffi::c_uint,
    pub wifi_shadow_rtt: uint64_t,
    pub quantum_ratio: ::core::ffi::c_double,
    pub exp_flags: bbr_exp,
}
pub type picoquic_min_max_rtt_t = st_picoquic_min_max_rtt_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_min_max_rtt_t {
    pub last_rtt_sample_time: uint64_t,
    pub rtt_filtered_min: uint64_t,
    pub nb_rtt_excess: ::core::ffi::c_int,
    pub sample_current: ::core::ffi::c_int,
    pub is_init: ::core::ffi::c_int,
    pub smoothed_drop_rate: ::core::ffi::c_double,
    pub smoothed_bytes_sent_16: uint64_t,
    pub smoothed_bytes_lost_16: uint64_t,
    pub last_lost_packet_number: uint64_t,
    pub sample_min: uint64_t,
    pub sample_max: uint64_t,
    pub samples: [uint64_t; 7],
}
pub type picoquic_bbr_ack_phase_t = ::core::ffi::c_uint;
pub const picoquic_bbr_acks_probe_feedback: picoquic_bbr_ack_phase_t = 3;
pub const picoquic_bbr_acks_refilling: picoquic_bbr_ack_phase_t = 2;
pub const picoquic_bbr_acks_probe_stopping: picoquic_bbr_ack_phase_t = 1;
pub const picoquic_bbr_acks_probe_starting: picoquic_bbr_ack_phase_t = 0;
pub type picoquic_bbr_alg_state_t = ::core::ffi::c_uint;
pub const picoquic_bbr_alg_startup_resume: picoquic_bbr_alg_state_t = 8;
pub const picoquic_bbr_alg_startup_long_rtt: picoquic_bbr_alg_state_t = 7;
pub const picoquic_bbr_alg_probe_rtt: picoquic_bbr_alg_state_t = 6;
pub const picoquic_bbr_alg_probe_bw_up: picoquic_bbr_alg_state_t = 5;
pub const picoquic_bbr_alg_probe_bw_refill: picoquic_bbr_alg_state_t = 4;
pub const picoquic_bbr_alg_probe_bw_cruise: picoquic_bbr_alg_state_t = 3;
pub const picoquic_bbr_alg_probe_bw_down: picoquic_bbr_alg_state_t = 2;
pub const picoquic_bbr_alg_drain: picoquic_bbr_alg_state_t = 1;
pub const picoquic_bbr_alg_startup: picoquic_bbr_alg_state_t = 0;
pub type bbr_per_ack_state_t = st_bbr_per_ack_state_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_bbr_per_ack_state_t {
    pub delivered: uint64_t,
    pub delivery_rate: uint64_t,
    pub rtt_sample: uint64_t,
    pub newly_acked: uint64_t,
    pub newly_lost: uint64_t,
    pub tx_in_flight: uint64_t,
    pub lost: uint64_t,
    pub ecn_ce: uint64_t,
    pub ecn_frac: ::core::ffi::c_double,
    pub ecn_alpha: ::core::ffi::c_double,
    #[bitfield(name = "is_app_limited", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "is_cwnd_limited", ty = "::core::ffi::c_uint", bits = "1..=1")]
    pub is_app_limited_is_cwnd_limited: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const PICOQUIC_MAX_PACKET_SIZE: ::core::ffi::c_int = 1536 as ::core::ffi::c_int;
pub const PICOQUIC_INITIAL_RTT: ::core::ffi::c_ulonglong = 250000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_TARGET_RENO_RTT: ::core::ffi::c_ulonglong = 100000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_TARGET_SATELLITE_RTT: ::core::ffi::c_ulonglong =
    610000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_MINRTT_MARGIN: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const PICOQUIC_MINRTT_THRESHOLD: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const PICOQUIC_CWIN_INITIAL: ::core::ffi::c_int =
    10 as ::core::ffi::c_int * PICOQUIC_MAX_PACKET_SIZE;
pub const PICOQUIC_CC_ALGO_NUMBER_BBR: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const BBRPacingMarginPercent: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BBRLossThresh: ::core::ffi::c_double = 0.2f64;
pub const BBRBeta: ::core::ffi::c_double = 0.7f64;
pub const BBRHeadroom: ::core::ffi::c_double = 0.15f64;
pub const BBRMinPipeCwnd: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const BBRMaxBwFilterLen: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const BBRExtraAckedFilterLen: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const BBRMinRTTFilterLen: ::core::ffi::c_int = 10000000 as ::core::ffi::c_int;
pub const BBRRTTJitterBufferLen: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
pub const BBRProbeRTTCwndGain: ::core::ffi::c_double = 0.5f64;
pub const BBRProbeRTTDuration: ::core::ffi::c_int = 200000 as ::core::ffi::c_int;
pub const BBRProbeRTTInterval: ::core::ffi::c_int = 5000000 as ::core::ffi::c_int;
pub const BBRStartupPacingGain: ::core::ffi::c_double = 2.77f64;
pub const BBRStartupCwndGain: ::core::ffi::c_double = 2.0f64;
pub const BBRStartupResumePacingGain: ::core::ffi::c_double = 1.25f64;
pub const BBRStartupResumeCwndGain: ::core::ffi::c_double = 1.25f64;
pub const BBRStartupResumeIncreaseThreshold: ::core::ffi::c_double = 1.125f64;
pub const BBRProbeBwDownPacingGain: ::core::ffi::c_double = 0.9f64;
pub const BBRProbeBwDownCwndGain: ::core::ffi::c_double = 2.0f64;
pub const BBRProbeBwCruisePacingGain: ::core::ffi::c_double = 1.0f64;
pub const BBRProbeBwCruiseCwndGain: ::core::ffi::c_double = 2.0f64;
pub const BBRProbeBwRefillPacingGain: ::core::ffi::c_double = 1.0f64;
pub const BBRProbeBwRefillCwndGain: ::core::ffi::c_double = 2.0f64;
pub const BBRProbeBwUpPacingGain: ::core::ffi::c_double = 1.25f64;
pub const BBRProbeBwUpCwndGain: ::core::ffi::c_double = 2.25f64;
pub const BBRAppLimitedRoundsThreshold: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const BBRMinRttMarginPercent: ::core::ffi::c_int = 5 as ::core::ffi::c_int;
pub const BBRLongRttThreshold: ::core::ffi::c_int = 250000 as ::core::ffi::c_int;
pub const BBRExcessiveEcnCE: ::core::ffi::c_double = 0.2f64;
#[no_mangle]
pub unsafe extern "C" fn update_windowed_max_filter(
    mut filter: *mut uint64_t,
    mut v: uint64_t,
    mut cycle: ::core::ffi::c_uint,
    mut filterLen: ::core::ffi::c_uint,
) -> uint64_t {
    if *filter.offset(cycle.wrapping_rem(filterLen) as isize) < v {
        *filter.offset(cycle.wrapping_rem(filterLen) as isize) = v;
    }
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < filterLen {
        if *filter.offset(i as isize) > v {
            v = *filter.offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    return v;
}
#[no_mangle]
pub unsafe extern "C" fn start_windowed_max_filter_period(
    mut filter: *mut uint64_t,
    mut cycle: ::core::ffi::c_uint,
    mut filterLen: ::core::ffi::c_uint,
) {
    *filter.offset(cycle.wrapping_rem(filterLen) as isize) = 0 as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn update_windowed_min_filter(
    mut filter: *mut uint64_t,
    mut v: uint64_t,
    mut cycle: ::core::ffi::c_uint,
    mut filterLen: ::core::ffi::c_uint,
) -> uint64_t {
    *filter.offset(cycle.wrapping_rem(filterLen) as isize) = v;
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < filterLen {
        if *filter.offset(i as isize) < v {
            v = *filter.offset(i as isize);
        }
        i = i.wrapping_add(1);
    }
    return v;
}
unsafe extern "C" fn BBRInitRandom(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) {
    let mut random_context: uint64_t = 0xfedcba9876543210 as uint64_t;
    random_context ^= current_time;
    if (*(*path_x).cnx).client_mode() != 0 {
        random_context = (random_context as ::core::ffi::c_ulonglong)
            .wrapping_add(0x123456789abcdef as ::core::ffi::c_ulonglong)
            as uint64_t as uint64_t;
    }
    if (*path_x).unique_path_id > 0 as uint64_t
        && (*path_x).unique_path_id != UINT64_MAX as uint64_t
    {
        random_context =
            random_context.wrapping_mul((*path_x).unique_path_id.wrapping_add(1 as uint64_t));
    }
    (*bbr_state).random_context = random_context;
}
unsafe extern "C" fn BBRInitFullPipe(mut bbr_state: *mut picoquic_bbr_state_t) {
    (*bbr_state).set_filled_pipe(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*bbr_state).full_bw = 0 as uint64_t;
    (*bbr_state).full_bw_count = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn BBROnInit(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) {
    memset(
        bbr_state as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<picoquic_bbr_state_t>() as size_t,
    );
    BBRInitRandom(bbr_state, path_x, current_time);
    if (*path_x).smoothed_rtt as ::core::ffi::c_ulonglong == PICOQUIC_INITIAL_RTT
        && (*path_x).rtt_variant == 0 as uint64_t
    {
        (*bbr_state).min_rtt = UINT64_MAX as uint64_t;
    } else {
        (*bbr_state).min_rtt = (*path_x).smoothed_rtt;
    }
    BBRResetRTTJitterBuffer(bbr_state, (*bbr_state).min_rtt, current_time);
    (*bbr_state).probe_rtt_min_stamp = current_time;
    (*bbr_state).probe_rtt_min_delay = (*bbr_state).min_rtt;
    (*bbr_state).min_rtt_stamp = current_time;
    (*bbr_state).extra_acked_interval_start = current_time;
    (*bbr_state).extra_acked_delivered = 0 as uint64_t;
    (*bbr_state).wifi_shadow_rtt = (*(*(*path_x).cnx).quic).wifi_shadow_rtt;
    (*bbr_state).exp_flags = (*(*(*path_x).cnx).quic).bbr_exp_flags;
    (*bbr_state).quantum_ratio = (*(*(*path_x).cnx).quic).bbr_quantum_ratio;
    if (*bbr_state).quantum_ratio == 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        (*bbr_state).quantum_ratio = 0.001f64;
    }
    BBRResetCongestionSignals(bbr_state);
    BBRResetLowerBounds(bbr_state);
    BBRInitRoundCounting(bbr_state, path_x);
    BBRInitFullPipe(bbr_state);
    BBRInitPacingRate(bbr_state, path_x);
    BBREnterStartup(bbr_state, path_x);
}
unsafe extern "C" fn picoquic_bbr_reset(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) {
    BBROnInit(bbr_state, path_x, current_time);
}
unsafe extern "C" fn picoquic_bbr_init(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) {
    let mut bbr_state: *mut picoquic_bbr_state_t =
        malloc(::core::mem::size_of::<picoquic_bbr_state_t>() as size_t)
            as *mut picoquic_bbr_state_t;
    (*path_x).congestion_alg_state = bbr_state as *mut ::core::ffi::c_void;
    if !bbr_state.is_null() {
        BBROnInit(bbr_state, path_x, current_time);
    }
}
unsafe extern "C" fn picoquic_bbr_delete(mut path_x: *mut picoquic_path_t) {
    if !(*path_x).congestion_alg_state.is_null() {
        free((*path_x).congestion_alg_state);
        (*path_x).congestion_alg_state = NULL;
    }
}
unsafe extern "C" fn BBRModulateCwndForRecovery(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
) {
    if (*rs).newly_lost > 0 as uint64_t {
        if (*path_x).cwin
            > (*rs)
                .newly_lost
                .wrapping_add((*path_x).send_mtu as uint64_t)
        {
            (*path_x).cwin = (*path_x).cwin.wrapping_sub((*rs).newly_lost);
        } else {
            (*path_x).cwin = (*path_x).send_mtu as uint64_t;
        }
    }
    if (*bbr_state).packet_conservation() as ::core::ffi::c_int != 0
        && (*path_x).cwin < (*path_x).bytes_in_transit.wrapping_add((*rs).newly_acked)
    {
        (*path_x).cwin = (*path_x).bytes_in_transit.wrapping_add((*rs).newly_acked);
    }
}
unsafe extern "C" fn BBRBoundCwndForModel(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    let mut cap: uint64_t = UINT64_MAX as uint64_t;
    if IsInAProbeBWState(bbr_state) != 0
        && (*bbr_state).state as ::core::ffi::c_uint
            != picoquic_bbr_alg_probe_bw_cruise as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*bbr_state).inflight_hi > 0 as uint64_t {
            cap = (*bbr_state).inflight_hi;
        }
    } else if (*bbr_state).state as ::core::ffi::c_uint
        == picoquic_bbr_alg_probe_rtt as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*bbr_state).state as ::core::ffi::c_uint
            == picoquic_bbr_alg_probe_bw_cruise as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        cap = BBRInflightWithHeadroom(bbr_state, path_x);
    }
    if cap > (*bbr_state).inflight_lo {
        cap = (*bbr_state).inflight_lo;
    }
    if cap < (BBRMinPipeCwnd as uint64_t).wrapping_mul((*path_x).send_mtu as uint64_t) {
        cap = (BBRMinPipeCwnd as size_t).wrapping_mul((*path_x).send_mtu) as uint64_t;
    }
    if (*path_x).cwin > cap {
        (*path_x).cwin = cap;
    }
}
unsafe extern "C" fn BBRProbeRTTCwnd(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) -> uint64_t {
    let mut probe_rtt_cwnd: uint64_t = BBRBDPMultiple(bbr_state, path_x, BBRProbeRTTCwndGain);
    if probe_rtt_cwnd < (BBRMinPipeCwnd as uint64_t).wrapping_mul((*path_x).send_mtu as uint64_t) {
        probe_rtt_cwnd = (BBRMinPipeCwnd as size_t).wrapping_mul((*path_x).send_mtu) as uint64_t;
    }
    return probe_rtt_cwnd;
}
unsafe extern "C" fn BBRBoundCwndForProbeRTT(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    if (*bbr_state).state as ::core::ffi::c_uint
        == picoquic_bbr_alg_probe_rtt as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut cap: uint64_t = BBRProbeRTTCwnd(bbr_state, path_x);
        if (*path_x).cwin > cap {
            (*path_x).cwin = cap;
        }
    }
}
unsafe extern "C" fn BBRSetCwnd(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
) {
    BBRUpdateMaxInflight(bbr_state, path_x);
    BBRModulateCwndForRecovery(bbr_state, path_x, rs);
    if (*bbr_state).packet_conservation() == 0 {
        if (*bbr_state).filled_pipe() != 0 {
            (*path_x).cwin = (*path_x).cwin.wrapping_add((*rs).newly_acked);
            if (*path_x).cwin > (*bbr_state).max_inflight {
                (*path_x).cwin = (*bbr_state).max_inflight;
            }
        } else if (*bbr_state).state as ::core::ffi::c_uint
            == picoquic_bbr_alg_startup_resume as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*bbr_state).bdp_seed > (*path_x).cwin
        {
            (*path_x).cwin = (*bbr_state).bdp_seed;
        } else if (*path_x).cwin < (*bbr_state).max_inflight
            || (*path_x).delivered < PICOQUIC_CWIN_INITIAL as uint64_t
        {
            (*path_x).cwin = (*path_x).cwin.wrapping_add((*rs).newly_acked);
        }
        if (*path_x).cwin
            < (BBRMinPipeCwnd as uint64_t).wrapping_mul((*path_x).send_mtu as uint64_t)
        {
            (*path_x).cwin =
                (BBRMinPipeCwnd as size_t).wrapping_mul((*path_x).send_mtu) as uint64_t;
        }
    }
    BBRBoundCwndForProbeRTT(bbr_state, path_x);
    BBRBoundCwndForModel(bbr_state, path_x);
}
unsafe extern "C" fn BBRSaveCwnd(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) -> uint64_t {
    if InLossRecovery(bbr_state) == 0
        && (*bbr_state).state as ::core::ffi::c_uint
            != picoquic_bbr_alg_probe_rtt as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return (*path_x).cwin;
    } else if (*bbr_state).prior_cwnd > (*path_x).cwin {
        return (*bbr_state).prior_cwnd;
    } else {
        return (*path_x).cwin;
    };
}
unsafe extern "C" fn BBRRestoreCwnd(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) -> uint64_t {
    if (*bbr_state).prior_cwnd > (*path_x).cwin {
        return (*bbr_state).prior_cwnd;
    } else {
        return (*path_x).cwin;
    };
}
unsafe extern "C" fn BBROnEnterFastRecovery(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
) {
    (*bbr_state).prior_cwnd = BBRSaveCwnd(bbr_state, path_x);
    let mut additional_cwnd: uint64_t = (*path_x).send_mtu as uint64_t;
    if (*rs).newly_acked > additional_cwnd {
        additional_cwnd = (*rs).newly_acked;
    }
    (*path_x).cwin = (*path_x).bytes_in_transit.wrapping_add(additional_cwnd);
    (*bbr_state).recovery_packet_number =
        picoquic_cc_get_sequence_number((*path_x).cnx as *mut picoquic_cnx_t, path_x);
    (*bbr_state).set_packet_conservation(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*bbr_state).is_in_recovery = 1 as ::core::ffi::c_uint;
    (*bbr_state).is_pto_recovery = 0 as ::core::ffi::c_uint;
    (*bbr_state).recovery_delivered = (*path_x).delivered;
}
unsafe extern "C" fn BBREnterLostFeedback(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    if (IsInAProbeBWState(bbr_state) != 0
        || (*bbr_state).state as ::core::ffi::c_uint
            == picoquic_bbr_alg_drain as ::core::ffi::c_int as ::core::ffi::c_uint)
        && (*bbr_state).is_handling_lost_feedback() == 0
        && (*(*path_x).cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*bbr_state).cwin_before_lost_feedback = (*path_x).cwin;
        (*path_x).cwin = (*path_x).bytes_in_transit;
        (*bbr_state).set_is_handling_lost_feedback(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn BBRExitLostFeedback(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    if (*bbr_state).is_handling_lost_feedback() != 0 {
        (*path_x).cwin = (*bbr_state).cwin_before_lost_feedback;
        (*bbr_state).set_is_handling_lost_feedback(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn BBROnEnterRTO(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut lost_packet_number: uint64_t,
) {
    if (*bbr_state).is_in_recovery == 0 {
        (*bbr_state).prior_cwnd = BBRSaveCwnd(bbr_state, path_x);
        (*bbr_state).is_in_recovery = 1 as ::core::ffi::c_uint;
    }
    if (*bbr_state).is_pto_recovery == 0 {
        (*path_x).cwin = (*path_x)
            .bytes_in_transit
            .wrapping_add((*path_x).send_mtu as uint64_t);
        (*bbr_state).recovery_packet_number = lost_packet_number;
        (*bbr_state).is_pto_recovery = 1 as ::core::ffi::c_uint;
        (*bbr_state).recovery_delivered = (*path_x).delivered;
    }
}
unsafe extern "C" fn BBROnExitRecovery(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) {
    if (*bbr_state).is_in_recovery != 0 {
        (*path_x).bandwidth_estimate_max = 0 as uint64_t;
        (*path_x).cwin = BBRRestoreCwnd(bbr_state, path_x);
        (*bbr_state).recovery_packet_number = UINT64_MAX as uint64_t;
        (*bbr_state).set_packet_conservation(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if (*bbr_state).is_pto_recovery != 0
            && (*bbr_state).exp_flags.do_handle_suspension() as ::core::ffi::c_int != 0
        {
            BBRReEnterStartup(bbr_state, path_x, current_time);
        } else if (*bbr_state).state as ::core::ffi::c_uint
            == picoquic_bbr_alg_probe_bw_up as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            BBRStartProbeBW_DOWN(bbr_state, path_x, current_time);
        }
        (*bbr_state).recovery_delivered = (*path_x).delivered;
        (*bbr_state).is_in_recovery = 0 as ::core::ffi::c_uint;
        (*bbr_state).is_pto_recovery = 0 as ::core::ffi::c_uint;
        (*bbr_state).probe_rtt_min_stamp = current_time;
        (*bbr_state).min_rtt_stamp = current_time;
    }
}
unsafe extern "C" fn BBROnSpuriousLoss(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut lost_packet_number: uint64_t,
    mut current_time: uint64_t,
) {
    if (*bbr_state).recovery_packet_number <= lost_packet_number
        && (*bbr_state).is_pto_recovery != 0
    {
        BBROnExitRecovery(bbr_state, path_x, current_time);
    }
}
unsafe extern "C" fn InLossRecovery(
    mut bbr_state: *mut picoquic_bbr_state_t,
) -> ::core::ffi::c_int {
    return (*bbr_state).is_in_recovery as ::core::ffi::c_int;
}
unsafe extern "C" fn BBRCheckRecovery(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) {
    if InLossRecovery(bbr_state) != 0 {
        if picoquic_cc_get_ack_number((*path_x).cnx as *mut picoquic_cnx_t, path_x)
            >= (*bbr_state).recovery_packet_number
        {
            BBROnExitRecovery(bbr_state, path_x, current_time);
        }
    } else if IsInflightTooHigh(bbr_state, path_x, rs) != 0 {
        BBROnEnterFastRecovery(bbr_state, path_x, rs);
    }
}
unsafe extern "C" fn BBRBDPMultipleWithBw(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut gain: ::core::ffi::c_double,
    mut bw: uint64_t,
) -> uint64_t {
    if (*bbr_state).min_rtt == UINT64_MAX as uint64_t {
        return (PICOQUIC_CWIN_INITIAL as uint64_t).wrapping_mul((*path_x).send_mtu as uint64_t);
    }
    (*bbr_state).bdp = bw
        .wrapping_mul((*bbr_state).min_rtt)
        .wrapping_div(1000000 as uint64_t);
    return (gain * (*bbr_state).bdp as ::core::ffi::c_double) as uint64_t;
}
unsafe extern "C" fn BBRBDPMultiple(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut gain: ::core::ffi::c_double,
) -> uint64_t {
    return BBRBDPMultipleWithBw(bbr_state, path_x, gain, (*bbr_state).bw);
}
unsafe extern "C" fn BBRUpdateOffloadBudget(mut bbr_state: *mut picoquic_bbr_state_t) {
    (*bbr_state).offload_budget = (3 as uint64_t).wrapping_mul((*bbr_state).send_quantum);
}
unsafe extern "C" fn BBRQuantizationBudget(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut inflight: uint64_t,
) -> uint64_t {
    BBRUpdateOffloadBudget(bbr_state);
    if inflight < (*bbr_state).offload_budget {
        inflight = (*bbr_state).offload_budget;
    }
    if inflight < (BBRMinPipeCwnd as uint64_t).wrapping_mul((*path_x).send_mtu as uint64_t) {
        inflight = (BBRMinPipeCwnd as size_t).wrapping_mul((*path_x).send_mtu) as uint64_t;
    }
    if (*bbr_state).state as ::core::ffi::c_uint
        == picoquic_bbr_alg_probe_bw_up as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        inflight = (inflight as ::core::ffi::c_ulong)
            .wrapping_add((2 as size_t).wrapping_mul((*path_x).send_mtu) as ::core::ffi::c_ulong)
            as uint64_t as uint64_t;
    }
    return inflight;
}
unsafe extern "C" fn BBRInflightWithBw(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut gain: ::core::ffi::c_double,
    mut bw: uint64_t,
) -> uint64_t {
    let mut inflight: uint64_t = BBRBDPMultipleWithBw(bbr_state, path_x, gain, bw);
    return BBRQuantizationBudget(bbr_state, path_x, inflight);
}
unsafe extern "C" fn BBRInflight(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut gain: ::core::ffi::c_double,
) -> uint64_t {
    return BBRInflightWithBw(bbr_state, path_x, gain, (*bbr_state).bw);
}
unsafe extern "C" fn BBRUpdateMaxInflight(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    let mut inflight: uint64_t = BBRBDPMultiple(bbr_state, path_x, (*bbr_state).cwnd_gain);
    inflight = inflight.wrapping_add((*bbr_state).extra_acked);
    if (*bbr_state).min_rtt < (*bbr_state).wifi_shadow_rtt && (*bbr_state).min_rtt > 0 as uint64_t {
        inflight = (inflight as ::core::ffi::c_double
            * (*bbr_state).wifi_shadow_rtt as ::core::ffi::c_double
            / (*bbr_state).min_rtt as ::core::ffi::c_double) as uint64_t;
    }
    (*bbr_state).max_inflight = BBRQuantizationBudget(bbr_state, path_x, inflight);
}
unsafe extern "C" fn BBRInitPacingRate(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    let mut initial_rtt: uint64_t = PICOQUIC_INITIAL_RTT as uint64_t;
    if (*path_x).smoothed_rtt as ::core::ffi::c_ulonglong != PICOQUIC_INITIAL_RTT
        || (*path_x).rtt_variant != 0 as uint64_t
    {
        initial_rtt = (*path_x).smoothed_rtt;
    }
    let mut nominal_bandwidth: ::core::ffi::c_double = (1000000 as ::core::ffi::c_ulonglong)
        .wrapping_mul(PICOQUIC_CWIN_INITIAL as ::core::ffi::c_ulonglong)
        as ::core::ffi::c_double
        / initial_rtt as ::core::ffi::c_double;
    (*bbr_state).pacing_rate = BBRStartupPacingGain * nominal_bandwidth;
}
unsafe extern "C" fn BBRSetPacingRateWithGain(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut pacing_gain: ::core::ffi::c_double,
) {
    let mut rate: ::core::ffi::c_double = pacing_gain
        * (*bbr_state)
            .bw
            .wrapping_mul((100 as ::core::ffi::c_int - BBRPacingMarginPercent) as uint64_t)
            as ::core::ffi::c_double
        / 100 as ::core::ffi::c_int as ::core::ffi::c_double;
    if (*bbr_state).state as ::core::ffi::c_uint
        == picoquic_bbr_alg_startup_resume as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*bbr_state).filled_pipe() == 0
        && (*bbr_state).bdp_seed > 0 as uint64_t
    {
        let mut bdp_rate: ::core::ffi::c_double = (*bbr_state).bdp_seed as ::core::ffi::c_double
            * 1000000.0f64
            / (*bbr_state).min_rtt as ::core::ffi::c_double;
        if bdp_rate > rate {
            rate = bdp_rate;
        }
    }
    if (*bbr_state).filled_pipe() as ::core::ffi::c_int != 0 || rate > (*bbr_state).pacing_rate {
        (*bbr_state).pacing_rate = rate;
    }
}
unsafe extern "C" fn BBRSetPacingRate(mut bbr_state: *mut picoquic_bbr_state_t) {
    BBRSetPacingRateWithGain(bbr_state, (*bbr_state).pacing_gain);
}
unsafe extern "C" fn BBRSetSendQuantum(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    let mut floor: uint64_t = (2 as uint64_t).wrapping_mul((*path_x).send_mtu as uint64_t);
    if (*bbr_state).pacing_rate < 150000 as ::core::ffi::c_int as ::core::ffi::c_double {
        floor = (1 as size_t).wrapping_mul((*path_x).send_mtu) as uint64_t;
    }
    (*bbr_state).send_quantum = ((*bbr_state).pacing_rate * (*bbr_state).quantum_ratio) as uint64_t;
    if (*bbr_state).send_quantum > 0x10000 as uint64_t {
        (*bbr_state).send_quantum = 0x10000 as uint64_t;
    }
    if (*bbr_state).send_quantum < floor {
        (*bbr_state).send_quantum = floor;
    }
}
unsafe extern "C" fn BBRUpdateLatestDeliverySignals(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
) {
    (*bbr_state).set_loss_round_start(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if (*bbr_state).bw_latest < (*rs).delivery_rate {
        (*bbr_state).bw_latest = (*rs).delivery_rate;
    }
    if (*bbr_state).inflight_latest < (*rs).delivered {
        (*bbr_state).inflight_latest = (*rs).delivered;
    }
    let mut prior_delivered: uint64_t = (*path_x).delivered.wrapping_sub((*rs).delivered);
    if prior_delivered >= (*bbr_state).loss_round_delivered {
        (*bbr_state).loss_round_delivered = (*path_x).delivered;
        (*bbr_state).set_loss_round_start(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn BBRAdvanceLatestDeliverySignals(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut rs: *mut bbr_per_ack_state_t,
) {
    if (*bbr_state).loss_round_start() != 0 {
        (*bbr_state).bw_latest = (*rs).delivery_rate;
        (*bbr_state).inflight_latest = (*rs).delivered;
    }
}
unsafe extern "C" fn BBRResetCongestionSignals(mut bbr_state: *mut picoquic_bbr_state_t) {
    (*bbr_state).set_loss_in_round(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*bbr_state).set_rtt_too_high_in_round(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*bbr_state).bw_latest = 0 as uint64_t;
    (*bbr_state).inflight_latest = 0 as uint64_t;
}
unsafe extern "C" fn BBRInitLowerBounds(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    if (*bbr_state).bw_lo == UINT64_MAX as uint64_t {
        (*bbr_state).bw_lo = (*bbr_state).max_bw;
    }
    if (*bbr_state).inflight_lo == UINT64_MAX as uint64_t {
        (*bbr_state).inflight_lo = (*path_x).cwin;
    }
}
unsafe extern "C" fn BBRLossLowerBounds(mut bbr_state: *mut picoquic_bbr_state_t) {
    (*bbr_state).bw_lo = (BBRBeta * (*bbr_state).bw_lo as ::core::ffi::c_double) as uint64_t;
    if (*bbr_state).bw_lo < (*bbr_state).bw_latest {
        (*bbr_state).bw_lo = (*bbr_state).bw_latest;
    }
    (*bbr_state).inflight_lo =
        (BBRBeta * (*bbr_state).inflight_lo as ::core::ffi::c_double) as uint64_t;
    if (*bbr_state).inflight_lo < (*bbr_state).inflight_latest {
        (*bbr_state).inflight_lo = (*bbr_state).inflight_latest;
    }
}
unsafe extern "C" fn BBRAdaptLowerBoundsFromCongestion(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    if BBRIsProbingBW(bbr_state) != 0 {
        return;
    }
    if (*bbr_state).loss_in_round() != 0 {
        BBRInitLowerBounds(bbr_state, path_x);
        BBRLossLowerBounds(bbr_state);
    }
}
unsafe extern "C" fn BBRUpdateCongestionSignals(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
) {
    BBRUpdateMaxBw(bbr_state, path_x, rs);
    if (*rs).newly_lost > 0 as uint64_t {
        (*bbr_state).set_loss_in_round(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if (*bbr_state).loss_round_start() == 0 {
        return;
    }
    BBRAdaptLowerBoundsFromCongestion(bbr_state, path_x);
    (*bbr_state).set_loss_in_round(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
unsafe extern "C" fn BBRResetLowerBounds(mut bbr_state: *mut picoquic_bbr_state_t) {
    (*bbr_state).bw_lo = UINT64_MAX as uint64_t;
    (*bbr_state).inflight_lo = UINT64_MAX as uint64_t;
}
unsafe extern "C" fn BBRBoundBWForModel(mut bbr_state: *mut picoquic_bbr_state_t) {
    (*bbr_state).bw = (*bbr_state).max_bw;
    if (*bbr_state).bw > (*bbr_state).bw_lo {
        (*bbr_state).bw = (*bbr_state).bw_lo;
    }
    if (*bbr_state).bw > (*bbr_state).bw_hi && (*bbr_state).bw_hi != 0 as uint64_t {
        (*bbr_state).bw = (*bbr_state).bw_hi;
    }
}
unsafe extern "C" fn BBRUpdateMaxBw(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
) {
    BBRUpdateRound(bbr_state, path_x);
    if (*rs).delivery_rate
        >= (*bbr_state).MaxBwFilter[(*bbr_state)
            .cycle_count
            .wrapping_rem(BBRMaxBwFilterLen as ::core::ffi::c_uint)
            as usize]
        || (*rs).is_app_limited() == 0
    {
        (*bbr_state).max_bw = update_windowed_max_filter(
            &raw mut (*bbr_state).MaxBwFilter as *mut uint64_t,
            (*rs).delivery_rate,
            (*bbr_state).cycle_count,
            BBRMaxBwFilterLen as ::core::ffi::c_uint,
        );
    }
}
unsafe extern "C" fn BBRAdvanceMaxBwFilter(mut bbr_state: *mut picoquic_bbr_state_t) {
    (*bbr_state).cycle_count = (*bbr_state).cycle_count.wrapping_add(1);
    (*bbr_state).ack_phase = picoquic_bbr_acks_probe_starting;
    start_windowed_max_filter_period(
        &raw mut (*bbr_state).MaxBwFilter as *mut uint64_t,
        (*bbr_state).cycle_count,
        BBRMaxBwFilterLen as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn BBRUpdateACKAggregation(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) {
    let mut interval: uint64_t = current_time.wrapping_sub((*bbr_state).extra_acked_interval_start);
    let mut expected_delivered: uint64_t = (*bbr_state).bw.wrapping_mul(interval);
    if (*bbr_state).extra_acked_delivered <= expected_delivered {
        (*bbr_state).extra_acked_delivered = 0 as uint64_t;
        (*bbr_state).extra_acked_interval_start = current_time;
        expected_delivered = 0 as uint64_t;
    }
    (*bbr_state).extra_acked_delivered = (*bbr_state)
        .extra_acked_delivered
        .wrapping_add((*rs).newly_acked);
    let mut extra: uint64_t = (*bbr_state)
        .extra_acked_delivered
        .wrapping_sub(expected_delivered);
    if extra > (*path_x).cwin {
        extra = (*path_x).cwin;
    }
    (*bbr_state).extra_acked = update_windowed_max_filter(
        &raw mut (*bbr_state).ExtraACKedFilter as *mut uint64_t,
        extra,
        (*bbr_state).round_count as ::core::ffi::c_uint,
        BBRExtraAckedFilterLen as ::core::ffi::c_uint,
    );
}
unsafe extern "C" fn IsInflightTooHigh(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
) -> ::core::ffi::c_int {
    if (*rs).ecn_alpha > BBRExcessiveEcnCE {
        return 1 as ::core::ffi::c_int;
    } else {
        let mut rs_delivered: uint64_t = (*path_x).delivered.wrapping_sub((*rs).delivered);
        if rs_delivered > (*bbr_state).recovery_delivered
            && (*rs).lost
                > ((*rs).tx_in_flight as ::core::ffi::c_double * BBRLossThresh) as uint64_t
            && (*rs).lost > (3 as uint64_t).wrapping_mul((*path_x).send_mtu as uint64_t)
        {
            return 1 as ::core::ffi::c_int;
        } else {
            return 0 as ::core::ffi::c_int;
        }
    };
}
unsafe extern "C" fn BBRHandleInflightTooHigh(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) {
    (*bbr_state).bw_probe_samples = 0 as uint32_t;
    if (*rs).is_app_limited() == 0 {
        let mut beta_target: uint64_t =
            (BBRTargetInflight(bbr_state, path_x) as ::core::ffi::c_double * BBRBeta) as uint64_t;
        (*bbr_state).inflight_hi = if (*rs).tx_in_flight > beta_target {
            (*rs).tx_in_flight
        } else {
            beta_target
        };
    }
    if (*bbr_state).state as ::core::ffi::c_uint
        == picoquic_bbr_alg_probe_bw_up as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        BBRStartProbeBW_DOWN(bbr_state, path_x, current_time);
    }
}
unsafe extern "C" fn CheckInflightTooHigh(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    if IsInflightTooHigh(bbr_state, path_x, rs) != 0 {
        if (*bbr_state).bw_probe_samples != 0 {
            BBRHandleInflightTooHigh(bbr_state, path_x, rs, current_time);
        }
        return 1 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn BBRInitRoundCounting(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    (*bbr_state).next_round_delivered = 0 as uint64_t;
    (*bbr_state).set_round_start(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*bbr_state).round_count = 0 as ::core::ffi::c_int;
    (*bbr_state).round_start_pn =
        picoquic_cc_get_sequence_number((*path_x).cnx as *mut picoquic_cnx_t, path_x);
}
unsafe extern "C" fn BBRStartRound(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    (*bbr_state).round_start_pn =
        picoquic_cc_get_sequence_number((*path_x).cnx as *mut picoquic_cnx_t, path_x);
    (*bbr_state).next_round_delivered = (*path_x).delivered;
}
unsafe extern "C" fn BBRUpdateRound(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    if picoquic_cc_get_ack_number((*path_x).cnx as *mut picoquic_cnx_t, path_x)
        >= (*bbr_state).round_start_pn
    {
        BBRStartRound(bbr_state, path_x);
        (*bbr_state).round_count += 1;
        (*bbr_state).rounds_since_probe += 1;
        (*bbr_state).set_round_start(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        start_windowed_max_filter_period(
            &raw mut (*bbr_state).ExtraACKedFilter as *mut uint64_t,
            (*bbr_state).round_count as ::core::ffi::c_uint,
            BBRExtraAckedFilterLen as ::core::ffi::c_uint,
        );
    } else {
        (*bbr_state).set_round_start(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    };
}
unsafe extern "C" fn BBRAdaptMinRttMargin(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    let mut margin: uint64_t = (*bbr_state)
        .min_rtt
        .wrapping_mul(BBRMinRttMarginPercent as uint64_t)
        .wrapping_mul(100 as uint64_t)
        .wrapping_div(1000000 as uint64_t);
    if (*bbr_state).max_bw > 0 as uint64_t {
        margin = (margin as ::core::ffi::c_ulong).wrapping_add(
            (2 as size_t)
                .wrapping_mul((*path_x).send_mtu)
                .wrapping_mul(1000000 as ::core::ffi::c_int as size_t)
                .wrapping_div((*bbr_state).max_bw as size_t) as ::core::ffi::c_ulong,
        ) as uint64_t as uint64_t;
    }
    (*bbr_state).min_rtt_margin = margin;
}
unsafe extern "C" fn BBRUpdateRTTJitterBuffer(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) {
    if current_time
        > (*bbr_state)
            .last_rtt_sample_stamp
            .wrapping_add(1000 as uint64_t)
    {
        (*bbr_state).rtt_jitter_buffer[(*bbr_state)
            .rtt_jitter_cycle
            .wrapping_rem(BBRRTTJitterBufferLen as uint64_t)
            as usize] = (*rs).rtt_sample;
        (*bbr_state).rtt_jitter_cycle = (*bbr_state).rtt_jitter_cycle.wrapping_add(1);
        (*bbr_state).last_rtt_sample_stamp = current_time;
        (*bbr_state).rtt_short_term_min = UINT64_MAX as uint64_t;
        (*bbr_state).rtt_short_term_max = 0 as uint64_t;
        let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        while i < BBRRTTJitterBufferLen as ::core::ffi::c_uint {
            if i as uint64_t >= (*bbr_state).rtt_jitter_cycle {
                break;
            }
            if (*bbr_state).rtt_jitter_buffer[i as usize] > (*bbr_state).rtt_short_term_max {
                (*bbr_state).rtt_short_term_max = (*bbr_state).rtt_jitter_buffer[i as usize];
            }
            if (*bbr_state).rtt_jitter_buffer[i as usize] < (*bbr_state).rtt_short_term_min {
                (*bbr_state).rtt_short_term_min = (*bbr_state).rtt_jitter_buffer[i as usize];
            }
            i = i.wrapping_add(1);
        }
    }
}
unsafe extern "C" fn BBRResetRTTJitterBuffer(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut rtt_init_value: uint64_t,
    mut current_time: uint64_t,
) {
    (*bbr_state).rtt_jitter_cycle = 0 as uint64_t;
    (*bbr_state).last_rtt_sample_stamp = current_time;
    (*bbr_state).rtt_short_term_min = rtt_init_value;
    (*bbr_state).rtt_short_term_max = rtt_init_value;
    (*bbr_state).probe_rtt_min_delay = rtt_init_value;
    (*bbr_state).nb_rtt_excess = 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn BBRUpdateMinRTT(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) {
    BBRAdaptMinRttMargin(bbr_state, path_x);
    BBRUpdateRTTJitterBuffer(bbr_state, rs, current_time);
    if (*bbr_state).min_rtt < UINT64_MAX as uint64_t {
        if (*bbr_state).min_rtt <= BBRLongRttThreshold as uint64_t {
            (*bbr_state).probe_rtt_expired = (current_time
                > (*bbr_state)
                    .probe_rtt_min_stamp
                    .wrapping_add(BBRProbeRTTInterval as uint64_t))
                as ::core::ffi::c_int
                as ::core::ffi::c_uint;
        } else {
            (*bbr_state).probe_rtt_expired = (current_time
                > (*bbr_state)
                    .probe_rtt_min_stamp
                    .wrapping_add((*bbr_state).min_rtt.wrapping_mul(100 as uint64_t)))
                as ::core::ffi::c_int
                as ::core::ffi::c_uint;
        }
    }
    if (*bbr_state).rtt_short_term_max < (*bbr_state).probe_rtt_min_delay
        || (*bbr_state).probe_rtt_expired != 0
        || (*bbr_state).rtt_jitter_cycle < BBRRTTJitterBufferLen as uint64_t
    {
        (*bbr_state).probe_rtt_min_delay = (*bbr_state).rtt_short_term_max;
        (*bbr_state).probe_rtt_min_stamp = current_time;
    } else if (*bbr_state).rtt_short_term_min
        < (*bbr_state)
            .min_rtt
            .wrapping_add((*bbr_state).min_rtt_margin)
    {
        (*bbr_state).probe_rtt_min_stamp = current_time;
        (*bbr_state).min_rtt_stamp = current_time;
    }
    let mut min_rtt_expired: ::core::ffi::c_int = (current_time
        > (*bbr_state)
            .min_rtt_stamp
            .wrapping_add(BBRMinRTTFilterLen as uint64_t))
        as ::core::ffi::c_int;
    if (*bbr_state).probe_rtt_min_delay < (*bbr_state).min_rtt
        || min_rtt_expired != 0
        || (*bbr_state).rtt_jitter_cycle < BBRRTTJitterBufferLen as uint64_t
    {
        (*bbr_state).min_rtt = (*bbr_state).probe_rtt_min_delay;
        (*bbr_state).min_rtt_stamp = (*bbr_state).probe_rtt_min_stamp;
    }
    if (*bbr_state).rtt_short_term_min > (*bbr_state).min_rtt
        && (*bbr_state).min_rtt > PICOQUIC_MINRTT_THRESHOLD as uint64_t
    {
        let mut delta_max: uint64_t = (PICOQUIC_MINRTT_MARGIN as uint64_t)
            .wrapping_add((*bbr_state).min_rtt.wrapping_div(4 as uint64_t));
        if (*bbr_state).rtt_short_term_min > (*bbr_state).min_rtt.wrapping_add(delta_max) {
            (*bbr_state).nb_rtt_excess += 1;
        }
    } else {
        (*bbr_state).nb_rtt_excess = 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn IsRTTTooHigh(mut bbr_state: *mut picoquic_bbr_state_t) -> ::core::ffi::c_int {
    return ((*bbr_state).nb_rtt_excess > BBRRTTJitterBufferLen) as ::core::ffi::c_int;
}
unsafe extern "C" fn BBRExitProbeRTT(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) {
    BBRResetLowerBounds(bbr_state);
    (*path_x).rtt_min = (*bbr_state).min_rtt;
    if (*bbr_state).filled_pipe() != 0 {
        BBREnterProbeBW(bbr_state, path_x, current_time);
        BBRStartProbeBW_CRUISE(bbr_state);
    } else {
        BBREnterStartup(bbr_state, path_x);
    };
}
unsafe extern "C" fn BBRCheckProbeRTTDone(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) {
    if (*bbr_state).probe_rtt_done_stamp != 0 as uint64_t
        && current_time > (*bbr_state).probe_rtt_done_stamp
    {
        (*bbr_state).probe_rtt_min_stamp = current_time;
        (*path_x).cwin = BBRRestoreCwnd(bbr_state, path_x);
        BBRExitProbeRTT(bbr_state, path_x, current_time);
    }
}
unsafe extern "C" fn BBRHandleProbeRTT(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) {
    if (*bbr_state).probe_rtt_done_stamp == 0 as uint64_t
        && (*rs).tx_in_flight <= BBRProbeRTTCwnd(bbr_state, path_x)
    {
        (*bbr_state).probe_rtt_done_stamp =
            current_time.wrapping_add(BBRProbeRTTDuration as uint64_t);
        (*bbr_state).probe_rtt_round_done = 0 as ::core::ffi::c_uint;
        BBRStartRound(bbr_state, path_x);
    } else if (*bbr_state).probe_rtt_done_stamp != 0 as uint64_t {
        if (*bbr_state).round_start() != 0 {
            (*bbr_state).probe_rtt_round_done = 1 as ::core::ffi::c_uint;
        }
        if (*bbr_state).probe_rtt_round_done != 0 {
            BBRCheckProbeRTTDone(bbr_state, path_x, current_time);
        }
    }
}
unsafe extern "C" fn BBREnterProbeRTT(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    (*bbr_state).state = picoquic_bbr_alg_probe_rtt;
    (*bbr_state).pacing_gain = 1.0f64;
    (*bbr_state).cwnd_gain = BBRProbeRTTCwndGain;
    (*path_x).set_is_cca_probing_up(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
unsafe extern "C" fn BBRCheckProbeRTT(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) {
    if (*bbr_state).state as ::core::ffi::c_uint
        != picoquic_bbr_alg_probe_rtt as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*bbr_state).probe_rtt_expired != 0
        && (*bbr_state).idle_restart() == 0
    {
        BBREnterProbeRTT(bbr_state, path_x);
        (*bbr_state).min_rtt = (*rs).rtt_sample;
        (*bbr_state).prior_cwnd = BBRSaveCwnd(bbr_state, path_x);
        (*bbr_state).probe_rtt_done_stamp = 0 as uint64_t;
        (*bbr_state).ack_phase = picoquic_bbr_acks_probe_stopping;
        BBRStartRound(bbr_state, path_x);
    }
    if (*bbr_state).state as ::core::ffi::c_uint
        == picoquic_bbr_alg_probe_rtt as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        BBRHandleProbeRTT(bbr_state, path_x, rs, current_time);
    }
    if (*rs).delivered > 0 as uint64_t {
        (*bbr_state).set_idle_restart(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn IsInAProbeBWState(
    mut bbr_state: *mut picoquic_bbr_state_t,
) -> ::core::ffi::c_int {
    let mut state: picoquic_bbr_alg_state_t = (*bbr_state).state;
    return (state as ::core::ffi::c_uint
        == picoquic_bbr_alg_probe_bw_down as ::core::ffi::c_int as ::core::ffi::c_uint
        || state as ::core::ffi::c_uint
            == picoquic_bbr_alg_probe_bw_cruise as ::core::ffi::c_int as ::core::ffi::c_uint
        || state as ::core::ffi::c_uint
            == picoquic_bbr_alg_probe_bw_refill as ::core::ffi::c_int as ::core::ffi::c_uint
        || state as ::core::ffi::c_uint
            == picoquic_bbr_alg_probe_bw_up as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
}
unsafe extern "C" fn BBRIsProbingBW(
    mut bbr_state: *mut picoquic_bbr_state_t,
) -> ::core::ffi::c_int {
    let mut state: picoquic_bbr_alg_state_t = (*bbr_state).state;
    return if state as ::core::ffi::c_uint
        == picoquic_bbr_alg_probe_bw_down as ::core::ffi::c_int as ::core::ffi::c_uint
        || state as ::core::ffi::c_uint
            == picoquic_bbr_alg_probe_bw_cruise as ::core::ffi::c_int as ::core::ffi::c_uint
        || state as ::core::ffi::c_uint
            == picoquic_bbr_alg_drain as ::core::ffi::c_int as ::core::ffi::c_uint
        || state as ::core::ffi::c_uint
            == picoquic_bbr_alg_probe_rtt as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
}
unsafe extern "C" fn BBRInflightWithHeadroom(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) -> uint64_t {
    if (*bbr_state).inflight_hi == UINT64_MAX as uint64_t {
        return UINT64_MAX as uint64_t;
    }
    let mut inflight_with_headroom: uint64_t =
        ((1.0f64 - BBRHeadroom) * (*bbr_state).inflight_hi as ::core::ffi::c_double) as uint64_t;
    if inflight_with_headroom
        < (BBRMinPipeCwnd as uint64_t).wrapping_mul((*path_x).send_mtu as uint64_t)
    {
        inflight_with_headroom =
            (BBRMinPipeCwnd as size_t).wrapping_mul((*path_x).send_mtu) as uint64_t;
    }
    return inflight_with_headroom;
}
unsafe extern "C" fn BBRRaiseInflightHiSlope(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    let mut growth_this_round: uint64_t =
        ((*path_x).send_mtu as uint64_t) << (*bbr_state).bw_probe_up_rounds;
    (*bbr_state).bw_probe_up_rounds =
        if (*bbr_state).bw_probe_up_rounds.wrapping_add(1 as uint32_t) < 30 as uint32_t {
            (*bbr_state).bw_probe_up_rounds.wrapping_add(1 as uint32_t)
        } else {
            30 as uint32_t
        };
    let mut up_cnt: uint32_t = (*path_x).cwin.wrapping_div(growth_this_round) as uint32_t;
    (*bbr_state).bw_probe_up_cnt = if up_cnt > 1 as uint32_t {
        up_cnt
    } else {
        1 as uint32_t
    };
}
unsafe extern "C" fn BBRProbeInflightHiUpward(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
) {
    if (*rs).is_cwnd_limited() == 0 || (*path_x).cwin < (*bbr_state).inflight_hi {
        return;
    }
    (*bbr_state).bw_probe_up_acks = (*bbr_state)
        .bw_probe_up_acks
        .wrapping_add((*rs).newly_acked);
    if (*bbr_state).bw_probe_up_acks
        >= ((*bbr_state).bw_probe_up_cnt as uint64_t).wrapping_mul((*path_x).send_mtu as uint64_t)
    {
        let mut delta: uint64_t = (*bbr_state)
            .bw_probe_up_acks
            .wrapping_div((*bbr_state).bw_probe_up_cnt as uint64_t);
        (*bbr_state).bw_probe_up_acks = (*bbr_state)
            .bw_probe_up_acks
            .wrapping_sub(delta.wrapping_mul((*bbr_state).bw_probe_up_cnt as uint64_t));
        (*bbr_state).inflight_hi = (*bbr_state).inflight_hi.wrapping_add(delta);
    }
    if (*bbr_state).round_start() != 0 {
        BBRRaiseInflightHiSlope(bbr_state, path_x);
    }
}
unsafe extern "C" fn BBRAdaptUpperBounds(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) {
    if (*bbr_state).ack_phase as ::core::ffi::c_uint
        == picoquic_bbr_acks_probe_starting as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*bbr_state).round_start() as ::core::ffi::c_int != 0
    {
        (*bbr_state).ack_phase = picoquic_bbr_acks_probe_feedback;
    }
    if (*bbr_state).ack_phase as ::core::ffi::c_uint
        == picoquic_bbr_acks_probe_stopping as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*bbr_state).round_start() as ::core::ffi::c_int != 0
    {
        if IsInAProbeBWState(bbr_state) != 0 && (*rs).is_app_limited() == 0 {
            BBRAdvanceMaxBwFilter(bbr_state);
        }
    }
    if CheckInflightTooHigh(bbr_state, path_x, rs, current_time) == 0 {
        if (*bbr_state).inflight_hi == UINT64_MAX as uint64_t
            || (*bbr_state).bw_hi == UINT64_MAX as uint64_t
        {
            return;
        }
        if (*rs).tx_in_flight > (*bbr_state).inflight_hi {
            (*bbr_state).inflight_hi = (*rs).tx_in_flight;
        }
        if (*rs).delivery_rate > (*bbr_state).bw_hi {
            (*bbr_state).bw_hi = (*rs).delivery_rate;
        }
        if (*bbr_state).state as ::core::ffi::c_uint
            == picoquic_bbr_alg_probe_bw_up as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            BBRProbeInflightHiUpward(bbr_state, path_x, rs);
        }
    }
}
unsafe extern "C" fn BBRCheckTimeToCruise(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) -> ::core::ffi::c_int {
    if (*path_x).bytes_in_transit > BBRInflightWithHeadroom(bbr_state, path_x) {
        return 0 as ::core::ffi::c_int;
    }
    if (*path_x).bytes_in_transit
        <= BBRInflightWithBw(bbr_state, path_x, 1.0f64, (*bbr_state).max_bw)
    {
        return 1 as ::core::ffi::c_int;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn BBRRandomIntBetween(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut low: uint64_t,
    mut high: uint64_t,
) -> uint64_t {
    return low.wrapping_add(picoquic_test_uniform_random(
        &raw mut (*bbr_state).random_context,
        high.wrapping_sub(low).wrapping_add(1 as uint64_t),
    ));
}
unsafe extern "C" fn BBRPickProbeWait(mut bbr_state: *mut picoquic_bbr_state_t) {
    (*bbr_state).rounds_since_bw_probe =
        BBRRandomIntBetween(bbr_state, 0 as uint64_t, 1 as uint64_t) as uint32_t;
    if (*bbr_state).min_rtt < BBRLongRttThreshold as uint64_t {
        (*bbr_state).bw_probe_wait = (2000000 as uint64_t).wrapping_add(BBRRandomIntBetween(
            bbr_state,
            0 as uint64_t,
            1000000 as uint64_t,
        ));
    } else {
        (*bbr_state).bw_probe_wait = (8 as uint64_t)
            .wrapping_mul((*bbr_state).min_rtt)
            .wrapping_add(BBRRandomIntBetween(
                bbr_state,
                0 as uint64_t,
                (4 as uint64_t).wrapping_mul((*bbr_state).min_rtt),
            ));
    };
}
unsafe extern "C" fn BBRPickProbeWaitEarly(mut bbr_state: *mut picoquic_bbr_state_t) {
    (*bbr_state).rounds_since_bw_probe =
        BBRRandomIntBetween(bbr_state, 0 as uint64_t, 1 as uint64_t) as uint32_t;
    if (*bbr_state).min_rtt < BBRLongRttThreshold as uint64_t {
        (*bbr_state).bw_probe_wait = (*bbr_state).min_rtt.wrapping_add(BBRRandomIntBetween(
            bbr_state,
            0 as uint64_t,
            BBRLongRttThreshold as uint64_t,
        ));
    } else {
        (*bbr_state).bw_probe_wait = (*bbr_state).min_rtt.wrapping_add(BBRRandomIntBetween(
            bbr_state,
            0 as uint64_t,
            (*bbr_state).min_rtt,
        ));
    };
}
unsafe extern "C" fn BBRTargetInflight(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) -> uint64_t {
    return if (*bbr_state).bdp < (*path_x).cwin {
        (*bbr_state).bdp
    } else {
        (*path_x).cwin
    };
}
unsafe extern "C" fn BBRCheckPathSaturated(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    if IsInAProbeBWState(bbr_state) != 0
        && (*rs).rtt_sample > (2 as uint64_t).wrapping_mul((*bbr_state).min_rtt)
        && (*bbr_state).rounds_since_bw_probe >= 1 as uint32_t
        && (*bbr_state).pacing_rate
            > (3 as uint64_t).wrapping_mul((*rs).delivery_rate) as ::core::ffi::c_double
        && (*bbr_state).wifi_shadow_rtt == 0 as uint64_t
    {
        (*bbr_state).prior_cwnd = (*rs).delivered;
        (*bbr_state).probe_rtt_done_stamp = 0 as uint64_t;
        (*bbr_state).ack_phase = picoquic_bbr_acks_probe_stopping;
        (*bbr_state).MaxBwFilter[0 as ::core::ffi::c_int as usize] = (*rs).delivery_rate;
        (*bbr_state).MaxBwFilter[1 as ::core::ffi::c_int as usize] = (*rs).delivery_rate;
        (*bbr_state).max_bw = (*rs).delivery_rate;
        (*bbr_state).full_bw = (*rs).delivery_rate;
        BBREnterDrain(bbr_state, path_x, current_time);
        BBRStartRound(bbr_state, path_x);
        return 1 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn BBRCheckAppLimitedEnded(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut rs: *mut bbr_per_ack_state_t,
) -> ::core::ffi::c_int {
    let mut app_limited_ended: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*bbr_state).round_start() != 0 {
        if (*bbr_state).app_limited_this_round != 0 {
            (*bbr_state).app_limited_round_count += 1;
        } else {
            app_limited_ended = ((*bbr_state).app_limited_round_count
                > BBRAppLimitedRoundsThreshold)
                as ::core::ffi::c_int;
            (*bbr_state).app_limited_round_count = 0 as ::core::ffi::c_int;
        }
        (*bbr_state).app_limited_this_round = 0 as ::core::ffi::c_int;
    } else {
        (*bbr_state).app_limited_this_round |= (*rs).is_app_limited() as ::core::ffi::c_int;
    }
    return app_limited_ended;
}
unsafe extern "C" fn BBRIsRenoCoexistenceProbeTime(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) -> ::core::ffi::c_int {
    let mut reno_rounds: uint64_t =
        BBRTargetInflight(bbr_state, path_x).wrapping_div((*path_x).send_mtu as uint64_t);
    let mut rounds: uint64_t = if reno_rounds < 63 as uint64_t {
        reno_rounds
    } else {
        63 as uint64_t
    };
    return ((*bbr_state).rounds_since_bw_probe as uint64_t >= rounds) as ::core::ffi::c_int;
}
unsafe extern "C" fn BBRHasElapsedInPhase(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut interval: uint64_t,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    return (current_time > (*bbr_state).cycle_stamp.wrapping_add(interval)) as ::core::ffi::c_int;
}
unsafe extern "C" fn BBRCheckTimeToProbeBW(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    if BBRHasElapsedInPhase(bbr_state, (*bbr_state).bw_probe_wait, current_time) != 0
        || BBRIsRenoCoexistenceProbeTime(bbr_state, path_x) != 0
        || (*bbr_state).exp_flags.do_enter_probeBW_after_limited() as ::core::ffi::c_int != 0
            && BBRCheckAppLimitedEnded(bbr_state, rs) != 0
    {
        BBRStartProbeBW_REFILL(bbr_state, path_x);
        return 1 as ::core::ffi::c_int;
    } else {
        return 0 as ::core::ffi::c_int;
    };
}
unsafe extern "C" fn BBRStartProbeBW_DOWN(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) {
    (*bbr_state).pacing_gain = BBRProbeBwDownPacingGain;
    (*bbr_state).cwnd_gain = BBRProbeBwDownCwndGain;
    BBRResetCongestionSignals(bbr_state);
    (*bbr_state).bw_probe_up_cnt = UINT32_MAX as uint32_t;
    if (*bbr_state).probe_probe_bw_quickly() as ::core::ffi::c_int != 0
        && (*bbr_state).exp_flags.do_rapid_start() as ::core::ffi::c_int != 0
    {
        BBRPickProbeWaitEarly(bbr_state);
    } else {
        BBRPickProbeWait(bbr_state);
    }
    (*bbr_state).cycle_stamp = current_time;
    (*bbr_state).ack_phase = picoquic_bbr_acks_probe_stopping;
    BBRStartRound(bbr_state, path_x);
    (*bbr_state).state = picoquic_bbr_alg_probe_bw_down;
    (*bbr_state).nb_rtt_excess = 0 as ::core::ffi::c_int;
    (*bbr_state).app_limited_round_count = 0 as ::core::ffi::c_int;
    (*bbr_state).app_limited_this_round = 0 as ::core::ffi::c_int;
    (*path_x).set_is_cca_probing_up(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
unsafe extern "C" fn BBRStartProbeBW_CRUISE(mut bbr_state: *mut picoquic_bbr_state_t) {
    (*bbr_state).pacing_gain = BBRProbeBwCruisePacingGain;
    (*bbr_state).cwnd_gain = BBRProbeBwCruiseCwndGain;
    (*bbr_state).state = picoquic_bbr_alg_probe_bw_cruise;
}
unsafe extern "C" fn BBRStartProbeBW_REFILL(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    (*bbr_state).pacing_gain = BBRProbeBwRefillPacingGain;
    (*bbr_state).cwnd_gain = BBRProbeBwRefillCwndGain;
    BBRResetLowerBounds(bbr_state);
    (*bbr_state).bw_probe_up_rounds = 0 as uint32_t;
    (*bbr_state).bw_probe_up_acks = 0 as uint64_t;
    (*bbr_state).full_bw = (*bbr_state).max_bw;
    (*bbr_state).ack_phase = picoquic_bbr_acks_refilling;
    BBRStartRound(bbr_state, path_x);
    (*bbr_state).state = picoquic_bbr_alg_probe_bw_refill;
    (*path_x).set_is_cca_probing_up(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
unsafe extern "C" fn BBRStartProbeBW_UP(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) {
    (*bbr_state).nb_rtt_excess = 0 as ::core::ffi::c_int;
    (*bbr_state).pacing_gain = BBRProbeBwUpPacingGain;
    (*bbr_state).cwnd_gain = BBRProbeBwUpCwndGain;
    (*bbr_state).ack_phase = picoquic_bbr_acks_probe_starting;
    BBRStartRound(bbr_state, path_x);
    (*bbr_state).cycle_stamp = current_time;
    (*bbr_state).state = picoquic_bbr_alg_probe_bw_up;
    BBRRaiseInflightHiSlope(bbr_state, path_x);
    (*path_x).set_is_cca_probing_up(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
unsafe extern "C" fn BBRUpdateProbeBWCyclePhase(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) {
    if (*bbr_state).filled_pipe() == 0 {
        return;
    }
    BBRAdaptUpperBounds(bbr_state, path_x, rs, current_time);
    match (*bbr_state).state as ::core::ffi::c_uint {
        2 => {
            if BBRCheckTimeToProbeBW(bbr_state, path_x, rs, current_time) != 0 {
                return;
            }
            if BBRCheckPathSaturated(bbr_state, path_x, rs, current_time) != 0 {
                return;
            }
            if BBRCheckTimeToCruise(bbr_state, path_x) != 0 {
                if (15 as uint64_t).wrapping_mul((*bbr_state).max_bw)
                    >= (16 as uint64_t).wrapping_mul((*bbr_state).full_bw)
                    && (*rs).ecn_alpha <= BBRExcessiveEcnCE
                {
                    (*bbr_state).full_bw = (*bbr_state).max_bw;
                    (*bbr_state).full_bw_count = 0 as ::core::ffi::c_int;
                    (*bbr_state).set_probe_probe_bw_quickly(
                        1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                    );
                } else {
                    (*bbr_state).full_bw_count += 1;
                    if (*bbr_state).full_bw_count > 3 as ::core::ffi::c_int
                        || (*rs).ecn_alpha > BBRExcessiveEcnCE
                    {
                        (*bbr_state).set_probe_probe_bw_quickly(
                            0 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                        );
                        (*bbr_state).full_bw_count = 0 as ::core::ffi::c_int;
                    }
                }
                BBRStartProbeBW_CRUISE(bbr_state);
            }
        }
        3 => {
            if BBRCheckPathSaturated(bbr_state, path_x, rs, current_time) != 0 {
                return;
            }
            if BBRCheckTimeToProbeBW(bbr_state, path_x, rs, current_time) != 0 {
                return;
            }
        }
        4 => {
            if (*bbr_state).round_start() != 0 {
                (*bbr_state).bw_probe_samples = 1 as uint32_t;
                BBRStartProbeBW_UP(bbr_state, path_x, current_time);
            }
        }
        5 => {
            if BBRHasElapsedInPhase(bbr_state, (*bbr_state).min_rtt, current_time) != 0
                && (*bbr_state).min_rtt > PICOQUIC_MINRTT_THRESHOLD as uint64_t
                && (*bbr_state).exp_flags.do_exit_probeBW_up_on_delay() as ::core::ffi::c_int != 0
                && ((*bbr_state).nb_rtt_excess > 0 as ::core::ffi::c_int
                    || (*path_x).bytes_in_transit
                        > BBRInflightWithBw(bbr_state, path_x, 1.25f64, (*bbr_state).max_bw))
            {
                BBRStartProbeBW_DOWN(bbr_state, path_x, current_time);
            }
        }
        _ => return,
    }
    if (*bbr_state).bw > (*bbr_state).bw_probe_ceiling {
        BBRReEnterStartup(bbr_state, path_x, current_time);
    }
}
unsafe extern "C" fn BBREnterProbeBW(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) {
    (*bbr_state).bw_probe_ceiling = (*bbr_state)
        .bw
        .wrapping_add((*bbr_state).bw.wrapping_div(2 as uint64_t));
    BBRStartProbeBW_DOWN(bbr_state, path_x, current_time);
}
unsafe extern "C" fn BBREnterDrain(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) {
    (*path_x).set_is_ssthresh_initialized(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*bbr_state).state = picoquic_bbr_alg_drain;
    (*bbr_state).pacing_gain = 1.0f64 / BBRStartupCwndGain;
    (*bbr_state).cwnd_gain = BBRStartupCwndGain;
    (*path_x).set_is_cca_probing_up(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
unsafe extern "C" fn BBRCheckDrain(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) {
    if (*bbr_state).state as ::core::ffi::c_uint
        == picoquic_bbr_alg_drain as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*path_x).bytes_in_transit <= BBRInflight(bbr_state, path_x, 1.0f64)
    {
        BBREnterProbeBW(bbr_state, path_x, current_time);
    }
}
unsafe extern "C" fn BBRCheckStartupFullBandwidthGeneric(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut threshold: ::core::ffi::c_double,
) {
    if (*bbr_state).filled_pipe() as ::core::ffi::c_int != 0
        || (*bbr_state).round_start() == 0
        || (*rs).is_app_limited() as ::core::ffi::c_int != 0
    {
        return;
    }
    if (*bbr_state).max_bw as ::core::ffi::c_double
        >= threshold * (*bbr_state).full_bw as ::core::ffi::c_double
    {
        (*bbr_state).full_bw = (*bbr_state).max_bw;
        (*bbr_state).full_bw_count = 0 as ::core::ffi::c_int;
        return;
    }
    (*bbr_state).full_bw_count += 1;
    if (*bbr_state).full_bw_count >= 3 as ::core::ffi::c_int {
        (*bbr_state).set_filled_pipe(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn BBREnterStartupResume(mut bbr_state: *mut picoquic_bbr_state_t) {
    (*bbr_state).state = picoquic_bbr_alg_startup_resume;
    (*bbr_state).pacing_gain = BBRStartupResumePacingGain;
    (*bbr_state).cwnd_gain = BBRStartupResumeCwndGain;
}
unsafe extern "C" fn BBRCheckStartupResume(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) {
    if (*bbr_state).state as ::core::ffi::c_uint
        == picoquic_bbr_alg_startup_resume as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        BBRCheckStartupHighLoss(bbr_state, path_x, rs);
        if (*bbr_state).filled_pipe() == 0
            && (*bbr_state).max_bw as ::core::ffi::c_double
                > BBRStartupResumeIncreaseThreshold * (*bbr_state).bdp_seed as ::core::ffi::c_double
        {
            BBREnterStartup(bbr_state, path_x);
        } else {
            BBRCheckStartupFullBandwidthGeneric(bbr_state, rs, BBRStartupResumeIncreaseThreshold);
            if (*bbr_state).filled_pipe() != 0 {
                if (*bbr_state).full_bw_count > 0 as ::core::ffi::c_int {
                    (*bbr_state).set_probe_probe_bw_quickly(
                        1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                    );
                    (*bbr_state).full_bw_count = 0 as ::core::ffi::c_int;
                }
                BBREnterDrain(bbr_state, path_x, current_time);
            }
        }
    }
}
unsafe extern "C" fn BBRCheckStartupHighLoss(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
) {
    if IsInflightTooHigh(bbr_state, path_x, rs) != 0 {
        (*bbr_state).set_filled_pipe(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn BBRCheckStartupFullBandwidth(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut rs: *mut bbr_per_ack_state_t,
) {
    if (*bbr_state).filled_pipe() as ::core::ffi::c_int != 0
        || (*bbr_state).round_start() == 0
        || (*rs).is_app_limited() as ::core::ffi::c_int != 0
    {
        return;
    }
    if (4 as uint64_t).wrapping_mul((*bbr_state).max_bw)
        >= (5 as uint64_t).wrapping_mul((*bbr_state).full_bw)
    {
        (*bbr_state).full_bw = (*bbr_state).max_bw;
        (*bbr_state).full_bw_count = 0 as ::core::ffi::c_int;
        if (*rs).ecn_frac < 0.2f64 {
            return;
        }
    }
    (*bbr_state).full_bw_count += 1;
    if (*bbr_state).full_bw_count >= 3 as ::core::ffi::c_int || (*rs).ecn_frac >= BBRExcessiveEcnCE
    {
        (*bbr_state).set_filled_pipe(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}
unsafe extern "C" fn BBRCheckStartupDone(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) {
    if (*bbr_state).state as ::core::ffi::c_uint
        == picoquic_bbr_alg_startup as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        BBRCheckStartupFullBandwidth(bbr_state, rs);
        BBRCheckStartupHighLoss(bbr_state, path_x, rs);
        if (*bbr_state).min_rtt > PICOQUIC_MINRTT_THRESHOLD as uint64_t
            && IsRTTTooHigh(bbr_state) != 0
        {
            (*bbr_state).set_filled_pipe(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        if (*bbr_state).filled_pipe() != 0 {
            (*bbr_state)
                .set_probe_probe_bw_quickly(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*bbr_state).full_bw_count = 0 as ::core::ffi::c_int;
            BBREnterDrain(bbr_state, path_x, current_time);
        }
    }
}
unsafe extern "C" fn BBREnterStartup(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    (*bbr_state).state = picoquic_bbr_alg_startup;
    (*bbr_state).pacing_gain = BBRStartupPacingGain;
    (*bbr_state).cwnd_gain = BBRStartupCwndGain;
    (*path_x).set_is_cca_probing_up(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
unsafe extern "C" fn BBRReEnterStartup(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) {
    (*bbr_state).full_bw = 0 as uint64_t;
    (*bbr_state).set_filled_pipe(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*bbr_state).full_bw_count = 0 as ::core::ffi::c_int;
    (*bbr_state).set_probe_probe_bw_quickly(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    BBREnterStartup(bbr_state, path_x);
}
#[no_mangle]
pub unsafe extern "C" fn BBREnterStartupLongRTT(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
) {
    let mut cwnd: uint64_t = PICOQUIC_CWIN_INITIAL as uint64_t;
    (*bbr_state).state = picoquic_bbr_alg_startup_long_rtt;
    if (*path_x).rtt_min as ::core::ffi::c_ulonglong > PICOQUIC_TARGET_RENO_RTT {
        if (*path_x).rtt_min as ::core::ffi::c_ulonglong > PICOQUIC_TARGET_SATELLITE_RTT {
            cwnd = (cwnd as ::core::ffi::c_double
                * PICOQUIC_TARGET_SATELLITE_RTT as ::core::ffi::c_double
                / PICOQUIC_TARGET_RENO_RTT as ::core::ffi::c_double) as uint64_t;
        } else {
            cwnd = (cwnd as ::core::ffi::c_double * (*path_x).rtt_min as ::core::ffi::c_double
                / PICOQUIC_TARGET_RENO_RTT as ::core::ffi::c_double) as uint64_t;
        }
    }
    if cwnd < (*bbr_state).bdp_seed {
        cwnd = (*bbr_state).bdp_seed;
    }
    if cwnd > (*path_x).cwin {
        (*path_x).cwin = cwnd;
    }
    (*path_x).set_is_cca_probing_up(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
unsafe extern "C" fn BBRExitStartupLongRtt(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) {
    BBRStartRound(bbr_state, path_x);
    (*bbr_state).round_count += 1;
    (*bbr_state).rounds_since_probe += 1;
    (*bbr_state).set_round_start(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*bbr_state).set_filled_pipe(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if ((*bbr_state).rtt_filter.is_init != 0
        || (*bbr_state).rtt_filter.sample_current > 0 as ::core::ffi::c_int)
        && (*bbr_state).min_rtt > 30000000 as uint64_t
        && (*bbr_state).rtt_filter.sample_max < (*bbr_state).min_rtt
    {
        (*bbr_state).min_rtt = (*bbr_state).rtt_filter.sample_max;
        (*bbr_state).min_rtt_stamp = current_time;
    }
    BBREnterDrain(bbr_state, path_x, current_time);
    BBRCheckDrain(bbr_state, path_x, current_time);
}
#[no_mangle]
pub unsafe extern "C" fn BBRCheckStartupLongRtt(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) {
    if ((*bbr_state).state as ::core::ffi::c_uint
        == picoquic_bbr_alg_startup as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*bbr_state).state as ::core::ffi::c_uint
            == picoquic_bbr_alg_startup_resume as ::core::ffi::c_int as ::core::ffi::c_uint)
        && (*path_x).rtt_min > BBRLongRttThreshold as uint64_t
    {
        BBREnterStartupLongRTT(bbr_state, path_x);
    } else if (*bbr_state).state as ::core::ffi::c_uint
        != picoquic_bbr_alg_startup_long_rtt as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return;
    }
    if picoquic_hystart_test(
        &raw mut (*bbr_state).rtt_filter,
        (*rs).rtt_sample,
        (*path_x).pacing.packet_time_microsec,
        current_time,
        0 as ::core::ffi::c_int,
    ) != 0
    {
        BBRExitStartupLongRtt(bbr_state, path_x, current_time);
    } else if (*rs).ecn_alpha > BBRExcessiveEcnCE {
        BBRExitStartupLongRtt(bbr_state, path_x, current_time);
    } else {
        let mut excessive_loss: ::core::ffi::c_int = picoquic_hystart_loss_volume_test(
            &raw mut (*bbr_state).rtt_filter,
            picoquic_congestion_notification_repeat,
            (*rs).newly_acked,
            (*rs).newly_lost,
        );
        if excessive_loss != 0 {
            BBRExitStartupLongRtt(bbr_state, path_x, current_time);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn BBRUpdateStartupLongRtt(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) {
    if (*path_x).last_time_acked_data_frame_sent > (*path_x).last_sender_limited_time {
        picoquic_hystart_increase(path_x, &raw mut (*bbr_state).rtt_filter, (*rs).newly_acked);
    }
    let mut max_win: uint64_t = (*path_x)
        .peak_bandwidth_estimate
        .wrapping_mul((*bbr_state).min_rtt)
        .wrapping_div(1000000 as uint64_t);
    if max_win < (*bbr_state).bdp_seed {
        max_win = (*bbr_state).bdp_seed;
    }
    max_win = max_win.wrapping_div(2 as uint64_t);
    let mut min_win: uint64_t = max_win;
    if (*path_x).cwin < min_win {
        (*path_x).cwin = min_win;
    }
}
#[no_mangle]
pub unsafe extern "C" fn BBRSetBdpSeed(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut bdp_seed: uint64_t,
) {
    (*bbr_state).bdp_seed = bdp_seed;
    if (*bbr_state).state as ::core::ffi::c_uint
        == picoquic_bbr_alg_startup as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*bbr_state).bdp_seed > (*bbr_state).max_bw
    {
        BBREnterStartupResume(bbr_state);
    }
}
unsafe extern "C" fn BBRUpdateRecoveryOnLoss(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut newly_lost: uint64_t,
) {
    if (*path_x).nb_retransmit >= 1 as uint64_t
        && (*bbr_state).is_in_recovery != 0
        && (*bbr_state).is_pto_recovery != 0
    {
        if (*path_x).cwin > newly_lost {
            (*path_x).cwin = (*path_x).cwin.wrapping_sub(newly_lost);
            if (*path_x).cwin < (2 as uint64_t).wrapping_mul((*path_x).send_mtu as uint64_t) {
                (*path_x).cwin = (2 as size_t).wrapping_mul((*path_x).send_mtu) as uint64_t;
            }
        }
    }
}
unsafe extern "C" fn BBRAccessEcnPacketContext(
    mut path_x: *mut picoquic_path_t,
) -> *mut picoquic_packet_context_t {
    let mut pkt_ctx: *mut picoquic_packet_context_t = (&raw mut (*(*path_x).cnx).pkt_ctx
        as *mut picoquic_packet_context_t)
        .offset(picoquic_packet_context_application as ::core::ffi::c_int as isize)
        as *mut picoquic_packet_context_t;
    if (*(*path_x).cnx).is_multipath_enabled() != 0 {
        pkt_ctx = &raw mut (*path_x).pkt_ctx;
    } else if path_x
        != *(*(*path_x).cnx)
            .path
            .offset(0 as ::core::ffi::c_int as isize)
    {
        pkt_ctx = ::core::ptr::null_mut::<picoquic_packet_context_t>();
    }
    return pkt_ctx;
}
unsafe extern "C" fn BBRComputeEcnFrac(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
) {
    let mut pkt_ctx: *mut picoquic_packet_context_t = BBRAccessEcnPacketContext(path_x);
    let mut delta_ect1: uint64_t = 0 as uint64_t;
    let mut delta_ce: uint64_t = 0 as uint64_t;
    (*rs).ecn_frac = 0.0f64;
    if !pkt_ctx.is_null()
        && (*pkt_ctx).ecn_ect1_total_remote >= (*bbr_state).ecn_ect1_last_round
        && (*pkt_ctx).ecn_ce_total_remote >= (*bbr_state).ecn_ce_last_round
    {
        if (*pkt_ctx).ecn_ect1_total_remote == 0 as uint64_t {
            delta_ect1 = (*rs).delivered.wrapping_div((*path_x).send_mtu as uint64_t);
        } else {
            delta_ect1 = (*pkt_ctx)
                .ecn_ect1_total_remote
                .wrapping_sub((*bbr_state).ecn_ect1_last_round);
            delta_ce = (*pkt_ctx)
                .ecn_ce_total_remote
                .wrapping_sub((*bbr_state).ecn_ce_last_round);
        }
        if delta_ect1.wrapping_add(delta_ce) > 0 as uint64_t {
            (*rs).ecn_ce = delta_ce;
            (*rs).ecn_frac = delta_ce as ::core::ffi::c_double
                / delta_ect1.wrapping_add(delta_ce) as ::core::ffi::c_double;
            (*rs).ecn_alpha = ((*rs).ecn_frac + 15.0f64 * (*bbr_state).ecn_alpha) / 16.0f64;
        }
    }
}
unsafe extern "C" fn BBRAdvanceEcnFrac(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
) {
    if (*bbr_state).round_start() != 0 {
        let mut pkt_ctx: *mut picoquic_packet_context_t = BBRAccessEcnPacketContext(path_x);
        if !pkt_ctx.is_null() {
            if (*pkt_ctx).ecn_ect1_total_remote < (*bbr_state).ecn_ect1_last_round
                || (*pkt_ctx).ecn_ce_total_remote < (*bbr_state).ecn_ce_last_round
            {
                (*bbr_state).ecn_alpha = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
            } else {
                (*bbr_state).ecn_alpha = ((*rs).ecn_frac + 15.0f64 * (*bbr_state).ecn_alpha)
                    / 16 as ::core::ffi::c_int as ::core::ffi::c_double;
            }
            (*bbr_state).ecn_ect1_last_round = (*pkt_ctx).ecn_ect1_total_remote;
            (*bbr_state).ecn_ce_last_round = (*pkt_ctx).ecn_ce_total_remote;
        }
    }
}
unsafe extern "C" fn BBRUpdateModelAndState(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) {
    BBRUpdateLatestDeliverySignals(bbr_state, path_x, rs);
    BBRUpdateCongestionSignals(bbr_state, path_x, rs);
    BBRUpdateACKAggregation(bbr_state, path_x, rs, current_time);
    BBRCheckStartupLongRtt(bbr_state, path_x, rs, current_time);
    BBRCheckStartupResume(bbr_state, path_x, rs, current_time);
    BBRCheckStartupDone(bbr_state, path_x, rs, current_time);
    BBRCheckRecovery(bbr_state, path_x, rs, current_time);
    BBRCheckDrain(bbr_state, path_x, current_time);
    BBRUpdateProbeBWCyclePhase(bbr_state, path_x, rs, current_time);
    BBRUpdateMinRTT(bbr_state, path_x, rs, current_time);
    BBRCheckProbeRTT(bbr_state, path_x, rs, current_time);
    BBRAdvanceLatestDeliverySignals(bbr_state, rs);
    BBRAdvanceEcnFrac(bbr_state, path_x, rs);
    BBRBoundBWForModel(bbr_state);
}
unsafe extern "C" fn BBRUpdateControlParameters(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
) {
    BBRSetPacingRate(bbr_state);
    BBRSetSendQuantum(bbr_state, path_x);
    BBRSetCwnd(bbr_state, path_x, rs);
}
#[no_mangle]
pub unsafe extern "C" fn BBRUpdateOnACK(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut rs: *mut bbr_per_ack_state_t,
    mut current_time: uint64_t,
) {
    BBRUpdateModelAndState(bbr_state, path_x, rs, current_time);
    if (*bbr_state).state as ::core::ffi::c_uint
        == picoquic_bbr_alg_startup_long_rtt as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        BBRUpdateStartupLongRtt(bbr_state, path_x, rs, current_time);
    } else {
        BBRUpdateControlParameters(bbr_state, path_x, rs);
    };
}
unsafe extern "C" fn BBRSetRsFromAckState(
    mut path_x: *mut picoquic_path_t,
    mut ack_state: *mut picoquic_per_ack_state_t,
    mut rs: *mut bbr_per_ack_state_t,
) {
    if (*path_x).bandwidth_estimate > 0 as uint64_t {
        (*rs).delivery_rate = (*path_x).bandwidth_estimate;
    } else if (*ack_state).rtt_measurement > 0 as uint64_t {
        (*rs).delivery_rate = (1000000 as uint64_t)
            .wrapping_mul((*ack_state).nb_bytes_delivered_since_packet_sent)
            .wrapping_div((*ack_state).rtt_measurement);
    } else {
        (*rs).delivery_rate = 40000 as uint64_t;
    }
    (*rs).delivered = (*ack_state).nb_bytes_delivered_since_packet_sent;
    (*rs).rtt_sample = (*path_x).rtt_sample;
    (*rs).newly_acked = (*ack_state).nb_bytes_acknowledged;
    (*rs).newly_lost = (*ack_state).nb_bytes_newly_lost;
    (*rs).lost = (*ack_state).nb_bytes_lost_since_packet_sent;
    (*rs).tx_in_flight = (*ack_state).inflight_prior;
    (*rs).set_is_app_limited((*ack_state).is_app_limited() as ::core::ffi::c_uint);
    (*rs).set_is_cwnd_limited((*ack_state).is_cwnd_limited() as ::core::ffi::c_uint);
}
unsafe extern "C" fn picoquic_bbr_notify_ack(
    mut bbr_state: *mut picoquic_bbr_state_t,
    mut path_x: *mut picoquic_path_t,
    mut ack_state: *mut picoquic_per_ack_state_t,
    mut current_time: uint64_t,
) {
    let mut rs: bbr_per_ack_state_t = {
        let mut init = st_bbr_per_ack_state_t {
            is_app_limited_is_cwnd_limited: [0; 1],
            c2rust_padding: [0; 7],
            delivered: 0 as uint64_t,
            delivery_rate: 0,
            rtt_sample: 0,
            newly_acked: 0,
            newly_lost: 0,
            tx_in_flight: 0,
            lost: 0,
            ecn_ce: 0,
            ecn_frac: 0.,
            ecn_alpha: 0.,
        };
        init.set_is_app_limited(0);
        init.set_is_cwnd_limited(0);
        init
    };
    BBRSetRsFromAckState(path_x, ack_state, &raw mut rs);
    BBRComputeEcnFrac(bbr_state, path_x, &raw mut rs);
    BBRUpdateOnACK(bbr_state, path_x, &raw mut rs, current_time);
}
unsafe extern "C" fn picoquic_bbr_notify(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut notification: picoquic_congestion_notification_t,
    mut ack_state: *mut picoquic_per_ack_state_t,
    mut current_time: uint64_t,
) {
    let mut bbr_state: *mut picoquic_bbr_state_t =
        (*path_x).congestion_alg_state as *mut picoquic_bbr_state_t;
    (*path_x).set_is_cc_data_updated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if !bbr_state.is_null() {
        match notification as ::core::ffi::c_uint {
            1 => {
                BBRUpdateRecoveryOnLoss(bbr_state, path_x, (*ack_state).nb_bytes_newly_lost);
            }
            2 => {
                BBRExitLostFeedback(bbr_state, path_x);
                BBROnEnterRTO(bbr_state, path_x, (*ack_state).lost_packet_number);
            }
            3 => {
                BBROnSpuriousLoss(
                    bbr_state,
                    path_x,
                    (*ack_state).lost_packet_number,
                    current_time,
                );
            }
            9 => {
                if !((*bbr_state).exp_flags.do_control_lost() != 0) {
                    BBREnterLostFeedback(bbr_state, path_x);
                }
            }
            0 => {
                BBRExitLostFeedback(bbr_state, path_x);
                picoquic_bbr_notify_ack(bbr_state, path_x, ack_state, current_time);
                if (*bbr_state).state as ::core::ffi::c_uint
                    == picoquic_bbr_alg_startup_long_rtt as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                {
                    picoquic_update_pacing_data(cnx, path_x, 1 as ::core::ffi::c_int);
                } else if (*bbr_state).pacing_rate
                    > 0 as ::core::ffi::c_int as ::core::ffi::c_double
                {
                    picoquic_update_pacing_rate(
                        cnx,
                        path_x,
                        (*bbr_state).pacing_rate,
                        (*bbr_state).send_quantum,
                    );
                }
            }
            8 => {
                picoquic_bbr_reset(bbr_state, path_x, current_time);
            }
            7 => {
                BBRSetBdpSeed(bbr_state, (*ack_state).nb_bytes_acknowledged);
            }
            5 | 4 | 6 | _ => {}
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_bbr_observe(
    mut path_x: *mut picoquic_path_t,
    mut cc_state: *mut uint64_t,
    mut cc_param: *mut uint64_t,
) {
    let mut bbr_state: *mut picoquic_bbr_state_t =
        (*path_x).congestion_alg_state as *mut picoquic_bbr_state_t;
    *cc_state = (*bbr_state).state as uint64_t;
    *cc_param = (*bbr_state).bw;
}
pub const picoquic_bbr_ID: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"bbr\0") };
#[no_mangle]
pub static mut picoquic_bbr_algorithm_struct: picoquic_congestion_algorithm_t = unsafe {
    st_picoquic_congestion_algorithm_t {
        congestion_algorithm_id: picoquic_bbr_ID.as_ptr(),
        congestion_algorithm_number: PICOQUIC_CC_ALGO_NUMBER_BBR as uint8_t,
        alg_init: Some(
            picoquic_bbr_init
                as unsafe extern "C" fn(*mut picoquic_cnx_t, *mut picoquic_path_t, uint64_t) -> (),
        ),
        alg_notify: Some(
            picoquic_bbr_notify
                as unsafe extern "C" fn(
                    *mut picoquic_cnx_t,
                    *mut picoquic_path_t,
                    picoquic_congestion_notification_t,
                    *mut picoquic_per_ack_state_t,
                    uint64_t,
                ) -> (),
        ),
        alg_delete: Some(picoquic_bbr_delete as unsafe extern "C" fn(*mut picoquic_path_t) -> ()),
        alg_observe: Some(
            picoquic_bbr_observe
                as unsafe extern "C" fn(*mut picoquic_path_t, *mut uint64_t, *mut uint64_t) -> (),
        ),
    }
};
#[no_mangle]
pub static mut picoquic_bbr_algorithm: *mut picoquic_congestion_algorithm_t =
    unsafe { &raw const picoquic_bbr_algorithm_struct as *mut picoquic_congestion_algorithm_t };

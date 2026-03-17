use ::c2rust_bitfields;
extern "C" {
    pub type st_ptls_iovec_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type st_ptls_buffer_t;
    pub type st_ptls_verify_certificate_t;
    fn picoquic_log_app_message(cnx: *mut picoquic_cnx_t, fmt: *const ::core::ffi::c_char, ...);
    fn picoquic_set_cipher_suite(
        quic: *mut picoquic_quic_t,
        cipher_suite_id: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    static mut picoquic_newreno_algorithm: *mut picoquic_congestion_algorithm_t;
    static mut picoquic_cubic_algorithm: *mut picoquic_congestion_algorithm_t;
    static mut picoquic_dcubic_algorithm: *mut picoquic_congestion_algorithm_t;
    static mut picoquic_fastcc_algorithm: *mut picoquic_congestion_algorithm_t;
    static mut picoquic_bbr_algorithm: *mut picoquic_congestion_algorithm_t;
    static mut picoquic_prague_algorithm: *mut picoquic_congestion_algorithm_t;
    static mut picoquic_bbr1_algorithm: *mut picoquic_congestion_algorithm_t;
    fn picohash_create_ex(
        nb_bin: size_t,
        picohash_hash: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> uint64_t>,
        picohash_compare: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
        picohash_key_to_item: Option<
            unsafe extern "C" fn(*const ::core::ffi::c_void) -> *mut picohash_item,
        >,
    ) -> *mut picohash_table;
    fn picohash_retrieve(
        hash_table: *mut picohash_table,
        key: *const ::core::ffi::c_void,
    ) -> *mut picohash_item;
    fn picohash_insert(
        hash_table: *mut picohash_table,
        key: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn picohash_delete_item(
        hash_table: *mut picohash_table,
        item: *mut picohash_item,
        delete_key_too: ::core::ffi::c_int,
    );
    fn picohash_delete_key(
        hash_table: *mut picohash_table,
        key: *mut ::core::ffi::c_void,
        delete_key_too: ::core::ffi::c_int,
    );
    fn picohash_delete(hash_table: *mut picohash_table, delete_key_too: ::core::ffi::c_int);
    fn picohash_hash_mix(hash: uint64_t, h2: uint64_t) -> uint64_t;
    fn picohash_bytes(key: *const uint8_t, length: uint32_t) -> uint64_t;
    fn picosplay_init_tree(
        tree: *mut picosplay_tree_t,
        comp: picosplay_comparator,
        create: picosplay_create,
        delete_node: picosplay_delete_node,
        node_value: picosplay_node_value,
    );
    fn picosplay_insert(
        tree: *mut picosplay_tree_t,
        value: *mut ::core::ffi::c_void,
    ) -> *mut picosplay_node_t;
    fn picosplay_find(
        tree: *mut picosplay_tree_t,
        value: *mut ::core::ffi::c_void,
    ) -> *mut picosplay_node_t;
    fn picosplay_first(tree: *mut picosplay_tree_t) -> *mut picosplay_node_t;
    fn picosplay_next(node: *mut picosplay_node_t) -> *mut picosplay_node_t;
    fn picosplay_last(tree: *mut picosplay_tree_t) -> *mut picosplay_node_t;
    fn picosplay_delete(tree: *mut picosplay_tree_t, value: *mut ::core::ffi::c_void);
    fn picosplay_delete_hint(tree: *mut picosplay_tree_t, node: *mut picosplay_node_t);
    fn picosplay_empty_tree(tree: *mut picosplay_tree_t);
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn picoquic_string_duplicate(original: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    fn picoquic_string_free(str: *mut ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static picoquic_null_connection_id: picoquic_connection_id_t;
    fn picoquic_parse_connection_id(
        bytes: *const uint8_t,
        len: uint8_t,
        cnx_id: *mut picoquic_connection_id_t,
    ) -> uint8_t;
    fn picoquic_is_connection_id_null(
        cnx_id: *const picoquic_connection_id_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_compare_connection_id(
        cnx_id1: *const picoquic_connection_id_t,
        cnx_id2: *const picoquic_connection_id_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_connection_id_hash(cid: *const picoquic_connection_id_t) -> uint64_t;
    fn picoquic_hash_addr(addr: *const sockaddr) -> uint64_t;
    fn picoquic_compare_addr(
        expected: *const sockaddr,
        actual: *const sockaddr,
    ) -> ::core::ffi::c_int;
    fn picoquic_store_addr(stored_addr: *mut sockaddr_storage, addr: *const sockaddr);
    fn picoquic_addr_text(
        addr: *const sockaddr,
        text: *mut ::core::ffi::c_char,
        text_size: size_t,
    ) -> *const ::core::ffi::c_char;
    fn picoquic_load_tickets(
        quic: *mut picoquic_quic_t,
        ticket_file_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn picoquic_free_tickets(pp_first_ticket: *mut *mut picoquic_stored_ticket_t);
    fn picoquic_load_tokens(
        quic: *mut picoquic_quic_t,
        token_file_name: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn picoquic_free_tokens(pp_first_token: *mut *mut picoquic_stored_token_t);
    fn picoquic_dequeue_retransmit_packet(
        cnx: *mut picoquic_cnx_t,
        pkt_ctx: *mut picoquic_packet_context_t,
        p: *mut picoquic_packet_t,
        should_free: ::core::ffi::c_int,
        add_to_data_repeat_queue: ::core::ffi::c_int,
    ) -> *mut picoquic_packet_t;
    fn picoquic_dequeue_retransmitted_packet(
        cnx: *mut picoquic_cnx_t,
        pkt_ctx: *mut picoquic_packet_context_t,
        p: *mut picoquic_packet_t,
    );
    fn picoquic_pacing_init(pacing: *mut picoquic_pacing_t, current_time: uint64_t);
    fn picoquic_update_pacing_parameters(
        pacing: *mut picoquic_pacing_t,
        pacing_rate: ::core::ffi::c_double,
        quantum: uint64_t,
        send_mtu: size_t,
        smoothed_rtt: uint64_t,
        signalled_path: *mut picoquic_path_t,
    );
    fn picoquic_sack_list_last(first_sack: *mut picoquic_sack_list_t) -> uint64_t;
    fn picoquic_sack_list_init(first_sack: *mut picoquic_sack_list_t);
    fn picoquic_sack_list_free(first_sack: *mut picoquic_sack_list_t);
    fn picoquic_delete_stream_if_closed(
        cnx: *mut picoquic_cnx_t,
        stream: *mut picoquic_stream_head_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_queue_data_repeat_init(cnx: *mut picoquic_cnx_t);
    fn picoquic_queue_retire_connection_id_frame(
        cnx: *mut picoquic_cnx_t,
        unique_path_id: uint64_t,
        sequence: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_queue_path_abandon_frame(
        cnx: *mut picoquic_cnx_t,
        unique_path_id: uint64_t,
        reason: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_queue_path_available_or_standby_frame(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
        status: picoquic_path_status_enum,
    ) -> ::core::ffi::c_int;
    fn picoquic_log_quic_pdu(
        quic: *mut picoquic_quic_t,
        receiving: ::core::ffi::c_int,
        current_time: uint64_t,
        cid64: uint64_t,
        addr_peer: *const sockaddr,
        addr_local: *const sockaddr,
        packet_length: size_t,
    );
    fn picoquic_log_close_logs(quic: *mut picoquic_quic_t);
    fn picoquic_log_new_connection(cnx: *mut picoquic_cnx_t);
    fn picoquic_log_close_connection(cnx: *mut picoquic_cnx_t);
    fn picoquic_master_tlscontext(
        quic: *mut picoquic_quic_t,
        cert_file_name: *const ::core::ffi::c_char,
        key_file_name: *const ::core::ffi::c_char,
        cert_root_file_name: *const ::core::ffi::c_char,
        ticket_key: *const uint8_t,
        ticket_key_length: size_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_master_tlscontext_free(quic: *mut picoquic_quic_t);
    fn picoquic_tlscontext_create(
        quic: *mut picoquic_quic_t,
        cnx: *mut picoquic_cnx_t,
        current_time: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_tlscontext_free(ctx: *mut ::core::ffi::c_void);
    fn picoquic_initialize_tls_stream(
        cnx: *mut picoquic_cnx_t,
        current_time: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_crypto_random(
        quic: *mut picoquic_quic_t,
        buf: *mut ::core::ffi::c_void,
        len: size_t,
    );
    fn picoquic_crypto_uniform_random(quic: *mut picoquic_quic_t, rnd_max: uint64_t) -> uint64_t;
    fn picoquic_public_random_64() -> uint64_t;
    fn picoquic_public_random(buf: *mut ::core::ffi::c_void, len: size_t);
    fn picoquic_public_uniform_random(rnd_max: uint64_t) -> uint64_t;
    fn picoquic_aead_free(aead_context: *mut ::core::ffi::c_void);
    fn picoquic_setup_initial_traffic_keys(cnx: *mut picoquic_cnx_t) -> ::core::ffi::c_int;
    fn picoquic_compute_new_rotated_keys(cnx: *mut picoquic_cnx_t) -> ::core::ffi::c_int;
    fn picoquic_apply_rotated_keys(cnx: *mut picoquic_cnx_t, is_enc: ::core::ffi::c_int);
    fn picoquic_crypto_context_free(ctx: *mut picoquic_crypto_context_t);
    fn picoquic_create_cnxid_reset_secret(
        quic: *mut picoquic_quic_t,
        cnx_id: *mut picoquic_connection_id_t,
        reset_secret: *mut uint8_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_tls_set_verify_certificate_callback(
        quic: *mut picoquic_quic_t,
        cb: *mut st_ptls_verify_certificate_t,
        free_fn: picoquic_free_verify_certificate_ctx,
    );
    fn picoquic_dispose_verify_certificate_callback(quic: *mut picoquic_quic_t);
    fn picoquic_tls_set_client_authentication(
        quic: *mut picoquic_quic_t,
        client_authentication: ::core::ffi::c_int,
    );
    fn picoquic_delete_retry_protection_contexts(quic: *mut picoquic_quic_t);
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
}
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
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
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
pub type picoquic_path_status_enum = ::core::ffi::c_uint;
pub const picoquic_path_status_standby: picoquic_path_status_enum = 1;
pub const picoquic_path_status_available: picoquic_path_status_enum = 0;
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
pub type picoquic_registered_token_t = st_picoquic_registered_token_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_registered_token_t {
    pub registered_token_node: picosplay_node_t,
    pub token_time: uint64_t,
    pub token_hash: uint64_t,
    pub count: ::core::ffi::c_int,
}
pub type picoquic_version_parameters_t = st_picoquic_version_parameters_t;
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
pub const picoquic_frame_type_path_challenge: C2Rust_Unnamed = 26;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_path_quality_t {
    pub receive_rate_estimate: uint64_t,
    pub pacing_rate: uint64_t,
    pub cwin: uint64_t,
    pub rtt: uint64_t,
    pub rtt_sample: uint64_t,
    pub rtt_variant: uint64_t,
    pub rtt_min: uint64_t,
    pub rtt_max: uint64_t,
    pub sent: uint64_t,
    pub lost: uint64_t,
    pub timer_losses: uint64_t,
    pub spurious_losses: uint64_t,
    pub max_spurious_rtt: uint64_t,
    pub max_reorder_delay: uint64_t,
    pub max_reorder_gap: uint64_t,
    pub bytes_in_transit: uint64_t,
}
pub type picoquic_path_quality_t = st_picoquic_path_quality_t;
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
pub const INT64_MAX: ::core::ffi::c_long = 9223372036854775807 as ::core::ffi::c_long;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const PICOQUIC_ERROR_CLASS: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_MEMORY: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 5 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_CNXID_CHECK: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 7 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_INVALID_STREAM_ID: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 14 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_DETECTED: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 21 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_NO_SUCH_FILE: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 29 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_CNXID_NOT_AVAILABLE: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 33 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_MIGRATION_DISABLED: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 34 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_CANNOT_CHANGE_ACTIVE_CONTEXT: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 37 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_KEY_ROTATION_NOT_READY: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 40 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_NO_CALLBACK_PROVIDED: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 43 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_INTERNAL_ERROR: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR: ::core::ffi::c_int = 0x7 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_CONNECTION_ID_LIMIT_ERROR: ::core::ffi::c_int =
    0x9 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION: ::core::ffi::c_int = 0xa as ::core::ffi::c_int;
pub const PICOQUIC_TLS_HANDSHAKE_FAILED: ::core::ffi::c_int = 0x201 as ::core::ffi::c_int;
pub const PICOQUIC_MAX_PACKET_SIZE: ::core::ffi::c_int = 1536 as ::core::ffi::c_int;
pub const PICOQUIC_INITIAL_MTU_IPV4: ::core::ffi::c_int = 1252 as ::core::ffi::c_int;
pub const PICOQUIC_INITIAL_MTU_IPV6: ::core::ffi::c_int = 1232 as ::core::ffi::c_int;
pub const PICOQUIC_RESET_SECRET_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PICOQUIC_RESET_PACKET_PAD_SIZE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const PICOQUIC_RESET_PACKET_MIN_SIZE: ::core::ffi::c_int =
    PICOQUIC_RESET_PACKET_PAD_SIZE + PICOQUIC_RESET_SECRET_SIZE;
pub const PICOQUIC_LOG_PACKET_MAX_SEQUENCE: ::core::ffi::c_int = 100 as ::core::ffi::c_int;
pub const PICOQUIC_CONNECTION_ID_MAX_SIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const PICOQUIC_DEFAULT_STREAM_PRIORITY: ::core::ffi::c_int = 9 as ::core::ffi::c_int;
pub const PICOQUIC_PRACTICAL_MAX_MTU: ::core::ffi::c_int = 1440 as ::core::ffi::c_int;
pub const PICOQUIC_NB_PATH_TARGET: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PICOQUIC_MAX_PACKETS_IN_POOL: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const PICOQUIC_STORED_IP_MAX: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PICOQUIC_INITIAL_RTT: ::core::ffi::c_ulonglong = 250000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_INITIAL_RETRANSMIT_TIMER: ::core::ffi::c_ulonglong =
    250000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_INITIAL_MAX_RETRANSMIT_TIMER: ::core::ffi::c_ulonglong =
    1000000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_ACK_DELAY_MAX: ::core::ffi::c_ulonglong = 100000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_ACK_DELAY_MAX_DEFAULT: ::core::ffi::c_ulonglong =
    25000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_ACK_DELAY_MIN: ::core::ffi::c_ulonglong = 1000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_MICROSEC_HANDSHAKE_MAX: ::core::ffi::c_ulonglong =
    30000000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_MICROSEC_STATELESS_RESET_INTERVAL_DEFAULT: ::core::ffi::c_ulonglong =
    100000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_CWIN_INITIAL: ::core::ffi::c_int =
    10 as ::core::ffi::c_int * PICOQUIC_MAX_PACKET_SIZE;
pub const PICOQUIC_PRIORITY_BYPASS_MAX_RATE: ::core::ffi::c_int = 125000 as ::core::ffi::c_int;
pub const PICOQUIC_PRIORITY_BYPASS_QUANTUM: ::core::ffi::c_int = 2560 as ::core::ffi::c_int;
pub const PICOQUIC_DEFAULT_CRYPTO_EPOCH_LENGTH: ::core::ffi::c_int =
    (1 as ::core::ffi::c_int) << 22 as ::core::ffi::c_int;
pub const PICOQUIC_DEFAULT_SIMULTANEOUS_LOGS: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const PICOQUIC_DEFAULT_HALF_OPEN_RETRY_THRESHOLD: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const PICOQUIC_PN_RANDOM_MIN: ::core::ffi::c_int = 0xffff as ::core::ffi::c_int;
pub const PICOQUIC_PN_RANDOM_RANGE: ::core::ffi::c_int = 0x10000 as ::core::ffi::c_int;
pub const PICOQUIC_SPIN_RESERVE_MOD_256: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const PICOQUIC_CHALLENGE_REPEAT_MAX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PICOQUIC_DEFAULT_HOLE_PERIOD: ::core::ffi::c_int = 256 as ::core::ffi::c_int;
pub const PICOQUIC_SEVENTEENTH_INTEROP_VERSION: ::core::ffi::c_uint =
    0xff00001b as ::core::ffi::c_uint;
pub const PICOQUIC_EIGHTEENTH_INTEROP_VERSION: ::core::ffi::c_uint =
    0xff00001c as ::core::ffi::c_uint;
pub const PICOQUIC_NINETEENTH_INTEROP_VERSION: ::core::ffi::c_uint =
    0xff00001d as ::core::ffi::c_uint;
pub const PICOQUIC_NINETEENTH_BIS_INTEROP_VERSION: ::core::ffi::c_uint =
    0xff00001e as ::core::ffi::c_uint;
pub const PICOQUIC_TWENTIETH_PRE_INTEROP_VERSION: ::core::ffi::c_uint =
    0xff00001f as ::core::ffi::c_uint;
pub const PICOQUIC_TWENTIETH_INTEROP_VERSION: ::core::ffi::c_uint =
    0xff000020 as ::core::ffi::c_uint;
pub const PICOQUIC_TWENTYFIRST_INTEROP_VERSION: ::core::ffi::c_uint =
    0xff000021 as ::core::ffi::c_uint;
pub const PICOQUIC_POST_IESG_VERSION: ::core::ffi::c_uint = 0xff000022 as ::core::ffi::c_uint;
pub const PICOQUIC_V1_VERSION: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PICOQUIC_V2_VERSION: ::core::ffi::c_int = 0x6b3343cf as ::core::ffi::c_int;
pub const PICOQUIC_V2_VERSION_DRAFT: ::core::ffi::c_int = 0x709a50c4 as ::core::ffi::c_int;
pub const PICOQUIC_INTERNAL_TEST_VERSION_1: ::core::ffi::c_int = 0x50435130 as ::core::ffi::c_int;
pub const PICOQUIC_INTERNAL_TEST_VERSION_2: ::core::ffi::c_int = 0x50435131 as ::core::ffi::c_int;
pub const PICOQUIC_INTEROP_VERSION_INDEX: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PICOQUIC_NUMBER_OF_EPOCHS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PICOQUIC_LABEL_V1_TRAFFIC_UPDATE: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"quic ku\0") };
pub const PICOQUIC_LABEL_V2_TRAFFIC_UPDATE: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"quicv2 ku\0") };
pub const PICOQUIC_LABEL_QUIC_V1_KEY_BASE: [::core::ffi::c_char; 12] =
    unsafe { ::core::mem::transmute::<[u8; 12], [::core::ffi::c_char; 12]>(*b"tls13 quic \0") };
pub const PICOQUIC_LABEL_QUIC_V2_KEY_BASE: [::core::ffi::c_char; 14] =
    unsafe { ::core::mem::transmute::<[u8; 14], [::core::ffi::c_char; 14]>(*b"tls13 quicv2 \0") };
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const CLOCK_MONOTONIC: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut picoquic_cleartext_internal_test_1_salt: [uint8_t; 20] = [
    0x30 as ::core::ffi::c_int as uint8_t,
    0x67 as ::core::ffi::c_int as uint8_t,
    0x16 as ::core::ffi::c_int as uint8_t,
    0xd7 as ::core::ffi::c_int as uint8_t,
    0x63 as ::core::ffi::c_int as uint8_t,
    0x75 as ::core::ffi::c_int as uint8_t,
    0xd5 as ::core::ffi::c_int as uint8_t,
    0x55 as ::core::ffi::c_int as uint8_t,
    0x4b as ::core::ffi::c_int as uint8_t,
    0x2f as ::core::ffi::c_int as uint8_t,
    0x60 as ::core::ffi::c_int as uint8_t,
    0x5e as ::core::ffi::c_int as uint8_t,
    0xef as ::core::ffi::c_int as uint8_t,
    0x78 as ::core::ffi::c_int as uint8_t,
    0xd8 as ::core::ffi::c_int as uint8_t,
    0x33 as ::core::ffi::c_int as uint8_t,
    0x3d as ::core::ffi::c_int as uint8_t,
    0xc1 as ::core::ffi::c_int as uint8_t,
    0xca as ::core::ffi::c_int as uint8_t,
    0x36 as ::core::ffi::c_int as uint8_t,
];
static mut picoquic_cleartext_draft_23_salt: [uint8_t; 20] = [
    0xc3 as ::core::ffi::c_int as uint8_t,
    0xee as ::core::ffi::c_int as uint8_t,
    0xf7 as ::core::ffi::c_int as uint8_t,
    0x12 as ::core::ffi::c_int as uint8_t,
    0xc7 as ::core::ffi::c_int as uint8_t,
    0x2e as ::core::ffi::c_int as uint8_t,
    0xbb as ::core::ffi::c_int as uint8_t,
    0x5a as ::core::ffi::c_int as uint8_t,
    0x11 as ::core::ffi::c_int as uint8_t,
    0xa7 as ::core::ffi::c_int as uint8_t,
    0xd2 as ::core::ffi::c_int as uint8_t,
    0x43 as ::core::ffi::c_int as uint8_t,
    0x2b as ::core::ffi::c_int as uint8_t,
    0xb4 as ::core::ffi::c_int as uint8_t,
    0x63 as ::core::ffi::c_int as uint8_t,
    0x65 as ::core::ffi::c_int as uint8_t,
    0xbe as ::core::ffi::c_int as uint8_t,
    0xf9 as ::core::ffi::c_int as uint8_t,
    0xf5 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub static mut picoquic_retry_protection_key_25: [uint8_t; 32] = [
    0x65 as ::core::ffi::c_int as uint8_t,
    0x6e as ::core::ffi::c_int as uint8_t,
    0x61 as ::core::ffi::c_int as uint8_t,
    0xe3 as ::core::ffi::c_int as uint8_t,
    0x36 as ::core::ffi::c_int as uint8_t,
    0xae as ::core::ffi::c_int as uint8_t,
    0x94 as ::core::ffi::c_int as uint8_t,
    0x17 as ::core::ffi::c_int as uint8_t,
    0xf7 as ::core::ffi::c_int as uint8_t,
    0xf0 as ::core::ffi::c_int as uint8_t,
    0xed as ::core::ffi::c_int as uint8_t,
    0xd8 as ::core::ffi::c_int as uint8_t,
    0xd7 as ::core::ffi::c_int as uint8_t,
    0x8d as ::core::ffi::c_int as uint8_t,
    0x46 as ::core::ffi::c_int as uint8_t,
    0x1e as ::core::ffi::c_int as uint8_t,
    0x2a as ::core::ffi::c_int as uint8_t,
    0xa7 as ::core::ffi::c_int as uint8_t,
    0x8 as ::core::ffi::c_int as uint8_t,
    0x4a as ::core::ffi::c_int as uint8_t,
    0xba as ::core::ffi::c_int as uint8_t,
    0x7a as ::core::ffi::c_int as uint8_t,
    0x14 as ::core::ffi::c_int as uint8_t,
    0xc1 as ::core::ffi::c_int as uint8_t,
    0xe9 as ::core::ffi::c_int as uint8_t,
    0xf7 as ::core::ffi::c_int as uint8_t,
    0x26 as ::core::ffi::c_int as uint8_t,
    0xd5 as ::core::ffi::c_int as uint8_t,
    0x57 as ::core::ffi::c_int as uint8_t,
    0x9 as ::core::ffi::c_int as uint8_t,
    0x16 as ::core::ffi::c_int as uint8_t,
    0x9a as ::core::ffi::c_int as uint8_t,
];
static mut picoquic_cleartext_draft_29_salt: [uint8_t; 20] = [
    0xaf as ::core::ffi::c_int as uint8_t,
    0xbf as ::core::ffi::c_int as uint8_t,
    0xec as ::core::ffi::c_int as uint8_t,
    0x28 as ::core::ffi::c_int as uint8_t,
    0x99 as ::core::ffi::c_int as uint8_t,
    0x93 as ::core::ffi::c_int as uint8_t,
    0xd2 as ::core::ffi::c_int as uint8_t,
    0x4c as ::core::ffi::c_int as uint8_t,
    0x9e as ::core::ffi::c_int as uint8_t,
    0x97 as ::core::ffi::c_int as uint8_t,
    0x86 as ::core::ffi::c_int as uint8_t,
    0xf1 as ::core::ffi::c_int as uint8_t,
    0x9c as ::core::ffi::c_int as uint8_t,
    0x61 as ::core::ffi::c_int as uint8_t,
    0x11 as ::core::ffi::c_int as uint8_t,
    0xe0 as ::core::ffi::c_int as uint8_t,
    0x43 as ::core::ffi::c_int as uint8_t,
    0x90 as ::core::ffi::c_int as uint8_t,
    0xa8 as ::core::ffi::c_int as uint8_t,
    0x99 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub static mut picoquic_retry_protection_key_29: [uint8_t; 32] = [
    0x8b as ::core::ffi::c_int as uint8_t,
    0xd as ::core::ffi::c_int as uint8_t,
    0x37 as ::core::ffi::c_int as uint8_t,
    0xeb as ::core::ffi::c_int as uint8_t,
    0x85 as ::core::ffi::c_int as uint8_t,
    0x35 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x2e as ::core::ffi::c_int as uint8_t,
    0xbc as ::core::ffi::c_int as uint8_t,
    0x8d as ::core::ffi::c_int as uint8_t,
    0x76 as ::core::ffi::c_int as uint8_t,
    0xa2 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0xd8 as ::core::ffi::c_int as uint8_t,
    0xd as ::core::ffi::c_int as uint8_t,
    0xf2 as ::core::ffi::c_int as uint8_t,
    0x26 as ::core::ffi::c_int as uint8_t,
    0x46 as ::core::ffi::c_int as uint8_t,
    0xec as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0xdc as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0x96 as ::core::ffi::c_int as uint8_t,
    0x42 as ::core::ffi::c_int as uint8_t,
    0xc3 as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0x8b as ::core::ffi::c_int as uint8_t,
    0xaa as ::core::ffi::c_int as uint8_t,
    0x2b as ::core::ffi::c_int as uint8_t,
    0xaa as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0x4c as ::core::ffi::c_int as uint8_t,
];
static mut picoquic_cleartext_v1_salt: [uint8_t; 20] = [
    0x38 as ::core::ffi::c_int as uint8_t,
    0x76 as ::core::ffi::c_int as uint8_t,
    0x2c as ::core::ffi::c_int as uint8_t,
    0xf7 as ::core::ffi::c_int as uint8_t,
    0xf5 as ::core::ffi::c_int as uint8_t,
    0x59 as ::core::ffi::c_int as uint8_t,
    0x34 as ::core::ffi::c_int as uint8_t,
    0xb3 as ::core::ffi::c_int as uint8_t,
    0x4d as ::core::ffi::c_int as uint8_t,
    0x17 as ::core::ffi::c_int as uint8_t,
    0x9a as ::core::ffi::c_int as uint8_t,
    0xe6 as ::core::ffi::c_int as uint8_t,
    0xa4 as ::core::ffi::c_int as uint8_t,
    0xc8 as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xad as ::core::ffi::c_int as uint8_t,
    0xcc as ::core::ffi::c_int as uint8_t,
    0xbb as ::core::ffi::c_int as uint8_t,
    0x7f as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub static mut picoquic_retry_protection_v1: [uint8_t; 32] = [
    0xd9 as ::core::ffi::c_int as uint8_t,
    0xc9 as ::core::ffi::c_int as uint8_t,
    0x94 as ::core::ffi::c_int as uint8_t,
    0x3e as ::core::ffi::c_int as uint8_t,
    0x61 as ::core::ffi::c_int as uint8_t,
    0x1 as ::core::ffi::c_int as uint8_t,
    0xfd as ::core::ffi::c_int as uint8_t,
    0x20 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0x21 as ::core::ffi::c_int as uint8_t,
    0x50 as ::core::ffi::c_int as uint8_t,
    0x6b as ::core::ffi::c_int as uint8_t,
    0xcc as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x81 as ::core::ffi::c_int as uint8_t,
    0x4c as ::core::ffi::c_int as uint8_t,
    0x73 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0xf as ::core::ffi::c_int as uint8_t,
    0x25 as ::core::ffi::c_int as uint8_t,
    0xc7 as ::core::ffi::c_int as uint8_t,
    0x9d as ::core::ffi::c_int as uint8_t,
    0x71 as ::core::ffi::c_int as uint8_t,
    0xce as ::core::ffi::c_int as uint8_t,
    0x87 as ::core::ffi::c_int as uint8_t,
    0x6e as ::core::ffi::c_int as uint8_t,
    0xca as ::core::ffi::c_int as uint8_t,
    0x87 as ::core::ffi::c_int as uint8_t,
    0x6e as ::core::ffi::c_int as uint8_t,
    0x6f as ::core::ffi::c_int as uint8_t,
    0xca as ::core::ffi::c_int as uint8_t,
    0x8e as ::core::ffi::c_int as uint8_t,
];
static mut picoquic_cleartext_v2_salt: [uint8_t; 20] = [
    0xd as ::core::ffi::c_int as uint8_t,
    0xed as ::core::ffi::c_int as uint8_t,
    0xe3 as ::core::ffi::c_int as uint8_t,
    0xde as ::core::ffi::c_int as uint8_t,
    0xf7 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0xa6 as ::core::ffi::c_int as uint8_t,
    0xdb as ::core::ffi::c_int as uint8_t,
    0x81 as ::core::ffi::c_int as uint8_t,
    0x93 as ::core::ffi::c_int as uint8_t,
    0x81 as ::core::ffi::c_int as uint8_t,
    0xbe as ::core::ffi::c_int as uint8_t,
    0x6e as ::core::ffi::c_int as uint8_t,
    0x26 as ::core::ffi::c_int as uint8_t,
    0x9d as ::core::ffi::c_int as uint8_t,
    0xcb as ::core::ffi::c_int as uint8_t,
    0xf9 as ::core::ffi::c_int as uint8_t,
    0xbd as ::core::ffi::c_int as uint8_t,
    0x2e as ::core::ffi::c_int as uint8_t,
    0xd9 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub static mut picoquic_retry_protection_v2: [uint8_t; 32] = [
    0xc4 as ::core::ffi::c_int as uint8_t,
    0xdd as ::core::ffi::c_int as uint8_t,
    0x24 as ::core::ffi::c_int as uint8_t,
    0x84 as ::core::ffi::c_int as uint8_t,
    0xd6 as ::core::ffi::c_int as uint8_t,
    0x81 as ::core::ffi::c_int as uint8_t,
    0xae as ::core::ffi::c_int as uint8_t,
    0xfa as ::core::ffi::c_int as uint8_t,
    0x4f as ::core::ffi::c_int as uint8_t,
    0xf4 as ::core::ffi::c_int as uint8_t,
    0xd6 as ::core::ffi::c_int as uint8_t,
    0x9c as ::core::ffi::c_int as uint8_t,
    0x2c as ::core::ffi::c_int as uint8_t,
    0x20 as ::core::ffi::c_int as uint8_t,
    0x29 as ::core::ffi::c_int as uint8_t,
    0x99 as ::core::ffi::c_int as uint8_t,
    0x84 as ::core::ffi::c_int as uint8_t,
    0xa7 as ::core::ffi::c_int as uint8_t,
    0x65 as ::core::ffi::c_int as uint8_t,
    0xa5 as ::core::ffi::c_int as uint8_t,
    0xd3 as ::core::ffi::c_int as uint8_t,
    0xc3 as ::core::ffi::c_int as uint8_t,
    0x19 as ::core::ffi::c_int as uint8_t,
    0x82 as ::core::ffi::c_int as uint8_t,
    0xf3 as ::core::ffi::c_int as uint8_t,
    0x8f as ::core::ffi::c_int as uint8_t,
    0xc7 as ::core::ffi::c_int as uint8_t,
    0x41 as ::core::ffi::c_int as uint8_t,
    0x62 as ::core::ffi::c_int as uint8_t,
    0x15 as ::core::ffi::c_int as uint8_t,
    0x5e as ::core::ffi::c_int as uint8_t,
    0x9f as ::core::ffi::c_int as uint8_t,
];
static mut picoquic_cleartext_v2_draft_salt: [uint8_t; 20] = [
    0xa7 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0xc2 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0xa5 as ::core::ffi::c_int as uint8_t,
    0x9b as ::core::ffi::c_int as uint8_t,
    0x47 as ::core::ffi::c_int as uint8_t,
    0x18 as ::core::ffi::c_int as uint8_t,
    0x4a as ::core::ffi::c_int as uint8_t,
    0x1d as ::core::ffi::c_int as uint8_t,
    0x62 as ::core::ffi::c_int as uint8_t,
    0xca as ::core::ffi::c_int as uint8_t,
    0x57 as ::core::ffi::c_int as uint8_t,
    0x4 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0xea as ::core::ffi::c_int as uint8_t,
    0x7a as ::core::ffi::c_int as uint8_t,
    0xe3 as ::core::ffi::c_int as uint8_t,
    0xe5 as ::core::ffi::c_int as uint8_t,
    0xd3 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub static mut picoquic_retry_protection_v2_draft: [uint8_t; 32] = [
    0x34 as ::core::ffi::c_int as uint8_t,
    0x25 as ::core::ffi::c_int as uint8_t,
    0xc2 as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xf8 as ::core::ffi::c_int as uint8_t,
    0x87 as ::core::ffi::c_int as uint8_t,
    0x79 as ::core::ffi::c_int as uint8_t,
    0xdf as ::core::ffi::c_int as uint8_t,
    0x2f as ::core::ffi::c_int as uint8_t,
    0xf7 as ::core::ffi::c_int as uint8_t,
    0x1e as ::core::ffi::c_int as uint8_t,
    0x8a as ::core::ffi::c_int as uint8_t,
    0xbf as ::core::ffi::c_int as uint8_t,
    0xa7 as ::core::ffi::c_int as uint8_t,
    0x82 as ::core::ffi::c_int as uint8_t,
    0x49 as ::core::ffi::c_int as uint8_t,
    0x89 as ::core::ffi::c_int as uint8_t,
    0x1e as ::core::ffi::c_int as uint8_t,
    0x76 as ::core::ffi::c_int as uint8_t,
    0x3b as ::core::ffi::c_int as uint8_t,
    0xbe as ::core::ffi::c_int as uint8_t,
    0xd2 as ::core::ffi::c_int as uint8_t,
    0xf1 as ::core::ffi::c_int as uint8_t,
    0x3c as ::core::ffi::c_int as uint8_t,
    0x4 as ::core::ffi::c_int as uint8_t,
    0x83 as ::core::ffi::c_int as uint8_t,
    0x43 as ::core::ffi::c_int as uint8_t,
    0xd3 as ::core::ffi::c_int as uint8_t,
    0x48 as ::core::ffi::c_int as uint8_t,
    0xc0 as ::core::ffi::c_int as uint8_t,
    0x60 as ::core::ffi::c_int as uint8_t,
    0xe2 as ::core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub static mut picoquic_version_upgrade_from_v1: [uint32_t; 2] = [
    PICOQUIC_V1_VERSION as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
];
#[no_mangle]
pub static mut picoquic_supported_versions: [picoquic_version_parameters_t; 13] = unsafe {
    [
        st_picoquic_version_parameters_t {
            version: PICOQUIC_V1_VERSION as uint32_t,
            version_aead_key_length: ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
            version_aead_key: &raw const picoquic_cleartext_v1_salt as *mut uint8_t,
            version_retry_key_length: ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
            version_retry_key: &raw const picoquic_retry_protection_v1 as *mut uint8_t,
            tls_prefix_label: PICOQUIC_LABEL_QUIC_V1_KEY_BASE.as_ptr() as *mut ::core::ffi::c_char,
            tls_traffic_update_label: PICOQUIC_LABEL_V1_TRAFFIC_UPDATE.as_ptr()
                as *mut ::core::ffi::c_char,
            packet_type_version: PICOQUIC_V1_VERSION as uint32_t,
            upgrade_from: ::core::ptr::null::<uint32_t>() as *mut uint32_t,
        },
        st_picoquic_version_parameters_t {
            version: PICOQUIC_V2_VERSION as uint32_t,
            version_aead_key_length: ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
            version_aead_key: &raw const picoquic_cleartext_v2_salt as *mut uint8_t,
            version_retry_key_length: ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
            version_retry_key: &raw const picoquic_retry_protection_v2 as *mut uint8_t,
            tls_prefix_label: PICOQUIC_LABEL_QUIC_V2_KEY_BASE.as_ptr() as *mut ::core::ffi::c_char,
            tls_traffic_update_label: PICOQUIC_LABEL_V2_TRAFFIC_UPDATE.as_ptr()
                as *mut ::core::ffi::c_char,
            packet_type_version: PICOQUIC_V2_VERSION as uint32_t,
            upgrade_from: &raw const picoquic_version_upgrade_from_v1 as *mut uint32_t,
        },
        st_picoquic_version_parameters_t {
            version: PICOQUIC_V2_VERSION_DRAFT as uint32_t,
            version_aead_key_length: ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
            version_aead_key: &raw const picoquic_cleartext_v2_draft_salt as *mut uint8_t,
            version_retry_key_length: ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
            version_retry_key: &raw const picoquic_retry_protection_v2_draft as *mut uint8_t,
            tls_prefix_label: PICOQUIC_LABEL_QUIC_V2_KEY_BASE.as_ptr() as *mut ::core::ffi::c_char,
            tls_traffic_update_label: PICOQUIC_LABEL_V2_TRAFFIC_UPDATE.as_ptr()
                as *mut ::core::ffi::c_char,
            packet_type_version: PICOQUIC_V2_VERSION as uint32_t,
            upgrade_from: &raw const picoquic_version_upgrade_from_v1 as *mut uint32_t,
        },
        st_picoquic_version_parameters_t {
            version: PICOQUIC_POST_IESG_VERSION as uint32_t,
            version_aead_key_length: ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
            version_aead_key: &raw const picoquic_cleartext_v1_salt as *mut uint8_t,
            version_retry_key_length: ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
            version_retry_key: &raw const picoquic_retry_protection_v1 as *mut uint8_t,
            tls_prefix_label: PICOQUIC_LABEL_QUIC_V1_KEY_BASE.as_ptr() as *mut ::core::ffi::c_char,
            tls_traffic_update_label: PICOQUIC_LABEL_V1_TRAFFIC_UPDATE.as_ptr()
                as *mut ::core::ffi::c_char,
            packet_type_version: PICOQUIC_V1_VERSION as uint32_t,
            upgrade_from: ::core::ptr::null::<uint32_t>() as *mut uint32_t,
        },
        st_picoquic_version_parameters_t {
            version: PICOQUIC_TWENTYFIRST_INTEROP_VERSION as uint32_t,
            version_aead_key_length: ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
            version_aead_key: &raw const picoquic_cleartext_v1_salt as *mut uint8_t,
            version_retry_key_length: ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
            version_retry_key: &raw const picoquic_retry_protection_v1 as *mut uint8_t,
            tls_prefix_label: PICOQUIC_LABEL_QUIC_V1_KEY_BASE.as_ptr() as *mut ::core::ffi::c_char,
            tls_traffic_update_label: PICOQUIC_LABEL_V1_TRAFFIC_UPDATE.as_ptr()
                as *mut ::core::ffi::c_char,
            packet_type_version: PICOQUIC_V1_VERSION as uint32_t,
            upgrade_from: ::core::ptr::null::<uint32_t>() as *mut uint32_t,
        },
        st_picoquic_version_parameters_t {
            version: PICOQUIC_TWENTIETH_INTEROP_VERSION as uint32_t,
            version_aead_key_length: ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
            version_aead_key: &raw const picoquic_cleartext_draft_29_salt as *mut uint8_t,
            version_retry_key_length: ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
            version_retry_key: &raw const picoquic_retry_protection_key_29 as *mut uint8_t,
            tls_prefix_label: PICOQUIC_LABEL_QUIC_V1_KEY_BASE.as_ptr() as *mut ::core::ffi::c_char,
            tls_traffic_update_label: PICOQUIC_LABEL_V1_TRAFFIC_UPDATE.as_ptr()
                as *mut ::core::ffi::c_char,
            packet_type_version: PICOQUIC_V1_VERSION as uint32_t,
            upgrade_from: ::core::ptr::null::<uint32_t>() as *mut uint32_t,
        },
        st_picoquic_version_parameters_t {
            version: PICOQUIC_TWENTIETH_PRE_INTEROP_VERSION as uint32_t,
            version_aead_key_length: ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
            version_aead_key: &raw const picoquic_cleartext_draft_29_salt as *mut uint8_t,
            version_retry_key_length: ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
            version_retry_key: &raw const picoquic_retry_protection_key_29 as *mut uint8_t,
            tls_prefix_label: PICOQUIC_LABEL_QUIC_V1_KEY_BASE.as_ptr() as *mut ::core::ffi::c_char,
            tls_traffic_update_label: PICOQUIC_LABEL_V1_TRAFFIC_UPDATE.as_ptr()
                as *mut ::core::ffi::c_char,
            packet_type_version: PICOQUIC_V1_VERSION as uint32_t,
            upgrade_from: ::core::ptr::null::<uint32_t>() as *mut uint32_t,
        },
        st_picoquic_version_parameters_t {
            version: PICOQUIC_NINETEENTH_INTEROP_VERSION as uint32_t,
            version_aead_key_length: ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
            version_aead_key: &raw const picoquic_cleartext_draft_29_salt as *mut uint8_t,
            version_retry_key_length: ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
            version_retry_key: &raw const picoquic_retry_protection_key_29 as *mut uint8_t,
            tls_prefix_label: PICOQUIC_LABEL_QUIC_V1_KEY_BASE.as_ptr() as *mut ::core::ffi::c_char,
            tls_traffic_update_label: PICOQUIC_LABEL_V1_TRAFFIC_UPDATE.as_ptr()
                as *mut ::core::ffi::c_char,
            packet_type_version: PICOQUIC_V1_VERSION as uint32_t,
            upgrade_from: ::core::ptr::null::<uint32_t>() as *mut uint32_t,
        },
        st_picoquic_version_parameters_t {
            version: PICOQUIC_NINETEENTH_BIS_INTEROP_VERSION as uint32_t,
            version_aead_key_length: ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
            version_aead_key: &raw const picoquic_cleartext_draft_29_salt as *mut uint8_t,
            version_retry_key_length: ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
            version_retry_key: &raw const picoquic_retry_protection_key_29 as *mut uint8_t,
            tls_prefix_label: PICOQUIC_LABEL_QUIC_V1_KEY_BASE.as_ptr() as *mut ::core::ffi::c_char,
            tls_traffic_update_label: PICOQUIC_LABEL_V1_TRAFFIC_UPDATE.as_ptr()
                as *mut ::core::ffi::c_char,
            packet_type_version: PICOQUIC_V1_VERSION as uint32_t,
            upgrade_from: ::core::ptr::null::<uint32_t>() as *mut uint32_t,
        },
        st_picoquic_version_parameters_t {
            version: PICOQUIC_EIGHTEENTH_INTEROP_VERSION as uint32_t,
            version_aead_key_length: ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
            version_aead_key: &raw const picoquic_cleartext_draft_23_salt as *mut uint8_t,
            version_retry_key_length: ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
            version_retry_key: &raw const picoquic_retry_protection_key_25 as *mut uint8_t,
            tls_prefix_label: PICOQUIC_LABEL_QUIC_V1_KEY_BASE.as_ptr() as *mut ::core::ffi::c_char,
            tls_traffic_update_label: PICOQUIC_LABEL_V1_TRAFFIC_UPDATE.as_ptr()
                as *mut ::core::ffi::c_char,
            packet_type_version: PICOQUIC_V1_VERSION as uint32_t,
            upgrade_from: ::core::ptr::null::<uint32_t>() as *mut uint32_t,
        },
        st_picoquic_version_parameters_t {
            version: PICOQUIC_SEVENTEENTH_INTEROP_VERSION as uint32_t,
            version_aead_key_length: ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
            version_aead_key: &raw const picoquic_cleartext_draft_23_salt as *mut uint8_t,
            version_retry_key_length: ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
            version_retry_key: &raw const picoquic_retry_protection_key_25 as *mut uint8_t,
            tls_prefix_label: PICOQUIC_LABEL_QUIC_V1_KEY_BASE.as_ptr() as *mut ::core::ffi::c_char,
            tls_traffic_update_label: PICOQUIC_LABEL_V1_TRAFFIC_UPDATE.as_ptr()
                as *mut ::core::ffi::c_char,
            packet_type_version: PICOQUIC_V1_VERSION as uint32_t,
            upgrade_from: ::core::ptr::null::<uint32_t>() as *mut uint32_t,
        },
        st_picoquic_version_parameters_t {
            version: PICOQUIC_INTERNAL_TEST_VERSION_2 as uint32_t,
            version_aead_key_length: ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
            version_aead_key: &raw const picoquic_cleartext_internal_test_1_salt as *mut uint8_t,
            version_retry_key_length: ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
            version_retry_key: &raw const picoquic_retry_protection_key_25 as *mut uint8_t,
            tls_prefix_label: PICOQUIC_LABEL_QUIC_V1_KEY_BASE.as_ptr() as *mut ::core::ffi::c_char,
            tls_traffic_update_label: PICOQUIC_LABEL_V1_TRAFFIC_UPDATE.as_ptr()
                as *mut ::core::ffi::c_char,
            packet_type_version: PICOQUIC_V1_VERSION as uint32_t,
            upgrade_from: ::core::ptr::null::<uint32_t>() as *mut uint32_t,
        },
        st_picoquic_version_parameters_t {
            version: PICOQUIC_INTERNAL_TEST_VERSION_1 as uint32_t,
            version_aead_key_length: ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
            version_aead_key: &raw const picoquic_cleartext_internal_test_1_salt as *mut uint8_t,
            version_retry_key_length: ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
            version_retry_key: &raw const picoquic_retry_protection_key_25 as *mut uint8_t,
            tls_prefix_label: PICOQUIC_LABEL_QUIC_V1_KEY_BASE.as_ptr() as *mut ::core::ffi::c_char,
            tls_traffic_update_label: PICOQUIC_LABEL_V1_TRAFFIC_UPDATE.as_ptr()
                as *mut ::core::ffi::c_char,
            packet_type_version: PICOQUIC_V1_VERSION as uint32_t,
            upgrade_from: ::core::ptr::null::<uint32_t>() as *mut uint32_t,
        },
    ]
};
#[no_mangle]
pub static mut picoquic_nb_supported_versions: size_t = 0;
unsafe extern "C" fn picoquic_local_cnxid_hash(mut key: *const ::core::ffi::c_void) -> uint64_t {
    let mut l_cid: *const picoquic_local_cnxid_t = key as *const picoquic_local_cnxid_t;
    return picoquic_connection_id_hash(&raw const (*l_cid).cnx_id);
}
unsafe extern "C" fn picoquic_local_cnxid_compare(
    mut key1: *const ::core::ffi::c_void,
    mut key2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut l_cid1: *const picoquic_local_cnxid_t = key1 as *const picoquic_local_cnxid_t;
    let mut l_cid2: *const picoquic_local_cnxid_t = key2 as *const picoquic_local_cnxid_t;
    return picoquic_compare_connection_id(
        &raw const (*l_cid1).cnx_id,
        &raw const (*l_cid2).cnx_id,
    );
}
unsafe extern "C" fn picoquic_local_cnxid_to_item(
    mut key: *const ::core::ffi::c_void,
) -> *mut picohash_item {
    let mut l_cid: *mut picoquic_local_cnxid_t = key as *mut picoquic_local_cnxid_t;
    return &raw mut (*l_cid).hash_item;
}
unsafe extern "C" fn picoquic_net_id_hash(mut key: *const ::core::ffi::c_void) -> uint64_t {
    let mut path_x: *const picoquic_path_t = key as *const picoquic_path_t;
    return picoquic_hash_addr(&raw const (*path_x).registered_peer_addr as *mut sockaddr);
}
unsafe extern "C" fn picoquic_local_netid_to_item(
    mut key: *const ::core::ffi::c_void,
) -> *mut picohash_item {
    let mut path_x: *mut picoquic_path_t = key as *mut picoquic_path_t;
    return &raw mut (*path_x).net_id_hash_item;
}
unsafe extern "C" fn picoquic_net_id_compare(
    mut key1: *const ::core::ffi::c_void,
    mut key2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut path_x1: *const picoquic_path_t = key1 as *const picoquic_path_t;
    let mut path_x2: *const picoquic_path_t = key2 as *const picoquic_path_t;
    return picoquic_compare_addr(
        &raw const (*path_x1).registered_peer_addr as *mut sockaddr,
        &raw const (*path_x2).registered_peer_addr as *mut sockaddr,
    );
}
unsafe extern "C" fn picoquic_net_icid_hash(mut key: *const ::core::ffi::c_void) -> uint64_t {
    let mut cnx: *const picoquic_cnx_t = key as *const picoquic_cnx_t;
    return picohash_hash_mix(
        picoquic_hash_addr(&raw const (*cnx).registered_icid_addr as *mut sockaddr),
        picoquic_connection_id_hash(&raw const (*cnx).initial_cnxid),
    );
}
unsafe extern "C" fn picoquic_net_icid_compare(
    mut key1: *const ::core::ffi::c_void,
    mut key2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut cnx1: *const picoquic_cnx_t = key1 as *const picoquic_cnx_t;
    let mut cnx2: *const picoquic_cnx_t = key2 as *const picoquic_cnx_t;
    let mut ret: ::core::ffi::c_int = picoquic_compare_addr(
        &raw const (*cnx1).registered_icid_addr as *mut sockaddr,
        &raw const (*cnx2).registered_icid_addr as *mut sockaddr,
    );
    if ret == 0 as ::core::ffi::c_int {
        ret = picoquic_compare_connection_id(
            &raw const (*cnx1).initial_cnxid,
            &raw const (*cnx2).initial_cnxid,
        );
    }
    return ret;
}
unsafe extern "C" fn picoquic_net_icid_to_item(
    mut key: *const ::core::ffi::c_void,
) -> *mut picohash_item {
    let mut cnx: *mut picoquic_cnx_t = key as *mut picoquic_cnx_t;
    return &raw mut (*cnx).registered_icid_item;
}
unsafe extern "C" fn picoquic_net_secret_hash(mut key: *const ::core::ffi::c_void) -> uint64_t {
    let mut cnx: *const picoquic_cnx_t = key as *const picoquic_cnx_t;
    return picohash_hash_mix(
        picoquic_hash_addr(&raw const (*cnx).registered_secret_addr as *mut sockaddr),
        picohash_bytes(
            &raw const (*cnx).registered_reset_secret as *const uint8_t,
            PICOQUIC_RESET_SECRET_SIZE as uint32_t,
        ),
    );
}
unsafe extern "C" fn picoquic_net_secret_compare(
    mut key1: *const ::core::ffi::c_void,
    mut key2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut cnx1: *const picoquic_cnx_t = key1 as *const picoquic_cnx_t;
    let mut cnx2: *const picoquic_cnx_t = key2 as *const picoquic_cnx_t;
    let mut ret: ::core::ffi::c_int = picoquic_compare_addr(
        &raw const (*cnx1).registered_secret_addr as *mut sockaddr,
        &raw const (*cnx2).registered_secret_addr as *mut sockaddr,
    );
    if ret == 0 as ::core::ffi::c_int {
        ret = memcmp(
            &raw const (*cnx1).registered_reset_secret as *const uint8_t
                as *const ::core::ffi::c_void,
            &raw const (*cnx2).registered_reset_secret as *const uint8_t
                as *const ::core::ffi::c_void,
            PICOQUIC_RESET_SECRET_SIZE as size_t,
        );
    }
    return ret;
}
unsafe extern "C" fn picoquic_net_secret_to_item(
    mut key: *const ::core::ffi::c_void,
) -> *mut picohash_item {
    let mut cnx: *mut picoquic_cnx_t = key as *mut picoquic_cnx_t;
    return &raw mut (*cnx).registered_reset_secret_item;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_context_from_epoch(
    mut epoch: ::core::ffi::c_int,
) -> picoquic_packet_context_enum {
    static mut pc: [picoquic_packet_context_enum; 4] = [
        picoquic_packet_context_initial,
        picoquic_packet_context_application,
        picoquic_packet_context_handshake,
        picoquic_packet_context_application,
    ];
    return (if epoch >= 0 as ::core::ffi::c_int && epoch < 4 as ::core::ffi::c_int {
        pc[epoch as usize] as ::core::ffi::c_uint
    } else {
        0 as ::core::ffi::c_uint
    }) as picoquic_packet_context_enum;
}
unsafe extern "C" fn picoquic_issued_ticket_hash(mut key: *const ::core::ffi::c_void) -> uint64_t {
    let mut ticket_key: *const picoquic_issued_ticket_t = key as *const picoquic_issued_ticket_t;
    return (*ticket_key).ticket_id;
}
unsafe extern "C" fn picoquic_issued_ticket_compare(
    mut key1: *const ::core::ffi::c_void,
    mut key2: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ticket_key1: *const picoquic_issued_ticket_t = key1 as *const picoquic_issued_ticket_t;
    let mut ticket_key2: *const picoquic_issued_ticket_t = key2 as *const picoquic_issued_ticket_t;
    let mut ret: ::core::ffi::c_int = if (*ticket_key1).ticket_id == (*ticket_key2).ticket_id {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_issued_ticket_key_to_item(
    mut key: *const ::core::ffi::c_void,
) -> *mut picohash_item {
    let mut ticket_key: *mut picoquic_issued_ticket_t = key as *mut picoquic_issued_ticket_t;
    return &raw mut (*ticket_key).hash_item;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_retrieve_issued_ticket(
    mut quic: *mut picoquic_quic_t,
    mut ticket_id: uint64_t,
) -> *mut picoquic_issued_ticket_t {
    let mut ret: *mut picoquic_issued_ticket_t =
        ::core::ptr::null_mut::<picoquic_issued_ticket_t>();
    let mut item: *mut picohash_item = ::core::ptr::null_mut::<picohash_item>();
    let mut key: picoquic_issued_ticket_t = st_picoquic_issued_ticket_t {
        next_ticket: ::core::ptr::null_mut::<st_picoquic_issued_ticket_t>(),
        previous_ticket: ::core::ptr::null_mut::<st_picoquic_issued_ticket_t>(),
        hash_item: _picohash_item {
            hash: 0,
            next_in_bin: ::core::ptr::null_mut::<_picohash_item>(),
            key: ::core::ptr::null::<::core::ffi::c_void>(),
        },
        ticket_id: 0,
        creation_time: 0,
        rtt: 0,
        cwin: 0,
        ip_addr: [0; 16],
        ip_addr_length: 0,
    };
    memset(
        &raw mut key as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<picoquic_issued_ticket_t>() as size_t,
    );
    key.ticket_id = ticket_id;
    item = picohash_retrieve(
        (*quic).table_issued_tickets,
        &raw mut key as *const ::core::ffi::c_void,
    );
    if !item.is_null() {
        ret = (*item).key as *mut picoquic_issued_ticket_t;
    }
    return ret;
}
unsafe extern "C" fn picoquic_update_issued_ticket(
    mut ticket: *mut picoquic_issued_ticket_t,
    mut rtt: uint64_t,
    mut cwin: uint64_t,
    mut ip_addr: *const uint8_t,
    mut ip_addr_length: uint8_t,
) {
    if ip_addr_length as ::core::ffi::c_int > PICOQUIC_STORED_IP_MAX {
        ip_addr_length = PICOQUIC_STORED_IP_MAX as uint8_t;
    }
    (*ticket).ip_addr_length = ip_addr_length;
    memcpy(
        &raw mut (*ticket).ip_addr as *mut uint8_t as *mut ::core::ffi::c_void,
        ip_addr as *const ::core::ffi::c_void,
        ip_addr_length as size_t,
    );
    (*ticket).rtt = rtt;
    (*ticket).cwin = cwin;
}
unsafe extern "C" fn picoquic_delete_issued_ticket(
    mut quic: *mut picoquic_quic_t,
    mut ticket: *mut picoquic_issued_ticket_t,
) {
    if (*ticket).next_ticket.is_null() {
        (*quic).table_issued_tickets_last =
            (*ticket).previous_ticket as *mut picoquic_issued_ticket_t;
    } else {
        (*(*ticket).next_ticket).previous_ticket = (*ticket).previous_ticket;
    }
    if (*ticket).previous_ticket.is_null() {
        (*quic).table_issued_tickets_first = (*ticket).next_ticket as *mut picoquic_issued_ticket_t;
    } else {
        (*(*ticket).previous_ticket).next_ticket = (*ticket).next_ticket;
    }
    picohash_delete_key(
        (*quic).table_issued_tickets,
        ticket as *mut ::core::ffi::c_void,
        1 as ::core::ffi::c_int,
    );
    if (*quic).table_issued_tickets_nb > 0 as size_t {
        (*quic).table_issued_tickets_nb = (*quic).table_issued_tickets_nb.wrapping_sub(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_remember_issued_ticket(
    mut quic: *mut picoquic_quic_t,
    mut ticket_id: uint64_t,
    mut rtt: uint64_t,
    mut cwin: uint64_t,
    mut ip_addr: *const uint8_t,
    mut ip_addr_length: uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ticket: *mut picoquic_issued_ticket_t =
        picoquic_retrieve_issued_ticket(quic, ticket_id);
    if !ticket.is_null() {
        picoquic_update_issued_ticket(ticket, rtt, cwin, ip_addr, ip_addr_length);
    } else {
        while (*quic).table_issued_tickets_nb > (*quic).max_number_connections as size_t {
            picoquic_delete_issued_ticket(quic, (*quic).table_issued_tickets_last);
        }
        ticket = malloc(::core::mem::size_of::<picoquic_issued_ticket_t>() as size_t)
            as *mut picoquic_issued_ticket_t;
        if !ticket.is_null() {
            memset(
                ticket as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<picoquic_issued_ticket_t>() as size_t,
            );
            (*ticket).ticket_id = ticket_id;
            picoquic_update_issued_ticket(ticket, rtt, cwin, ip_addr, ip_addr_length);
            (*ticket).next_ticket =
                (*quic).table_issued_tickets_first as *mut st_picoquic_issued_ticket_t;
            (*quic).table_issued_tickets_first = ticket;
            if (*ticket).next_ticket.is_null() {
                (*quic).table_issued_tickets_last = ticket;
            } else {
                (*(*ticket).next_ticket).previous_ticket =
                    ticket as *mut st_picoquic_issued_ticket_t;
            }
            picohash_insert(
                (*quic).table_issued_tickets,
                ticket as *const ::core::ffi::c_void,
            );
        } else {
            ret = PICOQUIC_ERROR_MEMORY;
        }
    }
    return ret;
}
unsafe extern "C" fn picoquic_registered_token_compare(
    mut l: *mut ::core::ffi::c_void,
    mut r: *mut ::core::ffi::c_void,
) -> int64_t {
    let mut rt_l: *mut picoquic_registered_token_t = l as *mut picoquic_registered_token_t;
    let mut rt_r: *mut picoquic_registered_token_t = r as *mut picoquic_registered_token_t;
    let mut ret: int64_t = 0 as int64_t;
    if (*rt_l).token_time == (*rt_r).token_time {
        if (*rt_l).token_hash > (*rt_r).token_hash {
            ret = 1 as int64_t;
        } else if (*rt_l).token_hash < (*rt_r).token_hash {
            ret = -(1 as ::core::ffi::c_int) as int64_t;
        }
    } else if (*rt_l).token_time > (*rt_r).token_time {
        ret = 1 as int64_t;
    } else {
        ret = -(1 as ::core::ffi::c_int) as int64_t;
    }
    return ret;
}
unsafe extern "C" fn picoquic_registered_token_create(
    mut value: *mut ::core::ffi::c_void,
) -> *mut picosplay_node_t {
    return &raw mut (*(value as *mut picoquic_registered_token_t)).registered_token_node;
}
unsafe extern "C" fn picoquic_registered_token_value(
    mut node: *mut picosplay_node_t,
) -> *mut ::core::ffi::c_void {
    return (if node.is_null() {
        ::core::ptr::null_mut::<::core::ffi::c_char>()
    } else {
        (node as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
    }) as *mut ::core::ffi::c_void;
}
unsafe extern "C" fn picoquic_registered_token_delete(
    mut tree: *mut ::core::ffi::c_void,
    mut node: *mut picosplay_node_t,
) {
    let mut rt: *mut picoquic_registered_token_t =
        picoquic_registered_token_value(node) as *mut picoquic_registered_token_t;
    free(rt as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_registered_token_check_reuse(
    mut quic: *mut picoquic_quic_t,
    mut token: *const uint8_t,
    mut token_length: size_t,
    mut expiry_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if token_length >= 8 as size_t {
        let mut rt: *mut picoquic_registered_token_t =
            malloc(::core::mem::size_of::<picoquic_registered_token_t>() as size_t)
                as *mut picoquic_registered_token_t;
        if !rt.is_null() {
            let mut rt_n: *mut picosplay_node_t = ::core::ptr::null_mut::<picosplay_node_t>();
            memset(
                rt as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<picoquic_registered_token_t>() as size_t,
            );
            (*rt).token_time = expiry_time;
            (*rt).token_hash = (((((*token
                .offset(token_length as isize)
                .offset(-(8 as ::core::ffi::c_int as isize))
                .offset(0 as ::core::ffi::c_int as isize)
                as uint16_t as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *token
                    .offset(token_length as isize)
                    .offset(-(8 as ::core::ffi::c_int as isize))
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t)
                << 16 as ::core::ffi::c_int
                | ((*token
                    .offset(token_length as isize)
                    .offset(-(8 as ::core::ffi::c_int as isize))
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *token
                        .offset(token_length as isize)
                        .offset(-(8 as ::core::ffi::c_int as isize))
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int) as uint32_t)
                as uint64_t)
                << 32 as ::core::ffi::c_int
                | ((((*token
                    .offset(token_length as isize)
                    .offset(-(8 as ::core::ffi::c_int as isize))
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *token
                        .offset(token_length as isize)
                        .offset(-(8 as ::core::ffi::c_int as isize))
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int) as uint32_t)
                    << 16 as ::core::ffi::c_int
                    | ((*token
                        .offset(token_length as isize)
                        .offset(-(8 as ::core::ffi::c_int as isize))
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as uint16_t as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *token
                            .offset(token_length as isize)
                            .offset(-(8 as ::core::ffi::c_int as isize))
                            .offset(4 as ::core::ffi::c_int as isize)
                            .offset(2 as ::core::ffi::c_int as isize)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as uint16_t as ::core::ffi::c_int) as uint32_t)
                    as uint64_t;
            (*rt).count = 1 as ::core::ffi::c_int;
            rt_n = picosplay_find(
                &raw mut (*quic).token_reuse_tree,
                rt as *mut ::core::ffi::c_void,
            );
            if !rt_n.is_null() {
                free(rt as *mut ::core::ffi::c_void);
                rt = picoquic_registered_token_value(rt_n) as *mut picoquic_registered_token_t;
                (*rt).count += 1;
            } else {
                picosplay_insert(
                    &raw mut (*quic).token_reuse_tree,
                    rt as *mut ::core::ffi::c_void,
                );
                ret = 0 as ::core::ffi::c_int;
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_registered_token_clear(
    mut quic: *mut picoquic_quic_t,
    mut expiry_time_max: uint64_t,
) {
    let mut end_reached: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        let mut rt_first: *mut picoquic_registered_token_t =
            picoquic_registered_token_value(picosplay_first(&raw mut (*quic).token_reuse_tree))
                as *mut picoquic_registered_token_t;
        if rt_first.is_null() || (*rt_first).token_time >= expiry_time_max {
            end_reached = 1 as ::core::ffi::c_int;
        } else {
            picosplay_delete_hint(
                &raw mut (*quic).token_reuse_tree,
                &raw mut (*rt_first).registered_token_node,
            );
        }
        if !(end_reached == 0) {
            break;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_adjust_max_connections(
    mut quic: *mut picoquic_quic_t,
    mut max_nb_connections: uint32_t,
) -> ::core::ffi::c_int {
    if max_nb_connections <= (*quic).max_number_connections {
        (*quic).tentative_max_number_connections = max_nb_connections;
        return 0 as ::core::ffi::c_int;
    }
    return -(1 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_current_number_connections(
    mut quic: *mut picoquic_quic_t,
) -> uint32_t {
    return (*quic).current_number_connections;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create(
    mut max_nb_connections: uint32_t,
    mut cert_file_name: *const ::core::ffi::c_char,
    mut key_file_name: *const ::core::ffi::c_char,
    mut cert_root_file_name: *const ::core::ffi::c_char,
    mut default_alpn: *const ::core::ffi::c_char,
    mut default_callback_fn: picoquic_stream_data_cb_fn,
    mut default_callback_ctx: *mut ::core::ffi::c_void,
    mut cnx_id_callback: picoquic_connection_id_cb_fn,
    mut cnx_id_callback_ctx: *mut ::core::ffi::c_void,
    mut reset_seed: *mut uint8_t,
    mut current_time: uint64_t,
    mut p_simulated_time: *mut uint64_t,
    mut ticket_file_name: *const ::core::ffi::c_char,
    mut ticket_encryption_key: *const uint8_t,
    mut ticket_encryption_key_length: size_t,
) -> *mut picoquic_quic_t {
    let mut quic: *mut picoquic_quic_t =
        malloc(::core::mem::size_of::<picoquic_quic_t>() as size_t) as *mut picoquic_quic_t;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !quic.is_null() {
        memset(
            quic as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<picoquic_quic_t>() as size_t,
        );
        (*quic).default_callback_fn = default_callback_fn;
        (*quic).default_callback_ctx = default_callback_ctx;
        (*quic).default_congestion_alg = picoquic_newreno_algorithm;
        (*quic).default_alpn = picoquic_string_duplicate(default_alpn);
        (*quic).cnx_id_callback_fn = cnx_id_callback;
        (*quic).cnx_id_callback_ctx = cnx_id_callback_ctx;
        (*quic).p_simulated_time = p_simulated_time;
        (*quic).local_cnxid_length = 8 as uint8_t;
        (*quic).padding_multiple_default = 0 as uint32_t;
        (*quic).padding_minsize_default = PICOQUIC_RESET_PACKET_MIN_SIZE as uint32_t;
        (*quic).crypto_epoch_length_max = 0 as uint64_t;
        (*quic).max_simultaneous_logs = PICOQUIC_DEFAULT_SIMULTANEOUS_LOGS as uint32_t;
        (*quic).max_half_open_before_retry = PICOQUIC_DEFAULT_HALF_OPEN_RETRY_THRESHOLD as uint32_t;
        (*quic).default_lossbit_policy = picoquic_lossbit_none;
        (*quic).local_cnxid_ttl = UINT64_MAX as uint64_t;
        (*quic).stateless_reset_next_time = current_time;
        (*quic).stateless_reset_min_interval =
            PICOQUIC_MICROSEC_STATELESS_RESET_INTERVAL_DEFAULT as uint64_t;
        (*quic).default_stream_priority = PICOQUIC_DEFAULT_STREAM_PRIORITY as uint8_t;
        (*quic).default_datagram_priority = PICOQUIC_DEFAULT_STREAM_PRIORITY as uint8_t;
        (*quic).cwin_max = UINT64_MAX as uint64_t;
        (*quic).sequence_hole_pseudo_period = PICOQUIC_DEFAULT_HOLE_PERIOD as uint32_t;
        picoquic_init_transport_parameters(&raw mut (*quic).default_tp, 0 as ::core::ffi::c_int);
        (*quic).set_random_initial(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        picoquic_wake_list_init(quic);
        if cnx_id_callback.is_some() {
            (*quic).set_unconditional_cnx_id(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        if !ticket_file_name.is_null() {
            (*quic).ticket_file_name = ticket_file_name;
        }
        if ret == 0 as ::core::ffi::c_int {
            let mut max_cnx4: size_t = 0 as size_t;
            if max_nb_connections == 0 as uint32_t {
                max_nb_connections = 1 as uint32_t;
            }
            (*quic).tentative_max_number_connections = max_nb_connections;
            (*quic).max_number_connections = max_nb_connections;
            max_cnx4 = (4 as size_t).wrapping_mul(max_nb_connections as size_t);
            if max_cnx4 < max_nb_connections as size_t
                || {
                    (*quic).table_cnx_by_id = picohash_create_ex(
                        (max_nb_connections as size_t).wrapping_mul(4 as size_t),
                        Some(
                            picoquic_local_cnxid_hash
                                as unsafe extern "C" fn(*const ::core::ffi::c_void) -> uint64_t,
                        ),
                        Some(
                            picoquic_local_cnxid_compare
                                as unsafe extern "C" fn(
                                    *const ::core::ffi::c_void,
                                    *const ::core::ffi::c_void,
                                )
                                    -> ::core::ffi::c_int,
                        ),
                        Some(
                            picoquic_local_cnxid_to_item
                                as unsafe extern "C" fn(
                                    *const ::core::ffi::c_void,
                                )
                                    -> *mut picohash_item,
                        ),
                    );
                    (*quic).table_cnx_by_id.is_null()
                }
                || {
                    (*quic).table_cnx_by_net = picohash_create_ex(
                        (max_nb_connections as size_t).wrapping_mul(4 as size_t),
                        Some(
                            picoquic_net_id_hash
                                as unsafe extern "C" fn(*const ::core::ffi::c_void) -> uint64_t,
                        ),
                        Some(
                            picoquic_net_id_compare
                                as unsafe extern "C" fn(
                                    *const ::core::ffi::c_void,
                                    *const ::core::ffi::c_void,
                                )
                                    -> ::core::ffi::c_int,
                        ),
                        Some(
                            picoquic_local_netid_to_item
                                as unsafe extern "C" fn(
                                    *const ::core::ffi::c_void,
                                )
                                    -> *mut picohash_item,
                        ),
                    );
                    (*quic).table_cnx_by_net.is_null()
                }
                || {
                    (*quic).table_cnx_by_icid = picohash_create_ex(
                        max_nb_connections as size_t,
                        Some(
                            picoquic_net_icid_hash
                                as unsafe extern "C" fn(*const ::core::ffi::c_void) -> uint64_t,
                        ),
                        Some(
                            picoquic_net_icid_compare
                                as unsafe extern "C" fn(
                                    *const ::core::ffi::c_void,
                                    *const ::core::ffi::c_void,
                                )
                                    -> ::core::ffi::c_int,
                        ),
                        Some(
                            picoquic_net_icid_to_item
                                as unsafe extern "C" fn(
                                    *const ::core::ffi::c_void,
                                )
                                    -> *mut picohash_item,
                        ),
                    );
                    (*quic).table_cnx_by_icid.is_null()
                }
                || {
                    (*quic).table_cnx_by_secret = picohash_create_ex(
                        (max_nb_connections as size_t).wrapping_mul(4 as size_t),
                        Some(
                            picoquic_net_secret_hash
                                as unsafe extern "C" fn(*const ::core::ffi::c_void) -> uint64_t,
                        ),
                        Some(
                            picoquic_net_secret_compare
                                as unsafe extern "C" fn(
                                    *const ::core::ffi::c_void,
                                    *const ::core::ffi::c_void,
                                )
                                    -> ::core::ffi::c_int,
                        ),
                        Some(
                            picoquic_net_secret_to_item
                                as unsafe extern "C" fn(
                                    *const ::core::ffi::c_void,
                                )
                                    -> *mut picohash_item,
                        ),
                    );
                    (*quic).table_cnx_by_secret.is_null()
                }
                || {
                    (*quic).table_issued_tickets = picohash_create_ex(
                        max_nb_connections as size_t,
                        Some(
                            picoquic_issued_ticket_hash
                                as unsafe extern "C" fn(*const ::core::ffi::c_void) -> uint64_t,
                        ),
                        Some(
                            picoquic_issued_ticket_compare
                                as unsafe extern "C" fn(
                                    *const ::core::ffi::c_void,
                                    *const ::core::ffi::c_void,
                                )
                                    -> ::core::ffi::c_int,
                        ),
                        Some(
                            picoquic_issued_ticket_key_to_item
                                as unsafe extern "C" fn(
                                    *const ::core::ffi::c_void,
                                )
                                    -> *mut picohash_item,
                        ),
                    );
                    (*quic).table_issued_tickets.is_null()
                }
            {
                ret = -(1 as ::core::ffi::c_int);
            } else {
                picosplay_init_tree(
                    &raw mut (*quic).token_reuse_tree,
                    Some(
                        picoquic_registered_token_compare
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut ::core::ffi::c_void,
                            ) -> int64_t,
                    ),
                    Some(
                        picoquic_registered_token_create
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                            )
                                -> *mut picosplay_node_t,
                    ),
                    Some(
                        picoquic_registered_token_delete
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                *mut picosplay_node_t,
                            ) -> (),
                    ),
                    Some(
                        picoquic_registered_token_value
                            as unsafe extern "C" fn(
                                *mut picosplay_node_t,
                            )
                                -> *mut ::core::ffi::c_void,
                    ),
                );
                if picoquic_master_tlscontext(
                    quic,
                    cert_file_name,
                    key_file_name,
                    cert_root_file_name,
                    ticket_encryption_key,
                    ticket_encryption_key_length,
                ) != 0 as ::core::ffi::c_int
                {
                    ret = -(1 as ::core::ffi::c_int);
                } else {
                    (*quic).set_enforce_client_only(
                        (cert_file_name.is_null() || key_file_name.is_null()) as ::core::ffi::c_int
                            as ::core::ffi::c_uint as ::core::ffi::c_uint,
                    );
                    if reset_seed.is_null() {
                        picoquic_crypto_random(
                            quic,
                            &raw mut (*quic).reset_seed as *mut uint8_t as *mut ::core::ffi::c_void,
                            ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
                        );
                    } else {
                        memcpy(
                            &raw mut (*quic).reset_seed as *mut uint8_t as *mut ::core::ffi::c_void,
                            reset_seed as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
                        );
                    }
                    picoquic_crypto_random(
                        quic,
                        &raw mut (*quic).retry_seed as *mut uint8_t as *mut ::core::ffi::c_void,
                        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
                    );
                    if !(*quic).ticket_file_name.is_null() {
                        ret = picoquic_load_tickets(quic as *mut picoquic_quic_t, ticket_file_name);
                        if ret == PICOQUIC_ERROR_NO_SUCH_FILE {
                            ret = 0 as ::core::ffi::c_int;
                        } else if ret != 0 as ::core::ffi::c_int {
                            ret = 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
        if ret == 0 as ::core::ffi::c_int {
            (*quic)
                .bbr_exp_flags
                .set_do_early_exit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*quic)
                .bbr_exp_flags
                .set_do_rapid_start(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*quic)
                .bbr_exp_flags
                .set_do_handle_suspension(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*quic)
                .bbr_exp_flags
                .set_do_control_lost(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*quic)
                .bbr_exp_flags
                .set_do_exit_probeBW_up_on_delay(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*quic).bbr_exp_flags.set_do_enter_probeBW_after_limited(
                1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
            );
        }
        if ret != 0 as ::core::ffi::c_int {
            picoquic_free(quic);
            quic = ::core::ptr::null_mut::<picoquic_quic_t>();
        }
    }
    return quic as *mut picoquic_quic_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_load_token_file(
    mut quic: *mut picoquic_quic_t,
    mut token_file_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int =
        picoquic_load_tokens(quic as *mut picoquic_quic_t, token_file_name);
    if ret == PICOQUIC_ERROR_NO_SUCH_FILE {
        ret = 0 as ::core::ffi::c_int;
    } else {
        ret != 0 as ::core::ffi::c_int;
    }
    if ret == 0 as ::core::ffi::c_int {
        (*quic).token_file_name = token_file_name;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_tp(
    mut quic: *mut picoquic_quic_t,
    mut tp: *mut picoquic_tp_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if tp.is_null() {
        picoquic_init_transport_parameters(&raw mut (*quic).default_tp, 0 as ::core::ffi::c_int);
    } else {
        memcpy(
            &raw mut (*quic).default_tp as *mut ::core::ffi::c_void,
            tp as *const ::core::ffi::c_void,
            ::core::mem::size_of::<picoquic_tp_t>() as size_t,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_default_tp(
    mut quic: *mut picoquic_quic_t,
) -> *const picoquic_tp_t {
    return &raw mut (*quic).default_tp;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_padding(
    mut quic: *mut picoquic_quic_t,
    mut padding_multiple: uint32_t,
    mut padding_minsize: uint32_t,
) {
    (*quic).padding_minsize_default = padding_minsize;
    (*quic).padding_multiple_default = padding_multiple;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_spinbit_policy(
    mut quic: *mut picoquic_quic_t,
    mut default_spinbit_policy: picoquic_spinbit_version_enum,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if default_spinbit_policy as ::core::ffi::c_uint
        <= picoquic_spinbit_on as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*quic).default_spin_policy = default_spinbit_policy;
    } else {
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_spinbit_policy(
    mut cnx: *mut picoquic_cnx_t,
    mut spinbit_policy: picoquic_spinbit_version_enum,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (spinbit_policy as ::core::ffi::c_uint)
        < picoquic_spinbit_on as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cnx).spin_policy = spinbit_policy;
    } else {
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_lossbit_policy(
    mut quic: *mut picoquic_quic_t,
    mut default_lossbit_policy: picoquic_lossbit_version_enum,
) {
    (*quic).default_lossbit_policy = default_lossbit_policy;
    (*quic).default_tp.enable_loss_bit = default_lossbit_policy as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_multipath_option(
    mut quic: *mut picoquic_quic_t,
    mut multipath_option: ::core::ffi::c_int,
) {
    (*quic).default_multipath_option = multipath_option as uint32_t;
    if multipath_option & 1 as ::core::ffi::c_int != 0 {
        (*quic).default_tp.is_multipath_enabled = 1 as ::core::ffi::c_int;
        (*quic).default_tp.initial_max_path_id = 2 as uint64_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_address_discovery_mode(
    mut quic: *mut picoquic_quic_t,
    mut mode: ::core::ffi::c_int,
) {
    if mode > 0 as ::core::ffi::c_int && mode <= 3 as ::core::ffi::c_int {
        (*quic).default_tp.address_discovery_mode = mode;
    } else {
        (*quic).default_tp.address_discovery_mode = 0 as ::core::ffi::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_cwin_max(
    mut quic: *mut picoquic_quic_t,
    mut cwin_max: uint64_t,
) {
    (*quic).cwin_max = if cwin_max == 0 as uint64_t {
        UINT64_MAX as uint64_t
    } else {
        cwin_max
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_max_data_control(
    mut quic: *mut picoquic_quic_t,
    mut max_data: uint64_t,
) {
    let mut cnx: *mut picoquic_cnx_t = (*quic).cnx_list as *mut picoquic_cnx_t;
    (*quic).max_data_limit = max_data;
    (*quic).default_tp.initial_max_data = max_data;
    while !cnx.is_null() {
        if (*cnx).client_mode() as ::core::ffi::c_int != 0
            && (*cnx).cnx_state as ::core::ffi::c_uint
                == picoquic_state_client_init as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*cnx).tls_stream[0 as ::core::ffi::c_int as usize].sent_offset == 0 as uint64_t
            && (*cnx).tls_stream[0 as ::core::ffi::c_int as usize]
                .send_queue
                .is_null()
        {
            (*cnx).local_parameters.initial_max_data = max_data;
            (*cnx).maxdata_local = max_data;
        }
        cnx = (*cnx).next_in_table as *mut picoquic_cnx_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_idle_timeout(
    mut quic: *mut picoquic_quic_t,
    mut idle_timeout_ms: uint64_t,
) {
    (*quic).default_tp.max_idle_timeout = idle_timeout_ms;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_handshake_timeout(
    mut quic: *mut picoquic_quic_t,
    mut handshake_timeout_us: uint64_t,
) {
    (*quic).default_handshake_timeout = handshake_timeout_us;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_crypto_epoch_length(
    mut quic: *mut picoquic_quic_t,
    mut crypto_epoch_length_max: uint64_t,
) {
    (*quic).crypto_epoch_length_max = if crypto_epoch_length_max == 0 as uint64_t {
        PICOQUIC_DEFAULT_CRYPTO_EPOCH_LENGTH as uint64_t
    } else {
        crypto_epoch_length_max
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_default_crypto_epoch_length(
    mut quic: *mut picoquic_quic_t,
) -> uint64_t {
    return (*quic).crypto_epoch_length_max;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_crypto_epoch_length(
    mut cnx: *mut picoquic_cnx_t,
    mut crypto_epoch_length_max: uint64_t,
) {
    (*cnx).crypto_epoch_length_max = if crypto_epoch_length_max == 0 as uint64_t {
        PICOQUIC_DEFAULT_CRYPTO_EPOCH_LENGTH as uint64_t
    } else {
        crypto_epoch_length_max
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_crypto_epoch_length(
    mut cnx: *mut picoquic_cnx_t,
) -> uint64_t {
    return (*cnx).crypto_epoch_length_max;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_local_cid_length(mut quic: *mut picoquic_quic_t) -> uint8_t {
    return (*quic).local_cnxid_length;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_local_cid(
    mut quic: *mut picoquic_quic_t,
    mut cid: *mut picoquic_connection_id_t,
) -> ::core::ffi::c_int {
    return ((*cid).id_len as ::core::ffi::c_int == (*quic).local_cnxid_length as ::core::ffi::c_int
        && !picoquic_cnx_by_id(
            quic,
            *cid,
            ::core::ptr::null_mut::<*mut st_picoquic_local_cnxid_t>(),
        )
        .is_null()) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_max_simultaneous_logs(
    mut quic: *mut picoquic_quic_t,
    mut max_simultaneous_logs: uint32_t,
) {
    (*quic).max_simultaneous_logs = max_simultaneous_logs;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_max_simultaneous_logs(
    mut quic: *mut picoquic_quic_t,
) -> uint32_t {
    return (*quic).max_simultaneous_logs;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_bdp_frame_option(
    mut quic: *mut picoquic_quic_t,
    mut bdp_option: ::core::ffi::c_int,
) {
    (*quic).set_default_send_receive_bdp_frame(
        bdp_option as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_free(mut quic: *mut picoquic_quic_t) {
    if !quic.is_null() {
        while !(*quic).cnx_list.is_null() {
            picoquic_delete_cnx((*quic).cnx_list as *mut picoquic_cnx_t);
        }
        picoquic_delete_retry_protection_contexts(quic);
        if !(*quic).aead_encrypt_ticket_ctx.is_null() {
            picoquic_aead_free((*quic).aead_encrypt_ticket_ctx);
            (*quic).aead_encrypt_ticket_ctx = NULL;
        }
        if !(*quic).aead_decrypt_ticket_ctx.is_null() {
            picoquic_aead_free((*quic).aead_decrypt_ticket_ctx);
            (*quic).aead_decrypt_ticket_ctx = NULL;
        }
        if !(*quic).default_alpn.is_null() {
            free((*quic).default_alpn as *mut ::core::ffi::c_void);
            (*quic).default_alpn = ::core::ptr::null::<::core::ffi::c_char>();
        }
        picoquic_free_tickets(&raw mut (*quic).p_first_ticket);
        picoquic_free_tokens(&raw mut (*quic).p_first_token);
        picosplay_empty_tree(&raw mut (*quic).token_reuse_tree);
        while !(*quic).p_first_packet.is_null() {
            let mut p: *mut picoquic_packet_t =
                (*(*quic).p_first_packet).packet_previous as *mut picoquic_packet_t;
            free((*quic).p_first_packet as *mut ::core::ffi::c_void);
            (*quic).p_first_packet = p;
            (*quic).nb_packets_allocated -= 1;
            (*quic).nb_packets_in_pool -= 1;
        }
        while !(*quic).p_first_data_node.is_null() {
            let mut p_0: *mut picoquic_stream_data_node_t =
                (*(*quic).p_first_data_node).next_stream_data as *mut picoquic_stream_data_node_t;
            free((*quic).p_first_data_node as *mut ::core::ffi::c_void);
            (*quic).p_first_data_node = p_0;
            (*quic).nb_data_nodes_allocated -= 1;
            (*quic).nb_data_nodes_in_pool -= 1;
        }
        while !(*quic).pending_stateless_packet.is_null() {
            let mut to_delete: *mut picoquic_stateless_packet_t = (*quic).pending_stateless_packet;
            (*quic).pending_stateless_packet =
                (*to_delete).next_packet as *mut picoquic_stateless_packet_t;
            free(to_delete as *mut ::core::ffi::c_void);
        }
        if !(*quic).table_cnx_by_id.is_null() {
            picohash_delete((*quic).table_cnx_by_id, 0 as ::core::ffi::c_int);
        }
        if !(*quic).table_cnx_by_net.is_null() {
            picohash_delete((*quic).table_cnx_by_net, 0 as ::core::ffi::c_int);
        }
        if !(*quic).table_cnx_by_icid.is_null() {
            picohash_delete((*quic).table_cnx_by_icid, 0 as ::core::ffi::c_int);
        }
        if !(*quic).table_issued_tickets.is_null() {
            picohash_delete((*quic).table_issued_tickets, 1 as ::core::ffi::c_int);
        }
        if !(*quic).table_cnx_by_secret.is_null() {
            picohash_delete((*quic).table_cnx_by_secret, 1 as ::core::ffi::c_int);
        }
        if !(*quic).verify_certificate_callback.is_null() {
            picoquic_dispose_verify_certificate_callback(quic);
        }
        if !(*quic).tls_master_ctx.is_null() {
            picoquic_master_tlscontext_free(quic);
            free((*quic).tls_master_ctx);
            (*quic).tls_master_ctx = NULL;
        }
        picoquic_log_close_logs(quic);
        (*quic).binlog_dir = picoquic_string_free((*quic).binlog_dir);
        (*quic).qlog_dir = picoquic_string_free((*quic).qlog_dir);
        if (*quic).perflog_fn.is_some() {
            (*quic).perflog_fn.expect("non-null function pointer")(
                quic as *mut picoquic_quic_t,
                ::core::ptr::null_mut::<picoquic_cnx_t>(),
                1 as ::core::ffi::c_int,
            );
        }
        free(quic as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_low_memory_mode(
    mut quic: *mut picoquic_quic_t,
    mut low_memory_mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    (*quic).set_use_low_memory(
        (if low_memory_mode == 0 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
    return picoquic_set_cipher_suite(quic as *mut picoquic_quic_t, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_null_verifier(mut quic: *mut picoquic_quic_t) {
    picoquic_dispose_verify_certificate_callback(quic);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_cookie_mode(
    mut quic: *mut picoquic_quic_t,
    mut cookie_mode: ::core::ffi::c_int,
) {
    if cookie_mode & 1 as ::core::ffi::c_int != 0 {
        (*quic).set_force_check_token(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    } else {
        (*quic).set_force_check_token(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if cookie_mode & 2 as ::core::ffi::c_int != 0 {
        (*quic).set_provide_token(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    } else {
        (*quic).set_provide_token(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    (*quic).set_check_token(
        ((*quic).force_check_token() as ::core::ffi::c_int != 0
            || (*quic).max_half_open_before_retry <= (*quic).current_number_half_open)
            as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_max_half_open_retry_threshold(
    mut quic: *mut picoquic_quic_t,
    mut max_half_open_before_retry: uint32_t,
) {
    (*quic).max_half_open_before_retry = max_half_open_before_retry;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_max_half_open_retry_threshold(
    mut quic: *mut picoquic_quic_t,
) -> uint32_t {
    return (*quic).max_half_open_before_retry;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_stateless_packet(
    mut quic: *mut picoquic_quic_t,
) -> *mut picoquic_stateless_packet_t {
    return malloc(::core::mem::size_of::<picoquic_stateless_packet_t>() as size_t)
        as *mut picoquic_stateless_packet_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_stateless_packet(
    mut sp: *mut picoquic_stateless_packet_t,
) {
    free(sp as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_stateless_packet(
    mut quic: *mut picoquic_quic_t,
    mut sp: *mut picoquic_stateless_packet_t,
) {
    let mut pnext: *mut *mut picoquic_stateless_packet_t =
        &raw mut (*quic).pending_stateless_packet;
    while !(*pnext).is_null() {
        pnext = &raw mut (**pnext).next_packet as *mut *mut picoquic_stateless_packet_t;
    }
    *pnext = sp;
    (*sp).next_packet = ::core::ptr::null_mut::<st_picoquic_stateless_packet_t>();
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_dequeue_stateless_packet(
    mut quic: *mut picoquic_quic_t,
) -> *mut picoquic_stateless_packet_t {
    let mut sp: *mut picoquic_stateless_packet_t = (*quic).pending_stateless_packet;
    if !sp.is_null() {
        (*quic).pending_stateless_packet = (*sp).next_packet as *mut picoquic_stateless_packet_t;
        (*sp).next_packet = ::core::ptr::null_mut::<st_picoquic_stateless_packet_t>();
        picoquic_log_quic_pdu(
            quic,
            0 as ::core::ffi::c_int,
            picoquic_get_quic_time(quic),
            (*sp).cnxid_log64,
            &raw mut (*sp).addr_to as *mut sockaddr,
            &raw mut (*sp).addr_local as *mut sockaddr,
            (*sp).length,
        );
    }
    return sp;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cnx_is_still_logging(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = ((*cnx).nb_packets_logged
        < PICOQUIC_LOG_PACKET_MAX_SEQUENCE as uint64_t
        || (*(*cnx).quic).use_long_log() as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_register_cnx_id(
    mut quic: *mut picoquic_quic_t,
    mut cnx: *mut picoquic_cnx_t,
    mut l_cid: *mut picoquic_local_cnxid_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut item: *mut picohash_item = ::core::ptr::null_mut::<picohash_item>();
    item = picohash_retrieve((*quic).table_cnx_by_id, l_cid as *const ::core::ffi::c_void);
    if !item.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        (*l_cid).registered_cnx = cnx as *mut picoquic_cnx_t;
        ret = picohash_insert((*quic).table_cnx_by_id, l_cid as *const ::core::ffi::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_unregister_net_id(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
) {
    if !(*path_x).net_id_hash_item.key.is_null() {
        let mut item: *mut picohash_item = picohash_retrieve(
            (*(*cnx).quic).table_cnx_by_net,
            path_x as *const ::core::ffi::c_void,
        );
        if !item.is_null() {
            picohash_delete_item(
                (*(*cnx).quic).table_cnx_by_net,
                item,
                0 as ::core::ffi::c_int,
            );
        }
        memset(
            &raw mut (*path_x).registered_peer_addr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_storage>() as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_register_net_id(
    mut quic: *mut picoquic_quic_t,
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut item: *mut picohash_item = ::core::ptr::null_mut::<picohash_item>();
    picoquic_unregister_net_id(cnx, path_x);
    picoquic_store_addr(
        &raw mut (*path_x).registered_peer_addr,
        &raw mut (*path_x).peer_addr as *mut sockaddr,
    );
    item = picohash_retrieve(
        (*quic).table_cnx_by_net,
        path_x as *const ::core::ffi::c_void,
    );
    if !item.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        ret = picohash_insert(
            (*quic).table_cnx_by_net,
            path_x as *const ::core::ffi::c_void,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_register_net_icid(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut item: *mut picohash_item = ::core::ptr::null_mut::<picohash_item>();
    picoquic_store_addr(
        &raw mut (*cnx).registered_icid_addr,
        &raw mut (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).peer_addr
            as *mut sockaddr,
    );
    item = picohash_retrieve(
        (*(*cnx).quic).table_cnx_by_icid,
        cnx as *const ::core::ffi::c_void,
    );
    if !item.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        ret = picohash_insert(
            (*(*cnx).quic).table_cnx_by_icid,
            cnx as *const ::core::ffi::c_void,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_unregister_net_icid(mut cnx: *mut picoquic_cnx_t) {
    if !(*cnx).registered_icid_item.key.is_null() {
        picohash_delete_item(
            (*(*cnx).quic).table_cnx_by_icid,
            &raw mut (*cnx).registered_icid_item,
            0 as ::core::ffi::c_int,
        );
        memset(
            &raw mut (*cnx).registered_icid_addr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_storage>() as size_t,
        );
        memset(
            &raw mut (*cnx).registered_icid_item as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<picohash_item>() as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_unregister_net_secret(mut cnx: *mut picoquic_cnx_t) {
    if (*cnx).registered_secret_addr.ss_family as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        picohash_delete_key(
            (*(*cnx).quic).table_cnx_by_secret,
            cnx as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
        );
        memset(
            &raw mut (*cnx).registered_secret_addr as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<sockaddr_storage>() as size_t,
        );
        memset(
            &raw mut (*cnx).registered_reset_secret as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<::core::ffi::c_int>() as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_register_net_secret(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
        .peer_addr
        .ss_family as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
    {
        let mut item: *mut picohash_item = ::core::ptr::null_mut::<picohash_item>();
        picoquic_unregister_net_secret(cnx);
        picoquic_store_addr(
            &raw mut (*cnx).registered_secret_addr,
            &raw mut (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).peer_addr
                as *mut sockaddr,
        );
        memcpy(
            &raw mut (*cnx).registered_reset_secret as *mut ::core::ffi::c_void,
            &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid)
                .reset_secret as *mut uint8_t as *const ::core::ffi::c_void,
            PICOQUIC_RESET_SECRET_SIZE as size_t,
        );
        item = picohash_retrieve(
            (*(*cnx).quic).table_cnx_by_secret,
            cnx as *const ::core::ffi::c_void,
        );
        if !item.is_null() {
            ret = -(1 as ::core::ffi::c_int);
        } else {
            ret = picohash_insert(
                (*(*cnx).quic).table_cnx_by_secret,
                cnx as *const ::core::ffi::c_void,
            );
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_init_transport_parameters(
    mut tp: *mut picoquic_tp_t,
    mut client_mode: ::core::ffi::c_int,
) {
    memset(
        tp as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<picoquic_tp_t>() as size_t,
    );
    (*tp).initial_max_stream_data_bidi_local = 0x200000 as uint64_t;
    (*tp).initial_max_stream_data_bidi_remote = 65635 as uint64_t;
    (*tp).initial_max_stream_data_uni = 65535 as uint64_t;
    (*tp).initial_max_data = 0x100000 as uint64_t;
    (*tp).initial_max_stream_id_bidir = 512 as uint64_t;
    (*tp).initial_max_stream_id_unidir = 512 as uint64_t;
    (*tp).max_idle_timeout =
        PICOQUIC_MICROSEC_HANDSHAKE_MAX.wrapping_div(1000 as ::core::ffi::c_ulonglong) as uint64_t;
    (*tp).max_packet_size = PICOQUIC_PRACTICAL_MAX_MTU as uint32_t;
    (*tp).max_datagram_frame_size = 0 as uint32_t;
    (*tp).ack_delay_exponent = 3 as uint8_t;
    (*tp).active_connection_id_limit = PICOQUIC_NB_PATH_TARGET as uint32_t;
    (*tp).max_ack_delay = PICOQUIC_ACK_DELAY_MAX as uint32_t;
    (*tp).enable_loss_bit = 2 as ::core::ffi::c_int;
    (*tp).min_ack_delay = PICOQUIC_ACK_DELAY_MIN as uint64_t;
    (*tp).enable_time_stamp = 0 as ::core::ffi::c_int;
    (*tp).enable_bdp_frame = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_quic_ctx(
    mut cnx: *mut picoquic_cnx_t,
) -> *mut picoquic_quic_t {
    return if cnx.is_null() {
        ::core::ptr::null_mut::<picoquic_quic_t>()
    } else {
        (*cnx).quic as *mut picoquic_quic_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_first_cnx(
    mut quic: *mut picoquic_quic_t,
) -> *mut picoquic_cnx_t {
    return (*quic).cnx_list as *mut picoquic_cnx_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_next_cnx(
    mut cnx: *mut picoquic_cnx_t,
) -> *mut picoquic_cnx_t {
    return (*cnx).next_in_table as *mut picoquic_cnx_t;
}
unsafe extern "C" fn picoquic_insert_cnx_in_list(
    mut quic: *mut picoquic_quic_t,
    mut cnx: *mut picoquic_cnx_t,
) {
    if !(*quic).cnx_list.is_null() {
        (*(*quic).cnx_list).previous_in_table = cnx as *mut st_picoquic_cnx_t;
        (*cnx).next_in_table = (*quic).cnx_list;
    } else {
        (*quic).cnx_last = cnx as *mut st_picoquic_cnx_t;
        (*cnx).next_in_table = ::core::ptr::null_mut::<st_picoquic_cnx_t>();
    }
    (*quic).cnx_list = cnx as *mut st_picoquic_cnx_t;
    (*cnx).previous_in_table = ::core::ptr::null_mut::<st_picoquic_cnx_t>();
    (*quic).current_number_connections = (*quic).current_number_connections.wrapping_add(1);
}
unsafe extern "C" fn picoquic_remove_cnx_from_list(mut cnx: *mut picoquic_cnx_t) {
    if (*cnx).next_in_table.is_null() {
        (*(*cnx).quic).cnx_last = (*cnx).previous_in_table;
    } else {
        (*(*cnx).next_in_table).previous_in_table = (*cnx).previous_in_table;
    }
    if (*cnx).previous_in_table.is_null() {
        (*(*cnx).quic).cnx_list = (*cnx).next_in_table;
    } else {
        (*(*cnx).previous_in_table).next_in_table = (*cnx).next_in_table;
    }
    picoquic_unregister_net_icid(cnx);
    picoquic_unregister_net_secret(cnx);
    (*(*cnx).quic).current_number_connections =
        (*(*cnx).quic).current_number_connections.wrapping_sub(1);
}
unsafe extern "C" fn picoquic_wake_list_node_value(
    mut cnx_wake_node: *mut picosplay_node_t,
) -> *mut ::core::ffi::c_void {
    return if cnx_wake_node.is_null() {
        NULL
    } else {
        (cnx_wake_node as *mut ::core::ffi::c_char).offset(-(1104 as ::core::ffi::c_ulong as isize))
            as *mut ::core::ffi::c_void
    };
}
unsafe extern "C" fn picoquic_wake_list_compare(
    mut l: *mut ::core::ffi::c_void,
    mut r: *mut ::core::ffi::c_void,
) -> int64_t {
    let ltime: uint64_t = (*(l as *mut picoquic_cnx_t)).next_wake_time;
    let rtime: uint64_t = (*(r as *mut picoquic_cnx_t)).next_wake_time;
    if ltime < rtime {
        return -(1 as ::core::ffi::c_int) as int64_t;
    }
    if ltime > rtime {
        return 1 as int64_t;
    }
    return 0 as int64_t;
}
unsafe extern "C" fn picoquic_wake_list_create_node(
    mut v_cnx: *mut ::core::ffi::c_void,
) -> *mut picosplay_node_t {
    return &raw mut (*(v_cnx as *mut picoquic_cnx_t)).cnx_wake_node;
}
unsafe extern "C" fn picoquic_wake_list_delete_node(
    mut tree: *mut ::core::ffi::c_void,
    mut node: *mut picosplay_node_t,
) {
    memset(
        node as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<picosplay_node_t>() as size_t,
    );
}
unsafe extern "C" fn picoquic_wake_list_init(mut quic: *mut picoquic_quic_t) {
    picosplay_init_tree(
        &raw mut (*quic).cnx_wake_tree,
        Some(
            picoquic_wake_list_compare
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> int64_t,
        ),
        Some(
            picoquic_wake_list_create_node
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut picosplay_node_t,
        ),
        Some(
            picoquic_wake_list_delete_node
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut picosplay_node_t) -> (),
        ),
        Some(
            picoquic_wake_list_node_value
                as unsafe extern "C" fn(*mut picosplay_node_t) -> *mut ::core::ffi::c_void,
        ),
    );
}
unsafe extern "C" fn picoquic_remove_cnx_from_wake_list(mut cnx: *mut picoquic_cnx_t) {
    picosplay_delete_hint(
        &raw mut (*(*cnx).quic).cnx_wake_tree,
        &raw mut (*cnx).cnx_wake_node,
    );
}
unsafe extern "C" fn picoquic_insert_cnx_by_wake_time(
    mut quic: *mut picoquic_quic_t,
    mut cnx: *mut picoquic_cnx_t,
) {
    picosplay_insert(
        &raw mut (*quic).cnx_wake_tree,
        cnx as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_reinsert_by_wake_time(
    mut quic: *mut picoquic_quic_t,
    mut cnx: *mut picoquic_cnx_t,
    mut next_time: uint64_t,
) {
    picoquic_remove_cnx_from_wake_list(cnx);
    (*cnx).next_wake_time = next_time;
    picoquic_insert_cnx_by_wake_time(quic, cnx);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_earliest_cnx_to_wake(
    mut quic: *mut picoquic_quic_t,
    mut max_wake_time: uint64_t,
) -> *mut picoquic_cnx_t {
    let mut cnx: *mut picoquic_cnx_t =
        picoquic_wake_list_node_value(picosplay_first(&raw mut (*quic).cnx_wake_tree))
            as *mut picoquic_cnx_t;
    if !cnx.is_null() && max_wake_time != 0 as uint64_t && (*cnx).next_wake_time > max_wake_time {
        cnx = ::core::ptr::null_mut::<picoquic_cnx_t>();
    }
    return cnx as *mut picoquic_cnx_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_next_wake_time(
    mut quic: *mut picoquic_quic_t,
    mut current_time: uint64_t,
) -> uint64_t {
    let mut wake_time: uint64_t = UINT64_MAX as uint64_t;
    if !(*quic).pending_stateless_packet.is_null() {
        wake_time = current_time;
    } else {
        let mut cnx_wake_first: *mut picoquic_cnx_t =
            picoquic_wake_list_node_value(picosplay_first(&raw mut (*quic).cnx_wake_tree))
                as *mut picoquic_cnx_t;
        if !cnx_wake_first.is_null() {
            wake_time = (*cnx_wake_first).next_wake_time;
        }
    }
    return wake_time;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_next_wake_delay(
    mut quic: *mut picoquic_quic_t,
    mut current_time: uint64_t,
    mut delay_max: int64_t,
) -> int64_t {
    let mut next_wake_time: uint64_t = picoquic_get_next_wake_time(quic, current_time);
    let mut wake_delay: int64_t = 0 as int64_t;
    if next_wake_time > current_time {
        let mut delta_m: uint64_t = current_time.wrapping_add(delay_max as uint64_t);
        if next_wake_time >= delta_m {
            wake_delay = delay_max;
        } else {
            wake_delay = next_wake_time.wrapping_sub(current_time) as int64_t;
        }
    }
    return wake_delay;
}
unsafe extern "C" fn picoquic_get_wake_time(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
) -> uint64_t {
    let mut wake_time: uint64_t = UINT64_MAX as uint64_t;
    if !(*(*cnx).quic).pending_stateless_packet.is_null() {
        wake_time = current_time;
    } else {
        wake_time = (*cnx).next_wake_time;
    }
    return wake_time;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_wake_delay(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
    mut delay_max: int64_t,
) -> int64_t {
    let mut next_wake_time: uint64_t = picoquic_get_wake_time(cnx, current_time);
    let mut wake_delay: int64_t = 0 as int64_t;
    if next_wake_time > current_time {
        let mut delta_m: uint64_t = current_time.wrapping_add(delay_max as uint64_t);
        if next_wake_time >= delta_m {
            wake_delay = delay_max;
        } else {
            wake_delay = next_wake_time.wrapping_sub(current_time) as int64_t;
        }
    }
    return wake_delay;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_version_index(
    mut proposed_version: uint32_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: size_t = 0 as size_t;
    while i < picoquic_nb_supported_versions {
        if picoquic_supported_versions[i as usize].version == proposed_version {
            ret = i as ::core::ffi::c_int;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    return ret;
}
unsafe extern "C" fn picoquic_create_random_cnx_id(
    mut quic: *mut picoquic_quic_t,
    mut cnx_id: *mut picoquic_connection_id_t,
    mut id_length: uint8_t,
) {
    if id_length as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        picoquic_crypto_random(
            quic,
            &raw mut (*cnx_id).id as *mut uint8_t as *mut ::core::ffi::c_void,
            id_length as size_t,
        );
    }
    if (id_length as usize) < ::core::mem::size_of::<[uint8_t; 20]>() as usize {
        memset(
            (&raw mut (*cnx_id).id as *mut uint8_t).offset(id_length as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<[uint8_t; 20]>() as size_t).wrapping_sub(id_length as size_t),
        );
    }
    (*cnx_id).id_len = id_length;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_local_cnx_id(
    mut quic: *mut picoquic_quic_t,
    mut cnx_id: *mut picoquic_connection_id_t,
    mut id_length: uint8_t,
    mut cnx_id_remote: picoquic_connection_id_t,
) {
    picoquic_create_random_cnx_id(quic, cnx_id, (*quic).local_cnxid_length);
    if (*quic).cnx_id_callback_fn.is_some() {
        (*quic)
            .cnx_id_callback_fn
            .expect("non-null function pointer")(
            quic as *mut picoquic_quic_t,
            *cnx_id,
            cnx_id_remote,
            (*quic).cnx_id_callback_ctx,
            cnx_id,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_find_avalaible_unique_path_id(
    mut cnx: *mut picoquic_cnx_t,
    mut requested_id: uint64_t,
) -> uint64_t {
    let mut unique_path_id: uint64_t = requested_id;
    if requested_id == UINT64_MAX as uint64_t {
        if (*cnx).is_multipath_enabled() == 0 {
            unique_path_id = (*cnx).unique_path_id_next;
            (*cnx).unique_path_id_next = (*cnx).unique_path_id_next.wrapping_add(1);
        } else {
            let mut stash: *mut picoquic_remote_cnxid_stash_t = (*cnx).first_remote_cnxid_stash;
            while !stash.is_null()
                && ((*stash).is_in_use() as ::core::ffi::c_int != 0
                    || (*stash).unique_path_id == 0 as uint64_t)
            {
                stash = (*stash).next_stash as *mut picoquic_remote_cnxid_stash_t;
            }
            if !stash.is_null() {
                unique_path_id = (*stash).unique_path_id;
            }
        }
    }
    return unique_path_id;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_sequence_number(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut pc: picoquic_packet_context_enum,
) -> uint64_t {
    return if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
        && pc as ::core::ffi::c_uint
            == picoquic_packet_context_application as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*path_x).pkt_ctx.send_sequence
    } else {
        (*cnx).pkt_ctx[pc as usize].send_sequence
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_ack_number(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut pc: picoquic_packet_context_enum,
) -> uint64_t {
    return if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
        && pc as ::core::ffi::c_uint
            == picoquic_packet_context_application as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*path_x).pkt_ctx.highest_acknowledged
    } else {
        (*cnx).pkt_ctx[pc as usize].highest_acknowledged
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_last_packet(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut pc: picoquic_packet_context_enum,
) -> *mut picoquic_packet_t {
    return if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
        && pc as ::core::ffi::c_uint
            == picoquic_packet_context_application as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*path_x).pkt_ctx.pending_last
    } else {
        (*cnx).pkt_ctx[pc as usize].pending_last
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_path(
    mut cnx: *mut picoquic_cnx_t,
    mut start_time: uint64_t,
    mut local_addr: *const sockaddr,
    mut peer_addr: *const sockaddr,
    mut requested_id: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if (*cnx).nb_paths >= (*cnx).nb_path_alloc {
        let mut new_alloc: ::core::ffi::c_int = if (*cnx).nb_path_alloc == 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            2 as ::core::ffi::c_int * (*cnx).nb_path_alloc
        };
        let mut new_path: *mut *mut picoquic_path_t = malloc(
            (new_alloc as size_t)
                .wrapping_mul(::core::mem::size_of::<*mut picoquic_path_t>() as size_t),
        ) as *mut *mut picoquic_path_t;
        if !new_path.is_null() {
            if !(*cnx).path.is_null() {
                memset(
                    new_path as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (new_alloc as size_t)
                        .wrapping_mul(::core::mem::size_of::<*mut picoquic_path_t>() as size_t),
                );
                if (*cnx).nb_paths > 0 as ::core::ffi::c_int {
                    memcpy(
                        new_path as *mut ::core::ffi::c_void,
                        (*cnx).path as *const ::core::ffi::c_void,
                        ((*cnx).nb_paths as size_t)
                            .wrapping_mul(::core::mem::size_of::<*mut picoquic_path_t>() as size_t),
                    );
                }
                free((*cnx).path as *mut ::core::ffi::c_void);
            }
            (*cnx).path = new_path;
            (*cnx).nb_path_alloc = new_alloc;
        }
    }
    if (*cnx).nb_paths < (*cnx).nb_path_alloc {
        let mut unique_path_id: uint64_t =
            picoquic_find_avalaible_unique_path_id(cnx, requested_id);
        let mut path_x: *mut picoquic_path_t = if unique_path_id == UINT64_MAX as uint64_t {
            ::core::ptr::null_mut::<picoquic_path_t>()
        } else {
            malloc(::core::mem::size_of::<picoquic_path_t>() as size_t) as *mut picoquic_path_t
        };
        if !path_x.is_null() {
            memset(
                path_x as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<picoquic_path_t>() as size_t,
            );
            (*path_x).unique_path_id = unique_path_id;
            (*path_x).cnx = cnx as *mut st_picoquic_cnx_t;
            picoquic_update_peer_addr(path_x, peer_addr);
            picoquic_store_addr(&raw mut (*path_x).local_addr, local_addr);
            (*path_x).smoothed_rtt = PICOQUIC_INITIAL_RTT as uint64_t;
            (*path_x).rtt_variant = 0 as uint64_t;
            (*path_x).retransmit_timer = PICOQUIC_INITIAL_RETRANSMIT_TIMER as uint64_t;
            (*path_x).rtt_min = 0 as uint64_t;
            (*path_x).cwin = PICOQUIC_CWIN_INITIAL as uint64_t;
            (*path_x).bytes_in_transit = 0 as uint64_t;
            (*path_x).congestion_alg_state = NULL;
            picoquic_pacing_init(&raw mut (*path_x).pacing, start_time);
            picoquic_reset_path_mtu(path_x);
            (*path_x).rtt_update_delta = (*cnx).rtt_update_delta;
            (*path_x).pacing_rate_update_delta = (*cnx).pacing_rate_update_delta;
            picoquic_refresh_path_quality_thresholds(path_x);
            picoquic_init_ack_ctx(cnx, &raw mut (*path_x).ack_ctx);
            picoquic_init_packet_ctx(
                cnx,
                &raw mut (*path_x).pkt_ctx,
                picoquic_packet_context_application,
            );
            let ref mut c2rust_fresh8 = *(*cnx).path.offset((*cnx).nb_paths as isize);
            *c2rust_fresh8 = path_x;
            let c2rust_fresh9 = (*cnx).nb_paths;
            (*cnx).nb_paths = (*cnx).nb_paths + 1;
            ret = c2rust_fresh9;
            picoquic_set_path_challenge(cnx, (*cnx).nb_paths - 1 as ::core::ffi::c_int, start_time);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_register_path(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
) {
    if (*path_x).peer_addr.ss_family as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && (*(*cnx).quic).local_cnxid_length as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        picoquic_register_net_id((*cnx).quic, cnx, path_x);
    }
}
unsafe extern "C" fn picoquic_clear_path_data(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
) {
    picoquic_unregister_net_id(cnx, path_x);
    if !(*cnx).congestion_alg.is_null() {
        (*(*cnx).congestion_alg)
            .alg_delete
            .expect("non-null function pointer")(path_x as *mut picoquic_path_t);
    }
    free(path_x as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_path(
    mut cnx: *mut picoquic_cnx_t,
    mut path_index: ::core::ffi::c_int,
) {
    let mut path_x: *mut picoquic_path_t = *(*cnx).path.offset(path_index as isize);
    let mut p: *mut picoquic_packet_t = ::core::ptr::null_mut::<picoquic_packet_t>();
    let mut stream: *mut picoquic_stream_head_t = ::core::ptr::null_mut::<picoquic_stream_head_t>();
    picoquic_reset_packet_context(cnx, &raw mut (*path_x).pkt_ctx);
    picoquic_reset_ack_context(&raw mut (*path_x).ack_ctx);
    if !(*(*cnx).quic).F_log.is_null() {
        fflush((*(*cnx).quic).F_log as *mut FILE);
    }
    stream = picoquic_first_stream(cnx);
    while !stream.is_null() {
        if (*stream).affinity_path == path_x {
            (*stream).affinity_path = ::core::ptr::null_mut::<st_picoquic_path_t>();
        }
        stream = picoquic_next_stream(stream);
    }
    if (*cnx).are_path_callbacks_enabled() as ::core::ffi::c_int != 0
        && (*cnx).callback_fn.expect("non-null function pointer")(
            cnx as *mut picoquic_cnx_t,
            (*path_x).unique_path_id,
            ::core::ptr::null_mut::<uint8_t>(),
            0 as size_t,
            picoquic_callback_path_deleted,
            (*cnx).callback_ctx,
            (*path_x).app_path_ctx,
        ) != 0 as ::core::ffi::c_int
    {
        picoquic_connection_error_ex(
            cnx,
            PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t,
            0 as uint64_t,
            b"Path deleted callback failed.\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    let mut pc: picoquic_packet_context_enum = picoquic_packet_context_application;
    while (pc as ::core::ffi::c_uint)
        < picoquic_nb_packet_context as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        p = (*cnx).pkt_ctx[pc as usize].retransmitted_newest;
        while !p.is_null() {
            if (*p).send_path == path_x {
                (*p).send_path = ::core::ptr::null_mut::<st_picoquic_path_t>();
            }
            p = (*p).packet_next as *mut picoquic_packet_t;
        }
        pc += 1;
    }
    if (*cnx).is_multipath_enabled() != 0 {
        let mut local_cnxid_list: *mut picoquic_local_cnxid_list_t =
            picoquic_find_or_create_local_cnxid_list(
                cnx,
                (*path_x).unique_path_id,
                0 as ::core::ffi::c_int,
            );
        if !local_cnxid_list.is_null() {
            picoquic_delete_local_cnxid_list(cnx, local_cnxid_list);
        }
    }
    picoquic_clear_path_data(cnx, path_x);
    let mut i: ::core::ffi::c_int = path_index + 1 as ::core::ffi::c_int;
    while i < (*cnx).nb_paths {
        let ref mut c2rust_fresh2 = *(*cnx).path.offset((i - 1 as ::core::ffi::c_int) as isize);
        *c2rust_fresh2 = *(*cnx).path.offset(i as isize);
        i += 1;
    }
    (*cnx).nb_paths -= 1;
    let ref mut c2rust_fresh3 = *(*cnx).path.offset((*cnx).nb_paths as isize);
    *c2rust_fresh3 = ::core::ptr::null_mut::<picoquic_path_t>();
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_abandoned_paths(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
    mut next_wake_time: *mut uint64_t,
) {
    let mut path_index_good: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut path_index_current: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut is_demotion_in_progress: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
        && (*cnx).nb_paths > 1 as ::core::ffi::c_int
    {
        path_index_good = 0 as ::core::ffi::c_int;
        path_index_current = 0 as ::core::ffi::c_int;
    }
    while path_index_current < (*cnx).nb_paths {
        if (**(*cnx).path.offset(path_index_current as isize)).path_is_demoted() == 0 {
            if (**(*cnx).path.offset(path_index_current as isize)).challenge_failed()
                as ::core::ffi::c_int
                != 0
                || path_index_current > 0 as ::core::ffi::c_int
                    && (**(*cnx).path.offset(path_index_current as isize)).challenge_verified()
                        as ::core::ffi::c_int
                        != 0
                    && current_time.wrapping_sub(
                        (**(*cnx).path.offset(path_index_current as isize)).latest_sent_time,
                    ) >= (*cnx).idle_timeout
            {
                picoquic_demote_path(
                    cnx,
                    path_index_current,
                    current_time,
                    0 as uint64_t,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        }
        if (**(*cnx).path.offset(path_index_current as isize)).path_is_demoted()
            as ::core::ffi::c_int
            != 0
            && current_time >= (**(*cnx).path.offset(path_index_current as isize)).demotion_time
        {
            path_index_current += 1;
            is_demotion_in_progress |= 1 as ::core::ffi::c_uint;
        } else {
            if (**(*cnx).path.offset(path_index_current as isize)).path_is_demoted()
                as ::core::ffi::c_int
                != 0
                && current_time < (**(*cnx).path.offset(path_index_current as isize)).demotion_time
            {
                is_demotion_in_progress |= 1 as ::core::ffi::c_uint;
                if *next_wake_time
                    > (**(*cnx).path.offset(path_index_current as isize)).demotion_time
                {
                    *next_wake_time =
                        (**(*cnx).path.offset(path_index_current as isize)).demotion_time;
                    (*(*cnx).quic).wake_file = 3 as ::core::ffi::c_int;
                    (*(*cnx).quic).wake_line = 1772 as ::core::ffi::c_int;
                }
            }
            if path_index_current > path_index_good {
                let mut path_x: *mut picoquic_path_t =
                    *(*cnx).path.offset(path_index_current as isize);
                let ref mut c2rust_fresh25 = *(*cnx).path.offset(path_index_current as isize);
                *c2rust_fresh25 = *(*cnx).path.offset(path_index_good as isize);
                let ref mut c2rust_fresh26 = *(*cnx).path.offset(path_index_good as isize);
                *c2rust_fresh26 = path_x;
            }
            path_index_current += 1;
            path_index_good += 1;
        }
    }
    while (*cnx).nb_paths > path_index_good {
        let mut d_path: ::core::ffi::c_int = (*cnx).nb_paths - 1 as ::core::ffi::c_int;
        picoquic_dereference_stashed_cnxid(
            cnx,
            *(*cnx).path.offset(d_path as isize),
            0 as ::core::ffi::c_int,
        );
        picoquic_delete_path(cnx, d_path);
    }
    (*cnx).set_path_demotion_needed(is_demotion_in_progress as ::core::ffi::c_uint);
    let mut path_left: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut path_backup: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if is_demotion_in_progress != 0 && (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0 {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*cnx).nb_paths {
            if !((**(*cnx).path.offset(i as isize)).path_is_demoted() != 0) {
                if (**(*cnx).path.offset(i as isize)).path_is_standby() as ::core::ffi::c_int != 0
                    && path_backup < 0 as ::core::ffi::c_int
                {
                    path_backup = i;
                } else {
                    path_left = i;
                    break;
                }
            }
            i += 1;
        }
        if path_left < 0 as ::core::ffi::c_int && path_backup >= 0 as ::core::ffi::c_int {
            let ref mut c2rust_fresh27 = **(*cnx).path.offset(path_backup as isize);
            (*c2rust_fresh27).set_path_is_standby(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            picoquic_queue_path_available_or_standby_frame(
                cnx,
                *(*cnx).path.offset(path_backup as isize),
                picoquic_path_status_available,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_demote_path(
    mut cnx: *mut picoquic_cnx_t,
    mut path_index: ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut reason: uint64_t,
    mut phrase: *const ::core::ffi::c_char,
) {
    if (**(*cnx).path.offset(path_index as isize)).path_is_demoted() == 0 {
        let mut demote_timer: uint64_t =
            (**(*cnx).path.offset(path_index as isize)).retransmit_timer;
        if (demote_timer as ::core::ffi::c_ulonglong) < PICOQUIC_INITIAL_MAX_RETRANSMIT_TIMER
            && (*cnx).is_multipath_enabled() == 0
        {
            demote_timer = PICOQUIC_INITIAL_MAX_RETRANSMIT_TIMER as uint64_t;
        }
        let ref mut c2rust_fresh17 = **(*cnx).path.offset(path_index as isize);
        (*c2rust_fresh17).set_path_is_demoted(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (**(*cnx).path.offset(path_index as isize)).demotion_time =
            current_time.wrapping_add((3 as uint64_t).wrapping_mul(demote_timer));
        (*cnx).set_path_demotion_needed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if (*cnx).is_multipath_enabled() != 0 {
            if path_index == 0 as ::core::ffi::c_int {
                let mut alt_path0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                while i < (*cnx).nb_paths {
                    if !(**(*cnx).path.offset(path_index as isize))
                        .p_remote_cnxid
                        .is_null()
                    {
                        alt_path0 = i;
                        break;
                    } else {
                        i += 1;
                    }
                }
                if alt_path0 != 0 as ::core::ffi::c_int {
                    let mut path_x: *mut picoquic_path_t =
                        *(*cnx).path.offset(0 as ::core::ffi::c_int as isize);
                    let ref mut c2rust_fresh18 =
                        *(*cnx).path.offset(0 as ::core::ffi::c_int as isize);
                    *c2rust_fresh18 = *(*cnx).path.offset(alt_path0 as isize);
                    let ref mut c2rust_fresh19 = *(*cnx).path.offset(alt_path0 as isize);
                    *c2rust_fresh19 = path_x;
                    path_index = alt_path0;
                }
            }
            if path_index == 0 as ::core::ffi::c_int {
                picoquic_log_app_message(
                    cnx as *mut picoquic_cnx_t,
                    b"Cannot demote path index 0, unique_id %lu, was reason % lu\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (**(*cnx).path.offset(path_index as isize)).unique_path_id,
                    reason,
                );
            } else if (**(*cnx).path.offset(path_index as isize)).path_abandon_sent() == 0 {
                let mut path_id: uint64_t =
                    (**(*cnx).path.offset(path_index as isize)).unique_path_id;
                if picoquic_queue_path_abandon_frame(cnx, path_id, reason)
                    == 0 as ::core::ffi::c_int
                {
                    let mut remote_cnxid_stash: *mut picoquic_remote_cnxid_stash_t =
                        picoquic_find_or_create_remote_cnxid_stash(
                            cnx,
                            (**(*cnx).path.offset(path_index as isize)).unique_path_id,
                            0 as ::core::ffi::c_int,
                        );
                    if !remote_cnxid_stash.is_null() && path_index != 0 as ::core::ffi::c_int {
                        let ref mut c2rust_fresh20 =
                            (**(*cnx).path.offset(path_index as isize)).p_remote_cnxid;
                        *c2rust_fresh20 = ::core::ptr::null_mut::<picoquic_remote_cnxid_t>();
                        picoquic_delete_remote_cnxid_stash(cnx, remote_cnxid_stash);
                    }
                    picoquic_log_app_message(
                        cnx as *mut picoquic_cnx_t,
                        b"Abandon path, unique_id %lu, reason % lu\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        (**(*cnx).path.offset(path_index as isize)).unique_path_id,
                        reason,
                    );
                    let ref mut c2rust_fresh21 = **(*cnx).path.offset(path_index as isize);
                    (*c2rust_fresh21)
                        .set_path_abandon_sent(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                } else {
                    picoquic_log_app_message(
                        cnx as *mut picoquic_cnx_t,
                        b"Cannot queue abandon path [%lu]\0".as_ptr() as *const ::core::ffi::c_char,
                        (**(*cnx).path.offset(path_index as isize)).unique_path_id,
                    );
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_promote_path_to_default(
    mut cnx: *mut picoquic_cnx_t,
    mut path_index: ::core::ffi::c_int,
    mut current_time: uint64_t,
) {
    if path_index > 0 as ::core::ffi::c_int && path_index < (*cnx).nb_paths {
        let mut path_x: *mut picoquic_path_t = *(*cnx).path.offset(path_index as isize);
        if (**(*cnx).path.offset(path_index as isize)).path_is_preferred_path() != 0 {
            if (*cnx).client_mode() != 0 {
                (*cnx).remote_parameters.migration_disabled = 0 as ::core::ffi::c_uint;
            } else {
                (*cnx).local_parameters.migration_disabled = 0 as ::core::ffi::c_uint;
            }
        }
        if !(*(*cnx).quic).F_log.is_null() || !(*cnx).f_binlog.is_null() {
            let mut src_ip: [::core::ffi::c_char; 128] = [0; 128];
            let mut dst_ip: [::core::ffi::c_char; 128] = [0; 128];
            picoquic_log_app_message(
                cnx as *mut picoquic_cnx_t,
                b"Path %d promoted to default at T=%fs, Local: %s, Remote: %s\0".as_ptr()
                    as *const ::core::ffi::c_char,
                path_index,
                current_time.wrapping_sub((*cnx).start_time) as ::core::ffi::c_double
                    / 1000000.0f64,
                picoquic_addr_text(
                    &raw mut (**(*cnx).path.offset(path_index as isize)).local_addr
                        as *mut sockaddr,
                    &raw mut src_ip as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                ),
                picoquic_addr_text(
                    &raw mut (**(*cnx).path.offset(path_index as isize)).peer_addr as *mut sockaddr,
                    &raw mut dst_ip as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                ),
            );
        }
        if !(*cnx).congestion_alg.is_null() {
            (*(*cnx).congestion_alg)
                .alg_init
                .expect("non-null function pointer")(
                cnx as *mut picoquic_cnx_t,
                path_x as *mut picoquic_path_t,
                current_time,
            );
        }
        picoquic_demote_path(
            cnx,
            0 as ::core::ffi::c_int,
            current_time,
            0 as uint64_t,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        let ref mut c2rust_fresh23 = *(*cnx).path.offset(path_index as isize);
        *c2rust_fresh23 = *(*cnx).path.offset(0 as ::core::ffi::c_int as isize);
        let ref mut c2rust_fresh24 = *(*cnx).path.offset(0 as ::core::ffi::c_int as isize);
        *c2rust_fresh24 = path_x;
        picoquic_register_net_secret(cnx);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_path_challenge(
    mut cnx: *mut picoquic_cnx_t,
    mut path_id: ::core::ffi::c_int,
    mut current_time: uint64_t,
) {
    if (**(*cnx).path.offset(path_id as isize)).challenge_required() == 0
        || (**(*cnx).path.offset(path_id as isize)).challenge_verified() as ::core::ffi::c_int != 0
    {
        let ref mut c2rust_fresh10 = **(*cnx).path.offset(path_id as isize);
        (*c2rust_fresh10).set_challenge_required(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (**(*cnx).path.offset(path_id as isize)).challenge_time_first = current_time;
        let mut ichal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while ichal < PICOQUIC_CHALLENGE_REPEAT_MAX {
            if (*(*cnx).quic).use_constant_challenges() != 0 {
                (**(*cnx).path.offset(path_id as isize)).challenge[ichal as usize] =
                    (current_time as ::core::ffi::c_ulonglong).wrapping_mul(
                        (0xdeadbeef as ::core::ffi::c_ulonglong)
                            .wrapping_add(ichal as ::core::ffi::c_ulonglong),
                    ) as uint64_t;
            } else {
                (**(*cnx).path.offset(path_id as isize)).challenge[ichal as usize] =
                    picoquic_public_random_64();
            }
            ichal += 1;
        }
        if (**(*cnx).path.offset(path_id as isize)).challenge_verified() as ::core::ffi::c_int != 0
            && (*cnx).are_path_callbacks_enabled() as ::core::ffi::c_int != 0
        {
            if (*cnx).callback_fn.expect("non-null function pointer")(
                cnx as *mut picoquic_cnx_t,
                (**(*cnx).path.offset(path_id as isize)).unique_path_id,
                ::core::ptr::null_mut::<uint8_t>(),
                0 as size_t,
                picoquic_callback_path_suspended,
                (*cnx).callback_ctx,
                (**(*cnx).path.offset(path_id as isize)).app_path_ctx,
            ) != 0 as ::core::ffi::c_int
            {
                picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t,
                    picoquic_frame_type_path_challenge as ::core::ffi::c_int as uint64_t,
                );
            }
        }
        let ref mut c2rust_fresh11 = **(*cnx).path.offset(path_id as isize);
        (*c2rust_fresh11).set_challenge_verified(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (**(*cnx).path.offset(path_id as isize)).challenge_time = current_time;
        (**(*cnx).path.offset(path_id as isize)).challenge_repeat_count = 0 as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_find_path_by_address(
    mut cnx: *mut picoquic_cnx_t,
    mut addr_local: *const sockaddr,
    mut addr_peer: *const sockaddr,
    mut partial_match: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut path_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut is_null_from: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut null_addr: sockaddr_storage = sockaddr_storage {
        ss_family: 0,
        __ss_padding: [0; 118],
        __ss_align: 0,
    };
    *partial_match = -(1 as ::core::ffi::c_int);
    if !addr_peer.is_null() || !addr_local.is_null() {
        if addr_peer.is_null() || addr_local.is_null() {
            memset(
                &raw mut null_addr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<sockaddr_storage>() as size_t,
            );
            if addr_peer.is_null() {
                addr_peer = &raw mut null_addr as *mut sockaddr;
            } else {
                addr_local = &raw mut null_addr as *mut sockaddr;
            }
            is_null_from = 1 as ::core::ffi::c_int;
        } else if (*addr_local).sa_family as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            is_null_from = 1 as ::core::ffi::c_int;
        }
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*cnx).nb_paths {
            if picoquic_compare_addr(
                &raw mut (**(*cnx).path.offset(i as isize)).peer_addr as *mut sockaddr,
                addr_peer,
            ) == 0 as ::core::ffi::c_int
            {
                if (**(*cnx).path.offset(i as isize)).local_addr.ss_family as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    *partial_match = i;
                } else if picoquic_compare_addr(
                    &raw mut (**(*cnx).path.offset(i as isize)).local_addr as *mut sockaddr,
                    addr_local,
                ) == 0 as ::core::ffi::c_int
                {
                    path_id = i;
                    break;
                }
            }
            if path_id < 0 as ::core::ffi::c_int && is_null_from != 0 {
                path_id = *partial_match;
                *partial_match = -(1 as ::core::ffi::c_int);
            }
            i += 1;
        }
    }
    if path_id == -(1 as ::core::ffi::c_int) {
        let mut text1: [::core::ffi::c_char; 128] = [0; 128];
        let mut text2: [::core::ffi::c_char; 128] = [0; 128];
    }
    return path_id;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_find_path_by_unique_id(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
) -> ::core::ffi::c_int {
    let mut path_index: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*cnx).nb_paths {
        if (**(*cnx).path.offset(i as isize)).unique_path_id == unique_path_id {
            path_index = i;
            break;
        } else {
            i += 1;
        }
    }
    return path_index;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_notify_destination_unreachable(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
    mut addr_peer: *mut sockaddr,
    mut addr_local: *mut sockaddr,
    mut if_index: ::core::ffi::c_int,
    mut socket_err: ::core::ffi::c_int,
) {
    if !cnx.is_null() && !addr_peer.is_null() {
        let mut no_path_left: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut partial_match: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut path_id: ::core::ffi::c_int =
            picoquic_find_path_by_address(cnx, addr_local, addr_peer, &raw mut partial_match);
        if path_id >= 0 as ::core::ffi::c_int {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while no_path_left != 0 && i < (*cnx).nb_paths {
                no_path_left &=
                    (**(*cnx).path.offset(i as isize)).path_is_demoted() as ::core::ffi::c_int;
                i += 1;
            }
            if no_path_left != 0 {
                if (*cnx).cnx_state as ::core::ffi::c_uint
                    == picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    picoquic_set_path_challenge(cnx, path_id, current_time);
                }
            } else {
                picoquic_log_app_message(
                    cnx as *mut picoquic_cnx_t,
                    b"Demoting path %d after socket error %d, if %d\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    path_id,
                    socket_err,
                    if_index,
                );
                picoquic_demote_path(
                    cnx,
                    path_id,
                    current_time,
                    0 as uint64_t,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_notify_destination_unreachable_by_cnxid(
    mut quic: *mut picoquic_quic_t,
    mut cnxid: *mut picoquic_connection_id_t,
    mut current_time: uint64_t,
    mut addr_peer: *mut sockaddr,
    mut addr_local: *mut sockaddr,
    mut if_index: ::core::ffi::c_int,
    mut socket_err: ::core::ffi::c_int,
) {
    let mut cnx: *mut picoquic_cnx_t = ::core::ptr::null_mut::<picoquic_cnx_t>();
    if (*quic).local_cnxid_length as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        || (*cnxid).id_len as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        cnx = picoquic_cnx_by_net(quic, addr_peer);
    } else if (*cnxid).id_len as ::core::ffi::c_int
        == (*quic).local_cnxid_length as ::core::ffi::c_int
    {
        cnx = picoquic_cnx_by_id(
            quic,
            *cnxid,
            ::core::ptr::null_mut::<*mut st_picoquic_local_cnxid_t>(),
        );
    }
    if !cnx.is_null() {
        picoquic_notify_destination_unreachable(
            cnx,
            current_time,
            addr_peer,
            addr_local,
            if_index,
            socket_err,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_assign_peer_cnxid_to_path(
    mut cnx: *mut picoquic_cnx_t,
    mut path_index: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut unique_path_id: uint64_t = if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0 {
        (**(*cnx).path.offset(path_index as isize)).unique_path_id
    } else {
        0 as uint64_t
    };
    let mut stash: *mut picoquic_remote_cnxid_stash_t =
        picoquic_find_or_create_remote_cnxid_stash(cnx, unique_path_id, 0 as ::core::ffi::c_int);
    if !stash.is_null() {
        let mut available_cnxid: *mut picoquic_remote_cnxid_t =
            picoquic_get_cnxid_from_stash(stash);
        if !available_cnxid.is_null() {
            let ref mut c2rust_fresh15 = (**(*cnx).path.offset(path_index as isize)).p_remote_cnxid;
            *c2rust_fresh15 = available_cnxid;
            (*available_cnxid).nb_path_references += 1;
            (*stash).set_is_in_use(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            ret = 0 as ::core::ffi::c_int;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_probe_new_path_ex(
    mut cnx: *mut picoquic_cnx_t,
    mut addr_peer: *const sockaddr,
    mut addr_local: *const sockaddr,
    mut if_index: ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut to_preferred_address: ::core::ffi::c_int,
    mut path_id_p: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut partial_match_path: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut path_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if (*cnx).remote_parameters.migration_disabled != 0 && to_preferred_address == 0
        || (*cnx).local_parameters.migration_disabled != 0
    {
        ret = PICOQUIC_ERROR_MIGRATION_DISABLED;
    } else {
        path_id =
            picoquic_find_path_by_address(cnx, addr_local, addr_peer, &raw mut partial_match_path);
        if path_id >= 0 as ::core::ffi::c_int {
            ret = -(1 as ::core::ffi::c_int);
        } else if partial_match_path >= 0 as ::core::ffi::c_int
            && (*addr_peer).sa_family as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            ret = -(1 as ::core::ffi::c_int);
        } else if (*(*cnx).first_remote_cnxid_stash)
            .cnxid_stash_first
            .is_null()
        {
            ret = -(1 as ::core::ffi::c_int);
        } else if (*cnx).nb_paths >= PICOQUIC_NB_PATH_TARGET {
            ret = -(1 as ::core::ffi::c_int);
        } else if picoquic_create_path(
            cnx,
            current_time,
            addr_local,
            addr_peer,
            UINT64_MAX as uint64_t,
        ) > 0 as ::core::ffi::c_int
        {
            path_id = (*cnx).nb_paths - 1 as ::core::ffi::c_int;
            ret = picoquic_assign_peer_cnxid_to_path(cnx, path_id);
            if ret != 0 as ::core::ffi::c_int {
                picoquic_dereference_stashed_cnxid(
                    cnx,
                    *(*cnx).path.offset(path_id as isize),
                    0 as ::core::ffi::c_int,
                );
                picoquic_delete_path(cnx, path_id);
            } else {
                let ref mut c2rust_fresh12 = **(*cnx).path.offset(path_id as isize);
                (*c2rust_fresh12)
                    .set_path_is_published(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                picoquic_register_path(cnx, *(*cnx).path.offset(path_id as isize));
                picoquic_set_path_challenge(cnx, path_id, current_time);
                let ref mut c2rust_fresh13 = **(*cnx).path.offset(path_id as isize);
                (*c2rust_fresh13).set_path_is_preferred_path(
                    to_preferred_address as ::core::ffi::c_uint as ::core::ffi::c_uint,
                );
                let ref mut c2rust_fresh14 = **(*cnx).path.offset(path_id as isize);
                (*c2rust_fresh14)
                    .set_is_nat_challenge(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (**(*cnx).path.offset(path_id as isize)).if_index_dest =
                    if_index as ::core::ffi::c_ulong;
            }
        }
    }
    if !path_id_p.is_null() {
        *path_id_p = path_id;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_enable_path_callbacks(
    mut cnx: *mut picoquic_cnx_t,
    mut are_enabled: ::core::ffi::c_int,
) {
    (*cnx)
        .set_are_path_callbacks_enabled(are_enabled as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_enable_path_callbacks_default(
    mut quic: *mut picoquic_quic_t,
    mut are_enabled: ::core::ffi::c_int,
) {
    (*quic)
        .set_are_path_callbacks_enabled(are_enabled as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_path_id_from_unique(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*cnx).nb_paths {
        if (**(*cnx).path.offset(i as isize)).unique_path_id == unique_path_id {
            ret = i;
            break;
        } else {
            i += 1;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_app_path_ctx(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut app_path_ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut path_id: ::core::ffi::c_int = picoquic_get_path_id_from_unique(cnx, unique_path_id);
    if path_id >= 0 as ::core::ffi::c_int {
        let ref mut c2rust_fresh16 = (**(*cnx).path.offset(path_id as isize)).app_path_ctx;
        *c2rust_fresh16 = app_path_ctx;
    } else {
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_probe_new_path(
    mut cnx: *mut picoquic_cnx_t,
    mut addr_peer: *const sockaddr,
    mut addr_local: *const sockaddr,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    return picoquic_probe_new_path_ex(
        cnx,
        addr_peer,
        addr_local,
        0 as ::core::ffi::c_int,
        current_time,
        0 as ::core::ffi::c_int,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_demote_local_cnxid_list(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut reason: uint64_t,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut local_cnxid_list: *mut picoquic_local_cnxid_list_t =
        picoquic_find_or_create_local_cnxid_list(cnx, unique_path_id, 0 as ::core::ffi::c_int);
    if !local_cnxid_list.is_null() && (*local_cnxid_list).is_demoted() == 0 {
        ret = picoquic_queue_path_abandon_frame(cnx, unique_path_id, reason);
        if ret == 0 as ::core::ffi::c_int {
            let mut remote_cnxid_stash: *mut picoquic_remote_cnxid_stash_t =
                picoquic_find_or_create_remote_cnxid_stash(
                    cnx,
                    unique_path_id,
                    0 as ::core::ffi::c_int,
                );
            if !remote_cnxid_stash.is_null() {
                picoquic_delete_remote_cnxid_stash(cnx, remote_cnxid_stash);
            }
            (*local_cnxid_list).set_is_demoted(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_abandon_path(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut reason: uint64_t,
    mut phrase: *const ::core::ffi::c_char,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*cnx).is_multipath_enabled() == 0 {
        ret = -(1 as ::core::ffi::c_int);
    } else if unique_path_id > (*cnx).max_path_id_remote
        || unique_path_id > (*cnx).max_path_id_local
    {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        let mut path_index: ::core::ffi::c_int =
            picoquic_get_path_id_from_unique(cnx, unique_path_id);
        if path_index >= 0 as ::core::ffi::c_int {
            if (*cnx).nb_paths <= 1 as ::core::ffi::c_int {
                ret = -(1 as ::core::ffi::c_int);
            } else if (**(*cnx).path.offset(path_index as isize)).path_is_demoted() == 0 {
                picoquic_demote_path(cnx, path_index, current_time, reason, phrase);
            }
        } else {
            ret = picoquic_demote_local_cnxid_list(cnx, unique_path_id, reason, current_time);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_refresh_path_quality_thresholds(
    mut path_x: *mut picoquic_path_t,
) {
    if (*path_x).rtt_update_delta > 0 as uint64_t {
        if (*path_x).smoothed_rtt > (*path_x).rtt_update_delta {
            (*path_x).rtt_threshold_low = (*path_x)
                .smoothed_rtt
                .wrapping_sub((*path_x).rtt_update_delta);
        } else {
            (*path_x).rtt_threshold_low = 0 as uint64_t;
        }
        (*path_x).rtt_threshold_high = (*path_x)
            .smoothed_rtt
            .wrapping_add((*path_x).rtt_update_delta);
    }
    if (*path_x).pacing_rate_update_delta > 0 as uint64_t {
        if (*path_x).pacing.rate > (*path_x).pacing_rate_update_delta {
            (*path_x).pacing_rate_threshold_low = (*path_x)
                .pacing
                .rate
                .wrapping_sub((*path_x).pacing_rate_update_delta);
        } else {
            (*path_x).pacing_rate_threshold_low = 0 as uint64_t;
        }
        (*path_x).pacing_rate_threshold_high = (*path_x)
            .pacing
            .rate
            .wrapping_add((*path_x).pacing_rate_update_delta);
        if (*path_x).receive_rate_estimate > (*path_x).pacing_rate_update_delta {
            (*path_x).receive_rate_threshold_low = (*path_x)
                .receive_rate_estimate
                .wrapping_sub((*path_x).pacing_rate_update_delta);
        } else {
            (*path_x).receive_rate_threshold_low = 0 as uint64_t;
        }
        (*path_x).receive_rate_threshold_high = (*path_x)
            .receive_rate_estimate
            .wrapping_add((*path_x).pacing_rate_update_delta);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_issue_path_quality_update(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*path_x).rtt_update_delta > 0 as uint64_t
        && ((*path_x).smoothed_rtt < (*path_x).rtt_threshold_low
            || (*path_x).smoothed_rtt > (*path_x).rtt_threshold_high)
        || (*path_x).pacing_rate_update_delta > 0 as uint64_t
            && ((*path_x).pacing.rate < (*path_x).pacing_rate_threshold_low
                || (*path_x).pacing.rate > (*path_x).pacing_rate_threshold_high
                || (*path_x).receive_rate_estimate < (*path_x).receive_rate_threshold_low
                || (*path_x).receive_rate_estimate > (*path_x).receive_rate_threshold_high)
    {
        picoquic_refresh_path_quality_thresholds(path_x);
        ret = (*cnx).callback_fn.expect("non-null function pointer")(
            cnx as *mut picoquic_cnx_t,
            (*path_x).unique_path_id,
            ::core::ptr::null_mut::<uint8_t>(),
            0 as size_t,
            picoquic_callback_path_quality_changed,
            (*cnx).callback_ctx,
            NULL,
        );
    }
    return ret;
}
unsafe extern "C" fn picoquic_get_path_quality_from_context(
    mut path_x: *mut picoquic_path_t,
    mut quality: *mut picoquic_path_quality_t,
) {
    picoquic_refresh_path_quality_thresholds(path_x);
    (*quality).cwin = (*path_x).cwin;
    (*quality).rtt = (*path_x).smoothed_rtt;
    (*quality).rtt_sample = (*path_x).rtt_sample;
    (*quality).rtt_min = (*path_x).rtt_min;
    (*quality).rtt_max = (*path_x).rtt_max;
    (*quality).rtt_variant = (*path_x).rtt_variant;
    (*quality).pacing_rate = (*path_x).pacing.rate;
    (*quality).receive_rate_estimate = (*path_x).receive_rate_estimate;
    (*quality).sent = picoquic_get_sequence_number(
        (*path_x).cnx as *mut picoquic_cnx_t,
        path_x,
        picoquic_packet_context_application,
    );
    (*quality).lost = (*path_x).nb_losses_found;
    (*quality).timer_losses = (*path_x).nb_timer_losses;
    (*quality).spurious_losses = (*path_x).nb_spurious;
    (*quality).max_spurious_rtt = (*path_x).max_spurious_rtt;
    (*quality).max_reorder_delay = (*path_x).max_reorder_delay;
    (*quality).max_reorder_gap = (*path_x).max_reorder_gap;
    (*quality).bytes_in_transit = (*path_x).bytes_in_transit;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_path_quality(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut quality: *mut picoquic_path_quality_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut path_id: ::core::ffi::c_int = picoquic_get_path_id_from_unique(cnx, unique_path_id);
    if path_id >= 0 as ::core::ffi::c_int {
        let mut path_x: *mut picoquic_path_t = *(*cnx).path.offset(path_id as isize);
        picoquic_get_path_quality_from_context(path_x, quality);
        ret = 0 as ::core::ffi::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_default_path_quality(
    mut cnx: *mut picoquic_cnx_t,
    mut quality: *mut picoquic_path_quality_t,
) {
    let mut path_x: *mut picoquic_path_t = *(*cnx).path.offset(0 as ::core::ffi::c_int as isize);
    picoquic_get_path_quality_from_context(path_x, quality);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_subscribe_to_quality_update_per_path_context(
    mut path_x: *mut picoquic_path_t,
    mut pacing_rate_delta: uint64_t,
    mut rtt_delta: uint64_t,
) {
    (*path_x).pacing_rate_update_delta = pacing_rate_delta;
    (*path_x).rtt_update_delta = rtt_delta;
    picoquic_refresh_path_quality_thresholds(path_x);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_subscribe_to_quality_update_per_path(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut pacing_rate_delta: uint64_t,
    mut rtt_delta: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*cnx).set_is_path_quality_update_requested(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    let mut path_id: ::core::ffi::c_int = picoquic_get_path_id_from_unique(cnx, unique_path_id);
    if path_id >= 0 as ::core::ffi::c_int {
        picoquic_subscribe_to_quality_update_per_path_context(
            *(*cnx).path.offset(path_id as isize),
            pacing_rate_delta,
            rtt_delta,
        );
    } else {
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_subscribe_to_quality_update(
    mut cnx: *mut picoquic_cnx_t,
    mut pacing_rate_delta: uint64_t,
    mut rtt_delta: uint64_t,
) {
    (*cnx).pacing_rate_update_delta = pacing_rate_delta;
    (*cnx).rtt_update_delta = rtt_delta;
    (*cnx).set_is_path_quality_update_requested(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*cnx).nb_paths {
        picoquic_subscribe_to_quality_update_per_path_context(
            *(*cnx).path.offset(i as isize),
            pacing_rate_delta,
            rtt_delta,
        );
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_default_quality_update(
    mut quic: *mut picoquic_quic_t,
    mut pacing_rate_delta: uint64_t,
    mut rtt_delta: uint64_t,
) {
    (*quic).pacing_rate_update_delta = pacing_rate_delta;
    (*quic).rtt_update_delta = rtt_delta;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_refresh_path_connection_id(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut path_id: ::core::ffi::c_int = picoquic_get_path_id_from_unique(cnx, unique_path_id);
    if path_id >= 0 as ::core::ffi::c_int {
        ret = picoquic_renew_path_connection_id(cnx, *(*cnx).path.offset(path_id as isize));
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_stream_path_affinity(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut unique_path_id: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream: *mut picoquic_stream_head_t = picoquic_find_stream(cnx, stream_id);
    if stream.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else if unique_path_id == UINT64_MAX as uint64_t {
        (*stream).affinity_path = ::core::ptr::null_mut::<st_picoquic_path_t>();
    } else {
        let mut path_id: ::core::ffi::c_int = picoquic_get_path_id_from_unique(cnx, unique_path_id);
        if path_id >= 0 as ::core::ffi::c_int {
            (*stream).affinity_path =
                *(*cnx).path.offset(path_id as isize) as *mut st_picoquic_path_t;
        } else {
            ret = -(1 as ::core::ffi::c_int);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_path_status(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut status: picoquic_path_status_enum,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut path_id: ::core::ffi::c_int = picoquic_get_path_id_from_unique(cnx, unique_path_id);
    if path_id >= 0 as ::core::ffi::c_int {
        let ref mut c2rust_fresh22 = **(*cnx).path.offset(path_id as isize);
        (*c2rust_fresh22).set_path_is_standby(
            (status as ::core::ffi::c_uint
                != picoquic_path_status_available as ::core::ffi::c_int as ::core::ffi::c_uint)
                as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_uint,
        );
        ret = picoquic_queue_path_available_or_standby_frame(
            cnx,
            *(*cnx).path.offset(path_id as isize),
            status,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_path_addr(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut local: ::core::ffi::c_int,
    mut addr: *mut sockaddr_storage,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut path_id: ::core::ffi::c_int = picoquic_get_path_id_from_unique(cnx, unique_path_id);
    if path_id >= 0 as ::core::ffi::c_int {
        let mut local_addr: *mut sockaddr_storage = ::core::ptr::null_mut::<sockaddr_storage>();
        match local {
            1 => {
                local_addr = &raw mut (**(*cnx).path.offset(path_id as isize)).local_addr;
            }
            2 => {
                local_addr = &raw mut (**(*cnx).path.offset(path_id as isize)).peer_addr;
            }
            3 => {
                local_addr = &raw mut (**(*cnx).path.offset(path_id as isize)).observed_addr;
            }
            _ => {}
        }
        if local_addr.is_null() {
            ret = -(1 as ::core::ffi::c_int);
        } else {
            picoquic_store_addr(addr, local_addr as *mut sockaddr);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_update_peer_addr(
    mut path_x: *mut picoquic_path_t,
    mut peer_addr: *const sockaddr,
) {
    picoquic_store_addr(&raw mut (*path_x).peer_addr, peer_addr);
    (*path_x).set_observed_addr_acked(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*path_x).nb_observed_repeat = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_reset_path_mtu(mut path_x: *mut picoquic_path_t) {
    let mut quic: *mut picoquic_quic_t = (*(*path_x).cnx).quic;
    let mut is_ipv4: ::core::ffi::c_int = ((*path_x).peer_addr.ss_family as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
        || (*path_x).peer_addr.ss_family as ::core::ffi::c_int == AF_INET)
        as ::core::ffi::c_int;
    if is_ipv4 != 0 && (*quic).initial_send_mtu_ipv4 > 0 as uint32_t {
        (*path_x).send_mtu = (*quic).initial_send_mtu_ipv4 as size_t;
    } else if is_ipv4 == 0 && (*quic).initial_send_mtu_ipv6 > 0 as uint32_t {
        (*path_x).send_mtu = (*quic).initial_send_mtu_ipv6 as size_t;
    } else {
        (*path_x).send_mtu = (if is_ipv4 != 0 {
            PICOQUIC_INITIAL_MTU_IPV4
        } else {
            PICOQUIC_INITIAL_MTU_IPV6
        }) as size_t;
    }
    (*path_x).send_mtu > (*quic).mtu_max as size_t;
    (*path_x).send_mtu_max_tried = 0 as size_t;
    (*path_x).set_mtu_probe_sent(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_init_ack_ctx(
    mut cnx: *mut picoquic_cnx_t,
    mut ack_ctx: *mut picoquic_ack_context_t,
) {
    picoquic_sack_list_init(&raw mut (*ack_ctx).sack_list);
    (*ack_ctx).time_stamp_largest_received = UINT64_MAX as uint64_t;
    (*ack_ctx).act[0 as ::core::ffi::c_int as usize].highest_ack_sent = 0 as uint64_t;
    (*ack_ctx).act[0 as ::core::ffi::c_int as usize].highest_ack_sent_time = (*cnx).start_time;
    (*ack_ctx).act[0 as ::core::ffi::c_int as usize]
        .set_ack_needed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*ack_ctx).act[1 as ::core::ffi::c_int as usize].highest_ack_sent = 0 as uint64_t;
    (*ack_ctx).act[1 as ::core::ffi::c_int as usize].highest_ack_sent_time = (*cnx).start_time;
    (*ack_ctx).act[1 as ::core::ffi::c_int as usize]
        .set_ack_needed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_init_packet_ctx(
    mut cnx: *mut picoquic_cnx_t,
    mut pkt_ctx: *mut picoquic_packet_context_t,
    mut pc: picoquic_packet_context_enum,
) {
    if (*(*cnx).quic).random_initial() as ::core::ffi::c_int != 0
        && (pc as ::core::ffi::c_uint
            == picoquic_packet_context_initial as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*(*cnx).quic).random_initial() as ::core::ffi::c_int > 1 as ::core::ffi::c_int)
    {
        (*pkt_ctx).send_sequence =
            picoquic_crypto_uniform_random((*cnx).quic, PICOQUIC_PN_RANDOM_RANGE as uint64_t)
                .wrapping_add(PICOQUIC_PN_RANDOM_MIN as uint64_t);
    } else {
        (*pkt_ctx).send_sequence = 0 as uint64_t;
    }
    (*pkt_ctx).pending_last = ::core::ptr::null_mut::<picoquic_packet_t>();
    (*pkt_ctx).pending_first = ::core::ptr::null_mut::<picoquic_packet_t>();
    (*pkt_ctx).highest_acknowledged = (*pkt_ctx).send_sequence.wrapping_sub(1 as uint64_t);
    (*pkt_ctx).latest_time_acknowledged = (*cnx).start_time;
    (*pkt_ctx).highest_acknowledged_time = (*cnx).start_time;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_find_or_create_remote_cnxid_stash(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut do_create: ::core::ffi::c_int,
) -> *mut picoquic_remote_cnxid_stash_t {
    let mut remote_cnxid_stash: *mut picoquic_remote_cnxid_stash_t =
        (*cnx).first_remote_cnxid_stash;
    let mut p_previous: *mut *mut picoquic_remote_cnxid_stash_t =
        &raw mut (*cnx).first_remote_cnxid_stash;
    while !remote_cnxid_stash.is_null() && (*remote_cnxid_stash).unique_path_id != unique_path_id {
        p_previous =
            &raw mut (*remote_cnxid_stash).next_stash as *mut *mut picoquic_remote_cnxid_stash_t;
        remote_cnxid_stash = (*remote_cnxid_stash).next_stash as *mut picoquic_remote_cnxid_stash_t;
    }
    if remote_cnxid_stash.is_null() && do_create != 0 {
        remote_cnxid_stash =
            malloc(::core::mem::size_of::<picoquic_remote_cnxid_stash_t>() as size_t)
                as *mut picoquic_remote_cnxid_stash_t;
        if !remote_cnxid_stash.is_null() {
            memset(
                remote_cnxid_stash as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<picoquic_remote_cnxid_stash_t>() as size_t,
            );
            (*remote_cnxid_stash).unique_path_id = unique_path_id;
            *p_previous = remote_cnxid_stash;
        }
    }
    return remote_cnxid_stash;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_init_cnxid_stash(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut remote_cnxid_stash: *mut picoquic_remote_cnxid_stash_t =
        picoquic_find_or_create_remote_cnxid_stash(cnx, 0 as uint64_t, 1 as ::core::ffi::c_int);
    if remote_cnxid_stash.is_null() || !(*remote_cnxid_stash).cnxid_stash_first.is_null() {
        ret = PICOQUIC_TRANSPORT_INTERNAL_ERROR;
    } else {
        (*remote_cnxid_stash).cnxid_stash_first =
            malloc(::core::mem::size_of::<picoquic_remote_cnxid_t>() as size_t)
                as *mut picoquic_remote_cnxid_t;
        let ref mut c2rust_fresh7 =
            (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid;
        *c2rust_fresh7 = (*remote_cnxid_stash).cnxid_stash_first;
        if (*remote_cnxid_stash).cnxid_stash_first.is_null() {
            ret = PICOQUIC_TRANSPORT_INTERNAL_ERROR;
        } else {
            memset(
                (*remote_cnxid_stash).cnxid_stash_first as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<picoquic_remote_cnxid_t>() as size_t,
            );
            (*(*remote_cnxid_stash).cnxid_stash_first).nb_path_references += 1;
            picoquic_public_random(
                &raw mut (*(*remote_cnxid_stash).cnxid_stash_first).reset_secret as *mut uint8_t
                    as *mut ::core::ffi::c_void,
                PICOQUIC_RESET_SECRET_SIZE as size_t,
            );
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_add_remote_cnxid_to_stash(
    mut cnx: *mut picoquic_cnx_t,
    mut remote_cnxid_stash: *mut picoquic_remote_cnxid_stash_t,
    mut retire_before: uint64_t,
    sequence: uint64_t,
    cid_length: uint8_t,
    mut cnxid_bytes: *const uint8_t,
    mut secret_bytes: *const uint8_t,
    mut pstashed: *mut *mut picoquic_remote_cnxid_t,
) -> uint64_t {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_duplicate: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut nb_cid_received: size_t = 0 as size_t;
    let mut cnx_id: picoquic_connection_id_t = st_picoquic_connection_id_t {
        id: [0; 20],
        id_len: 0,
    };
    let mut next_stash: *mut picoquic_remote_cnxid_t = (*remote_cnxid_stash).cnxid_stash_first;
    let mut last_stash: *mut picoquic_remote_cnxid_t =
        ::core::ptr::null_mut::<picoquic_remote_cnxid_t>();
    let mut stashed: *mut picoquic_remote_cnxid_t =
        ::core::ptr::null_mut::<picoquic_remote_cnxid_t>();
    let mut nb_cid_retired_before: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if retire_before < (*remote_cnxid_stash).retire_cnxid_before {
        retire_before = (*remote_cnxid_stash).retire_cnxid_before;
    }
    if picoquic_parse_connection_id(cnxid_bytes, cid_length, &raw mut cnx_id) as ::core::ffi::c_int
        == 0 as ::core::ffi::c_int
    {
        ret = PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR;
    }
    if ret == 0 as ::core::ffi::c_int
        && (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid)
            .cnx_id
            .id_len as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        ret = PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION;
    }
    while ret == 0 as ::core::ffi::c_int
        && is_duplicate == 0 as ::core::ffi::c_int
        && !next_stash.is_null()
    {
        if picoquic_compare_connection_id(&raw mut cnx_id, &raw mut (*next_stash).cnx_id)
            == 0 as ::core::ffi::c_int
        {
            if (*next_stash).sequence == sequence
                && cnx_id.id_len as ::core::ffi::c_int
                    == (*next_stash).cnx_id.id_len as ::core::ffi::c_int
                && (cnx_id.id_len as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    || memcmp(
                        &raw mut cnx_id.id as *mut uint8_t as *const ::core::ffi::c_void,
                        &raw mut (*next_stash).cnx_id.id as *mut uint8_t
                            as *const ::core::ffi::c_void,
                        cnx_id.id_len as size_t,
                    ) == 0 as ::core::ffi::c_int)
                && memcmp(
                    secret_bytes as *const ::core::ffi::c_void,
                    &raw mut (*next_stash).reset_secret as *mut uint8_t
                        as *const ::core::ffi::c_void,
                    PICOQUIC_RESET_SECRET_SIZE as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                is_duplicate = 1 as ::core::ffi::c_int;
            } else {
                ret = PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION;
            }
            break;
        } else {
            if (*next_stash).sequence == sequence {
                ret = PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION;
            } else if memcmp(
                secret_bytes as *const ::core::ffi::c_void,
                &raw mut (*next_stash).reset_secret as *mut uint8_t as *const ::core::ffi::c_void,
                PICOQUIC_RESET_SECRET_SIZE as size_t,
            ) == 0 as ::core::ffi::c_int
            {
                ret = PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION;
            } else {
                if (*next_stash).sequence < retire_before
                    || (*next_stash).retire_sent() as ::core::ffi::c_int != 0
                {
                    nb_cid_retired_before += 1;
                }
                nb_cid_received = nb_cid_received.wrapping_add(1);
            }
            last_stash = next_stash;
            next_stash = (*next_stash).next as *mut picoquic_remote_cnxid_t;
        }
    }
    if ret == 0 as ::core::ffi::c_int && is_duplicate == 0 as ::core::ffi::c_int {
        if nb_cid_received
            >= (*cnx)
                .local_parameters
                .active_connection_id_limit
                .wrapping_add(nb_cid_retired_before as uint32_t) as size_t
            || nb_cid_received
                >= (2 as uint32_t).wrapping_mul((*cnx).local_parameters.active_connection_id_limit)
                    as size_t
        {
            ret = PICOQUIC_TRANSPORT_CONNECTION_ID_LIMIT_ERROR;
        } else {
            stashed = malloc(::core::mem::size_of::<picoquic_remote_cnxid_t>() as size_t)
                as *mut picoquic_remote_cnxid_t;
            if stashed.is_null() {
                ret = PICOQUIC_TRANSPORT_INTERNAL_ERROR;
            } else {
                memset(
                    stashed as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<picoquic_remote_cnxid_t>() as size_t,
                );
                picoquic_parse_connection_id(cnxid_bytes, cid_length, &raw mut (*stashed).cnx_id);
                (*stashed).sequence = sequence;
                memcpy(
                    &raw mut (*stashed).reset_secret as *mut uint8_t as *mut ::core::ffi::c_void,
                    secret_bytes as *const ::core::ffi::c_void,
                    PICOQUIC_RESET_SECRET_SIZE as size_t,
                );
                (*stashed).next = ::core::ptr::null_mut::<st_picoquic_remote_cnxid_t>();
                if last_stash.is_null() {
                    (*remote_cnxid_stash).cnxid_stash_first = stashed;
                } else {
                    (*last_stash).next = stashed as *mut st_picoquic_remote_cnxid_t;
                }
            }
        }
    }
    if !pstashed.is_null() {
        *pstashed = stashed;
    }
    return ret as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_stash_remote_cnxid(
    mut cnx: *mut picoquic_cnx_t,
    mut retire_before_next: uint64_t,
    unique_path_id: uint64_t,
    sequence: uint64_t,
    cid_length: uint8_t,
    mut cnxid_bytes: *const uint8_t,
    mut secret_bytes: *const uint8_t,
    mut pstashed: *mut *mut picoquic_remote_cnxid_t,
) -> uint64_t {
    let mut transport_error: uint64_t = 0 as uint64_t;
    let mut remote_cnxid_stash: *mut picoquic_remote_cnxid_stash_t =
        picoquic_find_or_create_remote_cnxid_stash(cnx, unique_path_id, 1 as ::core::ffi::c_int);
    if remote_cnxid_stash.is_null() {
        transport_error = PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t;
    } else {
        transport_error = picoquic_add_remote_cnxid_to_stash(
            cnx,
            remote_cnxid_stash,
            retire_before_next,
            sequence,
            cid_length,
            cnxid_bytes,
            secret_bytes,
            pstashed,
        );
    }
    return transport_error;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_remove_cnxid_from_stash(
    mut cnx: *mut picoquic_cnx_t,
    mut remote_cnxid_stash: *mut picoquic_remote_cnxid_stash_t,
    mut removed: *mut picoquic_remote_cnxid_t,
    mut previous: *mut picoquic_remote_cnxid_t,
) -> *mut picoquic_remote_cnxid_t {
    let mut stashed: *mut picoquic_remote_cnxid_t =
        ::core::ptr::null_mut::<picoquic_remote_cnxid_t>();
    if !cnx.is_null()
        && !remote_cnxid_stash.is_null()
        && !(*remote_cnxid_stash).cnxid_stash_first.is_null()
        && !removed.is_null()
    {
        stashed = (*remote_cnxid_stash).cnxid_stash_first;
        if !previous.is_null() {
            if (*previous).next == removed {
                stashed = removed;
            } else {
                previous = ::core::ptr::null_mut::<picoquic_remote_cnxid_t>();
            }
        }
        if previous.is_null() {
            while !stashed.is_null() && removed != stashed {
                previous = stashed;
                stashed = (*stashed).next as *mut picoquic_remote_cnxid_t;
            }
        }
        if !stashed.is_null() {
            stashed = (*stashed).next as *mut picoquic_remote_cnxid_t;
            if previous.is_null() {
                (*remote_cnxid_stash).cnxid_stash_first = stashed;
            } else {
                (*previous).next = stashed as *mut st_picoquic_remote_cnxid_t;
            }
            free(removed as *mut ::core::ffi::c_void);
        }
    }
    return stashed;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_remove_stashed_cnxid(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut removed: *mut picoquic_remote_cnxid_t,
    mut previous: *mut picoquic_remote_cnxid_t,
) -> *mut picoquic_remote_cnxid_t {
    let mut remote_cnxid_stash: *mut picoquic_remote_cnxid_stash_t =
        picoquic_find_or_create_remote_cnxid_stash(
            cnx,
            if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0 {
                unique_path_id
            } else {
                0 as uint64_t
            },
            0 as ::core::ffi::c_int,
        );
    return picoquic_remove_cnxid_from_stash(cnx, remote_cnxid_stash, removed, previous);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_cnxid_from_stash(
    mut stash: *mut picoquic_remote_cnxid_stash_t,
) -> *mut picoquic_remote_cnxid_t {
    let mut stashed: *mut picoquic_remote_cnxid_t =
        ::core::ptr::null_mut::<picoquic_remote_cnxid_t>();
    if !stash.is_null() {
        stashed = (*stash).cnxid_stash_first;
        while !stashed.is_null()
            && (*stashed).cnx_id.id_len as ::core::ffi::c_int > 0 as ::core::ffi::c_int
            && ((*stashed).nb_path_references != 0 as ::core::ffi::c_int
                || (*stashed).needs_removal() as ::core::ffi::c_int != 0)
        {
            stashed = (*stashed).next as *mut picoquic_remote_cnxid_t;
        }
    }
    return stashed;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_obtain_stashed_cnxid(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
) -> *mut picoquic_remote_cnxid_t {
    let mut stash: *mut picoquic_remote_cnxid_stash_t =
        picoquic_find_or_create_remote_cnxid_stash(cnx, unique_path_id, 0 as ::core::ffi::c_int);
    let mut stashed: *mut picoquic_remote_cnxid_t = picoquic_get_cnxid_from_stash(stash);
    return stashed;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_dereference_stashed_cnxid(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut is_deleting_cnx: ::core::ffi::c_int,
) {
    if !(*path_x).p_remote_cnxid.is_null() {
        if (*(*path_x).p_remote_cnxid).nb_path_references <= 1 as ::core::ffi::c_int {
            let mut unique_path_id: uint64_t =
                if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0 {
                    (*path_x).unique_path_id
                } else {
                    0 as uint64_t
                };
            if is_deleting_cnx == 0 && (*(*path_x).p_remote_cnxid).retire_sent() == 0 {
                if !(picoquic_queue_retire_connection_id_frame(
                    cnx,
                    unique_path_id,
                    (*(*path_x).p_remote_cnxid).sequence,
                ) != 0 as ::core::ffi::c_int)
                {
                    (*(*path_x).p_remote_cnxid)
                        .set_retire_sent(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
            }
            if is_deleting_cnx != 0
                || (*(*path_x).p_remote_cnxid).retire_acked() as ::core::ffi::c_int != 0
            {
                picoquic_remove_stashed_cnxid(
                    cnx,
                    (*path_x).unique_path_id,
                    (*path_x).p_remote_cnxid,
                    ::core::ptr::null_mut::<picoquic_remote_cnxid_t>(),
                );
            }
        } else {
            (*(*path_x).p_remote_cnxid).nb_path_references -= 1;
        }
    }
    (*path_x).p_remote_cnxid = ::core::ptr::null_mut::<picoquic_remote_cnxid_t>();
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_remove_not_before_from_stash(
    mut cnx: *mut picoquic_cnx_t,
    mut cnxid_stash: *mut picoquic_remote_cnxid_stash_t,
    mut not_before: uint64_t,
    mut current_time: uint64_t,
) -> uint64_t {
    let mut ret: uint64_t = 0 as uint64_t;
    if !cnxid_stash.is_null() {
        let mut next_stash: *mut picoquic_remote_cnxid_t = (*cnxid_stash).cnxid_stash_first;
        let mut previous_stash: *mut picoquic_remote_cnxid_t =
            ::core::ptr::null_mut::<picoquic_remote_cnxid_t>();
        while ret == 0 as uint64_t && !next_stash.is_null() {
            (*next_stash).set_needs_removal(
                (*next_stash).needs_removal()
                    | ((*next_stash).sequence < not_before) as ::core::ffi::c_int
                        as ::core::ffi::c_uint,
            );
            if (*next_stash).needs_removal() as ::core::ffi::c_int != 0
                && (*next_stash).nb_path_references == 0 as ::core::ffi::c_int
            {
                if (*next_stash).retire_sent() == 0 {
                    ret = picoquic_queue_retire_connection_id_frame(
                        cnx,
                        (*cnxid_stash).unique_path_id,
                        (*next_stash).sequence,
                    ) as uint64_t;
                    if ret == 0 as uint64_t {
                        (*next_stash)
                            .set_retire_sent(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                }
                if ret == 0 as uint64_t && (*next_stash).retire_acked() as ::core::ffi::c_int != 0 {
                    next_stash = picoquic_remove_cnxid_from_stash(
                        cnx,
                        cnxid_stash,
                        next_stash,
                        previous_stash,
                    );
                } else {
                    previous_stash = next_stash;
                    next_stash = (*next_stash).next as *mut picoquic_remote_cnxid_t;
                }
            } else {
                previous_stash = next_stash;
                next_stash = (*next_stash).next as *mut picoquic_remote_cnxid_t;
            }
        }
        if (*cnx).is_multipath_enabled() != 0 {
            let mut path_id: ::core::ffi::c_int =
                picoquic_find_path_by_unique_id(cnx, (*cnxid_stash).unique_path_id);
            if path_id >= 0 as ::core::ffi::c_int {
                if (*(**(*cnx).path.offset(path_id as isize)).p_remote_cnxid).sequence < not_before
                    && (*(**(*cnx).path.offset(path_id as isize)).p_remote_cnxid)
                        .cnx_id
                        .id_len as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int
                    && (**(*cnx).path.offset(path_id as isize)).path_is_demoted() == 0
                {
                    ret = picoquic_renew_connection_id(cnx, path_id) as uint64_t;
                    if ret != 0 as uint64_t {
                        if path_id == 0 as ::core::ffi::c_int {
                            ret = PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t;
                        } else {
                            ret = 0 as uint64_t;
                            picoquic_demote_path(
                                cnx,
                                path_id,
                                current_time,
                                0 as uint64_t,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                            );
                        }
                    }
                }
            }
        } else {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while ret == 0 as uint64_t && i < (*cnx).nb_paths {
                if (*(**(*cnx).path.offset(i as isize)).p_remote_cnxid).sequence < not_before
                    && (*(**(*cnx).path.offset(i as isize)).p_remote_cnxid)
                        .cnx_id
                        .id_len as ::core::ffi::c_int
                        > 0 as ::core::ffi::c_int
                    && (**(*cnx).path.offset(i as isize)).path_is_demoted() == 0
                {
                    ret = picoquic_renew_connection_id(cnx, i) as uint64_t;
                    if ret != 0 as uint64_t {
                        if i == 0 as ::core::ffi::c_int {
                            ret = PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t;
                        } else {
                            ret = 0 as uint64_t;
                            picoquic_demote_path(
                                cnx,
                                i,
                                current_time,
                                0 as uint64_t,
                                ::core::ptr::null::<::core::ffi::c_char>(),
                            );
                        }
                    }
                }
                i += 1;
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_remove_not_before_cid(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut not_before: uint64_t,
    mut current_time: uint64_t,
) -> uint64_t {
    let mut transport_error: uint64_t = 0 as uint64_t;
    let mut cnxid_stash: *mut picoquic_remote_cnxid_stash_t =
        picoquic_find_or_create_remote_cnxid_stash(cnx, unique_path_id, 0 as ::core::ffi::c_int);
    if !cnxid_stash.is_null() {
        transport_error =
            picoquic_remove_not_before_from_stash(cnx, cnxid_stash, not_before, current_time);
    }
    return transport_error;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_remote_cnxid_stash(
    mut cnx: *mut picoquic_cnx_t,
    mut cnxid_stash: *mut picoquic_remote_cnxid_stash_t,
) {
    let mut previous: *mut picoquic_remote_cnxid_stash_t = (*cnx).first_remote_cnxid_stash;
    while !(*cnxid_stash).cnxid_stash_first.is_null() {
        picoquic_remove_cnxid_from_stash(
            cnx,
            cnxid_stash,
            (*cnxid_stash).cnxid_stash_first,
            ::core::ptr::null_mut::<picoquic_remote_cnxid_t>(),
        );
    }
    if previous == cnxid_stash {
        (*cnx).first_remote_cnxid_stash =
            (*cnxid_stash).next_stash as *mut picoquic_remote_cnxid_stash_t;
    } else {
        while !previous.is_null() {
            if (*previous).next_stash == cnxid_stash {
                (*previous).next_stash = (*cnxid_stash).next_stash;
                break;
            } else {
                previous = (*previous).next_stash as *mut picoquic_remote_cnxid_stash_t;
            }
        }
    }
    free(cnxid_stash as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_remote_cnxid_stashes(mut cnx: *mut picoquic_cnx_t) {
    while !(*cnx).first_remote_cnxid_stash.is_null() {
        picoquic_delete_remote_cnxid_stash(cnx, (*cnx).first_remote_cnxid_stash);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_renew_path_connection_id(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stashed: *mut picoquic_remote_cnxid_t =
        ::core::ptr::null_mut::<picoquic_remote_cnxid_t>();
    let mut cid_path_id: uint64_t = if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0 {
        (*path_x).unique_path_id
    } else {
        0 as uint64_t
    };
    let mut cnxid_stash: *mut picoquic_remote_cnxid_stash_t =
        picoquic_find_or_create_remote_cnxid_stash(cnx, cid_path_id, 0 as ::core::ffi::c_int);
    if cnxid_stash.is_null() {
        ret = PICOQUIC_ERROR_CNXID_NOT_AVAILABLE;
    } else if (*cnx).remote_parameters.migration_disabled != 0 as ::core::ffi::c_uint
        && !(*path_x).p_remote_cnxid.is_null()
        && (*(*path_x).p_remote_cnxid).sequence >= (*cnxid_stash).retire_cnxid_before
        || (*cnx).local_parameters.migration_disabled != 0 as ::core::ffi::c_uint
    {
        ret = PICOQUIC_ERROR_MIGRATION_DISABLED;
    } else {
        stashed = picoquic_obtain_stashed_cnxid(cnx, cid_path_id);
        if stashed.is_null() {
            ret = PICOQUIC_ERROR_CNXID_NOT_AVAILABLE;
        } else if !(*path_x).p_remote_cnxid.is_null()
            && (*stashed).sequence == (*(*path_x).p_remote_cnxid).sequence
        {
            ret = PICOQUIC_ERROR_CNXID_NOT_AVAILABLE;
        } else {
            picoquic_dereference_stashed_cnxid(cnx, path_x, 0 as ::core::ffi::c_int);
            (*path_x).p_remote_cnxid = stashed;
            (*stashed).nb_path_references += 1;
            if path_x == *(*cnx).path.offset(0 as ::core::ffi::c_int as isize) {
                ret = picoquic_register_net_secret(cnx);
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_renew_connection_id(
    mut cnx: *mut picoquic_cnx_t,
    mut path_id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if path_id >= (*cnx).nb_paths {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        ret = picoquic_renew_path_connection_id(cnx, *(*cnx).path.offset(path_id as isize));
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_stream_data_node_compare(
    mut l: *mut ::core::ffi::c_void,
    mut r: *mut ::core::ffi::c_void,
) -> int64_t {
    return (*(l as *mut picoquic_stream_data_node_t))
        .offset
        .wrapping_sub((*(r as *mut picoquic_stream_data_node_t)).offset) as int64_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_stream_data_node_create(
    mut value: *mut ::core::ffi::c_void,
) -> *mut picosplay_node_t {
    return &raw mut (*(value as *mut picoquic_stream_data_node_t)).stream_data_node;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_stream_data_node_value(
    mut node: *mut picosplay_node_t,
) -> *mut ::core::ffi::c_void {
    return (node as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
        as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_stream_data_node_recycle(
    mut stream_data: *mut picoquic_stream_data_node_t,
) {
    if (*(*stream_data).quic).nb_data_nodes_in_pool < PICOQUIC_MAX_PACKETS_IN_POOL {
        (*stream_data).next_stream_data =
            (*(*stream_data).quic).p_first_data_node as *mut st_picoquic_stream_data_node_t;
        (*(*stream_data).quic).p_first_data_node = stream_data;
        (*(*stream_data).quic).nb_data_nodes_in_pool += 1;
    } else {
        (*(*stream_data).quic).nb_data_nodes_allocated -= 1;
        free(stream_data as *mut ::core::ffi::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_stream_data_node_delete(
    mut tree: *mut ::core::ffi::c_void,
    mut node: *mut picosplay_node_t,
) {
    let mut stream_data: *mut picoquic_stream_data_node_t =
        picoquic_stream_data_node_value(node) as *mut picoquic_stream_data_node_t;
    picoquic_stream_data_node_recycle(stream_data);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_stream_data_node_alloc(
    mut quic: *mut picoquic_quic_t,
) -> *mut picoquic_stream_data_node_t {
    let mut stream_data: *mut picoquic_stream_data_node_t = (*quic).p_first_data_node;
    if stream_data.is_null() {
        stream_data = malloc(::core::mem::size_of::<picoquic_stream_data_node_t>() as size_t)
            as *mut picoquic_stream_data_node_t;
        if !stream_data.is_null() {
            memset(
                stream_data as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<picoquic_stream_data_node_t>() as size_t,
            );
            (*stream_data).quic = quic as *mut picoquic_quic_t;
            (*quic).nb_data_nodes_allocated += 1;
            if (*quic).nb_data_nodes_allocated > (*quic).nb_data_nodes_allocated_max {
                (*quic).nb_data_nodes_allocated_max = (*quic).nb_data_nodes_allocated;
            }
        }
    } else {
        (*quic).p_first_data_node =
            (*stream_data).next_stream_data as *mut picoquic_stream_data_node_t;
        (*stream_data).next_stream_data = ::core::ptr::null_mut::<st_picoquic_stream_data_node_t>();
        (*stream_data).bytes = ::core::ptr::null::<uint8_t>();
        (*quic).nb_data_nodes_in_pool -= 1;
    }
    return stream_data;
}
unsafe extern "C" fn picoquic_stream_node_compare(
    mut l: *mut ::core::ffi::c_void,
    mut r: *mut ::core::ffi::c_void,
) -> int64_t {
    return (*(l as *mut picoquic_stream_head_t))
        .stream_id
        .wrapping_sub((*(r as *mut picoquic_stream_head_t)).stream_id) as int64_t;
}
unsafe extern "C" fn picoquic_stream_node_create(
    mut value: *mut ::core::ffi::c_void,
) -> *mut picosplay_node_t {
    return &raw mut (*(value as *mut picoquic_stream_head_t)).stream_node;
}
unsafe extern "C" fn picoquic_stream_node_value(
    mut node: *mut picosplay_node_t,
) -> *mut ::core::ffi::c_void {
    return (node as *mut ::core::ffi::c_char).offset(-(0 as ::core::ffi::c_ulong as isize))
        as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_clear_stream(mut stream: *mut picoquic_stream_head_t) {
    let mut ready: *mut picoquic_stream_queue_node_t = (*stream).send_queue;
    let mut next: *mut picoquic_stream_queue_node_t =
        ::core::ptr::null_mut::<picoquic_stream_queue_node_t>();
    loop {
        next = ready;
        if next.is_null() {
            break;
        }
        ready = (*next).next_stream_data as *mut picoquic_stream_queue_node_t;
        if !(*next).bytes.is_null() {
            free((*next).bytes as *mut ::core::ffi::c_void);
        }
        free(next as *mut ::core::ffi::c_void);
    }
    (*stream).send_queue = ::core::ptr::null_mut::<picoquic_stream_queue_node_t>();
    if (*stream).is_output_stream() != 0 {
        picoquic_remove_output_stream((*stream).cnx as *mut picoquic_cnx_t, stream);
    }
    picosplay_empty_tree(&raw mut (*stream).stream_data_tree);
    picoquic_sack_list_free(&raw mut (*stream).sack_list);
}
unsafe extern "C" fn picoquic_stream_node_delete(
    mut tree: *mut ::core::ffi::c_void,
    mut node: *mut picosplay_node_t,
) {
    let mut stream: *mut picoquic_stream_head_t =
        picoquic_stream_node_value(node) as *mut picoquic_stream_head_t;
    picoquic_clear_stream(stream);
    free(stream as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_stream_from_node(
    mut node: *mut picosplay_node_t,
) -> *mut picoquic_stream_head_t {
    return node as *mut picoquic_stream_head_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_first_stream(
    mut cnx: *mut picoquic_cnx_t,
) -> *mut picoquic_stream_head_t {
    return picosplay_first(&raw mut (*cnx).stream_tree) as *mut picoquic_stream_head_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_last_stream(
    mut cnx: *mut picoquic_cnx_t,
) -> *mut picoquic_stream_head_t {
    return picosplay_last(&raw mut (*cnx).stream_tree) as *mut picoquic_stream_head_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_compare_stream_priority(
    mut stream: *mut picoquic_stream_head_t,
    mut other: *mut picoquic_stream_head_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if ((*stream).stream_priority as ::core::ffi::c_int)
        < (*other).stream_priority as ::core::ffi::c_int
    {
        ret = -(1 as ::core::ffi::c_int);
    } else if (*stream).stream_priority as ::core::ffi::c_int
        == (*other).stream_priority as ::core::ffi::c_int
    {
        if (*stream).stream_id < (*other).stream_id {
            ret = -(1 as ::core::ffi::c_int);
        } else if (*stream).stream_id == (*other).stream_id {
            ret = 0 as ::core::ffi::c_int;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_insert_output_stream(
    mut cnx: *mut picoquic_cnx_t,
    mut stream: *mut picoquic_stream_head_t,
) {
    if (*stream).is_output_stream() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        if ((*stream).stream_id & 1 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
            as ::core::ffi::c_uint
            == (*cnx).client_mode()
        {
            if (*stream).stream_id
                > (if ((*stream).stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                    != 0
                {
                    (*cnx).max_stream_id_bidir_remote
                } else {
                    (*cnx).max_stream_id_unidir_remote
                })
            {
                return;
            }
        }
        if (*cnx).last_output_stream.is_null() {
            (*cnx).last_output_stream = stream;
            (*cnx).first_output_stream = stream;
        } else if picoquic_compare_stream_priority(stream, (*cnx).last_output_stream)
            >= 0 as ::core::ffi::c_int
        {
            (*stream).previous_output_stream =
                (*cnx).last_output_stream as *mut st_picoquic_stream_head_t;
            (*(*cnx).last_output_stream).next_output_stream =
                stream as *mut st_picoquic_stream_head_t;
            (*cnx).last_output_stream = stream;
        } else {
            let mut current: *mut picoquic_stream_head_t = (*cnx).first_output_stream;
            while !current.is_null() {
                let mut cmp: ::core::ffi::c_int = picoquic_compare_stream_priority(stream, current);
                if cmp < 0 as ::core::ffi::c_int {
                    (*stream).previous_output_stream = (*current).previous_output_stream;
                    if (*stream).previous_output_stream.is_null() {
                        (*cnx).first_output_stream = stream;
                    } else {
                        (*(*stream).previous_output_stream).next_output_stream =
                            stream as *mut st_picoquic_stream_head_t;
                    }
                    (*current).previous_output_stream = stream as *mut st_picoquic_stream_head_t;
                    (*stream).next_output_stream = current as *mut st_picoquic_stream_head_t;
                    break;
                } else {
                    if cmp == 0 as ::core::ffi::c_int {
                        break;
                    }
                    current = (*current).next_output_stream as *mut picoquic_stream_head_t;
                }
            }
            if current.is_null() {
                (*stream).previous_output_stream =
                    (*cnx).last_output_stream as *mut st_picoquic_stream_head_t;
                (*(*cnx).last_output_stream).next_output_stream =
                    stream as *mut st_picoquic_stream_head_t;
                (*cnx).last_output_stream = stream;
            }
        }
        (*stream).set_is_output_stream(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_remove_output_stream(
    mut cnx: *mut picoquic_cnx_t,
    mut stream: *mut picoquic_stream_head_t,
) {
    if (*stream).is_output_stream() != 0 {
        (*stream).set_is_output_stream(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if (*stream).previous_output_stream.is_null() {
            (*cnx).first_output_stream =
                (*stream).next_output_stream as *mut picoquic_stream_head_t;
        } else {
            (*(*stream).previous_output_stream).next_output_stream = (*stream).next_output_stream;
        }
        if (*stream).next_output_stream.is_null() {
            (*cnx).last_output_stream =
                (*stream).previous_output_stream as *mut picoquic_stream_head_t;
        } else {
            (*(*stream).next_output_stream).previous_output_stream =
                (*stream).previous_output_stream;
        }
        (*stream).previous_output_stream = ::core::ptr::null_mut::<st_picoquic_stream_head_t>();
        (*stream).next_output_stream = ::core::ptr::null_mut::<st_picoquic_stream_head_t>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_reorder_output_stream(
    mut cnx: *mut picoquic_cnx_t,
    mut stream: *mut picoquic_stream_head_t,
) {
    if (*stream).is_output_stream() != 0 {
        if !(*stream).previous_output_stream.is_null()
            && picoquic_compare_stream_priority(
                stream,
                (*stream).previous_output_stream as *mut picoquic_stream_head_t,
            ) < 0 as ::core::ffi::c_int
            || !(*stream).next_output_stream.is_null()
                && picoquic_compare_stream_priority(
                    stream,
                    (*stream).next_output_stream as *mut picoquic_stream_head_t,
                ) > 0 as ::core::ffi::c_int
        {
            picoquic_remove_output_stream(cnx, stream);
            (*stream).set_is_output_stream(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            picoquic_insert_output_stream(cnx, stream);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_next_stream(
    mut stream: *mut picoquic_stream_head_t,
) -> *mut picoquic_stream_head_t {
    return picosplay_next(stream as *mut picosplay_node_t) as *mut picoquic_stream_head_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_find_stream(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
) -> *mut picoquic_stream_head_t {
    let mut target: picoquic_stream_head_t = st_picoquic_stream_head_t {
        stream_node: st_picosplay_node_t {
            parent: ::core::ptr::null_mut::<st_picosplay_node_t>(),
            left: ::core::ptr::null_mut::<st_picosplay_node_t>(),
            right: ::core::ptr::null_mut::<st_picosplay_node_t>(),
        },
        next_output_stream: ::core::ptr::null_mut::<st_picoquic_stream_head_t>(),
        previous_output_stream: ::core::ptr::null_mut::<st_picoquic_stream_head_t>(),
        cnx: ::core::ptr::null_mut::<picoquic_cnx_t>(),
        stream_id: 0,
        affinity_path: ::core::ptr::null_mut::<st_picoquic_path_t>(),
        consumed_offset: 0,
        fin_offset: 0,
        maxdata_local: 0,
        maxdata_local_acked: 0,
        maxdata_remote: 0,
        local_error: 0,
        remote_error: 0,
        local_stop_error: 0,
        remote_stop_error: 0,
        last_time_data_sent: 0,
        stream_data_tree: st_picosplay_tree_t {
            root: ::core::ptr::null_mut::<picosplay_node_t>(),
            comp: None,
            create: None,
            delete_node: None,
            node_value: None,
            size: 0,
        },
        sent_offset: 0,
        send_queue: ::core::ptr::null_mut::<picoquic_stream_queue_node_t>(),
        app_stream_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        direct_receive_fn: None,
        direct_receive_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        sack_list: st_picoquic_sack_list_t {
            ack_tree: st_picosplay_tree_t {
                root: ::core::ptr::null_mut::<picosplay_node_t>(),
                comp: None,
                create: None,
                delete_node: None,
                node_value: None,
                size: 0,
            },
            ack_horizon: 0,
            horizon_delay: 0,
            rc: [st_picoquic_sack_range_count_t {
                range_counts: [0; 4],
            }; 2],
        },
        stream_priority: 0,
        is_active_fin_requested_fin_sent_fin_received_fin_signalled_reset_requested_reset_sent_reset_acked_reset_received_reset_signalled_stop_sending_requested_stop_sending_sent_stop_sending_received_stop_sending_signalled_max_stream_updated_stream_data_blocked_sent_is_output_stream_is_closed_is_discarded: [0; 3],
        c2rust_padding: [0; 4],
    };
    target.stream_id = stream_id;
    return picosplay_find(
        &raw mut (*cnx).stream_tree,
        &raw mut target as *mut ::core::ffi::c_void,
    ) as *mut picoquic_stream_head_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_add_output_streams(
    mut cnx: *mut picoquic_cnx_t,
    mut old_limit: uint64_t,
    mut new_limit: uint64_t,
    mut is_bidir: ::core::ffi::c_uint,
) {
    let mut old_rank: uint64_t = old_limit.wrapping_add(4 as uint64_t) >> 2 as ::core::ffi::c_int;
    let mut first_new_id: uint64_t = ((old_rank as ::core::ffi::c_ulonglong)
        .wrapping_add(1 as ::core::ffi::c_ulonglong)
        as uint64_t)
        .wrapping_sub(1 as ::core::ffi::c_int as uint64_t)
        << 2 as ::core::ffi::c_int
        | ((is_bidir == 0) as ::core::ffi::c_int as uint64_t) << 1 as ::core::ffi::c_int
        | ((*cnx).client_mode() as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as uint64_t;
    let mut stream: *mut picoquic_stream_head_t = picoquic_find_stream(cnx, first_new_id);
    while !stream.is_null() {
        if (*stream).stream_id > old_limit {
            if (*stream).stream_id > new_limit {
                break;
            }
            if (((*stream).stream_id ^ (*cnx).client_mode() as uint64_t) & 1 as uint64_t)
                as ::core::ffi::c_uint
                != 0
                && ((*stream).stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                    == is_bidir
            {
                picoquic_insert_output_stream(cnx, stream);
            }
        }
        stream = picoquic_next_stream(stream);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_stream(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
) -> *mut picoquic_stream_head_t {
    let mut stream: *mut picoquic_stream_head_t = malloc(::core::mem::size_of::<
        picoquic_stream_head_t,
    >() as size_t) as *mut picoquic_stream_head_t;
    if !stream.is_null() {
        memset(
            stream as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<picoquic_stream_head_t>() as size_t,
        );
        picoquic_sack_list_init(&raw mut (*stream).sack_list);
    }
    if !stream.is_null() {
        let mut is_output_stream: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        (*stream).stream_id = stream_id;
        (*stream).cnx = cnx as *mut picoquic_cnx_t;
        if ((stream_id ^ (*cnx).client_mode() as uint64_t) & 1 as uint64_t) as ::core::ffi::c_uint
            != 0
        {
            if (stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
                as ::core::ffi::c_uint
                != 0
            {
                (*stream).maxdata_local =
                    (*cnx).local_parameters.initial_max_stream_data_bidi_local;
                (*stream).maxdata_remote =
                    (*cnx).remote_parameters.initial_max_stream_data_bidi_remote;
                is_output_stream = ((*stream).stream_id <= (*cnx).max_stream_id_bidir_remote)
                    as ::core::ffi::c_int;
            } else {
                (*stream).maxdata_local = 0 as uint64_t;
                (*stream).maxdata_remote = (*cnx).remote_parameters.initial_max_stream_data_uni;
                is_output_stream = ((*stream).stream_id <= (*cnx).max_stream_id_unidir_remote)
                    as ::core::ffi::c_int;
            }
        } else if (stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
            as ::core::ffi::c_uint
            != 0
        {
            (*stream).maxdata_local = (*cnx).local_parameters.initial_max_stream_data_bidi_remote;
            (*stream).maxdata_remote = (*cnx).remote_parameters.initial_max_stream_data_bidi_local;
            is_output_stream = 1 as ::core::ffi::c_int;
        } else {
            (*stream).maxdata_local = (*cnx).local_parameters.initial_max_stream_data_uni;
            (*stream).maxdata_remote = 0 as uint64_t;
            is_output_stream = 0 as ::core::ffi::c_int;
        }
        (*stream).stream_priority = (*(*cnx).quic).default_stream_priority;
        picosplay_init_tree(
            &raw mut (*stream).stream_data_tree,
            Some(
                picoquic_stream_data_node_compare
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> int64_t,
            ),
            Some(
                picoquic_stream_data_node_create
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut picosplay_node_t,
            ),
            Some(
                picoquic_stream_data_node_delete
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut picosplay_node_t) -> (),
            ),
            Some(
                picoquic_stream_data_node_value
                    as unsafe extern "C" fn(*mut picosplay_node_t) -> *mut ::core::ffi::c_void,
            ),
        );
        picosplay_insert(
            &raw mut (*cnx).stream_tree,
            stream as *mut ::core::ffi::c_void,
        );
        if is_output_stream != 0 {
            picoquic_insert_output_stream(cnx, stream);
        } else {
            picoquic_remove_output_stream(cnx, stream);
            picoquic_delete_stream_if_closed(cnx, stream);
        }
        if stream_id >= (*cnx).next_stream_id[(stream_id & 3 as uint64_t) as usize] {
            (*cnx).next_stream_id[(stream_id & 3 as uint64_t) as usize] =
                stream_id.wrapping_add(4 as uint64_t);
        }
    }
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_stream(
    mut cnx: *mut picoquic_cnx_t,
    mut stream: *mut picoquic_stream_head_t,
) {
    picosplay_delete(
        &raw mut (*cnx).stream_tree,
        stream as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_mark_direct_receive_stream(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut direct_receive_fn: picoquic_stream_direct_receive_fn,
    mut direct_receive_ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream: *mut picoquic_stream_head_t = picoquic_find_stream(cnx, stream_id);
    let mut data: *mut picoquic_stream_data_node_t =
        ::core::ptr::null_mut::<picoquic_stream_data_node_t>();
    if stream.is_null() {
        ret = PICOQUIC_ERROR_INVALID_STREAM_ID;
    } else if (stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
        as ::core::ffi::c_uint
        == 0
        && ((stream_id ^ (*cnx).client_mode() as uint64_t) & 1 as uint64_t) as ::core::ffi::c_uint
            != 0
    {
        ret = PICOQUIC_ERROR_INVALID_STREAM_ID;
    } else if direct_receive_fn.is_none() {
        ret = PICOQUIC_ERROR_NO_CALLBACK_PROVIDED;
    } else {
        (*stream).direct_receive_fn = direct_receive_fn;
        (*stream).direct_receive_ctx = direct_receive_ctx;
        loop {
            data = picosplay_first(&raw mut (*stream).stream_data_tree)
                as *mut picoquic_stream_data_node_t;
            if data.is_null() {
                break;
            }
            let mut length: size_t = (*data).length;
            let mut offset: uint64_t = (*data).offset;
            if offset < (*stream).consumed_offset {
                if offset.wrapping_add(length as uint64_t) < (*stream).consumed_offset {
                    length = 0 as size_t;
                } else {
                    let mut delta_offset: size_t =
                        (*stream).consumed_offset.wrapping_sub(offset) as size_t;
                    length = length.wrapping_sub(delta_offset);
                    offset = (offset as ::core::ffi::c_ulong)
                        .wrapping_add(delta_offset as ::core::ffi::c_ulong)
                        as uint64_t as uint64_t;
                }
            }
            if length > 0 as size_t {
                ret = direct_receive_fn.expect("non-null function pointer")(
                    cnx as *mut picoquic_cnx_t,
                    stream_id,
                    0 as ::core::ffi::c_int,
                    (*data).bytes,
                    offset,
                    length,
                    direct_receive_ctx,
                );
            }
            if !(ret == 0 as ::core::ffi::c_int) {
                break;
            }
            picosplay_delete_hint(
                &raw mut (*stream).stream_data_tree,
                &raw mut (*data).stream_data_node,
            );
        }
        if ret == 0 as ::core::ffi::c_int
            && (*stream).fin_received() as ::core::ffi::c_int != 0
            && (*stream).fin_signalled() == 0
        {
            let mut fin_bytes: [uint8_t; 8] = [0; 8];
            ret = direct_receive_fn.expect("non-null function pointer")(
                cnx as *mut picoquic_cnx_t,
                stream_id,
                1 as ::core::ffi::c_int,
                &raw mut fin_bytes as *mut uint8_t,
                (*stream).fin_offset,
                0 as size_t,
                direct_receive_ctx,
            );
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_find_or_create_local_cnxid_list(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut do_create: ::core::ffi::c_int,
) -> *mut picoquic_local_cnxid_list_t {
    let mut local_cnxid_list: *mut picoquic_local_cnxid_list_t = (*cnx).first_local_cnxid_list;
    let mut p_previous: *mut *mut picoquic_local_cnxid_list_t =
        &raw mut (*cnx).first_local_cnxid_list;
    while !local_cnxid_list.is_null() {
        if (*local_cnxid_list).unique_path_id == unique_path_id {
            break;
        }
        p_previous =
            &raw mut (*local_cnxid_list).next_list as *mut *mut picoquic_local_cnxid_list_t;
        local_cnxid_list = (*local_cnxid_list).next_list as *mut picoquic_local_cnxid_list_t;
    }
    if local_cnxid_list.is_null() && do_create != 0 {
        local_cnxid_list = malloc(::core::mem::size_of::<picoquic_local_cnxid_list_t>() as size_t)
            as *mut picoquic_local_cnxid_list_t;
        if !local_cnxid_list.is_null() {
            memset(
                local_cnxid_list as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<picoquic_local_cnxid_list_t>() as size_t,
            );
            (*local_cnxid_list).unique_path_id = unique_path_id;
            *p_previous = local_cnxid_list;
            (*cnx).nb_local_cnxid_lists = (*cnx).nb_local_cnxid_lists.wrapping_add(1);
            if unique_path_id >= (*cnx).next_path_id_in_lists {
                (*cnx).next_path_id_in_lists = unique_path_id.wrapping_add(1 as uint64_t);
            }
        }
    }
    return local_cnxid_list;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_local_cnxid(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut suggested_value: *mut picoquic_connection_id_t,
    mut current_time: uint64_t,
) -> *mut picoquic_local_cnxid_t {
    let mut local_cnxid_list: *mut picoquic_local_cnxid_list_t =
        picoquic_find_or_create_local_cnxid_list(cnx, unique_path_id, 1 as ::core::ffi::c_int);
    let mut l_cid: *mut picoquic_local_cnxid_t = ::core::ptr::null_mut::<picoquic_local_cnxid_t>();
    let mut is_unique: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !local_cnxid_list.is_null() {
        l_cid = malloc(::core::mem::size_of::<picoquic_local_cnxid_t>() as size_t)
            as *mut picoquic_local_cnxid_t;
        if !l_cid.is_null() {
            memset(
                l_cid as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<picoquic_local_cnxid_t>() as size_t,
            );
            (*l_cid).create_time = current_time;
            if (*(*cnx).quic).local_cnxid_length as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                is_unique = 1 as ::core::ffi::c_int;
            } else {
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < 32 as ::core::ffi::c_int {
                    if i == 0 as ::core::ffi::c_int && !suggested_value.is_null() {
                        (*l_cid).cnx_id = *suggested_value;
                    } else {
                        picoquic_create_local_cnx_id(
                            (*cnx).quic,
                            &raw mut (*l_cid).cnx_id,
                            (*(*cnx).quic).local_cnxid_length,
                            (*cnx).initial_cnxid,
                        );
                    }
                    if picoquic_cnx_by_id(
                        (*cnx).quic,
                        (*l_cid).cnx_id,
                        ::core::ptr::null_mut::<*mut st_picoquic_local_cnxid_t>(),
                    )
                    .is_null()
                    {
                        is_unique = 1 as ::core::ffi::c_int;
                        break;
                    } else {
                        i += 1;
                    }
                }
            }
            if is_unique != 0 {
                let mut previous: *mut picoquic_local_cnxid_t =
                    ::core::ptr::null_mut::<picoquic_local_cnxid_t>();
                let mut next: *mut picoquic_local_cnxid_t = (*local_cnxid_list).local_cnxid_first;
                while !next.is_null() {
                    previous = next;
                    next = (*next).next as *mut picoquic_local_cnxid_t;
                }
                if previous.is_null() {
                    (*local_cnxid_list).local_cnxid_first = l_cid;
                } else {
                    (*previous).next = l_cid as *mut st_picoquic_local_cnxid_t;
                }
                let c2rust_fresh6 = (*local_cnxid_list).local_cnxid_sequence_next;
                (*local_cnxid_list).local_cnxid_sequence_next = (*local_cnxid_list)
                    .local_cnxid_sequence_next
                    .wrapping_add(1);
                (*l_cid).sequence = c2rust_fresh6;
                (*l_cid).path_id = unique_path_id;
                (*local_cnxid_list).nb_local_cnxid += 1;
                if (*(*cnx).quic).local_cnxid_length as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                {
                    picoquic_register_cnx_id((*cnx).quic, cnx, l_cid);
                }
                if (*l_cid).sequence == 0 as uint64_t {
                    (*local_cnxid_list).local_cnxid_oldest_created = current_time;
                }
            } else {
                free(l_cid as *mut ::core::ffi::c_void);
                l_cid = ::core::ptr::null_mut::<picoquic_local_cnxid_t>();
            }
        }
    }
    return l_cid;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_local_cnxid_listed(
    mut cnx: *mut picoquic_cnx_t,
    mut local_cnxid_list: *mut picoquic_local_cnxid_list_t,
    mut l_cid: *mut picoquic_local_cnxid_t,
) {
    let mut previous: *mut picoquic_local_cnxid_t =
        ::core::ptr::null_mut::<picoquic_local_cnxid_t>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*cnx).nb_paths {
        if (**(*cnx).path.offset(i as isize)).p_local_cnxid == l_cid {
            let ref mut c2rust_fresh0 = (**(*cnx).path.offset(i as isize)).p_local_cnxid;
            *c2rust_fresh0 = ::core::ptr::null_mut::<picoquic_local_cnxid_t>();
            let ref mut c2rust_fresh1 = **(*cnx).path.offset(i as isize);
            (*c2rust_fresh1)
                .set_was_local_cnxid_retired(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        i += 1;
    }
    if (*l_cid).cnx_id.id_len as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        if !(*l_cid).registered_cnx.is_null() {
            let mut item: *mut picohash_item = &raw mut (*l_cid).hash_item;
            picohash_delete_item(
                (*(*cnx).quic).table_cnx_by_id,
                item,
                0 as ::core::ffi::c_int,
            );
        }
        (*l_cid).registered_cnx = ::core::ptr::null_mut::<picoquic_cnx_t>();
    }
    if !local_cnxid_list.is_null() {
        let mut next: *mut picoquic_local_cnxid_t = (*local_cnxid_list).local_cnxid_first;
        while !next.is_null() {
            if next == l_cid {
                if previous.is_null() {
                    (*local_cnxid_list).local_cnxid_first =
                        (*next).next as *mut picoquic_local_cnxid_t;
                } else {
                    (*previous).next = (*next).next;
                }
                (*local_cnxid_list).nb_local_cnxid -= 1;
                break;
            } else {
                previous = next;
                next = (*next).next as *mut picoquic_local_cnxid_t;
            }
        }
        if (*l_cid).sequence < (*local_cnxid_list).local_cnxid_retire_before
            && (*local_cnxid_list).nb_local_cnxid_expired > 0 as ::core::ffi::c_int
        {
            (*local_cnxid_list).nb_local_cnxid_expired -= 1;
        }
    }
    free(l_cid as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_local_cnxid(
    mut cnx: *mut picoquic_cnx_t,
    mut l_cid: *mut picoquic_local_cnxid_t,
) {
    let mut local_cnxid_list: *mut picoquic_local_cnxid_list_t =
        picoquic_find_or_create_local_cnxid_list(cnx, (*l_cid).path_id, 0 as ::core::ffi::c_int);
    picoquic_delete_local_cnxid_listed(cnx, local_cnxid_list, l_cid);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_local_cnxid_list(
    mut cnx: *mut picoquic_cnx_t,
    mut local_cnxid_list: *mut picoquic_local_cnxid_list_t,
) {
    while !(*local_cnxid_list).local_cnxid_first.is_null() {
        picoquic_delete_local_cnxid_listed(
            cnx,
            local_cnxid_list,
            (*local_cnxid_list).local_cnxid_first,
        );
    }
    if local_cnxid_list == (*cnx).first_local_cnxid_list {
        (*cnx).first_local_cnxid_list =
            (*local_cnxid_list).next_list as *mut picoquic_local_cnxid_list_t;
    } else {
        let mut previous: *mut picoquic_local_cnxid_list_t = (*cnx).first_local_cnxid_list;
        while !previous.is_null() {
            if (*previous).next_list == local_cnxid_list {
                (*previous).next_list = (*local_cnxid_list).next_list;
            }
            previous = (*previous).next_list as *mut picoquic_local_cnxid_list_t;
        }
    }
    free(local_cnxid_list as *mut ::core::ffi::c_void);
    (*cnx).nb_local_cnxid_lists = (*cnx).nb_local_cnxid_lists.wrapping_sub(1);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_local_cnxid_lists(mut cnx: *mut picoquic_cnx_t) {
    while !(*cnx).first_local_cnxid_list.is_null() {
        picoquic_delete_local_cnxid_list(cnx, (*cnx).first_local_cnxid_list);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_retire_local_cnxid(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut sequence: uint64_t,
) {
    let mut local_cnxid_list: *mut picoquic_local_cnxid_list_t =
        picoquic_find_or_create_local_cnxid_list(cnx, unique_path_id, 0 as ::core::ffi::c_int);
    if !local_cnxid_list.is_null() {
        let mut local_cnxid: *mut picoquic_local_cnxid_t = (*local_cnxid_list).local_cnxid_first;
        while !local_cnxid.is_null() {
            if (*local_cnxid).sequence == sequence {
                break;
            }
            local_cnxid = (*local_cnxid).next as *mut picoquic_local_cnxid_t;
        }
        if !local_cnxid.is_null() {
            picoquic_delete_local_cnxid_listed(cnx, local_cnxid_list, local_cnxid);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_check_local_cnxid_ttl(
    mut cnx: *mut picoquic_cnx_t,
    mut local_cnxid_list: *mut picoquic_local_cnxid_list_t,
    mut current_time: uint64_t,
    mut next_wake_time: *mut uint64_t,
) {
    if current_time.wrapping_sub((*local_cnxid_list).local_cnxid_oldest_created)
        >= (*(*cnx).quic).local_cnxid_ttl
    {
        let mut l_cid: *mut picoquic_local_cnxid_t = (*local_cnxid_list).local_cnxid_first;
        (*local_cnxid_list).local_cnxid_oldest_created = current_time;
        (*local_cnxid_list).nb_local_cnxid_expired = 0 as ::core::ffi::c_int;
        while !l_cid.is_null() {
            if current_time.wrapping_sub((*l_cid).create_time) >= (*(*cnx).quic).local_cnxid_ttl {
                (*local_cnxid_list).nb_local_cnxid_expired += 1;
                if (*l_cid).sequence >= (*local_cnxid_list).local_cnxid_retire_before {
                    (*local_cnxid_list).local_cnxid_retire_before =
                        (*l_cid).sequence.wrapping_add(1 as uint64_t);
                }
            } else if (*l_cid).create_time < (*local_cnxid_list).local_cnxid_oldest_created {
                (*local_cnxid_list).local_cnxid_oldest_created = (*l_cid).create_time;
            }
            l_cid = (*l_cid).next as *mut picoquic_local_cnxid_t;
        }
        (*cnx).next_wake_time = current_time;
        (*(*cnx).quic).wake_file = 3 as ::core::ffi::c_int;
        (*(*cnx).quic).wake_line = 3637 as ::core::ffi::c_int;
    } else if (*next_wake_time).wrapping_sub((*local_cnxid_list).local_cnxid_oldest_created)
        > (*(*cnx).quic).local_cnxid_ttl
    {
        *next_wake_time = (*local_cnxid_list)
            .local_cnxid_oldest_created
            .wrapping_add((*(*cnx).quic).local_cnxid_ttl);
        (*(*cnx).quic).wake_file = 3 as ::core::ffi::c_int;
        (*(*cnx).quic).wake_line = 3641 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_find_local_cnxid(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut cnxid: *mut picoquic_connection_id_t,
) -> *mut picoquic_local_cnxid_t {
    let mut local_cnxid: *mut picoquic_local_cnxid_t =
        ::core::ptr::null_mut::<picoquic_local_cnxid_t>();
    let mut local_cnxid_list: *mut picoquic_local_cnxid_list_t =
        picoquic_find_or_create_local_cnxid_list(cnx, unique_path_id, 0 as ::core::ffi::c_int);
    if !local_cnxid_list.is_null() && {
        local_cnxid = (*local_cnxid_list).local_cnxid_first;
        !local_cnxid.is_null()
    } {
        while !local_cnxid.is_null() {
            if picoquic_compare_connection_id(&raw mut (*local_cnxid).cnx_id, cnxid)
                == 0 as ::core::ffi::c_int
            {
                break;
            }
            local_cnxid = (*local_cnxid).next as *mut picoquic_local_cnxid_t;
        }
    }
    return local_cnxid;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_cnx(
    mut quic: *mut picoquic_quic_t,
    mut initial_cnx_id: picoquic_connection_id_t,
    mut remote_cnx_id: picoquic_connection_id_t,
    mut addr_to: *const sockaddr,
    mut start_time: uint64_t,
    mut preferred_version: uint32_t,
    mut sni: *const ::core::ffi::c_char,
    mut alpn: *const ::core::ffi::c_char,
    mut client_mode: ::core::ffi::c_char,
) -> *mut picoquic_cnx_t {
    let mut cnx: *mut picoquic_cnx_t =
        malloc(::core::mem::size_of::<picoquic_cnx_t>() as size_t) as *mut picoquic_cnx_t;
    if !cnx.is_null() {
        let mut ret: ::core::ffi::c_int = 0;
        let mut cnxid0: *mut picoquic_local_cnxid_t =
            ::core::ptr::null_mut::<picoquic_local_cnxid_t>();
        memset(
            cnx as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<picoquic_cnx_t>() as size_t,
        );
        (*cnx).start_time = start_time;
        (*cnx).phase_delay = INT64_MAX as int64_t;
        (*cnx).set_client_mode(client_mode as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if client_mode != 0 {
            if picoquic_is_connection_id_null(&raw mut initial_cnx_id) != 0 {
                picoquic_create_random_cnx_id(quic, &raw mut initial_cnx_id, 8 as uint8_t);
            }
        }
        (*cnx).initial_cnxid = initial_cnx_id;
        (*cnx).quic = quic;
        (*cnx).pmtud_policy = (*quic).default_pmtud_policy;
        cnxid0 = picoquic_create_local_cnxid(
            cnx,
            0 as uint64_t,
            ::core::ptr::null_mut::<picoquic_connection_id_t>(),
            start_time,
        );
        (*cnx).set_are_path_callbacks_enabled(
            (*quic).are_path_callbacks_enabled() as ::core::ffi::c_uint
        );
        (*cnx).rtt_update_delta = (*quic).rtt_update_delta;
        (*cnx).pacing_rate_update_delta = (*quic).pacing_rate_update_delta;
        picoquic_queue_data_repeat_init(cnx);
        ret = picoquic_create_path(
            cnx,
            start_time,
            ::core::ptr::null::<sockaddr>(),
            addr_to,
            0 as uint64_t,
        );
        if ret == 0 as ::core::ffi::c_int {
            ret = picoquic_init_cnxid_stash(cnx);
        }
        if ret != 0 as ::core::ffi::c_int || cnxid0.is_null() {
            picoquic_delete_cnx(cnx);
            cnx = ::core::ptr::null_mut::<picoquic_cnx_t>();
        } else {
            (*cnx).next_wake_time = start_time;
            (*quic).wake_file = 3 as ::core::ffi::c_int;
            (*quic).wake_line = 3715 as ::core::ffi::c_int;
            picoquic_insert_cnx_in_list(quic, cnx);
            picoquic_insert_cnx_by_wake_time(quic, cnx);
            let ref mut c2rust_fresh4 =
                (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_local_cnxid;
            *c2rust_fresh4 = cnxid0;
            let ref mut c2rust_fresh5 = **(*cnx).path.offset(0 as ::core::ffi::c_int as isize);
            (*c2rust_fresh5)
                .set_challenge_verified(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*cnx).datagram_priority = (*(*cnx).quic).default_datagram_priority as uint64_t;
            (*cnx).high_priority_stream_id = UINT64_MAX as uint64_t;
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < 4 as ::core::ffi::c_int {
                (*cnx).next_stream_id[i as usize] = i as uint64_t;
                i += 1;
            }
            picoquic_pacing_init(&raw mut (*cnx).priority_bypass_pacing, start_time);
            picoquic_register_path(cnx, *(*cnx).path.offset(0 as ::core::ffi::c_int as isize));
        }
    }
    if !cnx.is_null() {
        memcpy(
            &raw mut (*cnx).local_parameters as *mut ::core::ffi::c_void,
            &raw mut (*quic).default_tp as *const ::core::ffi::c_void,
            ::core::mem::size_of::<picoquic_tp_t>() as size_t,
        );
        if (*cnx).local_parameters.prefered_address.is_defined != 0 {
            let mut unique_path_id: uint64_t =
                (if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                }) as uint64_t;
            let mut cnxid1: *mut picoquic_local_cnxid_t = picoquic_create_local_cnxid(
                cnx,
                unique_path_id,
                ::core::ptr::null_mut::<picoquic_connection_id_t>(),
                start_time,
            );
            if !cnxid1.is_null() {
                (*cnx).local_parameters.prefered_address.connection_id = (*cnxid1).cnx_id;
                picoquic_create_cnxid_reset_secret(
                    (*cnx).quic,
                    &raw mut (*cnxid1).cnx_id,
                    &raw mut (*cnx).local_parameters.prefered_address.statelessResetToken
                        as *mut uint8_t,
                );
            }
        }
        if (*cnx).local_parameters.max_packet_size == 0 as uint32_t
            && (*(*cnx).quic).mtu_max > 0 as uint32_t
        {
            (*cnx).local_parameters.max_packet_size = (*(*cnx).quic).mtu_max.wrapping_sub(
                (if (*addr_to).sa_family as ::core::ffi::c_int == AF_INET6 {
                    48 as ::core::ffi::c_int
                } else {
                    28 as ::core::ffi::c_int
                }) as uint32_t,
            );
        }
        if (*cnx).client_mode() == 0
            && (*quic).local_cnxid_length as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        {
            (*cnx).local_parameters.migration_disabled = 1 as ::core::ffi::c_uint;
        }
        if (*quic).default_send_receive_bdp_frame() != 0 {
            (*cnx).local_parameters.enable_bdp_frame = 1 as ::core::ffi::c_int;
        }
        (*cnx).maxdata_local = (*cnx).local_parameters.initial_max_data;
        (*cnx).max_stream_id_bidir_local = (*cnx)
            .local_parameters
            .initial_max_stream_id_bidir
            .wrapping_sub(1 as ::core::ffi::c_int as uint64_t)
            << 2 as ::core::ffi::c_int
            | (0 as ::core::ffi::c_int as uint64_t) << 1 as ::core::ffi::c_int
            | ((*cnx).client_mode() as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as uint64_t;
        (*cnx).max_stream_id_bidir_local_computed =
            (*cnx).max_stream_id_bidir_local & 3 as uint64_t;
        (*cnx).max_stream_id_unidir_local = (*cnx)
            .local_parameters
            .initial_max_stream_id_unidir
            .wrapping_sub(1 as ::core::ffi::c_int as uint64_t)
            << 2 as ::core::ffi::c_int
            | (1 as ::core::ffi::c_int as uint64_t) << 1 as ::core::ffi::c_int
            | ((*cnx).client_mode() as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as uint64_t;
        (*cnx).max_stream_id_unidir_local_computed =
            (*cnx).max_stream_id_unidir_local & 3 as uint64_t;
        (*cnx).padding_multiple = (*quic).padding_multiple_default;
        (*cnx).padding_minsize = (*quic).padding_minsize_default;
        (*cnx).spin_policy = (*quic).default_spin_policy;
        if (*cnx).spin_policy as ::core::ffi::c_uint
            == picoquic_spinbit_basic as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut rand256: uint8_t = picoquic_public_random_64() as uint8_t;
            if (rand256 as ::core::ffi::c_int) < PICOQUIC_SPIN_RESERVE_MOD_256 {
                (*cnx).spin_policy = picoquic_spinbit_null;
            }
        } else if (*cnx).spin_policy as ::core::ffi::c_uint
            == picoquic_spinbit_on as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*cnx).spin_policy = picoquic_spinbit_basic;
        }
        if !sni.is_null() {
            (*cnx).sni = picoquic_string_duplicate(sni);
        }
        if !alpn.is_null() {
            (*cnx).alpn = picoquic_string_duplicate(alpn);
        }
        (*cnx).callback_fn = (*quic).default_callback_fn;
        (*cnx).callback_ctx = (*quic).default_callback_ctx;
        (*cnx).congestion_alg = (*quic).default_congestion_alg;
        (*cnx).set_is_preemptive_repeat_enabled(
            (*quic).is_preemptive_repeat_enabled() as ::core::ffi::c_uint
        );
        (*cnx).crypto_epoch_length_max = (*quic).crypto_epoch_length_max;
        let mut epoch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while epoch < PICOQUIC_NUMBER_OF_EPOCHS {
            (*cnx).tls_stream[epoch as usize].send_queue =
                ::core::ptr::null_mut::<picoquic_stream_queue_node_t>();
            epoch += 1;
        }
        if (*cnx).client_mode() != 0 {
            if preferred_version == 0 as uint32_t {
                (*cnx).proposed_version =
                    picoquic_supported_versions[0 as ::core::ffi::c_int as usize].version;
                (*cnx).version_index = 0 as ::core::ffi::c_int;
            } else {
                (*cnx).version_index = picoquic_get_version_index(preferred_version);
                if (*cnx).version_index < 0 as ::core::ffi::c_int {
                    (*cnx).version_index = PICOQUIC_INTEROP_VERSION_INDEX;
                    if preferred_version & 0xa0a0a0a as uint32_t == 0xa0a0a0a as uint32_t {
                        (*cnx).proposed_version = preferred_version;
                    } else {
                        (*cnx).proposed_version = picoquic_supported_versions
                            [PICOQUIC_INTEROP_VERSION_INDEX as usize]
                            .version;
                    }
                } else {
                    (*cnx).proposed_version = preferred_version;
                }
            }
            (*cnx).cnx_state = picoquic_state_client_init;
            if (*quic).is_cert_store_not_empty() == 0 {
                picoquic_log_app_message(
                    cnx as *mut picoquic_cnx_t,
                    b"No root crt list specified -- certificate will not be verified.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
                picoquic_set_null_verifier(quic);
            }
        } else {
            (*cnx).set_is_half_open(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*(*cnx).quic).current_number_half_open = (*(*cnx).quic)
                .current_number_half_open
                .wrapping_add(1 as uint32_t);
            if (*(*cnx).quic).current_number_half_open > (*(*cnx).quic).max_half_open_before_retry {
                (*(*cnx).quic).set_check_token(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            (*cnx).cnx_state = picoquic_state_server_init;
            (*cnx).initial_cnxid = initial_cnx_id;
            (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid).cnx_id =
                remote_cnx_id;
            (*cnx).version_index = picoquic_get_version_index(preferred_version);
            if (*cnx).version_index < 0 as ::core::ffi::c_int {
                (*cnx).version_index = 0 as ::core::ffi::c_int;
                (*cnx).proposed_version =
                    picoquic_supported_versions[0 as ::core::ffi::c_int as usize].version;
            } else {
                (*cnx).proposed_version = preferred_version;
            }
        }
        let mut pc: picoquic_packet_context_enum = picoquic_packet_context_application;
        while (pc as ::core::ffi::c_uint)
            < picoquic_nb_packet_context as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            picoquic_init_ack_ctx(
                cnx,
                (&raw mut (*cnx).ack_ctx as *mut picoquic_ack_context_t).offset(pc as isize)
                    as *mut picoquic_ack_context_t,
            );
            picoquic_init_packet_ctx(
                cnx,
                (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t).offset(pc as isize)
                    as *mut picoquic_packet_context_t,
                pc,
            );
            pc += 1;
        }
        (*cnx).set_ack_ignore_order_local(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*cnx).set_ack_ignore_order_remote(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*cnx).latest_progress_time = start_time;
        (*cnx).latest_receive_time = start_time;
        let mut epoch_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while epoch_0 < PICOQUIC_NUMBER_OF_EPOCHS {
            (*cnx).tls_stream[epoch_0 as usize].stream_id = 0 as uint64_t;
            (*cnx).tls_stream[epoch_0 as usize].consumed_offset = 0 as uint64_t;
            (*cnx).tls_stream[epoch_0 as usize].fin_offset = 0 as uint64_t;
            (*cnx).tls_stream[epoch_0 as usize].stream_node.left =
                ::core::ptr::null_mut::<st_picosplay_node_t>();
            (*cnx).tls_stream[epoch_0 as usize].stream_node.parent =
                ::core::ptr::null_mut::<st_picosplay_node_t>();
            (*cnx).tls_stream[epoch_0 as usize].stream_node.right =
                ::core::ptr::null_mut::<st_picosplay_node_t>();
            (*cnx).tls_stream[epoch_0 as usize].sent_offset = 0 as uint64_t;
            (*cnx).tls_stream[epoch_0 as usize].local_error = 0 as uint64_t;
            (*cnx).tls_stream[epoch_0 as usize].remote_error = 0 as uint64_t;
            (*cnx).tls_stream[epoch_0 as usize].maxdata_local = UINT64_MAX as uint64_t;
            (*cnx).tls_stream[epoch_0 as usize].maxdata_remote = UINT64_MAX as uint64_t;
            picosplay_init_tree(
                &raw mut (*(&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t)
                    .offset(epoch_0 as isize))
                .stream_data_tree,
                Some(
                    picoquic_stream_data_node_compare
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut ::core::ffi::c_void,
                        ) -> int64_t,
                ),
                Some(
                    picoquic_stream_data_node_create
                        as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut picosplay_node_t,
                ),
                Some(
                    picoquic_stream_data_node_delete
                        as unsafe extern "C" fn(
                            *mut ::core::ffi::c_void,
                            *mut picosplay_node_t,
                        ) -> (),
                ),
                Some(
                    picoquic_stream_data_node_value
                        as unsafe extern "C" fn(*mut picosplay_node_t) -> *mut ::core::ffi::c_void,
                ),
            );
            picoquic_sack_list_init(
                &raw mut (*(&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t)
                    .offset(epoch_0 as isize))
                .sack_list,
            );
            epoch_0 += 1;
        }
        (*cnx).ack_frequency_sequence_local = UINT64_MAX as uint64_t;
        (*cnx).ack_gap_local = 10 as uint64_t;
        (*cnx).ack_frequency_delay_local = PICOQUIC_ACK_DELAY_MAX_DEFAULT as uint64_t;
        (*cnx).ack_frequency_sequence_remote = UINT64_MAX as uint64_t;
        (*cnx).ack_gap_remote = 2 as uint64_t;
        (*cnx).ack_delay_remote = PICOQUIC_ACK_DELAY_MIN as uint64_t;
        (*cnx).max_ack_delay_remote = (*cnx).ack_delay_remote;
        (*cnx).max_ack_gap_remote = (*cnx).ack_gap_remote;
        (*cnx).max_ack_delay_local = (*cnx).ack_frequency_delay_local;
        (*cnx).max_ack_gap_local = (*cnx).ack_gap_local;
        (*cnx).min_ack_delay_remote = (*cnx).ack_delay_remote;
        (*cnx).min_ack_delay_local = (*cnx).ack_frequency_delay_local;
        picosplay_init_tree(
            &raw mut (*cnx).stream_tree,
            Some(
                picoquic_stream_node_compare
                    as unsafe extern "C" fn(
                        *mut ::core::ffi::c_void,
                        *mut ::core::ffi::c_void,
                    ) -> int64_t,
            ),
            Some(
                picoquic_stream_node_create
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut picosplay_node_t,
            ),
            Some(
                picoquic_stream_node_delete
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut picosplay_node_t) -> (),
            ),
            Some(
                picoquic_stream_node_value
                    as unsafe extern "C" fn(*mut picosplay_node_t) -> *mut ::core::ffi::c_void,
            ),
        );
        (*cnx).congestion_alg = (*(*cnx).quic).default_congestion_alg;
        if !(*cnx).congestion_alg.is_null() {
            (*(*cnx).congestion_alg)
                .alg_init
                .expect("non-null function pointer")(
                cnx as *mut picoquic_cnx_t,
                *(*cnx).path.offset(0 as ::core::ffi::c_int as isize) as *mut picoquic_path_t,
                start_time,
            );
        }
    }
    if !cnx.is_null()
        && picoquic_tlscontext_create(quic, cnx, start_time) != 0 as ::core::ffi::c_int
    {
        picoquic_delete_cnx(cnx);
        cnx = ::core::ptr::null_mut::<picoquic_cnx_t>();
    }
    if !cnx.is_null() {
        if picoquic_setup_initial_traffic_keys(cnx) != 0 {
            picoquic_delete_cnx(cnx);
            cnx = ::core::ptr::null_mut::<picoquic_cnx_t>();
        }
    }
    if !cnx.is_null()
        && client_mode == 0
        && (*quic).local_cnxid_length as ::core::ffi::c_int > 0 as ::core::ffi::c_int
    {
        if picoquic_register_net_icid(cnx) != 0 as ::core::ffi::c_int {
            picoquic_delete_cnx(cnx);
            cnx = ::core::ptr::null_mut::<picoquic_cnx_t>();
        }
    }
    if (*quic).use_unique_log_names() != 0 {
        picoquic_crypto_random(
            quic,
            &raw mut (*cnx).log_unique as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<uint16_t>() as size_t,
        );
    }
    if !cnx.is_null() && (*cnx).client_mode() == 0 {
        picoquic_log_new_connection(cnx);
    }
    return cnx as *mut picoquic_cnx_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_client_cnx(
    mut quic: *mut picoquic_quic_t,
    mut addr: *mut sockaddr,
    mut start_time: uint64_t,
    mut preferred_version: uint32_t,
    mut sni: *const ::core::ffi::c_char,
    mut alpn: *const ::core::ffi::c_char,
    mut callback_fn: picoquic_stream_data_cb_fn,
    mut callback_ctx: *mut ::core::ffi::c_void,
) -> *mut picoquic_cnx_t {
    let mut cnx: *mut picoquic_cnx_t = picoquic_create_cnx(
        quic,
        picoquic_null_connection_id,
        picoquic_null_connection_id,
        addr,
        start_time,
        preferred_version,
        sni,
        alpn,
        1 as ::core::ffi::c_char,
    ) as *mut picoquic_cnx_t;
    if !cnx.is_null() {
        let mut ret: ::core::ffi::c_int = 0;
        if callback_fn.is_some() {
            (*cnx).callback_fn = callback_fn;
        }
        if !callback_ctx.is_null() {
            (*cnx).callback_ctx = callback_ctx;
        }
        ret = picoquic_start_client_cnx(cnx);
        if ret != 0 as ::core::ffi::c_int {
            picoquic_delete_cnx(cnx);
            cnx = ::core::ptr::null_mut::<picoquic_cnx_t>();
        }
    }
    return cnx as *mut picoquic_cnx_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_start_client_cnx(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*cnx).cnx_state as ::core::ffi::c_uint
        != picoquic_state_client_init as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cnx).tls_stream[0 as ::core::ffi::c_int as usize].sent_offset > 0 as uint64_t
        || !(*cnx).tls_stream[0 as ::core::ffi::c_int as usize]
            .send_queue
            .is_null()
    {
        return -(1 as ::core::ffi::c_int);
    }
    picoquic_log_new_connection(cnx);
    ret = picoquic_initialize_tls_stream(cnx, picoquic_get_quic_time((*cnx).quic));
    (*cnx).maxdata_remote = (*cnx).remote_parameters.initial_max_data;
    (*cnx).max_stream_id_bidir_remote = (*cnx)
        .remote_parameters
        .initial_max_stream_id_bidir
        .wrapping_sub(1 as ::core::ffi::c_int as uint64_t)
        << 2 as ::core::ffi::c_int
        | (0 as ::core::ffi::c_int as uint64_t) << 1 as ::core::ffi::c_int
        | ((*cnx).client_mode() as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as uint64_t;
    (*cnx).max_stream_id_unidir_remote = (*cnx)
        .remote_parameters
        .initial_max_stream_id_unidir
        .wrapping_sub(1 as ::core::ffi::c_int as uint64_t)
        << 2 as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int as uint64_t) << 1 as ::core::ffi::c_int
        | ((*cnx).client_mode() as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as uint64_t;
    (*cnx).max_stream_data_remote = (*cnx).remote_parameters.initial_max_data;
    (*cnx).max_stream_data_local = (*cnx).local_parameters.initial_max_stream_data_bidi_local;
    picoquic_reinsert_by_wake_time((*cnx).quic, cnx, picoquic_get_quic_time((*cnx).quic));
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_transport_parameters(
    mut cnx: *mut picoquic_cnx_t,
    mut tp: *const picoquic_tp_t,
) {
    (*cnx).local_parameters = *tp;
    if (*(*cnx).quic).mtu_max > 0 as uint32_t
        && (*cnx).local_parameters.max_packet_size == 0 as uint32_t
    {
        (*cnx).local_parameters.max_packet_size = (*(*cnx).quic).mtu_max.wrapping_sub(
            (if (*(&raw mut (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).peer_addr
                as *mut sockaddr))
                .sa_family as ::core::ffi::c_int
                == AF_INET6
            {
                48 as ::core::ffi::c_int
            } else {
                28 as ::core::ffi::c_int
            }) as uint32_t,
        );
    }
    (*cnx).maxdata_local = (*cnx).local_parameters.initial_max_data;
    (*cnx).max_stream_id_bidir_local = (*cnx)
        .local_parameters
        .initial_max_stream_id_bidir
        .wrapping_sub(1 as ::core::ffi::c_int as uint64_t)
        << 2 as ::core::ffi::c_int
        | (0 as ::core::ffi::c_int as uint64_t) << 1 as ::core::ffi::c_int
        | ((*cnx).client_mode() as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as uint64_t;
    (*cnx).max_stream_id_unidir_local = (*cnx)
        .local_parameters
        .initial_max_stream_id_unidir
        .wrapping_sub(1 as ::core::ffi::c_int as uint64_t)
        << 2 as ::core::ffi::c_int
        | (1 as ::core::ffi::c_int as uint64_t) << 1 as ::core::ffi::c_int
        | ((*cnx).client_mode() as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_transport_parameters(
    mut cnx: *mut picoquic_cnx_t,
    mut get_local: ::core::ffi::c_int,
) -> *const picoquic_tp_t {
    return if get_local != 0 {
        &raw mut (*cnx).local_parameters
    } else {
        &raw mut (*cnx).remote_parameters
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_peer_addr(
    mut cnx: *mut picoquic_cnx_t,
    mut addr: *mut *mut sockaddr,
) {
    *addr = &raw mut (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).peer_addr
        as *mut sockaddr;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_local_addr(
    mut cnx: *mut picoquic_cnx_t,
    mut addr: *mut *mut sockaddr,
) {
    *addr = &raw mut (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).local_addr
        as *mut sockaddr;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_local_if_index(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_ulong {
    return (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).if_index_dest;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_local_cnxid(
    mut cnx: *mut picoquic_cnx_t,
) -> picoquic_connection_id_t {
    return (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_local_cnxid).cnx_id;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_remote_cnxid(
    mut cnx: *mut picoquic_cnx_t,
) -> picoquic_connection_id_t {
    return (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid).cnx_id;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_initial_cnxid(
    mut cnx: *mut picoquic_cnx_t,
) -> picoquic_connection_id_t {
    return (*cnx).initial_cnxid;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_client_cnxid(
    mut cnx: *mut picoquic_cnx_t,
) -> picoquic_connection_id_t {
    return if (*cnx).client_mode() as ::core::ffi::c_int != 0 {
        (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_local_cnxid).cnx_id
    } else {
        (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid).cnx_id
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_server_cnxid(
    mut cnx: *mut picoquic_cnx_t,
) -> picoquic_connection_id_t {
    return if (*cnx).client_mode() as ::core::ffi::c_int != 0 {
        (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid).cnx_id
    } else {
        (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_local_cnxid).cnx_id
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_logging_cnxid(
    mut cnx: *mut picoquic_cnx_t,
) -> picoquic_connection_id_t {
    return (*cnx).initial_cnxid;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_cnx_start_time(mut cnx: *mut picoquic_cnx_t) -> uint64_t {
    return (*cnx).start_time;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_cnx_state(
    mut cnx: *mut picoquic_cnx_t,
) -> picoquic_state_enum {
    return (*cnx).cnx_state;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_0rtt_available(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    return if (*cnx).crypto_context[picoquic_epoch_0rtt as ::core::ffi::c_int as usize]
        .aead_encrypt
        .is_null()
    {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cnx_set_padding_policy(
    mut cnx: *mut picoquic_cnx_t,
    mut padding_multiple: uint32_t,
    mut padding_minsize: uint32_t,
) {
    (*cnx).padding_multiple = padding_multiple;
    (*cnx).padding_minsize = padding_minsize;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cnx_get_padding_policy(
    mut cnx: *mut picoquic_cnx_t,
    mut padding_multiple: *mut uint32_t,
    mut padding_minsize: *mut uint32_t,
) {
    *padding_multiple = (*cnx).padding_multiple;
    *padding_minsize = (*cnx).padding_minsize;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cnx_set_spinbit_policy(
    mut cnx: *mut picoquic_cnx_t,
    mut spinbit_policy: picoquic_spinbit_version_enum,
) {
    (*cnx).spin_policy = spinbit_policy;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_seed_bandwidth(
    mut cnx: *mut picoquic_cnx_t,
    mut rtt_min: uint64_t,
    mut cwin: uint64_t,
    mut ip_addr: *const uint8_t,
    mut ip_addr_length: uint8_t,
) {
    (*cnx).seed_rtt_min = rtt_min;
    (*cnx).seed_cwin = cwin;
    if ip_addr_length as ::core::ffi::c_int > PICOQUIC_STORED_IP_MAX {
        ip_addr_length = PICOQUIC_STORED_IP_MAX as uint8_t;
    }
    memcpy(
        &raw mut (*cnx).seed_ip_addr as *mut uint8_t as *mut ::core::ffi::c_void,
        ip_addr as *const ::core::ffi::c_void,
        ip_addr_length as size_t,
    );
    (*cnx).seed_ip_addr_length = ip_addr_length;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_pmtud_policy(
    mut quic: *mut picoquic_quic_t,
    mut pmtud_policy: picoquic_pmtud_policy_enum,
) {
    (*quic).default_pmtud_policy = pmtud_policy;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cnx_set_pmtud_policy(
    mut cnx: *mut picoquic_cnx_t,
    mut pmtud_policy: picoquic_pmtud_policy_enum,
) {
    (*cnx).pmtud_policy = pmtud_policy;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cnx_set_pmtud_required(
    mut cnx: *mut picoquic_cnx_t,
    mut is_pmtud_required: ::core::ffi::c_int,
) {
    (*cnx).pmtud_policy = (if is_pmtud_required != 0 {
        picoquic_pmtud_required as ::core::ffi::c_int
    } else {
        picoquic_pmtud_basic as ::core::ffi::c_int
    }) as picoquic_pmtud_policy_enum;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_current_time() -> uint64_t {
    let mut now: uint64_t = 0;
    let mut currentTime: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    clock_gettime(CLOCK_MONOTONIC, &raw mut currentTime);
    now = (currentTime.tv_sec as ::core::ffi::c_ulonglong)
        .wrapping_mul(1000000 as ::core::ffi::c_ulonglong)
        .wrapping_add(
            (currentTime.tv_nsec as ::core::ffi::c_ulonglong)
                .wrapping_div(1000 as ::core::ffi::c_ulonglong),
        ) as uint64_t;
    return now;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_quic_time(mut quic: *mut picoquic_quic_t) -> uint64_t {
    let mut now: uint64_t = 0;
    if (*quic).p_simulated_time.is_null() {
        now = picoquic_current_time();
    } else {
        now = *(*quic).p_simulated_time;
    }
    return now;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_fuzz(
    mut quic: *mut picoquic_quic_t,
    mut fuzz_fn: picoquic_fuzz_fn,
    mut fuzz_ctx: *mut ::core::ffi::c_void,
) {
    (*quic).fuzz_fn = fuzz_fn;
    (*quic).fuzz_ctx = fuzz_ctx;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_log_level(
    mut quic: *mut picoquic_quic_t,
    mut log_level: ::core::ffi::c_int,
) {
    (*quic).set_use_long_log(
        (if log_level > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_use_unique_log_names(
    mut quic: *mut picoquic_quic_t,
    mut use_unique_log_names: ::core::ffi::c_int,
) {
    (*quic).set_use_unique_log_names(
        use_unique_log_names as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_enable_sslkeylog(
    mut quic: *mut picoquic_quic_t,
    mut enable_sslkeylog: ::core::ffi::c_int,
) {
    (*quic).set_enable_sslkeylog(
        (enable_sslkeylog != 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint
            as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_sslkeylog_enabled(
    mut quic: *mut picoquic_quic_t,
) -> ::core::ffi::c_int {
    return (*quic).enable_sslkeylog() as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_random_initial(
    mut quic: *mut picoquic_quic_t,
    mut random_initial: ::core::ffi::c_int,
) {
    (*quic).set_random_initial(
        (if random_initial > 1 as ::core::ffi::c_int {
            2 as ::core::ffi::c_int
        } else if random_initial > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_packet_train_mode(
    mut quic: *mut picoquic_quic_t,
    mut train_mode: ::core::ffi::c_int,
) {
    (*quic).set_packet_train_mode(
        (if train_mode > 0 as ::core::ffi::c_int {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_padding_policy(
    mut quic: *mut picoquic_quic_t,
    mut padding_min_size: uint32_t,
    mut padding_multiple: uint32_t,
) {
    (*quic).padding_minsize_default = padding_min_size;
    (*quic).padding_multiple_default = padding_multiple;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_connection_id_length(
    mut quic: *mut picoquic_quic_t,
    mut cid_length: uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if cid_length as ::core::ffi::c_int != (*quic).local_cnxid_length as ::core::ffi::c_int {
        if cid_length as ::core::ffi::c_int > PICOQUIC_CONNECTION_ID_MAX_SIZE {
            ret = PICOQUIC_ERROR_CNXID_CHECK;
        } else if !(*quic).cnx_list.is_null() {
            ret = PICOQUIC_ERROR_CANNOT_CHANGE_ACTIVE_CONTEXT;
        } else {
            (*quic).local_cnxid_length = cid_length;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_connection_id_ttl(
    mut quic: *mut picoquic_quic_t,
    mut ttl_usec: uint64_t,
) {
    (*quic).local_cnxid_ttl = ttl_usec;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_default_connection_id_ttl(
    mut quic: *mut picoquic_quic_t,
) -> uint64_t {
    return (*quic).local_cnxid_ttl;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_mtu_max(
    mut quic: *mut picoquic_quic_t,
    mut mtu_max: uint32_t,
) {
    (*quic).mtu_max = mtu_max;
    (*quic).default_tp.max_packet_size = mtu_max;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_initial_send_mtu(
    mut quic: *mut picoquic_quic_t,
    mut intitial_mtu_ipv4: uint32_t,
    mut intitial_mtu_ipv6: uint32_t,
) {
    (*quic).initial_send_mtu_ipv4 = intitial_mtu_ipv4;
    (*quic).initial_send_mtu_ipv6 = intitial_mtu_ipv6;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_alpn_select_fn(
    mut quic: *mut picoquic_quic_t,
    mut alpn_select_fn: picoquic_alpn_select_fn,
) {
    if !(*quic).default_alpn.is_null() {
        free((*quic).default_alpn as *mut ::core::ffi::c_void);
        (*quic).default_alpn = ::core::ptr::null::<::core::ffi::c_char>();
    }
    (*quic).alpn_select_fn = alpn_select_fn;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_callback(
    mut quic: *mut picoquic_quic_t,
    mut callback_fn: picoquic_stream_data_cb_fn,
    mut callback_ctx: *mut ::core::ffi::c_void,
) {
    (*quic).default_callback_fn = callback_fn;
    (*quic).default_callback_ctx = callback_ctx;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_stateless_reset_min_interval(
    mut quic: *mut picoquic_quic_t,
    mut min_interval_usec: uint64_t,
) {
    (*quic).stateless_reset_next_time = picoquic_get_quic_time(quic);
    (*quic).stateless_reset_min_interval = min_interval_usec;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_callback(
    mut cnx: *mut picoquic_cnx_t,
    mut callback_fn: picoquic_stream_data_cb_fn,
    mut callback_ctx: *mut ::core::ffi::c_void,
) {
    (*cnx).callback_fn = callback_fn;
    (*cnx).callback_ctx = callback_ctx;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_default_callback_function(
    mut quic: *mut picoquic_quic_t,
) -> picoquic_stream_data_cb_fn {
    return (*quic).default_callback_fn;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_default_callback_context(
    mut quic: *mut picoquic_quic_t,
) -> *mut ::core::ffi::c_void {
    return (*quic).default_callback_ctx;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_callback_function(
    mut cnx: *mut picoquic_cnx_t,
) -> picoquic_stream_data_cb_fn {
    return (*cnx).callback_fn;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_callback_context(
    mut cnx: *mut picoquic_cnx_t,
) -> *mut ::core::ffi::c_void {
    return (*cnx).callback_ctx;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_misc_frame(
    mut bytes: *const uint8_t,
    mut length: size_t,
    mut is_pure_ack: ::core::ffi::c_int,
    mut pc: picoquic_packet_context_enum,
) -> *mut picoquic_misc_frame_header_t {
    let mut l_alloc: size_t =
        (::core::mem::size_of::<picoquic_misc_frame_header_t>() as size_t).wrapping_add(length);
    if l_alloc < ::core::mem::size_of::<picoquic_misc_frame_header_t>() as usize {
        return ::core::ptr::null_mut::<picoquic_misc_frame_header_t>();
    } else {
        let mut head: *mut picoquic_misc_frame_header_t =
            malloc(l_alloc) as *mut picoquic_misc_frame_header_t;
        if !head.is_null() {
            memset(
                head as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<picoquic_misc_frame_header_t>() as size_t,
            );
            (*head).length = length;
            (*head).is_pure_ack = is_pure_ack;
            (*head).pc = pc;
            memcpy(
                (head as *mut uint8_t).offset(
                    ::core::mem::size_of::<picoquic_misc_frame_header_t>() as usize as isize,
                ) as *mut ::core::ffi::c_void,
                bytes as *const ::core::ffi::c_void,
                length,
            );
        }
        return head;
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_misc_or_dg_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut first: *mut *mut picoquic_misc_frame_header_t,
    mut last: *mut *mut picoquic_misc_frame_header_t,
    mut bytes: *const uint8_t,
    mut length: size_t,
    mut is_pure_ack: ::core::ffi::c_int,
    mut pc: picoquic_packet_context_enum,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut misc_frame: *mut picoquic_misc_frame_header_t =
        picoquic_create_misc_frame(bytes, length, is_pure_ack, pc);
    if misc_frame.is_null() {
        ret = PICOQUIC_ERROR_MEMORY;
    } else if (*last).is_null() {
        *first = misc_frame;
        *last = misc_frame;
    } else {
        (**last).next_misc_frame = misc_frame as *mut st_picoquic_misc_frame_header_t;
        (*misc_frame).previous_misc_frame = *last as *mut st_picoquic_misc_frame_header_t;
        *last = misc_frame;
    }
    picoquic_reinsert_by_wake_time((*cnx).quic, cnx, picoquic_get_quic_time((*cnx).quic));
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_misc_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut length: size_t,
    mut is_pure_ack: ::core::ffi::c_int,
    mut pc: picoquic_packet_context_enum,
) -> ::core::ffi::c_int {
    return picoquic_queue_misc_or_dg_frame(
        cnx,
        &raw mut (*cnx).first_misc_frame,
        &raw mut (*cnx).last_misc_frame,
        bytes,
        length,
        is_pure_ack,
        pc,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_purge_misc_frames_after_ready(mut cnx: *mut picoquic_cnx_t) {
    let mut misc_frame: *mut picoquic_misc_frame_header_t = (*cnx).first_misc_frame;
    while !misc_frame.is_null() {
        let mut next_frame: *mut picoquic_misc_frame_header_t =
            (*misc_frame).next_misc_frame as *mut picoquic_misc_frame_header_t;
        if (*misc_frame).pc as ::core::ffi::c_uint
            != picoquic_packet_context_application as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            picoquic_delete_misc_or_dg(
                &raw mut (*cnx).first_misc_frame,
                &raw mut (*cnx).last_misc_frame,
                misc_frame,
            );
        }
        misc_frame = next_frame;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_misc_or_dg(
    mut first: *mut *mut picoquic_misc_frame_header_t,
    mut last: *mut *mut picoquic_misc_frame_header_t,
    mut frame: *mut picoquic_misc_frame_header_t,
) {
    if !(*frame).next_misc_frame.is_null() {
        (*(*frame).next_misc_frame).previous_misc_frame = (*frame).previous_misc_frame;
    } else {
        *last = (*frame).previous_misc_frame as *mut picoquic_misc_frame_header_t;
    }
    if !(*frame).previous_misc_frame.is_null() {
        (*(*frame).previous_misc_frame).next_misc_frame = (*frame).next_misc_frame;
    } else {
        *first = (*frame).next_misc_frame as *mut picoquic_misc_frame_header_t;
    }
    free(frame as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_clear_ack_ctx(mut ack_ctx: *mut picoquic_ack_context_t) {
    picoquic_sack_list_free(&raw mut (*ack_ctx).sack_list);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_reset_ack_context(mut ack_ctx: *mut picoquic_ack_context_t) {
    picoquic_clear_ack_ctx(ack_ctx);
    picoquic_sack_list_init(&raw mut (*ack_ctx).sack_list);
    (*ack_ctx).ecn_ect0_total_local = 0 as uint64_t;
    (*ack_ctx).ecn_ect1_total_local = 0 as uint64_t;
    (*ack_ctx).ecn_ce_total_local = 0 as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_reset_packet_context(
    mut cnx: *mut picoquic_cnx_t,
    mut pkt_ctx: *mut picoquic_packet_context_t,
) {
    while !(*pkt_ctx).pending_last.is_null() {
        picoquic_dequeue_retransmit_packet(
            cnx,
            pkt_ctx,
            (*pkt_ctx).pending_last,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
    }
    while !(*pkt_ctx).retransmitted_newest.is_null() {
        picoquic_dequeue_retransmitted_packet(cnx, pkt_ctx, (*pkt_ctx).retransmitted_newest);
    }
    (*pkt_ctx).retransmitted_oldest = ::core::ptr::null_mut::<picoquic_packet_t>();
    (*pkt_ctx).ecn_ect0_total_remote = 0 as uint64_t;
    (*pkt_ctx).ecn_ect1_total_remote = 0 as uint64_t;
    (*pkt_ctx).ecn_ce_total_remote = 0 as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_reset_cnx(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pc: picoquic_packet_context_enum = picoquic_packet_context_application;
    while (pc as ::core::ffi::c_uint)
        < picoquic_nb_packet_context as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if pc as ::core::ffi::c_uint
            != picoquic_packet_context_application as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            picoquic_reset_packet_context(
                cnx,
                (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t).offset(pc as isize)
                    as *mut picoquic_packet_context_t,
            );
            picoquic_reset_ack_context(
                (&raw mut (*cnx).ack_ctx as *mut picoquic_ack_context_t).offset(pc as isize)
                    as *mut picoquic_ack_context_t,
            );
        }
        pc += 1;
    }
    let mut epoch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while epoch < PICOQUIC_NUMBER_OF_EPOCHS {
        picoquic_clear_stream(
            (&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t).offset(epoch as isize)
                as *mut picoquic_stream_head_t,
        );
        (*cnx).tls_stream[epoch as usize].consumed_offset = 0 as uint64_t;
        (*cnx).tls_stream[epoch as usize].fin_offset = 0 as uint64_t;
        (*cnx).tls_stream[epoch as usize].sent_offset = 0 as uint64_t;
        epoch += 1;
    }
    let mut k: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while k < 4 as ::core::ffi::c_int {
        picoquic_crypto_context_free(
            (&raw mut (*cnx).crypto_context as *mut picoquic_crypto_context_t).offset(k as isize)
                as *mut picoquic_crypto_context_t,
        );
        k += 1;
    }
    picoquic_crypto_context_free(&raw mut (*cnx).crypto_context_new);
    ret = picoquic_setup_initial_traffic_keys(cnx);
    if !(*cnx).tls_ctx.is_null() {
        picoquic_tlscontext_free((*cnx).tls_ctx);
        (*cnx).tls_ctx = NULL;
    }
    picoquic_log_new_connection(cnx);
    if ret == 0 as ::core::ffi::c_int {
        ret = picoquic_tlscontext_create((*cnx).quic, cnx, current_time);
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = picoquic_initialize_tls_stream(cnx, current_time);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_connection_error_ex(
    mut cnx: *mut picoquic_cnx_t,
    mut local_error: uint64_t,
    mut frame_type: uint64_t,
    mut local_reason: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if local_error > PICOQUIC_ERROR_CLASS as uint64_t {
        local_error = PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t;
    }
    if (*cnx).cnx_state as ::core::ffi::c_uint
        == picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_client_ready_start as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_server_false_start as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cnx).local_error = local_error;
        (*cnx).local_error_reason = local_reason;
        (*cnx).cnx_state = picoquic_state_disconnecting;
    } else if ((*cnx).cnx_state as ::core::ffi::c_uint)
        < picoquic_state_server_false_start as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*cnx).cnx_state as ::core::ffi::c_uint
            != picoquic_state_handshake_failure as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*cnx).cnx_state as ::core::ffi::c_uint
                != picoquic_state_handshake_failure_resend as ::core::ffi::c_int
                    as ::core::ffi::c_uint
        {
            (*cnx).local_error = local_error;
            (*cnx).local_error_reason = local_reason;
            (*cnx).cnx_state = picoquic_state_handshake_failure;
        }
    }
    (*cnx).offending_frame_type = frame_type;
    picoquic_log_app_message(
        cnx as *mut picoquic_cnx_t,
        b"Protocol error 0x%x, frame %lu, reason: %s\0".as_ptr() as *const ::core::ffi::c_char,
        local_error,
        frame_type,
        if local_reason.is_null() {
            b"?\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            local_reason
        },
    );
    return PICOQUIC_ERROR_DETECTED;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_connection_error(
    mut cnx: *mut picoquic_cnx_t,
    mut local_error: uint64_t,
    mut frame_type: uint64_t,
) -> ::core::ffi::c_int {
    return picoquic_connection_error_ex(
        cnx,
        local_error,
        frame_type,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_connection_disconnect(mut cnx: *mut picoquic_cnx_t) {
    if (*cnx).cnx_state as ::core::ffi::c_uint
        != picoquic_state_disconnected as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cnx).cnx_state = picoquic_state_disconnected;
        if (*cnx).callback_fn.is_some() {
            (*cnx).callback_fn.expect("non-null function pointer")(
                cnx as *mut picoquic_cnx_t,
                0 as uint64_t,
                ::core::ptr::null_mut::<uint8_t>(),
                0 as size_t,
                picoquic_callback_close,
                (*cnx).callback_ctx,
                NULL,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_start_key_rotation(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*cnx).cnx_state as ::core::ffi::c_uint
        != picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cnx).crypto_epoch_sequence
            > picoquic_sack_list_last(
                &raw mut (*(&raw mut (*cnx).ack_ctx as *mut picoquic_ack_context_t)
                    .offset(picoquic_packet_context_application as ::core::ffi::c_int as isize))
                .sack_list,
            )
    {
        ret = PICOQUIC_ERROR_KEY_ROTATION_NOT_READY;
    } else {
        ret = picoquic_compute_new_rotated_keys(cnx);
    }
    if ret == 0 as ::core::ffi::c_int {
        picoquic_apply_rotated_keys(cnx, 1 as ::core::ffi::c_int);
        picoquic_crypto_context_free(&raw mut (*cnx).crypto_context_old);
        (*cnx).crypto_epoch_sequence = (*cnx).pkt_ctx
            [picoquic_packet_context_application as ::core::ffi::c_int as usize]
            .send_sequence;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_sooner_packets(mut cnx: *mut picoquic_cnx_t) {
    let mut packet: *mut picoquic_stateless_packet_t = (*cnx).first_sooner;
    while !packet.is_null() {
        let mut next_packet: *mut picoquic_stateless_packet_t =
            (*packet).next_packet as *mut picoquic_stateless_packet_t;
        picoquic_delete_stateless_packet(packet);
        packet = next_packet;
    }
    (*cnx).first_sooner = ::core::ptr::null_mut::<picoquic_stateless_packet_t>();
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_cnx(mut cnx: *mut picoquic_cnx_t) {
    if !cnx.is_null() {
        if (*(*cnx).quic).perflog_fn.is_some() {
            (*(*cnx).quic)
                .perflog_fn
                .expect("non-null function pointer")(
                (*cnx).quic as *mut picoquic_quic_t,
                cnx as *mut picoquic_cnx_t,
                0 as ::core::ffi::c_int,
            );
        }
        picoquic_log_close_connection(cnx);
        if (*cnx).is_half_open() as ::core::ffi::c_int != 0
            && (*(*cnx).quic).current_number_half_open > 0 as uint32_t
        {
            (*(*cnx).quic).current_number_half_open =
                (*(*cnx).quic).current_number_half_open.wrapping_sub(1);
            (*cnx).set_is_half_open(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        if ((*cnx).cnx_state as ::core::ffi::c_uint)
            < picoquic_state_disconnected as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            picoquic_connection_disconnect(cnx);
        }
        if !(*cnx).alpn.is_null() {
            free((*cnx).alpn as *mut ::core::ffi::c_void);
            (*cnx).alpn = ::core::ptr::null::<::core::ffi::c_char>();
        }
        if !(*cnx).sni.is_null() {
            free((*cnx).sni as *mut ::core::ffi::c_void);
            (*cnx).sni = ::core::ptr::null::<::core::ffi::c_char>();
        }
        if !(*cnx).retry_token.is_null() {
            free((*cnx).retry_token as *mut ::core::ffi::c_void);
            (*cnx).retry_token = ::core::ptr::null_mut::<uint8_t>();
        }
        picoquic_delete_sooner_packets(cnx);
        picoquic_remove_cnx_from_list(cnx);
        picoquic_remove_cnx_from_wake_list(cnx);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < PICOQUIC_NUMBER_OF_EPOCHS {
            picoquic_crypto_context_free(
                (&raw mut (*cnx).crypto_context as *mut picoquic_crypto_context_t)
                    .offset(i as isize) as *mut picoquic_crypto_context_t,
            );
            i += 1;
        }
        picoquic_crypto_context_free(&raw mut (*cnx).crypto_context_new);
        picoquic_crypto_context_free(&raw mut (*cnx).crypto_context_old);
        let mut pc: picoquic_packet_context_enum = picoquic_packet_context_application;
        while (pc as ::core::ffi::c_uint)
            < picoquic_nb_packet_context as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            picoquic_reset_packet_context(
                cnx,
                (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t).offset(pc as isize)
                    as *mut picoquic_packet_context_t,
            );
            picoquic_reset_ack_context(
                (&raw mut (*cnx).ack_ctx as *mut picoquic_ack_context_t).offset(pc as isize)
                    as *mut picoquic_ack_context_t,
            );
            pc += 1;
        }
        while !(*cnx).first_misc_frame.is_null() {
            picoquic_delete_misc_or_dg(
                &raw mut (*cnx).first_misc_frame,
                &raw mut (*cnx).last_misc_frame,
                (*cnx).first_misc_frame,
            );
        }
        while !(*cnx).first_datagram.is_null() {
            picoquic_delete_misc_or_dg(
                &raw mut (*cnx).first_datagram,
                &raw mut (*cnx).last_datagram,
                (*cnx).first_datagram,
            );
        }
        picosplay_empty_tree(&raw mut (*cnx).queue_data_repeat_tree);
        let mut epoch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while epoch < PICOQUIC_NUMBER_OF_EPOCHS {
            picoquic_clear_stream(
                (&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t).offset(epoch as isize)
                    as *mut picoquic_stream_head_t,
            );
            epoch += 1;
        }
        picosplay_empty_tree(&raw mut (*cnx).stream_tree);
        if !(*cnx).tls_ctx.is_null() {
            picoquic_tlscontext_free((*cnx).tls_ctx);
            (*cnx).tls_ctx = NULL;
        }
        if !(*cnx).path.is_null() {
            while (*cnx).nb_paths > 0 as ::core::ffi::c_int {
                picoquic_dereference_stashed_cnxid(
                    cnx,
                    *(*cnx)
                        .path
                        .offset(((*cnx).nb_paths - 1 as ::core::ffi::c_int) as isize),
                    1 as ::core::ffi::c_int,
                );
                picoquic_delete_path(cnx, (*cnx).nb_paths - 1 as ::core::ffi::c_int);
            }
            free((*cnx).path as *mut ::core::ffi::c_void);
            (*cnx).path = ::core::ptr::null_mut::<*mut picoquic_path_t>();
        }
        picoquic_delete_local_cnxid_lists(cnx);
        picoquic_delete_remote_cnxid_stashes(cnx);
        picoquic_unregister_net_icid(cnx);
        free(cnx as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_handshake_error(
    mut error_code: uint64_t,
) -> ::core::ffi::c_int {
    return (error_code & 0xff00 as uint64_t
        == (0x100 as ::core::ffi::c_int as uint16_t as ::core::ffi::c_int
            | (0 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint16_t
                as ::core::ffi::c_int) as uint64_t
        || error_code == PICOQUIC_TLS_HANDSHAKE_FAILED as uint64_t)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_close_reasons(
    mut cnx: *mut picoquic_cnx_t,
    mut local_reason: *mut uint64_t,
    mut remote_reason: *mut uint64_t,
    mut local_application_reason: *mut uint64_t,
    mut remote_application_reason: *mut uint64_t,
) {
    *local_reason = (*cnx).local_error;
    *remote_reason = (*cnx).remote_error;
    *local_application_reason = (*cnx).application_error;
    *remote_application_reason = (*cnx).remote_application_error;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_app_wake_time(
    mut cnx: *mut picoquic_cnx_t,
    mut app_wake_time: uint64_t,
) {
    (*cnx).app_wake_time = app_wake_time;
    if (*cnx).app_wake_time != 0 as uint64_t && (*cnx).app_wake_time < (*cnx).next_wake_time {
        picoquic_reinsert_by_wake_time((*cnx).quic, cnx, app_wake_time);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_desired_version(
    mut cnx: *mut picoquic_cnx_t,
    mut desired_version: uint32_t,
) {
    (*cnx).desired_version = desired_version;
    (*cnx).set_do_version_negotiation(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_rejected_version(
    mut cnx: *mut picoquic_cnx_t,
    mut rejected_version: uint32_t,
) {
    (*cnx).desired_version = rejected_version;
    (*cnx).set_do_version_negotiation(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cnx_by_id_(
    mut quic: *mut picoquic_quic_t,
    mut cnx_id: picoquic_connection_id_t,
) -> *mut picoquic_cnx_t {
    return picoquic_cnx_by_id(
        quic,
        cnx_id,
        ::core::ptr::null_mut::<*mut st_picoquic_local_cnxid_t>(),
    ) as *mut picoquic_cnx_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cnx_by_id(
    mut quic: *mut picoquic_quic_t,
    mut cnx_id: picoquic_connection_id_t,
    mut l_cid: *mut *mut st_picoquic_local_cnxid_t,
) -> *mut picoquic_cnx_t {
    let mut ret: *mut picoquic_cnx_t = ::core::ptr::null_mut::<picoquic_cnx_t>();
    let mut item: *mut picohash_item = ::core::ptr::null_mut::<picohash_item>();
    let mut key: picoquic_local_cnxid_t = st_picoquic_local_cnxid_t {
        next: ::core::ptr::null_mut::<st_picoquic_local_cnxid_t>(),
        registered_cnx: ::core::ptr::null_mut::<picoquic_cnx_t>(),
        hash_item: _picohash_item {
            hash: 0,
            next_in_bin: ::core::ptr::null_mut::<_picohash_item>(),
            key: ::core::ptr::null::<::core::ffi::c_void>(),
        },
        path_id: 0,
        sequence: 0,
        create_time: 0,
        cnx_id: st_picoquic_connection_id_t {
            id: [0; 20],
            id_len: 0,
        },
        is_acked: 0,
    };
    memset(
        &raw mut key as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<picoquic_local_cnxid_t>() as size_t,
    );
    key.cnx_id = cnx_id;
    item = picohash_retrieve(
        (*quic).table_cnx_by_id,
        &raw mut key as *const ::core::ffi::c_void,
    );
    if !item.is_null() {
        ret = (*((*item).key as *mut picoquic_local_cnxid_t)).registered_cnx as *mut picoquic_cnx_t;
        if !l_cid.is_null() {
            *l_cid = (*item).key as *mut picoquic_local_cnxid_t as *mut st_picoquic_local_cnxid_t;
        }
    } else if !l_cid.is_null() {
        *l_cid = ::core::ptr::null_mut::<st_picoquic_local_cnxid_t>();
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cnx_by_net(
    mut quic: *mut picoquic_quic_t,
    mut addr: *const sockaddr,
) -> *mut picoquic_cnx_t {
    let mut ret: *mut picoquic_cnx_t = ::core::ptr::null_mut::<picoquic_cnx_t>();
    let mut item: *mut picohash_item = ::core::ptr::null_mut::<picohash_item>();
    let mut dummy_path_x: picoquic_path_t = {
        let mut init = st_picoquic_path_t {
            observed_addr_acked: [0; 1],
            c2rust_padding: [0; 3],
            mtu_probe_sent_path_is_published_challenge_required_challenge_verified_challenge_failed_response_required_nat_challenge_required_path_is_standby_path_is_demoted_path_abandon_received_path_abandon_sent_current_spin_last_bw_estimate_path_limited_path_cid_rotated_path_is_preferred_path_is_nat_challenge_is_cc_data_updated_is_multipath_probe_needed_was_local_cnxid_retired_is_ssthresh_initialized_is_token_published_is_ticket_seeded_is_bdp_sent_is_nominal_ack_path_is_ack_lost_is_ack_expected_is_datagram_ready_is_pto_required_is_probing_nat_is_lost_feedback_notified_is_cca_probing_up_rtt_is_initialized: [0; 4],
            c2rust_padding_0: [0; 4],
            p_local_cnxid: ::core::ptr::null_mut::<picoquic_local_cnxid_t>(),
            p_remote_cnxid: ::core::ptr::null_mut::<picoquic_remote_cnxid_t>(),
            registered_peer_addr: sockaddr_storage {
                ss_family: 0,
                __ss_padding: [0; 118],
                __ss_align: 0,
            },
            net_id_hash_item: _picohash_item {
                hash: 0,
                next_in_bin: ::core::ptr::null_mut::<_picohash_item>(),
                key: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            cnx: ::core::ptr::null_mut::<st_picoquic_cnx_t>(),
            unique_path_id: 0,
            app_path_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            ack_ctx: st_picoquic_ack_context_t {
                sack_list: st_picoquic_sack_list_t {
                    ack_tree: st_picosplay_tree_t {
                        root: ::core::ptr::null_mut::<picosplay_node_t>(),
                        comp: None,
                        create: None,
                        delete_node: None,
                        node_value: None,
                        size: 0,
                    },
                    ack_horizon: 0,
                    horizon_delay: 0,
                    rc: [st_picoquic_sack_range_count_t {
                        range_counts: [0; 4],
                    }; 2],
                },
                time_stamp_largest_received: 0,
                act: [st_picoquic_ack_context_track_t {
                    highest_ack_sent: 0,
                    highest_ack_sent_time: 0,
                    time_oldest_unack_packet_received: 0,
                    ack_needed_ack_after_fin_out_of_order_received_is_immediate_ack_required: [0; 1],
                    c2rust_padding: [0; 7],
                }; 2],
                crypto_rotation_sequence: 0,
                ecn_ect0_total_local: 0,
                ecn_ect1_total_local: 0,
                ecn_ce_total_local: 0,
                sending_ecn_ack: [0; 1],
                c2rust_padding: [0; 7],
            },
            pkt_ctx: st_picoquic_packet_context_t {
                send_sequence: 0,
                next_sequence_hole: 0,
                retransmit_sequence: 0,
                highest_acknowledged: 0,
                latest_time_acknowledged: 0,
                highest_acknowledged_time: 0,
                pending_last: ::core::ptr::null_mut::<picoquic_packet_t>(),
                pending_first: ::core::ptr::null_mut::<picoquic_packet_t>(),
                retransmitted_newest: ::core::ptr::null_mut::<picoquic_packet_t>(),
                retransmitted_oldest: ::core::ptr::null_mut::<picoquic_packet_t>(),
                preemptive_repeat_ptr: ::core::ptr::null_mut::<picoquic_packet_t>(),
                retransmitted_queue_size: 0,
                ecn_ect0_total_remote: 0,
                ecn_ect1_total_remote: 0,
                ecn_ce_total_remote: 0,
                ack_of_ack_requested: [0; 1],
                c2rust_padding: [0; 7],
            },
            peer_addr: sockaddr_storage {
                ss_family: 0,
                __ss_padding: [0; 118],
                __ss_align: 0,
            },
            local_addr: sockaddr_storage {
                ss_family: 0,
                __ss_padding: [0; 118],
                __ss_align: 0,
            },
            if_index_dest: 0,
            observed_addr: sockaddr_storage {
                ss_family: 0,
                __ss_padding: [0; 118],
                __ss_align: 0,
            },
            observed_address_received: 0,
            nb_observed_repeat: 0,
            observed_sequence_sent: 0,
            observed_time: 0,
            last_non_path_probing_pn: 0,
            challenge_response: 0,
            challenge: [0; 3],
            challenge_time: 0,
            demotion_time: 0,
            challenge_time_first: 0,
            challenge_repeat_count: 0,
            nat_challenge: [0; 3],
            nat_challenge_time: 0,
            nat_challenge_repeat_count: 0,
            p_remote_nat_cnxid: ::core::ptr::null_mut::<picoquic_remote_cnxid_t>(),
            if_index_nat_dest: 0,
            nat_peer_addr: sockaddr_storage {
                ss_family: 0,
                __ss_padding: [0; 118],
                __ss_align: 0,
            },
            nat_local_addr: sockaddr_storage {
                ss_family: 0,
                __ss_padding: [0; 118],
                __ss_align: 0,
            },
            last_sent_time: 0,
            status_sequence_to_receive_next: 0,
            status_sequence_sent_last: 0,
            last_packet_received_at: 0,
            last_loss_event_detected: 0,
            nb_retransmit: 0,
            total_bytes_lost: 0,
            nb_losses_found: 0,
            nb_timer_losses: 0,
            nb_spurious: 0,
            nb_losses_reported: 0,
            q_square: 0,
            max_ack_delay: 0,
            rtt_sample: 0,
            one_way_delay_sample: 0,
            smoothed_rtt: 0,
            rtt_variant: 0,
            retransmit_timer: 0,
            rtt_min: 0,
            rtt_max: 0,
            max_spurious_rtt: 0,
            max_reorder_delay: 0,
            max_reorder_gap: 0,
            latest_sent_time: 0,
            rtt_packet_previous_period: 0,
            rtt_time_previous_period: 0,
            nb_rtt_estimate_in_period: 0,
            sum_rtt_estimate_in_period: 0,
            max_rtt_estimate_in_period: 0,
            min_rtt_estimate_in_period: 0,
            send_mtu: 0,
            send_mtu_max_tried: 0,
            delivered: 0,
            delivered_last: 0,
            delivered_time_last: 0,
            delivered_sent_last: 0,
            delivered_limited_index: 0,
            delivered_last_packet: 0,
            bandwidth_estimate: 0,
            bandwidth_estimate_max: 0,
            max_sample_acked_time: 0,
            max_sample_sent_time: 0,
            max_sample_delivered: 0,
            peak_bandwidth_estimate: 0,
            bytes_sent: 0,
            received: 0,
            receive_rate_epoch: 0,
            received_prior: 0,
            receive_rate_estimate: 0,
            receive_rate_max: 0,
            cwin: 0,
            bytes_in_transit: 0,
            last_sender_limited_time: 0,
            last_cwin_blocked_time: 0,
            last_time_acked_data_frame_sent: 0,
            congestion_alg_state: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            pacing: st_picoquic_pacing_t {
                rate: 0,
                evaluation_time: 0,
                bucket_max: 0,
                packet_time_microsec: 0,
                quantum_max: 0,
                rate_max: 0,
                bandwidth_pause: 0,
                bucket_nanosec: 0,
                packet_time_nanosec: 0,
            },
            nb_mtu_losses: 0,
            lost_after_delivered: 0,
            responder: 0,
            challenger: 0,
            polled: 0,
            paced: 0,
            congested: 0,
            selected: 0,
            nb_delay_outliers: 0,
            rtt_update_delta: 0,
            pacing_rate_update_delta: 0,
            rtt_threshold_low: 0,
            rtt_threshold_high: 0,
            pacing_rate_threshold_low: 0,
            pacing_rate_threshold_high: 0,
            receive_rate_threshold_low: 0,
            receive_rate_threshold_high: 0,
            rtt_min_remote: 0,
            cwin_remote: 0,
            ip_client_remote: [0; 16],
            ip_client_remote_length: 0,
        };
        init.set_observed_addr_acked(0);
        init.set_mtu_probe_sent(0);
        init.set_path_is_published(0);
        init.set_challenge_required(0);
        init.set_challenge_verified(0);
        init.set_challenge_failed(0);
        init.set_response_required(0);
        init.set_nat_challenge_required(0);
        init.set_path_is_standby(0);
        init.set_path_is_demoted(0);
        init.set_path_abandon_received(0);
        init.set_path_abandon_sent(0);
        init.set_current_spin(0);
        init.set_last_bw_estimate_path_limited(0);
        init.set_path_cid_rotated(0);
        init.set_path_is_preferred_path(0);
        init.set_is_nat_challenge(0);
        init.set_is_cc_data_updated(0);
        init.set_is_multipath_probe_needed(0);
        init.set_was_local_cnxid_retired(0);
        init.set_is_ssthresh_initialized(0);
        init.set_is_token_published(0);
        init.set_is_ticket_seeded(0);
        init.set_is_bdp_sent(0);
        init.set_is_nominal_ack_path(0);
        init.set_is_ack_lost(0);
        init.set_is_ack_expected(0);
        init.set_is_datagram_ready(0);
        init.set_is_pto_required(0);
        init.set_is_probing_nat(0);
        init.set_is_lost_feedback_notified(0);
        init.set_is_cca_probing_up(0);
        init.set_rtt_is_initialized(0);
        init
    };
    picoquic_store_addr(&raw mut dummy_path_x.registered_peer_addr, addr);
    item = picohash_retrieve(
        (*quic).table_cnx_by_net,
        &raw mut dummy_path_x as *const ::core::ffi::c_void,
    );
    if !item.is_null() {
        ret = (*((*item).key as *mut picoquic_path_t)).cnx as *mut picoquic_cnx_t;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cnx_by_icid(
    mut quic: *mut picoquic_quic_t,
    mut icid: *mut picoquic_connection_id_t,
    mut addr: *const sockaddr,
) -> *mut picoquic_cnx_t {
    let mut ret: *mut picoquic_cnx_t = ::core::ptr::null_mut::<picoquic_cnx_t>();
    let mut item: *mut picohash_item = ::core::ptr::null_mut::<picohash_item>();
    let mut dummy_cnx: picoquic_cnx_t = {
        let mut init = st_picoquic_cnx_t {
            is_0RTT_accepted_remote_parameters_received_client_mode_key_phase_enc_key_phase_dec_zero_rtt_data_accepted_sending_ecn_ack_sent_blocked_frame_stream_blocked_bidir_sent_stream_blocked_unidir_sent_max_stream_data_needed_path_demotion_needed_alt_path_challenge_needed_is_handshake_finished_is_handshake_done_acked_is_new_token_acked_is_1rtt_received_is_1rtt_acked_has_successful_probe_grease_transport_parameters_test_large_chello_initial_validated_initial_repeat_needed_is_loss_bit_enabled_incoming_is_loss_bit_enabled_outgoing_is_ack_frequency_negotiated_is_ack_frequency_updated_recycle_sooner_needed_is_time_stamp_enabled_is_time_stamp_sent_is_pacing_update_requested_is_path_quality_update_requested_is_hcid_verified_do_grease_quic_bit_quic_bit_greased_quic_bit_received_0_is_half_open_did_receive_short_initial_ack_ignore_order_local_ack_ignore_order_remote_are_path_callbacks_enabled_is_sending_large_buffer_is_preemptive_repeat_enabled_do_version_negotiation_send_receive_bdp_frame_cwin_notified_from_seed_is_datagram_ready_is_immediate_ack_required_is_multipath_enabled_is_lost_feedback_notification_required_is_forced_probe_up_required_is_address_discovery_provider_is_address_discovery_receiver_is_poll_requested_no_ack_delay: [0; 7],
            c2rust_padding: [0; 1],
            cwin_blocked_flow_blocked_stream_blocked: [0; 1],
            c2rust_padding_0: [0; 7],
            quic: ::core::ptr::null_mut::<picoquic_quic_t>(),
            next_in_table: ::core::ptr::null_mut::<st_picoquic_cnx_t>(),
            previous_in_table: ::core::ptr::null_mut::<st_picoquic_cnx_t>(),
            proposed_version: 0,
            rejected_version: 0,
            desired_version: 0,
            version_index: 0,
            pmtud_policy: picoquic_pmtud_basic,
            spin_policy: picoquic_spinbit_basic,
            idle_timeout: 0,
            local_parameters: st_picoquic_tp_t {
                initial_max_stream_data_bidi_local: 0,
                initial_max_stream_data_bidi_remote: 0,
                initial_max_stream_data_uni: 0,
                initial_max_data: 0,
                initial_max_stream_id_bidir: 0,
                initial_max_stream_id_unidir: 0,
                max_idle_timeout: 0,
                max_packet_size: 0,
                max_ack_delay: 0,
                active_connection_id_limit: 0,
                ack_delay_exponent: 0,
                migration_disabled: 0,
                prefered_address: st_picoquic_tp_prefered_address_t {
                    is_defined: 0,
                    ipv4Address: [0; 4],
                    ipv4Port: 0,
                    ipv6Address: [0; 16],
                    ipv6Port: 0,
                    connection_id: st_picoquic_connection_id_t {
                        id: [0; 20],
                        id_len: 0,
                    },
                    statelessResetToken: [0; 16],
                },
                max_datagram_frame_size: 0,
                enable_loss_bit: 0,
                enable_time_stamp: 0,
                min_ack_delay: 0,
                do_grease_quic_bit: 0,
                version_negotiation: st_picoquic_tp_version_negotiation_t {
                    current: 0,
                    previous: 0,
                    nb_received: 0,
                    received: ::core::ptr::null_mut::<uint32_t>(),
                    nb_supported: 0,
                    supported: ::core::ptr::null_mut::<uint32_t>(),
                },
                enable_bdp_frame: 0,
                is_multipath_enabled: 0,
                initial_max_path_id: 0,
                address_discovery_mode: 0,
            },
            remote_parameters: st_picoquic_tp_t {
                initial_max_stream_data_bidi_local: 0,
                initial_max_stream_data_bidi_remote: 0,
                initial_max_stream_data_uni: 0,
                initial_max_data: 0,
                initial_max_stream_id_bidir: 0,
                initial_max_stream_id_unidir: 0,
                max_idle_timeout: 0,
                max_packet_size: 0,
                max_ack_delay: 0,
                active_connection_id_limit: 0,
                ack_delay_exponent: 0,
                migration_disabled: 0,
                prefered_address: st_picoquic_tp_prefered_address_t {
                    is_defined: 0,
                    ipv4Address: [0; 4],
                    ipv4Port: 0,
                    ipv6Address: [0; 16],
                    ipv6Port: 0,
                    connection_id: st_picoquic_connection_id_t {
                        id: [0; 20],
                        id_len: 0,
                    },
                    statelessResetToken: [0; 16],
                },
                max_datagram_frame_size: 0,
                enable_loss_bit: 0,
                enable_time_stamp: 0,
                min_ack_delay: 0,
                do_grease_quic_bit: 0,
                version_negotiation: st_picoquic_tp_version_negotiation_t {
                    current: 0,
                    previous: 0,
                    nb_received: 0,
                    received: ::core::ptr::null_mut::<uint32_t>(),
                    nb_supported: 0,
                    supported: ::core::ptr::null_mut::<uint32_t>(),
                },
                enable_bdp_frame: 0,
                is_multipath_enabled: 0,
                initial_max_path_id: 0,
                address_discovery_mode: 0,
            },
            padding_multiple: 0,
            padding_minsize: 0,
            seed_ip_addr: [0; 16],
            seed_ip_addr_length: 0,
            seed_rtt_min: 0,
            seed_cwin: 0,
            issued_ticket_id: 0,
            resumed_ticket_id: 0,
            sni: ::core::ptr::null::<::core::ffi::c_char>(),
            alpn: ::core::ptr::null::<::core::ffi::c_char>(),
            max_early_data_size: 0,
            callback_fn: None,
            callback_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            cnx_state: picoquic_state_client_init,
            initial_cnxid: st_picoquic_connection_id_t {
                id: [0; 20],
                id_len: 0,
            },
            original_cnxid: st_picoquic_connection_id_t {
                id: [0; 20],
                id_len: 0,
            },
            registered_icid_addr: sockaddr_storage {
                ss_family: 0,
                __ss_padding: [0; 118],
                __ss_align: 0,
            },
            registered_icid_item: _picohash_item {
                hash: 0,
                next_in_bin: ::core::ptr::null_mut::<_picohash_item>(),
                key: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            registered_secret_addr: sockaddr_storage {
                ss_family: 0,
                __ss_padding: [0; 118],
                __ss_align: 0,
            },
            registered_reset_secret: [0; 16],
            registered_reset_secret_item: _picohash_item {
                hash: 0,
                next_in_bin: ::core::ptr::null_mut::<_picohash_item>(),
                key: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            start_time: 0,
            phase_delay: 0,
            application_error: 0,
            local_error: 0,
            local_error_reason: ::core::ptr::null::<::core::ffi::c_char>(),
            remote_application_error: 0,
            remote_error: 0,
            offending_frame_type: 0,
            retry_token_length: 0,
            retry_token: ::core::ptr::null_mut::<uint8_t>(),
            next_wake_time: 0,
            cnx_wake_node: st_picosplay_node_t {
                parent: ::core::ptr::null_mut::<st_picosplay_node_t>(),
                left: ::core::ptr::null_mut::<st_picosplay_node_t>(),
                right: ::core::ptr::null_mut::<st_picosplay_node_t>(),
            },
            app_wake_time: 0,
            tls_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            crypto_epoch_length_max: 0,
            crypto_epoch_sequence: 0,
            crypto_rotation_time_guard: 0,
            tls_sendbuf: ::core::ptr::null_mut::<st_ptls_buffer_t>(),
            psk_cipher_suite_id: 0,
            tls_stream: [st_picoquic_stream_head_t {
                stream_node: st_picosplay_node_t {
                    parent: ::core::ptr::null_mut::<st_picosplay_node_t>(),
                    left: ::core::ptr::null_mut::<st_picosplay_node_t>(),
                    right: ::core::ptr::null_mut::<st_picosplay_node_t>(),
                },
                next_output_stream: ::core::ptr::null_mut::<st_picoquic_stream_head_t>(),
                previous_output_stream: ::core::ptr::null_mut::<
                    st_picoquic_stream_head_t,
                >(),
                cnx: ::core::ptr::null_mut::<picoquic_cnx_t>(),
                stream_id: 0,
                affinity_path: ::core::ptr::null_mut::<st_picoquic_path_t>(),
                consumed_offset: 0,
                fin_offset: 0,
                maxdata_local: 0,
                maxdata_local_acked: 0,
                maxdata_remote: 0,
                local_error: 0,
                remote_error: 0,
                local_stop_error: 0,
                remote_stop_error: 0,
                last_time_data_sent: 0,
                stream_data_tree: st_picosplay_tree_t {
                    root: ::core::ptr::null_mut::<picosplay_node_t>(),
                    comp: None,
                    create: None,
                    delete_node: None,
                    node_value: None,
                    size: 0,
                },
                sent_offset: 0,
                send_queue: ::core::ptr::null_mut::<picoquic_stream_queue_node_t>(),
                app_stream_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                direct_receive_fn: None,
                direct_receive_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sack_list: st_picoquic_sack_list_t {
                    ack_tree: st_picosplay_tree_t {
                        root: ::core::ptr::null_mut::<picosplay_node_t>(),
                        comp: None,
                        create: None,
                        delete_node: None,
                        node_value: None,
                        size: 0,
                    },
                    ack_horizon: 0,
                    horizon_delay: 0,
                    rc: [st_picoquic_sack_range_count_t {
                        range_counts: [0; 4],
                    }; 2],
                },
                stream_priority: 0,
                is_active_fin_requested_fin_sent_fin_received_fin_signalled_reset_requested_reset_sent_reset_acked_reset_received_reset_signalled_stop_sending_requested_stop_sending_sent_stop_sending_received_stop_sending_signalled_max_stream_updated_stream_data_blocked_sent_is_output_stream_is_closed_is_discarded: [0; 3],
                c2rust_padding: [0; 4],
            }; 4],
            crypto_context: [st_picoquic_crypto_context_t {
                aead_encrypt: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                aead_decrypt: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pn_enc: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pn_dec: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            }; 4],
            crypto_context_old: st_picoquic_crypto_context_t {
                aead_encrypt: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                aead_decrypt: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pn_enc: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pn_dec: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            },
            crypto_context_new: st_picoquic_crypto_context_t {
                aead_encrypt: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                aead_decrypt: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pn_enc: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pn_dec: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            },
            crypto_failure_count: 0,
            latest_progress_time: 0,
            latest_receive_time: 0,
            last_close_sent: 0,
            pkt_ctx: [st_picoquic_packet_context_t {
                send_sequence: 0,
                next_sequence_hole: 0,
                retransmit_sequence: 0,
                highest_acknowledged: 0,
                latest_time_acknowledged: 0,
                highest_acknowledged_time: 0,
                pending_last: ::core::ptr::null_mut::<picoquic_packet_t>(),
                pending_first: ::core::ptr::null_mut::<picoquic_packet_t>(),
                retransmitted_newest: ::core::ptr::null_mut::<picoquic_packet_t>(),
                retransmitted_oldest: ::core::ptr::null_mut::<picoquic_packet_t>(),
                preemptive_repeat_ptr: ::core::ptr::null_mut::<picoquic_packet_t>(),
                retransmitted_queue_size: 0,
                ecn_ect0_total_remote: 0,
                ecn_ect1_total_remote: 0,
                ecn_ce_total_remote: 0,
                ack_of_ack_requested: [0; 1],
                c2rust_padding: [0; 7],
            }; 3],
            ack_ctx: [st_picoquic_ack_context_t {
                sack_list: st_picoquic_sack_list_t {
                    ack_tree: st_picosplay_tree_t {
                        root: ::core::ptr::null_mut::<picosplay_node_t>(),
                        comp: None,
                        create: None,
                        delete_node: None,
                        node_value: None,
                        size: 0,
                    },
                    ack_horizon: 0,
                    horizon_delay: 0,
                    rc: [st_picoquic_sack_range_count_t {
                        range_counts: [0; 4],
                    }; 2],
                },
                time_stamp_largest_received: 0,
                act: [st_picoquic_ack_context_track_t {
                    highest_ack_sent: 0,
                    highest_ack_sent_time: 0,
                    time_oldest_unack_packet_received: 0,
                    ack_needed_ack_after_fin_out_of_order_received_is_immediate_ack_required: [0; 1],
                    c2rust_padding: [0; 7],
                }; 2],
                crypto_rotation_sequence: 0,
                ecn_ect0_total_local: 0,
                ecn_ect1_total_local: 0,
                ecn_ce_total_local: 0,
                sending_ecn_ack: [0; 1],
                c2rust_padding: [0; 7],
            }; 3],
            observed_number: 0,
            nb_bytes_queued: 0,
            nb_zero_rtt_sent: 0,
            nb_zero_rtt_acked: 0,
            nb_zero_rtt_received: 0,
            max_mtu_sent: 0,
            max_mtu_received: 0,
            nb_packets_received: 0,
            nb_trains_sent: 0,
            nb_trains_short: 0,
            nb_trains_blocked_cwin: 0,
            nb_trains_blocked_pacing: 0,
            nb_trains_blocked_others: 0,
            nb_packets_sent: 0,
            nb_packets_logged: 0,
            nb_retransmission_total: 0,
            nb_preemptive_repeat: 0,
            nb_spurious: 0,
            nb_crypto_key_rotations: 0,
            nb_packet_holes_inserted: 0,
            max_ack_delay_remote: 0,
            max_ack_gap_remote: 0,
            max_ack_delay_local: 0,
            max_ack_gap_local: 0,
            min_ack_delay_remote: 0,
            min_ack_delay_local: 0,
            congestion_alg: ::core::ptr::null::<picoquic_congestion_algorithm_t>(),
            rtt_update_delta: 0,
            pacing_rate_update_delta: 0,
            pacing_rate_signalled: 0,
            pacing_increase_threshold: 0,
            pacing_decrease_threshold: 0,
            pacing_change_threshold: 0,
            initial_data_received: 0,
            initial_data_sent: 0,
            data_sent: 0,
            data_received: 0,
            maxdata_local: 0,
            maxdata_local_acked: 0,
            maxdata_remote: 0,
            max_stream_data_local: 0,
            max_stream_data_remote: 0,
            max_stream_id_bidir_local: 0,
            max_stream_id_bidir_rank_acked: 0,
            max_stream_id_bidir_local_computed: 0,
            max_stream_id_bidir_remote: 0,
            max_stream_id_unidir_local: 0,
            max_stream_id_unidir_rank_acked: 0,
            max_stream_id_unidir_local_computed: 0,
            max_stream_id_unidir_remote: 0,
            first_misc_frame: ::core::ptr::null_mut::<picoquic_misc_frame_header_t>(),
            last_misc_frame: ::core::ptr::null_mut::<picoquic_misc_frame_header_t>(),
            stream_tree: st_picosplay_tree_t {
                root: ::core::ptr::null_mut::<picosplay_node_t>(),
                comp: None,
                create: None,
                delete_node: None,
                node_value: None,
                size: 0,
            },
            first_output_stream: ::core::ptr::null_mut::<picoquic_stream_head_t>(),
            last_output_stream: ::core::ptr::null_mut::<picoquic_stream_head_t>(),
            high_priority_stream_id: 0,
            next_stream_id: [0; 4],
            priority_limit_for_bypass: 0,
            priority_bypass_pacing: st_picoquic_pacing_t {
                rate: 0,
                evaluation_time: 0,
                bucket_max: 0,
                packet_time_microsec: 0,
                quantum_max: 0,
                rate_max: 0,
                bandwidth_pause: 0,
                bucket_nanosec: 0,
                packet_time_nanosec: 0,
            },
            queue_data_repeat_tree: st_picosplay_tree_t {
                root: ::core::ptr::null_mut::<picosplay_node_t>(),
                comp: None,
                create: None,
                delete_node: None,
                node_value: None,
                size: 0,
            },
            first_datagram: ::core::ptr::null_mut::<picoquic_misc_frame_header_t>(),
            last_datagram: ::core::ptr::null_mut::<picoquic_misc_frame_header_t>(),
            datagram_priority: 0,
            datagram_conflicts_count: 0,
            datagram_conflicts_max: 0,
            keep_alive_interval: 0,
            path: ::core::ptr::null_mut::<*mut picoquic_path_t>(),
            nb_paths: 0,
            nb_path_alloc: 0,
            last_path_polled: 0,
            unique_path_id_next: 0,
            nominal_path_for_ack: ::core::ptr::null_mut::<picoquic_path_t>(),
            status_sequence_to_send_next: 0,
            max_path_id_local: 0,
            max_path_id_acknowledged: 0,
            max_path_id_remote: 0,
            path_blocked_acknowledged: 0,
            first_remote_cnxid_stash: ::core::ptr::null_mut::<
                picoquic_remote_cnxid_stash_t,
            >(),
            nb_local_cnxid_lists: 0,
            next_path_id_in_lists: 0,
            first_local_cnxid_list: ::core::ptr::null_mut::<
                picoquic_local_cnxid_list_t,
            >(),
            ack_frequency_sequence_local: 0,
            ack_gap_local: 0,
            ack_frequency_delay_local: 0,
            ack_frequency_sequence_remote: 0,
            ack_gap_remote: 0,
            ack_delay_remote: 0,
            ack_reordering_threshold_remote: 0,
            first_sooner: ::core::ptr::null_mut::<picoquic_stateless_packet_t>(),
            last_sooner: ::core::ptr::null_mut::<picoquic_stateless_packet_t>(),
            log_unique: 0,
            f_binlog: ::core::ptr::null_mut::<FILE>(),
            binlog_file_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        init.set_is_0RTT_accepted(0);
        init.set_remote_parameters_received(0);
        init.set_client_mode(0);
        init.set_key_phase_enc(0);
        init.set_key_phase_dec(0);
        init.set_zero_rtt_data_accepted(0);
        init.set_sending_ecn_ack(0);
        init.set_sent_blocked_frame(0);
        init.set_stream_blocked_bidir_sent(0);
        init.set_stream_blocked_unidir_sent(0);
        init.set_max_stream_data_needed(0);
        init.set_path_demotion_needed(0);
        init.set_alt_path_challenge_needed(0);
        init.set_is_handshake_finished(0);
        init.set_is_handshake_done_acked(0);
        init.set_is_new_token_acked(0);
        init.set_is_1rtt_received(0);
        init.set_is_1rtt_acked(0);
        init.set_has_successful_probe(0);
        init.set_grease_transport_parameters(0);
        init.set_test_large_chello(0);
        init.set_initial_validated(0);
        init.set_initial_repeat_needed(0);
        init.set_is_loss_bit_enabled_incoming(0);
        init.set_is_loss_bit_enabled_outgoing(0);
        init.set_is_ack_frequency_negotiated(0);
        init.set_is_ack_frequency_updated(0);
        init.set_recycle_sooner_needed(0);
        init.set_is_time_stamp_enabled(0);
        init.set_is_time_stamp_sent(0);
        init.set_is_pacing_update_requested(0);
        init.set_is_path_quality_update_requested(0);
        init.set_is_hcid_verified(0);
        init.set_do_grease_quic_bit(0);
        init.set_quic_bit_greased(0);
        init.set_quic_bit_received_0(0);
        init.set_is_half_open(0);
        init.set_did_receive_short_initial(0);
        init.set_ack_ignore_order_local(0);
        init.set_ack_ignore_order_remote(0);
        init.set_are_path_callbacks_enabled(0);
        init.set_is_sending_large_buffer(0);
        init.set_is_preemptive_repeat_enabled(0);
        init.set_do_version_negotiation(0);
        init.set_send_receive_bdp_frame(0);
        init.set_cwin_notified_from_seed(0);
        init.set_is_datagram_ready(0);
        init.set_is_immediate_ack_required(0);
        init.set_is_multipath_enabled(0);
        init.set_is_lost_feedback_notification_required(0);
        init.set_is_forced_probe_up_required(0);
        init.set_is_address_discovery_provider(0);
        init.set_is_address_discovery_receiver(0);
        init.set_is_poll_requested(0);
        init.set_no_ack_delay(0);
        init.set_cwin_blocked(0);
        init.set_flow_blocked(0);
        init.set_stream_blocked(0);
        init
    };
    picoquic_store_addr(&raw mut dummy_cnx.registered_icid_addr, addr);
    dummy_cnx.initial_cnxid = *icid;
    item = picohash_retrieve(
        (*quic).table_cnx_by_icid,
        &raw mut dummy_cnx as *const ::core::ffi::c_void,
    );
    if !item.is_null() {
        ret = (*item).key as *mut picoquic_cnx_t;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cnx_by_secret(
    mut quic: *mut picoquic_quic_t,
    mut reset_secret: *const uint8_t,
    mut addr: *const sockaddr,
) -> *mut picoquic_cnx_t {
    let mut ret: *mut picoquic_cnx_t = ::core::ptr::null_mut::<picoquic_cnx_t>();
    let mut item: *mut picohash_item = ::core::ptr::null_mut::<picohash_item>();
    let mut dummy_cnx: picoquic_cnx_t = {
        let mut init = st_picoquic_cnx_t {
            is_0RTT_accepted_remote_parameters_received_client_mode_key_phase_enc_key_phase_dec_zero_rtt_data_accepted_sending_ecn_ack_sent_blocked_frame_stream_blocked_bidir_sent_stream_blocked_unidir_sent_max_stream_data_needed_path_demotion_needed_alt_path_challenge_needed_is_handshake_finished_is_handshake_done_acked_is_new_token_acked_is_1rtt_received_is_1rtt_acked_has_successful_probe_grease_transport_parameters_test_large_chello_initial_validated_initial_repeat_needed_is_loss_bit_enabled_incoming_is_loss_bit_enabled_outgoing_is_ack_frequency_negotiated_is_ack_frequency_updated_recycle_sooner_needed_is_time_stamp_enabled_is_time_stamp_sent_is_pacing_update_requested_is_path_quality_update_requested_is_hcid_verified_do_grease_quic_bit_quic_bit_greased_quic_bit_received_0_is_half_open_did_receive_short_initial_ack_ignore_order_local_ack_ignore_order_remote_are_path_callbacks_enabled_is_sending_large_buffer_is_preemptive_repeat_enabled_do_version_negotiation_send_receive_bdp_frame_cwin_notified_from_seed_is_datagram_ready_is_immediate_ack_required_is_multipath_enabled_is_lost_feedback_notification_required_is_forced_probe_up_required_is_address_discovery_provider_is_address_discovery_receiver_is_poll_requested_no_ack_delay: [0; 7],
            c2rust_padding: [0; 1],
            cwin_blocked_flow_blocked_stream_blocked: [0; 1],
            c2rust_padding_0: [0; 7],
            quic: ::core::ptr::null_mut::<picoquic_quic_t>(),
            next_in_table: ::core::ptr::null_mut::<st_picoquic_cnx_t>(),
            previous_in_table: ::core::ptr::null_mut::<st_picoquic_cnx_t>(),
            proposed_version: 0,
            rejected_version: 0,
            desired_version: 0,
            version_index: 0,
            pmtud_policy: picoquic_pmtud_basic,
            spin_policy: picoquic_spinbit_basic,
            idle_timeout: 0,
            local_parameters: st_picoquic_tp_t {
                initial_max_stream_data_bidi_local: 0,
                initial_max_stream_data_bidi_remote: 0,
                initial_max_stream_data_uni: 0,
                initial_max_data: 0,
                initial_max_stream_id_bidir: 0,
                initial_max_stream_id_unidir: 0,
                max_idle_timeout: 0,
                max_packet_size: 0,
                max_ack_delay: 0,
                active_connection_id_limit: 0,
                ack_delay_exponent: 0,
                migration_disabled: 0,
                prefered_address: st_picoquic_tp_prefered_address_t {
                    is_defined: 0,
                    ipv4Address: [0; 4],
                    ipv4Port: 0,
                    ipv6Address: [0; 16],
                    ipv6Port: 0,
                    connection_id: st_picoquic_connection_id_t {
                        id: [0; 20],
                        id_len: 0,
                    },
                    statelessResetToken: [0; 16],
                },
                max_datagram_frame_size: 0,
                enable_loss_bit: 0,
                enable_time_stamp: 0,
                min_ack_delay: 0,
                do_grease_quic_bit: 0,
                version_negotiation: st_picoquic_tp_version_negotiation_t {
                    current: 0,
                    previous: 0,
                    nb_received: 0,
                    received: ::core::ptr::null_mut::<uint32_t>(),
                    nb_supported: 0,
                    supported: ::core::ptr::null_mut::<uint32_t>(),
                },
                enable_bdp_frame: 0,
                is_multipath_enabled: 0,
                initial_max_path_id: 0,
                address_discovery_mode: 0,
            },
            remote_parameters: st_picoquic_tp_t {
                initial_max_stream_data_bidi_local: 0,
                initial_max_stream_data_bidi_remote: 0,
                initial_max_stream_data_uni: 0,
                initial_max_data: 0,
                initial_max_stream_id_bidir: 0,
                initial_max_stream_id_unidir: 0,
                max_idle_timeout: 0,
                max_packet_size: 0,
                max_ack_delay: 0,
                active_connection_id_limit: 0,
                ack_delay_exponent: 0,
                migration_disabled: 0,
                prefered_address: st_picoquic_tp_prefered_address_t {
                    is_defined: 0,
                    ipv4Address: [0; 4],
                    ipv4Port: 0,
                    ipv6Address: [0; 16],
                    ipv6Port: 0,
                    connection_id: st_picoquic_connection_id_t {
                        id: [0; 20],
                        id_len: 0,
                    },
                    statelessResetToken: [0; 16],
                },
                max_datagram_frame_size: 0,
                enable_loss_bit: 0,
                enable_time_stamp: 0,
                min_ack_delay: 0,
                do_grease_quic_bit: 0,
                version_negotiation: st_picoquic_tp_version_negotiation_t {
                    current: 0,
                    previous: 0,
                    nb_received: 0,
                    received: ::core::ptr::null_mut::<uint32_t>(),
                    nb_supported: 0,
                    supported: ::core::ptr::null_mut::<uint32_t>(),
                },
                enable_bdp_frame: 0,
                is_multipath_enabled: 0,
                initial_max_path_id: 0,
                address_discovery_mode: 0,
            },
            padding_multiple: 0,
            padding_minsize: 0,
            seed_ip_addr: [0; 16],
            seed_ip_addr_length: 0,
            seed_rtt_min: 0,
            seed_cwin: 0,
            issued_ticket_id: 0,
            resumed_ticket_id: 0,
            sni: ::core::ptr::null::<::core::ffi::c_char>(),
            alpn: ::core::ptr::null::<::core::ffi::c_char>(),
            max_early_data_size: 0,
            callback_fn: None,
            callback_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            cnx_state: picoquic_state_client_init,
            initial_cnxid: st_picoquic_connection_id_t {
                id: [0; 20],
                id_len: 0,
            },
            original_cnxid: st_picoquic_connection_id_t {
                id: [0; 20],
                id_len: 0,
            },
            registered_icid_addr: sockaddr_storage {
                ss_family: 0,
                __ss_padding: [0; 118],
                __ss_align: 0,
            },
            registered_icid_item: _picohash_item {
                hash: 0,
                next_in_bin: ::core::ptr::null_mut::<_picohash_item>(),
                key: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            registered_secret_addr: sockaddr_storage {
                ss_family: 0,
                __ss_padding: [0; 118],
                __ss_align: 0,
            },
            registered_reset_secret: [0; 16],
            registered_reset_secret_item: _picohash_item {
                hash: 0,
                next_in_bin: ::core::ptr::null_mut::<_picohash_item>(),
                key: ::core::ptr::null::<::core::ffi::c_void>(),
            },
            start_time: 0,
            phase_delay: 0,
            application_error: 0,
            local_error: 0,
            local_error_reason: ::core::ptr::null::<::core::ffi::c_char>(),
            remote_application_error: 0,
            remote_error: 0,
            offending_frame_type: 0,
            retry_token_length: 0,
            retry_token: ::core::ptr::null_mut::<uint8_t>(),
            next_wake_time: 0,
            cnx_wake_node: st_picosplay_node_t {
                parent: ::core::ptr::null_mut::<st_picosplay_node_t>(),
                left: ::core::ptr::null_mut::<st_picosplay_node_t>(),
                right: ::core::ptr::null_mut::<st_picosplay_node_t>(),
            },
            app_wake_time: 0,
            tls_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            crypto_epoch_length_max: 0,
            crypto_epoch_sequence: 0,
            crypto_rotation_time_guard: 0,
            tls_sendbuf: ::core::ptr::null_mut::<st_ptls_buffer_t>(),
            psk_cipher_suite_id: 0,
            tls_stream: [st_picoquic_stream_head_t {
                stream_node: st_picosplay_node_t {
                    parent: ::core::ptr::null_mut::<st_picosplay_node_t>(),
                    left: ::core::ptr::null_mut::<st_picosplay_node_t>(),
                    right: ::core::ptr::null_mut::<st_picosplay_node_t>(),
                },
                next_output_stream: ::core::ptr::null_mut::<st_picoquic_stream_head_t>(),
                previous_output_stream: ::core::ptr::null_mut::<
                    st_picoquic_stream_head_t,
                >(),
                cnx: ::core::ptr::null_mut::<picoquic_cnx_t>(),
                stream_id: 0,
                affinity_path: ::core::ptr::null_mut::<st_picoquic_path_t>(),
                consumed_offset: 0,
                fin_offset: 0,
                maxdata_local: 0,
                maxdata_local_acked: 0,
                maxdata_remote: 0,
                local_error: 0,
                remote_error: 0,
                local_stop_error: 0,
                remote_stop_error: 0,
                last_time_data_sent: 0,
                stream_data_tree: st_picosplay_tree_t {
                    root: ::core::ptr::null_mut::<picosplay_node_t>(),
                    comp: None,
                    create: None,
                    delete_node: None,
                    node_value: None,
                    size: 0,
                },
                sent_offset: 0,
                send_queue: ::core::ptr::null_mut::<picoquic_stream_queue_node_t>(),
                app_stream_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                direct_receive_fn: None,
                direct_receive_ctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                sack_list: st_picoquic_sack_list_t {
                    ack_tree: st_picosplay_tree_t {
                        root: ::core::ptr::null_mut::<picosplay_node_t>(),
                        comp: None,
                        create: None,
                        delete_node: None,
                        node_value: None,
                        size: 0,
                    },
                    ack_horizon: 0,
                    horizon_delay: 0,
                    rc: [st_picoquic_sack_range_count_t {
                        range_counts: [0; 4],
                    }; 2],
                },
                stream_priority: 0,
                is_active_fin_requested_fin_sent_fin_received_fin_signalled_reset_requested_reset_sent_reset_acked_reset_received_reset_signalled_stop_sending_requested_stop_sending_sent_stop_sending_received_stop_sending_signalled_max_stream_updated_stream_data_blocked_sent_is_output_stream_is_closed_is_discarded: [0; 3],
                c2rust_padding: [0; 4],
            }; 4],
            crypto_context: [st_picoquic_crypto_context_t {
                aead_encrypt: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                aead_decrypt: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pn_enc: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pn_dec: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            }; 4],
            crypto_context_old: st_picoquic_crypto_context_t {
                aead_encrypt: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                aead_decrypt: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pn_enc: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pn_dec: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            },
            crypto_context_new: st_picoquic_crypto_context_t {
                aead_encrypt: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                aead_decrypt: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pn_enc: ::core::ptr::null_mut::<::core::ffi::c_void>(),
                pn_dec: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            },
            crypto_failure_count: 0,
            latest_progress_time: 0,
            latest_receive_time: 0,
            last_close_sent: 0,
            pkt_ctx: [st_picoquic_packet_context_t {
                send_sequence: 0,
                next_sequence_hole: 0,
                retransmit_sequence: 0,
                highest_acknowledged: 0,
                latest_time_acknowledged: 0,
                highest_acknowledged_time: 0,
                pending_last: ::core::ptr::null_mut::<picoquic_packet_t>(),
                pending_first: ::core::ptr::null_mut::<picoquic_packet_t>(),
                retransmitted_newest: ::core::ptr::null_mut::<picoquic_packet_t>(),
                retransmitted_oldest: ::core::ptr::null_mut::<picoquic_packet_t>(),
                preemptive_repeat_ptr: ::core::ptr::null_mut::<picoquic_packet_t>(),
                retransmitted_queue_size: 0,
                ecn_ect0_total_remote: 0,
                ecn_ect1_total_remote: 0,
                ecn_ce_total_remote: 0,
                ack_of_ack_requested: [0; 1],
                c2rust_padding: [0; 7],
            }; 3],
            ack_ctx: [st_picoquic_ack_context_t {
                sack_list: st_picoquic_sack_list_t {
                    ack_tree: st_picosplay_tree_t {
                        root: ::core::ptr::null_mut::<picosplay_node_t>(),
                        comp: None,
                        create: None,
                        delete_node: None,
                        node_value: None,
                        size: 0,
                    },
                    ack_horizon: 0,
                    horizon_delay: 0,
                    rc: [st_picoquic_sack_range_count_t {
                        range_counts: [0; 4],
                    }; 2],
                },
                time_stamp_largest_received: 0,
                act: [st_picoquic_ack_context_track_t {
                    highest_ack_sent: 0,
                    highest_ack_sent_time: 0,
                    time_oldest_unack_packet_received: 0,
                    ack_needed_ack_after_fin_out_of_order_received_is_immediate_ack_required: [0; 1],
                    c2rust_padding: [0; 7],
                }; 2],
                crypto_rotation_sequence: 0,
                ecn_ect0_total_local: 0,
                ecn_ect1_total_local: 0,
                ecn_ce_total_local: 0,
                sending_ecn_ack: [0; 1],
                c2rust_padding: [0; 7],
            }; 3],
            observed_number: 0,
            nb_bytes_queued: 0,
            nb_zero_rtt_sent: 0,
            nb_zero_rtt_acked: 0,
            nb_zero_rtt_received: 0,
            max_mtu_sent: 0,
            max_mtu_received: 0,
            nb_packets_received: 0,
            nb_trains_sent: 0,
            nb_trains_short: 0,
            nb_trains_blocked_cwin: 0,
            nb_trains_blocked_pacing: 0,
            nb_trains_blocked_others: 0,
            nb_packets_sent: 0,
            nb_packets_logged: 0,
            nb_retransmission_total: 0,
            nb_preemptive_repeat: 0,
            nb_spurious: 0,
            nb_crypto_key_rotations: 0,
            nb_packet_holes_inserted: 0,
            max_ack_delay_remote: 0,
            max_ack_gap_remote: 0,
            max_ack_delay_local: 0,
            max_ack_gap_local: 0,
            min_ack_delay_remote: 0,
            min_ack_delay_local: 0,
            congestion_alg: ::core::ptr::null::<picoquic_congestion_algorithm_t>(),
            rtt_update_delta: 0,
            pacing_rate_update_delta: 0,
            pacing_rate_signalled: 0,
            pacing_increase_threshold: 0,
            pacing_decrease_threshold: 0,
            pacing_change_threshold: 0,
            initial_data_received: 0,
            initial_data_sent: 0,
            data_sent: 0,
            data_received: 0,
            maxdata_local: 0,
            maxdata_local_acked: 0,
            maxdata_remote: 0,
            max_stream_data_local: 0,
            max_stream_data_remote: 0,
            max_stream_id_bidir_local: 0,
            max_stream_id_bidir_rank_acked: 0,
            max_stream_id_bidir_local_computed: 0,
            max_stream_id_bidir_remote: 0,
            max_stream_id_unidir_local: 0,
            max_stream_id_unidir_rank_acked: 0,
            max_stream_id_unidir_local_computed: 0,
            max_stream_id_unidir_remote: 0,
            first_misc_frame: ::core::ptr::null_mut::<picoquic_misc_frame_header_t>(),
            last_misc_frame: ::core::ptr::null_mut::<picoquic_misc_frame_header_t>(),
            stream_tree: st_picosplay_tree_t {
                root: ::core::ptr::null_mut::<picosplay_node_t>(),
                comp: None,
                create: None,
                delete_node: None,
                node_value: None,
                size: 0,
            },
            first_output_stream: ::core::ptr::null_mut::<picoquic_stream_head_t>(),
            last_output_stream: ::core::ptr::null_mut::<picoquic_stream_head_t>(),
            high_priority_stream_id: 0,
            next_stream_id: [0; 4],
            priority_limit_for_bypass: 0,
            priority_bypass_pacing: st_picoquic_pacing_t {
                rate: 0,
                evaluation_time: 0,
                bucket_max: 0,
                packet_time_microsec: 0,
                quantum_max: 0,
                rate_max: 0,
                bandwidth_pause: 0,
                bucket_nanosec: 0,
                packet_time_nanosec: 0,
            },
            queue_data_repeat_tree: st_picosplay_tree_t {
                root: ::core::ptr::null_mut::<picosplay_node_t>(),
                comp: None,
                create: None,
                delete_node: None,
                node_value: None,
                size: 0,
            },
            first_datagram: ::core::ptr::null_mut::<picoquic_misc_frame_header_t>(),
            last_datagram: ::core::ptr::null_mut::<picoquic_misc_frame_header_t>(),
            datagram_priority: 0,
            datagram_conflicts_count: 0,
            datagram_conflicts_max: 0,
            keep_alive_interval: 0,
            path: ::core::ptr::null_mut::<*mut picoquic_path_t>(),
            nb_paths: 0,
            nb_path_alloc: 0,
            last_path_polled: 0,
            unique_path_id_next: 0,
            nominal_path_for_ack: ::core::ptr::null_mut::<picoquic_path_t>(),
            status_sequence_to_send_next: 0,
            max_path_id_local: 0,
            max_path_id_acknowledged: 0,
            max_path_id_remote: 0,
            path_blocked_acknowledged: 0,
            first_remote_cnxid_stash: ::core::ptr::null_mut::<
                picoquic_remote_cnxid_stash_t,
            >(),
            nb_local_cnxid_lists: 0,
            next_path_id_in_lists: 0,
            first_local_cnxid_list: ::core::ptr::null_mut::<
                picoquic_local_cnxid_list_t,
            >(),
            ack_frequency_sequence_local: 0,
            ack_gap_local: 0,
            ack_frequency_delay_local: 0,
            ack_frequency_sequence_remote: 0,
            ack_gap_remote: 0,
            ack_delay_remote: 0,
            ack_reordering_threshold_remote: 0,
            first_sooner: ::core::ptr::null_mut::<picoquic_stateless_packet_t>(),
            last_sooner: ::core::ptr::null_mut::<picoquic_stateless_packet_t>(),
            log_unique: 0,
            f_binlog: ::core::ptr::null_mut::<FILE>(),
            binlog_file_name: ::core::ptr::null_mut::<::core::ffi::c_char>(),
        };
        init.set_is_0RTT_accepted(0);
        init.set_remote_parameters_received(0);
        init.set_client_mode(0);
        init.set_key_phase_enc(0);
        init.set_key_phase_dec(0);
        init.set_zero_rtt_data_accepted(0);
        init.set_sending_ecn_ack(0);
        init.set_sent_blocked_frame(0);
        init.set_stream_blocked_bidir_sent(0);
        init.set_stream_blocked_unidir_sent(0);
        init.set_max_stream_data_needed(0);
        init.set_path_demotion_needed(0);
        init.set_alt_path_challenge_needed(0);
        init.set_is_handshake_finished(0);
        init.set_is_handshake_done_acked(0);
        init.set_is_new_token_acked(0);
        init.set_is_1rtt_received(0);
        init.set_is_1rtt_acked(0);
        init.set_has_successful_probe(0);
        init.set_grease_transport_parameters(0);
        init.set_test_large_chello(0);
        init.set_initial_validated(0);
        init.set_initial_repeat_needed(0);
        init.set_is_loss_bit_enabled_incoming(0);
        init.set_is_loss_bit_enabled_outgoing(0);
        init.set_is_ack_frequency_negotiated(0);
        init.set_is_ack_frequency_updated(0);
        init.set_recycle_sooner_needed(0);
        init.set_is_time_stamp_enabled(0);
        init.set_is_time_stamp_sent(0);
        init.set_is_pacing_update_requested(0);
        init.set_is_path_quality_update_requested(0);
        init.set_is_hcid_verified(0);
        init.set_do_grease_quic_bit(0);
        init.set_quic_bit_greased(0);
        init.set_quic_bit_received_0(0);
        init.set_is_half_open(0);
        init.set_did_receive_short_initial(0);
        init.set_ack_ignore_order_local(0);
        init.set_ack_ignore_order_remote(0);
        init.set_are_path_callbacks_enabled(0);
        init.set_is_sending_large_buffer(0);
        init.set_is_preemptive_repeat_enabled(0);
        init.set_do_version_negotiation(0);
        init.set_send_receive_bdp_frame(0);
        init.set_cwin_notified_from_seed(0);
        init.set_is_datagram_ready(0);
        init.set_is_immediate_ack_required(0);
        init.set_is_multipath_enabled(0);
        init.set_is_lost_feedback_notification_required(0);
        init.set_is_forced_probe_up_required(0);
        init.set_is_address_discovery_provider(0);
        init.set_is_address_discovery_receiver(0);
        init.set_is_poll_requested(0);
        init.set_no_ack_delay(0);
        init.set_cwin_blocked(0);
        init.set_flow_blocked(0);
        init.set_stream_blocked(0);
        init
    };
    picoquic_store_addr(&raw mut dummy_cnx.registered_secret_addr, addr);
    memcpy(
        &raw mut dummy_cnx.registered_reset_secret as *mut uint8_t as *mut ::core::ffi::c_void,
        reset_secret as *const ::core::ffi::c_void,
        PICOQUIC_RESET_SECRET_SIZE as size_t,
    );
    item = picohash_retrieve(
        (*quic).table_cnx_by_secret,
        &raw mut dummy_cnx as *const ::core::ffi::c_void,
    );
    if !item.is_null() {
        ret = (*item).key as *mut picoquic_cnx_t;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_congestion_algorithm(
    mut alg_name: *const ::core::ffi::c_char,
) -> *const picoquic_congestion_algorithm_t {
    let mut alg: *const picoquic_congestion_algorithm_t =
        ::core::ptr::null::<picoquic_congestion_algorithm_t>();
    if !alg_name.is_null() {
        if strcmp(alg_name, b"reno\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            alg = picoquic_newreno_algorithm;
        } else if strcmp(alg_name, b"cubic\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            alg = picoquic_cubic_algorithm;
        } else if strcmp(alg_name, b"dcubic\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            alg = picoquic_dcubic_algorithm;
        } else if strcmp(alg_name, b"fast\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            alg = picoquic_fastcc_algorithm;
        } else if strcmp(alg_name, b"bbr\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            alg = picoquic_bbr_algorithm;
        } else if strcmp(alg_name, b"prague\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            alg = picoquic_prague_algorithm;
        } else if strcmp(alg_name, b"bbr1\0".as_ptr() as *const ::core::ffi::c_char)
            == 0 as ::core::ffi::c_int
        {
            alg = picoquic_bbr1_algorithm;
        } else {
            alg = ::core::ptr::null::<picoquic_congestion_algorithm_t>();
        }
    }
    return alg;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_congestion_algorithm(
    mut quic: *mut picoquic_quic_t,
    mut alg: *const picoquic_congestion_algorithm_t,
) {
    (*quic).default_congestion_alg = alg;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_congestion_algorithm_by_name(
    mut quic: *mut picoquic_quic_t,
    mut alg_name: *const ::core::ffi::c_char,
) {
    (*quic).default_congestion_alg = picoquic_get_congestion_algorithm(alg_name);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_optimistic_ack_policy(
    mut quic: *mut picoquic_quic_t,
    mut sequence_hole_pseudo_period: uint32_t,
) {
    (*quic).sequence_hole_pseudo_period = sequence_hole_pseudo_period;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_preemptive_repeat_policy(
    mut quic: *mut picoquic_quic_t,
    mut do_repeat: ::core::ffi::c_int,
) {
    (*quic).set_is_preemptive_repeat_enabled(
        (if do_repeat != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_preemptive_repeat_per_cnx(
    mut cnx: *mut picoquic_cnx_t,
    mut do_repeat: ::core::ffi::c_int,
) {
    (*cnx).set_is_preemptive_repeat_enabled(
        (if do_repeat != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_congestion_algorithm(
    mut cnx: *mut picoquic_cnx_t,
    mut alg: *const picoquic_congestion_algorithm_t,
) {
    if !(*cnx).congestion_alg.is_null() {
        if !(*cnx).path.is_null() {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i < (*cnx).nb_paths {
                (*(*cnx).congestion_alg)
                    .alg_delete
                    .expect("non-null function pointer")(
                    *(*cnx).path.offset(i as isize) as *mut picoquic_path_t,
                );
                i += 1;
            }
        }
    }
    (*cnx).congestion_alg = alg;
    if !(*cnx).congestion_alg.is_null() {
        if !(*cnx).path.is_null() {
            let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while i_0 < (*cnx).nb_paths {
                (*(*cnx).congestion_alg)
                    .alg_init
                    .expect("non-null function pointer")(
                    cnx as *mut picoquic_cnx_t,
                    *(*cnx).path.offset(i_0 as isize) as *mut picoquic_path_t,
                    picoquic_get_quic_time((*cnx).quic),
                );
                i_0 += 1;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_wifi_shadow_rtt(
    mut quic: *mut picoquic_quic_t,
    mut wifi_shadow_rtt: uint64_t,
) {
    (*quic).wifi_shadow_rtt = wifi_shadow_rtt;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_bbr_quantum_ratio(
    mut quic: *mut picoquic_quic_t,
    mut quantum_ratio: ::core::ffi::c_double,
) {
    (*quic).bbr_quantum_ratio = quantum_ratio;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_bbr_exp(
    mut quic: *mut picoquic_quic_t,
    mut exp: *mut bbr_exp,
) {
    (*quic).bbr_exp_flags = *exp;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_priority_limit_for_bypass(
    mut cnx: *mut picoquic_cnx_t,
    mut priority_limit: uint8_t,
) {
    (*cnx).priority_limit_for_bypass = priority_limit as uint64_t;
    if priority_limit as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        picoquic_update_pacing_parameters(
            &raw mut (*cnx).priority_bypass_pacing,
            PICOQUIC_PRIORITY_BYPASS_MAX_RATE as ::core::ffi::c_double,
            PICOQUIC_PRIORITY_BYPASS_QUANTUM as uint64_t,
            (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).send_mtu,
            (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).smoothed_rtt,
            ::core::ptr::null_mut::<picoquic_path_t>(),
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_feedback_loss_notification(
    mut cnx: *mut picoquic_cnx_t,
    mut should_notify: ::core::ffi::c_uint,
) {
    (*cnx).set_is_lost_feedback_notification_required(should_notify as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_request_forced_probe_up(
    mut cnx: *mut picoquic_cnx_t,
    mut request_forced_probe_up: ::core::ffi::c_uint,
) {
    (*cnx).set_is_forced_probe_up_required(request_forced_probe_up as ::core::ffi::c_uint);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_subscribe_pacing_rate_updates(
    mut cnx: *mut picoquic_cnx_t,
    mut decrease_threshold: uint64_t,
    mut increase_threshold: uint64_t,
) {
    (*cnx).pacing_decrease_threshold = decrease_threshold;
    (*cnx).pacing_increase_threshold = increase_threshold;
    (*cnx).set_is_pacing_update_requested(
        (decrease_threshold != UINT64_MAX as uint64_t
            || increase_threshold != UINT64_MAX as uint64_t) as ::core::ffi::c_int
            as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_pacing_rate(mut cnx: *mut picoquic_cnx_t) -> uint64_t {
    return (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
        .pacing
        .rate;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_cwin(mut cnx: *mut picoquic_cnx_t) -> uint64_t {
    return (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).cwin;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_rtt(mut cnx: *mut picoquic_cnx_t) -> uint64_t {
    return (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).smoothed_rtt;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_local_addr(
    mut cnx: *mut picoquic_cnx_t,
    mut addr: *mut sockaddr,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !cnx.is_null()
        && !(*(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).is_null()
        && (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
            .local_addr
            .ss_family as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
    {
        picoquic_store_addr(
            &raw mut (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).local_addr,
            addr,
        );
        ret = if (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
            .local_addr
            .ss_family as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            -(1 as ::core::ffi::c_int)
        } else {
            0 as ::core::ffi::c_int
        };
    } else {
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_enable_keep_alive(
    mut cnx: *mut picoquic_cnx_t,
    mut interval: uint64_t,
) {
    if interval == 0 as uint64_t {
        let mut idle_timeout: uint64_t = (*cnx).idle_timeout;
        if idle_timeout == 0 as uint64_t {
            idle_timeout = ((*cnx).local_parameters.max_idle_timeout as ::core::ffi::c_ulonglong)
                .wrapping_mul(1000 as ::core::ffi::c_ulonglong)
                as uint64_t;
        }
        if idle_timeout
            < (3 as uint64_t).wrapping_mul(
                (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).retransmit_timer,
            )
        {
            idle_timeout = (3 as uint64_t).wrapping_mul(
                (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).retransmit_timer,
            );
        }
        (*cnx).keep_alive_interval = idle_timeout.wrapping_div(2 as uint64_t);
    } else {
        (*cnx).keep_alive_interval = interval;
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_disable_keep_alive(mut cnx: *mut picoquic_cnx_t) {
    (*cnx).keep_alive_interval = 0 as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_verify_certificate_callback(
    mut quic: *mut picoquic_quic_t,
    mut cb: *mut ptls_verify_certificate_t,
    mut free_fn: picoquic_free_verify_certificate_ctx,
) {
    picoquic_dispose_verify_certificate_callback(quic);
    picoquic_tls_set_verify_certificate_callback(
        quic,
        cb as *mut st_ptls_verify_certificate_t,
        free_fn,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_client(mut cnx: *mut picoquic_cnx_t) -> ::core::ffi::c_int {
    return (*cnx).client_mode() as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_local_error(mut cnx: *mut picoquic_cnx_t) -> uint64_t {
    return (*cnx).local_error;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_remote_error(mut cnx: *mut picoquic_cnx_t) -> uint64_t {
    return (*cnx).remote_error;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_application_error(mut cnx: *mut picoquic_cnx_t) -> uint64_t {
    return (*cnx).remote_application_error;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_remote_stream_error(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
) -> uint64_t {
    let mut remote_error: uint64_t = 0 as uint64_t;
    let mut stream: *mut picoquic_stream_head_t = picoquic_find_stream(cnx, stream_id);
    if !stream.is_null() {
        remote_error = (*stream).remote_error;
    }
    return remote_error;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_data_sent(mut cnx: *mut picoquic_cnx_t) -> uint64_t {
    return (*cnx).data_sent;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_data_received(mut cnx: *mut picoquic_cnx_t) -> uint64_t {
    return (*cnx).data_received;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_client_authentication(
    mut quic: *mut picoquic_quic_t,
    mut client_authentication: ::core::ffi::c_int,
) {
    picoquic_tls_set_client_authentication(quic, client_authentication);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_enforce_client_only(
    mut quic: *mut picoquic_quic_t,
    mut do_enforce: ::core::ffi::c_int,
) {
    (*quic).set_enforce_client_only(
        (if do_enforce != 0 {
            1 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_version_upgrade(
    mut cnx: *mut picoquic_cnx_t,
    mut old_version_index: ::core::ffi::c_int,
    mut new_version_index: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if new_version_index == old_version_index {
        ret = 0 as ::core::ffi::c_int;
    } else if !picoquic_supported_versions[new_version_index as usize]
        .upgrade_from
        .is_null()
    {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while *picoquic_supported_versions[new_version_index as usize]
            .upgrade_from
            .offset(i as isize)
            != 0 as uint32_t
        {
            if !(*picoquic_supported_versions[new_version_index as usize]
                .upgrade_from
                .offset(i as isize)
                == picoquic_supported_versions[old_version_index as usize].version)
            {
                continue;
            }
            ret = 0 as ::core::ffi::c_int;
            if cnx.is_null() {
                continue;
            }
            (*cnx).version_index = new_version_index;
            picoquic_crypto_context_free(
                (&raw mut (*cnx).crypto_context as *mut picoquic_crypto_context_t)
                    .offset(picoquic_epoch_initial as ::core::ffi::c_int as isize)
                    as *mut picoquic_crypto_context_t,
            );
            ret = picoquic_setup_initial_traffic_keys(cnx);
            break;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_uniform_random(mut rnd_max: uint64_t) -> uint64_t {
    return picoquic_public_uniform_random(rnd_max);
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    picoquic_nb_supported_versions = (::core::mem::size_of::<[picoquic_version_parameters_t; 13]>()
        as size_t)
        .wrapping_div(::core::mem::size_of::<picoquic_version_parameters_t>() as size_t);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];

use ::c2rust_bitfields;
extern "C" {
    pub type st_ptls_iovec_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type st_ptls_buffer_t;
    pub type st_picoquic_unified_logging_t;
    pub type st_ptls_verify_certificate_t;
    fn picoquic_get_quic_time(quic: *mut picoquic_quic_t) -> uint64_t;
    fn picoquic_log_app_message(cnx: *mut picoquic_cnx_t, fmt: *const ::core::ffi::c_char, ...);
    fn picoquic_queue_misc_frame(
        cnx: *mut picoquic_cnx_t,
        bytes: *const uint8_t,
        length: size_t,
        is_pure_ack: ::core::ffi::c_int,
        pc: picoquic_packet_context_enum,
    ) -> ::core::ffi::c_int;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
    fn picosplay_find_previous(
        tree: *mut picosplay_tree_t,
        value: *mut ::core::ffi::c_void,
    ) -> *mut picosplay_node_t;
    fn picosplay_first(tree: *mut picosplay_tree_t) -> *mut picosplay_node_t;
    fn picosplay_next(node: *mut picosplay_node_t) -> *mut picosplay_node_t;
    fn picosplay_delete_hint(tree: *mut picosplay_tree_t, node: *mut picosplay_node_t);
    fn picoquic_compare_addr(
        expected: *const sockaddr,
        actual: *const sockaddr,
    ) -> ::core::ffi::c_int;
    fn picoquic_compare_ip_addr(
        expected: *const sockaddr,
        actual: *const sockaddr,
    ) -> ::core::ffi::c_int;
    fn picoquic_get_addr_port(addr: *const sockaddr) -> uint16_t;
    fn picoquic_store_addr(stored_addr: *mut sockaddr_storage, addr: *const sockaddr);
    fn picoquic_get_ip_addr(
        addr: *mut sockaddr,
        ip_addr: *mut *mut uint8_t,
        ip_addr_len: *mut uint8_t,
    );
    fn picoquic_addr_text(
        addr: *const sockaddr,
        text: *mut ::core::ffi::c_char,
        text_size: size_t,
    ) -> *const ::core::ffi::c_char;
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
    fn picoquic_frames_uint8_decode(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
        n: *mut uint8_t,
    ) -> *const uint8_t;
    fn picoquic_frames_uint16_decode(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
        n: *mut uint16_t,
    ) -> *const uint8_t;
    fn picoquic_frames_uint64_decode(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
        n: *mut uint64_t,
    ) -> *const uint8_t;
    fn picoquic_frames_length_data_skip(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
    ) -> *const uint8_t;
    fn picoquic_frames_varint_encode(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        n64: uint64_t,
    ) -> *mut uint8_t;
    fn picoquic_frames_uint8_encode(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        n: uint8_t,
    ) -> *mut uint8_t;
    fn picoquic_frames_uint16_encode(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        n: uint16_t,
    ) -> *mut uint8_t;
    fn picoquic_frames_uint64_encode(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        n: uint64_t,
    ) -> *mut uint8_t;
    fn picoquic_frames_length_data_encode(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        l: size_t,
        v: *const uint8_t,
    ) -> *mut uint8_t;
    fn picoquic_frames_cid_encode(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        cid: *const picoquic_connection_id_t,
    ) -> *mut uint8_t;
    fn picoquic_frames_charz_encode(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        s: *const ::core::ffi::c_char,
    ) -> *mut uint8_t;
    static picoquic_supported_versions: [picoquic_version_parameters_t; 0];
    fn picoquic_recycle_packet(quic: *mut picoquic_quic_t, packet: *mut picoquic_packet_t);
    fn picoquic_get_stored_ticket(
        quic: *mut picoquic_quic_t,
        sni: *const ::core::ffi::c_char,
        sni_length: uint16_t,
        alpn: *const ::core::ffi::c_char,
        alpn_length: uint16_t,
        version: uint32_t,
        need_unused: ::core::ffi::c_int,
        ticket_id: uint64_t,
    ) -> *mut picoquic_stored_ticket_t;
    fn picoquic_seed_ticket(cnx: *mut picoquic_cnx_t, path_x: *mut picoquic_path_t);
    fn picoquic_store_token(
        quic: *mut picoquic_quic_t,
        sni: *const ::core::ffi::c_char,
        sni_length: uint16_t,
        ip_addr: *const uint8_t,
        ip_addr_length: uint8_t,
        token: *const uint8_t,
        token_length: uint16_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_retrieve_issued_ticket(
        quic: *mut picoquic_quic_t,
        ticket_id: uint64_t,
    ) -> *mut picoquic_issued_ticket_t;
    fn picoquic_context_from_epoch(epoch: ::core::ffi::c_int) -> picoquic_packet_context_enum;
    fn picoquic_demote_path(
        cnx: *mut picoquic_cnx_t,
        path_index: ::core::ffi::c_int,
        current_time: uint64_t,
        reason: uint64_t,
        phrase: *const ::core::ffi::c_char,
    );
    fn picoquic_queue_retransmit_on_ack(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
        current_time: uint64_t,
    );
    fn picoquic_find_path_by_unique_id(
        cnx: *mut picoquic_cnx_t,
        unique_path_id: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_find_or_create_remote_cnxid_stash(
        cnx: *mut picoquic_cnx_t,
        unique_path_id: uint64_t,
        do_create: ::core::ffi::c_int,
    ) -> *mut picoquic_remote_cnxid_stash_t;
    fn picoquic_add_remote_cnxid_to_stash(
        cnx: *mut picoquic_cnx_t,
        remote_cnxid_stash: *mut picoquic_remote_cnxid_stash_t,
        retire_before_next: uint64_t,
        sequence: uint64_t,
        cid_length: uint8_t,
        cnxid_bytes: *const uint8_t,
        secret_bytes: *const uint8_t,
        pstashed: *mut *mut picoquic_remote_cnxid_t,
    ) -> uint64_t;
    fn picoquic_remove_cnxid_from_stash(
        cnx: *mut picoquic_cnx_t,
        remote_cnxid_stash: *mut picoquic_remote_cnxid_stash_t,
        removed: *mut picoquic_remote_cnxid_t,
        previous: *mut picoquic_remote_cnxid_t,
    ) -> *mut picoquic_remote_cnxid_t;
    fn picoquic_dereference_stashed_cnxid(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
        is_deleting_cnx: ::core::ffi::c_int,
    );
    fn picoquic_remove_not_before_cid(
        cnx: *mut picoquic_cnx_t,
        unique_path_id: uint64_t,
        not_before: uint64_t,
        current_time: uint64_t,
    ) -> uint64_t;
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
    fn picoquic_varint_encode(bytes: *mut uint8_t, max_bytes: size_t, n64: uint64_t) -> size_t;
    fn picoquic_varint_decode(
        bytes: *const uint8_t,
        max_bytes: size_t,
        n64: *mut uint64_t,
    ) -> size_t;
    fn picoquic_encode_varint_length(n64: uint64_t) -> size_t;
    fn picoquic_decode_varint_length(byte: uint8_t) -> size_t;
    fn picoquic_ready_state_transition(cnx: *mut picoquic_cnx_t, current_time: uint64_t);
    fn picoquic_get_ack_number(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
        pc: picoquic_packet_context_enum,
    ) -> uint64_t;
    fn picoquic_sack_select_ack_ranges(
        sack_list: *mut picoquic_sack_list_t,
        first_sack: *mut picoquic_sack_item_t,
        max_ranges: ::core::ffi::c_int,
        is_opportunistic: ::core::ffi::c_int,
        nb_sent_max: *mut ::core::ffi::c_int,
        nb_sent_max_skip: *mut ::core::ffi::c_int,
    );
    fn picoquic_update_sack_list(
        sack: *mut picoquic_sack_list_t,
        pn64_min: uint64_t,
        pn64_max: uint64_t,
        current_time: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_check_sack_list(
        sack: *mut picoquic_sack_list_t,
        pn64_min: uint64_t,
        pn64_max: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_process_ack_of_ack_range(
        first_sack: *mut picoquic_sack_list_t,
        previous: *mut picoquic_sack_item_t,
        start_of_range: uint64_t,
        end_of_range: uint64_t,
    ) -> *mut picoquic_sack_item_t;
    fn picoquic_sack_last_item(sack_list: *mut picoquic_sack_list_t) -> *mut picoquic_sack_item_t;
    fn picoquic_sack_previous_item(sack: *mut picoquic_sack_item_t) -> *mut picoquic_sack_item_t;
    fn picoquic_sack_list_is_empty(sack_list: *mut picoquic_sack_list_t) -> ::core::ffi::c_int;
    fn picoquic_sack_list_last(first_sack: *mut picoquic_sack_list_t) -> uint64_t;
    fn picoquic_sack_item_range_start(sack_item: *mut picoquic_sack_item_t) -> uint64_t;
    fn picoquic_sack_item_range_end(sack_item: *mut picoquic_sack_item_t) -> uint64_t;
    fn picoquic_sack_item_nb_times_sent(
        sack_item: *mut picoquic_sack_item_t,
        is_opportunistic: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_sack_item_record_sent(
        sack_list: *mut picoquic_sack_list_t,
        sack_item: *mut picoquic_sack_item_t,
        is_opportunistic: ::core::ffi::c_int,
    );
    fn picoquic_seed_bandwidth(
        cnx: *mut picoquic_cnx_t,
        rtt_min: uint64_t,
        cwin: uint64_t,
        ip_addr: *const uint8_t,
        ip_addr_length: uint8_t,
    );
    fn picoquic_update_path_rtt(
        cnx: *mut picoquic_cnx_t,
        old_path: *mut picoquic_path_t,
        path_x: *mut picoquic_path_t,
        epoch: ::core::ffi::c_int,
        send_time: uint64_t,
        current_time: uint64_t,
        ack_delay: uint64_t,
        time_stamp: uint64_t,
    );
    fn picoquic_create_stream(
        cnx: *mut picoquic_cnx_t,
        stream_id: uint64_t,
    ) -> *mut picoquic_stream_head_t;
    fn picoquic_remove_output_stream(cnx: *mut picoquic_cnx_t, stream: *mut picoquic_stream_head_t);
    fn picoquic_first_stream(cnx: *mut picoquic_cnx_t) -> *mut picoquic_stream_head_t;
    fn picoquic_next_stream(stream: *mut picoquic_stream_head_t) -> *mut picoquic_stream_head_t;
    fn picoquic_find_stream(
        cnx: *mut picoquic_cnx_t,
        stream_id: uint64_t,
    ) -> *mut picoquic_stream_head_t;
    fn picoquic_add_output_streams(
        cnx: *mut picoquic_cnx_t,
        old_limit: uint64_t,
        new_limit: uint64_t,
        is_bidir: ::core::ffi::c_uint,
    );
    fn picoquic_cc_increased_window(
        cnx: *mut picoquic_cnx_t,
        previous_window: uint64_t,
    ) -> uint64_t;
    fn picoquic_stream_data_node_alloc(
        quic: *mut picoquic_quic_t,
    ) -> *mut picoquic_stream_data_node_t;
    fn picoquic_delete_stream(cnx: *mut picoquic_cnx_t, stream: *mut picoquic_stream_head_t);
    fn picoquic_find_or_create_local_cnxid_list(
        cnx: *mut picoquic_cnx_t,
        unique_path_id: uint64_t,
        do_create: ::core::ffi::c_int,
    ) -> *mut picoquic_local_cnxid_list_t;
    fn picoquic_demote_local_cnxid_list(
        cnx: *mut picoquic_cnx_t,
        unique_path_id: uint64_t,
        reason: uint64_t,
        current_time: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_delete_local_cnxid_list(
        cnx: *mut picoquic_cnx_t,
        local_cnxid_list: *mut picoquic_local_cnxid_list_t,
    );
    fn picoquic_retire_local_cnxid(
        cnx: *mut picoquic_cnx_t,
        unique_path_id: uint64_t,
        sequence: uint64_t,
    );
    fn picoquic_queue_misc_or_dg_frame(
        cnx: *mut picoquic_cnx_t,
        first: *mut *mut picoquic_misc_frame_header_t,
        last: *mut *mut picoquic_misc_frame_header_t,
        bytes: *const uint8_t,
        length: size_t,
        is_pure_ack: ::core::ffi::c_int,
        pc: picoquic_packet_context_enum,
    ) -> ::core::ffi::c_int;
    fn picoquic_delete_misc_or_dg(
        first: *mut *mut picoquic_misc_frame_header_t,
        last: *mut *mut picoquic_misc_frame_header_t,
        frame: *mut picoquic_misc_frame_header_t,
    );
    fn picoquic_update_peer_addr(path_x: *mut picoquic_path_t, peer_addr: *const sockaddr);
    fn picoquic_create_cnxid_reset_secret(
        quic: *mut picoquic_quic_t,
        cnx_id: *mut picoquic_connection_id_t,
        reset_secret: *mut uint8_t,
    ) -> ::core::ffi::c_int;
}
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
pub type size_t = usize;
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
pub const picoquic_frame_type_datagram_l: C2Rust_Unnamed_0 = 49;
pub type picoquic_stream_data_buffer_argument_t = st_picoquic_stream_data_buffer_argument_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_stream_data_buffer_argument_t {
    pub bytes: *mut uint8_t,
    pub byte_index: size_t,
    pub byte_space: size_t,
    pub allowed_space: size_t,
    pub length: size_t,
    pub is_fin: ::core::ffi::c_int,
    pub is_still_active: ::core::ffi::c_int,
    pub app_buffer: *mut uint8_t,
}
pub const picoquic_frame_type_padding: C2Rust_Unnamed_0 = 0;
pub type picoquic_datagram_active_enum = ::core::ffi::c_uint;
pub const picoquic_datagram_active_this_path_and_others: picoquic_datagram_active_enum = 3;
pub const picoquic_datagram_active_this_path_only: picoquic_datagram_active_enum = 2;
pub const picoquic_datagram_active_any_path: picoquic_datagram_active_enum = 1;
pub const picoquic_datagram_not_active: picoquic_datagram_active_enum = 0;
pub type picoquic_datagram_buffer_argument_t = st_picoquic_datagram_buffer_argument_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_datagram_buffer_argument_t {
    pub cnx: *mut picoquic_cnx_t,
    pub path_x: *mut picoquic_path_t,
    pub bytes0: *mut uint8_t,
    pub bytes: *mut uint8_t,
    pub bytes_max: *mut uint8_t,
    pub after_data: *mut uint8_t,
    pub allowed_space: size_t,
    pub is_active: ::core::ffi::c_int,
    pub is_old_api: ::core::ffi::c_int,
    pub was_called: ::core::ffi::c_int,
}
pub const picoquic_frame_type_datagram: C2Rust_Unnamed_0 = 48;
pub type C2Rust_Unnamed_0 = ::core::ffi::c_uint;
pub const picoquic_frame_type_observed_address_v6: C2Rust_Unnamed_0 = 10453415;
pub const picoquic_frame_type_observed_address_v4: C2Rust_Unnamed_0 = 10453414;
pub const picoquic_frame_type_path_blocked: C2Rust_Unnamed_0 = 354585613;
pub const picoquic_frame_type_max_path_id: C2Rust_Unnamed_0 = 354585612;
pub const picoquic_frame_type_bdp: C2Rust_Unnamed_0 = 60377;
pub const picoquic_frame_type_path_available: C2Rust_Unnamed_0 = 354585608;
pub const picoquic_frame_type_path_backup: C2Rust_Unnamed_0 = 354585607;
pub const picoquic_frame_type_path_abandon: C2Rust_Unnamed_0 = 354585605;
pub const picoquic_frame_type_path_ack_ecn: C2Rust_Unnamed_0 = 354585601;
pub const picoquic_frame_type_path_ack: C2Rust_Unnamed_0 = 354585600;
pub const picoquic_frame_type_time_stamp: C2Rust_Unnamed_0 = 757;
pub const picoquic_frame_type_immediate_ack: C2Rust_Unnamed_0 = 31;
pub const picoquic_frame_type_ack_frequency: C2Rust_Unnamed_0 = 175;
pub const picoquic_frame_type_handshake_done: C2Rust_Unnamed_0 = 30;
pub const picoquic_frame_type_application_close: C2Rust_Unnamed_0 = 29;
pub const picoquic_frame_type_connection_close: C2Rust_Unnamed_0 = 28;
pub const picoquic_frame_type_path_response: C2Rust_Unnamed_0 = 27;
pub const picoquic_frame_type_path_challenge: C2Rust_Unnamed_0 = 26;
pub const picoquic_frame_type_path_retire_connection_id: C2Rust_Unnamed_0 = 354585610;
pub const picoquic_frame_type_retire_connection_id: C2Rust_Unnamed_0 = 25;
pub const picoquic_frame_type_path_new_connection_id: C2Rust_Unnamed_0 = 354585609;
pub const picoquic_frame_type_new_connection_id: C2Rust_Unnamed_0 = 24;
pub const picoquic_frame_type_streams_blocked_unidir: C2Rust_Unnamed_0 = 23;
pub const picoquic_frame_type_streams_blocked_bidir: C2Rust_Unnamed_0 = 22;
pub const picoquic_frame_type_stream_data_blocked: C2Rust_Unnamed_0 = 21;
pub const picoquic_frame_type_data_blocked: C2Rust_Unnamed_0 = 20;
pub const picoquic_frame_type_max_streams_unidir: C2Rust_Unnamed_0 = 19;
pub const picoquic_frame_type_max_streams_bidir: C2Rust_Unnamed_0 = 18;
pub const picoquic_frame_type_max_stream_data: C2Rust_Unnamed_0 = 17;
pub const picoquic_frame_type_max_data: C2Rust_Unnamed_0 = 16;
pub const picoquic_frame_type_stream_range_max: C2Rust_Unnamed_0 = 15;
pub const picoquic_frame_type_stream_range_min: C2Rust_Unnamed_0 = 8;
pub const picoquic_frame_type_new_token: C2Rust_Unnamed_0 = 7;
pub const picoquic_frame_type_crypto_hs: C2Rust_Unnamed_0 = 6;
pub const picoquic_frame_type_stop_sending: C2Rust_Unnamed_0 = 5;
pub const picoquic_frame_type_reset_stream: C2Rust_Unnamed_0 = 4;
pub const picoquic_frame_type_ack_ecn: C2Rust_Unnamed_0 = 3;
pub const picoquic_frame_type_ack: C2Rust_Unnamed_0 = 2;
pub const picoquic_frame_type_poll: C2Rust_Unnamed_0 = 32;
pub const picoquic_frame_type_ping: C2Rust_Unnamed_0 = 1;
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
pub type C2Rust_Unnamed_1 = ::core::ffi::c_uint;
pub const picoquic_epoch_1rtt: C2Rust_Unnamed_1 = 3;
pub const picoquic_epoch_handshake: C2Rust_Unnamed_1 = 2;
pub const picoquic_epoch_0rtt: C2Rust_Unnamed_1 = 1;
pub const picoquic_epoch_initial: C2Rust_Unnamed_1 = 0;
pub type C2Rust_Unnamed_2 = ::core::ffi::c_uint;
pub const picoquic_tp_0rtt_cwin_remote: C2Rust_Unnamed_2 = 9;
pub const picoquic_tp_0rtt_rtt_remote: C2Rust_Unnamed_2 = 8;
pub const picoquic_tp_0rtt_cwin_local: C2Rust_Unnamed_2 = 7;
pub const picoquic_tp_0rtt_rtt_local: C2Rust_Unnamed_2 = 6;
pub const picoquic_tp_0rtt_max_streams_id_unidir: C2Rust_Unnamed_2 = 5;
pub const picoquic_tp_0rtt_max_streams_id_bidir: C2Rust_Unnamed_2 = 4;
pub const picoquic_tp_0rtt_max_stream_data_uni: C2Rust_Unnamed_2 = 3;
pub const picoquic_tp_0rtt_max_stream_data_bidi_remote: C2Rust_Unnamed_2 = 2;
pub const picoquic_tp_0rtt_max_stream_data_bidi_local: C2Rust_Unnamed_2 = 1;
pub const picoquic_tp_0rtt_max_data: C2Rust_Unnamed_2 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_sack_item_t {
    pub node: picosplay_node_t,
    pub start_of_sack_range: uint64_t,
    pub end_of_sack_range: uint64_t,
    pub time_created: uint64_t,
    pub nb_times_sent: [::core::ffi::c_int; 2],
}
pub type picoquic_sack_item_t = st_picoquic_sack_item_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_packet_data_t {
    pub last_time_stamp_received: uint64_t,
    pub last_ack_delay: uint64_t,
    pub nb_path_ack: ::core::ffi::c_int,
    pub path_ack: [C2Rust_Unnamed_3; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub acked_path: *mut picoquic_path_t,
    pub largest_sent_time: uint64_t,
    pub delivered_prior: uint64_t,
    pub delivered_time_prior: uint64_t,
    pub delivered_sent_prior: uint64_t,
    pub lost_prior: uint64_t,
    pub inflight_prior: uint64_t,
    pub rs_is_path_limited: ::core::ffi::c_uint,
    pub rs_is_cwnd_limited: ::core::ffi::c_uint,
    pub is_set: ::core::ffi::c_uint,
    pub data_acked: uint64_t,
}
pub type picoquic_packet_data_t = st_picoquic_packet_data_t;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const PF_UNSPEC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_UNSPEC: ::core::ffi::c_int = PF_UNSPEC;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const PICOQUIC_ERROR_CLASS: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_MEMORY: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 5 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_FRAME_BUFFER_TOO_SMALL: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 16 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_DETECTED: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 21 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_UNEXPECTED_ERROR: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 27 as ::core::ffi::c_int;
pub const PICOQUIC_STREAM_RECEIVE_COMPLETE: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 44 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_DATAGRAM_TOO_LONG: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 59 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_INTERNAL_ERROR: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_FLOW_CONTROL_ERROR: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_STREAM_LIMIT_ERROR: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_STREAM_STATE_ERROR: ::core::ffi::c_int = 0x5 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_FINAL_OFFSET_ERROR: ::core::ffi::c_int = 0x6 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR: ::core::ffi::c_int = 0x7 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION: ::core::ffi::c_int = 0xa as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_CRYPTO_BUFFER_EXCEEDED: ::core::ffi::c_int = 0xd as ::core::ffi::c_int;
pub const PICOQUIC_MAX_PACKET_SIZE: ::core::ffi::c_int = 1536 as ::core::ffi::c_int;
pub const PICOQUIC_RESET_SECRET_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PICOQUIC_MAX_CRYPTO_BUFFER_GAP: ::core::ffi::c_int = 16384 as ::core::ffi::c_int;
pub const PICOQUIC_CONNECTION_ID_MAX_SIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const PICOQUIC_DATAGRAM_QUEUE_MAX_LENGTH: ::core::ffi::c_int = 1200 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const PICOQUIC_NB_PATH_TARGET: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PICOQUIC_TARGET_RENO_RTT: ::core::ffi::c_ulonglong = 100000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_ACK_DELAY_MAX: ::core::ffi::c_ulonglong = 100000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_ACK_DELAY_MIN: ::core::ffi::c_ulonglong = 1000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_BANDWIDTH_TIME_INTERVAL_MIN: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const PICOQUIC_BANDWIDTH_MEDIUM: ::core::ffi::c_int = 2000000 as ::core::ffi::c_int;
pub const PICOQUIC_MAX_BANDWIDTH_TIME_INTERVAL_MIN: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const PICOQUIC_SPURIOUS_RETRANSMIT_DELAY_MAX: ::core::ffi::c_ulonglong =
    1000000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_CHALLENGE_REPEAT_MAX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PICOQUIC_CC_ALGO_NUMBER_NEW_RENO: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PICOQUIC_CC_ALGO_NUMBER_FAST: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
static mut challenge_length: size_t = 8 as size_t;
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_missing_streams(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut is_remote: ::core::ffi::c_int,
) -> *mut picoquic_stream_head_t {
    let mut stream: *mut picoquic_stream_head_t = ::core::ptr::null_mut::<picoquic_stream_head_t>();
    let mut expect_client_stream: ::core::ffi::c_uint =
        ((*cnx).client_mode() as ::core::ffi::c_int ^ is_remote) as ::core::ffi::c_uint;
    if is_remote != 0 && stream_id < (*cnx).next_stream_id[(stream_id & 3 as uint64_t) as usize] {
        return ::core::ptr::null_mut::<picoquic_stream_head_t>();
    } else if (stream_id & 1 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
        as ::core::ffi::c_uint
        != expect_client_stream
    {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_STREAM_LIMIT_ERROR as uint64_t,
            0 as uint64_t,
        );
    } else if is_remote != 0
        && stream_id
            > (if (stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
                as ::core::ffi::c_uint
                != 0
            {
                (*cnx).max_stream_id_bidir_local
            } else {
                (*cnx).max_stream_id_unidir_local
            })
    {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_STREAM_LIMIT_ERROR as uint64_t,
            0 as uint64_t,
        );
    } else if stream_id < (*cnx).next_stream_id[(stream_id & 3 as uint64_t) as usize] {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_STREAM_STATE_ERROR as uint64_t,
            0 as uint64_t,
        );
    } else {
        while stream_id >= (*cnx).next_stream_id[(stream_id & 3 as uint64_t) as usize] {
            stream = picoquic_create_stream(
                cnx,
                (*cnx).next_stream_id[(stream_id & 3 as uint64_t) as usize],
            );
            if stream.is_null() {
                picoquic_log_app_message(
                    cnx as *mut picoquic_cnx_t,
                    b"Create stream %lu returns error 0x%x\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    stream_id,
                    PICOQUIC_TRANSPORT_INTERNAL_ERROR,
                );
                picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t,
                    0 as uint64_t,
                );
                break;
            } else if (stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
                as ::core::ffi::c_uint
                == 0
            {
                if ((stream_id ^ (*cnx).client_mode() as uint64_t) & 1 as uint64_t)
                    as ::core::ffi::c_uint
                    == 0
                {
                    (*stream).set_fin_requested(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*stream).set_fin_sent(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
            }
        }
    }
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_stream_closed(
    mut stream: *mut picoquic_stream_head_t,
    mut client_mode: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut is_closed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if ((*stream).stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
        as ::core::ffi::c_uint
        != 0
    {
        is_closed = (((*stream).fin_requested() as ::core::ffi::c_int != 0
            && (*stream).fin_sent() as ::core::ffi::c_int != 0
            || (*stream).reset_requested() as ::core::ffi::c_int != 0
                && (*stream).reset_sent() as ::core::ffi::c_int != 0)
            && ((*stream).fin_received() as ::core::ffi::c_int != 0
                && (*stream).fin_signalled() as ::core::ffi::c_int != 0
                || (*stream).reset_received() as ::core::ffi::c_int != 0
                    && (*stream).reset_signalled() as ::core::ffi::c_int != 0))
            as ::core::ffi::c_int;
    } else if (((*stream).stream_id ^ client_mode as uint64_t) & 1 as uint64_t)
        as ::core::ffi::c_uint
        != 0
    {
        is_closed = ((*stream).fin_requested() as ::core::ffi::c_int != 0
            && (*stream).fin_sent() as ::core::ffi::c_int != 0
            || (*stream).reset_requested() as ::core::ffi::c_int != 0
                && (*stream).reset_sent() as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int;
    } else {
        is_closed = ((*stream).fin_received() as ::core::ffi::c_int != 0
            && (*stream).fin_signalled() as ::core::ffi::c_int != 0
            || (*stream).reset_received() as ::core::ffi::c_int != 0
                && (*stream).reset_signalled() as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int;
    }
    return is_closed;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_stream_acked(
    mut stream: *mut picoquic_stream_head_t,
) -> ::core::ffi::c_int {
    let mut is_acked: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*stream).is_closed() != 0 {
        if (*stream).reset_sent() != 0 {
            is_acked = (*stream).reset_acked() as ::core::ffi::c_int;
        } else {
            is_acked = picoquic_check_sack_list(
                &raw mut (*stream).sack_list,
                0 as uint64_t,
                (*stream).sent_offset,
            );
        }
    }
    return is_acked;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_stream_if_closed(
    mut cnx: *mut picoquic_cnx_t,
    mut stream: *mut picoquic_stream_head_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*stream).is_closed() == 0
        && picoquic_is_stream_closed(stream, (*cnx).client_mode() as ::core::ffi::c_int) != 0
    {
        picoquic_update_max_stream_ID_local(cnx, stream);
        (*stream).set_is_closed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        ret = 1 as ::core::ffi::c_int;
    }
    if (*stream).is_closed() as ::core::ffi::c_int != 0 && picoquic_is_stream_acked(stream) != 0 {
        picoquic_delete_stream(cnx, stream);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_update_stream_initial_remote(mut cnx: *mut picoquic_cnx_t) {
    let mut stream: *mut picoquic_stream_head_t = picoquic_first_stream(cnx);
    while !stream.is_null() {
        if (((*stream).stream_id ^ (*cnx).client_mode() as uint64_t) & 1 as uint64_t)
            as ::core::ffi::c_uint
            != 0
        {
            if ((*stream).stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
                as ::core::ffi::c_uint
                != 0
            {
                if (*stream).maxdata_remote
                    < (*cnx).remote_parameters.initial_max_stream_data_bidi_remote
                {
                    (*stream).maxdata_remote =
                        (*cnx).remote_parameters.initial_max_stream_data_bidi_remote;
                }
            } else if (*stream).maxdata_remote
                < (*cnx).remote_parameters.initial_max_stream_data_uni
            {
                (*stream).maxdata_remote = (*cnx).remote_parameters.initial_max_stream_data_uni;
            }
        } else if ((*stream).stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
            as ::core::ffi::c_uint
            != 0
        {
            if (*stream).maxdata_remote
                < (*cnx).remote_parameters.initial_max_stream_data_bidi_local
            {
                (*stream).maxdata_remote =
                    (*cnx).remote_parameters.initial_max_stream_data_bidi_local;
            }
        }
        stream = picoquic_next_stream(stream);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_find_or_create_stream(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut is_remote: ::core::ffi::c_int,
) -> *mut picoquic_stream_head_t {
    let mut stream: *mut picoquic_stream_head_t = picoquic_find_stream(cnx, stream_id);
    if stream.is_null() {
        stream = picoquic_create_missing_streams(cnx, stream_id, is_remote);
    }
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_flow_control_check_stream_offset(
    mut cnx: *mut picoquic_cnx_t,
    mut stream: *mut picoquic_stream_head_t,
    mut new_fin_offset: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if new_fin_offset > (*stream).maxdata_local {
        ret = picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FLOW_CONTROL_ERROR as uint64_t,
            0 as uint64_t,
        );
    } else if new_fin_offset > (*stream).fin_offset {
        let mut new_bytes: uint64_t = new_fin_offset.wrapping_sub((*stream).fin_offset);
        if new_bytes > (*cnx).maxdata_local
            || (*cnx).maxdata_local.wrapping_sub(new_bytes) < (*cnx).data_received
        {
            ret = picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_FLOW_CONTROL_ERROR as uint64_t,
                0 as uint64_t,
            );
        } else {
            (*cnx).data_received = (*cnx).data_received.wrapping_add(new_bytes);
            (*stream).fin_offset = new_fin_offset;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_stream_reset_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut stream: *mut picoquic_stream_head_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    if (*stream).reset_requested() as ::core::ffi::c_int != 0 && (*stream).reset_sent() == 0 {
        bytes = picoquic_frames_uint8_encode(
            bytes,
            bytes_max,
            picoquic_frame_type_reset_stream as ::core::ffi::c_int as uint8_t,
        );
        if !bytes.is_null()
            && {
                bytes = picoquic_frames_varint_encode(bytes, bytes_max, (*stream).stream_id);
                !bytes.is_null()
            }
            && {
                bytes = picoquic_frames_varint_encode(bytes, bytes_max, (*stream).local_error);
                !bytes.is_null()
            }
            && {
                bytes = picoquic_frames_varint_encode(bytes, bytes_max, (*stream).sent_offset);
                !bytes.is_null()
            }
        {
            *is_pure_ack = 0 as ::core::ffi::c_int;
            (*stream).set_reset_sent(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*stream).set_fin_sent(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            picoquic_update_max_stream_ID_local(cnx, stream);
            while !(*stream).send_queue.is_null() {
                let mut next: *mut picoquic_stream_queue_node_t =
                    (*(*stream).send_queue).next_stream_data as *mut picoquic_stream_queue_node_t;
                if !(*(*stream).send_queue).bytes.is_null() {
                    free((*(*stream).send_queue).bytes as *mut ::core::ffi::c_void);
                }
                free((*stream).send_queue as *mut ::core::ffi::c_void);
                (*stream).send_queue = next;
            }
            picoquic_delete_stream_if_closed(cnx, stream);
        } else {
            *more_data = 1 as ::core::ffi::c_int;
            bytes = bytes0;
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_stream_reset_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut stream_id: uint64_t = 0 as uint64_t;
    let mut error_code_64: uint64_t = 0 as uint64_t;
    let mut final_offset: uint64_t = 0 as uint64_t;
    let mut stream: *mut picoquic_stream_head_t = ::core::ptr::null_mut::<picoquic_stream_head_t>();
    bytes = picoquic_frames_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max,
        &raw mut stream_id,
    );
    if !bytes.is_null() {
        bytes = picoquic_frames_varint_decode(bytes, bytes_max, &raw mut error_code_64);
        if !bytes.is_null() {
            bytes = picoquic_frames_varint_decode(bytes, bytes_max, &raw mut final_offset);
        }
    }
    if bytes.is_null() {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_reset_stream as ::core::ffi::c_int as uint64_t,
        );
    } else {
        stream = picoquic_find_or_create_stream(cnx, stream_id, 1 as ::core::ffi::c_int);
        if stream.is_null() {
            if (*cnx).cnx_state as ::core::ffi::c_uint
                > picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                bytes = ::core::ptr::null::<uint8_t>();
            }
        } else if ((*stream).fin_received() as ::core::ffi::c_int != 0
            || (*stream).reset_received() as ::core::ffi::c_int != 0)
            && final_offset != (*stream).fin_offset
        {
            picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_FINAL_OFFSET_ERROR as uint64_t,
                picoquic_frame_type_reset_stream as ::core::ffi::c_int as uint64_t,
            );
            bytes = ::core::ptr::null::<uint8_t>();
        } else if picoquic_flow_control_check_stream_offset(cnx, stream, final_offset)
            != 0 as ::core::ffi::c_int
        {
            bytes = ::core::ptr::null::<uint8_t>();
        } else if (*stream).reset_received() == 0 {
            (*stream).set_reset_received(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*stream).remote_error = error_code_64;
            picoquic_update_max_stream_ID_local(cnx, stream);
            if (*cnx).callback_fn.is_some() && (*stream).reset_signalled() == 0 {
                if (*stream).is_discarded() == 0 {
                    if (*cnx).callback_fn.expect("non-null function pointer")(
                        cnx as *mut picoquic_cnx_t,
                        (*stream).stream_id,
                        ::core::ptr::null_mut::<uint8_t>(),
                        0 as size_t,
                        picoquic_callback_stream_reset,
                        (*cnx).callback_ctx,
                        (*stream).app_stream_ctx,
                    ) != 0 as ::core::ffi::c_int
                    {
                        picoquic_connection_error(
                            cnx,
                            PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t,
                            picoquic_frame_type_reset_stream as ::core::ffi::c_int as uint64_t,
                        );
                    }
                }
                (*stream).set_reset_signalled(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                picoquic_delete_stream_if_closed(cnx, stream);
            }
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_ack_of_reset_stream_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_size: size_t,
    mut consumed: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut byte_first: *const uint8_t = bytes;
    let mut bytes_max: *const uint8_t = bytes.offset(bytes_size as isize);
    let mut stream_id: uint64_t = 0 as uint64_t;
    let mut stream: *mut picoquic_stream_head_t = ::core::ptr::null_mut::<picoquic_stream_head_t>();
    bytes = picoquic_frames_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max,
        &raw mut stream_id,
    );
    if !bytes.is_null() {
        bytes = picoquic_frames_varint_skip(bytes, bytes_max);
        if !bytes.is_null() {
            bytes = picoquic_frames_varint_skip(bytes, bytes_max);
        }
    }
    if bytes.is_null() {
        *consumed = bytes_size;
        ret = -(1 as ::core::ffi::c_int);
    } else {
        *consumed = bytes.offset_from(byte_first) as ::core::ffi::c_long as size_t;
        stream = picoquic_find_stream(cnx, stream_id);
        if !stream.is_null() {
            (*stream).set_reset_acked(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            picoquic_delete_stream_if_closed(cnx, stream);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_check_reset_stream_needs_repeat(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_size: size_t,
    mut no_need_to_repeat: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bytes_max: *const uint8_t = bytes.offset(bytes_size as isize);
    let mut stream_id: uint64_t = 0 as uint64_t;
    let mut stream: *mut picoquic_stream_head_t = ::core::ptr::null_mut::<picoquic_stream_head_t>();
    bytes = picoquic_frames_varint_decode(bytes, bytes_max, &raw mut stream_id);
    if !bytes.is_null() {
        bytes = picoquic_frames_varint_skip(bytes, bytes_max);
        if !bytes.is_null() {
            bytes = picoquic_frames_varint_skip(bytes, bytes_max);
        }
    }
    if bytes.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        stream = picoquic_find_stream(cnx, stream_id);
        if stream.is_null() || (*stream).reset_acked() as ::core::ffi::c_int != 0 {
            *no_need_to_repeat = 1 as ::core::ffi::c_int;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_new_connection_id_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut local_cnxid_list: *mut picoquic_local_cnxid_list_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut l_cid: *mut picoquic_local_cnxid_t,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    let mut is_mp: ::core::ffi::c_uint = (*cnx).is_multipath_enabled();
    if !l_cid.is_null() && (*l_cid).cnx_id.id_len as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        bytes = picoquic_frames_varint_encode(
            bytes,
            bytes_max,
            (if is_mp != 0 {
                picoquic_frame_type_path_new_connection_id as ::core::ffi::c_int
            } else {
                picoquic_frame_type_new_connection_id as ::core::ffi::c_int
            }) as uint64_t,
        );
        if bytes.is_null()
            || is_mp != 0 && {
                bytes = picoquic_frames_varint_encode(bytes, bytes_max, (*l_cid).path_id);
                bytes.is_null()
            }
            || {
                bytes = picoquic_frames_varint_encode(bytes, bytes_max, (*l_cid).sequence);
                bytes.is_null()
            }
            || {
                bytes = picoquic_frames_varint_encode(
                    bytes,
                    bytes_max,
                    (*local_cnxid_list).local_cnxid_retire_before,
                );
                bytes.is_null()
            }
            || {
                bytes = picoquic_frames_cid_encode(bytes, bytes_max, &raw mut (*l_cid).cnx_id);
                bytes.is_null()
            }
            || bytes.offset(PICOQUIC_RESET_SECRET_SIZE as isize) > bytes_max
        {
            *more_data = 1 as ::core::ffi::c_int;
            bytes = bytes0;
        } else {
            *is_pure_ack = 0 as ::core::ffi::c_int;
            picoquic_create_cnxid_reset_secret(
                (*cnx).quic,
                &raw mut (*l_cid).cnx_id,
                bytes as *mut uint8_t,
            );
            bytes = bytes.offset(PICOQUIC_RESET_SECRET_SIZE as isize);
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_skip_new_connection_id_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut is_mp: ::core::ffi::c_int,
) -> *const uint8_t {
    let mut cid_length: uint8_t = 0 as uint8_t;
    bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    if !bytes.is_null()
        && (is_mp == 0 || {
            bytes = picoquic_frames_varint_skip(bytes, bytes_max);
            !bytes.is_null()
        })
        && {
            bytes = picoquic_frames_varint_skip(bytes, bytes_max);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_varint_skip(bytes, bytes_max);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_uint8_decode(bytes, bytes_max, &raw mut cid_length);
            !bytes.is_null()
        }
    {
        bytes = picoquic_frames_fixed_skip(
            bytes,
            bytes_max,
            (cid_length as uint64_t).wrapping_add(PICOQUIC_RESET_SECRET_SIZE as uint64_t),
        );
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_new_connection_id_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut is_mp: ::core::ffi::c_int,
    mut path_id: *mut uint64_t,
    mut sequence: *mut uint64_t,
    mut retire_before: *mut uint64_t,
    mut cid_length: *mut uint8_t,
    mut cnxid_bytes: *mut *const uint8_t,
    mut secret_bytes: *mut *const uint8_t,
) -> *const uint8_t {
    *path_id = 0 as uint64_t;
    bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    if !bytes.is_null() {
        if is_mp != 0 {
            bytes = picoquic_frames_varint_decode(bytes, bytes_max, path_id);
        }
    }
    if !bytes.is_null()
        && {
            bytes = picoquic_frames_varint_decode(bytes, bytes_max, sequence);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_varint_decode(bytes, bytes_max, retire_before);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_uint8_decode(bytes, bytes_max, cid_length);
            !bytes.is_null()
        }
    {
        *cnxid_bytes = bytes;
        *secret_bytes = bytes.offset(*cid_length as ::core::ffi::c_int as isize);
        bytes = picoquic_frames_fixed_skip(
            bytes,
            bytes_max,
            (*cid_length as uint64_t).wrapping_add(PICOQUIC_RESET_SECRET_SIZE as uint64_t),
        );
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_new_connection_id_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut current_time: uint64_t,
    mut is_mp: ::core::ffi::c_int,
) -> *const uint8_t {
    let mut unique_path_id: uint64_t = 0 as uint64_t;
    let mut sequence: uint64_t = 0 as uint64_t;
    let mut retire_before: uint64_t = 0 as uint64_t;
    let mut cid_length: uint8_t = 0 as uint8_t;
    let mut cnxid_bytes: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut secret_bytes: *const uint8_t = ::core::ptr::null::<uint8_t>();
    if is_mp != 0 && (*cnx).is_multipath_enabled() == 0 {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
            picoquic_frame_type_path_new_connection_id as ::core::ffi::c_int as uint64_t,
        );
        bytes = ::core::ptr::null::<uint8_t>();
    } else {
        bytes = picoquic_parse_new_connection_id_frame(
            bytes,
            bytes_max,
            is_mp,
            &raw mut unique_path_id,
            &raw mut sequence,
            &raw mut retire_before,
            &raw mut cid_length,
            &raw mut cnxid_bytes,
            &raw mut secret_bytes,
        );
    }
    if bytes.is_null() || retire_before > sequence {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_new_connection_id as ::core::ffi::c_int as uint64_t,
        );
        bytes = ::core::ptr::null::<uint8_t>();
    } else if cid_length as ::core::ffi::c_int > PICOQUIC_CONNECTION_ID_MAX_SIZE {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
            picoquic_frame_type_new_connection_id as ::core::ffi::c_int as uint64_t,
        );
        bytes = ::core::ptr::null::<uint8_t>();
    } else if unique_path_id > (*cnx).max_path_id_local
        && (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
    {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
            (if is_mp != 0 {
                picoquic_frame_type_path_new_connection_id as ::core::ffi::c_int
            } else {
                picoquic_frame_type_new_connection_id as ::core::ffi::c_int
            }) as uint64_t,
        );
        bytes = ::core::ptr::null::<uint8_t>();
    } else {
        let mut remote_cnxid_stash: *mut picoquic_remote_cnxid_stash_t =
            picoquic_find_or_create_remote_cnxid_stash(
                cnx,
                unique_path_id,
                1 as ::core::ffi::c_int,
            );
        if remote_cnxid_stash.is_null() {
            picoquic_connection_error_ex(
                cnx,
                PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t,
                picoquic_frame_type_new_connection_id as ::core::ffi::c_int as uint64_t,
                b"Find or Create CNXID\0".as_ptr() as *const ::core::ffi::c_char,
            );
            bytes = ::core::ptr::null::<uint8_t>();
        } else {
            let mut transport_error: uint64_t = picoquic_add_remote_cnxid_to_stash(
                cnx,
                remote_cnxid_stash,
                retire_before,
                sequence,
                cid_length,
                cnxid_bytes,
                secret_bytes,
                ::core::ptr::null_mut::<*mut picoquic_remote_cnxid_t>(),
            );
            if transport_error == 0 as uint64_t
                && (*remote_cnxid_stash).retire_cnxid_before < retire_before
            {
                (*remote_cnxid_stash).retire_cnxid_before = retire_before;
                transport_error = picoquic_remove_not_before_cid(
                    cnx,
                    unique_path_id,
                    retire_before,
                    current_time,
                );
            }
            if transport_error != 0 as uint64_t {
                picoquic_connection_error(
                    cnx,
                    transport_error,
                    (if is_mp != 0 {
                        picoquic_frame_type_path_new_connection_id as ::core::ffi::c_int
                    } else {
                        picoquic_frame_type_new_connection_id as ::core::ffi::c_int
                    }) as uint64_t,
                );
                bytes = ::core::ptr::null::<uint8_t>();
            }
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_ack_of_new_cid_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut is_mp: ::core::ffi::c_int,
    mut consumed: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut unique_path_id: uint64_t = 0 as uint64_t;
    let mut sequence: uint64_t = 0 as uint64_t;
    let mut retire_before: uint64_t = 0 as uint64_t;
    let mut cid_length: uint8_t = 0 as uint8_t;
    let mut cnxid_bytes: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut secret_bytes: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut bytes_next: *const uint8_t = picoquic_parse_new_connection_id_frame(
        bytes,
        bytes.offset(bytes_max as isize),
        is_mp,
        &raw mut unique_path_id,
        &raw mut sequence,
        &raw mut retire_before,
        &raw mut cid_length,
        &raw mut cnxid_bytes,
        &raw mut secret_bytes,
    );
    if !bytes_next.is_null() {
        let mut local_cnxid_list: *mut picoquic_local_cnxid_list_t =
            ::core::ptr::null_mut::<picoquic_local_cnxid_list_t>();
        *consumed = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
        local_cnxid_list =
            picoquic_find_or_create_local_cnxid_list(cnx, unique_path_id, 0 as ::core::ffi::c_int);
        if !local_cnxid_list.is_null() {
            let mut local_cnxid: *mut picoquic_local_cnxid_t =
                (*local_cnxid_list).local_cnxid_first;
            while !local_cnxid.is_null() {
                if (*local_cnxid).sequence == sequence {
                    (*local_cnxid).is_acked = 1 as ::core::ffi::c_uint;
                    break;
                } else {
                    local_cnxid = (*local_cnxid).next as *mut picoquic_local_cnxid_t;
                }
            }
        }
    } else {
        *consumed = bytes_max;
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_check_new_cid_needs_repeat(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut is_mp: ::core::ffi::c_int,
    mut no_need_to_repeat: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut unique_path_id: uint64_t = 0 as uint64_t;
    let mut sequence: uint64_t = 0 as uint64_t;
    let mut retire_before: uint64_t = 0 as uint64_t;
    let mut cid_length: uint8_t = 0 as uint8_t;
    let mut cnxid_bytes: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut secret_bytes: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut bytes_next: *const uint8_t = picoquic_parse_new_connection_id_frame(
        bytes,
        bytes.offset(bytes_max as isize),
        is_mp,
        &raw mut unique_path_id,
        &raw mut sequence,
        &raw mut retire_before,
        &raw mut cid_length,
        &raw mut cnxid_bytes,
        &raw mut secret_bytes,
    );
    *no_need_to_repeat = 1 as ::core::ffi::c_int;
    if bytes_next.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        let mut local_cnxid_list: *mut picoquic_local_cnxid_list_t =
            picoquic_find_or_create_local_cnxid_list(cnx, unique_path_id, 0 as ::core::ffi::c_int);
        if !local_cnxid_list.is_null() {
            let mut local_cnxid: *mut picoquic_local_cnxid_t =
                (*local_cnxid_list).local_cnxid_first;
            while !local_cnxid.is_null() {
                if (*local_cnxid).sequence == sequence {
                    *no_need_to_repeat = (*local_cnxid).is_acked as ::core::ffi::c_int;
                    break;
                } else {
                    local_cnxid = (*local_cnxid).next as *mut picoquic_local_cnxid_t;
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_retire_connection_id_frame(
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut is_mp: ::core::ffi::c_int,
    mut unique_path_id: uint64_t,
    mut sequence: uint64_t,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_varint_encode(
        bytes,
        bytes_max,
        (if is_mp != 0 {
            picoquic_frame_type_path_retire_connection_id as ::core::ffi::c_int
        } else {
            picoquic_frame_type_retire_connection_id as ::core::ffi::c_int
        }) as uint64_t,
    );
    if bytes.is_null()
        || is_mp != 0 && {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, unique_path_id);
            bytes.is_null()
        }
        || {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, sequence);
            bytes.is_null()
        }
    {
        bytes = bytes0;
        *more_data = 1 as ::core::ffi::c_int;
    } else {
        *is_pure_ack = 0 as ::core::ffi::c_int;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_retire_connection_id_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut sequence: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut consumed: size_t = 0 as size_t;
    let mut frame_buffer: [uint8_t; 258] = [0; 258];
    let mut is_pure_ack: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut more_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bytes_next: *mut uint8_t = picoquic_format_retire_connection_id_frame(
        &raw mut frame_buffer as *mut uint8_t,
        (&raw mut frame_buffer as *mut uint8_t)
            .offset(::core::mem::size_of::<[uint8_t; 258]>() as usize as isize),
        &raw mut more_data,
        &raw mut is_pure_ack,
        (*cnx).is_multipath_enabled() as ::core::ffi::c_int,
        unique_path_id,
        sequence,
    );
    consumed = bytes_next.offset_from(&raw mut frame_buffer as *mut uint8_t) as ::core::ffi::c_long
        as size_t;
    if consumed > 0 as size_t {
        ret = picoquic_queue_misc_frame(
            cnx as *mut picoquic_cnx_t,
            &raw mut frame_buffer as *mut uint8_t,
            consumed,
            is_pure_ack,
            picoquic_packet_context_application,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_skip_retire_connection_id_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut is_mp: ::core::ffi::c_int,
) -> *const uint8_t {
    if is_mp != 0 {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 3 as ::core::ffi::c_int && !bytes.is_null() {
            bytes = picoquic_frames_varint_skip(bytes, bytes_max);
            i += 1;
        }
    } else {
        bytes =
            picoquic_frames_varint_skip(bytes.offset(1 as ::core::ffi::c_int as isize), bytes_max);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_retire_connection_id_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut unique_path_id: *mut uint64_t,
    mut sequence: *mut uint64_t,
    mut is_mp: ::core::ffi::c_int,
) -> *const uint8_t {
    *unique_path_id = 0 as uint64_t;
    *sequence = 0 as uint64_t;
    if is_mp == 0 {
        bytes = picoquic_frames_varint_decode(bytes, bytes_max, sequence);
    } else {
        bytes = picoquic_frames_varint_decode(bytes, bytes_max, unique_path_id);
        if !bytes.is_null() {
            bytes = picoquic_frames_varint_decode(bytes, bytes_max, sequence);
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_retire_connection_id_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut current_time: uint64_t,
    mut path_x: *mut picoquic_path_t,
    mut is_mp: ::core::ffi::c_int,
) -> *const uint8_t {
    let mut sequence: uint64_t = 0;
    let mut unique_path_id: uint64_t = 0;
    if is_mp != 0 && (*cnx).is_multipath_enabled() == 0 {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
            picoquic_frame_type_path_retire_connection_id as ::core::ffi::c_int as uint64_t,
        );
        bytes = ::core::ptr::null::<uint8_t>();
    } else {
        bytes = picoquic_frames_varint_skip(bytes, bytes_max);
        if bytes.is_null() || {
            bytes = picoquic_parse_retire_connection_id_frame(
                bytes,
                bytes_max,
                &raw mut unique_path_id,
                &raw mut sequence,
                is_mp,
            );
            bytes.is_null()
        } {
            picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
                (if is_mp != 0 {
                    picoquic_frame_type_path_retire_connection_id as ::core::ffi::c_int
                } else {
                    picoquic_frame_type_retire_connection_id as ::core::ffi::c_int
                }) as uint64_t,
            );
        } else if !(*path_x).p_local_cnxid.is_null()
            && (is_mp == 0 || (*path_x).unique_path_id == unique_path_id)
            && sequence == (*(*path_x).p_local_cnxid).sequence
        {
            picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                (if is_mp != 0 {
                    picoquic_frame_type_path_retire_connection_id as ::core::ffi::c_int
                } else {
                    picoquic_frame_type_retire_connection_id as ::core::ffi::c_int
                }) as uint64_t,
            );
            bytes = ::core::ptr::null::<uint8_t>();
        } else {
            picoquic_retire_local_cnxid(cnx, unique_path_id, sequence);
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_check_retire_connection_id_needs_repeat(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_size: size_t,
    mut no_need_to_repeat: *mut ::core::ffi::c_int,
    mut is_mp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sequence: uint64_t = 0 as uint64_t;
    let mut unique_path_id: uint64_t = 0 as uint64_t;
    let mut bytes_first: *const uint8_t =
        picoquic_frames_varint_skip(bytes, bytes.offset(bytes_size as isize));
    let mut bytes_next: *const uint8_t = if bytes_first.is_null() {
        ::core::ptr::null::<uint8_t>()
    } else {
        picoquic_parse_retire_connection_id_frame(
            bytes_first,
            bytes.offset(bytes_size as isize),
            &raw mut unique_path_id,
            &raw mut sequence,
            is_mp,
        )
    };
    *no_need_to_repeat = 1 as ::core::ffi::c_int;
    if bytes_next.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        let mut remote_cnxid_stash: *mut picoquic_remote_cnxid_stash_t =
            picoquic_find_or_create_remote_cnxid_stash(
                cnx,
                unique_path_id,
                0 as ::core::ffi::c_int,
            );
        if !remote_cnxid_stash.is_null() {
            let mut stashed: *mut picoquic_remote_cnxid_t = (*remote_cnxid_stash).cnxid_stash_first;
            while !stashed.is_null() {
                if (*stashed).sequence == sequence {
                    *no_need_to_repeat = (*stashed).retire_acked() as ::core::ffi::c_int;
                    break;
                } else {
                    stashed = (*stashed).next as *mut picoquic_remote_cnxid_t;
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_ack_of_retire_connection_id_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_size: size_t,
    mut consumed: *mut size_t,
    mut is_mp: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sequence: uint64_t = 0 as uint64_t;
    let mut unique_path_id: uint64_t = 0 as uint64_t;
    let mut bytes_next: *const uint8_t = picoquic_parse_retire_connection_id_frame(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes.offset(bytes_size as isize),
        &raw mut unique_path_id,
        &raw mut sequence,
        is_mp,
    );
    if !bytes_next.is_null() {
        *consumed = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
        let mut remote_cnxid_stash: *mut picoquic_remote_cnxid_stash_t =
            picoquic_find_or_create_remote_cnxid_stash(
                cnx,
                unique_path_id,
                0 as ::core::ffi::c_int,
            );
        if !remote_cnxid_stash.is_null() {
            let mut stashed: *mut picoquic_remote_cnxid_t = (*remote_cnxid_stash).cnxid_stash_first;
            while !stashed.is_null() {
                if (*stashed).sequence == sequence {
                    (*stashed).set_retire_acked(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    picoquic_remove_cnxid_from_stash(
                        cnx,
                        remote_cnxid_stash,
                        stashed,
                        ::core::ptr::null_mut::<picoquic_remote_cnxid_t>(),
                    );
                    break;
                } else {
                    stashed = (*stashed).next as *mut picoquic_remote_cnxid_t;
                }
            }
        }
    } else {
        *consumed = bytes_size;
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_new_token_frame(
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut token: *mut uint8_t,
    mut token_length: size_t,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_uint8_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_new_token as ::core::ffi::c_int as uint8_t,
    );
    if !bytes.is_null() && {
        bytes = picoquic_frames_length_data_encode(bytes, bytes_max, token_length, token);
        !bytes.is_null()
    } {
        *is_pure_ack = 0 as ::core::ffi::c_int;
    } else {
        *more_data = 1 as ::core::ffi::c_int;
        bytes = bytes0;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_new_token_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut token: *mut uint8_t,
    mut token_length: size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut more_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_pure_ack: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut frame_buffer: [uint8_t; 258] = [0; 258];
    let mut bytes: *mut uint8_t = picoquic_format_new_token_frame(
        &raw mut frame_buffer as *mut uint8_t,
        (&raw mut frame_buffer as *mut uint8_t)
            .offset(::core::mem::size_of::<[uint8_t; 258]>() as usize as isize),
        &raw mut more_data,
        &raw mut is_pure_ack,
        token,
        token_length,
    );
    if bytes > &raw mut frame_buffer as *mut uint8_t {
        ret = picoquic_queue_misc_frame(
            cnx as *mut picoquic_cnx_t,
            &raw mut frame_buffer as *mut uint8_t,
            bytes.offset_from(&raw mut frame_buffer as *mut uint8_t) as ::core::ffi::c_long
                as size_t,
            1 as ::core::ffi::c_int,
            picoquic_packet_context_application,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_skip_new_token_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    return picoquic_frames_length_data_skip(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_new_token_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut current_time: uint64_t,
    mut addr_to: *mut sockaddr,
) -> *const uint8_t {
    let mut length: uint64_t = 0 as uint64_t;
    let mut token: *const uint8_t = ::core::ptr::null::<uint8_t>();
    bytes = picoquic_frames_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max,
        &raw mut length,
    );
    if !bytes.is_null() {
        token = bytes;
        bytes = picoquic_frames_fixed_skip(bytes, bytes_max, length);
    }
    if bytes.is_null() {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_new_token as ::core::ffi::c_int as uint64_t,
        );
    } else if (*cnx).client_mode() == 0 {
        picoquic_connection_error_ex(
            cnx,
            PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
            picoquic_frame_type_new_token as ::core::ffi::c_int as uint64_t,
            b"Only server can send tokens\0".as_ptr() as *const ::core::ffi::c_char,
        );
        bytes = ::core::ptr::null::<uint8_t>();
    } else if !addr_to.is_null() && !(*cnx).sni.is_null() {
        let mut ip_addr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut ip_addr_length: uint8_t = 0;
        picoquic_get_ip_addr(addr_to, &raw mut ip_addr, &raw mut ip_addr_length);
        picoquic_store_token(
            (*cnx).quic as *mut picoquic_quic_t,
            (*cnx).sni,
            strlen((*cnx).sni) as uint16_t,
            ip_addr,
            ip_addr_length,
            token,
            length as uint16_t,
        );
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_stop_sending_frame(
    mut stream: *mut picoquic_stream_head_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    if (*stream).stop_sending_requested() == 0
        || (*stream).stop_sending_sent() as ::core::ffi::c_int != 0
        || (*stream).fin_received() as ::core::ffi::c_int != 0
        || (*stream).reset_received() as ::core::ffi::c_int != 0
    {
        (*stream).set_stop_sending_sent(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    } else {
        let mut bytes0: *mut uint8_t = bytes;
        bytes = picoquic_frames_uint8_encode(
            bytes,
            bytes_max,
            picoquic_frame_type_stop_sending as ::core::ffi::c_int as uint8_t,
        );
        if !bytes.is_null()
            && {
                bytes = picoquic_frames_varint_encode(bytes, bytes_max, (*stream).stream_id);
                !bytes.is_null()
            }
            && {
                bytes = picoquic_frames_varint_encode(bytes, bytes_max, (*stream).local_stop_error);
                !bytes.is_null()
            }
        {
            *is_pure_ack = 0 as ::core::ffi::c_int;
            (*stream).set_stop_sending_sent(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        } else {
            bytes = bytes0;
            *more_data = 1 as ::core::ffi::c_int;
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_stop_sending_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut stream_id: uint64_t = 0 as uint64_t;
    let mut error_code: uint64_t = 0 as uint64_t;
    let mut stream: *mut picoquic_stream_head_t = ::core::ptr::null_mut::<picoquic_stream_head_t>();
    bytes = picoquic_frames_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max,
        &raw mut stream_id,
    );
    if bytes.is_null() || {
        bytes = picoquic_frames_varint_decode(bytes, bytes_max, &raw mut error_code);
        bytes.is_null()
    } {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_stop_sending as ::core::ffi::c_int as uint64_t,
        );
    } else {
        stream = picoquic_find_or_create_stream(cnx, stream_id, 1 as ::core::ffi::c_int);
        if stream.is_null() {
            picoquic_log_app_message(
                cnx as *mut picoquic_cnx_t,
                b"Received redundant stop sending for old stream %lu\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_id,
            );
        } else if (stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
            as ::core::ffi::c_uint
            == 0
            && ((stream_id ^ (*cnx).client_mode() as uint64_t) & 1 as uint64_t)
                as ::core::ffi::c_uint
                == 0
        {
            picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_STREAM_STATE_ERROR as uint64_t,
                picoquic_frame_type_stop_sending as ::core::ffi::c_int as uint64_t,
            );
            bytes = ::core::ptr::null::<uint8_t>();
        } else if (*stream).stop_sending_received() == 0
            && (*stream).reset_requested() == 0
            && (*stream).fin_sent() == 0
        {
            (*stream).set_stop_sending_received(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*stream).remote_stop_error = error_code;
            if (*cnx).callback_fn.is_some() && (*stream).stop_sending_signalled() == 0 {
                if (*stream).is_discarded() == 0 {
                    if (*cnx).callback_fn.expect("non-null function pointer")(
                        cnx as *mut picoquic_cnx_t,
                        (*stream).stream_id,
                        ::core::ptr::null_mut::<uint8_t>(),
                        0 as size_t,
                        picoquic_callback_stop_sending,
                        (*cnx).callback_ctx,
                        (*stream).app_stream_ctx,
                    ) != 0 as ::core::ffi::c_int
                    {
                        picoquic_log_app_message(
                            cnx as *mut picoquic_cnx_t,
                            b"Stop sending callback on stream %lu returns error 0x%x\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*stream).stream_id,
                            PICOQUIC_TRANSPORT_INTERNAL_ERROR,
                        );
                        picoquic_connection_error(
                            cnx,
                            PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t,
                            picoquic_frame_type_stop_sending as ::core::ffi::c_int as uint64_t,
                        );
                    }
                }
                (*stream)
                    .set_stop_sending_signalled(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        } else {
            picoquic_log_app_message(
                cnx as *mut picoquic_cnx_t,
                b"Received stop sending for finished stream %lu\0".as_ptr()
                    as *const ::core::ffi::c_char,
                stream_id,
            );
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_skip_stop_sending_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes.offset(1 as ::core::ffi::c_int as isize), bytes_max);
    if !bytes.is_null() {
        bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_check_stop_sending_needs_repeat(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_size: size_t,
    mut no_need_to_repeat: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut stream_id: uint64_t = 0 as uint64_t;
    let mut error_code: uint64_t = 0 as uint64_t;
    let mut bytes_max: *const uint8_t = bytes.offset(bytes_size as isize);
    let mut stream: *mut picoquic_stream_head_t = ::core::ptr::null_mut::<picoquic_stream_head_t>();
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    *no_need_to_repeat = 0 as ::core::ffi::c_int;
    bytes = picoquic_frames_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max,
        &raw mut stream_id,
    );
    if bytes.is_null() || {
        bytes = picoquic_frames_varint_decode(bytes, bytes_max, &raw mut error_code);
        bytes.is_null()
    } {
        *no_need_to_repeat = 1 as ::core::ffi::c_int;
    } else {
        stream = picoquic_find_stream(cnx, stream_id);
        if stream.is_null() {
            *no_need_to_repeat = 1 as ::core::ffi::c_int;
        } else if (*stream).fin_received() as ::core::ffi::c_int != 0
            || (*stream).reset_received() as ::core::ffi::c_int != 0
        {
            *no_need_to_repeat = 1 as ::core::ffi::c_int;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_stream_frame_unlimited(
    mut bytes: *const uint8_t,
) -> ::core::ffi::c_int {
    return (*bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & !(picoquic_frame_type_stream_range_min as ::core::ffi::c_int
            ^ picoquic_frame_type_stream_range_max as ::core::ffi::c_int
            ^ 0x2 as ::core::ffi::c_int)
        == picoquic_frame_type_stream_range_min as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_stream_header(
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut stream_id: *mut uint64_t,
    mut offset: *mut uint64_t,
    mut data_length: *mut size_t,
    mut fin: *mut ::core::ffi::c_int,
    mut consumed: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut len: ::core::ffi::c_int = *bytes.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & 2 as ::core::ffi::c_int;
    let mut off: ::core::ffi::c_int = *bytes.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & 4 as ::core::ffi::c_int;
    let mut length: uint64_t = 0 as uint64_t;
    let mut l_stream: size_t = 0 as size_t;
    let mut l_len: size_t = 0 as size_t;
    let mut l_off: size_t = 0 as size_t;
    let mut byte_index: size_t = 1 as size_t;
    *fin = *bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 1 as ::core::ffi::c_int;
    if bytes_max > byte_index {
        l_stream = picoquic_varint_decode(
            bytes.offset(byte_index as isize),
            bytes_max.wrapping_sub(byte_index),
            stream_id,
        );
        byte_index = byte_index.wrapping_add(l_stream);
    }
    if off == 0 as ::core::ffi::c_int {
        *offset = 0 as uint64_t;
    } else if bytes_max > byte_index {
        l_off = picoquic_varint_decode(
            bytes.offset(byte_index as isize),
            bytes_max.wrapping_sub(byte_index),
            offset,
        );
        byte_index = byte_index.wrapping_add(l_off);
    }
    if bytes_max < byte_index
        || l_stream == 0 as size_t
        || off != 0 as ::core::ffi::c_int && l_off == 0 as size_t
    {
        *data_length = 0 as size_t;
        byte_index = bytes_max;
        ret = -(1 as ::core::ffi::c_int);
    } else if len == 0 as ::core::ffi::c_int {
        *data_length = bytes_max.wrapping_sub(byte_index);
    } else {
        if bytes_max > byte_index {
            l_len = picoquic_varint_decode(
                bytes.offset(byte_index as isize),
                bytes_max.wrapping_sub(byte_index),
                &raw mut length,
            );
            byte_index = byte_index.wrapping_add(l_len);
            *data_length = length as size_t;
        }
        if l_len == 0 as size_t || bytes_max < byte_index {
            byte_index = bytes_max;
            ret = -(1 as ::core::ffi::c_int);
        } else if byte_index.wrapping_add(length as size_t) > bytes_max {
            ret = -(1 as ::core::ffi::c_int);
        }
    }
    *consumed = byte_index;
    return ret;
}
unsafe extern "C" fn picoquic_stream_data_chunk_callback(
    mut cnx: *mut picoquic_cnx_t,
    mut stream: *mut picoquic_stream_head_t,
    mut bytes: *const uint8_t,
    mut data_length: size_t,
) {
    let mut fin_now: picoquic_call_back_event_t = picoquic_callback_stream_data;
    let mut call_back_needed: ::core::ffi::c_int =
        (data_length > 0 as size_t) as ::core::ffi::c_int;
    (*stream).consumed_offset = ((*stream).consumed_offset as ::core::ffi::c_ulong)
        .wrapping_add(data_length as ::core::ffi::c_ulong)
        as uint64_t as uint64_t;
    if (*stream).consumed_offset >= (*stream).fin_offset
        && (*stream).fin_received() as ::core::ffi::c_int != 0
        && (*stream).fin_signalled() == 0
    {
        fin_now = picoquic_callback_stream_fin;
        (*stream).set_fin_signalled(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        call_back_needed = 1 as ::core::ffi::c_int;
    }
    if call_back_needed != 0
        && (*stream).stop_sending_requested() == 0
        && (*stream).is_discarded() == 0
        && (*cnx).callback_fn.expect("non-null function pointer")(
            cnx as *mut picoquic_cnx_t,
            (*stream).stream_id,
            bytes as *mut uint8_t,
            data_length,
            fin_now,
            (*cnx).callback_ctx,
            (*stream).app_stream_ctx,
        ) != 0 as ::core::ffi::c_int
    {
        picoquic_log_app_message(
            cnx as *mut picoquic_cnx_t,
            b"Data callback (%d, l=%zu) on stream %lu returns error 0x%x\0".as_ptr()
                as *const ::core::ffi::c_char,
            fin_now as ::core::ffi::c_uint,
            data_length,
            (*stream).stream_id,
            PICOQUIC_TRANSPORT_INTERNAL_ERROR,
        );
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t,
            0 as uint64_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_stream_data_callback(
    mut cnx: *mut picoquic_cnx_t,
    mut stream: *mut picoquic_stream_head_t,
) {
    let mut data: *mut picoquic_stream_data_node_t =
        ::core::ptr::null_mut::<picoquic_stream_data_node_t>();
    loop {
        data = picosplay_first(&raw mut (*stream).stream_data_tree)
            as *mut picoquic_stream_data_node_t;
        if !(!data.is_null() && (*data).offset <= (*stream).consumed_offset) {
            break;
        }
        let mut start: size_t = (*stream).consumed_offset.wrapping_sub((*data).offset) as size_t;
        if (*data).length >= start {
            let mut data_length: size_t = (*data).length.wrapping_sub(start);
            picoquic_stream_data_chunk_callback(
                cnx,
                stream,
                (*data).bytes.offset(start as isize),
                data_length,
            );
        }
        picosplay_delete_hint(
            &raw mut (*stream).stream_data_tree,
            &raw mut (*data).stream_data_node,
        );
    }
    picoquic_stream_data_chunk_callback(cnx, stream, ::core::ptr::null::<uint8_t>(), 0 as size_t);
}
unsafe extern "C" fn add_chunk_node(
    mut quic: *mut picoquic_quic_t,
    mut tree: *mut picosplay_tree_t,
    mut offset: uint64_t,
    mut length: size_t,
    mut is_last_frame: ::core::ffi::c_int,
    mut bytes: *const uint8_t,
    mut chunk_added: *mut ::core::ffi::c_int,
    mut received_data: *mut picoquic_stream_data_node_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut node: *mut picoquic_stream_data_node_t = received_data;
    if received_data.is_null() || !(*received_data).bytes.is_null() || is_last_frame == 0 {
        node = picoquic_stream_data_node_alloc(quic);
        if node.is_null() {
            ret = PICOQUIC_ERROR_MEMORY;
        } else {
            (*node).bytes = &raw mut (*node).data as *mut uint8_t;
            memmove(
                &raw mut (*node).data as *mut uint8_t as *mut ::core::ffi::c_void,
                bytes as *const ::core::ffi::c_void,
                length,
            );
            (*node).offset = offset;
            (*node).length = length;
        }
    } else {
        (*node).bytes = bytes;
        (*node).offset = offset;
        (*node).length = length;
    }
    if !node.is_null() {
        picosplay_insert(tree, node as *mut ::core::ffi::c_void);
        *chunk_added = 1 as ::core::ffi::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_network_input(
    mut quic: *mut picoquic_quic_t,
    mut tree: *mut picosplay_tree_t,
    mut consumed_offset: uint64_t,
    mut frame_data_offset: uint64_t,
    mut bytes: *const uint8_t,
    mut length: size_t,
    mut is_last_frame: ::core::ffi::c_int,
    mut received_data: *mut picoquic_stream_data_node_t,
    mut new_data_available: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let input_begin: uint64_t = frame_data_offset;
    let input_end: uint64_t = frame_data_offset.wrapping_add(length as uint64_t);
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if frame_data_offset < consumed_offset {
        frame_data_offset = consumed_offset;
    }
    if frame_data_offset < input_end {
        let mut target: picoquic_stream_data_node_t = st_picoquic_stream_data_node_t {
            stream_data_node: st_picosplay_node_t {
                parent: ::core::ptr::null_mut::<st_picosplay_node_t>(),
                left: ::core::ptr::null_mut::<st_picosplay_node_t>(),
                right: ::core::ptr::null_mut::<st_picosplay_node_t>(),
            },
            quic: ::core::ptr::null_mut::<picoquic_quic_t>(),
            next_stream_data: ::core::ptr::null_mut::<st_picoquic_stream_data_node_t>(),
            offset: 0,
            length: 0,
            bytes: ::core::ptr::null::<uint8_t>(),
            data: [0; 1536],
        };
        memset(
            &raw mut target as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<picoquic_stream_data_node_t>() as size_t,
        );
        target.offset = frame_data_offset;
        let mut prev: *mut picoquic_stream_data_node_t =
            picosplay_find_previous(tree, &raw mut target as *mut ::core::ffi::c_void)
                as *mut picoquic_stream_data_node_t;
        if !prev.is_null() {
            let prev_end: uint64_t = (*prev).offset.wrapping_add((*prev).length as uint64_t);
            frame_data_offset = if frame_data_offset > prev_end {
                frame_data_offset
            } else {
                prev_end
            };
        }
        let mut next: *mut picoquic_stream_data_node_t = if prev.is_null() {
            picosplay_first(tree) as *mut picoquic_stream_data_node_t
        } else {
            picosplay_next(&raw mut (*prev).stream_data_node) as *mut picoquic_stream_data_node_t
        };
        while ret == 0 as ::core::ffi::c_int
            && frame_data_offset < input_end
            && !next.is_null()
            && (*next).offset < input_end
        {
            let chunk_ofs: uint64_t = frame_data_offset;
            let chunk_len: uint64_t = if (*next).offset > frame_data_offset {
                (*next).offset.wrapping_sub(frame_data_offset)
            } else {
                0 as uint64_t
            };
            if chunk_len > 0 as uint64_t {
                ret = add_chunk_node(
                    quic,
                    tree,
                    chunk_ofs,
                    chunk_len as size_t,
                    is_last_frame,
                    bytes
                        .offset(frame_data_offset as isize)
                        .offset(-(input_begin as isize)),
                    new_data_available,
                    received_data,
                );
            }
            frame_data_offset = (*next).offset.wrapping_add((*next).length as uint64_t);
            next = picosplay_next(&raw mut (*next).stream_data_node)
                as *mut picoquic_stream_data_node_t;
        }
        if ret == 0 as ::core::ffi::c_int && frame_data_offset < input_end {
            let chunk_ofs_0: uint64_t = frame_data_offset;
            let chunk_len_0: uint64_t = input_end.wrapping_sub(frame_data_offset);
            ret = add_chunk_node(
                quic,
                tree,
                chunk_ofs_0,
                chunk_len_0 as size_t,
                is_last_frame,
                bytes
                    .offset(frame_data_offset as isize)
                    .offset(-(input_begin as isize)),
                new_data_available,
                received_data,
            );
        }
    }
    return ret;
}
unsafe extern "C" fn picoquic_stream_network_input(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut offset: uint64_t,
    mut fin: ::core::ffi::c_int,
    mut bytes: *const uint8_t,
    mut length: size_t,
    mut received_data: *mut picoquic_stream_data_node_t,
    mut is_last_frame: ::core::ffi::c_int,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut should_notify: uint64_t = 0 as uint64_t;
    let mut stream: *mut picoquic_stream_head_t = ::core::ptr::null_mut::<picoquic_stream_head_t>();
    let mut new_fin_offset: uint64_t = offset.wrapping_add(length as uint64_t);
    stream = picoquic_find_or_create_stream(cnx, stream_id, 1 as ::core::ffi::c_int);
    if stream.is_null() {
        if stream_id < (*cnx).next_stream_id[(stream_id & 3 as uint64_t) as usize] {
            return 0 as ::core::ffi::c_int;
        } else {
            ret = 1 as ::core::ffi::c_int;
        }
    } else if (*stream).fin_received() != 0 {
        if if fin != 0 as ::core::ffi::c_int {
            ((*stream).fin_offset != new_fin_offset) as ::core::ffi::c_int
        } else {
            (new_fin_offset > (*stream).fin_offset) as ::core::ffi::c_int
        } != 0
        {
            ret = picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_FINAL_OFFSET_ERROR as uint64_t,
                0 as uint64_t,
            );
        }
    } else {
        if fin != 0 {
            (*stream).set_fin_received(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            should_notify = 1 as uint64_t;
            (*cnx).latest_receive_time = current_time;
            picoquic_update_max_stream_ID_local(cnx, stream);
        }
        if new_fin_offset > (*stream).fin_offset {
            ret = picoquic_flow_control_check_stream_offset(cnx, stream, new_fin_offset);
        }
    }
    if ret == 0 as ::core::ffi::c_int {
        if (*stream).direct_receive_fn.is_some() {
            ret = (*stream)
                .direct_receive_fn
                .expect("non-null function pointer")(
                cnx as *mut picoquic_cnx_t,
                stream_id,
                fin,
                bytes,
                offset,
                length,
                (*stream).direct_receive_ctx,
            );
            if ret == PICOQUIC_STREAM_RECEIVE_COMPLETE
                && (*stream).fin_received() as ::core::ffi::c_int != 0
            {
                (*stream).set_fin_signalled(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                ret = 0 as ::core::ffi::c_int;
            } else if ret != 0 as ::core::ffi::c_int {
                let mut err: uint64_t = if ret >= PICOQUIC_ERROR_CLASS {
                    PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t
                } else {
                    ret as uint64_t
                };
                ret = picoquic_connection_error(cnx, err, 0 as uint64_t);
            }
        } else if (*stream).consumed_offset >= offset && (*cnx).callback_fn.is_some() {
            if new_fin_offset >= (*stream).consumed_offset {
                let mut delivered_index: uint64_t = (*stream).consumed_offset.wrapping_sub(offset);
                let mut data_length: uint64_t = (length as uint64_t).wrapping_sub(delivered_index);
                picoquic_stream_data_chunk_callback(
                    cnx,
                    stream,
                    (bytes as *mut uint8_t).offset(delivered_index as isize),
                    data_length as size_t,
                );
                picoquic_stream_data_callback(cnx, stream);
            }
        } else {
            let mut new_data_available: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            ret = picoquic_queue_network_input(
                (*cnx).quic,
                &raw mut (*stream).stream_data_tree,
                (*stream).consumed_offset,
                offset,
                bytes,
                length,
                is_last_frame,
                received_data,
                &raw mut new_data_available,
            );
            if ret != 0 as ::core::ffi::c_int {
                ret = picoquic_connection_error(cnx, ret as int64_t as uint64_t, 0 as uint64_t);
            } else if new_data_available != 0 {
                should_notify = 1 as uint64_t;
                (*cnx).latest_receive_time = current_time;
            }
            if ret == 0 as ::core::ffi::c_int
                && should_notify != 0 as uint64_t
                && (*cnx).callback_fn.is_some()
            {
                picoquic_stream_data_callback(cnx, stream);
            }
        }
    }
    if ret == 0 as ::core::ffi::c_int {
        let mut is_deleted: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*stream).fin_signalled() != 0 {
            is_deleted = picoquic_delete_stream_if_closed(cnx, stream);
        }
        if is_deleted == 0 {
            if (*stream).fin_signalled() == 0 {
                if (*stream).fin_received() == 0
                    && (*stream).reset_received() == 0
                    && (2 as uint64_t).wrapping_mul((*stream).consumed_offset)
                        > (*stream).maxdata_local
                {
                    (*cnx).set_max_stream_data_needed(
                        1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                    );
                }
            }
            if (*stream).fin_received() as ::core::ffi::c_int != 0
                || (*stream).reset_received() as ::core::ffi::c_int != 0
            {
                (*cnx).ack_ctx[picoquic_packet_context_application as ::core::ffi::c_int as usize]
                    .act[0 as ::core::ffi::c_int as usize]
                    .set_ack_after_fin(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*cnx).ack_ctx[picoquic_packet_context_application as ::core::ffi::c_int as usize]
                    .act[1 as ::core::ffi::c_int as usize]
                    .set_ack_after_fin(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_last_stream_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> ::core::ffi::c_int {
    while bytes < bytes_max
        && *bytes as ::core::ffi::c_int == picoquic_frame_type_padding as ::core::ffi::c_int
    {
        bytes = bytes.offset(1);
    }
    return if bytes < bytes_max {
        0 as ::core::ffi::c_int
    } else {
        1 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_stream_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut received_data: *mut picoquic_stream_data_node_t,
    mut current_time: uint64_t,
) -> *const uint8_t {
    let mut stream_id: uint64_t = 0;
    let mut data_length: size_t = 0;
    let mut offset: uint64_t = 0;
    let mut fin: ::core::ffi::c_int = 0;
    let mut consumed: size_t = 0;
    if picoquic_parse_stream_header(
        bytes,
        bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t,
        &raw mut stream_id,
        &raw mut offset,
        &raw mut data_length,
        &raw mut fin,
        &raw mut consumed,
    ) != 0 as ::core::ffi::c_int
    {
        bytes = ::core::ptr::null::<uint8_t>();
    } else if offset.wrapping_add(data_length as uint64_t) as ::core::ffi::c_ulonglong
        >= (1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int
    {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FINAL_OFFSET_ERROR as uint64_t,
            0 as uint64_t,
        );
        bytes = ::core::ptr::null::<uint8_t>();
    } else {
        bytes = bytes.offset(consumed as isize);
        if picoquic_stream_network_input(
            cnx,
            stream_id,
            offset,
            fin,
            bytes,
            data_length,
            received_data,
            picoquic_is_last_stream_frame(bytes.offset(data_length as isize), bytes_max),
            current_time,
        ) != 0 as ::core::ffi::c_int
        {
            bytes = ::core::ptr::null::<uint8_t>();
        } else {
            bytes = bytes.offset(data_length as isize);
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_find_ready_stream_path(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
) -> *mut picoquic_stream_head_t {
    let mut first_stream: *mut picoquic_stream_head_t = (*cnx).first_output_stream;
    let mut stream: *mut picoquic_stream_head_t = first_stream;
    let mut found_stream: *mut picoquic_stream_head_t =
        ::core::ptr::null_mut::<picoquic_stream_head_t>();
    while !stream.is_null() {
        let mut has_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut next_stream: *mut picoquic_stream_head_t =
            (*stream).next_output_stream as *mut picoquic_stream_head_t;
        if !found_stream.is_null()
            && (*stream).stream_priority as ::core::ffi::c_int
                > (*found_stream).stream_priority as ::core::ffi::c_int
        {
            break;
        }
        has_data = ((*cnx).maxdata_remote > (*cnx).data_sent
            && (*stream).sent_offset < (*stream).maxdata_remote
            && ((*stream).is_active() as ::core::ffi::c_int != 0
                || !(*stream).send_queue.is_null()
                    && (*(*stream).send_queue).length > (*(*stream).send_queue).offset as size_t
                || (*stream).fin_requested() as ::core::ffi::c_int != 0
                    && (*stream).fin_sent() == 0)) as ::core::ffi::c_int;
        if has_data != 0
            && !path_x.is_null()
            && (*stream).affinity_path != path_x
            && !(*stream).affinity_path.is_null()
        {
            has_data = 0 as ::core::ffi::c_int;
        }
        if (*stream).reset_requested() as ::core::ffi::c_int != 0 && (*stream).reset_sent() == 0
            || (*stream).stop_sending_requested() as ::core::ffi::c_int != 0
                && (*stream).stop_sending_sent() == 0
        {
            found_stream = stream;
            break;
        } else {
            if has_data != 0 {
                if (*stream).sent_offset == 0 as uint64_t {
                    if ((*stream).stream_id & 1 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
                        as ::core::ffi::c_uint
                        == (*cnx).client_mode()
                    {
                        if (*stream).stream_id
                            > (if ((*stream).stream_id & 2 as uint64_t == 0 as uint64_t)
                                as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                                != 0
                            {
                                (*cnx).max_stream_id_bidir_remote
                            } else {
                                (*cnx).max_stream_id_unidir_remote
                            })
                        {
                            has_data = 0 as ::core::ffi::c_int;
                        }
                    }
                }
                if has_data != 0 {
                    if (*stream).stream_priority as ::core::ffi::c_int & 1 as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int
                    {
                        found_stream = stream;
                        break;
                    } else if found_stream.is_null()
                        || (*stream).last_time_data_sent < (*found_stream).last_time_data_sent
                    {
                        found_stream = stream;
                    }
                }
            } else if ((*stream).fin_requested() as ::core::ffi::c_int != 0
                && (*stream).fin_sent() as ::core::ffi::c_int != 0
                || (*stream).reset_requested() as ::core::ffi::c_int != 0
                    && (*stream).reset_sent() as ::core::ffi::c_int != 0)
                && ((*stream).stop_sending_requested() == 0
                    || (*stream).stop_sending_sent() as ::core::ffi::c_int != 0)
            {
                picoquic_remove_output_stream(cnx, stream);
                picoquic_delete_stream_if_closed(cnx, stream);
            } else if (*stream).is_active() as ::core::ffi::c_int != 0
                || !(*stream).send_queue.is_null()
                    && (*(*stream).send_queue).length > (*(*stream).send_queue).offset as size_t
            {
                if (*stream).sent_offset >= (*stream).maxdata_remote {
                    (*cnx).set_stream_blocked(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                } else if (*cnx).maxdata_remote <= (*cnx).data_sent {
                    (*cnx).set_flow_blocked(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
            }
            stream = next_stream;
        }
    }
    return found_stream;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_find_ready_stream(
    mut cnx: *mut picoquic_cnx_t,
) -> *mut picoquic_stream_head_t {
    return picoquic_find_ready_stream_path(cnx, ::core::ptr::null_mut::<picoquic_path_t>());
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_data_blocked_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_uint8_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_data_blocked as ::core::ffi::c_int as uint8_t,
    );
    if !bytes.is_null() && {
        bytes = picoquic_frames_varint_encode(bytes, bytes_max, (*cnx).maxdata_remote);
        !bytes.is_null()
    } {
        *is_pure_ack = 0 as ::core::ffi::c_int;
        (*cnx).set_sent_blocked_frame(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    } else {
        *more_data = 1 as ::core::ffi::c_int;
        bytes = bytes0;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_stream_data_blocked_frame(
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut stream: *mut picoquic_stream_head_t,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_uint8_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_stream_data_blocked as ::core::ffi::c_int as uint8_t,
    );
    if !bytes.is_null()
        && {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, (*stream).stream_id);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, (*stream).maxdata_remote);
            !bytes.is_null()
        }
    {
        *is_pure_ack = 0 as ::core::ffi::c_int;
        (*stream).set_stream_data_blocked_sent(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    } else {
        *more_data = 1 as ::core::ffi::c_int;
        bytes = bytes0;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_stream_blocked_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut stream: *mut picoquic_stream_head_t,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    let mut f_type: uint8_t = 0 as uint8_t;
    let mut stream_limit: uint64_t = 0 as uint64_t;
    let mut should_not_send: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if ((*stream).stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
        as ::core::ffi::c_uint
        != 0
    {
        f_type = picoquic_frame_type_streams_blocked_bidir as ::core::ffi::c_int as uint8_t;
        stream_limit = (*cnx)
            .max_stream_id_bidir_remote
            .wrapping_add(4 as uint64_t)
            >> 2 as ::core::ffi::c_int;
        should_not_send = (*cnx).stream_blocked_bidir_sent() as ::core::ffi::c_int;
    } else {
        f_type = picoquic_frame_type_streams_blocked_unidir as ::core::ffi::c_int as uint8_t;
        stream_limit = (*cnx)
            .max_stream_id_unidir_remote
            .wrapping_add(4 as uint64_t)
            >> 2 as ::core::ffi::c_int;
        should_not_send = (*cnx).stream_blocked_unidir_sent() as ::core::ffi::c_int;
    }
    if should_not_send == 0 {
        bytes = picoquic_frames_uint8_encode(bytes, bytes_max, f_type);
        if !bytes.is_null() && {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, stream_limit);
            !bytes.is_null()
        } {
            *is_pure_ack = 0 as ::core::ffi::c_int;
            if ((*stream).stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
                as ::core::ffi::c_uint
                != 0
            {
                (*cnx)
                    .set_stream_blocked_bidir_sent(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            } else {
                (*cnx).set_stream_blocked_unidir_sent(
                    1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                );
            }
        } else {
            *more_data = 1 as ::core::ffi::c_int;
            bytes = bytes0;
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_one_blocked_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut stream: *mut picoquic_stream_head_t,
) -> *mut uint8_t {
    if (*stream).is_active() as ::core::ffi::c_int != 0
        || !(*stream).send_queue.is_null()
            && (*(*stream).send_queue).length > (*(*stream).send_queue).offset as size_t
    {
        if ((*stream).stream_id & 1 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
            as ::core::ffi::c_uint
            == (*cnx).client_mode()
            && (*stream).stream_id
                > (if ((*stream).stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                    != 0
                {
                    (*cnx).max_stream_id_bidir_remote
                } else {
                    (*cnx).max_stream_id_unidir_remote
                })
        {
            if if ((*stream).stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
                as ::core::ffi::c_uint
                != 0
            {
                (*cnx).stream_blocked_bidir_sent() as ::core::ffi::c_int
            } else {
                (*cnx).stream_blocked_unidir_sent() as ::core::ffi::c_int
            } == 0
            {
                bytes = picoquic_format_stream_blocked_frame(
                    cnx,
                    bytes,
                    bytes_max,
                    more_data,
                    is_pure_ack,
                    stream,
                );
            }
        } else {
            if (*cnx).maxdata_remote <= (*cnx).data_sent && (*cnx).sent_blocked_frame() == 0 {
                bytes = picoquic_format_data_blocked_frame(
                    cnx,
                    bytes,
                    bytes_max,
                    more_data,
                    is_pure_ack,
                );
            }
            if (*stream).sent_offset >= (*stream).maxdata_remote
                && (*stream).stream_data_blocked_sent() == 0
            {
                bytes = picoquic_format_stream_data_blocked_frame(
                    bytes,
                    bytes_max,
                    more_data,
                    is_pure_ack,
                    stream,
                );
            }
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_blocked_frames(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut stream: *mut picoquic_stream_head_t = picoquic_first_stream(cnx);
    let mut hi_pri_stream: *mut picoquic_stream_head_t =
        ::core::ptr::null_mut::<picoquic_stream_head_t>();
    if (*cnx).high_priority_stream_id != UINT64_MAX as uint64_t {
        hi_pri_stream = picoquic_find_stream(cnx, (*cnx).high_priority_stream_id);
    }
    while !stream.is_null() {
        if hi_pri_stream.is_null() || stream == hi_pri_stream {
            bytes = picoquic_format_one_blocked_frame(
                cnx,
                bytes,
                bytes_max,
                more_data,
                is_pure_ack,
                stream,
            );
            if *more_data != 0 {
                break;
            }
        }
        stream = picoquic_next_stream(stream);
    }
    return bytes;
}
unsafe extern "C" fn picoquic_encode_length_of_stream_frame(
    mut bytes: *mut uint8_t,
    mut byte_index: size_t,
    mut byte_space: size_t,
    mut length: size_t,
    mut start_index: *mut size_t,
) -> size_t {
    if length < byte_space {
        if length == byte_space.wrapping_sub(1 as size_t) {
            memmove(
                bytes.offset(1 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void,
                bytes as *const ::core::ffi::c_void,
                byte_index,
            );
            *bytes.offset(0 as ::core::ffi::c_int as isize) =
                picoquic_frame_type_padding as ::core::ffi::c_int as uint8_t;
            *start_index = 1 as size_t;
            byte_index = byte_index.wrapping_add(1);
        } else {
            byte_index = byte_index.wrapping_add(picoquic_varint_encode(
                bytes.offset(byte_index as isize),
                byte_space,
                length as uint64_t,
            ));
            let ref mut c2rust_fresh1 = *bytes.offset(0 as ::core::ffi::c_int as isize);
            *c2rust_fresh1 =
                (*c2rust_fresh1 as ::core::ffi::c_int | 2 as ::core::ffi::c_int) as uint8_t;
        }
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_provide_stream_data_buffer(
    mut context: *mut ::core::ffi::c_void,
    mut length: size_t,
    mut is_fin: ::core::ffi::c_int,
    mut is_still_active: ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut data_ctx: *mut picoquic_stream_data_buffer_argument_t =
        context as *mut picoquic_stream_data_buffer_argument_t;
    let mut buffer: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut start_index: size_t = 0 as size_t;
    if length <= (*data_ctx).allowed_space {
        (*data_ctx).length = length;
        if is_fin != 0 {
            (*data_ctx).is_fin = 1 as ::core::ffi::c_int;
            let ref mut c2rust_fresh0 = *(*data_ctx).bytes.offset(0 as ::core::ffi::c_int as isize);
            *c2rust_fresh0 =
                (*c2rust_fresh0 as ::core::ffi::c_int | 1 as ::core::ffi::c_int) as uint8_t;
        }
        (*data_ctx).is_still_active = is_still_active;
        (*data_ctx).byte_index = picoquic_encode_length_of_stream_frame(
            (*data_ctx).bytes,
            (*data_ctx).byte_index,
            (*data_ctx).byte_space,
            length,
            &raw mut start_index,
        );
        buffer = (*data_ctx).bytes.offset((*data_ctx).byte_index as isize);
        (*data_ctx).app_buffer = buffer;
    }
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_stream_frame_header(
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut stream_id: uint64_t,
    mut offset: uint64_t,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_uint8_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_stream_range_min as ::core::ffi::c_int as uint8_t,
    );
    if !bytes.is_null() && {
        bytes = picoquic_frames_varint_encode(bytes, bytes_max, stream_id);
        !bytes.is_null()
    } {
        if offset > 0 as uint64_t {
            *bytes0 = (*bytes0 as ::core::ffi::c_int | 4 as ::core::ffi::c_int) as uint8_t;
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, offset);
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_stream_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut stream: *mut picoquic_stream_head_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut is_still_active: *mut ::core::ffi::c_int,
    mut ret: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut may_close: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    *ret = 0 as ::core::ffi::c_int;
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
            return ::core::ptr::null_mut::<uint8_t>();
        }
    }
    if (*stream).reset_requested() as ::core::ffi::c_int != 0 && (*stream).reset_sent() == 0 {
        return picoquic_format_stream_reset_frame(
            cnx,
            stream,
            bytes,
            bytes_max,
            more_data,
            is_pure_ack,
        );
    }
    if (*stream).stop_sending_requested() as ::core::ffi::c_int != 0
        && (*stream).stop_sending_sent() == 0
    {
        return picoquic_format_stop_sending_frame(
            stream,
            bytes,
            bytes_max,
            more_data,
            is_pure_ack,
        );
    }
    if !((*stream).is_active() == 0
        && ((*stream).send_queue.is_null()
            || (*(*stream).send_queue).length <= (*(*stream).send_queue).offset as size_t)
        && ((*stream).fin_requested() == 0 || (*stream).fin_sent() as ::core::ffi::c_int != 0))
    {
        let mut bytes0: *mut uint8_t = bytes;
        let mut byte_index: size_t = 0 as size_t;
        let mut length: size_t = 0 as size_t;
        bytes = picoquic_format_stream_frame_header(
            bytes,
            bytes_max,
            (*stream).stream_id,
            (*stream).sent_offset,
        );
        if bytes.is_null() {
            bytes = bytes0;
            *more_data = 1 as ::core::ffi::c_int;
        } else {
            let mut byte_space: size_t =
                bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t;
            let mut allowed_space: size_t = byte_space;
            if allowed_space
                > ((*stream).maxdata_remote as size_t).wrapping_sub((*stream).sent_offset as size_t)
            {
                allowed_space =
                    (*stream).maxdata_remote.wrapping_sub((*stream).sent_offset) as size_t;
            }
            if allowed_space
                > ((*cnx).maxdata_remote as size_t).wrapping_sub((*cnx).data_sent as size_t)
            {
                allowed_space = (*cnx).maxdata_remote.wrapping_sub((*cnx).data_sent) as size_t;
            }
            if (*stream).is_active() as ::core::ffi::c_int != 0
                && (*stream).send_queue.is_null()
                && (*stream).fin_requested() == 0
            {
                let mut stream_data_context: picoquic_stream_data_buffer_argument_t =
                    st_picoquic_stream_data_buffer_argument_t {
                        bytes: ::core::ptr::null_mut::<uint8_t>(),
                        byte_index: 0,
                        byte_space: 0,
                        allowed_space: 0,
                        length: 0,
                        is_fin: 0,
                        is_still_active: 0,
                        app_buffer: ::core::ptr::null_mut::<uint8_t>(),
                    };
                stream_data_context.bytes = bytes0;
                stream_data_context.byte_index =
                    bytes.offset_from(bytes0) as ::core::ffi::c_long as size_t;
                stream_data_context.allowed_space = allowed_space;
                stream_data_context.byte_space =
                    bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t;
                stream_data_context.length = 0 as size_t;
                stream_data_context.is_fin = 0 as ::core::ffi::c_int;
                stream_data_context.is_still_active = 0 as ::core::ffi::c_int;
                stream_data_context.app_buffer = ::core::ptr::null_mut::<uint8_t>();
                if (*cnx).callback_fn.expect("non-null function pointer")(
                    cnx as *mut picoquic_cnx_t,
                    (*stream).stream_id,
                    &raw mut stream_data_context as *mut uint8_t,
                    allowed_space,
                    picoquic_callback_prepare_to_send,
                    (*cnx).callback_ctx,
                    (*stream).app_stream_ctx,
                ) != 0 as ::core::ffi::c_int
                {
                    picoquic_log_app_message(
                        cnx as *mut picoquic_cnx_t,
                        b"Prepare to send returns error 0x%x\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        PICOQUIC_TRANSPORT_INTERNAL_ERROR,
                    );
                    *ret = picoquic_connection_error_ex(
                        cnx,
                        PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t,
                        0 as uint64_t,
                        b"Prepare to send callback\0".as_ptr() as *const ::core::ffi::c_char,
                    );
                    bytes = bytes0;
                } else if stream_data_context.length == 0 as size_t
                    && stream_data_context.is_fin == 0 as ::core::ffi::c_int
                {
                    bytes = bytes0;
                    (*stream).set_is_active(
                        stream_data_context.is_still_active as ::core::ffi::c_uint
                            as ::core::ffi::c_uint,
                    );
                } else {
                    bytes = bytes0
                        .offset(stream_data_context.byte_index as isize)
                        .offset(stream_data_context.length as isize);
                    (*stream).sent_offset = ((*stream).sent_offset as ::core::ffi::c_ulong)
                        .wrapping_add(stream_data_context.length as ::core::ffi::c_ulong)
                        as uint64_t as uint64_t;
                    (*stream).last_time_data_sent =
                        picoquic_get_quic_time((*cnx).quic as *mut picoquic_quic_t);
                    (*cnx).data_sent = ((*cnx).data_sent as ::core::ffi::c_ulong)
                        .wrapping_add(stream_data_context.length as ::core::ffi::c_ulong)
                        as uint64_t as uint64_t;
                    if stream_data_context.length > 0 as size_t {
                        if stream_data_context.app_buffer.is_null()
                            || stream_data_context.app_buffer < bytes0
                            || stream_data_context.app_buffer >= bytes_max
                        {
                            let mut delta_buf: ::core::ffi::c_longlong =
                                stream_data_context.app_buffer.offset_from(bytes)
                                    as ::core::ffi::c_long
                                    as ::core::ffi::c_longlong;
                            *ret = PICOQUIC_ERROR_UNEXPECTED_ERROR;
                        }
                    }
                    if stream_data_context.is_fin != 0 {
                        (*stream).set_is_active(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        (*stream)
                            .set_fin_requested(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        (*stream).set_fin_sent(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        picoquic_remove_output_stream(cnx, stream);
                        picoquic_update_max_stream_ID_local(cnx, stream);
                        may_close = 1 as ::core::ffi::c_int;
                        if !is_still_active.is_null() {
                            *is_still_active = 0 as ::core::ffi::c_int;
                        }
                    } else {
                        (*stream).set_is_active(
                            stream_data_context.is_still_active as ::core::ffi::c_uint
                                as ::core::ffi::c_uint,
                        );
                        if !is_still_active.is_null() {
                            *is_still_active = stream_data_context.is_still_active;
                        }
                    }
                }
            } else {
                let mut start_index: size_t = 0 as size_t;
                byte_index = bytes.offset_from(bytes0) as ::core::ffi::c_long as size_t;
                if (*stream).send_queue.is_null() {
                    length = 0 as size_t;
                } else {
                    length = (*(*stream).send_queue)
                        .length
                        .wrapping_sub((*(*stream).send_queue).offset as size_t);
                }
                if length >= allowed_space {
                    length = allowed_space;
                }
                byte_index = picoquic_encode_length_of_stream_frame(
                    bytes0,
                    byte_index,
                    byte_space,
                    length,
                    &raw mut start_index,
                );
                if length > 0 as size_t
                    && !(*stream).send_queue.is_null()
                    && !(*(*stream).send_queue).bytes.is_null()
                {
                    memcpy(
                        bytes0.offset(byte_index as isize) as *mut uint8_t
                            as *mut ::core::ffi::c_void,
                        (*(*stream).send_queue)
                            .bytes
                            .offset((*(*stream).send_queue).offset as isize)
                            as *const ::core::ffi::c_void,
                        length,
                    );
                    byte_index = byte_index.wrapping_add(length);
                    (*(*stream).send_queue).offset =
                        ((*(*stream).send_queue).offset as ::core::ffi::c_ulong)
                            .wrapping_add(length as ::core::ffi::c_ulong)
                            as uint64_t as uint64_t;
                    if (*(*stream).send_queue).offset >= (*(*stream).send_queue).length as uint64_t
                    {
                        let mut next: *mut picoquic_stream_queue_node_t = (*(*stream).send_queue)
                            .next_stream_data
                            as *mut picoquic_stream_queue_node_t;
                        free((*(*stream).send_queue).bytes as *mut ::core::ffi::c_void);
                        free((*stream).send_queue as *mut ::core::ffi::c_void);
                        (*stream).send_queue = next;
                    }
                    (*stream).sent_offset = ((*stream).sent_offset as ::core::ffi::c_ulong)
                        .wrapping_add(length as ::core::ffi::c_ulong)
                        as uint64_t as uint64_t;
                    (*stream).last_time_data_sent =
                        picoquic_get_quic_time((*cnx).quic as *mut picoquic_quic_t);
                    (*cnx).data_sent = ((*cnx).data_sent as ::core::ffi::c_ulong)
                        .wrapping_add(length as ::core::ffi::c_ulong)
                        as uint64_t as uint64_t;
                }
                bytes = bytes0.offset(byte_index as isize);
                if (*stream).send_queue.is_null() {
                    if (*stream).fin_requested() != 0 {
                        (*stream).set_fin_sent(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        let ref mut c2rust_fresh3 = *bytes0.offset(start_index as isize);
                        *c2rust_fresh3 = (*c2rust_fresh3 as ::core::ffi::c_int
                            | 1 as ::core::ffi::c_int)
                            as uint8_t;
                        picoquic_update_max_stream_ID_local(cnx, stream);
                        may_close = 1 as ::core::ffi::c_int;
                    }
                } else if length == 0 as size_t {
                    bytes = bytes0;
                    *more_data = 1 as ::core::ffi::c_int;
                }
            }
        }
        if *ret == 0 as ::core::ffi::c_int {
            *is_pure_ack &= (bytes == bytes0) as ::core::ffi::c_int;
            if may_close == 0 || picoquic_delete_stream_if_closed(cnx, stream) == 0 {
                (*stream)
                    .set_stream_data_blocked_sent(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*cnx).set_sent_blocked_frame(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_available_stream_frames(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut bytes_next: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut current_priority: uint64_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut stream_tried_and_failed: *mut ::core::ffi::c_int,
    mut ret: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut bytes_previous: *mut uint8_t = bytes_next;
    let mut stream: *mut picoquic_stream_head_t = picoquic_find_ready_stream_path(
        cnx,
        if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0 {
            path_x
        } else {
            ::core::ptr::null_mut::<picoquic_path_t>()
        },
    );
    let mut more_stream_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while *ret == 0 as ::core::ffi::c_int
        && !stream.is_null()
        && (*stream).stream_priority as uint64_t <= current_priority
        && bytes_next < bytes_max
    {
        let mut is_still_active: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        bytes_next = picoquic_format_stream_frame(
            cnx,
            stream,
            bytes_next,
            bytes_max,
            &raw mut more_stream_data,
            is_pure_ack,
            &raw mut is_still_active,
            ret,
        );
        if !(*ret == 0 as ::core::ffi::c_int) {
            break;
        }
        stream = picoquic_find_ready_stream_path(
            cnx,
            if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0 {
                path_x
            } else {
                ::core::ptr::null_mut::<picoquic_path_t>()
            },
        );
        if !(!stream.is_null() && bytes_next.offset(17 as ::core::ffi::c_int as isize) >= bytes_max)
        {
            continue;
        }
        more_stream_data = 1 as ::core::ffi::c_int;
        break;
    }
    *stream_tried_and_failed =
        (more_stream_data == 0 && bytes_next == bytes_previous) as ::core::ffi::c_int;
    if more_stream_data == 0 && current_priority != UINT64_MAX as uint64_t {
        more_stream_data |=
            (picoquic_find_ready_stream_path(cnx, ::core::ptr::null_mut::<picoquic_path_t>())
                != NULL as *mut picoquic_stream_head_t) as ::core::ffi::c_int;
    }
    *more_data |= more_stream_data;
    return bytes_next;
}
unsafe extern "C" fn picoquic_queue_data_repeat_node_create(
    mut value: *mut ::core::ffi::c_void,
) -> *mut picosplay_node_t {
    return &raw mut (*(value as *mut picoquic_packet_t)).queue_data_repeat_node;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_data_repeat_node_value(
    mut node: *mut picosplay_node_t,
) -> *mut ::core::ffi::c_void {
    return (node as *mut ::core::ffi::c_char).offset(-(24 as ::core::ffi::c_ulong as isize))
        as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_data_repeat_compare(
    mut l: *mut ::core::ffi::c_void,
    mut r: *mut ::core::ffi::c_void,
) -> int64_t {
    let mut lp: *mut picoquic_packet_t =
        picoquic_queue_data_repeat_node_value(l as *mut picosplay_node_t) as *mut picoquic_packet_t;
    let mut rp: *mut picoquic_packet_t =
        picoquic_queue_data_repeat_node_value(r as *mut picosplay_node_t) as *mut picoquic_packet_t;
    let mut ret: int64_t = 0 as int64_t;
    if (*lp).data_repeat_priority > (*rp).data_repeat_priority {
        ret = 1 as int64_t;
    } else if (*lp).data_repeat_priority < (*rp).data_repeat_priority {
        ret = -(1 as ::core::ffi::c_int) as int64_t;
    } else {
        ret = (*lp)
            .data_repeat_stream_id
            .wrapping_sub((*rp).data_repeat_stream_id) as int64_t;
        if ret == 0 as int64_t {
            ret = (*lp)
                .data_repeat_stream_offset
                .wrapping_sub((*rp).data_repeat_stream_offset) as int64_t;
            if ret == 0 as int64_t {
                ret = (*rp)
                    .data_repeat_stream_data_length
                    .wrapping_sub((*lp).data_repeat_stream_data_length)
                    as int64_t;
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_data_repeat_delete(
    mut tree: *mut ::core::ffi::c_void,
    mut node: *mut picosplay_node_t,
) {
    let mut packet: *mut picoquic_packet_t =
        picoquic_queue_data_repeat_node_value(node) as *mut picoquic_packet_t;
    let mut cnx: *mut picoquic_cnx_t = (tree as *mut ::core::ffi::c_char)
        .offset(-(4360 as ::core::ffi::c_ulong as isize))
        as *mut ::core::ffi::c_void as *mut picoquic_cnx_t;
    (*packet).set_is_queued_for_data_repeat(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if (*packet).is_queued_for_spurious_detection() == 0 {
        picoquic_recycle_packet((*cnx).quic as *mut picoquic_quic_t, packet);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_data_repeat_init(mut cnx: *mut picoquic_cnx_t) {
    picosplay_init_tree(
        &raw mut (*cnx).queue_data_repeat_tree,
        Some(
            picoquic_queue_data_repeat_compare
                as unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    *mut ::core::ffi::c_void,
                ) -> int64_t,
        ),
        Some(
            picoquic_queue_data_repeat_node_create
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut picosplay_node_t,
        ),
        Some(
            picoquic_queue_data_repeat_delete
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut picosplay_node_t) -> (),
        ),
        Some(
            picoquic_queue_data_repeat_node_value
                as unsafe extern "C" fn(*mut picosplay_node_t) -> *mut ::core::ffi::c_void,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_dequeue_data_repeat_packet(
    mut cnx: *mut picoquic_cnx_t,
    mut packet: *mut picoquic_packet_t,
) {
    picosplay_delete_hint(
        &raw mut (*cnx).queue_data_repeat_tree,
        &raw mut (*packet).queue_data_repeat_node,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_data_repeat_adjust(
    mut cnx: *mut picoquic_cnx_t,
    mut packet: *mut picoquic_packet_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (*packet).data_repeat_frame < (*packet).length {
        let mut data_byte: *mut uint8_t =
            (&raw mut (*packet).bytes as *mut uint8_t).offset((*packet).data_repeat_frame as isize);
        if *data_byte as ::core::ffi::c_int
            >= picoquic_frame_type_stream_range_min as ::core::ffi::c_int
            && *data_byte as ::core::ffi::c_int
                <= picoquic_frame_type_stream_range_max as ::core::ffi::c_int
        {
            let mut consumed: size_t = 0;
            let mut fin: ::core::ffi::c_int = 0;
            (*packet).data_repeat_priority = 0 as uint64_t;
            (*packet).data_repeat_stream_id = 0 as uint64_t;
            (*packet).data_repeat_stream_offset = 0 as uint64_t;
            (*packet).data_repeat_stream_data_length = 0 as size_t;
            if picoquic_parse_stream_header(
                data_byte,
                (*packet).length.wrapping_sub((*packet).data_repeat_frame),
                &raw mut (*packet).data_repeat_stream_id,
                &raw mut (*packet).data_repeat_stream_offset,
                &raw mut (*packet).data_repeat_stream_data_length,
                &raw mut fin,
                &raw mut consumed,
            ) == 0 as ::core::ffi::c_int
            {
                let mut stream: *mut picoquic_stream_head_t =
                    picoquic_find_stream(cnx, (*packet).data_repeat_stream_id);
                if stream.is_null() {
                    (*packet).data_repeat_priority = 0 as uint64_t;
                } else {
                    (*packet).data_repeat_priority = (*stream).stream_priority as uint64_t;
                }
            } else {
                ret = -(1 as ::core::ffi::c_int);
            }
            break;
        } else {
            let mut forget_about_ack: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut consumed_0: size_t = 0 as size_t;
            if picoquic_skip_frame(
                data_byte,
                (*packet).length.wrapping_sub((*packet).data_repeat_frame),
                &raw mut consumed_0,
                &raw mut forget_about_ack,
            ) != 0 as ::core::ffi::c_int
            {
                ret = -(1 as ::core::ffi::c_int);
                break;
            } else {
                (*packet).data_repeat_frame = (*packet).data_repeat_frame.wrapping_add(consumed_0);
                (*packet).data_repeat_index = (*packet).data_repeat_frame;
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_data_repeat_packet(
    mut cnx: *mut picoquic_cnx_t,
    mut packet: *mut picoquic_packet_t,
) {
    if (*packet).is_queued_for_data_repeat() == 0 {
        (*packet).data_repeat_frame = (*packet).offset;
        (*packet).data_repeat_index = (*packet).offset;
        if picoquic_queue_data_repeat_adjust(cnx, packet) == 0 as ::core::ffi::c_int
            && (*packet).data_repeat_frame < (*packet).length
        {
            picosplay_insert(
                &raw mut (*cnx).queue_data_repeat_tree,
                packet as *mut ::core::ffi::c_void,
            );
            (*packet)
                .set_is_queued_for_data_repeat(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_first_data_repeat_packet(
    mut cnx: *mut picoquic_cnx_t,
) -> *mut picoquic_packet_t {
    let mut first_node: *mut picosplay_node_t =
        picosplay_first(&raw mut (*cnx).queue_data_repeat_tree);
    let mut first_packet: *mut picoquic_packet_t = (if first_node.is_null() {
        NULL
    } else {
        picoquic_queue_data_repeat_node_value(first_node)
    }) as *mut picoquic_packet_t;
    return first_packet;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_copy_stream_frame_for_retransmit(
    mut cnx: *mut picoquic_cnx_t,
    mut packet: *mut picoquic_packet_t,
    mut bytes_next: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
) -> *mut uint8_t {
    let mut frame: *mut uint8_t =
        (&raw mut (*packet).bytes as *mut uint8_t).offset((*packet).data_repeat_frame as isize);
    let mut frame_length_max: size_t = (*packet).length.wrapping_sub((*packet).data_repeat_frame);
    let mut stream_id: uint64_t = 0;
    let mut offset: uint64_t = 0;
    let mut data_length: size_t = 0;
    let mut consumed: size_t = 0;
    let mut bytes_not_sent: size_t = 0 as size_t;
    let mut fin: ::core::ffi::c_int = 0;
    if picoquic_parse_stream_header(
        frame,
        frame_length_max,
        &raw mut stream_id,
        &raw mut offset,
        &raw mut data_length,
        &raw mut fin,
        &raw mut consumed,
    ) != 0 as ::core::ffi::c_int
    {
        bytes_next = ::core::ptr::null_mut::<uint8_t>();
    } else {
        let mut bytes_first: *mut uint8_t = bytes_next;
        let mut data_available: size_t = data_length;
        let mut frame_bytes: *mut uint8_t = frame.offset(consumed as isize);
        let mut is_needed: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        if (*packet).data_repeat_index > (*packet).data_repeat_frame.wrapping_add(consumed) {
            let mut already_sent: size_t = (*packet)
                .data_repeat_index
                .wrapping_sub((*packet).data_repeat_frame)
                .wrapping_sub(consumed);
            if already_sent <= data_length {
                offset = (offset as ::core::ffi::c_ulong)
                    .wrapping_add(already_sent as ::core::ffi::c_ulong)
                    as uint64_t as uint64_t;
                frame_bytes = frame_bytes.offset(already_sent as isize);
                data_available = data_available.wrapping_sub(already_sent);
            } else {
                offset = (offset as ::core::ffi::c_ulong)
                    .wrapping_add(data_length as ::core::ffi::c_ulong)
                    as uint64_t as uint64_t;
                frame_bytes = frame_bytes.offset(data_length as isize);
                data_available = 0 as size_t;
            }
        }
        if !cnx.is_null() {
            let mut stream: *mut picoquic_stream_head_t = picoquic_find_stream(cnx, stream_id);
            if stream.is_null()
                || (*stream).reset_sent() as ::core::ffi::c_int != 0
                || picoquic_check_sack_list(
                    &raw mut (*stream).sack_list,
                    offset,
                    offset
                        .wrapping_add(data_available as uint64_t)
                        .wrapping_sub(
                            (if fin != 0 {
                                0 as ::core::ffi::c_int
                            } else {
                                1 as ::core::ffi::c_int
                            }) as uint64_t,
                        ),
                ) != 0
            {
                is_needed = 0 as ::core::ffi::c_int;
            }
        }
        if is_needed != 0 {
            bytes_next =
                picoquic_format_stream_frame_header(bytes_next, bytes_max, stream_id, offset);
            if bytes_next.is_null() || bytes_next == bytes_max {
                bytes_not_sent = data_available;
                bytes_next = bytes_first;
            } else {
                let mut before_length: *mut uint8_t = bytes_next;
                bytes_next = picoquic_frames_varint_encode(
                    bytes_next,
                    bytes_max,
                    data_available as uint64_t,
                );
                if !bytes_next.is_null() && bytes_next.offset(data_available as isize) <= bytes_max
                {
                    *bytes_first =
                        (*bytes_first as ::core::ffi::c_int | 2 as ::core::ffi::c_int) as uint8_t;
                    *bytes_first = (*bytes_first as ::core::ffi::c_int | fin) as uint8_t;
                    memcpy(
                        bytes_next as *mut ::core::ffi::c_void,
                        frame_bytes as *const ::core::ffi::c_void,
                        data_available,
                    );
                    bytes_next = bytes_next.offset(data_available as isize);
                } else if before_length.offset(data_available as isize) <= bytes_max {
                    let mut space_available: size_t =
                        bytes_max.offset_from(before_length) as ::core::ffi::c_long as size_t;
                    let mut pad_required: size_t = space_available.wrapping_sub(data_available);
                    bytes_next = before_length;
                    *bytes_first = (*bytes_first as ::core::ffi::c_int | fin) as uint8_t;
                    if pad_required > 0 as size_t {
                        memmove(
                            bytes_first.offset(pad_required as isize) as *mut ::core::ffi::c_void,
                            bytes_first as *const ::core::ffi::c_void,
                            before_length.offset_from(bytes_first) as ::core::ffi::c_long as size_t,
                        );
                        let mut i: size_t = 0 as size_t;
                        while i < pad_required {
                            *bytes_first.offset(i as isize) = 0 as uint8_t;
                            i = i.wrapping_add(1);
                        }
                        bytes_next = bytes_next.offset(pad_required as isize);
                    }
                    memcpy(
                        bytes_next as *mut ::core::ffi::c_void,
                        frame_bytes as *const ::core::ffi::c_void,
                        data_available,
                    );
                    bytes_next = bytes_next.offset(data_available as isize);
                } else {
                    let mut available: size_t =
                        bytes_max.offset_from(before_length) as ::core::ffi::c_long as size_t;
                    bytes_next = before_length;
                    memcpy(
                        bytes_next as *mut ::core::ffi::c_void,
                        frame_bytes as *const ::core::ffi::c_void,
                        available,
                    );
                    bytes_next = bytes_next.offset(available as isize);
                    bytes_not_sent = data_available.wrapping_sub(available);
                }
            }
        }
        if bytes_not_sent == 0 as size_t {
            (*packet).data_repeat_index = (*packet)
                .data_repeat_frame
                .wrapping_add(consumed)
                .wrapping_add(data_length);
            (*packet).data_repeat_frame = (*packet).data_repeat_index;
        } else if bytes_not_sent < data_length {
            (*packet).data_repeat_index = (*packet)
                .data_repeat_frame
                .wrapping_add(consumed)
                .wrapping_add(data_length)
                .wrapping_sub(bytes_not_sent);
        }
    }
    return bytes_next;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_copy_single_stream_frame_for_retransmit(
    mut cnx: *mut picoquic_cnx_t,
    mut packet: *mut picoquic_packet_t,
    mut bytes_next: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut current_priority: uint64_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut packet_dequeued: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut last_frame: size_t = (*packet).data_repeat_frame;
    if (*packet).data_repeat_frame < (*packet).length {
        let mut data_byte: *mut uint8_t =
            (&raw mut (*packet).bytes as *mut uint8_t).offset((*packet).data_repeat_frame as isize);
        if *data_byte as ::core::ffi::c_int
            >= picoquic_frame_type_stream_range_min as ::core::ffi::c_int
            && *data_byte as ::core::ffi::c_int
                <= picoquic_frame_type_stream_range_max as ::core::ffi::c_int
        {
            let mut bytes_first: *mut uint8_t = bytes_next;
            bytes_next =
                picoquic_copy_stream_frame_for_retransmit(cnx, packet, bytes_next, bytes_max);
            if !bytes_next.is_null() && bytes_next > bytes_first {
                *is_pure_ack &= 0 as ::core::ffi::c_int;
            }
        }
    }
    if (*packet).data_repeat_frame < (*packet).length
        && picoquic_queue_data_repeat_adjust(cnx, packet) != 0 as ::core::ffi::c_int
    {
        bytes_next = ::core::ptr::null_mut::<uint8_t>();
    }
    if (*packet).data_repeat_frame >= (*packet).length {
        picoquic_dequeue_data_repeat_packet(cnx, packet);
        *packet_dequeued = 1 as ::core::ffi::c_int;
    } else if (*packet).data_repeat_frame > last_frame {
        let mut was_queued: ::core::ffi::c_int =
            (*packet).is_queued_for_spurious_detection() as ::core::ffi::c_int;
        (*packet)
            .set_is_queued_for_spurious_detection(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        picosplay_delete_hint(
            &raw mut (*cnx).queue_data_repeat_tree,
            &raw mut (*packet).queue_data_repeat_node,
        );
        (*packet).set_is_queued_for_spurious_detection(
            was_queued as ::core::ffi::c_uint as ::core::ffi::c_uint,
        );
        picosplay_insert(
            &raw mut (*cnx).queue_data_repeat_tree,
            packet as *mut ::core::ffi::c_void,
        );
        (*packet).set_is_queued_for_data_repeat(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        *more_data |= 1 as ::core::ffi::c_int;
    } else {
        *more_data |= 1 as ::core::ffi::c_int;
    }
    return bytes_next;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_copy_stream_frames_for_retransmit(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes_next: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut current_priority: uint64_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut more_retransmit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut packet_dequeued: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bytes_first: *mut uint8_t = bytes_next;
    let mut packet: *mut picoquic_packet_t = ::core::ptr::null_mut::<picoquic_packet_t>();
    loop {
        packet_dequeued = 0 as ::core::ffi::c_int;
        packet = picoquic_first_data_repeat_packet(cnx);
        if packet.is_null() {
            break;
        }
        if (*packet).data_repeat_priority > current_priority {
            more_retransmit = 1 as ::core::ffi::c_int;
            break;
        } else {
            more_retransmit = 0 as ::core::ffi::c_int;
            bytes_next = picoquic_copy_single_stream_frame_for_retransmit(
                cnx,
                packet,
                bytes_next,
                bytes_max,
                current_priority,
                &raw mut more_retransmit,
                &raw mut packet_dequeued,
                is_pure_ack,
            );
            if !(!bytes_next.is_null() && packet_dequeued != 0 && bytes_next < bytes_max) {
                break;
            }
        }
    }
    if bytes_next.is_null() {
        picoquic_connection_error_ex(
            cnx,
            PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t,
            0 as uint64_t,
            b"data frame was fuzzed, cannot be resent\0".as_ptr() as *const ::core::ffi::c_char,
        );
        bytes_next = bytes_first;
    }
    if packet_dequeued != 0 {
        more_retransmit = (picoquic_first_data_repeat_packet(cnx) != NULL as *mut picoquic_packet_t)
            as ::core::ffi::c_int;
    }
    *more_data |= more_retransmit;
    return bytes_next;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_tls_stream_ready(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut epoch: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while epoch < 4 as ::core::ffi::c_int {
        let mut stream: *mut picoquic_stream_head_t =
            (&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t).offset(epoch as isize)
                as *mut picoquic_stream_head_t;
        if !(*stream).send_queue.is_null()
            && (*(*stream).send_queue).length > (*(*stream).send_queue).offset as size_t
            && !(*cnx).crypto_context[epoch as usize].aead_encrypt.is_null()
        {
            ret = 1 as ::core::ffi::c_int;
            break;
        } else {
            epoch += 1;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_crypto_hs_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut offset: *mut uint64_t,
    mut data_length: *mut uint64_t,
    mut data_bytes: *mut *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max,
        offset,
    );
    if !bytes.is_null() && {
        bytes = picoquic_frames_varint_decode(bytes, bytes_max, data_length);
        !bytes.is_null()
    } {
        *data_bytes = bytes;
        bytes = picoquic_frames_fixed_skip(bytes, bytes_max, *data_length);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_crypto_hs_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut received_data: *mut picoquic_stream_data_node_t,
    mut epoch: ::core::ffi::c_int,
) -> *const uint8_t {
    let mut offset: uint64_t = 0;
    let mut data_length: uint64_t = 0;
    let mut data_bytes: *const uint8_t = ::core::ptr::null::<uint8_t>();
    bytes = picoquic_parse_crypto_hs_frame(
        bytes,
        bytes_max,
        &raw mut offset,
        &raw mut data_length,
        &raw mut data_bytes,
    );
    if bytes.is_null() {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_crypto_hs as ::core::ffi::c_int as uint64_t,
        );
    } else {
        let mut stream: *mut picoquic_stream_head_t =
            (&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t).offset(epoch as isize)
                as *mut picoquic_stream_head_t;
        if (*stream).consumed_offset < offset
            && (*stream)
                .consumed_offset
                .wrapping_add(PICOQUIC_MAX_CRYPTO_BUFFER_GAP as uint64_t)
                < offset.wrapping_add(data_length)
        {
            picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_CRYPTO_BUFFER_EXCEEDED as uint64_t,
                picoquic_frame_type_crypto_hs as ::core::ffi::c_int as uint64_t,
            );
            bytes = ::core::ptr::null::<uint8_t>();
        } else {
            let mut new_data_available: ::core::ffi::c_int = 0;
            let mut ret: ::core::ffi::c_int = picoquic_queue_network_input(
                (*cnx).quic,
                &raw mut (*stream).stream_data_tree,
                (*stream).consumed_offset,
                offset,
                data_bytes,
                data_length as size_t,
                picoquic_is_last_stream_frame(bytes.offset(data_length as isize), bytes_max),
                received_data,
                &raw mut new_data_available,
            );
            if ret != 0 as ::core::ffi::c_int {
                picoquic_connection_error(
                    cnx,
                    ret as int64_t as uint64_t,
                    picoquic_frame_type_crypto_hs as ::core::ffi::c_int as uint64_t,
                );
                bytes = ::core::ptr::null::<uint8_t>();
            }
        }
    }
    return bytes;
}
unsafe extern "C" fn picoquic_crypto_stream_from_ptype(
    mut cnx: *mut picoquic_cnx_t,
    mut p_type: picoquic_packet_type_enum,
) -> *mut picoquic_stream_head_t {
    let mut stream: *mut picoquic_stream_head_t = ::core::ptr::null_mut::<picoquic_stream_head_t>();
    match p_type as ::core::ffi::c_uint {
        2 => {
            stream = (&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t)
                .offset(picoquic_epoch_initial as ::core::ffi::c_int as isize)
                as *mut picoquic_stream_head_t;
        }
        5 => {
            stream = (&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t)
                .offset(picoquic_epoch_0rtt as ::core::ffi::c_int as isize)
                as *mut picoquic_stream_head_t;
        }
        4 => {
            stream = (&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t)
                .offset(picoquic_epoch_handshake as ::core::ffi::c_int as isize)
                as *mut picoquic_stream_head_t;
        }
        6 => {
            stream = (&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t)
                .offset(picoquic_epoch_1rtt as ::core::ffi::c_int as isize)
                as *mut picoquic_stream_head_t;
        }
        _ => {}
    }
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_ack_of_crypto_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_size: size_t,
    mut p_type: picoquic_packet_type_enum,
    mut consumed: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut offset: uint64_t = 0 as uint64_t;
    let mut data_length: uint64_t = 0 as uint64_t;
    let mut data_bytes: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut byte_zero: *const uint8_t = bytes;
    bytes = picoquic_parse_crypto_hs_frame(
        bytes,
        bytes.offset(bytes_size as isize),
        &raw mut offset,
        &raw mut data_length,
        &raw mut data_bytes,
    );
    if bytes.is_null() {
        *consumed = bytes_size;
        ret = -(1 as ::core::ffi::c_int);
    } else {
        let mut stream: *mut picoquic_stream_head_t =
            picoquic_crypto_stream_from_ptype(cnx, p_type);
        *consumed = bytes.offset_from(byte_zero) as ::core::ffi::c_long as size_t;
        if !stream.is_null() {
            picoquic_update_sack_list(
                &raw mut (*stream).sack_list,
                offset,
                offset.wrapping_add(data_length).wrapping_sub(1 as uint64_t),
                0 as uint64_t,
            );
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_check_crypto_frame_needs_repeat(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_size: size_t,
    mut p_type: picoquic_packet_type_enum,
    mut no_need_to_repeat: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut offset: uint64_t = 0 as uint64_t;
    let mut data_length: uint64_t = 0 as uint64_t;
    let mut data_bytes: *const uint8_t = ::core::ptr::null::<uint8_t>();
    bytes = picoquic_parse_crypto_hs_frame(
        bytes,
        bytes.offset(bytes_size as isize),
        &raw mut offset,
        &raw mut data_length,
        &raw mut data_bytes,
    );
    if bytes.is_null() {
        *no_need_to_repeat = 1 as ::core::ffi::c_int;
        ret = -(1 as ::core::ffi::c_int);
    } else {
        let mut stream: *mut picoquic_stream_head_t =
            picoquic_crypto_stream_from_ptype(cnx, p_type);
        if stream.is_null() {
            *no_need_to_repeat = 1 as ::core::ffi::c_int;
        } else {
            *no_need_to_repeat = picoquic_check_sack_list(
                &raw mut (*stream).sack_list,
                offset,
                offset.wrapping_add(data_length).wrapping_sub(1 as uint64_t),
            );
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_crypto_hs_frame(
    mut stream: *mut picoquic_stream_head_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    if !(*stream).send_queue.is_null()
        && (*(*stream).send_queue).length > (*(*stream).send_queue).offset as size_t
    {
        bytes = picoquic_frames_uint8_encode(
            bytes,
            bytes_max,
            picoquic_frame_type_crypto_hs as ::core::ffi::c_int as uint8_t,
        );
        if !bytes.is_null() && {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, (*stream).sent_offset);
            !bytes.is_null()
        } {
            let mut length: size_t = (*(*stream).send_queue)
                .length
                .wrapping_sub((*(*stream).send_queue).offset as size_t);
            let mut bytes_l: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
            if bytes.offset(length as isize) > bytes_max {
                length = bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t;
            }
            bytes_l = picoquic_frames_varint_encode(bytes, bytes_max, length as uint64_t);
            if bytes_l.is_null() {
                bytes = bytes0;
            } else {
                if bytes_l.offset(length as isize) > bytes_max {
                    length = bytes_max.offset_from(bytes_l) as ::core::ffi::c_long as size_t;
                    bytes = picoquic_frames_varint_encode(bytes, bytes_max, length as uint64_t);
                } else {
                    bytes = bytes_l;
                }
                if !bytes.is_null() && length > 0 as size_t {
                    memcpy(
                        bytes as *mut ::core::ffi::c_void,
                        (*(*stream).send_queue)
                            .bytes
                            .offset((*(*stream).send_queue).offset as isize)
                            as *const ::core::ffi::c_void,
                        length,
                    );
                    bytes = bytes.offset(length as isize);
                    (*(*stream).send_queue).offset =
                        ((*(*stream).send_queue).offset as ::core::ffi::c_ulong)
                            .wrapping_add(length as ::core::ffi::c_ulong)
                            as uint64_t as uint64_t;
                    if (*(*stream).send_queue).offset >= (*(*stream).send_queue).length as uint64_t
                    {
                        let mut next: *mut picoquic_stream_queue_node_t = (*(*stream).send_queue)
                            .next_stream_data
                            as *mut picoquic_stream_queue_node_t;
                        free((*(*stream).send_queue).bytes as *mut ::core::ffi::c_void);
                        free((*stream).send_queue as *mut ::core::ffi::c_void);
                        (*stream).send_queue = next;
                    }
                    (*stream).sent_offset = ((*stream).sent_offset as ::core::ffi::c_ulong)
                        .wrapping_add(length as ::core::ffi::c_ulong)
                        as uint64_t as uint64_t;
                    *is_pure_ack = 0 as ::core::ffi::c_int;
                }
            }
        } else {
            bytes = bytes0;
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_ack_header(
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut num_block: *mut uint64_t,
    mut path_id: *mut uint64_t,
    mut largest: *mut uint64_t,
    mut ack_delay: *mut uint64_t,
    mut consumed: *mut size_t,
    mut ack_delay_exponent: uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut byte_index: size_t =
        picoquic_decode_varint_length(*bytes.offset(0 as ::core::ffi::c_int as isize));
    let mut l_largest: size_t = 0 as size_t;
    let mut l_delay: size_t = 0 as size_t;
    let mut l_blocks: size_t = 0 as size_t;
    let mut l_path_id: size_t = 0 as size_t;
    if !path_id.is_null() && bytes_max > byte_index {
        l_path_id = picoquic_varint_decode(
            bytes.offset(byte_index as isize),
            bytes_max.wrapping_sub(byte_index),
            path_id,
        );
        byte_index = byte_index.wrapping_add(l_path_id);
    }
    if bytes_max > byte_index {
        l_largest = picoquic_varint_decode(
            bytes.offset(byte_index as isize),
            bytes_max.wrapping_sub(byte_index),
            largest,
        );
        byte_index = byte_index.wrapping_add(l_largest);
    }
    if bytes_max > byte_index {
        l_delay = picoquic_varint_decode(
            bytes.offset(byte_index as isize),
            bytes_max.wrapping_sub(byte_index),
            ack_delay,
        );
        *ack_delay <<= ack_delay_exponent as ::core::ffi::c_int;
        byte_index = byte_index.wrapping_add(l_delay);
    }
    if bytes_max > byte_index {
        l_blocks = picoquic_varint_decode(
            bytes.offset(byte_index as isize),
            bytes_max.wrapping_sub(byte_index),
            num_block,
        );
        byte_index = byte_index.wrapping_add(l_blocks);
    }
    if l_largest == 0 as size_t
        || l_delay == 0 as size_t
        || l_blocks == 0 as size_t
        || bytes_max < byte_index
        || !path_id.is_null() && l_path_id == 0 as size_t
    {
        byte_index = bytes_max;
        ret = -(1 as ::core::ffi::c_int);
    }
    *consumed = byte_index;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_check_spurious_retransmission(
    mut cnx: *mut picoquic_cnx_t,
    mut pc: picoquic_packet_context_enum,
    mut pkt_ctx: *mut picoquic_packet_context_t,
    mut start_of_range: uint64_t,
    mut end_of_range: uint64_t,
    mut current_time: uint64_t,
    mut time_stamp: uint64_t,
    mut p: *mut picoquic_packet_t,
    mut packet_data: *mut picoquic_packet_data_t,
) -> *mut picoquic_packet_t {
    while !p.is_null() && (*p).sequence_number >= start_of_range {
        let mut should_delete: *mut picoquic_packet_t =
            ::core::ptr::null_mut::<picoquic_packet_t>();
        if (*p).sequence_number <= end_of_range {
            let mut spurious_rtt: uint64_t = current_time.wrapping_sub((*p).send_time);
            let mut reorder_delay: uint64_t = (*pkt_ctx)
                .latest_time_acknowledged
                .wrapping_sub((*p).send_time);
            let mut reorder_gap: uint64_t = (*pkt_ctx)
                .highest_acknowledged
                .wrapping_sub((*p).sequence_number);
            let mut old_path: *mut picoquic_path_t = (*p).send_path as *mut picoquic_path_t;
            picoquic_process_ack_of_frames(
                cnx,
                p,
                packet_data,
                1 as ::core::ffi::c_int,
                current_time,
            );
            if !old_path.is_null() {
                (*old_path).nb_spurious = (*old_path).nb_spurious.wrapping_add(1);
                if (*p).sequence_number >= picoquic_get_ack_number(cnx, old_path, pc) {
                    (*old_path).nb_retransmit = 0 as uint64_t;
                }
                picoquic_record_ack_packet_data(packet_data, p);
                if (*p).length.wrapping_add((*p).checksum_overhead) > (*old_path).send_mtu {
                    (*old_path).send_mtu = (*p).length.wrapping_add((*p).checksum_overhead);
                    if (*old_path).send_mtu > (*old_path).send_mtu_max_tried {
                        (*old_path).send_mtu_max_tried = (*old_path).send_mtu;
                    }
                    (*old_path).set_mtu_probe_sent(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
                if spurious_rtt > (*old_path).max_spurious_rtt {
                    (*old_path).max_spurious_rtt = spurious_rtt;
                }
                if reorder_delay > (*old_path).max_reorder_delay {
                    (*old_path).max_reorder_delay = reorder_delay;
                }
                if reorder_gap > (*old_path).max_reorder_gap {
                    (*old_path).max_reorder_gap = reorder_gap;
                }
                if (*old_path).total_bytes_lost > (*p).length as uint64_t {
                    (*old_path).total_bytes_lost = ((*old_path).total_bytes_lost
                        as ::core::ffi::c_ulong)
                        .wrapping_sub((*p).length as ::core::ffi::c_ulong)
                        as uint64_t as uint64_t;
                } else {
                    (*old_path).total_bytes_lost = 0 as uint64_t;
                }
                if !(*cnx).congestion_alg.is_null() {
                    let mut ack_state: picoquic_per_ack_state_t = {
                        let mut init = st_picoquic_per_ack_state_t {
                            is_app_limited_is_cwnd_limited: [0; 1],
                            c2rust_padding: [0; 7],
                            rtt_measurement: 0 as uint64_t,
                            one_way_delay: 0,
                            nb_bytes_acknowledged: 0,
                            nb_bytes_newly_lost: 0,
                            nb_bytes_lost_since_packet_sent: 0,
                            nb_bytes_delivered_since_packet_sent: 0,
                            inflight_prior: 0,
                            lost_packet_number: 0,
                            lost_packet_sent_time: 0,
                        };
                        init.set_is_app_limited(0);
                        init.set_is_cwnd_limited(0);
                        init
                    };
                    ack_state.lost_packet_number = (*p).sequence_number;
                    (*(*cnx).congestion_alg)
                        .alg_notify
                        .expect("non-null function pointer")(
                        cnx as *mut picoquic_cnx_t,
                        old_path as *mut picoquic_path_t,
                        picoquic_congestion_notification_spurious_repeat,
                        &raw mut ack_state,
                        current_time,
                    );
                }
            }
            (*cnx).nb_spurious = (*cnx).nb_spurious.wrapping_add(1);
            should_delete = p;
        }
        p = (*p).packet_next as *mut picoquic_packet_t;
        if !should_delete.is_null() {
            picoquic_dequeue_retransmitted_packet(cnx, pkt_ctx, should_delete);
        }
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_dequeue_old_retransmitted_packets(
    mut cnx: *mut picoquic_cnx_t,
    mut pkt_ctx: *mut picoquic_packet_context_t,
) {
    let mut p: *mut picoquic_packet_t = (*pkt_ctx).retransmitted_oldest;
    if !p.is_null() {
        let mut oldest_possible: uint64_t = (*pkt_ctx).latest_time_acknowledged;
        if oldest_possible as ::core::ffi::c_ulonglong > PICOQUIC_SPURIOUS_RETRANSMIT_DELAY_MAX {
            oldest_possible = (oldest_possible as ::core::ffi::c_ulonglong)
                .wrapping_sub(PICOQUIC_SPURIOUS_RETRANSMIT_DELAY_MAX)
                as uint64_t as uint64_t;
            while !p.is_null() && (*p).send_time < oldest_possible {
                let mut should_delete: *mut picoquic_packet_t = p;
                p = (*p).packet_previous as *mut picoquic_packet_t;
                if !should_delete.is_null() {
                    picoquic_dequeue_retransmitted_packet(cnx, pkt_ctx, should_delete);
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_estimate_path_bandwidth(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut send_time: uint64_t,
    mut delivered_prior: uint64_t,
    mut delivered_time_prior: uint64_t,
    mut delivered_sent_prior: uint64_t,
    mut delivery_time: uint64_t,
    mut current_time: uint64_t,
    mut rs_is_path_limited: ::core::ffi::c_int,
) {
    if send_time >= (*path_x).delivered_sent_last {
        if (*path_x).delivered_time_last == 0 as uint64_t {
            (*path_x).delivered_last = (*path_x).delivered;
            (*path_x).delivered_time_last = delivery_time;
            (*path_x).delivered_sent_last = send_time;
        } else {
            let mut receive_interval: uint64_t = delivery_time.wrapping_sub(delivered_time_prior);
            if receive_interval > PICOQUIC_BANDWIDTH_TIME_INTERVAL_MIN as uint64_t {
                let mut delivered: uint64_t = (*path_x).delivered.wrapping_sub(delivered_prior);
                let mut send_interval: uint64_t = send_time.wrapping_sub(delivered_sent_prior);
                let mut bw_estimate: uint64_t = 0;
                if send_interval > receive_interval {
                    receive_interval = send_interval;
                }
                bw_estimate = delivered.wrapping_mul(1000000 as uint64_t);
                bw_estimate = bw_estimate.wrapping_div(receive_interval);
                (*path_x).bandwidth_estimate = bw_estimate;
                if rs_is_path_limited == 0 || bw_estimate > (*path_x).bandwidth_estimate {
                    if path_x == *(*cnx).path.offset(0 as ::core::ffi::c_int as isize) {
                        if (*cnx).is_ack_frequency_negotiated() != 0 {
                            let mut ack_gap: uint64_t = 0;
                            let mut ack_delay_max: uint64_t = 0;
                            picoquic_compute_ack_gap_and_delay(
                                cnx,
                                (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).rtt_min,
                                (*cnx).remote_parameters.min_ack_delay,
                                bw_estimate,
                                &raw mut ack_gap,
                                &raw mut ack_delay_max,
                            );
                            if ack_gap != (*cnx).ack_gap_local {
                                (*cnx).set_is_ack_frequency_updated(
                                    1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                );
                            }
                        }
                    }
                }
                (*path_x).delivered_last = (*path_x).delivered;
                (*path_x).delivered_time_last = delivery_time;
                (*path_x).delivered_sent_last = send_time;
                (*path_x).delivered_last_packet = delivered_prior;
                (*path_x).set_last_bw_estimate_path_limited(
                    rs_is_path_limited as ::core::ffi::c_uint as ::core::ffi::c_uint,
                );
                if (*path_x).delivered_last_packet > (*path_x).delivered_limited_index {
                    (*path_x).delivered_limited_index = 0 as uint64_t;
                }
                if bw_estimate > (*path_x).bandwidth_estimate_max {
                    (*path_x).bandwidth_estimate_max = bw_estimate;
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_estimate_max_path_bandwidth(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut send_time: uint64_t,
    mut delivery_time: uint64_t,
    mut current_time: uint64_t,
) {
    if send_time >= (*path_x).max_sample_sent_time {
        if (*path_x).max_sample_sent_time == 0 as uint64_t {
            (*path_x).max_sample_delivered = (*path_x).delivered;
            (*path_x).max_sample_acked_time = delivery_time;
            (*path_x).max_sample_sent_time = send_time;
        } else {
            let mut receive_interval: uint64_t =
                delivery_time.wrapping_sub((*path_x).max_sample_acked_time);
            if receive_interval > PICOQUIC_MAX_BANDWIDTH_TIME_INTERVAL_MIN as uint64_t {
                let mut delivered: uint64_t = (*path_x)
                    .delivered
                    .wrapping_sub((*path_x).max_sample_delivered);
                let mut send_interval: uint64_t =
                    send_time.wrapping_sub((*path_x).max_sample_sent_time);
                let mut bw_estimate: uint64_t = 0;
                if send_interval > receive_interval {
                    receive_interval = send_interval;
                }
                bw_estimate = delivered.wrapping_mul(1000000 as uint64_t);
                bw_estimate = bw_estimate.wrapping_div(receive_interval);
                if bw_estimate > (*path_x).peak_bandwidth_estimate {
                    (*path_x).peak_bandwidth_estimate = bw_estimate;
                }
                (*path_x).max_sample_delivered = (*path_x).delivered;
                (*path_x).max_sample_acked_time = delivery_time;
                (*path_x).max_sample_sent_time = send_time;
            }
        }
    }
}
unsafe extern "C" fn picoquic_compute_packets_in_window(
    mut cnx: *mut picoquic_cnx_t,
    mut data_rate: uint64_t,
) -> uint64_t {
    let mut nb_packets: uint64_t = 0 as uint64_t;
    if (*cnx).is_ack_frequency_negotiated() != 0 {
        nb_packets = (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
            .cwin
            .wrapping_div(
                (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).send_mtu as uint64_t,
            );
    } else {
        let mut rtt_bytes_times_1000000: uint64_t = data_rate
            .wrapping_mul((**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).smoothed_rtt);
        let mut rtt_packets_times_1000000: uint64_t = rtt_bytes_times_1000000.wrapping_div(
            (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).send_mtu as uint64_t,
        );
        nb_packets = rtt_packets_times_1000000
            .wrapping_add(999999 as uint64_t)
            .wrapping_div(1000000 as uint64_t);
    }
    if nb_packets < 2 as uint64_t {
        nb_packets = 2 as uint64_t;
    }
    return nb_packets;
}
unsafe extern "C" fn picoquic_compute_ack_gap(
    mut cnx: *mut picoquic_cnx_t,
    mut data_rate: uint64_t,
    mut nb_packets: uint64_t,
) -> uint64_t {
    let mut ack_gap: uint64_t = 0;
    let mut ack_gap_min: uint64_t = 2 as uint64_t;
    if (*cnx).is_ack_frequency_negotiated() as ::core::ffi::c_int != 0
        && (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).is_ssthresh_initialized() == 0
    {
        nb_packets = nb_packets.wrapping_div(2 as uint64_t);
    }
    if ((**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).rtt_min
        as ::core::ffi::c_ulonglong)
        < (4 as ::core::ffi::c_ulonglong).wrapping_mul(PICOQUIC_ACK_DELAY_MIN)
    {
        let mut mult: uint64_t = 4 as uint64_t;
        if (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).rtt_min
            as ::core::ffi::c_ulonglong
            > PICOQUIC_ACK_DELAY_MIN
        {
            mult = ((4 as ::core::ffi::c_ulonglong).wrapping_mul(PICOQUIC_ACK_DELAY_MIN)
                as uint64_t)
                .wrapping_div((**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).rtt_min);
        }
        nb_packets = nb_packets.wrapping_mul(mult);
    }
    ack_gap = nb_packets
        .wrapping_add(3 as uint64_t)
        .wrapping_div(4 as uint64_t);
    if data_rate > PICOQUIC_BANDWIDTH_MEDIUM as uint64_t {
        if (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).rtt_min
            as ::core::ffi::c_ulonglong
            > PICOQUIC_TARGET_RENO_RTT
        {
            ack_gap_min = 10 as uint64_t;
        } else {
            ack_gap_min = 4 as uint64_t;
        }
    }
    if ack_gap < ack_gap_min {
        ack_gap = ack_gap_min;
    } else if ack_gap > 32 as uint64_t {
        if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
            || (*cnx).congestion_alg.is_null()
            || (*(*cnx).congestion_alg).congestion_algorithm_number as ::core::ffi::c_int
                == PICOQUIC_CC_ALGO_NUMBER_NEW_RENO
            || (*(*cnx).congestion_alg).congestion_algorithm_number as ::core::ffi::c_int
                == PICOQUIC_CC_ALGO_NUMBER_FAST
        {
            ack_gap = 32 as uint64_t;
        } else {
            ack_gap = (32 as uint64_t).wrapping_add(
                nb_packets
                    .wrapping_sub(128 as uint64_t)
                    .wrapping_div(8 as uint64_t),
            );
            if ack_gap > 64 as uint64_t {
                ack_gap = 64 as uint64_t;
            }
        }
    }
    return ack_gap;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_compute_ack_delay_max(
    mut cnx: *mut picoquic_cnx_t,
    mut rtt: uint64_t,
    mut remote_min_ack_delay: uint64_t,
) -> uint64_t {
    let mut ack_delay_max: uint64_t = rtt.wrapping_div(4 as uint64_t);
    if (*cnx).is_ack_frequency_negotiated() == 0
        && (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).is_ssthresh_initialized() == 0
    {
        ack_delay_max = ack_delay_max.wrapping_div(2 as uint64_t);
    }
    if ack_delay_max as ::core::ffi::c_ulonglong > PICOQUIC_ACK_DELAY_MAX {
        ack_delay_max = PICOQUIC_ACK_DELAY_MAX as uint64_t;
    }
    if ack_delay_max < remote_min_ack_delay {
        ack_delay_max = remote_min_ack_delay;
    }
    return ack_delay_max;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_compute_ack_gap_and_delay(
    mut cnx: *mut picoquic_cnx_t,
    mut rtt: uint64_t,
    mut remote_min_ack_delay: uint64_t,
    mut data_rate: uint64_t,
    mut ack_gap: *mut uint64_t,
    mut ack_delay_max: *mut uint64_t,
) {
    let mut nb_packets: uint64_t = picoquic_compute_packets_in_window(cnx, data_rate);
    *ack_delay_max = picoquic_compute_ack_delay_max(cnx, rtt, remote_min_ack_delay);
    *ack_gap = picoquic_compute_ack_gap(cnx, data_rate, nb_packets);
    if (2 as uint64_t)
        .wrapping_mul((**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).smoothed_rtt)
        > (3 as uint64_t)
            .wrapping_mul((**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).rtt_min)
    {
        let mut return_data_rate: uint64_t = 0 as uint64_t;
        if (*cnx).is_ack_frequency_negotiated() != 0 {
            return_data_rate =
                (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).receive_rate_max;
        } else {
            return_data_rate =
                (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).bandwidth_estimate;
        }
        if nb_packets < 2 as uint64_t {
            nb_packets = 2 as uint64_t;
        }
        if return_data_rate > 0 as uint64_t {
            let ack_size: uint64_t = (12 as ::core::ffi::c_int
                + 40 as ::core::ffi::c_int
                + 8 as ::core::ffi::c_int
                + 55 as ::core::ffi::c_int) as uint64_t;
            let mut ack_transmission_time: uint64_t = ack_size
                .wrapping_mul(1000000 as uint64_t)
                .wrapping_div(return_data_rate);
            if ack_transmission_time > *ack_delay_max {
                *ack_delay_max = ack_transmission_time;
                if *ack_delay_max as ::core::ffi::c_ulonglong > PICOQUIC_ACK_DELAY_MAX {
                    *ack_delay_max = PICOQUIC_ACK_DELAY_MAX as uint64_t;
                }
            }
            let mut rtt_target: uint64_t = (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                .smoothed_rtt
                .wrapping_add((**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).rtt_min)
                .wrapping_div(2 as uint64_t);
            if (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).is_ssthresh_initialized()
                == 0
            {
                nb_packets = nb_packets.wrapping_div(2 as uint64_t);
            }
            let mut nb_ack_per_rtt: uint64_t = if *ack_gap > 0 as uint64_t {
                nb_packets
                    .wrapping_add(*ack_gap)
                    .wrapping_sub(1 as uint64_t)
                    .wrapping_div(*ack_gap)
            } else {
                nb_packets
            };
            if nb_ack_per_rtt.wrapping_mul(*ack_delay_max) > rtt_target {
                let mut nb_acks_max: uint64_t =
                    (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                        .smoothed_rtt
                        .wrapping_div(*ack_delay_max);
                if nb_acks_max <= 1 as uint64_t {
                    *ack_gap = nb_packets;
                } else {
                    let mut ack_gap_min: uint64_t = nb_packets
                        .wrapping_add(nb_acks_max)
                        .wrapping_sub(1 as uint64_t)
                        .wrapping_div(nb_acks_max);
                    if *ack_gap < ack_gap_min {
                        *ack_gap = ack_gap_min;
                    }
                }
            }
        }
    }
    if (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).rtt_min
        < (*ack_delay_max).wrapping_mul(4 as uint64_t)
        && *ack_gap > 32 as uint64_t
    {
        *ack_gap = 32 as uint64_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_record_ack_packet_data(
    mut packet_data: *mut picoquic_packet_data_t,
    mut acked_packet: *mut picoquic_packet_t,
) {
    let mut old_path: *mut picoquic_path_t = (*acked_packet).send_path as *mut picoquic_path_t;
    if !old_path.is_null() {
        let mut path_i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while path_i < (*packet_data).nb_path_ack
            && (*packet_data).path_ack[path_i as usize].acked_path != old_path
        {
            path_i += 1;
        }
        if path_i == (*packet_data).nb_path_ack {
            if path_i > PICOQUIC_NB_PATH_TARGET {
                return;
            }
            (*packet_data).nb_path_ack += 1;
            (*packet_data).path_ack[path_i as usize].acked_path = old_path;
        }
        if (*packet_data).path_ack[path_i as usize].is_set == 0 {
            (*packet_data).path_ack[path_i as usize].largest_sent_time = (*acked_packet).send_time;
            (*packet_data).path_ack[path_i as usize].delivered_prior =
                (*acked_packet).delivered_prior;
            (*packet_data).path_ack[path_i as usize].delivered_time_prior =
                (*acked_packet).delivered_time_prior;
            (*packet_data).path_ack[path_i as usize].delivered_sent_prior =
                (*acked_packet).delivered_sent_prior;
            (*packet_data).path_ack[path_i as usize].lost_prior = (*acked_packet).lost_prior;
            (*packet_data).path_ack[path_i as usize].inflight_prior =
                (*acked_packet).inflight_prior;
            (*packet_data).path_ack[path_i as usize].rs_is_path_limited =
                (*acked_packet).delivered_app_limited();
            (*packet_data).path_ack[path_i as usize].rs_is_cwnd_limited =
                (*acked_packet).sent_cwin_limited();
            (*packet_data).path_ack[path_i as usize].is_set = 1 as ::core::ffi::c_uint;
        }
        (*packet_data).path_ack[path_i as usize].data_acked =
            ((*packet_data).path_ack[path_i as usize].data_acked as ::core::ffi::c_ulong)
                .wrapping_add((*acked_packet).length as ::core::ffi::c_ulong)
                as uint64_t as uint64_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn process_decoded_packet_data(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut epoch: ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut packet_data: *mut picoquic_packet_data_t,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < (*packet_data).nb_path_ack {
        let mut lost_before_ack: uint64_t = (*path_x).total_bytes_lost;
        let mut nb_bytes_newly_lost: uint64_t = 0 as uint64_t;
        picoquic_update_path_rtt(
            cnx,
            (*packet_data).path_ack[i as usize].acked_path,
            path_x,
            epoch,
            (*packet_data).path_ack[i as usize].largest_sent_time,
            current_time,
            (*packet_data).last_ack_delay,
            (*packet_data).last_time_stamp_received,
        );
        picoquic_estimate_path_bandwidth(
            cnx,
            (*packet_data).path_ack[i as usize].acked_path,
            (*packet_data).path_ack[i as usize].largest_sent_time,
            (*packet_data).path_ack[i as usize].delivered_prior,
            (*packet_data).path_ack[i as usize].delivered_time_prior,
            (*packet_data).path_ack[i as usize].delivered_sent_prior,
            if (*packet_data).last_time_stamp_received == 0 as uint64_t {
                current_time
            } else {
                (*packet_data).last_time_stamp_received
            },
            current_time,
            (*packet_data).path_ack[i as usize].rs_is_path_limited as ::core::ffi::c_int,
        );
        picoquic_estimate_max_path_bandwidth(
            cnx,
            (*packet_data).path_ack[i as usize].acked_path,
            (*packet_data).path_ack[i as usize].largest_sent_time,
            if (*packet_data).last_time_stamp_received == 0 as uint64_t {
                current_time
            } else {
                (*packet_data).last_time_stamp_received
            },
            current_time,
        );
        if epoch == picoquic_epoch_1rtt as ::core::ffi::c_int
            && (*cnx).cnx_state as ::core::ffi::c_uint
                >= picoquic_state_client_ready_start as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            picoquic_queue_retransmit_on_ack(cnx, path_x, current_time);
            nb_bytes_newly_lost = (*path_x).total_bytes_lost.wrapping_sub(lost_before_ack);
        }
        if !(*cnx).congestion_alg.is_null()
            && (*(*packet_data).path_ack[i as usize].acked_path).rtt_sample > 0 as uint64_t
        {
            let mut ack_state: picoquic_per_ack_state_t = {
                let mut init = st_picoquic_per_ack_state_t {
                    is_app_limited_is_cwnd_limited: [0; 1],
                    c2rust_padding: [0; 7],
                    rtt_measurement: 0 as uint64_t,
                    one_way_delay: 0,
                    nb_bytes_acknowledged: 0,
                    nb_bytes_newly_lost: 0,
                    nb_bytes_lost_since_packet_sent: 0,
                    nb_bytes_delivered_since_packet_sent: 0,
                    inflight_prior: 0,
                    lost_packet_number: 0,
                    lost_packet_sent_time: 0,
                };
                init.set_is_app_limited(0);
                init.set_is_cwnd_limited(0);
                init
            };
            ack_state.rtt_measurement =
                (*(*packet_data).path_ack[i as usize].acked_path).rtt_sample;
            ack_state.one_way_delay =
                (*(*packet_data).path_ack[i as usize].acked_path).one_way_delay_sample;
            ack_state.nb_bytes_acknowledged = (*packet_data).path_ack[i as usize].data_acked;
            ack_state.nb_bytes_newly_lost = nb_bytes_newly_lost;
            if (*cnx).cnx_state as ::core::ffi::c_uint
                == picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                ack_state.nb_bytes_lost_since_packet_sent = (*path_x)
                    .total_bytes_lost
                    .wrapping_sub((*packet_data).path_ack[i as usize].lost_prior);
            } else {
                ack_state.nb_bytes_lost_since_packet_sent = nb_bytes_newly_lost;
            }
            ack_state.nb_bytes_delivered_since_packet_sent = (*path_x)
                .delivered
                .wrapping_sub((*packet_data).path_ack[i as usize].delivered_prior);
            ack_state.inflight_prior = (*packet_data).path_ack[i as usize].inflight_prior;
            ack_state.set_is_app_limited(
                (*packet_data).path_ack[i as usize].rs_is_path_limited as ::core::ffi::c_uint,
            );
            ack_state.set_is_cwnd_limited(
                (*packet_data).path_ack[i as usize].rs_is_cwnd_limited as ::core::ffi::c_uint,
            );
            (*(*packet_data).path_ack[i as usize].acked_path)
                .set_is_lost_feedback_notified(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*(*cnx).congestion_alg)
                .alg_notify
                .expect("non-null function pointer")(
                cnx as *mut picoquic_cnx_t,
                (*packet_data).path_ack[i as usize].acked_path as *mut picoquic_path_t,
                picoquic_congestion_notification_acknowledgement,
                &raw mut ack_state,
                current_time,
            );
        }
        i += 1;
    }
    if (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).is_ssthresh_initialized()
        as ::core::ffi::c_int
        != 0
        && (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).is_ticket_seeded() == 0
    {
        picoquic_seed_ticket(
            cnx as *mut picoquic_cnx_t,
            *(*cnx).path.offset(0 as ::core::ffi::c_int as isize) as *mut picoquic_path_t,
        );
    }
}
unsafe extern "C" fn picoquic_find_acked_packet(
    mut cnx: *mut picoquic_cnx_t,
    mut pkt_ctx: *mut picoquic_packet_context_t,
    mut largest: uint64_t,
    mut current_time: uint64_t,
    mut is_new_ack: *mut ::core::ffi::c_int,
) -> *mut picoquic_packet_t {
    let mut packet: *mut picoquic_packet_t = (*pkt_ctx).pending_first;
    if largest > (*pkt_ctx).highest_acknowledged
        || (*pkt_ctx).highest_acknowledged == UINT64_MAX as uint64_t
    {
        (*pkt_ctx).highest_acknowledged = largest;
        (*pkt_ctx).highest_acknowledged_time = current_time;
        (*pkt_ctx).set_ack_of_ack_requested(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        *is_new_ack = 1 as ::core::ffi::c_int;
        while !packet.is_null()
            && !(*packet).packet_next.is_null()
            && (*packet).sequence_number < largest
        {
            packet = (*packet).packet_next as *mut picoquic_packet_t;
        }
    }
    return packet;
}
unsafe extern "C" fn picoquic_process_ack_of_ack_body(
    mut sack_list: *mut picoquic_sack_list_t,
    mut largest: uint64_t,
    mut num_block: uint64_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: size_t,
    mut consumed: *mut size_t,
    mut is_ecn: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut byte_index: size_t = *consumed;
    let mut previous_sack_item: *mut picoquic_sack_item_t =
        ::core::ptr::null_mut::<picoquic_sack_item_t>();
    loop {
        let mut range: uint64_t = 0;
        let mut l_range: size_t = 0;
        let mut block_to_block: uint64_t = 0;
        if byte_index >= bytes_max {
            ret = -(1 as ::core::ffi::c_int);
            break;
        } else {
            l_range = picoquic_varint_decode(
                bytes.offset(byte_index as isize),
                bytes_max.wrapping_sub(byte_index),
                &raw mut range,
            );
            if l_range == 0 as size_t {
                byte_index = bytes_max;
                ret = -(1 as ::core::ffi::c_int);
                break;
            } else {
                byte_index = byte_index.wrapping_add(l_range);
                range = range.wrapping_add(1);
                if largest.wrapping_add(1 as uint64_t) < range {
                    ret = -(1 as ::core::ffi::c_int);
                    break;
                } else {
                    if range > 0 as uint64_t {
                        previous_sack_item = picoquic_process_ack_of_ack_range(
                            sack_list,
                            previous_sack_item,
                            largest.wrapping_add(1 as uint64_t).wrapping_sub(range),
                            largest,
                        );
                    }
                    let c2rust_fresh2 = num_block;
                    num_block = num_block.wrapping_sub(1);
                    if c2rust_fresh2 == 0 as uint64_t {
                        break;
                    }
                    if byte_index >= bytes_max {
                        ret = -(1 as ::core::ffi::c_int);
                        break;
                    } else {
                        let mut l_gap: size_t = picoquic_varint_decode(
                            bytes.offset(byte_index as isize),
                            bytes_max.wrapping_sub(byte_index),
                            &raw mut block_to_block,
                        );
                        if l_gap == 0 as size_t {
                            byte_index = bytes_max;
                            ret = -(1 as ::core::ffi::c_int);
                            break;
                        } else {
                            byte_index = byte_index.wrapping_add(l_gap);
                            block_to_block = block_to_block.wrapping_add(1 as uint64_t);
                            block_to_block = block_to_block.wrapping_add(range);
                            if largest < block_to_block {
                                ret = -(1 as ::core::ffi::c_int);
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
    if ret == 0 as ::core::ffi::c_int && is_ecn != 0 {
        if byte_index >= bytes_max {
            ret = -(1 as ::core::ffi::c_int);
        } else {
            let mut ecnx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while ecnx < 3 as ::core::ffi::c_int {
                let mut ecn: uint64_t = 0;
                let mut l_ecn: size_t = picoquic_varint_decode(
                    bytes.offset(byte_index as isize),
                    bytes_max.wrapping_sub(byte_index),
                    &raw mut ecn,
                );
                if l_ecn == 0 as size_t {
                    byte_index = bytes_max;
                    ret = -(1 as ::core::ffi::c_int);
                    break;
                } else {
                    byte_index = byte_index.wrapping_add(l_ecn);
                    ecnx += 1;
                }
            }
        }
    }
    *consumed = byte_index;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_ack_of_ack_frame(
    mut sack_list: *mut picoquic_sack_list_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: size_t,
    mut consumed: *mut size_t,
    mut is_ecn: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut largest: uint64_t = 0;
    let mut ack_delay: uint64_t = 0;
    let mut num_block: uint64_t = 0;
    ret = picoquic_parse_ack_header(
        bytes,
        bytes_max,
        &raw mut num_block,
        ::core::ptr::null_mut::<uint64_t>(),
        &raw mut largest,
        &raw mut ack_delay,
        consumed,
        0 as uint8_t,
    );
    if ret == 0 as ::core::ffi::c_int {
        ret = picoquic_process_ack_of_ack_body(
            sack_list, largest, num_block, bytes, bytes_max, consumed, is_ecn,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_ack_of_path_ack_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: size_t,
    mut consumed: *mut size_t,
    mut is_ecn: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut largest: uint64_t = 0;
    let mut ack_delay: uint64_t = 0;
    let mut num_block: uint64_t = 0;
    let mut path_id: uint64_t = 0 as uint64_t;
    ret = picoquic_parse_ack_header(
        bytes,
        bytes_max,
        &raw mut num_block,
        &raw mut path_id,
        &raw mut largest,
        &raw mut ack_delay,
        consumed,
        0 as uint8_t,
    );
    if ret == 0 as ::core::ffi::c_int {
        let mut ack_ctx: *mut picoquic_ack_context_t =
            ::core::ptr::null_mut::<picoquic_ack_context_t>();
        if (*cnx).is_multipath_enabled() != 0 {
            let mut path_index: ::core::ffi::c_int = picoquic_find_path_by_unique_id(cnx, path_id);
            if path_index >= 0 as ::core::ffi::c_int {
                ack_ctx = &raw mut (**(*cnx).path.offset(path_index as isize)).ack_ctx;
            }
        }
        if ack_ctx.is_null() {
            let mut bytes_next: *const uint8_t = picoquic_skip_ack_frame_maybe_ecn(
                bytes,
                bytes.offset(bytes_max as isize),
                is_ecn,
                1 as ::core::ffi::c_int,
            );
            if bytes_next.is_null() {
                ret = -(1 as ::core::ffi::c_int);
                *consumed = bytes_max;
            } else {
                *consumed = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
            }
        } else {
            ret = picoquic_process_ack_of_ack_body(
                &raw mut (*ack_ctx).sack_list,
                largest,
                num_block,
                bytes,
                bytes_max,
                consumed,
                is_ecn,
            );
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_check_frame_needs_repeat(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut p_type: picoquic_packet_type_enum,
    mut no_need_to_repeat: *mut ::core::ffi::c_int,
    mut do_not_detect_spurious: *mut ::core::ffi::c_int,
    mut is_preemptive_needed: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut fin: ::core::ffi::c_int = 0;
    let mut data_length: size_t = 0;
    let mut stream_id: uint64_t = 0;
    let mut offset: uint64_t = 0;
    let mut maxdata: uint64_t = 0;
    let mut max_stream_rank: uint64_t = 0;
    let mut stream: *mut picoquic_stream_head_t = ::core::ptr::null_mut::<picoquic_stream_head_t>();
    let mut consumed: size_t = 0 as size_t;
    *no_need_to_repeat = 0 as ::core::ffi::c_int;
    if *bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & !(picoquic_frame_type_stream_range_min as ::core::ffi::c_int
            ^ picoquic_frame_type_stream_range_max as ::core::ffi::c_int)
        == picoquic_frame_type_stream_range_min as ::core::ffi::c_int
    {
        ret = picoquic_parse_stream_header(
            bytes,
            bytes_max,
            &raw mut stream_id,
            &raw mut offset,
            &raw mut data_length,
            &raw mut fin,
            &raw mut consumed,
        );
        if ret == 0 as ::core::ffi::c_int {
            stream = picoquic_find_stream(cnx, stream_id);
            if stream.is_null() {
                *no_need_to_repeat = 1 as ::core::ffi::c_int;
            } else {
                if (*stream).reset_sent() != 0 {
                    *no_need_to_repeat = 1 as ::core::ffi::c_int;
                } else {
                    *no_need_to_repeat = picoquic_check_sack_list(
                        &raw mut (*stream).sack_list,
                        offset,
                        offset.wrapping_add(data_length as uint64_t).wrapping_sub(
                            (if fin != 0 {
                                0 as ::core::ffi::c_int
                            } else {
                                1 as ::core::ffi::c_int
                            }) as uint64_t,
                        ),
                    );
                }
                if !is_preemptive_needed.is_null()
                    && (*stream).fin_sent() as ::core::ffi::c_int != 0
                {
                    *is_preemptive_needed |= 1 as ::core::ffi::c_int;
                }
            }
        }
    } else {
        let mut p_last_byte: *const uint8_t = bytes.offset(bytes_max as isize);
        match *bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int {
            16 => {
                bytes = picoquic_frames_varint_decode(
                    bytes.offset(1 as ::core::ffi::c_int as isize),
                    p_last_byte,
                    &raw mut maxdata,
                );
                if bytes.is_null() {
                    *no_need_to_repeat = 1 as ::core::ffi::c_int;
                } else if maxdata < (*cnx).maxdata_local || maxdata <= (*cnx).maxdata_local_acked {
                    *no_need_to_repeat = 1 as ::core::ffi::c_int;
                }
            }
            17 => {
                bytes = picoquic_frames_varint_decode(
                    bytes.offset(1 as ::core::ffi::c_int as isize),
                    p_last_byte,
                    &raw mut stream_id,
                );
                if bytes.is_null() || {
                    bytes = picoquic_frames_varint_decode(bytes, p_last_byte, &raw mut maxdata);
                    bytes.is_null()
                } {
                    *no_need_to_repeat = 1 as ::core::ffi::c_int;
                } else {
                    stream = picoquic_find_stream(cnx, stream_id);
                    if stream.is_null() {
                        *no_need_to_repeat = 1 as ::core::ffi::c_int;
                    } else if (*stream).fin_received() as ::core::ffi::c_int != 0
                        || (*stream).reset_received() as ::core::ffi::c_int != 0
                        || (*stream).stop_sending_sent() as ::core::ffi::c_int != 0
                    {
                        *no_need_to_repeat = 1 as ::core::ffi::c_int;
                    } else if maxdata < (*stream).maxdata_local
                        || maxdata <= (*stream).maxdata_local_acked
                    {
                        *no_need_to_repeat = 1 as ::core::ffi::c_int;
                    }
                }
            }
            18 | 19 => {
                ret = picoquic_check_max_streams_frame_needs_repeat(
                    cnx,
                    bytes,
                    p_last_byte,
                    no_need_to_repeat,
                );
            }
            20 => {
                bytes = picoquic_frames_varint_decode(
                    bytes.offset(1 as ::core::ffi::c_int as isize),
                    p_last_byte,
                    &raw mut maxdata,
                );
                if bytes.is_null() {
                    *no_need_to_repeat = 1 as ::core::ffi::c_int;
                } else if maxdata < (*cnx).maxdata_remote {
                    *no_need_to_repeat = 1 as ::core::ffi::c_int;
                } else {
                    *no_need_to_repeat = ((*cnx).sent_blocked_frame() == 0) as ::core::ffi::c_int;
                }
            }
            22 => {
                bytes = picoquic_frames_varint_decode(
                    bytes.offset(1 as ::core::ffi::c_int as isize),
                    p_last_byte,
                    &raw mut max_stream_rank,
                );
                if bytes.is_null() {
                    *no_need_to_repeat = 1 as ::core::ffi::c_int;
                } else if (*cnx).max_stream_id_bidir_remote
                    > max_stream_rank.wrapping_sub(1 as ::core::ffi::c_int as uint64_t)
                        << 2 as ::core::ffi::c_int
                        | (0 as ::core::ffi::c_int as uint64_t) << 1 as ::core::ffi::c_int
                        | ((*cnx).client_mode() as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int)
                            as uint64_t
                {
                    *no_need_to_repeat = 1 as ::core::ffi::c_int;
                } else {
                    *no_need_to_repeat =
                        ((*cnx).stream_blocked_bidir_sent() == 0) as ::core::ffi::c_int;
                }
            }
            23 => {
                bytes = picoquic_frames_varint_decode(
                    bytes.offset(1 as ::core::ffi::c_int as isize),
                    p_last_byte,
                    &raw mut max_stream_rank,
                );
                if bytes.is_null() {
                    *no_need_to_repeat = 1 as ::core::ffi::c_int;
                } else if (*cnx).max_stream_id_unidir_remote
                    > max_stream_rank.wrapping_sub(1 as ::core::ffi::c_int as uint64_t)
                        << 2 as ::core::ffi::c_int
                        | (1 as ::core::ffi::c_int as uint64_t) << 1 as ::core::ffi::c_int
                        | ((*cnx).client_mode() as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int)
                            as uint64_t
                {
                    *no_need_to_repeat = 1 as ::core::ffi::c_int;
                } else {
                    *no_need_to_repeat =
                        ((*cnx).stream_blocked_unidir_sent() == 0) as ::core::ffi::c_int;
                }
            }
            21 => {
                bytes = picoquic_frames_varint_decode(
                    bytes.offset(1 as ::core::ffi::c_int as isize),
                    p_last_byte,
                    &raw mut stream_id,
                );
                if bytes.is_null() || {
                    bytes = picoquic_frames_varint_decode(bytes, p_last_byte, &raw mut maxdata);
                    bytes.is_null()
                } {
                    *no_need_to_repeat = 1 as ::core::ffi::c_int;
                } else {
                    stream = picoquic_find_stream(cnx, stream_id);
                    if stream.is_null() {
                        *no_need_to_repeat = 1 as ::core::ffi::c_int;
                    } else if (*stream).fin_requested() as ::core::ffi::c_int != 0
                        || (*stream).reset_requested() as ::core::ffi::c_int != 0
                        || (*stream).fin_sent() as ::core::ffi::c_int != 0
                        || (*stream).reset_sent() as ::core::ffi::c_int != 0
                    {
                        *no_need_to_repeat = 1 as ::core::ffi::c_int;
                    } else if maxdata < (*stream).maxdata_remote
                        || (*stream).stream_data_blocked_sent() == 0
                    {
                        *no_need_to_repeat = 1 as ::core::ffi::c_int;
                    }
                }
            }
            26 => {
                *no_need_to_repeat = 1 as ::core::ffi::c_int;
            }
            27 => {
                *no_need_to_repeat =
                    picoquic_should_repeat_path_response_frame(cnx, bytes, bytes_max);
            }
            48 | 49 => {
                *no_need_to_repeat = 1 as ::core::ffi::c_int;
                *do_not_detect_spurious = 0 as ::core::ffi::c_int;
            }
            30 => {
                if (*cnx).is_handshake_done_acked() != 0 {
                    *no_need_to_repeat = 1 as ::core::ffi::c_int;
                }
            }
            7 => {
                if (*cnx).is_new_token_acked() != 0 {
                    *no_need_to_repeat = 1 as ::core::ffi::c_int;
                }
            }
            6 => {
                ret = picoquic_check_crypto_frame_needs_repeat(
                    cnx,
                    bytes,
                    bytes_max,
                    p_type,
                    no_need_to_repeat,
                );
            }
            24 => {
                ret = picoquic_check_new_cid_needs_repeat(
                    cnx,
                    bytes,
                    bytes_max,
                    0 as ::core::ffi::c_int,
                    no_need_to_repeat,
                );
            }
            25 => {
                ret = picoquic_check_retire_connection_id_needs_repeat(
                    cnx,
                    bytes,
                    bytes_max,
                    no_need_to_repeat,
                    0 as ::core::ffi::c_int,
                );
            }
            4 => {
                ret = picoquic_check_reset_stream_needs_repeat(
                    cnx,
                    bytes,
                    bytes_max,
                    no_need_to_repeat,
                );
            }
            5 => {
                ret = picoquic_check_stop_sending_needs_repeat(
                    cnx,
                    bytes,
                    bytes_max,
                    no_need_to_repeat,
                );
            }
            _ => {
                let mut frame_id64: uint64_t = 0;
                let mut type_bytes: *const uint8_t = bytes;
                let mut p_bytes_max: *const uint8_t = bytes.offset(bytes_max as isize);
                *no_need_to_repeat = 0 as ::core::ffi::c_int;
                bytes = picoquic_frames_varint_decode(bytes, p_bytes_max, &raw mut frame_id64);
                if !bytes.is_null() {
                    match frame_id64 {
                        175 => {
                            let mut seq: uint64_t = 0;
                            let mut packets: uint64_t = 0;
                            let mut microsec: uint64_t = 0;
                            let mut ignore_order: uint8_t = 0;
                            let mut reordering_threshold: uint64_t = 0;
                            bytes = picoquic_parse_ack_frequency_frame(
                                bytes,
                                p_bytes_max,
                                &raw mut seq,
                                &raw mut packets,
                                &raw mut microsec,
                                &raw mut ignore_order,
                                &raw mut reordering_threshold,
                            );
                            if bytes.is_null() {
                                ret = -(1 as ::core::ffi::c_int);
                            } else if seq == (*cnx).ack_frequency_sequence_local {
                                *no_need_to_repeat = 1 as ::core::ffi::c_int;
                            }
                        }
                        31 => {
                            *no_need_to_repeat = 0 as ::core::ffi::c_int;
                        }
                        354585600 | 354585601 | 757 => {
                            *no_need_to_repeat = 1 as ::core::ffi::c_int;
                        }
                        354585605 => {
                            *no_need_to_repeat = 0 as ::core::ffi::c_int;
                        }
                        354585607 | 354585608 => {
                            picoquic_path_available_or_backup_frame_need_repeat(
                                cnx,
                                bytes,
                                p_bytes_max,
                                no_need_to_repeat,
                            );
                        }
                        354585612 => {
                            picoquic_max_path_id_frame_needs_repeat(
                                cnx,
                                bytes,
                                p_bytes_max,
                                no_need_to_repeat,
                            );
                        }
                        354585613 => {
                            picoquic_path_blocked_frame_needs_repeat(
                                cnx,
                                bytes,
                                p_bytes_max,
                                no_need_to_repeat,
                            );
                        }
                        354585609 => {
                            ret = picoquic_check_new_cid_needs_repeat(
                                cnx,
                                type_bytes,
                                bytes_max,
                                1 as ::core::ffi::c_int,
                                no_need_to_repeat,
                            );
                        }
                        354585610 => {
                            ret = picoquic_check_retire_connection_id_needs_repeat(
                                cnx,
                                type_bytes,
                                bytes_max,
                                no_need_to_repeat,
                                1 as ::core::ffi::c_int,
                            );
                        }
                        10453414 | 10453415 => {
                            ret = 0 as ::core::ffi::c_int;
                        }
                        _ => {
                            *no_need_to_repeat = 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_ack_of_stream_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: size_t,
    mut consumed: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut fin: ::core::ffi::c_int = 0;
    let mut data_length: size_t = 0;
    let mut stream_id: uint64_t = 0;
    let mut offset: uint64_t = 0;
    let mut stream: *mut picoquic_stream_head_t = ::core::ptr::null_mut::<picoquic_stream_head_t>();
    ret = picoquic_parse_stream_header(
        bytes,
        bytes_max,
        &raw mut stream_id,
        &raw mut offset,
        &raw mut data_length,
        &raw mut fin,
        consumed,
    );
    if ret == 0 as ::core::ffi::c_int {
        *consumed = (*consumed).wrapping_add(data_length);
        stream = picoquic_find_stream(cnx, stream_id);
        if !stream.is_null() {
            picoquic_update_sack_list(
                &raw mut (*stream).sack_list,
                offset,
                offset.wrapping_add(data_length as uint64_t).wrapping_sub(
                    (if fin != 0 {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    }) as uint64_t,
                ),
                0 as uint64_t,
            );
            picoquic_delete_stream_if_closed(cnx, stream);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_ack_of_frames(
    mut cnx: *mut picoquic_cnx_t,
    mut p: *mut picoquic_packet_t,
    mut packet_data: *mut picoquic_packet_data_t,
    mut is_spurious: ::core::ffi::c_int,
    mut current_time: uint64_t,
) {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut byte_index: size_t = 0;
    let mut frame_is_pure_ack: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut frame_length: size_t = 0 as size_t;
    if (*p).ptype as ::core::ffi::c_uint
        == picoquic_packet_0rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cnx).nb_zero_rtt_acked = (*cnx).nb_zero_rtt_acked.wrapping_add(1);
    }
    byte_index = (*p).offset;
    while ret == 0 as ::core::ffi::c_int && byte_index < (*p).length {
        let mut ftype: uint64_t = 0;
        let mut l_ftype: size_t = picoquic_varint_decode(
            (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize) as *mut uint8_t,
            (*p).length.wrapping_sub(byte_index),
            &raw mut ftype,
        );
        if l_ftype == 0 as size_t {
            break;
        }
        match ftype {
            2 => {
                ret = picoquic_process_ack_of_ack_frame(
                    &raw mut (*(&raw mut (*cnx).ack_ctx as *mut picoquic_ack_context_t)
                        .offset((*p).pc as isize))
                    .sack_list,
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    &raw mut frame_length,
                    0 as ::core::ffi::c_int,
                );
                byte_index = byte_index.wrapping_add(frame_length);
            }
            3 => {
                ret = picoquic_process_ack_of_ack_frame(
                    &raw mut (*(&raw mut (*cnx).ack_ctx as *mut picoquic_ack_context_t)
                        .offset((*p).pc as isize))
                    .sack_list,
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    &raw mut frame_length,
                    1 as ::core::ffi::c_int,
                );
                byte_index = byte_index.wrapping_add(frame_length);
            }
            354585600 => {
                ret = picoquic_process_ack_of_path_ack_frame(
                    cnx,
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    &raw mut frame_length,
                    0 as ::core::ffi::c_int,
                );
                byte_index = byte_index.wrapping_add(frame_length);
            }
            354585601 => {
                ret = picoquic_process_ack_of_path_ack_frame(
                    cnx,
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    &raw mut frame_length,
                    1 as ::core::ffi::c_int,
                );
                byte_index = byte_index.wrapping_add(frame_length);
            }
            30 => {
                (*cnx).set_is_handshake_done_acked(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                byte_index = byte_index.wrapping_add(l_ftype);
            }
            24 => {
                ret = picoquic_process_ack_of_new_cid_frame(
                    cnx,
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    0 as ::core::ffi::c_int,
                    &raw mut frame_length,
                );
                byte_index = byte_index.wrapping_add(frame_length);
            }
            354585609 => {
                ret = picoquic_process_ack_of_new_cid_frame(
                    cnx,
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    1 as ::core::ffi::c_int,
                    &raw mut frame_length,
                );
                byte_index = byte_index.wrapping_add(frame_length);
            }
            25 => {
                ret = picoquic_process_ack_of_retire_connection_id_frame(
                    cnx,
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    &raw mut frame_length,
                    0 as ::core::ffi::c_int,
                );
                byte_index = byte_index.wrapping_add(frame_length);
            }
            354585610 => {
                ret = picoquic_process_ack_of_retire_connection_id_frame(
                    cnx,
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    &raw mut frame_length,
                    1 as ::core::ffi::c_int,
                );
                byte_index = byte_index.wrapping_add(frame_length);
            }
            6 => {
                ret = picoquic_process_ack_of_crypto_frame(
                    cnx,
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    (*p).ptype,
                    &raw mut frame_length,
                );
                byte_index = byte_index.wrapping_add(frame_length);
            }
            7 => {
                ret = picoquic_skip_frame(
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    &raw mut frame_length,
                    &raw mut frame_is_pure_ack,
                );
                byte_index = byte_index.wrapping_add(frame_length);
                (*cnx).set_is_new_token_acked(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
            16 => {
                ret = picoquic_process_ack_of_max_data_frame(
                    cnx,
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    &raw mut frame_length,
                );
                byte_index = byte_index.wrapping_add(frame_length);
            }
            17 => {
                ret = picoquic_process_ack_of_max_stream_data_frame(
                    cnx,
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    &raw mut frame_length,
                );
                byte_index = byte_index.wrapping_add(frame_length);
            }
            18 | 19 => {
                ret = picoquic_process_ack_of_max_streams_frame(
                    cnx,
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    &raw mut frame_length,
                );
                byte_index = byte_index.wrapping_add(frame_length);
            }
            4 => {
                ret = picoquic_process_ack_of_reset_stream_frame(
                    cnx,
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    &raw mut frame_length,
                );
                byte_index = byte_index.wrapping_add(frame_length);
            }
            354585612 => {
                ret = picoquic_process_ack_of_max_path_id_frame(
                    cnx,
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    &raw mut frame_length,
                );
                byte_index = byte_index.wrapping_add(frame_length);
            }
            354585613 => {
                ret = picoquic_process_ack_of_path_blocked_frame(
                    cnx,
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    &raw mut frame_length,
                );
                byte_index = byte_index.wrapping_add(frame_length);
            }
            10453414 | 10453415 => {
                ret = picoquic_process_ack_of_observed_address_frame(
                    cnx,
                    (*p).send_path as *mut picoquic_path_t,
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub(byte_index),
                    ftype,
                    &raw mut frame_length,
                );
                byte_index = byte_index.wrapping_add(frame_length);
            }
            32 => {
                if !(*p).send_path.is_null() {
                    if (*p).send_time > (*(*p).send_path).last_time_acked_data_frame_sent {
                        (*(*p).send_path).last_time_acked_data_frame_sent = current_time;
                    }
                }
                picoquic_record_ack_packet_data(packet_data, p);
                byte_index = byte_index.wrapping_add(l_ftype);
            }
            _ => {
                if ftype
                    & !(picoquic_frame_type_stream_range_min as ::core::ffi::c_int
                        ^ picoquic_frame_type_stream_range_max as ::core::ffi::c_int)
                        as uint64_t
                    == picoquic_frame_type_stream_range_min as ::core::ffi::c_int as uint64_t
                {
                    ret = picoquic_process_ack_of_stream_frame(
                        cnx,
                        (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                            as *mut uint8_t,
                        (*p).length.wrapping_sub(byte_index),
                        &raw mut frame_length,
                    );
                    byte_index = byte_index.wrapping_add(frame_length);
                    if !(*p).send_path.is_null() {
                        if (*p).send_time > (*(*p).send_path).last_time_acked_data_frame_sent {
                            (*(*p).send_path).last_time_acked_data_frame_sent = (*p).send_time;
                        }
                    }
                } else {
                    if ftype
                        & !(picoquic_frame_type_datagram as ::core::ffi::c_int
                            ^ picoquic_frame_type_datagram_l as ::core::ffi::c_int)
                            as uint64_t
                        == picoquic_frame_type_datagram as ::core::ffi::c_int as uint64_t
                    {
                        if !(*p).send_path.is_null()
                            && (*p).send_time > (*(*p).send_path).last_time_acked_data_frame_sent
                        {
                            (*(*p).send_path).last_time_acked_data_frame_sent = (*p).send_time;
                        }
                        if (*cnx).callback_fn.is_some() {
                            let mut frame_id: uint8_t = 0;
                            let mut content_length: uint64_t = 0;
                            let mut content_bytes: *mut uint8_t =
                                ::core::ptr::null_mut::<uint8_t>();
                            content_bytes = picoquic_decode_datagram_frame_header(
                                (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                                    as *mut uint8_t,
                                (&raw mut (*p).bytes as *mut uint8_t).offset((*p).length as isize)
                                    as *mut uint8_t,
                                &raw mut frame_id,
                                &raw mut content_length,
                            );
                            ret = (*cnx).callback_fn.expect("non-null function pointer")(
                                cnx as *mut picoquic_cnx_t,
                                (*p).send_time,
                                content_bytes,
                                content_length as size_t,
                                (if is_spurious != 0 {
                                    picoquic_callback_datagram_spurious as ::core::ffi::c_int
                                } else {
                                    picoquic_callback_datagram_acked as ::core::ffi::c_int
                                }) as picoquic_call_back_event_t,
                                (*cnx).callback_ctx,
                                NULL,
                            );
                        }
                    }
                    ret = picoquic_skip_frame(
                        (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                            as *mut uint8_t,
                        (*p).length.wrapping_sub(byte_index),
                        &raw mut frame_length,
                        &raw mut frame_is_pure_ack,
                    );
                    byte_index = byte_index.wrapping_add(frame_length);
                }
            }
        }
    }
}
unsafe extern "C" fn picoquic_process_ack_range(
    mut cnx: *mut picoquic_cnx_t,
    mut pc: picoquic_packet_context_enum,
    mut pkt_ctx: *mut picoquic_packet_context_t,
    mut highest: uint64_t,
    mut range: uint64_t,
    mut ppacket: *mut *mut picoquic_packet_t,
    mut current_time: uint64_t,
    mut packet_data: *mut picoquic_packet_data_t,
) -> ::core::ffi::c_int {
    let mut p: *mut picoquic_packet_t = *ppacket;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while !p.is_null() && range > 0 as uint64_t {
        if (*p).sequence_number > highest {
            p = (*p).packet_previous as *mut picoquic_packet_t;
        } else {
            if (*p).sequence_number == highest {
                let mut next: *mut picoquic_packet_t =
                    (*p).packet_previous as *mut picoquic_packet_t;
                let mut old_path: *mut picoquic_path_t = (*p).send_path as *mut picoquic_path_t;
                if (*p).is_ack_trap() != 0 {
                    ret = picoquic_connection_error(
                        cnx,
                        PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                        picoquic_frame_type_ack as ::core::ffi::c_int as uint64_t,
                    );
                    break;
                } else {
                    if !old_path.is_null() {
                        (*old_path).delivered = ((*old_path).delivered as ::core::ffi::c_ulong)
                            .wrapping_add((*p).length as ::core::ffi::c_ulong)
                            as uint64_t as uint64_t;
                        (*old_path)
                            .set_is_ack_lost(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        (*old_path)
                            .set_is_ack_expected(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        if (*p).sequence_number >= picoquic_get_ack_number(cnx, old_path, pc) {
                            (*old_path).nb_retransmit = 0 as uint64_t;
                        }
                        picoquic_record_ack_packet_data(packet_data, p);
                        if (*p).length.wrapping_add((*p).checksum_overhead) == (*old_path).send_mtu
                        {
                            (*old_path).nb_mtu_losses = 0 as uint64_t;
                        } else if (*p).length.wrapping_add((*p).checksum_overhead)
                            > (*old_path).send_mtu
                        {
                            (*old_path).send_mtu = (*p).length.wrapping_add((*p).checksum_overhead);
                            (*old_path).set_mtu_probe_sent(
                                0 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                            );
                        }
                    }
                    picoquic_process_ack_of_frames(
                        cnx,
                        p,
                        packet_data,
                        0 as ::core::ffi::c_int,
                        current_time,
                    );
                    if (*p).ptype as ::core::ffi::c_uint
                        == picoquic_packet_1rtt_protected as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        && ((*cnx).cnx_state as ::core::ffi::c_uint
                            == picoquic_state_client_ready_start as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                            || (*cnx).cnx_state as ::core::ffi::c_uint
                                == picoquic_state_server_false_start as ::core::ffi::c_int
                                    as ::core::ffi::c_uint)
                    {
                        picoquic_ready_state_transition(cnx, current_time);
                    }
                    picoquic_dequeue_retransmit_packet(
                        cnx,
                        pkt_ctx,
                        p,
                        1 as ::core::ffi::c_int,
                        0 as ::core::ffi::c_int,
                    );
                    p = next;
                }
            }
            range = range.wrapping_sub(1);
            highest = highest.wrapping_sub(1);
        }
    }
    *ppacket = p;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_ack_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut current_time: uint64_t,
    mut epoch: ::core::ffi::c_int,
    mut is_ecn: ::core::ffi::c_int,
    mut has_path_id: ::core::ffi::c_int,
    mut packet_data: *mut picoquic_packet_data_t,
) -> *const uint8_t {
    let mut path_id: uint64_t = 0 as uint64_t;
    let mut num_block: uint64_t = 0;
    let mut largest: uint64_t = 0;
    let mut ack_delay: uint64_t = 0;
    let mut consumed: size_t = 0;
    let mut pc: picoquic_packet_context_enum = picoquic_context_from_epoch(epoch);
    let mut ecnx3: [uint64_t; 3] = [
        0 as ::core::ffi::c_int as uint64_t,
        0 as ::core::ffi::c_int as uint64_t,
        0 as ::core::ffi::c_int as uint64_t,
    ];
    let mut ftype: uint64_t = (if has_path_id != 0 {
        if is_ecn != 0 {
            picoquic_frame_type_path_ack_ecn as ::core::ffi::c_int
        } else {
            picoquic_frame_type_path_ack as ::core::ffi::c_int
        }
    } else if is_ecn != 0 {
        picoquic_frame_type_ack_ecn as ::core::ffi::c_int
    } else {
        picoquic_frame_type_ack as ::core::ffi::c_int
    }) as uint64_t;
    let mut pkt_ctx: *mut picoquic_packet_context_t =
        (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t).offset(pc as isize)
            as *mut picoquic_packet_context_t;
    let mut largest_in_path: uint64_t = 0 as uint64_t;
    let mut ack_path: *mut picoquic_path_t = *(*cnx).path.offset(0 as ::core::ffi::c_int as isize);
    if picoquic_parse_ack_header(
        bytes,
        bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t,
        &raw mut num_block,
        (if has_path_id != 0 {
            &raw mut path_id
        } else {
            ::core::ptr::null_mut::<uint64_t>()
        }),
        &raw mut largest,
        &raw mut ack_delay,
        &raw mut consumed,
        (*cnx).remote_parameters.ack_delay_exponent,
    ) != 0 as ::core::ffi::c_int
    {
        bytes = ::core::ptr::null::<uint8_t>();
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            ftype,
        );
    } else if has_path_id != 0 && (*cnx).is_multipath_enabled() == 0 {
        bytes = ::core::ptr::null::<uint8_t>();
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
            ftype,
        );
    } else {
        if pc as ::core::ffi::c_uint
            == picoquic_packet_context_application as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if (*cnx).is_multipath_enabled() != 0 {
                let mut path_index: ::core::ffi::c_int =
                    picoquic_find_path_by_unique_id(cnx, path_id);
                if path_index < 0 as ::core::ffi::c_int {
                    bytes =
                        picoquic_skip_ack_frame_maybe_ecn(bytes, bytes_max, is_ecn, has_path_id);
                    return bytes;
                } else {
                    pkt_ctx = &raw mut (**(*cnx).path.offset(path_index as isize)).pkt_ctx;
                }
            }
        }
        if largest >= (*pkt_ctx).send_sequence {
            bytes = ::core::ptr::null::<uint8_t>();
            picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                ftype,
            );
        } else {
            bytes = bytes.offset(consumed as isize);
            let mut time_stamp: uint64_t = 0 as uint64_t;
            let mut is_new_ack: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut top_packet: *mut picoquic_packet_t = picoquic_find_acked_packet(
                cnx,
                pkt_ctx,
                largest,
                current_time,
                &raw mut is_new_ack,
            );
            let mut p_retransmitted_previous: *mut picoquic_packet_t =
                (*pkt_ctx).retransmitted_newest;
            if !top_packet.is_null() && is_new_ack != 0 {
                largest_in_path = (*top_packet).sequence_number;
                ack_path = (*top_packet).send_path as *mut picoquic_path_t;
                if (*pkt_ctx).latest_time_acknowledged < (*top_packet).send_time {
                    (*pkt_ctx).latest_time_acknowledged = (*top_packet).send_time;
                }
                (*cnx).latest_receive_time = current_time;
                if !packet_data.is_null() {
                    (*packet_data).last_ack_delay = ack_delay;
                }
            }
            loop {
                let mut range: uint64_t = 0;
                let mut block_to_block: uint64_t = 0;
                bytes = picoquic_frames_varint_decode(bytes, bytes_max, &raw mut range);
                if bytes.is_null() {
                    picoquic_connection_error(
                        cnx,
                        PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
                        ftype,
                    );
                    bytes = ::core::ptr::null::<uint8_t>();
                    break;
                } else {
                    range = range.wrapping_add(1);
                    if largest.wrapping_add(1 as uint64_t) < range {
                        picoquic_connection_error(
                            cnx,
                            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
                            ftype,
                        );
                        bytes = ::core::ptr::null::<uint8_t>();
                        break;
                    } else if picoquic_process_ack_range(
                        cnx,
                        pc,
                        pkt_ctx,
                        largest,
                        range,
                        &raw mut top_packet,
                        current_time,
                        packet_data,
                    ) != 0 as ::core::ffi::c_int
                    {
                        bytes = ::core::ptr::null::<uint8_t>();
                        break;
                    } else {
                        if (*cnx).cnx_state as ::core::ffi::c_uint
                            != picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
                            && range > 0 as uint64_t
                        {
                            p_retransmitted_previous = picoquic_check_spurious_retransmission(
                                cnx,
                                pc,
                                pkt_ctx,
                                largest.wrapping_add(1 as uint64_t).wrapping_sub(range),
                                largest,
                                current_time,
                                time_stamp,
                                p_retransmitted_previous,
                                packet_data,
                            );
                        }
                        let c2rust_fresh11 = num_block;
                        num_block = num_block.wrapping_sub(1);
                        if c2rust_fresh11 == 0 as uint64_t {
                            break;
                        }
                        bytes = picoquic_frames_varint_decode(
                            bytes,
                            bytes_max,
                            &raw mut block_to_block,
                        );
                        if bytes.is_null() {
                            picoquic_connection_error(
                                cnx,
                                PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
                                ftype,
                            );
                            bytes = ::core::ptr::null::<uint8_t>();
                            break;
                        } else {
                            block_to_block = block_to_block.wrapping_add(1 as uint64_t);
                            block_to_block = block_to_block.wrapping_add(range);
                            if largest < block_to_block {
                                picoquic_connection_error(
                                    cnx,
                                    PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
                                    ftype,
                                );
                                bytes = ::core::ptr::null::<uint8_t>();
                                break;
                            } else {
                                largest = largest.wrapping_sub(block_to_block);
                                if bytes.is_null() {
                                    break;
                                }
                            }
                        }
                    }
                }
            }
            picoquic_dequeue_old_retransmitted_packets(cnx, pkt_ctx);
        }
    }
    if !bytes.is_null() && is_ecn != 0 {
        let mut ecnx: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !bytes.is_null() && ecnx < 3 as ::core::ffi::c_int {
            bytes = picoquic_frames_varint_decode(
                bytes,
                bytes_max,
                (&raw mut ecnx3 as *mut uint64_t).offset(ecnx as isize) as *mut uint64_t,
            );
            ecnx += 1;
        }
    }
    if !bytes.is_null() && is_ecn != 0 {
        if ecnx3[0 as ::core::ffi::c_int as usize] > (*pkt_ctx).ecn_ect0_total_remote {
            (*pkt_ctx).ecn_ect0_total_remote = ecnx3[0 as ::core::ffi::c_int as usize];
        }
        if ecnx3[1 as ::core::ffi::c_int as usize] > (*pkt_ctx).ecn_ect1_total_remote {
            (*pkt_ctx).ecn_ect1_total_remote = ecnx3[1 as ::core::ffi::c_int as usize];
        }
        if ecnx3[2 as ::core::ffi::c_int as usize] > (*pkt_ctx).ecn_ce_total_remote {
            let mut ack_state: picoquic_per_ack_state_t = {
                let mut init = st_picoquic_per_ack_state_t {
                    is_app_limited_is_cwnd_limited: [0; 1],
                    c2rust_padding: [0; 7],
                    rtt_measurement: 0 as uint64_t,
                    one_way_delay: 0,
                    nb_bytes_acknowledged: 0,
                    nb_bytes_newly_lost: 0,
                    nb_bytes_lost_since_packet_sent: 0,
                    nb_bytes_delivered_since_packet_sent: 0,
                    inflight_prior: 0,
                    lost_packet_number: 0,
                    lost_packet_sent_time: 0,
                };
                init.set_is_app_limited(0);
                init.set_is_cwnd_limited(0);
                init
            };
            ack_state.lost_packet_number = largest_in_path;
            (*pkt_ctx).ecn_ce_total_remote = ecnx3[2 as ::core::ffi::c_int as usize];
            (*(*cnx).congestion_alg)
                .alg_notify
                .expect("non-null function pointer")(
                cnx as *mut picoquic_cnx_t,
                ack_path as *mut picoquic_path_t,
                picoquic_congestion_notification_ecn_ec,
                &raw mut ack_state,
                current_time,
            );
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_ack_frame_in_context(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut ack_ctx: *mut picoquic_ack_context_t,
    mut need_time_stamp: *mut ::core::ffi::c_int,
    mut multipath_sequence: uint64_t,
    mut is_opportunistic: ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut num_block: uint64_t = 0 as uint64_t;
    let mut ack_delay: uint64_t = 0 as uint64_t;
    let mut ack_range: uint64_t = 0 as uint64_t;
    let mut ack_gap: uint64_t = 0 as uint64_t;
    let mut lowest_acknowledged: uint64_t = 0 as uint64_t;
    let mut is_ecn: ::core::ffi::c_int = (*ack_ctx).sending_ecn_ack() as ::core::ffi::c_int;
    let mut after_stamp: *mut uint8_t = bytes;
    let mut ack_type_byte: uint64_t = (if multipath_sequence == UINT64_MAX as uint64_t {
        if is_ecn != 0 {
            picoquic_frame_type_ack_ecn as ::core::ffi::c_int
        } else {
            picoquic_frame_type_ack as ::core::ffi::c_int
        }
    } else if is_ecn != 0 {
        picoquic_frame_type_path_ack_ecn as ::core::ffi::c_int
    } else {
        picoquic_frame_type_path_ack as ::core::ffi::c_int
    }) as uint64_t;
    if picoquic_sack_list_is_empty(&raw mut (*ack_ctx).sack_list) == 0 {
        let mut num_block_byte: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut last_sack: *mut picoquic_sack_item_t =
            picoquic_sack_last_item(&raw mut (*ack_ctx).sack_list);
        if current_time > (*ack_ctx).time_stamp_largest_received {
            ack_delay = current_time.wrapping_sub((*ack_ctx).time_stamp_largest_received);
            ack_delay >>= (*cnx).local_parameters.ack_delay_exponent as ::core::ffi::c_int;
        }
        if *need_time_stamp != 0 {
            bytes =
                picoquic_format_time_stamp_frame(cnx, bytes, bytes_max, more_data, current_time);
            after_stamp = bytes;
            *need_time_stamp = 0 as ::core::ffi::c_int;
        }
        bytes = picoquic_frames_varint_encode(bytes, bytes_max, ack_type_byte);
        if !bytes.is_null()
            && (multipath_sequence == UINT64_MAX as uint64_t || {
                bytes = picoquic_frames_varint_encode(bytes, bytes_max, multipath_sequence);
                !bytes.is_null()
            })
            && {
                bytes = picoquic_frames_varint_encode(
                    bytes,
                    bytes_max,
                    picoquic_sack_item_range_end(last_sack),
                );
                !bytes.is_null()
            }
            && {
                bytes = picoquic_frames_varint_encode(bytes, bytes_max, ack_delay);
                !bytes.is_null()
            }
        {
            let c2rust_fresh7 = bytes;
            bytes = bytes.offset(1);
            num_block_byte = c2rust_fresh7;
            ack_range = picoquic_sack_item_range_end(last_sack)
                .wrapping_sub(picoquic_sack_item_range_start(last_sack));
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, ack_range);
        }
        if bytes.is_null() || num_block_byte.is_null() {
            bytes = after_stamp;
            *more_data = 1 as ::core::ffi::c_int;
        } else {
            let mut nb_sent_max_acked: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut nb_sent_max_skip: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut next_sack: *mut picoquic_sack_item_t = picoquic_sack_previous_item(last_sack);
            picoquic_sack_item_record_sent(
                &raw mut (*ack_ctx).sack_list,
                last_sack,
                is_opportunistic,
            );
            picoquic_sack_select_ack_ranges(
                &raw mut (*ack_ctx).sack_list,
                last_sack,
                32 as ::core::ffi::c_int,
                is_opportunistic,
                &raw mut nb_sent_max_acked,
                &raw mut nb_sent_max_skip,
            );
            lowest_acknowledged = picoquic_sack_item_range_start(last_sack);
            while num_block < 32 as uint64_t && !next_sack.is_null() {
                if picoquic_sack_item_nb_times_sent(next_sack, is_opportunistic)
                    <= nb_sent_max_acked
                {
                    if picoquic_sack_item_nb_times_sent(next_sack, is_opportunistic)
                        == nb_sent_max_acked
                        && nb_sent_max_skip > 0 as ::core::ffi::c_int
                    {
                        nb_sent_max_skip -= 1;
                    } else {
                        let mut bytes_start_range: *mut uint8_t = bytes;
                        ack_gap = lowest_acknowledged
                            .wrapping_sub(picoquic_sack_item_range_end(next_sack))
                            .wrapping_sub(2 as uint64_t);
                        ack_range = picoquic_sack_item_range_end(next_sack)
                            .wrapping_sub(picoquic_sack_item_range_start(next_sack));
                        bytes = picoquic_frames_varint_encode(bytes, bytes_max, ack_gap);
                        if bytes.is_null() || {
                            bytes = picoquic_frames_varint_encode(bytes, bytes_max, ack_range);
                            bytes.is_null()
                        } {
                            bytes = bytes_start_range;
                            *more_data = 1 as ::core::ffi::c_int;
                            break;
                        } else {
                            picoquic_sack_item_record_sent(
                                &raw mut (*ack_ctx).sack_list,
                                next_sack,
                                is_opportunistic,
                            );
                            lowest_acknowledged = picoquic_sack_item_range_start(next_sack);
                            num_block = num_block.wrapping_add(1);
                        }
                    }
                }
                next_sack = picoquic_sack_previous_item(next_sack);
            }
            *num_block_byte = num_block as uint8_t;
            if is_opportunistic == 0 {
                (*ack_ctx).act[0 as ::core::ffi::c_int as usize].highest_ack_sent =
                    picoquic_sack_list_last(&raw mut (*ack_ctx).sack_list);
                (*ack_ctx).act[0 as ::core::ffi::c_int as usize].highest_ack_sent_time =
                    current_time;
            } else {
                (*ack_ctx).act[1 as ::core::ffi::c_int as usize].highest_ack_sent =
                    picoquic_sack_list_last(&raw mut (*ack_ctx).sack_list);
                (*ack_ctx).act[1 as ::core::ffi::c_int as usize].highest_ack_sent_time =
                    current_time;
            }
        }
        if bytes > after_stamp && is_ecn != 0 {
            let mut bytes_ecn: *mut uint8_t = bytes;
            bytes =
                picoquic_frames_varint_encode(bytes, bytes_max, (*ack_ctx).ecn_ect0_total_local);
            if bytes.is_null()
                || {
                    bytes = picoquic_frames_varint_encode(
                        bytes,
                        bytes_max,
                        (*ack_ctx).ecn_ect1_total_local,
                    );
                    bytes.is_null()
                }
                || {
                    bytes = picoquic_frames_varint_encode(
                        bytes,
                        bytes_max,
                        (*ack_ctx).ecn_ce_total_local,
                    );
                    bytes.is_null()
                }
            {
                bytes = bytes_ecn;
                *more_data = 1 as ::core::ffi::c_int;
                *after_stamp = picoquic_frame_type_ack as ::core::ffi::c_int as uint8_t;
            }
        }
    }
    if bytes > after_stamp {
        if is_opportunistic != 0 {
            (*ack_ctx).act[1 as ::core::ffi::c_int as usize]
                .set_ack_needed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*ack_ctx).act[1 as ::core::ffi::c_int as usize]
                .set_ack_after_fin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*ack_ctx).act[1 as ::core::ffi::c_int as usize]
                .set_out_of_order_received(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        } else {
            (*cnx).set_is_immediate_ack_required(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*ack_ctx).act[0 as ::core::ffi::c_int as usize]
                .set_ack_needed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*ack_ctx).act[0 as ::core::ffi::c_int as usize]
                .set_ack_after_fin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*ack_ctx).act[0 as ::core::ffi::c_int as usize]
                .set_out_of_order_received(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*ack_ctx).act[0 as ::core::ffi::c_int as usize]
                .set_is_immediate_ack_required(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_ack_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut pc: picoquic_packet_context_enum,
    mut is_opportunistic: ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut need_time_stamp: ::core::ffi::c_int = (pc as ::core::ffi::c_uint
        == picoquic_packet_context_application as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*cnx).is_time_stamp_sent() as ::core::ffi::c_int != 0)
        as ::core::ffi::c_int;
    let mut ack_ctx: *mut picoquic_ack_context_t =
        ::core::ptr::null_mut::<picoquic_ack_context_t>();
    if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
        && pc as ::core::ffi::c_uint
            == picoquic_packet_context_application as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut ack_still_needed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut ack_after_fin: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut path_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while path_id < (*cnx).nb_paths {
            if !bytes.is_null() {
                ack_ctx = &raw mut (**(*cnx).path.offset(path_id as isize)).ack_ctx;
                bytes = picoquic_format_ack_frame_in_context(
                    cnx,
                    bytes,
                    bytes_max,
                    more_data,
                    current_time,
                    ack_ctx,
                    &raw mut need_time_stamp,
                    (**(*cnx).path.offset(path_id as isize)).unique_path_id,
                    is_opportunistic,
                );
                if is_opportunistic != 0 {
                    ack_still_needed |= (*ack_ctx).act[1 as ::core::ffi::c_int as usize]
                        .ack_needed() as ::core::ffi::c_int;
                    ack_after_fin |= (*ack_ctx).act[1 as ::core::ffi::c_int as usize]
                        .ack_after_fin() as ::core::ffi::c_int;
                } else {
                    ack_still_needed |= (*ack_ctx).act[0 as ::core::ffi::c_int as usize]
                        .ack_needed() as ::core::ffi::c_int;
                    ack_after_fin |= (*ack_ctx).act[0 as ::core::ffi::c_int as usize]
                        .ack_after_fin() as ::core::ffi::c_int;
                }
            }
            path_id += 1;
        }
        if is_opportunistic != 0 {
            (*cnx).ack_ctx[pc as usize].act[1 as ::core::ffi::c_int as usize]
                .set_ack_needed(ack_still_needed as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*cnx).ack_ctx[pc as usize].act[1 as ::core::ffi::c_int as usize]
                .set_ack_after_fin(ack_after_fin as ::core::ffi::c_uint as ::core::ffi::c_uint);
        } else {
            (*cnx).ack_ctx[pc as usize].act[0 as ::core::ffi::c_int as usize]
                .set_ack_needed(ack_still_needed as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*cnx).ack_ctx[pc as usize].act[0 as ::core::ffi::c_int as usize]
                .set_ack_after_fin(ack_after_fin as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    } else {
        bytes = picoquic_format_ack_frame_in_context(
            cnx,
            bytes,
            bytes_max,
            more_data,
            current_time,
            (&raw mut (*cnx).ack_ctx as *mut picoquic_ack_context_t).offset(pc as isize)
                as *mut picoquic_ack_context_t,
            &raw mut need_time_stamp,
            UINT64_MAX as uint64_t,
            is_opportunistic,
        );
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_ack_needed(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
    mut pc: picoquic_packet_context_enum,
    mut path_x: *mut picoquic_path_t,
    mut is_immediate_ack_required: ::core::ffi::c_int,
) {
    if pc as ::core::ffi::c_uint
        == picoquic_packet_context_application as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
    {
        (*path_x).ack_ctx.act[0 as ::core::ffi::c_int as usize].set_is_immediate_ack_required(
            (*path_x).ack_ctx.act[0 as ::core::ffi::c_int as usize].is_immediate_ack_required()
                | is_immediate_ack_required as ::core::ffi::c_uint,
        );
        if (*path_x).ack_ctx.act[0 as ::core::ffi::c_int as usize].ack_needed() == 0 {
            (*path_x).ack_ctx.act[0 as ::core::ffi::c_int as usize]
                .set_ack_needed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*path_x).ack_ctx.act[0 as ::core::ffi::c_int as usize]
                .time_oldest_unack_packet_received = current_time;
            (*path_x).ack_ctx.act[1 as ::core::ffi::c_int as usize]
                .set_ack_needed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*path_x).ack_ctx.act[1 as ::core::ffi::c_int as usize]
                .time_oldest_unack_packet_received = current_time;
        }
    }
    if (*cnx).ack_ctx[pc as usize].act[0 as ::core::ffi::c_int as usize].ack_needed() == 0 {
        (*cnx).ack_ctx[pc as usize].act[0 as ::core::ffi::c_int as usize]
            .set_is_immediate_ack_required(
                (*cnx).ack_ctx[pc as usize].act[0 as ::core::ffi::c_int as usize]
                    .is_immediate_ack_required()
                    | is_immediate_ack_required as ::core::ffi::c_uint,
            );
        (*cnx).ack_ctx[pc as usize].act[0 as ::core::ffi::c_int as usize]
            .set_ack_needed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*cnx).ack_ctx[pc as usize].act[0 as ::core::ffi::c_int as usize]
            .time_oldest_unack_packet_received = current_time;
        (*cnx).ack_ctx[pc as usize].act[1 as ::core::ffi::c_int as usize]
            .set_ack_needed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*cnx).ack_ctx[pc as usize].act[1 as ::core::ffi::c_int as usize]
            .time_oldest_unack_packet_received = current_time;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_ack_gap_override_if_needed(
    mut cnx: *mut picoquic_cnx_t,
    mut path_index: ::core::ffi::c_int,
) -> uint64_t {
    let mut ack_gap: uint64_t = (*cnx).ack_gap_remote;
    if (*cnx).is_multipath_enabled() != 0 {
        if (**(*cnx).path.offset(path_index as isize)).path_is_demoted() == 0
            && (**(*cnx).path.offset(path_index as isize)).challenge_failed() == 0
            && (**(*cnx).path.offset(path_index as isize)).response_required() == 0
            && (**(*cnx).path.offset(path_index as isize)).challenge_verified()
                as ::core::ffi::c_int
                != 0
            && (**(*cnx).path.offset(path_index as isize)).received
                < (100 as ::core::ffi::c_int * PICOQUIC_MAX_PACKET_SIZE) as uint64_t
        {
            ack_gap = 2 as uint64_t;
        }
    } else if (*cnx).nb_packets_received < 128 as uint64_t {
        ack_gap = 2 as uint64_t;
    }
    return ack_gap;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_ack_needed_in_ctx(
    mut cnx: *mut picoquic_cnx_t,
    mut ack_ctx: *mut picoquic_ack_context_t,
    mut current_time: uint64_t,
    mut path_index: ::core::ffi::c_int,
    mut next_wake_time: *mut uint64_t,
    mut pc: picoquic_packet_context_enum,
    mut is_opportunistic: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*ack_ctx).act[is_opportunistic as usize].ack_needed() != 0 {
        if (*cnx).no_ack_delay() != 0 {
            ret = 1 as ::core::ffi::c_int;
        } else if (*ack_ctx).act[is_opportunistic as usize].is_immediate_ack_required() != 0 {
            ret = 1 as ::core::ffi::c_int;
        } else if pc as ::core::ffi::c_uint
            != picoquic_packet_context_application as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*ack_ctx).act[is_opportunistic as usize].ack_after_fin() as ::core::ffi::c_int != 0
        {
            ret = 1 as ::core::ffi::c_int;
            (*ack_ctx).act[is_opportunistic as usize]
                .set_ack_after_fin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        } else if (*ack_ctx).act[is_opportunistic as usize].out_of_order_received()
            as ::core::ffi::c_int
            != 0
            && (*cnx).ack_ignore_order_remote() == 0
        {
            ret = 1 as ::core::ffi::c_int;
        } else {
            let mut ack_gap: uint64_t = picoquic_ack_gap_override_if_needed(cnx, path_index);
            if (*ack_ctx).act[is_opportunistic as usize]
                .highest_ack_sent
                .wrapping_add(ack_gap)
                <= picoquic_sack_list_last(&raw mut (*ack_ctx).sack_list)
                || (*ack_ctx).act[is_opportunistic as usize]
                    .time_oldest_unack_packet_received
                    .wrapping_add((*cnx).ack_delay_remote)
                    <= current_time
            {
                ret = 1 as ::core::ffi::c_int;
            } else if (*ack_ctx).act[is_opportunistic as usize]
                .time_oldest_unack_packet_received
                .wrapping_add((*cnx).ack_delay_remote)
                < *next_wake_time
            {
                *next_wake_time = (*ack_ctx).act[is_opportunistic as usize]
                    .time_oldest_unack_packet_received
                    .wrapping_add((*cnx).ack_delay_remote);
                (*(*cnx).quic).wake_file = 4 as ::core::ffi::c_int;
                (*(*cnx).quic).wake_line = 4052 as ::core::ffi::c_int;
            }
        }
    } else if (*ack_ctx).act[is_opportunistic as usize]
        .highest_ack_sent
        .wrapping_add(8 as uint64_t)
        <= picoquic_sack_list_last(&raw mut (*ack_ctx).sack_list)
        && (*ack_ctx).act[is_opportunistic as usize]
            .highest_ack_sent_time
            .wrapping_add((*cnx).ack_delay_remote)
            <= current_time
    {
        if picoquic_sack_list_last(&raw mut (*ack_ctx).sack_list) == UINT64_MAX as uint64_t {
            ret = 0 as ::core::ffi::c_int;
        } else {
            ret = 1 as ::core::ffi::c_int;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_ack_needed(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
    mut next_wake_time: *mut uint64_t,
    mut pc: picoquic_packet_context_enum,
    mut is_opportunistic: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = picoquic_is_ack_needed_in_ctx(
        cnx,
        (&raw mut (*cnx).ack_ctx as *mut picoquic_ack_context_t).offset(pc as isize)
            as *mut picoquic_ack_context_t,
        current_time,
        0 as ::core::ffi::c_int,
        next_wake_time,
        pc,
        is_opportunistic,
    );
    if pc as ::core::ffi::c_uint
        == picoquic_packet_context_application as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*cnx).is_multipath_enabled() != 0 {
            let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while ret == 0 as ::core::ffi::c_int && i < (*cnx).nb_paths {
                ret |= picoquic_is_ack_needed_in_ctx(
                    cnx,
                    &raw mut (**(*cnx).path.offset(i as isize)).ack_ctx,
                    current_time,
                    i,
                    next_wake_time,
                    pc,
                    is_opportunistic,
                );
                i += 1;
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_connection_close_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_uint8_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_connection_close as ::core::ffi::c_int as uint8_t,
    );
    if !bytes.is_null()
        && {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, (*cnx).local_error);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, (*cnx).offending_frame_type);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_charz_encode(bytes, bytes_max, (*cnx).local_error_reason);
            !bytes.is_null()
        }
    {
        *is_pure_ack = 0 as ::core::ffi::c_int;
    } else {
        bytes = bytes0;
        *more_data = 1 as ::core::ffi::c_int;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_connection_close_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max,
        &raw mut (*cnx).remote_error,
    );
    if bytes.is_null()
        || {
            bytes = picoquic_frames_varint_skip(bytes, bytes_max);
            bytes.is_null()
        }
        || {
            bytes = picoquic_frames_length_data_skip(bytes, bytes_max);
            bytes.is_null()
        }
    {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_connection_close as ::core::ffi::c_int as uint64_t,
        );
    } else {
        let mut old_state: picoquic_state_enum = (*cnx).cnx_state;
        (*cnx).cnx_state = (if ((*cnx).cnx_state as ::core::ffi::c_uint)
            < picoquic_state_client_ready_start as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*cnx).crypto_context[picoquic_epoch_1rtt as ::core::ffi::c_int as usize]
                .aead_decrypt
                .is_null()
        {
            picoquic_state_disconnected as ::core::ffi::c_int
        } else {
            picoquic_state_closing_received as ::core::ffi::c_int
        }) as picoquic_state_enum;
        if (*cnx).callback_fn.is_some()
            && (*cnx).cnx_state as ::core::ffi::c_uint != old_state as ::core::ffi::c_uint
            && (*cnx).cnx_state as ::core::ffi::c_uint
                == picoquic_state_disconnected as ::core::ffi::c_int as ::core::ffi::c_uint
        {
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
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_application_close_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_uint8_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_application_close as ::core::ffi::c_int as uint8_t,
    );
    if !bytes.is_null()
        && {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, (*cnx).application_error);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_uint8_encode(bytes, bytes_max, 0 as uint8_t);
            !bytes.is_null()
        }
    {
        *is_pure_ack = 0 as ::core::ffi::c_int;
    } else {
        bytes = bytes0;
        *more_data = 1 as ::core::ffi::c_int;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_application_close_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max,
        &raw mut (*cnx).remote_application_error,
    );
    if bytes.is_null() || {
        bytes = picoquic_frames_length_data_skip(bytes, bytes_max);
        bytes.is_null()
    } {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_application_close as ::core::ffi::c_int as uint64_t,
        );
    } else {
        (*cnx).cnx_state = (if ((*cnx).cnx_state as ::core::ffi::c_uint)
            < picoquic_state_client_ready_start as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            picoquic_state_disconnected as ::core::ffi::c_int
        } else {
            picoquic_state_closing_received as ::core::ffi::c_int
        }) as picoquic_state_enum;
        if (*cnx).callback_fn.is_some() {
            (*cnx).callback_fn.expect("non-null function pointer")(
                cnx as *mut picoquic_cnx_t,
                0 as uint64_t,
                ::core::ptr::null_mut::<uint8_t>(),
                0 as size_t,
                picoquic_callback_application_close,
                (*cnx).callback_ctx,
                NULL,
            );
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_max_data_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut maxdata_increase: uint64_t,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_uint8_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_max_data as ::core::ffi::c_int as uint8_t,
    );
    if !bytes.is_null() && {
        bytes = picoquic_frames_varint_encode(
            bytes,
            bytes_max,
            (*cnx).maxdata_local.wrapping_add(maxdata_increase),
        );
        !bytes.is_null()
    } {
        (*cnx).maxdata_local = (*cnx).maxdata_local.wrapping_add(maxdata_increase);
        *is_pure_ack = 0 as ::core::ffi::c_int;
    } else {
        *more_data = 1 as ::core::ffi::c_int;
        bytes = bytes0;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_max_data_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut maxdata: uint64_t = 0;
    bytes = picoquic_frames_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max,
        &raw mut maxdata,
    );
    if bytes.is_null() {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_max_data as ::core::ffi::c_int as uint64_t,
        );
    } else if maxdata > (*cnx).maxdata_remote {
        (*cnx).maxdata_remote = maxdata;
        (*cnx).set_sent_blocked_frame(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_ack_of_max_data_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut consumed: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut maxdata: uint64_t = 0;
    let mut bytes_next: *const uint8_t = picoquic_frames_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes.offset(bytes_max as isize),
        &raw mut maxdata,
    );
    if !bytes_next.is_null() {
        *consumed = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
        if maxdata > (*cnx).maxdata_local_acked {
            (*cnx).maxdata_local_acked = maxdata;
        }
    } else {
        *consumed = bytes_max;
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_max_stream_data_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut stream: *mut picoquic_stream_head_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut new_max_data: uint64_t,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_uint8_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_max_stream_data as ::core::ffi::c_int as uint8_t,
    );
    if !bytes.is_null()
        && {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, (*stream).stream_id);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, new_max_data);
            !bytes.is_null()
        }
    {
        (*stream).maxdata_local = new_max_data;
        if new_max_data > (*cnx).max_stream_data_local {
            (*cnx).max_stream_data_local = new_max_data;
        }
        *is_pure_ack = 0 as ::core::ffi::c_int;
    } else {
        *more_data = 1 as ::core::ffi::c_int;
        bytes = bytes0;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_max_stream_data_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut stream_id: uint64_t = 0;
    let mut maxdata: uint64_t = 0 as uint64_t;
    let mut stream: *mut picoquic_stream_head_t = ::core::ptr::null_mut::<picoquic_stream_head_t>();
    bytes = picoquic_frames_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max,
        &raw mut stream_id,
    );
    if bytes.is_null() || {
        bytes = picoquic_frames_varint_decode(bytes, bytes_max, &raw mut maxdata);
        bytes.is_null()
    } {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_max_stream_data as ::core::ffi::c_int as uint64_t,
        );
    } else {
        stream = picoquic_find_stream(cnx, stream_id);
        if stream.is_null() {
            stream = picoquic_create_missing_streams(cnx, stream_id, 1 as ::core::ffi::c_int);
        }
    }
    if !stream.is_null() && maxdata > (*stream).maxdata_remote {
        (*stream).maxdata_remote = maxdata;
        if maxdata > (*cnx).max_stream_data_remote {
            (*cnx).max_stream_data_remote = maxdata;
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_ack_of_max_stream_data_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_size: size_t,
    mut consumed: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream_id: uint64_t = 0;
    let mut maxdata: uint64_t = 0;
    let mut bytes_next: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut bytes_max: *const uint8_t = bytes.offset(bytes_size as isize);
    bytes_next = picoquic_frames_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max,
        &raw mut stream_id,
    );
    if !bytes_next.is_null() && {
        bytes_next = picoquic_frames_varint_decode(bytes_next, bytes_max, &raw mut maxdata);
        !bytes_next.is_null()
    } {
        let mut stream: *mut picoquic_stream_head_t =
            ::core::ptr::null_mut::<picoquic_stream_head_t>();
        *consumed = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
        stream = picoquic_find_stream(cnx, stream_id);
        if !stream.is_null() {
            if maxdata > (*stream).maxdata_local_acked {
                (*stream).maxdata_local_acked = maxdata;
            }
        }
    } else {
        *consumed = bytes_size;
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_required_max_stream_data_frames(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut stream: *mut picoquic_stream_head_t = picoquic_first_stream(cnx);
    while !stream.is_null() {
        if (*stream).fin_received() == 0 {
            let mut new_window: uint64_t =
                picoquic_cc_increased_window(cnx, (*stream).maxdata_local);
            if (*stream).reset_received() == 0
                && (2 as uint64_t).wrapping_mul((*stream).consumed_offset) > (*stream).maxdata_local
            {
                bytes0 = bytes;
                bytes = picoquic_format_max_stream_data_frame(
                    cnx,
                    stream,
                    bytes,
                    bytes_max,
                    more_data,
                    is_pure_ack,
                    (*stream).maxdata_local.wrapping_add(new_window),
                );
                if bytes == bytes0 {
                    break;
                }
            }
        }
        stream = picoquic_next_stream(stream);
    }
    if stream.is_null() {
        (*cnx).set_max_stream_data_needed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_max_streams_frame_if_needed(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    if (*cnx).max_stream_id_bidir_local_computed.wrapping_add(
        (2 as uint64_t).wrapping_mul((*cnx).local_parameters.initial_max_stream_id_bidir),
    ) > (*cnx).max_stream_id_bidir_local
    {
        let mut new_bidir_local: uint64_t = (*cnx).max_stream_id_bidir_local.wrapping_add(
            (4 as uint64_t).wrapping_mul((*cnx).local_parameters.initial_max_stream_id_bidir),
        );
        bytes = picoquic_frames_uint8_encode(
            bytes,
            bytes_max,
            picoquic_frame_type_max_streams_bidir as ::core::ffi::c_int as uint8_t,
        );
        if !bytes.is_null() && {
            bytes = picoquic_frames_varint_encode(
                bytes,
                bytes_max,
                new_bidir_local.wrapping_add(4 as uint64_t) >> 2 as ::core::ffi::c_int,
            );
            !bytes.is_null()
        } {
            (*cnx).max_stream_id_bidir_local = new_bidir_local;
            *is_pure_ack = 0 as ::core::ffi::c_int;
            bytes0 = bytes;
        } else {
            *more_data = 1 as ::core::ffi::c_int;
            bytes = bytes0;
        }
    }
    if (*cnx).max_stream_id_unidir_local_computed.wrapping_add(
        (2 as uint64_t).wrapping_mul((*cnx).local_parameters.initial_max_stream_id_unidir),
    ) > (*cnx).max_stream_id_unidir_local
    {
        let mut new_unidir_local: uint64_t = (*cnx).max_stream_id_unidir_local.wrapping_add(
            (4 as uint64_t).wrapping_mul((*cnx).local_parameters.initial_max_stream_id_unidir),
        );
        bytes = picoquic_frames_uint8_encode(
            bytes,
            bytes_max,
            picoquic_frame_type_max_streams_unidir as ::core::ffi::c_int as uint8_t,
        );
        if !bytes.is_null() && {
            bytes = picoquic_frames_varint_encode(
                bytes,
                bytes_max,
                new_unidir_local.wrapping_add(4 as uint64_t) >> 2 as ::core::ffi::c_int,
            );
            !bytes.is_null()
        } {
            (*cnx).max_stream_id_unidir_local = new_unidir_local;
            *is_pure_ack = 0 as ::core::ffi::c_int;
        } else {
            *more_data = 1 as ::core::ffi::c_int;
            bytes = bytes0;
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_update_max_stream_ID_local(
    mut cnx: *mut picoquic_cnx_t,
    mut stream: *mut picoquic_stream_head_t,
) {
    if (*cnx).client_mode()
        != ((*stream).stream_id & 1 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
            as ::core::ffi::c_uint
        && (*stream).max_stream_updated() == 0
    {
        if (*stream).consumed_offset >= (*stream).fin_offset
            && ((*stream).fin_received() as ::core::ffi::c_int != 0
                || (*stream).reset_received() as ::core::ffi::c_int != 0)
        {
            if ((*stream).stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
                as ::core::ffi::c_uint
                != 0
            {
                if (*stream).fin_sent() as ::core::ffi::c_int != 0
                    || (*stream).reset_sent() as ::core::ffi::c_int != 0
                {
                    (*stream)
                        .set_max_stream_updated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*cnx).max_stream_id_bidir_local_computed = (*cnx)
                        .max_stream_id_bidir_local_computed
                        .wrapping_add(4 as uint64_t);
                }
            } else {
                (*stream).set_max_stream_updated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*cnx).max_stream_id_unidir_local_computed = (*cnx)
                    .max_stream_id_unidir_local_computed
                    .wrapping_add(4 as uint64_t);
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_max_streams_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut max_streams_frame_type: ::core::ffi::c_int,
) -> *const uint8_t {
    let mut max_stream_rank: uint64_t = 0;
    bytes = picoquic_frames_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max,
        &raw mut max_stream_rank,
    );
    if bytes.is_null() {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            max_streams_frame_type as uint64_t,
        );
    } else {
        let mut max_stream_id: uint64_t = 0;
        if max_streams_frame_type == picoquic_frame_type_max_streams_bidir as ::core::ffi::c_int {
            max_stream_id = max_stream_rank.wrapping_sub(1 as ::core::ffi::c_int as uint64_t)
                << 2 as ::core::ffi::c_int
                | (0 as ::core::ffi::c_int as uint64_t) << 1 as ::core::ffi::c_int
                | ((*cnx).client_mode() as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int)
                    as uint64_t;
            if max_stream_id > (*cnx).max_stream_id_bidir_remote {
                let mut old_limit: uint64_t = (*cnx).max_stream_id_bidir_remote;
                (*cnx).max_stream_id_bidir_remote = max_stream_id;
                picoquic_add_output_streams(
                    cnx,
                    old_limit,
                    max_stream_id,
                    1 as ::core::ffi::c_uint,
                );
                (*cnx)
                    .set_stream_blocked_bidir_sent(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        } else {
            max_stream_id = max_stream_rank.wrapping_sub(1 as ::core::ffi::c_int as uint64_t)
                << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int as uint64_t) << 1 as ::core::ffi::c_int
                | ((*cnx).client_mode() as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int)
                    as uint64_t;
            if max_stream_id > (*cnx).max_stream_id_unidir_remote {
                let mut old_limit_0: uint64_t = (*cnx).max_stream_id_unidir_remote;
                (*cnx).max_stream_id_unidir_remote = max_stream_id;
                picoquic_add_output_streams(
                    cnx,
                    old_limit_0,
                    max_stream_id,
                    0 as ::core::ffi::c_uint,
                );
                (*cnx).set_stream_blocked_unidir_sent(
                    0 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                );
            }
        }
        if max_stream_id as ::core::ffi::c_ulonglong
            >= (1 as ::core::ffi::c_ulonglong) << 62 as ::core::ffi::c_int
        {
            picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_STREAM_LIMIT_ERROR as uint64_t,
                max_streams_frame_type as uint64_t,
            );
            bytes = ::core::ptr::null::<uint8_t>();
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_ack_of_max_streams_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_size: size_t,
    mut consumed: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max_stream_rank: uint64_t = 0;
    let mut bytes_next: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut bytes_max: *const uint8_t = bytes.offset(bytes_size as isize);
    bytes_next = picoquic_frames_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max,
        &raw mut max_stream_rank,
    );
    if !bytes_next.is_null() {
        *consumed = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
        if *bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == picoquic_frame_type_max_streams_bidir as ::core::ffi::c_int
        {
            if max_stream_rank > (*cnx).max_stream_id_bidir_rank_acked {
                (*cnx).max_stream_id_bidir_rank_acked = max_stream_rank;
            }
        } else if max_stream_rank > (*cnx).max_stream_id_unidir_rank_acked {
            (*cnx).max_stream_id_unidir_rank_acked = max_stream_rank;
        }
    } else {
        *consumed = bytes_size;
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_check_max_streams_frame_needs_repeat(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut p_last_byte: *const uint8_t,
    mut no_need_to_repeat: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max_stream_rank: uint64_t = 0 as uint64_t;
    if picoquic_frames_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        p_last_byte,
        &raw mut max_stream_rank,
    )
    .is_null()
    {
        *no_need_to_repeat = 1 as ::core::ffi::c_int;
    } else if *bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        == picoquic_frame_type_max_streams_bidir as ::core::ffi::c_int
    {
        if max_stream_rank <= (*cnx).max_stream_id_bidir_rank_acked
            || (*cnx).max_stream_id_bidir_local
                > max_stream_rank.wrapping_sub(1 as ::core::ffi::c_int as uint64_t)
                    << 2 as ::core::ffi::c_int
                    | (0 as ::core::ffi::c_int as uint64_t) << 1 as ::core::ffi::c_int
                    | ((*cnx).client_mode() as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int)
                        as uint64_t
        {
            *no_need_to_repeat = 1 as ::core::ffi::c_int;
        }
    } else if max_stream_rank <= (*cnx).max_stream_id_unidir_rank_acked
        || (*cnx).max_stream_id_unidir_local
            > max_stream_rank.wrapping_sub(1 as ::core::ffi::c_int as uint64_t)
                << 2 as ::core::ffi::c_int
                | (1 as ::core::ffi::c_int as uint64_t) << 1 as ::core::ffi::c_int
                | ((*cnx).client_mode() as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int) as uint64_t
    {
        *no_need_to_repeat = 1 as ::core::ffi::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_first_misc_or_dg_frame(
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut misc_frame: *mut picoquic_misc_frame_header_t,
    mut first: *mut *mut picoquic_misc_frame_header_t,
    mut last: *mut *mut picoquic_misc_frame_header_t,
) -> *mut uint8_t {
    if bytes.offset((*misc_frame).length as isize) > bytes_max {
        *more_data = 1 as ::core::ffi::c_int;
    } else {
        let mut frame: *mut uint8_t = (misc_frame as *mut uint8_t)
            .offset(::core::mem::size_of::<picoquic_misc_frame_header_t>() as usize as isize);
        memcpy(
            bytes as *mut ::core::ffi::c_void,
            frame as *const ::core::ffi::c_void,
            (*misc_frame).length,
        );
        bytes = bytes.offset((*misc_frame).length as isize);
        *is_pure_ack &= (*misc_frame).is_pure_ack;
        picoquic_delete_misc_or_dg(first, last, misc_frame);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_find_first_misc_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut pc: picoquic_packet_context_enum,
) -> *mut picoquic_misc_frame_header_t {
    let mut misc_frame: *mut picoquic_misc_frame_header_t = (*cnx).first_misc_frame;
    while !misc_frame.is_null()
        && (*misc_frame).pc as ::core::ffi::c_uint != pc as ::core::ffi::c_uint
    {
        misc_frame = (*misc_frame).next_misc_frame as *mut picoquic_misc_frame_header_t;
    }
    return misc_frame;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_misc_frames_in_context(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut pc: picoquic_packet_context_enum,
) -> *mut uint8_t {
    let mut misc_frame: *mut picoquic_misc_frame_header_t =
        ::core::ptr::null_mut::<picoquic_misc_frame_header_t>();
    loop {
        misc_frame = picoquic_find_first_misc_frame(cnx, pc);
        if misc_frame.is_null() {
            break;
        }
        let mut bytes_misc: *mut uint8_t = bytes;
        let mut frame_is_pure_ack: ::core::ffi::c_int = (*misc_frame).is_pure_ack;
        bytes = picoquic_format_first_misc_or_dg_frame(
            bytes,
            bytes_max,
            more_data,
            is_pure_ack,
            misc_frame,
            &raw mut (*cnx).first_misc_frame,
            &raw mut (*cnx).last_misc_frame,
        );
        if bytes <= bytes_misc {
            break;
        }
        *is_pure_ack &= frame_is_pure_ack;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_path_challenge_frame(
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut challenge: uint64_t,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_uint8_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_path_challenge as ::core::ffi::c_int as uint8_t,
    );
    if !bytes.is_null() && {
        bytes = picoquic_frames_uint64_encode(bytes, bytes_max, challenge);
        !bytes.is_null()
    } {
        *is_pure_ack = 0 as ::core::ffi::c_int;
    } else {
        *more_data = 1 as ::core::ffi::c_int;
        bytes = bytes0;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_path_challenge_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut path_x: *mut picoquic_path_t,
    mut addr_from: *mut sockaddr,
    mut addr_to: *mut sockaddr,
) -> *const uint8_t {
    if bytes_max.offset_from(bytes) as ::core::ffi::c_long
        <= challenge_length as ::core::ffi::c_int as ::core::ffi::c_long
    {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_path_challenge as ::core::ffi::c_int as uint64_t,
        );
        bytes = ::core::ptr::null::<uint8_t>();
    } else {
        let mut challenge_response: uint64_t = 0;
        bytes = bytes.offset(1);
        challenge_response = (((((*bytes.offset(0 as ::core::ffi::c_int as isize) as uint16_t
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *bytes.offset(1 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
            as uint32_t)
            << 16 as ::core::ffi::c_int
            | ((*bytes
                .offset(2 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *bytes
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t) as uint64_t)
            << 32 as ::core::ffi::c_int
            | ((((*bytes
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *bytes
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t)
                << 16 as ::core::ffi::c_int
                | ((*bytes
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *bytes
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int) as uint32_t) as uint64_t;
        bytes = bytes.offset(challenge_length as isize);
        if path_x.is_null() {
            picoquic_log_app_message(
                cnx as *mut picoquic_cnx_t,
                b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                b"Incoming challenge ignored, path=NULL.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        } else {
            let mut is_valid: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            if (*cnx).is_multipath_enabled() != 0 {
                is_valid = 1 as ::core::ffi::c_int;
            }
            if is_valid == 0 {
                if addr_from.is_null()
                    || picoquic_compare_addr(
                        addr_from,
                        &raw mut (*path_x).peer_addr as *mut sockaddr,
                    ) == 0 as ::core::ffi::c_int
                {
                    if addr_to.is_null()
                        || picoquic_get_addr_port(&raw mut (*path_x).local_addr as *mut sockaddr)
                            as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                            && picoquic_compare_ip_addr(
                                addr_to,
                                &raw mut (*path_x).local_addr as *mut sockaddr,
                            ) == 0 as ::core::ffi::c_int
                        || picoquic_compare_addr(
                            addr_to,
                            &raw mut (*path_x).local_addr as *mut sockaddr,
                        ) == 0 as ::core::ffi::c_int
                    {
                        is_valid = 1 as ::core::ffi::c_int;
                    }
                }
            }
            if is_valid != 0 {
                (*path_x).challenge_response = challenge_response;
                (*path_x).set_response_required(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            } else {
                let mut buf1: [::core::ffi::c_char; 128] = [0; 128];
                let mut buf2: [::core::ffi::c_char; 128] = [0; 128];
                let mut buf3: [::core::ffi::c_char; 128] = [0; 128];
                let mut buf4: [::core::ffi::c_char; 128] = [0; 128];
                picoquic_log_app_message(
                    cnx as *mut picoquic_cnx_t,
                    b"Path challenge[%lu] from %s to %s ignored, wrong addresses, expected %s - %s.\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    (*path_x).unique_path_id,
                    picoquic_addr_text(
                        addr_from,
                        &raw mut buf1 as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                    ),
                    picoquic_addr_text(
                        addr_to,
                        &raw mut buf2 as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                    ),
                    picoquic_addr_text(
                        &raw mut (*path_x).peer_addr as *mut sockaddr,
                        &raw mut buf3 as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                    ),
                    picoquic_addr_text(
                        &raw mut (*path_x).local_addr as *mut sockaddr,
                        &raw mut buf4 as *mut ::core::ffi::c_char,
                        ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as size_t,
                    ),
                );
            }
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_path_response_frame(
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut challenge: uint64_t,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_uint8_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_path_response as ::core::ffi::c_int as uint8_t,
    );
    if !bytes.is_null() && {
        bytes = picoquic_frames_uint64_encode(bytes, bytes_max, challenge);
        !bytes.is_null()
    } {
        *is_pure_ack = 0 as ::core::ffi::c_int;
    } else {
        *more_data = 1 as ::core::ffi::c_int;
        bytes = bytes0;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_path_response_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) -> *const uint8_t {
    let mut response: uint64_t = 0;
    bytes = picoquic_frames_uint64_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max,
        &raw mut response,
    );
    if bytes.is_null() {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_path_response as ::core::ffi::c_int as uint64_t,
        );
    } else if !path_x.is_null() {
        let mut found_challenge: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut found_nat_challenge: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut ichal: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while ichal < PICOQUIC_CHALLENGE_REPEAT_MAX {
            if response == (*path_x).challenge[ichal as usize] {
                found_challenge = 1 as ::core::ffi::c_int;
                break;
            } else {
                ichal += 1;
            }
        }
        if found_challenge == 0
            && (*path_x).nat_peer_addr.ss_family as ::core::ffi::c_int != AF_UNSPEC
        {
            let mut ichal_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while ichal_0 < PICOQUIC_CHALLENGE_REPEAT_MAX {
                if response == (*path_x).nat_challenge[ichal_0 as usize] {
                    found_nat_challenge = 1 as ::core::ffi::c_int;
                    break;
                } else {
                    ichal_0 += 1;
                }
            }
        }
        if found_nat_challenge != 0 && (*path_x).challenge_verified() == 0 {
            picoquic_store_addr(
                &raw mut (*path_x).local_addr,
                &raw mut (*path_x).nat_local_addr as *mut sockaddr,
            );
            picoquic_update_peer_addr(path_x, &raw mut (*path_x).nat_peer_addr as *mut sockaddr);
            (*path_x).if_index_dest = (*path_x).if_index_nat_dest;
            if !(*path_x).p_remote_nat_cnxid.is_null() {
                picoquic_dereference_stashed_cnxid(cnx, path_x, 0 as ::core::ffi::c_int);
                (*path_x).p_remote_cnxid = (*path_x).p_remote_nat_cnxid;
                (*path_x).p_remote_nat_cnxid = ::core::ptr::null_mut::<picoquic_remote_cnxid_t>();
            }
            found_challenge = 1 as ::core::ffi::c_int;
        }
        if found_challenge != 0 && (*path_x).challenge_verified() == 0 {
            (*path_x).set_challenge_verified(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            picoquic_update_path_rtt(
                cnx,
                path_x,
                path_x,
                -(1 as ::core::ffi::c_int),
                (*path_x).challenge_time_first,
                current_time,
                0 as uint64_t,
                0 as uint64_t,
            );
            if (*cnx).are_path_callbacks_enabled() as ::core::ffi::c_int != 0
                && (*cnx).callback_fn.expect("non-null function pointer")(
                    cnx as *mut picoquic_cnx_t,
                    (*path_x).unique_path_id,
                    ::core::ptr::null_mut::<uint8_t>(),
                    0 as size_t,
                    picoquic_callback_path_available,
                    (*cnx).callback_ctx,
                    (*path_x).app_path_ctx,
                ) != 0 as ::core::ffi::c_int
            {
                picoquic_connection_error_ex(
                    cnx,
                    PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t,
                    picoquic_frame_type_path_response as ::core::ffi::c_int as uint64_t,
                    b"path available callback\0".as_ptr() as *const ::core::ffi::c_char,
                );
                bytes = ::core::ptr::null::<uint8_t>();
            }
            (*path_x).nat_peer_addr.ss_family = AF_UNSPEC as sa_family_t;
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_should_repeat_path_response_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
) -> ::core::ffi::c_int {
    let mut should_repeat: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut response: uint64_t = 0;
    if !picoquic_frames_uint64_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes.offset(bytes_max as isize),
        &raw mut response,
    )
    .is_null()
    {
        let mut path_index: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*cnx).nb_paths {
            if (**(*cnx).path.offset(i as isize)).challenge_response == response {
                path_index = i;
                break;
            } else {
                i += 1;
            }
        }
        if path_index >= 0 as ::core::ffi::c_int
            && ((**(*cnx).path.offset(path_index as isize)).challenge_verified()
                as ::core::ffi::c_int
                != 0
                || (*cnx).client_mode() as ::core::ffi::c_int != 0
                    && (**(*cnx).path.offset(path_index as isize)).challenge_failed() == 0)
        {
            should_repeat = 1 as ::core::ffi::c_int;
        } else {
            should_repeat = 0 as ::core::ffi::c_int;
        }
    }
    return should_repeat;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_blocked_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes.offset(1 as ::core::ffi::c_int as isize), bytes_max);
    if bytes.is_null() {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_data_blocked as ::core::ffi::c_int as uint64_t,
        );
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_stream_blocked_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes.offset(1 as ::core::ffi::c_int as isize), bytes_max);
    if bytes.is_null() || {
        bytes = picoquic_frames_varint_skip(bytes, bytes_max);
        bytes.is_null()
    } {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_stream_data_blocked as ::core::ffi::c_int as uint64_t,
        );
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_streams_blocked_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut frame_id: uint8_t,
) -> *const uint8_t {
    let mut stream_limit: uint64_t = 0 as uint64_t;
    bytes = picoquic_frames_varint_decode(
        bytes.offset(1 as ::core::ffi::c_int as isize),
        bytes_max,
        &raw mut stream_limit,
    );
    if bytes.is_null() {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            frame_id as uint64_t,
        );
    } else {
        let mut max_stream_id: uint64_t = if frame_id as ::core::ffi::c_int
            == picoquic_frame_type_streams_blocked_unidir as ::core::ffi::c_int
        {
            (*cnx).max_stream_id_unidir_local
        } else {
            (*cnx).max_stream_id_bidir_local
        };
        let mut local_limit: uint64_t =
            max_stream_id.wrapping_add(4 as uint64_t) >> 2 as ::core::ffi::c_int;
        if stream_limit > local_limit {
            picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_STREAM_LIMIT_ERROR as uint64_t,
                frame_id as uint64_t,
            );
        }
    }
    return bytes;
}
unsafe extern "C" fn picoquic_skip_0len_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut frame: uint8_t = *bytes.offset(0 as ::core::ffi::c_int as isize);
    loop {
        bytes = bytes.offset(1);
        if !(bytes < bytes_max && *bytes as ::core::ffi::c_int == frame as ::core::ffi::c_int) {
            break;
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_handshake_done_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut current_time: uint64_t,
) -> *const uint8_t {
    if (*cnx).client_mode() == 0 {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
            *bytes.offset(0 as ::core::ffi::c_int as isize) as uint64_t,
        );
        bytes = ::core::ptr::null::<uint8_t>();
    } else {
        bytes = bytes.offset(1);
        if (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_client_ready_start as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            picoquic_ready_state_transition(cnx, current_time);
        } else if ((*cnx).cnx_state as ::core::ffi::c_uint)
            < picoquic_state_client_ready_start as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                picoquic_frame_type_handshake_done as ::core::ffi::c_int as uint64_t,
            );
            bytes = ::core::ptr::null::<uint8_t>();
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_handshake_done_frame(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut frame_buffer: uint8_t =
        picoquic_frame_type_handshake_done as ::core::ffi::c_int as uint8_t;
    return picoquic_queue_misc_or_dg_frame(
        cnx,
        &raw mut (*cnx).first_datagram,
        &raw mut (*cnx).last_datagram,
        &raw mut frame_buffer,
        1 as size_t,
        0 as ::core::ffi::c_int,
        picoquic_packet_context_application,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_skip_datagram_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let c2rust_fresh5 = bytes;
    bytes = bytes.offset(1);
    let mut frame_id: uint8_t = *c2rust_fresh5;
    let mut has_length: ::core::ffi::c_uint =
        (frame_id as ::core::ffi::c_int & 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    let mut length: uint64_t = 0 as uint64_t;
    if !bytes.is_null() {
        if has_length != 0 {
            bytes = picoquic_frames_varint_decode(bytes, bytes_max, &raw mut length);
        } else {
            length = bytes_max.offset_from(bytes) as ::core::ffi::c_long as uint64_t;
        }
        if !bytes.is_null() {
            bytes = bytes.offset(length as isize);
            if bytes > bytes_max {
                bytes = ::core::ptr::null::<uint8_t>();
            }
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_datagram_frame_header(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut frame_id: *mut uint8_t,
    mut length: *mut uint64_t,
) -> *mut uint8_t {
    if !bytes.is_null() {
        let c2rust_fresh6 = bytes;
        bytes = bytes.offset(1);
        *frame_id = *c2rust_fresh6;
        if *frame_id as ::core::ffi::c_int & 1 as ::core::ffi::c_int != 0 {
            bytes = picoquic_frames_varint_decode(bytes, bytes_max, length) as *mut uint8_t;
            if !bytes.is_null() && bytes.offset(*length as isize) > bytes_max as *mut uint8_t {
                bytes = ::core::ptr::null_mut::<uint8_t>();
            }
        } else {
            *length = bytes_max.offset_from(bytes) as ::core::ffi::c_long as uint64_t;
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_datagram_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let c2rust_fresh12 = bytes;
    bytes = bytes.offset(1);
    let mut frame_id: uint8_t = *c2rust_fresh12;
    let mut has_length: ::core::ffi::c_uint =
        (frame_id as ::core::ffi::c_int & 1 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    let mut length: uint64_t = 0 as uint64_t;
    if !bytes.is_null() {
        if has_length != 0 {
            bytes = picoquic_frames_varint_decode(bytes, bytes_max, &raw mut length);
            if bytes.is_null()
                || bytes.offset(length as isize) > bytes_max
                || length > (*cnx).local_parameters.max_datagram_frame_size as uint64_t
            {
                picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
                    frame_id as uint64_t,
                );
                bytes = ::core::ptr::null::<uint8_t>();
            }
        } else {
            length = bytes_max.offset_from(bytes) as ::core::ffi::c_long as uint64_t;
            if length > (*cnx).local_parameters.max_datagram_frame_size as uint64_t {
                picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
                    frame_id as uint64_t,
                );
                bytes = ::core::ptr::null::<uint8_t>();
            }
        }
    }
    if !bytes.is_null() && (*cnx).callback_fn.is_some() {
        if (*cnx).callback_fn.expect("non-null function pointer")(
            cnx as *mut picoquic_cnx_t,
            (if (*cnx).are_path_callbacks_enabled() as ::core::ffi::c_int != 0 {
                (*path_x).unique_path_id
            } else {
                0 as uint64_t
            }),
            bytes as *mut uint8_t,
            length as size_t,
            picoquic_callback_datagram,
            (*cnx).callback_ctx,
            NULL,
        ) != 0 as ::core::ffi::c_int
        {
            picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t,
                picoquic_frame_type_datagram as ::core::ffi::c_int as uint64_t,
            );
            bytes = ::core::ptr::null::<uint8_t>();
        }
    }
    if !bytes.is_null() {
        bytes = bytes.offset(length as isize);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_datagram_frame(
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut length: size_t,
    mut src: *const uint8_t,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_uint8_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_datagram_l as ::core::ffi::c_int as uint8_t,
    );
    if !bytes.is_null()
        && {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, length as uint64_t);
            !bytes.is_null()
        }
        && bytes.offset(length as isize) <= bytes_max
    {
        memcpy(
            bytes as *mut ::core::ffi::c_void,
            src as *const ::core::ffi::c_void,
            length,
        );
        bytes = bytes.offset(length as isize);
        *is_pure_ack = 0 as ::core::ffi::c_int;
    } else {
        *more_data = 1 as ::core::ffi::c_int;
        bytes = bytes0;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_datagram_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut length: size_t,
    mut src: *const uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if length > PICOQUIC_DATAGRAM_QUEUE_MAX_LENGTH as size_t {
        ret = PICOQUIC_ERROR_DATAGRAM_TOO_LONG;
    } else {
        let mut consumed: size_t = 0 as size_t;
        let mut frame_buffer: [uint8_t; 1536] = [0; 1536];
        let mut more_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut is_pure_ack: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut bytes_next: *mut uint8_t = picoquic_format_datagram_frame(
            &raw mut frame_buffer as *mut uint8_t,
            (&raw mut frame_buffer as *mut uint8_t)
                .offset(::core::mem::size_of::<[uint8_t; 1536]>() as usize as isize),
            &raw mut more_data,
            &raw mut is_pure_ack,
            length,
            src,
        );
        consumed = bytes_next.offset_from(&raw mut frame_buffer as *mut uint8_t)
            as ::core::ffi::c_long as size_t;
        if consumed > 0 as size_t {
            ret = picoquic_queue_misc_or_dg_frame(
                cnx,
                &raw mut (*cnx).first_datagram,
                &raw mut (*cnx).last_datagram,
                &raw mut frame_buffer as *mut uint8_t,
                consumed,
                0 as ::core::ffi::c_int,
                picoquic_packet_context_application,
            );
        } else {
            ret = PICOQUIC_ERROR_FRAME_BUFFER_TOO_SMALL;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_first_datagram_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    if bytes.offset((*(*cnx).first_datagram).length as isize) > bytes_max {
        *more_data = 1 as ::core::ffi::c_int;
    } else {
        bytes = picoquic_format_first_misc_or_dg_frame(
            bytes,
            bytes_max,
            more_data,
            is_pure_ack,
            (*cnx).first_datagram,
            &raw mut (*cnx).first_datagram,
            &raw mut (*cnx).last_datagram,
        );
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_provide_datagram_buffer_ex(
    mut context: *mut ::core::ffi::c_void,
    mut length: size_t,
    mut is_active: picoquic_datagram_active_enum,
) -> *mut uint8_t {
    let mut data_ctx: *mut picoquic_datagram_buffer_argument_t =
        context as *mut picoquic_datagram_buffer_argument_t;
    let mut buffer: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    (*data_ctx).is_active = is_active as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
    (*data_ctx).was_called = 1 as ::core::ffi::c_int;
    if (*data_ctx).is_old_api == 0 {
        (*(*data_ctx).cnx)
            .set_is_datagram_ready(is_active as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if !(*data_ctx).path_x.is_null() {
            (*(*data_ctx).path_x).set_is_datagram_ready(
                (is_active as ::core::ffi::c_int >> 1 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    as ::core::ffi::c_uint,
            );
        }
    }
    if length > 0 as size_t && length <= (*data_ctx).allowed_space {
        let mut after_length: *mut uint8_t = picoquic_frames_varint_encode(
            (*data_ctx).bytes,
            (*data_ctx).bytes_max,
            length as uint64_t,
        );
        if after_length.is_null() || after_length.offset(length as isize) > (*data_ctx).bytes_max {
            let mut bytes: *mut uint8_t = picoquic_frames_varint_encode(
                (*data_ctx).bytes0,
                (*data_ctx).bytes_max,
                picoquic_frame_type_datagram as ::core::ffi::c_int as uint64_t,
            );
            let mut tail: *mut uint8_t = bytes.offset(length as isize);
            if tail < (*data_ctx).bytes_max {
                let mut delta: size_t =
                    (*data_ctx).bytes_max.offset_from(tail) as ::core::ffi::c_long as size_t;
                memset(
                    (*data_ctx).bytes0 as *mut ::core::ffi::c_void,
                    picoquic_frame_type_padding as ::core::ffi::c_int,
                    delta,
                );
                bytes = picoquic_frames_varint_encode(
                    (*data_ctx).bytes0.offset(delta as isize),
                    (*data_ctx).bytes_max,
                    picoquic_frame_type_datagram as ::core::ffi::c_int as uint64_t,
                );
            }
            (*data_ctx).after_data = bytes.offset(length as isize);
            buffer = bytes;
        } else {
            buffer = after_length;
            (*data_ctx).after_data = after_length.offset(length as isize);
        }
    }
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_provide_datagram_buffer(
    mut context: *mut ::core::ffi::c_void,
    mut length: size_t,
) -> *mut uint8_t {
    return picoquic_provide_datagram_buffer_ex(context, length, picoquic_datagram_not_active);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_ready_datagram_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut ret: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_varint_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_datagram_l as ::core::ffi::c_int as uint64_t,
    );
    if bytes.is_null() || bytes.offset(16 as ::core::ffi::c_int as isize) > bytes_max {
        bytes = bytes0;
        *more_data = 1 as ::core::ffi::c_int;
    } else {
        let mut allowed_space: size_t =
            bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t;
        let mut datagram_data_context: picoquic_datagram_buffer_argument_t =
            st_picoquic_datagram_buffer_argument_t {
                cnx: ::core::ptr::null_mut::<picoquic_cnx_t>(),
                path_x: ::core::ptr::null_mut::<picoquic_path_t>(),
                bytes0: ::core::ptr::null_mut::<uint8_t>(),
                bytes: ::core::ptr::null_mut::<uint8_t>(),
                bytes_max: ::core::ptr::null_mut::<uint8_t>(),
                after_data: ::core::ptr::null_mut::<uint8_t>(),
                allowed_space: 0,
                is_active: 0,
                is_old_api: 0,
                was_called: 0,
            };
        if allowed_space > (*cnx).remote_parameters.max_datagram_frame_size as size_t {
            allowed_space = (*cnx).remote_parameters.max_datagram_frame_size as size_t;
        }
        datagram_data_context.cnx = cnx;
        datagram_data_context.path_x = path_x;
        datagram_data_context.bytes0 = bytes0;
        datagram_data_context.bytes = bytes;
        datagram_data_context.bytes_max = bytes_max;
        datagram_data_context.allowed_space = allowed_space;
        datagram_data_context.after_data = bytes0;
        datagram_data_context.is_active = 0 as ::core::ffi::c_int;
        datagram_data_context.is_old_api = 0 as ::core::ffi::c_int;
        datagram_data_context.was_called = 0 as ::core::ffi::c_int;
        if (*cnx).callback_fn.expect("non-null function pointer")(
            cnx as *mut picoquic_cnx_t,
            (if (*cnx).are_path_callbacks_enabled() as ::core::ffi::c_int != 0 {
                (*path_x).unique_path_id
            } else {
                0 as uint64_t
            }),
            &raw mut datagram_data_context as *mut uint8_t,
            allowed_space,
            picoquic_callback_prepare_datagram,
            (*cnx).callback_ctx,
            NULL,
        ) != 0 as ::core::ffi::c_int
        {
            picoquic_log_app_message(
                cnx as *mut picoquic_cnx_t,
                b"Prepare datagram returns error 0x%x\0".as_ptr() as *const ::core::ffi::c_char,
                PICOQUIC_TRANSPORT_INTERNAL_ERROR,
            );
            *ret = picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t,
                0 as uint64_t,
            );
            bytes = bytes0;
        } else {
            bytes = datagram_data_context.after_data;
            if bytes > bytes0 {
                *is_pure_ack = 0 as ::core::ffi::c_int;
            }
            if datagram_data_context.is_old_api != 0 || datagram_data_context.was_called == 0 {
                *more_data |= (*cnx).is_datagram_ready() as ::core::ffi::c_int;
            } else {
                *more_data |= datagram_data_context.is_active;
            }
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_skip_ack_frequency_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    if !bytes.is_null()
        && {
            bytes = picoquic_frames_varint_skip(bytes, bytes_max);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_varint_skip(bytes, bytes_max);
            !bytes.is_null()
        }
    {
        bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_ack_frequency_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut seq: *mut uint64_t,
    mut packets: *mut uint64_t,
    mut microsec: *mut uint64_t,
    mut ignore_order: *mut uint8_t,
    mut reordering_threshold: *mut uint64_t,
) -> *const uint8_t {
    *reordering_threshold = 0 as uint64_t;
    bytes = picoquic_frames_varint_decode(bytes, bytes_max, seq);
    if !bytes.is_null()
        && {
            bytes = picoquic_frames_varint_decode(bytes, bytes_max, packets);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_varint_decode(bytes, bytes_max, microsec);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_varint_decode(bytes, bytes_max, reordering_threshold);
            !bytes.is_null()
        }
    {
        *ignore_order = (*reordering_threshold == 0 as uint64_t) as ::core::ffi::c_int as uint8_t;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_ack_frequency_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut cnx: *mut picoquic_cnx_t,
) -> *const uint8_t {
    let mut seq: uint64_t = 0 as uint64_t;
    let mut packets: uint64_t = 0 as uint64_t;
    let mut microsec: uint64_t = 0 as uint64_t;
    let mut ignore_order: uint8_t = 0 as uint8_t;
    let mut reordering_threshold: uint64_t = 0 as uint64_t;
    bytes = picoquic_parse_ack_frequency_frame(
        bytes,
        bytes_max,
        &raw mut seq,
        &raw mut packets,
        &raw mut microsec,
        &raw mut ignore_order,
        &raw mut reordering_threshold,
    );
    if !bytes.is_null() {
        if (*cnx).is_ack_frequency_negotiated() == 0
            || microsec < (*cnx).local_parameters.min_ack_delay
            || packets == 0 as uint64_t
            || ignore_order as ::core::ffi::c_int > 1 as ::core::ffi::c_int
        {
            picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
                picoquic_frame_type_ack_frequency as ::core::ffi::c_int as uint64_t,
            );
            bytes = ::core::ptr::null::<uint8_t>();
        } else {
            let mut delta: int64_t =
                seq.wrapping_sub((*cnx).ack_frequency_sequence_remote) as int64_t;
            if delta > 0 as int64_t {
                (*cnx).ack_frequency_sequence_remote = seq;
                (*cnx).ack_gap_remote = packets;
                (*cnx).ack_delay_remote = microsec;
                (*cnx).set_ack_ignore_order_remote(
                    (if ignore_order as ::core::ffi::c_int != 0 {
                        1 as ::core::ffi::c_int
                    } else {
                        0 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint as ::core::ffi::c_uint,
                );
                (*cnx).ack_reordering_threshold_remote = reordering_threshold;
                if packets > (*cnx).max_ack_gap_remote {
                    (*cnx).max_ack_gap_remote = packets;
                }
                if microsec > (*cnx).max_ack_delay_remote {
                    (*cnx).max_ack_delay_remote = microsec;
                } else if microsec < (*cnx).min_ack_delay_remote {
                    (*cnx).min_ack_delay_remote = microsec;
                }
            }
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_ack_frequency_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    let mut seq: uint64_t = (*cnx)
        .ack_frequency_sequence_local
        .wrapping_add(1 as uint64_t);
    let mut ack_gap: uint64_t = 0;
    let mut ack_delay_max: uint64_t = 0;
    let mut reordering_threshold: uint64_t =
        (if (*cnx).ack_ignore_order_local() as ::core::ffi::c_int != 0 {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        }) as uint64_t;
    picoquic_compute_ack_gap_and_delay(
        cnx,
        (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).rtt_min,
        (*cnx).remote_parameters.min_ack_delay,
        (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).bandwidth_estimate,
        &raw mut ack_gap,
        &raw mut ack_delay_max,
    );
    if ack_gap <= (*cnx).ack_gap_local
        && ack_delay_max
            >= (7 as uint64_t)
                .wrapping_mul((*cnx).ack_frequency_delay_local)
                .wrapping_div(8 as uint64_t)
        && ack_delay_max
            <= (9 as uint64_t)
                .wrapping_mul((*cnx).ack_frequency_delay_local)
                .wrapping_div(8 as uint64_t)
    {
        (*cnx).set_is_ack_frequency_updated(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    } else {
        if ack_gap < (*cnx).ack_gap_local {
            ack_gap = (*cnx).ack_gap_local;
        }
        bytes = picoquic_frames_varint_encode(
            bytes,
            bytes_max,
            picoquic_frame_type_ack_frequency as ::core::ffi::c_int as uint64_t,
        );
        if !bytes.is_null()
            && {
                bytes = picoquic_frames_varint_encode(bytes, bytes_max, seq);
                !bytes.is_null()
            }
            && {
                bytes = picoquic_frames_varint_encode(bytes, bytes_max, ack_gap);
                !bytes.is_null()
            }
            && {
                bytes = picoquic_frames_varint_encode(bytes, bytes_max, ack_delay_max);
                !bytes.is_null()
            }
            && {
                bytes = picoquic_frames_varint_encode(bytes, bytes_max, reordering_threshold);
                !bytes.is_null()
            }
        {
            (*cnx).ack_frequency_sequence_local = seq;
            (*cnx).ack_gap_local = ack_gap;
            (*cnx).ack_frequency_delay_local = ack_delay_max;
            (*cnx).set_is_ack_frequency_updated(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            if ack_gap > (*cnx).max_ack_gap_local {
                (*cnx).max_ack_gap_local = ack_gap;
            }
            if ack_delay_max < (*cnx).min_ack_delay_local {
                (*cnx).min_ack_delay_local = ack_delay_max;
            }
            if ack_delay_max > (*cnx).max_ack_delay_local {
                (*cnx).max_ack_delay_local = ack_delay_max;
            }
        } else {
            bytes = bytes0;
            *more_data = 1 as ::core::ffi::c_int;
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_skip_immediate_ack_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_immediate_ack_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) -> *const uint8_t {
    if !bytes.is_null() && bytes < bytes_max {
        if (*cnx).is_ack_frequency_negotiated() == 0 {
            picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
                picoquic_frame_type_immediate_ack as ::core::ffi::c_int as uint64_t,
            );
            bytes = ::core::ptr::null::<uint8_t>();
        } else {
            (*cnx).set_is_immediate_ack_required(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            picoquic_set_ack_needed(
                cnx,
                current_time,
                picoquic_packet_context_application,
                path_x,
                1 as ::core::ffi::c_int,
            );
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_immediate_ack_frame(
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut bytes_0: *mut uint8_t = bytes;
    bytes = picoquic_frames_varint_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_immediate_ack as ::core::ffi::c_int as uint64_t,
    );
    if bytes.is_null() {
        bytes = bytes_0;
        *more_data = 1 as ::core::ffi::c_int;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_skip_time_stamp_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_time_stamp_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut time_stamp: *mut uint64_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_decode(bytes, bytes_max, time_stamp);
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_time_stamp_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut cnx: *mut picoquic_cnx_t,
    mut packet_data: *mut picoquic_packet_data_t,
) -> *const uint8_t {
    let mut time_stamp: uint64_t = 0 as uint64_t;
    bytes = picoquic_parse_time_stamp_frame(bytes, bytes_max, &raw mut time_stamp);
    if !bytes.is_null() {
        if (*cnx).is_time_stamp_enabled() == 0 {
            picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                picoquic_frame_type_time_stamp as ::core::ffi::c_int as uint64_t,
            );
            bytes = ::core::ptr::null::<uint8_t>();
        } else {
            time_stamp <<= (*cnx).remote_parameters.ack_delay_exponent as ::core::ffi::c_int;
            if time_stamp > (*packet_data).last_time_stamp_received {
                (*packet_data).last_time_stamp_received = time_stamp;
            }
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_time_stamp_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut current_time: uint64_t,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    let mut time_stamp: uint64_t = current_time.wrapping_sub((*cnx).start_time)
        >> (*cnx).local_parameters.ack_delay_exponent as ::core::ffi::c_int;
    bytes = picoquic_frames_varint_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_time_stamp as ::core::ffi::c_int as uint64_t,
    );
    if bytes.is_null() || {
        bytes = picoquic_frames_varint_encode(bytes, bytes_max, time_stamp);
        bytes.is_null()
    } {
        bytes = bytes0;
        *more_data = 1 as ::core::ffi::c_int;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_encode_time_stamp_length(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
) -> size_t {
    let mut time_stamp: uint64_t = current_time.wrapping_sub((*cnx).start_time)
        >> (*cnx).local_parameters.ack_delay_exponent as ::core::ffi::c_int;
    return (2 as size_t).wrapping_add(picoquic_encode_varint_length(time_stamp));
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_skip_path_abandon_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    if !bytes.is_null() {
        bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_path_abandon_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut path_id: *mut uint64_t,
    mut reason: *mut uint64_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_decode(bytes, bytes_max, path_id);
    if !bytes.is_null() {
        bytes = picoquic_frames_varint_decode(bytes, bytes_max, reason);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_path_abandon_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
) -> *const uint8_t {
    let mut unique_path_id: uint64_t = 0;
    let mut reason: uint64_t = 0 as uint64_t;
    if (*cnx).is_multipath_enabled() == 0 {
        picoquic_connection_error_ex(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_path_abandon as ::core::ffi::c_int as uint64_t,
            b"multipath not negotiated\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        bytes = picoquic_parse_path_abandon_frame(
            bytes,
            bytes_max,
            &raw mut unique_path_id,
            &raw mut reason,
        );
        if bytes.is_null() {
            picoquic_connection_error_ex(
                cnx,
                PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
                picoquic_frame_type_path_abandon as ::core::ffi::c_int as uint64_t,
                b"bad abandon frame\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if unique_path_id > (*cnx).max_path_id_local {
            picoquic_connection_error_ex(
                cnx,
                PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                picoquic_frame_type_path_abandon as ::core::ffi::c_int as uint64_t,
                b"Path ID over limit\0".as_ptr() as *const ::core::ffi::c_char,
            );
            bytes = ::core::ptr::null::<uint8_t>();
        } else {
            let mut path_index: ::core::ffi::c_int =
                picoquic_find_path_by_unique_id(cnx, unique_path_id);
            if path_index >= 0 as ::core::ffi::c_int {
                if (**(*cnx).path.offset(path_index as isize)).path_is_demoted() == 0 {
                    let ref mut c2rust_fresh9 = **(*cnx).path.offset(path_index as isize);
                    (*c2rust_fresh9)
                        .set_path_abandon_received(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    picoquic_demote_path(
                        cnx,
                        path_index,
                        current_time,
                        0 as uint64_t,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                    );
                } else if (**(*cnx).path.offset(path_index as isize)).path_abandon_received() == 0 {
                    let ref mut c2rust_fresh10 = **(*cnx).path.offset(path_index as isize);
                    (*c2rust_fresh10)
                        .set_path_abandon_received(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                } else {
                    picoquic_log_app_message(
                        cnx as *mut picoquic_cnx_t,
                        b"Ignore redundant abandon path with ID: %lu\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        unique_path_id,
                    );
                }
            } else {
                let mut local_cnxid_list: *mut picoquic_local_cnxid_list_t =
                    picoquic_find_or_create_local_cnxid_list(
                        cnx,
                        unique_path_id,
                        0 as ::core::ffi::c_int,
                    );
                if local_cnxid_list.is_null() {
                    picoquic_log_app_message(
                        cnx as *mut picoquic_cnx_t,
                        b"Ignore abandon path with deleted ID: %lu\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        unique_path_id,
                    );
                } else {
                    if (*local_cnxid_list).is_demoted() == 0 {
                        if picoquic_demote_local_cnxid_list(
                            cnx,
                            unique_path_id,
                            0 as uint64_t,
                            current_time,
                        ) != 0 as ::core::ffi::c_int
                        {
                            bytes = ::core::ptr::null::<uint8_t>();
                        }
                    }
                    picoquic_delete_local_cnxid_list(cnx, local_cnxid_list);
                }
            }
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_path_abandon_frame(
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut path_id: uint64_t,
    mut reason: uint64_t,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_varint_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_path_abandon as ::core::ffi::c_int as uint64_t,
    );
    if bytes.is_null()
        || {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, path_id);
            bytes.is_null()
        }
        || {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, reason);
            bytes.is_null()
        }
    {
        bytes = bytes0;
        *more_data = 1 as ::core::ffi::c_int;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_path_abandon_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut reason: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut buffer: [uint8_t; 512] = [0; 512];
    let mut end_bytes: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut more_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    end_bytes = picoquic_format_path_abandon_frame(
        &raw mut buffer as *mut uint8_t,
        (&raw mut buffer as *mut uint8_t)
            .offset(::core::mem::size_of::<[uint8_t; 512]>() as usize as isize),
        &raw mut more_data,
        unique_path_id,
        reason,
    );
    if end_bytes.is_null()
        || picoquic_queue_misc_frame(
            cnx as *mut picoquic_cnx_t,
            &raw mut buffer as *mut uint8_t,
            end_bytes.offset_from(&raw mut buffer as *mut uint8_t) as ::core::ffi::c_long as size_t,
            0 as ::core::ffi::c_int,
            picoquic_packet_context_application,
        ) != 0 as ::core::ffi::c_int
    {
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_path_available_or_standby_frame(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut frame_type: uint64_t,
    mut path_id: uint64_t,
    mut sequence: uint64_t,
    mut more_data: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_varint_encode(bytes, bytes_max, frame_type);
    if bytes.is_null()
        || {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, path_id);
            bytes.is_null()
        }
        || {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, sequence);
            bytes.is_null()
        }
    {
        bytes = bytes0;
        *more_data = 1 as ::core::ffi::c_int;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_path_available_or_standby_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut status: picoquic_path_status_enum,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*path_x).p_remote_cnxid.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        let mut frame_buffer: [uint8_t; 256] = [0; 256];
        let mut frame_type: uint64_t = (if status as ::core::ffi::c_uint
            == picoquic_path_status_available as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            picoquic_frame_type_path_available as ::core::ffi::c_int
        } else {
            picoquic_frame_type_path_backup as ::core::ffi::c_int
        }) as uint64_t;
        let c2rust_fresh14 = (*cnx).status_sequence_to_send_next;
        (*cnx).status_sequence_to_send_next = (*cnx).status_sequence_to_send_next.wrapping_add(1);
        let mut sequence: uint64_t = c2rust_fresh14;
        let mut path_id: uint64_t = if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0 {
            (*path_x).unique_path_id
        } else {
            (*(*path_x).p_remote_cnxid).sequence
        };
        let mut is_pure_ack: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut more_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut bytes_next: *mut uint8_t = picoquic_format_path_available_or_standby_frame(
            &raw mut frame_buffer as *mut uint8_t,
            (&raw mut frame_buffer as *mut uint8_t)
                .offset(::core::mem::size_of::<[uint8_t; 256]>() as usize as isize),
            frame_type,
            path_id,
            sequence,
            &raw mut more_data,
        );
        let mut consumed: size_t = bytes_next.offset_from(&raw mut frame_buffer as *mut uint8_t)
            as ::core::ffi::c_long as size_t;
        ret = picoquic_queue_misc_frame(
            cnx as *mut picoquic_cnx_t,
            &raw mut frame_buffer as *mut uint8_t,
            consumed,
            is_pure_ack,
            picoquic_packet_context_application,
        );
        if ret == 0 as ::core::ffi::c_int {
            (*path_x).status_sequence_sent_last = sequence;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_skip_path_available_or_standby_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    if !bytes.is_null() {
        bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_path_available_or_standby_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut path_id: *mut uint64_t,
    mut sequence: *mut uint64_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_decode(bytes, bytes_max, path_id);
    if !bytes.is_null() {
        bytes = picoquic_frames_varint_decode(bytes, bytes_max, sequence);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_path_available_or_standby_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut frame_id64: uint64_t,
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
) -> *const uint8_t {
    let mut path_id: uint64_t = 0;
    let mut sequence: uint64_t = 0;
    if (*cnx).is_multipath_enabled() == 0 {
        picoquic_connection_error_ex(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            frame_id64,
            b"multipath not negotiated\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        bytes = picoquic_parse_path_available_or_standby_frame(
            bytes,
            bytes_max,
            &raw mut path_id,
            &raw mut sequence,
        );
        if bytes.is_null() {
            picoquic_connection_error_ex(
                cnx,
                PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
                frame_id64,
                b"bad status frame\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else {
            let mut path_number: ::core::ffi::c_int = picoquic_find_path_by_unique_id(cnx, path_id);
            if path_number < 0 as ::core::ffi::c_int {
                picoquic_log_app_message(
                    cnx as *mut picoquic_cnx_t,
                    b"Ignore path %s frame with invalid ID: %lu\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    if frame_id64
                        == picoquic_frame_type_path_available as ::core::ffi::c_int as uint64_t
                    {
                        b"available\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"standby\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    path_id,
                );
            } else if !((**(*cnx).path.offset(path_number as isize))
                .status_sequence_to_receive_next
                > sequence)
            {
                (**(*cnx).path.offset(path_number as isize)).status_sequence_to_receive_next =
                    sequence.wrapping_add(1 as uint64_t);
                let ref mut c2rust_fresh8 = **(*cnx).path.offset(path_number as isize);
                (*c2rust_fresh8).set_path_is_standby(
                    (if frame_id64
                        == picoquic_frame_type_path_available as ::core::ffi::c_int as uint64_t
                    {
                        0 as ::core::ffi::c_int
                    } else {
                        1 as ::core::ffi::c_int
                    }) as ::core::ffi::c_uint as ::core::ffi::c_uint,
                );
            }
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_path_available_or_backup_frame_need_repeat(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut no_need_to_repeat: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut path_id: uint64_t = 0 as uint64_t;
    let mut sequence: uint64_t = 0 as uint64_t;
    *no_need_to_repeat = 0 as ::core::ffi::c_int;
    bytes = picoquic_parse_path_available_or_standby_frame(
        bytes,
        bytes_max,
        &raw mut path_id,
        &raw mut sequence,
    );
    if bytes.is_null() {
        *no_need_to_repeat = 1 as ::core::ffi::c_int;
    } else {
        let mut path_number: ::core::ffi::c_int = picoquic_find_path_by_unique_id(cnx, path_id);
        if path_number < 0 as ::core::ffi::c_int
            || (**(*cnx).path.offset(path_number as isize)).status_sequence_sent_last != sequence
            || (**(*cnx).path.offset(path_number as isize)).path_is_demoted() as ::core::ffi::c_int
                != 0
        {
            *no_need_to_repeat = 1 as ::core::ffi::c_int;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_max_path_id_frame(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut max_path_id: uint64_t,
    mut more_data: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_varint_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_max_path_id as ::core::ffi::c_int as uint64_t,
    );
    if bytes.is_null() || {
        bytes = picoquic_frames_varint_encode(bytes, bytes_max, max_path_id);
        bytes.is_null()
    } {
        bytes = bytes0;
        *more_data = 1 as ::core::ffi::c_int;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_max_path_id_frame(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut frame_buffer: [uint8_t; 256] = [0; 256];
    let mut is_pure_ack: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut more_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bytes_next: *mut uint8_t = picoquic_format_max_path_id_frame(
        &raw mut frame_buffer as *mut uint8_t,
        (&raw mut frame_buffer as *mut uint8_t)
            .offset(::core::mem::size_of::<[uint8_t; 256]>() as usize as isize),
        (*cnx).max_path_id_local,
        &raw mut more_data,
    );
    let mut consumed: size_t = bytes_next.offset_from(&raw mut frame_buffer as *mut uint8_t)
        as ::core::ffi::c_long as size_t;
    ret = picoquic_queue_misc_frame(
        cnx as *mut picoquic_cnx_t,
        &raw mut frame_buffer as *mut uint8_t,
        consumed,
        is_pure_ack,
        picoquic_packet_context_application,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_skip_max_path_id_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_max_path_id_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut max_path_id: *mut uint64_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_decode(bytes, bytes_max, max_path_id);
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_max_path_id_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut cnx: *mut picoquic_cnx_t,
) -> *const uint8_t {
    let mut max_path_id: uint64_t = 0;
    if (*cnx).is_multipath_enabled() == 0 {
        picoquic_connection_error_ex(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_max_path_id as ::core::ffi::c_int as uint64_t,
            b"unique path_id not negotiated\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        bytes = picoquic_parse_max_path_id_frame(bytes, bytes_max, &raw mut max_path_id);
        if bytes.is_null() {
            picoquic_connection_error_ex(
                cnx,
                PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
                picoquic_frame_type_max_path_id as ::core::ffi::c_int as uint64_t,
                b"bad max paths frame\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if (*cnx).max_path_id_remote < max_path_id {
            (*cnx).max_path_id_remote = max_path_id;
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_max_path_id_frame_needs_repeat(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut no_need_to_repeat: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max_path_id: uint64_t = 0 as uint64_t;
    *no_need_to_repeat = 0 as ::core::ffi::c_int;
    bytes = picoquic_parse_max_path_id_frame(bytes, bytes_max, &raw mut max_path_id);
    if bytes.is_null() {
        *no_need_to_repeat = 1 as ::core::ffi::c_int;
    } else if max_path_id <= (*cnx).max_path_id_local
        || max_path_id <= (*cnx).max_path_id_acknowledged
    {
        *no_need_to_repeat = 1 as ::core::ffi::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_ack_of_max_path_id_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut consumed: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max_path_id: uint64_t = 0 as uint64_t;
    let mut bytes_next: *const uint8_t = picoquic_parse_max_path_id_frame(
        bytes,
        bytes.offset(bytes_max as isize),
        &raw mut max_path_id,
    );
    if !bytes_next.is_null() {
        if (*cnx).max_path_id_acknowledged < max_path_id {
            (*cnx).max_path_id_acknowledged = max_path_id;
        }
        *consumed = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
    } else {
        *consumed = bytes_max;
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_path_blocked_frame(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut max_path_id: uint64_t,
    mut more_data: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_varint_encode(
        bytes,
        bytes_max,
        picoquic_frame_type_path_blocked as ::core::ffi::c_int as uint64_t,
    );
    if bytes.is_null() || {
        bytes = picoquic_frames_varint_encode(bytes, bytes_max, max_path_id);
        bytes.is_null()
    } {
        bytes = bytes0;
        *more_data = 1 as ::core::ffi::c_int;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_path_blocked_frame(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut frame_buffer: [uint8_t; 256] = [0; 256];
    let mut is_pure_ack: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut more_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bytes_next: *mut uint8_t = picoquic_format_path_blocked_frame(
        &raw mut frame_buffer as *mut uint8_t,
        (&raw mut frame_buffer as *mut uint8_t)
            .offset(::core::mem::size_of::<[uint8_t; 256]>() as usize as isize),
        (*cnx).max_path_id_remote,
        &raw mut more_data,
    );
    let mut consumed: size_t = bytes_next.offset_from(&raw mut frame_buffer as *mut uint8_t)
        as ::core::ffi::c_long as size_t;
    ret = picoquic_queue_misc_frame(
        cnx as *mut picoquic_cnx_t,
        &raw mut frame_buffer as *mut uint8_t,
        consumed,
        is_pure_ack,
        picoquic_packet_context_application,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_skip_path_blocked_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_path_blocked_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut max_path_id: *mut uint64_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_decode(bytes, bytes_max, max_path_id);
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_path_blocked_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut cnx: *mut picoquic_cnx_t,
) -> *const uint8_t {
    let mut max_path_id: uint64_t = 0;
    if (*cnx).is_multipath_enabled() == 0 {
        picoquic_connection_error_ex(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_path_blocked as ::core::ffi::c_int as uint64_t,
            b"multipath extension not negotiated\0".as_ptr() as *const ::core::ffi::c_char,
        );
    } else {
        bytes = picoquic_parse_path_blocked_frame(bytes, bytes_max, &raw mut max_path_id);
        if bytes.is_null() {
            picoquic_connection_error_ex(
                cnx,
                PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
                picoquic_frame_type_path_blocked as ::core::ffi::c_int as uint64_t,
                b"bad path blocked frame\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_path_blocked_frame_needs_repeat(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut no_need_to_repeat: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max_path_id: uint64_t = 0 as uint64_t;
    *no_need_to_repeat = 0 as ::core::ffi::c_int;
    bytes = picoquic_parse_path_blocked_frame(bytes, bytes_max, &raw mut max_path_id);
    if bytes.is_null() {
        *no_need_to_repeat = 1 as ::core::ffi::c_int;
    } else if max_path_id <= (*cnx).max_path_id_remote
        || max_path_id <= (*cnx).path_blocked_acknowledged
    {
        *no_need_to_repeat = 1 as ::core::ffi::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_ack_of_path_blocked_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut consumed: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut max_path_id: uint64_t = 0 as uint64_t;
    let mut bytes_next: *const uint8_t = picoquic_parse_path_blocked_frame(
        bytes,
        bytes.offset(bytes_max as isize),
        &raw mut max_path_id,
    );
    if !bytes_next.is_null() {
        if (*cnx).path_blocked_acknowledged < max_path_id {
            (*cnx).path_blocked_acknowledged = max_path_id;
        }
        *consumed = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
    } else {
        *consumed = bytes_max;
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_observed_address_frame(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut ftype: uint64_t,
    mut sequence_number: uint64_t,
    mut addr: *mut uint8_t,
    mut port: uint16_t,
    mut more_data: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut l_addr: size_t = (if ftype & 1 as uint64_t == 0 as uint64_t {
        4 as ::core::ffi::c_int
    } else {
        16 as ::core::ffi::c_int
    }) as size_t;
    let mut bytes0: *mut uint8_t = bytes;
    bytes = picoquic_frames_varint_encode(bytes, bytes_max, ftype);
    if !bytes.is_null()
        && {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, sequence_number);
            !bytes.is_null()
        }
        && bytes.offset(l_addr as isize) < bytes_max as *mut uint8_t
    {
        memcpy(
            bytes as *mut ::core::ffi::c_void,
            addr as *const ::core::ffi::c_void,
            l_addr,
        );
        bytes = picoquic_frames_uint16_encode(bytes.offset(l_addr as isize), bytes_max, port);
    } else {
        bytes = ::core::ptr::null_mut::<uint8_t>();
    }
    if bytes.is_null() {
        *more_data = 1 as ::core::ffi::c_int;
        bytes = bytes0;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_observed_address_frame(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
    mut next_wake_time: *mut uint64_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    if (*path_x).observed_addr_acked() == 0
        && (*path_x).nb_observed_repeat < 4 as ::core::ffi::c_int
        && (*path_x).peer_addr.ss_family as ::core::ffi::c_int != AF_UNSPEC
    {
        let mut is_needed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if (*path_x).nb_observed_repeat == 0 as ::core::ffi::c_int {
            is_needed = 1 as ::core::ffi::c_int;
            let c2rust_fresh13 = (*(*path_x).cnx).observed_number;
            (*(*path_x).cnx).observed_number = (*(*path_x).cnx).observed_number.wrapping_add(1);
            (*path_x).observed_sequence_sent = c2rust_fresh13;
        } else {
            let mut repeat_time: uint64_t = (*path_x)
                .observed_time
                .wrapping_add((*path_x).retransmit_timer);
            if repeat_time <= current_time {
                is_needed = 1 as ::core::ffi::c_int;
            } else if *next_wake_time > repeat_time {
                *next_wake_time = repeat_time;
            }
        }
        if is_needed != 0 {
            let mut ftype: uint64_t = 0 as uint64_t;
            let mut ip_addr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
            let mut port: uint16_t = 0 as uint16_t;
            if (*path_x).peer_addr.ss_family as ::core::ffi::c_int == AF_INET6 {
                let mut addr: *mut sockaddr_in6 = &raw mut (*path_x).peer_addr as *mut sockaddr_in6;
                ftype = picoquic_frame_type_observed_address_v6 as ::core::ffi::c_int as uint64_t;
                ip_addr = &raw mut (*addr).sin6_addr as *mut uint8_t;
                port = (*addr).sin6_port as uint16_t;
            } else {
                let mut addr_0: *mut sockaddr_in = &raw mut (*path_x).peer_addr as *mut sockaddr_in;
                ftype = picoquic_frame_type_observed_address_v4 as ::core::ffi::c_int as uint64_t;
                ip_addr = &raw mut (*addr_0).sin_addr as *mut uint8_t;
                port = (*addr_0).sin_port as uint16_t;
            }
            let mut bytes_next: *mut uint8_t = picoquic_format_observed_address_frame(
                bytes,
                bytes_max,
                ftype,
                (*path_x).observed_sequence_sent,
                ip_addr,
                port,
                more_data,
            );
            if bytes_next > bytes {
                *is_pure_ack = 0 as ::core::ffi::c_int;
                bytes = bytes_next;
                (*path_x).nb_observed_repeat += 1 as ::core::ffi::c_int;
                (*path_x).observed_time = current_time;
            }
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_skip_observed_address_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut ftype: uint64_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    if !bytes.is_null() {
        let mut l_addr: size_t = (if ftype & 1 as uint64_t == 0 as uint64_t {
            4 as ::core::ffi::c_int
        } else {
            16 as ::core::ffi::c_int
        }) as size_t;
        let mut l_frame: size_t = l_addr.wrapping_add(2 as size_t);
        bytes = picoquic_frames_fixed_skip(bytes, bytes_max, l_frame as uint64_t);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_observed_address_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut ftype: uint64_t,
    mut sequence: *mut uint64_t,
    mut addr: *mut *const uint8_t,
    mut port: *mut uint16_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_decode(bytes, bytes_max, sequence);
    if !bytes.is_null() {
        let mut l_addr: size_t = (if ftype & 1 as uint64_t == 0 as uint64_t {
            4 as ::core::ffi::c_int
        } else {
            16 as ::core::ffi::c_int
        }) as size_t;
        *addr = bytes;
        bytes = picoquic_frames_fixed_skip(bytes, bytes_max, l_addr as uint64_t);
        if !bytes.is_null() {
            bytes = picoquic_frames_uint16_decode(bytes, bytes_max, port);
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_observed_address_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut path_x: *mut picoquic_path_t,
    mut ftype: uint64_t,
) -> *const uint8_t {
    let mut addr: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut port: uint16_t = 0 as uint16_t;
    let mut sequence: uint64_t = 0 as uint64_t;
    if (*cnx).is_address_discovery_receiver() == 0 {
        picoquic_connection_error_ex(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            ftype,
            b"address discovery not negotiated as receiver\0".as_ptr()
                as *const ::core::ffi::c_char,
        );
    } else {
        bytes = picoquic_parse_observed_address_frame(
            bytes,
            bytes_max,
            ftype,
            &raw mut sequence,
            &raw mut addr,
            &raw mut port,
        );
        if bytes.is_null() {
            picoquic_connection_error_ex(
                cnx,
                PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
                ftype,
                b"bad observed address frame\0".as_ptr() as *const ::core::ffi::c_char,
            );
        } else if sequence > (*path_x).observed_address_received
            || (*path_x).observed_address_received == 0 as uint64_t
                && (*path_x).observed_addr.ss_family as ::core::ffi::c_int == AF_UNSPEC
        {
            (*path_x).observed_address_received = sequence;
            if ftype & 1 as uint64_t == 0 as uint64_t {
                let mut o_addr: *mut sockaddr_in =
                    &raw mut (*path_x).observed_addr as *mut sockaddr_in;
                memset(
                    o_addr as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<sockaddr_in>() as size_t,
                );
                (*o_addr).sin_family = AF_INET as sa_family_t;
                memcpy(
                    &raw mut (*o_addr).sin_addr as *mut ::core::ffi::c_void,
                    addr as *const ::core::ffi::c_void,
                    4 as size_t,
                );
                (*o_addr).sin_port = port as in_port_t;
            } else {
                let mut o_addr_0: *mut sockaddr_in6 =
                    &raw mut (*path_x).observed_addr as *mut sockaddr_in6;
                memset(
                    o_addr_0 as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<sockaddr_in6>() as size_t,
                );
                (*o_addr_0).sin6_family = AF_INET6 as sa_family_t;
                memcpy(
                    &raw mut (*o_addr_0).sin6_addr as *mut ::core::ffi::c_void,
                    addr as *const ::core::ffi::c_void,
                    16 as size_t,
                );
                (*o_addr_0).sin6_port = port as in_port_t;
            }
            if (*cnx).callback_fn.is_some() {
                (*cnx).callback_fn.expect("non-null function pointer")(
                    cnx as *mut picoquic_cnx_t,
                    (*path_x).unique_path_id,
                    ::core::ptr::null_mut::<uint8_t>(),
                    0 as size_t,
                    picoquic_callback_path_address_observed,
                    (*cnx).callback_ctx,
                    (*path_x).app_path_ctx,
                );
            }
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_ack_of_observed_address_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut ftype: uint64_t,
    mut consumed: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bytes_next: *const uint8_t =
        picoquic_skip_observed_address_frame(bytes, bytes.offset(bytes_max as isize), ftype);
    if bytes_next.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        (*path_x).set_observed_addr_acked(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        *consumed = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_skip_bdp_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    if !bytes.is_null()
        && {
            bytes = picoquic_frames_varint_skip(bytes, bytes_max);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_varint_skip(bytes, bytes_max);
            !bytes.is_null()
        }
    {
        bytes = picoquic_frames_length_data_skip(bytes, bytes_max);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_bdp_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut lifetime: *mut uint64_t,
    mut recon_bytes_in_flight: *mut uint64_t,
    mut recon_min_rtt: *mut uint64_t,
    mut saved_ip_length: *mut uint64_t,
    mut saved_ip: *mut *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_decode(bytes, bytes_max, lifetime);
    if !bytes.is_null()
        && {
            bytes = picoquic_frames_varint_decode(bytes, bytes_max, recon_bytes_in_flight);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_varint_decode(bytes, bytes_max, recon_min_rtt);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_varint_decode(bytes, bytes_max, saved_ip_length);
            !bytes.is_null()
        }
    {
        if *saved_ip_length != 4 as uint64_t && *saved_ip_length != 16 as uint64_t {
            bytes = ::core::ptr::null::<uint8_t>();
        } else {
            *saved_ip = bytes;
            bytes = picoquic_frames_fixed_skip(bytes, bytes_max, *saved_ip_length);
        }
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_bdp_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut current_time: uint64_t,
    mut addr_from: *mut sockaddr,
    mut path_x: *mut picoquic_path_t,
) -> *const uint8_t {
    let mut lifetime: uint64_t = 0;
    let mut recon_bytes_in_flight: uint64_t = 0;
    let mut recon_min_rtt: uint64_t = 0;
    let mut saved_ip_length: uint64_t = 0;
    let mut saved_ip: *const uint8_t = ::core::ptr::null::<uint8_t>();
    bytes = picoquic_parse_bdp_frame(
        cnx,
        bytes,
        bytes_max,
        &raw mut lifetime,
        &raw mut recon_bytes_in_flight,
        &raw mut recon_min_rtt,
        &raw mut saved_ip_length,
        &raw mut saved_ip,
    );
    if !bytes.is_null() {
        if (*cnx).send_receive_bdp_frame() != 0 {
            if (*cnx).client_mode() != 0 {
                (*path_x).cwin_remote = recon_bytes_in_flight;
                (*path_x).rtt_min_remote = recon_min_rtt;
                (*path_x).ip_client_remote_length = saved_ip_length as uint8_t;
                memcpy(
                    &raw mut (*path_x).ip_client_remote as *mut uint8_t as *mut ::core::ffi::c_void,
                    saved_ip as *const ::core::ffi::c_void,
                    (*path_x).ip_client_remote_length as size_t,
                );
                let mut is_ticket_seed: ::core::ffi::c_int =
                    (*path_x).is_ticket_seeded() as ::core::ffi::c_int;
                picoquic_seed_ticket(cnx as *mut picoquic_cnx_t, path_x as *mut picoquic_path_t);
                (*path_x).set_is_ticket_seeded(
                    is_ticket_seed as ::core::ffi::c_uint as ::core::ffi::c_uint,
                );
            } else if lifetime > current_time {
                let mut client_ip: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
                let mut client_ip_length: uint8_t = 0;
                picoquic_get_ip_addr(
                    &raw mut (*path_x).peer_addr as *mut sockaddr,
                    &raw mut client_ip,
                    &raw mut client_ip_length,
                );
                if saved_ip_length > 0 as uint64_t
                    && client_ip_length as uint64_t == saved_ip_length
                    && memcmp(
                        client_ip as *const ::core::ffi::c_void,
                        saved_ip as *const ::core::ffi::c_void,
                        client_ip_length as size_t,
                    ) == 0 as ::core::ffi::c_int
                {
                    picoquic_seed_bandwidth(
                        cnx,
                        recon_min_rtt,
                        recon_bytes_in_flight,
                        saved_ip,
                        saved_ip_length as uint8_t,
                    );
                }
            }
        }
    } else {
        picoquic_connection_error(
            cnx,
            PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
            picoquic_frame_type_bdp as ::core::ffi::c_int as uint64_t,
        );
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_bdp_frame(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut path_x: *mut picoquic_path_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes;
    let mut lifetime: uint64_t = ((24 as ::core::ffi::c_int * 3600 as ::core::ffi::c_int)
        as uint64_t)
        .wrapping_mul(1000000 as ::core::ffi::c_int as uint64_t);
    let mut recon_bytes_in_flight: uint64_t = 0 as uint64_t;
    let mut recon_min_rtt: uint64_t = 0 as uint64_t;
    let mut ip_addr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut ip_addr_length: uint8_t = 0 as uint8_t;
    if (*cnx).client_mode() == 0 {
        if (*path_x).is_ticket_seeded() as ::core::ffi::c_int != 0 && (*path_x).is_bdp_sent() == 0 {
            let mut server_ticket: *mut picoquic_issued_ticket_t =
                ::core::ptr::null_mut::<picoquic_issued_ticket_t>();
            server_ticket = picoquic_retrieve_issued_ticket(
                (*cnx).quic as *mut picoquic_quic_t,
                (*cnx).issued_ticket_id,
            );
            if !server_ticket.is_null() && (*server_ticket).cwin > 0 as uint64_t {
                recon_bytes_in_flight = (*server_ticket).cwin;
                recon_min_rtt = (*server_ticket).rtt;
                ip_addr = &raw mut (*server_ticket).ip_addr as *mut uint8_t;
                ip_addr_length = (*server_ticket).ip_addr_length;
            }
        }
    } else {
        let mut stored_ticket: *mut picoquic_stored_ticket_t = picoquic_get_stored_ticket(
            (*cnx).quic as *mut picoquic_quic_t,
            (*cnx).sni,
            strlen((*cnx).sni) as uint16_t,
            (*cnx).alpn,
            strlen((*cnx).alpn) as uint16_t,
            (*(&raw const picoquic_supported_versions as *const picoquic_version_parameters_t)
                .offset((*cnx).version_index as isize))
            .version,
            1 as ::core::ffi::c_int,
            0 as uint64_t,
        );
        if !stored_ticket.is_null() {
            recon_bytes_in_flight = (*stored_ticket).tp_0rtt
                [picoquic_tp_0rtt_cwin_remote as ::core::ffi::c_int as usize];
            recon_min_rtt = (*stored_ticket).tp_0rtt
                [picoquic_tp_0rtt_rtt_remote as ::core::ffi::c_int as usize];
            ip_addr = (*stored_ticket).ip_addr_client;
            ip_addr_length = (*stored_ticket).ip_addr_client_length;
        }
    }
    if recon_bytes_in_flight == 0 as uint64_t
        || {
            bytes = picoquic_frames_varint_encode(
                bytes,
                bytes_max,
                picoquic_frame_type_bdp as ::core::ffi::c_int as uint64_t,
            );
            bytes.is_null()
        }
        || {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, lifetime);
            bytes.is_null()
        }
        || {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, recon_bytes_in_flight);
            bytes.is_null()
        }
        || {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, recon_min_rtt);
            bytes.is_null()
        }
        || {
            bytes = picoquic_frames_length_data_encode(
                bytes,
                bytes_max,
                ip_addr_length as size_t,
                ip_addr,
            );
            bytes.is_null()
        }
    {
        bytes = bytes0;
    } else {
        *is_pure_ack = 0 as ::core::ffi::c_int;
        (*path_x).set_is_bdp_sent(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_frames(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut bytes: *const uint8_t,
    mut bytes_maxsize: size_t,
    mut received_data: *mut picoquic_stream_data_node_t,
    mut epoch: ::core::ffi::c_int,
    mut addr_from: *mut sockaddr,
    mut addr_to: *mut sockaddr,
    mut pn64: uint64_t,
    mut path_is_not_allocated: ::core::ffi::c_int,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut bytes_max: *const uint8_t = bytes.offset(bytes_maxsize as isize);
    let mut ack_needed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_path_probing_packet: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut pc: picoquic_packet_context_enum = picoquic_context_from_epoch(epoch);
    let mut packet_data: picoquic_packet_data_t = st_picoquic_packet_data_t {
        last_time_stamp_received: 0,
        last_ack_delay: 0,
        nb_path_ack: 0,
        path_ack: [C2Rust_Unnamed_3 {
            acked_path: ::core::ptr::null_mut::<picoquic_path_t>(),
            largest_sent_time: 0,
            delivered_prior: 0,
            delivered_time_prior: 0,
            delivered_sent_prior: 0,
            lost_prior: 0,
            inflight_prior: 0,
            rs_is_path_limited: 0,
            rs_is_cwnd_limited: 0,
            is_set: 0,
            data_acked: 0,
        }; 8],
    };
    memset(
        &raw mut packet_data as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<picoquic_packet_data_t>() as size_t,
    );
    while !bytes.is_null() && bytes < bytes_max {
        let mut first_byte: uint8_t = *bytes.offset(0 as ::core::ffi::c_int as isize);
        let mut is_path_probing_frame: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if first_byte as ::core::ffi::c_int
            & !(picoquic_frame_type_stream_range_min as ::core::ffi::c_int
                ^ picoquic_frame_type_stream_range_max as ::core::ffi::c_int)
            == picoquic_frame_type_stream_range_min as ::core::ffi::c_int
        {
            if epoch != picoquic_epoch_0rtt as ::core::ffi::c_int
                && epoch != picoquic_epoch_1rtt as ::core::ffi::c_int
            {
                picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                    first_byte as uint64_t,
                );
                bytes = ::core::ptr::null::<uint8_t>();
                break;
            } else {
                bytes = picoquic_decode_stream_frame(
                    cnx,
                    bytes,
                    bytes_max,
                    received_data,
                    current_time,
                );
                ack_needed = 1 as ::core::ffi::c_int;
            }
        } else if first_byte as ::core::ffi::c_int == picoquic_frame_type_ack as ::core::ffi::c_int
        {
            if epoch == picoquic_epoch_0rtt as ::core::ffi::c_int {
                picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                    first_byte as uint64_t,
                );
                bytes = ::core::ptr::null::<uint8_t>();
                break;
            } else {
                bytes = picoquic_decode_ack_frame(
                    cnx,
                    bytes,
                    bytes_max,
                    current_time,
                    epoch,
                    0 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    &raw mut packet_data,
                );
            }
        } else if first_byte as ::core::ffi::c_int
            == picoquic_frame_type_ack_ecn as ::core::ffi::c_int
        {
            if epoch == picoquic_epoch_0rtt as ::core::ffi::c_int {
                picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                    first_byte as uint64_t,
                );
                bytes = ::core::ptr::null::<uint8_t>();
                break;
            } else {
                bytes = picoquic_decode_ack_frame(
                    cnx,
                    bytes,
                    bytes_max,
                    current_time,
                    epoch,
                    1 as ::core::ffi::c_int,
                    0 as ::core::ffi::c_int,
                    &raw mut packet_data,
                );
            }
        } else if epoch != picoquic_epoch_0rtt as ::core::ffi::c_int
            && epoch != picoquic_epoch_1rtt as ::core::ffi::c_int
            && first_byte as ::core::ffi::c_int != picoquic_frame_type_padding as ::core::ffi::c_int
            && first_byte as ::core::ffi::c_int != picoquic_frame_type_ping as ::core::ffi::c_int
            && first_byte as ::core::ffi::c_int != picoquic_frame_type_poll as ::core::ffi::c_int
            && first_byte as ::core::ffi::c_int
                != picoquic_frame_type_connection_close as ::core::ffi::c_int
            && first_byte as ::core::ffi::c_int
                != picoquic_frame_type_crypto_hs as ::core::ffi::c_int
        {
            picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                first_byte as uint64_t,
            );
            bytes = ::core::ptr::null::<uint8_t>();
            break;
        } else if epoch == picoquic_epoch_0rtt as ::core::ffi::c_int
            && (first_byte as ::core::ffi::c_int
                == picoquic_frame_type_crypto_hs as ::core::ffi::c_int
                || first_byte as ::core::ffi::c_int
                    == picoquic_frame_type_handshake_done as ::core::ffi::c_int
                || first_byte as ::core::ffi::c_int
                    == picoquic_frame_type_new_token as ::core::ffi::c_int
                || first_byte as ::core::ffi::c_int
                    == picoquic_frame_type_path_response as ::core::ffi::c_int
                || first_byte as ::core::ffi::c_int
                    == picoquic_frame_type_retire_connection_id as ::core::ffi::c_int)
        {
            picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                first_byte as uint64_t,
            );
            bytes = ::core::ptr::null::<uint8_t>();
            break;
        } else {
            match first_byte as ::core::ffi::c_int {
                0 => {
                    is_path_probing_frame = 1 as ::core::ffi::c_int;
                    bytes = picoquic_skip_0len_frame(bytes, bytes_max);
                }
                4 => {
                    bytes = picoquic_decode_stream_reset_frame(cnx, bytes, bytes_max);
                    ack_needed = 1 as ::core::ffi::c_int;
                }
                28 => {
                    bytes = picoquic_decode_connection_close_frame(cnx, bytes, bytes_max);
                    ack_needed = 1 as ::core::ffi::c_int;
                }
                29 => {
                    bytes = picoquic_decode_application_close_frame(cnx, bytes, bytes_max);
                    ack_needed = 1 as ::core::ffi::c_int;
                }
                16 => {
                    bytes = picoquic_decode_max_data_frame(cnx, bytes, bytes_max);
                    ack_needed = 1 as ::core::ffi::c_int;
                }
                17 => {
                    bytes = picoquic_decode_max_stream_data_frame(cnx, bytes, bytes_max);
                    ack_needed = 1 as ::core::ffi::c_int;
                }
                18 | 19 => {
                    bytes = picoquic_decode_max_streams_frame(
                        cnx,
                        bytes,
                        bytes_max,
                        first_byte as ::core::ffi::c_int,
                    );
                    ack_needed = 1 as ::core::ffi::c_int;
                }
                1 => {
                    bytes = picoquic_skip_0len_frame(bytes, bytes_max);
                    ack_needed = 1 as ::core::ffi::c_int;
                }
                32 => {
                    bytes = picoquic_skip_0len_frame(bytes, bytes_max);
                }
                20 => {
                    bytes = picoquic_decode_blocked_frame(cnx, bytes, bytes_max);
                    ack_needed = 1 as ::core::ffi::c_int;
                }
                21 => {
                    bytes = picoquic_decode_stream_blocked_frame(cnx, bytes, bytes_max);
                    ack_needed = 1 as ::core::ffi::c_int;
                }
                23 | 22 => {
                    bytes =
                        picoquic_decode_streams_blocked_frame(cnx, bytes, bytes_max, first_byte);
                    ack_needed = 1 as ::core::ffi::c_int;
                }
                24 => {
                    is_path_probing_frame = 1 as ::core::ffi::c_int;
                    bytes = picoquic_decode_new_connection_id_frame(
                        cnx,
                        bytes,
                        bytes_max,
                        current_time,
                        0 as ::core::ffi::c_int,
                    );
                    ack_needed = 1 as ::core::ffi::c_int;
                }
                5 => {
                    bytes = picoquic_decode_stop_sending_frame(cnx, bytes, bytes_max);
                    ack_needed = 1 as ::core::ffi::c_int;
                }
                26 => {
                    is_path_probing_frame = 1 as ::core::ffi::c_int;
                    bytes = picoquic_decode_path_challenge_frame(
                        cnx,
                        bytes,
                        bytes_max,
                        if path_is_not_allocated != 0 {
                            ::core::ptr::null_mut::<picoquic_path_t>()
                        } else {
                            path_x
                        },
                        addr_from,
                        addr_to,
                    );
                }
                27 => {
                    is_path_probing_frame = 1 as ::core::ffi::c_int;
                    bytes = picoquic_decode_path_response_frame(
                        cnx,
                        bytes,
                        bytes_max,
                        if path_is_not_allocated != 0 {
                            ::core::ptr::null_mut::<picoquic_path_t>()
                        } else {
                            path_x
                        },
                        current_time,
                    );
                }
                6 => {
                    bytes = picoquic_decode_crypto_hs_frame(
                        cnx,
                        bytes,
                        bytes_max,
                        received_data,
                        epoch,
                    );
                    ack_needed = 1 as ::core::ffi::c_int;
                }
                7 => {
                    bytes = picoquic_decode_new_token_frame(
                        cnx,
                        bytes,
                        bytes_max,
                        current_time,
                        addr_to,
                    );
                    ack_needed = 1 as ::core::ffi::c_int;
                }
                25 => {
                    bytes = picoquic_decode_retire_connection_id_frame(
                        cnx,
                        bytes,
                        bytes_max,
                        current_time,
                        path_x,
                        0 as ::core::ffi::c_int,
                    );
                    ack_needed = 1 as ::core::ffi::c_int;
                }
                30 => {
                    bytes = picoquic_decode_handshake_done_frame(cnx, bytes, current_time);
                    ack_needed = 1 as ::core::ffi::c_int;
                }
                48 | 49 => {
                    ack_needed = 1 as ::core::ffi::c_int;
                    bytes = picoquic_decode_datagram_frame(cnx, path_x, bytes, bytes_max);
                }
                _ => {
                    let mut frame_id64: uint64_t = 0;
                    let mut bytes0: *const uint8_t = bytes;
                    bytes = picoquic_frames_varint_decode(bytes, bytes_max, &raw mut frame_id64);
                    if !bytes.is_null() {
                        if epoch == picoquic_epoch_0rtt as ::core::ffi::c_int
                            && frame_id64
                                != picoquic_frame_type_bdp as ::core::ffi::c_int as uint64_t
                        {
                            picoquic_connection_error(
                                cnx,
                                PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                                first_byte as uint64_t,
                            );
                            bytes = ::core::ptr::null::<uint8_t>();
                        } else {
                            match frame_id64 {
                                175 => {
                                    bytes =
                                        picoquic_decode_ack_frequency_frame(bytes, bytes_max, cnx);
                                    ack_needed = 1 as ::core::ffi::c_int;
                                }
                                31 => {
                                    bytes = picoquic_decode_immediate_ack_frame(
                                        bytes,
                                        bytes_max,
                                        cnx,
                                        path_x,
                                        current_time,
                                    );
                                    ack_needed = 1 as ::core::ffi::c_int;
                                }
                                757 => {
                                    bytes = picoquic_decode_time_stamp_frame(
                                        bytes,
                                        bytes_max,
                                        cnx,
                                        &raw mut packet_data,
                                    );
                                }
                                354585600 => {
                                    bytes = picoquic_decode_ack_frame(
                                        cnx,
                                        bytes0,
                                        bytes_max,
                                        current_time,
                                        epoch,
                                        0 as ::core::ffi::c_int,
                                        1 as ::core::ffi::c_int,
                                        &raw mut packet_data,
                                    );
                                }
                                354585601 => {
                                    bytes = picoquic_decode_ack_frame(
                                        cnx,
                                        bytes0,
                                        bytes_max,
                                        current_time,
                                        epoch,
                                        1 as ::core::ffi::c_int,
                                        1 as ::core::ffi::c_int,
                                        &raw mut packet_data,
                                    );
                                }
                                354585605 => {
                                    bytes = picoquic_decode_path_abandon_frame(
                                        bytes,
                                        bytes_max,
                                        cnx,
                                        current_time,
                                    );
                                    ack_needed = 1 as ::core::ffi::c_int;
                                }
                                354585607 | 354585608 => {
                                    bytes = picoquic_decode_path_available_or_standby_frame(
                                        bytes,
                                        bytes_max,
                                        frame_id64,
                                        cnx,
                                        current_time,
                                    );
                                    ack_needed = 1 as ::core::ffi::c_int;
                                }
                                354585612 => {
                                    bytes =
                                        picoquic_decode_max_path_id_frame(bytes, bytes_max, cnx);
                                    ack_needed = 1 as ::core::ffi::c_int;
                                }
                                354585613 => {
                                    bytes =
                                        picoquic_decode_path_blocked_frame(bytes, bytes_max, cnx);
                                    ack_needed = 1 as ::core::ffi::c_int;
                                }
                                354585609 => {
                                    is_path_probing_frame = 1 as ::core::ffi::c_int;
                                    bytes = picoquic_decode_new_connection_id_frame(
                                        cnx,
                                        bytes0,
                                        bytes_max,
                                        current_time,
                                        1 as ::core::ffi::c_int,
                                    );
                                    ack_needed = 1 as ::core::ffi::c_int;
                                }
                                354585610 => {
                                    bytes = picoquic_decode_retire_connection_id_frame(
                                        cnx,
                                        bytes0,
                                        bytes_max,
                                        current_time,
                                        path_x,
                                        1 as ::core::ffi::c_int,
                                    );
                                    ack_needed = 1 as ::core::ffi::c_int;
                                }
                                60377 => {
                                    if (*cnx).client_mode() as ::core::ffi::c_int != 0
                                        && epoch != picoquic_epoch_1rtt as ::core::ffi::c_int
                                    {
                                        picoquic_connection_error(
                                            cnx,
                                            PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                                            first_byte as uint64_t,
                                        );
                                        bytes = ::core::ptr::null::<uint8_t>();
                                    } else if (*cnx).client_mode() == 0
                                        && epoch != picoquic_epoch_0rtt as ::core::ffi::c_int
                                        && epoch != picoquic_epoch_1rtt as ::core::ffi::c_int
                                    {
                                        picoquic_connection_error(
                                            cnx,
                                            PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                                            first_byte as uint64_t,
                                        );
                                        bytes = ::core::ptr::null::<uint8_t>();
                                    } else if (*cnx).client_mode() as ::core::ffi::c_int != 0
                                        && (*cnx).local_parameters.enable_bdp_frame
                                            == 0 as ::core::ffi::c_int
                                    {
                                        picoquic_connection_error(
                                            cnx,
                                            PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                                            0 as uint64_t,
                                        );
                                        bytes = ::core::ptr::null::<uint8_t>();
                                    } else {
                                        bytes = picoquic_decode_bdp_frame(
                                            cnx,
                                            bytes,
                                            bytes_max,
                                            current_time,
                                            addr_from,
                                            path_x,
                                        );
                                        ack_needed = 1 as ::core::ffi::c_int;
                                    }
                                }
                                10453414 | 10453415 => {
                                    is_path_probing_frame = 1 as ::core::ffi::c_int;
                                    ack_needed = 1 as ::core::ffi::c_int;
                                    bytes = picoquic_decode_observed_address_frame(
                                        cnx, bytes, bytes_max, path_x, frame_id64,
                                    );
                                }
                                _ => {
                                    picoquic_connection_error(
                                        cnx,
                                        PICOQUIC_TRANSPORT_FRAME_FORMAT_ERROR as uint64_t,
                                        frame_id64,
                                    );
                                    bytes = ::core::ptr::null::<uint8_t>();
                                }
                            }
                        }
                    }
                }
            }
        }
        is_path_probing_packet &= is_path_probing_frame;
    }
    if !bytes.is_null() {
        process_decoded_packet_data(cnx, path_x, epoch, current_time, &raw mut packet_data);
        if ack_needed != 0 {
            (*cnx).latest_receive_time = current_time;
            picoquic_set_ack_needed(cnx, current_time, pc, path_x, 0 as ::core::ffi::c_int);
        }
        if epoch == picoquic_epoch_1rtt as ::core::ffi::c_int
            && is_path_probing_packet == 0
            && pn64 > (*path_x).last_non_path_probing_pn
        {
            (*path_x).last_non_path_probing_pn = pn64;
        }
    }
    return if !bytes.is_null() {
        0 as ::core::ffi::c_int
    } else {
        PICOQUIC_ERROR_DETECTED
    };
}
unsafe extern "C" fn picoquic_skip_stream_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut len: uint8_t = (*bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 2 as ::core::ffi::c_int) as uint8_t;
    let mut off: uint8_t = (*bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 4 as ::core::ffi::c_int) as uint8_t;
    bytes = picoquic_frames_varint_skip(bytes.offset(1 as ::core::ffi::c_int as isize), bytes_max);
    if !bytes.is_null()
        && (off as ::core::ffi::c_int == 0 as ::core::ffi::c_int || {
            bytes = picoquic_frames_varint_skip(bytes, bytes_max);
            !bytes.is_null()
        })
    {
        bytes = if len as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            bytes_max as *mut uint8_t as *const uint8_t
        } else {
            picoquic_frames_length_data_skip(bytes, bytes_max)
        };
    }
    return bytes;
}
unsafe extern "C" fn picoquic_skip_crypto_hs_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes.offset(1 as ::core::ffi::c_int as isize), bytes_max);
    if !bytes.is_null() {
        bytes = picoquic_frames_length_data_skip(bytes, bytes_max);
    }
    return bytes;
}
unsafe extern "C" fn picoquic_skip_connection_close_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes.offset(1 as ::core::ffi::c_int as isize), bytes_max);
    if !bytes.is_null() && {
        bytes = picoquic_frames_varint_skip(bytes, bytes_max);
        !bytes.is_null()
    } {
        bytes = picoquic_frames_length_data_skip(bytes, bytes_max);
    }
    return bytes;
}
unsafe extern "C" fn picoquic_skip_application_close_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes.offset(1 as ::core::ffi::c_int as isize), bytes_max);
    if !bytes.is_null() {
        bytes = picoquic_frames_length_data_skip(bytes, bytes_max);
    }
    return bytes;
}
unsafe extern "C" fn picoquic_skip_ack_frame_maybe_ecn(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut is_ecn: ::core::ffi::c_int,
    mut has_path: ::core::ffi::c_int,
) -> *const uint8_t {
    let mut nb_blocks: uint64_t = 0;
    bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    if !bytes.is_null() && {
        bytes = picoquic_frames_varint_skip(bytes, bytes_max);
        !bytes.is_null()
    } {
        if has_path != 0 {
            bytes = picoquic_frames_varint_skip(bytes, bytes_max);
        }
        if !bytes.is_null()
            && {
                bytes = picoquic_frames_varint_skip(bytes, bytes_max);
                !bytes.is_null()
            }
            && {
                bytes = picoquic_frames_varint_decode(bytes, bytes_max, &raw mut nb_blocks);
                !bytes.is_null()
            }
            && {
                bytes = picoquic_frames_varint_skip(bytes, bytes_max);
                !bytes.is_null()
            }
        {
            loop {
                let c2rust_fresh4 = nb_blocks;
                nb_blocks = nb_blocks.wrapping_sub(1);
                if !(c2rust_fresh4 != 0 as uint64_t) {
                    break;
                }
                bytes = picoquic_frames_varint_skip(bytes, bytes_max);
                if bytes.is_null() || {
                    bytes = picoquic_frames_varint_skip(bytes, bytes_max);
                    bytes.is_null()
                } {
                    break;
                }
            }
        }
    }
    if !bytes.is_null() && is_ecn != 0 {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while !bytes.is_null() && i < 3 as ::core::ffi::c_int {
            bytes = picoquic_frames_varint_skip(bytes, bytes_max);
            i += 1;
        }
    }
    return bytes;
}
unsafe extern "C" fn picoquic_skip_ack_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    return picoquic_skip_ack_frame_maybe_ecn(
        bytes,
        bytes_max,
        0 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn picoquic_skip_ack_ecn_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    return picoquic_skip_ack_frame_maybe_ecn(
        bytes,
        bytes_max,
        1 as ::core::ffi::c_int,
        0 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn picoquic_skip_stream_reset_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes.offset(1 as ::core::ffi::c_int as isize), bytes_max);
    if !bytes.is_null() {
        bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    }
    if !bytes.is_null() {
        bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    }
    return bytes;
}
unsafe extern "C" fn picoquic_skip_max_stream_data_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes.offset(1 as ::core::ffi::c_int as isize), bytes_max);
    if !bytes.is_null() {
        bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    }
    return bytes;
}
unsafe extern "C" fn picoquic_skip_stream_blocked_frame(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    bytes = picoquic_frames_varint_skip(bytes.offset(1 as ::core::ffi::c_int as isize), bytes_max);
    if !bytes.is_null() {
        bytes = picoquic_frames_varint_skip(bytes, bytes_max);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_skip_frame(
    mut bytes: *const uint8_t,
    mut bytes_maxsize: size_t,
    mut consumed: *mut size_t,
    mut pure_ack: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut bytes_max: *const uint8_t = bytes.offset(bytes_maxsize as isize);
    let mut first_byte: uint8_t = *bytes.offset(0 as ::core::ffi::c_int as isize);
    *pure_ack = 1 as ::core::ffi::c_int;
    if first_byte as ::core::ffi::c_int
        & !(picoquic_frame_type_stream_range_min as ::core::ffi::c_int
            ^ picoquic_frame_type_stream_range_max as ::core::ffi::c_int)
        == picoquic_frame_type_stream_range_min as ::core::ffi::c_int
    {
        *pure_ack = 0 as ::core::ffi::c_int;
        bytes = picoquic_skip_stream_frame(bytes, bytes_max);
    } else {
        match first_byte as ::core::ffi::c_int {
            2 => {
                bytes = picoquic_skip_ack_frame(bytes, bytes_max);
            }
            3 => {
                bytes = picoquic_skip_ack_ecn_frame(bytes, bytes_max);
            }
            0 => {
                bytes = picoquic_skip_0len_frame(bytes, bytes_max);
            }
            4 => {
                bytes = picoquic_skip_stream_reset_frame(bytes, bytes_max);
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            28 => {
                bytes = picoquic_skip_connection_close_frame(bytes, bytes_max);
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            29 => {
                bytes = picoquic_skip_application_close_frame(bytes, bytes_max);
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            16 => {
                bytes = picoquic_frames_varint_skip(
                    bytes.offset(1 as ::core::ffi::c_int as isize),
                    bytes_max,
                );
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            17 => {
                bytes = picoquic_skip_max_stream_data_frame(bytes, bytes_max);
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            18 | 19 => {
                bytes = picoquic_frames_varint_skip(
                    bytes.offset(1 as ::core::ffi::c_int as isize),
                    bytes_max,
                );
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            1 => {
                bytes = picoquic_skip_0len_frame(bytes, bytes_max);
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            32 => {
                bytes = picoquic_skip_0len_frame(bytes, bytes_max);
            }
            20 => {
                bytes = picoquic_frames_varint_skip(
                    bytes.offset(1 as ::core::ffi::c_int as isize),
                    bytes_max,
                );
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            21 => {
                bytes = picoquic_skip_stream_blocked_frame(bytes, bytes_max);
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            22 | 23 => {
                bytes = picoquic_frames_varint_skip(
                    bytes.offset(1 as ::core::ffi::c_int as isize),
                    bytes_max,
                );
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            24 => {
                bytes = picoquic_skip_new_connection_id_frame(
                    bytes,
                    bytes_max,
                    0 as ::core::ffi::c_int,
                );
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            5 => {
                bytes = picoquic_skip_stop_sending_frame(bytes, bytes_max);
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            26 => {
                bytes = picoquic_frames_fixed_skip(
                    bytes.offset(1 as ::core::ffi::c_int as isize),
                    bytes_max,
                    challenge_length as uint64_t,
                );
            }
            27 => {
                bytes = picoquic_frames_fixed_skip(
                    bytes.offset(1 as ::core::ffi::c_int as isize),
                    bytes_max,
                    challenge_length as uint64_t,
                );
            }
            6 => {
                bytes = picoquic_skip_crypto_hs_frame(bytes, bytes_max);
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            7 => {
                bytes = picoquic_skip_new_token_frame(bytes, bytes_max);
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            25 => {
                bytes = picoquic_skip_retire_connection_id_frame(
                    bytes,
                    bytes_max,
                    0 as ::core::ffi::c_int,
                );
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            30 => {
                bytes = bytes.offset(1 as ::core::ffi::c_int as isize);
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            48 | 49 => {
                bytes = picoquic_skip_datagram_frame(bytes, bytes_max);
                *pure_ack = 0 as ::core::ffi::c_int;
            }
            _ => {
                let mut frame_id64: uint64_t = 0;
                let mut bytes_before_type: *const uint8_t = bytes;
                bytes = picoquic_frames_varint_decode(bytes, bytes_max, &raw mut frame_id64);
                if !bytes.is_null() {
                    match frame_id64 {
                        175 => {
                            bytes = picoquic_skip_ack_frequency_frame(bytes, bytes_max);
                            *pure_ack = 0 as ::core::ffi::c_int;
                        }
                        31 => {
                            bytes = picoquic_skip_immediate_ack_frame(bytes, bytes_max);
                            *pure_ack = 0 as ::core::ffi::c_int;
                        }
                        757 => {
                            bytes = picoquic_skip_time_stamp_frame(bytes, bytes_max);
                        }
                        354585600 => {
                            bytes = picoquic_skip_ack_frame_maybe_ecn(
                                bytes_before_type,
                                bytes_max,
                                0 as ::core::ffi::c_int,
                                1 as ::core::ffi::c_int,
                            );
                        }
                        354585601 => {
                            bytes = picoquic_skip_ack_frame_maybe_ecn(
                                bytes_before_type,
                                bytes_max,
                                1 as ::core::ffi::c_int,
                                1 as ::core::ffi::c_int,
                            );
                        }
                        354585605 => {
                            bytes = picoquic_skip_path_abandon_frame(bytes, bytes_max);
                            *pure_ack = 0 as ::core::ffi::c_int;
                        }
                        354585607 | 354585608 => {
                            bytes = picoquic_skip_path_available_or_standby_frame(bytes, bytes_max);
                            *pure_ack = 0 as ::core::ffi::c_int;
                        }
                        354585612 => {
                            bytes = picoquic_skip_max_path_id_frame(bytes, bytes_max);
                            *pure_ack = 0 as ::core::ffi::c_int;
                        }
                        354585613 => {
                            bytes = picoquic_skip_path_blocked_frame(bytes, bytes_max);
                            *pure_ack = 0 as ::core::ffi::c_int;
                        }
                        60377 => {
                            bytes = picoquic_skip_bdp_frame(bytes, bytes_max);
                            *pure_ack = 0 as ::core::ffi::c_int;
                        }
                        354585609 => {
                            bytes = picoquic_skip_new_connection_id_frame(
                                bytes_before_type,
                                bytes_max,
                                1 as ::core::ffi::c_int,
                            );
                            *pure_ack = 0 as ::core::ffi::c_int;
                        }
                        354585610 => {
                            bytes = picoquic_skip_retire_connection_id_frame(
                                bytes_before_type,
                                bytes_max,
                                1 as ::core::ffi::c_int,
                            );
                            *pure_ack = 0 as ::core::ffi::c_int;
                        }
                        10453414 | 10453415 => {
                            bytes =
                                picoquic_skip_observed_address_frame(bytes, bytes_max, frame_id64);
                            *pure_ack = 0 as ::core::ffi::c_int;
                        }
                        _ => {
                            bytes = ::core::ptr::null::<uint8_t>();
                        }
                    }
                }
            }
        }
    }
    *consumed = if !bytes.is_null() {
        bytes_maxsize.wrapping_sub(bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t)
    } else {
        bytes_maxsize
    };
    return (bytes == NULL as *const uint8_t) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_decode_closing_frames(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: size_t,
    mut closing_received: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut byte_index: size_t = 0 as size_t;
    *closing_received = 0 as ::core::ffi::c_int;
    while ret == 0 as ::core::ffi::c_int && byte_index < bytes_max {
        let mut first_byte: uint8_t = *bytes.offset(byte_index as isize);
        if first_byte as ::core::ffi::c_int
            == picoquic_frame_type_connection_close as ::core::ffi::c_int
            || first_byte as ::core::ffi::c_int
                == picoquic_frame_type_application_close as ::core::ffi::c_int
        {
            *closing_received = 1 as ::core::ffi::c_int;
            break;
        } else {
            let mut consumed: size_t = 0 as size_t;
            let mut pure_ack: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            ret = picoquic_skip_frame(
                bytes.offset(byte_index as isize),
                bytes_max.wrapping_sub(byte_index),
                &raw mut consumed,
                &raw mut pure_ack,
            );
            byte_index = byte_index.wrapping_add(consumed);
        }
    }
    return ret;
}

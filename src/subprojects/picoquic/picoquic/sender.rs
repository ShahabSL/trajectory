use ::c2rust_bitfields;
extern "C" {
    pub type st_ptls_iovec_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type st_ptls_buffer_t;
    pub type st_ptls_verify_certificate_t;
    fn picoquic_get_quic_time(quic: *mut picoquic_quic_t) -> uint64_t;
    fn picoquic_log_app_message(cnx: *mut picoquic_cnx_t, fmt: *const ::core::ffi::c_char, ...);
    fn picoquic_check_addr_blocked(addr_from: *const sockaddr) -> ::core::ffi::c_int;
    fn picoquic_delete_cnx(cnx: *mut picoquic_cnx_t);
    fn picoquic_probe_new_path_ex(
        cnx: *mut picoquic_cnx_t,
        addr_peer: *const sockaddr,
        addr_local: *const sockaddr,
        if_index: ::core::ffi::c_int,
        current_time: uint64_t,
        to_preferred_address: ::core::ffi::c_int,
        path_id_p: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_set_path_status(
        cnx: *mut picoquic_cnx_t,
        unique_path_id: uint64_t,
        status: picoquic_path_status_enum,
    ) -> ::core::ffi::c_int;
    fn picoquic_start_key_rotation(cnx: *mut picoquic_cnx_t) -> ::core::ffi::c_int;
    fn picoquic_get_earliest_cnx_to_wake(
        quic: *mut picoquic_quic_t,
        max_wake_time: uint64_t,
    ) -> *mut picoquic_cnx_t;
    fn picoquic_get_local_if_index(cnx: *mut picoquic_cnx_t) -> ::core::ffi::c_ulong;
    fn picoquic_queue_misc_frame(
        cnx: *mut picoquic_cnx_t,
        bytes: *const uint8_t,
        length: size_t,
        is_pure_ack: ::core::ffi::c_int,
        pc: picoquic_packet_context_enum,
    ) -> ::core::ffi::c_int;
    fn picoquic_cnx_is_still_logging(cnx: *mut picoquic_cnx_t) -> ::core::ffi::c_int;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    static picoquic_null_connection_id: picoquic_connection_id_t;
    fn picoquic_format_connection_id(
        bytes: *mut uint8_t,
        bytes_max: size_t,
        cnx_id: picoquic_connection_id_t,
    ) -> uint8_t;
    fn picoquic_is_connection_id_null(
        cnx_id: *const picoquic_connection_id_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_compare_addr(
        expected: *const sockaddr,
        actual: *const sockaddr,
    ) -> ::core::ffi::c_int;
    fn picoquic_store_addr(stored_addr: *mut sockaddr_storage, addr: *const sockaddr);
    static picoquic_supported_versions: [picoquic_version_parameters_t; 0];
    static mut picoquic_spin_function_table: [picoquic_spinbit_def_t; 0];
    fn picoquic_dequeue_stateless_packet(
        quic: *mut picoquic_quic_t,
    ) -> *mut picoquic_stateless_packet_t;
    fn picoquic_delete_stateless_packet(sp: *mut picoquic_stateless_packet_t);
    fn picoquic_get_token(
        quic: *mut picoquic_quic_t,
        sni: *const ::core::ffi::c_char,
        sni_length: uint16_t,
        ip_addr: *const uint8_t,
        ip_addr_length: uint8_t,
        token: *mut *mut uint8_t,
        token_length: *mut uint16_t,
        mark_used: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_register_net_secret(cnx: *mut picoquic_cnx_t) -> ::core::ffi::c_int;
    fn picoquic_demote_path(
        cnx: *mut picoquic_cnx_t,
        path_index: ::core::ffi::c_int,
        current_time: uint64_t,
        reason: uint64_t,
        phrase: *const ::core::ffi::c_char,
    );
    fn picoquic_promote_path_to_default(
        cnx: *mut picoquic_cnx_t,
        path_index: ::core::ffi::c_int,
        current_time: uint64_t,
    );
    fn picoquic_delete_abandoned_paths(
        cnx: *mut picoquic_cnx_t,
        current_time: uint64_t,
        next_wake_time: *mut uint64_t,
    );
    fn picoquic_get_path_id_from_unique(
        cnx: *mut picoquic_cnx_t,
        unique_path_id: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_stash_remote_cnxid(
        cnx: *mut picoquic_cnx_t,
        retire_before_next: uint64_t,
        unique_path_id: uint64_t,
        sequence: uint64_t,
        cid_length: uint8_t,
        cnxid_bytes: *const uint8_t,
        secret_bytes: *const uint8_t,
        pstashed: *mut *mut picoquic_remote_cnxid_t,
    ) -> uint64_t;
    fn picoquic_renew_path_connection_id(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_connection_error(
        cnx: *mut picoquic_cnx_t,
        local_error: uint64_t,
        frame_type: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_connection_disconnect(cnx: *mut picoquic_cnx_t);
    fn picoquic_is_pacing_blocked(pacing: *mut picoquic_pacing_t) -> ::core::ffi::c_int;
    fn picoquic_is_authorized_by_pacing(
        pacing: *mut picoquic_pacing_t,
        current_time: uint64_t,
        next_time: *mut uint64_t,
        packet_train_mode: ::core::ffi::c_uint,
        quic: *mut picoquic_quic_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_update_pacing_data_after_send(
        pacing: *mut picoquic_pacing_t,
        length: size_t,
        send_mtu: size_t,
        current_time: uint64_t,
    );
    fn picoquic_update_pacing_after_send(
        path_x: *mut picoquic_path_t,
        length: size_t,
        current_time: uint64_t,
    );
    fn picoquic_is_sending_authorized_by_pacing(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
        current_time: uint64_t,
        next_time: *mut uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_reinsert_by_wake_time(
        quic: *mut picoquic_quic_t,
        cnx: *mut picoquic_cnx_t,
        next_time: uint64_t,
    );
    fn picoformat_16(bytes: *mut uint8_t, n16: uint16_t);
    fn picoformat_24(bytes: *mut uint8_t, n24: uint32_t);
    fn picoformat_32(bytes: *mut uint8_t, n32: uint32_t);
    fn picoquic_varint_encode(bytes: *mut uint8_t, max_bytes: size_t, n64: uint64_t) -> size_t;
    fn picoquic_varint_encode_16(bytes: *mut uint8_t, n16: uint16_t);
    fn picoquic_log_pn_dec_trial(cnx: *mut picoquic_cnx_t);
    fn picoquic_is_ack_needed(
        cnx: *mut picoquic_cnx_t,
        current_time: uint64_t,
        next_wake_time: *mut uint64_t,
        pc: picoquic_packet_context_enum,
        is_opportunistic: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_check_sack_list(
        sack: *mut picoquic_sack_list_t,
        pn64_min: uint64_t,
        pn64_max: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_sack_list_first(first_sack: *mut picoquic_sack_list_t) -> uint64_t;
    fn picoquic_sack_list_last(first_sack: *mut picoquic_sack_list_t) -> uint64_t;
    fn picoquic_init_packet_ctx(
        cnx: *mut picoquic_cnx_t,
        pkt_ctx: *mut picoquic_packet_context_t,
        pc: picoquic_packet_context_enum,
    );
    fn picoquic_compute_ack_gap_and_delay(
        cnx: *mut picoquic_cnx_t,
        rtt: uint64_t,
        remote_min_ack_delay: uint64_t,
        data_rate: uint64_t,
        ack_gap: *mut uint64_t,
        ack_delay_max: *mut uint64_t,
    );
    fn picoquic_current_retransmit_timer(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
    ) -> uint64_t;
    fn picoquic_create_missing_streams(
        cnx: *mut picoquic_cnx_t,
        stream_id: uint64_t,
        is_remote: ::core::ffi::c_int,
    ) -> *mut picoquic_stream_head_t;
    fn picoquic_insert_output_stream(cnx: *mut picoquic_cnx_t, stream: *mut picoquic_stream_head_t);
    fn picoquic_reorder_output_stream(
        cnx: *mut picoquic_cnx_t,
        stream: *mut picoquic_stream_head_t,
    );
    fn picoquic_find_stream(
        cnx: *mut picoquic_cnx_t,
        stream_id: uint64_t,
    ) -> *mut picoquic_stream_head_t;
    fn picoquic_find_ready_stream_path(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
    ) -> *mut picoquic_stream_head_t;
    fn picoquic_find_ready_stream(cnx: *mut picoquic_cnx_t) -> *mut picoquic_stream_head_t;
    fn picoquic_is_tls_stream_ready(cnx: *mut picoquic_cnx_t) -> ::core::ffi::c_int;
    fn picoquic_check_frame_needs_repeat(
        cnx: *mut picoquic_cnx_t,
        bytes: *const uint8_t,
        bytes_max: size_t,
        p_type: picoquic_packet_type_enum,
        no_need_to_repeat: *mut ::core::ffi::c_int,
        do_not_detect_spurious: *mut ::core::ffi::c_int,
        is_preemptive_needed: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_format_available_stream_frames(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
        bytes_next: *mut uint8_t,
        bytes_max: *mut uint8_t,
        current_priority: uint64_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
        stream_tried_and_failed: *mut ::core::ffi::c_int,
        ret: *mut ::core::ffi::c_int,
    ) -> *mut uint8_t;
    fn picoquic_queue_data_repeat_packet(cnx: *mut picoquic_cnx_t, packet: *mut picoquic_packet_t);
    fn picoquic_first_data_repeat_packet(cnx: *mut picoquic_cnx_t) -> *mut picoquic_packet_t;
    fn picoquic_copy_stream_frames_for_retransmit(
        cnx: *mut picoquic_cnx_t,
        bytes_next: *mut uint8_t,
        bytes_max: *mut uint8_t,
        current_priority: uint64_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
    ) -> *mut uint8_t;
    fn picoquic_retransmit_needed(
        cnx: *mut picoquic_cnx_t,
        pc: picoquic_packet_context_enum,
        path_x: *mut picoquic_path_t,
        current_time: uint64_t,
        next_wake_time: *mut uint64_t,
        packet: *mut picoquic_packet_t,
        send_buffer_max: size_t,
        header_length: *mut size_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_is_stream_frame_unlimited(bytes: *const uint8_t) -> ::core::ffi::c_int;
    fn picoquic_format_crypto_hs_frame(
        stream: *mut picoquic_stream_head_t,
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
    ) -> *mut uint8_t;
    fn picoquic_format_ack_frame(
        cnx: *mut picoquic_cnx_t,
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        more_data: *mut ::core::ffi::c_int,
        current_time: uint64_t,
        pc: picoquic_packet_context_enum,
        is_opportunistic: ::core::ffi::c_int,
    ) -> *mut uint8_t;
    fn picoquic_format_connection_close_frame(
        cnx: *mut picoquic_cnx_t,
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
    ) -> *mut uint8_t;
    fn picoquic_format_application_close_frame(
        cnx: *mut picoquic_cnx_t,
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
    ) -> *mut uint8_t;
    fn picoquic_format_required_max_stream_data_frames(
        cnx: *mut picoquic_cnx_t,
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
    ) -> *mut uint8_t;
    fn picoquic_format_max_data_frame(
        cnx: *mut picoquic_cnx_t,
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
        maxdata_increase: uint64_t,
    ) -> *mut uint8_t;
    fn picoquic_format_max_stream_data_frame(
        cnx: *mut picoquic_cnx_t,
        stream: *mut picoquic_stream_head_t,
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
        new_max_data: uint64_t,
    ) -> *mut uint8_t;
    fn picoquic_cc_increased_window(
        cnx: *mut picoquic_cnx_t,
        previous_window: uint64_t,
    ) -> uint64_t;
    fn picoquic_format_max_streams_frame_if_needed(
        cnx: *mut picoquic_cnx_t,
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
    ) -> *mut uint8_t;
    fn picoquic_find_or_create_local_cnxid_list(
        cnx: *mut picoquic_cnx_t,
        unique_path_id: uint64_t,
        do_create: ::core::ffi::c_int,
    ) -> *mut picoquic_local_cnxid_list_t;
    fn picoquic_create_local_cnxid(
        cnx: *mut picoquic_cnx_t,
        unique_path_id: uint64_t,
        suggested_value: *mut picoquic_connection_id_t,
        current_time: uint64_t,
    ) -> *mut picoquic_local_cnxid_t;
    fn picoquic_delete_local_cnxid(cnx: *mut picoquic_cnx_t, l_cid: *mut picoquic_local_cnxid_t);
    fn picoquic_check_local_cnxid_ttl(
        cnx: *mut picoquic_cnx_t,
        local_cnxid_list: *mut picoquic_local_cnxid_list_t,
        current_time: uint64_t,
        next_wake_time: *mut uint64_t,
    );
    fn picoquic_format_path_challenge_frame(
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
        challenge: uint64_t,
    ) -> *mut uint8_t;
    fn picoquic_format_path_response_frame(
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
        challenge: uint64_t,
    ) -> *mut uint8_t;
    fn picoquic_format_new_connection_id_frame(
        cnx: *mut picoquic_cnx_t,
        local_cnxid_list: *mut picoquic_local_cnxid_list_t,
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
        l_cid: *mut picoquic_local_cnxid_t,
    ) -> *mut uint8_t;
    fn picoquic_format_max_path_id_frame(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        max_path_id: uint64_t,
        more_data: *mut ::core::ffi::c_int,
    ) -> *mut uint8_t;
    fn picoquic_format_blocked_frames(
        cnx: *mut picoquic_cnx_t,
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
    ) -> *mut uint8_t;
    fn picoquic_queue_new_token_frame(
        cnx: *mut picoquic_cnx_t,
        token: *mut uint8_t,
        token_length: size_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_find_first_misc_frame(
        cnx: *mut picoquic_cnx_t,
        pc: picoquic_packet_context_enum,
    ) -> *mut picoquic_misc_frame_header_t;
    fn picoquic_format_misc_frames_in_context(
        cnx: *mut picoquic_cnx_t,
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
        pc: picoquic_packet_context_enum,
    ) -> *mut uint8_t;
    fn picoquic_purge_misc_frames_after_ready(cnx: *mut picoquic_cnx_t);
    fn picoquic_queue_handshake_done_frame(cnx: *mut picoquic_cnx_t) -> ::core::ffi::c_int;
    fn picoquic_format_first_datagram_frame(
        cnx: *mut picoquic_cnx_t,
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
    ) -> *mut uint8_t;
    fn picoquic_format_ready_datagram_frame(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
        ret: *mut ::core::ffi::c_int,
    ) -> *mut uint8_t;
    fn picoquic_format_ack_frequency_frame(
        cnx: *mut picoquic_cnx_t,
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        more_data: *mut ::core::ffi::c_int,
    ) -> *mut uint8_t;
    fn picoquic_format_bdp_frame(
        cnx: *mut picoquic_cnx_t,
        bytes: *mut uint8_t,
        bytes_max: *mut uint8_t,
        path_x: *mut picoquic_path_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
    ) -> *mut uint8_t;
    fn picoquic_prepare_observed_address_frame(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        path_x: *mut picoquic_path_t,
        current_time: uint64_t,
        next_wake_time: *mut uint64_t,
        more_data: *mut ::core::ffi::c_int,
        is_pure_ack: *mut ::core::ffi::c_int,
    ) -> *mut uint8_t;
    fn picoquic_skip_frame(
        bytes: *const uint8_t,
        bytes_max: size_t,
        consumed: *mut size_t,
        pure_ack: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_process_sooner_packets(cnx: *mut picoquic_cnx_t, current_time: uint64_t);
    fn picoquic_log_pdu(
        cnx: *mut picoquic_cnx_t,
        receiving: ::core::ffi::c_int,
        current_time: uint64_t,
        addr_peer: *const sockaddr,
        addr_local: *const sockaddr,
        packet_length: size_t,
    );
    fn picoquic_log_outgoing_packet(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
        bytes: *mut uint8_t,
        sequence_number: uint64_t,
        pn_length: size_t,
        length: size_t,
        send_buffer: *mut uint8_t,
        send_length: size_t,
        current_time: uint64_t,
    );
    fn picoquic_log_cc_dump(cnx: *mut picoquic_cnx_t, current_time: uint64_t);
    fn picoquic_tlscontext_trim_after_handshake(cnx: *mut picoquic_cnx_t);
    fn picoquic_public_random_64() -> uint64_t;
    fn picoquic_public_random_seed(quic: *mut picoquic_quic_t);
    fn picoquic_public_uniform_random(rnd_max: uint64_t) -> uint64_t;
    fn picoquic_aead_get_checksum_length(aead_context: *mut ::core::ffi::c_void) -> size_t;
    fn picoquic_aead_encrypt_generic(
        output: *mut uint8_t,
        input: *const uint8_t,
        input_length: size_t,
        seq_num: uint64_t,
        auth_data: *const uint8_t,
        auth_data_length: size_t,
        aead_context: *mut ::core::ffi::c_void,
    ) -> size_t;
    fn picoquic_aead_encrypt_mp(
        output: *mut uint8_t,
        input: *const uint8_t,
        input_length: size_t,
        path_id: uint64_t,
        seq_num: uint64_t,
        auth_data: *const uint8_t,
        auth_data_length: size_t,
        aead_context: *mut ::core::ffi::c_void,
    ) -> size_t;
    fn picoquic_aead_confidentiality_limit(aead_ctx: *mut ::core::ffi::c_void) -> uint64_t;
    fn picoquic_pn_encrypt(
        pn_enc: *mut ::core::ffi::c_void,
        iv: *const ::core::ffi::c_void,
        output: *mut ::core::ffi::c_void,
        input: *const ::core::ffi::c_void,
        len: size_t,
    );
    fn picoquic_crypto_context_free(ctx: *mut picoquic_crypto_context_t);
    fn picoquic_tls_client_authentication_activated(
        quic: *mut picoquic_quic_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_prepare_retry_token(
        quic: *mut picoquic_quic_t,
        addr_peer: *const sockaddr,
        current_time: uint64_t,
        odcid: *const picoquic_connection_id_t,
        rcid: *const picoquic_connection_id_t,
        initial_pn: uint32_t,
        token: *mut uint8_t,
        token_max: size_t,
        token_size: *mut size_t,
    ) -> ::core::ffi::c_int;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
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
pub type picoquic_spinbit_outgoing_fn =
    Option<unsafe extern "C" fn(*mut picoquic_cnx_t) -> uint8_t>;
pub type picoquic_spinbit_def_t = st_picoquic_spinbit_def_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_spinbit_def_t {
    pub spinbit_incoming: picoquic_spinbit_incoming_fn,
    pub spinbit_outgoing: picoquic_spinbit_outgoing_fn,
}
pub type picoquic_spinbit_incoming_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_cnx_t,
        *mut picoquic_path_t,
        *mut picoquic_packet_header,
    ) -> (),
>;
pub type picoquic_packet_header = st_picoquic_packet_header_t;
pub const picoquic_frame_type_ping: C2Rust_Unnamed_0 = 1;
pub const picoquic_frame_type_poll: C2Rust_Unnamed_0 = 32;
pub const picoquic_pmtu_discovery_not_needed: picoquic_pmtu_discovery_status_enum = 0;
pub type picoquic_pmtu_discovery_status_enum = ::core::ffi::c_uint;
pub const picoquic_pmtu_discovery_required: picoquic_pmtu_discovery_status_enum = 2;
pub const picoquic_pmtu_discovery_optional: picoquic_pmtu_discovery_status_enum = 1;
pub const picoquic_frame_type_padding: C2Rust_Unnamed_0 = 0;
pub const picoquic_frame_type_stream_range_min: C2Rust_Unnamed_0 = 8;
pub const picoquic_frame_type_stream_range_max: C2Rust_Unnamed_0 = 15;
pub const picoquic_frame_type_new_connection_id: C2Rust_Unnamed_0 = 24;
pub const picoquic_frame_type_new_token: C2Rust_Unnamed_0 = 7;
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
pub const picoquic_frame_type_datagram_l: C2Rust_Unnamed_0 = 49;
pub const picoquic_frame_type_datagram: C2Rust_Unnamed_0 = 48;
pub const picoquic_frame_type_handshake_done: C2Rust_Unnamed_0 = 30;
pub const picoquic_frame_type_application_close: C2Rust_Unnamed_0 = 29;
pub const picoquic_frame_type_connection_close: C2Rust_Unnamed_0 = 28;
pub const picoquic_frame_type_path_response: C2Rust_Unnamed_0 = 27;
pub const picoquic_frame_type_path_challenge: C2Rust_Unnamed_0 = 26;
pub const picoquic_frame_type_path_retire_connection_id: C2Rust_Unnamed_0 = 354585610;
pub const picoquic_frame_type_retire_connection_id: C2Rust_Unnamed_0 = 25;
pub const picoquic_frame_type_path_new_connection_id: C2Rust_Unnamed_0 = 354585609;
pub const picoquic_frame_type_streams_blocked_unidir: C2Rust_Unnamed_0 = 23;
pub const picoquic_frame_type_streams_blocked_bidir: C2Rust_Unnamed_0 = 22;
pub const picoquic_frame_type_stream_data_blocked: C2Rust_Unnamed_0 = 21;
pub const picoquic_frame_type_data_blocked: C2Rust_Unnamed_0 = 20;
pub const picoquic_frame_type_max_streams_unidir: C2Rust_Unnamed_0 = 19;
pub const picoquic_frame_type_max_streams_bidir: C2Rust_Unnamed_0 = 18;
pub const picoquic_frame_type_max_stream_data: C2Rust_Unnamed_0 = 17;
pub const picoquic_frame_type_max_data: C2Rust_Unnamed_0 = 16;
pub const picoquic_frame_type_crypto_hs: C2Rust_Unnamed_0 = 6;
pub const picoquic_frame_type_stop_sending: C2Rust_Unnamed_0 = 5;
pub const picoquic_frame_type_reset_stream: C2Rust_Unnamed_0 = 4;
pub const picoquic_frame_type_ack_ecn: C2Rust_Unnamed_0 = 3;
pub const picoquic_frame_type_ack: C2Rust_Unnamed_0 = 2;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
pub const PF_UNSPEC: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_UNSPEC: ::core::ffi::c_int = PF_UNSPEC;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const PICOQUIC_ERROR_CLASS: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_MEMORY: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 5 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_INVALID_STREAM_ID: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 14 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_STREAM_ALREADY_CLOSED: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 15 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_DISCONNECTED: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 20 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_UNEXPECTED_STATE: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 26 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_UNEXPECTED_ERROR: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 27 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_CANNOT_SET_ACTIVE_STREAM: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 36 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_IDLE_TIMEOUT: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 51 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_REPEAT_TIMEOUT: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 52 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_INTERNAL_ERROR: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_APPLICATION_ERROR: ::core::ffi::c_int = 0xc as ::core::ffi::c_int;
pub const PICOQUIC_MAX_PACKET_SIZE: ::core::ffi::c_int = 1536 as ::core::ffi::c_int;
pub const PICOQUIC_MIN_SEGMENT_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const PICOQUIC_ENFORCED_INITIAL_MTU: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const PICOQUIC_PRACTICAL_MAX_MTU: ::core::ffi::c_int = 1440 as ::core::ffi::c_int;
pub const PICOQUIC_DEFAULT_0RTT_WINDOW: ::core::ffi::c_int =
    10 as ::core::ffi::c_int * PICOQUIC_ENFORCED_INITIAL_MTU;
pub const PICOQUIC_NB_PATH_TARGET: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PICOQUIC_MAX_PACKETS_IN_POOL: ::core::ffi::c_int = 0x2000 as ::core::ffi::c_int;
pub const PICOQUIC_INITIAL_RTT: ::core::ffi::c_ulonglong = 250000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_INITIAL_RETRANSMIT_TIMER: ::core::ffi::c_ulonglong =
    250000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_ACK_DELAY_MAX: ::core::ffi::c_ulonglong = 100000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_ACK_DELAY_MIN: ::core::ffi::c_ulonglong = 1000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_TOKEN_DELAY_LONG: ::core::ffi::c_ulonglong =
    ((24 as ::core::ffi::c_int * 60 as ::core::ffi::c_int * 60 as ::core::ffi::c_int)
        as ::core::ffi::c_ulonglong)
        .wrapping_mul(1000000 as ::core::ffi::c_ulonglong);
pub const PICOQUIC_CID_REFRESH_DELAY: ::core::ffi::c_ulonglong =
    (5 as ::core::ffi::c_ulonglong).wrapping_mul(1000000 as ::core::ffi::c_ulonglong);
pub const PICOQUIC_MICROSEC_SILENCE_MAX: ::core::ffi::c_ulonglong =
    120000000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_MICROSEC_HANDSHAKE_MAX: ::core::ffi::c_ulonglong =
    30000000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_CHALLENGE_REPEAT_MAX: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PICOQUIC_V1_VERSION: uint32_t = 1 as uint32_t;
pub const PICOQUIC_V2_VERSION: uint32_t = 1798521807 as uint32_t;
pub const PICOQUIC_LOSS_BIT_Q_HALF_PERIOD: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
unsafe extern "C" fn picoquic_find_stream_for_writing(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut ret: *mut ::core::ffi::c_int,
) -> *mut picoquic_stream_head_t {
    let mut stream: *mut picoquic_stream_head_t = picoquic_find_stream(cnx, stream_id);
    *ret = 0 as ::core::ffi::c_int;
    if stream.is_null() {
        if (stream_id & 1 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int as ::core::ffi::c_uint
            != (*cnx).client_mode()
        {
            *ret = PICOQUIC_ERROR_INVALID_STREAM_ID;
        }
        if *ret == 0 as ::core::ffi::c_int {
            stream = picoquic_create_missing_streams(cnx, stream_id, 0 as ::core::ffi::c_int);
            if stream.is_null() {
                *ret = PICOQUIC_ERROR_MEMORY;
            }
        }
    }
    return stream;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_app_stream_ctx(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut app_stream_ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream: *mut picoquic_stream_head_t =
        picoquic_find_stream_for_writing(cnx, stream_id, &raw mut ret);
    if ret == 0 as ::core::ffi::c_int {
        (*stream).app_stream_ctx = app_stream_ctx;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_unlink_app_stream_ctx(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
) {
    let mut stream: *mut picoquic_stream_head_t = picoquic_find_stream(cnx, stream_id);
    if !stream.is_null() {
        (*stream).app_stream_ctx = NULL;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_mark_datagram_ready(
    mut cnx: *mut picoquic_cnx_t,
    mut is_ready: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut was_ready: ::core::ffi::c_int = (*cnx).is_datagram_ready() as ::core::ffi::c_int;
    (*cnx).set_is_datagram_ready(is_ready as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if was_ready == 0 && is_ready != 0 {
        if (*cnx).remote_parameters.max_datagram_frame_size == 0 as uint32_t {
            ret = -(1 as ::core::ffi::c_int);
        } else {
            picoquic_reinsert_by_wake_time(
                (*cnx).quic,
                cnx,
                picoquic_get_quic_time((*cnx).quic as *mut picoquic_quic_t),
            );
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_mark_datagram_ready_path(
    mut cnx: *mut picoquic_cnx_t,
    mut unique_path_id: uint64_t,
    mut is_path_ready: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut path_id: ::core::ffi::c_int = picoquic_get_path_id_from_unique(cnx, unique_path_id);
    if path_id >= 0 as ::core::ffi::c_int {
        let mut was_ready: ::core::ffi::c_int =
            (**(*cnx).path.offset(path_id as isize)).is_datagram_ready() as ::core::ffi::c_int;
        let ref mut c2rust_fresh39 = **(*cnx).path.offset(path_id as isize);
        (*c2rust_fresh39)
            .set_is_datagram_ready(is_path_ready as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if was_ready == 0 && is_path_ready != 0 {
            if (*cnx).remote_parameters.max_datagram_frame_size == 0 as uint32_t {
                ret = -(1 as ::core::ffi::c_int);
            } else {
                picoquic_reinsert_by_wake_time(
                    (*cnx).quic,
                    cnx,
                    picoquic_get_quic_time((*cnx).quic as *mut picoquic_quic_t),
                );
            }
        }
    } else {
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_mark_active_stream(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut is_active: ::core::ffi::c_int,
    mut app_stream_ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream: *mut picoquic_stream_head_t =
        picoquic_find_stream_for_writing(cnx, stream_id, &raw mut ret);
    if ret == 0 as ::core::ffi::c_int {
        if is_active != 0 {
            if (*stream).fin_requested() == 0
                && (*stream).reset_requested() == 0
                && (*cnx).callback_fn.is_some()
            {
                (*stream).app_stream_ctx = app_stream_ctx;
                if (*stream).is_active() == 0 {
                    (*stream).set_is_active(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    picoquic_reinsert_by_wake_time(
                        (*cnx).quic,
                        cnx,
                        picoquic_get_quic_time((*cnx).quic as *mut picoquic_quic_t),
                    );
                }
            } else {
                ret = PICOQUIC_ERROR_CANNOT_SET_ACTIVE_STREAM;
            }
        } else {
            (*stream).set_is_active(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*stream).app_stream_ctx = app_stream_ctx;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_datagram_priority(
    mut quic: *mut picoquic_quic_t,
    mut default_datagram_priority: uint8_t,
) {
    (*quic).default_datagram_priority = default_datagram_priority;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_datagram_priority(
    mut cnx: *mut picoquic_cnx_t,
    mut datagram_priority: uint8_t,
) {
    (*cnx).datagram_priority = datagram_priority as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_default_priority(
    mut quic: *mut picoquic_quic_t,
    mut default_stream_priority: uint8_t,
) {
    (*quic).default_stream_priority = default_stream_priority;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_stream_priority(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut stream_priority: uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream: *mut picoquic_stream_head_t =
        picoquic_find_stream_for_writing(cnx, stream_id, &raw mut ret);
    if ret == 0 as ::core::ffi::c_int {
        (*stream).stream_priority = stream_priority;
        picoquic_reorder_output_stream(cnx, stream);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_mark_high_priority_stream(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut is_high_priority: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    if is_high_priority != 0 {
        (*cnx).high_priority_stream_id = stream_id;
    } else if (*cnx).high_priority_stream_id == stream_id {
        (*cnx).high_priority_stream_id = UINT64_MAX as uint64_t;
    }
    ret = picoquic_set_stream_priority(
        cnx,
        stream_id,
        (if is_high_priority != 0 {
            0 as ::core::ffi::c_int
        } else {
            (*(*cnx).quic).default_stream_priority as ::core::ffi::c_int
        }) as uint8_t,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_add_to_stream_with_ctx(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut data: *const uint8_t,
    mut length: size_t,
    mut set_fin: ::core::ffi::c_int,
    mut app_stream_ctx: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream: *mut picoquic_stream_head_t =
        picoquic_find_stream_for_writing(cnx, stream_id, &raw mut ret);
    if ret == 0 as ::core::ffi::c_int && set_fin != 0 {
        if (*stream).fin_requested() != 0 {
            if length > 0 as size_t {
                ret = -(1 as ::core::ffi::c_int);
            }
        } else {
            (*stream).set_fin_requested(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    }
    if ret == 0 as ::core::ffi::c_int
        && ((*stream).reset_sent() as ::core::ffi::c_int != 0
            || (*stream).stop_sending_received() as ::core::ffi::c_int != 0)
    {
        ret = -(1 as ::core::ffi::c_int);
    }
    if ret == 0 as ::core::ffi::c_int && length > 0 as size_t {
        let mut stream_data: *mut picoquic_stream_queue_node_t =
            malloc(::core::mem::size_of::<picoquic_stream_queue_node_t>() as size_t)
                as *mut picoquic_stream_queue_node_t;
        if stream_data.is_null() {
            ret = -(1 as ::core::ffi::c_int);
        } else {
            (*stream_data).bytes = malloc(length) as *mut uint8_t;
            if (*stream_data).bytes.is_null() {
                free(stream_data as *mut ::core::ffi::c_void);
                stream_data = ::core::ptr::null_mut::<picoquic_stream_queue_node_t>();
                ret = -(1 as ::core::ffi::c_int);
            } else {
                let mut pprevious: *mut *mut picoquic_stream_queue_node_t =
                    &raw mut (*stream).send_queue;
                let mut next: *mut picoquic_stream_queue_node_t = (*stream).send_queue;
                memcpy(
                    (*stream_data).bytes as *mut ::core::ffi::c_void,
                    data as *const ::core::ffi::c_void,
                    length,
                );
                (*stream_data).length = length;
                (*stream_data).offset = 0 as uint64_t;
                (*stream_data).next_stream_data =
                    ::core::ptr::null_mut::<st_picoquic_stream_queue_node_t>();
                while !next.is_null() {
                    pprevious =
                        &raw mut (*next).next_stream_data as *mut *mut picoquic_stream_queue_node_t;
                    next = (*next).next_stream_data as *mut picoquic_stream_queue_node_t;
                }
                *pprevious = stream_data;
            }
        }
        picoquic_reinsert_by_wake_time(
            (*cnx).quic,
            cnx,
            picoquic_get_quic_time((*cnx).quic as *mut picoquic_quic_t),
        );
    }
    if ret == 0 as ::core::ffi::c_int {
        (*cnx).nb_bytes_queued = ((*cnx).nb_bytes_queued as ::core::ffi::c_ulong)
            .wrapping_add(length as ::core::ffi::c_ulong)
            as uint64_t as uint64_t;
        (*stream).set_is_active(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*stream).app_stream_ctx = app_stream_ctx;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_add_to_stream(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut data: *const uint8_t,
    mut length: size_t,
    mut set_fin: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return picoquic_add_to_stream_with_ctx(cnx, stream_id, data, length, set_fin, NULL);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_open_flow_control(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut expected_data_size: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut buffer: [uint8_t; 512] = [0; 512];
    let mut length: size_t = 0 as size_t;
    let mut consumed: size_t = 0 as size_t;
    let mut stream: *mut picoquic_stream_head_t = picoquic_find_stream(cnx, stream_id);
    if (*cnx).cnx_state as ::core::ffi::c_uint
        == picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*(*cnx).quic).max_data_limit == 0 as uint64_t
    {
        if stream.is_null() {
            ret = PICOQUIC_ERROR_INVALID_STREAM_ID;
        } else {
            let mut max_required: uint64_t =
                (*stream).consumed_offset.wrapping_add(expected_data_size);
            let mut bytes_max: *mut uint8_t = (&raw mut buffer as *mut uint8_t)
                .offset(::core::mem::size_of::<[uint8_t; 512]>() as usize as isize);
            let mut more_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut is_pure_ack: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
            if max_required > (*stream).maxdata_local {
                let mut bytes_next: *mut uint8_t = picoquic_format_max_stream_data_frame(
                    cnx,
                    stream,
                    (&raw mut buffer as *mut uint8_t).offset(consumed as isize),
                    bytes_max,
                    &raw mut more_data,
                    &raw mut is_pure_ack,
                    max_required,
                );
                bytes_next = picoquic_format_max_data_frame(
                    cnx,
                    bytes_next,
                    bytes_max,
                    &raw mut more_data,
                    &raw mut is_pure_ack,
                    expected_data_size,
                );
                length = bytes_next.offset_from(&raw mut buffer as *mut uint8_t)
                    as ::core::ffi::c_long as size_t;
                if length > 0 as size_t {
                    ret = picoquic_queue_misc_frame(
                        cnx as *mut picoquic_cnx_t,
                        &raw mut buffer as *mut uint8_t,
                        length,
                        is_pure_ack,
                        picoquic_packet_context_application,
                    );
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_reset_stream_ctx(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
) {
    let mut stream: *mut picoquic_stream_head_t = picoquic_find_stream(cnx, stream_id);
    if !stream.is_null() {
        (*stream).app_stream_ctx = NULL;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_reset_stream(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut local_stream_error: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream: *mut picoquic_stream_head_t = ::core::ptr::null_mut::<picoquic_stream_head_t>();
    stream = picoquic_find_stream(cnx, stream_id);
    if stream.is_null() {
        ret = PICOQUIC_ERROR_INVALID_STREAM_ID;
    } else {
        (*stream).app_stream_ctx = NULL;
        if (*stream).fin_sent() as ::core::ffi::c_int != 0
            && picoquic_check_sack_list(
                &raw mut (*stream).sack_list,
                0 as uint64_t,
                (*stream).fin_offset,
            ) == 0 as ::core::ffi::c_int
        {
            ret = PICOQUIC_ERROR_STREAM_ALREADY_CLOSED;
        } else if (*stream).reset_requested() == 0 {
            (*stream).local_error = local_stream_error;
            (*stream).set_reset_requested(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    }
    picoquic_reinsert_by_wake_time(
        (*cnx).quic,
        cnx,
        picoquic_get_quic_time((*cnx).quic as *mut picoquic_quic_t),
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_next_local_stream_id(
    mut cnx: *mut picoquic_cnx_t,
    mut is_unidir: ::core::ffi::c_int,
) -> uint64_t {
    let mut stream_type_id: ::core::ffi::c_int =
        (*cnx).client_mode() as ::core::ffi::c_int ^ 1 as ::core::ffi::c_int;
    if is_unidir != 0 {
        stream_type_id |= 2 as ::core::ffi::c_int;
    }
    return (*cnx).next_stream_id[stream_type_id as usize];
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_stop_sending(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut local_stream_error: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream: *mut picoquic_stream_head_t = ::core::ptr::null_mut::<picoquic_stream_head_t>();
    stream = picoquic_find_stream(cnx, stream_id);
    if stream.is_null() {
        ret = PICOQUIC_ERROR_INVALID_STREAM_ID;
    } else {
        (*stream).app_stream_ctx = NULL;
        if (*stream).reset_received() != 0 {
            ret = PICOQUIC_ERROR_STREAM_ALREADY_CLOSED;
        } else if (*stream).stop_sending_requested() == 0 {
            (*stream).local_stop_error = local_stream_error;
            (*stream).set_stop_sending_requested(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            picoquic_insert_output_stream(cnx, stream);
        }
    }
    picoquic_reinsert_by_wake_time(
        (*cnx).quic,
        cnx,
        picoquic_get_quic_time((*cnx).quic as *mut picoquic_quic_t),
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_discard_stream(
    mut cnx: *mut picoquic_cnx_t,
    mut stream_id: uint64_t,
    mut local_stream_error: uint16_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream: *mut picoquic_stream_head_t = ::core::ptr::null_mut::<picoquic_stream_head_t>();
    stream = picoquic_find_stream(cnx, stream_id);
    if stream.is_null() {
        ret = PICOQUIC_ERROR_INVALID_STREAM_ID;
    } else {
        if (stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int as ::core::ffi::c_uint
            != 0
            || (stream_id & 1 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
                as ::core::ffi::c_uint
                == 0
        {
            ret = picoquic_stop_sending(cnx, stream_id, local_stream_error as uint64_t);
            if ret == PICOQUIC_ERROR_STREAM_ALREADY_CLOSED {
                ret = 0 as ::core::ffi::c_int;
            }
        }
        if ret == 0 as ::core::ffi::c_int
            && ((stream_id & 2 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
                as ::core::ffi::c_uint
                != 0
                || (stream_id & 1 as uint64_t == 0 as uint64_t) as ::core::ffi::c_int
                    as ::core::ffi::c_uint
                    != 0)
        {
            ret = picoquic_reset_stream(cnx, stream_id, local_stream_error as uint64_t);
            if ret == PICOQUIC_ERROR_STREAM_ALREADY_CLOSED {
                ret = 0 as ::core::ffi::c_int;
            }
        }
        (*stream).app_stream_ctx = NULL;
        (*stream).set_is_discarded(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_pad_to_target_length(
    mut bytes: *mut uint8_t,
    mut length: size_t,
    mut target: size_t,
) -> size_t {
    if length < target {
        memset(
            bytes.offset(length as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            target.wrapping_sub(length),
        );
        length = target;
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_pad_to_policy(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut length: size_t,
    mut max_length: uint32_t,
) -> size_t {
    let mut target: size_t = (*cnx).padding_minsize as size_t;
    if length > target && (*cnx).padding_multiple != 0 as uint32_t {
        let mut delta: uint32_t = length
            .wrapping_sub(target)
            .wrapping_rem((*cnx).padding_multiple as size_t)
            as uint32_t;
        if delta == 0 as uint32_t {
            target = length;
        } else {
            target = length
                .wrapping_add((*cnx).padding_multiple as size_t)
                .wrapping_sub(delta as size_t);
        }
    }
    if target > max_length as size_t {
        target = max_length as size_t;
    }
    return picoquic_pad_to_target_length(bytes, length, target);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_packet(
    mut quic: *mut picoquic_quic_t,
) -> *mut picoquic_packet_t {
    let mut packet: *mut picoquic_packet_t = (*quic).p_first_packet;
    if packet.is_null() {
        packet =
            malloc(::core::mem::size_of::<picoquic_packet_t>() as size_t) as *mut picoquic_packet_t;
        if !packet.is_null() {
            (*quic).nb_packets_allocated += 1;
            if (*quic).nb_packets_allocated > (*quic).nb_packets_allocated_max {
                (*quic).nb_packets_allocated_max = (*quic).nb_packets_allocated;
            }
        }
    } else {
        (*quic).p_first_packet = (*packet).packet_previous as *mut picoquic_packet_t;
        (*quic).nb_packets_in_pool -= 1;
    }
    if !packet.is_null() {
        memset(
            packet as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<picoquic_packet_t>() as size_t,
        );
    }
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_recycle_packet(
    mut quic: *mut picoquic_quic_t,
    mut packet: *mut picoquic_packet_t,
) {
    if !packet.is_null() {
        if (*quic).nb_packets_in_pool >= PICOQUIC_MAX_PACKETS_IN_POOL {
            free(packet as *mut ::core::ffi::c_void);
            (*quic).nb_packets_allocated -= 1;
        } else {
            memset(
                packet as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                186 as size_t,
            );
            (*packet).packet_previous = (*quic).p_first_packet as *mut st_picoquic_packet_t;
            (*quic).p_first_packet = packet;
            (*quic).nb_packets_in_pool += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_update_payload_length(
    mut bytes: *mut uint8_t,
    mut pnum_index: size_t,
    mut header_length: size_t,
    mut packet_length: size_t,
) {
    if *bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 0x80 as ::core::ffi::c_int
        != 0 as ::core::ffi::c_int
        && header_length > 6 as size_t
        && packet_length > header_length
        && packet_length < 0x4000 as size_t
    {
        picoquic_varint_encode_16(
            bytes
                .offset(pnum_index as isize)
                .offset(-(2 as ::core::ffi::c_int as isize)),
            packet_length.wrapping_sub(header_length) as uint16_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_long_packet_type(
    mut pt: picoquic_packet_type_enum,
    mut version_index: ::core::ffi::c_int,
) -> uint8_t {
    let mut flags: uint8_t = 0xff as uint8_t;
    if version_index < 0 as ::core::ffi::c_int {
        version_index = 0 as ::core::ffi::c_int;
    }
    match (*(&raw const picoquic_supported_versions as *const picoquic_version_parameters_t)
        .offset(version_index as isize))
    .packet_type_version
    {
        1 => match pt as ::core::ffi::c_uint {
            2 => {
                flags = 0xc3 as uint8_t;
            }
            5 => {
                flags = 0xd3 as uint8_t;
            }
            4 => {
                flags = 0xe3 as uint8_t;
            }
            3 => {
                flags = 0xf0 as uint8_t;
            }
            _ => {}
        },
        1798521807 => match pt as ::core::ffi::c_uint {
            2 => {
                flags = 0xd3 as uint8_t;
            }
            5 => {
                flags = 0xe3 as uint8_t;
            }
            4 => {
                flags = 0xf3 as uint8_t;
            }
            3 => {
                flags = 0xc0 as uint8_t;
            }
            _ => {}
        },
        _ => {}
    }
    return flags;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_long_header(
    mut packet_type: picoquic_packet_type_enum,
    mut dest_cnx_id: *mut picoquic_connection_id_t,
    mut srce_cnx_id: *mut picoquic_connection_id_t,
    mut do_grease_quic_bit: ::core::ffi::c_int,
    mut version: uint32_t,
    mut version_index: ::core::ffi::c_int,
    mut sequence_number: uint64_t,
    mut retry_token_length: size_t,
    mut retry_token: *mut uint8_t,
    mut bytes: *mut uint8_t,
    mut pn_offset: *mut size_t,
    mut pn_length: *mut size_t,
) -> size_t {
    let mut length: size_t = 0 as size_t;
    *bytes.offset(0 as ::core::ffi::c_int as isize) =
        picoquic_create_long_packet_type(packet_type, version_index);
    if do_grease_quic_bit != 0 {
        let ref mut c2rust_fresh20 = *bytes.offset(0 as ::core::ffi::c_int as isize);
        *c2rust_fresh20 =
            (*c2rust_fresh20 as ::core::ffi::c_int & 0xbf as ::core::ffi::c_int) as uint8_t;
    }
    length = 1 as size_t;
    picoformat_32(bytes.offset(length as isize) as *mut uint8_t, version);
    length = length.wrapping_add(4 as size_t);
    let c2rust_fresh21 = length;
    length = length.wrapping_add(1);
    *bytes.offset(c2rust_fresh21 as isize) = (*dest_cnx_id).id_len;
    length = length.wrapping_add(picoquic_format_connection_id(
        bytes.offset(length as isize) as *mut uint8_t,
        (PICOQUIC_MAX_PACKET_SIZE as size_t).wrapping_sub(length),
        *dest_cnx_id,
    ) as size_t);
    let c2rust_fresh22 = length;
    length = length.wrapping_add(1);
    *bytes.offset(c2rust_fresh22 as isize) = (*srce_cnx_id).id_len;
    length = length.wrapping_add(picoquic_format_connection_id(
        bytes.offset(length as isize) as *mut uint8_t,
        (PICOQUIC_MAX_PACKET_SIZE as size_t).wrapping_sub(length),
        *srce_cnx_id,
    ) as size_t);
    if packet_type as ::core::ffi::c_uint
        == picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        length = length.wrapping_add(picoquic_varint_encode(
            bytes.offset(length as isize) as *mut uint8_t,
            (PICOQUIC_MAX_PACKET_SIZE as size_t).wrapping_sub(length),
            retry_token_length as uint64_t,
        ));
        if retry_token_length > 0 as size_t {
            memcpy(
                bytes.offset(length as isize) as *mut uint8_t as *mut ::core::ffi::c_void,
                retry_token as *const ::core::ffi::c_void,
                retry_token_length,
            );
            length = length.wrapping_add(retry_token_length);
        }
    }
    if packet_type as ::core::ffi::c_uint
        == picoquic_packet_retry as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        *pn_offset = 0 as size_t;
        *pn_length = 0 as size_t;
    } else {
        let c2rust_fresh23 = length;
        length = length.wrapping_add(1);
        *bytes.offset(c2rust_fresh23 as isize) = 0 as uint8_t;
        let c2rust_fresh24 = length;
        length = length.wrapping_add(1);
        *bytes.offset(c2rust_fresh24 as isize) = 0 as uint8_t;
        *pn_offset = length;
        *pn_length = 4 as size_t;
        picoformat_32(
            bytes.offset(length as isize) as *mut uint8_t,
            sequence_number as uint32_t,
        );
        length = length.wrapping_add(4 as size_t);
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_packet_header(
    mut cnx: *mut picoquic_cnx_t,
    mut packet_type: picoquic_packet_type_enum,
    mut sequence_number: uint64_t,
    mut path_x: *mut picoquic_path_t,
    mut header_length: size_t,
    mut bytes: *mut uint8_t,
    mut pn_offset: *mut size_t,
    mut pn_length: *mut size_t,
) -> size_t {
    let mut length: size_t = 0 as size_t;
    if packet_type as ::core::ffi::c_uint
        == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut K: uint8_t = (if (*cnx).key_phase_enc() as ::core::ffi::c_int != 0 {
            0x4 as ::core::ffi::c_int
        } else {
            0 as ::core::ffi::c_int
        }) as uint8_t;
        let mut C: uint8_t = 0x40 as uint8_t;
        let mut pn_l: size_t = 4 as size_t;
        if (*cnx).do_grease_quic_bit() != 0 {
            C = (C as ::core::ffi::c_int
                & picoquic_public_random_64() as uint8_t as ::core::ffi::c_int)
                as uint8_t;
            (*cnx).set_quic_bit_greased(
                (*cnx).quic_bit_greased()
                    | (C as ::core::ffi::c_int == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_uint,
            );
        }
        length = 0 as size_t;
        let c2rust_fresh18 = length;
        length = length.wrapping_add(1);
        *bytes.offset(c2rust_fresh18 as isize) = (K as ::core::ffi::c_int
            | C as ::core::ffi::c_int
            | (*(&raw mut picoquic_spin_function_table as *mut picoquic_spinbit_def_t)
                .offset((*cnx).spin_policy as isize))
            .spinbit_outgoing
            .expect("non-null function pointer")(cnx as *mut picoquic_cnx_t)
                as ::core::ffi::c_int) as uint8_t;
        length = length.wrapping_add(picoquic_format_connection_id(
            bytes.offset(length as isize) as *mut uint8_t,
            (PICOQUIC_MAX_PACKET_SIZE as size_t).wrapping_sub(length),
            if (*path_x).is_probing_nat() as ::core::ffi::c_int != 0
                && !(*path_x).p_remote_nat_cnxid.is_null()
            {
                (*(*path_x).p_remote_nat_cnxid).cnx_id
            } else {
                (*(*path_x).p_remote_cnxid).cnx_id
            },
        ) as size_t);
        *pn_offset = length;
        if header_length > length && header_length < length.wrapping_add(4 as size_t) {
            pn_l = header_length.wrapping_sub(length);
        }
        *pn_length = pn_l;
        let ref mut c2rust_fresh19 = *bytes.offset(0 as ::core::ffi::c_int as isize);
        *c2rust_fresh19 = (*c2rust_fresh19 as size_t | pn_l.wrapping_sub(1 as size_t)) as uint8_t;
        match pn_l {
            1 => {
                *bytes.offset(length as isize) = sequence_number as uint8_t;
            }
            2 => {
                picoformat_16(
                    bytes.offset(length as isize) as *mut uint8_t,
                    sequence_number as uint16_t,
                );
            }
            3 => {
                picoformat_24(
                    bytes.offset(length as isize) as *mut uint8_t,
                    sequence_number as uint32_t,
                );
            }
            _ => {
                picoformat_32(
                    bytes.offset(length as isize) as *mut uint8_t,
                    sequence_number as uint32_t,
                );
            }
        }
        length = length.wrapping_add(pn_l);
    } else {
        let mut dest_cnx_id: *mut picoquic_connection_id_t = if (*cnx).client_mode()
            as ::core::ffi::c_int
            != 0
            && (packet_type as ::core::ffi::c_uint
                == picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint
                || packet_type as ::core::ffi::c_uint
                    == picoquic_packet_0rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint)
            && picoquic_is_connection_id_null(&raw mut (*(*path_x).p_remote_cnxid).cnx_id) != 0
        {
            &raw mut (*cnx).initial_cnxid
        } else {
            &raw mut (*(*path_x).p_remote_cnxid).cnx_id
        };
        let mut srce_cnx_id: *mut picoquic_connection_id_t =
            &raw mut (*(*path_x).p_local_cnxid).cnx_id;
        let mut version: uint32_t = if ((*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_client_init as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*cnx).cnx_state as ::core::ffi::c_uint
                == picoquic_state_client_init_sent as ::core::ffi::c_int as ::core::ffi::c_uint)
            && packet_type as ::core::ffi::c_uint
                == picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*cnx).proposed_version
        } else {
            (*(&raw const picoquic_supported_versions as *const picoquic_version_parameters_t)
                .offset((*cnx).version_index as isize))
            .version
        };
        length = picoquic_create_long_header(
            packet_type,
            dest_cnx_id,
            srce_cnx_id,
            (*cnx).do_grease_quic_bit() as ::core::ffi::c_int,
            version,
            (*cnx).version_index,
            sequence_number,
            (*cnx).retry_token_length as size_t,
            (*cnx).retry_token,
            bytes,
            pn_offset,
            pn_length,
        );
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_predict_packet_header_length(
    mut cnx: *mut picoquic_cnx_t,
    mut packet_type: picoquic_packet_type_enum,
    mut pkt_ctx: *mut picoquic_packet_context_t,
) -> size_t {
    let mut header_length: uint32_t = 0 as uint32_t;
    if cnx.is_null() {
        return 0 as size_t;
    }
    if packet_type as ::core::ffi::c_uint
        == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut pn_l: uint8_t = 4 as uint8_t;
        let mut delta: int64_t = (*pkt_ctx).send_sequence as int64_t;
        if !(*pkt_ctx).pending_first.is_null() {
            delta = (delta as uint64_t).wrapping_sub((*(*pkt_ctx).pending_first).sequence_number)
                as int64_t as int64_t;
        }
        if delta < 262144 as int64_t {
            pn_l = 3 as uint8_t;
            if (*pkt_ctx).send_sequence < 1024 as uint64_t {
                pn_l = 2 as uint8_t;
                if (*pkt_ctx).send_sequence < 16 as uint64_t {
                    pn_l = 1 as uint8_t;
                }
            }
        }
        header_length = (1 as ::core::ffi::c_int
            + (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid)
                .cnx_id
                .id_len as ::core::ffi::c_int
            + pn_l as ::core::ffi::c_int) as uint32_t;
    } else {
        header_length = (1 as ::core::ffi::c_int
            + 4 as ::core::ffi::c_int
            + 2 as ::core::ffi::c_int) as uint32_t;
        if (*cnx).client_mode() as ::core::ffi::c_int != 0
            && (packet_type as ::core::ffi::c_uint
                == picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint
                || packet_type as ::core::ffi::c_uint
                    == picoquic_packet_0rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint)
            && picoquic_is_connection_id_null(
                &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid)
                    .cnx_id,
            ) != 0
        {
            header_length = header_length.wrapping_add((*cnx).initial_cnxid.id_len as uint32_t);
        } else {
            header_length = header_length.wrapping_add(
                (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid)
                    .cnx_id
                    .id_len as uint32_t,
            );
        }
        header_length = header_length.wrapping_add(
            (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_local_cnxid)
                .cnx_id
                .id_len as uint32_t,
        );
        header_length = header_length
            .wrapping_add((2 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint32_t);
        if packet_type as ::core::ffi::c_uint
            == picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut useless: [uint8_t; 16] = [0; 16];
            header_length = header_length.wrapping_add(picoquic_varint_encode(
                &raw mut useless as *mut uint8_t,
                16 as size_t,
                (*cnx).retry_token_length as uint64_t,
            ) as uint32_t);
            header_length = header_length.wrapping_add((*cnx).retry_token_length as uint32_t);
        }
    }
    return header_length as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_checksum_length(
    mut cnx: *mut picoquic_cnx_t,
    mut epoch: picoquic_epoch_enum,
) -> size_t {
    let mut ret: size_t = 16 as size_t;
    if !(*cnx).crypto_context[epoch as usize].aead_encrypt.is_null() {
        ret = picoquic_aead_get_checksum_length((*cnx).crypto_context[epoch as usize].aead_encrypt);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_protect_packet_header(
    mut send_buffer: *mut uint8_t,
    mut pn_offset: size_t,
    mut first_mask: uint8_t,
    mut pn_enc: *mut ::core::ffi::c_void,
) {
    let mut sample_offset: size_t = pn_offset.wrapping_add(4 as size_t);
    if pn_offset < sample_offset {
        let mut mask_bytes: [uint8_t; 5] = [
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ];
        let mut pn_l: uint8_t = 0;
        picoquic_pn_encrypt(
            pn_enc,
            send_buffer.offset(sample_offset as isize) as *const ::core::ffi::c_void,
            &raw mut mask_bytes as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw mut mask_bytes as *mut uint8_t as *const ::core::ffi::c_void,
            5 as size_t,
        );
        pn_l = ((*send_buffer.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 3 as ::core::ffi::c_int)
            + 1 as ::core::ffi::c_int) as uint8_t;
        let ref mut c2rust_fresh16 = *send_buffer.offset(0 as ::core::ffi::c_int as isize);
        *c2rust_fresh16 = (*c2rust_fresh16 as ::core::ffi::c_int
            ^ mask_bytes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                & first_mask as ::core::ffi::c_int) as uint8_t;
        let mut i: uint8_t = 0 as uint8_t;
        while (i as ::core::ffi::c_int) < pn_l as ::core::ffi::c_int {
            let ref mut c2rust_fresh17 =
                *send_buffer.offset(pn_offset.wrapping_add(i as size_t) as isize);
            *c2rust_fresh17 = (*c2rust_fresh17 as ::core::ffi::c_int
                ^ mask_bytes[(i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as usize]
                    as ::core::ffi::c_int) as uint8_t;
            i = i.wrapping_add(1);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_protect_packet(
    mut cnx: *mut picoquic_cnx_t,
    mut ptype: picoquic_packet_type_enum,
    mut bytes: *mut uint8_t,
    mut sequence_number: uint64_t,
    mut length: size_t,
    mut header_length: size_t,
    mut send_buffer: *mut uint8_t,
    mut send_buffer_max: size_t,
    mut aead_context: *mut ::core::ffi::c_void,
    mut pn_enc: *mut ::core::ffi::c_void,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) -> size_t {
    let mut send_length: size_t = 0;
    let mut h_length: size_t = 0;
    let mut pn_offset: size_t = 0 as size_t;
    let mut pn_length: size_t = 0 as size_t;
    let mut aead_checksum_length: size_t = picoquic_aead_get_checksum_length(aead_context);
    let mut first_mask: uint8_t = 0xf as uint8_t;
    h_length = picoquic_create_packet_header(
        cnx,
        ptype,
        sequence_number,
        path_x,
        header_length,
        send_buffer,
        &raw mut pn_offset,
        &raw mut pn_length,
    );
    if h_length != header_length {
        picoquic_log_app_message(
            cnx as *mut picoquic_cnx_t,
            b"BUFFER OVERFLOW? Packet header prediction fails, %zu instead of %zu\n\0".as_ptr()
                as *const ::core::ffi::c_char,
            h_length,
            header_length,
        );
    }
    if ptype as ::core::ffi::c_uint
        == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*cnx).is_loss_bit_enabled_outgoing() != 0 {
            first_mask = 0x7 as uint8_t;
            (*path_x).q_square = (*path_x).q_square.wrapping_add(1);
            if (*path_x).q_square & PICOQUIC_LOSS_BIT_Q_HALF_PERIOD as uint64_t != 0 as uint64_t {
                let ref mut c2rust_fresh14 = *send_buffer.offset(0 as ::core::ffi::c_int as isize);
                *c2rust_fresh14 =
                    (*c2rust_fresh14 as ::core::ffi::c_int | 0x10 as ::core::ffi::c_int) as uint8_t;
            }
            if (*path_x).nb_losses_found > (*path_x).nb_losses_reported {
                let ref mut c2rust_fresh15 = *send_buffer.offset(0 as ::core::ffi::c_int as isize);
                *c2rust_fresh15 =
                    (*c2rust_fresh15 as ::core::ffi::c_int | 0x8 as ::core::ffi::c_int) as uint8_t;
                (*path_x).nb_losses_reported = (*path_x).nb_losses_reported.wrapping_add(1);
            }
        } else {
            first_mask = 0x1f as uint8_t;
        }
    }
    picoquic_update_payload_length(
        send_buffer,
        pn_offset,
        h_length.wrapping_sub(pn_length),
        length.wrapping_add(aead_checksum_length),
    );
    if (*(*cnx).quic).fuzz_fn.is_some() {
        if h_length == header_length {
            memcpy(
                bytes as *mut ::core::ffi::c_void,
                send_buffer as *const ::core::ffi::c_void,
                header_length,
            );
        }
        length = (*(*cnx).quic).fuzz_fn.expect("non-null function pointer")(
            (*(*cnx).quic).fuzz_ctx,
            cnx as *mut picoquic_cnx_t,
            bytes,
            send_buffer_max.wrapping_sub(aead_checksum_length),
            length,
            header_length,
        ) as size_t;
        if h_length == header_length {
            memcpy(
                send_buffer as *mut ::core::ffi::c_void,
                bytes as *const ::core::ffi::c_void,
                header_length,
            );
        }
    }
    if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
        && ptype as ::core::ffi::c_uint
            == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        send_length = picoquic_aead_encrypt_mp(
            send_buffer.offset(h_length as isize),
            bytes.offset(header_length as isize),
            length.wrapping_sub(header_length),
            (*path_x).unique_path_id,
            sequence_number,
            send_buffer,
            h_length,
            aead_context,
        );
    } else {
        send_length = picoquic_aead_encrypt_generic(
            send_buffer.offset(h_length as isize),
            bytes.offset(header_length as isize),
            length.wrapping_sub(header_length),
            sequence_number,
            send_buffer,
            h_length,
            aead_context,
        );
    }
    send_length = send_length.wrapping_add(h_length);
    picoquic_log_outgoing_packet(
        cnx,
        path_x,
        bytes,
        sequence_number,
        pn_length,
        length,
        send_buffer,
        send_length,
        current_time,
    );
    picoquic_protect_packet_header(send_buffer, pn_offset, first_mask, pn_enc);
    return send_length;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_for_retransmit(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut packet: *mut picoquic_packet_t,
    mut length: size_t,
    mut current_time: uint64_t,
) {
    let mut pkt_ctx: *mut picoquic_packet_context_t =
        ::core::ptr::null_mut::<picoquic_packet_context_t>();
    if (*packet).ptype as ::core::ffi::c_uint
        == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
    {
        pkt_ctx = &raw mut (*path_x).pkt_ctx;
    } else {
        pkt_ctx = (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t)
            .offset((*packet).pc as isize) as *mut picoquic_packet_context_t;
    }
    (*packet).packet_next = ::core::ptr::null_mut::<st_picoquic_packet_t>();
    if (*pkt_ctx).pending_last.is_null() {
        (*packet).packet_previous = ::core::ptr::null_mut::<st_picoquic_packet_t>();
        (*pkt_ctx).pending_first = packet;
    } else {
        (*packet).packet_previous = (*pkt_ctx).pending_last as *mut st_picoquic_packet_t;
        (*(*packet).packet_previous).packet_next = packet as *mut st_picoquic_packet_t;
    }
    (*pkt_ctx).pending_last = packet;
    (*packet).set_is_queued_for_retransmit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if (*packet).is_ack_trap() == 0 {
        (*path_x).bytes_in_transit = ((*path_x).bytes_in_transit as ::core::ffi::c_ulong)
            .wrapping_add(length as ::core::ffi::c_ulong)
            as uint64_t as uint64_t;
        (*path_x).set_is_cc_data_updated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        picoquic_update_pacing_after_send(path_x, length, current_time);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_dequeue_retransmit_packet(
    mut cnx: *mut picoquic_cnx_t,
    mut pkt_ctx: *mut picoquic_packet_context_t,
    mut p: *mut picoquic_packet_t,
    mut should_free: ::core::ffi::c_int,
    mut add_to_data_repeat_queue: ::core::ffi::c_int,
) -> *mut picoquic_packet_t {
    let mut dequeued_length: size_t = (*p).length.wrapping_add((*p).checksum_overhead);
    if (*p).is_queued_for_retransmit() != 0 {
        if (*p).packet_next.is_null() {
            (*pkt_ctx).pending_last = (*p).packet_previous as *mut picoquic_packet_t;
        } else {
            (*(*p).packet_next).packet_previous = (*p).packet_previous;
        }
        if (*p).packet_previous.is_null() {
            (*pkt_ctx).pending_first = (*p).packet_next as *mut picoquic_packet_t;
        } else {
            (*(*p).packet_previous).packet_next = (*p).packet_next;
        }
        (*p).set_is_queued_for_retransmit(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if !(*p).send_path.is_null() && (*p).is_ack_trap() == 0 {
        if (*(*p).send_path).bytes_in_transit > dequeued_length as uint64_t {
            (*(*p).send_path).bytes_in_transit = ((*(*p).send_path).bytes_in_transit
                as ::core::ffi::c_ulong)
                .wrapping_sub(dequeued_length as ::core::ffi::c_ulong)
                as uint64_t as uint64_t;
        } else {
            (*(*p).send_path).bytes_in_transit = 0 as uint64_t;
        }
        (*(*p).send_path).set_is_cc_data_updated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if (*pkt_ctx).preemptive_repeat_ptr == p {
        (*pkt_ctx).preemptive_repeat_ptr = (*p).packet_next as *mut picoquic_packet_t;
    }
    if should_free != 0 || (*p).is_ack_trap() as ::core::ffi::c_int != 0 {
        if add_to_data_repeat_queue != 0 {
            picoquic_queue_data_repeat_packet(cnx, p);
        } else {
            picoquic_recycle_packet((*cnx).quic, p);
            p = ::core::ptr::null_mut::<picoquic_packet_t>();
        }
    } else {
        (*p).packet_previous = ::core::ptr::null_mut::<st_picoquic_packet_t>();
        if (*pkt_ctx).retransmitted_oldest.is_null() {
            (*pkt_ctx).retransmitted_newest = p;
            (*pkt_ctx).retransmitted_oldest = p;
            (*p).packet_next = ::core::ptr::null_mut::<st_picoquic_packet_t>();
        } else {
            (*(*pkt_ctx).retransmitted_newest).packet_previous = p as *mut st_picoquic_packet_t;
            (*p).packet_next = (*pkt_ctx).retransmitted_newest as *mut st_picoquic_packet_t;
            (*pkt_ctx).retransmitted_newest = p;
        }
        (*pkt_ctx).retransmitted_queue_size = (*pkt_ctx)
            .retransmitted_queue_size
            .wrapping_add(1 as uint64_t);
        (*p).set_is_queued_for_spurious_detection(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if add_to_data_repeat_queue != 0 {
            picoquic_queue_data_repeat_packet(cnx, p);
        }
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_dequeue_retransmitted_packet(
    mut cnx: *mut picoquic_cnx_t,
    mut pkt_ctx: *mut picoquic_packet_context_t,
    mut p: *mut picoquic_packet_t,
) {
    (*pkt_ctx).retransmitted_queue_size = (*pkt_ctx)
        .retransmitted_queue_size
        .wrapping_sub(1 as uint64_t);
    if (*p).packet_previous.is_null() {
        (*pkt_ctx).retransmitted_newest = (*p).packet_next as *mut picoquic_packet_t;
    } else {
        (*(*p).packet_previous).packet_next = (*p).packet_next;
    }
    if (*p).packet_next.is_null() {
        (*pkt_ctx).retransmitted_oldest = (*p).packet_previous as *mut picoquic_packet_t;
    } else {
        (*(*p).packet_next).packet_previous = (*p).packet_previous;
    }
    (*p).set_is_queued_for_spurious_detection(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if (*p).is_queued_for_data_repeat() == 0 {
        picoquic_recycle_packet((*cnx).quic, p);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_insert_hole_in_send_sequence_if_needed(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut pkt_ctx: *mut picoquic_packet_context_t,
    mut current_time: uint64_t,
    mut next_wake_time: *mut uint64_t,
) {
    if (*(*cnx).quic).sequence_hole_pseudo_period == 0 as uint32_t {
        (*pkt_ctx).next_sequence_hole = UINT64_MAX as uint64_t;
    } else if (*cnx).cnx_state as ::core::ffi::c_uint
        == picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
        && !(*pkt_ctx).pending_last.is_null()
        && (*pkt_ctx).send_sequence >= (*pkt_ctx).next_sequence_hole
    {
        if (*pkt_ctx).next_sequence_hole != 0 as uint64_t
            && (*(*pkt_ctx).pending_last).is_ack_trap() == 0
        {
            let mut packet: *mut picoquic_packet_t = picoquic_create_packet((*cnx).quic);
            if !packet.is_null() {
                (*packet).set_is_ack_trap(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*packet).pc = picoquic_packet_context_application;
                (*packet).ptype = picoquic_packet_1rtt_protected;
                (*packet).send_time = current_time;
                (*packet).send_path = ::core::ptr::null_mut::<st_picoquic_path_t>();
                let c2rust_fresh34 = (*pkt_ctx).send_sequence;
                (*pkt_ctx).send_sequence = (*pkt_ctx).send_sequence.wrapping_add(1);
                (*packet).sequence_number = c2rust_fresh34;
                picoquic_queue_for_retransmit(cnx, path_x, packet, 0 as size_t, current_time);
                *next_wake_time = current_time;
                (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
                (*(*cnx).quic).wake_line = 1082 as ::core::ffi::c_int;
                (*path_x).q_square = (*path_x).q_square.wrapping_add(1);
                (*cnx).nb_packet_holes_inserted = (*cnx).nb_packet_holes_inserted.wrapping_add(1);
            }
        }
        (*pkt_ctx).next_sequence_hole = (*pkt_ctx)
            .send_sequence
            .wrapping_add(3 as uint64_t)
            .wrapping_add(picoquic_public_uniform_random(
                ((*(*cnx).quic).sequence_hole_pseudo_period as uint64_t)
                    << (*cnx).nb_packet_holes_inserted,
            ));
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_finalize_and_protect_packet(
    mut cnx: *mut picoquic_cnx_t,
    mut packet: *mut picoquic_packet_t,
    mut ret: ::core::ffi::c_int,
    mut length: size_t,
    mut header_length: size_t,
    mut checksum_overhead: size_t,
    mut send_length: *mut size_t,
    mut send_buffer: *mut uint8_t,
    mut send_buffer_max: size_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
) {
    if length != 0 as size_t && length < header_length {
        length = 0 as size_t;
    }
    if ret == 0 as ::core::ffi::c_int && length > 0 as size_t {
        (*packet).length = length;
        if (*packet).ptype as ::core::ffi::c_uint
            == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
        {
            let c2rust_fresh12 = (*path_x).pkt_ctx.send_sequence;
            (*path_x).pkt_ctx.send_sequence = (*path_x).pkt_ctx.send_sequence.wrapping_add(1);
            (*packet).sequence_number = c2rust_fresh12;
        } else {
            let c2rust_fresh13 = (*cnx).pkt_ctx[(*packet).pc as usize].send_sequence;
            (*cnx).pkt_ctx[(*packet).pc as usize].send_sequence = (*cnx).pkt_ctx
                [(*packet).pc as usize]
                .send_sequence
                .wrapping_add(1);
            (*packet).sequence_number = c2rust_fresh13;
        }
        (*path_x).latest_sent_time = current_time;
        (*path_x).set_path_cid_rotated(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*packet).delivered_prior = (*path_x).delivered_last;
        (*packet).delivered_time_prior = (*path_x).delivered_time_last;
        (*packet).delivered_sent_prior = (*path_x).delivered_sent_last;
        (*packet).lost_prior = (*path_x).total_bytes_lost;
        (*packet).inflight_prior = (*path_x).bytes_in_transit;
        (*packet).set_delivered_app_limited(
            (((*cnx).cnx_state as ::core::ffi::c_uint)
                < picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*path_x).delivered_limited_index != 0 as uint64_t)
                as ::core::ffi::c_int as ::core::ffi::c_uint as ::core::ffi::c_uint,
        );
        if (*path_x).bytes_in_transit >= (*path_x).cwin
            && (*cnx).cnx_state as ::core::ffi::c_uint
                == picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*packet).set_sent_cwin_limited(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        match (*packet).ptype as ::core::ffi::c_uint {
            1 => {}
            2 => {
                length = picoquic_protect_packet(
                    cnx,
                    (*packet).ptype,
                    &raw mut (*packet).bytes as *mut uint8_t,
                    (*packet).sequence_number,
                    length,
                    header_length,
                    send_buffer,
                    send_buffer_max,
                    (*cnx).crypto_context[picoquic_epoch_initial as ::core::ffi::c_int as usize]
                        .aead_encrypt,
                    (*cnx).crypto_context[picoquic_epoch_initial as ::core::ffi::c_int as usize]
                        .pn_enc,
                    path_x,
                    current_time,
                );
            }
            4 => {
                length = picoquic_protect_packet(
                    cnx,
                    (*packet).ptype,
                    &raw mut (*packet).bytes as *mut uint8_t,
                    (*packet).sequence_number,
                    length,
                    header_length,
                    send_buffer,
                    send_buffer_max,
                    (*cnx).crypto_context[picoquic_epoch_handshake as ::core::ffi::c_int as usize]
                        .aead_encrypt,
                    (*cnx).crypto_context[picoquic_epoch_handshake as ::core::ffi::c_int as usize]
                        .pn_enc,
                    path_x,
                    current_time,
                );
            }
            3 => {
                length = picoquic_protect_packet(
                    cnx,
                    (*packet).ptype,
                    &raw mut (*packet).bytes as *mut uint8_t,
                    (*packet).sequence_number,
                    length,
                    header_length,
                    send_buffer,
                    send_buffer_max,
                    (*cnx).crypto_context[picoquic_epoch_0rtt as ::core::ffi::c_int as usize]
                        .aead_encrypt,
                    (*cnx).crypto_context[picoquic_epoch_0rtt as ::core::ffi::c_int as usize]
                        .pn_enc,
                    path_x,
                    current_time,
                );
            }
            5 => {
                length = picoquic_protect_packet(
                    cnx,
                    (*packet).ptype,
                    &raw mut (*packet).bytes as *mut uint8_t,
                    (*packet).sequence_number,
                    length,
                    header_length,
                    send_buffer,
                    send_buffer_max,
                    (*cnx).crypto_context[picoquic_epoch_0rtt as ::core::ffi::c_int as usize]
                        .aead_encrypt,
                    (*cnx).crypto_context[picoquic_epoch_0rtt as ::core::ffi::c_int as usize]
                        .pn_enc,
                    path_x,
                    current_time,
                );
            }
            6 => {
                length = picoquic_protect_packet(
                    cnx,
                    (*packet).ptype,
                    &raw mut (*packet).bytes as *mut uint8_t,
                    (*packet).sequence_number,
                    length,
                    header_length,
                    send_buffer,
                    send_buffer_max,
                    (*cnx).crypto_context[picoquic_epoch_1rtt as ::core::ffi::c_int as usize]
                        .aead_encrypt,
                    (*cnx).crypto_context[picoquic_epoch_1rtt as ::core::ffi::c_int as usize]
                        .pn_enc,
                    path_x,
                    current_time,
                );
            }
            _ => {
                length = 0 as size_t;
            }
        }
        *send_length = length;
        if length > 0 as size_t {
            (*packet).checksum_overhead = checksum_overhead;
            picoquic_queue_for_retransmit(cnx, path_x, packet, length, current_time);
            (*path_x).last_sent_time = current_time;
            (*path_x).bytes_sent = ((*path_x).bytes_sent as ::core::ffi::c_ulong)
                .wrapping_add(length as ::core::ffi::c_ulong)
                as uint64_t as uint64_t;
        } else {
            *send_length = 0 as size_t;
        }
    } else {
        *send_length = 0 as size_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_pkt_ctx_backlog_empty(
    mut pkt_ctx: *mut picoquic_packet_context_t,
) -> ::core::ffi::c_int {
    let mut backlog_empty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut p: *mut picoquic_packet_t = (*pkt_ctx).pending_first;
    while !p.is_null() && backlog_empty == 1 as ::core::ffi::c_int {
        let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut frame_is_pure_ack: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut frame_length: size_t = 0 as size_t;
        let mut byte_index: size_t = 0 as size_t;
        byte_index = (*p).offset;
        if (*p).is_ack_trap() == 0 && (*p).is_multipath_probe() == 0 && (*p).is_mtu_probe() == 0 {
            while ret == 0 as ::core::ffi::c_int && byte_index < (*p).length {
                ret = picoquic_skip_frame(
                    (&raw mut (*p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    (*p).length.wrapping_sub((*p).offset),
                    &raw mut frame_length,
                    &raw mut frame_is_pure_ack,
                );
                if frame_is_pure_ack == 0 {
                    backlog_empty = 0 as ::core::ffi::c_int;
                    break;
                } else {
                    byte_index = byte_index.wrapping_add(frame_length);
                }
            }
        }
        p = (*p).packet_next as *mut picoquic_packet_t;
    }
    return backlog_empty;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_cnx_backlog_empty(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut backlog_empty: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if ((*cnx).cnx_state as ::core::ffi::c_uint)
        < picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        backlog_empty = (picoquic_is_pkt_ctx_backlog_empty(
            (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t)
                .offset(picoquic_packet_context_initial as ::core::ffi::c_int as isize)
                as *mut picoquic_packet_context_t,
        ) != 0
            && picoquic_is_pkt_ctx_backlog_empty(
                (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t)
                    .offset(picoquic_packet_context_handshake as ::core::ffi::c_int as isize)
                    as *mut picoquic_packet_context_t,
            ) != 0) as ::core::ffi::c_int;
    }
    if (*cnx).is_multipath_enabled() != 0 {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while backlog_empty != 0 && i < (*cnx).nb_paths {
            backlog_empty &= picoquic_is_pkt_ctx_backlog_empty(
                &raw mut (**(*cnx).path.offset(i as isize)).pkt_ctx,
            );
            i += 1;
        }
    } else if backlog_empty != 0 {
        backlog_empty = picoquic_is_pkt_ctx_backlog_empty(
            (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t)
                .offset(picoquic_packet_context_application as ::core::ffi::c_int as isize)
                as *mut picoquic_packet_context_t,
        );
    }
    return backlog_empty;
}
unsafe extern "C" fn picoquic_preemptive_retransmit_packet(
    mut old_p: *mut picoquic_packet_t,
    mut cnx: *mut picoquic_cnx_t,
    mut new_bytes: *mut uint8_t,
    mut send_buffer_max_minus_checksum: size_t,
    mut length: *mut size_t,
    mut has_data: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut frame_is_pure_ack: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut frame_length: size_t = 0 as size_t;
    let mut byte_index: size_t = 0 as size_t;
    let mut write_index: size_t = 0 as size_t;
    let mut is_repeated: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut do_not_detect_spurious: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_preemptive_needed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut initial_length: size_t = *length;
    *has_data = 0 as ::core::ffi::c_int;
    if (*old_p).is_mtu_probe() == 0
        && (*old_p).is_ack_trap() == 0
        && (*old_p).is_multipath_probe() == 0
    {
        byte_index = (*old_p).offset;
        while ret == 0 as ::core::ffi::c_int && byte_index < (*old_p).length {
            ret = picoquic_skip_frame(
                (&raw mut (*old_p).bytes as *mut uint8_t).offset(byte_index as isize)
                    as *mut uint8_t,
                (*old_p).length.wrapping_sub(byte_index),
                &raw mut frame_length,
                &raw mut frame_is_pure_ack,
            );
            if ret == 0 as ::core::ffi::c_int && frame_is_pure_ack == 0 as ::core::ffi::c_int {
                ret = picoquic_check_frame_needs_repeat(
                    cnx,
                    (&raw mut (*old_p).bytes as *mut uint8_t).offset(byte_index as isize)
                        as *mut uint8_t,
                    frame_length,
                    (*old_p).ptype,
                    &raw mut frame_is_pure_ack,
                    &raw mut do_not_detect_spurious,
                    &raw mut is_preemptive_needed,
                );
            }
            if ret == 0 as ::core::ffi::c_int && frame_is_pure_ack == 0 {
                if (*old_p).bytes[byte_index as usize] as ::core::ffi::c_int
                    & !(picoquic_frame_type_stream_range_min as ::core::ffi::c_int
                        ^ picoquic_frame_type_stream_range_max as ::core::ffi::c_int)
                    == picoquic_frame_type_stream_range_min as ::core::ffi::c_int
                    && picoquic_is_stream_frame_unlimited(
                        (&raw mut (*old_p).bytes as *mut uint8_t).offset(byte_index as isize)
                            as *mut uint8_t,
                    ) != 0
                {
                    if write_index.wrapping_add(frame_length) < send_buffer_max_minus_checksum {
                        let mut pad_needed: size_t = send_buffer_max_minus_checksum
                            .wrapping_sub(write_index)
                            .wrapping_sub(frame_length);
                        memset(
                            new_bytes.offset(write_index as isize) as *mut uint8_t
                                as *mut ::core::ffi::c_void,
                            picoquic_frame_type_padding as ::core::ffi::c_int,
                            pad_needed,
                        );
                        *length = (*length).wrapping_add(pad_needed);
                        write_index = write_index.wrapping_add(pad_needed);
                    }
                }
                if write_index.wrapping_add(frame_length) <= send_buffer_max_minus_checksum {
                    memcpy(
                        new_bytes.offset(write_index as isize) as *mut uint8_t
                            as *mut ::core::ffi::c_void,
                        (&raw mut (*old_p).bytes as *mut uint8_t).offset(byte_index as isize)
                            as *mut uint8_t as *const ::core::ffi::c_void,
                        frame_length,
                    );
                    write_index = write_index.wrapping_add(frame_length);
                    *length = (*length).wrapping_add(frame_length);
                    *has_data = 1 as ::core::ffi::c_int;
                } else {
                    is_repeated = 0 as ::core::ffi::c_int;
                }
            }
            byte_index = byte_index.wrapping_add(frame_length);
        }
    }
    if *has_data != 0 {
        if is_preemptive_needed == 0 {
            *length = initial_length;
            *has_data = 0 as ::core::ffi::c_int;
            is_repeated = 0 as ::core::ffi::c_int;
        } else if is_repeated != 0 {
            (*old_p).set_was_preemptively_repeated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_preemptive_retransmit_in_context(
    mut cnx: *mut picoquic_cnx_t,
    mut pkt_ctx: *mut picoquic_packet_context_t,
    mut rtt: uint64_t,
    mut current_time: uint64_t,
    mut next_wake_time: *mut uint64_t,
    mut new_bytes: *mut uint8_t,
    mut send_buffer_max_minus_checksum: size_t,
    mut length: *mut size_t,
    mut has_data: *mut ::core::ffi::c_int,
    mut more_data: *mut ::core::ffi::c_int,
    mut test_only: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*cnx).latest_progress_time.wrapping_add(rtt) < current_time
        || (*cnx)
            .latest_receive_time
            .wrapping_add((2 as uint64_t).wrapping_mul(rtt))
            < current_time
    {
        return 0 as ::core::ffi::c_int;
    }
    if (*pkt_ctx).preemptive_repeat_ptr.is_null() {
        (*pkt_ctx).preemptive_repeat_ptr = (*pkt_ctx).pending_first;
    }
    while !(*pkt_ctx).preemptive_repeat_ptr.is_null() {
        if (*(*pkt_ctx).preemptive_repeat_ptr)
            .send_time
            .wrapping_add(rtt.wrapping_div(2 as uint64_t))
            >= current_time
        {
            break;
        }
        (*pkt_ctx).preemptive_repeat_ptr =
            (*(*pkt_ctx).preemptive_repeat_ptr).packet_next as *mut picoquic_packet_t;
    }
    while !(*pkt_ctx).preemptive_repeat_ptr.is_null() {
        let mut early_delay: uint64_t = (if rtt as ::core::ffi::c_ulonglong
            > (8 as ::core::ffi::c_ulonglong).wrapping_mul(PICOQUIC_ACK_DELAY_MAX)
        {
            rtt.wrapping_div(8 as uint64_t) as ::core::ffi::c_ulonglong
        } else {
            PICOQUIC_ACK_DELAY_MAX
        }) as uint64_t;
        let mut early_time: uint64_t = (*(*pkt_ctx).preemptive_repeat_ptr)
            .send_time
            .wrapping_add(early_delay);
        if (*(*pkt_ctx).preemptive_repeat_ptr).was_preemptively_repeated() == 0 {
            if early_time > current_time {
                if *next_wake_time > early_time {
                    *next_wake_time = early_time;
                    (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
                    (*(*cnx).quic).wake_line = 1377 as ::core::ffi::c_int;
                }
                break;
            } else if test_only != 0 {
                *more_data = 1 as ::core::ffi::c_int;
                break;
            } else {
                ret = picoquic_preemptive_retransmit_packet(
                    (*pkt_ctx).preemptive_repeat_ptr,
                    cnx,
                    new_bytes,
                    send_buffer_max_minus_checksum,
                    length,
                    has_data,
                );
                if ret != 0 as ::core::ffi::c_int {
                    break;
                }
            }
        }
        (*pkt_ctx).preemptive_repeat_ptr =
            (*(*pkt_ctx).preemptive_repeat_ptr).packet_next as *mut picoquic_packet_t;
        if !(*has_data != 0) {
            continue;
        }
        (*cnx).nb_preemptive_repeat = (*cnx).nb_preemptive_repeat.wrapping_add(1);
        if !(*pkt_ctx).preemptive_repeat_ptr.is_null() {
            *more_data = 1 as ::core::ffi::c_int;
        }
        break;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_preemptive_retransmit_as_needed(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut pc: picoquic_packet_context_enum,
    mut current_time: uint64_t,
    mut next_wake_time: *mut uint64_t,
    mut new_bytes: *mut uint8_t,
    mut send_buffer_max_minus_checksum: size_t,
    mut length: *mut size_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut has_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pkt_ctx: *mut picoquic_packet_context_t =
        ::core::ptr::null_mut::<picoquic_packet_context_t>();
    let mut rtt: uint64_t = (*path_x).smoothed_rtt;
    if pc as ::core::ffi::c_uint
        == picoquic_packet_context_application as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
    {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*cnx).nb_paths {
            pkt_ctx = &raw mut (**(*cnx).path.offset(i as isize)).pkt_ctx;
            ret = picoquic_preemptive_retransmit_in_context(
                cnx,
                pkt_ctx,
                rtt,
                current_time,
                next_wake_time,
                new_bytes,
                send_buffer_max_minus_checksum,
                length,
                &raw mut has_data,
                more_data,
                (is_pure_ack == NULL as *mut ::core::ffi::c_int) as ::core::ffi::c_int,
            );
            if ret != 0 as ::core::ffi::c_int || has_data != 0 as ::core::ffi::c_int {
                break;
            }
            i += 1;
        }
    } else {
        pkt_ctx = (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t).offset(pc as isize)
            as *mut picoquic_packet_context_t;
        ret = picoquic_preemptive_retransmit_in_context(
            cnx,
            pkt_ctx,
            rtt,
            current_time,
            next_wake_time,
            new_bytes,
            send_buffer_max_minus_checksum,
            length,
            &raw mut has_data,
            more_data,
            (is_pure_ack == NULL as *mut ::core::ffi::c_int) as ::core::ffi::c_int,
        );
    }
    if ret == 0 as ::core::ffi::c_int && !is_pure_ack.is_null() {
        *is_pure_ack &= (has_data == 0) as ::core::ffi::c_int;
    }
    return ret;
}
unsafe extern "C" fn picoquic_next_mtu_probe_length(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
) -> size_t {
    let mut probe_length: size_t = 0;
    if (*path_x).send_mtu_max_tried == 0 as size_t {
        if (*cnx).remote_parameters.max_packet_size > 0 as uint32_t {
            probe_length = (*cnx).remote_parameters.max_packet_size as size_t;
            if (*(*cnx).quic).mtu_max > 0 as uint32_t
                && probe_length as ::core::ffi::c_int as uint32_t
                    > (*(*cnx).quic).mtu_max.wrapping_sub(
                        (if (*(&raw mut (*path_x).peer_addr as *mut sockaddr)).sa_family
                            as ::core::ffi::c_int
                            == AF_INET6
                        {
                            48 as ::core::ffi::c_int
                        } else {
                            28 as ::core::ffi::c_int
                        }) as uint32_t,
                    )
            {
                probe_length = (*(*cnx).quic).mtu_max.wrapping_sub(
                    (if (*(&raw mut (*path_x).peer_addr as *mut sockaddr)).sa_family
                        as ::core::ffi::c_int
                        == AF_INET6
                    {
                        48 as ::core::ffi::c_int
                    } else {
                        28 as ::core::ffi::c_int
                    }) as uint32_t,
                ) as size_t;
            } else if probe_length > PICOQUIC_MAX_PACKET_SIZE as size_t {
                probe_length = PICOQUIC_MAX_PACKET_SIZE as size_t;
            }
            if probe_length < (*path_x).send_mtu {
                probe_length = (*path_x).send_mtu;
            }
        } else if (*(*cnx).quic).mtu_max > 0 as uint32_t {
            probe_length = (*(*cnx).quic).mtu_max.wrapping_sub(
                (if (*(&raw mut (*path_x).peer_addr as *mut sockaddr)).sa_family
                    as ::core::ffi::c_int
                    == AF_INET6
                {
                    48 as ::core::ffi::c_int
                } else {
                    28 as ::core::ffi::c_int
                }) as uint32_t,
            ) as size_t;
        } else {
            probe_length = PICOQUIC_PRACTICAL_MAX_MTU as size_t;
        }
    } else if (*path_x).send_mtu_max_tried > 1500 as size_t {
        probe_length = 1500 as size_t;
    } else if (*path_x).send_mtu_max_tried > 1400 as size_t {
        probe_length = 1400 as size_t;
    } else {
        probe_length = (*path_x)
            .send_mtu
            .wrapping_add((*path_x).send_mtu_max_tried)
            .wrapping_div(2 as size_t);
    }
    return probe_length;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_mtu_probe_needed(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
) -> picoquic_pmtu_discovery_status_enum {
    let mut ret: ::core::ffi::c_int = picoquic_pmtu_discovery_not_needed as ::core::ffi::c_int;
    if ((*cnx).cnx_state as ::core::ffi::c_uint
        == picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_client_ready_start as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_server_false_start as ::core::ffi::c_int as ::core::ffi::c_uint)
        && (*path_x).mtu_probe_sent() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*cnx).pmtud_policy as ::core::ffi::c_uint
            != picoquic_pmtud_blocked as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*path_x).send_mtu_max_tried == 0 as size_t
            || (*path_x).send_mtu_max_tried > 1400 as size_t
        {
            let mut next_probe: uint64_t = picoquic_next_mtu_probe_length(cnx, path_x) as uint64_t;
            if next_probe > (*path_x).send_mtu as uint64_t {
                if (*cnx).pmtud_policy as ::core::ffi::c_uint
                    == picoquic_pmtud_required as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    ret = picoquic_pmtu_discovery_required as ::core::ffi::c_int;
                } else {
                    let mut packets_to_send_before: uint64_t = (*cnx)
                        .nb_bytes_queued
                        .wrapping_div((*path_x).send_mtu as uint64_t);
                    let mut packets_to_send_after: uint64_t =
                        (*cnx).nb_bytes_queued.wrapping_div(next_probe);
                    let mut delta: uint64_t = packets_to_send_before
                        .wrapping_sub(packets_to_send_after)
                        .wrapping_mul(60 as uint64_t);
                    if delta > next_probe {
                        ret = picoquic_pmtu_discovery_required as ::core::ffi::c_int;
                    } else if (*cnx).pmtud_policy as ::core::ffi::c_uint
                        == picoquic_pmtud_basic as ::core::ffi::c_int as ::core::ffi::c_uint
                    {
                        ret = picoquic_pmtu_discovery_optional as ::core::ffi::c_int;
                    } else {
                        ret = picoquic_pmtu_discovery_not_needed as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    return ret as picoquic_pmtu_discovery_status_enum;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_mtu_probe(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut header_length: size_t,
    mut checksum_length: size_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: size_t,
) -> size_t {
    let mut probe_length: size_t = picoquic_next_mtu_probe_length(cnx, path_x);
    let mut length: size_t = header_length;
    if probe_length > bytes_max {
        probe_length = bytes_max;
    }
    let c2rust_fresh31 = length;
    length = length.wrapping_add(1);
    *bytes.offset(c2rust_fresh31 as isize) =
        picoquic_frame_type_ping as ::core::ffi::c_int as uint8_t;
    memset(
        bytes.offset(length as isize) as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        probe_length
            .wrapping_sub(checksum_length)
            .wrapping_sub(length),
    );
    return probe_length.wrapping_sub(checksum_length);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_packet_0rtt(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut packet: *mut picoquic_packet_t,
    mut current_time: uint64_t,
    mut send_buffer: *mut uint8_t,
    mut send_buffer_max: size_t,
    mut send_length: *mut size_t,
    mut padding_required: ::core::ffi::c_int,
    mut next_wake_time: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream: *mut picoquic_stream_head_t = ::core::ptr::null_mut::<picoquic_stream_head_t>();
    let mut packet_type: picoquic_packet_type_enum = picoquic_packet_0rtt_protected;
    let mut header_length: size_t = 0 as size_t;
    let mut bytes: *mut uint8_t = &raw mut (*packet).bytes as *mut uint8_t;
    let mut length: size_t = 0 as size_t;
    let mut checksum_overhead: size_t = picoquic_aead_get_checksum_length(
        (*cnx).crypto_context[1 as ::core::ffi::c_int as usize].aead_encrypt,
    );
    let mut bytes_max: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut bytes_next: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut more_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_pure_ack: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut stream_tried_and_failed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    send_buffer_max = if send_buffer_max > (*path_x).send_mtu {
        (*path_x).send_mtu
    } else {
        send_buffer_max
    };
    if (*path_x)
        .bytes_in_transit
        .wrapping_add(send_buffer_max as uint64_t)
        > PICOQUIC_DEFAULT_0RTT_WINDOW as uint64_t
    {
        if (*path_x).bytes_in_transit > PICOQUIC_DEFAULT_0RTT_WINDOW as uint64_t {
            send_buffer_max = 0 as size_t;
        } else {
            send_buffer_max = (PICOQUIC_DEFAULT_0RTT_WINDOW as size_t)
                .wrapping_sub((*path_x).bytes_in_transit as size_t);
        }
    }
    bytes_max = bytes
        .offset(send_buffer_max as isize)
        .offset(-(checksum_overhead as isize));
    stream = picoquic_find_ready_stream(cnx);
    length = picoquic_predict_packet_header_length(
        cnx,
        packet_type,
        (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t)
            .offset(picoquic_packet_context_application as ::core::ffi::c_int as isize)
            as *mut picoquic_packet_context_t,
    );
    (*packet).ptype = picoquic_packet_0rtt_protected;
    (*packet).offset = length;
    header_length = length;
    (*packet).pc = picoquic_packet_context_application;
    (*packet).sequence_number = (*cnx).pkt_ctx
        [picoquic_packet_context_application as ::core::ffi::c_int as usize]
        .send_sequence;
    (*packet).send_time = current_time;
    (*packet).send_path = path_x as *mut st_picoquic_path_t;
    (*packet).checksum_overhead = checksum_overhead;
    bytes_next = bytes.offset(length as isize);
    if stream.is_null()
        && (*cnx).first_misc_frame.is_null()
        && padding_required == 0 as ::core::ffi::c_int
        || send_buffer_max < PICOQUIC_MIN_SEGMENT_SIZE as size_t
    {
        length = 0 as size_t;
    } else {
        bytes_next = picoquic_format_misc_frames_in_context(
            cnx,
            bytes_next,
            bytes_max,
            &raw mut more_data,
            &raw mut is_pure_ack,
            picoquic_packet_context_application,
        );
        if (*cnx).local_parameters.enable_bdp_frame != 0 {
            bytes_next = picoquic_format_bdp_frame(
                cnx,
                bytes_next,
                bytes_max,
                path_x,
                &raw mut more_data,
                &raw mut is_pure_ack,
            );
        }
        bytes_next = picoquic_format_available_stream_frames(
            cnx,
            ::core::ptr::null_mut::<picoquic_path_t>(),
            bytes_next,
            bytes_max,
            UINT64_MAX as uint64_t,
            &raw mut more_data,
            &raw mut is_pure_ack,
            &raw mut stream_tried_and_failed,
            &raw mut ret,
        );
        length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
        if more_data != 0 {
            *next_wake_time = current_time;
            (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
            (*(*cnx).quic).wake_line = 1626 as ::core::ffi::c_int;
        }
        if stream_tried_and_failed != 0 {
            (*path_x).last_sender_limited_time = current_time;
        }
        if padding_required != 0 {
            length = picoquic_pad_to_target_length(
                bytes,
                length,
                send_buffer_max.wrapping_sub(checksum_overhead),
            );
        }
    }
    picoquic_finalize_and_protect_packet(
        cnx,
        packet,
        ret,
        length,
        header_length,
        checksum_overhead,
        send_length,
        send_buffer,
        send_buffer_max,
        path_x,
        current_time,
    );
    if length > 0 as size_t {
        (*cnx).nb_zero_rtt_sent = (*cnx).nb_zero_rtt_sent.wrapping_add(1);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_packet_type_from_epoch(
    mut epoch: ::core::ffi::c_int,
) -> picoquic_packet_type_enum {
    let mut ptype: picoquic_packet_type_enum = picoquic_packet_error;
    match epoch {
        0 => {
            ptype = picoquic_packet_initial;
        }
        1 => {
            ptype = picoquic_packet_0rtt_protected;
        }
        2 => {
            ptype = picoquic_packet_handshake;
        }
        3 => {
            ptype = picoquic_packet_1rtt_protected;
        }
        _ => {
            ptype = picoquic_packet_error;
        }
    }
    return ptype;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_packet_old_context(
    mut cnx: *mut picoquic_cnx_t,
    mut pc: picoquic_packet_context_enum,
    mut path_x: *mut picoquic_path_t,
    mut packet: *mut picoquic_packet_t,
    mut send_buffer_max: size_t,
    mut current_time: uint64_t,
    mut next_wake_time: *mut uint64_t,
    mut header_length: *mut size_t,
) -> size_t {
    let mut epoch: picoquic_epoch_enum = (if pc as ::core::ffi::c_uint
        == picoquic_packet_context_initial as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        picoquic_epoch_initial as ::core::ffi::c_int
    } else if pc as ::core::ffi::c_uint
        == picoquic_packet_context_application as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        picoquic_epoch_0rtt as ::core::ffi::c_int
    } else {
        picoquic_epoch_handshake as ::core::ffi::c_int
    }) as picoquic_epoch_enum;
    let mut length: size_t = 0 as size_t;
    if !(*cnx).crypto_context[epoch as usize].aead_encrypt.is_null() {
        let mut bytes: *mut uint8_t = &raw mut (*packet).bytes as *mut uint8_t;
        let mut more_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut checksum_overhead: size_t = picoquic_get_checksum_length(cnx, epoch);
        let mut bytes_max: *mut uint8_t = bytes
            .offset(send_buffer_max as isize)
            .offset(-(checksum_overhead as isize));
        let mut bytes_next: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut this_header_length: size_t = 0 as size_t;
        let mut is_pure_ack: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        send_buffer_max = if send_buffer_max > (*path_x).send_mtu {
            (*path_x).send_mtu
        } else {
            send_buffer_max
        };
        length = picoquic_retransmit_needed(
            cnx,
            pc,
            path_x,
            current_time,
            next_wake_time,
            packet,
            send_buffer_max,
            &raw mut this_header_length,
        ) as size_t;
        if length > 0 as size_t
            && (pc as ::core::ffi::c_uint
                == picoquic_packet_context_handshake as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cnx).pkt_ctx[picoquic_packet_context_handshake as ::core::ffi::c_int as usize]
                    .pending_first
                    .is_null()
                || (*cnx).cnx_state as ::core::ffi::c_uint
                    == picoquic_state_server_init as ::core::ffi::c_int as ::core::ffi::c_uint
                || (*cnx).cnx_state as ::core::ffi::c_uint
                    == picoquic_state_server_handshake as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            (*cnx).set_initial_repeat_needed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        if length == 0 as size_t
            && (*cnx).ack_ctx[pc as usize].act[0 as ::core::ffi::c_int as usize].ack_needed()
                as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
            && pc as ::core::ffi::c_uint
                != picoquic_packet_context_application as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*packet).ptype = (if pc as ::core::ffi::c_uint
                == picoquic_packet_context_initial as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                picoquic_packet_initial as ::core::ffi::c_int
            } else if pc as ::core::ffi::c_uint
                == picoquic_packet_context_handshake as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                picoquic_packet_handshake as ::core::ffi::c_int
            } else {
                picoquic_packet_0rtt_protected as ::core::ffi::c_int
            }) as picoquic_packet_type_enum;
            length = picoquic_predict_packet_header_length(
                cnx,
                (*packet).ptype,
                (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t).offset(pc as isize)
                    as *mut picoquic_packet_context_t,
            );
            (*packet).offset = length;
            this_header_length = length;
            (*packet).sequence_number = (*cnx).pkt_ctx[pc as usize].send_sequence;
            (*packet).send_time = current_time;
            (*packet).send_path = path_x as *mut st_picoquic_path_t;
        }
        if length > 0 as size_t {
            bytes_next = bytes.offset(length as isize);
            bytes_next = picoquic_format_misc_frames_in_context(
                cnx,
                bytes_next,
                bytes_max,
                &raw mut more_data,
                &raw mut is_pure_ack,
                pc,
            );
            if (*packet).ptype as ::core::ffi::c_uint
                != picoquic_packet_0rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                bytes_next = picoquic_format_ack_frame(
                    cnx,
                    bytes_next,
                    bytes_max,
                    &raw mut more_data,
                    current_time,
                    pc,
                    0 as ::core::ffi::c_int,
                );
            }
            length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
            (*packet).length = length;
            (*packet).send_time = current_time;
            (*packet).checksum_overhead = checksum_overhead;
            (*packet).pc = pc;
            *header_length = this_header_length;
        }
    }
    return length;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_implicit_handshake_ack(
    mut cnx: *mut picoquic_cnx_t,
    mut pc: picoquic_packet_context_enum,
    mut current_time: uint64_t,
) {
    let mut p: *mut picoquic_packet_t = (*cnx).pkt_ctx[pc as usize].pending_first;
    while !p.is_null() {
        let mut p_next: *mut picoquic_packet_t = (*p).packet_next as *mut picoquic_packet_t;
        let mut old_path: *mut picoquic_path_t = (*p).send_path as *mut picoquic_path_t;
        if !old_path.is_null()
            && !(*cnx).congestion_alg.is_null()
            && ((*p).send_time as ::core::ffi::c_ulonglong)
                < ((*cnx).start_time as ::core::ffi::c_ulonglong).wrapping_add(PICOQUIC_INITIAL_RTT)
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
            ack_state.rtt_measurement = (*old_path).rtt_sample;
            ack_state.nb_bytes_acknowledged = (*p).length as uint64_t;
            (*old_path).delivered = ((*old_path).delivered as ::core::ffi::c_ulong)
                .wrapping_add((*p).length as ::core::ffi::c_ulong)
                as uint64_t as uint64_t;
            ack_state.nb_bytes_delivered_since_packet_sent =
                (*old_path).delivered.wrapping_sub((*p).delivered_prior);
            ack_state.set_is_app_limited(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*(*cnx).congestion_alg)
                .alg_notify
                .expect("non-null function pointer")(
                cnx as *mut picoquic_cnx_t,
                old_path as *mut picoquic_path_t,
                picoquic_congestion_notification_acknowledgement,
                &raw mut ack_state,
                current_time,
            );
        }
        picoquic_dequeue_retransmit_packet(
            cnx,
            (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t).offset(pc as isize)
                as *mut picoquic_packet_context_t,
            p,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
        );
        p = p_next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_server_address_migration(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut transport_error: uint64_t = 0 as uint64_t;
    if (*cnx).remote_parameters.prefered_address.is_defined != 0 {
        let mut unique_path_id: uint64_t =
            (if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0 {
                1 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) as uint64_t;
        let mut ipv4_received: ::core::ffi::c_int =
            ((*cnx).remote_parameters.prefered_address.ipv4Port as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let mut ipv6_received: ::core::ffi::c_int =
            ((*cnx).remote_parameters.prefered_address.ipv6Port as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
        transport_error = picoquic_stash_remote_cnxid(
            cnx,
            0 as uint64_t,
            unique_path_id,
            1 as uint64_t,
            (*cnx)
                .remote_parameters
                .prefered_address
                .connection_id
                .id_len,
            &raw mut (*cnx).remote_parameters.prefered_address.connection_id.id as *mut uint8_t,
            &raw mut (*cnx)
                .remote_parameters
                .prefered_address
                .statelessResetToken as *mut uint8_t,
            ::core::ptr::null_mut::<*mut picoquic_remote_cnxid_t>(),
        );
        if transport_error != 0 as uint64_t {
            ret = picoquic_connection_error(
                cnx,
                transport_error,
                picoquic_frame_type_new_connection_id as ::core::ffi::c_int as uint64_t,
            );
        } else if ipv4_received != 0 || ipv6_received != 0 {
            let mut dest_addr: sockaddr_storage = sockaddr_storage {
                ss_family: 0,
                __ss_padding: [0; 118],
                __ss_align: 0,
            };
            memset(
                &raw mut dest_addr as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                ::core::mem::size_of::<sockaddr_storage>() as size_t,
            );
            if ipv4_received != 0
                && (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                    .peer_addr
                    .ss_family as ::core::ffi::c_int
                    == AF_INET
            {
                ipv6_received = 0 as ::core::ffi::c_int;
            }
            if ipv6_received != 0 {
                let mut d6: *mut sockaddr_in6 = &raw mut dest_addr as *mut sockaddr_in6;
                (*d6).sin6_family = AF_INET6 as sa_family_t;
                (*d6).sin6_port =
                    __bswap_16((*cnx).remote_parameters.prefered_address.ipv6Port as __uint16_t)
                        as in_port_t;
                memcpy(
                    &raw mut (*d6).sin6_addr as *mut ::core::ffi::c_void,
                    &raw mut (*cnx).remote_parameters.prefered_address.ipv6Address as *mut uint8_t
                        as *const ::core::ffi::c_void,
                    16 as size_t,
                );
            } else {
                let mut d4: *mut sockaddr_in = &raw mut dest_addr as *mut sockaddr_in;
                (*d4).sin_family = AF_INET as sa_family_t;
                (*d4).sin_port =
                    __bswap_16((*cnx).remote_parameters.prefered_address.ipv4Port as __uint16_t)
                        as in_port_t;
                memcpy(
                    &raw mut (*d4).sin_addr as *mut ::core::ffi::c_void,
                    &raw mut (*cnx).remote_parameters.prefered_address.ipv4Address as *mut uint8_t
                        as *const ::core::ffi::c_void,
                    4 as size_t,
                );
            }
            if picoquic_compare_addr(
                &raw mut dest_addr as *mut sockaddr,
                &raw mut (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).peer_addr
                    as *mut sockaddr,
            ) != 0 as ::core::ffi::c_int
                && ((*(*cnx).quic).is_port_blocking_disabled() as ::core::ffi::c_int != 0
                    || picoquic_check_addr_blocked(&raw mut dest_addr as *mut sockaddr) == 0)
            {
                let mut local_addr: *mut sockaddr = ::core::ptr::null_mut::<sockaddr>();
                if (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                    .local_addr
                    .ss_family as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                    && (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                        .local_addr
                        .ss_family as ::core::ffi::c_int
                        == dest_addr.ss_family as ::core::ffi::c_int
                {
                    local_addr = &raw mut (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                        .local_addr as *mut sockaddr;
                }
                ret = picoquic_probe_new_path_ex(
                    cnx as *mut picoquic_cnx_t,
                    &raw mut dest_addr as *mut sockaddr,
                    local_addr,
                    0 as ::core::ffi::c_int,
                    picoquic_get_quic_time((*cnx).quic as *mut picoquic_quic_t),
                    1 as ::core::ffi::c_int,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                );
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_packet_client_init(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut packet: *mut picoquic_packet_t,
    mut current_time: uint64_t,
    mut send_buffer: *mut uint8_t,
    mut send_buffer_max: size_t,
    mut send_length: *mut size_t,
    mut next_wake_time: *mut uint64_t,
    mut is_initial_sent: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tls_ready: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut checksum_overhead: size_t = 16 as size_t;
    let mut is_cleartext_mode: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut retransmit_possible: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut header_length: size_t = 0 as size_t;
    let mut bytes: *mut uint8_t = &raw mut (*packet).bytes as *mut uint8_t;
    let mut bytes_max: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut bytes_next: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut length: size_t = 0 as size_t;
    let mut is_pure_ack: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut more_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut epoch: ::core::ffi::c_int = picoquic_epoch_initial as ::core::ffi::c_int;
    let mut packet_type: picoquic_packet_type_enum = picoquic_packet_initial;
    let mut pc: picoquic_packet_context_enum = picoquic_packet_context_initial;
    (*cnx).set_initial_validated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if (*cnx).tls_stream[picoquic_epoch_initial as ::core::ffi::c_int as usize]
        .send_queue
        .is_null()
    {
        if !(*cnx).crypto_context[picoquic_epoch_0rtt as ::core::ffi::c_int as usize]
            .aead_encrypt
            .is_null()
            && !(*cnx).tls_stream[picoquic_epoch_0rtt as ::core::ffi::c_int as usize]
                .send_queue
                .is_null()
        {
            epoch = picoquic_epoch_0rtt as ::core::ffi::c_int;
            pc = picoquic_packet_context_application;
            packet_type = picoquic_packet_0rtt_protected;
        } else if !(*cnx).crypto_context[picoquic_epoch_handshake as ::core::ffi::c_int as usize]
            .aead_encrypt
            .is_null()
            && (*cnx).tls_stream[picoquic_epoch_0rtt as ::core::ffi::c_int as usize]
                .send_queue
                .is_null()
        {
            epoch = picoquic_epoch_handshake as ::core::ffi::c_int;
            pc = picoquic_packet_context_handshake;
            packet_type = picoquic_packet_handshake;
        }
    }
    send_buffer_max = if send_buffer_max > (*path_x).send_mtu {
        (*path_x).send_mtu
    } else {
        send_buffer_max
    };
    match (*cnx).cnx_state as ::core::ffi::c_uint {
        0 => {
            if (*cnx).retry_token_length as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && !(*cnx).sni.is_null()
            {
                picoquic_get_token(
                    (*cnx).quic as *mut picoquic_quic_t,
                    (*cnx).sni,
                    strlen((*cnx).sni) as uint16_t,
                    ::core::ptr::null::<uint8_t>(),
                    0 as uint8_t,
                    &raw mut (*cnx).retry_token,
                    &raw mut (*cnx).retry_token_length,
                    1 as ::core::ffi::c_int,
                );
            }
        }
        1 | 4 => {
            retransmit_possible = 1 as ::core::ffi::c_int;
        }
        2 => {
            packet_type = picoquic_packet_initial;
        }
        7 => {
            retransmit_possible = 1 as ::core::ffi::c_int;
        }
        10 => {}
        _ => {
            ret = -(1 as ::core::ffi::c_int);
        }
    }
    let mut force_handshake_padding: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if ret == 0 as ::core::ffi::c_int {
        if epoch > picoquic_epoch_initial as ::core::ffi::c_int {
            if !(*cnx).crypto_context[picoquic_epoch_handshake as ::core::ffi::c_int as usize]
                .aead_encrypt
                .is_null()
            {
                if (*cnx).ack_ctx[picoquic_packet_context_initial as ::core::ffi::c_int as usize]
                    .act[0 as ::core::ffi::c_int as usize]
                    .ack_needed()
                    != 0
                {
                    let mut ack_delay: uint64_t =
                        (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                            .smoothed_rtt
                            .wrapping_div(8 as uint64_t);
                    let mut ack_time: uint64_t = 0;
                    if ack_delay as ::core::ffi::c_ulonglong > PICOQUIC_ACK_DELAY_MAX {
                        ack_delay = PICOQUIC_ACK_DELAY_MAX as uint64_t;
                    }
                    ack_time = (*cnx).ack_ctx
                        [picoquic_packet_context_initial as ::core::ffi::c_int as usize]
                        .act[0 as ::core::ffi::c_int as usize]
                        .time_oldest_unack_packet_received
                        .wrapping_add(ack_delay);
                    if ack_time <= current_time {
                        force_handshake_padding = 1 as ::core::ffi::c_int;
                    } else if ack_time < *next_wake_time {
                        *next_wake_time = ack_time;
                        (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
                        (*(*cnx).quic).wake_line = 1925 as ::core::ffi::c_int;
                    }
                } else if !(*cnx).pkt_ctx[pc as usize].pending_last.is_null() {
                    let mut rto: uint64_t = picoquic_current_retransmit_timer(
                        cnx,
                        *(*cnx).path.offset(0 as ::core::ffi::c_int as isize),
                    );
                    let mut repeat_time: uint64_t = (*(*cnx).pkt_ctx[pc as usize].pending_last)
                        .send_time
                        .wrapping_add(rto);
                    if repeat_time <= current_time {
                        force_handshake_padding = 1 as ::core::ffi::c_int;
                        let ref mut c2rust_fresh37 =
                            (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).nb_retransmit;
                        *c2rust_fresh37 = (*c2rust_fresh37).wrapping_add(1);
                        (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                            .last_loss_event_detected = current_time;
                    } else if repeat_time < *next_wake_time {
                        *next_wake_time = repeat_time;
                        (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
                        (*(*cnx).quic).wake_line = 1942 as ::core::ffi::c_int;
                    }
                }
            } else {
                length = picoquic_prepare_packet_old_context(
                    cnx,
                    picoquic_packet_context_initial,
                    path_x,
                    packet,
                    send_buffer_max,
                    current_time,
                    next_wake_time,
                    &raw mut header_length,
                );
                *is_initial_sent |= (length > 0 as size_t) as ::core::ffi::c_int;
            }
        } else {
            let mut rto_0: uint64_t = picoquic_current_retransmit_timer(
                cnx,
                *(*cnx).path.offset(0 as ::core::ffi::c_int as isize),
            );
            let mut repeat_time_0: uint64_t =
                (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                    .latest_sent_time
                    .wrapping_add(rto_0);
            if repeat_time_0 <= current_time {
                force_handshake_padding = 1 as ::core::ffi::c_int;
            } else if *next_wake_time > repeat_time_0 {
                *next_wake_time = repeat_time_0;
                (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
                (*(*cnx).quic).wake_line = 1962 as ::core::ffi::c_int;
            }
        }
    }
    if ret == 0 as ::core::ffi::c_int
        && epoch > picoquic_epoch_0rtt as ::core::ffi::c_int
        && length == 0 as size_t
        && !(*cnx).crypto_context[picoquic_epoch_0rtt as ::core::ffi::c_int as usize]
            .aead_encrypt
            .is_null()
    {
        length = picoquic_prepare_packet_old_context(
            cnx,
            picoquic_packet_context_application,
            path_x,
            packet,
            send_buffer_max,
            current_time,
            next_wake_time,
            &raw mut header_length,
        );
    }
    if length == 0 as size_t {
        checksum_overhead = picoquic_get_checksum_length(cnx, epoch as picoquic_epoch_enum);
        (*packet).checksum_overhead = checksum_overhead;
        bytes_max = bytes
            .offset(send_buffer_max as isize)
            .offset(-(checksum_overhead as isize));
        (*packet).pc = pc;
        tls_ready = picoquic_is_tls_stream_ready(cnx);
        if ret == 0 as ::core::ffi::c_int && retransmit_possible != 0 && {
            length = picoquic_retransmit_needed(
                cnx,
                pc,
                path_x,
                current_time,
                next_wake_time,
                packet,
                send_buffer_max,
                &raw mut header_length,
            ) as size_t;
            length > 0 as size_t
        } {
            if epoch != picoquic_epoch_0rtt as ::core::ffi::c_int && length > header_length {
                bytes_next = picoquic_format_ack_frame(
                    cnx,
                    bytes.offset(length as isize),
                    bytes_max,
                    &raw mut more_data,
                    current_time,
                    pc,
                    0 as ::core::ffi::c_int,
                );
                length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
            }
            (*packet).length = length;
            (*packet).send_time = current_time;
            (*packet).checksum_overhead = checksum_overhead;
            *is_initial_sent = ((*packet).ptype as ::core::ffi::c_uint
                == picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint)
                as ::core::ffi::c_int;
        } else if ret == 0 as ::core::ffi::c_int
            && is_cleartext_mode != 0
            && tls_ready == 0 as ::core::ffi::c_int
            && picoquic_find_first_misc_frame(cnx, pc).is_null()
            && (*cnx).ack_ctx[pc as usize].act[0 as ::core::ffi::c_int as usize].ack_needed() == 0
            && force_handshake_padding == 0
        {
            (*packet).length = 0 as size_t;
        } else if ret == 0 as ::core::ffi::c_int {
            if (*cnx).crypto_context[epoch as usize].aead_encrypt.is_null() {
                (*packet).length = 0 as size_t;
            } else {
                length = picoquic_predict_packet_header_length(
                    cnx,
                    packet_type,
                    (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t).offset(pc as isize)
                        as *mut picoquic_packet_context_t,
                );
                (*packet).ptype = packet_type;
                (*packet).offset = length;
                header_length = length;
                (*packet).sequence_number = (*cnx).pkt_ctx[pc as usize].send_sequence;
                (*packet).send_time = current_time;
                (*packet).send_path = path_x as *mut st_picoquic_path_t;
                bytes_next = bytes.offset(length as isize);
                bytes_max = bytes
                    .offset(send_buffer_max as isize)
                    .offset(-(checksum_overhead as isize));
                if (tls_ready == 0 as ::core::ffi::c_int
                    || (*path_x).cwin <= (*path_x).bytes_in_transit
                    || (*(*cnx).quic).cwin_max <= (*path_x).bytes_in_transit)
                    && ((*cnx).cnx_state as ::core::ffi::c_uint
                        == picoquic_state_client_almost_ready as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                        || picoquic_is_ack_needed(
                            cnx,
                            current_time,
                            next_wake_time,
                            pc,
                            0 as ::core::ffi::c_int,
                        ) == 0 as ::core::ffi::c_int)
                    && picoquic_find_first_misc_frame(cnx, pc).is_null()
                    && force_handshake_padding == 0
                {
                    length = 0 as size_t;
                } else {
                    if force_handshake_padding != 0 {
                        let c2rust_fresh38 = bytes_next;
                        bytes_next = bytes_next.offset(1);
                        *c2rust_fresh38 = picoquic_frame_type_ping as ::core::ffi::c_int as uint8_t;
                    }
                    if epoch != picoquic_epoch_0rtt as ::core::ffi::c_int
                        && ((*cnx).ack_ctx[pc as usize].act[0 as ::core::ffi::c_int as usize]
                            .ack_needed() as ::core::ffi::c_int
                            != 0
                            || force_handshake_padding != 0
                                && picoquic_sack_list_last(
                                    &raw mut (*(&raw mut (*cnx).ack_ctx
                                        as *mut picoquic_ack_context_t)
                                        .offset(pc as isize))
                                    .sack_list,
                                ) != UINT64_MAX as uint64_t)
                    {
                        bytes_next = picoquic_format_ack_frame(
                            cnx,
                            bytes_next,
                            bytes_max,
                            &raw mut more_data,
                            current_time,
                            pc,
                            0 as ::core::ffi::c_int,
                        );
                    }
                    bytes_next = picoquic_format_misc_frames_in_context(
                        cnx,
                        bytes_next,
                        bytes_max,
                        &raw mut more_data,
                        &raw mut is_pure_ack,
                        pc,
                    );
                    length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
                    if ret == 0 as ::core::ffi::c_int
                        && (*path_x).cwin > (*path_x).bytes_in_transit
                        && (*(*cnx).quic).cwin_max > (*path_x).bytes_in_transit
                    {
                        if tls_ready != 0 as ::core::ffi::c_int {
                            bytes_next = picoquic_format_crypto_hs_frame(
                                (&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t)
                                    .offset(epoch as isize)
                                    as *mut picoquic_stream_head_t,
                                bytes_next,
                                bytes_max,
                                &raw mut more_data,
                                &raw mut is_pure_ack,
                            );
                            length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
                        }
                        if packet_type as ::core::ffi::c_uint
                            == picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint
                        {
                            *is_initial_sent = 1 as ::core::ffi::c_int;
                            if (*cnx).crypto_context[1 as ::core::ffi::c_int as usize]
                                .aead_encrypt
                                .is_null()
                                || (*cnx).cnx_state as ::core::ffi::c_uint
                                    == picoquic_state_client_renegotiate as ::core::ffi::c_int
                                        as ::core::ffi::c_uint
                                || (*cnx).original_cnxid.id_len as ::core::ffi::c_int
                                    != 0 as ::core::ffi::c_int
                            {
                                length = picoquic_pad_to_target_length(
                                    bytes,
                                    length,
                                    send_buffer_max.wrapping_sub(checksum_overhead),
                                );
                            }
                        }
                    }
                    if length > header_length
                        && epoch == picoquic_epoch_handshake as ::core::ffi::c_int
                    {
                        (*cnx).ack_ctx
                            [picoquic_packet_context_initial as ::core::ffi::c_int as usize]
                            .act[0 as ::core::ffi::c_int as usize]
                            .set_ack_needed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    }
                    if ret == 0 as ::core::ffi::c_int
                        && tls_ready != 0 as ::core::ffi::c_int
                        && (*cnx).tls_stream[epoch as usize].send_queue.is_null()
                    {
                        match (*cnx).cnx_state as ::core::ffi::c_uint {
                            0 => {
                                (*cnx).cnx_state = picoquic_state_client_init_sent;
                            }
                            2 => {
                                (*cnx).cnx_state = picoquic_state_client_init_resent;
                            }
                            10 => {
                                if (*cnx).tls_stream[0 as ::core::ffi::c_int as usize]
                                    .send_queue
                                    .is_null()
                                    && (*cnx).tls_stream[1 as ::core::ffi::c_int as usize]
                                        .send_queue
                                        .is_null()
                                    && (*cnx).tls_stream[2 as ::core::ffi::c_int as usize]
                                        .send_queue
                                        .is_null()
                                {
                                    (*cnx).cnx_state = picoquic_state_client_ready_start;
                                    if (*cnx).callback_fn.is_some() {
                                        if (*cnx).callback_fn.expect("non-null function pointer")(
                                            cnx as *mut picoquic_cnx_t,
                                            0 as uint64_t,
                                            ::core::ptr::null_mut::<uint8_t>(),
                                            0 as size_t,
                                            picoquic_callback_almost_ready,
                                            (*cnx).callback_ctx,
                                            NULL,
                                        ) != 0 as ::core::ffi::c_int
                                        {
                                            picoquic_log_app_message(
                                                cnx as *mut picoquic_cnx_t,
                                                b"Callback almost ready returns error 0x%x\0"
                                                    .as_ptr()
                                                    as *const ::core::ffi::c_char,
                                                PICOQUIC_TRANSPORT_INTERNAL_ERROR,
                                            );
                                            picoquic_connection_error(
                                                cnx,
                                                PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t,
                                                0 as uint64_t,
                                            );
                                        }
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    if ret == 0 as ::core::ffi::c_int
        && length == 0 as size_t
        && !(*cnx).crypto_context[1 as ::core::ffi::c_int as usize]
            .aead_encrypt
            .is_null()
    {
        ret = picoquic_prepare_packet_0rtt(
            cnx,
            path_x,
            packet,
            current_time,
            send_buffer,
            send_buffer_max,
            send_length,
            *is_initial_sent,
            next_wake_time,
        );
    } else {
        if ret == 0 as ::core::ffi::c_int && more_data != 0 {
            *next_wake_time = current_time;
            (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
            (*(*cnx).quic).wake_line = 2105 as ::core::ffi::c_int;
        }
        if ret == 0 as ::core::ffi::c_int && *is_initial_sent != 0 {
            if (*packet).ptype as ::core::ffi::c_uint
                == picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if length > 0 as size_t
                    && (*cnx).crypto_context[1 as ::core::ffi::c_int as usize]
                        .aead_encrypt
                        .is_null()
                    && ((*cnx).crypto_context[2 as ::core::ffi::c_int as usize]
                        .aead_encrypt
                        .is_null()
                        || length
                            .wrapping_add(checksum_overhead)
                            .wrapping_add(PICOQUIC_MIN_SEGMENT_SIZE as size_t)
                            > send_buffer_max
                        || picoquic_is_tls_stream_ready(cnx) == 0)
                {
                    length = picoquic_pad_to_target_length(
                        bytes,
                        length,
                        send_buffer_max.wrapping_sub(checksum_overhead),
                    );
                }
            } else if (*packet).ptype as ::core::ffi::c_uint
                == picoquic_packet_handshake as ::core::ffi::c_int as ::core::ffi::c_uint
                && length.wrapping_add(checksum_overhead) < send_buffer_max
                && ((*cnx).crypto_context[3 as ::core::ffi::c_int as usize]
                    .aead_encrypt
                    .is_null()
                    || length
                        .wrapping_add(checksum_overhead)
                        .wrapping_add(PICOQUIC_MIN_SEGMENT_SIZE as size_t)
                        > send_buffer_max)
            {
                length = picoquic_pad_to_target_length(
                    bytes,
                    length,
                    send_buffer_max.wrapping_sub(checksum_overhead),
                );
            } else if (*packet).ptype as ::core::ffi::c_uint
                == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                length = picoquic_pad_to_target_length(
                    bytes,
                    length,
                    send_buffer_max.wrapping_sub(checksum_overhead),
                );
            }
        }
        if length > 0 as size_t
            && (*packet).ptype as ::core::ffi::c_uint
                == picoquic_packet_handshake as ::core::ffi::c_int as ::core::ffi::c_uint
            && is_pure_ack == 0
        {
            picoquic_implicit_handshake_ack(cnx, picoquic_packet_context_initial, current_time);
            picoquic_crypto_context_free(
                (&raw mut (*cnx).crypto_context as *mut picoquic_crypto_context_t)
                    .offset(picoquic_epoch_initial as ::core::ffi::c_int as isize)
                    as *mut picoquic_crypto_context_t,
            );
        }
        picoquic_finalize_and_protect_packet(
            cnx,
            packet,
            ret,
            length,
            header_length,
            checksum_overhead,
            send_length,
            send_buffer,
            send_buffer_max,
            path_x,
            current_time,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_next_challenge_time(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut current_time: uint64_t,
    mut is_nat: *mut ::core::ffi::c_uint,
) -> uint64_t {
    let mut next_challenge_time: uint64_t = (*path_x).challenge_time;
    if (*path_x).challenge_repeat_count as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        next_challenge_time = current_time;
    } else if (*path_x).challenge_repeat_count as ::core::ffi::c_int >= 2 as ::core::ffi::c_int {
        next_challenge_time = next_challenge_time.wrapping_add(
            (*path_x).retransmit_timer
                << (*path_x).challenge_repeat_count as ::core::ffi::c_int - 1 as ::core::ffi::c_int,
        );
    } else {
        next_challenge_time = (next_challenge_time as ::core::ffi::c_ulonglong)
            .wrapping_add(PICOQUIC_INITIAL_RETRANSMIT_TIMER)
            as uint64_t as uint64_t;
    }
    if !is_nat.is_null() {
        *is_nat = 0 as ::core::ffi::c_uint;
        if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
            && (*path_x).nat_local_addr.ss_family as ::core::ffi::c_int != AF_UNSPEC
        {
            let mut nat_challenge_time: uint64_t = (*path_x).nat_challenge_time;
            if (*path_x).nat_challenge_repeat_count == 0 as uint64_t {
                nat_challenge_time = current_time;
            } else if (*path_x).challenge_repeat_count as ::core::ffi::c_int
                >= 2 as ::core::ffi::c_int
            {
                nat_challenge_time = nat_challenge_time.wrapping_add(
                    (*path_x).retransmit_timer
                        << (*path_x).challenge_repeat_count as ::core::ffi::c_int,
                );
            } else {
                nat_challenge_time = nat_challenge_time.wrapping_add((*path_x).retransmit_timer);
            }
            if nat_challenge_time <= current_time
                && (*path_x).nat_challenge_repeat_count >= PICOQUIC_CHALLENGE_REPEAT_MAX as uint64_t
            {
                picoquic_log_app_message(
                    cnx as *mut picoquic_cnx_t,
                    b"NAT Challenge failed on path %lu\0".as_ptr() as *const ::core::ffi::c_char,
                    (*path_x).unique_path_id,
                );
                (*path_x).nat_local_addr.ss_family = AF_UNSPEC as sa_family_t;
            } else if nat_challenge_time < next_challenge_time {
                *is_nat = 1 as ::core::ffi::c_uint;
                next_challenge_time = nat_challenge_time;
            }
        }
    }
    return next_challenge_time;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_packet_server_init(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut packet: *mut picoquic_packet_t,
    mut current_time: uint64_t,
    mut send_buffer: *mut uint8_t,
    mut send_buffer_max: size_t,
    mut send_length: *mut size_t,
    mut next_wake_time: *mut uint64_t,
    mut is_initial_sent: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tls_ready: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut epoch: picoquic_epoch_enum = picoquic_epoch_initial;
    let mut packet_type: picoquic_packet_type_enum = picoquic_packet_initial;
    let mut pc: picoquic_packet_context_enum = picoquic_packet_context_initial;
    let mut checksum_overhead: size_t = 8 as size_t;
    let mut header_length: size_t = 0 as size_t;
    let mut bytes: *mut uint8_t = &raw mut (*packet).bytes as *mut uint8_t;
    let mut bytes_max: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut bytes_next: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut length: size_t = 0 as size_t;
    let mut more_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_pure_ack: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if *next_wake_time as ::core::ffi::c_ulonglong
        > ((*cnx).start_time as ::core::ffi::c_ulonglong)
            .wrapping_add(PICOQUIC_MICROSEC_HANDSHAKE_MAX)
    {
        *next_wake_time = ((*cnx).start_time as ::core::ffi::c_ulonglong)
            .wrapping_add(PICOQUIC_MICROSEC_HANDSHAKE_MAX) as uint64_t;
        (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
        (*(*cnx).quic).wake_line = 2211 as ::core::ffi::c_int;
    }
    if path_x.is_null() {
        return PICOQUIC_ERROR_UNEXPECTED_ERROR;
    }
    if !(*cnx).crypto_context[picoquic_epoch_handshake as ::core::ffi::c_int as usize]
        .aead_encrypt
        .is_null()
        && (*cnx).tls_stream[picoquic_epoch_initial as ::core::ffi::c_int as usize]
            .send_queue
            .is_null()
    {
        epoch = picoquic_epoch_handshake;
        pc = picoquic_packet_context_handshake;
        packet_type = picoquic_packet_handshake;
    }
    send_buffer_max = if send_buffer_max > (*path_x).send_mtu {
        (*path_x).send_mtu
    } else {
        send_buffer_max
    };
    if (*cnx).initial_validated() == 0
        && (*cnx)
            .initial_data_sent
            .wrapping_add(send_buffer_max as uint64_t)
            > (3 as uint64_t).wrapping_mul((*cnx).initial_data_received)
    {
        *send_length = 0 as size_t;
        return 0 as ::core::ffi::c_int;
    }
    if pc as ::core::ffi::c_uint
        == picoquic_packet_context_handshake as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        length = picoquic_prepare_packet_old_context(
            cnx,
            picoquic_packet_context_initial,
            path_x,
            packet,
            send_buffer_max,
            current_time,
            next_wake_time,
            &raw mut header_length,
        );
    }
    if length == 0 as size_t {
        checksum_overhead = picoquic_get_checksum_length(cnx, epoch);
        bytes_max = bytes
            .offset(send_buffer_max as isize)
            .offset(-(checksum_overhead as isize));
        tls_ready = (!(*cnx).tls_stream[epoch as usize].send_queue.is_null()
            && (*(*cnx).tls_stream[epoch as usize].send_queue).length
                > (*(*cnx).tls_stream[epoch as usize].send_queue).offset as size_t)
            as ::core::ffi::c_int;
        length = picoquic_predict_packet_header_length(
            cnx,
            packet_type,
            (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t).offset(pc as isize)
                as *mut picoquic_packet_context_t,
        );
        (*packet).ptype = packet_type;
        (*packet).offset = length;
        header_length = length;
        (*packet).sequence_number = (*cnx).pkt_ctx[pc as usize].send_sequence;
        (*packet).send_time = current_time;
        (*packet).send_path = path_x as *mut st_picoquic_path_t;
        (*packet).pc = pc;
        bytes_next = bytes.offset(length as isize);
        if (tls_ready != 0 || !picoquic_find_first_misc_frame(cnx, pc).is_null())
            && (*path_x).cwin > (*path_x).bytes_in_transit
            && (*(*cnx).quic).cwin_max > (*path_x).bytes_in_transit
            || (*cnx).ack_ctx[pc as usize].act[0 as ::core::ffi::c_int as usize].ack_needed()
                as ::core::ffi::c_int
                != 0
        {
            bytes_next = picoquic_format_ack_frame(
                cnx,
                bytes_next,
                bytes_max,
                &raw mut more_data,
                current_time,
                pc,
                0 as ::core::ffi::c_int,
            );
            bytes_next = picoquic_format_misc_frames_in_context(
                cnx,
                bytes_next,
                bytes_max,
                &raw mut more_data,
                &raw mut is_pure_ack,
                pc,
            );
            bytes_next = picoquic_format_crypto_hs_frame(
                (&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t).offset(epoch as isize)
                    as *mut picoquic_stream_head_t,
                bytes_next,
                bytes_max,
                &raw mut more_data,
                &raw mut is_pure_ack,
            );
            length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
            *is_initial_sent |= (epoch as ::core::ffi::c_uint
                == picoquic_epoch_initial as ::core::ffi::c_int as ::core::ffi::c_uint
                && is_pure_ack == 0) as ::core::ffi::c_int;
            if ret == 0 as ::core::ffi::c_int
                && tls_ready != 0 as ::core::ffi::c_int
                && (*cnx).tls_stream[epoch as usize].send_queue.is_null()
            {
                if epoch as ::core::ffi::c_uint
                    == picoquic_epoch_handshake as ::core::ffi::c_int as ::core::ffi::c_uint
                    && picoquic_tls_client_authentication_activated((*cnx).quic)
                        == 0 as ::core::ffi::c_int
                {
                    picoquic_false_start_transition(cnx, current_time);
                    if (*cnx).callback_fn.is_some() {
                        if (*cnx).callback_fn.expect("non-null function pointer")(
                            cnx as *mut picoquic_cnx_t,
                            0 as uint64_t,
                            ::core::ptr::null_mut::<uint8_t>(),
                            0 as size_t,
                            picoquic_callback_almost_ready,
                            (*cnx).callback_ctx,
                            NULL,
                        ) != 0 as ::core::ffi::c_int
                        {
                            picoquic_log_app_message(
                                cnx as *mut picoquic_cnx_t,
                                b"Callback almost ready returns error 0x%x\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                PICOQUIC_TRANSPORT_INTERNAL_ERROR,
                            );
                            picoquic_connection_error(
                                cnx,
                                PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t,
                                0 as uint64_t,
                            );
                        }
                    }
                } else {
                    (*cnx).cnx_state = picoquic_state_server_handshake;
                }
            }
            (*packet).length = length;
        } else {
            length = picoquic_retransmit_needed(
                cnx,
                pc,
                path_x,
                current_time,
                next_wake_time,
                packet,
                send_buffer_max,
                &raw mut header_length,
            ) as size_t;
            if length > 0 as size_t {
                checksum_overhead = picoquic_get_checksum_length(cnx, epoch);
                (*cnx).set_initial_repeat_needed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                bytes_max = bytes
                    .offset(send_buffer_max as isize)
                    .offset(-(checksum_overhead as isize));
                bytes_next = picoquic_format_ack_frame(
                    cnx,
                    bytes.offset(length as isize),
                    bytes_max,
                    &raw mut more_data,
                    current_time,
                    pc,
                    0 as ::core::ffi::c_int,
                );
                length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
                (*packet).length = length;
                (*packet).send_time = current_time;
                (*packet).checksum_overhead = checksum_overhead;
            } else {
                length = 0 as size_t;
                (*packet).length = 0 as size_t;
            }
        }
    }
    if ret == 0 as ::core::ffi::c_int && length == 0 as size_t && more_data != 0 {
        *next_wake_time = current_time;
        (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
        (*(*cnx).quic).wake_line = 2310 as ::core::ffi::c_int;
    }
    if ret == 0 as ::core::ffi::c_int && *is_initial_sent != 0 {
        if (*packet).ptype as ::core::ffi::c_uint
            == picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if length > 0 as size_t
                && (*cnx).crypto_context[1 as ::core::ffi::c_int as usize]
                    .aead_encrypt
                    .is_null()
                && ((*cnx).crypto_context[2 as ::core::ffi::c_int as usize]
                    .aead_encrypt
                    .is_null()
                    || length
                        .wrapping_add(checksum_overhead)
                        .wrapping_add(PICOQUIC_MIN_SEGMENT_SIZE as size_t)
                        > send_buffer_max
                    || picoquic_is_tls_stream_ready(cnx) == 0
                    || (*(*cnx).quic).dont_coalesce_init() as ::core::ffi::c_int != 0)
            {
                length = picoquic_pad_to_target_length(
                    bytes,
                    length,
                    send_buffer_max.wrapping_sub(checksum_overhead),
                );
            }
        } else if (*packet).ptype as ::core::ffi::c_uint
            == picoquic_packet_handshake as ::core::ffi::c_int as ::core::ffi::c_uint
            && length.wrapping_add(checksum_overhead) < send_buffer_max
            && ((*cnx).crypto_context[3 as ::core::ffi::c_int as usize]
                .aead_encrypt
                .is_null()
                || length
                    .wrapping_add(checksum_overhead)
                    .wrapping_add(PICOQUIC_MIN_SEGMENT_SIZE as size_t)
                    > send_buffer_max)
        {
            length = picoquic_pad_to_target_length(
                bytes,
                length,
                send_buffer_max.wrapping_sub(checksum_overhead),
            );
        }
    }
    picoquic_finalize_and_protect_packet(
        cnx,
        packet,
        ret,
        length,
        header_length,
        checksum_overhead,
        send_length,
        send_buffer,
        send_buffer_max,
        path_x,
        current_time,
    );
    if (*cnx).initial_validated() == 0 {
        (*cnx).initial_data_sent = ((*cnx).initial_data_sent as ::core::ffi::c_ulong)
            .wrapping_add(*send_length as ::core::ffi::c_ulong)
            as uint64_t as uint64_t;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_packet_closing(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut packet: *mut picoquic_packet_t,
    mut current_time: uint64_t,
    mut send_buffer: *mut uint8_t,
    mut send_buffer_max: size_t,
    mut send_length: *mut size_t,
    mut next_wake_time: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut packet_type: picoquic_packet_type_enum = picoquic_packet_error;
    let mut checksum_overhead: size_t = 8 as size_t;
    let mut header_length: size_t = 0 as size_t;
    let mut bytes: *mut uint8_t = &raw mut (*packet).bytes as *mut uint8_t;
    let mut bytes_max: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut bytes_next: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut more_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut length: size_t = 0 as size_t;
    let mut is_pure_ack: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut pc: picoquic_packet_context_enum = picoquic_packet_context_application;
    let mut pkt_ctx: *mut picoquic_packet_context_t =
        ::core::ptr::null_mut::<picoquic_packet_context_t>();
    let mut epoch: picoquic_epoch_enum = picoquic_epoch_1rtt;
    if path_x.is_null() {
        return PICOQUIC_ERROR_UNEXPECTED_ERROR;
    }
    send_buffer_max = if send_buffer_max > (*path_x).send_mtu {
        (*path_x).send_mtu
    } else {
        send_buffer_max
    };
    match (*cnx).cnx_state as ::core::ffi::c_uint {
        8 => {
            if !(*cnx).crypto_context[picoquic_epoch_handshake as ::core::ffi::c_int as usize]
                .aead_encrypt
                .is_null()
                && picoquic_sack_list_first(
                    &raw mut (*(&raw mut (*cnx).ack_ctx as *mut picoquic_ack_context_t)
                        .offset(picoquic_packet_context_handshake as ::core::ffi::c_int as isize))
                    .sack_list,
                ) != UINT64_MAX as uint64_t
            {
                pc = picoquic_packet_context_handshake;
                packet_type = picoquic_packet_handshake;
                epoch = picoquic_epoch_handshake;
            } else {
                pc = picoquic_packet_context_initial;
                packet_type = picoquic_packet_initial;
            }
        }
        9 => {
            pc = picoquic_packet_context_handshake;
            packet_type = picoquic_packet_handshake;
            epoch = picoquic_epoch_handshake;
        }
        15 => {
            packet_type = picoquic_packet_1rtt_protected;
        }
        16 => {
            packet_type = picoquic_packet_1rtt_protected;
        }
        17 => {
            packet_type = picoquic_packet_1rtt_protected;
        }
        18 => {
            packet_type = picoquic_packet_1rtt_protected;
        }
        19 => {
            ret = PICOQUIC_ERROR_DISCONNECTED;
        }
        _ => {
            ret = -(1 as ::core::ffi::c_int);
        }
    }
    if packet_type as ::core::ffi::c_uint
        == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
    {
        pkt_ctx = &raw mut (*path_x).pkt_ctx;
    } else {
        pkt_ctx = (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t).offset(pc as isize)
            as *mut picoquic_packet_context_t;
    }
    checksum_overhead = picoquic_get_checksum_length(cnx, epoch);
    (*packet).pc = pc;
    bytes_max = bytes
        .offset(send_buffer_max as isize)
        .offset(-(checksum_overhead as isize));
    if ret == 0 as ::core::ffi::c_int
        && (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_closing_received as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut exit_time: uint64_t = (*cnx)
            .latest_progress_time
            .wrapping_add((3 as uint64_t).wrapping_mul((*path_x).retransmit_timer));
        length = picoquic_predict_packet_header_length(cnx, packet_type, pkt_ctx);
        bytes_next = bytes.offset(length as isize);
        (*packet).ptype = packet_type;
        (*packet).offset = length;
        header_length = length;
        (*packet).sequence_number = (*pkt_ctx).send_sequence;
        (*packet).send_time = current_time;
        (*packet).send_path = path_x as *mut st_picoquic_path_t;
        bytes_next = picoquic_format_connection_close_frame(
            cnx,
            bytes_next,
            bytes_max,
            &raw mut more_data,
            &raw mut is_pure_ack,
        );
        length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
        (*cnx).last_close_sent = current_time;
        (*cnx).cnx_state = picoquic_state_draining;
        *next_wake_time = exit_time;
        (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
        (*(*cnx).quic).wake_line = 2442 as ::core::ffi::c_int;
    } else if ret == 0 as ::core::ffi::c_int
        && (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_closing as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut exit_time_0: uint64_t = (*cnx)
            .latest_progress_time
            .wrapping_add((3 as uint64_t).wrapping_mul((*path_x).retransmit_timer));
        let mut next_close_time: uint64_t =
            (*cnx).last_close_sent.wrapping_add((*path_x).smoothed_rtt);
        if current_time >= exit_time_0 {
            picoquic_connection_disconnect(cnx);
            *next_wake_time = current_time;
            (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
            (*(*cnx).quic).wake_line = 2451 as ::core::ffi::c_int;
        } else if current_time >= next_close_time {
            let mut delta_t: uint64_t = (*path_x).rtt_min;
            let mut next_time: uint64_t = 0 as uint64_t;
            if delta_t.wrapping_mul(2 as uint64_t) < (*path_x).retransmit_timer {
                delta_t = (*path_x).retransmit_timer.wrapping_div(2 as uint64_t);
            }
            if (*cnx).ack_ctx[pc as usize].act[0 as ::core::ffi::c_int as usize].ack_needed() != 0 {
                length = picoquic_predict_packet_header_length(cnx, packet_type, pkt_ctx);
                (*packet).ptype = packet_type;
                (*packet).offset = length;
                header_length = length;
                (*packet).sequence_number = (*pkt_ctx).send_sequence;
                (*packet).send_time = current_time;
                (*packet).send_path = path_x as *mut st_picoquic_path_t;
                bytes_next = bytes.offset(length as isize);
                if (*cnx).local_error == 0 as uint64_t {
                    bytes_next = picoquic_format_application_close_frame(
                        cnx,
                        bytes_next,
                        bytes_max,
                        &raw mut more_data,
                        &raw mut is_pure_ack,
                    );
                } else {
                    bytes_next = picoquic_format_connection_close_frame(
                        cnx,
                        bytes_next,
                        bytes_max,
                        &raw mut more_data,
                        &raw mut is_pure_ack,
                    );
                }
                length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
                (*cnx).ack_ctx[pc as usize].act[0 as ::core::ffi::c_int as usize]
                    .set_ack_needed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*cnx).ack_ctx[pc as usize].act[0 as ::core::ffi::c_int as usize]
                    .set_out_of_order_received(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*cnx).last_close_sent = current_time;
            }
            next_time = current_time.wrapping_add(delta_t);
            if next_time > exit_time_0 {
                next_time = exit_time_0;
            }
            *next_wake_time = next_time;
            (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
            (*(*cnx).quic).wake_line = 2489 as ::core::ffi::c_int;
        } else if *next_wake_time > exit_time_0 {
            *next_wake_time = exit_time_0;
            (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
            (*(*cnx).quic).wake_line = 2494 as ::core::ffi::c_int;
        }
    } else if ret == 0 as ::core::ffi::c_int
        && (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_draining as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut exit_time_1: uint64_t = (*cnx)
            .latest_progress_time
            .wrapping_add((3 as uint64_t).wrapping_mul((*path_x).retransmit_timer));
        if current_time >= exit_time_1 {
            picoquic_connection_disconnect(cnx);
            *next_wake_time = current_time;
            (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
            (*(*cnx).quic).wake_line = 2505 as ::core::ffi::c_int;
        } else {
            *next_wake_time = exit_time_1;
            (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
            (*(*cnx).quic).wake_line = 2509 as ::core::ffi::c_int;
        }
        length = 0 as size_t;
    } else if ret == 0 as ::core::ffi::c_int
        && ((*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_disconnecting as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*cnx).cnx_state as ::core::ffi::c_uint
                == picoquic_state_handshake_failure as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*cnx).cnx_state as ::core::ffi::c_uint
                == picoquic_state_handshake_failure_resend as ::core::ffi::c_int
                    as ::core::ffi::c_uint)
    {
        length = picoquic_predict_packet_header_length(cnx, packet_type, pkt_ctx);
        bytes_next = bytes.offset(length as isize);
        (*packet).ptype = packet_type;
        (*packet).offset = length;
        header_length = length;
        (*packet).sequence_number = (*pkt_ctx).send_sequence;
        (*packet).send_time = current_time;
        (*packet).send_path = path_x as *mut st_picoquic_path_t;
        let mut delta_t_0: uint64_t = (*path_x).rtt_min;
        if (2 as uint64_t).wrapping_mul(delta_t_0) < (*path_x).retransmit_timer {
            delta_t_0 = (*path_x).retransmit_timer.wrapping_div(2 as uint64_t);
        }
        bytes_next = picoquic_format_ack_frame(
            cnx,
            bytes_next,
            bytes_max,
            &raw mut more_data,
            current_time,
            pc,
            0 as ::core::ffi::c_int,
        );
        if (*cnx).local_error == 0 as uint64_t {
            bytes_next = picoquic_format_application_close_frame(
                cnx,
                bytes_next,
                bytes_max,
                &raw mut more_data,
                &raw mut is_pure_ack,
            );
        } else {
            bytes_next = picoquic_format_connection_close_frame(
                cnx,
                bytes_next,
                bytes_max,
                &raw mut more_data,
                &raw mut is_pure_ack,
            );
        }
        length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
        if (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_handshake_failure as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if pc as ::core::ffi::c_uint
                == picoquic_packet_context_initial as ::core::ffi::c_int as ::core::ffi::c_uint
                && !(*cnx).crypto_context[2 as ::core::ffi::c_int as usize]
                    .aead_encrypt
                    .is_null()
            {
                (*cnx).cnx_state = picoquic_state_handshake_failure_resend;
            } else {
                picoquic_connection_disconnect(cnx);
            }
        } else if (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_handshake_failure_resend as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            picoquic_connection_disconnect(cnx);
        } else {
            (*cnx).cnx_state = picoquic_state_closing;
        }
        (*cnx).latest_progress_time = current_time;
        (*cnx).last_close_sent = current_time;
        *next_wake_time = current_time.wrapping_add(delta_t_0);
        (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
        (*(*cnx).quic).wake_line = 2563 as ::core::ffi::c_int;
        (*cnx).ack_ctx[pc as usize].act[0 as ::core::ffi::c_int as usize]
            .set_ack_needed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    } else {
        length = 0 as size_t;
    }
    if length > 0 as size_t
        && (*packet).ptype as ::core::ffi::c_uint
            == picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*cnx).client_mode() as ::core::ffi::c_int != 0
    {
        length = picoquic_pad_to_target_length(
            bytes,
            length,
            send_buffer_max.wrapping_sub(checksum_overhead),
        );
    }
    picoquic_finalize_and_protect_packet(
        cnx,
        packet,
        ret,
        length,
        header_length,
        checksum_overhead,
        send_length,
        send_buffer,
        send_buffer_max,
        path_x,
        current_time,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_new_local_id_as_needed(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut current_time: uint64_t,
    mut next_wake_time: *mut uint64_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut no_space_left: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut local_cnxid_list: *mut picoquic_local_cnxid_list_t = (*cnx).first_local_cnxid_list;
    if (*cnx).is_multipath_enabled() != 0 {
        let mut new_max_path_id: uint64_t = (*cnx)
            .next_path_id_in_lists
            .wrapping_add((*cnx).local_parameters.initial_max_path_id)
            .wrapping_sub((*cnx).nb_local_cnxid_lists);
        if (*cnx).max_path_id_local < new_max_path_id {
            let mut bytes_next: *mut uint8_t =
                picoquic_format_max_path_id_frame(bytes, bytes_max, new_max_path_id, more_data);
            if bytes_next == bytes {
                no_space_left = 1 as ::core::ffi::c_int;
            } else {
                bytes = bytes_next;
                (*cnx).max_path_id_local = new_max_path_id;
            }
        }
        while no_space_left == 0
            && (*cnx).nb_local_cnxid_lists <= (*cnx).local_parameters.initial_max_path_id
            && (*cnx).next_path_id_in_lists <= (*cnx).max_path_id_remote
        {
            picoquic_find_or_create_local_cnxid_list(
                cnx,
                (*cnx).next_path_id_in_lists,
                1 as ::core::ffi::c_int,
            );
        }
    }
    while !local_cnxid_list.is_null() && no_space_left == 0 {
        picoquic_check_local_cnxid_ttl(cnx, local_cnxid_list, current_time, next_wake_time);
        while (*local_cnxid_list).nb_local_cnxid
            < (*cnx).remote_parameters.active_connection_id_limit as ::core::ffi::c_int
                + (*local_cnxid_list).nb_local_cnxid_expired
            && (*local_cnxid_list).nb_local_cnxid
                <= PICOQUIC_NB_PATH_TARGET + (*local_cnxid_list).nb_local_cnxid_expired
        {
            let mut bytes0: *mut uint8_t = bytes;
            let mut l_cid: *mut picoquic_local_cnxid_t = picoquic_create_local_cnxid(
                cnx,
                (*local_cnxid_list).unique_path_id,
                ::core::ptr::null_mut::<picoquic_connection_id_t>(),
                current_time,
            );
            if l_cid.is_null() {
                no_space_left = 1 as ::core::ffi::c_int;
                break;
            } else {
                bytes = picoquic_format_new_connection_id_frame(
                    cnx,
                    local_cnxid_list,
                    bytes,
                    bytes_max,
                    more_data,
                    is_pure_ack,
                    l_cid,
                );
                if !(bytes == bytes0) {
                    continue;
                }
                no_space_left = 1 as ::core::ffi::c_int;
                picoquic_delete_local_cnxid(cnx, l_cid);
                (*local_cnxid_list).local_cnxid_sequence_next = (*local_cnxid_list)
                    .local_cnxid_sequence_next
                    .wrapping_sub(1);
                break;
            }
        }
        local_cnxid_list = (*local_cnxid_list).next_list as *mut picoquic_local_cnxid_list_t;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_false_start_transition(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
) {
    (*cnx).cnx_state = picoquic_state_server_false_start;
    if (*cnx).client_mode() == 0
        && ((*(*cnx).quic).check_token() as ::core::ffi::c_int != 0
            || (*(*cnx).quic).provide_token() as ::core::ffi::c_int != 0)
    {
        let mut token_buffer: [uint8_t; 256] = [0; 256];
        let mut token_size: size_t = 0;
        let mut n_cid: picoquic_connection_id_t = picoquic_null_connection_id;
        if picoquic_prepare_retry_token(
            (*cnx).quic,
            &raw mut (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).peer_addr
                as *mut sockaddr,
            (current_time as ::core::ffi::c_ulonglong).wrapping_add(PICOQUIC_TOKEN_DELAY_LONG)
                as uint64_t,
            &raw mut n_cid,
            &raw mut n_cid,
            0 as uint32_t,
            &raw mut token_buffer as *mut uint8_t,
            ::core::mem::size_of::<[uint8_t; 256]>() as size_t,
            &raw mut token_size,
        ) == 0 as ::core::ffi::c_int
        {
            if picoquic_queue_new_token_frame(
                cnx,
                &raw mut token_buffer as *mut uint8_t,
                token_size,
            ) != 0 as ::core::ffi::c_int
            {
                picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t,
                    picoquic_frame_type_new_token as ::core::ffi::c_int as uint64_t,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_client_almost_ready_transition(mut cnx: *mut picoquic_cnx_t) {
    (*cnx).cnx_state = picoquic_state_client_almost_ready;
    if (*cnx).is_multipath_enabled() != 0 {
        let mut o_pkt_ctx: *mut picoquic_packet_context_t = (&raw mut (*cnx).pkt_ctx
            as *mut picoquic_packet_context_t)
            .offset(0 as ::core::ffi::c_int as isize)
            as *mut picoquic_packet_context_t;
        let mut n_pkt_ctx: *mut picoquic_packet_context_t =
            &raw mut (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).pkt_ctx;
        *n_pkt_ctx = *o_pkt_ctx;
        picoquic_init_packet_ctx(cnx, o_pkt_ctx, picoquic_packet_context_application);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_ready_state_transition(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
) {
    (*cnx).cnx_state = picoquic_state_ready;
    (*cnx).set_is_handshake_finished(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    picoquic_implicit_handshake_ack(cnx, picoquic_packet_context_initial, current_time);
    picoquic_implicit_handshake_ack(cnx, picoquic_packet_context_handshake, current_time);
    picoquic_register_net_secret(cnx);
    if (*(*cnx).quic).use_predictable_random() == 0 {
        picoquic_public_random_seed((*cnx).quic);
    }
    if (*cnx).client_mode() == 0 {
        picoquic_queue_handshake_done_frame(cnx);
    }
    if (*cnx).is_half_open() != 0 {
        if (*(*cnx).quic).current_number_half_open > 0 as uint32_t {
            (*(*cnx).quic).current_number_half_open =
                (*(*cnx).quic).current_number_half_open.wrapping_sub(1);
        }
        (*cnx).set_is_half_open(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if (*(*cnx).quic).current_number_half_open < (*(*cnx).quic).max_half_open_before_retry {
            (*(*cnx).quic)
                .set_check_token((*(*cnx).quic).force_check_token() as ::core::ffi::c_uint);
        }
    }
    picoquic_crypto_context_free(
        (&raw mut (*cnx).crypto_context as *mut picoquic_crypto_context_t)
            .offset(picoquic_epoch_initial as ::core::ffi::c_int as isize)
            as *mut picoquic_crypto_context_t,
    );
    picoquic_crypto_context_free(
        (&raw mut (*cnx).crypto_context as *mut picoquic_crypto_context_t)
            .offset(picoquic_epoch_0rtt as ::core::ffi::c_int as isize)
            as *mut picoquic_crypto_context_t,
    );
    picoquic_crypto_context_free(
        (&raw mut (*cnx).crypto_context as *mut picoquic_crypto_context_t)
            .offset(picoquic_epoch_handshake as ::core::ffi::c_int as isize)
            as *mut picoquic_crypto_context_t,
    );
    picoquic_purge_misc_frames_after_ready(cnx);
    picoquic_tlscontext_trim_after_handshake(cnx);
    if (*cnx).crypto_epoch_length_max == 0 as uint64_t {
        (*cnx).crypto_epoch_length_max = picoquic_aead_confidentiality_limit(
            (*cnx).crypto_context[picoquic_epoch_1rtt as ::core::ffi::c_int as usize].aead_decrypt,
        );
    }
    if (*cnx).client_mode() != 0 {
        picoquic_prepare_server_address_migration(cnx);
    }
    if (*cnx).callback_fn.is_some() {
        if (*cnx).callback_fn.expect("non-null function pointer")(
            cnx as *mut picoquic_cnx_t,
            0 as uint64_t,
            ::core::ptr::null_mut::<uint8_t>(),
            0 as size_t,
            picoquic_callback_ready,
            (*cnx).callback_ctx,
            NULL,
        ) != 0 as ::core::ffi::c_int
        {
            picoquic_log_app_message(
                cnx as *mut picoquic_cnx_t,
                b"Callback ready returns error 0x%x\0".as_ptr() as *const ::core::ffi::c_char,
                PICOQUIC_TRANSPORT_INTERNAL_ERROR,
            );
            picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint64_t,
                0 as uint64_t,
            );
        }
    }
    if (*cnx).is_ack_frequency_negotiated() != 0 {
        (*cnx).set_is_ack_frequency_updated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    } else {
        picoquic_compute_ack_gap_and_delay(
            cnx,
            (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).rtt_min,
            PICOQUIC_ACK_DELAY_MIN as uint64_t,
            (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).receive_rate_max,
            &raw mut (*cnx).ack_gap_remote,
            &raw mut (*cnx).ack_delay_remote,
        );
        if (*cnx).ack_gap_remote > (*cnx).max_ack_gap_remote {
            (*cnx).max_ack_gap_remote = (*cnx).ack_gap_remote;
        }
        if (*cnx).ack_delay_remote > (*cnx).max_ack_delay_remote {
            (*cnx).max_ack_delay_remote = (*cnx).ack_delay_remote;
        } else if (*cnx).ack_delay_remote < (*cnx).min_ack_delay_remote {
            (*cnx).min_ack_delay_remote = (*cnx).ack_delay_remote;
        }
    }
    picoquic_log_pn_dec_trial(cnx);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_path_challenge_frames(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut pc: picoquic_packet_context_enum,
    mut is_nominal_ack_path: ::core::ffi::c_int,
    mut bytes_next: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut is_challenge_padding_needed: *mut ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut next_wake_time: *mut uint64_t,
) -> *mut uint8_t {
    if (*path_x).challenge_verified() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
        && (*path_x).challenge_failed() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
    {
        let mut is_nat: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
        let mut next_challenge_time: uint64_t =
            picoquic_next_challenge_time(cnx, path_x, current_time, &raw mut is_nat);
        if next_challenge_time <= current_time {
            let mut ack_needed: ::core::ffi::c_int =
                (*cnx).ack_ctx[pc as usize].act[0 as ::core::ffi::c_int as usize].ack_needed()
                    as ::core::ffi::c_int;
            let mut bytes_challenge: *mut uint8_t = bytes_next;
            if is_nat != 0 {
                bytes_next = picoquic_format_path_challenge_frame(
                    bytes_next,
                    bytes_max,
                    more_data,
                    is_pure_ack,
                    (*path_x).nat_challenge[(*path_x).nat_challenge_repeat_count as usize],
                );
                if bytes_next > bytes_challenge {
                    (*path_x).nat_challenge_time = current_time;
                    (*path_x).nat_challenge_repeat_count =
                        (*path_x).nat_challenge_repeat_count.wrapping_add(1);
                    *is_challenge_padding_needed = 1 as ::core::ffi::c_int;
                }
            } else if ((*path_x).challenge_repeat_count as ::core::ffi::c_int)
                < PICOQUIC_CHALLENGE_REPEAT_MAX
            {
                bytes_next = picoquic_format_path_challenge_frame(
                    bytes_next,
                    bytes_max,
                    more_data,
                    is_pure_ack,
                    (*path_x).challenge[(*path_x).challenge_repeat_count as usize],
                );
                if bytes_next > bytes_challenge {
                    (*path_x).challenge_time = current_time;
                    (*path_x).challenge_repeat_count =
                        (*path_x).challenge_repeat_count.wrapping_add(1);
                    if (*path_x).is_nat_challenge() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    {
                        if (*cnx).client_mode() as ::core::ffi::c_int != 0
                            || (*path_x)
                                .bytes_sent
                                .wrapping_add(PICOQUIC_ENFORCED_INITIAL_MTU as uint64_t)
                                <= (*path_x).received
                        {
                            *is_challenge_padding_needed = 1 as ::core::ffi::c_int;
                        } else {
                            *is_challenge_padding_needed = 0 as ::core::ffi::c_int;
                        }
                    } else {
                        *is_challenge_padding_needed = 0 as ::core::ffi::c_int;
                    }
                }
                if ack_needed != 0 && is_nominal_ack_path != 0 {
                    bytes_next = picoquic_format_ack_frame(
                        cnx,
                        bytes_next,
                        bytes_max,
                        more_data,
                        current_time,
                        pc,
                        1 as ::core::ffi::c_int,
                    );
                }
                next_challenge_time =
                    picoquic_next_challenge_time(cnx, path_x, current_time, &raw mut is_nat);
                if next_challenge_time < *next_wake_time {
                    *next_wake_time = next_challenge_time;
                    (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
                    (*(*cnx).quic).wake_line = 2826 as ::core::ffi::c_int;
                }
            } else {
                if path_x == *(*cnx).path.offset(0 as ::core::ffi::c_int as isize) {
                    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    while i < (*cnx).nb_paths {
                        if (**(*cnx).path.offset(i as isize)).challenge_failed()
                            as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        {
                            let ref mut c2rust_fresh32 =
                                *(*cnx).path.offset(0 as ::core::ffi::c_int as isize);
                            *c2rust_fresh32 = *(*cnx).path.offset(i as isize);
                            let ref mut c2rust_fresh33 = *(*cnx).path.offset(i as isize);
                            *c2rust_fresh33 = path_x;
                            break;
                        } else {
                            i += 1;
                        }
                    }
                }
                if path_x == *(*cnx).path.offset(0 as ::core::ffi::c_int as isize) {
                    picoquic_log_app_message(
                        cnx as *mut picoquic_cnx_t,
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        b"Too many challenge retransmits, disconnect\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    (*cnx).local_error = PICOQUIC_ERROR_REPEAT_TIMEOUT as uint64_t;
                    picoquic_connection_disconnect(cnx);
                } else {
                    picoquic_log_app_message(
                        cnx as *mut picoquic_cnx_t,
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        b"Too many challenge retransmits, abandon path\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                    (*path_x).set_challenge_failed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*cnx)
                        .set_path_demotion_needed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
            }
        } else if next_challenge_time < *next_wake_time {
            *next_wake_time = next_challenge_time;
            (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
            (*(*cnx).quic).wake_line = 2858 as ::core::ffi::c_int;
        }
    }
    if (*path_x).response_required() != 0 {
        let mut bytes_response: *mut uint8_t = bytes_next;
        bytes_next = picoquic_format_path_response_frame(
            bytes_response,
            bytes_max,
            more_data,
            is_pure_ack,
            (*path_x).challenge_response,
        );
        if bytes_next > bytes_response {
            (*path_x).set_response_required(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            *is_challenge_padding_needed |= ((*cnx).client_mode() as ::core::ffi::c_int != 0
                || (*path_x)
                    .bytes_sent
                    .wrapping_add(PICOQUIC_ENFORCED_INITIAL_MTU as uint64_t)
                    <= (*path_x).received)
                as ::core::ffi::c_int;
        }
    }
    return bytes_next;
}
unsafe extern "C" fn picoquic_prepare_datagram_ready(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut bytes_next: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut datagram_tried_and_failed: *mut ::core::ffi::c_int,
    mut datagram_sent: *mut ::core::ffi::c_int,
    mut ret: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut bytes0: *mut uint8_t = bytes_next;
    if !(*cnx).first_datagram.is_null() {
        bytes_next = picoquic_format_first_datagram_frame(
            cnx,
            bytes_next,
            bytes_max,
            more_data,
            is_pure_ack,
        );
        *more_data |= ((*cnx).first_datagram != NULL as *mut picoquic_misc_frame_header_t)
            as ::core::ffi::c_int;
    } else {
        while (*cnx).is_datagram_ready() as ::core::ffi::c_int != 0
            || (*path_x).is_datagram_ready() as ::core::ffi::c_int != 0
        {
            let mut dg_start: *mut uint8_t = bytes_next;
            bytes_next = picoquic_format_ready_datagram_frame(
                cnx,
                path_x,
                bytes_next,
                bytes_max,
                more_data,
                is_pure_ack,
                ret,
            );
            if bytes_next.is_null() || bytes_next == dg_start {
                break;
            }
        }
    }
    *datagram_tried_and_failed = (bytes_next == bytes0) as ::core::ffi::c_int;
    *datagram_sent = (*datagram_tried_and_failed == 0) as ::core::ffi::c_int;
    return bytes_next;
}
unsafe extern "C" fn picoquic_prepare_stream_and_datagrams(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut bytes_next: *mut uint8_t,
    mut bytes_max: *mut uint8_t,
    mut max_priority_allowed: uint64_t,
    mut current_time: uint64_t,
    mut more_data: *mut ::core::ffi::c_int,
    mut is_pure_ack: *mut ::core::ffi::c_int,
    mut no_data_to_send: *mut ::core::ffi::c_int,
    mut ret: *mut ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut datagram_sent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut datagram_tried_and_failed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream_tried_and_failed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut more_data_this_round: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_first_round: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while bytes_next.offset(8 as ::core::ffi::c_int as isize) < bytes_max
        && *ret == 0 as ::core::ffi::c_int
    {
        let mut datagram_present: uint64_t = (!(*cnx).first_datagram.is_null()
            || (*cnx).is_datagram_ready() as ::core::ffi::c_int != 0
            || (*path_x).is_datagram_ready() as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int as uint64_t;
        let mut first_stream: *mut picoquic_stream_head_t = picoquic_find_ready_stream_path(
            cnx,
            if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0 {
                path_x
            } else {
                ::core::ptr::null_mut::<picoquic_path_t>()
            },
        );
        let mut first_repeat: *mut picoquic_packet_t = picoquic_first_data_repeat_packet(cnx);
        let mut current_priority: uint64_t = UINT64_MAX as uint64_t;
        let mut stream_priority: uint64_t = UINT64_MAX as uint64_t;
        let mut bytes_before_iteration: *mut uint8_t = bytes_next;
        let mut something_sent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        let mut conflict_found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        more_data_this_round = 0 as ::core::ffi::c_int;
        let mut datagram_first: ::core::ffi::c_int = ((*cnx).datagram_conflicts_max
            >= (*cnx).datagram_conflicts_count)
            as ::core::ffi::c_int;
        if datagram_present != 0 {
            current_priority = (*cnx).datagram_priority;
        }
        if !first_stream.is_null() {
            stream_priority = (*first_stream).stream_priority as uint64_t;
        }
        if !first_repeat.is_null() && (*first_repeat).data_repeat_priority < stream_priority {
            stream_priority = (*first_repeat).data_repeat_priority;
        }
        if stream_priority < current_priority {
            current_priority = stream_priority;
        }
        if current_priority == UINT64_MAX as uint64_t || current_priority >= max_priority_allowed {
            if is_first_round != 0 {
                *no_data_to_send = 1 as ::core::ffi::c_int;
            }
            break;
        } else {
            if datagram_present != 0
                && (*cnx).datagram_priority == current_priority
                && ((*cnx).datagram_priority < stream_priority || datagram_first != 0)
            {
                bytes_next = picoquic_prepare_datagram_ready(
                    cnx,
                    path_x,
                    bytes_next,
                    bytes_max,
                    &raw mut more_data_this_round,
                    is_pure_ack,
                    &raw mut datagram_tried_and_failed,
                    &raw mut datagram_sent,
                    ret,
                );
                something_sent = datagram_sent;
            }
            if !first_repeat.is_null() && (*first_repeat).data_repeat_priority == current_priority {
                let mut bytes_first: *mut uint8_t = bytes_next;
                if bytes_next.offset(8 as ::core::ffi::c_int as isize) < bytes_max {
                    bytes_next = picoquic_copy_stream_frames_for_retransmit(
                        cnx,
                        bytes_next,
                        bytes_max,
                        UINT64_MAX as uint64_t,
                        &raw mut more_data_this_round,
                        is_pure_ack,
                    );
                    if bytes_next > bytes_first {
                        (*cnx).datagram_conflicts_count = 0 as ::core::ffi::c_int;
                        something_sent = 1 as ::core::ffi::c_int;
                    }
                } else {
                    more_data_this_round |= 1 as ::core::ffi::c_int;
                    conflict_found = 1 as ::core::ffi::c_int;
                }
            }
            if !first_stream.is_null()
                && (*first_stream).stream_priority as uint64_t == current_priority
            {
                let mut bytes_first_0: *mut uint8_t = bytes_next;
                if bytes_next.offset(8 as ::core::ffi::c_int as isize) < bytes_max {
                    bytes_next = picoquic_format_available_stream_frames(
                        cnx,
                        path_x,
                        bytes_next,
                        bytes_max,
                        UINT64_MAX as uint64_t,
                        &raw mut more_data_this_round,
                        is_pure_ack,
                        &raw mut stream_tried_and_failed,
                        ret,
                    );
                    if bytes_next > bytes_first_0 {
                        (*cnx).datagram_conflicts_count = 0 as ::core::ffi::c_int;
                        something_sent = 1 as ::core::ffi::c_int;
                    }
                } else {
                    more_data_this_round |= 1 as ::core::ffi::c_int;
                    conflict_found = 1 as ::core::ffi::c_int;
                }
            }
            if datagram_sent != 0 && conflict_found != 0 {
                (*cnx).datagram_conflicts_count += 1 as ::core::ffi::c_int;
            }
            if datagram_present != 0
                && (*cnx).datagram_priority == current_priority
                && (*cnx).datagram_priority <= stream_priority
                && datagram_first == 0
            {
                bytes_next = picoquic_prepare_datagram_ready(
                    cnx,
                    path_x,
                    bytes_next,
                    bytes_max,
                    more_data,
                    is_pure_ack,
                    &raw mut datagram_tried_and_failed,
                    &raw mut datagram_sent,
                    ret,
                );
                something_sent = datagram_sent;
            }
            if current_priority < (*cnx).priority_limit_for_bypass
                && bytes_next > bytes_before_iteration
            {
                picoquic_update_pacing_data_after_send(
                    &raw mut (*cnx).priority_bypass_pacing,
                    bytes_next.offset_from(bytes_before_iteration) as ::core::ffi::c_long as size_t,
                    (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).send_mtu,
                    current_time,
                );
            }
            if is_first_round != 0 {
                *no_data_to_send = ((first_stream.is_null() && first_repeat.is_null()
                    || stream_tried_and_failed != 0)
                    && (datagram_present == 0 || datagram_tried_and_failed != 0))
                    as ::core::ffi::c_int;
            }
            is_first_round = 0 as ::core::ffi::c_int;
            if something_sent == 0 {
                break;
            }
        }
    }
    *more_data |= more_data_this_round;
    return bytes_next;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_packet_almost_ready(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut packet: *mut picoquic_packet_t,
    mut current_time: uint64_t,
    mut send_buffer: *mut uint8_t,
    mut send_buffer_max: size_t,
    mut send_length: *mut size_t,
    mut next_wake_time: *mut uint64_t,
    mut is_initial_sent: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut packet_type: picoquic_packet_type_enum = picoquic_packet_1rtt_protected;
    let mut pc: picoquic_packet_context_enum = picoquic_packet_context_application;
    let mut tls_ready: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_pure_ack: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut header_length: size_t = 0 as size_t;
    let mut bytes: *mut uint8_t = &raw mut (*packet).bytes as *mut uint8_t;
    let mut length: size_t = 0 as size_t;
    let mut checksum_overhead: size_t = picoquic_get_checksum_length(cnx, picoquic_epoch_1rtt);
    let mut send_buffer_min_max: size_t = if send_buffer_max > (*path_x).send_mtu {
        (*path_x).send_mtu
    } else {
        send_buffer_max
    };
    let mut bytes_max: *mut uint8_t = bytes
        .offset(send_buffer_min_max as isize)
        .offset(-(checksum_overhead as isize));
    let mut bytes_next: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut more_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut no_data_to_send: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_challenge_padding_needed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*cnx).initial_validated() == 0
        && (*cnx)
            .initial_data_sent
            .wrapping_add(send_buffer_min_max as uint64_t)
            > (3 as uint64_t).wrapping_mul((*cnx).initial_data_received)
    {
        *send_length = 0 as size_t;
        return 0 as ::core::ffi::c_int;
    }
    if !(*path_x).p_local_cnxid.is_null() {
        if !(*cnx).crypto_context[picoquic_epoch_initial as ::core::ffi::c_int as usize]
            .aead_encrypt
            .is_null()
        {
            length = picoquic_prepare_packet_old_context(
                cnx,
                picoquic_packet_context_initial,
                path_x,
                packet,
                send_buffer_min_max,
                current_time,
                next_wake_time,
                &raw mut header_length,
            );
        } else {
            length = 0 as size_t;
        }
        if length == 0 as size_t {
            length = picoquic_prepare_packet_old_context(
                cnx,
                picoquic_packet_context_handshake,
                path_x,
                packet,
                send_buffer_min_max,
                current_time,
                next_wake_time,
                &raw mut header_length,
            );
            if length == 0 as size_t
                && *is_initial_sent == 1 as ::core::ffi::c_int
                && (*cnx).cnx_state as ::core::ffi::c_uint
                    == picoquic_state_server_false_start as ::core::ffi::c_int
                        as ::core::ffi::c_uint
            {
                (*packet).ptype = picoquic_packet_handshake;
                length = picoquic_predict_packet_header_length(
                    cnx,
                    (*packet).ptype,
                    (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t)
                        .offset(picoquic_packet_context_handshake as ::core::ffi::c_int as isize)
                        as *mut picoquic_packet_context_t,
                );
                header_length = length;
                (*packet).offset = length;
                (*packet).sequence_number = (*cnx).pkt_ctx
                    [picoquic_packet_context_handshake as ::core::ffi::c_int as usize]
                    .send_sequence;
                (*packet).send_time = current_time;
                (*packet).send_path = path_x as *mut st_picoquic_path_t;
                (*packet).bytes[length as usize] =
                    picoquic_frame_type_ping as ::core::ffi::c_int as uint8_t;
                length = length.wrapping_add(1);
                checksum_overhead = picoquic_get_checksum_length(cnx, picoquic_epoch_handshake);
                (*packet).checksum_overhead = checksum_overhead;
                (*packet).pc = picoquic_packet_context_handshake;
                is_pure_ack = 0 as ::core::ffi::c_int;
                *is_initial_sent += 1 as ::core::ffi::c_int;
            }
            if length > 0 as size_t {
                checksum_overhead = picoquic_get_checksum_length(cnx, picoquic_epoch_handshake);
                bytes_max = bytes
                    .offset(send_buffer_min_max as isize)
                    .offset(-(checksum_overhead as isize));
            }
        } else {
            checksum_overhead = picoquic_get_checksum_length(cnx, picoquic_epoch_initial);
            bytes_max = bytes
                .offset(send_buffer_min_max as isize)
                .offset(-(checksum_overhead as isize));
            *is_initial_sent = 1 as ::core::ffi::c_int;
        }
        if length > 0 as size_t {
            (*cnx).set_initial_repeat_needed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            if (*cnx).client_mode() as ::core::ffi::c_int != 0
                && *is_initial_sent != 0
                && send_buffer_min_max
                    < length
                        .wrapping_add(checksum_overhead)
                        .wrapping_add(PICOQUIC_MIN_SEGMENT_SIZE as size_t)
            {
                length = picoquic_pad_to_target_length(
                    &raw mut (*packet).bytes as *mut uint8_t,
                    length,
                    send_buffer_min_max.wrapping_sub(checksum_overhead),
                );
            }
        }
    }
    if length == 0 as size_t {
        let mut pkt_ctx: *mut picoquic_packet_context_t =
            (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t).offset(pc as isize)
                as *mut picoquic_packet_context_t;
        if (*cnx).is_multipath_enabled() != 0 {
            pkt_ctx = &raw mut (*path_x).pkt_ctx;
        }
        tls_ready = picoquic_is_tls_stream_ready(cnx);
        (*packet).pc = pc;
        length = picoquic_predict_packet_header_length(cnx, packet_type, pkt_ctx);
        (*packet).ptype = packet_type;
        (*packet).offset = length;
        header_length = length;
        (*packet).sequence_number = (*pkt_ctx).send_sequence;
        (*packet).send_time = current_time;
        (*packet).send_path = path_x as *mut st_picoquic_path_t;
        bytes_next = bytes.offset(length as isize);
        bytes_next = picoquic_prepare_path_challenge_frames(
            cnx,
            path_x,
            pc,
            1 as ::core::ffi::c_int,
            bytes_next,
            bytes_max,
            &raw mut more_data,
            &raw mut is_pure_ack,
            &raw mut is_challenge_padding_needed,
            current_time,
            next_wake_time,
        );
        length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
        if (*cnx).cnx_state as ::core::ffi::c_uint
            != picoquic_state_disconnected as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*path_x).challenge_verified() as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            if picoquic_is_sending_authorized_by_pacing(cnx, path_x, current_time, next_wake_time)
                != 0
            {
                if length <= header_length
                    && (*cnx).client_mode() as ::core::ffi::c_int != 0
                    && picoquic_find_first_misc_frame(cnx, pc).is_null()
                    && {
                        length = picoquic_retransmit_needed(
                            cnx,
                            pc,
                            path_x,
                            current_time,
                            next_wake_time,
                            packet,
                            send_buffer_min_max,
                            &raw mut header_length,
                        ) as size_t;
                        length > 0 as size_t
                    }
                {
                    if bytes
                        .offset(length as isize)
                        .offset(256 as ::core::ffi::c_int as isize)
                        < bytes_max
                        && length > header_length
                    {
                        bytes_next = picoquic_format_ack_frame(
                            cnx,
                            bytes.offset(length as isize),
                            bytes_max,
                            &raw mut more_data,
                            current_time,
                            pc,
                            0 as ::core::ffi::c_int,
                        );
                        length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
                    }
                    is_pure_ack = 0 as ::core::ffi::c_int;
                    (*packet).send_time = current_time;
                    (*packet).checksum_overhead = checksum_overhead;
                }
                if picoquic_is_ack_needed(
                    cnx,
                    current_time,
                    next_wake_time,
                    pc,
                    0 as ::core::ffi::c_int,
                ) != 0
                {
                    bytes_next = picoquic_format_ack_frame(
                        cnx,
                        bytes_next,
                        bytes_max,
                        &raw mut more_data,
                        current_time,
                        pc,
                        0 as ::core::ffi::c_int,
                    );
                }
                length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
                if (*path_x).cwin < (*path_x).bytes_in_transit {
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
                    (*cnx).set_cwin_blocked(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*path_x).last_cwin_blocked_time = current_time;
                    if !(*cnx).congestion_alg.is_null() {
                        (*(*cnx).congestion_alg)
                            .alg_notify
                            .expect("non-null function pointer")(
                            cnx as *mut picoquic_cnx_t,
                            path_x as *mut picoquic_path_t,
                            picoquic_congestion_notification_cwin_blocked,
                            &raw mut ack_state,
                            current_time,
                        );
                    }
                } else {
                    let mut pmtu_discovery_needed: picoquic_pmtu_discovery_status_enum =
                        picoquic_is_mtu_probe_needed(cnx, path_x);
                    if tls_ready != 0 {
                        bytes_next = picoquic_format_crypto_hs_frame(
                            (&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t)
                                .offset(picoquic_epoch_1rtt as ::core::ffi::c_int as isize)
                                as *mut picoquic_stream_head_t,
                            bytes_next,
                            bytes_max,
                            &raw mut more_data,
                            &raw mut is_pure_ack,
                        );
                    }
                    if pc as ::core::ffi::c_uint
                        != picoquic_packet_context_application as ::core::ffi::c_int
                            as ::core::ffi::c_uint
                    {
                        bytes_next = picoquic_format_misc_frames_in_context(
                            cnx,
                            bytes_next,
                            bytes_max,
                            &raw mut more_data,
                            &raw mut is_pure_ack,
                            pc,
                        );
                        length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
                    } else {
                        length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
                        if length > header_length
                            || pmtu_discovery_needed as ::core::ffi::c_uint
                                != picoquic_pmtu_discovery_required as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            || send_buffer_max <= (*path_x).send_mtu
                        {
                            bytes_next = picoquic_format_misc_frames_in_context(
                                cnx,
                                bytes_next,
                                bytes_max,
                                &raw mut more_data,
                                &raw mut is_pure_ack,
                                pc,
                            );
                            if (*cnx).is_address_discovery_provider() != 0 {
                                bytes_next = picoquic_prepare_observed_address_frame(
                                    bytes_next,
                                    bytes_max,
                                    path_x,
                                    current_time,
                                    next_wake_time,
                                    &raw mut more_data,
                                    &raw mut is_pure_ack,
                                );
                            }
                            if ret == 0 as ::core::ffi::c_int {
                                bytes_next = picoquic_format_new_local_id_as_needed(
                                    cnx,
                                    bytes_next,
                                    bytes_max,
                                    current_time,
                                    next_wake_time,
                                    &raw mut more_data,
                                    &raw mut is_pure_ack,
                                );
                            }
                            if (*cnx).is_ack_frequency_updated() as ::core::ffi::c_int != 0
                                && (*cnx).is_ack_frequency_negotiated() as ::core::ffi::c_int != 0
                            {
                                bytes_next = picoquic_format_ack_frequency_frame(
                                    cnx,
                                    bytes_next,
                                    bytes_max,
                                    &raw mut more_data,
                                );
                            }
                            if ret == 0 as ::core::ffi::c_int {
                                bytes_next = picoquic_prepare_stream_and_datagrams(
                                    cnx,
                                    path_x,
                                    bytes_next,
                                    bytes_max,
                                    UINT64_MAX as uint64_t,
                                    current_time,
                                    &raw mut more_data,
                                    &raw mut is_pure_ack,
                                    &raw mut no_data_to_send,
                                    &raw mut ret,
                                );
                            }
                            if (*cnx).client_mode() == 0
                                && (*cnx).send_receive_bdp_frame() as ::core::ffi::c_int != 0
                            {
                                bytes_next = picoquic_format_bdp_frame(
                                    cnx,
                                    bytes_next,
                                    bytes_max,
                                    path_x,
                                    &raw mut more_data,
                                    &raw mut is_pure_ack,
                                );
                            }
                            length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
                            if length <= header_length {
                                (*path_x).delivered_limited_index = (*path_x).delivered;
                                bytes_next = picoquic_format_blocked_frames(
                                    cnx,
                                    bytes.offset(length as isize) as *mut uint8_t,
                                    bytes_max,
                                    &raw mut more_data,
                                    &raw mut is_pure_ack,
                                );
                                length =
                                    bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
                            }
                            if no_data_to_send != 0 {
                                (*path_x).last_sender_limited_time = current_time;
                            }
                        }
                        if ret == 0 as ::core::ffi::c_int
                            && length <= header_length
                            && (*path_x).cwin > (*path_x).bytes_in_transit
                            && (*(*cnx).quic).cwin_max > (*path_x).bytes_in_transit
                            && pmtu_discovery_needed as ::core::ffi::c_uint
                                != picoquic_pmtu_discovery_not_needed as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                        {
                            if send_buffer_max > (*path_x).send_mtu {
                                length = picoquic_prepare_mtu_probe(
                                    cnx,
                                    path_x,
                                    header_length,
                                    checksum_overhead,
                                    bytes,
                                    send_buffer_max,
                                );
                                (*packet).length = length;
                                (*packet).send_path = path_x as *mut st_picoquic_path_t;
                                (*packet).set_is_mtu_probe(
                                    1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                );
                                (*path_x).set_mtu_probe_sent(
                                    1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                );
                                is_pure_ack = 0 as ::core::ffi::c_int;
                            } else if (*cnx).is_sending_large_buffer() != 0 {
                                *next_wake_time = current_time;
                                (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
                                (*(*cnx).quic).wake_line = 3308 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
        if length <= header_length {
            length = 0 as size_t;
        }
        if (*cnx).cnx_state as ::core::ffi::c_uint
            != picoquic_state_disconnected as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if is_pure_ack == 0 as ::core::ffi::c_int {
                (*cnx).latest_progress_time = current_time;
            } else if (*cnx).keep_alive_interval != 0 as uint64_t {
                if (*cnx)
                    .latest_progress_time
                    .wrapping_add((*cnx).keep_alive_interval)
                    <= current_time
                    && length == 0 as size_t
                {
                    length = picoquic_predict_packet_header_length(cnx, packet_type, pkt_ctx);
                    (*packet).ptype = packet_type;
                    (*packet).pc = pc;
                    (*packet).offset = length;
                    header_length = length;
                    (*packet).sequence_number = (*pkt_ctx).send_sequence;
                    (*packet).send_path = path_x as *mut st_picoquic_path_t;
                    (*packet).send_time = current_time;
                    let c2rust_fresh35 = length;
                    length = length.wrapping_add(1);
                    *bytes.offset(c2rust_fresh35 as isize) =
                        picoquic_frame_type_ping as ::core::ffi::c_int as uint8_t;
                    let c2rust_fresh36 = length;
                    length = length.wrapping_add(1);
                    *bytes.offset(c2rust_fresh36 as isize) = 0 as uint8_t;
                    (*cnx).latest_progress_time = current_time;
                } else if (*cnx)
                    .latest_progress_time
                    .wrapping_add((*cnx).keep_alive_interval)
                    < *next_wake_time
                {
                    *next_wake_time = (*cnx)
                        .latest_progress_time
                        .wrapping_add((*cnx).keep_alive_interval);
                    (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
                    (*(*cnx).quic).wake_line = 3344 as ::core::ffi::c_int;
                }
            }
        }
    }
    if ret == 0 as ::core::ffi::c_int && length > header_length {
        if more_data != 0 {
            *next_wake_time = current_time;
            (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
            (*(*cnx).quic).wake_line = 3353 as ::core::ffi::c_int;
            ret = 0 as ::core::ffi::c_int;
        }
        if *is_initial_sent != 0
            && ((*packet).ptype as ::core::ffi::c_uint
                != picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint
                || length
                    .wrapping_add(checksum_overhead)
                    .wrapping_add(PICOQUIC_MIN_SEGMENT_SIZE as size_t)
                    > send_buffer_min_max
                || (*(*cnx).quic).dont_coalesce_init() as ::core::ffi::c_int != 0)
            || is_challenge_padding_needed != 0 && length < PICOQUIC_ENFORCED_INITIAL_MTU as size_t
        {
            length = picoquic_pad_to_target_length(
                bytes,
                length,
                send_buffer_min_max.wrapping_sub(checksum_overhead) as uint32_t as size_t,
            );
        } else {
            length = picoquic_pad_to_policy(
                cnx,
                bytes,
                length,
                send_buffer_min_max.wrapping_sub(checksum_overhead) as uint32_t,
            );
        }
    }
    picoquic_finalize_and_protect_packet(
        cnx,
        packet,
        ret,
        length,
        header_length,
        checksum_overhead,
        send_length,
        send_buffer,
        send_buffer_min_max,
        path_x,
        current_time,
    );
    if *send_length > 0 as size_t {
        if (*cnx).initial_validated() == 0 {
            (*cnx).initial_data_sent = ((*cnx).initial_data_sent as ::core::ffi::c_ulong)
                .wrapping_add(*send_length as ::core::ffi::c_ulong)
                as uint64_t as uint64_t;
        }
        *next_wake_time = current_time;
        (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
        (*(*cnx).quic).wake_line = 3379 as ::core::ffi::c_int;
        if picoquic_cnx_is_still_logging(cnx as *mut picoquic_cnx_t) != 0 {
            picoquic_log_cc_dump(cnx, current_time);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_packet_ready(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut packet: *mut picoquic_packet_t,
    mut current_time: uint64_t,
    mut send_buffer: *mut uint8_t,
    mut send_buffer_max: size_t,
    mut send_length: *mut size_t,
    mut next_wake_time: *mut uint64_t,
    mut is_initial_sent: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut packet_type: picoquic_packet_type_enum = picoquic_packet_1rtt_protected;
    let mut pc: picoquic_packet_context_enum = picoquic_packet_context_application;
    let mut is_pure_ack: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    let mut header_length: size_t = 0 as size_t;
    let mut length: size_t = 0 as size_t;
    let mut checksum_overhead: size_t = picoquic_get_checksum_length(cnx, picoquic_epoch_1rtt);
    let mut send_buffer_min_max: size_t = if send_buffer_max > (*path_x).send_mtu {
        (*path_x).send_mtu
    } else {
        send_buffer_max
    };
    let mut bytes: *mut uint8_t = &raw mut (*packet).bytes as *mut uint8_t;
    let mut bytes_max: *mut uint8_t = bytes
        .offset(send_buffer_min_max as isize)
        .offset(-(checksum_overhead as isize));
    let mut bytes_next: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut more_data: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ack_sent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_challenge_padding_needed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_nominal_ack_path: ::core::ffi::c_int =
        if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0 {
            ((*path_x).is_nominal_ack_path() as ::core::ffi::c_int != 0
                || (*cnx).nb_paths == 1 as ::core::ffi::c_int) as ::core::ffi::c_int
        } else {
            (path_x == *(*cnx).path.offset(0 as ::core::ffi::c_int as isize)) as ::core::ffi::c_int
        };
    let mut pkt_ctx: *mut picoquic_packet_context_t =
        if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0 {
            &raw mut (*path_x).pkt_ctx
        } else {
            (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t)
                .offset(picoquic_packet_context_application as ::core::ffi::c_int as isize)
                as *mut picoquic_packet_context_t
        };
    if (*pkt_ctx).send_sequence >= (*pkt_ctx).next_sequence_hole {
        picoquic_insert_hole_in_send_sequence_if_needed(
            cnx,
            path_x,
            pkt_ctx,
            current_time,
            next_wake_time,
        );
    }
    (*packet).pc = picoquic_packet_context_application;
    if (*cnx).client_mode() as ::core::ffi::c_int != 0
        && (*path_x).challenge_verified() as ::core::ffi::c_int != 0
        && (*path_x).path_cid_rotated() == 0
        && ((*path_x).latest_sent_time as ::core::ffi::c_ulonglong)
            .wrapping_add(PICOQUIC_CID_REFRESH_DELAY)
            < current_time as ::core::ffi::c_ulonglong
        && (*path_x)
            .latest_sent_time
            .wrapping_add((3 as uint64_t).wrapping_mul((*path_x).rtt_min))
            < current_time
    {
        picoquic_renew_path_connection_id(cnx, path_x);
        (*path_x).set_path_cid_rotated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if (*cnx)
        .nb_packets_sent
        .wrapping_sub((*cnx).crypto_epoch_sequence)
        > (*cnx).crypto_epoch_length_max
        && current_time > (*cnx).crypto_rotation_time_guard
    {
        if picoquic_start_key_rotation(cnx as *mut picoquic_cnx_t) != 0 as ::core::ffi::c_int {
            picoquic_log_app_message(
                cnx as *mut picoquic_cnx_t,
                b"Cannot start key rotation after %lu packets\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*cnx).pkt_ctx[picoquic_packet_context_application as ::core::ffi::c_int as usize]
                    .send_sequence,
            );
        }
    }
    if (*cnx).first_misc_frame.is_null() && {
        length = picoquic_retransmit_needed(
            cnx,
            pc,
            path_x,
            current_time,
            next_wake_time,
            packet,
            send_buffer_min_max,
            &raw mut header_length,
        ) as size_t;
        length > 0 as size_t
    } {
        if bytes
            .offset(length as isize)
            .offset(256 as ::core::ffi::c_int as isize)
            < bytes_max
            && length > header_length
        {
            bytes_next = picoquic_format_ack_frame(
                cnx,
                bytes.offset(length as isize),
                bytes_max,
                &raw mut more_data,
                current_time,
                pc,
                (is_nominal_ack_path == 0) as ::core::ffi::c_int,
            );
            length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
        }
        is_pure_ack = 0 as ::core::ffi::c_int;
        (*packet).send_time = current_time;
        (*packet).checksum_overhead = checksum_overhead;
    } else if !((*cnx).cnx_state as ::core::ffi::c_uint
        == picoquic_state_disconnected as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        length = picoquic_predict_packet_header_length(cnx, packet_type, pkt_ctx);
        (*packet).ptype = packet_type;
        (*packet).offset = length;
        header_length = length;
        (*packet).sequence_number = (*pkt_ctx).send_sequence;
        (*packet).send_time = current_time;
        (*packet).send_path = path_x as *mut st_picoquic_path_t;
        bytes_next = bytes.offset(length as isize);
        bytes_next = picoquic_prepare_path_challenge_frames(
            cnx,
            path_x,
            pc,
            is_nominal_ack_path,
            bytes_next,
            bytes_max,
            &raw mut more_data,
            &raw mut is_pure_ack,
            &raw mut is_challenge_padding_needed,
            current_time,
            next_wake_time,
        );
        length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
        if (*path_x).is_multipath_probe_needed() != 0 {
            (*packet).set_is_multipath_probe(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*path_x)
                .set_is_multipath_probe_needed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            is_pure_ack = 0 as ::core::ffi::c_int;
            *bytes_next = picoquic_frame_type_ping as ::core::ffi::c_int as uint8_t;
            length = length.wrapping_add(1);
            length = picoquic_pad_to_target_length(
                bytes,
                length,
                send_buffer_min_max.wrapping_sub(checksum_overhead) as uint32_t as size_t,
            );
            bytes_next = bytes.offset(length as isize);
        } else if (*cnx).cnx_state as ::core::ffi::c_uint
            != picoquic_state_disconnected as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*path_x).challenge_verified() as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        {
            if picoquic_is_sending_authorized_by_pacing(cnx, path_x, current_time, next_wake_time)
                != 0
            {
                if picoquic_is_ack_needed(
                    cnx,
                    current_time,
                    next_wake_time,
                    pc,
                    (is_nominal_ack_path == 0) as ::core::ffi::c_int,
                ) != 0
                {
                    let mut bytes_ack: *mut uint8_t = bytes_next;
                    bytes_next = picoquic_format_ack_frame(
                        cnx,
                        bytes_next,
                        bytes_max,
                        &raw mut more_data,
                        current_time,
                        pc,
                        (is_nominal_ack_path == 0) as ::core::ffi::c_int,
                    );
                    ack_sent = (bytes_next > bytes_ack) as ::core::ffi::c_int;
                }
                if ret == 0 as ::core::ffi::c_int {
                    bytes_next = picoquic_format_max_streams_frame_if_needed(
                        cnx,
                        bytes_next,
                        bytes_max,
                        &raw mut more_data,
                        &raw mut is_pure_ack,
                    );
                }
                if ret == 0 as ::core::ffi::c_int {
                    if (*(*cnx).quic).max_data_limit != 0 as uint64_t {
                        if (*cnx).data_received.wrapping_add(
                            (3 as uint64_t)
                                .wrapping_mul((*(*cnx).quic).max_data_limit)
                                .wrapping_div(4 as uint64_t),
                        ) > (*cnx).maxdata_local
                        {
                            let mut max_data_increase: uint64_t = (*cnx)
                                .data_received
                                .wrapping_add((*(*cnx).quic).max_data_limit)
                                .wrapping_sub((*cnx).maxdata_local);
                            bytes_next = picoquic_format_max_data_frame(
                                cnx,
                                bytes_next,
                                bytes_max,
                                &raw mut more_data,
                                &raw mut is_pure_ack,
                                max_data_increase,
                            );
                        }
                    } else if (2 as uint64_t).wrapping_mul((*cnx).data_received)
                        > (*cnx).maxdata_local
                    {
                        bytes_next = picoquic_format_max_data_frame(
                            cnx,
                            bytes_next,
                            bytes_max,
                            &raw mut more_data,
                            &raw mut is_pure_ack,
                            picoquic_cc_increased_window(cnx, (*cnx).maxdata_local),
                        );
                    }
                }
                if ret == 0 as ::core::ffi::c_int
                    && (*cnx).max_stream_data_needed() as ::core::ffi::c_int != 0
                {
                    bytes_next = picoquic_format_required_max_stream_data_frames(
                        cnx,
                        bytes_next,
                        bytes_max,
                        &raw mut more_data,
                        &raw mut is_pure_ack,
                    );
                }
                if !(*cnx).first_misc_frame.is_null() {
                    more_data = 1 as ::core::ffi::c_int;
                }
                bytes_next = picoquic_format_misc_frames_in_context(
                    cnx,
                    bytes_next,
                    bytes_max,
                    &raw mut more_data,
                    &raw mut is_pure_ack,
                    pc,
                );
                length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
                if ((*path_x).cwin < (*path_x).bytes_in_transit
                    || (*(*cnx).quic).cwin_max < (*path_x).bytes_in_transit)
                    && (*path_x).is_pto_required() == 0
                {
                    let mut bytes_next_before_bypass: *mut uint8_t = bytes_next;
                    let mut no_data_to_send: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    if (*cnx).priority_limit_for_bypass > 0 as uint64_t
                        && (*cnx).nb_paths == 1 as ::core::ffi::c_int
                        && picoquic_is_authorized_by_pacing(
                            &raw mut (*cnx).priority_bypass_pacing,
                            current_time,
                            next_wake_time,
                            (*(*cnx).quic).packet_train_mode(),
                            (*cnx).quic,
                        ) != 0
                    {
                        bytes_next = picoquic_prepare_stream_and_datagrams(
                            cnx,
                            path_x,
                            bytes_next,
                            bytes_max,
                            (*cnx).priority_limit_for_bypass,
                            current_time,
                            &raw mut more_data,
                            &raw mut is_pure_ack,
                            &raw mut no_data_to_send,
                            &raw mut ret,
                        );
                    }
                    if bytes_next != bytes_next_before_bypass {
                        length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
                    } else {
                        (*cnx).set_cwin_blocked(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        (*path_x).last_cwin_blocked_time = current_time;
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
                            (*(*cnx).congestion_alg)
                                .alg_notify
                                .expect("non-null function pointer")(
                                cnx as *mut picoquic_cnx_t,
                                path_x as *mut picoquic_path_t,
                                picoquic_congestion_notification_cwin_blocked,
                                &raw mut ack_state,
                                current_time,
                            );
                        }
                    }
                } else {
                    let mut no_data_to_send_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
                    let mut preemptive_repeat: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut pmtu_discovery_needed: picoquic_pmtu_discovery_status_enum =
                        picoquic_is_mtu_probe_needed(cnx, path_x);
                    if picoquic_is_tls_stream_ready(cnx) != 0 {
                        bytes_next = picoquic_format_crypto_hs_frame(
                            (&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t)
                                .offset(picoquic_epoch_1rtt as ::core::ffi::c_int as isize)
                                as *mut picoquic_stream_head_t,
                            bytes_next,
                            bytes_max,
                            &raw mut more_data,
                            &raw mut is_pure_ack,
                        );
                    }
                    if (*cnx).is_address_discovery_provider() != 0 {
                        bytes_next = picoquic_prepare_observed_address_frame(
                            bytes_next,
                            bytes_max,
                            path_x,
                            current_time,
                            next_wake_time,
                            &raw mut more_data,
                            &raw mut is_pure_ack,
                        );
                    }
                    if length > header_length
                        || pmtu_discovery_needed as ::core::ffi::c_uint
                            != picoquic_pmtu_discovery_required as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                        || send_buffer_max <= (*path_x).send_mtu
                    {
                        if ret == 0 as ::core::ffi::c_int {
                            bytes_next = picoquic_format_new_local_id_as_needed(
                                cnx,
                                bytes_next,
                                bytes_max,
                                current_time,
                                next_wake_time,
                                &raw mut more_data,
                                &raw mut is_pure_ack,
                            );
                        }
                        if ret == 0 as ::core::ffi::c_int
                            && (*cnx).is_ack_frequency_updated() as ::core::ffi::c_int != 0
                            && (*cnx).is_ack_frequency_negotiated() as ::core::ffi::c_int != 0
                        {
                            bytes_next = picoquic_format_ack_frequency_frame(
                                cnx,
                                bytes_next,
                                bytes_max,
                                &raw mut more_data,
                            );
                        }
                        if ret == 0 as ::core::ffi::c_int {
                            bytes_next = picoquic_prepare_stream_and_datagrams(
                                cnx,
                                path_x,
                                bytes_next,
                                bytes_max,
                                UINT64_MAX as uint64_t,
                                current_time,
                                &raw mut more_data,
                                &raw mut is_pure_ack,
                                &raw mut no_data_to_send_0,
                                &raw mut ret,
                            );
                        }
                        if (*cnx).client_mode() == 0
                            && (*cnx).send_receive_bdp_frame() as ::core::ffi::c_int != 0
                        {
                            bytes_next = picoquic_format_bdp_frame(
                                cnx,
                                bytes_next,
                                bytes_max,
                                path_x,
                                &raw mut more_data,
                                &raw mut is_pure_ack,
                            );
                        }
                        length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
                        if length <= header_length || is_pure_ack != 0 {
                            (*path_x).delivered_limited_index = (*path_x).delivered;
                            bytes_next = picoquic_format_blocked_frames(
                                cnx,
                                bytes.offset(length as isize) as *mut uint8_t,
                                bytes_max,
                                &raw mut more_data,
                                &raw mut is_pure_ack,
                            );
                            length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
                        }
                        if (*cnx).is_preemptive_repeat_enabled() as ::core::ffi::c_int != 0
                            || (*cnx).is_forced_probe_up_required() as ::core::ffi::c_int != 0
                                && (*path_x).is_cca_probing_up() as ::core::ffi::c_int != 0
                        {
                            if length <= header_length {
                                ret = picoquic_preemptive_retransmit_as_needed(
                                    cnx,
                                    path_x,
                                    pc,
                                    current_time,
                                    next_wake_time,
                                    bytes_next,
                                    bytes_max.offset_from(bytes_next) as ::core::ffi::c_long
                                        as size_t,
                                    &raw mut length,
                                    &raw mut more_data,
                                    &raw mut is_pure_ack,
                                );
                                if length > header_length {
                                    preemptive_repeat = 1 as ::core::ffi::c_int;
                                    (*packet).set_is_preemptive_repeat(
                                        1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                    );
                                    bytes_next = bytes.offset(length as isize);
                                } else if (*cnx).is_forced_probe_up_required() as ::core::ffi::c_int
                                    != 0
                                    && (*path_x).is_cca_probing_up() as ::core::ffi::c_int != 0
                                {
                                    let c2rust_fresh25 = bytes_next;
                                    bytes_next = bytes_next.offset(1);
                                    *c2rust_fresh25 =
                                        picoquic_frame_type_ping as ::core::ffi::c_int as uint8_t;
                                    memset(
                                        bytes_next as *mut ::core::ffi::c_void,
                                        picoquic_frame_type_padding as ::core::ffi::c_int,
                                        bytes_max.offset_from(bytes_next) as ::core::ffi::c_long
                                            as size_t,
                                    );
                                    bytes_next = bytes_max;
                                    length = bytes_next.offset_from(bytes) as ::core::ffi::c_long
                                        as size_t;
                                    is_pure_ack = 0 as ::core::ffi::c_int;
                                }
                            } else if more_data == 0 {
                                ret = picoquic_preemptive_retransmit_as_needed(
                                    cnx,
                                    path_x,
                                    pc,
                                    current_time,
                                    next_wake_time,
                                    bytes_next,
                                    bytes_max.offset_from(bytes_next) as ::core::ffi::c_long
                                        as size_t,
                                    &raw mut length,
                                    &raw mut more_data,
                                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                                );
                            }
                        }
                        if no_data_to_send_0 != 0 && preemptive_repeat == 0 {
                            (*path_x).last_sender_limited_time = current_time;
                        }
                    }
                    if ret == 0 as ::core::ffi::c_int
                        && (*path_x).is_pto_required() as ::core::ffi::c_int != 0
                    {
                        if (length <= header_length || is_pure_ack != 0) && bytes_next < bytes_max {
                            let c2rust_fresh26 = bytes_next;
                            bytes_next = bytes_next.offset(1);
                            *c2rust_fresh26 =
                                picoquic_frame_type_ping as ::core::ffi::c_int as uint8_t;
                            length = length.wrapping_add(1);
                            is_pure_ack = 0 as ::core::ffi::c_int;
                        }
                    }
                    if ret == 0 as ::core::ffi::c_int && length <= header_length {
                        if send_buffer_max > (*path_x).send_mtu
                            && (*path_x).cwin > (*path_x).bytes_in_transit
                            && (*(*cnx).quic).cwin_max > (*path_x).bytes_in_transit
                            && pmtu_discovery_needed as ::core::ffi::c_uint
                                != picoquic_pmtu_discovery_not_needed as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                        {
                            length = picoquic_prepare_mtu_probe(
                                cnx,
                                path_x,
                                header_length,
                                checksum_overhead,
                                bytes,
                                send_buffer_max,
                            );
                            (*packet).length = length;
                            (*packet).send_path = path_x as *mut st_picoquic_path_t;
                            (*packet)
                                .set_is_mtu_probe(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                            (*path_x).set_mtu_probe_sent(
                                1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                            );
                            is_pure_ack = 0 as ::core::ffi::c_int;
                        }
                    }
                    if ret == 0 as ::core::ffi::c_int
                        && (*cnx).is_poll_requested() as ::core::ffi::c_int != 0
                        && length <= header_length
                    {
                        if bytes_next < bytes_max {
                            let c2rust_fresh27 = bytes_next;
                            bytes_next = bytes_next.offset(1);
                            *c2rust_fresh27 =
                                picoquic_frame_type_poll as ::core::ffi::c_int as uint8_t;
                            length = length.wrapping_add(1);
                            (*cnx).set_is_poll_requested(
                                0 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                            );
                        }
                    }
                }
            } else if (*cnx).priority_limit_for_bypass > 0 as uint64_t
                && (*cnx).nb_paths == 1 as ::core::ffi::c_int
                && picoquic_is_authorized_by_pacing(
                    &raw mut (*cnx).priority_bypass_pacing,
                    current_time,
                    next_wake_time,
                    (*(*cnx).quic).packet_train_mode(),
                    (*cnx).quic,
                ) != 0
            {
                let mut no_data_to_send_1: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                bytes_next = picoquic_prepare_stream_and_datagrams(
                    cnx,
                    path_x,
                    bytes_next,
                    bytes_max,
                    (*cnx).priority_limit_for_bypass,
                    current_time,
                    &raw mut more_data,
                    &raw mut is_pure_ack,
                    &raw mut no_data_to_send_1,
                    &raw mut ret,
                );
                if !bytes_next.is_null() {
                    length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
                }
            }
        }
    }
    if length <= header_length {
        length = 0 as size_t;
    }
    if (*cnx).cnx_state as ::core::ffi::c_uint
        != picoquic_state_disconnected as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if length > 0 as size_t {
            (*path_x).set_is_pto_required(
                (*path_x).is_pto_required() & is_pure_ack as ::core::ffi::c_uint,
            );
            (*pkt_ctx).set_ack_of_ack_requested(
                (*pkt_ctx).ack_of_ack_requested()
                    | (is_pure_ack == 0) as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
            if (*pkt_ctx).ack_of_ack_requested() == 0 && ack_sent != 0 {
                let ack_repeat_interval: uint64_t = 24 as uint64_t;
                bytes_next = bytes.offset(length as isize);
                if bytes_next < bytes_max
                    && (*pkt_ctx)
                        .highest_acknowledged
                        .wrapping_add(ack_repeat_interval)
                        < (*pkt_ctx).send_sequence
                    && path_x == *(*cnx).path.offset(0 as ::core::ffi::c_int as isize)
                    && (*pkt_ctx)
                        .highest_acknowledged_time
                        .wrapping_add((*path_x).smoothed_rtt)
                        < current_time
                {
                    let c2rust_fresh28 = bytes_next;
                    bytes_next = bytes_next.offset(1);
                    *c2rust_fresh28 = picoquic_frame_type_ping as ::core::ffi::c_int as uint8_t;
                    (*pkt_ctx)
                        .set_ack_of_ack_requested(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    is_pure_ack = 0 as ::core::ffi::c_int;
                    length = bytes_next.offset_from(bytes) as ::core::ffi::c_long as size_t;
                }
            }
            if is_pure_ack != 0
                && (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
                && (*path_x).is_ack_lost() as ::core::ffi::c_int != 0
                && (*path_x).is_ack_expected() == 0
            {
                bytes_next = bytes.offset(length as isize);
                if bytes_next < bytes_max {
                    is_pure_ack = 0 as ::core::ffi::c_int;
                    *bytes_next = picoquic_frame_type_ping as ::core::ffi::c_int as uint8_t;
                    length = length.wrapping_add(1);
                }
            }
            if is_pure_ack == 0 {
                (*path_x).set_is_ack_expected(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
        if is_pure_ack == 0 as ::core::ffi::c_int {
            (*cnx).latest_progress_time = current_time;
        } else if (*cnx).keep_alive_interval != 0 as uint64_t {
            if (*cnx)
                .latest_progress_time
                .wrapping_add((*cnx).keep_alive_interval)
                <= current_time
                && length == 0 as size_t
            {
                length = picoquic_predict_packet_header_length(cnx, packet_type, pkt_ctx);
                (*packet).ptype = packet_type;
                (*packet).pc = pc;
                (*packet).offset = length;
                header_length = length;
                (*packet).sequence_number = (*pkt_ctx).send_sequence;
                (*packet).send_path = path_x as *mut st_picoquic_path_t;
                (*packet).send_time = current_time;
                let c2rust_fresh29 = length;
                length = length.wrapping_add(1);
                *bytes.offset(c2rust_fresh29 as isize) =
                    picoquic_frame_type_ping as ::core::ffi::c_int as uint8_t;
                let c2rust_fresh30 = length;
                length = length.wrapping_add(1);
                *bytes.offset(c2rust_fresh30 as isize) = 0 as uint8_t;
                (*cnx).latest_progress_time = current_time;
            } else if (*cnx)
                .latest_progress_time
                .wrapping_add((*cnx).keep_alive_interval)
                < *next_wake_time
            {
                *next_wake_time = (*cnx)
                    .latest_progress_time
                    .wrapping_add((*cnx).keep_alive_interval);
                (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
                (*(*cnx).quic).wake_line = 3799 as ::core::ffi::c_int;
            }
        }
        if more_data != 0 {
            *next_wake_time = current_time;
            (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
            (*(*cnx).quic).wake_line = 3805 as ::core::ffi::c_int;
            ret = 0 as ::core::ffi::c_int;
        }
    }
    if ret == 0 as ::core::ffi::c_int && length > header_length {
        if *is_initial_sent != 0
            || is_challenge_padding_needed != 0 && length < PICOQUIC_ENFORCED_INITIAL_MTU as size_t
        {
            length = picoquic_pad_to_target_length(
                bytes,
                length,
                send_buffer_min_max.wrapping_sub(checksum_overhead) as uint32_t as size_t,
            );
        } else {
            length = picoquic_pad_to_policy(
                cnx,
                bytes,
                length,
                send_buffer_min_max.wrapping_sub(checksum_overhead) as uint32_t,
            );
        }
    }
    picoquic_finalize_and_protect_packet(
        cnx,
        packet,
        ret,
        length,
        header_length,
        checksum_overhead,
        send_length,
        send_buffer,
        send_buffer_min_max,
        path_x,
        current_time,
    );
    if *send_length > 0 as size_t {
        *next_wake_time = current_time;
        (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
        (*(*cnx).quic).wake_line = 3828 as ::core::ffi::c_int;
        if ret == 0 as ::core::ffi::c_int
            && picoquic_cnx_is_still_logging(cnx as *mut picoquic_cnx_t) != 0
        {
            picoquic_log_cc_dump(cnx, current_time);
        }
    }
    return ret;
}
unsafe extern "C" fn picoquic_check_idle_timer(
    mut cnx: *mut picoquic_cnx_t,
    mut next_wake_time: *mut uint64_t,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut idle_timer: uint64_t = 0 as uint64_t;
    if (*cnx).cnx_state as ::core::ffi::c_uint
        >= picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut rto: uint64_t = picoquic_current_retransmit_timer(
            cnx,
            *(*cnx).path.offset(0 as ::core::ffi::c_int as isize),
        );
        idle_timer = (*cnx).idle_timeout;
        if idle_timer < (3 as uint64_t).wrapping_mul(rto) {
            idle_timer = (3 as uint64_t).wrapping_mul(rto);
        }
        idle_timer = idle_timer.wrapping_add((*cnx).latest_receive_time);
        if idle_timer < (*cnx).idle_timeout {
            idle_timer = UINT64_MAX as uint64_t;
        }
    } else if (*(*cnx).quic).default_handshake_timeout > 0 as uint64_t {
        idle_timer = (*cnx)
            .start_time
            .wrapping_add((*(*cnx).quic).default_handshake_timeout);
    } else if (*cnx).local_parameters.max_idle_timeout > 0 as uint64_t {
        idle_timer = ((*cnx).start_time as ::core::ffi::c_ulonglong).wrapping_add(
            ((*cnx).local_parameters.max_idle_timeout as ::core::ffi::c_ulonglong)
                .wrapping_mul(1000 as ::core::ffi::c_ulonglong),
        ) as uint64_t;
    } else {
        idle_timer = ((*cnx).start_time as ::core::ffi::c_ulonglong)
            .wrapping_add(PICOQUIC_MICROSEC_HANDSHAKE_MAX) as uint64_t;
    }
    if current_time >= idle_timer {
        if (*cnx).cnx_state as ::core::ffi::c_uint
            != picoquic_state_draining as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*cnx).local_error = PICOQUIC_ERROR_IDLE_TIMEOUT as uint64_t;
        }
        ret = PICOQUIC_ERROR_DISCONNECTED;
        picoquic_connection_disconnect(cnx);
    } else if idle_timer < *next_wake_time {
        *next_wake_time = idle_timer;
        (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
        (*(*cnx).quic).wake_line = 3873 as ::core::ffi::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_segment(
    mut cnx: *mut picoquic_cnx_t,
    mut path_x: *mut picoquic_path_t,
    mut packet: *mut picoquic_packet_t,
    mut current_time: uint64_t,
    mut send_buffer: *mut uint8_t,
    mut send_buffer_max: size_t,
    mut send_length: *mut size_t,
    mut next_wake_time: *mut uint64_t,
    mut is_initial_sent: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*cnx).set_cwin_blocked(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*cnx).set_flow_blocked(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*cnx).set_stream_blocked(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    let mut c2rust_current_block_16: u64;
    match (*cnx).cnx_state as ::core::ffi::c_uint {
        0 | 1 | 4 | 2 | 7 | 10 => {
            ret = picoquic_prepare_packet_client_init(
                cnx,
                path_x,
                packet,
                current_time,
                send_buffer,
                send_buffer_max,
                send_length,
                next_wake_time,
                is_initial_sent,
            );
            c2rust_current_block_16 = 14401909646449704462;
        }
        12 | 5 | 6 => {
            ret = picoquic_prepare_packet_server_init(
                cnx,
                path_x,
                packet,
                current_time,
                send_buffer,
                send_buffer_max,
                send_length,
                next_wake_time,
                is_initial_sent,
            );
            c2rust_current_block_16 = 14401909646449704462;
        }
        11 => {
            if (*cnx).cnx_state as ::core::ffi::c_uint
                == picoquic_state_server_false_start as ::core::ffi::c_int as ::core::ffi::c_uint
                && !(*cnx).crypto_context[3 as ::core::ffi::c_int as usize]
                    .aead_decrypt
                    .is_null()
            {
                picoquic_ready_state_transition(cnx, current_time);
                return picoquic_prepare_packet_ready(
                    cnx,
                    path_x,
                    packet,
                    current_time,
                    send_buffer,
                    send_buffer_max,
                    send_length,
                    next_wake_time,
                    is_initial_sent,
                );
            }
            c2rust_current_block_16 = 7146806494120673115;
        }
        13 => {
            c2rust_current_block_16 = 7146806494120673115;
        }
        14 => {
            ret = picoquic_prepare_packet_ready(
                cnx,
                path_x,
                packet,
                current_time,
                send_buffer,
                send_buffer_max,
                send_length,
                next_wake_time,
                is_initial_sent,
            );
            c2rust_current_block_16 = 14401909646449704462;
        }
        8 | 9 | 15 | 16 | 17 | 18 => {
            ret = picoquic_prepare_packet_closing(
                cnx,
                path_x,
                packet,
                current_time,
                send_buffer,
                send_buffer_max,
                send_length,
                next_wake_time,
            );
            c2rust_current_block_16 = 14401909646449704462;
        }
        19 => {
            ret = PICOQUIC_ERROR_DISCONNECTED;
            c2rust_current_block_16 = 14401909646449704462;
        }
        3 => {
            ret = PICOQUIC_ERROR_UNEXPECTED_STATE;
            c2rust_current_block_16 = 14401909646449704462;
        }
        _ => {
            ret = PICOQUIC_ERROR_UNEXPECTED_STATE;
            c2rust_current_block_16 = 14401909646449704462;
        }
    }
    match c2rust_current_block_16 {
        7146806494120673115 => {
            ret = picoquic_prepare_packet_almost_ready(
                cnx,
                path_x,
                packet,
                current_time,
                send_buffer,
                send_buffer_max,
                send_length,
                next_wake_time,
                is_initial_sent,
            );
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn picoquic_set_path_addresses(
    mut cnx: *mut picoquic_cnx_t,
    mut path_id: ::core::ffi::c_int,
    mut is_nat: ::core::ffi::c_int,
    mut p_addr_to: *mut sockaddr_storage,
    mut p_addr_from: *mut sockaddr_storage,
    mut if_index: *mut ::core::ffi::c_int,
) {
    if is_nat != 0 {
        if !p_addr_to.is_null() {
            picoquic_store_addr(
                p_addr_to,
                &raw mut (**(*cnx).path.offset(path_id as isize)).nat_peer_addr as *mut sockaddr,
            );
        }
        if !p_addr_from.is_null() {
            picoquic_store_addr(
                p_addr_from,
                &raw mut (**(*cnx).path.offset(path_id as isize)).nat_local_addr as *mut sockaddr,
            );
        }
        if !if_index.is_null() {
            *if_index =
                (**(*cnx).path.offset(path_id as isize)).if_index_nat_dest as ::core::ffi::c_int;
        }
    } else {
        if !p_addr_to.is_null() {
            picoquic_store_addr(
                p_addr_to,
                &raw mut (**(*cnx).path.offset(path_id as isize)).peer_addr as *mut sockaddr,
            );
        }
        if !p_addr_from.is_null() {
            picoquic_store_addr(
                p_addr_from,
                &raw mut (**(*cnx).path.offset(path_id as isize)).local_addr as *mut sockaddr,
            );
        }
        if !if_index.is_null() {
            *if_index =
                (**(*cnx).path.offset(path_id as isize)).if_index_dest as ::core::ffi::c_int;
        }
    };
}
unsafe extern "C" fn picoquic_select_next_path_mp(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
    mut next_wake_time: *mut uint64_t,
    mut p_addr_to: *mut sockaddr_storage,
    mut p_addr_from: *mut sockaddr_storage,
    mut if_index: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut path_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut highest_priority: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut data_path_cwin: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut data_path_pacing: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut challenge_path: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut pacing_time_next: uint64_t = UINT64_MAX as uint64_t;
    let mut challenge_time_next: uint64_t = UINT64_MAX as uint64_t;
    let mut highest_retransmit: uint64_t = UINT64_MAX as uint64_t;
    let mut last_sent_pacing: uint64_t = UINT64_MAX as uint64_t;
    let mut last_sent_cwin: uint64_t = UINT64_MAX as uint64_t;
    let mut i: ::core::ffi::c_int = 0;
    let mut i_min_rtt: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut is_min_rtt_pacing_ok: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_ack_needed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut next_stream: *mut picoquic_stream_head_t = picoquic_find_ready_stream(cnx);
    let mut affinity_path_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut is_nat: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    (*cnx).last_path_polled += 1;
    if (*cnx).last_path_polled > (*cnx).nb_paths {
        (*cnx).last_path_polled = 0 as ::core::ffi::c_int;
    }
    i = 0 as ::core::ffi::c_int;
    while i < (*cnx).nb_paths {
        let mut path_priority: ::core::ffi::c_int =
            if (**(*cnx).path.offset(i as isize)).path_is_standby() as ::core::ffi::c_int != 0 {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            };
        let ref mut c2rust_fresh0 = **(*cnx).path.offset(i as isize);
        (*c2rust_fresh0).set_is_probing_nat(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if (**(*cnx).path.offset(i as isize)).nb_retransmit > 0 as uint64_t {
            path_priority = 0 as ::core::ffi::c_int;
        }
        let ref mut c2rust_fresh1 = **(*cnx).path.offset(i as isize);
        (*c2rust_fresh1).set_is_nominal_ack_path(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        if !((**(*cnx).path.offset(i as isize)).path_is_demoted() != 0) {
            if (**(*cnx).path.offset(i as isize)).challenge_failed() != 0 {
                picoquic_demote_path(
                    cnx,
                    i,
                    current_time,
                    0 as uint64_t,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            } else if (**(*cnx).path.offset(i as isize)).response_required() != 0 {
                challenge_path = i;
                let ref mut c2rust_fresh2 = (**(*cnx).path.offset(i as isize)).responder;
                *c2rust_fresh2 += 1;
                break;
            } else {
                if (**(*cnx).path.offset(i as isize)).challenge_required() as ::core::ffi::c_int
                    != 0
                    && (**(*cnx).path.offset(i as isize)).challenge_verified() == 0
                {
                    let mut next_challenge_time: uint64_t = picoquic_next_challenge_time(
                        cnx,
                        *(*cnx).path.offset(i as isize),
                        current_time,
                        &raw mut is_nat,
                    );
                    if current_time >= next_challenge_time {
                        let ref mut c2rust_fresh3 = (**(*cnx).path.offset(i as isize)).challenger;
                        *c2rust_fresh3 += 1;
                        let ref mut c2rust_fresh4 = **(*cnx).path.offset(i as isize);
                        (*c2rust_fresh4).set_is_probing_nat(
                            (if is_nat != 0 {
                                1 as ::core::ffi::c_int
                            } else {
                                0 as ::core::ffi::c_int
                            }) as ::core::ffi::c_uint
                                as ::core::ffi::c_uint,
                        );
                        challenge_path = i;
                        break;
                    } else if next_challenge_time < challenge_time_next {
                        challenge_time_next = next_challenge_time;
                    }
                } else if (**(*cnx).path.offset(i as isize)).challenge_verified()
                    as ::core::ffi::c_int
                    != 0
                    && (**(*cnx).path.offset(i as isize)).nb_retransmit > 0 as uint64_t
                    && (*cnx).cnx_state as ::core::ffi::c_uint
                        == picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (**(*cnx).path.offset(i as isize)).bytes_in_transit == 0 as uint64_t
                {
                    let ref mut c2rust_fresh5 = **(*cnx).path.offset(i as isize);
                    (*c2rust_fresh5).set_is_multipath_probe_needed(
                        1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                    );
                    challenge_path = i;
                    break;
                }
                if (**(*cnx).path.offset(i as isize)).challenge_verified() != 0 {
                    let mut is_polled: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut is_new_priority: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    if !(*cnx).congestion_alg.is_null()
                        && (**(*cnx).path.offset(i as isize))
                            .congestion_alg_state
                            .is_null()
                    {
                        (*(*cnx).congestion_alg)
                            .alg_init
                            .expect("non-null function pointer")(
                            cnx as *mut picoquic_cnx_t,
                            *(*cnx).path.offset(i as isize) as *mut picoquic_path_t,
                            current_time,
                        );
                    }
                    if path_priority > highest_priority {
                        is_polled = 1 as ::core::ffi::c_int;
                        is_new_priority = 1 as ::core::ffi::c_int;
                    } else if path_priority == highest_priority {
                        if (**(*cnx).path.offset(i as isize)).nb_retransmit < highest_retransmit {
                            is_polled = 1 as ::core::ffi::c_int;
                            is_new_priority = 1 as ::core::ffi::c_int;
                        } else if (**(*cnx).path.offset(i as isize)).nb_retransmit
                            == highest_retransmit
                        {
                            is_polled = 1 as ::core::ffi::c_int;
                        }
                    }
                    if is_new_priority != 0 {
                        highest_priority = path_priority;
                        highest_retransmit = (**(*cnx).path.offset(i as isize)).nb_retransmit;
                        data_path_cwin = -(1 as ::core::ffi::c_int);
                        data_path_pacing = -(1 as ::core::ffi::c_int);
                        pacing_time_next = UINT64_MAX as uint64_t;
                        last_sent_pacing = UINT64_MAX as uint64_t;
                        last_sent_cwin = UINT64_MAX as uint64_t;
                        i_min_rtt = -(1 as ::core::ffi::c_int);
                        is_min_rtt_pacing_ok = 0 as ::core::ffi::c_int;
                    }
                    if is_polled != 0 {
                        if i_min_rtt < 0 as ::core::ffi::c_int
                            || (**(*cnx).path.offset(i as isize)).nb_retransmit
                                < (**(*cnx).path.offset(i_min_rtt as isize)).nb_retransmit
                            || (**(*cnx).path.offset(i as isize)).nb_retransmit
                                == (**(*cnx).path.offset(i_min_rtt as isize)).nb_retransmit
                                && (**(*cnx).path.offset(i as isize)).rtt_min
                                    < (**(*cnx).path.offset(i_min_rtt as isize)).rtt_min
                        {
                            i_min_rtt = i;
                            is_min_rtt_pacing_ok = 0 as ::core::ffi::c_int;
                        }
                        let ref mut c2rust_fresh6 = (**(*cnx).path.offset(i as isize)).polled;
                        *c2rust_fresh6 += 1;
                        if picoquic_is_sending_authorized_by_pacing(
                            cnx,
                            *(*cnx).path.offset(i as isize),
                            current_time,
                            &raw mut pacing_time_next,
                        ) != 0
                        {
                            if (**(*cnx).path.offset(i as isize)).last_sent_time < last_sent_pacing
                            {
                                last_sent_pacing =
                                    (**(*cnx).path.offset(i as isize)).last_sent_time;
                                data_path_pacing = i;
                                if i == i_min_rtt {
                                    is_min_rtt_pacing_ok = 1 as ::core::ffi::c_int;
                                }
                            }
                            if (**(*cnx).path.offset(i as isize)).bytes_in_transit
                                < (**(*cnx).path.offset(i as isize)).cwin
                                && (**(*cnx).path.offset(i as isize)).bytes_in_transit
                                    < (*(*cnx).quic).cwin_max
                            {
                                if (**(*cnx).path.offset(i as isize)).last_sent_time
                                    < last_sent_cwin
                                {
                                    last_sent_cwin =
                                        (**(*cnx).path.offset(i as isize)).last_sent_time;
                                    data_path_cwin = i;
                                }
                                if affinity_path_id < 0 as ::core::ffi::c_int {
                                    if !next_stream.is_null()
                                        && *(*cnx).path.offset(i as isize)
                                            == (*next_stream).affinity_path
                                    {
                                        affinity_path_id = i;
                                    } else if (**(*cnx).path.offset(i as isize)).is_datagram_ready()
                                        as ::core::ffi::c_int
                                        != 0
                                        || (*cnx).is_datagram_ready() as ::core::ffi::c_int != 0
                                    {
                                        affinity_path_id = i;
                                    }
                                }
                            } else {
                                let ref mut c2rust_fresh7 =
                                    (**(*cnx).path.offset(i as isize)).congested;
                                *c2rust_fresh7 += 1;
                            }
                        } else {
                            let ref mut c2rust_fresh8 = (**(*cnx).path.offset(i as isize)).paced;
                            *c2rust_fresh8 += 1;
                        }
                    }
                }
            }
        }
        i += 1;
    }
    i += 1 as ::core::ffi::c_int;
    while i < (*cnx).nb_paths {
        let ref mut c2rust_fresh9 = **(*cnx).path.offset(i as isize);
        (*c2rust_fresh9).set_is_nominal_ack_path(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        i += 1;
    }
    if i_min_rtt >= 0 as ::core::ffi::c_int {
        is_ack_needed = picoquic_is_ack_needed(
            cnx,
            current_time,
            next_wake_time,
            picoquic_packet_context_application,
            0 as ::core::ffi::c_int,
        );
        let ref mut c2rust_fresh10 = **(*cnx).path.offset(i_min_rtt as isize);
        (*c2rust_fresh10).set_is_nominal_ack_path(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if challenge_path >= 0 as ::core::ffi::c_int {
        path_id = challenge_path;
    } else if is_ack_needed != 0 && is_min_rtt_pacing_ok != 0 {
        path_id = i_min_rtt;
    } else if data_path_cwin >= 0 as ::core::ffi::c_int {
        if affinity_path_id >= 0 as ::core::ffi::c_int {
            path_id = affinity_path_id;
        } else {
            path_id = data_path_cwin;
        }
    } else if data_path_pacing >= 0 as ::core::ffi::c_int {
        path_id = data_path_pacing;
    } else {
        let mut path_wake_time: uint64_t = pacing_time_next;
        if challenge_time_next < path_wake_time {
            path_wake_time = challenge_time_next;
        }
        if path_wake_time < *next_wake_time {
            *next_wake_time = path_wake_time;
            (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
            (*(*cnx).quic).wake_line = 4228 as ::core::ffi::c_int;
        }
        path_id = 0 as ::core::ffi::c_int;
    }
    if (**(*cnx).path.offset(path_id as isize)).path_is_standby() as ::core::ffi::c_int != 0
        && challenge_path != path_id
    {
        picoquic_set_path_status(
            cnx as *mut picoquic_cnx_t,
            (**(*cnx).path.offset(path_id as isize)).unique_path_id,
            picoquic_path_status_available,
        );
    }
    let ref mut c2rust_fresh11 = (**(*cnx).path.offset(path_id as isize)).selected;
    *c2rust_fresh11 += 1;
    picoquic_set_path_addresses(
        cnx,
        path_id,
        is_nat as ::core::ffi::c_int,
        p_addr_to,
        p_addr_from,
        if_index,
    );
    return path_id;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_select_next_path(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
    mut next_wake_time: *mut uint64_t,
    mut p_addr_to: *mut sockaddr_storage,
    mut p_addr_from: *mut sockaddr_storage,
    mut if_index: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut path_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
        && (*cnx).cnx_state as ::core::ffi::c_uint
            >= picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return picoquic_select_next_path_mp(
            cnx,
            current_time,
            next_wake_time,
            p_addr_to,
            p_addr_from,
            if_index,
        );
    }
    let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    while i < (*cnx).nb_paths {
        if !((**(*cnx).path.offset(i as isize)).path_is_demoted() != 0) {
            if (**(*cnx).path.offset(i as isize)).challenge_failed() != 0 {
                picoquic_demote_path(
                    cnx,
                    i,
                    current_time,
                    0 as uint64_t,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                );
            } else if (**(*cnx).path.offset(i as isize)).challenge_verified() as ::core::ffi::c_int
                != 0
                && (*cnx).cnx_state as ::core::ffi::c_uint
                    == picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if (*cnx).client_mode() as ::core::ffi::c_int != 0
                    || (**(*cnx).path.offset(i as isize)).last_non_path_probing_pn
                        >= picoquic_sack_list_last(
                            &raw mut (*(&raw mut (*cnx).ack_ctx as *mut picoquic_ack_context_t)
                                .offset(
                                    picoquic_packet_context_application as ::core::ffi::c_int
                                        as isize,
                                ))
                            .sack_list,
                        )
                    || (**(*cnx).path.offset(i as isize)).is_nat_challenge() as ::core::ffi::c_int
                        != 0
                {
                    picoquic_promote_path_to_default(cnx, i, current_time);
                    path_id = 0 as ::core::ffi::c_int;
                }
                break;
            } else if path_id < 0 as ::core::ffi::c_int {
                if (**(*cnx).path.offset(i as isize)).response_required() != 0 {
                    path_id = i;
                } else if (**(*cnx).path.offset(i as isize)).challenge_required() != 0 {
                    let mut next_challenge_time: uint64_t = picoquic_next_challenge_time(
                        cnx,
                        *(*cnx).path.offset(i as isize),
                        current_time,
                        ::core::ptr::null_mut::<::core::ffi::c_uint>(),
                    );
                    if (**(*cnx).path.offset(i as isize)).challenge_repeat_count
                        as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                        || current_time >= next_challenge_time
                    {
                        path_id = i;
                    } else if next_challenge_time < *next_wake_time {
                        *next_wake_time = next_challenge_time;
                        (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
                        (*(*cnx).quic).wake_line = 4295 as ::core::ffi::c_int;
                    }
                }
            }
        }
        i += 1;
    }
    if path_id < 0 as ::core::ffi::c_int {
        path_id = 0 as ::core::ffi::c_int;
    }
    picoquic_set_path_addresses(
        cnx,
        path_id,
        0 as ::core::ffi::c_int,
        p_addr_to,
        p_addr_from,
        if_index,
    );
    return path_id;
}
unsafe extern "C" fn picoquic_check_cc_feedback_timer(
    mut cnx: *mut picoquic_cnx_t,
    mut next_wake_time: *mut uint64_t,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*cnx).is_lost_feedback_notification_required() as ::core::ffi::c_int != 0
        && !(*cnx).congestion_alg.is_null()
    {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*cnx).nb_paths {
            let mut path_x: *mut picoquic_path_t = *(*cnx).path.offset(i as isize);
            if (*path_x).is_lost_feedback_notified() == 0 {
                let mut pkt_ctx: *mut picoquic_packet_context_t =
                    if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0 {
                        &raw mut (*path_x).pkt_ctx
                    } else {
                        (&raw mut (*cnx).pkt_ctx as *mut picoquic_packet_context_t).offset(
                            picoquic_packet_context_application as ::core::ffi::c_int as isize,
                        ) as *mut picoquic_packet_context_t
                    };
                if !(*pkt_ctx).pending_first.is_null() {
                    let mut delta_sent: uint64_t = if (*(*pkt_ctx).pending_first).send_time
                        <= (*path_x).last_time_acked_data_frame_sent
                    {
                        0 as uint64_t
                    } else {
                        (*(*pkt_ctx).pending_first)
                            .send_time
                            .wrapping_sub((*path_x).last_time_acked_data_frame_sent)
                    };
                    let mut lost_feedback_time: uint64_t = (*pkt_ctx)
                        .highest_acknowledged_time
                        .wrapping_add(delta_sent)
                        .wrapping_add(
                            (2 as uint64_t).wrapping_mul((*cnx).ack_frequency_delay_local),
                        );
                    if lost_feedback_time <= current_time {
                        (*path_x).set_is_lost_feedback_notified(
                            1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                        );
                        (*(*cnx).congestion_alg)
                            .alg_notify
                            .expect("non-null function pointer")(
                            cnx as *mut picoquic_cnx_t,
                            path_x as *mut picoquic_path_t,
                            picoquic_congestion_notification_lost_feedback,
                            ::core::ptr::null_mut::<picoquic_per_ack_state_t>(),
                            current_time,
                        );
                    } else if lost_feedback_time < *next_wake_time {
                        *next_wake_time = lost_feedback_time;
                        (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
                        (*(*cnx).quic).wake_line = 4334 as ::core::ffi::c_int;
                    }
                }
            }
            i += 1;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_handle_app_wake_time(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (*cnx).app_wake_time != 0 as uint64_t && (*cnx).app_wake_time <= current_time {
        (*cnx).app_wake_time = 0 as uint64_t;
        if (*cnx).callback_fn.is_some() {
            ret = (*cnx).callback_fn.expect("non-null function pointer")(
                cnx as *mut picoquic_cnx_t,
                current_time,
                ::core::ptr::null_mut::<uint8_t>(),
                0 as size_t,
                picoquic_callback_app_wakeup,
                (*cnx).callback_ctx,
                NULL,
            );
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_program_app_wake_time(
    mut cnx: *mut picoquic_cnx_t,
    mut next_wake_time: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*cnx).app_wake_time != 0 as uint64_t && (*cnx).app_wake_time < *next_wake_time {
        *next_wake_time = (*cnx).app_wake_time;
        (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
        (*(*cnx).quic).wake_line = 4362 as ::core::ffi::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_packet_ex(
    mut cnx: *mut picoquic_cnx_t,
    mut path_id_request: ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut send_buffer: *mut uint8_t,
    mut send_buffer_max: size_t,
    mut send_length: *mut size_t,
    mut p_addr_to: *mut sockaddr_storage,
    mut p_addr_from: *mut sockaddr_storage,
    mut if_index: *mut ::core::ffi::c_int,
    mut send_msg_size: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut packet: *mut picoquic_packet_t = ::core::ptr::null_mut::<picoquic_packet_t>();
    let mut initial_next_time: uint64_t = 0;
    let mut next_wake_time: uint64_t = ((*cnx).latest_receive_time as ::core::ffi::c_ulonglong)
        .wrapping_add((2 as ::core::ffi::c_ulonglong).wrapping_mul(PICOQUIC_MICROSEC_SILENCE_MAX))
        as uint64_t;
    if (*cnx).local_parameters.max_idle_timeout as ::core::ffi::c_ulonglong
        > PICOQUIC_MICROSEC_SILENCE_MAX.wrapping_div(500 as ::core::ffi::c_ulonglong)
    {
        next_wake_time = ((*cnx).latest_receive_time as ::core::ffi::c_ulonglong).wrapping_add(
            ((*cnx).local_parameters.max_idle_timeout as ::core::ffi::c_ulonglong)
                .wrapping_mul(1000 as ::core::ffi::c_ulonglong),
        ) as uint64_t;
    }
    (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
    (*(*cnx).quic).wake_line = 4382 as ::core::ffi::c_int;
    if (*cnx).recycle_sooner_needed() != 0 {
        picoquic_process_sooner_packets(cnx, current_time);
    }
    *send_length = 0 as size_t;
    ret = picoquic_handle_app_wake_time(cnx, current_time);
    if ret == 0 as ::core::ffi::c_int {
        ret = picoquic_check_idle_timer(cnx, &raw mut next_wake_time, current_time);
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = picoquic_check_cc_feedback_timer(cnx, &raw mut next_wake_time, current_time);
    }
    if send_buffer_max < PICOQUIC_ENFORCED_INITIAL_MTU as size_t {
        ret = -(1 as ::core::ffi::c_int);
    }
    if ret == 0 as ::core::ffi::c_int {
        let mut path_id: ::core::ffi::c_int = 0;
        if (*cnx).path_demotion_needed() != 0 {
            picoquic_delete_abandoned_paths(cnx, current_time, &raw mut next_wake_time);
        }
        path_id = picoquic_select_next_path(
            cnx,
            current_time,
            &raw mut next_wake_time,
            p_addr_to,
            p_addr_from,
            if_index,
        );
        if path_id_request != -(1 as ::core::ffi::c_int) {
            path_id = path_id_request;
        }
        if !send_msg_size.is_null() {
            *send_msg_size = (**(*cnx).path.offset(path_id as isize)).send_mtu;
        }
        initial_next_time = next_wake_time;
        if send_buffer_max > (**(*cnx).path.offset(path_id as isize)).send_mtu {
            (*cnx).set_is_sending_large_buffer(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        let mut segment_count: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while ret == 0 as ::core::ffi::c_int && segment_count < 128 as ::core::ffi::c_int {
            let mut is_initial_sent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut packet_size: size_t = 0 as size_t;
            let mut packet_max: size_t = send_buffer_max.wrapping_sub(*send_length);
            let mut packet_buffer: *mut uint8_t = send_buffer.offset(*send_length as isize);
            next_wake_time = initial_next_time;
            if !send_msg_size.is_null()
                && *send_msg_size > 0 as size_t
                && *send_length > 0 as size_t
                && packet_max > *send_msg_size
            {
                packet_max = *send_msg_size;
            }
            while ret == 0 as ::core::ffi::c_int && segment_count < 128 as ::core::ffi::c_int {
                let mut available: size_t = packet_max;
                let mut segment_length: size_t = 0 as size_t;
                if packet_size > 0 as size_t {
                    packet_max = (**(*cnx).path.offset(path_id as isize)).send_mtu;
                    if packet_max < packet_size.wrapping_add(PICOQUIC_MIN_SEGMENT_SIZE as size_t) {
                        break;
                    }
                    available = packet_max.wrapping_sub(packet_size);
                }
                packet = picoquic_create_packet((*cnx).quic);
                if packet.is_null() {
                    ret = PICOQUIC_ERROR_MEMORY;
                    break;
                } else {
                    ret = picoquic_prepare_segment(
                        cnx,
                        *(*cnx).path.offset(path_id as isize),
                        packet,
                        current_time,
                        packet_buffer.offset(packet_size as isize),
                        available,
                        &raw mut segment_length,
                        &raw mut next_wake_time,
                        &raw mut is_initial_sent,
                    );
                    if ret == 0 as ::core::ffi::c_int {
                        packet_size = packet_size.wrapping_add(segment_length);
                        segment_count += 1;
                        if (*packet).length == 0 as size_t {
                            picoquic_recycle_packet((*cnx).quic, packet);
                            break;
                        } else {
                            if (*packet).ptype as ::core::ffi::c_uint
                                == picoquic_packet_1rtt_protected as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            {
                                break;
                            }
                            if segment_length == 0 as size_t {
                                break;
                            }
                            if segment_count == 128 as ::core::ffi::c_int {
                                break;
                            }
                            if (*(*cnx).quic).dont_coalesce_init() != 0 {
                                break;
                            }
                        }
                    } else {
                        picoquic_recycle_packet((*cnx).quic, packet);
                        packet = ::core::ptr::null_mut::<picoquic_packet_t>();
                        if packet_size != 0 as size_t {
                            ret = 0 as ::core::ffi::c_int;
                        }
                        break;
                    }
                }
            }
            if packet_size > packet_max {
                picoquic_log_app_message(
                    cnx as *mut picoquic_cnx_t,
                    b"BUFFER OVERFLOW? Packet size %zu larger than %zu\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    packet_size,
                    packet_max,
                );
            }
            if packet_size > 0 as size_t {
                if packet_size > (*cnx).max_mtu_sent {
                    (*cnx).max_mtu_sent = packet_size;
                }
                (*cnx).nb_packets_sent = (*cnx).nb_packets_sent.wrapping_add(1);
                if !p_addr_to.is_null() && !p_addr_from.is_null() {
                    picoquic_log_pdu(
                        cnx,
                        0 as ::core::ffi::c_int,
                        current_time,
                        p_addr_to as *mut sockaddr,
                        p_addr_from as *mut sockaddr,
                        packet_size,
                    );
                }
            }
            if packet_size > 0 as size_t
                || (*cnx).cnx_state as ::core::ffi::c_uint
                    == picoquic_state_disconnected as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                next_wake_time = current_time;
                (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
                (*(*cnx).quic).wake_line = 4531 as ::core::ffi::c_int;
            }
            *send_length = (*send_length).wrapping_add(packet_size);
            if send_msg_size.is_null() {
                break;
            }
            if packet_size > *send_msg_size {
                *send_msg_size = packet_size;
            } else if packet_size != *send_msg_size {
                if *send_length > 0 as size_t {
                    if packet_size == 0 as size_t
                        && *send_length < (8 as size_t).wrapping_mul(*send_msg_size)
                    {
                        if (**(*cnx).path.offset(path_id as isize)).cwin
                            <= (**(*cnx).path.offset(path_id as isize)).bytes_in_transit
                        {
                            (*cnx).nb_trains_blocked_cwin =
                                (*cnx).nb_trains_blocked_cwin.wrapping_add(1);
                        } else if picoquic_is_pacing_blocked(
                            &raw mut (**(*cnx).path.offset(path_id as isize)).pacing,
                        ) != 0
                        {
                            (*cnx).nb_trains_blocked_pacing =
                                (*cnx).nb_trains_blocked_pacing.wrapping_add(1);
                        } else {
                            (*cnx).nb_trains_blocked_others =
                                (*cnx).nb_trains_blocked_others.wrapping_add(1);
                        }
                    } else {
                        (*cnx).nb_trains_short = (*cnx).nb_trains_short.wrapping_add(1);
                    }
                }
                break;
            } else if (*send_length).wrapping_add(*send_msg_size) > send_buffer_max {
                break;
            }
        }
        if *send_length > 0 as size_t {
            (*cnx).nb_trains_sent = (*cnx).nb_trains_sent.wrapping_add(1);
        }
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = picoquic_program_app_wake_time(cnx, &raw mut next_wake_time);
    }
    picoquic_reinsert_by_wake_time((*cnx).quic, cnx, next_wake_time);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_packet(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
    mut send_buffer: *mut uint8_t,
    mut send_buffer_max: size_t,
    mut send_length: *mut size_t,
    mut p_addr_to: *mut sockaddr_storage,
    mut p_addr_from: *mut sockaddr_storage,
    mut if_index: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    return picoquic_prepare_packet_ex(
        cnx,
        -(1 as ::core::ffi::c_int),
        current_time,
        send_buffer,
        send_buffer_max,
        send_length,
        p_addr_to,
        p_addr_from,
        if_index,
        ::core::ptr::null_mut::<size_t>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_close(
    mut cnx: *mut picoquic_cnx_t,
    mut application_reason_code: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current_time: uint64_t = picoquic_get_quic_time((*cnx).quic as *mut picoquic_quic_t);
    if (*cnx).cnx_state as ::core::ffi::c_uint
        == picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_server_false_start as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_client_ready_start as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cnx).cnx_state = picoquic_state_disconnecting;
        (*cnx).application_error = application_reason_code;
    } else if ((*cnx).cnx_state as ::core::ffi::c_uint)
        < picoquic_state_client_ready_start as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cnx).cnx_state = picoquic_state_handshake_failure;
        (*cnx).application_error = 0 as uint64_t;
        (*cnx).local_error = PICOQUIC_TRANSPORT_APPLICATION_ERROR as uint64_t;
    } else {
        ret = -(1 as ::core::ffi::c_int);
    }
    (*cnx).offending_frame_type = 0 as uint64_t;
    picoquic_reinsert_by_wake_time((*cnx).quic, cnx, current_time);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_close_immediate(mut cnx: *mut picoquic_cnx_t) {
    if ((*cnx).cnx_state as ::core::ffi::c_uint)
        < picoquic_state_draining as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut current_time: uint64_t =
            picoquic_get_quic_time((*cnx).quic as *mut picoquic_quic_t);
        let mut exit_time: uint64_t = current_time.wrapping_add((3 as uint64_t).wrapping_mul(
            (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).retransmit_timer,
        ));
        (*cnx).cnx_state = picoquic_state_draining;
        (*cnx).local_error = UINT64_MAX as uint64_t;
        (*cnx).latest_progress_time = current_time;
        (*cnx).last_close_sent = current_time;
        picoquic_reinsert_by_wake_time((*cnx).quic, cnx, exit_time);
        (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
        (*(*cnx).quic).wake_line = 4623 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_next_packet_ex(
    mut quic: *mut picoquic_quic_t,
    mut current_time: uint64_t,
    mut send_buffer: *mut uint8_t,
    mut send_buffer_max: size_t,
    mut send_length: *mut size_t,
    mut p_addr_to: *mut sockaddr_storage,
    mut p_addr_from: *mut sockaddr_storage,
    mut if_index: *mut ::core::ffi::c_int,
    mut log_cid: *mut picoquic_connection_id_t,
    mut p_last_cnx: *mut *mut picoquic_cnx_t,
    mut send_msg_size: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sp: *mut picoquic_stateless_packet_t =
        picoquic_dequeue_stateless_packet(quic as *mut picoquic_quic_t);
    if !p_last_cnx.is_null() {
        *p_last_cnx = ::core::ptr::null_mut::<picoquic_cnx_t>();
    }
    if !sp.is_null() {
        if (*sp).length > send_buffer_max {
            *send_length = 0 as size_t;
        } else {
            memcpy(
                send_buffer as *mut ::core::ffi::c_void,
                &raw mut (*sp).bytes as *mut uint8_t as *const ::core::ffi::c_void,
                (*sp).length,
            );
            *send_length = (*sp).length;
            picoquic_store_addr(p_addr_to, &raw mut (*sp).addr_to as *mut sockaddr);
            picoquic_store_addr(p_addr_from, &raw mut (*sp).addr_local as *mut sockaddr);
            *if_index = (*sp).if_index_local;
            if !log_cid.is_null() {
                *log_cid = (*sp).initial_cid;
            }
        }
        picoquic_delete_stateless_packet(sp);
    } else {
        let mut cnx: *mut picoquic_cnx_t =
            picoquic_get_earliest_cnx_to_wake(quic as *mut picoquic_quic_t, current_time)
                as *mut picoquic_cnx_t;
        if cnx.is_null() {
            *send_length = 0 as size_t;
        } else {
            ret = picoquic_prepare_packet_ex(
                cnx,
                -(1 as ::core::ffi::c_int),
                current_time,
                send_buffer,
                send_buffer_max,
                send_length,
                p_addr_to,
                p_addr_from,
                if_index,
                send_msg_size,
            );
            if !log_cid.is_null() {
                *log_cid = (*cnx).initial_cnxid;
            }
            if ret == PICOQUIC_ERROR_DISCONNECTED {
                ret = 0 as ::core::ffi::c_int;
                picoquic_log_app_message(
                    cnx as *mut picoquic_cnx_t,
                    b"Closed. Retrans= %d, spurious= %d, max sp gap = %d, max sp delay = %d, dg-coal: %f\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    (*cnx).nb_retransmission_total as ::core::ffi::c_int,
                    (*cnx).nb_spurious as ::core::ffi::c_int,
                    (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                        .max_reorder_gap as ::core::ffi::c_int,
                    (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                        .max_spurious_rtt as ::core::ffi::c_int,
                    if (*cnx).nb_trains_sent > 0 as uint64_t {
                        (*cnx).nb_packets_sent as ::core::ffi::c_double
                            / (*cnx).nb_trains_sent as ::core::ffi::c_double
                    } else {
                        0.0f64
                    },
                );
                if !(*quic).F_log.is_null() {
                    fflush((*quic).F_log as *mut FILE);
                }
                if !(*cnx).f_binlog.is_null() {
                    fflush((*cnx).f_binlog);
                }
                if (*cnx).client_mode() != 0 {
                    picoquic_reinsert_by_wake_time((*cnx).quic, cnx, UINT64_MAX as uint64_t);
                    (*(*cnx).quic).wake_file = 1 as ::core::ffi::c_int;
                    (*(*cnx).quic).wake_line = 4691 as ::core::ffi::c_int;
                } else {
                    picoquic_delete_cnx(cnx as *mut picoquic_cnx_t);
                }
            } else {
                if *if_index == -(1 as ::core::ffi::c_int) {
                    *if_index = picoquic_get_local_if_index(cnx as *mut picoquic_cnx_t)
                        as ::core::ffi::c_int;
                }
                if !p_last_cnx.is_null() {
                    *p_last_cnx = cnx;
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_next_packet(
    mut quic: *mut picoquic_quic_t,
    mut current_time: uint64_t,
    mut send_buffer: *mut uint8_t,
    mut send_buffer_max: size_t,
    mut send_length: *mut size_t,
    mut p_addr_to: *mut sockaddr_storage,
    mut p_addr_from: *mut sockaddr_storage,
    mut if_index: *mut ::core::ffi::c_int,
    mut log_cid: *mut picoquic_connection_id_t,
    mut p_last_cnx: *mut *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    return picoquic_prepare_next_packet_ex(
        quic,
        current_time,
        send_buffer,
        send_buffer_max,
        send_length,
        p_addr_to,
        p_addr_from,
        if_index,
        log_cid,
        p_last_cnx,
        ::core::ptr::null_mut::<size_t>(),
    );
}

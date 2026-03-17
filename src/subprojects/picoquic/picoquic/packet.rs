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
    fn picoquic_create_cnx(
        quic: *mut picoquic_quic_t,
        initial_cnx_id: picoquic_connection_id_t,
        remote_cnx_id: picoquic_connection_id_t,
        addr_to: *const sockaddr,
        start_time: uint64_t,
        preferred_version: uint32_t,
        sni: *const ::core::ffi::c_char,
        alpn: *const ::core::ffi::c_char,
        client_mode: ::core::ffi::c_char,
    ) -> *mut picoquic_cnx_t;
    fn picoquic_delete_cnx(cnx: *mut picoquic_cnx_t);
    fn picoquic_prepare_packet_ex(
        cnx: *mut picoquic_cnx_t,
        path_id_request: ::core::ffi::c_int,
        current_time: uint64_t,
        send_buffer: *mut uint8_t,
        send_buffer_max: size_t,
        send_length: *mut size_t,
        p_addr_to: *mut sockaddr_storage,
        p_addr_from: *mut sockaddr_storage,
        if_index: *mut ::core::ffi::c_int,
        send_msg_size: *mut size_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_cnx_is_still_logging(cnx: *mut picoquic_cnx_t) -> ::core::ffi::c_int;
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
    fn picoquic_is_connection_id_null(
        cnx_id: *const picoquic_connection_id_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_compare_connection_id(
        cnx_id1: *const picoquic_connection_id_t,
        cnx_id2: *const picoquic_connection_id_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_val64_connection_id(cnx_id: picoquic_connection_id_t) -> uint64_t;
    fn picoquic_compare_addr(
        expected: *const sockaddr,
        actual: *const sockaddr,
    ) -> ::core::ffi::c_int;
    fn picoquic_store_addr(stored_addr: *mut sockaddr_storage, addr: *const sockaddr);
    fn picoquic_frames_varlen_decode(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
        n: *mut size_t,
    ) -> *const uint8_t;
    fn picoquic_frames_uint8_decode(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
        n: *mut uint8_t,
    ) -> *const uint8_t;
    fn picoquic_frames_uint32_decode(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
        n: *mut uint32_t,
    ) -> *const uint8_t;
    fn picoquic_frames_cid_decode(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
        cid: *mut picoquic_connection_id_t,
    ) -> *const uint8_t;
    static picoquic_supported_versions: [picoquic_version_parameters_t; 0];
    static picoquic_nb_supported_versions: size_t;
    fn picoquic_get_version_index(proposed_version: uint32_t) -> ::core::ffi::c_int;
    static mut picoquic_spin_function_table: [picoquic_spinbit_def_t; 0];
    fn picoquic_create_stateless_packet(
        quic: *mut picoquic_quic_t,
    ) -> *mut picoquic_stateless_packet_t;
    fn picoquic_queue_stateless_packet(
        quic: *mut picoquic_quic_t,
        sp: *mut picoquic_stateless_packet_t,
    );
    fn picoquic_delete_stateless_packet(sp: *mut picoquic_stateless_packet_t);
    fn picoquic_create_local_cnx_id(
        quic: *mut picoquic_quic_t,
        cnx_id: *mut picoquic_connection_id_t,
        id_length: uint8_t,
        cnx_id_remote: picoquic_connection_id_t,
    );
    fn picoquic_create_path(
        cnx: *mut picoquic_cnx_t,
        start_time: uint64_t,
        local_addr: *const sockaddr,
        peer_addr: *const sockaddr,
        unique_path_id: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_register_path(cnx: *mut picoquic_cnx_t, path_x: *mut picoquic_path_t);
    fn picoquic_renew_connection_id(
        cnx: *mut picoquic_cnx_t,
        path_id: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_set_path_challenge(
        cnx: *mut picoquic_cnx_t,
        path_id: ::core::ffi::c_int,
        current_time: uint64_t,
    );
    fn picoquic_find_path_by_address(
        cnx: *mut picoquic_cnx_t,
        addr_local: *const sockaddr,
        addr_peer: *const sockaddr,
        partial_match: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_find_path_by_unique_id(
        cnx: *mut picoquic_cnx_t,
        unique_path_id: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_assign_peer_cnxid_to_path(
        cnx: *mut picoquic_cnx_t,
        path_id: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_dereference_stashed_cnxid(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
        is_deleting_cnx: ::core::ffi::c_int,
    );
    fn picoquic_reset_cnx(cnx: *mut picoquic_cnx_t, current_time: uint64_t) -> ::core::ffi::c_int;
    fn picoquic_connection_error(
        cnx: *mut picoquic_cnx_t,
        local_error: uint64_t,
        frame_type: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_connection_disconnect(cnx: *mut picoquic_cnx_t);
    fn picoquic_cnx_by_id(
        quic: *mut picoquic_quic_t,
        cnx_id: picoquic_connection_id_t,
        l_cid_sequence: *mut *mut st_picoquic_local_cnxid_t,
    ) -> *mut picoquic_cnx_t;
    fn picoquic_cnx_by_net(
        quic: *mut picoquic_quic_t,
        addr: *const sockaddr,
    ) -> *mut picoquic_cnx_t;
    fn picoquic_cnx_by_icid(
        quic: *mut picoquic_quic_t,
        icid: *mut picoquic_connection_id_t,
        addr: *const sockaddr,
    ) -> *mut picoquic_cnx_t;
    fn picoquic_cnx_by_secret(
        quic: *mut picoquic_quic_t,
        reset_secret: *const uint8_t,
        addr: *const sockaddr,
    ) -> *mut picoquic_cnx_t;
    fn picoquic_reinsert_by_wake_time(
        quic: *mut picoquic_quic_t,
        cnx: *mut picoquic_cnx_t,
        next_time: uint64_t,
    );
    fn picoformat_32(bytes: *mut uint8_t, n32: uint32_t);
    fn picoquic_create_long_header(
        packet_type: picoquic_packet_type_enum,
        dest_cnx_id: *mut picoquic_connection_id_t,
        srce_cnx_id: *mut picoquic_connection_id_t,
        do_grease_quic_bit: ::core::ffi::c_int,
        version: uint32_t,
        version_index: ::core::ffi::c_int,
        sequence_number: uint64_t,
        retry_token_length: size_t,
        retry_token: *mut uint8_t,
        bytes: *mut uint8_t,
        pn_offset: *mut size_t,
        pn_length: *mut size_t,
    ) -> size_t;
    fn picoquic_update_payload_length(
        bytes: *mut uint8_t,
        pnum_index: size_t,
        header_length: size_t,
        packet_length: size_t,
    );
    fn picoquic_get_checksum_length(
        cnx: *mut picoquic_cnx_t,
        is_cleartext_mode: picoquic_epoch_enum,
    ) -> size_t;
    fn picoquic_protect_packet_header(
        send_buffer: *mut uint8_t,
        pn_offset: size_t,
        first_mask: uint8_t,
        pn_enc: *mut ::core::ffi::c_void,
    );
    fn picoquic_implicit_handshake_ack(
        cnx: *mut picoquic_cnx_t,
        pc: picoquic_packet_context_enum,
        current_time: uint64_t,
    );
    fn picoquic_ready_state_transition(cnx: *mut picoquic_cnx_t, current_time: uint64_t);
    fn picoquic_is_pn_already_received(
        cnx: *mut picoquic_cnx_t,
        pc: picoquic_packet_context_enum,
        l_cid: *mut picoquic_local_cnxid_t,
        pn64: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_record_pn_received(
        cnx: *mut picoquic_cnx_t,
        pc: picoquic_packet_context_enum,
        l_cid: *mut picoquic_local_cnxid_t,
        pn64: uint64_t,
        current_microsec: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_ack_ctx_from_cnx_context(
        cnx: *mut picoquic_cnx_t,
        pc: picoquic_packet_context_enum,
        l_cid: *mut picoquic_local_cnxid_t,
    ) -> *mut picoquic_ack_context_t;
    fn picoquic_sack_list_from_cnx_context(
        cnx: *mut picoquic_cnx_t,
        pc: picoquic_packet_context_enum,
        l_cid: *mut picoquic_local_cnxid_t,
    ) -> *mut picoquic_sack_list_t;
    fn picoquic_sack_list_last(first_sack: *mut picoquic_sack_list_t) -> uint64_t;
    fn picoquic_compute_ack_gap_and_delay(
        cnx: *mut picoquic_cnx_t,
        rtt: uint64_t,
        remote_min_ack_delay: uint64_t,
        data_rate: uint64_t,
        ack_gap: *mut uint64_t,
        ack_delay_max: *mut uint64_t,
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
    fn picoquic_set_ack_needed(
        cnx: *mut picoquic_cnx_t,
        current_time: uint64_t,
        pc: picoquic_packet_context_enum,
        path_x: *mut picoquic_path_t,
        is_immediate_ack_required: ::core::ffi::c_int,
    );
    fn picoquic_stream_data_node_recycle(stream_data: *mut picoquic_stream_data_node_t);
    fn picoquic_stream_data_node_alloc(
        quic: *mut picoquic_quic_t,
    ) -> *mut picoquic_stream_data_node_t;
    fn picoquic_find_local_cnxid(
        cnx: *mut picoquic_cnx_t,
        unique_path_id: uint64_t,
        cnxid: *mut picoquic_connection_id_t,
    ) -> *mut picoquic_local_cnxid_t;
    fn picoquic_decode_frames(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
        bytes: *const uint8_t,
        bytes_max: size_t,
        received_data: *mut picoquic_stream_data_node_t,
        epoch: ::core::ffi::c_int,
        addr_from: *mut sockaddr,
        addr_to: *mut sockaddr,
        pn64: uint64_t,
        path_is_not_allocated: ::core::ffi::c_int,
        current_time: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_skip_frame(
        bytes: *const uint8_t,
        bytes_max: size_t,
        consumed: *mut size_t,
        pure_ack: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_decode_closing_frames(
        cnx: *mut picoquic_cnx_t,
        bytes: *mut uint8_t,
        bytes_max: size_t,
        closing_received: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_process_version_upgrade(
        cnx: *mut picoquic_cnx_t,
        old_version_index: ::core::ffi::c_int,
        new_version_index: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
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
    fn picoquic_log_context_free_app_message(
        quic: *mut picoquic_quic_t,
        cid: *const picoquic_connection_id_t,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn picoquic_log_quic_pdu(
        quic: *mut picoquic_quic_t,
        receiving: ::core::ffi::c_int,
        current_time: uint64_t,
        cid64: uint64_t,
        addr_peer: *const sockaddr,
        addr_local: *const sockaddr,
        packet_length: size_t,
    );
    fn picoquic_log_pdu(
        cnx: *mut picoquic_cnx_t,
        receiving: ::core::ffi::c_int,
        current_time: uint64_t,
        addr_peer: *const sockaddr,
        addr_local: *const sockaddr,
        packet_length: size_t,
    );
    fn picoquic_log_packet(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
        receiving: ::core::ffi::c_int,
        current_time: uint64_t,
        ph: *mut st_picoquic_packet_header_t,
        bytes: *const uint8_t,
        bytes_max: size_t,
    );
    fn picoquic_log_dropped_packet(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
        ph: *mut st_picoquic_packet_header_t,
        packet_size: size_t,
        err: ::core::ffi::c_int,
        raw_data: *mut uint8_t,
        current_time: uint64_t,
    );
    fn picoquic_log_buffered_packet(
        cnx: *mut picoquic_cnx_t,
        path_x: *mut picoquic_path_t,
        ptype: picoquic_packet_type_enum,
        current_time: uint64_t,
    );
    fn picoquic_log_close_connection(cnx: *mut picoquic_cnx_t);
    fn picoquic_log_cc_dump(cnx: *mut picoquic_cnx_t, current_time: uint64_t);
    fn picoquic_tls_stream_process(
        cnx: *mut picoquic_cnx_t,
        data_consumed: *mut ::core::ffi::c_int,
        current_time: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_is_tls_complete(cnx: *mut picoquic_cnx_t) -> ::core::ffi::c_int;
    fn picoquic_public_random_64() -> uint64_t;
    fn picoquic_public_random(buf: *mut ::core::ffi::c_void, len: size_t);
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
    fn picoquic_aead_decrypt_generic(
        output: *mut uint8_t,
        input: *const uint8_t,
        input_length: size_t,
        seq_num: uint64_t,
        auth_data: *const uint8_t,
        auth_data_length: size_t,
        aead_ctx: *mut ::core::ffi::c_void,
    ) -> size_t;
    fn picoquic_aead_decrypt_mp(
        output: *mut uint8_t,
        input: *const uint8_t,
        input_length: size_t,
        path_id: uint64_t,
        seq_num: uint64_t,
        auth_data: *const uint8_t,
        auth_data_length: size_t,
        aead_context: *mut ::core::ffi::c_void,
    ) -> size_t;
    fn picoquic_aead_integrity_limit(aead_ctx: *mut ::core::ffi::c_void) -> uint64_t;
    fn picoquic_aead_free(aead_context: *mut ::core::ffi::c_void);
    fn picoquic_cipher_free(cipher_context: *mut ::core::ffi::c_void);
    fn picoquic_pn_iv_size(pn_enc: *mut ::core::ffi::c_void) -> size_t;
    fn picoquic_pn_encrypt(
        pn_enc: *mut ::core::ffi::c_void,
        iv: *const ::core::ffi::c_void,
        output: *mut ::core::ffi::c_void,
        input: *const ::core::ffi::c_void,
        len: size_t,
    );
    fn picoquic_get_initial_aead_context(
        quic: *mut picoquic_quic_t,
        version_index: ::core::ffi::c_int,
        initial_cnxid: *mut picoquic_connection_id_t,
        is_client: ::core::ffi::c_int,
        is_enc: ::core::ffi::c_int,
        aead_ctx: *mut *mut ::core::ffi::c_void,
        pn_enc_ctx: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn picoquic_compute_new_rotated_keys(cnx: *mut picoquic_cnx_t) -> ::core::ffi::c_int;
    fn picoquic_apply_rotated_keys(cnx: *mut picoquic_cnx_t, is_enc: ::core::ffi::c_int);
    fn picoquic_crypto_context_free(ctx: *mut picoquic_crypto_context_t);
    fn picoquic_create_cnxid_reset_secret(
        quic: *mut picoquic_quic_t,
        cnx_id: *mut picoquic_connection_id_t,
        reset_secret: *mut uint8_t,
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
    fn picoquic_verify_retry_token(
        quic: *mut picoquic_quic_t,
        addr_peer: *const sockaddr,
        current_time: uint64_t,
        is_new_token: *mut ::core::ffi::c_int,
        odcid: *mut picoquic_connection_id_t,
        rcid: *const picoquic_connection_id_t,
        initial_pn: uint32_t,
        token: *const uint8_t,
        token_size: size_t,
        check_reuse: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn picoquic_find_retry_protection_context(
        quic: *mut picoquic_quic_t,
        version_index: ::core::ffi::c_int,
        sending: ::core::ffi::c_int,
    ) -> *mut ::core::ffi::c_void;
    fn picoquic_encode_retry_protection(
        integrity_aead: *mut ::core::ffi::c_void,
        bytes: *mut uint8_t,
        bytes_max: size_t,
        byte_index: size_t,
        odcid: *const picoquic_connection_id_t,
    ) -> size_t;
    fn picoquic_verify_retry_protection(
        integrity_aead: *mut ::core::ffi::c_void,
        bytes: *mut uint8_t,
        length: *mut size_t,
        byte_index: size_t,
        odcid: *const picoquic_connection_id_t,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
pub type picoquic_packet_header = st_picoquic_packet_header_t;
pub type picoquic_spinbit_incoming_fn = Option<
    unsafe extern "C" fn(
        *mut picoquic_cnx_t,
        *mut picoquic_path_t,
        *mut picoquic_packet_header,
    ) -> (),
>;
pub type picoquic_spinbit_def_t = st_picoquic_spinbit_def_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_spinbit_def_t {
    pub spinbit_incoming: picoquic_spinbit_incoming_fn,
    pub spinbit_outgoing: picoquic_spinbit_outgoing_fn,
}
pub type picoquic_spinbit_outgoing_fn =
    Option<unsafe extern "C" fn(*mut picoquic_cnx_t) -> uint8_t>;
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
pub const picoquic_frame_type_connection_close: C2Rust_Unnamed_0 = 28;
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
pub const picoquic_frame_type_padding: C2Rust_Unnamed_0 = 0;
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
pub const PICOQUIC_ERROR_DUPLICATE: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 1 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_AEAD_CHECK: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 3 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_UNEXPECTED_PACKET: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 4 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_MEMORY: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 5 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_CNXID_CHECK: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 7 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_INITIAL_TOO_SHORT: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 8 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_RETRY: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 19 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_DETECTED: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 21 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_STATELESS_RESET: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 30 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_CONNECTION_DELETED: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 31 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_CNXID_SEGMENT: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 32 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_INVALID_TOKEN: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 38 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_INITIAL_CID_TOO_SHORT: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 39 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_AEAD_NOT_READY: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 41 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_PACKET_HEADER_PARSING: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 45 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_VERSION_NOT_SUPPORTED: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 50 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_VERSION_NEGOTIATION: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 55 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_PACKET_TOO_LONG: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 56 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_PACKET_WRONG_VERSION: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 57 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_PORT_BLOCKED: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 58 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_RETRY_NEEDED: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 61 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_SERVER_BUSY: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 62 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_SERVER_BUSY: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION: ::core::ffi::c_int = 0xa as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_AEAD_LIMIT_REACHED: ::core::ffi::c_int = 0xf as ::core::ffi::c_int;
pub const PICOQUIC_MAX_PACKET_SIZE: ::core::ffi::c_int = 1536 as ::core::ffi::c_int;
pub const PICOQUIC_RESET_SECRET_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PICOQUIC_RESET_PACKET_PAD_SIZE: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const PICOQUIC_RESET_PACKET_MIN_SIZE: ::core::ffi::c_int =
    PICOQUIC_RESET_PACKET_PAD_SIZE + PICOQUIC_RESET_SECRET_SIZE;
pub const PICOQUIC_CONNECTION_ID_MAX_SIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const PICOQUIC_ENFORCED_INITIAL_MTU: ::core::ffi::c_int = 129 as ::core::ffi::c_int;
pub const PICOQUIC_ENFORCED_INITIAL_CID_LENGTH: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PICOQUIC_NB_PATH_TARGET: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PICOQUIC_ACK_DELAY_MIN: ::core::ffi::c_ulonglong = 1000 as ::core::ffi::c_ulonglong;
pub const PICOQUIC_TOKEN_DELAY_SHORT: ::core::ffi::c_ulonglong =
    ((2 as ::core::ffi::c_int * 60 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong)
        .wrapping_mul(1000000 as ::core::ffi::c_ulonglong);
pub const PICOQUIC_BANDWIDTH_TIME_INTERVAL_MIN: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
pub const PICOQUIC_V1_VERSION: uint32_t = 1 as uint32_t;
pub const PICOQUIC_V2_VERSION: uint32_t = 1798521807 as uint32_t;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_long_packet_type(
    mut flags: uint8_t,
    mut version_index: ::core::ffi::c_int,
) -> picoquic_packet_type_enum {
    let mut pt: picoquic_packet_type_enum = picoquic_packet_error;
    match (*(&raw const picoquic_supported_versions as *const picoquic_version_parameters_t)
        .offset(version_index as isize))
    .packet_type_version
    {
        1 => {
            match flags as ::core::ffi::c_int >> 4 as ::core::ffi::c_int & 3 as ::core::ffi::c_int {
                0 => {
                    pt = picoquic_packet_initial;
                }
                1 => {
                    pt = picoquic_packet_0rtt_protected;
                }
                2 => {
                    pt = picoquic_packet_handshake;
                }
                3 => {
                    pt = picoquic_packet_retry;
                }
                _ => {}
            }
        }
        1798521807 => {
            match flags as ::core::ffi::c_int >> 4 as ::core::ffi::c_int & 3 as ::core::ffi::c_int {
                1 => {
                    pt = picoquic_packet_initial;
                }
                2 => {
                    pt = picoquic_packet_0rtt_protected;
                }
                3 => {
                    pt = picoquic_packet_handshake;
                }
                0 => {
                    pt = picoquic_packet_retry;
                }
                _ => {}
            }
        }
        _ => {}
    }
    return pt;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_screen_initial_packet(
    mut quic: *mut picoquic_quic_t,
    mut bytes: *const uint8_t,
    mut packet_length: size_t,
    mut addr_from: *const sockaddr,
    mut ph: *mut picoquic_packet_header,
    mut current_time: uint64_t,
    mut pcnx: *mut *mut picoquic_cnx_t,
    mut new_ctx_created: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if packet_length < PICOQUIC_ENFORCED_INITIAL_MTU as size_t {
        ret = PICOQUIC_ERROR_INITIAL_TOO_SHORT;
    } else if ((*ph).dest_cnx_id.id_len as ::core::ffi::c_int)
        < PICOQUIC_ENFORCED_INITIAL_CID_LENGTH
    {
        ret = PICOQUIC_ERROR_INITIAL_CID_TOO_SHORT;
    } else if (*ph).has_reserved_bit_set() != 0 {
        ret = PICOQUIC_ERROR_PACKET_HEADER_PARSING;
    } else if (*pcnx).is_null() {
        let mut aead_ctx: *mut ::core::ffi::c_void = NULL;
        let mut pn_dec_ctx: *mut ::core::ffi::c_void = NULL;
        let mut decrypted_bytes: [uint8_t; 1536] = [0; 1536];
        let mut dph: picoquic_packet_header = *ph;
        if picoquic_get_initial_aead_context(
            quic,
            (*ph).version_index,
            &raw mut (*ph).dest_cnx_id,
            0 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            &raw mut aead_ctx,
            &raw mut pn_dec_ctx,
        ) == 0 as ::core::ffi::c_int
        {
            ret = picoquic_remove_header_protection_inner(
                bytes as *mut uint8_t,
                (*ph).offset.wrapping_add((*ph).payload_length),
                &raw mut decrypted_bytes as *mut uint8_t,
                &raw mut dph,
                pn_dec_ctx,
                0 as ::core::ffi::c_uint,
                0 as uint64_t,
            );
            if ret == 0 as ::core::ffi::c_int {
                let mut decrypted_length: size_t = picoquic_aead_decrypt_generic(
                    (&raw mut decrypted_bytes as *mut uint8_t).offset(dph.offset as isize),
                    bytes.offset(dph.offset as isize),
                    dph.payload_length,
                    dph.pn64,
                    &raw mut decrypted_bytes as *mut uint8_t,
                    dph.offset,
                    aead_ctx,
                );
                if decrypted_length >= dph.payload_length {
                    ret = PICOQUIC_ERROR_AEAD_CHECK;
                }
            }
        } else {
            ret = PICOQUIC_ERROR_MEMORY;
        }
        if !aead_ctx.is_null() {
            picoquic_aead_free(aead_ctx);
        }
        if !pn_dec_ctx.is_null() {
            picoquic_cipher_free(pn_dec_ctx);
        }
        if ret == 0 as ::core::ffi::c_int {
            if (*quic).enforce_client_only() != 0 {
                ret = PICOQUIC_ERROR_SERVER_BUSY;
            } else if (*quic).server_busy() != 0 {
                ret = PICOQUIC_ERROR_SERVER_BUSY;
            } else {
                let mut is_address_blocked: ::core::ffi::c_int =
                    ((*quic).is_port_blocking_disabled() == 0
                        && picoquic_check_addr_blocked(addr_from) != 0)
                        as ::core::ffi::c_int;
                let mut is_new_token: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut has_good_token: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut has_bad_token: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                let mut original_cnxid: picoquic_connection_id_t = st_picoquic_connection_id_t {
                    id: [
                        0 as ::core::ffi::c_int as uint8_t,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                        0,
                    ],
                    id_len: 0,
                };
                if (*ph).token_length > 0 as size_t {
                    if picoquic_verify_retry_token(
                        quic,
                        addr_from,
                        current_time,
                        &raw mut is_new_token,
                        &raw mut original_cnxid,
                        &raw mut (*ph).dest_cnx_id,
                        dph.pn64 as uint32_t,
                        (*ph).token_bytes,
                        (*ph).token_length,
                        1 as ::core::ffi::c_int,
                    ) == 0 as ::core::ffi::c_int
                    {
                        has_good_token = 1 as ::core::ffi::c_int;
                    } else {
                        has_bad_token = 1 as ::core::ffi::c_int;
                    }
                }
                if has_bad_token != 0 && is_new_token == 0 {
                    ret = PICOQUIC_ERROR_INVALID_TOKEN;
                } else if has_good_token == 0
                    && ((*quic).force_check_token() as ::core::ffi::c_int != 0
                        || (*quic).max_half_open_before_retry <= (*quic).current_number_half_open
                        || is_address_blocked != 0)
                {
                    ret = PICOQUIC_ERROR_RETRY_NEEDED;
                } else {
                    *pcnx = picoquic_create_cnx(
                        quic as *mut picoquic_quic_t,
                        (*ph).dest_cnx_id,
                        (*ph).srce_cnx_id,
                        addr_from,
                        current_time,
                        (*ph).vn,
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        ::core::ptr::null::<::core::ffi::c_char>(),
                        0 as ::core::ffi::c_char,
                    ) as *mut picoquic_cnx_t;
                    if (*pcnx).is_null() {
                        ret = PICOQUIC_ERROR_MEMORY;
                    } else if has_good_token != 0 {
                        (**pcnx)
                            .set_initial_validated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                        picoquic_parse_connection_id(
                            &raw mut original_cnxid.id as *mut uint8_t,
                            original_cnxid.id_len,
                            &raw mut (**pcnx).original_cnxid,
                        );
                    }
                }
            }
        }
    } else if (**pcnx).client_mode() == 0
        && picoquic_compare_connection_id(
            &raw mut (*ph).dest_cnx_id,
            &raw mut (**pcnx).initial_cnxid,
        ) == 0 as ::core::ffi::c_int
        || picoquic_compare_connection_id(
            &raw mut (*ph).dest_cnx_id,
            &raw mut (*(**(**pcnx).path.offset(0 as ::core::ffi::c_int as isize)).p_local_cnxid)
                .cnx_id,
        ) == 0 as ::core::ffi::c_int
    {
        if picoquic_is_connection_id_null(
            &raw mut (*(**(**pcnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid)
                .cnx_id,
        ) != 0
        {
            (*(**(**pcnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid).cnx_id =
                (*ph).srce_cnx_id;
        } else if picoquic_compare_connection_id(
            &raw mut (*(**(**pcnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid)
                .cnx_id,
            &raw mut (*ph).srce_cnx_id,
        ) != 0 as ::core::ffi::c_int
        {
            ret = PICOQUIC_ERROR_UNEXPECTED_PACKET;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_long_packet_header(
    mut quic: *mut picoquic_quic_t,
    mut bytes: *const uint8_t,
    mut length: size_t,
    mut addr_from: *const sockaddr,
    mut ph: *mut picoquic_packet_header,
    mut pcnx: *mut *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bytes_start: *const uint8_t = bytes;
    let mut bytes_max: *const uint8_t = bytes.offset(length as isize);
    let mut flags: uint8_t = 0 as uint8_t;
    bytes = picoquic_frames_uint8_decode(bytes, bytes_max, &raw mut flags);
    if bytes.is_null() || {
        bytes = picoquic_frames_uint32_decode(bytes, bytes_max, &raw mut (*ph).vn);
        bytes.is_null()
    } {
        ret = -(1 as ::core::ffi::c_int);
    } else if (*ph).vn != 0 as uint32_t {
        (*ph).version_index = picoquic_get_version_index((*ph).vn);
        if (*ph).version_index < 0 as ::core::ffi::c_int {
            (*ph).ptype = picoquic_packet_error;
            (*ph).pc = picoquic_packet_context_application;
            ret = PICOQUIC_ERROR_VERSION_NOT_SUPPORTED;
        }
    }
    if ret == 0 as ::core::ffi::c_int && {
        bytes = picoquic_frames_cid_decode(bytes, bytes_max, &raw mut (*ph).dest_cnx_id);
        bytes.is_null() || {
            bytes = picoquic_frames_cid_decode(bytes, bytes_max, &raw mut (*ph).srce_cnx_id);
            bytes.is_null()
        }
    } {
        ret = -(1 as ::core::ffi::c_int);
    }
    if ret == 0 as ::core::ffi::c_int {
        (*ph).offset = bytes.offset_from(bytes_start) as ::core::ffi::c_long as size_t;
        if (*ph).vn == 0 as uint32_t {
            (*ph).ptype = picoquic_packet_version_negotiation;
            (*ph).pc = picoquic_packet_context_initial;
            (*ph).payload_length = (if length > (*ph).offset {
                length.wrapping_sub((*ph).offset)
            } else {
                0 as size_t
            }) as uint16_t as size_t;
            (*ph).pl_val = (*ph).payload_length;
            if (*pcnx).is_null() && !quic.is_null() {
                if (*quic).local_cnxid_length as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    *pcnx = picoquic_cnx_by_net(quic, addr_from);
                } else if (*ph).dest_cnx_id.id_len as ::core::ffi::c_int
                    == (*quic).local_cnxid_length as ::core::ffi::c_int
                {
                    *pcnx = picoquic_cnx_by_id(quic, (*ph).dest_cnx_id, &raw mut (*ph).l_cid);
                }
            }
        } else {
            let mut payload_length: size_t = 0 as size_t;
            (*ph).set_spin(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*ph).set_has_spin_bit(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*ph).set_quic_bit_is_zero(
                (flags as ::core::ffi::c_int & 0x40 as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                    as ::core::ffi::c_uint as ::core::ffi::c_uint,
            );
            (*ph).ptype = picoquic_parse_long_packet_type(flags, (*ph).version_index);
            match (*ph).ptype as ::core::ffi::c_uint {
                2 => {
                    let mut tok_len: size_t = 0 as size_t;
                    bytes = picoquic_frames_varlen_decode(bytes, bytes_max, &raw mut tok_len);
                    let mut bytes_left: size_t =
                        bytes_max.offset_from(bytes) as ::core::ffi::c_long as size_t;
                    (*ph).epoch = picoquic_epoch_initial;
                    if bytes.is_null() || bytes_left < tok_len {
                        (*ph).ptype = picoquic_packet_error;
                        (*ph).pc = picoquic_packet_context_application;
                        (*ph).offset = length;
                    } else {
                        (*ph).pc = picoquic_packet_context_initial;
                        (*ph).token_length = tok_len;
                        (*ph).token_bytes = bytes;
                        bytes = bytes.offset(tok_len as isize);
                        (*ph).offset =
                            bytes.offset_from(bytes_start) as ::core::ffi::c_long as size_t;
                    }
                }
                5 => {
                    (*ph).pc = picoquic_packet_context_application;
                    (*ph).epoch = picoquic_epoch_0rtt;
                }
                4 => {
                    (*ph).pc = picoquic_packet_context_handshake;
                    (*ph).epoch = picoquic_epoch_handshake;
                }
                3 => {
                    (*ph).pc = picoquic_packet_context_initial;
                    (*ph).epoch = picoquic_epoch_initial;
                }
                _ => {
                    (*ph).ptype = picoquic_packet_error;
                    (*ph).version_index = -(1 as ::core::ffi::c_int);
                    (*ph).pc = picoquic_packet_context_application;
                }
            }
            if (*ph).ptype as ::core::ffi::c_uint
                == picoquic_packet_retry as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                if length > (*ph).offset {
                    payload_length = length.wrapping_sub((*ph).offset);
                } else {
                    payload_length = 0 as size_t;
                    (*ph).ptype = picoquic_packet_error;
                }
            } else if (*ph).ptype as ::core::ffi::c_uint
                != picoquic_packet_error as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                bytes = picoquic_frames_varlen_decode(bytes, bytes_max, &raw mut payload_length);
                let mut bytes_left_0: size_t = (if bytes_max > bytes {
                    bytes_max.offset_from(bytes) as ::core::ffi::c_long
                } else {
                    0 as ::core::ffi::c_long
                }) as size_t;
                if bytes.is_null()
                    || bytes_left_0 < payload_length
                    || (*ph).version_index < 0 as ::core::ffi::c_int
                {
                    (*ph).ptype = picoquic_packet_error;
                    (*ph).payload_length = (if length > (*ph).offset {
                        length.wrapping_sub((*ph).offset)
                    } else {
                        0 as size_t
                    }) as uint16_t as size_t;
                    (*ph).pl_val = (*ph).payload_length;
                }
            }
            if (*ph).ptype as ::core::ffi::c_uint
                != picoquic_packet_error as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*ph).pl_val = payload_length as uint16_t as size_t;
                (*ph).payload_length = payload_length as uint16_t as size_t;
                (*ph).offset = bytes.offset_from(bytes_start) as ::core::ffi::c_long as size_t;
                (*ph).pn_offset = (*ph).offset;
                if (*pcnx).is_null() {
                    if (*quic).local_cnxid_length as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                        *pcnx = picoquic_cnx_by_net(quic, addr_from);
                    } else {
                        if (*ph).dest_cnx_id.id_len as ::core::ffi::c_int
                            == (*quic).local_cnxid_length as ::core::ffi::c_int
                        {
                            *pcnx =
                                picoquic_cnx_by_id(quic, (*ph).dest_cnx_id, &raw mut (*ph).l_cid);
                        }
                        if (*pcnx).is_null()
                            && ((*ph).ptype as ::core::ffi::c_uint
                                == picoquic_packet_initial as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                                || (*ph).ptype as ::core::ffi::c_uint
                                    == picoquic_packet_0rtt_protected as ::core::ffi::c_int
                                        as ::core::ffi::c_uint)
                        {
                            *pcnx =
                                picoquic_cnx_by_icid(quic, &raw mut (*ph).dest_cnx_id, addr_from);
                        } else {
                            (*pcnx).is_null();
                        }
                    }
                }
                if (*ph).quic_bit_is_zero() as ::core::ffi::c_int != 0
                    && !(*pcnx).is_null()
                    && (**pcnx).local_parameters.do_grease_quic_bit == 0
                {
                    (*ph).ptype = picoquic_packet_error;
                }
            } else if (*pcnx).is_null() {
                if (*quic).local_cnxid_length as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                    *pcnx = picoquic_cnx_by_net(quic, addr_from);
                } else if (*ph).dest_cnx_id.id_len as ::core::ffi::c_int
                    == (*quic).local_cnxid_length as ::core::ffi::c_int
                {
                    *pcnx = picoquic_cnx_by_id(quic, (*ph).dest_cnx_id, &raw mut (*ph).l_cid);
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_short_packet_header(
    mut quic: *mut picoquic_quic_t,
    mut bytes: *const uint8_t,
    mut length: size_t,
    mut addr_from: *const sockaddr,
    mut ph: *mut picoquic_packet_header,
    mut pcnx: *mut *mut picoquic_cnx_t,
    mut receiving: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cnxid_length: uint8_t = (if receiving == 0 as ::core::ffi::c_int && !(*pcnx).is_null() {
        (*(**(**pcnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid)
            .cnx_id
            .id_len as ::core::ffi::c_int
    } else {
        (*quic).local_cnxid_length as ::core::ffi::c_int
    }) as uint8_t;
    (*ph).pc = picoquic_packet_context_application;
    (*ph).pl_val = 0 as size_t;
    if length as ::core::ffi::c_int >= 1 as ::core::ffi::c_int + cnxid_length as ::core::ffi::c_int
    {
        (*ph).offset =
            (1 as ::core::ffi::c_int as size_t).wrapping_add(picoquic_parse_connection_id(
                bytes.offset(1 as ::core::ffi::c_int as isize),
                cnxid_length,
                &raw mut (*ph).dest_cnx_id,
            ) as size_t);
        if (*pcnx).is_null() {
            if (*quic).local_cnxid_length as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
                *pcnx = picoquic_cnx_by_id(quic, (*ph).dest_cnx_id, &raw mut (*ph).l_cid);
            } else {
                *pcnx = picoquic_cnx_by_net(quic, addr_from);
            }
        }
    } else {
        (*ph).ptype = picoquic_packet_error;
        (*ph).offset = length;
        (*ph).payload_length = 0 as size_t;
    }
    if !(*pcnx).is_null() {
        let mut has_loss_bit: ::core::ffi::c_int = (receiving != 0
            && (**pcnx).is_loss_bit_enabled_incoming() as ::core::ffi::c_int != 0
            || receiving == 0 && (**pcnx).is_loss_bit_enabled_outgoing() as ::core::ffi::c_int != 0)
            as ::core::ffi::c_int;
        (*ph).epoch = picoquic_epoch_1rtt;
        (*ph).version_index = (**pcnx).version_index;
        (*ph).set_quic_bit_is_zero(
            (*bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                & 0x40 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int) as ::core::ffi::c_int as ::core::ffi::c_uint
                as ::core::ffi::c_uint,
        );
        if (*ph).quic_bit_is_zero() == 0 || (**pcnx).local_parameters.do_grease_quic_bit != 0 {
            (*ph).ptype = picoquic_packet_1rtt_protected;
        } else {
            (*ph).ptype = picoquic_packet_error;
        }
        (*ph).set_has_spin_bit(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*ph).set_spin(
            (*bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >> 5 as ::core::ffi::c_int
                & 1 as ::core::ffi::c_int) as ::core::ffi::c_uint
                as ::core::ffi::c_uint,
        );
        (*ph).pn_offset = (*ph).offset;
        (*ph).pn = 0 as uint32_t;
        (*ph).pnmask = 0 as uint64_t;
        (*ph).set_key_phase(
            (*bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >> 2 as ::core::ffi::c_int
                & 1 as ::core::ffi::c_int) as ::core::ffi::c_uint
                as ::core::ffi::c_uint,
        );
        if has_loss_bit != 0 {
            (*ph).set_has_loss_bits(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*ph).set_loss_bit_L(
                (*bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    >> 3 as ::core::ffi::c_int
                    & 1 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    as ::core::ffi::c_uint,
            );
            (*ph).set_loss_bit_Q(
                (*bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                    >> 4 as ::core::ffi::c_int
                    & 1 as ::core::ffi::c_int) as ::core::ffi::c_uint
                    as ::core::ffi::c_uint,
            );
        }
        if length < (*ph).offset
            || (*ph).ptype as ::core::ffi::c_uint
                == picoquic_packet_error as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            ret = -(1 as ::core::ffi::c_int);
            (*ph).payload_length = 0 as size_t;
        } else {
            (*ph).payload_length = length.wrapping_sub((*ph).offset) as uint16_t as size_t;
        }
    } else {
        (*ph).ptype = picoquic_packet_1rtt_protected;
        (*ph).payload_length = (if length > (*ph).offset {
            length.wrapping_sub((*ph).offset)
        } else {
            0 as size_t
        }) as uint16_t as size_t;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_packet_header(
    mut quic: *mut picoquic_quic_t,
    mut bytes: *const uint8_t,
    mut length: size_t,
    mut addr_from: *const sockaddr,
    mut ph: *mut picoquic_packet_header,
    mut pcnx: *mut *mut picoquic_cnx_t,
    mut receiving: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    memset(
        ph as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<picoquic_packet_header>() as size_t,
    );
    (*ph).version_index = -(1 as ::core::ffi::c_int);
    if *bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
        & 0x80 as ::core::ffi::c_int
        == 0x80 as ::core::ffi::c_int
    {
        ret = picoquic_parse_long_packet_header(quic, bytes, length, addr_from, ph, pcnx);
    } else {
        ret =
            picoquic_parse_short_packet_header(quic, bytes, length, addr_from, ph, pcnx, receiving);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_packet_number64(
    mut highest: uint64_t,
    mut mask: uint64_t,
    mut pn: uint32_t,
) -> uint64_t {
    let mut expected: uint64_t = highest.wrapping_add(1 as uint64_t);
    let mut not_mask_plus_one: uint64_t = (!mask).wrapping_add(1 as uint64_t);
    let mut pn64: uint64_t = expected & mask | pn as uint64_t;
    if pn64 < expected {
        let mut delta1: uint64_t = expected.wrapping_sub(pn64);
        let mut delta2: uint64_t = not_mask_plus_one.wrapping_sub(delta1);
        if delta2 < delta1 {
            pn64 = pn64.wrapping_add(not_mask_plus_one);
        }
    } else {
        let mut delta1_0: uint64_t = pn64.wrapping_sub(expected);
        let mut delta2_0: uint64_t = not_mask_plus_one.wrapping_sub(delta1_0);
        if delta2_0 <= delta1_0 && pn64 & mask > 0 as uint64_t {
            pn64 = pn64.wrapping_sub(not_mask_plus_one);
        }
    }
    return pn64;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_log_pn_dec_trial(mut cnx: *mut picoquic_cnx_t) {
    if (*(*cnx).quic).log_pn_dec() as ::core::ffi::c_int != 0
        && (!(*(*cnx).quic).F_log.is_null() || !(*cnx).f_binlog.is_null())
    {
        let mut pn_dec: *mut ::core::ffi::c_void =
            (*cnx).crypto_context[picoquic_epoch_1rtt as ::core::ffi::c_int as usize].pn_dec;
        let mut pn_enc: *mut ::core::ffi::c_void =
            (*cnx).crypto_context[picoquic_epoch_1rtt as ::core::ffi::c_int as usize].pn_enc;
        let mut test_iv: [uint8_t; 32] = [
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            3 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            4 as ::core::ffi::c_int as uint8_t,
            6 as ::core::ffi::c_int as uint8_t,
            7 as ::core::ffi::c_int as uint8_t,
            8 as ::core::ffi::c_int as uint8_t,
            9 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            1 as ::core::ffi::c_int as uint8_t,
            0,
            0,
            0,
        ];
        let mut mask_length: size_t = 5 as size_t;
        let mut mask_bytes: [uint8_t; 5] = [
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ];
        let mut demask_bytes: [uint8_t; 5] = [
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ];
        if !pn_enc.is_null() {
            picoquic_pn_encrypt(
                pn_enc,
                &raw mut test_iv as *mut uint8_t as *const ::core::ffi::c_void,
                &raw mut mask_bytes as *mut uint8_t as *mut ::core::ffi::c_void,
                &raw mut mask_bytes as *mut uint8_t as *const ::core::ffi::c_void,
                mask_length,
            );
        }
        if !pn_dec.is_null() {
            picoquic_pn_encrypt(
                pn_dec,
                &raw mut test_iv as *mut uint8_t as *const ::core::ffi::c_void,
                &raw mut demask_bytes as *mut uint8_t as *mut ::core::ffi::c_void,
                &raw mut demask_bytes as *mut uint8_t as *const ::core::ffi::c_void,
                mask_length,
            );
        }
        picoquic_log_app_message(
            cnx as *mut picoquic_cnx_t,
            b"1RTT PN ENC/DEC, Phi: %d, signature = %02x%02x%02x%02x%02x, %02x%02x%02x%02x%02x\0"
                .as_ptr() as *const ::core::ffi::c_char,
            (*cnx).key_phase_enc() as ::core::ffi::c_int,
            mask_bytes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            mask_bytes[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            mask_bytes[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            mask_bytes[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            mask_bytes[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            demask_bytes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            demask_bytes[1 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            demask_bytes[2 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            demask_bytes[3 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
            demask_bytes[4 as ::core::ffi::c_int as usize] as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_remove_header_protection_inner(
    mut bytes: *mut uint8_t,
    mut length: size_t,
    mut decrypted_bytes: *mut uint8_t,
    mut ph: *mut picoquic_packet_header,
    mut pn_enc: *mut ::core::ffi::c_void,
    mut is_loss_bit_enabled_incoming: ::core::ffi::c_uint,
    mut sack_list_last: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !pn_enc.is_null() {
        let mut mask_length: size_t = 5 as size_t;
        let mut sample_offset: size_t = (*ph).pn_offset.wrapping_add(4 as size_t);
        let mut sample_size: size_t = picoquic_pn_iv_size(pn_enc);
        let mut mask_bytes: [uint8_t; 5] = [
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ];
        if sample_offset.wrapping_add(sample_size) > length {
            (*ph).pn = 0xffffffff as ::core::ffi::c_uint as uint32_t;
            (*ph).pnmask = 0xffffffff00000000 as uint64_t;
            (*ph).offset = (*ph).pn_offset;
        } else {
            let mut first_byte: uint8_t = *bytes.offset(0 as ::core::ffi::c_int as isize);
            let mut first_mask: uint8_t = (if first_byte as ::core::ffi::c_int
                & 0x80 as ::core::ffi::c_int
                == 0x80 as ::core::ffi::c_int
            {
                0xf as ::core::ffi::c_int
            } else if is_loss_bit_enabled_incoming != 0 {
                0x7 as ::core::ffi::c_int
            } else {
                0x1f as ::core::ffi::c_int
            }) as uint8_t;
            let mut pn_l: uint8_t = 0;
            let mut pn_val: uint32_t = 0 as uint32_t;
            memcpy(
                decrypted_bytes as *mut ::core::ffi::c_void,
                bytes as *const ::core::ffi::c_void,
                (*ph).pn_offset,
            );
            picoquic_pn_encrypt(
                pn_enc,
                bytes.offset(sample_offset as isize) as *const ::core::ffi::c_void,
                &raw mut mask_bytes as *mut uint8_t as *mut ::core::ffi::c_void,
                &raw mut mask_bytes as *mut uint8_t as *const ::core::ffi::c_void,
                mask_length,
            );
            first_byte = (first_byte as ::core::ffi::c_int
                ^ mask_bytes[0 as ::core::ffi::c_int as usize] as ::core::ffi::c_int
                    & first_mask as ::core::ffi::c_int) as uint8_t;
            pn_l = ((first_byte as ::core::ffi::c_int & 3 as ::core::ffi::c_int)
                + 1 as ::core::ffi::c_int) as uint8_t;
            (*ph).pnmask = 0xffffffffffffffff as uint64_t;
            *decrypted_bytes.offset(0 as ::core::ffi::c_int as isize) = first_byte;
            let mut i: uint8_t = 1 as uint8_t;
            while i as ::core::ffi::c_int <= pn_l as ::core::ffi::c_int {
                pn_val <<= 8 as ::core::ffi::c_int;
                *decrypted_bytes.offset((*ph).offset as isize) =
                    (*bytes.offset((*ph).offset as isize) as ::core::ffi::c_int
                        ^ mask_bytes[i as usize] as ::core::ffi::c_int)
                        as uint8_t;
                let c2rust_fresh21 = (*ph).offset;
                (*ph).offset = (*ph).offset.wrapping_add(1);
                pn_val = pn_val
                    .wrapping_add(*decrypted_bytes.offset(c2rust_fresh21 as isize) as uint32_t);
                (*ph).pnmask <<= 8 as ::core::ffi::c_int;
                i = i.wrapping_add(1);
            }
            (*ph).pn = pn_val;
            (*ph).payload_length = (*ph).payload_length.wrapping_sub(pn_l as size_t);
            if (*ph).ptype as ::core::ffi::c_uint
                == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                (*ph).set_key_phase(
                    (first_byte as ::core::ffi::c_int >> 2 as ::core::ffi::c_int
                        & 1 as ::core::ffi::c_int) as ::core::ffi::c_uint
                        as ::core::ffi::c_uint,
                );
            }
            (*ph).pn64 = picoquic_get_packet_number64(sack_list_last, (*ph).pnmask, (*ph).pn);
            if first_byte as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
            {
                (*ph).set_has_reserved_bit_set(
                    (is_loss_bit_enabled_incoming == 0
                        && first_byte as ::core::ffi::c_int & 0x18 as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_uint as ::core::ffi::c_uint,
                );
            } else {
                (*ph).set_has_reserved_bit_set(
                    (first_byte as ::core::ffi::c_int & 0xc as ::core::ffi::c_int
                        != 0 as ::core::ffi::c_int) as ::core::ffi::c_int
                        as ::core::ffi::c_uint as ::core::ffi::c_uint,
                );
            }
        }
    } else {
        (*ph).pn = 0xffffffff as ::core::ffi::c_uint as uint32_t;
        (*ph).pnmask = 0xffffffff00000000 as uint64_t;
        (*ph).offset = (*ph).pn_offset;
        (*ph).pn64 = 0xffffffffffffffff as uint64_t;
        ret = PICOQUIC_ERROR_AEAD_NOT_READY;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_remove_header_protection(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut decrypted_bytes: *mut uint8_t,
    mut ph: *mut picoquic_packet_header,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut length: size_t = (*ph).offset.wrapping_add((*ph).payload_length);
    let mut pn_enc: *mut ::core::ffi::c_void = (*cnx).crypto_context[(*ph).epoch as usize].pn_dec;
    let mut sack_list: *mut picoquic_sack_list_t = picoquic_sack_list_from_cnx_context(
        cnx,
        (*ph).pc,
        (*ph).l_cid as *mut picoquic_local_cnxid_t,
    );
    ret = picoquic_remove_header_protection_inner(
        bytes,
        length,
        decrypted_bytes,
        ph,
        pn_enc,
        (*cnx).is_loss_bit_enabled_incoming(),
        picoquic_sack_list_last(sack_list),
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_remove_packet_protection(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut decoded_bytes: *mut uint8_t,
    mut ph: *mut picoquic_packet_header,
    mut current_time: uint64_t,
    mut already_received: *mut ::core::ffi::c_int,
) -> size_t {
    let mut decoded: size_t = 0;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !already_received.is_null() {
        if picoquic_is_pn_already_received(
            cnx,
            (*ph).pc,
            (*ph).l_cid as *mut picoquic_local_cnxid_t,
            (*ph).pn64,
        ) != 0 as ::core::ffi::c_int
        {
            *already_received = 1 as ::core::ffi::c_int;
        } else {
            *already_received = 0 as ::core::ffi::c_int;
        }
    }
    if (*ph).epoch as ::core::ffi::c_uint
        == picoquic_epoch_1rtt as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut need_integrity_check: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut ack_ctx: *mut picoquic_ack_context_t = picoquic_ack_ctx_from_cnx_context(
            cnx,
            picoquic_packet_context_application,
            (*ph).l_cid as *mut picoquic_local_cnxid_t,
        );
        if (*ph).key_phase() as ::core::ffi::c_int == (*cnx).key_phase_dec() as ::core::ffi::c_int {
            if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
                && (*ph).ptype as ::core::ffi::c_uint != 0
            {
                decoded = picoquic_aead_decrypt_mp(
                    decoded_bytes.offset((*ph).offset as isize),
                    bytes.offset((*ph).offset as isize),
                    (*ph).payload_length,
                    (*(*ph).l_cid).path_id,
                    (*ph).pn64,
                    decoded_bytes,
                    (*ph).offset,
                    (*cnx).crypto_context[picoquic_epoch_1rtt as ::core::ffi::c_int as usize]
                        .aead_decrypt,
                );
            } else {
                decoded = picoquic_aead_decrypt_generic(
                    decoded_bytes.offset((*ph).offset as isize),
                    bytes.offset((*ph).offset as isize),
                    (*ph).payload_length,
                    (*ph).pn64,
                    decoded_bytes,
                    (*ph).offset,
                    (*cnx).crypto_context[picoquic_epoch_1rtt as ::core::ffi::c_int as usize]
                        .aead_decrypt,
                );
            }
            if decoded <= (*ph).payload_length && (*ph).pn64 < (*ack_ctx).crypto_rotation_sequence {
                (*ack_ctx).crypto_rotation_sequence = (*ph).pn64;
            }
        } else if (*ack_ctx).crypto_rotation_sequence == UINT64_MAX as uint64_t
            && current_time <= (*cnx).crypto_rotation_time_guard
            || (*ph).pn64 < (*ack_ctx).crypto_rotation_sequence
        {
            if current_time > (*cnx).crypto_rotation_time_guard {
                decoded = (*ph).payload_length.wrapping_add(1 as size_t);
                need_integrity_check = 0 as ::core::ffi::c_int;
            } else if !(*cnx).crypto_context_old.aead_decrypt.is_null() {
                if (*cnx).is_multipath_enabled() != 0 {
                    decoded = picoquic_aead_decrypt_mp(
                        decoded_bytes.offset((*ph).offset as isize),
                        bytes.offset((*ph).offset as isize),
                        (*ph).payload_length,
                        (*(*ph).l_cid).path_id,
                        (*ph).pn64,
                        decoded_bytes,
                        (*ph).offset,
                        (*cnx).crypto_context_old.aead_decrypt,
                    );
                } else {
                    decoded = picoquic_aead_decrypt_generic(
                        decoded_bytes.offset((*ph).offset as isize),
                        bytes.offset((*ph).offset as isize),
                        (*ph).payload_length,
                        (*ph).pn64,
                        decoded_bytes,
                        (*ph).offset,
                        (*cnx).crypto_context_old.aead_decrypt,
                    );
                }
            } else {
                decoded = (*ph).payload_length.wrapping_add(1 as size_t);
                need_integrity_check = 0 as ::core::ffi::c_int;
            }
        } else {
            if (*cnx).crypto_context_new.aead_decrypt.is_null()
                && (*cnx).crypto_context_new.aead_encrypt.is_null()
            {
                ret = picoquic_compute_new_rotated_keys(cnx);
            }
            if ret == 0 as ::core::ffi::c_int && !(*cnx).crypto_context_new.aead_decrypt.is_null() {
                if (*cnx).is_multipath_enabled() != 0 {
                    decoded = picoquic_aead_decrypt_mp(
                        decoded_bytes.offset((*ph).offset as isize),
                        bytes.offset((*ph).offset as isize),
                        (*ph).payload_length,
                        (*(*ph).l_cid).path_id,
                        (*ph).pn64,
                        decoded_bytes,
                        (*ph).offset,
                        (*cnx).crypto_context_new.aead_decrypt,
                    );
                } else {
                    decoded = picoquic_aead_decrypt_generic(
                        decoded_bytes.offset((*ph).offset as isize),
                        bytes.offset((*ph).offset as isize),
                        (*ph).payload_length,
                        (*ph).pn64,
                        decoded_bytes,
                        (*ph).offset,
                        (*cnx).crypto_context_new.aead_decrypt,
                    );
                }
                if decoded <= (*ph).payload_length {
                    (*cnx).crypto_rotation_time_guard = current_time.wrapping_add(
                        (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).retransmit_timer,
                    );
                    if (*cnx).is_multipath_enabled() != 0 {
                        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        while i < (*cnx).nb_paths {
                            (**(*cnx).path.offset(i as isize))
                                .ack_ctx
                                .crypto_rotation_sequence = UINT64_MAX as uint64_t;
                            i += 1;
                        }
                    }
                    (*ack_ctx).crypto_rotation_sequence = (*ph).pn64;
                    picoquic_apply_rotated_keys(cnx, 0 as ::core::ffi::c_int);
                    (*cnx).nb_crypto_key_rotations = (*cnx).nb_crypto_key_rotations.wrapping_add(1);
                    if !(*cnx).crypto_context_new.aead_encrypt.is_null() {
                        picoquic_apply_rotated_keys(cnx, 1 as ::core::ffi::c_int);
                    }
                }
            } else {
                decoded = (*ph).payload_length.wrapping_add(1 as size_t);
                need_integrity_check = 0 as ::core::ffi::c_int;
            }
        }
        if need_integrity_check != 0 && decoded > (*ph).payload_length {
            (*cnx).crypto_failure_count = (*cnx).crypto_failure_count.wrapping_add(1);
            if (*cnx).crypto_failure_count
                > picoquic_aead_integrity_limit(
                    (*cnx).crypto_context[picoquic_epoch_1rtt as ::core::ffi::c_int as usize]
                        .aead_decrypt,
                )
            {
                picoquic_log_app_message(
                    cnx as *mut picoquic_cnx_t,
                    b"AEAD Integrity limit reached after 0x%lx failed decryptions.\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*cnx).crypto_failure_count,
                );
                picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_AEAD_LIMIT_REACHED as uint64_t,
                    0 as uint64_t,
                );
            }
        }
    } else if !(*cnx).crypto_context[(*ph).epoch as usize]
        .aead_decrypt
        .is_null()
    {
        if (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
            && (*ph).ptype as ::core::ffi::c_uint
                == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            decoded = picoquic_aead_decrypt_mp(
                decoded_bytes.offset((*ph).offset as isize),
                bytes.offset((*ph).offset as isize),
                (*ph).payload_length,
                (*(*ph).l_cid).path_id,
                (*ph).pn64,
                decoded_bytes,
                (*ph).offset,
                (*cnx).crypto_context[picoquic_epoch_1rtt as ::core::ffi::c_int as usize]
                    .aead_decrypt,
            );
        } else {
            decoded = picoquic_aead_decrypt_generic(
                decoded_bytes.offset((*ph).offset as isize),
                bytes.offset((*ph).offset as isize),
                (*ph).payload_length,
                (*ph).pn64,
                decoded_bytes,
                (*ph).offset,
                (*cnx).crypto_context[(*ph).epoch as usize].aead_decrypt,
            );
        }
    } else {
        decoded = (*ph).payload_length.wrapping_add(1 as size_t);
    }
    if decoded > (*ph).payload_length {
        picoquic_log_pn_dec_trial(cnx);
    }
    return decoded;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_header_and_decrypt(
    mut quic: *mut picoquic_quic_t,
    mut bytes: *const uint8_t,
    mut length: size_t,
    mut packet_length: size_t,
    mut addr_from: *const sockaddr,
    mut current_time: uint64_t,
    mut decrypted_data: *mut picoquic_stream_data_node_t,
    mut ph: *mut picoquic_packet_header,
    mut pcnx: *mut *mut picoquic_cnx_t,
    mut consumed: *mut size_t,
    mut new_ctx_created: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut already_received: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut decoded_length: size_t = 0 as size_t;
    let mut ret: ::core::ffi::c_int = picoquic_parse_packet_header(
        quic,
        bytes,
        length,
        addr_from,
        ph,
        pcnx,
        1 as ::core::ffi::c_int,
    );
    *new_ctx_created = 0 as ::core::ffi::c_int;
    if ret == 0 as ::core::ffi::c_int {
        if (*ph).offset.wrapping_add((*ph).payload_length) > PICOQUIC_MAX_PACKET_SIZE as size_t {
            ret = PICOQUIC_ERROR_PACKET_TOO_LONG;
            if *new_ctx_created != 0 {
                picoquic_delete_cnx(*pcnx);
                *pcnx = ::core::ptr::null_mut::<picoquic_cnx_t>();
                *new_ctx_created = 0 as ::core::ffi::c_int;
            }
        } else if (*ph).ptype as ::core::ffi::c_uint
            != picoquic_packet_version_negotiation as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*ph).ptype as ::core::ffi::c_uint
                != picoquic_packet_retry as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*ph).ptype as ::core::ffi::c_uint
                != picoquic_packet_error as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            length = (*ph).offset.wrapping_add((*ph).payload_length);
            *consumed = length;
            if (*pcnx).is_null() {
                if (*ph).ptype as ::core::ffi::c_uint
                    == picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    ret = picoquic_screen_initial_packet(
                        quic,
                        bytes,
                        packet_length,
                        addr_from,
                        ph,
                        current_time,
                        pcnx,
                        new_ctx_created,
                    );
                }
            } else if (**pcnx).client_mode() == 0
                && (*ph).ptype as ::core::ffi::c_uint
                    == picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint
                && packet_length < PICOQUIC_ENFORCED_INITIAL_MTU as size_t
            {
                ret = PICOQUIC_ERROR_INITIAL_TOO_SHORT;
            }
            if ret == 0 as ::core::ffi::c_int {
                if !(*pcnx).is_null() {
                    if (*ph).version_index != (**pcnx).version_index {
                        if (**pcnx).client_mode() as ::core::ffi::c_int != 0
                            && ((**pcnx).cnx_state as ::core::ffi::c_uint)
                                < picoquic_state_client_almost_ready as ::core::ffi::c_int
                                    as ::core::ffi::c_uint
                            && (*ph).version_index >= 0 as ::core::ffi::c_int
                            && (*(&raw const picoquic_supported_versions
                                as *const picoquic_version_parameters_t)
                                .offset((*ph).version_index as isize))
                            .version
                                == (**pcnx).desired_version
                        {
                            ret = picoquic_process_version_upgrade(
                                *pcnx,
                                (**pcnx).version_index,
                                (*ph).version_index,
                            );
                        } else {
                            ret = PICOQUIC_ERROR_PACKET_WRONG_VERSION;
                        }
                    }
                    if ret == 0 as ::core::ffi::c_int {
                        ret = picoquic_remove_header_protection(
                            *pcnx,
                            bytes as *mut uint8_t,
                            &raw mut (*decrypted_data).data as *mut uint8_t,
                            ph,
                        );
                    }
                    if ret == 0 as ::core::ffi::c_int {
                        decoded_length = picoquic_remove_packet_protection(
                            *pcnx,
                            bytes as *mut uint8_t,
                            &raw mut (*decrypted_data).data as *mut uint8_t,
                            ph,
                            current_time,
                            &raw mut already_received,
                        );
                    } else {
                        decoded_length = (*ph).payload_length.wrapping_add(1 as size_t);
                    }
                    if decoded_length > length.wrapping_sub((*ph).offset) {
                        if (*ph).ptype as ::core::ffi::c_uint
                            == picoquic_packet_1rtt_protected as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                            && length >= PICOQUIC_RESET_PACKET_MIN_SIZE as size_t
                            && memcmp(
                                bytes
                                    .offset(length as isize)
                                    .offset(-(PICOQUIC_RESET_SECRET_SIZE as isize))
                                    as *const ::core::ffi::c_void,
                                &raw mut (*(**(**pcnx)
                                    .path
                                    .offset(0 as ::core::ffi::c_int as isize))
                                .p_remote_cnxid)
                                    .reset_secret as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                PICOQUIC_RESET_SECRET_SIZE as size_t,
                            ) == 0 as ::core::ffi::c_int
                        {
                            ret = PICOQUIC_ERROR_STATELESS_RESET;
                            picoquic_log_app_message(
                                *pcnx,
                                b"Decrypt error, matching reset secret, ret = %d\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                ret,
                            );
                        } else {
                            if ret != PICOQUIC_ERROR_AEAD_NOT_READY {
                                ret = PICOQUIC_ERROR_AEAD_CHECK;
                            }
                            if *new_ctx_created != 0 {
                                picoquic_delete_cnx(*pcnx);
                                *pcnx = ::core::ptr::null_mut::<picoquic_cnx_t>();
                                *new_ctx_created = 0 as ::core::ffi::c_int;
                            }
                        }
                    } else if already_received != 0 as ::core::ffi::c_int {
                        ret = PICOQUIC_ERROR_DUPLICATE;
                    } else {
                        (*ph).payload_length = decoded_length as uint16_t as size_t;
                    }
                } else if (*ph).ptype as ::core::ffi::c_uint
                    == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
                {
                    if length >= PICOQUIC_RESET_PACKET_MIN_SIZE as size_t {
                        *pcnx = picoquic_cnx_by_secret(
                            quic,
                            bytes
                                .offset(length as isize)
                                .offset(-(PICOQUIC_RESET_SECRET_SIZE as isize)),
                            addr_from,
                        );
                        if !(*pcnx).is_null() {
                            ret = PICOQUIC_ERROR_STATELESS_RESET;
                            picoquic_log_app_message(
                                *pcnx,
                                b"Found connection from reset secret, ret = %d\0".as_ptr()
                                    as *const ::core::ffi::c_char,
                                ret,
                            );
                        }
                    }
                }
            }
        } else {
            memmove(
                &raw mut (*decrypted_data).data as *mut uint8_t as *mut ::core::ffi::c_void,
                bytes as *const ::core::ffi::c_void,
                length,
            );
            *consumed = length;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_incoming_version_negotiation(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut length: size_t,
    mut addr_from: *mut sockaddr,
    mut ph: *mut picoquic_packet_header,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !((*cnx).cnx_state as ::core::ffi::c_uint
        != picoquic_state_client_init_sent as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        if picoquic_compare_connection_id(
            &raw mut (*ph).dest_cnx_id,
            &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_local_cnxid)
                .cnx_id,
        ) != 0 as ::core::ffi::c_int
            || (*ph).vn != 0 as uint32_t
        {
            ret = PICOQUIC_ERROR_DETECTED;
        } else if picoquic_compare_connection_id(
            &raw mut (*ph).srce_cnx_id,
            &raw mut (*cnx).initial_cnxid,
        ) != 0 as ::core::ffi::c_int
            || (*ph).vn != 0 as uint32_t
        {
            ret = PICOQUIC_ERROR_DETECTED;
        } else {
            let mut v_bytes: *const uint8_t = bytes.offset((*ph).offset as isize);
            let mut bytes_max: *const uint8_t = bytes.offset(length as isize);
            let mut nb_vn: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while v_bytes < bytes_max {
                let mut vn: uint32_t = 0 as uint32_t;
                v_bytes = picoquic_frames_uint32_decode(v_bytes, bytes_max, &raw mut vn);
                if v_bytes.is_null() {
                    ret = PICOQUIC_ERROR_DETECTED;
                    break;
                } else if vn == (*cnx).proposed_version || vn == 0 as uint32_t {
                    ret = PICOQUIC_ERROR_DETECTED;
                    break;
                } else if picoquic_get_version_index(vn) >= 0 as ::core::ffi::c_int {
                    nb_vn += 1;
                }
            }
            if ret == 0 as ::core::ffi::c_int {
                if nb_vn == 0 as ::core::ffi::c_int {
                    ret = PICOQUIC_ERROR_DETECTED;
                } else {
                    if (*cnx).callback_fn.is_some() && length > (*ph).offset {
                        (*cnx).callback_fn.expect("non-null function pointer")(
                            cnx as *mut picoquic_cnx_t,
                            0 as uint64_t,
                            bytes.offset((*ph).offset as isize),
                            length.wrapping_sub((*ph).offset),
                            picoquic_callback_version_negotiation,
                            (*cnx).callback_ctx,
                            NULL,
                        );
                    }
                    (*cnx).remote_error = PICOQUIC_ERROR_VERSION_NEGOTIATION as uint64_t;
                    picoquic_connection_disconnect(cnx);
                    ret = 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_version_negotiation(
    mut quic: *mut picoquic_quic_t,
    mut addr_from: *mut sockaddr,
    mut addr_to: *mut sockaddr,
    mut if_index_to: ::core::ffi::c_ulong,
    mut ph: *mut picoquic_packet_header,
    mut original_bytes: *mut uint8_t,
) {
    let mut cnx: *mut picoquic_cnx_t = ::core::ptr::null_mut::<picoquic_cnx_t>();
    let mut dcid_length: uint8_t = *original_bytes.offset(5 as ::core::ffi::c_int as isize);
    let mut dcid: *mut uint8_t = original_bytes.offset(6 as ::core::ffi::c_int as isize);
    let mut scid_length: uint8_t = *original_bytes
        .offset((6 as ::core::ffi::c_int + dcid_length as ::core::ffi::c_int) as isize);
    let mut scid: *mut uint8_t = original_bytes
        .offset(6 as ::core::ffi::c_int as isize)
        .offset(dcid_length as ::core::ffi::c_int as isize)
        .offset(1 as ::core::ffi::c_int as isize);
    if dcid_length as ::core::ffi::c_int <= PICOQUIC_CONNECTION_ID_MAX_SIZE {
        picoquic_parse_connection_id(dcid, dcid_length, &raw mut (*ph).dest_cnx_id);
        if (*ph).dest_cnx_id.id_len as ::core::ffi::c_int
            == (*quic).local_cnxid_length as ::core::ffi::c_int
        {
            if (*quic).local_cnxid_length as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                cnx = picoquic_cnx_by_net(quic, addr_from);
            } else {
                cnx = picoquic_cnx_by_id(quic, (*ph).dest_cnx_id, &raw mut (*ph).l_cid);
            }
        }
        if cnx.is_null() {
            cnx = picoquic_cnx_by_icid(quic, &raw mut (*ph).dest_cnx_id, addr_from);
        }
    }
    if cnx.is_null() {
        let mut sp: *mut picoquic_stateless_packet_t =
            picoquic_create_stateless_packet(quic as *mut picoquic_quic_t);
        if !sp.is_null() {
            let mut bytes: *mut uint8_t = &raw mut (*sp).bytes as *mut uint8_t;
            let mut byte_index: size_t = 0 as size_t;
            let mut rand_vn: uint32_t = 0;
            picoquic_public_random(
                bytes.offset(byte_index as isize) as *mut ::core::ffi::c_void,
                1 as size_t,
            );
            let c2rust_fresh3 = byte_index;
            byte_index = byte_index.wrapping_add(1);
            let ref mut c2rust_fresh4 = *bytes.offset(c2rust_fresh3 as isize);
            *c2rust_fresh4 =
                (*c2rust_fresh4 as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as uint8_t;
            picoformat_32(bytes.offset(byte_index as isize), 0 as uint32_t);
            byte_index = byte_index.wrapping_add(4 as size_t);
            let c2rust_fresh5 = byte_index;
            byte_index = byte_index.wrapping_add(1);
            *bytes.offset(c2rust_fresh5 as isize) = scid_length;
            memcpy(
                bytes.offset(byte_index as isize) as *mut ::core::ffi::c_void,
                scid as *const ::core::ffi::c_void,
                scid_length as size_t,
            );
            byte_index = byte_index.wrapping_add(scid_length as size_t);
            let c2rust_fresh6 = byte_index;
            byte_index = byte_index.wrapping_add(1);
            *bytes.offset(c2rust_fresh6 as isize) = dcid_length;
            memcpy(
                bytes.offset(byte_index as isize) as *mut ::core::ffi::c_void,
                dcid as *const ::core::ffi::c_void,
                dcid_length as size_t,
            );
            byte_index = byte_index.wrapping_add(dcid_length as size_t);
            let mut i: size_t = 0 as size_t;
            while i < picoquic_nb_supported_versions {
                picoformat_32(
                    bytes.offset(byte_index as isize),
                    (*(&raw const picoquic_supported_versions
                        as *const picoquic_version_parameters_t)
                        .offset(i as isize))
                    .version,
                );
                byte_index = byte_index.wrapping_add(4 as size_t);
                i = i.wrapping_add(1);
            }
            loop {
                rand_vn = picoquic_public_random_64() as uint32_t & 0xf0f0f0f0 as uint32_t
                    | 0xa0a0a0a as uint32_t;
                if !(rand_vn == (*ph).vn) {
                    break;
                }
            }
            picoformat_32(bytes.offset(byte_index as isize), rand_vn);
            byte_index = byte_index.wrapping_add(4 as size_t);
            (*sp).length = byte_index;
            picoquic_store_addr(&raw mut (*sp).addr_to, addr_from);
            picoquic_store_addr(&raw mut (*sp).addr_local, addr_to);
            (*sp).if_index_local = if_index_to as ::core::ffi::c_int;
            (*sp).initial_cid = (*ph).dest_cnx_id;
            (*sp).cnxid_log64 = picoquic_val64_connection_id((*sp).initial_cid);
            (*sp).ptype = picoquic_packet_version_negotiation;
            picoquic_log_quic_pdu(
                quic,
                1 as ::core::ffi::c_int,
                picoquic_get_quic_time(quic as *mut picoquic_quic_t),
                0 as uint64_t,
                addr_to,
                addr_from,
                (*sp).length,
            );
            picoquic_queue_stateless_packet(quic as *mut picoquic_quic_t, sp);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_unexpected_cnxid(
    mut quic: *mut picoquic_quic_t,
    mut length: size_t,
    mut addr_from: *mut sockaddr,
    mut addr_to: *mut sockaddr,
    mut if_index_to: ::core::ffi::c_ulong,
    mut ph: *mut picoquic_packet_header,
    mut current_time: uint64_t,
) {
    if length > PICOQUIC_RESET_PACKET_MIN_SIZE as size_t
        && (*ph).ptype as ::core::ffi::c_uint
            == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*quic).stateless_reset_next_time <= current_time
    {
        let mut sp: *mut picoquic_stateless_packet_t =
            picoquic_create_stateless_packet(quic as *mut picoquic_quic_t);
        if !sp.is_null() {
            let mut pad_size: size_t = length
                .wrapping_sub(PICOQUIC_RESET_SECRET_SIZE as size_t)
                .wrapping_sub(2 as size_t);
            let mut bytes: *mut uint8_t = &raw mut (*sp).bytes as *mut uint8_t;
            let mut byte_index: size_t = 0 as size_t;
            if pad_size
                > (PICOQUIC_RESET_PACKET_MIN_SIZE
                    - PICOQUIC_RESET_SECRET_SIZE
                    - 1 as ::core::ffi::c_int) as size_t
            {
                pad_size = pad_size.wrapping_sub(picoquic_public_uniform_random(
                    (pad_size as uint64_t).wrapping_sub(
                        (PICOQUIC_RESET_PACKET_MIN_SIZE
                            - PICOQUIC_RESET_SECRET_SIZE
                            - 1 as ::core::ffi::c_int) as uint64_t,
                    ),
                ) as size_t);
            }
            let c2rust_fresh1 = byte_index;
            byte_index = byte_index.wrapping_add(1);
            *bytes.offset(c2rust_fresh1 as isize) = (0x40 as ::core::ffi::c_int
                | (picoquic_public_random_64() & 0x3f as uint64_t) as uint8_t as ::core::ffi::c_int)
                as uint8_t;
            picoquic_public_random(
                bytes.offset(byte_index as isize) as *mut ::core::ffi::c_void,
                pad_size,
            );
            byte_index = byte_index.wrapping_add(pad_size);
            picoquic_create_cnxid_reset_secret(
                quic,
                &raw mut (*ph).dest_cnx_id,
                bytes.offset(byte_index as isize),
            );
            byte_index = byte_index.wrapping_add(PICOQUIC_RESET_SECRET_SIZE as size_t);
            (*sp).length = byte_index;
            (*sp).ptype = picoquic_packet_1rtt_protected;
            picoquic_store_addr(&raw mut (*sp).addr_to, addr_from);
            picoquic_store_addr(&raw mut (*sp).addr_local, addr_to);
            (*sp).if_index_local = if_index_to as ::core::ffi::c_int;
            (*sp).initial_cid = (*ph).dest_cnx_id;
            (*sp).cnxid_log64 = picoquic_val64_connection_id((*sp).initial_cid);
            picoquic_log_context_free_app_message(
                quic,
                &raw mut (*sp).initial_cid,
                b"Unexpected connection ID, sending stateless reset.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
            picoquic_queue_stateless_packet(quic as *mut picoquic_quic_t, sp);
            (*quic).stateless_reset_next_time =
                current_time.wrapping_add((*quic).stateless_reset_min_interval);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_stateless_retry(
    mut quic: *mut picoquic_quic_t,
    mut ph: *mut picoquic_packet_header,
    mut s_cid: *mut picoquic_connection_id_t,
    mut addr_from: *const sockaddr,
    mut addr_to: *const sockaddr,
    mut if_index_to: ::core::ffi::c_ulong,
    mut retry_token: *mut uint8_t,
    mut retry_token_length: size_t,
) {
    let mut sp: *mut picoquic_stateless_packet_t =
        picoquic_create_stateless_packet(quic as *mut picoquic_quic_t);
    let mut integrity_aead: *mut ::core::ffi::c_void =
        picoquic_find_retry_protection_context(quic, (*ph).version_index, 1 as ::core::ffi::c_int);
    let mut checksum_length: size_t = if integrity_aead.is_null() {
        0 as size_t
    } else {
        picoquic_aead_get_checksum_length(integrity_aead)
    };
    if !sp.is_null() {
        let mut bytes: *mut uint8_t = &raw mut (*sp).bytes as *mut uint8_t;
        let mut byte_index: size_t = 0 as size_t;
        let mut header_length: size_t = 0 as size_t;
        let mut pn_offset: size_t = 0;
        let mut pn_length: size_t = 0;
        header_length = picoquic_create_long_header(
            picoquic_packet_retry,
            &raw mut (*ph).srce_cnx_id,
            s_cid,
            0 as ::core::ffi::c_int,
            (*ph).vn,
            (*ph).version_index,
            0 as uint64_t,
            retry_token_length,
            retry_token,
            bytes,
            &raw mut pn_offset,
            &raw mut pn_length,
        );
        byte_index = header_length;
        if byte_index.wrapping_add(retry_token_length) < PICOQUIC_MAX_PACKET_SIZE as size_t {
            memcpy(
                bytes.offset(byte_index as isize) as *mut ::core::ffi::c_void,
                retry_token as *const ::core::ffi::c_void,
                retry_token_length,
            );
            byte_index = byte_index.wrapping_add(retry_token_length);
        }
        if integrity_aead.is_null() {
            let c2rust_fresh2 = byte_index;
            byte_index = byte_index.wrapping_add(1);
            *bytes.offset(c2rust_fresh2 as isize) = (*ph).dest_cnx_id.id_len;
            byte_index = byte_index.wrapping_add(picoquic_format_connection_id(
                bytes.offset(byte_index as isize),
                (PICOQUIC_MAX_PACKET_SIZE as size_t)
                    .wrapping_sub(byte_index)
                    .wrapping_sub(checksum_length),
                (*ph).dest_cnx_id,
            ) as size_t);
        } else {
            byte_index = picoquic_encode_retry_protection(
                integrity_aead,
                bytes,
                PICOQUIC_MAX_PACKET_SIZE as size_t,
                byte_index,
                &raw mut (*ph).dest_cnx_id,
            );
        }
        (*sp).length = byte_index;
        (*sp).ptype = picoquic_packet_retry;
        picoquic_store_addr(&raw mut (*sp).addr_to, addr_from);
        picoquic_store_addr(&raw mut (*sp).addr_local, addr_to);
        (*sp).if_index_local = if_index_to as ::core::ffi::c_int;
        (*sp).cnxid_log64 = picoquic_val64_connection_id((*ph).dest_cnx_id);
        picoquic_queue_stateless_packet(quic as *mut picoquic_quic_t, sp);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_retry_packet(
    mut quic: *mut picoquic_quic_t,
    mut addr_from: *const sockaddr,
    mut addr_to: *const sockaddr,
    mut if_index_to: ::core::ffi::c_int,
    mut ph: *mut picoquic_packet_header,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut token_buffer: [uint8_t; 256] = [0; 256];
    let mut token_size: size_t = 0;
    let mut s_cid: picoquic_connection_id_t = st_picoquic_connection_id_t {
        id: [
            0 as ::core::ffi::c_int as uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        id_len: 0,
    };
    picoquic_create_local_cnx_id(
        quic,
        &raw mut s_cid,
        (*quic).local_cnxid_length,
        (*ph).dest_cnx_id,
    );
    if picoquic_prepare_retry_token(
        quic,
        addr_from,
        (current_time as ::core::ffi::c_ulonglong).wrapping_add(PICOQUIC_TOKEN_DELAY_SHORT)
            as uint64_t,
        &raw mut (*ph).dest_cnx_id,
        &raw mut s_cid,
        (*ph).pn,
        &raw mut token_buffer as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 256]>() as size_t,
        &raw mut token_size,
    ) != 0 as ::core::ffi::c_int
    {
        ret = PICOQUIC_ERROR_MEMORY;
    } else {
        picoquic_queue_stateless_retry(
            quic,
            ph,
            &raw mut s_cid,
            addr_from,
            addr_to,
            if_index_to as ::core::ffi::c_ulong,
            &raw mut token_buffer as *mut uint8_t,
            token_size,
        );
        ret = PICOQUIC_ERROR_RETRY;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_busy_packet(
    mut quic: *mut picoquic_quic_t,
    mut addr_from: *const sockaddr,
    mut addr_to: *const sockaddr,
    mut if_index_to: ::core::ffi::c_int,
    mut ph: *mut picoquic_packet_header,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut s_cid: picoquic_connection_id_t = st_picoquic_connection_id_t {
        id: [
            0 as ::core::ffi::c_int as uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        id_len: 0,
    };
    let mut sp: *mut picoquic_stateless_packet_t =
        picoquic_create_stateless_packet(quic as *mut picoquic_quic_t);
    let mut aead_ctx: *mut ::core::ffi::c_void = NULL;
    let mut pn_enc_ctx: *mut ::core::ffi::c_void = NULL;
    if !sp.is_null() {
        let mut bytes: *mut uint8_t = &raw mut (*sp).bytes as *mut uint8_t;
        let mut byte_index: size_t = 0 as size_t;
        let mut header_length: size_t = 0 as size_t;
        let mut pn_offset: size_t = 0;
        let mut pn_length: size_t = 0;
        let mut payload: [uint8_t; 4] = [
            picoquic_frame_type_connection_close as ::core::ffi::c_int as uint8_t,
            PICOQUIC_TRANSPORT_SERVER_BUSY as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ];
        let mut payload_length: size_t = 0 as size_t;
        picoquic_create_local_cnx_id(
            quic,
            &raw mut s_cid,
            (*quic).local_cnxid_length,
            (*ph).dest_cnx_id,
        );
        header_length = picoquic_create_long_header(
            picoquic_packet_initial,
            &raw mut (*ph).srce_cnx_id,
            &raw mut s_cid,
            0 as ::core::ffi::c_int,
            (*ph).vn,
            (*ph).version_index,
            0 as uint64_t,
            0 as size_t,
            ::core::ptr::null_mut::<uint8_t>(),
            bytes,
            &raw mut pn_offset,
            &raw mut pn_length,
        );
        byte_index = header_length;
        if picoquic_get_initial_aead_context(
            quic,
            (*ph).version_index,
            &raw mut (*ph).dest_cnx_id,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            &raw mut aead_ctx,
            &raw mut pn_enc_ctx,
        ) == 0 as ::core::ffi::c_int
        {
            picoquic_update_payload_length(
                bytes,
                pn_offset,
                header_length.wrapping_sub(pn_length),
                header_length
                    .wrapping_add(::core::mem::size_of::<[uint8_t; 4]>() as size_t)
                    .wrapping_add(picoquic_aead_get_checksum_length(aead_ctx)),
            );
            payload_length = picoquic_aead_encrypt_generic(
                bytes.offset(header_length as isize),
                &raw mut payload as *mut uint8_t,
                ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
                0 as uint64_t,
                bytes,
                header_length,
                aead_ctx,
            );
            picoquic_protect_packet_header(bytes, pn_offset, 0xf as uint8_t, pn_enc_ctx);
            (*sp).length = byte_index.wrapping_add(payload_length);
            (*sp).ptype = picoquic_packet_initial;
            picoquic_store_addr(&raw mut (*sp).addr_to, addr_from);
            picoquic_store_addr(&raw mut (*sp).addr_local, addr_to);
            (*sp).if_index_local = if_index_to;
            (*sp).cnxid_log64 = picoquic_val64_connection_id((*ph).dest_cnx_id);
            picoquic_queue_stateless_packet(quic as *mut picoquic_quic_t, sp);
        }
        if !aead_ctx.is_null() {
            picoquic_aead_free(aead_ctx);
        }
        if !pn_enc_ctx.is_null() {
            picoquic_cipher_free(pn_enc_ctx);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_queue_immediate_close(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
) {
    let mut sp: *mut picoquic_stateless_packet_t =
        picoquic_create_stateless_packet((*cnx).quic as *mut picoquic_quic_t);
    if !sp.is_null() {
        let mut ret: ::core::ffi::c_int = picoquic_prepare_packet_ex(
            cnx as *mut picoquic_cnx_t,
            -(1 as ::core::ffi::c_int),
            current_time,
            &raw mut (*sp).bytes as *mut uint8_t,
            PICOQUIC_MAX_PACKET_SIZE as size_t,
            &raw mut (*sp).length,
            &raw mut (*sp).addr_to,
            &raw mut (*sp).addr_local,
            &raw mut (*sp).if_index_local,
            ::core::ptr::null_mut::<size_t>(),
        );
        if ret == 0 as ::core::ffi::c_int && (*sp).length > 0 as size_t {
            picoquic_queue_stateless_packet((*cnx).quic as *mut picoquic_quic_t, sp);
        } else {
            picoquic_delete_stateless_packet(sp);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_ignore_incoming_handshake(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut ph: *mut picoquic_packet_header,
    mut current_time: uint64_t,
) {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut byte_index: size_t = 0 as size_t;
    let mut ack_needed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut pc: picoquic_packet_context_enum = picoquic_packet_context_application;
    if (*ph).ptype as ::core::ffi::c_uint
        == picoquic_packet_initial as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        pc = picoquic_packet_context_initial;
    } else if (*ph).ptype as ::core::ffi::c_uint
        == picoquic_packet_handshake as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        pc = picoquic_packet_context_handshake;
    } else {
        return;
    }
    bytes = bytes.offset((*ph).offset as isize);
    while ret == 0 as ::core::ffi::c_int && byte_index < (*ph).payload_length {
        let mut frame_length: size_t = 0 as size_t;
        let mut frame_is_pure_ack: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        ret = picoquic_skip_frame(
            bytes.offset(byte_index as isize) as *mut uint8_t,
            (*ph).payload_length.wrapping_sub(byte_index),
            &raw mut frame_length,
            &raw mut frame_is_pure_ack,
        );
        byte_index = byte_index.wrapping_add(frame_length);
        if frame_is_pure_ack == 0 as ::core::ffi::c_int {
            ack_needed = 1 as ::core::ffi::c_int;
        }
    }
    if ret == 0 as ::core::ffi::c_int && ack_needed != 0 {
        picoquic_set_ack_needed(
            cnx,
            current_time,
            pc,
            *(*cnx).path.offset(0 as ::core::ffi::c_int as isize),
            0 as ::core::ffi::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_incoming_client_initial(
    mut pcnx: *mut *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut packet_length: size_t,
    mut received_data: *mut picoquic_stream_data_node_t,
    mut addr_from: *mut sockaddr,
    mut addr_to: *mut sockaddr,
    mut if_index_to: ::core::ffi::c_ulong,
    mut ph: *mut picoquic_packet_header,
    mut current_time: uint64_t,
    mut new_context_created: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if ret == 0 as ::core::ffi::c_int {
        if (*(**(**pcnx).path.offset(0 as ::core::ffi::c_int as isize)).p_local_cnxid)
            .cnx_id
            .id_len as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
            && picoquic_compare_connection_id(
                &raw mut (*ph).dest_cnx_id,
                &raw mut (*(**(**pcnx).path.offset(0 as ::core::ffi::c_int as isize))
                    .p_local_cnxid)
                    .cnx_id,
            ) == 0 as ::core::ffi::c_int
        {
            (**pcnx).set_initial_validated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        if (**pcnx).initial_validated() == 0
            && !(**pcnx).pkt_ctx[picoquic_packet_context_initial as ::core::ffi::c_int as usize]
                .pending_first
                .is_null()
            && packet_length >= PICOQUIC_ENFORCED_INITIAL_MTU as size_t
        {
            (**pcnx).set_initial_repeat_needed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        if (**pcnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_server_init as ::core::ffi::c_int as ::core::ffi::c_uint
            && ((*(**pcnx).quic).server_busy() as ::core::ffi::c_int != 0
                || (*(**pcnx).quic).current_number_connections
                    > (*(**pcnx).quic).tentative_max_number_connections)
        {
            (**pcnx).local_error = PICOQUIC_TRANSPORT_SERVER_BUSY as uint64_t;
            (**pcnx).cnx_state = picoquic_state_handshake_failure;
        } else if (**pcnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_server_init as ::core::ffi::c_int as ::core::ffi::c_uint
            && ((**pcnx).initial_cnxid.id_len as ::core::ffi::c_int)
                < PICOQUIC_ENFORCED_INITIAL_CID_LENGTH
        {
            (**pcnx).local_error = PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t;
            (**pcnx).cnx_state = picoquic_state_handshake_failure;
        } else if ((**pcnx).cnx_state as ::core::ffi::c_uint)
            < picoquic_state_server_almost_ready as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if (**(**pcnx).path.offset(0 as ::core::ffi::c_int as isize))
                .local_addr
                .ss_family as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && !addr_to.is_null()
            {
                picoquic_store_addr(
                    &raw mut (**(**pcnx).path.offset(0 as ::core::ffi::c_int as isize)).local_addr,
                    addr_to,
                );
            }
            if (**(**pcnx).path.offset(0 as ::core::ffi::c_int as isize))
                .peer_addr
                .ss_family as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && !addr_from.is_null()
            {
                picoquic_store_addr(
                    &raw mut (**(**pcnx).path.offset(0 as ::core::ffi::c_int as isize)).peer_addr,
                    addr_from,
                );
            }
            (**(**pcnx).path.offset(0 as ::core::ffi::c_int as isize)).if_index_dest = if_index_to;
            if ret == 0 as ::core::ffi::c_int {
                let mut highest_ack_before: uint64_t = (**pcnx).pkt_ctx
                    [picoquic_packet_context_initial as ::core::ffi::c_int as usize]
                    .highest_acknowledged;
                ret = picoquic_decode_frames(
                    *pcnx,
                    *(**pcnx).path.offset(0 as ::core::ffi::c_int as isize),
                    bytes.offset((*ph).offset as isize),
                    (*ph).payload_length,
                    received_data,
                    (*ph).epoch as ::core::ffi::c_int,
                    addr_from,
                    addr_to,
                    (*ph).pn64,
                    0 as ::core::ffi::c_int,
                    current_time,
                );
                if (**pcnx).pkt_ctx[picoquic_packet_context_initial as ::core::ffi::c_int as usize]
                    .highest_acknowledged
                    > highest_ack_before
                    && (*(**pcnx).quic).random_initial() as ::core::ffi::c_int
                        > 1 as ::core::ffi::c_int
                {
                    (**pcnx).set_initial_validated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
            }
            if ret == 0 as ::core::ffi::c_int {
                let mut data_consumed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                ret = picoquic_tls_stream_process(*pcnx, &raw mut data_consumed, current_time);
                if data_consumed != 0 {
                    (**pcnx)
                        .set_initial_repeat_needed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                }
            }
        } else if ((**pcnx).cnx_state as ::core::ffi::c_uint)
            < picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            picoquic_ignore_incoming_handshake(*pcnx, bytes, ph, current_time);
        } else {
            ret = PICOQUIC_ERROR_UNEXPECTED_PACKET;
        }
    }
    if ret == PICOQUIC_ERROR_INVALID_TOKEN
        && (**pcnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_handshake_failure as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ret = 0 as ::core::ffi::c_int;
    }
    if ret == 0 as ::core::ffi::c_int
        && (**pcnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_handshake_failure as ::core::ffi::c_int as ::core::ffi::c_uint
        && new_context_created != 0
    {
        picoquic_queue_immediate_close(*pcnx, current_time);
    }
    if ret != 0 as ::core::ffi::c_int
        || (**pcnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_disconnected as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if new_context_created != 0 {
            picoquic_delete_cnx(*pcnx);
            *pcnx = ::core::ptr::null_mut::<picoquic_cnx_t>();
            ret = PICOQUIC_ERROR_CONNECTION_DELETED;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_incoming_retry(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut ph: *mut picoquic_packet_header,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut token_length: size_t = 0 as size_t;
    let mut token: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    if (*cnx).cnx_state as ::core::ffi::c_uint
        != picoquic_state_client_init_sent as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*cnx).cnx_state as ::core::ffi::c_uint
            != picoquic_state_client_init_resent as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cnx).original_cnxid.id_len as ::core::ffi::c_int != 0 as ::core::ffi::c_int
    {
        ret = PICOQUIC_ERROR_UNEXPECTED_PACKET;
    } else if (*ph).vn
        != (*(&raw const picoquic_supported_versions as *const picoquic_version_parameters_t)
            .offset((*cnx).version_index as isize))
        .version
    {
        ret = PICOQUIC_ERROR_UNEXPECTED_PACKET;
    } else if (*ph).pn64 != 0 as uint64_t {
        ret = PICOQUIC_ERROR_UNEXPECTED_PACKET;
    }
    if ret == 0 as ::core::ffi::c_int {
        let mut integrity_aead: *mut ::core::ffi::c_void = picoquic_find_retry_protection_context(
            (*cnx).quic,
            (*cnx).version_index,
            0 as ::core::ffi::c_int,
        );
        let mut byte_index: size_t = (*ph).offset;
        let mut data_length: size_t = (*ph).offset.wrapping_add((*ph).payload_length);
        if integrity_aead.is_null() {
            let c2rust_fresh0 = byte_index;
            byte_index = byte_index.wrapping_add(1);
            let mut odcil: uint8_t = *bytes.offset(c2rust_fresh0 as isize);
            if odcil as ::core::ffi::c_int != (*cnx).initial_cnxid.id_len as ::core::ffi::c_int
                || (odcil as size_t).wrapping_add(1 as size_t) > (*ph).payload_length
                || memcmp(
                    &raw mut (*cnx).initial_cnxid.id as *mut uint8_t as *const ::core::ffi::c_void,
                    bytes.offset(byte_index as isize) as *mut uint8_t as *const ::core::ffi::c_void,
                    odcil as size_t,
                ) != 0 as ::core::ffi::c_int
            {
                ret = PICOQUIC_ERROR_UNEXPECTED_PACKET;
                picoquic_log_app_message(
                    cnx as *mut picoquic_cnx_t,
                    b"Retry packet rejected: odcid check failed\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            } else {
                byte_index = byte_index.wrapping_add(odcil as size_t);
            }
        } else {
            ret = picoquic_verify_retry_protection(
                integrity_aead,
                bytes,
                &raw mut data_length,
                byte_index,
                &raw mut (*cnx).initial_cnxid,
            );
            if ret != 0 as ::core::ffi::c_int {
                picoquic_log_app_message(
                    cnx as *mut picoquic_cnx_t,
                    b"Retry packet rejected: integrity check failed, ret=0x%x\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ret,
                );
            }
        }
        if ret == 0 as ::core::ffi::c_int {
            token_length = data_length.wrapping_sub(byte_index);
            if token_length > 0 as size_t {
                token = malloc(token_length) as *mut uint8_t;
                if token.is_null() {
                    ret = PICOQUIC_ERROR_MEMORY;
                } else {
                    memcpy(
                        token as *mut ::core::ffi::c_void,
                        bytes.offset(byte_index as isize) as *mut uint8_t
                            as *const ::core::ffi::c_void,
                        token_length,
                    );
                }
            }
        }
    }
    if ret == 0 as ::core::ffi::c_int {
        picoquic_log_close_connection(cnx);
        if (*cnx).original_cnxid.id_len as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            (*cnx).original_cnxid = (*cnx).initial_cnxid;
        }
        (*cnx).initial_cnxid = (*ph).srce_cnx_id;
        if !(*cnx).retry_token.is_null() {
            free((*cnx).retry_token as *mut ::core::ffi::c_void);
        }
        (*cnx).retry_token = token;
        (*cnx).retry_token_length = token_length as uint16_t;
        picoquic_reset_cnx(cnx, current_time);
        ret = PICOQUIC_ERROR_RETRY;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_incoming_server_initial(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut packet_length: size_t,
    mut received_data: *mut picoquic_stream_data_node_t,
    mut addr_to: *mut sockaddr,
    mut if_index_to: ::core::ffi::c_ulong,
    mut ph: *mut picoquic_packet_header,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*cnx).cnx_state as ::core::ffi::c_uint
        == picoquic_state_client_init_sent as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_client_init_resent as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cnx).cnx_state = picoquic_state_client_handshake_start;
    }
    if (picoquic_is_connection_id_null(
        &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid).cnx_id,
    ) == 0
        || (*cnx).cnx_state as ::core::ffi::c_uint
            > picoquic_state_client_handshake_start as ::core::ffi::c_int as ::core::ffi::c_uint)
        && picoquic_compare_connection_id(
            &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid)
                .cnx_id,
            &raw mut (*ph).srce_cnx_id,
        ) != 0 as ::core::ffi::c_int
    {
        ret = PICOQUIC_ERROR_CNXID_CHECK;
    }
    if ret == 0 as ::core::ffi::c_int {
        if (*cnx).cnx_state as ::core::ffi::c_uint
            <= picoquic_state_client_handshake_start as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                .local_addr
                .ss_family as ::core::ffi::c_int
                == 0 as ::core::ffi::c_int
                && !addr_to.is_null()
            {
                picoquic_store_addr(
                    &raw mut (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).local_addr,
                    addr_to,
                );
            }
            (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).if_index_dest = if_index_to;
            if (*ph).payload_length == 0 as size_t {
                ret = picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                    0 as uint64_t,
                );
            } else {
                if packet_length < PICOQUIC_ENFORCED_INITIAL_MTU as size_t {
                    let mut byte_index: size_t = (*ph).offset;
                    let mut ack_needed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    let mut skip_ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while skip_ret == 0 as ::core::ffi::c_int && byte_index < (*ph).payload_length {
                        let mut frame_length: size_t = 0 as size_t;
                        let mut frame_is_pure_ack: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                        skip_ret = picoquic_skip_frame(
                            bytes.offset(byte_index as isize) as *mut uint8_t,
                            (*ph).payload_length.wrapping_sub(byte_index),
                            &raw mut frame_length,
                            &raw mut frame_is_pure_ack,
                        );
                        byte_index = byte_index.wrapping_add(frame_length);
                        if !(frame_is_pure_ack == 0 as ::core::ffi::c_int) {
                            continue;
                        }
                        ack_needed = 1 as ::core::ffi::c_int;
                        break;
                    }
                    if ack_needed != 0
                        && (*cnx).retry_token_length as ::core::ffi::c_int
                            == 0 as ::core::ffi::c_int
                        && (*cnx).crypto_context[1 as ::core::ffi::c_int as usize]
                            .aead_encrypt
                            .is_null()
                    {
                        picoquic_log_app_message(
                            cnx as *mut picoquic_cnx_t,
                            b"Server initial too short (%zu bytes)\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            packet_length,
                        );
                        ret = PICOQUIC_ERROR_INITIAL_TOO_SHORT;
                    }
                }
                if ret == 0 as ::core::ffi::c_int {
                    ret = picoquic_decode_frames(
                        cnx,
                        *(*cnx).path.offset(0 as ::core::ffi::c_int as isize),
                        bytes.offset((*ph).offset as isize),
                        (*ph).payload_length,
                        received_data,
                        (*ph).epoch as ::core::ffi::c_int,
                        ::core::ptr::null_mut::<sockaddr>(),
                        addr_to,
                        (*ph).pn64,
                        0 as ::core::ffi::c_int,
                        current_time,
                    );
                }
            }
            if ret == 0 as ::core::ffi::c_int {
                ret = picoquic_tls_stream_process(
                    cnx,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    current_time,
                );
            }
        } else if ((*cnx).cnx_state as ::core::ffi::c_uint)
            < picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            picoquic_ignore_incoming_handshake(cnx, bytes, ph, current_time);
        } else {
            ret = PICOQUIC_ERROR_UNEXPECTED_PACKET;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_incoming_server_handshake(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut received_data: *mut picoquic_stream_data_node_t,
    mut addr_to: *mut sockaddr,
    mut if_index_to: ::core::ffi::c_ulong,
    mut ph: *mut picoquic_packet_header,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut restricted: ::core::ffi::c_int = ((*cnx).cnx_state as ::core::ffi::c_uint
        != picoquic_state_client_handshake_start as ::core::ffi::c_int as ::core::ffi::c_uint)
        as ::core::ffi::c_int;
    if picoquic_compare_connection_id(
        &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid).cnx_id,
        &raw mut (*ph).srce_cnx_id,
    ) != 0 as ::core::ffi::c_int
    {
        ret = PICOQUIC_ERROR_CNXID_CHECK;
    }
    if ret == 0 as ::core::ffi::c_int {
        if ((*cnx).cnx_state as ::core::ffi::c_uint)
            < picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if (*ph).payload_length == 0 as size_t {
                ret = picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                    0 as uint64_t,
                );
            } else {
                ret = picoquic_decode_frames(
                    cnx,
                    *(*cnx).path.offset(0 as ::core::ffi::c_int as isize),
                    bytes.offset((*ph).offset as isize),
                    (*ph).payload_length,
                    received_data,
                    (*ph).epoch as ::core::ffi::c_int,
                    ::core::ptr::null_mut::<sockaddr>(),
                    addr_to,
                    (*ph).pn64,
                    0 as ::core::ffi::c_int,
                    current_time,
                );
            }
            if ret == 0 as ::core::ffi::c_int && restricted == 0 as ::core::ffi::c_int {
                ret = picoquic_tls_stream_process(
                    cnx,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    current_time,
                );
            }
        } else {
            ret = PICOQUIC_ERROR_UNEXPECTED_PACKET;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_incoming_client_handshake(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut received_data: *mut picoquic_stream_data_node_t,
    mut ph: *mut picoquic_packet_header,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*cnx).set_initial_validated(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*cnx).set_initial_repeat_needed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    if ((*cnx).cnx_state as ::core::ffi::c_uint)
        < picoquic_state_server_almost_ready as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if picoquic_compare_connection_id(
            &raw mut (*ph).srce_cnx_id,
            &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid)
                .cnx_id,
        ) != 0 as ::core::ffi::c_int
        {
            ret = PICOQUIC_ERROR_CNXID_CHECK;
        } else {
            if (*ph).payload_length == 0 as size_t {
                ret = picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                    0 as uint64_t,
                );
            } else {
                ret = picoquic_decode_frames(
                    cnx,
                    *(*cnx).path.offset(0 as ::core::ffi::c_int as isize),
                    bytes.offset((*ph).offset as isize),
                    (*ph).payload_length,
                    received_data,
                    (*ph).epoch as ::core::ffi::c_int,
                    ::core::ptr::null_mut::<sockaddr>(),
                    ::core::ptr::null_mut::<sockaddr>(),
                    (*ph).pn64,
                    0 as ::core::ffi::c_int,
                    current_time,
                );
            }
            if ret == 0 as ::core::ffi::c_int {
                picoquic_implicit_handshake_ack(cnx, picoquic_packet_context_initial, current_time);
                picoquic_crypto_context_free(
                    (&raw mut (*cnx).crypto_context as *mut picoquic_crypto_context_t)
                        .offset(picoquic_epoch_initial as ::core::ffi::c_int as isize)
                        as *mut picoquic_crypto_context_t,
                );
                ret = picoquic_tls_stream_process(
                    cnx,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    current_time,
                );
                if (*cnx).client_mode() == 0
                    && ((*cnx).cnx_state as ::core::ffi::c_uint)
                        < picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
                    && picoquic_is_tls_complete(cnx) != 0
                {
                    picoquic_ready_state_transition(cnx, current_time);
                }
            }
        }
    } else if (*cnx).cnx_state as ::core::ffi::c_uint
        <= picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        picoquic_ignore_incoming_handshake(cnx, bytes, ph, current_time);
    } else {
        ret = PICOQUIC_ERROR_UNEXPECTED_PACKET;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_incoming_stateless_reset(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    if (*cnx).cnx_state as ::core::ffi::c_uint
        <= picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cnx).remote_error = PICOQUIC_ERROR_STATELESS_RESET as uint64_t;
    }
    if (*cnx).callback_fn.is_some() {
        (*cnx).callback_fn.expect("non-null function pointer")(
            cnx as *mut picoquic_cnx_t,
            0 as uint64_t,
            ::core::ptr::null_mut::<uint8_t>(),
            0 as size_t,
            picoquic_callback_stateless_reset,
            (*cnx).callback_ctx,
            NULL,
        );
    }
    picoquic_connection_disconnect(cnx);
    return PICOQUIC_ERROR_AEAD_CHECK;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_incoming_0rtt(
    mut cnx: *mut picoquic_cnx_t,
    mut bytes: *mut uint8_t,
    mut received_data: *mut picoquic_stream_data_node_t,
    mut ph: *mut picoquic_packet_header,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(picoquic_compare_connection_id(&raw mut (*ph).dest_cnx_id, &raw mut (*cnx).initial_cnxid)
        == 0 as ::core::ffi::c_int
        || picoquic_compare_connection_id(
            &raw mut (*ph).dest_cnx_id,
            &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_local_cnxid)
                .cnx_id,
        ) == 0 as ::core::ffi::c_int)
        || picoquic_compare_connection_id(
            &raw mut (*ph).srce_cnx_id,
            &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_remote_cnxid)
                .cnx_id,
        ) != 0 as ::core::ffi::c_int
    {
        ret = PICOQUIC_ERROR_CNXID_CHECK;
    } else if (*cnx).cnx_state as ::core::ffi::c_uint
        == picoquic_state_server_almost_ready as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_server_false_start as ::core::ffi::c_int as ::core::ffi::c_uint
        || (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
            && (*cnx).is_1rtt_received() == 0
    {
        if (*ph).vn
            != (*(&raw const picoquic_supported_versions as *const picoquic_version_parameters_t)
                .offset((*cnx).version_index as isize))
            .version
        {
            ret = picoquic_connection_error(
                cnx,
                PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                0 as uint64_t,
            );
        } else {
            if (*ph).payload_length == 0 as size_t {
                ret = picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                    0 as uint64_t,
                );
            } else {
                (*cnx).nb_zero_rtt_received = (*cnx).nb_zero_rtt_received.wrapping_add(1);
                ret = picoquic_decode_frames(
                    cnx,
                    *(*cnx).path.offset(0 as ::core::ffi::c_int as isize),
                    bytes.offset((*ph).offset as isize),
                    (*ph).payload_length,
                    received_data,
                    (*ph).epoch as ::core::ffi::c_int,
                    ::core::ptr::null_mut::<sockaddr>(),
                    ::core::ptr::null_mut::<sockaddr>(),
                    (*ph).pn64,
                    0 as ::core::ffi::c_int,
                    current_time,
                );
            }
            if ret == 0 as ::core::ffi::c_int {
                ret = picoquic_tls_stream_process(
                    cnx,
                    ::core::ptr::null_mut::<::core::ffi::c_int>(),
                    current_time,
                );
            }
        }
    } else {
        ret = PICOQUIC_ERROR_UNEXPECTED_PACKET;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_find_incoming_unique_path(
    mut cnx: *mut picoquic_cnx_t,
    mut ph: *mut picoquic_packet_header,
    mut addr_from: *mut sockaddr,
    mut addr_to: *mut sockaddr,
    mut if_index_to: ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut p_path_id: *mut ::core::ffi::c_int,
    mut path_is_not_allocated: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut path_x: *mut picoquic_path_t = ::core::ptr::null_mut::<picoquic_path_t>();
    let mut path_id: ::core::ffi::c_int =
        picoquic_find_path_by_unique_id(cnx, (*(*ph).l_cid).path_id);
    if path_id < 0 as ::core::ffi::c_int {
        if (*cnx).nb_paths < PICOQUIC_NB_PATH_TARGET
            && ((*(*cnx).quic).is_port_blocking_disabled() as ::core::ffi::c_int != 0
                || picoquic_check_addr_blocked(addr_from) == 0)
            && picoquic_create_path(
                cnx,
                current_time,
                addr_to,
                addr_from,
                (*(*ph).l_cid).path_id,
            ) > 0 as ::core::ffi::c_int
        {
            path_id = (*cnx).nb_paths - 1 as ::core::ffi::c_int;
            path_x = *(*cnx).path.offset(path_id as isize);
            (*path_x).if_index_dest = if_index_to as ::core::ffi::c_ulong;
            (*path_x).p_local_cnxid = picoquic_find_local_cnxid(
                cnx,
                (*path_x).unique_path_id,
                &raw mut (*ph).dest_cnx_id,
            );
            picoquic_assign_peer_cnxid_to_path(cnx, path_id);
        }
    } else {
        path_x = *(*cnx).path.offset(path_id as isize);
        if (*path_x).p_local_cnxid.is_null() {
            (*path_x).p_local_cnxid = picoquic_find_local_cnxid(
                cnx,
                (*path_x).unique_path_id,
                &raw mut (*ph).dest_cnx_id,
            );
            if (*cnx).client_mode() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
            {
                picoquic_renew_connection_id(cnx, path_id);
            }
        } else if picoquic_compare_connection_id(
            &raw mut (*(*path_x).p_local_cnxid).cnx_id,
            &raw mut (*ph).dest_cnx_id,
        ) != 0 as ::core::ffi::c_int
        {
            (*path_x).p_local_cnxid = picoquic_find_local_cnxid(
                cnx,
                (*path_x).unique_path_id,
                &raw mut (*ph).dest_cnx_id,
            );
            if (*cnx).client_mode() as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                picoquic_renew_connection_id(cnx, path_id);
            }
        }
        if picoquic_compare_addr(addr_from, &raw mut (*path_x).peer_addr as *mut sockaddr)
            == 0 as ::core::ffi::c_int
        {
            if (*path_x).local_addr.ss_family as ::core::ffi::c_int == AF_UNSPEC {
                picoquic_store_addr(
                    &raw mut (**(*cnx).path.offset(path_id as isize)).local_addr,
                    addr_to,
                );
            }
        }
    }
    if path_id < 0 as ::core::ffi::c_int {
        path_id = 0 as ::core::ffi::c_int;
    }
    *p_path_id = path_id;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_find_incoming_path(
    mut cnx: *mut picoquic_cnx_t,
    mut ph: *mut picoquic_packet_header,
    mut addr_from: *mut sockaddr,
    mut addr_to: *mut sockaddr,
    mut if_index_to: ::core::ffi::c_int,
    mut current_time: uint64_t,
    mut p_path_id: *mut ::core::ffi::c_int,
    mut path_is_not_allocated: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    if (*ph).ptype as ::core::ffi::c_uint
        == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
    {
        return picoquic_find_incoming_unique_path(
            cnx,
            ph,
            addr_from,
            addr_to,
            if_index_to,
            current_time,
            p_path_id,
            path_is_not_allocated,
        );
    }
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut partial_match_path: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut nat_rebinding_path: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    let mut nat_rebinding_total: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut path_id: ::core::ffi::c_int =
        picoquic_find_path_by_address(cnx, addr_to, addr_from, &raw mut partial_match_path);
    *path_is_not_allocated = 0 as ::core::ffi::c_int;
    if path_id < 0 as ::core::ffi::c_int && partial_match_path >= 0 as ::core::ffi::c_int {
        path_id = partial_match_path;
        picoquic_store_addr(
            &raw mut (**(*cnx).path.offset(path_id as isize)).local_addr,
            addr_to,
        );
    }
    if path_id >= 0 as ::core::ffi::c_int {
        if (**(*cnx).path.offset(path_id as isize))
            .p_local_cnxid
            .is_null()
        {
            let ref mut c2rust_fresh7 = (**(*cnx).path.offset(path_id as isize)).p_local_cnxid;
            *c2rust_fresh7 =
                picoquic_find_local_cnxid(cnx, 0 as uint64_t, &raw mut (*ph).dest_cnx_id);
            if (**(*cnx).path.offset(path_id as isize)).was_local_cnxid_retired() != 0 {
                if (*cnx).client_mode() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                    && path_id == 0 as ::core::ffi::c_int
                {
                    picoquic_renew_connection_id(cnx, path_id);
                }
                let ref mut c2rust_fresh8 = **(*cnx).path.offset(path_id as isize);
                (*c2rust_fresh8)
                    .set_was_local_cnxid_retired(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        } else if picoquic_compare_connection_id(
            &raw mut (*(**(*cnx).path.offset(path_id as isize)).p_local_cnxid).cnx_id,
            &raw mut (*ph).dest_cnx_id,
        ) != 0 as ::core::ffi::c_int
        {
            let ref mut c2rust_fresh9 = (**(*cnx).path.offset(path_id as isize)).p_local_cnxid;
            *c2rust_fresh9 =
                picoquic_find_local_cnxid(cnx, 0 as uint64_t, &raw mut (*ph).dest_cnx_id);
            if (*cnx).client_mode() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                && !(*(*cnx).first_remote_cnxid_stash)
                    .cnxid_stash_first
                    .is_null()
                && path_id == 0 as ::core::ffi::c_int
            {
                picoquic_renew_connection_id(cnx, path_id);
                let ref mut c2rust_fresh10 = **(*cnx).path.offset(path_id as isize);
                (*c2rust_fresh10)
                    .set_was_local_cnxid_retired(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        }
    } else {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*cnx).nb_paths {
            if !(**(*cnx).path.offset(i as isize)).p_local_cnxid.is_null()
                && picoquic_compare_connection_id(
                    &raw mut (*(**(*cnx).path.offset(i as isize)).p_local_cnxid).cnx_id,
                    &raw mut (*ph).dest_cnx_id,
                ) == 0 as ::core::ffi::c_int
            {
                if nat_rebinding_total == 0 as ::core::ffi::c_int {
                    nat_rebinding_path = i;
                }
                nat_rebinding_total += 1;
                break;
            } else {
                i += 1;
            }
        }
        if (*cnx).nb_paths < PICOQUIC_NB_PATH_TARGET
            && ((*(*cnx).quic).is_port_blocking_disabled() as ::core::ffi::c_int != 0
                || picoquic_check_addr_blocked(addr_from) == 0)
            && picoquic_create_path(
                cnx,
                current_time,
                addr_to,
                addr_from,
                UINT64_MAX as uint64_t,
            ) > 0 as ::core::ffi::c_int
        {
            path_id = (*cnx).nb_paths - 1 as ::core::ffi::c_int;
            if (*cnx).client_mode() == 0 && (*cnx).local_parameters.prefered_address.is_defined != 0
            {
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
                if (*addr_to).sa_family as ::core::ffi::c_int == AF_INET {
                    let mut d4: *mut sockaddr_in = &raw mut dest_addr as *mut sockaddr_in;
                    (*d4).sin_family = AF_INET as sa_family_t;
                    (*d4).sin_port =
                        __bswap_16((*cnx).local_parameters.prefered_address.ipv4Port as __uint16_t)
                            as in_port_t;
                    memcpy(
                        &raw mut (*d4).sin_addr as *mut ::core::ffi::c_void,
                        &raw mut (*cnx).local_parameters.prefered_address.ipv4Address
                            as *mut uint8_t as *const ::core::ffi::c_void,
                        4 as size_t,
                    );
                } else if (*addr_to).sa_family as ::core::ffi::c_int == AF_INET6 {
                    let mut d6: *mut sockaddr_in6 = &raw mut dest_addr as *mut sockaddr_in6;
                    (*d6).sin6_family = AF_INET6 as sa_family_t;
                    (*d6).sin6_port =
                        __bswap_16((*cnx).local_parameters.prefered_address.ipv6Port as __uint16_t)
                            as in_port_t;
                    memcpy(
                        &raw mut (*d6).sin6_addr as *mut ::core::ffi::c_void,
                        &raw mut (*cnx).local_parameters.prefered_address.ipv6Address
                            as *mut uint8_t as *const ::core::ffi::c_void,
                        16 as size_t,
                    );
                }
                if picoquic_compare_addr(addr_to, &raw mut dest_addr as *mut sockaddr)
                    == 0 as ::core::ffi::c_int
                {
                    let ref mut c2rust_fresh11 = **(*cnx).path.offset(path_id as isize);
                    (*c2rust_fresh11).set_path_is_preferred_path(
                        1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                    );
                }
            }
            if picoquic_assign_peer_cnxid_to_path(cnx, path_id) != 0 as ::core::ffi::c_int {
                let mut alt_path: ::core::ffi::c_int =
                    if nat_rebinding_path >= 0 as ::core::ffi::c_int {
                        nat_rebinding_path
                    } else {
                        0 as ::core::ffi::c_int
                    };
                if (**(*cnx).path.offset(path_id as isize))
                    .p_remote_cnxid
                    .is_null()
                {
                    let ref mut c2rust_fresh12 =
                        (**(*cnx).path.offset(path_id as isize)).p_remote_cnxid;
                    *c2rust_fresh12 = (**(*cnx).path.offset(alt_path as isize)).p_remote_cnxid;
                    if !(**(*cnx).path.offset(path_id as isize))
                        .p_remote_cnxid
                        .is_null()
                    {
                        let ref mut c2rust_fresh13 = (*(**(*cnx).path.offset(path_id as isize))
                            .p_remote_cnxid)
                            .nb_path_references;
                        *c2rust_fresh13 += 1;
                    }
                } else if (*(**(*cnx).path.offset(path_id as isize)).p_remote_cnxid).sequence
                    != (*(**(*cnx).path.offset(alt_path as isize)).p_remote_cnxid).sequence
                {
                    picoquic_dereference_stashed_cnxid(
                        cnx,
                        *(*cnx).path.offset(path_id as isize),
                        0 as ::core::ffi::c_int,
                    );
                    let ref mut c2rust_fresh14 =
                        (**(*cnx).path.offset(path_id as isize)).p_remote_cnxid;
                    *c2rust_fresh14 = (**(*cnx).path.offset(alt_path as isize)).p_remote_cnxid;
                    let ref mut c2rust_fresh15 = (*(**(*cnx).path.offset(path_id as isize))
                        .p_remote_cnxid)
                        .nb_path_references;
                    *c2rust_fresh15 += 1;
                }
            }
            let ref mut c2rust_fresh16 = **(*cnx).path.offset(path_id as isize);
            (*c2rust_fresh16)
                .set_path_is_published(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            let ref mut c2rust_fresh17 = (**(*cnx).path.offset(path_id as isize)).p_local_cnxid;
            *c2rust_fresh17 =
                picoquic_find_local_cnxid(cnx, 0 as uint64_t, &raw mut (*ph).dest_cnx_id);
            picoquic_register_path(cnx, *(*cnx).path.offset(path_id as isize));
            picoquic_set_path_challenge(cnx, path_id, current_time);
            if nat_rebinding_path >= 0 as ::core::ffi::c_int {
                picoquic_set_path_challenge(cnx, nat_rebinding_path, current_time);
                let ref mut c2rust_fresh18 = **(*cnx).path.offset(path_id as isize);
                (*c2rust_fresh18)
                    .set_is_nat_challenge(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                let ref mut c2rust_fresh19 = **(*cnx).path.offset(0 as ::core::ffi::c_int as isize);
                (*c2rust_fresh19)
                    .set_is_nat_challenge(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            } else {
                let ref mut c2rust_fresh20 = **(*cnx).path.offset(path_id as isize);
                (*c2rust_fresh20)
                    .set_is_nat_challenge(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            }
        } else {
            *path_is_not_allocated = 1 as ::core::ffi::c_int;
            if nat_rebinding_path >= 0 as ::core::ffi::c_int {
                path_id = nat_rebinding_path;
            } else {
                path_id = 0 as ::core::ffi::c_int;
            }
        }
    }
    *p_path_id = path_id;
    (**(*cnx).path.offset(path_id as isize)).last_packet_received_at = current_time;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_ecn_accounting(
    mut cnx: *mut picoquic_cnx_t,
    mut received_ecn: ::core::ffi::c_uchar,
    mut pc: picoquic_packet_context_enum,
    mut l_cid: *mut picoquic_local_cnxid_t,
) {
    let mut ack_ctx: *mut picoquic_ack_context_t =
        (&raw mut (*cnx).ack_ctx as *mut picoquic_ack_context_t).offset(pc as isize)
            as *mut picoquic_ack_context_t;
    if pc as ::core::ffi::c_uint
        == picoquic_packet_context_application as ::core::ffi::c_int as ::core::ffi::c_uint
        && (*cnx).is_multipath_enabled() as ::core::ffi::c_int != 0
    {
        ack_ctx = picoquic_ack_ctx_from_cnx_context(cnx, pc, l_cid);
    }
    match received_ecn as ::core::ffi::c_int & 0x3 as ::core::ffi::c_int {
        1 => {
            (*ack_ctx).ecn_ect1_total_local = (*ack_ctx).ecn_ect1_total_local.wrapping_add(1);
            (*ack_ctx).set_sending_ecn_ack(
                (*ack_ctx).sending_ecn_ack() | 1 as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
        2 => {
            (*ack_ctx).ecn_ect0_total_local = (*ack_ctx).ecn_ect0_total_local.wrapping_add(1);
            (*ack_ctx).set_sending_ecn_ack(
                (*ack_ctx).sending_ecn_ack() | 1 as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
        3 => {
            (*ack_ctx).ecn_ce_total_local = (*ack_ctx).ecn_ce_total_local.wrapping_add(1);
            (*ack_ctx).set_sending_ecn_ack(
                (*ack_ctx).sending_ecn_ack() | 1 as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
        }
        0 | _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_incoming_1rtt(
    mut cnx: *mut picoquic_cnx_t,
    mut path_id: ::core::ffi::c_int,
    mut bytes: *mut uint8_t,
    mut received_data: *mut picoquic_stream_data_node_t,
    mut ph: *mut picoquic_packet_header,
    mut addr_from: *mut sockaddr,
    mut addr_to: *mut sockaddr,
    mut if_index_to: ::core::ffi::c_int,
    mut received_ecn: ::core::ffi::c_uchar,
    mut path_is_not_allocated: ::core::ffi::c_int,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if ((*cnx).cnx_state as ::core::ffi::c_uint)
        < picoquic_state_client_almost_ready as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ret = PICOQUIC_ERROR_UNEXPECTED_PACKET;
    } else if (*cnx).cnx_state as ::core::ffi::c_uint
        == picoquic_state_disconnected as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        ret = PICOQUIC_ERROR_UNEXPECTED_PACKET;
    } else if (*cnx).cnx_state as ::core::ffi::c_uint
        >= picoquic_state_disconnecting as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_closing as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*cnx).cnx_state as ::core::ffi::c_uint
                == picoquic_state_disconnecting as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            let mut closing_received: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            ret = picoquic_decode_closing_frames(
                cnx,
                bytes.offset((*ph).offset as isize),
                (*ph).payload_length,
                &raw mut closing_received,
            );
            if ret == 0 as ::core::ffi::c_int {
                if closing_received != 0 {
                    if (*cnx).client_mode() != 0 {
                        picoquic_connection_disconnect(cnx);
                    } else {
                        (*cnx).cnx_state = picoquic_state_draining;
                    }
                } else {
                    picoquic_set_ack_needed(
                        cnx,
                        current_time,
                        (*ph).pc,
                        *(*cnx).path.offset(path_id as isize),
                        0 as ::core::ffi::c_int,
                    );
                }
            }
        } else {
            ret = PICOQUIC_ERROR_UNEXPECTED_PACKET;
        }
    } else if ret == 0 as ::core::ffi::c_int {
        let mut path_x: *mut picoquic_path_t = *(*cnx).path.offset(path_id as isize);
        (*path_x).if_index_dest = if_index_to as ::core::ffi::c_ulong;
        (*cnx).set_is_1rtt_received(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        (*(&raw mut picoquic_spin_function_table as *mut picoquic_spinbit_def_t)
            .offset((*cnx).spin_policy as isize))
        .spinbit_incoming
        .expect("non-null function pointer")(
            cnx as *mut picoquic_cnx_t,
            path_x as *mut picoquic_path_t,
            ph,
        );
        ret = picoquic_decode_frames(
            cnx,
            *(*cnx).path.offset(path_id as isize),
            bytes.offset((*ph).offset as isize),
            (*ph).payload_length,
            received_data,
            (*ph).epoch as ::core::ffi::c_int,
            addr_from,
            addr_to,
            (*ph).pn64,
            path_is_not_allocated,
            current_time,
        );
        if ret == 0 as ::core::ffi::c_int {
            (*path_x).received =
                ((*path_x).received as ::core::ffi::c_ulong).wrapping_add(
                    ((*ph).offset as uint64_t)
                        .wrapping_add((*ph).payload_length as uint64_t)
                        .wrapping_add(
                            picoquic_get_checksum_length(cnx, picoquic_epoch_1rtt) as uint64_t
                        ) as ::core::ffi::c_ulong,
                ) as uint64_t as uint64_t;
            if (*path_x).receive_rate_epoch == 0 as uint64_t {
                (*path_x).received_prior = (**(*cnx).path.offset(path_id as isize)).received;
                (*path_x).receive_rate_epoch = current_time;
            } else {
                let mut delta: uint64_t = current_time
                    .wrapping_sub((**(*cnx).path.offset(path_id as isize)).receive_rate_epoch);
                if delta > (*path_x).smoothed_rtt
                    && delta > PICOQUIC_BANDWIDTH_TIME_INTERVAL_MIN as uint64_t
                {
                    (*path_x).receive_rate_estimate = (**(*cnx).path.offset(path_id as isize))
                        .received
                        .wrapping_sub((**(*cnx).path.offset(path_id as isize)).received_prior)
                        .wrapping_mul(1000000 as uint64_t)
                        .wrapping_div(delta);
                    (*path_x).received_prior = (**(*cnx).path.offset(path_id as isize)).received;
                    (*path_x).receive_rate_epoch = current_time;
                    if (*path_x).receive_rate_estimate
                        > (**(*cnx).path.offset(path_id as isize)).receive_rate_max
                    {
                        (*path_x).receive_rate_max =
                            (**(*cnx).path.offset(path_id as isize)).receive_rate_estimate;
                        if path_id == 0 as ::core::ffi::c_int
                            && (*cnx).is_ack_frequency_negotiated() == 0
                        {
                            picoquic_compute_ack_gap_and_delay(
                                cnx,
                                (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).rtt_min,
                                PICOQUIC_ACK_DELAY_MIN as uint64_t,
                                (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                                    .receive_rate_max,
                                &raw mut (*cnx).ack_gap_remote,
                                &raw mut (*cnx).ack_delay_remote,
                            );
                        }
                    }
                }
            }
            ret = picoquic_tls_stream_process(
                cnx,
                ::core::ptr::null_mut::<::core::ffi::c_int>(),
                current_time,
            );
        }
        if ret == 0 as ::core::ffi::c_int
            && picoquic_cnx_is_still_logging(cnx as *mut picoquic_cnx_t) != 0
        {
            picoquic_log_cc_dump(cnx, current_time);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_incoming_not_decrypted(
    mut cnx: *mut picoquic_cnx_t,
    mut ph: *mut picoquic_packet_header,
    mut current_time: uint64_t,
    mut bytes: *mut uint8_t,
    mut length: size_t,
    mut addr_from: *mut sockaddr,
    mut addr_to: *mut sockaddr,
    mut if_index_to: ::core::ffi::c_int,
    mut received_ecn: ::core::ffi::c_uchar,
) -> ::core::ffi::c_int {
    let mut buffered: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if ((*cnx).cnx_state as ::core::ffi::c_uint)
        < picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        if (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_local_cnxid)
            .cnx_id
            .id_len as ::core::ffi::c_int
            > 0 as ::core::ffi::c_int
            && picoquic_compare_connection_id(
                &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).p_local_cnxid)
                    .cnx_id,
                &raw mut (*ph).dest_cnx_id,
            ) == 0 as ::core::ffi::c_int
        {
            picoquic_update_path_rtt(
                cnx,
                *(*cnx).path.offset(0 as ::core::ffi::c_int as isize),
                *(*cnx).path.offset(0 as ::core::ffi::c_int as isize),
                -(1 as ::core::ffi::c_int),
                (*cnx).start_time,
                current_time,
                0 as uint64_t,
                0 as uint64_t,
            );
            if length <= PICOQUIC_MAX_PACKET_SIZE as size_t
                && ((*ph).ptype as ::core::ffi::c_uint
                    == picoquic_packet_handshake as ::core::ffi::c_int as ::core::ffi::c_uint
                    && (*cnx).client_mode() as ::core::ffi::c_int != 0
                    || (*ph).ptype as ::core::ffi::c_uint
                        == picoquic_packet_1rtt_protected as ::core::ffi::c_int
                            as ::core::ffi::c_uint)
            {
                let mut packet: *mut picoquic_stateless_packet_t =
                    picoquic_create_stateless_packet((*cnx).quic as *mut picoquic_quic_t);
                if !packet.is_null() {
                    (*packet).length = length;
                    (*packet).ptype = (*ph).ptype;
                    memcpy(
                        &raw mut (*packet).bytes as *mut uint8_t as *mut ::core::ffi::c_void,
                        bytes as *const ::core::ffi::c_void,
                        length,
                    );
                    (*packet).next_packet =
                        (*cnx).first_sooner as *mut st_picoquic_stateless_packet_t;
                    (*cnx).first_sooner = packet;
                    picoquic_store_addr(&raw mut (*packet).addr_local, addr_to);
                    picoquic_store_addr(&raw mut (*packet).addr_to, addr_from);
                    (*packet).if_index_local = if_index_to;
                    (*packet).received_ecn = received_ecn;
                    (*packet).receive_time = current_time;
                    buffered = 1 as ::core::ffi::c_int;
                }
            }
        }
    }
    return buffered;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_incoming_segment(
    mut quic: *mut picoquic_quic_t,
    mut raw_bytes: *mut uint8_t,
    mut length: size_t,
    mut packet_length: size_t,
    mut consumed: *mut size_t,
    mut addr_from: *mut sockaddr,
    mut addr_to: *mut sockaddr,
    mut if_index_to: ::core::ffi::c_int,
    mut received_ecn: ::core::ffi::c_uchar,
    mut current_time: uint64_t,
    mut receive_time: uint64_t,
    mut previous_dest_id: *mut picoquic_connection_id_t,
    mut first_cnx: *mut *mut picoquic_cnx_t,
    mut path_id: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cnx: *mut picoquic_cnx_t = ::core::ptr::null_mut::<picoquic_cnx_t>();
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
    let mut new_context_created: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_first_segment: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut is_buffered: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut path_is_not_allocated: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bytes: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut decrypted_data: *mut picoquic_stream_data_node_t =
        picoquic_stream_data_node_alloc(quic);
    if decrypted_data.is_null() {
        return -(1 as ::core::ffi::c_int);
    }
    ret = picoquic_parse_header_and_decrypt(
        quic,
        raw_bytes,
        length,
        packet_length,
        addr_from,
        current_time,
        decrypted_data,
        &raw mut ph,
        &raw mut cnx,
        consumed,
        &raw mut new_context_created,
    );
    bytes = &raw mut (*decrypted_data).data as *mut uint8_t;
    if picoquic_is_connection_id_null(previous_dest_id) != 0 {
        *previous_dest_id = ph.dest_cnx_id;
        is_first_segment = 1 as ::core::ffi::c_int;
        *first_cnx = cnx;
        if !cnx.is_null() {
            picoquic_log_pdu(
                cnx,
                1 as ::core::ffi::c_int,
                current_time,
                addr_from,
                addr_to,
                packet_length,
            );
        } else {
            picoquic_log_quic_pdu(
                quic,
                1 as ::core::ffi::c_int,
                current_time,
                picoquic_val64_connection_id(ph.dest_cnx_id),
                addr_from,
                addr_to,
                packet_length,
            );
        }
    } else {
        if ret == 0 as ::core::ffi::c_int
            && picoquic_compare_connection_id(previous_dest_id, &raw mut ph.dest_cnx_id)
                != 0 as ::core::ffi::c_int
        {
            ret = PICOQUIC_ERROR_CNXID_SEGMENT;
        } else if ret == PICOQUIC_ERROR_VERSION_NOT_SUPPORTED {
            ret = PICOQUIC_ERROR_CNXID_SEGMENT;
        }
        if ret == PICOQUIC_ERROR_CNXID_SEGMENT && *first_cnx != cnx && !(*first_cnx).is_null() {
            picoquic_log_dropped_packet(
                *first_cnx,
                ::core::ptr::null_mut::<picoquic_path_t>(),
                &raw mut ph,
                length,
                ret,
                bytes,
                current_time,
            );
        }
    }
    if ret == PICOQUIC_ERROR_AEAD_NOT_READY && !cnx.is_null() {
        is_buffered = picoquic_incoming_not_decrypted(
            cnx,
            &raw mut ph,
            current_time,
            raw_bytes,
            length,
            addr_from,
            addr_to,
            if_index_to,
            received_ecn,
        );
    }
    if !cnx.is_null() {
        if ret == 0 as ::core::ffi::c_int
            && ph.ptype as ::core::ffi::c_uint
                == picoquic_packet_1rtt_protected as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            if ph.payload_length == 0 as size_t {
                ret = picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                    0 as uint64_t,
                );
            } else if ph.has_reserved_bit_set() != 0 {
                ret = picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                    0 as uint64_t,
                );
            } else {
                ret = picoquic_find_incoming_path(
                    cnx,
                    &raw mut ph,
                    addr_from,
                    addr_to,
                    if_index_to,
                    current_time,
                    path_id,
                    &raw mut path_is_not_allocated,
                );
            }
        }
        if ret == 0 as ::core::ffi::c_int {
            picoquic_log_packet(
                cnx,
                if *path_id < 0 as ::core::ffi::c_int {
                    ::core::ptr::null_mut::<picoquic_path_t>()
                } else {
                    *(*cnx).path.offset(*path_id as isize)
                },
                1 as ::core::ffi::c_int,
                current_time,
                &raw mut ph,
                bytes,
                *consumed,
            );
        } else if is_buffered != 0 {
            picoquic_log_buffered_packet(
                cnx,
                if *path_id < 0 as ::core::ffi::c_int {
                    ::core::ptr::null_mut::<picoquic_path_t>()
                } else {
                    *(*cnx).path.offset(*path_id as isize)
                },
                ph.ptype,
                current_time,
            );
        } else {
            picoquic_log_dropped_packet(
                cnx,
                if *path_id < 0 as ::core::ffi::c_int {
                    ::core::ptr::null_mut::<picoquic_path_t>()
                } else {
                    *(*cnx).path.offset(*path_id as isize)
                },
                &raw mut ph,
                length,
                ret,
                bytes,
                current_time,
            );
        }
    }
    if ret == PICOQUIC_ERROR_VERSION_NOT_SUPPORTED {
        if packet_length >= PICOQUIC_ENFORCED_INITIAL_MTU as size_t {
            if (*quic).is_port_blocking_disabled() as ::core::ffi::c_int != 0
                || picoquic_check_addr_blocked(addr_from) == 0
            {
                picoquic_prepare_version_negotiation(
                    quic,
                    addr_from,
                    addr_to,
                    if_index_to as ::core::ffi::c_ulong,
                    &raw mut ph,
                    raw_bytes,
                );
            }
        }
    } else if ret == PICOQUIC_ERROR_RETRY_NEEDED {
        if packet_length >= PICOQUIC_ENFORCED_INITIAL_MTU as size_t {
            if (*quic).is_port_blocking_disabled() as ::core::ffi::c_int != 0
                || picoquic_check_addr_blocked(addr_from) == 0
            {
                picoquic_queue_retry_packet(
                    quic,
                    addr_from,
                    addr_to,
                    if_index_to,
                    &raw mut ph,
                    current_time,
                );
            }
        }
    } else if ret == PICOQUIC_ERROR_SERVER_BUSY {
        if packet_length >= PICOQUIC_ENFORCED_INITIAL_MTU as size_t {
            if (*quic).is_port_blocking_disabled() as ::core::ffi::c_int != 0
                || picoquic_check_addr_blocked(addr_from) == 0
            {
                picoquic_queue_busy_packet(
                    quic,
                    addr_from,
                    addr_to,
                    if_index_to,
                    &raw mut ph,
                    current_time,
                );
            }
        }
    } else if ret == 0 as ::core::ffi::c_int {
        if cnx.is_null() {
            if picoquic_is_connection_id_null(&raw mut ph.dest_cnx_id) == 0
                && ((*quic).is_port_blocking_disabled() as ::core::ffi::c_int != 0
                    || picoquic_check_addr_blocked(addr_from) == 0)
            {
                picoquic_process_unexpected_cnxid(
                    quic,
                    length,
                    addr_from,
                    addr_to,
                    if_index_to as ::core::ffi::c_ulong,
                    &raw mut ph,
                    current_time,
                );
            }
            ret = PICOQUIC_ERROR_DETECTED;
        } else {
            (*cnx).set_quic_bit_received_0(
                (*cnx).quic_bit_received_0()
                    | ph.quic_bit_is_zero() as ::core::ffi::c_int as ::core::ffi::c_uint,
            );
            match ph.ptype as ::core::ffi::c_uint {
                1 => {
                    ret = picoquic_incoming_version_negotiation(
                        cnx,
                        bytes,
                        length,
                        addr_from,
                        &raw mut ph,
                        current_time,
                    );
                }
                2 => {
                    if ph.has_reserved_bit_set() != 0 {
                        ret = PICOQUIC_ERROR_PACKET_HEADER_PARSING;
                    } else if (*cnx).client_mode() == 0
                        && picoquic_compare_connection_id(
                            &raw mut ph.dest_cnx_id,
                            &raw mut (*cnx).initial_cnxid,
                        ) == 0 as ::core::ffi::c_int
                        || picoquic_compare_connection_id(
                            &raw mut ph.dest_cnx_id,
                            &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                                .p_local_cnxid)
                                .cnx_id,
                        ) == 0 as ::core::ffi::c_int
                    {
                        if picoquic_is_connection_id_null(
                            &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                                .p_remote_cnxid)
                                .cnx_id,
                        ) != 0
                        {
                            (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                                .p_remote_cnxid)
                                .cnx_id = ph.srce_cnx_id;
                        } else if picoquic_compare_connection_id(
                            &raw mut (*(**(*cnx).path.offset(0 as ::core::ffi::c_int as isize))
                                .p_remote_cnxid)
                                .cnx_id,
                            &raw mut ph.srce_cnx_id,
                        ) != 0 as ::core::ffi::c_int
                        {
                            ret = PICOQUIC_ERROR_UNEXPECTED_PACKET;
                        }
                        if ret == 0 as ::core::ffi::c_int {
                            if packet_length < PICOQUIC_ENFORCED_INITIAL_MTU as size_t {
                                if (*cnx).did_receive_short_initial() == 0 {
                                    picoquic_log_app_message(
                                        cnx as *mut picoquic_cnx_t,
                                        b"Received unpadded initial, length=%zu\0".as_ptr()
                                            as *const ::core::ffi::c_char,
                                        packet_length,
                                    );
                                }
                                (*cnx).set_did_receive_short_initial(
                                    1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                );
                            }
                            if (*cnx).client_mode() as ::core::ffi::c_int == 0 as ::core::ffi::c_int
                            {
                                if is_first_segment != 0 {
                                    (*cnx).initial_data_received = ((*cnx).initial_data_received
                                        as ::core::ffi::c_ulong)
                                        .wrapping_add(packet_length as ::core::ffi::c_ulong)
                                        as uint64_t
                                        as uint64_t;
                                }
                                ret = picoquic_incoming_client_initial(
                                    &raw mut cnx,
                                    bytes,
                                    packet_length,
                                    decrypted_data,
                                    addr_from,
                                    addr_to,
                                    if_index_to as ::core::ffi::c_ulong,
                                    &raw mut ph,
                                    current_time,
                                    new_context_created,
                                );
                                *first_cnx = cnx;
                            } else {
                                ret = picoquic_incoming_server_initial(
                                    cnx,
                                    bytes,
                                    packet_length,
                                    decrypted_data,
                                    addr_to,
                                    if_index_to as ::core::ffi::c_ulong,
                                    &raw mut ph,
                                    current_time,
                                );
                            }
                        }
                    } else {
                        ret = PICOQUIC_ERROR_DETECTED;
                    }
                }
                3 => {
                    ret = picoquic_incoming_retry(cnx, raw_bytes, &raw mut ph, current_time);
                }
                4 => {
                    if ph.has_reserved_bit_set() != 0 {
                        ret = picoquic_connection_error(
                            cnx,
                            PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                            0 as uint64_t,
                        );
                    } else if ph.has_reserved_bit_set() != 0 {
                        ret = PICOQUIC_ERROR_PACKET_HEADER_PARSING;
                    } else if (*cnx).client_mode() != 0 {
                        ret = picoquic_incoming_server_handshake(
                            cnx,
                            bytes,
                            decrypted_data,
                            addr_to,
                            if_index_to as ::core::ffi::c_ulong,
                            &raw mut ph,
                            current_time,
                        );
                    } else {
                        ret = picoquic_incoming_client_handshake(
                            cnx,
                            bytes,
                            decrypted_data,
                            &raw mut ph,
                            current_time,
                        );
                    }
                }
                5 => {
                    if ph.has_reserved_bit_set() != 0 {
                        ret = picoquic_connection_error(
                            cnx,
                            PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                            0 as uint64_t,
                        );
                    } else {
                        if is_first_segment != 0 {
                            (*cnx).initial_data_received =
                                ((*cnx).initial_data_received as ::core::ffi::c_ulong)
                                    .wrapping_add(packet_length as ::core::ffi::c_ulong)
                                    as uint64_t as uint64_t;
                        }
                        ret = picoquic_incoming_0rtt(
                            cnx,
                            bytes,
                            decrypted_data,
                            &raw mut ph,
                            current_time,
                        );
                    }
                }
                6 => {
                    ret = picoquic_incoming_1rtt(
                        cnx,
                        *path_id,
                        bytes,
                        decrypted_data,
                        &raw mut ph,
                        addr_from,
                        addr_to,
                        if_index_to,
                        received_ecn,
                        path_is_not_allocated,
                        current_time,
                    );
                }
                _ => {
                    ret = PICOQUIC_ERROR_DETECTED;
                }
            }
        }
    } else if ret == PICOQUIC_ERROR_STATELESS_RESET {
        ret = picoquic_incoming_stateless_reset(cnx);
    } else if ret == PICOQUIC_ERROR_AEAD_CHECK
        && ph.ptype as ::core::ffi::c_uint
            == picoquic_packet_handshake as ::core::ffi::c_int as ::core::ffi::c_uint
        && !cnx.is_null()
        && ((*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_client_init_sent as ::core::ffi::c_int as ::core::ffi::c_uint
            || (*cnx).cnx_state as ::core::ffi::c_uint
                == picoquic_state_client_init_resent as ::core::ffi::c_int as ::core::ffi::c_uint)
    {
        if !(*cnx).pkt_ctx[picoquic_packet_context_initial as ::core::ffi::c_int as usize]
            .pending_first
            .is_null()
            && (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).nb_retransmit
                == 0 as uint64_t
        {
            (**(*cnx).path.offset(0 as ::core::ffi::c_int as isize)).retransmit_timer =
                current_time.wrapping_sub(
                    (*(*cnx).pkt_ctx
                        [picoquic_packet_context_initial as ::core::ffi::c_int as usize]
                        .pending_first)
                        .send_time,
                );
        }
    }
    if ret == 0 as ::core::ffi::c_int {
        if !cnx.is_null()
            && (*cnx).cnx_state as ::core::ffi::c_uint
                != picoquic_state_disconnected as ::core::ffi::c_int as ::core::ffi::c_uint
            && ph.ptype as ::core::ffi::c_uint
                != picoquic_packet_version_negotiation as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            (*cnx).nb_packets_received = (*cnx).nb_packets_received.wrapping_add(1);
            (*cnx).latest_receive_time = current_time;
            ret = picoquic_record_pn_received(
                cnx,
                ph.pc,
                ph.l_cid as *mut picoquic_local_cnxid_t,
                ph.pn64,
                receive_time,
            );
            picoquic_ecn_accounting(
                cnx,
                received_ecn,
                ph.pc,
                ph.l_cid as *mut picoquic_local_cnxid_t,
            );
        }
        if !cnx.is_null() {
            picoquic_reinsert_by_wake_time((*cnx).quic, cnx, current_time);
        }
    } else if ret == PICOQUIC_ERROR_AEAD_CHECK
        || ret == PICOQUIC_ERROR_INITIAL_TOO_SHORT
        || ret == PICOQUIC_ERROR_PACKET_WRONG_VERSION
        || ret == PICOQUIC_ERROR_INITIAL_CID_TOO_SHORT
        || ret == PICOQUIC_ERROR_PORT_BLOCKED
        || ret == PICOQUIC_ERROR_UNEXPECTED_PACKET
        || ret == PICOQUIC_ERROR_CNXID_CHECK
        || ret == PICOQUIC_ERROR_RETRY
        || ret == PICOQUIC_ERROR_DETECTED
        || ret == PICOQUIC_ERROR_SERVER_BUSY
        || ret == PICOQUIC_ERROR_CONNECTION_DELETED
        || ret == PICOQUIC_ERROR_CNXID_SEGMENT
        || ret == PICOQUIC_ERROR_VERSION_NOT_SUPPORTED
        || ret == PICOQUIC_ERROR_PACKET_TOO_LONG
        || ret == PICOQUIC_ERROR_DUPLICATE
        || ret == PICOQUIC_ERROR_AEAD_NOT_READY
    {
        if ret == PICOQUIC_ERROR_AEAD_CHECK
            || ret == PICOQUIC_ERROR_PACKET_WRONG_VERSION
            || ret == PICOQUIC_ERROR_AEAD_NOT_READY
            || ret == PICOQUIC_ERROR_PACKET_TOO_LONG
            || ret == PICOQUIC_ERROR_VERSION_NOT_SUPPORTED
            || ret == PICOQUIC_ERROR_RETRY
            || ret == PICOQUIC_ERROR_SERVER_BUSY
        {
            ret = 0 as ::core::ffi::c_int;
        } else {
            ret = -(1 as ::core::ffi::c_int);
        }
        if !cnx.is_null() {
            picoquic_reinsert_by_wake_time((*cnx).quic, cnx, current_time);
        }
    } else if ret == 1 as ::core::ffi::c_int {
        ret = -(1 as ::core::ffi::c_int);
    } else if ret != 0 as ::core::ffi::c_int {
        ret = -(1 as ::core::ffi::c_int);
    }
    if !decrypted_data.is_null() && (*decrypted_data).bytes.is_null() {
        picoquic_stream_data_node_recycle(decrypted_data);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_incoming_packet_ex(
    mut quic: *mut picoquic_quic_t,
    mut bytes: *mut uint8_t,
    mut packet_length: size_t,
    mut addr_from: *mut sockaddr,
    mut addr_to: *mut sockaddr,
    mut if_index_to: ::core::ffi::c_int,
    mut received_ecn: ::core::ffi::c_uchar,
    mut first_cnx: *mut *mut picoquic_cnx_t,
    mut first_path_id: *mut ::core::ffi::c_int,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut consumed_index: size_t = 0 as size_t;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut previous_destid: picoquic_connection_id_t = picoquic_null_connection_id;
    while consumed_index < packet_length {
        let mut consumed: size_t = 0 as size_t;
        ret = picoquic_incoming_segment(
            quic,
            bytes.offset(consumed_index as isize),
            packet_length.wrapping_sub(consumed_index),
            packet_length,
            &raw mut consumed,
            addr_from,
            addr_to,
            if_index_to,
            received_ecn,
            current_time,
            current_time,
            &raw mut previous_destid,
            first_cnx,
            first_path_id,
        );
        if ret == 0 as ::core::ffi::c_int {
            consumed_index = consumed_index.wrapping_add(consumed);
            if consumed == 0 as size_t {
                break;
            }
        } else {
            ret = 0 as ::core::ffi::c_int;
            break;
        }
    }
    if !(*first_cnx).is_null() && packet_length > (**first_cnx).max_mtu_received {
        (**first_cnx).max_mtu_received = packet_length;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_incoming_packet(
    mut quic: *mut picoquic_quic_t,
    mut bytes: *mut uint8_t,
    mut packet_length: size_t,
    mut addr_from: *mut sockaddr,
    mut addr_to: *mut sockaddr,
    mut if_index_to: ::core::ffi::c_int,
    mut received_ecn: ::core::ffi::c_uchar,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut first_cnx: *mut picoquic_cnx_t = ::core::ptr::null_mut::<picoquic_cnx_t>();
    let mut path_id: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = picoquic_incoming_packet_ex(
        quic,
        bytes,
        packet_length,
        addr_from,
        addr_to,
        if_index_to,
        received_ecn,
        &raw mut first_cnx,
        &raw mut path_id,
        current_time,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_process_sooner_packets(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
) {
    let mut packet: *mut picoquic_stateless_packet_t = (*cnx).first_sooner;
    let mut previous: *mut picoquic_stateless_packet_t =
        ::core::ptr::null_mut::<picoquic_stateless_packet_t>();
    (*cnx).set_recycle_sooner_needed(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    while !packet.is_null() {
        let mut next_packet: *mut picoquic_stateless_packet_t =
            (*packet).next_packet as *mut picoquic_stateless_packet_t;
        let mut could_try_now: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        let mut epoch: picoquic_epoch_enum = picoquic_epoch_initial;
        match (*packet).ptype as ::core::ffi::c_uint {
            4 => {
                epoch = picoquic_epoch_handshake;
            }
            6 => {
                epoch = picoquic_epoch_1rtt;
            }
            _ => {
                could_try_now = 0 as ::core::ffi::c_int;
            }
        }
        if could_try_now != 0
            && (!(*cnx).crypto_context[epoch as usize].aead_decrypt.is_null()
                || !(*cnx).crypto_context[epoch as usize].pn_dec.is_null())
        {
            let mut consumed_index: size_t = 0 as size_t;
            let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut previous_destid: picoquic_connection_id_t = picoquic_null_connection_id;
            let mut first_cnx: *mut picoquic_cnx_t = ::core::ptr::null_mut::<picoquic_cnx_t>();
            let mut path_id: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
            while consumed_index < (*packet).length {
                let mut consumed: size_t = 0 as size_t;
                ret = picoquic_incoming_segment(
                    (*cnx).quic,
                    (&raw mut (*packet).bytes as *mut uint8_t).offset(consumed_index as isize),
                    (*packet).length.wrapping_sub(consumed_index),
                    (*packet).length,
                    &raw mut consumed,
                    &raw mut (*packet).addr_to as *mut sockaddr,
                    &raw mut (*packet).addr_local as *mut sockaddr,
                    (*packet).if_index_local,
                    (*packet).received_ecn,
                    current_time,
                    (*packet).receive_time,
                    &raw mut previous_destid,
                    &raw mut first_cnx,
                    &raw mut path_id,
                );
                if !(ret == 0 as ::core::ffi::c_int && consumed > 0 as size_t) {
                    break;
                }
                consumed_index = consumed_index.wrapping_add(consumed);
            }
            ret != 0 as ::core::ffi::c_int;
            if previous.is_null() {
                (*cnx).first_sooner = (*packet).next_packet as *mut picoquic_stateless_packet_t;
            } else {
                (*previous).next_packet = (*packet).next_packet;
            }
            picoquic_delete_stateless_packet(packet);
        } else {
            previous = packet;
        }
        packet = next_packet;
    }
}

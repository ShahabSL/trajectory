use ::c2rust_bitfields;
extern "C" {
    pub type st_ptls_t;
    pub type st_ptls_key_schedule_t;
    pub type st_ptls_traffic_protection_t;
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn ptls_buffer__release_memory(buf: *mut ptls_buffer_t);
    fn ptls_buffer_reserve(buf: *mut ptls_buffer_t, delta: size_t) -> ::core::ffi::c_int;
    fn ptls_client_new(ctx: *mut ptls_context_t) -> *mut ptls_t;
    fn ptls_server_new(ctx: *mut ptls_context_t) -> *mut ptls_t;
    fn ptls_free(tls: *mut ptls_t);
    fn ptls_get_client_random(tls: *mut ptls_t) -> ptls_iovec_t;
    fn ptls_get_cipher(tls: *mut ptls_t) -> *const ptls_cipher_suite_t;
    fn ptls_get_server_name(tls: *mut ptls_t) -> *const ::core::ffi::c_char;
    fn ptls_set_server_name(
        tls: *mut ptls_t,
        server_name: *const ::core::ffi::c_char,
        server_name_len: size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_get_negotiated_protocol(tls: *mut ptls_t) -> *const ::core::ffi::c_char;
    fn ptls_set_negotiated_protocol(
        tls: *mut ptls_t,
        protocol: *const ::core::ffi::c_char,
        protocol_len: size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_handshake_is_complete(tls: *mut ptls_t) -> ::core::ffi::c_int;
    fn ptls_is_psk_handshake(tls: *mut ptls_t) -> ::core::ffi::c_int;
    fn ptls_get_data_ptr(tls: *mut ptls_t) -> *mut *mut ::core::ffi::c_void;
    fn ptls_is_server(tls: *mut ptls_t) -> ::core::ffi::c_int;
    fn ptls_hkdf_extract(
        hash: *const ptls_hash_algorithm_t,
        output: *mut ::core::ffi::c_void,
        salt: ptls_iovec_t,
        ikm: ptls_iovec_t,
    ) -> ::core::ffi::c_int;
    fn ptls_hkdf_expand_label(
        algo: *const ptls_hash_algorithm_t,
        output: *mut ::core::ffi::c_void,
        outlen: size_t,
        secret: ptls_iovec_t,
        label: *const ::core::ffi::c_char,
        hash_value: ptls_iovec_t,
        label_prefix: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn ptls_cipher_new(
        algo: *const ptls_cipher_algorithm_t,
        is_enc: ::core::ffi::c_int,
        key: *const ::core::ffi::c_void,
    ) -> *mut ptls_cipher_context_t;
    fn ptls_cipher_free(ctx: *mut ptls_cipher_context_t);
    fn ptls_aead_new(
        aead: *const ptls_aead_algorithm_t,
        hash: *const ptls_hash_algorithm_t,
        is_enc: ::core::ffi::c_int,
        secret: *const ::core::ffi::c_void,
        label_prefix: *const ::core::ffi::c_char,
    ) -> *mut ptls_aead_context_t;
    fn ptls_aead_free(ctx: *mut ptls_aead_context_t);
    fn ptls_aead_xor_iv(
        ctx: *mut ptls_aead_context_t,
        bytes: *const ::core::ffi::c_void,
        len: size_t,
    );
    fn ptls_get_read_epoch(tls: *mut ptls_t) -> size_t;
    fn ptls_handle_message(
        tls: *mut ptls_t,
        sendbuf: *mut ptls_buffer_t,
        epoch_offsets: *mut size_t,
        in_epoch: size_t,
        input: *const ::core::ffi::c_void,
        inlen: size_t,
        properties: *mut ptls_handshake_properties_t,
    ) -> ::core::ffi::c_int;
    static mut ptls_clear_memory:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
    fn ptls_load_certificates(
        ctx: *mut ptls_context_t,
        cert_pem_file: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn ptls_hexdump(
        dst: *mut ::core::ffi::c_char,
        src: *const ::core::ffi::c_void,
        len: size_t,
    ) -> *mut ::core::ffi::c_char;
    static mut ptls_get_time: ptls_get_time_t;
    fn picosplay_first(tree: *mut picosplay_tree_t) -> *mut picosplay_node_t;
    fn picosplay_delete_hint(tree: *mut picosplay_tree_t, node: *mut picosplay_node_t);
    fn picoquic_log_app_message(cnx: *mut picoquic_cnx_t, fmt: *const ::core::ffi::c_char, ...);
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn picoquic_string_create(
        original: *const ::core::ffi::c_char,
        len: size_t,
    ) -> *mut ::core::ffi::c_char;
    fn picoquic_string_duplicate(original: *const ::core::ffi::c_char) -> *mut ::core::ffi::c_char;
    static picoquic_null_connection_id: picoquic_connection_id_t;
    fn picoquic_format_connection_id(
        bytes: *mut uint8_t,
        bytes_max: size_t,
        cnx_id: picoquic_connection_id_t,
    ) -> uint8_t;
    fn picoquic_compare_connection_id(
        cnx_id1: *const picoquic_connection_id_t,
        cnx_id2: *const picoquic_connection_id_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_file_open(
        file_name: *const ::core::ffi::c_char,
        flags: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn picoquic_file_close(F: *mut FILE) -> *mut FILE;
    fn picoquic_frames_varint_decode(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
        n64: *mut uint64_t,
    ) -> *const uint8_t;
    fn picoquic_frames_uint64_decode(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
        n: *mut uint64_t,
    ) -> *const uint8_t;
    fn picoquic_frames_cid_decode(
        bytes: *const uint8_t,
        bytes_max: *const uint8_t,
        cid: *mut picoquic_connection_id_t,
    ) -> *const uint8_t;
    fn picoquic_frames_varint_encode(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        n64: uint64_t,
    ) -> *mut uint8_t;
    fn picoquic_frames_uint64_encode(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        n: uint64_t,
    ) -> *mut uint8_t;
    fn picoquic_frames_cid_encode(
        bytes: *mut uint8_t,
        bytes_max: *const uint8_t,
        cid: *const picoquic_connection_id_t,
    ) -> *mut uint8_t;
    static picoquic_supported_versions: [picoquic_version_parameters_t; 0];
    static picoquic_nb_supported_versions: size_t;
    fn picoquic_store_ticket(
        quic: *mut picoquic_quic_t,
        sni: *const ::core::ffi::c_char,
        sni_length: uint16_t,
        alpn: *const ::core::ffi::c_char,
        alpn_length: uint16_t,
        version: uint32_t,
        ip_addr: *const uint8_t,
        ip_addr_length: uint8_t,
        ip_addr_client: *const uint8_t,
        ip_addr_client_length: uint8_t,
        ticket: *mut uint8_t,
        ticket_length: uint16_t,
        tp: *const picoquic_tp_t,
    ) -> ::core::ffi::c_int;
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
    fn picoquic_retrieve_issued_ticket(
        quic: *mut picoquic_quic_t,
        ticket_id: uint64_t,
    ) -> *mut picoquic_issued_ticket_t;
    fn picoquic_registered_token_check_reuse(
        quic: *mut picoquic_quic_t,
        token: *const uint8_t,
        token_length: size_t,
        expiry_time: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_registered_token_clear(quic: *mut picoquic_quic_t, expiry_time_max: uint64_t);
    fn picoquic_connection_error(
        cnx: *mut picoquic_cnx_t,
        local_error: uint64_t,
        frame_type: uint64_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_connection_disconnect(cnx: *mut picoquic_cnx_t);
    fn picoformat_32(bytes: *mut uint8_t, n32: uint32_t);
    fn picoformat_64(bytes: *mut uint8_t, n64: uint64_t);
    fn picoquic_log_pn_dec_trial(cnx: *mut picoquic_cnx_t);
    fn picoquic_false_start_transition(cnx: *mut picoquic_cnx_t, current_time: uint64_t);
    fn picoquic_client_almost_ready_transition(cnx: *mut picoquic_cnx_t);
    fn picoquic_seed_bandwidth(
        cnx: *mut picoquic_cnx_t,
        rtt_min: uint64_t,
        cwin: uint64_t,
        ip_addr: *const uint8_t,
        ip_addr_length: uint8_t,
    );
    fn picoquic_prepare_transport_extensions(
        cnx: *mut picoquic_cnx_t,
        extension_mode: ::core::ffi::c_int,
        bytes: *mut uint8_t,
        bytes_max: size_t,
        consumed: *mut size_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_receive_transport_extensions(
        cnx: *mut picoquic_cnx_t,
        extension_mode: ::core::ffi::c_int,
        bytes: *mut uint8_t,
        bytes_max: size_t,
        consumed: *mut size_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_log_context_free_app_message(
        quic: *mut picoquic_quic_t,
        cid: *const picoquic_connection_id_t,
        fmt: *const ::core::ffi::c_char,
        ...
    );
    fn picoquic_log_negotiated_alpn(
        cnx: *mut picoquic_cnx_t,
        is_local: ::core::ffi::c_int,
        sni: *const uint8_t,
        sni_len: size_t,
        alpn: *const uint8_t,
        alpn_len: size_t,
        alpn_list: *const ptls_iovec_t,
        alpn_count: size_t,
    );
    fn picoquic_ptls_fusion_load(unload: ::core::ffi::c_int);
    fn picoquic_ptls_openssl_load(unload: ::core::ffi::c_int);
    fn picoquic_ptls_minicrypto_load(unload: ::core::ffi::c_int);
}
pub type __builtin_va_list = [__va_list_tag; 1];
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
pub type ssize_t = isize;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = isize;
pub type ptls_t = st_ptls_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_ptls_context_t {
    pub random_bytes: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>,
    pub get_time: *mut ptls_get_time_t,
    pub key_exchanges: *mut *const ptls_key_exchange_algorithm_t,
    pub cipher_suites: *mut *const ptls_cipher_suite_t,
    pub certificates: C2Rust_Unnamed_11,
    pub pre_shared_key: C2Rust_Unnamed_10,
    pub ech: C2Rust_Unnamed_7,
    pub on_client_hello: *mut ptls_on_client_hello_t,
    pub emit_certificate: *mut ptls_emit_certificate_t,
    pub sign_certificate: *mut ptls_sign_certificate_t,
    pub verify_certificate: *mut ptls_verify_certificate_t,
    pub ticket_lifetime: uint32_t,
    pub max_early_data_size: uint32_t,
    pub max_buffer_size: size_t,
    pub hkdf_label_prefix__obsolete: *const ::core::ffi::c_char,
    #[bitfield(
        name = "require_dhe_on_psk",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    #[bitfield(name = "use_exporter", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(
        name = "send_change_cipher_spec",
        ty = "::core::ffi::c_uint",
        bits = "2..=2"
    )]
    #[bitfield(
        name = "require_client_authentication",
        ty = "::core::ffi::c_uint",
        bits = "3..=3"
    )]
    #[bitfield(
        name = "omit_end_of_early_data",
        ty = "::core::ffi::c_uint",
        bits = "4..=4"
    )]
    #[bitfield(
        name = "use_raw_public_keys",
        ty = "::core::ffi::c_uint",
        bits = "5..=5"
    )]
    #[bitfield(
        name = "server_cipher_preference",
        ty = "::core::ffi::c_uint",
        bits = "6..=6"
    )]
    #[bitfield(
        name = "server_cipher_chacha_priority",
        ty = "::core::ffi::c_uint",
        bits = "7..=7"
    )]
    pub require_dhe_on_psk_use_exporter_send_change_cipher_spec_require_client_authentication_omit_end_of_early_data_use_raw_public_keys_server_cipher_preference_server_cipher_chacha_priority:
        [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub encrypt_ticket: *mut ptls_encrypt_ticket_t,
    pub save_ticket: *mut ptls_save_ticket_t,
    pub log_event: *mut ptls_log_event_t,
    pub update_open_count: *mut ptls_update_open_count_t,
    pub update_traffic_key: *mut ptls_update_traffic_key_t,
    pub decompress_certificate: *mut ptls_decompress_certificate_t,
    pub on_extension: *mut ptls_on_extension_t,
    pub tls12_cipher_suites: *mut *const ptls_cipher_suite_t,
    pub ticket_context: C2Rust_Unnamed_0,
    pub client_ca_names: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub list: *const ptls_iovec_t,
    pub count: size_t,
}
pub type ptls_iovec_t = st_ptls_iovec_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_iovec_t {
    pub base: *mut uint8_t,
    pub len: size_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub bytes: [uint8_t; 32],
    #[bitfield(name = "is_set", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub is_set: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type ptls_cipher_suite_t = st_ptls_cipher_suite_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_cipher_suite_t {
    pub id: uint16_t,
    pub aead: *const ptls_aead_algorithm_t,
    pub hash: *const ptls_hash_algorithm_t,
    pub name: *const ::core::ffi::c_char,
}
pub type ptls_hash_algorithm_t = st_ptls_hash_algorithm_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_hash_algorithm_t {
    pub name: *const ::core::ffi::c_char,
    pub block_size: size_t,
    pub digest_size: size_t,
    pub create: Option<unsafe extern "C" fn() -> *mut ptls_hash_context_t>,
    pub empty_digest: [uint8_t; 64],
}
pub type ptls_hash_context_t = st_ptls_hash_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_hash_context_t {
    pub update: Option<
        unsafe extern "C" fn(*mut st_ptls_hash_context_t, *const ::core::ffi::c_void, size_t) -> (),
    >,
    pub final_0: Option<
        unsafe extern "C" fn(
            *mut st_ptls_hash_context_t,
            *mut ::core::ffi::c_void,
            ptls_hash_final_mode_t,
        ) -> (),
    >,
    pub clone_:
        Option<unsafe extern "C" fn(*mut st_ptls_hash_context_t) -> *mut st_ptls_hash_context_t>,
}
pub type ptls_hash_final_mode_t = en_ptls_hash_final_mode_t;
pub type en_ptls_hash_final_mode_t = ::core::ffi::c_uint;
pub const PTLS_HASH_FINAL_MODE_SNAPSHOT: en_ptls_hash_final_mode_t = 2;
pub const PTLS_HASH_FINAL_MODE_RESET: en_ptls_hash_final_mode_t = 1;
pub const PTLS_HASH_FINAL_MODE_FREE: en_ptls_hash_final_mode_t = 0;
pub type ptls_aead_algorithm_t = st_ptls_aead_algorithm_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_ptls_aead_algorithm_t {
    pub name: *const ::core::ffi::c_char,
    pub confidentiality_limit: uint64_t,
    pub integrity_limit: uint64_t,
    pub ctr_cipher: *const ptls_cipher_algorithm_t,
    pub ecb_cipher: *const ptls_cipher_algorithm_t,
    pub key_size: size_t,
    pub iv_size: size_t,
    pub tag_size: size_t,
    pub tls12: C2Rust_Unnamed_1,
    #[bitfield(name = "non_temporal", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub non_temporal: [u8; 1],
    pub align_bits: uint8_t,
    pub context_size: size_t,
    pub setup_crypto: Option<
        unsafe extern "C" fn(
            *mut ptls_aead_context_t,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_aead_context_t = st_ptls_aead_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_aead_context_t {
    pub algo: *const st_ptls_aead_algorithm_t,
    pub dispose_crypto: Option<unsafe extern "C" fn(*mut st_ptls_aead_context_t) -> ()>,
    pub do_get_iv:
        Option<unsafe extern "C" fn(*mut st_ptls_aead_context_t, *mut ::core::ffi::c_void) -> ()>,
    pub do_set_iv:
        Option<unsafe extern "C" fn(*mut st_ptls_aead_context_t, *const ::core::ffi::c_void) -> ()>,
    pub do_encrypt_init: Option<
        unsafe extern "C" fn(
            *mut st_ptls_aead_context_t,
            uint64_t,
            *const ::core::ffi::c_void,
            size_t,
        ) -> (),
    >,
    pub do_encrypt_update: Option<
        unsafe extern "C" fn(
            *mut st_ptls_aead_context_t,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            size_t,
        ) -> size_t,
    >,
    pub do_encrypt_final: Option<
        unsafe extern "C" fn(*mut st_ptls_aead_context_t, *mut ::core::ffi::c_void) -> size_t,
    >,
    pub do_encrypt: Option<
        unsafe extern "C" fn(
            *mut st_ptls_aead_context_t,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            size_t,
            uint64_t,
            *const ::core::ffi::c_void,
            size_t,
            *mut ptls_aead_supplementary_encryption_t,
        ) -> (),
    >,
    pub do_encrypt_v: Option<
        unsafe extern "C" fn(
            *mut st_ptls_aead_context_t,
            *mut ::core::ffi::c_void,
            *mut ptls_iovec_t,
            size_t,
            uint64_t,
            *const ::core::ffi::c_void,
            size_t,
        ) -> (),
    >,
    pub do_decrypt: Option<
        unsafe extern "C" fn(
            *mut st_ptls_aead_context_t,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            size_t,
            uint64_t,
            *const ::core::ffi::c_void,
            size_t,
        ) -> size_t,
    >,
}
pub type ptls_aead_supplementary_encryption_t = st_ptls_aead_supplementary_encryption_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_aead_supplementary_encryption_t {
    pub ctx: *mut ptls_cipher_context_t,
    pub input: *const ::core::ffi::c_void,
    pub output: [uint8_t; 16],
}
pub type ptls_cipher_context_t = st_ptls_cipher_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_cipher_context_t {
    pub algo: *const st_ptls_cipher_algorithm_t,
    pub do_dispose: Option<unsafe extern "C" fn(*mut st_ptls_cipher_context_t) -> ()>,
    pub do_init: Option<
        unsafe extern "C" fn(*mut st_ptls_cipher_context_t, *const ::core::ffi::c_void) -> (),
    >,
    pub do_transform: Option<
        unsafe extern "C" fn(
            *mut st_ptls_cipher_context_t,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            size_t,
        ) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_cipher_algorithm_t {
    pub name: *const ::core::ffi::c_char,
    pub key_size: size_t,
    pub block_size: size_t,
    pub iv_size: size_t,
    pub context_size: size_t,
    pub setup_crypto: Option<
        unsafe extern "C" fn(
            *mut ptls_cipher_context_t,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub fixed_iv_size: size_t,
    pub record_iv_size: size_t,
}
pub type ptls_cipher_algorithm_t = st_ptls_cipher_algorithm_t;
pub type ptls_on_extension_t = st_ptls_on_extension_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_on_extension_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_on_extension_t,
            *mut ptls_t,
            uint8_t,
            uint16_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_decompress_certificate_t = st_ptls_decompress_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_decompress_certificate_t {
    pub supported_algorithms: *const uint16_t,
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_decompress_certificate_t,
            *mut ptls_t,
            uint16_t,
            ptls_iovec_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_update_traffic_key_t = st_ptls_update_traffic_key_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_update_traffic_key_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_update_traffic_key_t,
            *mut ptls_t,
            ::core::ffi::c_int,
            size_t,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_update_open_count_t = st_ptls_update_open_count_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_update_open_count_t {
    pub cb: Option<unsafe extern "C" fn(*mut st_ptls_update_open_count_t, ssize_t) -> ()>,
}
pub type ptls_log_event_t = st_ptls_log_event_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_log_event_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_log_event_t,
            *mut ptls_t,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            ...
        ) -> (),
    >,
}
pub type ptls_save_ticket_t = st_ptls_save_ticket_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_save_ticket_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_save_ticket_t,
            *mut ptls_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_encrypt_ticket_t = st_ptls_encrypt_ticket_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_encrypt_ticket_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_encrypt_ticket_t,
            *mut ptls_t,
            ::core::ffi::c_int,
            *mut ptls_buffer_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_buffer_t = st_ptls_buffer_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_buffer_t {
    pub base: *mut uint8_t,
    pub capacity: size_t,
    pub off: size_t,
    pub is_allocated: uint8_t,
    pub align_bits: uint8_t,
}
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
pub type ptls_sign_certificate_t = st_ptls_sign_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_sign_certificate_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_sign_certificate_t,
            *mut ptls_t,
            *mut *mut ptls_async_job_t,
            *mut uint16_t,
            *mut ptls_buffer_t,
            ptls_iovec_t,
            *const uint16_t,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_async_job_t = st_ptls_async_job_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_async_job_t {
    pub destroy_: Option<unsafe extern "C" fn(*mut st_ptls_async_job_t) -> ()>,
    pub get_fd: Option<unsafe extern "C" fn(*mut st_ptls_async_job_t) -> ::core::ffi::c_int>,
    pub set_completion_callback: Option<
        unsafe extern "C" fn(
            *mut st_ptls_async_job_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
}
pub type ptls_emit_certificate_t = st_ptls_emit_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_emit_certificate_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_emit_certificate_t,
            *mut ptls_t,
            *mut ptls_message_emitter_t,
            *mut ptls_key_schedule_t,
            ptls_iovec_t,
            ::core::ffi::c_int,
            *const uint16_t,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_key_schedule_t = st_ptls_key_schedule_t;
pub type ptls_message_emitter_t = st_ptls_message_emitter_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_message_emitter_t {
    pub buf: *mut ptls_buffer_t,
    pub enc: *mut st_ptls_traffic_protection_t,
    pub record_header_length: size_t,
    pub begin_message:
        Option<unsafe extern "C" fn(*mut st_ptls_message_emitter_t) -> ::core::ffi::c_int>,
    pub commit_message:
        Option<unsafe extern "C" fn(*mut st_ptls_message_emitter_t) -> ::core::ffi::c_int>,
}
pub type ptls_on_client_hello_t = st_ptls_on_client_hello_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_on_client_hello_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_on_client_hello_t,
            *mut ptls_t,
            *mut ptls_on_client_hello_parameters_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_on_client_hello_parameters_t = st_ptls_on_client_hello_parameters_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_ptls_on_client_hello_parameters_t {
    pub server_name: ptls_iovec_t,
    pub raw_message: ptls_iovec_t,
    pub cipher_suites: ptls_iovec_t,
    pub negotiated_protocols: C2Rust_Unnamed_6,
    pub signature_algorithms: C2Rust_Unnamed_5,
    pub certificate_compression_algorithms: C2Rust_Unnamed_4,
    pub server_certificate_types: C2Rust_Unnamed_3,
    pub psk_identities: C2Rust_Unnamed_2,
    #[bitfield(
        name = "incompatible_version",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    pub incompatible_version: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub list: *const ptls_client_hello_psk_identity_t,
    pub count: size_t,
}
pub type ptls_client_hello_psk_identity_t = st_ptls_client_hello_psk_identity_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_client_hello_psk_identity_t {
    pub identity: ptls_iovec_t,
    pub obfuscated_ticket_age: uint32_t,
    pub binder: ptls_iovec_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub list: *const uint8_t,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub list: *const uint16_t,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub list: *const uint16_t,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub list: *mut ptls_iovec_t,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub client: C2Rust_Unnamed_9,
    pub server: C2Rust_Unnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub create_opener: *mut ptls_ech_create_opener_t,
    pub retry_configs: ptls_iovec_t,
}
pub type ptls_ech_create_opener_t = st_ptls_ech_create_opener_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_ech_create_opener_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_ech_create_opener_t,
            *mut *const ptls_hpke_kem_t,
            *mut *const ptls_hpke_cipher_suite_t,
            *mut ptls_t,
            uint8_t,
            ptls_hpke_cipher_suite_id_t,
            ptls_iovec_t,
            ptls_iovec_t,
        ) -> *mut ptls_aead_context_t,
    >,
}
pub type ptls_hpke_cipher_suite_id_t = st_ptls_hpke_cipher_suite_id_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_hpke_cipher_suite_id_t {
    pub kdf: uint16_t,
    pub aead: uint16_t,
}
pub type ptls_hpke_cipher_suite_t = st_ptls_hpke_cipher_suite_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_hpke_cipher_suite_t {
    pub id: ptls_hpke_cipher_suite_id_t,
    pub name: *const ::core::ffi::c_char,
    pub hash: *const ptls_hash_algorithm_t,
    pub aead: *const ptls_aead_algorithm_t,
}
pub type ptls_hpke_kem_t = st_ptls_hpke_kem_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_hpke_kem_t {
    pub id: uint16_t,
    pub keyex: *const ptls_key_exchange_algorithm_t,
    pub hash: *const ptls_hash_algorithm_t,
}
pub type ptls_key_exchange_algorithm_t = st_ptls_key_exchange_algorithm_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_key_exchange_algorithm_t {
    pub id: uint16_t,
    pub create: Option<
        unsafe extern "C" fn(
            *const st_ptls_key_exchange_algorithm_t,
            *mut *mut ptls_key_exchange_context_t,
        ) -> ::core::ffi::c_int,
    >,
    pub exchange: Option<
        unsafe extern "C" fn(
            *const st_ptls_key_exchange_algorithm_t,
            *mut ptls_iovec_t,
            *mut ptls_iovec_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
    pub data: intptr_t,
    pub name: *const ::core::ffi::c_char,
}
pub type ptls_key_exchange_context_t = st_ptls_key_exchange_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_key_exchange_context_t {
    pub algo: *const st_ptls_key_exchange_algorithm_t,
    pub pubkey: ptls_iovec_t,
    pub on_exchange: Option<
        unsafe extern "C" fn(
            *mut *mut st_ptls_key_exchange_context_t,
            ::core::ffi::c_int,
            *mut ptls_iovec_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
    pub ciphers: *mut *const ptls_hpke_cipher_suite_t,
    pub kems: *mut *const ptls_hpke_kem_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub identity: ptls_iovec_t,
    pub secret: ptls_iovec_t,
    pub hash: *const ptls_hash_algorithm_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub list: *mut ptls_iovec_t,
    pub count: size_t,
}
pub type ptls_get_time_t = st_ptls_get_time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_get_time_t {
    pub cb: Option<unsafe extern "C" fn(*mut st_ptls_get_time_t) -> uint64_t>,
}
pub type ptls_context_t = st_ptls_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_raw_extension_t {
    pub type_0: uint16_t,
    pub data: ptls_iovec_t,
}
pub type ptls_raw_extension_t = st_ptls_raw_extension_t;
pub type en_ptls_early_data_acceptance_t = ::core::ffi::c_uint;
pub const PTLS_EARLY_DATA_ACCEPTED: en_ptls_early_data_acceptance_t = 2;
pub const PTLS_EARLY_DATA_REJECTED: en_ptls_early_data_acceptance_t = 1;
pub const PTLS_EARLY_DATA_ACCEPTANCE_UNKNOWN: en_ptls_early_data_acceptance_t = 0;
pub type ptls_early_data_acceptance_t = en_ptls_early_data_acceptance_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_handshake_properties_t {
    pub c2rust_unnamed: C2Rust_Unnamed_12,
    pub additional_extensions: *mut ptls_raw_extension_t,
    pub collect_extension: Option<
        unsafe extern "C" fn(
            *mut ptls_t,
            *mut st_ptls_handshake_properties_t,
            uint16_t,
        ) -> ::core::ffi::c_int,
    >,
    pub collected_extensions: Option<
        unsafe extern "C" fn(
            *mut ptls_t,
            *mut st_ptls_handshake_properties_t,
            *mut ptls_raw_extension_t,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_12 {
    pub client: C2Rust_Unnamed_16,
    pub server: C2Rust_Unnamed_13,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_13 {
    pub selected_psk_binder: C2Rust_Unnamed_15,
    pub cookie: C2Rust_Unnamed_14,
    #[bitfield(name = "enforce_retry", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "retry_uses_cookie", ty = "::core::ffi::c_uint", bits = "1..=1")]
    pub enforce_retry_retry_uses_cookie: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub key: *const ::core::ffi::c_void,
    pub additional_data: ptls_iovec_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub base: [uint8_t; 64],
    pub len: size_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub negotiated_protocols: C2Rust_Unnamed_18,
    pub session_ticket: ptls_iovec_t,
    pub max_early_data_size: *mut size_t,
    pub early_data_acceptance: ptls_early_data_acceptance_t,
    #[bitfield(
        name = "negotiate_before_key_exchange",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    pub negotiate_before_key_exchange: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub ech: C2Rust_Unnamed_17,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub configs: ptls_iovec_t,
    pub retry_configs: *mut ptls_iovec_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub list: *const ptls_iovec_t,
    pub count: size_t,
}
pub type ptls_handshake_properties_t = st_ptls_handshake_properties_t;
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
pub type va_list = __builtin_va_list;
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
    pub __in6_u: C2Rust_Unnamed_19,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_19 {
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
pub type picoquic_tls_ctx_t = st_picoquic_tls_ctx_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_tls_ctx_t {
    pub tls: *mut ptls_t,
    pub cnx: *mut picoquic_cnx_t,
    pub client_mode: ::core::ffi::c_int,
    pub ext: [ptls_raw_extension_t; 2],
    pub handshake_properties: ptls_handshake_properties_t,
    pub alpn_vec: *mut ptls_iovec_t,
    pub alpn_vec_size: size_t,
    pub alpn_count: size_t,
    pub ext_data: *mut uint8_t,
    pub ext_data_size: size_t,
    pub app_secret_enc: [uint8_t; 64],
    pub app_secret_dec: [uint8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_log_event_t {
    pub super_0: ptls_log_event_t,
    pub fp: *mut FILE,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_cipher_suites_t {
    pub high_memory_suite: *const ptls_cipher_suite_t,
    pub low_memory_suite: *const ptls_cipher_suite_t,
}
pub type picoquic_set_tls_root_certificates_t = Option<
    unsafe extern "C" fn(*mut ptls_context_t, *mut ptls_iovec_t, size_t) -> ::core::ffi::c_int,
>;
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
pub type C2Rust_Unnamed_20 = ::core::ffi::c_uint;
pub const picoquic_tp_0rtt_cwin_remote: C2Rust_Unnamed_20 = 9;
pub const picoquic_tp_0rtt_rtt_remote: C2Rust_Unnamed_20 = 8;
pub const picoquic_tp_0rtt_cwin_local: C2Rust_Unnamed_20 = 7;
pub const picoquic_tp_0rtt_rtt_local: C2Rust_Unnamed_20 = 6;
pub const picoquic_tp_0rtt_max_streams_id_unidir: C2Rust_Unnamed_20 = 5;
pub const picoquic_tp_0rtt_max_streams_id_bidir: C2Rust_Unnamed_20 = 4;
pub const picoquic_tp_0rtt_max_stream_data_uni: C2Rust_Unnamed_20 = 3;
pub const picoquic_tp_0rtt_max_stream_data_bidi_remote: C2Rust_Unnamed_20 = 2;
pub const picoquic_tp_0rtt_max_stream_data_bidi_local: C2Rust_Unnamed_20 = 1;
pub const picoquic_tp_0rtt_max_data: C2Rust_Unnamed_20 = 0;
pub type picoquic_dispose_certificate_verifier_t =
    Option<unsafe extern "C" fn(*mut ptls_verify_certificate_t) -> ()>;
pub type picoquic_dispose_sign_certificate_t =
    Option<unsafe extern "C" fn(*mut ptls_sign_certificate_t) -> ()>;
pub type picoquic_get_certificate_verifier_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        *mut ::core::ffi::c_uint,
        *mut picoquic_dispose_certificate_verifier_t,
    ) -> *mut ptls_verify_certificate_t,
>;
pub type picoquic_set_private_key_from_file_t = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, *mut ptls_context_t) -> ::core::ffi::c_int,
>;
pub type picoquic_crypto_random_provider_t =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
pub type picoquic_clear_crypto_errors_t = Option<unsafe extern "C" fn() -> ()>;
pub type picoquic_explain_crypto_error_t = Option<
    unsafe extern "C" fn(
        *mut *const ::core::ffi::c_char,
        *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type picoquic_get_certs_from_file_t =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut size_t) -> *mut ptls_iovec_t>;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const NULL_0: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const UINT32_MAX: ::core::ffi::c_uint = 4294967295 as ::core::ffi::c_uint;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const PTLS_HELLO_RANDOM_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const PTLS_ERROR_CLASS_SELF_ALERT: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PTLS_ERROR_CLASS_INTERNAL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const PTLS_ALERT_NO_APPLICATION_PROTOCOL: ::core::ffi::c_int = 120 as ::core::ffi::c_int;
pub const PTLS_ERROR_NO_MEMORY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 1 as ::core::ffi::c_int;
pub const PTLS_ERROR_IN_PROGRESS: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 2 as ::core::ffi::c_int;
pub const PTLS_ERROR_STATELESS_RETRY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 6 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn ptls_new(
    mut ctx: *mut ptls_context_t,
    mut is_server: ::core::ffi::c_int,
) -> *mut ptls_t {
    return if is_server != 0 {
        ptls_server_new(ctx)
    } else {
        ptls_client_new(ctx)
    };
}
#[inline]
unsafe extern "C" fn ptls_iovec_init(
    mut p: *const ::core::ffi::c_void,
    mut len: size_t,
) -> ptls_iovec_t {
    let mut r: ptls_iovec_t = st_ptls_iovec_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    };
    r.base = p as *mut uint8_t;
    r.len = len;
    return r;
}
#[inline]
unsafe extern "C" fn ptls_buffer_init(
    mut buf: *mut ptls_buffer_t,
    mut smallbuf: *mut ::core::ffi::c_void,
    mut smallbuf_size: size_t,
) {
    (*buf).base = smallbuf as *mut uint8_t;
    (*buf).off = 0 as size_t;
    (*buf).capacity = smallbuf_size;
    (*buf).is_allocated = 0 as uint8_t;
    (*buf).align_bits = 0 as uint8_t;
}
#[inline]
unsafe extern "C" fn ptls_buffer_dispose(mut buf: *mut ptls_buffer_t) {
    ptls_buffer__release_memory(buf);
    *buf = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0 as size_t,
        off: 0 as size_t,
        is_allocated: 0 as uint8_t,
        align_bits: 0 as uint8_t,
    };
}
#[inline]
unsafe extern "C" fn ptls_cipher_init(
    mut ctx: *mut ptls_cipher_context_t,
    mut iv: *const ::core::ffi::c_void,
) {
    (*ctx).do_init.expect("non-null function pointer")(ctx as *mut st_ptls_cipher_context_t, iv);
}
#[inline]
unsafe extern "C" fn ptls_cipher_encrypt(
    mut ctx: *mut ptls_cipher_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    (*ctx).do_transform.expect("non-null function pointer")(
        ctx as *mut st_ptls_cipher_context_t,
        output,
        input,
        len,
    );
}
#[inline]
unsafe extern "C" fn ptls_aead_encrypt(
    mut ctx: *mut ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) -> size_t {
    (*ctx).do_encrypt.expect("non-null function pointer")(
        ctx as *mut st_ptls_aead_context_t,
        output,
        input,
        inlen,
        seq,
        aad,
        aadlen,
        ::core::ptr::null_mut::<ptls_aead_supplementary_encryption_t>(),
    );
    return inlen.wrapping_add((*(*ctx).algo).tag_size);
}
#[inline]
unsafe extern "C" fn ptls_aead_decrypt(
    mut ctx: *mut ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) -> size_t {
    return (*ctx).do_decrypt.expect("non-null function pointer")(
        ctx as *mut st_ptls_aead_context_t,
        output,
        input,
        inlen,
        seq,
        aad,
        aadlen,
    );
}
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const PICOQUIC_ERROR_CLASS: ::core::ffi::c_int = 0x400 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_AEAD_CHECK: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 3 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_MEMORY: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 5 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_SEND_BUFFER_TOO_SMALL: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 25 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_UNEXPECTED_ERROR: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 27 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_TLS_SERVER_CON_WITHOUT_CERT: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 28 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_CANNOT_COMPUTE_KEY: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 35 as ::core::ffi::c_int;
pub const PICOQUIC_ERROR_NO_ALPN_PROVIDED: ::core::ffi::c_int =
    PICOQUIC_ERROR_CLASS + 42 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_INTERNAL_ERROR: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_PARAMETER_ERROR: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION: ::core::ffi::c_int = 0xa as ::core::ffi::c_int;
pub const PICOQUIC_MAX_PACKET_SIZE: ::core::ffi::c_int = 1536 as ::core::ffi::c_int;
pub const PICOQUIC_RESET_SECRET_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PICOQUIC_AES_128_GCM_SHA256: ::core::ffi::c_int = 0x1301 as ::core::ffi::c_int;
pub const PICOQUIC_GROUP_SECP256R1: ::core::ffi::c_int = 23;
pub const PICOQUIC_CONNECTION_ID_MAX_SIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const PICOQUIC_RETRY_TOKEN_PAD_SIZE: ::core::ffi::c_int = 26 as ::core::ffi::c_int;
pub const PICOQUIC_ALPN_NUMBER_MAX: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
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
pub const PICOQUIC_INTERNAL_TEST_VERSION_1: ::core::ffi::c_int = 0x50435130 as ::core::ffi::c_int;
pub const PICOQUIC_INTERNAL_TEST_VERSION_2: ::core::ffi::c_int = 0x50435131 as ::core::ffi::c_int;
pub const PICOQUIC_NUMBER_OF_EPOCHS: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PICOQUIC_LABEL_INITIAL_CLIENT: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"client in\0") };
pub const PICOQUIC_LABEL_INITIAL_SERVER: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"server in\0") };
pub const PICOQUIC_LABEL_HP: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"hp\0") };
pub const PICOQUIC_LABEL_CID: [::core::ffi::c_char; 4] =
    unsafe { ::core::mem::transmute::<[u8; 4], [::core::ffi::c_char; 4]>(*b"cid\0") };
pub const PICOQUIC_LABEL_QUIC_BASE: *mut ::core::ffi::c_void = NULL_0;
pub const TLS_API_INIT_FLAGS_NO_OPENSSL: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const TLS_API_INIT_FLAGS_NO_MINICRYPTO: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const TLS_API_INIT_FLAGS_NO_FUSION: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PICOQUIC_CIPHER_SUITES_NB_MAX: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_PARAMETERS_TLS_EXTENSION_DRAFT: ::core::ffi::c_int =
    0xffa5 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_PARAMETERS_TLS_EXTENSION_V1: ::core::ffi::c_int =
    0x39 as ::core::ffi::c_int;
pub const PICOQUIC_TRANSPORT_PARAMETERS_MAX_SIZE: ::core::ffi::c_int = 2048 as ::core::ffi::c_int;
#[no_mangle]
pub static mut picoquic_cipher_suites: [st_picoquic_cipher_suites_t; 9] =
    [st_picoquic_cipher_suites_t {
        high_memory_suite: ::core::ptr::null::<ptls_cipher_suite_t>(),
        low_memory_suite: ::core::ptr::null::<ptls_cipher_suite_t>(),
    }; 9];
pub const PICOQUIC_KEY_EXCHANGES_NB_MAX: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
#[no_mangle]
pub static mut picoquic_key_exchanges: [*const ptls_key_exchange_algorithm_t; 5] =
    [::core::ptr::null::<ptls_key_exchange_algorithm_t>(); 5];
#[no_mangle]
pub static mut picoquic_key_exchange_secp256r1: [*const ptls_key_exchange_algorithm_t; 2] =
    [::core::ptr::null::<ptls_key_exchange_algorithm_t>(); 2];
#[no_mangle]
pub static mut picoquic_set_private_key_from_file_fn: picoquic_set_private_key_from_file_t = None;
#[no_mangle]
pub static mut picoquic_dispose_sign_certificate_fn: picoquic_dispose_sign_certificate_t = None;
#[no_mangle]
pub static mut picoquic_get_certs_from_file_fn: picoquic_get_certs_from_file_t = None;
#[no_mangle]
pub static mut picoquic_get_certificate_verifier_fn: picoquic_get_certificate_verifier_t = None;
#[no_mangle]
pub static mut picoquic_dispose_certificate_verifier_fn: picoquic_dispose_certificate_verifier_t =
    None;
#[no_mangle]
pub static mut picoquic_set_tls_root_certificates_fn: picoquic_set_tls_root_certificates_t = None;
#[no_mangle]
pub static mut picoquic_explain_crypto_error_fn: picoquic_explain_crypto_error_t = None;
#[no_mangle]
pub static mut picoquic_clear_crypto_errors_fn: picoquic_clear_crypto_errors_t = None;
#[no_mangle]
pub static mut picoquic_crypto_random_provider_fn: picoquic_crypto_random_provider_t = None;
static mut tls_api_init_flags: uint64_t = 0 as uint64_t;
static mut tls_api_is_init: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn picoquic_tls_api_init_providers(mut unload: ::core::ffi::c_int) {
    if tls_api_init_flags & TLS_API_INIT_FLAGS_NO_MINICRYPTO as uint64_t == 0 as uint64_t {
        picoquic_ptls_minicrypto_load(unload);
    }
    if tls_api_init_flags & TLS_API_INIT_FLAGS_NO_OPENSSL as uint64_t == 0 as uint64_t {
        picoquic_ptls_openssl_load(unload);
    }
    if tls_api_init_flags & TLS_API_INIT_FLAGS_NO_FUSION as uint64_t == 0 as uint64_t {
        picoquic_ptls_fusion_load(unload);
    }
}
unsafe extern "C" fn picoquic_tls_api_zero() {
    memset(
        &raw mut picoquic_cipher_suites as *mut st_picoquic_cipher_suites_t
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[st_picoquic_cipher_suites_t; 9]>() as size_t,
    );
    memset(
        &raw mut picoquic_key_exchanges as *mut *const ptls_key_exchange_algorithm_t
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[*const ptls_key_exchange_algorithm_t; 5]>() as size_t,
    );
    memset(
        &raw mut picoquic_key_exchange_secp256r1 as *mut *const ptls_key_exchange_algorithm_t
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[*const ptls_key_exchange_algorithm_t; 2]>() as size_t,
    );
    picoquic_set_private_key_from_file_fn = None;
    picoquic_dispose_sign_certificate_fn = None;
    picoquic_get_certs_from_file_fn = None;
    picoquic_get_certificate_verifier_fn = None;
    picoquic_dispose_certificate_verifier_fn = None;
    picoquic_set_tls_root_certificates_fn = None;
    picoquic_explain_crypto_error_fn = None;
    picoquic_clear_crypto_errors_fn = None;
    picoquic_crypto_random_provider_fn = None;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tls_api_init() {
    if tls_api_is_init == 0 {
        picoquic_tls_api_zero();
        picoquic_tls_api_init_providers(0 as ::core::ffi::c_int);
        tls_api_is_init = 1 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tls_api_unload() {
    if tls_api_is_init != 0 {
        picoquic_tls_api_init_providers(1 as ::core::ffi::c_int);
        picoquic_tls_api_zero();
        tls_api_is_init = 0 as ::core::ffi::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tls_api_reset(mut init_flags: uint64_t) {
    if tls_api_is_init != 0 {
        tls_api_is_init = 0 as ::core::ffi::c_int;
        picoquic_tls_api_init_providers(2 as ::core::ffi::c_int);
        picoquic_tls_api_zero();
    }
    tls_api_init_flags = init_flags;
    picoquic_tls_api_init_providers(0 as ::core::ffi::c_int);
    tls_api_is_init = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_register_ciphersuite(
    mut suite: *const ptls_cipher_suite_t,
    mut is_low_memory: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < PICOQUIC_CIPHER_SUITES_NB_MAX {
        if picoquic_cipher_suites[i as usize]
            .high_memory_suite
            .is_null()
            || (*picoquic_cipher_suites[i as usize].high_memory_suite).id as ::core::ffi::c_int
                == (*suite).id as ::core::ffi::c_int
        {
            picoquic_cipher_suites[i as usize].high_memory_suite = suite;
            if is_low_memory != 0 {
                picoquic_cipher_suites[i as usize].low_memory_suite = suite;
            }
            break;
        } else {
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_register_key_exchange_algorithm(
    mut key_exchange: *const ptls_key_exchange_algorithm_t,
) {
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < PICOQUIC_KEY_EXCHANGES_NB_MAX {
        if picoquic_key_exchanges[i as usize].is_null()
            || (*picoquic_key_exchanges[i as usize]).id as ::core::ffi::c_int
                == (*key_exchange).id as ::core::ffi::c_int
        {
            picoquic_key_exchanges[i as usize] = key_exchange;
            break;
        } else {
            i += 1;
        }
    }
    if (*key_exchange).id as ::core::ffi::c_int == PICOQUIC_GROUP_SECP256R1 {
        picoquic_key_exchange_secp256r1[0 as ::core::ffi::c_int as usize] = key_exchange;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_register_tls_key_provider_fn(
    mut set_key_from_key_file_fn: picoquic_set_private_key_from_file_t,
    mut dispose_sign_certificate_fn: picoquic_dispose_sign_certificate_t,
    mut get_certs_from_file_fn: picoquic_get_certs_from_file_t,
) {
    picoquic_set_private_key_from_file_fn = set_key_from_key_file_fn;
    picoquic_dispose_sign_certificate_fn = dispose_sign_certificate_fn;
    picoquic_get_certs_from_file_fn = get_certs_from_file_fn;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_register_verify_certificate_fn(
    mut certificate_verifier_fn: picoquic_get_certificate_verifier_t,
    mut dispose_certificate_verifier_fn: picoquic_dispose_certificate_verifier_t,
    mut set_tls_root_certificates_fn: picoquic_set_tls_root_certificates_t,
) {
    picoquic_get_certificate_verifier_fn = certificate_verifier_fn;
    picoquic_dispose_certificate_verifier_fn = dispose_certificate_verifier_fn;
    picoquic_set_tls_root_certificates_fn = set_tls_root_certificates_fn;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_register_explain_crypto_error_fn(
    mut explain_crypto_error_fn: picoquic_explain_crypto_error_t,
    mut clear_crypto_errors_fn: picoquic_clear_crypto_errors_t,
) {
    picoquic_explain_crypto_error_fn = explain_crypto_error_fn;
    picoquic_clear_crypto_errors_fn = clear_crypto_errors_fn;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_register_crypto_random_provider_fn(
    mut crypto_random_provider_fn: picoquic_crypto_random_provider_t,
) {
    picoquic_crypto_random_provider_fn = crypto_random_provider_fn;
}
unsafe extern "C" fn picoquic_set_cipher_suite_list(
    mut selected_suites: *mut *const ptls_cipher_suite_t,
    mut cipher_suite_id: ::core::ffi::c_int,
    mut use_low_memory: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut nb_suites: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < PICOQUIC_CIPHER_SUITES_NB_MAX && nb_suites < 4 as ::core::ffi::c_int {
        if picoquic_cipher_suites[i as usize]
            .high_memory_suite
            .is_null()
        {
            break;
        }
        if cipher_suite_id == 0 as ::core::ffi::c_int
            || cipher_suite_id
                == (*picoquic_cipher_suites[i as usize].high_memory_suite).id as ::core::ffi::c_int
        {
            if use_low_memory != 0 {
                if !picoquic_cipher_suites[i as usize]
                    .low_memory_suite
                    .is_null()
                {
                    let c2rust_fresh3 = nb_suites;
                    nb_suites = nb_suites + 1;
                    let ref mut c2rust_fresh4 = *selected_suites.offset(c2rust_fresh3 as isize);
                    *c2rust_fresh4 = picoquic_cipher_suites[i as usize].low_memory_suite;
                }
            } else {
                let c2rust_fresh5 = nb_suites;
                nb_suites = nb_suites + 1;
                let ref mut c2rust_fresh6 = *selected_suites.offset(c2rust_fresh5 as isize);
                *c2rust_fresh6 = picoquic_cipher_suites[i as usize].high_memory_suite;
            }
        }
        i += 1;
    }
    return nb_suites;
}
unsafe extern "C" fn picoquic_set_cipher_suite_in_ctx(
    mut ctx: *mut ptls_context_t,
    mut cipher_suite_id: ::core::ffi::c_int,
    mut use_low_memory: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut selected_suites: *mut *const ptls_cipher_suite_t = malloc(
        (::core::mem::size_of::<*const ptls_cipher_suite_t>() as size_t).wrapping_mul(4 as size_t),
    )
        as *mut *const ptls_cipher_suite_t;
    let mut nb_suites: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(*ctx).cipher_suites.is_null() {
        free((*ctx).cipher_suites as *mut ::core::ffi::c_void);
    }
    if ctx.is_null() || selected_suites.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        nb_suites =
            picoquic_set_cipher_suite_list(selected_suites, cipher_suite_id, use_low_memory);
        if nb_suites == 0 as ::core::ffi::c_int {
            (*ctx).cipher_suites = ::core::ptr::null_mut::<*const ptls_cipher_suite_t>();
            ret = -(1 as ::core::ffi::c_int);
        } else {
            while nb_suites < 4 as ::core::ffi::c_int {
                let c2rust_fresh1 = nb_suites;
                nb_suites = nb_suites + 1;
                let ref mut c2rust_fresh2 = *selected_suites.offset(c2rust_fresh1 as isize);
                *c2rust_fresh2 = ::core::ptr::null::<ptls_cipher_suite_t>();
            }
            (*ctx).cipher_suites = selected_suites as *mut *const ptls_cipher_suite_t;
        }
    }
    if ret != 0 as ::core::ffi::c_int && !selected_suites.is_null() {
        free(selected_suites as *mut ::core::ffi::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_cipher_suite(
    mut quic: *mut picoquic_quic_t,
    mut cipher_suite_id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ctx: *mut ptls_context_t = (*quic).tls_master_ctx as *mut ptls_context_t;
    return picoquic_set_cipher_suite_in_ctx(
        ctx,
        cipher_suite_id,
        (*quic).use_low_memory() as ::core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_selected_cipher_suite_by_id(
    mut cipher_suite_id: ::core::ffi::c_int,
    mut use_low_memory: ::core::ffi::c_int,
) -> *const ptls_cipher_suite_t {
    let mut selected_suites: [*const ptls_cipher_suite_t; 4] =
        [::core::ptr::null::<ptls_cipher_suite_t>(); 4];
    let mut cipher: *const ptls_cipher_suite_t = ::core::ptr::null::<ptls_cipher_suite_t>();
    let mut nb_suites: ::core::ffi::c_int = picoquic_set_cipher_suite_list(
        &raw mut selected_suites as *mut *const ptls_cipher_suite_t,
        cipher_suite_id,
        use_low_memory,
    );
    if nb_suites <= 0 as ::core::ffi::c_int {
        cipher = ::core::ptr::null::<ptls_cipher_suite_t>();
    } else {
        cipher = selected_suites[0 as ::core::ffi::c_int as usize];
    }
    return cipher;
}
unsafe extern "C" fn picoquic_get_cipher_suite_by_id(
    mut cipher_suite_id: ::core::ffi::c_int,
    mut use_low_memory: ::core::ffi::c_int,
) -> *const ptls_cipher_suite_t {
    return picoquic_get_selected_cipher_suite_by_id(cipher_suite_id, use_low_memory);
}
unsafe extern "C" fn picoquic_get_ecb_cipher_by_id(
    mut ecb_cipher_name: *const ::core::ffi::c_char,
) -> *const ptls_cipher_algorithm_t {
    let mut ecb_cipher: *const ptls_cipher_algorithm_t =
        ::core::ptr::null::<ptls_cipher_algorithm_t>();
    let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while j < 2 as ::core::ffi::c_int && ecb_cipher.is_null() {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < PICOQUIC_CIPHER_SUITES_NB_MAX && ecb_cipher.is_null() {
            let mut suite: *const ptls_cipher_suite_t = if j == 0 as ::core::ffi::c_int {
                picoquic_cipher_suites[i as usize].high_memory_suite
            } else {
                picoquic_cipher_suites[i as usize].low_memory_suite
            };
            if !suite.is_null()
                && !(*suite).aead.is_null()
                && !(*(*suite).aead).ecb_cipher.is_null()
                && strcmp((*(*(*suite).aead).ecb_cipher).name, ecb_cipher_name)
                    == 0 as ::core::ffi::c_int
            {
                ecb_cipher = (*(*suite).aead).ecb_cipher;
                break;
            } else {
                i += 1;
            }
        }
        j += 1;
    }
    return ecb_cipher;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_aes128_ecb_create(
    mut is_enc: ::core::ffi::c_int,
    mut ecb_key: *const ::core::ffi::c_void,
) -> *mut ::core::ffi::c_void {
    let mut created: *mut ::core::ffi::c_void = NULL_0;
    let mut ecb_cipher: *const ptls_cipher_algorithm_t =
        picoquic_get_ecb_cipher_by_id(b"AES128-ECB\0".as_ptr() as *const ::core::ffi::c_char);
    if !ecb_cipher.is_null() {
        created = ptls_cipher_new(ecb_cipher, is_enc, ecb_key) as *mut ::core::ffi::c_void;
    }
    return created;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_hash_algorithm_by_name(
    mut hash_algorithm_name: *const ::core::ffi::c_char,
) -> *const ptls_hash_algorithm_t {
    let mut hash: *const ptls_hash_algorithm_t = ::core::ptr::null::<ptls_hash_algorithm_t>();
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < PICOQUIC_CIPHER_SUITES_NB_MAX && hash.is_null() {
        if picoquic_cipher_suites[i as usize]
            .high_memory_suite
            .is_null()
        {
            break;
        }
        if strcmp(
            (*(*picoquic_cipher_suites[i as usize].high_memory_suite).hash).name,
            hash_algorithm_name,
        ) == 0 as ::core::ffi::c_int
        {
            hash = (*picoquic_cipher_suites[i as usize].high_memory_suite).hash;
            break;
        } else {
            i += 1;
        }
    }
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_sha256() -> *const ptls_hash_algorithm_t {
    return picoquic_get_hash_algorithm_by_name(b"sha256\0".as_ptr() as *const ::core::ffi::c_char);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_sha256_v() -> *mut ::core::ffi::c_void {
    return picoquic_get_sha256() as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_hash_create(
    mut algorithm_name: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    let mut ctx: *mut ptls_hash_context_t = ::core::ptr::null_mut::<ptls_hash_context_t>();
    let mut hash: *const ptls_hash_algorithm_t =
        picoquic_get_hash_algorithm_by_name(algorithm_name);
    if !hash.is_null() {
        ctx = (*hash).create.expect("non-null function pointer")();
    }
    return ctx as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_hash_get_length(
    mut algorithm_name: *const ::core::ffi::c_char,
) -> size_t {
    let mut len: size_t = 0 as size_t;
    let mut hash: *const ptls_hash_algorithm_t =
        picoquic_get_hash_algorithm_by_name(algorithm_name);
    if !hash.is_null() {
        len = (*hash).digest_size;
    }
    return len;
}
unsafe extern "C" fn picoquic_set_key_exchange_in_ctx(
    mut ctx: *mut ptls_context_t,
    mut key_exchange_id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    match key_exchange_id {
        0 => {
            (*ctx).key_exchanges =
                &raw mut picoquic_key_exchanges as *mut *const ptls_key_exchange_algorithm_t;
        }
        PICOQUIC_GROUP_SECP256R1 => {
            if picoquic_key_exchange_secp256r1[0 as ::core::ffi::c_int as usize].is_null() {
                ret = -(1 as ::core::ffi::c_int);
            } else {
                (*ctx).key_exchanges = &raw mut picoquic_key_exchange_secp256r1
                    as *mut *const ptls_key_exchange_algorithm_t;
            }
        }
        _ => {
            ret = -(1 as ::core::ffi::c_int);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_key_exchange(
    mut quic: *mut picoquic_quic_t,
    mut key_exchange_id: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ctx: *mut ptls_context_t = (*quic).tls_master_ctx as *mut ptls_context_t;
    ret = picoquic_set_key_exchange_in_ctx(ctx, key_exchange_id);
    return ret;
}
unsafe extern "C" fn picoquic_set_random_provider_in_ctx(mut ctx: *mut ptls_context_t) {
    (*ctx).random_bytes = picoquic_crypto_random_provider_fn
        as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
}
unsafe extern "C" fn set_private_key_from_file(
    mut keypem: *const ::core::ffi::c_char,
    mut ctx: *mut ptls_context_t,
) -> ::core::ffi::c_int {
    if picoquic_set_private_key_from_file_fn.is_none() {
        return -(1 as ::core::ffi::c_int);
    } else {
        return picoquic_set_private_key_from_file_fn.expect("non-null function pointer")(
            keypem, ctx,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_private_key_from_file(
    mut quic: *mut picoquic_quic_t,
    mut file_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    return set_private_key_from_file(file_name, (*quic).tls_master_ctx as *mut ptls_context_t);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_dispose_sign_certificate(mut ctx: *mut ptls_context_t) {
    if !(*ctx).sign_certificate.is_null() {
        if picoquic_dispose_sign_certificate_fn.is_some() {
            picoquic_dispose_sign_certificate_fn.expect("non-null function pointer")(
                (*ctx).sign_certificate,
            );
        }
        free((*ctx).sign_certificate as *mut ::core::ffi::c_void);
        (*ctx).sign_certificate = ::core::ptr::null_mut::<ptls_sign_certificate_t>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_certs_from_file(
    mut file_name: *const ::core::ffi::c_char,
    mut count: *mut size_t,
) -> *mut ptls_iovec_t {
    if picoquic_get_certs_from_file_fn.is_none() {
        return ::core::ptr::null_mut::<ptls_iovec_t>();
    } else {
        return picoquic_get_certs_from_file_fn.expect("non-null function pointer")(
            file_name, count,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_certificate_verifier(
    mut cert_root_file_name: *const ::core::ffi::c_char,
    mut is_cert_store_not_empty: *mut ::core::ffi::c_uint,
    mut p_free_certificate_verifier_fn: *mut picoquic_free_verify_certificate_ctx,
) -> *mut ptls_verify_certificate_t {
    if picoquic_get_certificate_verifier_fn.is_none() {
        return ::core::ptr::null_mut::<ptls_verify_certificate_t>();
    } else {
        return picoquic_get_certificate_verifier_fn.expect("non-null function pointer")(
            cert_root_file_name,
            is_cert_store_not_empty,
            p_free_certificate_verifier_fn as *mut picoquic_dispose_certificate_verifier_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_dispose_certificate_verifier(
    mut verifier: *mut ptls_verify_certificate_t,
) {
    if picoquic_dispose_certificate_verifier_fn.is_some() {
        picoquic_dispose_certificate_verifier_fn.expect("non-null function pointer")(verifier);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_tls_root_certificates(
    mut quic: *mut picoquic_quic_t,
    mut certs: *mut ptls_iovec_t,
    mut count: size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if picoquic_set_tls_root_certificates_fn.is_some() {
        ret = picoquic_set_tls_root_certificates_fn.expect("non-null function pointer")(
            (*quic).tls_master_ctx as *mut ptls_context_t,
            certs,
            count,
        );
        if ret == 0 as ::core::ffi::c_int {
            (*quic).set_is_cert_store_not_empty(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_explain_crypto_error(
    mut err_file: *mut *const ::core::ffi::c_char,
    mut err_line: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if picoquic_explain_crypto_error_fn.is_some() {
        ret = picoquic_explain_crypto_error_fn.expect("non-null function pointer")(
            err_file, err_line,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_clear_crypto_errors() {
    if picoquic_clear_crypto_errors_fn.is_some() {
        ::core::mem::transmute::<_, fn()>(
            picoquic_clear_crypto_errors_fn.expect("non-null function pointer"),
        )();
    }
}
unsafe extern "C" fn picoquic_get_aes128gcm_sha256(
    mut use_low_memory: ::core::ffi::c_int,
) -> *const ptls_cipher_suite_t {
    return picoquic_get_cipher_suite_by_id(PICOQUIC_AES_128_GCM_SHA256, use_low_memory);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_aes128gcm_sha256_v(
    mut use_low_memory: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    return picoquic_get_aes128gcm_sha256(use_low_memory) as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_aes128gcm_v(
    mut use_low_memory: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut aead: *mut ::core::ffi::c_void = NULL_0;
    let mut cipher: *const ptls_cipher_suite_t = picoquic_get_aes128gcm_sha256(use_low_memory);
    if !cipher.is_null() {
        aead = (*cipher).aead as *mut ::core::ffi::c_void;
    }
    return aead;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_cipher_suite_by_id_v(
    mut cipher_suite_id: ::core::ffi::c_int,
    mut use_low_memory: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    return picoquic_get_cipher_suite_by_id(cipher_suite_id, use_low_memory)
        as *mut ::core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_hash_update(
    mut input: *mut uint8_t,
    mut input_length: size_t,
    mut hash_context: *mut ::core::ffi::c_void,
) {
    (*(hash_context as *mut ptls_hash_context_t))
        .update
        .expect("non-null function pointer")(
        hash_context as *mut st_ptls_hash_context_t,
        input as *const ::core::ffi::c_void,
        input_length,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_hash_finalize(
    mut output: *mut uint8_t,
    mut hash_context: *mut ::core::ffi::c_void,
) {
    (*(hash_context as *mut ptls_hash_context_t))
        .final_0
        .expect("non-null function pointer")(
        hash_context as *mut st_ptls_hash_context_t,
        output as *mut ::core::ffi::c_void,
        PTLS_HASH_FINAL_MODE_FREE,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_log_crypto_errors(
    mut cnx: *mut picoquic_cnx_t,
    mut ret: ::core::ffi::c_int,
) {
    let mut crypto_err: ::core::ffi::c_ulong = 0;
    let mut err_file: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut err_line: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    loop {
        crypto_err = picoquic_explain_crypto_error(&raw mut err_file, &raw mut err_line)
            as ::core::ffi::c_ulong;
        if !(crypto_err != 0 as ::core::ffi::c_ulong) {
            break;
        }
        picoquic_log_app_message(
            cnx as *mut picoquic_cnx_t,
            b"Crypto SSL error: %lu, file %s, line %d\0".as_ptr() as *const ::core::ffi::c_char,
            crypto_err,
            if err_file.is_null() {
                b"?\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                err_file
            },
            err_line,
        );
    }
    picoquic_log_app_message(
        cnx as *mut picoquic_cnx_t,
        b"Picotls returns error: %d (0x%x)\0".as_ptr() as *const ::core::ffi::c_char,
        ret,
        ret,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_crypto_random(
    mut quic: *mut picoquic_quic_t,
    mut buf: *mut ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut ctx: *mut ptls_context_t = (*quic).tls_master_ctx as *mut ptls_context_t;
    (*ctx).random_bytes.expect("non-null function pointer")(buf, len);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_crypto_uniform_random(
    mut quic: *mut picoquic_quic_t,
    mut rnd_max: uint64_t,
) -> uint64_t {
    let mut rnd: uint64_t = 0;
    let mut rnd_min: uint64_t = (UINT64_MAX as uint64_t).wrapping_rem(rnd_max);
    loop {
        picoquic_crypto_random(
            quic,
            &raw mut rnd as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<uint64_t>() as size_t,
        );
        if !(rnd < rnd_min) {
            break;
        }
    }
    return rnd.wrapping_rem(rnd_max);
}
static mut public_random_seed: [uint64_t; 16] = [
    1 as ::core::ffi::c_int as uint64_t,
    2 as ::core::ffi::c_int as uint64_t,
    3 as ::core::ffi::c_int as uint64_t,
    4 as ::core::ffi::c_int as uint64_t,
    5 as ::core::ffi::c_int as uint64_t,
    6 as ::core::ffi::c_int as uint64_t,
    7 as ::core::ffi::c_int as uint64_t,
    8 as ::core::ffi::c_int as uint64_t,
    9 as ::core::ffi::c_int as uint64_t,
    10 as ::core::ffi::c_int as uint64_t,
    11 as ::core::ffi::c_int as uint64_t,
    12 as ::core::ffi::c_int as uint64_t,
    13 as ::core::ffi::c_int as uint64_t,
    14 as ::core::ffi::c_int as uint64_t,
    15 as ::core::ffi::c_int as uint64_t,
    16 as ::core::ffi::c_int as uint64_t,
];
static mut public_random_index: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut public_random_multiplier: uint64_t = 1181783497276652981 as uint64_t;
static mut public_random_obfuscator: uint64_t = 0x5555555555555555 as uint64_t;
unsafe extern "C" fn picoquic_public_random_step() -> uint64_t {
    let mut s1: uint64_t = 0;
    let c2rust_fresh7 = public_random_index;
    public_random_index = public_random_index + 1;
    let s0: uint64_t = public_random_seed[c2rust_fresh7 as usize];
    public_random_index &= 15 as ::core::ffi::c_int;
    s1 = public_random_seed[public_random_index as usize];
    s1 ^= s1 << 31 as ::core::ffi::c_int;
    s1 ^= s1 >> 11 as ::core::ffi::c_int;
    s1 ^= s0 ^ s0 >> 30 as ::core::ffi::c_int;
    public_random_seed[public_random_index as usize] = s1;
    return s1;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_public_random_64() -> uint64_t {
    let mut s1: uint64_t = picoquic_public_random_step();
    s1 = s1.wrapping_mul(public_random_multiplier);
    s1 ^= public_random_obfuscator;
    return s1;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_public_random_seed_64(
    mut seed: uint64_t,
    mut reset: ::core::ffi::c_int,
) {
    if reset != 0 {
        public_random_index = 0 as ::core::ffi::c_int;
        let mut i: uint64_t = 0 as uint64_t;
        while i < 16 as uint64_t {
            public_random_seed[i as usize] = i.wrapping_add(1 as uint64_t);
            i = i.wrapping_add(1);
        }
        public_random_obfuscator = 0x5555555555555555 as uint64_t;
    }
    public_random_seed[public_random_index as usize] ^= seed;
    let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i_0 < 16 as ::core::ffi::c_int {
        picoquic_public_random_step();
        i_0 += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_public_random_seed(mut quic: *mut picoquic_quic_t) {
    let mut seed: [uint64_t; 3] = [0; 3];
    picoquic_crypto_random(
        quic,
        &raw mut seed as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint64_t; 3]>() as size_t,
    );
    picoquic_public_random_seed_64(
        seed[0 as ::core::ffi::c_int as usize],
        0 as ::core::ffi::c_int,
    );
    public_random_obfuscator = seed[1 as ::core::ffi::c_int as usize];
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_public_random(
    mut buf: *mut ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut x: *mut uint8_t = buf as *mut uint8_t;
    while len > 0 as size_t {
        let mut y: uint64_t = picoquic_public_random_64();
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int && len > 0 as size_t {
            let c2rust_fresh10 = x;
            x = x.offset(1);
            *c2rust_fresh10 = (y & 255 as uint64_t) as uint8_t;
            y >>= 8 as ::core::ffi::c_int;
            len = len.wrapping_sub(1);
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_public_uniform_random(mut rnd_max: uint64_t) -> uint64_t {
    let mut rnd: uint64_t = 0;
    let mut rnd_min: uint64_t = (UINT64_MAX as uint64_t).wrapping_rem(rnd_max);
    loop {
        rnd = picoquic_public_random_64();
        if !(rnd < rnd_min) {
            break;
        }
    }
    return rnd.wrapping_rem(rnd_max);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tls_get_quic_extension_id(
    mut cnx: *mut picoquic_cnx_t,
) -> uint16_t {
    let mut v: ::core::ffi::c_int = (*(&raw const picoquic_supported_versions
        as *const picoquic_version_parameters_t)
        .offset((*cnx).version_index as isize))
    .version as ::core::ffi::c_int;
    let mut quic_ext_id: uint16_t = PICOQUIC_TRANSPORT_PARAMETERS_TLS_EXTENSION_V1 as uint16_t;
    if v as ::core::ffi::c_uint == PICOQUIC_SEVENTEENTH_INTEROP_VERSION
        || v as ::core::ffi::c_uint == PICOQUIC_EIGHTEENTH_INTEROP_VERSION
        || v as ::core::ffi::c_uint == PICOQUIC_NINETEENTH_INTEROP_VERSION
        || v as ::core::ffi::c_uint == PICOQUIC_NINETEENTH_BIS_INTEROP_VERSION
        || v as ::core::ffi::c_uint == PICOQUIC_TWENTIETH_PRE_INTEROP_VERSION
        || v as ::core::ffi::c_uint == PICOQUIC_TWENTIETH_INTEROP_VERSION
        || v == PICOQUIC_INTERNAL_TEST_VERSION_1
        || v == PICOQUIC_INTERNAL_TEST_VERSION_2
    {
        quic_ext_id = PICOQUIC_TRANSPORT_PARAMETERS_TLS_EXTENSION_DRAFT as uint16_t;
    }
    return quic_ext_id;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tls_collect_extensions_cb(
    mut tls: *mut ptls_t,
    mut properties: *mut st_ptls_handshake_properties_t,
    mut type_0: uint16_t,
) -> ::core::ffi::c_int {
    let mut ctx: *mut picoquic_tls_ctx_t = (properties as *mut ::core::ffi::c_char)
        .offset(-(72 as ::core::ffi::c_ulong as isize))
        as *mut picoquic_tls_ctx_t;
    return picoquic_tls_get_quic_extension_id((*ctx).cnx) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tls_set_extensions(
    mut cnx: *mut picoquic_cnx_t,
    mut tls_ctx: *mut picoquic_tls_ctx_t,
) {
    let mut consumed: size_t = 0 as size_t;
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if !(*tls_ctx).ext_data.is_null() {
        ret = picoquic_prepare_transport_extensions(
            cnx,
            if (*tls_ctx).client_mode != 0 {
                0 as ::core::ffi::c_int
            } else {
                1 as ::core::ffi::c_int
            },
            (*tls_ctx).ext_data,
            (*tls_ctx).ext_data_size,
            &raw mut consumed,
        );
    }
    if ret == 0 as ::core::ffi::c_int {
        (*tls_ctx).ext[0 as ::core::ffi::c_int as usize].type_0 =
            picoquic_tls_get_quic_extension_id(cnx);
        (*tls_ctx).ext[0 as ::core::ffi::c_int as usize].data.base = (*tls_ctx).ext_data;
        (*tls_ctx).ext[0 as ::core::ffi::c_int as usize].data.len = consumed;
        (*tls_ctx).ext[1 as ::core::ffi::c_int as usize].type_0 = 0xffff as uint16_t;
        (*tls_ctx).ext[1 as ::core::ffi::c_int as usize].data.base =
            ::core::ptr::null_mut::<uint8_t>();
        (*tls_ctx).ext[1 as ::core::ffi::c_int as usize].data.len = 0 as size_t;
    } else {
        (*tls_ctx).ext[0 as ::core::ffi::c_int as usize].type_0 = 0xffff as uint16_t;
        (*tls_ctx).ext[0 as ::core::ffi::c_int as usize].data.base =
            ::core::ptr::null_mut::<uint8_t>();
        (*tls_ctx).ext[0 as ::core::ffi::c_int as usize].data.len = 0 as size_t;
    }
    (*tls_ctx).handshake_properties.additional_extensions =
        &raw mut (*tls_ctx).ext as *mut ptls_raw_extension_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tls_collected_extensions_cb(
    mut tls: *mut ptls_t,
    mut properties: *mut ptls_handshake_properties_t,
    mut slots: *mut ptls_raw_extension_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut consumed: size_t = 0 as size_t;
    let mut ctx: *mut picoquic_tls_ctx_t = (properties as *mut ::core::ffi::c_char)
        .offset(-(72 as ::core::ffi::c_ulong as isize))
        as *mut picoquic_tls_ctx_t;
    let mut i_slot: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while (*slots.offset(i_slot as isize)).type_0 as ::core::ffi::c_int
        != 0xffff as ::core::ffi::c_int
    {
        if (*slots.offset(i_slot as isize)).type_0 as ::core::ffi::c_int
            == picoquic_tls_get_quic_extension_id((*ctx).cnx) as ::core::ffi::c_int
        {
            ret = picoquic_receive_transport_extensions(
                (*ctx).cnx,
                if (*ctx).client_mode != 0 {
                    1 as ::core::ffi::c_int
                } else {
                    0 as ::core::ffi::c_int
                },
                (*slots.offset(i_slot as isize)).data.base,
                (*slots.offset(i_slot as isize)).data.len,
                &raw mut consumed,
            );
            ret = 0 as ::core::ffi::c_int;
            if (*ctx).client_mode == 0 as ::core::ffi::c_int {
                picoquic_tls_set_extensions((*ctx).cnx, ctx);
            }
        }
        i_slot += 1;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_client_hello_call_back(
    mut on_hello_cb_ctx: *mut ptls_on_client_hello_t,
    mut tls: *mut ptls_t,
    mut params: *mut ptls_on_client_hello_parameters_t,
) -> ::core::ffi::c_int {
    let mut alpn_found: *const uint8_t = ::core::ptr::null::<uint8_t>();
    let mut alpn_found_length: size_t = 0 as size_t;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ppquic: *mut *mut picoquic_quic_t = (on_hello_cb_ctx as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<ptls_on_client_hello_t>() as usize as isize)
        as *mut *mut picoquic_quic_t;
    let mut quic: *mut picoquic_quic_t = *ppquic;
    ptls_set_server_name(
        tls,
        (*params).server_name.base as *const ::core::ffi::c_char,
        (*params).server_name.len,
    );
    if !(*quic).default_alpn.is_null() {
        let mut len: size_t = strlen((*quic).default_alpn);
        let mut i: size_t = 0 as size_t;
        while i < (*params).negotiated_protocols.count {
            if (*(*params).negotiated_protocols.list.offset(i as isize)).len == len
                && memcmp(
                    (*(*params).negotiated_protocols.list.offset(i as isize)).base
                        as *const ::core::ffi::c_void,
                    (*quic).default_alpn as *const ::core::ffi::c_void,
                    len,
                ) == 0 as ::core::ffi::c_int
            {
                if !(*quic).cnx_in_progress.is_null() {
                    picoquic_log_app_message(
                        (*quic).cnx_in_progress as *mut picoquic_cnx_t,
                        b"ALPN[%d] matches default alpn (%s)\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        i as ::core::ffi::c_int,
                        (*quic).default_alpn,
                    );
                }
                alpn_found = (*quic).default_alpn as *const uint8_t;
                alpn_found_length = len;
                ptls_set_negotiated_protocol(tls, (*quic).default_alpn, len);
                break;
            } else {
                i = i.wrapping_add(1);
            }
        }
    } else if (*quic).alpn_select_fn.is_some() {
        let mut selected: size_t = (*quic).alpn_select_fn.expect("non-null function pointer")(
            quic as *mut picoquic_quic_t,
            (*params).negotiated_protocols.list as *mut ptls_iovec_t,
            (*params).negotiated_protocols.count,
        );
        if selected < (*params).negotiated_protocols.count {
            alpn_found = (*(*params)
                .negotiated_protocols
                .list
                .offset(selected as isize))
            .base;
            alpn_found_length = (*(*params)
                .negotiated_protocols
                .list
                .offset(selected as isize))
            .len;
            ptls_set_negotiated_protocol(
                tls,
                (*(*params)
                    .negotiated_protocols
                    .list
                    .offset(selected as isize))
                .base as *const ::core::ffi::c_char,
                (*(*params)
                    .negotiated_protocols
                    .list
                    .offset(selected as isize))
                .len,
            );
        }
    }
    if !(*quic).cnx_in_progress.is_null() {
        if (*(*quic).cnx_in_progress).alpn.is_null() && alpn_found_length > 0 as size_t {
            (*(*quic).cnx_in_progress).alpn =
                picoquic_string_create(alpn_found as *const ::core::ffi::c_char, alpn_found_length);
        }
        picoquic_log_negotiated_alpn(
            (*quic).cnx_in_progress as *mut picoquic_cnx_t,
            0 as ::core::ffi::c_int,
            (*params).server_name.base,
            (*params).server_name.len,
            alpn_found,
            alpn_found_length,
            (*params).negotiated_protocols.list,
            (*params).negotiated_protocols.count,
        );
    }
    if alpn_found.is_null() {
        ret = PTLS_ALERT_NO_APPLICATION_PROTOCOL;
    }
    if ret != 0 as ::core::ffi::c_int && !(*quic).cnx_in_progress.is_null() {
        picoquic_log_app_message(
            (*quic).cnx_in_progress as *mut picoquic_cnx_t,
            b"Client Hello call back returns %d (0x%x)\0".as_ptr() as *const ::core::ffi::c_char,
            ret,
            ret,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_server_encrypt_ticket_call_back(
    mut encrypt_ticket_ctx: *mut ptls_encrypt_ticket_t,
    mut tls: *mut ptls_t,
    mut is_encrypt: ::core::ffi::c_int,
    mut dst: *mut ptls_buffer_t,
    mut src: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ppquic: *mut *mut picoquic_quic_t = (encrypt_ticket_ctx as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<ptls_encrypt_ticket_t>() as usize as isize)
        as *mut *mut picoquic_quic_t;
    let mut quic: *mut picoquic_quic_t = *ppquic;
    if is_encrypt != 0 as ::core::ffi::c_int {
        let mut aead_enc: *mut ptls_aead_context_t =
            (*quic).aead_encrypt_ticket_ctx as *mut ptls_aead_context_t;
        if aead_enc.is_null() {
            ret = -(1 as ::core::ffi::c_int);
        } else {
            ret = ptls_buffer_reserve(
                dst,
                ((8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as size_t)
                    .wrapping_add(src.len)
                    .wrapping_add((*(*aead_enc).algo).tag_size),
            );
            if ret == 0 as ::core::ffi::c_int {
                let mut version_number: uint32_t = (*(&raw const picoquic_supported_versions
                    as *const picoquic_version_parameters_t)
                    .offset((*(*quic).cnx_in_progress).version_index as isize))
                .version;
                let mut seq_num: uint64_t = picoquic_public_random_64();
                let mut start_off: size_t = 0;
                let mut data_length: size_t = 0;
                picoformat_64((*dst).base.offset((*dst).off as isize), seq_num);
                (*dst).off = (*dst).off.wrapping_add(8 as size_t);
                start_off = (*dst).off;
                memcpy(
                    (*dst).base.offset((*dst).off as isize) as *mut ::core::ffi::c_void,
                    src.base as *const ::core::ffi::c_void,
                    src.len,
                );
                data_length = src.len;
                picoformat_32(
                    (*dst)
                        .base
                        .offset((*dst).off as isize)
                        .offset(data_length as isize),
                    version_number,
                );
                data_length = data_length.wrapping_add(4 as size_t);
                (*dst).off = (*dst).off.wrapping_add(ptls_aead_encrypt(
                    aead_enc,
                    (*dst).base.offset((*dst).off as isize) as *mut ::core::ffi::c_void,
                    (*dst).base.offset(start_off as isize) as *const ::core::ffi::c_void,
                    data_length,
                    seq_num,
                    ::core::ptr::null::<::core::ffi::c_void>(),
                    0 as size_t,
                ));
                (*(*quic).cnx_in_progress).issued_ticket_id = seq_num;
            }
        }
    } else {
        let mut aead_dec: *mut ptls_aead_context_t =
            (*quic).aead_decrypt_ticket_ctx as *mut ptls_aead_context_t;
        if aead_dec.is_null() {
            ret = -(1 as ::core::ffi::c_int);
        } else if src.len
            < ((8 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as size_t)
                .wrapping_add((*(*aead_dec).algo).tag_size)
        {
            ret = -(1 as ::core::ffi::c_int);
        } else {
            ret = ptls_buffer_reserve(dst, src.len);
            if ret == 0 as ::core::ffi::c_int {
                let mut seq_num_0: uint64_t =
                    (((((*src.base.offset(0 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *src.base.offset(1 as ::core::ffi::c_int as isize) as uint16_t
                            as ::core::ffi::c_int) as uint32_t)
                        << 16 as ::core::ffi::c_int
                        | ((*src
                            .base
                            .offset(2 as ::core::ffi::c_int as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as uint16_t as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int
                            | *src
                                .base
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as uint16_t as ::core::ffi::c_int)
                            as uint32_t) as uint64_t)
                        << 32 as ::core::ffi::c_int
                        | ((((*src
                            .base
                            .offset(4 as ::core::ffi::c_int as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as uint16_t as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int
                            | *src
                                .base
                                .offset(4 as ::core::ffi::c_int as isize)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as uint16_t as ::core::ffi::c_int)
                            as uint32_t)
                            << 16 as ::core::ffi::c_int
                            | ((*src
                                .base
                                .offset(4 as ::core::ffi::c_int as isize)
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset(0 as ::core::ffi::c_int as isize)
                                as uint16_t as ::core::ffi::c_int)
                                << 8 as ::core::ffi::c_int
                                | *src
                                    .base
                                    .offset(4 as ::core::ffi::c_int as isize)
                                    .offset(2 as ::core::ffi::c_int as isize)
                                    .offset(1 as ::core::ffi::c_int as isize)
                                    as uint16_t
                                    as ::core::ffi::c_int)
                                as uint32_t) as uint64_t;
                let mut decrypted: size_t = ptls_aead_decrypt(
                    aead_dec,
                    (*dst).base.offset((*dst).off as isize) as *mut ::core::ffi::c_void,
                    src.base.offset(8 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                    src.len.wrapping_sub(8 as size_t),
                    seq_num_0,
                    ::core::ptr::null::<::core::ffi::c_void>(),
                    0 as size_t,
                );
                if decrypted > src.len.wrapping_sub(8 as size_t) {
                    ret = -(1 as ::core::ffi::c_int);
                    picoquic_log_app_message(
                        (*quic).cnx_in_progress as *mut picoquic_cnx_t,
                        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                        b"Session ticket could not be decrypted\0".as_ptr()
                            as *const ::core::ffi::c_char,
                    );
                } else {
                    let mut version_number_0: uint32_t = (((*(*dst)
                        .base
                        .offset((*dst).off as isize)
                        .offset(decrypted as isize)
                        .offset(-(4 as ::core::ffi::c_int as isize))
                        .offset(0 as ::core::ffi::c_int as isize)
                        as uint16_t
                        as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *(*dst)
                            .base
                            .offset((*dst).off as isize)
                            .offset(decrypted as isize)
                            .offset(-(4 as ::core::ffi::c_int as isize))
                            .offset(1 as ::core::ffi::c_int as isize)
                            as uint16_t as ::core::ffi::c_int)
                        as uint32_t)
                        << 16 as ::core::ffi::c_int
                        | ((*(*dst)
                            .base
                            .offset((*dst).off as isize)
                            .offset(decrypted as isize)
                            .offset(-(4 as ::core::ffi::c_int as isize))
                            .offset(2 as ::core::ffi::c_int as isize)
                            .offset(0 as ::core::ffi::c_int as isize)
                            as uint16_t as ::core::ffi::c_int)
                            << 8 as ::core::ffi::c_int
                            | *(*dst)
                                .base
                                .offset((*dst).off as isize)
                                .offset(decrypted as isize)
                                .offset(-(4 as ::core::ffi::c_int as isize))
                                .offset(2 as ::core::ffi::c_int as isize)
                                .offset(1 as ::core::ffi::c_int as isize)
                                as uint16_t as ::core::ffi::c_int)
                            as uint32_t;
                    if version_number_0
                        != (*(&raw const picoquic_supported_versions
                            as *const picoquic_version_parameters_t)
                            .offset((*(*quic).cnx_in_progress).version_index as isize))
                        .version
                    {
                        ret = -(1 as ::core::ffi::c_int);
                        picoquic_log_app_message(
                            (*quic).cnx_in_progress as *mut picoquic_cnx_t,
                            b"Ticket version mismatch, expected 0x%x, got 0x%x\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            (*(&raw const picoquic_supported_versions
                                as *const picoquic_version_parameters_t)
                                .offset((*(*quic).cnx_in_progress).version_index as isize))
                            .version,
                            version_number_0,
                        );
                    } else {
                        let mut server_ticket: *mut picoquic_issued_ticket_t =
                            ::core::ptr::null_mut::<picoquic_issued_ticket_t>();
                        (*dst).off = (*dst).off.wrapping_add(decrypted.wrapping_sub(4 as size_t));
                        picoquic_log_app_message(
                            (*quic).cnx_in_progress as *mut picoquic_cnx_t,
                            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                            b"Session ticket properly decrypted\0".as_ptr()
                                as *const ::core::ffi::c_char,
                        );
                        (*(*quic).cnx_in_progress).resumed_ticket_id = seq_num_0;
                        server_ticket = picoquic_retrieve_issued_ticket(
                            quic as *mut picoquic_quic_t,
                            seq_num_0,
                        );
                        if !server_ticket.is_null() && (*server_ticket).cwin > 0 as uint64_t {
                            picoquic_seed_bandwidth(
                                (*quic).cnx_in_progress as *mut picoquic_cnx_t,
                                (*server_ticket).rtt,
                                (*server_ticket).cwin,
                                &raw mut (*server_ticket).ip_addr as *mut uint8_t,
                                (*server_ticket).ip_addr_length,
                            );
                        }
                    }
                }
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_client_save_ticket_call_back(
    mut save_ticket_ctx: *mut ptls_save_ticket_t,
    mut tls: *mut ptls_t,
    mut input: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut quic: *mut picoquic_quic_t = *((save_ticket_ctx as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<ptls_save_ticket_t>() as usize as isize)
        as *mut *mut picoquic_quic_t);
    let mut sni: *const ::core::ffi::c_char = ptls_get_server_name(tls);
    let mut alpn: *const ::core::ffi::c_char = ptls_get_negotiated_protocol(tls);
    let mut cnx: *mut picoquic_cnx_t = *ptls_get_data_ptr(tls) as *mut picoquic_cnx_t;
    let mut version: uint32_t = (*(&raw const picoquic_supported_versions
        as *const picoquic_version_parameters_t)
        .offset((*cnx).version_index as isize))
    .version;
    if alpn.is_null() && !quic.is_null() {
        alpn = (*quic).default_alpn;
    }
    if !sni.is_null() && !alpn.is_null() {
        ret = picoquic_store_ticket(
            quic as *mut picoquic_quic_t,
            sni,
            strlen(sni) as uint16_t,
            alpn,
            strlen(alpn) as uint16_t,
            version,
            ::core::ptr::null::<uint8_t>(),
            0 as uint8_t,
            ::core::ptr::null::<uint8_t>(),
            0 as uint8_t,
            input.base,
            input.len as uint16_t,
            &raw mut (*cnx).remote_parameters,
        );
        if input.len > 8 as size_t {
            (*cnx).issued_ticket_id = (((((*input.base.offset(0 as ::core::ffi::c_int as isize)
                as uint16_t as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *input.base.offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t)
                << 16 as ::core::ffi::c_int
                | ((*input
                    .base
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *input
                        .base
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int) as uint32_t)
                as uint64_t)
                << 32 as ::core::ffi::c_int
                | ((((*input
                    .base
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *input
                        .base
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int) as uint32_t)
                    << 16 as ::core::ffi::c_int
                    | ((*input
                        .base
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as uint16_t as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *input
                            .base
                            .offset(4 as ::core::ffi::c_int as isize)
                            .offset(2 as ::core::ffi::c_int as isize)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as uint16_t as ::core::ffi::c_int) as uint32_t)
                    as uint64_t;
        }
    } else {
        picoquic_log_app_message(
            cnx as *mut picoquic_cnx_t,
            b"Received incorrect session resume ticket, sni = %s, alpn = %s, length = %d\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            if sni.is_null() {
                b"NULL\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                sni
            },
            if alpn.is_null() {
                b"NULL\0".as_ptr() as *const ::core::ffi::c_char
            } else {
                alpn
            },
            input.len as ::core::ffi::c_int,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_simulated_time_cb(
    mut self_0: *mut ptls_get_time_t,
) -> uint64_t {
    let mut pp_simulated_time: *mut *mut uint64_t = (self_0 as *mut ::core::ffi::c_char)
        .offset(::core::mem::size_of::<ptls_get_time_t>() as usize as isize)
        as *mut *mut uint64_t;
    return (**pp_simulated_time).wrapping_div(1000 as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_enable_custom_verify_certificate_callback(
    mut quic: *mut picoquic_quic_t,
) -> ::core::ffi::c_int {
    let mut ctx: *mut ptls_context_t = (*quic).tls_master_ctx as *mut ptls_context_t;
    (*ctx).verify_certificate =
        (*quic).verify_certificate_callback as *mut ptls_verify_certificate_t;
    (*quic).set_is_cert_store_not_empty(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_dispose_verify_certificate_callback(
    mut quic: *mut picoquic_quic_t,
) {
    let mut ctx: *mut ptls_context_t = (*quic).tls_master_ctx as *mut ptls_context_t;
    if !(*ctx).verify_certificate.is_null() {
        if (*quic).free_verify_certificate_callback_fn.is_some() {
            let mut disposer: picoquic_dispose_certificate_verifier_t =
                (*quic).free_verify_certificate_callback_fn;
            disposer.expect("non-null function pointer")(
                (*ctx).verify_certificate as *mut ptls_verify_certificate_t,
            );
            (*quic).free_verify_certificate_callback_fn = None;
        }
        (*ctx).verify_certificate = ::core::ptr::null_mut::<ptls_verify_certificate_t>();
    }
    (*ctx).verify_certificate = ::core::ptr::null_mut::<ptls_verify_certificate_t>();
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tls_set_verify_certificate_callback(
    mut quic: *mut picoquic_quic_t,
    mut cb: *mut st_ptls_verify_certificate_t,
    mut free_fn: picoquic_free_verify_certificate_ctx,
) {
    let mut ctx: *mut ptls_context_t = (*quic).tls_master_ctx as *mut ptls_context_t;
    picoquic_dispose_verify_certificate_callback(quic);
    (*ctx).verify_certificate = cb as *mut ptls_verify_certificate_t;
    (*quic).set_is_cert_store_not_empty(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    (*quic).free_verify_certificate_callback_fn = free_fn;
}
unsafe extern "C" fn picoquic_set_aead_from_secret(
    mut v_aead: *mut *mut ::core::ffi::c_void,
    mut cipher: *const ptls_cipher_suite_t,
    mut is_enc: ::core::ffi::c_int,
    mut secret: *const ::core::ffi::c_void,
    mut prefix_label: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !(*v_aead).is_null() {
        ptls_aead_free(*v_aead as *mut ptls_aead_context_t);
    }
    *v_aead = ptls_aead_new((*cipher).aead, (*cipher).hash, is_enc, secret, prefix_label)
        as *mut ::core::ffi::c_void;
    if (*v_aead).is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    }
    return ret;
}
unsafe extern "C" fn picoquic_set_pn_enc_from_secret(
    mut v_pn_enc: *mut *mut ::core::ffi::c_void,
    mut cipher: *const ptls_cipher_suite_t,
    mut is_enc: ::core::ffi::c_int,
    mut secret: *const ::core::ffi::c_void,
    mut prefix_label: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pnekey: [uint8_t; 32] = [0; 32];
    let mut ret: ::core::ffi::c_int = 0;
    if !(*v_pn_enc).is_null() {
        ptls_cipher_free(*v_pn_enc as *mut ptls_cipher_context_t);
        *v_pn_enc = NULL_0;
    }
    ret = ptls_hkdf_expand_label(
        (*cipher).hash,
        &raw mut pnekey as *mut uint8_t as *mut ::core::ffi::c_void,
        (*(*(*cipher).aead).ctr_cipher).key_size,
        ptls_iovec_init(secret, (*(*cipher).hash).digest_size),
        PICOQUIC_LABEL_HP.as_ptr(),
        ptls_iovec_init(::core::ptr::null::<::core::ffi::c_void>(), 0 as size_t),
        prefix_label,
    );
    if ret == 0 as ::core::ffi::c_int {
        *v_pn_enc = ptls_cipher_new(
            (*(*cipher).aead).ctr_cipher,
            is_enc,
            &raw mut pnekey as *mut uint8_t as *const ::core::ffi::c_void,
        ) as *mut ::core::ffi::c_void;
        if (*v_pn_enc).is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_aes128_ecb_free(mut v_aesecb: *mut ::core::ffi::c_void) {
    ptls_cipher_free(v_aesecb as *mut ptls_cipher_context_t);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_aes128_ecb_encrypt(
    mut v_aesecb: *mut ::core::ffi::c_void,
    mut output: *mut uint8_t,
    mut input: *const uint8_t,
    mut len: size_t,
) {
    ptls_cipher_encrypt(
        v_aesecb as *mut ptls_cipher_context_t,
        output as *mut ::core::ffi::c_void,
        input as *const ::core::ffi::c_void,
        len,
    );
}
unsafe extern "C" fn picoquic_set_key_from_secret(
    mut cipher: *const ptls_cipher_suite_t,
    mut is_enc: ::core::ffi::c_int,
    mut is_rotation: ::core::ffi::c_int,
    mut ctx: *mut picoquic_crypto_context_t,
    mut secret: *const ::core::ffi::c_void,
    mut prefix_label: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if is_enc != 0 as ::core::ffi::c_int {
        ret = picoquic_set_aead_from_secret(
            &raw mut (*ctx).aead_encrypt,
            cipher,
            is_enc,
            secret,
            prefix_label,
        );
        if ret == 0 as ::core::ffi::c_int && is_rotation == 0 {
            ret = picoquic_set_pn_enc_from_secret(
                &raw mut (*ctx).pn_enc,
                cipher,
                is_enc,
                secret,
                prefix_label,
            );
        }
    } else {
        ret = picoquic_set_aead_from_secret(
            &raw mut (*ctx).aead_decrypt,
            cipher,
            is_enc,
            secret,
            prefix_label,
        );
        if ret == 0 as ::core::ffi::c_int && is_rotation == 0 {
            ret = picoquic_set_pn_enc_from_secret(
                &raw mut (*ctx).pn_dec,
                cipher,
                is_enc,
                secret,
                prefix_label,
            );
        }
    }
    return ret;
}
unsafe extern "C" fn picoquic_update_traffic_key_callback(
    mut self_0: *mut ptls_update_traffic_key_t,
    mut tls: *mut ptls_t,
    mut is_enc: ::core::ffi::c_int,
    mut epoch: size_t,
    mut secret: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut cnx: *mut picoquic_cnx_t = *ptls_get_data_ptr(tls) as *mut picoquic_cnx_t;
    let mut tls_ctx: *mut picoquic_tls_ctx_t = (*cnx).tls_ctx as *mut picoquic_tls_ctx_t;
    let mut ctx: *mut ptls_context_t = (*(*cnx).quic).tls_master_ctx as *mut ptls_context_t;
    let mut cipher: *const ptls_cipher_suite_t = ptls_get_cipher(tls) as *const ptls_cipher_suite_t;
    let mut prefix_label: *const ::core::ffi::c_char = (*(&raw const picoquic_supported_versions
        as *const picoquic_version_parameters_t)
        .offset((*cnx).version_index as isize))
    .tls_prefix_label;
    let mut ret: ::core::ffi::c_int = picoquic_set_key_from_secret(
        cipher,
        is_enc,
        0 as ::core::ffi::c_int,
        (&raw mut (*cnx).crypto_context as *mut picoquic_crypto_context_t).offset(epoch as isize)
            as *mut picoquic_crypto_context_t,
        secret,
        prefix_label,
    );
    if ((*cnx).cnx_state as ::core::ffi::c_uint)
        < picoquic_state_ready as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*cnx).set_recycle_sooner_needed(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if ret == 0 as ::core::ffi::c_int && epoch == 3 as size_t {
        memcpy(
            (if is_enc != 0 {
                &raw mut (*tls_ctx).app_secret_enc as *mut uint8_t
            } else {
                &raw mut (*tls_ctx).app_secret_dec as *mut uint8_t
            }) as *mut ::core::ffi::c_void,
            secret,
            (*(*cipher).hash).digest_size,
        );
    }
    if !(*ctx).log_event.is_null() {
        let mut hexbuf: [::core::ffi::c_char; 129] = [0; 129];
        static mut log_labels: [[*const ::core::ffi::c_char; 4]; 2] = [
            [
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"CLIENT_EARLY_TRAFFIC_SECRET\0".as_ptr() as *const ::core::ffi::c_char,
                b"CLIENT_HANDSHAKE_TRAFFIC_SECRET\0".as_ptr() as *const ::core::ffi::c_char,
                b"CLIENT_TRAFFIC_SECRET_0\0".as_ptr() as *const ::core::ffi::c_char,
            ],
            [
                ::core::ptr::null::<::core::ffi::c_char>(),
                ::core::ptr::null::<::core::ffi::c_char>(),
                b"SERVER_HANDSHAKE_TRAFFIC_SECRET\0".as_ptr() as *const ::core::ffi::c_char,
                b"SERVER_TRAFFIC_SECRET_0\0".as_ptr() as *const ::core::ffi::c_char,
            ],
        ];
        let mut secret_label: *const ::core::ffi::c_char = log_labels
            [(ptls_is_server(tls) == is_enc) as ::core::ffi::c_int as usize][epoch as usize];
        ptls_hexdump(
            &raw mut hexbuf as *mut ::core::ffi::c_char,
            secret,
            (*(*cipher).hash).digest_size,
        );
        (*(*ctx).log_event).cb.expect("non-null function pointer")(
            (*ctx).log_event as *mut st_ptls_log_event_t,
            tls,
            secret_label,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut hexbuf as *mut ::core::ffi::c_char,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_update_traffic_key_callback() -> *mut ptls_update_traffic_key_t
{
    let mut cb_st: *mut ptls_update_traffic_key_t =
        malloc(::core::mem::size_of::<ptls_update_traffic_key_t>() as size_t)
            as *mut ptls_update_traffic_key_t;
    if !cb_st.is_null() {
        memset(
            cb_st as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<ptls_update_traffic_key_t>() as size_t,
        );
        (*cb_st).cb = Some(
            picoquic_update_traffic_key_callback
                as unsafe extern "C" fn(
                    *mut ptls_update_traffic_key_t,
                    *mut ptls_t,
                    ::core::ffi::c_int,
                    size_t,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut st_ptls_update_traffic_key_t,
                    *mut ptls_t,
                    ::core::ffi::c_int,
                    size_t,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
            >;
    }
    return cb_st;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_setup_initial_master_secret(
    mut cipher: *const ptls_cipher_suite_t,
    mut salt: ptls_iovec_t,
    mut initial_cnxid: picoquic_connection_id_t,
    mut master_secret: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ikm: ptls_iovec_t = st_ptls_iovec_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    };
    let mut cnx_id_serialized: [uint8_t; 20] = [0; 20];
    ikm.len = picoquic_format_connection_id(
        &raw mut cnx_id_serialized as *mut uint8_t,
        PICOQUIC_CONNECTION_ID_MAX_SIZE as size_t,
        initial_cnxid,
    ) as size_t;
    ikm.base = &raw mut cnx_id_serialized as *mut uint8_t;
    ret = ptls_hkdf_extract(
        (*cipher).hash,
        master_secret as *mut ::core::ffi::c_void,
        salt as ptls_iovec_t,
        ikm as ptls_iovec_t,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_setup_initial_secrets(
    mut cipher: *const ptls_cipher_suite_t,
    mut master_secret: *mut uint8_t,
    mut client_secret: *mut uint8_t,
    mut server_secret: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut prk: ptls_iovec_t = st_ptls_iovec_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    };
    prk.base = master_secret;
    prk.len = (*(*cipher).hash).digest_size;
    ret = ptls_hkdf_expand_label(
        (*cipher).hash,
        client_secret as *mut ::core::ffi::c_void,
        (*(*cipher).hash).digest_size,
        prk as ptls_iovec_t,
        PICOQUIC_LABEL_INITIAL_CLIENT.as_ptr(),
        ptls_iovec_init(::core::ptr::null::<::core::ffi::c_void>(), 0 as size_t),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if ret == 0 as ::core::ffi::c_int {
        ret = ptls_hkdf_expand_label(
            (*cipher).hash,
            server_secret as *mut ::core::ffi::c_void,
            (*(*cipher).hash).digest_size,
            prk as ptls_iovec_t,
            PICOQUIC_LABEL_INITIAL_SERVER.as_ptr(),
            ptls_iovec_init(::core::ptr::null::<::core::ffi::c_void>(), 0 as size_t),
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
    }
    return ret;
}
unsafe extern "C" fn picoquic_compute_initial_secrets(
    mut quic: *mut picoquic_quic_t,
    mut version_index: ::core::ffi::c_int,
    mut initial_cnxid: *mut picoquic_connection_id_t,
    mut cipher: *mut *const ptls_cipher_suite_t,
    mut client_secret: *mut uint8_t,
    mut server_secret: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut salt: ptls_iovec_t = st_ptls_iovec_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    };
    let mut master_secret: [uint8_t; 256] = [0; 256];
    *cipher = picoquic_get_aes128gcm_sha256((*quic).use_low_memory() as ::core::ffi::c_int);
    if (*cipher).is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        picoquic_setup_cleartext_aead_salt(version_index as size_t, &raw mut salt);
        ret = picoquic_setup_initial_master_secret(
            *cipher,
            salt,
            *initial_cnxid,
            &raw mut master_secret as *mut uint8_t,
        );
        if ret == 0 as ::core::ffi::c_int {
            ret = picoquic_setup_initial_secrets(
                *cipher,
                &raw mut master_secret as *mut uint8_t,
                client_secret,
                server_secret,
            );
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_setup_initial_traffic_keys(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut prefix_label: *const ::core::ffi::c_char = (*(&raw const picoquic_supported_versions
        as *const picoquic_version_parameters_t)
        .offset((*cnx).version_index as isize))
    .tls_prefix_label;
    let mut cipher: *const ptls_cipher_suite_t = ::core::ptr::null::<ptls_cipher_suite_t>();
    let mut client_secret: [uint8_t; 256] = [0; 256];
    let mut server_secret: [uint8_t; 256] = [0; 256];
    let mut secret1: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut secret2: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    ret = picoquic_compute_initial_secrets(
        (*cnx).quic,
        (*cnx).version_index,
        &raw mut (*cnx).initial_cnxid,
        &raw mut cipher,
        &raw mut client_secret as *mut uint8_t,
        &raw mut server_secret as *mut uint8_t,
    );
    if ret == 0 as ::core::ffi::c_int {
        if (*cnx).client_mode() == 0 {
            secret1 = &raw mut server_secret as *mut uint8_t;
            secret2 = &raw mut client_secret as *mut uint8_t;
        } else {
            secret1 = &raw mut client_secret as *mut uint8_t;
            secret2 = &raw mut server_secret as *mut uint8_t;
        }
        ret = picoquic_set_key_from_secret(
            cipher,
            1 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            (&raw mut (*cnx).crypto_context as *mut picoquic_crypto_context_t)
                .offset(0 as ::core::ffi::c_int as isize)
                as *mut picoquic_crypto_context_t,
            secret1 as *const ::core::ffi::c_void,
            prefix_label,
        );
        if ret == 0 as ::core::ffi::c_int {
            ret = picoquic_set_key_from_secret(
                cipher,
                0 as ::core::ffi::c_int,
                0 as ::core::ffi::c_int,
                (&raw mut (*cnx).crypto_context as *mut picoquic_crypto_context_t)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as *mut picoquic_crypto_context_t,
                secret2 as *const ::core::ffi::c_void,
                prefix_label,
            );
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_initial_aead_context(
    mut quic: *mut picoquic_quic_t,
    mut version_index: ::core::ffi::c_int,
    mut initial_cnxid: *mut picoquic_connection_id_t,
    mut is_client: ::core::ffi::c_int,
    mut is_enc: ::core::ffi::c_int,
    mut aead_ctx: *mut *mut ::core::ffi::c_void,
    mut pn_enc_ctx: *mut *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cipher: *const ptls_cipher_suite_t = ::core::ptr::null::<ptls_cipher_suite_t>();
    let mut client_secret: [uint8_t; 256] = [0; 256];
    let mut server_secret: [uint8_t; 256] = [0; 256];
    let mut prefix_label: *const ::core::ffi::c_char = (*(&raw const picoquic_supported_versions
        as *const picoquic_version_parameters_t)
        .offset(version_index as isize))
    .tls_prefix_label;
    *aead_ctx = NULL_0;
    *pn_enc_ctx = NULL_0;
    ret = picoquic_compute_initial_secrets(
        quic,
        version_index,
        initial_cnxid,
        &raw mut cipher,
        &raw mut client_secret as *mut uint8_t,
        &raw mut server_secret as *mut uint8_t,
    );
    if ret == 0 as ::core::ffi::c_int {
        let mut selected_secret: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        if is_client == 0 {
            selected_secret = if is_enc != 0 {
                &raw mut server_secret as *mut uint8_t
            } else {
                &raw mut client_secret as *mut uint8_t
            };
        } else {
            selected_secret = if is_enc != 0 {
                &raw mut client_secret as *mut uint8_t
            } else {
                &raw mut server_secret as *mut uint8_t
            };
        }
        ret = picoquic_set_aead_from_secret(
            aead_ctx,
            cipher,
            is_enc,
            selected_secret as *const ::core::ffi::c_void,
            prefix_label,
        );
        if ret == 0 as ::core::ffi::c_int {
            ret = picoquic_set_pn_enc_from_secret(
                pn_enc_ctx,
                cipher,
                is_enc,
                selected_secret as *const ::core::ffi::c_void,
                prefix_label,
            );
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_rotate_app_secret(
    mut cipher: *const ptls_cipher_suite_t,
    mut secret: *mut uint8_t,
    mut traffic_update_label: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut new_secret: [uint8_t; 64] = [0; 64];
    ret = ptls_hkdf_expand_label(
        (*cipher).hash,
        &raw mut new_secret as *mut uint8_t as *mut ::core::ffi::c_void,
        (*(*cipher).hash).digest_size,
        ptls_iovec_init(
            secret as *const ::core::ffi::c_void,
            (*(*cipher).hash).digest_size,
        ),
        traffic_update_label,
        ptls_iovec_init(::core::ptr::null::<::core::ffi::c_void>(), 0 as size_t),
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if ret == 0 as ::core::ffi::c_int {
        memcpy(
            secret as *mut ::core::ffi::c_void,
            &raw mut new_secret as *mut uint8_t as *const ::core::ffi::c_void,
            (*(*cipher).hash).digest_size,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_app_secret(
    mut cnx: *mut picoquic_cnx_t,
    mut is_enc: ::core::ffi::c_int,
) -> *mut uint8_t {
    let mut tls_ctx: *mut picoquic_tls_ctx_t = (*cnx).tls_ctx as *mut picoquic_tls_ctx_t;
    return if is_enc != 0 {
        &raw mut (*tls_ctx).app_secret_enc as *mut uint8_t
    } else {
        &raw mut (*tls_ctx).app_secret_dec as *mut uint8_t
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_app_secret_size(mut cnx: *mut picoquic_cnx_t) -> size_t {
    let mut tls_ctx: *mut picoquic_tls_ctx_t = (*cnx).tls_ctx as *mut picoquic_tls_ctx_t;
    let mut cipher: *const ptls_cipher_suite_t =
        ptls_get_cipher((*tls_ctx).tls) as *const ptls_cipher_suite_t;
    return (*(*cipher).hash).digest_size;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_compute_new_rotated_keys(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut tls_ctx: *mut picoquic_tls_ctx_t = (*cnx).tls_ctx as *mut picoquic_tls_ctx_t;
    let mut cipher: *const ptls_cipher_suite_t =
        ptls_get_cipher((*tls_ctx).tls) as *const ptls_cipher_suite_t;
    let mut prefix_label: *const ::core::ffi::c_char = (*(&raw const picoquic_supported_versions
        as *const picoquic_version_parameters_t)
        .offset((*cnx).version_index as isize))
    .tls_prefix_label;
    let mut traffic_update_label: *const ::core::ffi::c_char =
        (*(&raw const picoquic_supported_versions as *const picoquic_version_parameters_t)
            .offset((*cnx).version_index as isize))
        .tls_traffic_update_label;
    if !(*cnx).crypto_context_new.aead_decrypt.is_null()
        || !(*cnx).crypto_context_new.aead_encrypt.is_null()
    {
        if (*cnx).crypto_context_new.aead_decrypt.is_null()
            || (*cnx).crypto_context_new.aead_encrypt.is_null()
        {
            ret = PICOQUIC_ERROR_CANNOT_COMPUTE_KEY;
        } else {
            return 0 as ::core::ffi::c_int;
        }
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = picoquic_rotate_app_secret(
            cipher,
            &raw mut (*tls_ctx).app_secret_enc as *mut uint8_t,
            traffic_update_label,
        );
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = picoquic_set_key_from_secret(
            cipher,
            1 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            &raw mut (*cnx).crypto_context_new,
            &raw mut (*tls_ctx).app_secret_enc as *mut uint8_t as *const ::core::ffi::c_void,
            prefix_label,
        );
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = picoquic_rotate_app_secret(
            cipher,
            &raw mut (*tls_ctx).app_secret_dec as *mut uint8_t,
            traffic_update_label,
        );
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = picoquic_set_key_from_secret(
            cipher,
            0 as ::core::ffi::c_int,
            1 as ::core::ffi::c_int,
            &raw mut (*cnx).crypto_context_new,
            &raw mut (*tls_ctx).app_secret_dec as *mut uint8_t as *const ::core::ffi::c_void,
            prefix_label,
        );
    }
    return if ret == 0 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        PICOQUIC_ERROR_CANNOT_COMPUTE_KEY
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_apply_rotated_keys(
    mut cnx: *mut picoquic_cnx_t,
    mut is_enc: ::core::ffi::c_int,
) {
    if is_enc != 0 {
        if !(*cnx).crypto_context[3 as ::core::ffi::c_int as usize]
            .aead_encrypt
            .is_null()
        {
            ptls_aead_free(
                (*cnx).crypto_context[3 as ::core::ffi::c_int as usize].aead_encrypt
                    as *mut ptls_aead_context_t,
            );
        }
        (*cnx).crypto_context[3 as ::core::ffi::c_int as usize].aead_encrypt =
            (*cnx).crypto_context_new.aead_encrypt;
        (*cnx).crypto_context_new.aead_encrypt = NULL_0;
        (*cnx).set_key_phase_enc(
            (*cnx).key_phase_enc() ^ 1 as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
        picoquic_log_pn_dec_trial(cnx);
    } else {
        if !(*cnx).crypto_context_old.aead_decrypt.is_null() {
            ptls_aead_free((*cnx).crypto_context_old.aead_decrypt as *mut ptls_aead_context_t);
        }
        (*cnx).crypto_context_old.aead_decrypt =
            (*cnx).crypto_context[3 as ::core::ffi::c_int as usize].aead_decrypt;
        (*cnx).crypto_context[3 as ::core::ffi::c_int as usize].aead_decrypt =
            (*cnx).crypto_context_new.aead_decrypt;
        (*cnx).crypto_context_new.aead_decrypt = NULL_0;
        (*cnx).set_key_phase_dec(
            (*cnx).key_phase_dec() ^ 1 as ::core::ffi::c_int as ::core::ffi::c_uint,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_crypto_context_free(mut ctx: *mut picoquic_crypto_context_t) {
    if !(*ctx).aead_encrypt.is_null() {
        ptls_aead_free((*ctx).aead_encrypt as *mut ptls_aead_context_t);
        (*ctx).aead_encrypt = NULL_0;
    }
    if !(*ctx).aead_decrypt.is_null() {
        ptls_aead_free((*ctx).aead_decrypt as *mut ptls_aead_context_t);
        (*ctx).aead_decrypt = NULL_0;
    }
    if !(*ctx).pn_enc.is_null() {
        ptls_cipher_free((*ctx).pn_enc as *mut ptls_cipher_context_t);
        (*ctx).pn_enc = NULL_0;
    }
    if !(*ctx).pn_dec.is_null() {
        ptls_cipher_free((*ctx).pn_dec as *mut ptls_cipher_context_t);
        (*ctx).pn_dec = NULL_0;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_master_tlscontext(
    mut quic: *mut picoquic_quic_t,
    mut cert_file_name: *const ::core::ffi::c_char,
    mut key_file_name: *const ::core::ffi::c_char,
    mut cert_root_file_name: *const ::core::ffi::c_char,
    mut ticket_key: *const uint8_t,
    mut ticket_key_length: size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ctx: *mut ptls_context_t = ::core::ptr::null_mut::<ptls_context_t>();
    let mut och: *mut ptls_on_client_hello_t = ::core::ptr::null_mut::<ptls_on_client_hello_t>();
    let mut encrypt_ticket: *mut ptls_encrypt_ticket_t =
        ::core::ptr::null_mut::<ptls_encrypt_ticket_t>();
    let mut save_ticket: *mut ptls_save_ticket_t = ::core::ptr::null_mut::<ptls_save_ticket_t>();
    let mut is_cert_store_not_empty: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    picoquic_tls_api_init();
    ctx = malloc(::core::mem::size_of::<ptls_context_t>() as size_t) as *mut ptls_context_t;
    if ctx.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        memset(
            ctx as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<ptls_context_t>() as size_t,
        );
        picoquic_set_random_provider_in_ctx(ctx);
        ret = picoquic_set_key_exchange_in_ctx(ctx, 0 as ::core::ffi::c_int);
        if ret == 0 as ::core::ffi::c_int {
            ret = picoquic_set_cipher_suite_in_ctx(
                ctx,
                0 as ::core::ffi::c_int,
                (*quic).use_low_memory() as ::core::ffi::c_int,
            );
        }
        if ret == 0 as ::core::ffi::c_int {
            (*ctx).set_send_change_cipher_spec(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
            (*ctx).hkdf_label_prefix__obsolete = ::core::ptr::null::<::core::ffi::c_char>();
            (*ctx).update_traffic_key = picoquic_set_update_traffic_key_callback();
            if (*quic).p_simulated_time.is_null() {
                (*ctx).get_time = &raw mut ptls_get_time;
            } else {
                let mut time_getter: *mut ptls_get_time_t = malloc(
                    (::core::mem::size_of::<ptls_get_time_t>() as size_t)
                        .wrapping_add(::core::mem::size_of::<*mut uint64_t>() as size_t),
                )
                    as *mut ptls_get_time_t;
                if time_getter.is_null() {
                    ret = PICOQUIC_ERROR_MEMORY;
                } else {
                    let mut pp_simulated_time: *mut *mut uint64_t = (time_getter
                        as *mut ::core::ffi::c_char)
                        .offset(::core::mem::size_of::<ptls_get_time_t>() as usize as isize)
                        as *mut *mut uint64_t;
                    (*time_getter).cb = Some(
                        picoquic_get_simulated_time_cb
                            as unsafe extern "C" fn(*mut ptls_get_time_t) -> uint64_t,
                    )
                        as Option<unsafe extern "C" fn(*mut st_ptls_get_time_t) -> uint64_t>;
                    *pp_simulated_time = (*quic).p_simulated_time;
                    (*ctx).get_time = time_getter;
                }
            }
            if !cert_file_name.is_null() && !key_file_name.is_null() {
                if ptls_load_certificates(ctx, cert_file_name as *mut ::core::ffi::c_char)
                    != 0 as ::core::ffi::c_int
                {
                    ret = -(1 as ::core::ffi::c_int);
                } else {
                    ret = set_private_key_from_file(key_file_name, ctx);
                    ret != 0 as ::core::ffi::c_int;
                }
            }
        }
        if ret == 0 as ::core::ffi::c_int {
            och = malloc(
                (::core::mem::size_of::<ptls_on_client_hello_t>() as size_t)
                    .wrapping_add(::core::mem::size_of::<*mut picoquic_quic_t>() as size_t),
            ) as *mut ptls_on_client_hello_t;
            if !och.is_null() {
                let mut ppquic: *mut *mut picoquic_quic_t = (och as *mut ::core::ffi::c_char)
                    .offset(::core::mem::size_of::<ptls_on_client_hello_t>() as usize as isize)
                    as *mut *mut picoquic_quic_t;
                (*och).cb = Some(
                    picoquic_client_hello_call_back
                        as unsafe extern "C" fn(
                            *mut ptls_on_client_hello_t,
                            *mut ptls_t,
                            *mut ptls_on_client_hello_parameters_t,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut st_ptls_on_client_hello_t,
                            *mut ptls_t,
                            *mut ptls_on_client_hello_parameters_t,
                        ) -> ::core::ffi::c_int,
                    >;
                (*ctx).on_client_hello = och;
                *ppquic = quic;
            } else {
                ret = PICOQUIC_ERROR_MEMORY;
            }
        }
        if ret == 0 as ::core::ffi::c_int {
            ret = picoquic_server_setup_ticket_aead_contexts(
                quic,
                ctx,
                ticket_key,
                ticket_key_length,
            );
        }
        if ret == 0 as ::core::ffi::c_int {
            encrypt_ticket = malloc(
                (::core::mem::size_of::<ptls_encrypt_ticket_t>() as size_t)
                    .wrapping_add(::core::mem::size_of::<*mut picoquic_quic_t>() as size_t),
            ) as *mut ptls_encrypt_ticket_t;
            if encrypt_ticket.is_null() {
                ret = PICOQUIC_ERROR_MEMORY;
            } else {
                let mut ppquic_0: *mut *mut picoquic_quic_t = (encrypt_ticket
                    as *mut ::core::ffi::c_char)
                    .offset(::core::mem::size_of::<ptls_encrypt_ticket_t>() as usize as isize)
                    as *mut *mut picoquic_quic_t;
                (*encrypt_ticket).cb = Some(
                    picoquic_server_encrypt_ticket_call_back
                        as unsafe extern "C" fn(
                            *mut ptls_encrypt_ticket_t,
                            *mut ptls_t,
                            ::core::ffi::c_int,
                            *mut ptls_buffer_t,
                            ptls_iovec_t,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut st_ptls_encrypt_ticket_t,
                            *mut ptls_t,
                            ::core::ffi::c_int,
                            *mut ptls_buffer_t,
                            ptls_iovec_t,
                        ) -> ::core::ffi::c_int,
                    >;
                *ppquic_0 = quic;
                (*ctx).encrypt_ticket = encrypt_ticket;
                (*ctx).ticket_lifetime = 100000 as uint32_t;
                (*ctx).set_require_dhe_on_psk(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                (*ctx).max_early_data_size = 0xffffffff as ::core::ffi::c_uint as uint32_t;
            }
        }
        if ret == 0 as ::core::ffi::c_int {
            (*ctx).verify_certificate = picoquic_get_certificate_verifier(
                cert_root_file_name,
                &raw mut is_cert_store_not_empty,
                &raw mut (*quic).free_verify_certificate_callback_fn,
            ) as *mut ptls_verify_certificate_t;
            (*quic).set_is_cert_store_not_empty(is_cert_store_not_empty as ::core::ffi::c_uint);
        }
        if ret == 0 as ::core::ffi::c_int && !(*quic).ticket_file_name.is_null() {
            save_ticket = malloc(
                (::core::mem::size_of::<ptls_save_ticket_t>() as size_t)
                    .wrapping_add(::core::mem::size_of::<*mut picoquic_quic_t>() as size_t),
            ) as *mut ptls_save_ticket_t;
            if !save_ticket.is_null() {
                let mut ppquic_1: *mut *mut picoquic_quic_t = (save_ticket
                    as *mut ::core::ffi::c_char)
                    .offset(::core::mem::size_of::<ptls_save_ticket_t>() as usize as isize)
                    as *mut *mut picoquic_quic_t;
                (*save_ticket).cb = Some(
                    picoquic_client_save_ticket_call_back
                        as unsafe extern "C" fn(
                            *mut ptls_save_ticket_t,
                            *mut ptls_t,
                            ptls_iovec_t,
                        ) -> ::core::ffi::c_int,
                )
                    as Option<
                        unsafe extern "C" fn(
                            *mut st_ptls_save_ticket_t,
                            *mut ptls_t,
                            ptls_iovec_t,
                        ) -> ::core::ffi::c_int,
                    >;
                (*ctx).save_ticket = save_ticket;
                *ppquic_1 = quic;
            }
        }
        if ret == 0 as ::core::ffi::c_int {
            (*ctx).set_omit_end_of_early_data(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
        }
        if ret == 0 as ::core::ffi::c_int {
            (*quic).tls_master_ctx = ctx as *mut ::core::ffi::c_void;
            picoquic_public_random_seed(quic);
        } else {
            (*quic).tls_master_ctx = ctx as *mut ::core::ffi::c_void;
            picoquic_master_tlscontext_free(quic);
            (*quic).tls_master_ctx = NULL_0;
            free(ctx as *mut ::core::ffi::c_void);
        }
    }
    return ret;
}
unsafe extern "C" fn free_certificates_list(mut certs: *mut ptls_iovec_t, mut len: size_t) {
    if certs.is_null() {
        return;
    }
    let mut i: size_t = 0 as size_t;
    while i < len {
        free((*certs.offset(i as isize)).base as *mut ::core::ffi::c_void);
        i = i.wrapping_add(1);
    }
    free(certs as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_master_tlscontext_free(mut quic: *mut picoquic_quic_t) {
    if !(*quic).tls_master_ctx.is_null() {
        let mut ctx: *mut ptls_context_t = (*quic).tls_master_ctx as *mut ptls_context_t;
        if !(*quic).p_simulated_time.is_null() && !(*ctx).get_time.is_null() {
            free((*ctx).get_time as *mut ::core::ffi::c_void);
            (*ctx).get_time = ::core::ptr::null_mut::<ptls_get_time_t>();
        }
        free_certificates_list(
            (*ctx).certificates.list as *mut ptls_iovec_t,
            (*ctx).certificates.count,
        );
        picoquic_dispose_sign_certificate(ctx);
        picoquic_dispose_verify_certificate_callback(quic);
        if !(*ctx).on_client_hello.is_null() {
            free((*ctx).on_client_hello as *mut ::core::ffi::c_void);
        }
        if !(*ctx).encrypt_ticket.is_null() {
            free((*ctx).encrypt_ticket as *mut ::core::ffi::c_void);
        }
        if !(*ctx).update_traffic_key.is_null() {
            free((*ctx).update_traffic_key as *mut ::core::ffi::c_void);
        }
        if !(*ctx).save_ticket.is_null() {
            free((*ctx).save_ticket as *mut ::core::ffi::c_void);
        }
        if !(*ctx).cipher_suites.is_null() {
            free((*ctx).cipher_suites as *mut ::core::ffi::c_void);
        }
        picoquic_free_log_event(quic);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_tls_time(mut quic: *mut picoquic_quic_t) -> uint64_t {
    let mut ctx: *mut ptls_context_t = (*quic).tls_master_ctx as *mut ptls_context_t;
    let mut now: uint64_t = (*(*ctx).get_time).cb.expect("non-null function pointer")(
        (*ctx).get_time as *mut st_ptls_get_time_t,
    )
    .wrapping_mul(1000 as uint64_t);
    return now;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tlscontext_create(
    mut quic: *mut picoquic_quic_t,
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ctx: *mut picoquic_tls_ctx_t =
        malloc(::core::mem::size_of::<picoquic_tls_ctx_t>() as size_t) as *mut picoquic_tls_ctx_t;
    if ctx.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        memset(
            ctx as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<picoquic_tls_ctx_t>() as size_t,
        );
        (*ctx).ext_data_size = PICOQUIC_TRANSPORT_PARAMETERS_MAX_SIZE as size_t;
        if (*cnx).client_mode() == 0
            && (*quic).test_large_server_flight() as ::core::ffi::c_int != 0
        {
            (*ctx).ext_data_size = (*ctx).ext_data_size.wrapping_add(4096 as size_t);
        }
        (*ctx).ext_data = malloc((*ctx).ext_data_size) as *mut uint8_t;
        (*ctx).alpn_vec = malloc(
            (::core::mem::size_of::<ptls_iovec_t>() as size_t)
                .wrapping_mul(PICOQUIC_ALPN_NUMBER_MAX as size_t),
        ) as *mut ptls_iovec_t;
        if (*ctx).ext_data.is_null() || (*ctx).alpn_vec.is_null() {
            ret = -(1 as ::core::ffi::c_int);
        } else {
            (*ctx).alpn_vec_size = PICOQUIC_ALPN_NUMBER_MAX as size_t;
            (*ctx).cnx = cnx;
            (*ctx).handshake_properties.collect_extension = Some(
                picoquic_tls_collect_extensions_cb
                    as unsafe extern "C" fn(
                        *mut ptls_t,
                        *mut st_ptls_handshake_properties_t,
                        uint16_t,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut ptls_t,
                        *mut st_ptls_handshake_properties_t,
                        uint16_t,
                    ) -> ::core::ffi::c_int,
                >;
            (*ctx).handshake_properties.collected_extensions = Some(
                picoquic_tls_collected_extensions_cb
                    as unsafe extern "C" fn(
                        *mut ptls_t,
                        *mut ptls_handshake_properties_t,
                        *mut ptls_raw_extension_t,
                    ) -> ::core::ffi::c_int,
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut ptls_t,
                        *mut st_ptls_handshake_properties_t,
                        *mut ptls_raw_extension_t,
                    ) -> ::core::ffi::c_int,
                >;
            (*ctx).client_mode = (*cnx).client_mode() as ::core::ffi::c_int;
            (*ctx).tls = ptls_new(
                (*quic).tls_master_ctx as *mut ptls_context_t,
                if (*ctx).client_mode != 0 {
                    0 as ::core::ffi::c_int
                } else {
                    1 as ::core::ffi::c_int
                },
            );
            let ref mut c2rust_fresh8 = *ptls_get_data_ptr((*ctx).tls);
            *c2rust_fresh8 = cnx as *mut ::core::ffi::c_void;
            if (*ctx).tls.is_null() {
                free(ctx as *mut ::core::ffi::c_void);
                ctx = ::core::ptr::null_mut::<picoquic_tls_ctx_t>();
                ret = -(1 as ::core::ffi::c_int);
            } else if (*ctx).client_mode == 0 {
                if (*((*quic).tls_master_ctx as *mut ptls_context_t))
                    .encrypt_ticket
                    .is_null()
                {
                    ret = PICOQUIC_ERROR_TLS_SERVER_CON_WITHOUT_CERT;
                    picoquic_tlscontext_free(ctx as *mut ::core::ffi::c_void);
                    ctx = ::core::ptr::null_mut::<picoquic_tls_ctx_t>();
                }
                if !ctx.is_null() {
                    (*ctx)
                        .handshake_properties
                        .c2rust_unnamed
                        .server
                        .set_enforce_retry(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*ctx)
                        .handshake_properties
                        .c2rust_unnamed
                        .server
                        .set_retry_uses_cookie(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
                    (*ctx).handshake_properties.c2rust_unnamed.server.cookie.key =
                        ::core::ptr::null::<::core::ffi::c_void>();
                    (*ctx)
                        .handshake_properties
                        .c2rust_unnamed
                        .server
                        .cookie
                        .additional_data
                        .base = ::core::ptr::null_mut::<uint8_t>();
                    (*ctx)
                        .handshake_properties
                        .c2rust_unnamed
                        .server
                        .cookie
                        .additional_data
                        .len = 0 as size_t;
                }
            }
        }
    }
    if !(*cnx).tls_ctx.is_null() {
        picoquic_tlscontext_free((*cnx).tls_ctx);
    }
    (*cnx).tls_ctx = ctx as *mut ::core::ffi::c_void;
    return ret;
}
unsafe extern "C" fn picoquic_log_event_call_back(
    mut _self: *mut ptls_log_event_t,
    mut tls: *mut ptls_t,
    mut type_0: *const ::core::ffi::c_char,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut self_0: *mut st_picoquic_log_event_t = _self as *mut st_picoquic_log_event_t;
    let mut randomhex: [::core::ffi::c_char; 65] = [0; 65];
    let mut args: ::core::ffi::VaListImpl;
    if !(*self_0).fp.is_null() {
        ptls_hexdump(
            &raw mut randomhex as *mut ::core::ffi::c_char,
            ptls_get_client_random(tls).base as *const ::core::ffi::c_void,
            PTLS_HELLO_RANDOM_SIZE as size_t,
        );
        fprintf(
            (*self_0).fp,
            b"%s %s \0".as_ptr() as *const ::core::ffi::c_char,
            type_0,
            &raw mut randomhex as *mut ::core::ffi::c_char,
        );
        args = c2rust_args.clone();
        vfprintf((*self_0).fp, fmt, args.as_va_list());
        fprintf((*self_0).fp, b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        fflush((*self_0).fp);
    }
}
unsafe extern "C" fn picoquic_free_log_event(mut quic: *mut picoquic_quic_t) {
    let mut ctx: *mut ptls_context_t = (*quic).tls_master_ctx as *mut ptls_context_t;
    if !(*ctx).log_event.is_null() {
        let mut picoquic_log_event: *mut st_picoquic_log_event_t =
            (*ctx).log_event as *mut st_picoquic_log_event_t;
        if !picoquic_log_event.is_null() && !(*picoquic_log_event).fp.is_null() {
            picoquic_file_close((*picoquic_log_event).fp);
        }
        free((*ctx).log_event as *mut ::core::ffi::c_void);
        (*ctx).log_event = ::core::ptr::null_mut::<ptls_log_event_t>();
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_key_log_file(
    mut quic: *mut picoquic_quic_t,
    mut keylog_filename: *const ::core::ffi::c_char,
) {
    let mut ctx: *mut ptls_context_t = (*quic).tls_master_ctx as *mut ptls_context_t;
    let mut log_event: *mut st_picoquic_log_event_t =
        (*ctx).log_event as *mut st_picoquic_log_event_t;
    if log_event.is_null() {
        log_event = malloc(::core::mem::size_of::<st_picoquic_log_event_t>() as size_t)
            as *mut st_picoquic_log_event_t;
        if !log_event.is_null() {
            (*log_event).super_0.cb = Some(
                picoquic_log_event_call_back
                    as unsafe extern "C" fn(
                        *mut ptls_log_event_t,
                        *mut ptls_t,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        ...
                    ) -> (),
            )
                as Option<
                    unsafe extern "C" fn(
                        *mut st_ptls_log_event_t,
                        *mut ptls_t,
                        *const ::core::ffi::c_char,
                        *const ::core::ffi::c_char,
                        ...
                    ) -> (),
                >;
        }
    } else if !(*log_event).fp.is_null() {
        picoquic_file_close((*log_event).fp);
        (*log_event).fp = ::core::ptr::null_mut::<FILE>();
    }
    if !log_event.is_null() {
        (*log_event).fp = picoquic_file_open(
            keylog_filename,
            b"a\0".as_ptr() as *const ::core::ffi::c_char,
        );
        (*log_event).super_0.cb = Some(
            picoquic_log_event_call_back
                as unsafe extern "C" fn(
                    *mut ptls_log_event_t,
                    *mut ptls_t,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut st_ptls_log_event_t,
                    *mut ptls_t,
                    *const ::core::ffi::c_char,
                    *const ::core::ffi::c_char,
                    ...
                ) -> (),
            >;
        (*ctx).log_event = log_event as *mut ptls_log_event_t;
    }
    (*ctx).log_event = log_event as *mut ptls_log_event_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_does_tls_ticket_allow_early_data(
    mut ticket: *mut uint8_t,
    mut ticket_length: uint16_t,
) -> ::core::ffi::c_int {
    let mut nonce_length: uint8_t = 0 as uint8_t;
    let mut ticket_val_length: uint16_t = 0 as uint16_t;
    let mut extension_length: uint16_t = 0 as uint16_t;
    let mut extension_ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut byte_index: uint16_t = 0 as uint16_t;
    let mut min_length: uint16_t = (4 as ::core::ffi::c_int
        + 4 as ::core::ffi::c_int
        + 1 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int
        + 2 as ::core::ffi::c_int) as uint16_t;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if ticket_length as ::core::ffi::c_int >= min_length as ::core::ffi::c_int {
        byte_index = (byte_index as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint16_t;
        byte_index = (byte_index as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as uint16_t;
        let c2rust_fresh17 = byte_index;
        byte_index = byte_index.wrapping_add(1);
        nonce_length = *ticket.offset(c2rust_fresh17 as isize);
        min_length =
            (min_length as ::core::ffi::c_int + nonce_length as ::core::ffi::c_int) as uint16_t;
        if ticket_length as ::core::ffi::c_int >= min_length as ::core::ffi::c_int {
            byte_index =
                (byte_index as ::core::ffi::c_int + nonce_length as ::core::ffi::c_int) as uint16_t;
            ticket_val_length = ((*ticket
                .offset(byte_index as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize)
                as uint16_t as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *ticket
                    .offset(byte_index as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint16_t;
            byte_index = (byte_index as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint16_t;
            min_length = (min_length as ::core::ffi::c_int
                + ticket_val_length as ::core::ffi::c_int) as uint16_t;
            if ticket_length as ::core::ffi::c_int >= min_length as ::core::ffi::c_int {
                byte_index = (byte_index as ::core::ffi::c_int
                    + ticket_val_length as ::core::ffi::c_int)
                    as uint16_t;
                extension_length = ((*ticket
                    .offset(byte_index as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize)
                    as uint16_t as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *ticket
                        .offset(byte_index as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int) as uint16_t;
                byte_index =
                    (byte_index as ::core::ffi::c_int + 2 as ::core::ffi::c_int) as uint16_t;
                min_length = (min_length as ::core::ffi::c_int
                    + extension_length as ::core::ffi::c_int)
                    as uint16_t;
                if ticket_length as ::core::ffi::c_int >= min_length as ::core::ffi::c_int {
                    extension_ptr = ticket.offset(byte_index as isize) as *mut uint8_t;
                }
            }
        }
    }
    if !extension_ptr.is_null() {
        let mut x_index: uint16_t = 0 as uint16_t;
        while (x_index as ::core::ffi::c_int + 4 as ::core::ffi::c_int)
            < extension_length as ::core::ffi::c_int
        {
            let mut x_type: uint16_t = ((*extension_ptr
                .offset(x_index as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize)
                as uint16_t as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *extension_ptr
                    .offset(x_index as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint16_t;
            let mut x_len: uint16_t = ((*extension_ptr
                .offset(x_index as ::core::ffi::c_int as isize)
                .offset(2 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize)
                as uint16_t as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *extension_ptr
                    .offset(x_index as ::core::ffi::c_int as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint16_t;
            x_index = (x_index as ::core::ffi::c_int
                + (4 as ::core::ffi::c_int + x_len as ::core::ffi::c_int))
                as uint16_t;
            if !(x_type as ::core::ffi::c_int == 42 as ::core::ffi::c_int
                && x_len as ::core::ffi::c_int == 4 as ::core::ffi::c_int)
            {
                continue;
            }
            let mut ed_len: uint32_t = (((*extension_ptr
                .offset(x_index as ::core::ffi::c_int as isize)
                .offset(-(4 as ::core::ffi::c_int as isize))
                .offset(0 as ::core::ffi::c_int as isize)
                as uint16_t as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *extension_ptr
                    .offset(x_index as ::core::ffi::c_int as isize)
                    .offset(-(4 as ::core::ffi::c_int as isize))
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t)
                << 16 as ::core::ffi::c_int
                | ((*extension_ptr
                    .offset(x_index as ::core::ffi::c_int as isize)
                    .offset(-(4 as ::core::ffi::c_int as isize))
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *extension_ptr
                        .offset(x_index as ::core::ffi::c_int as isize)
                        .offset(-(4 as ::core::ffi::c_int as isize))
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int) as uint32_t;
            if ed_len == 0xffffffff as uint32_t {
                ret = 1 as ::core::ffi::c_int;
            }
            break;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tlscontext_remove_ticket(mut cnx: *mut picoquic_cnx_t) {
    let mut ctx: *mut picoquic_tls_ctx_t = (*cnx).tls_ctx as *mut picoquic_tls_ctx_t;
    (*ctx)
        .handshake_properties
        .c2rust_unnamed
        .client
        .session_ticket
        .base = ::core::ptr::null_mut::<uint8_t>();
    (*ctx)
        .handshake_properties
        .c2rust_unnamed
        .client
        .session_ticket
        .len = 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tlscontext_free(mut vctx: *mut ::core::ffi::c_void) {
    let mut ctx: *mut picoquic_tls_ctx_t = vctx as *mut picoquic_tls_ctx_t;
    if !(*ctx).ext_data.is_null() {
        free((*ctx).ext_data as *mut ::core::ffi::c_void);
    }
    if !(*ctx).alpn_vec.is_null() {
        free((*ctx).alpn_vec as *mut ::core::ffi::c_void);
    }
    if !(*ctx).tls.is_null() {
        ptls_free((*ctx).tls);
        (*ctx).tls = ::core::ptr::null_mut::<ptls_t>();
    }
    free(ctx as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tlscontext_trim_after_handshake(mut cnx: *mut picoquic_cnx_t) {
    let mut ctx: *mut picoquic_tls_ctx_t = (*cnx).tls_ctx as *mut picoquic_tls_ctx_t;
    if !(*ctx).ext_data.is_null() {
        free((*ctx).ext_data as *mut ::core::ffi::c_void);
        (*ctx).ext_data = ::core::ptr::null_mut::<uint8_t>();
        (*ctx).ext_data_size = 0 as size_t;
    }
    if !(*ctx).alpn_vec.is_null() {
        free((*ctx).alpn_vec as *mut ::core::ffi::c_void);
        (*ctx).alpn_vec = ::core::ptr::null_mut::<ptls_iovec_t>();
        (*ctx).alpn_vec_size = 0 as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tls_get_negotiated_alpn(
    mut cnx: *mut picoquic_cnx_t,
) -> *const ::core::ffi::c_char {
    let mut ctx: *mut picoquic_tls_ctx_t = (*cnx).tls_ctx as *mut picoquic_tls_ctx_t;
    return ptls_get_negotiated_protocol((*ctx).tls);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tls_get_sni(
    mut cnx: *mut picoquic_cnx_t,
) -> *const ::core::ffi::c_char {
    let mut ctx: *mut picoquic_tls_ctx_t = (*cnx).tls_ctx as *mut picoquic_tls_ctx_t;
    return ptls_get_server_name((*ctx).tls);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tls_is_psk_handshake(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int =
        ptls_is_psk_handshake((*((*cnx).tls_ctx as *mut picoquic_tls_ctx_t)).tls);
    return ret;
}
unsafe extern "C" fn picoquic_add_to_tls_stream(
    mut cnx: *mut picoquic_cnx_t,
    mut data: *const uint8_t,
    mut length: size_t,
    mut epoch: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut stream: *mut picoquic_stream_head_t =
        (&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t).offset(epoch as isize)
            as *mut picoquic_stream_head_t;
    if length > 0 as size_t {
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
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_add_proposed_alpn(
    mut tls_context: *mut ::core::ffi::c_void,
    mut alpn: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ctx: *mut picoquic_tls_ctx_t = tls_context as *mut picoquic_tls_ctx_t;
    if ctx.is_null() {
        ret = PICOQUIC_ERROR_UNEXPECTED_ERROR;
    } else if (*ctx).alpn_count >= (*ctx).alpn_vec_size {
        ret = PICOQUIC_ERROR_SEND_BUFFER_TOO_SMALL;
    } else {
        let ref mut c2rust_fresh0 = (*(*ctx).alpn_vec.offset((*ctx).alpn_count as isize)).base;
        *c2rust_fresh0 = alpn as *mut uint8_t;
        (*(*ctx).alpn_vec.offset((*ctx).alpn_count as isize)).len = strlen(alpn);
        (*ctx).alpn_count = (*ctx).alpn_count.wrapping_add(1);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_initialize_tls_stream(
    mut cnx: *mut picoquic_cnx_t,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sendbuf: st_ptls_buffer_t = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0,
        off: 0,
        is_allocated: 0,
        align_bits: 0,
    };
    let mut ctx: *mut picoquic_tls_ctx_t = (*cnx).tls_ctx as *mut picoquic_tls_ctx_t;
    let mut epoch_offsets: [size_t; 5] = [
        0 as ::core::ffi::c_int as size_t,
        0 as ::core::ffi::c_int as size_t,
        0 as ::core::ffi::c_int as size_t,
        0 as ::core::ffi::c_int as size_t,
        0 as ::core::ffi::c_int as size_t,
    ];
    if !(*cnx).sni.is_null() {
        ptls_set_server_name((*ctx).tls, (*cnx).sni, strlen((*cnx).sni));
    }
    if !(*cnx).alpn.is_null() {
        let ref mut c2rust_fresh9 =
            (*(*ctx).alpn_vec.offset(0 as ::core::ffi::c_int as isize)).base;
        *c2rust_fresh9 = (*cnx).alpn as *mut uint8_t;
        (*(*ctx).alpn_vec.offset(0 as ::core::ffi::c_int as isize)).len = strlen((*cnx).alpn);
        (*ctx)
            .handshake_properties
            .c2rust_unnamed
            .client
            .negotiated_protocols
            .count = 1 as size_t;
        (*ctx)
            .handshake_properties
            .c2rust_unnamed
            .client
            .negotiated_protocols
            .list = (*ctx).alpn_vec;
    } else if (*cnx).callback_fn.is_some() {
        ret = (*cnx).callback_fn.expect("non-null function pointer")(
            cnx as *mut picoquic_cnx_t,
            0 as uint64_t,
            ctx as *mut uint8_t,
            0 as size_t,
            picoquic_callback_request_alpn_list,
            (*cnx).callback_ctx,
            NULL_0,
        );
        (*ctx)
            .handshake_properties
            .c2rust_unnamed
            .client
            .negotiated_protocols
            .count = (*ctx).alpn_count;
        (*ctx)
            .handshake_properties
            .c2rust_unnamed
            .client
            .negotiated_protocols
            .list = (*ctx).alpn_vec;
        ret != 0 as ::core::ffi::c_int;
    }
    if ret == 0 as ::core::ffi::c_int
        && (*ctx)
            .handshake_properties
            .c2rust_unnamed
            .client
            .negotiated_protocols
            .count
            == 0 as size_t
    {
        ret = PICOQUIC_ERROR_NO_ALPN_PROVIDED;
    }
    picoquic_log_negotiated_alpn(
        cnx,
        1 as ::core::ffi::c_int,
        (*cnx).sni as *const uint8_t,
        if (*cnx).sni.is_null() {
            0 as size_t
        } else {
            strlen((*cnx).sni)
        },
        ::core::ptr::null::<uint8_t>(),
        0 as size_t,
        (*ctx)
            .handshake_properties
            .c2rust_unnamed
            .client
            .negotiated_protocols
            .list as *const ptls_iovec_t,
        (*ctx)
            .handshake_properties
            .c2rust_unnamed
            .client
            .negotiated_protocols
            .count,
    );
    if !(*cnx).sni.is_null() && !(*cnx).alpn.is_null() && (*(*cnx).quic).client_zero_share() == 0 {
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
            (*ctx)
                .handshake_properties
                .c2rust_unnamed
                .client
                .session_ticket
                .base = (*stored_ticket).ticket;
            (*ctx)
                .handshake_properties
                .c2rust_unnamed
                .client
                .session_ticket
                .len = (*stored_ticket).ticket_length as size_t;
            (*ctx)
                .handshake_properties
                .c2rust_unnamed
                .client
                .max_early_data_size = &raw mut (*cnx).max_early_data_size;
            (*cnx).resumed_ticket_id = (((((*(*stored_ticket)
                .ticket
                .offset(0 as ::core::ffi::c_int as isize)
                as uint16_t as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(*stored_ticket)
                    .ticket
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t)
                << 16 as ::core::ffi::c_int
                | ((*(*stored_ticket)
                    .ticket
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(*stored_ticket)
                        .ticket
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int) as uint32_t)
                as uint64_t)
                << 32 as ::core::ffi::c_int
                | ((((*(*stored_ticket)
                    .ticket
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(*stored_ticket)
                        .ticket
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int) as uint32_t)
                    << 16 as ::core::ffi::c_int
                    | ((*(*stored_ticket)
                        .ticket
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(0 as ::core::ffi::c_int as isize)
                        as uint16_t as ::core::ffi::c_int)
                        << 8 as ::core::ffi::c_int
                        | *(*stored_ticket)
                            .ticket
                            .offset(4 as ::core::ffi::c_int as isize)
                            .offset(2 as ::core::ffi::c_int as isize)
                            .offset(1 as ::core::ffi::c_int as isize)
                            as uint16_t as ::core::ffi::c_int) as uint32_t)
                    as uint64_t;
            (*cnx).psk_cipher_suite_id = ((*(*stored_ticket)
                .ticket
                .offset(8 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize)
                as uint16_t as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(*stored_ticket)
                    .ticket
                    .offset(8 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint16_t;
            (*cnx).remote_parameters.initial_max_data =
                (*stored_ticket).tp_0rtt[picoquic_tp_0rtt_max_data as ::core::ffi::c_int as usize];
            (*cnx).remote_parameters.initial_max_stream_data_bidi_local = (*stored_ticket).tp_0rtt
                [picoquic_tp_0rtt_max_stream_data_bidi_local as ::core::ffi::c_int as usize];
            (*cnx).remote_parameters.initial_max_stream_data_bidi_remote = (*stored_ticket).tp_0rtt
                [picoquic_tp_0rtt_max_stream_data_bidi_remote as ::core::ffi::c_int as usize];
            (*cnx).remote_parameters.initial_max_stream_data_uni = (*stored_ticket).tp_0rtt
                [picoquic_tp_0rtt_max_stream_data_uni as ::core::ffi::c_int as usize];
            (*cnx).remote_parameters.initial_max_stream_id_bidir = (*stored_ticket).tp_0rtt
                [picoquic_tp_0rtt_max_streams_id_bidir as ::core::ffi::c_int as usize];
            (*cnx).remote_parameters.initial_max_stream_id_unidir = (*stored_ticket).tp_0rtt
                [picoquic_tp_0rtt_max_streams_id_unidir as ::core::ffi::c_int as usize];
            if (*stored_ticket).time_valid_until > current_time {
                picoquic_seed_bandwidth(
                    cnx,
                    (*stored_ticket).tp_0rtt
                        [picoquic_tp_0rtt_rtt_local as ::core::ffi::c_int as usize],
                    (*stored_ticket).tp_0rtt
                        [picoquic_tp_0rtt_cwin_local as ::core::ffi::c_int as usize],
                    (*stored_ticket).ip_addr,
                    (*stored_ticket).ip_addr_length,
                );
            }
        }
    }
    if (*(*cnx).quic).client_zero_share() as ::core::ffi::c_int != 0
        && (*cnx).cnx_state as ::core::ffi::c_uint
            == picoquic_state_client_init as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*ctx)
            .handshake_properties
            .c2rust_unnamed
            .client
            .set_negotiate_before_key_exchange(1 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    } else {
        (*ctx)
            .handshake_properties
            .c2rust_unnamed
            .client
            .set_negotiate_before_key_exchange(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    }
    if ret != 0 as ::core::ffi::c_int {
        picoquic_connection_disconnect(cnx);
    } else {
        picoquic_tls_set_extensions(cnx, ctx);
        ptls_buffer_init(
            &raw mut sendbuf,
            b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_void,
            0 as size_t,
        );
        picoquic_clear_crypto_errors();
        ret = ptls_handle_message(
            (*ctx).tls,
            &raw mut sendbuf,
            &raw mut epoch_offsets as *mut size_t,
            0 as size_t,
            ::core::ptr::null::<::core::ffi::c_void>(),
            0 as size_t,
            &raw mut (*ctx).handshake_properties,
        );
        if ret == 0 as ::core::ffi::c_int || ret == PTLS_ERROR_IN_PROGRESS {
            if sendbuf.off > 0 as size_t {
                ret = picoquic_add_to_tls_stream(
                    cnx,
                    sendbuf.base,
                    sendbuf.off,
                    0 as ::core::ffi::c_int,
                );
            } else {
                ret = 0 as ::core::ffi::c_int;
            }
        } else {
            picoquic_log_crypto_errors(cnx, ret);
            ret = -(1 as ::core::ffi::c_int);
        }
        ptls_buffer_dispose(&raw mut sendbuf);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_pn_enc_create_for_test(
    mut secret: *const uint8_t,
    mut prefix_label: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    let mut cipher: *const ptls_cipher_suite_t =
        picoquic_get_aes128gcm_sha256(1 as ::core::ffi::c_int);
    let mut v_pn_enc: *mut ::core::ffi::c_void = NULL_0;
    picoquic_set_pn_enc_from_secret(
        &raw mut v_pn_enc,
        cipher,
        1 as ::core::ffi::c_int,
        secret as *const ::core::ffi::c_void,
        prefix_label,
    );
    return v_pn_enc;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_pn_iv_size(mut pn_enc: *mut ::core::ffi::c_void) -> size_t {
    return (*(*(pn_enc as *mut ptls_cipher_context_t)).algo).iv_size;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_pn_encrypt(
    mut pn_enc: *mut ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    ptls_cipher_init(pn_enc as *mut ptls_cipher_context_t, iv);
    ptls_cipher_encrypt(pn_enc as *mut ptls_cipher_context_t, output, input, len);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_aead_free(mut aead_context: *mut ::core::ffi::c_void) {
    ptls_aead_free(aead_context as *mut ptls_aead_context_t);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cipher_free(mut cipher_context: *mut ::core::ffi::c_void) {
    ptls_cipher_free(cipher_context as *mut ptls_cipher_context_t);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_aead_get_checksum_length(
    mut aead_context: *mut ::core::ffi::c_void,
) -> size_t {
    let mut tag_size: size_t = (*(*(aead_context as *mut ptls_aead_context_t)).algo).tag_size;
    if tag_size > 16 as size_t {
        tag_size = 16 as size_t;
    }
    return tag_size;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_setup_test_aead_context(
    mut is_encrypt: ::core::ffi::c_int,
    mut secret: *const uint8_t,
    mut prefix_label: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    let mut v_aead: *mut ::core::ffi::c_void = NULL_0;
    let mut cipher: *const ptls_cipher_suite_t =
        picoquic_get_aes128gcm_sha256(1 as ::core::ffi::c_int);
    picoquic_set_aead_from_secret(
        &raw mut v_aead,
        cipher,
        is_encrypt,
        secret as *const ::core::ffi::c_void,
        prefix_label,
    );
    return v_aead;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_server_setup_ticket_aead_contexts(
    mut quic: *mut picoquic_quic_t,
    mut tls_ctx: *mut ptls_context_t,
    mut secret: *const uint8_t,
    mut secret_length: size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut temp_secret: [uint8_t; 256] = [0; 256];
    let mut cipher: *const ptls_cipher_suite_t =
        picoquic_get_aes128gcm_sha256(0 as ::core::ffi::c_int);
    if (*(*cipher).hash).digest_size > ::core::mem::size_of::<[uint8_t; 256]>() as usize {
        ret = PICOQUIC_ERROR_UNEXPECTED_ERROR;
    } else {
        if !secret.is_null() && secret_length > 0 as size_t {
            memset(
                &raw mut temp_secret as *mut uint8_t as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                (*(*cipher).hash).digest_size,
            );
            memcpy(
                &raw mut temp_secret as *mut uint8_t as *mut ::core::ffi::c_void,
                secret as *const ::core::ffi::c_void,
                if secret_length > (*(*cipher).hash).digest_size {
                    (*(*cipher).hash).digest_size
                } else {
                    secret_length
                },
            );
        } else {
            (*tls_ctx).random_bytes.expect("non-null function pointer")(
                &raw mut temp_secret as *mut uint8_t as *mut ::core::ffi::c_void,
                (*(*cipher).hash).digest_size,
            );
        }
        ret = picoquic_set_aead_from_secret(
            &raw mut (*quic).aead_encrypt_ticket_ctx,
            cipher,
            1 as ::core::ffi::c_int,
            &raw mut temp_secret as *mut uint8_t as *const ::core::ffi::c_void,
            b"random label\0".as_ptr() as *const ::core::ffi::c_char,
        );
        if ret == 0 as ::core::ffi::c_int {
            ret = picoquic_set_aead_from_secret(
                &raw mut (*quic).aead_decrypt_ticket_ctx,
                cipher,
                0 as ::core::ffi::c_int,
                &raw mut temp_secret as *mut uint8_t as *const ::core::ffi::c_void,
                b"random label\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        ptls_clear_memory.expect("non-null function pointer")(
            &raw mut temp_secret as *mut uint8_t as *mut ::core::ffi::c_void,
            (*(*cipher).hash).digest_size,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_aead_integrity_limit(
    mut aead_ctx: *mut ::core::ffi::c_void,
) -> uint64_t {
    return (*(*(aead_ctx as *mut ptls_aead_context_t)).algo).integrity_limit;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_aead_confidentiality_limit(
    mut aead_ctx: *mut ::core::ffi::c_void,
) -> uint64_t {
    return (*(*(aead_ctx as *mut ptls_aead_context_t)).algo).confidentiality_limit;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_aead_decrypt_generic(
    mut output: *mut uint8_t,
    mut input: *const uint8_t,
    mut input_length: size_t,
    mut seq_num: uint64_t,
    mut auth_data: *const uint8_t,
    mut auth_data_length: size_t,
    mut aead_ctx: *mut ::core::ffi::c_void,
) -> size_t {
    let mut decrypted: size_t = 0 as size_t;
    if aead_ctx.is_null() {
        decrypted = SIZE_MAX as size_t;
    } else {
        decrypted = ptls_aead_decrypt(
            aead_ctx as *mut ptls_aead_context_t,
            output as *mut ::core::ffi::c_void,
            input as *const ::core::ffi::c_void,
            input_length,
            seq_num,
            auth_data as *mut ::core::ffi::c_void,
            auth_data_length,
        );
    }
    return decrypted;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_aead_encrypt_generic(
    mut output: *mut uint8_t,
    mut input: *const uint8_t,
    mut input_length: size_t,
    mut seq_num: uint64_t,
    mut auth_data: *const uint8_t,
    mut auth_data_length: size_t,
    mut aead_context: *mut ::core::ffi::c_void,
) -> size_t {
    let mut encrypted: size_t = 0 as size_t;
    encrypted = ptls_aead_encrypt(
        aead_context as *mut ptls_aead_context_t,
        output as *mut ::core::ffi::c_void,
        input as *const ::core::ffi::c_void,
        input_length,
        seq_num,
        auth_data as *mut ::core::ffi::c_void,
        auth_data_length,
    );
    return encrypted;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_aead_decrypt_mp(
    mut output: *mut uint8_t,
    mut input: *const uint8_t,
    mut input_length: size_t,
    mut path_id: uint64_t,
    mut seq_num: uint64_t,
    mut auth_data: *const uint8_t,
    mut auth_data_length: size_t,
    mut aead_context: *mut ::core::ffi::c_void,
) -> size_t {
    let mut decrypted: size_t = 0 as size_t;
    if aead_context.is_null() {
        decrypted = SIZE_MAX as size_t;
    } else {
        let mut seq32: [uint8_t; 4] = [0; 4];
        picoformat_32(&raw mut seq32 as *mut uint8_t, path_id as uint32_t);
        ptls_aead_xor_iv(
            aead_context as *mut ptls_aead_context_t,
            &raw mut seq32 as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
        );
        decrypted = ptls_aead_decrypt(
            aead_context as *mut ptls_aead_context_t,
            output as *mut ::core::ffi::c_void,
            input as *const ::core::ffi::c_void,
            input_length,
            seq_num,
            auth_data as *mut ::core::ffi::c_void,
            auth_data_length,
        );
        ptls_aead_xor_iv(
            aead_context as *mut ptls_aead_context_t,
            &raw mut seq32 as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
        );
    }
    return decrypted;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_aead_encrypt_mp(
    mut output: *mut uint8_t,
    mut input: *const uint8_t,
    mut input_length: size_t,
    mut path_id: uint64_t,
    mut seq_num: uint64_t,
    mut auth_data: *const uint8_t,
    mut auth_data_length: size_t,
    mut aead_context: *mut ::core::ffi::c_void,
) -> size_t {
    let mut encrypted: size_t = 0 as size_t;
    let mut seq32: [uint8_t; 4] = [0; 4];
    picoformat_32(&raw mut seq32 as *mut uint8_t, path_id as uint32_t);
    ptls_aead_xor_iv(
        aead_context as *mut ptls_aead_context_t,
        &raw mut seq32 as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
    );
    encrypted = ptls_aead_encrypt(
        aead_context as *mut ptls_aead_context_t,
        output as *mut ::core::ffi::c_void,
        input as *const ::core::ffi::c_void,
        input_length,
        seq_num,
        auth_data as *mut ::core::ffi::c_void,
        auth_data_length,
    );
    ptls_aead_xor_iv(
        aead_context as *mut ptls_aead_context_t,
        &raw mut seq32 as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
    );
    return encrypted;
}
#[no_mangle]
pub static mut picoquic_cleartext_null_salt: [uint8_t; 20] = [
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
unsafe extern "C" fn picoquic_setup_cleartext_aead_salt(
    mut version_index: size_t,
    mut salt: *mut ptls_iovec_t,
) {
    if !(*(&raw const picoquic_supported_versions as *const picoquic_version_parameters_t)
        .offset(version_index as isize))
    .version_aead_key
    .is_null()
        && (*(&raw const picoquic_supported_versions as *const picoquic_version_parameters_t)
            .offset(version_index as isize))
        .version_aead_key_length
            > 0 as size_t
    {
        (*salt).base = (*(&raw const picoquic_supported_versions
            as *const picoquic_version_parameters_t)
            .offset(version_index as isize))
        .version_aead_key;
        (*salt).len = (*(&raw const picoquic_supported_versions
            as *const picoquic_version_parameters_t)
            .offset(version_index as isize))
        .version_aead_key_length;
    } else {
        (*salt).base = &raw mut picoquic_cleartext_null_salt as *mut uint8_t;
        (*salt).len = ::core::mem::size_of::<[uint8_t; 20]>() as usize as size_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tls_stream_process(
    mut cnx: *mut picoquic_cnx_t,
    mut data_consumed: *mut ::core::ffi::c_int,
    mut current_time: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ctx: *mut picoquic_tls_ctx_t = (*cnx).tls_ctx as *mut picoquic_tls_ctx_t;
    let mut next_epoch: size_t = 0 as size_t;
    (*(*cnx).quic).cnx_in_progress = cnx as *mut st_picoquic_cnx_t;
    let mut epoch: size_t = 0 as size_t;
    while epoch < PICOQUIC_NUMBER_OF_EPOCHS as size_t && ret == 0 as ::core::ffi::c_int {
        let mut stream: *mut picoquic_stream_head_t =
            (&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t).offset(epoch as isize)
                as *mut picoquic_stream_head_t;
        let mut data: *mut picoquic_stream_data_node_t =
            picosplay_first(&raw mut (*stream).stream_data_tree)
                as *mut picoquic_stream_data_node_t;
        let mut processed: size_t = 0 as size_t;
        let mut data_pushed: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        next_epoch = ptls_get_read_epoch((*ctx).tls);
        if epoch != next_epoch {
            if epoch > next_epoch {
                break;
            }
            if !data.is_null() && (*data).offset > (*stream).consumed_offset {
                ret = picoquic_connection_error(
                    cnx,
                    PICOQUIC_TRANSPORT_PROTOCOL_VIOLATION as uint64_t,
                    0 as uint64_t,
                );
            }
        } else {
            while (ret == 0 as ::core::ffi::c_int || ret == PTLS_ERROR_IN_PROGRESS)
                && !data.is_null()
                && (*data).offset <= (*stream).consumed_offset
            {
                let mut sendbuf: st_ptls_buffer_t = st_ptls_buffer_t {
                    base: ::core::ptr::null_mut::<uint8_t>(),
                    capacity: 0,
                    off: 0,
                    is_allocated: 0,
                    align_bits: 0,
                };
                let mut start: size_t =
                    (*stream).consumed_offset.wrapping_sub((*data).offset) as size_t;
                let mut epoch_data: size_t = (*data).length.wrapping_sub(start);
                let mut send_offset: [size_t; 5] = [
                    0 as ::core::ffi::c_int as size_t,
                    0 as ::core::ffi::c_int as size_t,
                    0 as ::core::ffi::c_int as size_t,
                    0 as ::core::ffi::c_int as size_t,
                    0 as ::core::ffi::c_int as size_t,
                ];
                if !data_consumed.is_null() {
                    *data_consumed = 1 as ::core::ffi::c_int;
                }
                ptls_buffer_init(
                    &raw mut sendbuf,
                    b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_void,
                    0 as size_t,
                );
                picoquic_clear_crypto_errors();
                ret = ptls_handle_message(
                    (*ctx).tls,
                    &raw mut sendbuf,
                    &raw mut send_offset as *mut size_t,
                    epoch,
                    (*data).bytes.offset(start as isize) as *const ::core::ffi::c_void,
                    epoch_data,
                    &raw mut (*ctx).handshake_properties,
                );
                if ret == 0 as ::core::ffi::c_int
                    || ret == PTLS_ERROR_IN_PROGRESS
                    || ret == PTLS_ERROR_STATELESS_RETRY
                {
                    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                    while i < PICOQUIC_NUMBER_OF_EPOCHS {
                        if send_offset[i as usize]
                            < send_offset[(i + 1 as ::core::ffi::c_int) as usize]
                        {
                            data_pushed = 1 as ::core::ffi::c_int;
                            ret = picoquic_add_to_tls_stream(
                                cnx,
                                sendbuf.base.offset(send_offset[i as usize] as isize),
                                send_offset[(i + 1 as ::core::ffi::c_int) as usize]
                                    .wrapping_sub(send_offset[i as usize]),
                                i,
                            );
                        }
                        i += 1;
                    }
                    if (*cnx).client_mode() != 0 {
                        if (*cnx).alpn.is_null() {
                            let mut alpn: *const ::core::ffi::c_char =
                                ptls_get_negotiated_protocol((*ctx).tls);
                            if !alpn.is_null() {
                                (*cnx).alpn = picoquic_string_duplicate(alpn);
                                picoquic_log_negotiated_alpn(
                                    cnx,
                                    0 as ::core::ffi::c_int,
                                    ::core::ptr::null::<uint8_t>(),
                                    0 as size_t,
                                    alpn as *const uint8_t,
                                    strlen(alpn),
                                    ::core::ptr::null::<ptls_iovec_t>(),
                                    0 as size_t,
                                );
                                if (*cnx).callback_fn.is_some() {
                                    (*cnx).callback_fn.expect("non-null function pointer")(
                                        cnx as *mut picoquic_cnx_t,
                                        0 as uint64_t,
                                        alpn as *mut uint8_t,
                                        0 as size_t,
                                        picoquic_callback_set_alpn,
                                        (*cnx).callback_ctx,
                                        NULL_0,
                                    );
                                }
                            }
                        }
                        match (*ctx)
                            .handshake_properties
                            .c2rust_unnamed
                            .client
                            .early_data_acceptance
                            as ::core::ffi::c_uint
                        {
                            1 => {
                                (*cnx).set_zero_rtt_data_accepted(
                                    0 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                );
                            }
                            2 => {
                                (*cnx).set_zero_rtt_data_accepted(
                                    1 as ::core::ffi::c_uint as ::core::ffi::c_uint,
                                );
                            }
                            _ => {}
                        }
                    }
                } else {
                    picoquic_log_crypto_errors(cnx, ret);
                }
                (*stream).consumed_offset = ((*stream).consumed_offset as ::core::ffi::c_ulong)
                    .wrapping_add(epoch_data as ::core::ffi::c_ulong)
                    as uint64_t as uint64_t;
                processed = processed.wrapping_add(epoch_data);
                if start.wrapping_add(epoch_data) >= (*data).length {
                    picosplay_delete_hint(
                        &raw mut (*(&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t)
                            .offset(epoch as isize))
                        .stream_data_tree,
                        &raw mut (*data).stream_data_node,
                    );
                    data = picosplay_first(
                        &raw mut (*(&raw mut (*cnx).tls_stream as *mut picoquic_stream_head_t)
                            .offset(epoch as isize))
                        .stream_data_tree,
                    ) as *mut picoquic_stream_data_node_t;
                }
                ptls_buffer_dispose(&raw mut sendbuf);
            }
            if processed > 0 as size_t {
                if ret == 0 as ::core::ffi::c_int {
                    match (*cnx).cnx_state as ::core::ffi::c_uint {
                        0 | 1 | 2 | 4 | 7 => {
                            if ptls_handshake_is_complete((*ctx).tls) != 0 {
                                if (*cnx).remote_parameters_received() as ::core::ffi::c_int
                                    == 0 as ::core::ffi::c_int
                                {
                                    ret = picoquic_connection_error(
                                        cnx,
                                        PICOQUIC_TRANSPORT_PARAMETER_ERROR as uint64_t,
                                        0 as uint64_t,
                                    );
                                } else if !(*cnx).crypto_context[3 as ::core::ffi::c_int as usize]
                                    .aead_encrypt
                                    .is_null()
                                {
                                    picoquic_client_almost_ready_transition(cnx);
                                }
                            }
                        }
                        5 | 6 => {
                            if data_pushed == 0 as ::core::ffi::c_int
                                && (*((*(*cnx).quic).tls_master_ctx as *mut ptls_context_t))
                                    .require_client_authentication()
                                    as ::core::ffi::c_int
                                    == 1 as ::core::ffi::c_int
                            {
                                picoquic_false_start_transition(cnx, current_time);
                            } else if !(*cnx).crypto_context[3 as ::core::ffi::c_int as usize]
                                .aead_encrypt
                                .is_null()
                            {
                                (*cnx).cnx_state = picoquic_state_server_almost_ready;
                            }
                        }
                        3 | 10 | 8 | 9 | 13 | 12 | 11 | 14 | 15 | 16 | 17 | 18 | 19 | _ => {}
                    }
                } else if !(ret == PTLS_ERROR_IN_PROGRESS
                    && ((*cnx).cnx_state as ::core::ffi::c_uint
                        == picoquic_state_client_init as ::core::ffi::c_int as ::core::ffi::c_uint
                        || (*cnx).cnx_state as ::core::ffi::c_uint
                            == picoquic_state_client_init_sent as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                        || (*cnx).cnx_state as ::core::ffi::c_uint
                            == picoquic_state_client_init_resent as ::core::ffi::c_int
                                as ::core::ffi::c_uint))
                {
                    if ret == PTLS_ERROR_IN_PROGRESS
                        && ((*cnx).cnx_state as ::core::ffi::c_uint
                            == picoquic_state_server_init as ::core::ffi::c_int
                                as ::core::ffi::c_uint
                            || (*cnx).cnx_state as ::core::ffi::c_uint
                                == picoquic_state_server_handshake as ::core::ffi::c_int
                                    as ::core::ffi::c_uint)
                    {
                        if ptls_handshake_is_complete((*ctx).tls) != 0 {
                            (*cnx).cnx_state = picoquic_state_server_almost_ready;
                        }
                    }
                }
                if ret == 0 as ::core::ffi::c_int
                    || ret == PTLS_ERROR_IN_PROGRESS
                    || ret == PTLS_ERROR_STATELESS_RETRY
                {
                    ret = 0 as ::core::ffi::c_int;
                } else {
                    let mut error_code: uint16_t = PICOQUIC_TRANSPORT_INTERNAL_ERROR as uint16_t;
                    if ret & !(0xff as ::core::ffi::c_int) == PTLS_ERROR_CLASS_SELF_ALERT {
                        error_code = (0x100 as ::core::ffi::c_int as uint16_t as ::core::ffi::c_int
                            | (ret & 0xff as ::core::ffi::c_int) as uint16_t as ::core::ffi::c_int)
                            as uint16_t;
                    }
                    picoquic_connection_error(cnx, error_code as uint64_t, 0 as uint64_t);
                    ret = 0 as ::core::ffi::c_int;
                }
            }
        }
        epoch = epoch.wrapping_add(1);
    }
    (*(*cnx).quic).cnx_in_progress = ::core::ptr::null_mut::<st_picoquic_cnx_t>();
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_tls_complete(
    mut cnx: *mut picoquic_cnx_t,
) -> ::core::ffi::c_int {
    let mut ctx: *mut picoquic_tls_ctx_t = (*cnx).tls_ctx as *mut picoquic_tls_ctx_t;
    return ptls_handshake_is_complete((*ctx).tls);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_cnxid_reset_secret(
    mut quic: *mut picoquic_quic_t,
    mut cnx_id: *mut picoquic_connection_id_t,
    mut reset_secret: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut algo: *const ptls_hash_algorithm_t = picoquic_get_sha256();
    if algo.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        let mut hash_ctx: *mut ptls_hash_context_t =
            (*algo).create.expect("non-null function pointer")();
        let mut final_hash: [uint8_t; 64] = [0; 64];
        if hash_ctx.is_null() {
            ret = -(1 as ::core::ffi::c_int);
            memset(
                reset_secret as *mut ::core::ffi::c_void,
                0 as ::core::ffi::c_int,
                PICOQUIC_RESET_SECRET_SIZE as size_t,
            );
        } else {
            (*hash_ctx).update.expect("non-null function pointer")(
                hash_ctx as *mut st_ptls_hash_context_t,
                &raw mut (*quic).reset_seed as *mut uint8_t as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
            );
            (*hash_ctx).update.expect("non-null function pointer")(
                hash_ctx as *mut st_ptls_hash_context_t,
                cnx_id as *const ::core::ffi::c_void,
                ::core::mem::size_of::<picoquic_connection_id_t>() as size_t,
            );
            (*hash_ctx).final_0.expect("non-null function pointer")(
                hash_ctx as *mut st_ptls_hash_context_t,
                &raw mut final_hash as *mut uint8_t as *mut ::core::ffi::c_void,
                PTLS_HASH_FINAL_MODE_FREE,
            );
            memcpy(
                reset_secret as *mut ::core::ffi::c_void,
                &raw mut final_hash as *mut uint8_t as *const ::core::ffi::c_void,
                PICOQUIC_RESET_SECRET_SIZE as size_t,
            );
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_tls_certificate_chain(
    mut quic: *mut picoquic_quic_t,
    mut certs: *mut ptls_iovec_t,
    mut count: size_t,
) {
    let mut ctx: *mut ptls_context_t = (*quic).tls_master_ctx as *mut ptls_context_t;
    free_certificates_list(
        (*ctx).certificates.list as *mut ptls_iovec_t,
        (*ctx).certificates.count,
    );
    (*ctx).certificates.list = certs as *mut ptls_iovec_t;
    (*ctx).certificates.count = count;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tls_set_client_authentication(
    mut quic: *mut picoquic_quic_t,
    mut client_authentication: ::core::ffi::c_int,
) {
    let ref mut c2rust_fresh11 = *((*quic).tls_master_ctx as *mut ptls_context_t);
    (*c2rust_fresh11).set_require_client_authentication(
        client_authentication as ::core::ffi::c_uint as ::core::ffi::c_uint,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_tls_client_authentication_activated(
    mut quic: *mut picoquic_quic_t,
) -> ::core::ffi::c_int {
    return (*((*quic).tls_master_ctx as *mut ptls_context_t)).require_client_authentication()
        as ::core::ffi::c_int;
}
unsafe extern "C" fn picoquic_server_encrypt_retry_token(
    mut quic: *mut picoquic_quic_t,
    mut addr_peer: *const sockaddr,
    mut is_new_token: ::core::ffi::c_int,
    mut token: *mut uint8_t,
    mut token_length: *mut size_t,
    mut token_max: size_t,
    mut text: *const uint8_t,
    mut text_length: size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sequence: uint64_t = 0;
    let mut auth_data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut auth_data_length: size_t = 0;
    if text_length
        .wrapping_add(1 as size_t)
        .wrapping_add(16 as size_t)
        > token_max
    {
        ret = -(1 as ::core::ffi::c_int);
        *token_length = 0 as size_t;
    } else {
        if (*addr_peer).sa_family as ::core::ffi::c_int == AF_INET {
            auth_data = &raw mut (*(addr_peer as *mut sockaddr_in)).sin_addr as *mut uint8_t;
            auth_data_length = 4 as size_t;
        } else {
            auth_data = &raw mut (*(addr_peer as *mut sockaddr_in6)).sin6_addr as *mut uint8_t;
            auth_data_length = 16 as size_t;
        }
        picoquic_crypto_random(quic, token as *mut ::core::ffi::c_void, 8 as size_t);
        if is_new_token != 0 {
            let ref mut c2rust_fresh13 = *token.offset(0 as ::core::ffi::c_int as isize);
            *c2rust_fresh13 =
                (*c2rust_fresh13 as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as uint8_t;
        } else {
            let ref mut c2rust_fresh14 = *token.offset(0 as ::core::ffi::c_int as isize);
            *c2rust_fresh14 =
                (*c2rust_fresh14 as ::core::ffi::c_int & 0x7f as ::core::ffi::c_int) as uint8_t;
        }
        sequence = (((((*token.offset(0 as ::core::ffi::c_int as isize) as uint16_t
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *token.offset(1 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
            as uint32_t)
            << 16 as ::core::ffi::c_int
            | ((*token
                .offset(2 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *token
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t) as uint64_t)
            << 32 as ::core::ffi::c_int
            | ((((*token
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *token
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t)
                << 16 as ::core::ffi::c_int
                | ((*token
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *token
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int) as uint32_t) as uint64_t;
        *token_length =
            (8 as ::core::ffi::c_uint as size_t).wrapping_add(picoquic_aead_encrypt_generic(
                token.offset(8 as ::core::ffi::c_int as isize),
                text,
                text_length,
                sequence,
                auth_data,
                auth_data_length,
                (*quic).aead_encrypt_ticket_ctx,
            ));
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_server_decrypt_retry_token(
    mut quic: *mut picoquic_quic_t,
    mut addr_peer: *const sockaddr,
    mut is_new_token: *mut ::core::ffi::c_int,
    mut token: *const uint8_t,
    mut token_length: size_t,
    mut text: *mut uint8_t,
    mut text_length: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut sequence: uint64_t = 0;
    let mut auth_data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut auth_data_length: size_t = 0;
    if (*addr_peer).sa_family as ::core::ffi::c_int == AF_INET {
        auth_data = &raw mut (*(addr_peer as *mut sockaddr_in)).sin_addr as *mut uint8_t;
        auth_data_length = 4 as size_t;
    } else {
        auth_data = &raw mut (*(addr_peer as *mut sockaddr_in6)).sin6_addr as *mut uint8_t;
        auth_data_length = 16 as size_t;
    }
    if token_length < 8 as size_t {
        *is_new_token = 0 as ::core::ffi::c_int;
        ret = -(1 as ::core::ffi::c_int);
    } else {
        *is_new_token = if *token.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x80 as ::core::ffi::c_int
            == 0 as ::core::ffi::c_int
        {
            0 as ::core::ffi::c_int
        } else {
            1 as ::core::ffi::c_int
        };
        sequence = (((((*token.offset(0 as ::core::ffi::c_int as isize) as uint16_t
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *token.offset(1 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
            as uint32_t)
            << 16 as ::core::ffi::c_int
            | ((*token
                .offset(2 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *token
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t) as uint64_t)
            << 32 as ::core::ffi::c_int
            | ((((*token
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *token
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t)
                << 16 as ::core::ffi::c_int
                | ((*token
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *token
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int) as uint32_t) as uint64_t;
        *text_length = picoquic_aead_decrypt_generic(
            text,
            token.offset(8 as ::core::ffi::c_int as isize),
            token_length.wrapping_sub(8 as size_t),
            sequence,
            auth_data,
            auth_data_length,
            (*quic).aead_decrypt_ticket_ctx,
        );
        if *text_length >= token_length.wrapping_sub(8 as size_t) {
            ret = -(1 as ::core::ffi::c_int);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_prepare_retry_token(
    mut quic: *mut picoquic_quic_t,
    mut addr_peer: *const sockaddr,
    mut current_time: uint64_t,
    mut odcid: *const picoquic_connection_id_t,
    mut rcid: *const picoquic_connection_id_t,
    mut initial_pn: uint32_t,
    mut token: *mut uint8_t,
    mut token_max: size_t,
    mut token_size: *mut size_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut text: [uint8_t; 128] = [0; 128];
    let mut token_time: uint64_t = current_time;
    let mut bytes: *mut uint8_t = &raw mut text as *mut uint8_t;
    let mut bytes_max: *mut uint8_t = (&raw mut text as *mut uint8_t)
        .offset(::core::mem::size_of::<[uint8_t; 128]>() as usize as isize);
    if (*odcid).id_len as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        token_time = (token_time as ::core::ffi::c_ulonglong).wrapping_add(
            (24 as ::core::ffi::c_ulonglong)
                .wrapping_mul(3600 as ::core::ffi::c_ulonglong)
                .wrapping_mul(1000000 as ::core::ffi::c_ulonglong),
        ) as uint64_t as uint64_t;
    } else {
        token_time = (token_time as ::core::ffi::c_ulonglong)
            .wrapping_add(4000000 as ::core::ffi::c_ulonglong) as uint64_t
            as uint64_t;
    }
    bytes = picoquic_frames_uint64_encode(bytes, bytes_max, token_time);
    if !bytes.is_null()
        && {
            bytes = picoquic_frames_cid_encode(bytes, bytes_max, odcid);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_cid_encode(bytes, bytes_max, rcid);
            !bytes.is_null()
        }
        && {
            bytes = picoquic_frames_varint_encode(bytes, bytes_max, initial_pn as uint64_t);
            !bytes.is_null()
        }
    {
        while bytes < (&raw mut text as *mut uint8_t).offset(PICOQUIC_RETRY_TOKEN_PAD_SIZE as isize)
        {
            let c2rust_fresh12 = bytes;
            bytes = bytes.offset(1);
            *c2rust_fresh12 = 0 as uint8_t;
        }
        ret = picoquic_server_encrypt_retry_token(
            quic,
            addr_peer,
            ((*odcid).id_len as ::core::ffi::c_int == 0 as ::core::ffi::c_int)
                as ::core::ffi::c_int,
            token,
            token_size,
            token_max,
            &raw mut text as *mut uint8_t,
            bytes.offset_from(&raw mut text as *mut uint8_t) as ::core::ffi::c_long as size_t,
        );
    } else {
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_verify_retry_token(
    mut quic: *mut picoquic_quic_t,
    mut addr_peer: *const sockaddr,
    mut current_time: uint64_t,
    mut is_new_token: *mut ::core::ffi::c_int,
    mut odcid: *mut picoquic_connection_id_t,
    mut rcid: *const picoquic_connection_id_t,
    mut initial_pn: uint32_t,
    mut token: *const uint8_t,
    mut token_size: size_t,
    mut check_reuse: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut text: [uint8_t; 128] = [0; 128];
    let mut text_len: size_t = 0 as size_t;
    let mut cid: picoquic_connection_id_t = st_picoquic_connection_id_t {
        id: [0; 20],
        id_len: 0,
    };
    let mut token_pn: uint64_t = 0;
    (*odcid).id_len = 0 as uint8_t;
    if token_size > ::core::mem::size_of::<[uint8_t; 128]>() as usize {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        ret = picoquic_server_decrypt_retry_token(
            quic,
            addr_peer,
            is_new_token,
            token,
            token_size,
            &raw mut text as *mut uint8_t,
            &raw mut text_len,
        );
    }
    if ret == 0 as ::core::ffi::c_int {
        let mut bytes: *const uint8_t = &raw mut text as *mut uint8_t;
        let mut bytes_max: *const uint8_t =
            (&raw mut text as *mut uint8_t).offset(text_len as isize);
        let mut token_time: uint64_t = (((((text[0 as ::core::ffi::c_int as usize] as uint16_t
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | text[1 as ::core::ffi::c_int as usize] as uint16_t as ::core::ffi::c_int)
            as uint32_t)
            << 16 as ::core::ffi::c_int
            | ((*(&raw mut text as *mut uint8_t)
                .offset(2 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(&raw mut text as *mut uint8_t)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t)
            as uint64_t)
            << 32 as ::core::ffi::c_int
            | ((((*(&raw mut text as *mut uint8_t)
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *(&raw mut text as *mut uint8_t)
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t)
                << 16 as ::core::ffi::c_int
                | ((*(&raw mut text as *mut uint8_t)
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *(&raw mut text as *mut uint8_t)
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int) as uint32_t) as uint64_t;
        bytes = picoquic_frames_uint64_decode(bytes, bytes_max, &raw mut token_time);
        if !bytes.is_null()
            && {
                bytes = picoquic_frames_cid_decode(bytes, bytes_max, odcid);
                !bytes.is_null()
            }
            && {
                bytes = picoquic_frames_cid_decode(bytes, bytes_max, &raw mut cid);
                !bytes.is_null()
            }
            && {
                bytes = picoquic_frames_varint_decode(bytes, bytes_max, &raw mut token_pn);
                !bytes.is_null()
            }
        {
            if token_time < current_time {
                ret = -(1 as ::core::ffi::c_int);
            } else if initial_pn != UINT32_MAX as uint32_t
                && (*odcid).id_len as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                && token_pn >= initial_pn as uint64_t
            {
                ret = -(1 as ::core::ffi::c_int);
            } else {
                picoquic_registered_token_clear(quic, current_time);
                if check_reuse != 0 && {
                    ret =
                        picoquic_registered_token_check_reuse(quic, token, token_size, token_time);
                    ret != 0 as ::core::ffi::c_int
                } {
                    picoquic_log_context_free_app_message(
                        quic,
                        rcid,
                        b"Duplicate token test returns %d\0".as_ptr() as *const ::core::ffi::c_char,
                        ret,
                    );
                } else if (*odcid).id_len as ::core::ffi::c_int > 0 as ::core::ffi::c_int
                    && picoquic_compare_connection_id(rcid, &raw mut cid) != 0 as ::core::ffi::c_int
                {
                    ret = -(1 as ::core::ffi::c_int);
                }
            }
        } else {
            *odcid = picoquic_null_connection_id;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cid_free_under_mask_ctx(mut v_cid_enc: *mut ::core::ffi::c_void) {
    if !v_cid_enc.is_null() {
        ptls_cipher_free(v_cid_enc as *mut ptls_cipher_context_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cid_get_under_mask_ctx(
    mut v_cid_enc: *mut *mut ::core::ffi::c_void,
    mut secret: *const ::core::ffi::c_void,
    mut prefix_label: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut cidkey: [uint8_t; 32] = [0; 32];
    let mut long_secret: [uint8_t; 64] = [0; 64];
    let mut cipher: *const ptls_cipher_suite_t =
        picoquic_get_aes128gcm_sha256(1 as ::core::ffi::c_int);
    let mut ret: ::core::ffi::c_int = 0;
    picoquic_cid_free_under_mask_ctx(*v_cid_enc);
    *v_cid_enc = NULL_0;
    memset(
        &raw mut long_secret as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    memcpy(
        &raw mut long_secret as *mut uint8_t as *mut ::core::ffi::c_void,
        secret,
        16 as size_t,
    );
    ret = ptls_hkdf_expand_label(
        (*cipher).hash,
        &raw mut cidkey as *mut uint8_t as *mut ::core::ffi::c_void,
        (*(*(*cipher).aead).ctr_cipher).key_size,
        ptls_iovec_init(
            &raw mut long_secret as *mut uint8_t as *const ::core::ffi::c_void,
            (*(*cipher).hash).digest_size,
        ),
        PICOQUIC_LABEL_CID.as_ptr(),
        ptls_iovec_init(::core::ptr::null::<::core::ffi::c_void>(), 0 as size_t),
        prefix_label,
    );
    if ret == 0 as ::core::ffi::c_int {
        *v_cid_enc = ptls_cipher_new(
            (*(*cipher).aead).ctr_cipher,
            1 as ::core::ffi::c_int,
            &raw mut cidkey as *mut uint8_t as *const ::core::ffi::c_void,
        ) as *mut ::core::ffi::c_void;
        if (*v_cid_enc).is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cid_encrypt_under_mask(
    mut cid_enc: *mut ::core::ffi::c_void,
    mut cid_in: *const picoquic_connection_id_t,
    mut mask: *const picoquic_connection_id_t,
    mut cid_out: *mut picoquic_connection_id_t,
) {
    let mut unmasked: [uint8_t; 18] = [0; 18];
    let mut val: [uint8_t; 18] = [0; 18];
    memset(
        &raw mut unmasked as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        18 as size_t,
    );
    memset(
        &raw mut val as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        18 as size_t,
    );
    let mut i: uint8_t = 0 as uint8_t;
    while (i as ::core::ffi::c_int) < (*cid_in).id_len as ::core::ffi::c_int {
        unmasked[i as usize] = ((*cid_in).id[i as usize] as ::core::ffi::c_int
            & (*mask).id[i as usize] as ::core::ffi::c_int)
            as uint8_t;
        i = i.wrapping_add(1);
    }
    ptls_cipher_init(
        cid_enc as *mut ptls_cipher_context_t,
        &raw mut unmasked as *mut uint8_t as *const ::core::ffi::c_void,
    );
    ptls_cipher_encrypt(
        cid_enc as *mut ptls_cipher_context_t,
        &raw mut val as *mut uint8_t as *mut ::core::ffi::c_void,
        &raw mut val as *mut uint8_t as *const ::core::ffi::c_void,
        (*cid_in).id_len as size_t,
    );
    let mut i_0: uint8_t = 0 as uint8_t;
    while (i_0 as ::core::ffi::c_int) < (*cid_in).id_len as ::core::ffi::c_int {
        (*cid_out).id[i_0 as usize] = ((*cid_in).id[i_0 as usize] as ::core::ffi::c_int
            ^ val[i_0 as usize] as ::core::ffi::c_int
                & !((*mask).id[i_0 as usize] as ::core::ffi::c_int))
            as uint8_t;
        i_0 = i_0.wrapping_add(1);
    }
    (*cid_out).id_len = (*cid_in).id_len;
    if ((*cid_out).id_len as ::core::ffi::c_int) < 18 as ::core::ffi::c_int {
        memset(
            (&raw mut (*cid_out).id as *mut uint8_t)
                .offset((*cid_out).id_len as ::core::ffi::c_int as isize)
                as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (18 as ::core::ffi::c_int - (*cid_out).id_len as ::core::ffi::c_int) as size_t,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cid_decrypt_under_mask(
    mut cid_enc: *mut ::core::ffi::c_void,
    mut cid_in: *const picoquic_connection_id_t,
    mut mask: *const picoquic_connection_id_t,
    mut cid_out: *mut picoquic_connection_id_t,
) {
    picoquic_cid_encrypt_under_mask(cid_enc, cid_in, mask, cid_out);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_retry_protection_context(
    mut is_enc: ::core::ffi::c_int,
    mut key: *mut uint8_t,
    mut prefix_label: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_void {
    return picoquic_setup_test_aead_context(is_enc, key, prefix_label);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_find_retry_protection_context(
    mut quic: *mut picoquic_quic_t,
    mut version_index: ::core::ffi::c_int,
    mut sending: ::core::ffi::c_int,
) -> *mut ::core::ffi::c_void {
    let mut aead_ctx: *mut ::core::ffi::c_void = NULL_0;
    let mut aead_vector: *mut *mut ::core::ffi::c_void = if sending != 0 {
        (*quic).retry_integrity_sign_ctx
    } else {
        (*quic).retry_integrity_verify_ctx
    };
    if !(*(&raw const picoquic_supported_versions as *const picoquic_version_parameters_t)
        .offset(version_index as isize))
    .version_retry_key
    .is_null()
    {
        if aead_vector.is_null() {
            if sending != 0 {
                (*quic).retry_integrity_sign_ctx = malloc(
                    (::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
                        .wrapping_mul(picoquic_nb_supported_versions),
                )
                    as *mut *mut ::core::ffi::c_void;
                aead_vector = (*quic).retry_integrity_sign_ctx;
            } else {
                (*quic).retry_integrity_verify_ctx = malloc(
                    (::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
                        .wrapping_mul(picoquic_nb_supported_versions),
                )
                    as *mut *mut ::core::ffi::c_void;
                aead_vector = (*quic).retry_integrity_verify_ctx;
            }
            if !aead_vector.is_null() {
                memset(
                    aead_vector as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    (::core::mem::size_of::<*mut ::core::ffi::c_void>() as size_t)
                        .wrapping_mul(picoquic_nb_supported_versions),
                );
            }
        }
        if !aead_vector.is_null() {
            aead_ctx = *aead_vector.offset(version_index as isize);
            if aead_ctx.is_null() {
                aead_ctx = picoquic_create_retry_protection_context(
                    sending,
                    (*(&raw const picoquic_supported_versions
                        as *const picoquic_version_parameters_t)
                        .offset(version_index as isize))
                    .version_retry_key,
                    (*(&raw const picoquic_supported_versions
                        as *const picoquic_version_parameters_t)
                        .offset(version_index as isize))
                    .tls_prefix_label,
                );
                let ref mut c2rust_fresh15 = *aead_vector.offset(version_index as isize);
                *c2rust_fresh15 = aead_ctx;
            }
        }
    }
    return aead_ctx;
}
unsafe extern "C" fn picoquic_delete_one_retry_protection_context(
    mut ctx: *mut *mut ::core::ffi::c_void,
) -> *mut *mut ::core::ffi::c_void {
    if !ctx.is_null() {
        let mut i: size_t = 0 as size_t;
        while i < picoquic_nb_supported_versions {
            if !(*ctx.offset(i as isize)).is_null() {
                picoquic_aead_free(*ctx.offset(i as isize));
            }
            i = i.wrapping_add(1);
        }
        free(ctx as *mut ::core::ffi::c_void);
    }
    return ::core::ptr::null_mut::<*mut ::core::ffi::c_void>();
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_retry_protection_contexts(mut quic: *mut picoquic_quic_t) {
    (*quic).retry_integrity_sign_ctx =
        picoquic_delete_one_retry_protection_context((*quic).retry_integrity_sign_ctx);
    (*quic).retry_integrity_verify_ctx =
        picoquic_delete_one_retry_protection_context((*quic).retry_integrity_verify_ctx);
}
unsafe extern "C" fn picoquic_format_retry_protection_pseudo_packet(
    mut pseudo_packet: *mut uint8_t,
    mut bytes: *mut uint8_t,
    mut byte_index: size_t,
    mut odcid: *const picoquic_connection_id_t,
) -> size_t {
    let mut pseudo_index: size_t = 0 as size_t;
    if byte_index
        .wrapping_add((*odcid).id_len as size_t)
        .wrapping_add(1 as size_t)
        < PICOQUIC_MAX_PACKET_SIZE as size_t
    {
        let c2rust_fresh16 = pseudo_index;
        pseudo_index = pseudo_index.wrapping_add(1);
        *pseudo_packet.offset(c2rust_fresh16 as isize) = (*odcid).id_len;
        memcpy(
            pseudo_packet.offset(pseudo_index as isize) as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw const (*odcid).id as *const uint8_t as *const ::core::ffi::c_void,
            (*odcid).id_len as size_t,
        );
        pseudo_index = pseudo_index.wrapping_add((*odcid).id_len as size_t);
        memcpy(
            pseudo_packet.offset(pseudo_index as isize) as *mut uint8_t as *mut ::core::ffi::c_void,
            bytes as *const ::core::ffi::c_void,
            byte_index,
        );
        pseudo_index = pseudo_index.wrapping_add(byte_index);
    }
    return pseudo_index;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_encode_retry_protection(
    mut integrity_aead: *mut ::core::ffi::c_void,
    mut bytes: *mut uint8_t,
    mut bytes_max: size_t,
    mut byte_index: size_t,
    mut odcid: *const picoquic_connection_id_t,
) -> size_t {
    let mut pseudo_index: size_t = 0;
    let mut pseudo_packet: [uint8_t; 1536] = [0; 1536];
    if !integrity_aead.is_null()
        && byte_index.wrapping_add(picoquic_aead_get_checksum_length(integrity_aead)) < bytes_max
        && {
            pseudo_index = picoquic_format_retry_protection_pseudo_packet(
                &raw mut pseudo_packet as *mut uint8_t,
                bytes,
                byte_index,
                odcid,
            );
            pseudo_index > 0 as size_t
        }
    {
        byte_index = byte_index.wrapping_add(picoquic_aead_encrypt_generic(
            bytes.offset(byte_index as isize),
            bytes.offset(byte_index as isize),
            0 as size_t,
            0 as uint64_t,
            &raw mut pseudo_packet as *mut uint8_t,
            pseudo_index,
            integrity_aead,
        ));
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_verify_retry_protection(
    mut integrity_aead: *mut ::core::ffi::c_void,
    mut bytes: *mut uint8_t,
    mut length: *mut size_t,
    mut byte_index: size_t,
    mut odcid: *const picoquic_connection_id_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = PICOQUIC_ERROR_AEAD_CHECK;
    let mut pseudo_index: size_t = 0;
    let mut pseudo_packet: [uint8_t; 1536] = [0; 1536];
    let mut decoded: [uint8_t; 1536] = [0; 1536];
    let mut checksum_length: size_t = picoquic_aead_get_checksum_length(integrity_aead);
    if byte_index.wrapping_add(checksum_length) < *length {
        *length = (*length).wrapping_sub(checksum_length);
        pseudo_index = picoquic_format_retry_protection_pseudo_packet(
            &raw mut pseudo_packet as *mut uint8_t,
            bytes,
            *length,
            odcid,
        );
        if pseudo_index > 0 as size_t
            && picoquic_aead_decrypt_generic(
                &raw mut decoded as *mut uint8_t,
                bytes.offset(*length as isize),
                checksum_length,
                0 as uint64_t,
                &raw mut pseudo_packet as *mut uint8_t,
                pseudo_index,
                integrity_aead,
            ) == 0 as size_t
        {
            ret = 0 as ::core::ffi::c_int;
        }
    }
    return ret;
}

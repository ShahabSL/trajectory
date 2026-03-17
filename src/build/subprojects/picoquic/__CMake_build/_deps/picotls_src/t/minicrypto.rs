use ::c2rust_bitfields;
extern "C" {
    pub type st_ptls_t;
    pub type st_ptls_key_schedule_t;
    pub type st_ptls_traffic_protection_t;
    pub type uECC_Curve_t;
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn _ok(cond: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
    fn done_testing() -> ::core::ffi::c_int;
    fn enter_subtest(name: *const ::core::ffi::c_char);
    fn exit_subtest(name: *const ::core::ffi::c_char);
    fn ptls_buffer__release_memory(buf: *mut ptls_buffer_t);
    fn ptls_buffer__do_pushv(
        buf: *mut ptls_buffer_t,
        src: *const ::core::ffi::c_void,
        len: size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_buffer__adjust_asn1_blocksize(
        buf: *mut ptls_buffer_t,
        body_size: size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_buffer_push_asn1_ubigint(
        buf: *mut ptls_buffer_t,
        bignum: *const ::core::ffi::c_void,
        size: size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_client_new(ctx_0: *mut ptls_context_t) -> *mut ptls_t;
    fn ptls_server_new(ctx_0: *mut ptls_context_t) -> *mut ptls_t;
    fn ptls_free(tls: *mut ptls_t);
    fn ptls_handshake(
        tls: *mut ptls_t,
        sendbuf: *mut ptls_buffer_t,
        input: *const ::core::ffi::c_void,
        inlen: *mut size_t,
        args: *mut ptls_handshake_properties_t,
    ) -> ::core::ffi::c_int;
    fn ptls_receive(
        tls: *mut ptls_t,
        plaintextbuf: *mut ptls_buffer_t,
        input: *const ::core::ffi::c_void,
        len: *mut size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_send(
        tls: *mut ptls_t,
        sendbuf: *mut ptls_buffer_t,
        input: *const ::core::ffi::c_void,
        inlen: size_t,
    ) -> ::core::ffi::c_int;
    static mut ptls_clear_memory:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
    static mut ptls_get_time: ptls_get_time_t;
    fn ptls_minicrypto_random_bytes(buf: *mut ::core::ffi::c_void, len: size_t);
    static ptls_minicrypto_x25519: ptls_key_exchange_algorithm_t;
    static ptls_minicrypto_chacha20: ptls_cipher_algorithm_t;
    static ptls_minicrypto_aes128ctr: ptls_cipher_algorithm_t;
    static ptls_minicrypto_chacha20poly1305sha256: ptls_cipher_suite_t;
    static ptls_minicrypto_aes128gcmsha256: ptls_cipher_suite_t;
    static ptls_minicrypto_aes256gcmsha384: ptls_cipher_suite_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn cf_sha256_init(ctx_0: *mut cf_sha256_context);
    fn cf_sha256_update(
        ctx_0: *mut cf_sha256_context,
        data: *const ::core::ffi::c_void,
        nbytes: size_t,
    );
    fn cf_sha256_digest_final(ctx_0: *mut cf_sha256_context, hash: *mut uint8_t);
    fn uECC_secp256r1() -> uECC_Curve;
    fn uECC_make_key(
        public_key: *mut uint8_t,
        private_key: *mut uint8_t,
        curve: uECC_Curve,
    ) -> ::core::ffi::c_int;
    fn uECC_shared_secret(
        public_key: *const uint8_t,
        private_key: *const uint8_t,
        secret: *mut uint8_t,
        curve: uECC_Curve,
    ) -> ::core::ffi::c_int;
    fn uECC_sign(
        private_key: *const uint8_t,
        message_hash: *const uint8_t,
        hash_size: ::core::ffi::c_uint,
        signature: *mut uint8_t,
        curve: uECC_Curve,
    ) -> ::core::ffi::c_int;
    fn ptls_ffx_setup_crypto(
        _ctx: *mut ptls_cipher_context_t,
        algo: *const ptls_cipher_algorithm_t,
        is_enc: ::core::ffi::c_int,
        nb_rounds: ::core::ffi::c_int,
        bit_length: size_t,
        key: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    static mut ctx: *mut ptls_context_t;
    static mut ctx_peer: *mut ptls_context_t;
    static mut ffx_variants: [st_ptls_ffx_test_variants_t; 7];
    fn test_key_exchange(
        client: *const ptls_key_exchange_algorithm_t,
        server: *const ptls_key_exchange_algorithm_t,
    );
    fn test_picotls();
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __ssize_t = ::core::ffi::c_long;
pub type ssize_t = __ssize_t;
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
pub struct st_ptls_minicrypto_secp256r1sha256_sign_certificate_t {
    pub super_0: ptls_sign_certificate_t,
    pub key: [uint8_t; 32],
}
pub type ptls_minicrypto_secp256r1sha256_sign_certificate_t =
    st_ptls_minicrypto_secp256r1sha256_sign_certificate_t;
pub type uECC_Curve = *const uECC_Curve_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_sha256_context {
    pub H: [uint32_t; 8],
    pub partial: [uint8_t; 64],
    pub blocks: uint32_t,
    pub npartial: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_secp256r1_key_exhchange_t {
    pub super_0: ptls_key_exchange_context_t,
    pub priv_0: [uint8_t; 32],
    pub pub_0: [uint8_t; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_ffx_context_t {
    pub super_0: ptls_cipher_context_t,
    pub enc_ctx: *mut ptls_cipher_context_t,
    pub nb_rounds: ::core::ffi::c_int,
    pub is_enc: ::core::ffi::c_int,
    pub byte_length: size_t,
    pub nb_left: size_t,
    pub nb_right: size_t,
    pub mask_last_byte: uint8_t,
    pub tweaks: [uint8_t; 16],
}
pub type ptls_ffx_context_t = st_ptls_ffx_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_ffx_test_variants_t {
    pub algo: *const ptls_cipher_algorithm_t,
    pub bit_length: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const NULL_0: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const PTLS_GROUP_SECP256R1: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const PTLS_GROUP_NAME_SECP256R1: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"secp256r1\0") };
pub const PTLS_SIGNATURE_ECDSA_SECP256R1_SHA256: ::core::ffi::c_int = 0x403 as ::core::ffi::c_int;
pub const PTLS_ERROR_CLASS_INTERNAL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const PTLS_ALERT_HANDSHAKE_FAILURE: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const PTLS_ALERT_DECRYPT_ERROR: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const PTLS_ERROR_NO_MEMORY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 1 as ::core::ffi::c_int;
pub const PTLS_ERROR_INCOMPATIBLE_KEY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 4 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn ptls_new(
    mut ctx_0: *mut ptls_context_t,
    mut is_server: ::core::ffi::c_int,
) -> *mut ptls_t {
    return if is_server != 0 {
        ptls_server_new(ctx_0)
    } else {
        ptls_client_new(ctx_0)
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
pub const SECP256R1_PRIVATE_KEY_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const SECP256R1_PUBLIC_KEY_SIZE: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const SECP256R1_SHARED_SECRET_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
#[no_mangle]
pub static mut ptls_minicrypto_cipher_suites: [*const ptls_cipher_suite_t; 4] = unsafe {
    [
        &raw const ptls_minicrypto_aes256gcmsha384,
        &raw const ptls_minicrypto_aes128gcmsha256,
        &raw const ptls_minicrypto_chacha20poly1305sha256,
        ::core::ptr::null::<ptls_cipher_suite_t>(),
    ]
};
#[no_mangle]
pub static mut ptls_minicrypto_cipher_suites_all: [*const ptls_cipher_suite_t; 4] = unsafe {
    [
        &raw const ptls_minicrypto_aes256gcmsha384,
        &raw const ptls_minicrypto_aes128gcmsha256,
        &raw const ptls_minicrypto_chacha20poly1305sha256,
        ::core::ptr::null::<ptls_cipher_suite_t>(),
    ]
};
pub const TYPE_UNCOMPRESSED_PUBLIC_KEY: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn secp256r1_on_exchange(
    mut _ctx: *mut *mut ptls_key_exchange_context_t,
    mut release: ::core::ffi::c_int,
    mut secret: *mut ptls_iovec_t,
    mut peerkey: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut ctx_0: *mut st_secp256r1_key_exhchange_t = *_ctx as *mut st_secp256r1_key_exhchange_t;
    let mut secbytes: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut ret: ::core::ffi::c_int = 0;
    if secret.is_null() {
        ret = 0 as ::core::ffi::c_int;
    } else if peerkey.len != SECP256R1_PUBLIC_KEY_SIZE as size_t
        || *peerkey.base.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != TYPE_UNCOMPRESSED_PUBLIC_KEY
    {
        ret = PTLS_ALERT_DECRYPT_ERROR;
    } else {
        secbytes = malloc(SECP256R1_SHARED_SECRET_SIZE as size_t) as *mut uint8_t;
        if secbytes.is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        } else if uECC_shared_secret(
            peerkey.base.offset(1 as ::core::ffi::c_int as isize),
            &raw mut (*ctx_0).priv_0 as *mut uint8_t,
            secbytes,
            uECC_secp256r1(),
        ) == 0
        {
            ret = PTLS_ALERT_DECRYPT_ERROR;
        } else {
            *secret = ptls_iovec_init(
                secbytes as *const ::core::ffi::c_void,
                SECP256R1_SHARED_SECRET_SIZE as size_t,
            );
            ret = 0 as ::core::ffi::c_int;
        }
    }
    if ret != 0 as ::core::ffi::c_int {
        free(secbytes as *mut ::core::ffi::c_void);
    }
    if release != 0 {
        ptls_clear_memory.expect("non-null function pointer")(
            &raw mut (*ctx_0).priv_0 as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        );
        free(ctx_0 as *mut ::core::ffi::c_void);
        *_ctx = ::core::ptr::null_mut::<ptls_key_exchange_context_t>();
    }
    return ret;
}
unsafe extern "C" fn secp256r1_create_key_exchange(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut _ctx: *mut *mut ptls_key_exchange_context_t,
) -> ::core::ffi::c_int {
    let mut ctx_0: *mut st_secp256r1_key_exhchange_t =
        ::core::ptr::null_mut::<st_secp256r1_key_exhchange_t>();
    ctx_0 = malloc(::core::mem::size_of::<st_secp256r1_key_exhchange_t>() as size_t)
        as *mut st_secp256r1_key_exhchange_t;
    if ctx_0.is_null() {
        return PTLS_ERROR_NO_MEMORY;
    }
    (*ctx_0).super_0 = st_ptls_key_exchange_context_t {
        algo: algo as *const st_ptls_key_exchange_algorithm_t,
        pubkey: ptls_iovec_init(
            &raw mut (*ctx_0).pub_0 as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 65]>() as size_t,
        ),
        on_exchange: Some(
            secp256r1_on_exchange
                as unsafe extern "C" fn(
                    *mut *mut ptls_key_exchange_context_t,
                    ::core::ffi::c_int,
                    *mut ptls_iovec_t,
                    ptls_iovec_t,
                ) -> ::core::ffi::c_int,
        ),
    };
    (*ctx_0).pub_0[0 as ::core::ffi::c_int as usize] = TYPE_UNCOMPRESSED_PUBLIC_KEY as uint8_t;
    uECC_make_key(
        (&raw mut (*ctx_0).pub_0 as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize),
        &raw mut (*ctx_0).priv_0 as *mut uint8_t,
        uECC_secp256r1(),
    );
    *_ctx = &raw mut (*ctx_0).super_0;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn secp256r1_key_exchange(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut pubkey: *mut ptls_iovec_t,
    mut secret: *mut ptls_iovec_t,
    mut peerkey: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut priv_0: [uint8_t; 32] = [0; 32];
    let mut pub_0: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut secbytes: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut ret: ::core::ffi::c_int = 0;
    if peerkey.len != SECP256R1_PUBLIC_KEY_SIZE as size_t
        || *peerkey.base.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != TYPE_UNCOMPRESSED_PUBLIC_KEY
    {
        ret = PTLS_ALERT_DECRYPT_ERROR;
    } else {
        pub_0 = malloc(SECP256R1_PUBLIC_KEY_SIZE as size_t) as *mut uint8_t;
        if pub_0.is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        } else {
            secbytes = malloc(SECP256R1_SHARED_SECRET_SIZE as size_t) as *mut uint8_t;
            if secbytes.is_null() {
                ret = PTLS_ERROR_NO_MEMORY;
            } else {
                *pub_0.offset(0 as ::core::ffi::c_int as isize) =
                    TYPE_UNCOMPRESSED_PUBLIC_KEY as uint8_t;
                uECC_make_key(
                    pub_0.offset(1 as ::core::ffi::c_int as isize),
                    &raw mut priv_0 as *mut uint8_t,
                    uECC_secp256r1(),
                );
                if uECC_shared_secret(
                    peerkey.base.offset(1 as ::core::ffi::c_int as isize),
                    &raw mut priv_0 as *mut uint8_t,
                    secbytes,
                    uECC_secp256r1(),
                ) == 0
                {
                    ret = PTLS_ALERT_DECRYPT_ERROR;
                } else {
                    *pubkey = ptls_iovec_init(
                        pub_0 as *const ::core::ffi::c_void,
                        SECP256R1_PUBLIC_KEY_SIZE as size_t,
                    );
                    *secret = ptls_iovec_init(
                        secbytes as *const ::core::ffi::c_void,
                        SECP256R1_SHARED_SECRET_SIZE as size_t,
                    );
                    ret = 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut priv_0 as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    if ret != 0 as ::core::ffi::c_int {
        free(secbytes as *mut ::core::ffi::c_void);
        free(pub_0 as *mut ::core::ffi::c_void);
    }
    return ret;
}
unsafe extern "C" fn secp256r1sha256_sign(
    mut _self: *mut ptls_sign_certificate_t,
    mut tls: *mut ptls_t,
    mut async_0: *mut *mut ptls_async_job_t,
    mut selected_algorithm: *mut uint16_t,
    mut outbuf: *mut ptls_buffer_t,
    mut input: ptls_iovec_t,
    mut algorithms: *const uint16_t,
    mut num_algorithms: size_t,
) -> ::core::ffi::c_int {
    let mut c2rust_current_block: u64;
    let mut self_0: *mut ptls_minicrypto_secp256r1sha256_sign_certificate_t =
        _self as *mut ptls_minicrypto_secp256r1sha256_sign_certificate_t;
    let mut hash: [uint8_t; 32] = [0; 32];
    let mut sig: [uint8_t; 64] = [0; 64];
    let mut i: size_t = 0;
    let mut ret: ::core::ffi::c_int = 0;
    i = 0 as size_t;
    while i != num_algorithms {
        if *algorithms.offset(i as isize) as ::core::ffi::c_int
            == PTLS_SIGNATURE_ECDSA_SECP256R1_SHA256
        {
            break;
        }
        i = i.wrapping_add(1);
    }
    if i == num_algorithms {
        return PTLS_ALERT_HANDSHAKE_FAILURE;
    }
    let mut ctx_0: cf_sha256_context = cf_sha256_context {
        H: [0; 8],
        partial: [0; 64],
        blocks: 0,
        npartial: 0,
    };
    cf_sha256_init(&raw mut ctx_0);
    cf_sha256_update(
        &raw mut ctx_0,
        input.base as *const ::core::ffi::c_void,
        input.len,
    );
    cf_sha256_digest_final(&raw mut ctx_0, &raw mut hash as *mut uint8_t);
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut ctx_0 as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<cf_sha256_context>() as size_t,
    );
    uECC_sign(
        &raw mut (*self_0).key as *mut uint8_t,
        &raw mut hash as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 32]>() as ::core::ffi::c_uint,
        &raw mut sig as *mut uint8_t,
        uECC_secp256r1(),
    );
    let mut c2rust_fresh0: [uint8_t; 1] = [0x30 as ::core::ffi::c_int as uint8_t];
    ret = ptls_buffer__do_pushv(
        outbuf,
        &raw mut c2rust_fresh0 as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 1]>() as size_t,
    );
    if !(ret != 0 as ::core::ffi::c_int) {
        let mut c2rust_fresh1: [uint8_t; 1] = [0xff as ::core::ffi::c_int as uint8_t];
        ret = ptls_buffer__do_pushv(
            outbuf,
            &raw mut c2rust_fresh1 as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 1]>() as size_t,
        );
        if !(ret != 0 as ::core::ffi::c_int) {
            let mut body_start: size_t = (*outbuf).off;
            ret = ptls_buffer_push_asn1_ubigint(
                outbuf,
                &raw mut sig as *mut uint8_t as *const ::core::ffi::c_void,
                32 as size_t,
            );
            if !(ret != 0 as ::core::ffi::c_int) {
                ret = ptls_buffer_push_asn1_ubigint(
                    outbuf,
                    (&raw mut sig as *mut uint8_t).offset(32 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_void,
                    32 as size_t,
                );
                if !(ret != 0 as ::core::ffi::c_int) {
                    let mut body_size: size_t = (*outbuf).off.wrapping_sub(body_start);
                    if body_size < 128 as size_t {
                        *(*outbuf)
                            .base
                            .offset(body_start.wrapping_sub(1 as size_t) as isize) =
                            body_size as uint8_t;
                        c2rust_current_block = 15925075030174552612;
                    } else {
                        ret = ptls_buffer__adjust_asn1_blocksize(outbuf, body_size);
                        if ret != 0 as ::core::ffi::c_int {
                            c2rust_current_block = 830133701824667179;
                        } else {
                            c2rust_current_block = 15925075030174552612;
                        }
                    }
                    match c2rust_current_block {
                        830133701824667179 => {}
                        _ => {
                            *selected_algorithm = PTLS_SIGNATURE_ECDSA_SECP256R1_SHA256 as uint16_t;
                            ret = 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
    }
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut hash as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut sig as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_minicrypto_init_secp256r1sha256_sign_certificate(
    mut self_0: *mut ptls_minicrypto_secp256r1sha256_sign_certificate_t,
    mut key: ptls_iovec_t,
) -> ::core::ffi::c_int {
    if key.len != ::core::mem::size_of::<[uint8_t; 32]>() as usize {
        return PTLS_ERROR_INCOMPATIBLE_KEY;
    }
    (*self_0).super_0.cb = Some(
        secp256r1sha256_sign
            as unsafe extern "C" fn(
                *mut ptls_sign_certificate_t,
                *mut ptls_t,
                *mut *mut ptls_async_job_t,
                *mut uint16_t,
                *mut ptls_buffer_t,
                ptls_iovec_t,
                *const uint16_t,
                size_t,
            ) -> ::core::ffi::c_int,
    )
        as Option<
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
        >;
    memcpy(
        &raw mut (*self_0).key as *mut uint8_t as *mut ::core::ffi::c_void,
        key.base as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub static mut ptls_minicrypto_secp256r1: ptls_key_exchange_algorithm_t = unsafe {
    st_ptls_key_exchange_algorithm_t {
        id: PTLS_GROUP_SECP256R1 as uint16_t,
        create: Some(
            secp256r1_create_key_exchange
                as unsafe extern "C" fn(
                    *const ptls_key_exchange_algorithm_t,
                    *mut *mut ptls_key_exchange_context_t,
                ) -> ::core::ffi::c_int,
        ),
        exchange: Some(
            secp256r1_key_exchange
                as unsafe extern "C" fn(
                    *const ptls_key_exchange_algorithm_t,
                    *mut ptls_iovec_t,
                    *mut ptls_iovec_t,
                    ptls_iovec_t,
                ) -> ::core::ffi::c_int,
        ),
        data: 0,
        name: PTLS_GROUP_NAME_SECP256R1.as_ptr(),
    }
};
#[no_mangle]
pub static mut ptls_minicrypto_key_exchanges: [*const ptls_key_exchange_algorithm_t; 2] = unsafe {
    [
        &raw const ptls_minicrypto_secp256r1,
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
    ]
};
pub const SECP256R1_PRIVATE_KEY: [::core::ffi::c_char; 33] = unsafe {
    ::core::mem::transmute::<
        [u8; 33],
        [::core::ffi::c_char; 33],
    >(
        *b"\xC1t\xB4\xF9^\xFEz\x01\x0E\xBEJ\xE83\xB26\x13\xFCe\xE9e\x91\xA89\x9E\x9A\x80\xFB\xAB\xD1\xFF\xBA:\0",
    )
};
pub const SECP256R1_CERTIFICATE: [::core::ffi::c_char; 613] = unsafe {
    ::core::mem::transmute::<
        [u8; 613],
        [::core::ffi::c_char; 613],
    >(
        *b"0\x82\x02`0\x82\x01H\xA0\x03\x02\x01\x02\x02\x01\x010\r\x06\t*\x86H\x86\xF7\r\x01\x01\x0B\x05\x000\x1A1\x180\x16\x06\x03U\x04\x03\x13\x0Fpicotls test ca0\x1E\x17\r180223053104Z\x17\r280221053104Z0\x1B1\x190\x17\x06\x03U\x04\x03\x13\x10test.example.com0Y0\x13\x06\x07*\x86H\xCE=\x02\x01\x06\x08*\x86H\xCE=\x03\x01\x07\x03B\0\x04\xDA\xC8\xA5@T\xBA3\xDA\x18\xA9A\x7FIS\xDF`\xE6\xA6=\xB6\x8ES:\x9F\xDD\x19\x14^\xAB\x03\xCF\xBC\xFB6\x98\x16$\x8F\x07)m\x15\xD8O0\xE8\td\xFB\x14\xFC\x86|\xD4\x06\xC2\xFD\x9D\xE8\x99?H\x8C+\xA3{0y0\t\x06\x03U\x1D\x13\x04\x020\x000,\x06\t`\x86H\x01\x86\xF8B\x01\r\x04\x1F\x16\x1DOpenSSL Generated Certificate0\x1D\x06\x03U\x1D\x0E\x04\x16\x04\x14\xEE0\x86\x16\xA1\xD2i\xADd\xE4\xD7wk\xB2\xFD\\O\x01\xA2\xB50\x1F\x06\x03U\x1D#\x04\x180\x16\x80\x14\xBFy\xCA\x97\xB2`x \x96\xAAFW\x9C\xDF\xA7\xB2#\xF5%c0\r\x06\t*\x86H\x86\xF7\r\x01\x01\x0B\x05\0\x03\x82\x01\x01\0\x8F\xAC\x9C\x01m\x81\xAA\x8C\xAE]\xB5\x16t\xEA\xE8\xEB&[\xB1f\xD5k\xD4My\rm\x87\xA9\xB6\xBFt-\xC1\xB2.R\xB6K\xCA\r\x01E8X\x1A\xD2jm \x98ZQ\xB0o,?\x0F\x12\x88\xED|\t\xA5t\0!=K\xD2-T\xAAS\x8Bd\xF9\x1E\xEA\xA5\x8A\xE7a^V\x92R6>\xA0hY\x9C}\xB3\xE8\\Kwn\xDE(\xED\x18\x91\xA9\x9C9\xD2\x96\xCC\x98\x05\x8Ct\xDC\x1E\x12[8\xBDV\xCB\xA3\xE8\xE1*Z+\xD22E\xC1\x10\x85 lk4\xEAf\x91\x0E.\xB8d\x87\x9F\x07\xBC#O#\xAD\xBE\x89\xDF\n\x98G\xE9c\x02\xD3A\xF4-\xA4\xCE\xDD\xE3\xD8A\x08\xFE\xDFG\xC0\xE7c\x8E\x1F\xF0K\xC5\xAE\xAB\xC0\xBA8>\xE3\x90\x9C\x08\xBDu\x1C\xB9\xB8TC\x1D\x99B\xE0\xA2\xB7u\xBB\x14\x03y\x9A\xF6\x07\xD8\xA5\xAB+:p\x8Bw\x85p\x8A\x988\x9B5\t\xF6bk)J\xA7\xA7\xF9;\xDE\xD8\xC8\x90W\xF2v*#\x0B\x01h\xC6\x9A\xF2\0",
    )
};
unsafe extern "C" fn test_secp256r1_key_exchange() {
    test_key_exchange(
        &raw const ptls_minicrypto_secp256r1,
        &raw const ptls_minicrypto_secp256r1,
    );
}
unsafe extern "C" fn test_x25519_key_exchange() {
    test_key_exchange(
        &raw const ptls_minicrypto_x25519,
        &raw const ptls_minicrypto_x25519,
    );
}
unsafe extern "C" fn test_secp256r1_sign() {
    let mut msg: *const ::core::ffi::c_char =
        b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef\0".as_ptr() as *const ::core::ffi::c_char;
    let mut signer: ptls_minicrypto_secp256r1sha256_sign_certificate_t =
        st_ptls_minicrypto_secp256r1sha256_sign_certificate_t {
            super_0: st_ptls_sign_certificate_t {
                cb: Some(
                    secp256r1sha256_sign
                        as unsafe extern "C" fn(
                            *mut ptls_sign_certificate_t,
                            *mut ptls_t,
                            *mut *mut ptls_async_job_t,
                            *mut uint16_t,
                            *mut ptls_buffer_t,
                            ptls_iovec_t,
                            *const uint16_t,
                            size_t,
                        ) -> ::core::ffi::c_int,
                ),
            },
            key: [0; 32],
        };
    let mut pub_0: [uint8_t; 65] = [0; 65];
    let mut selected: uint16_t = 0;
    let mut sigbuf: ptls_buffer_t = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0,
        off: 0,
        is_allocated: 0,
        align_bits: 0,
    };
    let mut sigbuf_small: [uint32_t; 128] = [0; 128];
    uECC_make_key(
        &raw mut pub_0 as *mut uint8_t,
        &raw mut signer.key as *mut uint8_t,
        uECC_secp256r1(),
    );
    ptls_buffer_init(
        &raw mut sigbuf,
        &raw mut sigbuf_small as *mut uint32_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint32_t; 128]>() as size_t,
    );
    let mut c2rust_fresh2: [uint16_t; 1] = [0x403 as ::core::ffi::c_int as uint16_t];
    _ok(
        (secp256r1sha256_sign(
            &raw mut signer.super_0,
            ::core::ptr::null_mut::<ptls_t>(),
            ::core::ptr::null_mut::<*mut ptls_async_job_t>(),
            &raw mut selected,
            &raw mut sigbuf,
            ptls_iovec_init(msg as *const ::core::ffi::c_void, 32 as size_t),
            &raw mut c2rust_fresh2 as *mut uint16_t,
            1 as size_t,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        56 as ::core::ffi::c_int,
    );
    _ok(
        (selected as ::core::ffi::c_int == 0x403 as ::core::ffi::c_int)
            as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        57 as ::core::ffi::c_int,
    );
    ptls_buffer_dispose(&raw mut sigbuf);
}
unsafe extern "C" fn test_hrr() {
    let mut client_keyex: [*const ptls_key_exchange_algorithm_t; 3] = [
        &raw const ptls_minicrypto_x25519,
        &raw const ptls_minicrypto_secp256r1,
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
    ];
    let mut client_ctx: ptls_context_t = {
        let mut init = st_ptls_context_t {
            require_dhe_on_psk_use_exporter_send_change_cipher_spec_require_client_authentication_omit_end_of_early_data_use_raw_public_keys_server_cipher_preference_server_cipher_chacha_priority: [0; 1],
            c2rust_padding: [0; 7],
            random_bytes: Some(
                ptls_minicrypto_random_bytes
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> (),
            ),
            get_time: &raw mut ptls_get_time,
            key_exchanges: &raw mut client_keyex
                as *mut *const ptls_key_exchange_algorithm_t,
            cipher_suites: &raw mut ptls_minicrypto_cipher_suites_all
                as *mut *const ptls_cipher_suite_t,
            certificates: C2Rust_Unnamed_11 {
                list: ::core::ptr::null_mut::<ptls_iovec_t>(),
                count: 0,
            },
            pre_shared_key: C2Rust_Unnamed_10 {
                identity: st_ptls_iovec_t {
                    base: ::core::ptr::null_mut::<uint8_t>(),
                    len: 0,
                },
                secret: st_ptls_iovec_t {
                    base: ::core::ptr::null_mut::<uint8_t>(),
                    len: 0,
                },
                hash: ::core::ptr::null::<ptls_hash_algorithm_t>(),
            },
            ech: C2Rust_Unnamed_7 {
                client: C2Rust_Unnamed_9 {
                    ciphers: ::core::ptr::null_mut::<*const ptls_hpke_cipher_suite_t>(),
                    kems: ::core::ptr::null_mut::<*const ptls_hpke_kem_t>(),
                },
                server: C2Rust_Unnamed_8 {
                    create_opener: ::core::ptr::null_mut::<ptls_ech_create_opener_t>(),
                    retry_configs: st_ptls_iovec_t {
                        base: ::core::ptr::null_mut::<uint8_t>(),
                        len: 0,
                    },
                },
            },
            on_client_hello: ::core::ptr::null_mut::<ptls_on_client_hello_t>(),
            emit_certificate: ::core::ptr::null_mut::<ptls_emit_certificate_t>(),
            sign_certificate: ::core::ptr::null_mut::<ptls_sign_certificate_t>(),
            verify_certificate: ::core::ptr::null_mut::<ptls_verify_certificate_t>(),
            ticket_lifetime: 0,
            max_early_data_size: 0,
            max_buffer_size: 0,
            hkdf_label_prefix__obsolete: ::core::ptr::null::<::core::ffi::c_char>(),
            encrypt_ticket: ::core::ptr::null_mut::<ptls_encrypt_ticket_t>(),
            save_ticket: ::core::ptr::null_mut::<ptls_save_ticket_t>(),
            log_event: ::core::ptr::null_mut::<ptls_log_event_t>(),
            update_open_count: ::core::ptr::null_mut::<ptls_update_open_count_t>(),
            update_traffic_key: ::core::ptr::null_mut::<ptls_update_traffic_key_t>(),
            decompress_certificate: ::core::ptr::null_mut::<
                ptls_decompress_certificate_t,
            >(),
            on_extension: ::core::ptr::null_mut::<ptls_on_extension_t>(),
            tls12_cipher_suites: ::core::ptr::null_mut::<*const ptls_cipher_suite_t>(),
            ticket_context: C2Rust_Unnamed_0 {
                bytes: [0; 32],
                is_set: [0; 1],
                c2rust_padding: [0; 3],
            },
            client_ca_names: C2Rust_Unnamed {
                list: ::core::ptr::null::<ptls_iovec_t>(),
                count: 0,
            },
        };
        init.set_require_dhe_on_psk(0);
        init.set_use_exporter(0);
        init.set_send_change_cipher_spec(0);
        init.set_require_client_authentication(0);
        init.set_omit_end_of_early_data(0);
        init.set_use_raw_public_keys(0);
        init.set_server_cipher_preference(0);
        init.set_server_cipher_chacha_priority(0);
        init
    };
    let mut client: *mut ptls_t = ::core::ptr::null_mut::<ptls_t>();
    let mut server: *mut ptls_t = ::core::ptr::null_mut::<ptls_t>();
    let mut cbuf: ptls_buffer_t = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0,
        off: 0,
        is_allocated: 0,
        align_bits: 0,
    };
    let mut sbuf: ptls_buffer_t = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0,
        off: 0,
        is_allocated: 0,
        align_bits: 0,
    };
    let mut decbuf: ptls_buffer_t = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0,
        off: 0,
        is_allocated: 0,
        align_bits: 0,
    };
    let mut cbuf_small: [uint8_t; 16384] = [0; 16384];
    let mut sbuf_small: [uint8_t; 16384] = [0; 16384];
    let mut decbuf_small: [uint8_t; 16384] = [0; 16384];
    let mut consumed: size_t = 0;
    let mut ret: ::core::ffi::c_int = 0;
    client = ptls_new(&raw mut client_ctx, 0 as ::core::ffi::c_int);
    server = ptls_new(ctx_peer, 1 as ::core::ffi::c_int);
    ptls_buffer_init(
        &raw mut cbuf,
        &raw mut cbuf_small as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16384]>() as size_t,
    );
    ptls_buffer_init(
        &raw mut sbuf,
        &raw mut sbuf_small as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16384]>() as size_t,
    );
    ptls_buffer_init(
        &raw mut decbuf,
        &raw mut decbuf_small as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16384]>() as size_t,
    );
    ret = ptls_handshake(
        client,
        &raw mut cbuf,
        ::core::ptr::null::<::core::ffi::c_void>(),
        ::core::ptr::null_mut::<size_t>(),
        ::core::ptr::null_mut::<ptls_handshake_properties_t>(),
    );
    _ok(
        (ret == 0x200 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        84 as ::core::ffi::c_int,
    );
    consumed = cbuf.off;
    ret = ptls_handshake(
        server,
        &raw mut sbuf,
        cbuf.base as *const ::core::ffi::c_void,
        &raw mut consumed,
        ::core::ptr::null_mut::<ptls_handshake_properties_t>(),
    );
    _ok(
        (ret == 0x200 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        88 as ::core::ffi::c_int,
    );
    _ok(
        (consumed == cbuf.off) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        89 as ::core::ffi::c_int,
    );
    cbuf.off = 0 as size_t;
    _ok(
        (sbuf.off > (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as size_t)
            as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        92 as ::core::ffi::c_int,
    );
    _ok(
        (*sbuf.base.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 2 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        93 as ::core::ffi::c_int,
    );
    consumed = sbuf.off;
    ret = ptls_handshake(
        client,
        &raw mut cbuf,
        sbuf.base as *const ::core::ffi::c_void,
        &raw mut consumed,
        ::core::ptr::null_mut::<ptls_handshake_properties_t>(),
    );
    _ok(
        (ret == 0x200 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        97 as ::core::ffi::c_int,
    );
    _ok(
        (consumed == sbuf.off) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        98 as ::core::ffi::c_int,
    );
    sbuf.off = 0 as size_t;
    _ok(
        (cbuf.off >= (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as size_t)
            as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        101 as ::core::ffi::c_int,
    );
    _ok(
        (*cbuf.base.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 1 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        102 as ::core::ffi::c_int,
    );
    consumed = cbuf.off;
    ret = ptls_handshake(
        server,
        &raw mut sbuf,
        cbuf.base as *const ::core::ffi::c_void,
        &raw mut consumed,
        ::core::ptr::null_mut::<ptls_handshake_properties_t>(),
    );
    _ok(
        (ret == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        106 as ::core::ffi::c_int,
    );
    _ok(
        (consumed == cbuf.off) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        107 as ::core::ffi::c_int,
    );
    cbuf.off = 0 as size_t;
    _ok(
        (sbuf.off >= (5 as ::core::ffi::c_int + 4 as ::core::ffi::c_int) as size_t)
            as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        110 as ::core::ffi::c_int,
    );
    _ok(
        (*sbuf.base.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            == 2 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        111 as ::core::ffi::c_int,
    );
    consumed = sbuf.off;
    ret = ptls_handshake(
        client,
        &raw mut cbuf,
        sbuf.base as *const ::core::ffi::c_void,
        &raw mut consumed,
        ::core::ptr::null_mut::<ptls_handshake_properties_t>(),
    );
    _ok(
        (ret == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        115 as ::core::ffi::c_int,
    );
    _ok(
        (consumed == sbuf.off) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        116 as ::core::ffi::c_int,
    );
    sbuf.off = 0 as size_t;
    ret = ptls_send(
        client,
        &raw mut cbuf,
        b"hello world\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        11 as size_t,
    );
    _ok(
        (ret == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        120 as ::core::ffi::c_int,
    );
    consumed = cbuf.off;
    ret = ptls_receive(
        server,
        &raw mut decbuf,
        cbuf.base as *const ::core::ffi::c_void,
        &raw mut consumed,
    );
    _ok(
        (ret == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        124 as ::core::ffi::c_int,
    );
    _ok(
        (consumed == cbuf.off) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        125 as ::core::ffi::c_int,
    );
    cbuf.off = 0 as size_t;
    _ok(
        (decbuf.off == 11 as size_t) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        128 as ::core::ffi::c_int,
    );
    _ok(
        (memcmp(
            decbuf.base as *const ::core::ffi::c_void,
            b"hello world\0".as_ptr() as *const ::core::ffi::c_char
                as *const ::core::ffi::c_void,
            11 as size_t,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/minicrypto.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        129 as ::core::ffi::c_int,
    );
    ptls_buffer_dispose(&raw mut decbuf);
    ptls_buffer_dispose(&raw mut sbuf);
    ptls_buffer_dispose(&raw mut cbuf);
    ptls_free(client);
    ptls_free(server);
}
static mut ptls_ffx_ptls_minicrypto_aes128ctr_b53_r4: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"ptls_minicrypto_aes128ctr-ffx-b53-r4\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: 16 as size_t,
        block_size: ((53 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int)
            as size_t,
        iv_size: 16 as size_t,
        context_size: ::core::mem::size_of::<ptls_ffx_context_t>() as size_t,
        setup_crypto: Some(
            ptls_ffx_ptls_minicrypto_aes128ctr_b53_r4_setup
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
unsafe extern "C" fn ptls_ffx_ptls_minicrypto_aes128ctr_b125_r8_setup(
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return ptls_ffx_setup_crypto(
        ctx_0,
        &raw const ptls_minicrypto_aes128ctr,
        is_enc,
        8 as ::core::ffi::c_int,
        125 as size_t,
        key,
    );
}
static mut ptls_ffx_ptls_minicrypto_aes128ctr_b125_r8: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"ptls_minicrypto_aes128ctr-ffx-b125-r8\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: 16 as size_t,
        block_size: ((125 as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
            / 8 as ::core::ffi::c_int) as size_t,
        iv_size: 16 as size_t,
        context_size: ::core::mem::size_of::<ptls_ffx_context_t>() as size_t,
        setup_crypto: Some(
            ptls_ffx_ptls_minicrypto_aes128ctr_b125_r8_setup
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
unsafe extern "C" fn ptls_ffx_ptls_minicrypto_aes128ctr_b53_r4_setup(
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return ptls_ffx_setup_crypto(
        ctx_0,
        &raw const ptls_minicrypto_aes128ctr,
        is_enc,
        4 as ::core::ffi::c_int,
        53 as size_t,
        key,
    );
}
unsafe extern "C" fn ptls_ffx_ptls_minicrypto_aes128ctr_b31_r6_setup(
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return ptls_ffx_setup_crypto(
        ctx_0,
        &raw const ptls_minicrypto_aes128ctr,
        is_enc,
        6 as ::core::ffi::c_int,
        31 as size_t,
        key,
    );
}
static mut ptls_ffx_ptls_minicrypto_aes128ctr_b31_r6: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"ptls_minicrypto_aes128ctr-ffx-b31-r6\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: 16 as size_t,
        block_size: ((31 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int)
            as size_t,
        iv_size: 16 as size_t,
        context_size: ::core::mem::size_of::<ptls_ffx_context_t>() as size_t,
        setup_crypto: Some(
            ptls_ffx_ptls_minicrypto_aes128ctr_b31_r6_setup
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
unsafe extern "C" fn ptls_ffx_ptls_minicrypto_chacha20_b32_r6_setup(
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return ptls_ffx_setup_crypto(
        ctx_0,
        &raw const ptls_minicrypto_chacha20,
        is_enc,
        6 as ::core::ffi::c_int,
        32 as size_t,
        key,
    );
}
unsafe extern "C" fn ptls_ffx_ptls_minicrypto_chacha20_b57_r4_setup(
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return ptls_ffx_setup_crypto(
        ctx_0,
        &raw const ptls_minicrypto_chacha20,
        is_enc,
        4 as ::core::ffi::c_int,
        57 as size_t,
        key,
    );
}
static mut ptls_ffx_ptls_minicrypto_chacha20_b57_r4: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"ptls_minicrypto_chacha20-ffx-b57-r4\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: 32 as size_t,
        block_size: ((57 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int)
            as size_t,
        iv_size: 16 as size_t,
        context_size: ::core::mem::size_of::<ptls_ffx_context_t>() as size_t,
        setup_crypto: Some(
            ptls_ffx_ptls_minicrypto_chacha20_b57_r4_setup
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
static mut ptls_ffx_ptls_minicrypto_chacha20_b256_r8: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"ptls_minicrypto_chacha20-ffx-b256-r8\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: 32 as size_t,
        block_size: ((256 as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
            / 8 as ::core::ffi::c_int) as size_t,
        iv_size: 16 as size_t,
        context_size: ::core::mem::size_of::<ptls_ffx_context_t>() as size_t,
        setup_crypto: Some(
            ptls_ffx_ptls_minicrypto_chacha20_b256_r8_setup
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
static mut ptls_ffx_ptls_minicrypto_chacha20_b32_r6: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"ptls_minicrypto_chacha20-ffx-b32-r6\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: 32 as size_t,
        block_size: ((32 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int)
            as size_t,
        iv_size: 16 as size_t,
        context_size: ::core::mem::size_of::<ptls_ffx_context_t>() as size_t,
        setup_crypto: Some(
            ptls_ffx_ptls_minicrypto_chacha20_b32_r6_setup
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
unsafe extern "C" fn ptls_ffx_ptls_minicrypto_chacha20_b256_r8_setup(
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return ptls_ffx_setup_crypto(
        ctx_0,
        &raw const ptls_minicrypto_chacha20,
        is_enc,
        8 as ::core::ffi::c_int,
        256 as size_t,
        key,
    );
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut _name: *const ::core::ffi::c_char =
        b"secp256r1\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name);
    test_secp256r1_key_exchange();
    exit_subtest(_name);
    let mut _name_0: *const ::core::ffi::c_char =
        b"x25519\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_0);
    test_x25519_key_exchange();
    exit_subtest(_name_0);
    let mut _name_1: *const ::core::ffi::c_char =
        b"secp256r1-sign\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_1);
    test_secp256r1_sign();
    exit_subtest(_name_1);
    let mut cert: ptls_iovec_t = ptls_iovec_init(
        SECP256R1_CERTIFICATE.as_ptr() as *const ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 613]>() as size_t).wrapping_sub(1 as size_t),
    );
    let mut sign_certificate: ptls_minicrypto_secp256r1sha256_sign_certificate_t =
        st_ptls_minicrypto_secp256r1sha256_sign_certificate_t {
            super_0: st_ptls_sign_certificate_t { cb: None },
            key: [0; 32],
        };
    ptls_minicrypto_init_secp256r1sha256_sign_certificate(
        &raw mut sign_certificate,
        ptls_iovec_init(
            SECP256R1_PRIVATE_KEY.as_ptr() as *const ::core::ffi::c_void,
            SECP256R1_PRIVATE_KEY_SIZE as size_t,
        ),
    );
    let mut ctxbuf: ptls_context_t = {
        let mut init = st_ptls_context_t {
            require_dhe_on_psk_use_exporter_send_change_cipher_spec_require_client_authentication_omit_end_of_early_data_use_raw_public_keys_server_cipher_preference_server_cipher_chacha_priority: [0; 1],
            c2rust_padding: [0; 7],
            random_bytes: Some(
                ptls_minicrypto_random_bytes
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> (),
            ),
            get_time: &raw mut ptls_get_time,
            key_exchanges: &raw mut ptls_minicrypto_key_exchanges
                as *mut *const ptls_key_exchange_algorithm_t,
            cipher_suites: &raw mut ptls_minicrypto_cipher_suites_all
                as *mut *const ptls_cipher_suite_t,
            certificates: C2Rust_Unnamed_11 {
                list: &raw mut cert,
                count: 1 as size_t,
            },
            pre_shared_key: C2Rust_Unnamed_10 {
                identity: st_ptls_iovec_t {
                    base: ::core::ptr::null_mut::<uint8_t>(),
                    len: 0,
                },
                secret: st_ptls_iovec_t {
                    base: ::core::ptr::null_mut::<uint8_t>(),
                    len: 0,
                },
                hash: ::core::ptr::null::<ptls_hash_algorithm_t>(),
            },
            ech: C2Rust_Unnamed_7 {
                client: C2Rust_Unnamed_9 {
                    ciphers: ::core::ptr::null_mut::<*const ptls_hpke_cipher_suite_t>(),
                    kems: ::core::ptr::null_mut::<*const ptls_hpke_kem_t>(),
                },
                server: C2Rust_Unnamed_8 {
                    create_opener: ::core::ptr::null_mut::<ptls_ech_create_opener_t>(),
                    retry_configs: st_ptls_iovec_t {
                        base: ::core::ptr::null_mut::<uint8_t>(),
                        len: 0,
                    },
                },
            },
            on_client_hello: ::core::ptr::null_mut::<ptls_on_client_hello_t>(),
            emit_certificate: ::core::ptr::null_mut::<ptls_emit_certificate_t>(),
            sign_certificate: &raw mut sign_certificate.super_0,
            verify_certificate: ::core::ptr::null_mut::<ptls_verify_certificate_t>(),
            ticket_lifetime: 0,
            max_early_data_size: 0,
            max_buffer_size: 0,
            hkdf_label_prefix__obsolete: ::core::ptr::null::<::core::ffi::c_char>(),
            encrypt_ticket: ::core::ptr::null_mut::<ptls_encrypt_ticket_t>(),
            save_ticket: ::core::ptr::null_mut::<ptls_save_ticket_t>(),
            log_event: ::core::ptr::null_mut::<ptls_log_event_t>(),
            update_open_count: ::core::ptr::null_mut::<ptls_update_open_count_t>(),
            update_traffic_key: ::core::ptr::null_mut::<ptls_update_traffic_key_t>(),
            decompress_certificate: ::core::ptr::null_mut::<
                ptls_decompress_certificate_t,
            >(),
            on_extension: ::core::ptr::null_mut::<ptls_on_extension_t>(),
            tls12_cipher_suites: ::core::ptr::null_mut::<*const ptls_cipher_suite_t>(),
            ticket_context: C2Rust_Unnamed_0 {
                bytes: [0; 32],
                is_set: [0; 1],
                c2rust_padding: [0; 3],
            },
            client_ca_names: C2Rust_Unnamed {
                list: ::core::ptr::null::<ptls_iovec_t>(),
                count: 0,
            },
        };
        init.set_require_dhe_on_psk(0);
        init.set_use_exporter(0);
        init.set_send_change_cipher_spec(0);
        init.set_require_client_authentication(0);
        init.set_omit_end_of_early_data(0);
        init.set_use_raw_public_keys(0);
        init.set_server_cipher_preference(0);
        init.set_server_cipher_chacha_priority(0);
        init
    };
    ctx_peer = &raw mut ctxbuf;
    ctx = ctx_peer;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while !ffx_variants[i as usize].algo.is_null() {
        i = i.wrapping_add(1);
    }
    ffx_variants[i as usize] = st_ptls_ffx_test_variants_t {
        algo: &raw const ptls_ffx_ptls_minicrypto_aes128ctr_b125_r8,
        bit_length: 125 as ::core::ffi::c_int,
    };
    let mut i_0: size_t = 0;
    i_0 = 0 as size_t;
    while !ffx_variants[i_0 as usize].algo.is_null() {
        i_0 = i_0.wrapping_add(1);
    }
    ffx_variants[i_0 as usize] = st_ptls_ffx_test_variants_t {
        algo: &raw const ptls_ffx_ptls_minicrypto_aes128ctr_b31_r6,
        bit_length: 31 as ::core::ffi::c_int,
    };
    let mut i_1: size_t = 0;
    i_1 = 0 as size_t;
    while !ffx_variants[i_1 as usize].algo.is_null() {
        i_1 = i_1.wrapping_add(1);
    }
    ffx_variants[i_1 as usize] = st_ptls_ffx_test_variants_t {
        algo: &raw const ptls_ffx_ptls_minicrypto_aes128ctr_b53_r4,
        bit_length: 53 as ::core::ffi::c_int,
    };
    let mut i_2: size_t = 0;
    i_2 = 0 as size_t;
    while !ffx_variants[i_2 as usize].algo.is_null() {
        i_2 = i_2.wrapping_add(1);
    }
    ffx_variants[i_2 as usize] = st_ptls_ffx_test_variants_t {
        algo: &raw const ptls_ffx_ptls_minicrypto_chacha20_b256_r8,
        bit_length: 256 as ::core::ffi::c_int,
    };
    let mut i_3: size_t = 0;
    i_3 = 0 as size_t;
    while !ffx_variants[i_3 as usize].algo.is_null() {
        i_3 = i_3.wrapping_add(1);
    }
    ffx_variants[i_3 as usize] = st_ptls_ffx_test_variants_t {
        algo: &raw const ptls_ffx_ptls_minicrypto_chacha20_b32_r6,
        bit_length: 32 as ::core::ffi::c_int,
    };
    let mut i_4: size_t = 0;
    i_4 = 0 as size_t;
    while !ffx_variants[i_4 as usize].algo.is_null() {
        i_4 = i_4.wrapping_add(1);
    }
    ffx_variants[i_4 as usize] = st_ptls_ffx_test_variants_t {
        algo: &raw const ptls_ffx_ptls_minicrypto_chacha20_b57_r4,
        bit_length: 57 as ::core::ffi::c_int,
    };
    let mut _name_2: *const ::core::ffi::c_char =
        b"picotls\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_2);
    test_picotls();
    exit_subtest(_name_2);
    let mut _name_3: *const ::core::ffi::c_char = b"hrr\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_3);
    test_hrr();
    exit_subtest(_name_3);
    return done_testing();
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}

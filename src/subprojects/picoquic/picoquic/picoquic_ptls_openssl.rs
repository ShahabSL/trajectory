use ::c2rust_bitfields;
extern "C" {
    pub type st_ptls_t;
    pub type st_ptls_key_schedule_t;
    pub type st_ptls_traffic_protection_t;
    pub type ossl_provider_st;
    pub type bio_st;
    pub type evp_md_st;
    pub type evp_pkey_st;
    pub type x509_st;
    pub type x509_store_st;
    pub type x509_lookup_st;
    pub type x509_lookup_method_st;
    pub type ossl_lib_ctx_st;
    pub type ossl_init_settings_st;
    pub type stack_st_X509;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn picoquic_register_ciphersuite(
        suite: *const ptls_cipher_suite_t,
        is_low_memory: ::core::ffi::c_int,
    );
    fn picoquic_register_key_exchange_algorithm(key_exchange: *const ptls_key_exchange_algorithm_t);
    fn picoquic_register_tls_key_provider_fn(
        set_private_key_from_file_fn: picoquic_set_private_key_from_file_t,
        dispose_sign_certificate_fn: picoquic_dispose_sign_certificate_t,
        get_certs_from_file_fn: picoquic_get_certs_from_file_t,
    );
    fn picoquic_register_verify_certificate_fn(
        certificate_verifier_fn: picoquic_get_certificate_verifier_t,
        dispose_certificate_verifier_fn: picoquic_dispose_certificate_verifier_t,
        set_tls_root_certificates_fn: picoquic_set_tls_root_certificates_t,
    );
    fn picoquic_register_explain_crypto_error_fn(
        explain_crypto_error_fn: picoquic_explain_crypto_error_t,
        clear_crypto_errors_fn: picoquic_clear_crypto_errors_t,
    );
    fn picoquic_register_crypto_random_provider_fn(
        random_provider: picoquic_crypto_random_provider_t,
    );
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn OPENSSL_init_crypto(
        opts: uint64_t,
        settings: *const OPENSSL_INIT_SETTINGS,
    ) -> ::core::ffi::c_int;
    fn BIO_new_file(
        filename: *const ::core::ffi::c_char,
        mode: *const ::core::ffi::c_char,
    ) -> *mut BIO;
    fn BIO_free(a: *mut BIO) -> ::core::ffi::c_int;
    fn EVP_PKEY_free(pkey: *mut EVP_PKEY);
    fn X509_STORE_new() -> *mut X509_STORE;
    fn X509_STORE_free(v: *mut X509_STORE);
    fn X509_STORE_add_lookup(v: *mut X509_STORE, m: *mut X509_LOOKUP_METHOD) -> *mut X509_LOOKUP;
    fn X509_LOOKUP_file() -> *mut X509_LOOKUP_METHOD;
    fn X509_STORE_add_cert(ctx: *mut X509_STORE, x: *mut X509) -> ::core::ffi::c_int;
    fn X509_LOOKUP_ctrl(
        ctx: *mut X509_LOOKUP,
        cmd: ::core::ffi::c_int,
        argc: *const ::core::ffi::c_char,
        argl: ::core::ffi::c_long,
        ret: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn X509_free(a: *mut X509);
    fn d2i_X509(
        a: *mut *mut X509,
        in_0: *mut *const ::core::ffi::c_uchar,
        len: ::core::ffi::c_long,
    ) -> *mut X509;
    fn i2d_X509(a: *const X509, out: *mut *mut ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    static ptls_openssl_secp256r1: ptls_key_exchange_algorithm_t;
    static ptls_openssl_x25519: ptls_key_exchange_algorithm_t;
    static ptls_openssl_aes128gcmsha256: ptls_cipher_suite_t;
    static ptls_openssl_aes256gcmsha384: ptls_cipher_suite_t;
    static ptls_openssl_chacha20poly1305sha256: ptls_cipher_suite_t;
    fn ptls_openssl_random_bytes(buf: *mut ::core::ffi::c_void, len: size_t);
    fn ptls_openssl_init_sign_certificate(
        self_0: *mut ptls_openssl_sign_certificate_t,
        key: *mut EVP_PKEY,
    ) -> ::core::ffi::c_int;
    fn ptls_openssl_dispose_sign_certificate(self_0: *mut ptls_openssl_sign_certificate_t);
    fn ptls_openssl_init_verify_certificate(
        self_0: *mut ptls_openssl_verify_certificate_t,
        store: *mut X509_STORE,
    ) -> ::core::ffi::c_int;
    fn ptls_openssl_dispose_verify_certificate(self_0: *mut ptls_openssl_verify_certificate_t);
    fn PEM_read_bio_X509(
        out: *mut BIO,
        x: *mut *mut X509,
        cb: Option<pem_password_cb>,
        u: *mut ::core::ffi::c_void,
    ) -> *mut X509;
    fn PEM_read_bio_PrivateKey(
        out: *mut BIO,
        x: *mut *mut EVP_PKEY,
        cb: Option<pem_password_cb>,
        u: *mut ::core::ffi::c_void,
    ) -> *mut EVP_PKEY;
    fn ERR_get_error_all(
        file: *mut *const ::core::ffi::c_char,
        line: *mut ::core::ffi::c_int,
        func: *mut *const ::core::ffi::c_char,
        data: *mut *const ::core::ffi::c_char,
        flags: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_ulong;
    fn ERR_clear_error();
    fn OSSL_PROVIDER_load(
        _: *mut OSSL_LIB_CTX,
        name: *const ::core::ffi::c_char,
    ) -> *mut OSSL_PROVIDER;
    fn OSSL_PROVIDER_unload(prov: *mut OSSL_PROVIDER) -> ::core::ffi::c_int;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = isize;
pub type size_t = usize;
pub type ssize_t = isize;
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
pub type picoquic_free_verify_certificate_ctx =
    Option<unsafe extern "C" fn(*mut ptls_verify_certificate_t) -> ()>;
pub type picoquic_set_private_key_from_file_t = Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, *mut ptls_context_t) -> ::core::ffi::c_int,
>;
pub type picoquic_dispose_sign_certificate_t =
    Option<unsafe extern "C" fn(*mut ptls_sign_certificate_t) -> ()>;
pub type picoquic_get_certs_from_file_t =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_char, *mut size_t) -> *mut ptls_iovec_t>;
pub type picoquic_dispose_certificate_verifier_t =
    Option<unsafe extern "C" fn(*mut ptls_verify_certificate_t) -> ()>;
pub type picoquic_get_certificate_verifier_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_char,
        *mut ::core::ffi::c_uint,
        *mut picoquic_dispose_certificate_verifier_t,
    ) -> *mut ptls_verify_certificate_t,
>;
pub type picoquic_set_tls_root_certificates_t = Option<
    unsafe extern "C" fn(*mut ptls_context_t, *mut ptls_iovec_t, size_t) -> ::core::ffi::c_int,
>;
pub type picoquic_explain_crypto_error_t = Option<
    unsafe extern "C" fn(
        *mut *const ::core::ffi::c_char,
        *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int,
>;
pub type picoquic_clear_crypto_errors_t = Option<unsafe extern "C" fn() -> ()>;
pub type picoquic_crypto_random_provider_t =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
pub type OSSL_PROVIDER = ossl_provider_st;
pub type BIO = bio_st;
pub type EVP_MD = evp_md_st;
pub type EVP_PKEY = evp_pkey_st;
pub type X509 = x509_st;
pub type X509_STORE = x509_store_st;
pub type X509_LOOKUP = x509_lookup_st;
pub type X509_LOOKUP_METHOD = x509_lookup_method_st;
pub type OSSL_LIB_CTX = ossl_lib_ctx_st;
pub type OPENSSL_INIT_SETTINGS = ossl_init_settings_st;
pub type pem_password_cb = unsafe extern "C" fn(
    *mut ::core::ffi::c_char,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_openssl_signature_scheme_t {
    pub scheme_id: uint16_t,
    pub scheme_md: Option<unsafe extern "C" fn() -> *const EVP_MD>,
}
pub type ptls_openssl_signature_scheme_t = st_ptls_openssl_signature_scheme_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_ptls_openssl_sign_certificate_t {
    pub super_0: ptls_sign_certificate_t,
    pub key: *mut EVP_PKEY,
    pub schemes: *const ptls_openssl_signature_scheme_t,
    #[bitfield(name = "async_0", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub async_0: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type ptls_openssl_sign_certificate_t = st_ptls_openssl_sign_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_openssl_override_verify_certificate_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_openssl_override_verify_certificate_t,
            *mut ptls_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut X509,
            *mut stack_st_X509,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_openssl_override_verify_certificate_t = st_ptls_openssl_override_verify_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_openssl_verify_certificate_t {
    pub super_0: ptls_verify_certificate_t,
    pub cert_store: *mut X509_STORE,
    pub override_callback: *mut ptls_openssl_override_verify_certificate_t,
}
pub type ptls_openssl_verify_certificate_t = st_ptls_openssl_verify_certificate_t;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
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
pub const OPENSSL_INIT_ADD_ALL_CIPHERS: ::core::ffi::c_long = 0x4 as ::core::ffi::c_long;
pub const OPENSSL_INIT_ADD_ALL_DIGESTS: ::core::ffi::c_long = 0x8 as ::core::ffi::c_long;
pub const X509_L_FILE_LOAD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
static mut openssl_is_init: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
static mut openssl_default_provider: *mut OSSL_PROVIDER =
    ::core::ptr::null::<OSSL_PROVIDER>() as *mut OSSL_PROVIDER;
unsafe extern "C" fn picoquic_init_openssl() {
    if openssl_is_init == 0 as ::core::ffi::c_int {
        openssl_is_init = 1 as ::core::ffi::c_int;
        OPENSSL_init_crypto(
            (OPENSSL_INIT_ADD_ALL_CIPHERS | OPENSSL_INIT_ADD_ALL_DIGESTS) as uint64_t,
            ::core::ptr::null::<OPENSSL_INIT_SETTINGS>(),
        );
        openssl_default_provider = OSSL_PROVIDER_load(
            ::core::ptr::null_mut::<OSSL_LIB_CTX>(),
            b"default\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
}
unsafe extern "C" fn picoquic_clear_openssl() {
    if openssl_is_init != 0 {
        if !openssl_default_provider.is_null() {
            OSSL_PROVIDER_unload(openssl_default_provider);
            openssl_default_provider = ::core::ptr::null_mut::<OSSL_PROVIDER>();
        }
        openssl_is_init = 0 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn set_openssl_sign_certificate_from_key(
    mut pkey: *mut EVP_PKEY,
    mut ctx: *mut ptls_context_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut signer: *mut ptls_openssl_sign_certificate_t =
        ::core::ptr::null_mut::<ptls_openssl_sign_certificate_t>();
    signer = malloc(::core::mem::size_of::<ptls_openssl_sign_certificate_t>() as size_t)
        as *mut ptls_openssl_sign_certificate_t;
    if signer.is_null() || pkey.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        ret = ptls_openssl_init_sign_certificate(signer, pkey);
        (*ctx).sign_certificate = &raw mut (*signer).super_0;
    }
    if !pkey.is_null() {
        EVP_PKEY_free(pkey);
    }
    if ret != 0 as ::core::ffi::c_int && !signer.is_null() {
        free(signer as *mut ::core::ffi::c_void);
    }
    return ret;
}
unsafe extern "C" fn set_openssl_private_key_from_key_file(
    mut keypem: *const ::core::ffi::c_char,
    mut ctx: *mut ptls_context_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut bio: *mut BIO = BIO_new_file(keypem, b"rb\0".as_ptr() as *const ::core::ffi::c_char);
    if bio.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        let mut pkey: *mut EVP_PKEY =
            PEM_read_bio_PrivateKey(bio, ::core::ptr::null_mut::<*mut EVP_PKEY>(), None, NULL);
        if pkey.is_null() {
            ret = -(1 as ::core::ffi::c_int);
        } else {
            ret = set_openssl_sign_certificate_from_key(pkey, ctx);
        }
        BIO_free(bio);
    }
    return ret;
}
unsafe extern "C" fn picoquic_openssl_dispose_sign_certificate(
    mut cert: *mut ptls_sign_certificate_t,
) {
    ptls_openssl_dispose_sign_certificate(cert as *mut ptls_openssl_sign_certificate_t);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_openssl_get_certs_from_file(
    mut file_name: *const ::core::ffi::c_char,
    mut count: *mut size_t,
) -> *mut ptls_iovec_t {
    let mut bio_key: *mut BIO =
        BIO_new_file(file_name, b"rb\0".as_ptr() as *const ::core::ffi::c_char);
    let max_count: size_t = 16 as size_t;
    let mut chain: *mut ptls_iovec_t =
        malloc((::core::mem::size_of::<ptls_iovec_t>() as size_t).wrapping_mul(max_count))
            as *mut ptls_iovec_t;
    *count = 0 as size_t;
    if !chain.is_null() {
        let mut cert: *mut X509 = ::core::ptr::null_mut::<X509>();
        memset(
            chain as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<ptls_iovec_t>() as size_t).wrapping_mul(max_count),
        );
        while *count < max_count && {
            cert = PEM_read_bio_X509(bio_key, ::core::ptr::null_mut::<*mut X509>(), None, NULL);
            !cert.is_null()
        } {
            let mut length: ::core::ffi::c_int =
                i2d_X509(cert, ::core::ptr::null_mut::<*mut ::core::ffi::c_uchar>());
            let mut cert_der: *mut ::core::ffi::c_uchar =
                malloc(length as size_t) as *mut ::core::ffi::c_uchar;
            let mut tmp: *mut ::core::ffi::c_uchar = cert_der;
            i2d_X509(cert, &raw mut tmp);
            X509_free(cert);
            *chain.offset(*count as isize) =
                ptls_iovec_init(cert_der as *const ::core::ffi::c_void, length as size_t)
                    as ptls_iovec_t;
            *count = (*count).wrapping_add(1 as size_t);
        }
    }
    BIO_free(bio_key);
    return chain;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_openssl_get_openssl_certificate_verifier(
    mut cert_root_file_name: *const ::core::ffi::c_char,
    mut is_cert_store_not_empty: *mut ::core::ffi::c_uint,
) -> *mut ptls_openssl_verify_certificate_t {
    let mut verifier: *mut ptls_openssl_verify_certificate_t =
        malloc(::core::mem::size_of::<ptls_openssl_verify_certificate_t>() as size_t)
            as *mut ptls_openssl_verify_certificate_t;
    if !verifier.is_null() {
        let mut store: *mut X509_STORE = X509_STORE_new();
        if !cert_root_file_name.is_null() && !store.is_null() {
            let mut file_ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut lookup: *mut X509_LOOKUP = X509_STORE_add_lookup(store, X509_LOOKUP_file());
            file_ret = X509_LOOKUP_ctrl(
                lookup,
                X509_L_FILE_LOAD,
                cert_root_file_name,
                1 as ::core::ffi::c_int as ::core::ffi::c_long,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            if file_ret == 1 as ::core::ffi::c_int {
                *is_cert_store_not_empty = 1 as ::core::ffi::c_uint;
            }
        }
        ptls_openssl_init_verify_certificate(verifier, store);
        if !store.is_null() {
            X509_STORE_free(store);
        }
    }
    return verifier;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_openssl_dispose_certificate_verifier(
    mut verifier: *mut ptls_verify_certificate_t,
) {
    ptls_openssl_dispose_verify_certificate(verifier as *mut ptls_openssl_verify_certificate_t);
    free(verifier as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_openssl_get_certificate_verifier(
    mut cert_root_file_name: *const ::core::ffi::c_char,
    mut is_cert_store_not_empty: *mut ::core::ffi::c_uint,
    mut free_certificate_verifier_fn: *mut picoquic_free_verify_certificate_ctx,
) -> *mut ptls_verify_certificate_t {
    let mut verify_cert: *mut ptls_verify_certificate_t =
        ::core::ptr::null_mut::<ptls_verify_certificate_t>();
    let mut verifier: *mut ptls_openssl_verify_certificate_t =
        picoquic_openssl_get_openssl_certificate_verifier(
            cert_root_file_name,
            is_cert_store_not_empty,
        );
    if verifier.is_null() {
        free_certificate_verifier_fn =
            ::core::ptr::null_mut::<picoquic_free_verify_certificate_ctx>();
    } else {
        verify_cert = &raw mut (*verifier).super_0;
        *free_certificate_verifier_fn = Some(
            picoquic_openssl_dispose_certificate_verifier
                as unsafe extern "C" fn(*mut ptls_verify_certificate_t) -> (),
        ) as picoquic_free_verify_certificate_ctx;
    }
    return verify_cert;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_openssl_set_tls_root_certificates(
    mut ctx: *mut ptls_context_t,
    mut certs: *mut ptls_iovec_t,
    mut count: size_t,
) -> ::core::ffi::c_int {
    let mut verify_ctx: *mut ptls_openssl_verify_certificate_t =
        (*ctx).verify_certificate as *mut ptls_openssl_verify_certificate_t;
    let mut i: size_t = 0 as size_t;
    while i < count {
        let mut cert_i_base: *mut uint8_t = (*certs.offset(i as isize)).base;
        let mut cert: *mut X509 = d2i_X509(
            ::core::ptr::null_mut::<*mut X509>(),
            &raw mut cert_i_base as *mut *const ::core::ffi::c_uchar,
            (*certs.offset(i as isize)).len as ::core::ffi::c_long,
        );
        if cert.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
        if X509_STORE_add_cert((*verify_ctx).cert_store, cert) == 0 as ::core::ffi::c_int {
            X509_free(cert);
            return -(2 as ::core::ffi::c_int);
        }
        X509_free(cert);
        i = i.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_open_ssl_explain_crypto_error(
    mut err_file: *mut *const ::core::ffi::c_char,
    mut err_line: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut func: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut data: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut flags: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    return ERR_get_error_all(
        err_file,
        err_line,
        &raw mut func,
        &raw mut data,
        &raw mut flags,
    ) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_openssl_clear_crypto_errors() {
    ERR_clear_error();
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_ptls_openssl_load(mut unload: ::core::ffi::c_int) {
    if unload != 0 {
        if unload == 1 as ::core::ffi::c_int {
            picoquic_clear_openssl();
        }
    } else {
        picoquic_init_openssl();
        picoquic_register_ciphersuite(
            &raw const ptls_openssl_aes128gcmsha256,
            1 as ::core::ffi::c_int,
        );
        picoquic_register_ciphersuite(
            &raw const ptls_openssl_aes256gcmsha384,
            1 as ::core::ffi::c_int,
        );
        picoquic_register_key_exchange_algorithm(&raw const ptls_openssl_secp256r1);
        picoquic_register_ciphersuite(
            &raw const ptls_openssl_chacha20poly1305sha256,
            1 as ::core::ffi::c_int,
        );
        picoquic_register_key_exchange_algorithm(&raw const ptls_openssl_x25519);
        picoquic_register_tls_key_provider_fn(
            Some(
                set_openssl_private_key_from_key_file
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut ptls_context_t,
                    ) -> ::core::ffi::c_int,
            ),
            Some(
                picoquic_openssl_dispose_sign_certificate
                    as unsafe extern "C" fn(*mut ptls_sign_certificate_t) -> (),
            ),
            Some(
                picoquic_openssl_get_certs_from_file
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut size_t,
                    ) -> *mut ptls_iovec_t,
            ),
        );
        picoquic_register_verify_certificate_fn(
            Some(
                picoquic_openssl_get_certificate_verifier
                    as unsafe extern "C" fn(
                        *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_uint,
                        *mut picoquic_free_verify_certificate_ctx,
                    ) -> *mut ptls_verify_certificate_t,
            ),
            Some(
                picoquic_openssl_dispose_certificate_verifier
                    as unsafe extern "C" fn(*mut ptls_verify_certificate_t) -> (),
            ),
            Some(
                picoquic_openssl_set_tls_root_certificates
                    as unsafe extern "C" fn(
                        *mut ptls_context_t,
                        *mut ptls_iovec_t,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
        );
        picoquic_register_explain_crypto_error_fn(
            Some(
                picoquic_open_ssl_explain_crypto_error
                    as unsafe extern "C" fn(
                        *mut *const ::core::ffi::c_char,
                        *mut ::core::ffi::c_int,
                    ) -> ::core::ffi::c_int,
            ),
            Some(picoquic_openssl_clear_crypto_errors),
        );
        picoquic_register_crypto_random_provider_fn(Some(
            ptls_openssl_random_bytes
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> (),
        ));
    };
}

use ::c2rust_bitfields;
extern "C" {
    pub type st_ptls_t;
    pub type st_ptls_key_schedule_t;
    pub type st_ptls_traffic_protection_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
    static mut ptls_clear_memory:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
    fn ptls_minicrypto_init_secp256r1sha256_sign_certificate(
        self_0: *mut ptls_minicrypto_secp256r1sha256_sign_certificate_t,
        key: ptls_iovec_t,
    ) -> ::core::ffi::c_int;
    fn ptls_asn1_error_message(
        error_label: *const ::core::ffi::c_char,
        bytes_max: size_t,
        byte_index: size_t,
        level: ::core::ffi::c_int,
        log_ctx: *mut ptls_minicrypto_log_ctx_t,
    ) -> size_t;
    fn ptls_asn1_dump_content(
        bytes: *const uint8_t,
        bytes_max: size_t,
        byte_index: size_t,
        log_ctx: *mut ptls_minicrypto_log_ctx_t,
    );
    fn ptls_asn1_get_expected_type_and_length(
        bytes: *const uint8_t,
        bytes_max: size_t,
        byte_index: size_t,
        expected_type: uint8_t,
        length: *mut uint32_t,
        indefinite_length: *mut ::core::ffi::c_int,
        last_byte: *mut size_t,
        decode_error: *mut ::core::ffi::c_int,
        log_ctx: *mut ptls_minicrypto_log_ctx_t,
    ) -> size_t;
    fn ptls_asn1_validation_recursive(
        bytes: *const uint8_t,
        bytes_max: size_t,
        decode_error: *mut ::core::ffi::c_int,
        level: ::core::ffi::c_int,
        log_ctx: *mut ptls_minicrypto_log_ctx_t,
    ) -> size_t;
    fn ptls_load_pem_objects(
        pem_fname: *const ::core::ffi::c_char,
        label: *const ::core::ffi::c_char,
        list: *mut ptls_iovec_t,
        list_max: size_t,
        nb_objects: *mut size_t,
    ) -> ::core::ffi::c_int;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __ssize_t = ::core::ffi::c_long;
pub type __intptr_t = ::core::ffi::c_long;
pub type size_t = usize;
pub type ssize_t = __ssize_t;
pub type intptr_t = __intptr_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
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
pub struct st_ptls_minicrypto_secp256r1sha256_sign_certificate_t {
    pub super_0: ptls_sign_certificate_t,
    pub key: [uint8_t; 32],
}
pub type ptls_minicrypto_secp256r1sha256_sign_certificate_t =
    st_ptls_minicrypto_secp256r1sha256_sign_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_asn1_pkcs8_private_key_t {
    pub vec: ptls_iovec_t,
    pub algorithm_index: size_t,
    pub algorithm_length: uint32_t,
    pub parameters_index: size_t,
    pub parameters_length: uint32_t,
    pub key_data_index: size_t,
    pub key_data_length: uint32_t,
}
pub type ptls_asn1_pkcs8_private_key_t = st_ptls_asn1_pkcs8_private_key_t;
pub type ptls_minicrypto_log_ctx_t = st_ptls_minicrypto_log_ctx_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_minicrypto_log_ctx_t {
    pub ctx: *mut ::core::ffi::c_void,
    pub fn_0: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> (),
    >,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const PTLS_ERROR_CLASS_INTERNAL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const PTLS_ERROR_NO_MEMORY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 1 as ::core::ffi::c_int;
pub const PTLS_ERROR_PEM_LABEL_NOT_FOUND: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 51 as ::core::ffi::c_int;
pub const PTLS_ERROR_BER_EXCESSIVE_LENGTH: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 55 as ::core::ffi::c_int;
pub const PTLS_ERROR_BER_ELEMENT_TOO_SHORT: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 56 as ::core::ffi::c_int;
pub const PTLS_ERROR_INCORRECT_PEM_KEY_VERSION: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 60 as ::core::ffi::c_int;
pub const PTLS_ERROR_INCORRECT_PEM_ECDSA_KEY_VERSION: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 61 as ::core::ffi::c_int;
pub const PTLS_ERROR_INCORRECT_PEM_ECDSA_CURVE: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 62 as ::core::ffi::c_int;
pub const PTLS_ERROR_INCORRECT_PEM_ECDSA_KEYSIZE: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 63 as ::core::ffi::c_int;
pub const PTLS_ERROR_INCORRECT_ASN1_ECDSA_KEY_SYNTAX: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 64 as ::core::ffi::c_int;
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
pub const SECP256R1_PRIVATE_KEY_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ptls_minicrypto_asn1_decode_private_key(
    mut pkey: *mut ptls_asn1_pkcs8_private_key_t,
    mut decode_error: *mut ::core::ffi::c_int,
    mut log_ctx: *mut ptls_minicrypto_log_ctx_t,
) -> size_t {
    let mut bytes: *mut uint8_t = (*pkey).vec.base;
    let mut bytes_max: size_t = (*pkey).vec.len;
    let mut byte_index: size_t = 0 as size_t;
    let mut seq0_length: uint32_t = 0 as uint32_t;
    let mut last_byte0: size_t = 0;
    let mut seq1_length: uint32_t = 0 as uint32_t;
    let mut last_byte1: size_t = 0 as size_t;
    let mut oid_length: uint32_t = 0;
    let mut last_oid_byte: size_t = 0;
    let mut key_data_length: uint32_t = 0;
    let mut key_data_last: size_t = 0;
    byte_index = ptls_asn1_get_expected_type_and_length(
        bytes,
        bytes_max,
        byte_index,
        0x30 as uint8_t,
        &raw mut seq0_length,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut last_byte0,
        decode_error,
        log_ctx,
    );
    if *decode_error == 0 as ::core::ffi::c_int && bytes_max != last_byte0 {
        byte_index = ptls_asn1_error_message(
            b"Length larger than message\0".as_ptr() as *const ::core::ffi::c_char,
            bytes_max,
            byte_index,
            0 as ::core::ffi::c_int,
            log_ctx,
        );
        *decode_error = PTLS_ERROR_BER_EXCESSIVE_LENGTH;
    }
    if *decode_error == 0 as ::core::ffi::c_int {
        if byte_index.wrapping_add(3 as size_t) > bytes_max {
            byte_index = ptls_asn1_error_message(
                b"Cannot find key version\0".as_ptr() as *const ::core::ffi::c_char,
                bytes_max,
                byte_index,
                0 as ::core::ffi::c_int,
                log_ctx,
            );
            *decode_error = PTLS_ERROR_INCORRECT_PEM_KEY_VERSION;
        } else if *bytes.offset(byte_index as isize) as ::core::ffi::c_int
            != 0x2 as ::core::ffi::c_int
            || *bytes.offset(byte_index.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                != 0x1 as ::core::ffi::c_int
            || *bytes.offset(byte_index.wrapping_add(2 as size_t) as isize) as ::core::ffi::c_int
                != 0 as ::core::ffi::c_int
        {
            *decode_error = PTLS_ERROR_INCORRECT_PEM_KEY_VERSION;
            byte_index = ptls_asn1_error_message(
                b"Incorrect PEM Version\0".as_ptr() as *const ::core::ffi::c_char,
                bytes_max,
                byte_index,
                0 as ::core::ffi::c_int,
                log_ctx,
            );
        } else {
            byte_index = byte_index.wrapping_add(3 as size_t);
            if !log_ctx.is_null() {
                (*log_ctx).fn_0.expect("non-null function pointer")(
                    (*log_ctx).ctx,
                    b"   Version = 1,\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        }
    }
    if *decode_error == 0 as ::core::ffi::c_int {
        byte_index = ptls_asn1_get_expected_type_and_length(
            bytes,
            bytes_max,
            byte_index,
            0x30 as uint8_t,
            &raw mut seq1_length,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            &raw mut last_byte1,
            decode_error,
            log_ctx,
        );
    }
    if *decode_error == 0 as ::core::ffi::c_int {
        if !log_ctx.is_null() {
            (*log_ctx).fn_0.expect("non-null function pointer")(
                (*log_ctx).ctx,
                b"   Algorithm Identifier:\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        byte_index = ptls_asn1_get_expected_type_and_length(
            bytes,
            last_byte1,
            byte_index,
            0x6 as uint8_t,
            &raw mut oid_length,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            &raw mut last_oid_byte,
            decode_error,
            log_ctx,
        );
        if *decode_error == 0 as ::core::ffi::c_int {
            if !log_ctx.is_null() {
                (*log_ctx).fn_0.expect("non-null function pointer")(
                    (*log_ctx).ctx,
                    b"      Algorithm:\0".as_ptr() as *const ::core::ffi::c_char,
                );
                ptls_asn1_dump_content(
                    bytes.offset(byte_index as isize),
                    oid_length as size_t,
                    0 as size_t,
                    log_ctx,
                );
                (*log_ctx).fn_0.expect("non-null function pointer")(
                    (*log_ctx).ctx,
                    b",\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            (*pkey).algorithm_index = byte_index;
            (*pkey).algorithm_length = oid_length;
            byte_index = byte_index.wrapping_add(oid_length as size_t);
        }
    }
    if *decode_error == 0 as ::core::ffi::c_int {
        if !log_ctx.is_null() {
            (*log_ctx).fn_0.expect("non-null function pointer")(
                (*log_ctx).ctx,
                b"      Parameters:\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if last_byte1 <= byte_index {
            (*pkey).parameters_index = 0 as size_t;
            (*pkey).parameters_length = 0 as uint32_t;
        } else {
            (*pkey).parameters_index = byte_index;
            (*pkey).parameters_length = ptls_asn1_validation_recursive(
                bytes.offset(byte_index as isize),
                last_byte1.wrapping_sub(byte_index),
                decode_error,
                2 as ::core::ffi::c_int,
                log_ctx,
            ) as uint32_t;
            if *decode_error == 0 as ::core::ffi::c_int {
                byte_index = byte_index.wrapping_add((*pkey).parameters_length as size_t);
            }
        }
        if !log_ctx.is_null() {
            (*log_ctx).fn_0.expect("non-null function pointer")(
                (*log_ctx).ctx,
                b"\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
        if *decode_error == 0 as ::core::ffi::c_int && byte_index != last_byte1 {
            byte_index = ptls_asn1_error_message(
                b"Length larger than element\0".as_ptr() as *const ::core::ffi::c_char,
                bytes_max,
                byte_index,
                2 as ::core::ffi::c_int,
                log_ctx,
            );
            *decode_error = PTLS_ERROR_BER_ELEMENT_TOO_SHORT;
        }
    }
    if *decode_error == 0 as ::core::ffi::c_int {
        byte_index = ptls_asn1_get_expected_type_and_length(
            bytes,
            last_byte0,
            byte_index,
            0x4 as uint8_t,
            &raw mut key_data_length,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            &raw mut key_data_last,
            decode_error,
            log_ctx,
        );
        if *decode_error == 0 as ::core::ffi::c_int {
            (*pkey).key_data_index = byte_index;
            (*pkey).key_data_length = key_data_length;
            byte_index = byte_index.wrapping_add(key_data_length as size_t);
            if !log_ctx.is_null() {
                (*log_ctx).fn_0.expect("non-null function pointer")(
                    (*log_ctx).ctx,
                    b"   Key data (%d bytes):\n\0".as_ptr() as *const ::core::ffi::c_char,
                    key_data_length,
                );
                ptls_asn1_validation_recursive(
                    bytes.offset((*pkey).key_data_index as isize),
                    key_data_length as size_t,
                    decode_error,
                    1 as ::core::ffi::c_int,
                    log_ctx,
                );
                (*log_ctx).fn_0.expect("non-null function pointer")(
                    (*log_ctx).ctx,
                    b"\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        }
    }
    if *decode_error == 0 as ::core::ffi::c_int && byte_index != last_byte0 {
        byte_index = ptls_asn1_error_message(
            b"Length larger than element\0".as_ptr() as *const ::core::ffi::c_char,
            bytes_max,
            byte_index,
            0 as ::core::ffi::c_int,
            log_ctx,
        );
        *decode_error = PTLS_ERROR_BER_ELEMENT_TOO_SHORT;
    }
    if !log_ctx.is_null() {
        (*log_ctx).fn_0.expect("non-null function pointer")(
            (*log_ctx).ctx,
            b"\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
    }
    return byte_index;
}
unsafe extern "C" fn ptls_pem_parse_private_key(
    mut pem_fname: *const ::core::ffi::c_char,
    mut pkey: *mut ptls_asn1_pkcs8_private_key_t,
    mut log_ctx: *mut ptls_minicrypto_log_ctx_t,
) -> ::core::ffi::c_int {
    let mut nb_keys: size_t = 0 as size_t;
    let mut ret: ::core::ffi::c_int = ptls_load_pem_objects(
        pem_fname,
        b"PRIVATE KEY\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut (*pkey).vec,
        1 as size_t,
        &raw mut nb_keys,
    );
    if ret == 0 as ::core::ffi::c_int {
        if nb_keys != 1 as size_t {
            ret = PTLS_ERROR_PEM_LABEL_NOT_FOUND;
        }
    }
    if ret == 0 as ::core::ffi::c_int && nb_keys == 1 as size_t {
        let mut decode_error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        if !log_ctx.is_null() {
            (*log_ctx).fn_0.expect("non-null function pointer")(
                (*log_ctx).ctx,
                b"\nFound PRIVATE KEY, length = %d bytes\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                (*pkey).vec.len as ::core::ffi::c_int,
            );
        }
        ptls_minicrypto_asn1_decode_private_key(pkey, &raw mut decode_error, log_ctx);
        if decode_error != 0 as ::core::ffi::c_int {
            ret = decode_error;
        }
    }
    return ret;
}
static mut ptls_asn1_algorithm_ecdsa: [uint8_t; 7] = [
    0x2a as ::core::ffi::c_int as uint8_t,
    0x86 as ::core::ffi::c_int as uint8_t,
    0x48 as ::core::ffi::c_int as uint8_t,
    0xce as ::core::ffi::c_int as uint8_t,
    0x3d as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x1 as ::core::ffi::c_int as uint8_t,
];
static mut ptls_asn1_curve_secp256r1: [uint8_t; 8] = [
    0x2a as ::core::ffi::c_int as uint8_t,
    0x86 as ::core::ffi::c_int as uint8_t,
    0x48 as ::core::ffi::c_int as uint8_t,
    0xce as ::core::ffi::c_int as uint8_t,
    0x3d as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x1 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
];
unsafe extern "C" fn ptls_set_ecdsa_private_key(
    mut ctx: *mut ptls_context_t,
    mut pkey: *mut ptls_asn1_pkcs8_private_key_t,
    mut log_ctx: *mut ptls_minicrypto_log_ctx_t,
) -> ::core::ffi::c_int {
    let mut bytes: *mut uint8_t = (*pkey).vec.base.offset((*pkey).parameters_index as isize);
    let mut bytes_max: size_t = (*pkey).parameters_length as size_t;
    let mut byte_index: size_t = 0 as size_t;
    let mut curve_id: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut curve_id_length: uint32_t = 0 as uint32_t;
    let mut decode_error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut seq_length: uint32_t = 0;
    let mut last_byte: size_t = 0 as size_t;
    let mut ecdsa_key_data: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut ecdsa_key_data_length: uint32_t = 0 as uint32_t;
    let mut ecdsa_key_data_last: size_t = 0 as size_t;
    byte_index = ptls_asn1_get_expected_type_and_length(
        bytes,
        bytes_max,
        byte_index,
        0x6 as uint8_t,
        &raw mut curve_id_length,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut last_byte,
        &raw mut decode_error,
        log_ctx,
    );
    if decode_error == 0 as ::core::ffi::c_int && bytes_max != last_byte {
        byte_index = ptls_asn1_error_message(
            b"Length larger than parameters\0".as_ptr() as *const ::core::ffi::c_char,
            bytes_max,
            byte_index,
            0 as ::core::ffi::c_int,
            log_ctx,
        );
        decode_error = PTLS_ERROR_BER_EXCESSIVE_LENGTH;
    }
    if decode_error == 0 as ::core::ffi::c_int {
        curve_id = bytes.offset(byte_index as isize);
        if !log_ctx.is_null() {
            (*log_ctx).fn_0.expect("non-null function pointer")(
                (*log_ctx).ctx,
                b"Curve: \0".as_ptr() as *const ::core::ffi::c_char,
            );
            ptls_asn1_dump_content(curve_id, curve_id_length as size_t, 0 as size_t, log_ctx);
            (*log_ctx).fn_0.expect("non-null function pointer")(
                (*log_ctx).ctx,
                b"\n\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
    bytes = (*pkey).vec.base.offset((*pkey).key_data_index as isize);
    bytes_max = (*pkey).key_data_length as size_t;
    byte_index = 0 as size_t;
    if decode_error == 0 as ::core::ffi::c_int {
        byte_index = ptls_asn1_get_expected_type_and_length(
            bytes,
            bytes_max,
            byte_index,
            0x30 as uint8_t,
            &raw mut seq_length,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            &raw mut last_byte,
            &raw mut decode_error,
            log_ctx,
        );
    }
    if decode_error == 0 as ::core::ffi::c_int && bytes_max != last_byte {
        byte_index = ptls_asn1_error_message(
            b"Length larger than key data\0".as_ptr() as *const ::core::ffi::c_char,
            bytes_max,
            byte_index,
            0 as ::core::ffi::c_int,
            log_ctx,
        );
        decode_error = PTLS_ERROR_BER_ELEMENT_TOO_SHORT;
    }
    if decode_error == 0 as ::core::ffi::c_int {
        if byte_index.wrapping_add(3 as size_t) > bytes_max {
            byte_index = ptls_asn1_error_message(
                b"Cannot find ECDSA Key Data Version\0".as_ptr() as *const ::core::ffi::c_char,
                bytes_max,
                byte_index,
                0 as ::core::ffi::c_int,
                log_ctx,
            );
            decode_error = PTLS_ERROR_INCORRECT_ASN1_ECDSA_KEY_SYNTAX;
        } else if *bytes.offset(byte_index as isize) as ::core::ffi::c_int
            != 0x2 as ::core::ffi::c_int
            || *bytes.offset(byte_index.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                != 0x1 as ::core::ffi::c_int
            || *bytes.offset(byte_index.wrapping_add(2 as size_t) as isize) as ::core::ffi::c_int
                != 0x1 as ::core::ffi::c_int
        {
            decode_error = PTLS_ERROR_INCORRECT_PEM_ECDSA_KEY_VERSION;
            byte_index = ptls_asn1_error_message(
                b"Incorrect ECDSA Key Data Version\0".as_ptr() as *const ::core::ffi::c_char,
                bytes_max,
                byte_index,
                0 as ::core::ffi::c_int,
                log_ctx,
            );
        } else {
            byte_index = byte_index.wrapping_add(3 as size_t);
            if !log_ctx.is_null() {
                (*log_ctx).fn_0.expect("non-null function pointer")(
                    (*log_ctx).ctx,
                    b"ECDSA Version = 1,\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        }
    }
    if decode_error == 0 as ::core::ffi::c_int {
        byte_index = ptls_asn1_get_expected_type_and_length(
            bytes,
            last_byte,
            byte_index,
            0x4 as uint8_t,
            &raw mut ecdsa_key_data_length,
            ::core::ptr::null_mut::<::core::ffi::c_int>(),
            &raw mut ecdsa_key_data_last,
            &raw mut decode_error,
            log_ctx,
        );
        if decode_error == 0 as ::core::ffi::c_int {
            ecdsa_key_data = bytes.offset(byte_index as isize);
        }
    }
    if curve_id_length as usize == ::core::mem::size_of::<[uint8_t; 8]>() as usize
        && !curve_id.is_null()
        && memcmp(
            curve_id as *const ::core::ffi::c_void,
            &raw const ptls_asn1_curve_secp256r1 as *const uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 8]>() as size_t,
        ) == 0 as ::core::ffi::c_int
    {
        if SECP256R1_PRIVATE_KEY_SIZE as uint32_t != ecdsa_key_data_length {
            decode_error = PTLS_ERROR_INCORRECT_PEM_ECDSA_KEYSIZE;
            if !log_ctx.is_null() {
                (*log_ctx).fn_0.expect("non-null function pointer")(
                    (*log_ctx).ctx,
                    b"Wrong SECP256R1 key length, %d instead of %d.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ecdsa_key_data_length,
                    SECP256R1_PRIVATE_KEY_SIZE,
                );
            }
        } else {
            let mut minicrypto_sign_certificate: *mut ptls_minicrypto_secp256r1sha256_sign_certificate_t = ::core::ptr::null_mut::<
                ptls_minicrypto_secp256r1sha256_sign_certificate_t,
            >();
            minicrypto_sign_certificate = malloc(::core::mem::size_of::<
                ptls_minicrypto_secp256r1sha256_sign_certificate_t,
            >() as size_t)
                as *mut ptls_minicrypto_secp256r1sha256_sign_certificate_t;
            if minicrypto_sign_certificate.is_null() {
                decode_error = PTLS_ERROR_NO_MEMORY;
            } else {
                memset(
                    minicrypto_sign_certificate as *mut ::core::ffi::c_void,
                    0 as ::core::ffi::c_int,
                    ::core::mem::size_of::<ptls_minicrypto_secp256r1sha256_sign_certificate_t>()
                        as size_t,
                );
                decode_error = ptls_minicrypto_init_secp256r1sha256_sign_certificate(
                    minicrypto_sign_certificate,
                    ptls_iovec_init(
                        ecdsa_key_data as *const ::core::ffi::c_void,
                        ecdsa_key_data_length as size_t,
                    ),
                );
            }
            if decode_error == 0 as ::core::ffi::c_int {
                (*ctx).sign_certificate = &raw mut (*minicrypto_sign_certificate).super_0;
                if !log_ctx.is_null() {
                    (*log_ctx).fn_0.expect("non-null function pointer")(
                        (*log_ctx).ctx,
                        b"Initialized SECP512R1 signing key with %d bytes.\n\0".as_ptr()
                            as *const ::core::ffi::c_char,
                        ecdsa_key_data_length,
                    );
                }
            } else if !log_ctx.is_null() {
                (*log_ctx).fn_0.expect("non-null function pointer")(
                    (*log_ctx).ctx,
                    b"SECP512R1 init with %d bytes returns %d.\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    ecdsa_key_data_length,
                    decode_error,
                );
            }
        }
    } else {
        decode_error = PTLS_ERROR_INCORRECT_PEM_ECDSA_CURVE;
        if !log_ctx.is_null() {
            (*log_ctx).fn_0.expect("non-null function pointer")(
                (*log_ctx).ctx,
                b"Curve is not supported for signatures.\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    }
    return decode_error;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_minicrypto_load_private_key(
    mut ctx: *mut ptls_context_t,
    mut pem_fname: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut pkey: ptls_asn1_pkcs8_private_key_t = st_ptls_asn1_pkcs8_private_key_t {
        vec: st_ptls_iovec_t {
            base: ::core::ptr::null_mut::<uint8_t>(),
            len: 0,
        },
        algorithm_index: 0,
        algorithm_length: 0,
        parameters_index: 0,
        parameters_length: 0,
        key_data_index: 0,
        key_data_length: 0,
    };
    let mut ret: ::core::ffi::c_int = ptls_pem_parse_private_key(
        pem_fname,
        &raw mut pkey,
        ::core::ptr::null_mut::<ptls_minicrypto_log_ctx_t>(),
    );
    if !(ret != 0 as ::core::ffi::c_int) {
        if pkey.algorithm_length as usize != ::core::mem::size_of::<[uint8_t; 7]>() as usize
            || memcmp(
                pkey.vec.base.offset(pkey.algorithm_index as isize) as *const ::core::ffi::c_void,
                &raw const ptls_asn1_algorithm_ecdsa as *const uint8_t
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[uint8_t; 7]>() as size_t,
            ) != 0 as ::core::ffi::c_int
        {
            ret = -(1 as ::core::ffi::c_int);
        } else {
            ret = ptls_set_ecdsa_private_key(
                ctx,
                &raw mut pkey,
                ::core::ptr::null_mut::<ptls_minicrypto_log_ctx_t>(),
            );
        }
    }
    if !pkey.vec.base.is_null() {
        ptls_clear_memory.expect("non-null function pointer")(
            pkey.vec.base as *mut ::core::ffi::c_void,
            pkey.vec.len,
        );
        free(pkey.vec.base as *mut ::core::ffi::c_void);
    }
    return ret;
}

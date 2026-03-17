use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type st_ptls_t;
    pub type st_ptls_key_schedule_t;
    pub type st_ptls_traffic_protection_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn strncmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fgets(
        __s: *mut ::core::ffi::c_char,
        __n: ::core::ffi::c_int,
        __stream: *mut FILE,
    ) -> *mut ::core::ffi::c_char;
    fn ptls_buffer__release_memory(buf: *mut ptls_buffer_t);
    fn ptls_buffer__do_pushv(
        buf: *mut ptls_buffer_t,
        src: *const ::core::ffi::c_void,
        len: size_t,
    ) -> ::core::ffi::c_int;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type size_t = usize;
pub type ssize_t = isize;
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
pub type FILE = _IO_FILE;
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
pub type ptls_base64_decode_state_t = st_ptls_base64_decode_state_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_base64_decode_state_t {
    pub nbc: ::core::ffi::c_int,
    pub nbo: ::core::ffi::c_int,
    pub status: ::core::ffi::c_int,
    pub v: uint32_t,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const PTLS_ERROR_CLASS_INTERNAL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const PTLS_ERROR_NO_MEMORY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 1 as ::core::ffi::c_int;
pub const PTLS_ERROR_INCORRECT_BASE64: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 50 as ::core::ffi::c_int;
pub const PTLS_ERROR_PEM_LABEL_NOT_FOUND: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 51 as ::core::ffi::c_int;
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
pub const PTLS_BASE64_DECODE_DONE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PTLS_BASE64_DECODE_IN_PROGRESS: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PTLS_BASE64_DECODE_FAILED: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
static mut ptls_base64_alphabet: [::core::ffi::c_char; 64] = [
    'A' as i32 as ::core::ffi::c_char,
    'B' as i32 as ::core::ffi::c_char,
    'C' as i32 as ::core::ffi::c_char,
    'D' as i32 as ::core::ffi::c_char,
    'E' as i32 as ::core::ffi::c_char,
    'F' as i32 as ::core::ffi::c_char,
    'G' as i32 as ::core::ffi::c_char,
    'H' as i32 as ::core::ffi::c_char,
    'I' as i32 as ::core::ffi::c_char,
    'J' as i32 as ::core::ffi::c_char,
    'K' as i32 as ::core::ffi::c_char,
    'L' as i32 as ::core::ffi::c_char,
    'M' as i32 as ::core::ffi::c_char,
    'N' as i32 as ::core::ffi::c_char,
    'O' as i32 as ::core::ffi::c_char,
    'P' as i32 as ::core::ffi::c_char,
    'Q' as i32 as ::core::ffi::c_char,
    'R' as i32 as ::core::ffi::c_char,
    'S' as i32 as ::core::ffi::c_char,
    'T' as i32 as ::core::ffi::c_char,
    'U' as i32 as ::core::ffi::c_char,
    'V' as i32 as ::core::ffi::c_char,
    'W' as i32 as ::core::ffi::c_char,
    'X' as i32 as ::core::ffi::c_char,
    'Y' as i32 as ::core::ffi::c_char,
    'Z' as i32 as ::core::ffi::c_char,
    'a' as i32 as ::core::ffi::c_char,
    'b' as i32 as ::core::ffi::c_char,
    'c' as i32 as ::core::ffi::c_char,
    'd' as i32 as ::core::ffi::c_char,
    'e' as i32 as ::core::ffi::c_char,
    'f' as i32 as ::core::ffi::c_char,
    'g' as i32 as ::core::ffi::c_char,
    'h' as i32 as ::core::ffi::c_char,
    'i' as i32 as ::core::ffi::c_char,
    'j' as i32 as ::core::ffi::c_char,
    'k' as i32 as ::core::ffi::c_char,
    'l' as i32 as ::core::ffi::c_char,
    'm' as i32 as ::core::ffi::c_char,
    'n' as i32 as ::core::ffi::c_char,
    'o' as i32 as ::core::ffi::c_char,
    'p' as i32 as ::core::ffi::c_char,
    'q' as i32 as ::core::ffi::c_char,
    'r' as i32 as ::core::ffi::c_char,
    's' as i32 as ::core::ffi::c_char,
    't' as i32 as ::core::ffi::c_char,
    'u' as i32 as ::core::ffi::c_char,
    'v' as i32 as ::core::ffi::c_char,
    'w' as i32 as ::core::ffi::c_char,
    'x' as i32 as ::core::ffi::c_char,
    'y' as i32 as ::core::ffi::c_char,
    'z' as i32 as ::core::ffi::c_char,
    '0' as i32 as ::core::ffi::c_char,
    '1' as i32 as ::core::ffi::c_char,
    '2' as i32 as ::core::ffi::c_char,
    '3' as i32 as ::core::ffi::c_char,
    '4' as i32 as ::core::ffi::c_char,
    '5' as i32 as ::core::ffi::c_char,
    '6' as i32 as ::core::ffi::c_char,
    '7' as i32 as ::core::ffi::c_char,
    '8' as i32 as ::core::ffi::c_char,
    '9' as i32 as ::core::ffi::c_char,
    '+' as i32 as ::core::ffi::c_char,
    '/' as i32 as ::core::ffi::c_char,
];
static mut ptls_base64_values: [::core::ffi::c_schar; 128] = [
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    62 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    63 as ::core::ffi::c_int as ::core::ffi::c_schar,
    52 as ::core::ffi::c_int as ::core::ffi::c_schar,
    53 as ::core::ffi::c_int as ::core::ffi::c_schar,
    54 as ::core::ffi::c_int as ::core::ffi::c_schar,
    55 as ::core::ffi::c_int as ::core::ffi::c_schar,
    56 as ::core::ffi::c_int as ::core::ffi::c_schar,
    57 as ::core::ffi::c_int as ::core::ffi::c_schar,
    58 as ::core::ffi::c_int as ::core::ffi::c_schar,
    59 as ::core::ffi::c_int as ::core::ffi::c_schar,
    60 as ::core::ffi::c_int as ::core::ffi::c_schar,
    61 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    0 as ::core::ffi::c_int as ::core::ffi::c_schar,
    1 as ::core::ffi::c_int as ::core::ffi::c_schar,
    2 as ::core::ffi::c_int as ::core::ffi::c_schar,
    3 as ::core::ffi::c_int as ::core::ffi::c_schar,
    4 as ::core::ffi::c_int as ::core::ffi::c_schar,
    5 as ::core::ffi::c_int as ::core::ffi::c_schar,
    6 as ::core::ffi::c_int as ::core::ffi::c_schar,
    7 as ::core::ffi::c_int as ::core::ffi::c_schar,
    8 as ::core::ffi::c_int as ::core::ffi::c_schar,
    9 as ::core::ffi::c_int as ::core::ffi::c_schar,
    10 as ::core::ffi::c_int as ::core::ffi::c_schar,
    11 as ::core::ffi::c_int as ::core::ffi::c_schar,
    12 as ::core::ffi::c_int as ::core::ffi::c_schar,
    13 as ::core::ffi::c_int as ::core::ffi::c_schar,
    14 as ::core::ffi::c_int as ::core::ffi::c_schar,
    15 as ::core::ffi::c_int as ::core::ffi::c_schar,
    16 as ::core::ffi::c_int as ::core::ffi::c_schar,
    17 as ::core::ffi::c_int as ::core::ffi::c_schar,
    18 as ::core::ffi::c_int as ::core::ffi::c_schar,
    19 as ::core::ffi::c_int as ::core::ffi::c_schar,
    20 as ::core::ffi::c_int as ::core::ffi::c_schar,
    21 as ::core::ffi::c_int as ::core::ffi::c_schar,
    22 as ::core::ffi::c_int as ::core::ffi::c_schar,
    23 as ::core::ffi::c_int as ::core::ffi::c_schar,
    24 as ::core::ffi::c_int as ::core::ffi::c_schar,
    25 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    26 as ::core::ffi::c_int as ::core::ffi::c_schar,
    27 as ::core::ffi::c_int as ::core::ffi::c_schar,
    28 as ::core::ffi::c_int as ::core::ffi::c_schar,
    29 as ::core::ffi::c_int as ::core::ffi::c_schar,
    30 as ::core::ffi::c_int as ::core::ffi::c_schar,
    31 as ::core::ffi::c_int as ::core::ffi::c_schar,
    32 as ::core::ffi::c_int as ::core::ffi::c_schar,
    33 as ::core::ffi::c_int as ::core::ffi::c_schar,
    34 as ::core::ffi::c_int as ::core::ffi::c_schar,
    35 as ::core::ffi::c_int as ::core::ffi::c_schar,
    36 as ::core::ffi::c_int as ::core::ffi::c_schar,
    37 as ::core::ffi::c_int as ::core::ffi::c_schar,
    38 as ::core::ffi::c_int as ::core::ffi::c_schar,
    39 as ::core::ffi::c_int as ::core::ffi::c_schar,
    40 as ::core::ffi::c_int as ::core::ffi::c_schar,
    41 as ::core::ffi::c_int as ::core::ffi::c_schar,
    42 as ::core::ffi::c_int as ::core::ffi::c_schar,
    43 as ::core::ffi::c_int as ::core::ffi::c_schar,
    44 as ::core::ffi::c_int as ::core::ffi::c_schar,
    45 as ::core::ffi::c_int as ::core::ffi::c_schar,
    46 as ::core::ffi::c_int as ::core::ffi::c_schar,
    47 as ::core::ffi::c_int as ::core::ffi::c_schar,
    48 as ::core::ffi::c_int as ::core::ffi::c_schar,
    49 as ::core::ffi::c_int as ::core::ffi::c_schar,
    50 as ::core::ffi::c_int as ::core::ffi::c_schar,
    51 as ::core::ffi::c_int as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
    -(1 as ::core::ffi::c_int) as ::core::ffi::c_schar,
];
unsafe extern "C" fn ptls_base64_cell(
    mut data: *const uint8_t,
    mut text: *mut ::core::ffi::c_char,
) {
    let mut n: [::core::ffi::c_int; 4] = [0; 4];
    n[0 as ::core::ffi::c_int as usize] = *data.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        >> 2 as ::core::ffi::c_int;
    n[1 as ::core::ffi::c_int as usize] = (*data.offset(0 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & 3 as ::core::ffi::c_int)
        << 4 as ::core::ffi::c_int
        | *data.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 4 as ::core::ffi::c_int;
    n[2 as ::core::ffi::c_int as usize] = (*data.offset(1 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & 15 as ::core::ffi::c_int)
        << 2 as ::core::ffi::c_int
        | *data.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            >> 6 as ::core::ffi::c_int;
    n[3 as ::core::ffi::c_int as usize] = *data.offset(2 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & 63 as ::core::ffi::c_int;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 4 as ::core::ffi::c_int {
        *text.offset(i as isize) = ptls_base64_alphabet[n[i as usize] as usize];
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ptls_base64_howlong(mut data_length: size_t) -> size_t {
    return data_length
        .wrapping_add(2 as size_t)
        .wrapping_div(3 as size_t)
        .wrapping_mul(4 as size_t);
}
#[no_mangle]
pub unsafe extern "C" fn ptls_base64_encode(
    mut data: *const uint8_t,
    mut data_len: size_t,
    mut ptls_base64_text: *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut l: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut lt: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while data_len.wrapping_sub(l as size_t) >= 3 as size_t {
        ptls_base64_cell(
            data.offset(l as isize),
            ptls_base64_text.offset(lt as isize),
        );
        l += 3 as ::core::ffi::c_int;
        lt += 4 as ::core::ffi::c_int;
    }
    match data_len.wrapping_sub(l as size_t) {
        1 => {
            let c2rust_fresh3 = lt;
            lt = lt + 1;
            *ptls_base64_text.offset(c2rust_fresh3 as isize) =
                ptls_base64_alphabet[(*data.offset(l as isize) as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as usize];
            let c2rust_fresh4 = lt;
            lt = lt + 1;
            *ptls_base64_text.offset(c2rust_fresh4 as isize) =
                ptls_base64_alphabet[((*data.offset(l as isize) as ::core::ffi::c_int
                    & 3 as ::core::ffi::c_int)
                    << 4 as ::core::ffi::c_int) as usize];
            let c2rust_fresh5 = lt;
            lt = lt + 1;
            *ptls_base64_text.offset(c2rust_fresh5 as isize) = '=' as i32 as ::core::ffi::c_char;
            let c2rust_fresh6 = lt;
            lt = lt + 1;
            *ptls_base64_text.offset(c2rust_fresh6 as isize) = '=' as i32 as ::core::ffi::c_char;
        }
        2 => {
            let c2rust_fresh7 = lt;
            lt = lt + 1;
            *ptls_base64_text.offset(c2rust_fresh7 as isize) =
                ptls_base64_alphabet[(*data.offset(l as isize) as ::core::ffi::c_int
                    >> 2 as ::core::ffi::c_int) as usize];
            let c2rust_fresh8 = lt;
            lt = lt + 1;
            *ptls_base64_text.offset(c2rust_fresh8 as isize) =
                ptls_base64_alphabet[((*data.offset(l as isize) as ::core::ffi::c_int
                    & 3 as ::core::ffi::c_int)
                    << 4 as ::core::ffi::c_int
                    | *data.offset((l + 1 as ::core::ffi::c_int) as isize) as ::core::ffi::c_int
                        >> 4 as ::core::ffi::c_int) as usize];
            let c2rust_fresh9 = lt;
            lt = lt + 1;
            *ptls_base64_text.offset(c2rust_fresh9 as isize) =
                ptls_base64_alphabet[((*data.offset((l + 1 as ::core::ffi::c_int) as isize)
                    as ::core::ffi::c_int
                    & 15 as ::core::ffi::c_int)
                    << 2 as ::core::ffi::c_int) as usize];
            let c2rust_fresh10 = lt;
            lt = lt + 1;
            *ptls_base64_text.offset(c2rust_fresh10 as isize) = '=' as i32 as ::core::ffi::c_char;
        }
        0 | _ => {}
    }
    let c2rust_fresh11 = lt;
    lt = lt + 1;
    *ptls_base64_text.offset(c2rust_fresh11 as isize) = 0 as ::core::ffi::c_char;
    return lt;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_base64_decode_init(mut state: *mut ptls_base64_decode_state_t) {
    (*state).nbc = 0 as ::core::ffi::c_int;
    (*state).nbo = 3 as ::core::ffi::c_int;
    (*state).v = 0 as uint32_t;
    (*state).status = PTLS_BASE64_DECODE_IN_PROGRESS;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_base64_decode(
    mut text: *const ::core::ffi::c_char,
    mut state: *mut ptls_base64_decode_state_t,
    mut buf: *mut ptls_buffer_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut decoded: [uint8_t; 3] = [0; 3];
    let mut text_index: size_t = 0 as size_t;
    let mut c: ::core::ffi::c_int = 0;
    let mut vc: ::core::ffi::c_schar = 0;
    while *text.offset(text_index as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
        c = *text.offset(text_index as isize) as ::core::ffi::c_int;
        if !(c == ' ' as i32 || c == '\t' as i32 || c == '\r' as i32 || c == '\n' as i32) {
            break;
        }
        text_index = text_index.wrapping_add(1);
    }
    while *text.offset(text_index as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
        && ret == 0 as ::core::ffi::c_int
        && (*state).status == PTLS_BASE64_DECODE_IN_PROGRESS
    {
        let c2rust_fresh1 = text_index;
        text_index = text_index.wrapping_add(1);
        c = *text.offset(c2rust_fresh1 as isize) as ::core::ffi::c_int;
        vc = (if (0 as ::core::ffi::c_int) < c && c < 0x7f as ::core::ffi::c_int {
            ptls_base64_values[c as usize] as ::core::ffi::c_int
        } else {
            -(1 as ::core::ffi::c_int)
        }) as ::core::ffi::c_schar;
        if vc as ::core::ffi::c_int == -(1 as ::core::ffi::c_int) {
            if (*state).nbc == 2 as ::core::ffi::c_int
                && c == '=' as i32
                && *text.offset(text_index as isize) as ::core::ffi::c_int == '=' as i32
            {
                (*state).nbc = 4 as ::core::ffi::c_int;
                text_index = text_index.wrapping_add(1);
                (*state).nbo = 1 as ::core::ffi::c_int;
                (*state).v <<= 12 as ::core::ffi::c_int;
            } else if (*state).nbc == 3 as ::core::ffi::c_int && c == '=' as i32 {
                (*state).nbc = 4 as ::core::ffi::c_int;
                (*state).nbo = 2 as ::core::ffi::c_int;
                (*state).v <<= 6 as ::core::ffi::c_int;
            } else {
                text_index = text_index.wrapping_sub(1);
                while *text.offset(text_index as isize) as ::core::ffi::c_int
                    != 0 as ::core::ffi::c_int
                {
                    c = *text.offset(text_index as isize) as ::core::ffi::c_int;
                    if !(c == ' ' as i32
                        || c == '\t' as i32
                        || c == '\r' as i32
                        || c == '\n' as i32
                        || c == 0xb as ::core::ffi::c_int
                        || c == 0xc as ::core::ffi::c_int)
                    {
                        break;
                    }
                    text_index = text_index.wrapping_add(1);
                }
                if *text.offset(text_index as isize) as ::core::ffi::c_int
                    == 0 as ::core::ffi::c_int
                {
                    break;
                }
                (*state).nbo = 0 as ::core::ffi::c_int;
                (*state).status = PTLS_BASE64_DECODE_FAILED;
                ret = PTLS_ERROR_INCORRECT_BASE64;
            }
        } else {
            (*state).nbc += 1;
            (*state).v <<= 6 as ::core::ffi::c_int;
            (*state).v |= vc as uint32_t;
        }
        if !(ret == 0 as ::core::ffi::c_int && (*state).nbc == 4 as ::core::ffi::c_int) {
            continue;
        }
        let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while j < (*state).nbo {
            decoded[j as usize] =
                ((*state).v >> 8 as ::core::ffi::c_int * (2 as ::core::ffi::c_int - j)) as uint8_t;
            j += 1;
        }
        ret = ptls_buffer__do_pushv(
            buf,
            &raw mut decoded as *mut uint8_t as *const ::core::ffi::c_void,
            (*state).nbo as size_t,
        );
        if !(ret == 0 as ::core::ffi::c_int) {
            continue;
        }
        if (*state).nbo < 3 as ::core::ffi::c_int {
            while *text.offset(text_index as isize) as ::core::ffi::c_int != 0 as ::core::ffi::c_int
            {
                let c2rust_fresh2 = text_index;
                text_index = text_index.wrapping_add(1);
                c = *text.offset(c2rust_fresh2 as isize) as ::core::ffi::c_int;
                c == ' ' as i32
                    || c == '\t' as i32
                    || c == '\r' as i32
                    || c == '\n' as i32
                    || c == 0xb as ::core::ffi::c_int
                    || c == 0xc as ::core::ffi::c_int;
            }
            if *text.offset(text_index as isize) as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
                (*state).status = PTLS_BASE64_DECODE_DONE;
            } else {
                (*state).status = PTLS_BASE64_DECODE_FAILED;
                ret = PTLS_ERROR_INCORRECT_BASE64;
            }
            break;
        } else {
            (*state).v = 0 as uint32_t;
            (*state).nbo = 3 as ::core::ffi::c_int;
            (*state).nbc = 0 as ::core::ffi::c_int;
        }
    }
    return ret;
}
unsafe extern "C" fn ptls_compare_separator_line(
    mut line: *const ::core::ffi::c_char,
    mut begin_or_end: *const ::core::ffi::c_char,
    mut label: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = strncmp(
        line,
        b"-----\0".as_ptr() as *const ::core::ffi::c_char,
        5 as size_t,
    );
    let mut text_index: size_t = 5 as size_t;
    if ret == 0 as ::core::ffi::c_int {
        let mut begin_or_end_length: size_t = strlen(begin_or_end);
        ret = strncmp(
            line.offset(text_index as isize),
            begin_or_end,
            begin_or_end_length,
        );
        text_index = text_index.wrapping_add(begin_or_end_length);
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = *line.offset(text_index as isize) as ::core::ffi::c_int - ' ' as i32;
        text_index = text_index.wrapping_add(1);
    }
    if ret == 0 as ::core::ffi::c_int {
        let mut label_length: size_t = strlen(label);
        ret = strncmp(line.offset(text_index as isize), label, label_length);
        text_index = text_index.wrapping_add(label_length);
    }
    if ret == 0 as ::core::ffi::c_int {
        ret = strncmp(
            line.offset(text_index as isize),
            b"-----\0".as_ptr() as *const ::core::ffi::c_char,
            5 as size_t,
        );
    }
    return ret;
}
unsafe extern "C" fn ptls_get_pem_object(
    mut F: *mut FILE,
    mut label: *const ::core::ffi::c_char,
    mut buf: *mut ptls_buffer_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = PTLS_ERROR_PEM_LABEL_NOT_FOUND;
    let mut line: [::core::ffi::c_char; 256] = [0; 256];
    let mut state: ptls_base64_decode_state_t = st_ptls_base64_decode_state_t {
        nbc: 0,
        nbo: 0,
        status: 0,
        v: 0,
    };
    while !fgets(
        &raw mut line as *mut ::core::ffi::c_char,
        256 as ::core::ffi::c_int,
        F,
    )
    .is_null()
    {
        if !(ptls_compare_separator_line(
            &raw mut line as *mut ::core::ffi::c_char,
            b"BEGIN\0".as_ptr() as *const ::core::ffi::c_char,
            label,
        ) == 0 as ::core::ffi::c_int)
        {
            continue;
        }
        ret = 0 as ::core::ffi::c_int;
        ptls_base64_decode_init(&raw mut state);
        break;
    }
    while ret == 0 as ::core::ffi::c_int
        && !fgets(
            &raw mut line as *mut ::core::ffi::c_char,
            256 as ::core::ffi::c_int,
            F,
        )
        .is_null()
    {
        if ptls_compare_separator_line(
            &raw mut line as *mut ::core::ffi::c_char,
            b"END\0".as_ptr() as *const ::core::ffi::c_char,
            label,
        ) == 0 as ::core::ffi::c_int
        {
            if state.status == PTLS_BASE64_DECODE_DONE
                || state.status == PTLS_BASE64_DECODE_IN_PROGRESS
                    && state.nbc == 0 as ::core::ffi::c_int
            {
                ret = 0 as ::core::ffi::c_int;
            } else {
                ret = PTLS_ERROR_INCORRECT_BASE64;
            }
            break;
        } else {
            ret = ptls_base64_decode(
                &raw mut line as *mut ::core::ffi::c_char,
                &raw mut state,
                buf,
            );
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_load_pem_objects(
    mut pem_fname: *const ::core::ffi::c_char,
    mut label: *const ::core::ffi::c_char,
    mut list: *mut ptls_iovec_t,
    mut list_max: size_t,
    mut nb_objects: *mut size_t,
) -> ::core::ffi::c_int {
    let mut F: *mut FILE = ::core::ptr::null_mut::<FILE>();
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut count: size_t = 0 as size_t;
    F = fopen(pem_fname, b"r\0".as_ptr() as *const ::core::ffi::c_char) as *mut FILE;
    if F.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    }
    *nb_objects = 0 as size_t;
    if ret == 0 as ::core::ffi::c_int {
        while count < list_max {
            let mut buf: ptls_buffer_t = st_ptls_buffer_t {
                base: ::core::ptr::null_mut::<uint8_t>(),
                capacity: 0,
                off: 0,
                is_allocated: 0,
                align_bits: 0,
            };
            ptls_buffer_init(
                &raw mut buf,
                b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_void,
                0 as size_t,
            );
            ret = ptls_get_pem_object(F, label, &raw mut buf);
            if ret == 0 as ::core::ffi::c_int {
                if buf.off > 0 as size_t && buf.is_allocated as ::core::ffi::c_int != 0 {
                    let ref mut c2rust_fresh0 = (*list.offset(count as isize)).base;
                    *c2rust_fresh0 = buf.base;
                    (*list.offset(count as isize)).len = buf.off;
                    count = count.wrapping_add(1);
                } else {
                    ptls_buffer_dispose(&raw mut buf);
                }
            } else {
                ptls_buffer_dispose(&raw mut buf);
                break;
            }
        }
    }
    if ret == PTLS_ERROR_PEM_LABEL_NOT_FOUND && count > 0 as size_t {
        ret = 0 as ::core::ffi::c_int;
    }
    *nb_objects = count;
    if !F.is_null() {
        fclose(F);
    }
    return ret;
}
pub const PTLS_MAX_CERTS_IN_CONTEXT: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ptls_load_certificates(
    mut ctx: *mut ptls_context_t,
    mut cert_pem_file: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    (*ctx).certificates.list = malloc(
        (PTLS_MAX_CERTS_IN_CONTEXT as size_t)
            .wrapping_mul(::core::mem::size_of::<ptls_iovec_t>() as size_t),
    ) as *mut ptls_iovec_t;
    if (*ctx).certificates.list.is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else {
        ret = ptls_load_pem_objects(
            cert_pem_file,
            b"CERTIFICATE\0".as_ptr() as *const ::core::ffi::c_char,
            (*ctx).certificates.list,
            PTLS_MAX_CERTS_IN_CONTEXT as size_t,
            &raw mut (*ctx).certificates.count,
        );
    }
    return ret;
}

use ::c2rust_bitfields;
extern "C" {
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn ptls_buffer__release_memory(buf: *mut ptls_buffer_t);
    fn ptls_buffer_reserve(buf: *mut ptls_buffer_t, delta: size_t) -> ::core::ffi::c_int;
    fn ptls_buffer__do_pushv(
        buf: *mut ptls_buffer_t,
        src: *const ::core::ffi::c_void,
        len: size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_hkdf_extract(
        hash: *const ptls_hash_algorithm_t,
        output: *mut ::core::ffi::c_void,
        salt: ptls_iovec_t,
        ikm: ptls_iovec_t,
    ) -> ::core::ffi::c_int;
    fn ptls_hkdf_expand(
        hash: *const ptls_hash_algorithm_t,
        output: *mut ::core::ffi::c_void,
        outlen: size_t,
        prk: ptls_iovec_t,
        info: ptls_iovec_t,
    ) -> ::core::ffi::c_int;
    fn ptls_aead_new_direct(
        aead: *const ptls_aead_algorithm_t,
        is_enc: ::core::ffi::c_int,
        key: *const ::core::ffi::c_void,
        iv: *const ::core::ffi::c_void,
    ) -> *mut ptls_aead_context_t;
    fn ptls_aead_free(ctx: *mut ptls_aead_context_t);
    static mut ptls_clear_memory:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
    fn note(fmt: *const ::core::ffi::c_char, ...);
    fn _ok(cond: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
    fn enter_subtest(name: *const ::core::ffi::c_char);
    fn exit_subtest(name: *const ::core::ffi::c_char);
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = isize;
pub type ptls_iovec_t = st_ptls_iovec_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_iovec_t {
    pub base: *mut uint8_t,
    pub len: size_t,
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
    pub tls12: C2Rust_Unnamed,
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
pub struct C2Rust_Unnamed {
    pub fixed_iv_size: size_t,
    pub record_iv_size: size_t,
}
pub type ptls_cipher_algorithm_t = st_ptls_cipher_algorithm_t;
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
pub struct C2Rust_Unnamed_0 {
    pub id: C2Rust_Unnamed_2,
    pub server_pubkey: C2Rust_Unnamed_1,
    pub client_pubkey: C2Rust_Unnamed_1,
    pub dh: C2Rust_Unnamed_1,
    pub expected_secret: [uint8_t; 64],
    pub expected_ciphertext: [[uint8_t; 61]; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub bytes: [uint8_t; 65],
    pub len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub kem: uint16_t,
    pub kdf: uint16_t,
    pub aead: uint16_t,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const NULL_1: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const NULL_0: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const PTLS_HPKE_KEM_P256_SHA256: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_HPKE_KEM_X25519_SHA256: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const PTLS_HPKE_HKDF_SHA256: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PTLS_HPKE_HKDF_SHA512: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PTLS_HPKE_AEAD_AES_128_GCM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
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
unsafe extern "C" fn build_suite_id(
    mut buf: *mut ptls_buffer_t,
    mut kem: *const ptls_hpke_kem_t,
    mut cipher: *const ptls_hpke_cipher_suite_t,
) -> ::core::ffi::c_int {
    let mut c2rust_current_block: u64;
    let mut ret: ::core::ffi::c_int = 0;
    if cipher.is_null() {
        ret = ptls_buffer__do_pushv(
            buf,
            b"KEM\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            3 as size_t,
        );
        if ret != 0 as ::core::ffi::c_int {
            c2rust_current_block = 18318620098634223919;
        } else {
            let mut _v: uint16_t = (*kem).id;
            let mut c2rust_fresh2: [uint8_t; 2] = [
                (_v as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uint8_t,
                _v as uint8_t,
            ];
            ret = ptls_buffer__do_pushv(
                buf,
                &raw mut c2rust_fresh2 as *mut uint8_t as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[uint8_t; 2]>() as size_t,
            );
            if ret != 0 as ::core::ffi::c_int {
                c2rust_current_block = 18318620098634223919;
            } else {
                c2rust_current_block = 6669252993407410313;
            }
        }
    } else {
        ret = ptls_buffer__do_pushv(
            buf,
            b"HPKE\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
        if ret != 0 as ::core::ffi::c_int {
            c2rust_current_block = 18318620098634223919;
        } else {
            let mut _v_0: uint16_t = (*kem).id;
            let mut c2rust_fresh3: [uint8_t; 2] = [
                (_v_0 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uint8_t,
                _v_0 as uint8_t,
            ];
            ret = ptls_buffer__do_pushv(
                buf,
                &raw mut c2rust_fresh3 as *mut uint8_t as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[uint8_t; 2]>() as size_t,
            );
            if ret != 0 as ::core::ffi::c_int {
                c2rust_current_block = 18318620098634223919;
            } else {
                let mut _v_1: uint16_t = (*cipher).id.kdf;
                let mut c2rust_fresh4: [uint8_t; 2] = [
                    (_v_1 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uint8_t,
                    _v_1 as uint8_t,
                ];
                ret = ptls_buffer__do_pushv(
                    buf,
                    &raw mut c2rust_fresh4 as *mut uint8_t as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[uint8_t; 2]>() as size_t,
                );
                if ret != 0 as ::core::ffi::c_int {
                    c2rust_current_block = 18318620098634223919;
                } else {
                    let mut _v_2: uint16_t = (*cipher).id.aead;
                    let mut c2rust_fresh5: [uint8_t; 2] = [
                        (_v_2 as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uint8_t,
                        _v_2 as uint8_t,
                    ];
                    ret = ptls_buffer__do_pushv(
                        buf,
                        &raw mut c2rust_fresh5 as *mut uint8_t as *const ::core::ffi::c_void,
                        ::core::mem::size_of::<[uint8_t; 2]>() as size_t,
                    );
                    if ret != 0 as ::core::ffi::c_int {
                        c2rust_current_block = 18318620098634223919;
                    } else {
                        c2rust_current_block = 6669252993407410313;
                    }
                }
            }
        }
    }
    match c2rust_current_block {
        6669252993407410313 => {
            ret = 0 as ::core::ffi::c_int;
        }
        _ => {}
    }
    return ret;
}
unsafe extern "C" fn labeled_extract(
    mut kem: *const ptls_hpke_kem_t,
    mut cipher: *const ptls_hpke_cipher_suite_t,
    mut output: *mut ::core::ffi::c_void,
    mut salt: ptls_iovec_t,
    mut label: *const ::core::ffi::c_char,
    mut ikm: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut labeled_ikm: ptls_buffer_t = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0,
        off: 0,
        is_allocated: 0,
        align_bits: 0,
    };
    let mut labeled_ikm_smallbuf: [uint8_t; 64] = [0; 64];
    let mut ret: ::core::ffi::c_int = 0;
    ptls_buffer_init(
        &raw mut labeled_ikm,
        &raw mut labeled_ikm_smallbuf as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    ret = ptls_buffer__do_pushv(
        &raw mut labeled_ikm,
        b"HPKE-v1\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        strlen(b"HPKE-v1\0".as_ptr() as *const ::core::ffi::c_char),
    );
    if !(ret != 0 as ::core::ffi::c_int) {
        ret = build_suite_id(&raw mut labeled_ikm, kem, cipher);
        if !(ret != 0 as ::core::ffi::c_int) {
            ret = ptls_buffer__do_pushv(
                &raw mut labeled_ikm,
                label as *const ::core::ffi::c_void,
                strlen(label),
            );
            if !(ret != 0 as ::core::ffi::c_int) {
                ret = ptls_buffer__do_pushv(
                    &raw mut labeled_ikm,
                    ikm.base as *const ::core::ffi::c_void,
                    ikm.len,
                );
                if !(ret != 0 as ::core::ffi::c_int) {
                    ret = ptls_hkdf_extract(
                        if !cipher.is_null() {
                            (*cipher).hash
                        } else {
                            (*kem).hash
                        },
                        output,
                        salt,
                        ptls_iovec_init(
                            labeled_ikm.base as *const ::core::ffi::c_void,
                            labeled_ikm.off,
                        ),
                    );
                }
            }
        }
    }
    ptls_buffer_dispose(&raw mut labeled_ikm);
    return ret;
}
unsafe extern "C" fn labeled_expand(
    mut kem: *const ptls_hpke_kem_t,
    mut cipher: *const ptls_hpke_cipher_suite_t,
    mut output: *mut ::core::ffi::c_void,
    mut outlen: size_t,
    mut prk: ptls_iovec_t,
    mut label: *const ::core::ffi::c_char,
    mut info: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut labeled_info: ptls_buffer_t = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0,
        off: 0,
        is_allocated: 0,
        align_bits: 0,
    };
    let mut labeled_info_smallbuf: [uint8_t; 64] = [0; 64];
    let mut ret: ::core::ffi::c_int = 0;
    ptls_buffer_init(
        &raw mut labeled_info,
        &raw mut labeled_info_smallbuf as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    let mut _v: uint16_t = outlen as uint16_t;
    let mut c2rust_fresh1: [uint8_t; 2] = [
        (_v as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uint8_t,
        _v as uint8_t,
    ];
    ret = ptls_buffer__do_pushv(
        &raw mut labeled_info,
        &raw mut c2rust_fresh1 as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 2]>() as size_t,
    );
    if !(ret != 0 as ::core::ffi::c_int) {
        ret = ptls_buffer__do_pushv(
            &raw mut labeled_info,
            b"HPKE-v1\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            strlen(b"HPKE-v1\0".as_ptr() as *const ::core::ffi::c_char),
        );
        if !(ret != 0 as ::core::ffi::c_int) {
            ret = build_suite_id(&raw mut labeled_info, kem, cipher);
            if !(ret != 0 as ::core::ffi::c_int) {
                ret = ptls_buffer__do_pushv(
                    &raw mut labeled_info,
                    label as *const ::core::ffi::c_void,
                    strlen(label),
                );
                if !(ret != 0 as ::core::ffi::c_int) {
                    ret = ptls_buffer__do_pushv(
                        &raw mut labeled_info,
                        info.base as *const ::core::ffi::c_void,
                        info.len,
                    );
                    if !(ret != 0 as ::core::ffi::c_int) {
                        ret = ptls_hkdf_expand(
                            if !cipher.is_null() {
                                (*cipher).hash
                            } else {
                                (*kem).hash
                            },
                            output,
                            outlen,
                            prk,
                            ptls_iovec_init(
                                labeled_info.base as *const ::core::ffi::c_void,
                                labeled_info.off,
                            ),
                        );
                    }
                }
            }
        }
    }
    ptls_buffer_dispose(&raw mut labeled_info);
    return ret;
}
unsafe extern "C" fn extract_and_expand(
    mut kem: *const ptls_hpke_kem_t,
    mut secret: *mut ::core::ffi::c_void,
    mut secret_len: size_t,
    mut pk_s: ptls_iovec_t,
    mut pk_r: ptls_iovec_t,
    mut dh: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut kem_context: ptls_buffer_t = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0,
        off: 0,
        is_allocated: 0,
        align_bits: 0,
    };
    let mut kem_context_smallbuf: [uint8_t; 128] = [0; 128];
    let mut eae_prk: [uint8_t; 64] = [0; 64];
    let mut ret: ::core::ffi::c_int = 0;
    ptls_buffer_init(
        &raw mut kem_context,
        &raw mut kem_context_smallbuf as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 128]>() as size_t,
    );
    ret = ptls_buffer__do_pushv(
        &raw mut kem_context,
        pk_s.base as *const ::core::ffi::c_void,
        pk_s.len,
    );
    if !(ret != 0 as ::core::ffi::c_int) {
        ret = ptls_buffer__do_pushv(
            &raw mut kem_context,
            pk_r.base as *const ::core::ffi::c_void,
            pk_r.len,
        );
        if !(ret != 0 as ::core::ffi::c_int) {
            ret = labeled_extract(
                kem,
                ::core::ptr::null::<ptls_hpke_cipher_suite_t>(),
                &raw mut eae_prk as *mut uint8_t as *mut ::core::ffi::c_void,
                ptls_iovec_init(
                    b"\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                    0 as size_t,
                ),
                b"eae_prk\0".as_ptr() as *const ::core::ffi::c_char,
                dh,
            );
            if !(ret != 0 as ::core::ffi::c_int) {
                ret = labeled_expand(
                    kem,
                    ::core::ptr::null::<ptls_hpke_cipher_suite_t>(),
                    secret,
                    secret_len,
                    ptls_iovec_init(
                        &raw mut eae_prk as *mut uint8_t as *const ::core::ffi::c_void,
                        (*(*kem).hash).digest_size,
                    ),
                    b"shared_secret\0".as_ptr() as *const ::core::ffi::c_char,
                    ptls_iovec_init(
                        kem_context.base as *const ::core::ffi::c_void,
                        kem_context.off,
                    ),
                );
                ret != 0 as ::core::ffi::c_int;
            }
        }
    }
    ptls_buffer_dispose(&raw mut kem_context);
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut eae_prk as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    return ret;
}
unsafe extern "C" fn dh_derive(
    mut kem: *const ptls_hpke_kem_t,
    mut secret: *mut ::core::ffi::c_void,
    mut pk_s: ptls_iovec_t,
    mut pk_r: ptls_iovec_t,
    mut dh: ptls_iovec_t,
) -> ::core::ffi::c_int {
    return extract_and_expand(kem, secret, (*(*kem).hash).digest_size, pk_s, pk_r, dh);
}
unsafe extern "C" fn dh_encap(
    mut kem: *const ptls_hpke_kem_t,
    mut secret: *mut ::core::ffi::c_void,
    mut pk_s: *mut ptls_iovec_t,
    mut pk_r: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut dh: ptls_iovec_t = st_ptls_iovec_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    };
    let mut ret: ::core::ffi::c_int = 0;
    *pk_s = ptls_iovec_init(::core::ptr::null::<::core::ffi::c_void>(), 0 as size_t);
    ret = (*(*kem).keyex).exchange.expect("non-null function pointer")(
        (*kem).keyex as *const st_ptls_key_exchange_algorithm_t,
        pk_s,
        &raw mut dh,
        pk_r,
    );
    if !(ret != 0 as ::core::ffi::c_int) {
        ret = dh_derive(kem, secret, *pk_s, pk_r, dh);
        ret != 0 as ::core::ffi::c_int;
    }
    if !dh.base.is_null() {
        ptls_clear_memory.expect("non-null function pointer")(
            dh.base as *mut ::core::ffi::c_void,
            dh.len,
        );
        free(dh.base as *mut ::core::ffi::c_void);
    }
    if ret != 0 as ::core::ffi::c_int {
        free((*pk_s).base as *mut ::core::ffi::c_void);
        *pk_s = ptls_iovec_init(::core::ptr::null::<::core::ffi::c_void>(), 0 as size_t);
    }
    return ret;
}
unsafe extern "C" fn dh_decap(
    mut kem: *const ptls_hpke_kem_t,
    mut secret: *mut ::core::ffi::c_void,
    mut keyex: *mut ptls_key_exchange_context_t,
    mut pk_s: ptls_iovec_t,
    mut pk_r: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut dh: ptls_iovec_t = st_ptls_iovec_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    };
    let mut ret: ::core::ffi::c_int = 0;
    ret = (*keyex).on_exchange.expect("non-null function pointer")(
        &raw mut keyex,
        0 as ::core::ffi::c_int,
        &raw mut dh,
        pk_s,
    );
    if !(ret != 0 as ::core::ffi::c_int) {
        ret = dh_derive(kem, secret, pk_s, pk_r, dh);
        ret != 0 as ::core::ffi::c_int;
    }
    if !dh.base.is_null() {
        ptls_clear_memory.expect("non-null function pointer")(
            dh.base as *mut ::core::ffi::c_void,
            dh.len,
        );
        free(dh.base as *mut ::core::ffi::c_void);
    }
    return ret;
}
unsafe extern "C" fn key_schedule(
    mut kem: *const ptls_hpke_kem_t,
    mut cipher: *const ptls_hpke_cipher_suite_t,
    mut ctx: *mut *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut shared_secret: *const ::core::ffi::c_void,
    mut info: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut key_schedule_context: ptls_buffer_t = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0,
        off: 0,
        is_allocated: 0,
        align_bits: 0,
    };
    let mut key_schedule_context_smallbuf: [uint8_t; 128] = [0; 128];
    let mut secret: [uint8_t; 64] = [0; 64];
    let mut key: [uint8_t; 32] = [0; 32];
    let mut base_nonce: [uint8_t; 32] = [0; 32];
    let mut ret: ::core::ffi::c_int = 0;
    *ctx = ::core::ptr::null_mut::<ptls_aead_context_t>();
    ptls_buffer_init(
        &raw mut key_schedule_context,
        &raw mut key_schedule_context_smallbuf as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 128]>() as size_t,
    );
    let mut c2rust_fresh0: [uint8_t; 1] = [0 as ::core::ffi::c_int as uint8_t];
    ret = ptls_buffer__do_pushv(
        &raw mut key_schedule_context,
        &raw mut c2rust_fresh0 as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 1]>() as size_t,
    );
    if !(ret != 0 as ::core::ffi::c_int) {
        ret = ptls_buffer_reserve(&raw mut key_schedule_context, (*(*cipher).hash).digest_size);
        if !(ret != 0 as ::core::ffi::c_int || {
            ret = labeled_extract(
                kem,
                cipher,
                key_schedule_context
                    .base
                    .offset(key_schedule_context.off as isize)
                    as *mut ::core::ffi::c_void,
                ptls_iovec_init(::core::ptr::null::<::core::ffi::c_void>(), 0 as size_t),
                b"psk_id_hash\0".as_ptr() as *const ::core::ffi::c_char,
                ptls_iovec_init(::core::ptr::null::<::core::ffi::c_void>(), 0 as size_t),
            );
            ret != 0 as ::core::ffi::c_int
        }) {
            key_schedule_context.off = key_schedule_context
                .off
                .wrapping_add((*(*cipher).hash).digest_size);
            ret = ptls_buffer_reserve(&raw mut key_schedule_context, (*(*cipher).hash).digest_size);
            if !(ret != 0 as ::core::ffi::c_int || {
                ret = labeled_extract(
                    kem,
                    cipher,
                    key_schedule_context
                        .base
                        .offset(key_schedule_context.off as isize)
                        as *mut ::core::ffi::c_void,
                    ptls_iovec_init(::core::ptr::null::<::core::ffi::c_void>(), 0 as size_t),
                    b"info_hash\0".as_ptr() as *const ::core::ffi::c_char,
                    info,
                );
                ret != 0 as ::core::ffi::c_int
            }) {
                key_schedule_context.off = key_schedule_context
                    .off
                    .wrapping_add((*(*cipher).hash).digest_size);
                ret = labeled_extract(
                    kem,
                    cipher,
                    &raw mut secret as *mut uint8_t as *mut ::core::ffi::c_void,
                    ptls_iovec_init(shared_secret, (*(*kem).hash).digest_size),
                    b"secret\0".as_ptr() as *const ::core::ffi::c_char,
                    ptls_iovec_init(
                        b"\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
                        0 as size_t,
                    ),
                );
                if !(ret != 0 as ::core::ffi::c_int) {
                    ret = labeled_expand(
                        kem,
                        cipher,
                        &raw mut key as *mut uint8_t as *mut ::core::ffi::c_void,
                        (*(*cipher).aead).key_size,
                        ptls_iovec_init(
                            &raw mut secret as *mut uint8_t as *const ::core::ffi::c_void,
                            (*(*cipher).hash).digest_size,
                        ),
                        b"key\0".as_ptr() as *const ::core::ffi::c_char,
                        ptls_iovec_init(
                            key_schedule_context.base as *const ::core::ffi::c_void,
                            key_schedule_context.off,
                        ),
                    );
                    if !(ret != 0 as ::core::ffi::c_int) {
                        ret = labeled_expand(
                            kem,
                            cipher,
                            &raw mut base_nonce as *mut uint8_t as *mut ::core::ffi::c_void,
                            (*(*cipher).aead).iv_size,
                            ptls_iovec_init(
                                &raw mut secret as *mut uint8_t as *const ::core::ffi::c_void,
                                (*(*cipher).hash).digest_size,
                            ),
                            b"base_nonce\0".as_ptr() as *const ::core::ffi::c_char,
                            ptls_iovec_init(
                                key_schedule_context.base as *const ::core::ffi::c_void,
                                key_schedule_context.off,
                            ),
                        );
                        if !(ret != 0 as ::core::ffi::c_int) {
                            *ctx = ptls_aead_new_direct(
                                (*cipher).aead,
                                is_enc,
                                &raw mut key as *mut uint8_t as *const ::core::ffi::c_void,
                                &raw mut base_nonce as *mut uint8_t as *const ::core::ffi::c_void,
                            );
                        }
                    }
                }
            }
        }
    }
    ptls_buffer_dispose(&raw mut key_schedule_context);
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut secret as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut key as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut base_nonce as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_hpke_setup_base_s(
    mut kem: *const ptls_hpke_kem_t,
    mut cipher: *const ptls_hpke_cipher_suite_t,
    mut pk_s: *mut ptls_iovec_t,
    mut ctx: *mut *mut ptls_aead_context_t,
    mut pk_r: ptls_iovec_t,
    mut info: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut secret: [uint8_t; 64] = [0; 64];
    let mut ret: ::core::ffi::c_int = 0;
    *pk_s = ptls_iovec_init(::core::ptr::null::<::core::ffi::c_void>(), 0 as size_t);
    ret = dh_encap(
        kem,
        &raw mut secret as *mut uint8_t as *mut ::core::ffi::c_void,
        pk_s,
        pk_r,
    );
    if !(ret != 0 as ::core::ffi::c_int) {
        ret = key_schedule(
            kem,
            cipher,
            ctx,
            1 as ::core::ffi::c_int,
            &raw mut secret as *mut uint8_t as *const ::core::ffi::c_void,
            info,
        );
        ret != 0 as ::core::ffi::c_int;
    }
    if ret != 0 as ::core::ffi::c_int && (*pk_s).len != 0 as size_t {
        ptls_clear_memory.expect("non-null function pointer")(
            (*pk_s).base as *mut ::core::ffi::c_void,
            (*pk_s).len,
        );
        free((*pk_s).base as *mut ::core::ffi::c_void);
        *pk_s = ptls_iovec_init(::core::ptr::null::<::core::ffi::c_void>(), 0 as size_t);
    }
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut secret as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_hpke_setup_base_r(
    mut kem: *const ptls_hpke_kem_t,
    mut cipher: *const ptls_hpke_cipher_suite_t,
    mut keyex: *mut ptls_key_exchange_context_t,
    mut ctx: *mut *mut ptls_aead_context_t,
    mut pk_s: ptls_iovec_t,
    mut info: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut secret: [uint8_t; 64] = [0; 64];
    let mut ret: ::core::ffi::c_int = 0;
    ret = dh_decap(
        kem,
        &raw mut secret as *mut uint8_t as *mut ::core::ffi::c_void,
        keyex,
        pk_s,
        (*keyex).pubkey,
    );
    if !(ret != 0 as ::core::ffi::c_int) {
        ret = key_schedule(
            kem,
            cipher,
            ctx,
            0 as ::core::ffi::c_int,
            &raw mut secret as *mut uint8_t as *const ::core::ffi::c_void,
            info,
        );
        ret != 0 as ::core::ffi::c_int;
    }
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut secret as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    return ret;
}
static mut test_kem: *const ptls_hpke_kem_t = ::core::ptr::null::<ptls_hpke_kem_t>();
static mut test_cipher: *const ptls_hpke_cipher_suite_t =
    ::core::ptr::null::<ptls_hpke_cipher_suite_t>();
#[no_mangle]
pub unsafe extern "C" fn test_one_hpke() {
    static mut cleartext: [uint8_t; 29] = [
        0x42 as ::core::ffi::c_int as uint8_t,
        0x65 as ::core::ffi::c_int as uint8_t,
        0x61 as ::core::ffi::c_int as uint8_t,
        0x75 as ::core::ffi::c_int as uint8_t,
        0x74 as ::core::ffi::c_int as uint8_t,
        0x79 as ::core::ffi::c_int as uint8_t,
        0x20 as ::core::ffi::c_int as uint8_t,
        0x69 as ::core::ffi::c_int as uint8_t,
        0x73 as ::core::ffi::c_int as uint8_t,
        0x20 as ::core::ffi::c_int as uint8_t,
        0x74 as ::core::ffi::c_int as uint8_t,
        0x72 as ::core::ffi::c_int as uint8_t,
        0x75 as ::core::ffi::c_int as uint8_t,
        0x74 as ::core::ffi::c_int as uint8_t,
        0x68 as ::core::ffi::c_int as uint8_t,
        0x2c as ::core::ffi::c_int as uint8_t,
        0x20 as ::core::ffi::c_int as uint8_t,
        0x74 as ::core::ffi::c_int as uint8_t,
        0x72 as ::core::ffi::c_int as uint8_t,
        0x75 as ::core::ffi::c_int as uint8_t,
        0x74 as ::core::ffi::c_int as uint8_t,
        0x68 as ::core::ffi::c_int as uint8_t,
        0x20 as ::core::ffi::c_int as uint8_t,
        0x62 as ::core::ffi::c_int as uint8_t,
        0x65 as ::core::ffi::c_int as uint8_t,
        0x61 as ::core::ffi::c_int as uint8_t,
        0x75 as ::core::ffi::c_int as uint8_t,
        0x74 as ::core::ffi::c_int as uint8_t,
        0x79 as ::core::ffi::c_int as uint8_t,
    ];
    static mut info: [uint8_t; 20] = [
        0x4f as ::core::ffi::c_int as uint8_t,
        0x64 as ::core::ffi::c_int as uint8_t,
        0x65 as ::core::ffi::c_int as uint8_t,
        0x20 as ::core::ffi::c_int as uint8_t,
        0x6f as ::core::ffi::c_int as uint8_t,
        0x6e as ::core::ffi::c_int as uint8_t,
        0x20 as ::core::ffi::c_int as uint8_t,
        0x61 as ::core::ffi::c_int as uint8_t,
        0x20 as ::core::ffi::c_int as uint8_t,
        0x47 as ::core::ffi::c_int as uint8_t,
        0x72 as ::core::ffi::c_int as uint8_t,
        0x65 as ::core::ffi::c_int as uint8_t,
        0x63 as ::core::ffi::c_int as uint8_t,
        0x69 as ::core::ffi::c_int as uint8_t,
        0x61 as ::core::ffi::c_int as uint8_t,
        0x6e as ::core::ffi::c_int as uint8_t,
        0x20 as ::core::ffi::c_int as uint8_t,
        0x55 as ::core::ffi::c_int as uint8_t,
        0x72 as ::core::ffi::c_int as uint8_t,
        0x6e as ::core::ffi::c_int as uint8_t,
    ];
    static mut aad: [[uint8_t; 7]; 2] = [
        [
            0x43 as ::core::ffi::c_int as uint8_t,
            0x6f as ::core::ffi::c_int as uint8_t,
            0x75 as ::core::ffi::c_int as uint8_t,
            0x6e as ::core::ffi::c_int as uint8_t,
            0x74 as ::core::ffi::c_int as uint8_t,
            0x2d as ::core::ffi::c_int as uint8_t,
            0x30 as ::core::ffi::c_int as uint8_t,
        ],
        [
            0x43 as ::core::ffi::c_int as uint8_t,
            0x6f as ::core::ffi::c_int as uint8_t,
            0x75 as ::core::ffi::c_int as uint8_t,
            0x6e as ::core::ffi::c_int as uint8_t,
            0x74 as ::core::ffi::c_int as uint8_t,
            0x2d as ::core::ffi::c_int as uint8_t,
            0x31 as ::core::ffi::c_int as uint8_t,
        ],
    ];
    static mut all: [C2Rust_Unnamed_0; 4] = [
        C2Rust_Unnamed_0 {
            id: C2Rust_Unnamed_2 {
                kem: PTLS_HPKE_KEM_X25519_SHA256 as uint16_t,
                kdf: PTLS_HPKE_HKDF_SHA256 as uint16_t,
                aead: PTLS_HPKE_AEAD_AES_128_GCM as uint16_t,
            },
            server_pubkey: C2Rust_Unnamed_1 {
                bytes: [
                    0x39 as ::core::ffi::c_int as uint8_t,
                    0x48 as ::core::ffi::c_int as uint8_t,
                    0xcf as ::core::ffi::c_int as uint8_t,
                    0xe0 as ::core::ffi::c_int as uint8_t,
                    0xad as ::core::ffi::c_int as uint8_t,
                    0x1d as ::core::ffi::c_int as uint8_t,
                    0xdb as ::core::ffi::c_int as uint8_t,
                    0x69 as ::core::ffi::c_int as uint8_t,
                    0x5d as ::core::ffi::c_int as uint8_t,
                    0x78 as ::core::ffi::c_int as uint8_t,
                    0xe as ::core::ffi::c_int as uint8_t,
                    0x59 as ::core::ffi::c_int as uint8_t,
                    0x7 as ::core::ffi::c_int as uint8_t,
                    0x71 as ::core::ffi::c_int as uint8_t,
                    0x95 as ::core::ffi::c_int as uint8_t,
                    0xda as ::core::ffi::c_int as uint8_t,
                    0x6c as ::core::ffi::c_int as uint8_t,
                    0x56 as ::core::ffi::c_int as uint8_t,
                    0x50 as ::core::ffi::c_int as uint8_t,
                    0x6b as ::core::ffi::c_int as uint8_t,
                    0x2 as ::core::ffi::c_int as uint8_t,
                    0x73 as ::core::ffi::c_int as uint8_t,
                    0x29 as ::core::ffi::c_int as uint8_t,
                    0x79 as ::core::ffi::c_int as uint8_t,
                    0x4a as ::core::ffi::c_int as uint8_t,
                    0xb0 as ::core::ffi::c_int as uint8_t,
                    0x2b as ::core::ffi::c_int as uint8_t,
                    0xca as ::core::ffi::c_int as uint8_t,
                    0x80 as ::core::ffi::c_int as uint8_t,
                    0x81 as ::core::ffi::c_int as uint8_t,
                    0x5c as ::core::ffi::c_int as uint8_t,
                    0x4d as ::core::ffi::c_int as uint8_t,
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
                len: 32 as size_t,
            },
            client_pubkey: C2Rust_Unnamed_1 {
                bytes: [
                    0x37 as ::core::ffi::c_int as uint8_t,
                    0xfd as ::core::ffi::c_int as uint8_t,
                    0xa3 as ::core::ffi::c_int as uint8_t,
                    0x56 as ::core::ffi::c_int as uint8_t,
                    0x7b as ::core::ffi::c_int as uint8_t,
                    0xdb as ::core::ffi::c_int as uint8_t,
                    0xd6 as ::core::ffi::c_int as uint8_t,
                    0x28 as ::core::ffi::c_int as uint8_t,
                    0xe8 as ::core::ffi::c_int as uint8_t,
                    0x86 as ::core::ffi::c_int as uint8_t,
                    0x68 as ::core::ffi::c_int as uint8_t,
                    0xc3 as ::core::ffi::c_int as uint8_t,
                    0xc8 as ::core::ffi::c_int as uint8_t,
                    0xd7 as ::core::ffi::c_int as uint8_t,
                    0xe9 as ::core::ffi::c_int as uint8_t,
                    0x7d as ::core::ffi::c_int as uint8_t,
                    0x1d as ::core::ffi::c_int as uint8_t,
                    0x12 as ::core::ffi::c_int as uint8_t,
                    0x53 as ::core::ffi::c_int as uint8_t,
                    0xb6 as ::core::ffi::c_int as uint8_t,
                    0xd4 as ::core::ffi::c_int as uint8_t,
                    0xea as ::core::ffi::c_int as uint8_t,
                    0x6d as ::core::ffi::c_int as uint8_t,
                    0x44 as ::core::ffi::c_int as uint8_t,
                    0xc1 as ::core::ffi::c_int as uint8_t,
                    0x50 as ::core::ffi::c_int as uint8_t,
                    0xf7 as ::core::ffi::c_int as uint8_t,
                    0x41 as ::core::ffi::c_int as uint8_t,
                    0xf1 as ::core::ffi::c_int as uint8_t,
                    0xbf as ::core::ffi::c_int as uint8_t,
                    0x44 as ::core::ffi::c_int as uint8_t,
                    0x31 as ::core::ffi::c_int as uint8_t,
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
                len: 32 as size_t,
            },
            dh: C2Rust_Unnamed_1 {
                bytes: [
                    0xb3 as ::core::ffi::c_int as uint8_t,
                    0xb5 as ::core::ffi::c_int as uint8_t,
                    0xc1 as ::core::ffi::c_int as uint8_t,
                    0x9e as ::core::ffi::c_int as uint8_t,
                    0xab as ::core::ffi::c_int as uint8_t,
                    0x3f as ::core::ffi::c_int as uint8_t,
                    0x8 as ::core::ffi::c_int as uint8_t,
                    0x8a as ::core::ffi::c_int as uint8_t,
                    0xc1 as ::core::ffi::c_int as uint8_t,
                    0x8f as ::core::ffi::c_int as uint8_t,
                    0x23 as ::core::ffi::c_int as uint8_t,
                    0xf7 as ::core::ffi::c_int as uint8_t,
                    0x74 as ::core::ffi::c_int as uint8_t,
                    0xff as ::core::ffi::c_int as uint8_t,
                    0x64 as ::core::ffi::c_int as uint8_t,
                    0x14 as ::core::ffi::c_int as uint8_t,
                    0xba as ::core::ffi::c_int as uint8_t,
                    0x4f as ::core::ffi::c_int as uint8_t,
                    0xde as ::core::ffi::c_int as uint8_t,
                    0x45 as ::core::ffi::c_int as uint8_t,
                    0x40 as ::core::ffi::c_int as uint8_t,
                    0x4d as ::core::ffi::c_int as uint8_t,
                    0x10 as ::core::ffi::c_int as uint8_t,
                    0x8 as ::core::ffi::c_int as uint8_t,
                    0x5e as ::core::ffi::c_int as uint8_t,
                    0xfc as ::core::ffi::c_int as uint8_t,
                    0x3e as ::core::ffi::c_int as uint8_t,
                    0x4d as ::core::ffi::c_int as uint8_t,
                    0xc9 as ::core::ffi::c_int as uint8_t,
                    0xc7 as ::core::ffi::c_int as uint8_t,
                    0x2e as ::core::ffi::c_int as uint8_t,
                    0x35 as ::core::ffi::c_int as uint8_t,
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
                len: 32 as size_t,
            },
            expected_secret: [
                0xfe as ::core::ffi::c_int as uint8_t,
                0xe as ::core::ffi::c_int as uint8_t,
                0x18 as ::core::ffi::c_int as uint8_t,
                0xc9 as ::core::ffi::c_int as uint8_t,
                0xf0 as ::core::ffi::c_int as uint8_t,
                0x24 as ::core::ffi::c_int as uint8_t,
                0xce as ::core::ffi::c_int as uint8_t,
                0x43 as ::core::ffi::c_int as uint8_t,
                0x79 as ::core::ffi::c_int as uint8_t,
                0x9a as ::core::ffi::c_int as uint8_t,
                0xe3 as ::core::ffi::c_int as uint8_t,
                0x93 as ::core::ffi::c_int as uint8_t,
                0xc7 as ::core::ffi::c_int as uint8_t,
                0xe8 as ::core::ffi::c_int as uint8_t,
                0xfe as ::core::ffi::c_int as uint8_t,
                0x8f as ::core::ffi::c_int as uint8_t,
                0xce as ::core::ffi::c_int as uint8_t,
                0x9d as ::core::ffi::c_int as uint8_t,
                0x21 as ::core::ffi::c_int as uint8_t,
                0x88 as ::core::ffi::c_int as uint8_t,
                0x75 as ::core::ffi::c_int as uint8_t,
                0xe8 as ::core::ffi::c_int as uint8_t,
                0x22 as ::core::ffi::c_int as uint8_t,
                0x7b as ::core::ffi::c_int as uint8_t,
                0x1 as ::core::ffi::c_int as uint8_t,
                0x87 as ::core::ffi::c_int as uint8_t,
                0xc0 as ::core::ffi::c_int as uint8_t,
                0x4e as ::core::ffi::c_int as uint8_t,
                0x7d as ::core::ffi::c_int as uint8_t,
                0x2e as ::core::ffi::c_int as uint8_t,
                0xa1 as ::core::ffi::c_int as uint8_t,
                0xfc as ::core::ffi::c_int as uint8_t,
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
            expected_ciphertext: [
                [
                    0xf9 as ::core::ffi::c_int as uint8_t,
                    0x38 as ::core::ffi::c_int as uint8_t,
                    0x55 as ::core::ffi::c_int as uint8_t,
                    0x8b as ::core::ffi::c_int as uint8_t,
                    0x5d as ::core::ffi::c_int as uint8_t,
                    0x72 as ::core::ffi::c_int as uint8_t,
                    0xf1 as ::core::ffi::c_int as uint8_t,
                    0xa2 as ::core::ffi::c_int as uint8_t,
                    0x38 as ::core::ffi::c_int as uint8_t,
                    0x10 as ::core::ffi::c_int as uint8_t,
                    0xb4 as ::core::ffi::c_int as uint8_t,
                    0xbe as ::core::ffi::c_int as uint8_t,
                    0x2a as ::core::ffi::c_int as uint8_t,
                    0xb4 as ::core::ffi::c_int as uint8_t,
                    0xf8 as ::core::ffi::c_int as uint8_t,
                    0x43 as ::core::ffi::c_int as uint8_t,
                    0x31 as ::core::ffi::c_int as uint8_t,
                    0xac as ::core::ffi::c_int as uint8_t,
                    0xc0 as ::core::ffi::c_int as uint8_t,
                    0x2f as ::core::ffi::c_int as uint8_t,
                    0xc9 as ::core::ffi::c_int as uint8_t,
                    0x7b as ::core::ffi::c_int as uint8_t,
                    0xab as ::core::ffi::c_int as uint8_t,
                    0xc5 as ::core::ffi::c_int as uint8_t,
                    0x3a as ::core::ffi::c_int as uint8_t,
                    0x52 as ::core::ffi::c_int as uint8_t,
                    0xae as ::core::ffi::c_int as uint8_t,
                    0x82 as ::core::ffi::c_int as uint8_t,
                    0x18 as ::core::ffi::c_int as uint8_t,
                    0xa3 as ::core::ffi::c_int as uint8_t,
                    0x55 as ::core::ffi::c_int as uint8_t,
                    0xa9 as ::core::ffi::c_int as uint8_t,
                    0x6d as ::core::ffi::c_int as uint8_t,
                    0x87 as ::core::ffi::c_int as uint8_t,
                    0x70 as ::core::ffi::c_int as uint8_t,
                    0xac as ::core::ffi::c_int as uint8_t,
                    0x83 as ::core::ffi::c_int as uint8_t,
                    0xd0 as ::core::ffi::c_int as uint8_t,
                    0x7b as ::core::ffi::c_int as uint8_t,
                    0xea as ::core::ffi::c_int as uint8_t,
                    0x87 as ::core::ffi::c_int as uint8_t,
                    0xe1 as ::core::ffi::c_int as uint8_t,
                    0x3c as ::core::ffi::c_int as uint8_t,
                    0x51 as ::core::ffi::c_int as uint8_t,
                    0x2a as ::core::ffi::c_int as uint8_t,
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
                [
                    0xaf as ::core::ffi::c_int as uint8_t,
                    0x2d as ::core::ffi::c_int as uint8_t,
                    0x7e as ::core::ffi::c_int as uint8_t,
                    0x9a as ::core::ffi::c_int as uint8_t,
                    0xc9 as ::core::ffi::c_int as uint8_t,
                    0xae as ::core::ffi::c_int as uint8_t,
                    0x7e as ::core::ffi::c_int as uint8_t,
                    0x27 as ::core::ffi::c_int as uint8_t,
                    0xf as ::core::ffi::c_int as uint8_t,
                    0x46 as ::core::ffi::c_int as uint8_t,
                    0xba as ::core::ffi::c_int as uint8_t,
                    0x1f as ::core::ffi::c_int as uint8_t,
                    0x97 as ::core::ffi::c_int as uint8_t,
                    0x5b as ::core::ffi::c_int as uint8_t,
                    0xe5 as ::core::ffi::c_int as uint8_t,
                    0x3c as ::core::ffi::c_int as uint8_t,
                    0x9 as ::core::ffi::c_int as uint8_t,
                    0xf8 as ::core::ffi::c_int as uint8_t,
                    0xd8 as ::core::ffi::c_int as uint8_t,
                    0x75 as ::core::ffi::c_int as uint8_t,
                    0xbd as ::core::ffi::c_int as uint8_t,
                    0xc8 as ::core::ffi::c_int as uint8_t,
                    0x53 as ::core::ffi::c_int as uint8_t,
                    0x54 as ::core::ffi::c_int as uint8_t,
                    0x58 as ::core::ffi::c_int as uint8_t,
                    0xc2 as ::core::ffi::c_int as uint8_t,
                    0x49 as ::core::ffi::c_int as uint8_t,
                    0x4e as ::core::ffi::c_int as uint8_t,
                    0x8a as ::core::ffi::c_int as uint8_t,
                    0x6e as ::core::ffi::c_int as uint8_t,
                    0xab as ::core::ffi::c_int as uint8_t,
                    0x25 as ::core::ffi::c_int as uint8_t,
                    0x1c as ::core::ffi::c_int as uint8_t,
                    0x3 as ::core::ffi::c_int as uint8_t,
                    0xd0 as ::core::ffi::c_int as uint8_t,
                    0xc2 as ::core::ffi::c_int as uint8_t,
                    0x2a as ::core::ffi::c_int as uint8_t,
                    0x56 as ::core::ffi::c_int as uint8_t,
                    0xb8 as ::core::ffi::c_int as uint8_t,
                    0xca as ::core::ffi::c_int as uint8_t,
                    0x42 as ::core::ffi::c_int as uint8_t,
                    0xc2 as ::core::ffi::c_int as uint8_t,
                    0x6 as ::core::ffi::c_int as uint8_t,
                    0x3b as ::core::ffi::c_int as uint8_t,
                    0x84 as ::core::ffi::c_int as uint8_t,
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
            ],
        },
        C2Rust_Unnamed_0 {
            id: C2Rust_Unnamed_2 {
                kem: PTLS_HPKE_KEM_P256_SHA256 as uint16_t,
                kdf: PTLS_HPKE_HKDF_SHA256 as uint16_t,
                aead: PTLS_HPKE_AEAD_AES_128_GCM as uint16_t,
            },
            server_pubkey: C2Rust_Unnamed_1 {
                bytes: [
                    0x4 as ::core::ffi::c_int as uint8_t,
                    0xfe as ::core::ffi::c_int as uint8_t,
                    0x8c as ::core::ffi::c_int as uint8_t,
                    0x19 as ::core::ffi::c_int as uint8_t,
                    0xce as ::core::ffi::c_int as uint8_t,
                    0x9 as ::core::ffi::c_int as uint8_t,
                    0x5 as ::core::ffi::c_int as uint8_t,
                    0x19 as ::core::ffi::c_int as uint8_t,
                    0x1e as ::core::ffi::c_int as uint8_t,
                    0xbc as ::core::ffi::c_int as uint8_t,
                    0x29 as ::core::ffi::c_int as uint8_t,
                    0x8a as ::core::ffi::c_int as uint8_t,
                    0x92 as ::core::ffi::c_int as uint8_t,
                    0x45 as ::core::ffi::c_int as uint8_t,
                    0x79 as ::core::ffi::c_int as uint8_t,
                    0x25 as ::core::ffi::c_int as uint8_t,
                    0x31 as ::core::ffi::c_int as uint8_t,
                    0xf2 as ::core::ffi::c_int as uint8_t,
                    0x6f as ::core::ffi::c_int as uint8_t,
                    0xc as ::core::ffi::c_int as uint8_t,
                    0xec as ::core::ffi::c_int as uint8_t,
                    0xe2 as ::core::ffi::c_int as uint8_t,
                    0x46 as ::core::ffi::c_int as uint8_t,
                    0x6 as ::core::ffi::c_int as uint8_t,
                    0x39 as ::core::ffi::c_int as uint8_t,
                    0xe8 as ::core::ffi::c_int as uint8_t,
                    0xbc as ::core::ffi::c_int as uint8_t,
                    0x39 as ::core::ffi::c_int as uint8_t,
                    0xcb as ::core::ffi::c_int as uint8_t,
                    0x7f as ::core::ffi::c_int as uint8_t,
                    0x70 as ::core::ffi::c_int as uint8_t,
                    0x6a as ::core::ffi::c_int as uint8_t,
                    0x82 as ::core::ffi::c_int as uint8_t,
                    0x6a as ::core::ffi::c_int as uint8_t,
                    0x77 as ::core::ffi::c_int as uint8_t,
                    0x9b as ::core::ffi::c_int as uint8_t,
                    0x4c as ::core::ffi::c_int as uint8_t,
                    0xf9 as ::core::ffi::c_int as uint8_t,
                    0x69 as ::core::ffi::c_int as uint8_t,
                    0xb8 as ::core::ffi::c_int as uint8_t,
                    0xa0 as ::core::ffi::c_int as uint8_t,
                    0xe5 as ::core::ffi::c_int as uint8_t,
                    0x39 as ::core::ffi::c_int as uint8_t,
                    0xc7 as ::core::ffi::c_int as uint8_t,
                    0xf6 as ::core::ffi::c_int as uint8_t,
                    0x2f as ::core::ffi::c_int as uint8_t,
                    0xb3 as ::core::ffi::c_int as uint8_t,
                    0xd3 as ::core::ffi::c_int as uint8_t,
                    0xa as ::core::ffi::c_int as uint8_t,
                    0xd6 as ::core::ffi::c_int as uint8_t,
                    0xaa as ::core::ffi::c_int as uint8_t,
                    0x8f as ::core::ffi::c_int as uint8_t,
                    0x80 as ::core::ffi::c_int as uint8_t,
                    0xe3 as ::core::ffi::c_int as uint8_t,
                    0xf as ::core::ffi::c_int as uint8_t,
                    0x1d as ::core::ffi::c_int as uint8_t,
                    0x12 as ::core::ffi::c_int as uint8_t,
                    0x8a as ::core::ffi::c_int as uint8_t,
                    0xaf as ::core::ffi::c_int as uint8_t,
                    0xd6 as ::core::ffi::c_int as uint8_t,
                    0x8a as ::core::ffi::c_int as uint8_t,
                    0x2c as ::core::ffi::c_int as uint8_t,
                    0xe7 as ::core::ffi::c_int as uint8_t,
                    0x2e as ::core::ffi::c_int as uint8_t,
                    0xa0 as ::core::ffi::c_int as uint8_t,
                ],
                len: 65 as size_t,
            },
            client_pubkey: C2Rust_Unnamed_1 {
                bytes: [
                    0x4 as ::core::ffi::c_int as uint8_t,
                    0xa9 as ::core::ffi::c_int as uint8_t,
                    0x27 as ::core::ffi::c_int as uint8_t,
                    0x19 as ::core::ffi::c_int as uint8_t,
                    0xc6 as ::core::ffi::c_int as uint8_t,
                    0x19 as ::core::ffi::c_int as uint8_t,
                    0x5d as ::core::ffi::c_int as uint8_t,
                    0x50 as ::core::ffi::c_int as uint8_t,
                    0x85 as ::core::ffi::c_int as uint8_t,
                    0x10 as ::core::ffi::c_int as uint8_t,
                    0x4f as ::core::ffi::c_int as uint8_t,
                    0x46 as ::core::ffi::c_int as uint8_t,
                    0x9a as ::core::ffi::c_int as uint8_t,
                    0x8b as ::core::ffi::c_int as uint8_t,
                    0x98 as ::core::ffi::c_int as uint8_t,
                    0x14 as ::core::ffi::c_int as uint8_t,
                    0xd5 as ::core::ffi::c_int as uint8_t,
                    0x83 as ::core::ffi::c_int as uint8_t,
                    0x8f as ::core::ffi::c_int as uint8_t,
                    0xf7 as ::core::ffi::c_int as uint8_t,
                    0x2b as ::core::ffi::c_int as uint8_t,
                    0x60 as ::core::ffi::c_int as uint8_t,
                    0x50 as ::core::ffi::c_int as uint8_t,
                    0x1e as ::core::ffi::c_int as uint8_t,
                    0x2c as ::core::ffi::c_int as uint8_t,
                    0x44 as ::core::ffi::c_int as uint8_t,
                    0x66 as ::core::ffi::c_int as uint8_t,
                    0xe5 as ::core::ffi::c_int as uint8_t,
                    0xe6 as ::core::ffi::c_int as uint8_t,
                    0x7b as ::core::ffi::c_int as uint8_t,
                    0x32 as ::core::ffi::c_int as uint8_t,
                    0x5a as ::core::ffi::c_int as uint8_t,
                    0xc9 as ::core::ffi::c_int as uint8_t,
                    0x85 as ::core::ffi::c_int as uint8_t,
                    0x36 as ::core::ffi::c_int as uint8_t,
                    0xd7 as ::core::ffi::c_int as uint8_t,
                    0xb6 as ::core::ffi::c_int as uint8_t,
                    0x1a as ::core::ffi::c_int as uint8_t,
                    0x1a as ::core::ffi::c_int as uint8_t,
                    0xf4 as ::core::ffi::c_int as uint8_t,
                    0xb7 as ::core::ffi::c_int as uint8_t,
                    0x8e as ::core::ffi::c_int as uint8_t,
                    0x5b as ::core::ffi::c_int as uint8_t,
                    0x7f as ::core::ffi::c_int as uint8_t,
                    0x95 as ::core::ffi::c_int as uint8_t,
                    0x1c as ::core::ffi::c_int as uint8_t,
                    0x9 as ::core::ffi::c_int as uint8_t,
                    0 as ::core::ffi::c_int as uint8_t,
                    0xbe as ::core::ffi::c_int as uint8_t,
                    0x86 as ::core::ffi::c_int as uint8_t,
                    0x3c as ::core::ffi::c_int as uint8_t,
                    0x40 as ::core::ffi::c_int as uint8_t,
                    0x3c as ::core::ffi::c_int as uint8_t,
                    0xe6 as ::core::ffi::c_int as uint8_t,
                    0x5c as ::core::ffi::c_int as uint8_t,
                    0x9b as ::core::ffi::c_int as uint8_t,
                    0xfc as ::core::ffi::c_int as uint8_t,
                    0xb9 as ::core::ffi::c_int as uint8_t,
                    0x38 as ::core::ffi::c_int as uint8_t,
                    0x26 as ::core::ffi::c_int as uint8_t,
                    0x57 as ::core::ffi::c_int as uint8_t,
                    0x22 as ::core::ffi::c_int as uint8_t,
                    0x2d as ::core::ffi::c_int as uint8_t,
                    0x18 as ::core::ffi::c_int as uint8_t,
                    0xc4 as ::core::ffi::c_int as uint8_t,
                ],
                len: 65 as size_t,
            },
            dh: C2Rust_Unnamed_1 {
                bytes: [
                    0x13 as ::core::ffi::c_int as uint8_t,
                    0xf9 as ::core::ffi::c_int as uint8_t,
                    0x18 as ::core::ffi::c_int as uint8_t,
                    0x52 as ::core::ffi::c_int as uint8_t,
                    0x94 as ::core::ffi::c_int as uint8_t,
                    0x58 as ::core::ffi::c_int as uint8_t,
                    0xd2 as ::core::ffi::c_int as uint8_t,
                    0x54 as ::core::ffi::c_int as uint8_t,
                    0x25 as ::core::ffi::c_int as uint8_t,
                    0x31 as ::core::ffi::c_int as uint8_t,
                    0x40 as ::core::ffi::c_int as uint8_t,
                    0x68 as ::core::ffi::c_int as uint8_t,
                    0x88 as ::core::ffi::c_int as uint8_t,
                    0xc8 as ::core::ffi::c_int as uint8_t,
                    0xa6 as ::core::ffi::c_int as uint8_t,
                    0xd4 as ::core::ffi::c_int as uint8_t,
                    0xea as ::core::ffi::c_int as uint8_t,
                    0x7f as ::core::ffi::c_int as uint8_t,
                    0xf4 as ::core::ffi::c_int as uint8_t,
                    0x73 as ::core::ffi::c_int as uint8_t,
                    0xa6 as ::core::ffi::c_int as uint8_t,
                    0xf4 as ::core::ffi::c_int as uint8_t,
                    0xdb as ::core::ffi::c_int as uint8_t,
                    0x45 as ::core::ffi::c_int as uint8_t,
                    0x2a as ::core::ffi::c_int as uint8_t,
                    0xc3 as ::core::ffi::c_int as uint8_t,
                    0xc4 as ::core::ffi::c_int as uint8_t,
                    0xae as ::core::ffi::c_int as uint8_t,
                    0x1d as ::core::ffi::c_int as uint8_t,
                    0x1 as ::core::ffi::c_int as uint8_t,
                    0xce as ::core::ffi::c_int as uint8_t,
                    0xa1 as ::core::ffi::c_int as uint8_t,
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
                len: 32 as size_t,
            },
            expected_secret: [
                0xc0 as ::core::ffi::c_int as uint8_t,
                0xd2 as ::core::ffi::c_int as uint8_t,
                0x6a as ::core::ffi::c_int as uint8_t,
                0xea as ::core::ffi::c_int as uint8_t,
                0xb5 as ::core::ffi::c_int as uint8_t,
                0x36 as ::core::ffi::c_int as uint8_t,
                0x60 as ::core::ffi::c_int as uint8_t,
                0x9a as ::core::ffi::c_int as uint8_t,
                0x57 as ::core::ffi::c_int as uint8_t,
                0x2b as ::core::ffi::c_int as uint8_t,
                0x7 as ::core::ffi::c_int as uint8_t,
                0x69 as ::core::ffi::c_int as uint8_t,
                0x5d as ::core::ffi::c_int as uint8_t,
                0x93 as ::core::ffi::c_int as uint8_t,
                0x3b as ::core::ffi::c_int as uint8_t,
                0x58 as ::core::ffi::c_int as uint8_t,
                0x9d as ::core::ffi::c_int as uint8_t,
                0xcf as ::core::ffi::c_int as uint8_t,
                0x36 as ::core::ffi::c_int as uint8_t,
                0x3f as ::core::ffi::c_int as uint8_t,
                0xf9 as ::core::ffi::c_int as uint8_t,
                0xd9 as ::core::ffi::c_int as uint8_t,
                0x3c as ::core::ffi::c_int as uint8_t,
                0x93 as ::core::ffi::c_int as uint8_t,
                0xad as ::core::ffi::c_int as uint8_t,
                0xea as ::core::ffi::c_int as uint8_t,
                0x53 as ::core::ffi::c_int as uint8_t,
                0x7a as ::core::ffi::c_int as uint8_t,
                0xea as ::core::ffi::c_int as uint8_t,
                0xbb as ::core::ffi::c_int as uint8_t,
                0x8c as ::core::ffi::c_int as uint8_t,
                0xb8 as ::core::ffi::c_int as uint8_t,
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
            expected_ciphertext: [
                [
                    0x5a as ::core::ffi::c_int as uint8_t,
                    0xd5 as ::core::ffi::c_int as uint8_t,
                    0x90 as ::core::ffi::c_int as uint8_t,
                    0xbb as ::core::ffi::c_int as uint8_t,
                    0x8b as ::core::ffi::c_int as uint8_t,
                    0xaa as ::core::ffi::c_int as uint8_t,
                    0x57 as ::core::ffi::c_int as uint8_t,
                    0x7f as ::core::ffi::c_int as uint8_t,
                    0x86 as ::core::ffi::c_int as uint8_t,
                    0x19 as ::core::ffi::c_int as uint8_t,
                    0xdb as ::core::ffi::c_int as uint8_t,
                    0x35 as ::core::ffi::c_int as uint8_t,
                    0xa3 as ::core::ffi::c_int as uint8_t,
                    0x63 as ::core::ffi::c_int as uint8_t,
                    0x11 as ::core::ffi::c_int as uint8_t,
                    0x22 as ::core::ffi::c_int as uint8_t,
                    0x6a as ::core::ffi::c_int as uint8_t,
                    0x89 as ::core::ffi::c_int as uint8_t,
                    0x6e as ::core::ffi::c_int as uint8_t,
                    0x73 as ::core::ffi::c_int as uint8_t,
                    0x42 as ::core::ffi::c_int as uint8_t,
                    0xa6 as ::core::ffi::c_int as uint8_t,
                    0xd8 as ::core::ffi::c_int as uint8_t,
                    0x36 as ::core::ffi::c_int as uint8_t,
                    0xd8 as ::core::ffi::c_int as uint8_t,
                    0xb7 as ::core::ffi::c_int as uint8_t,
                    0xbc as ::core::ffi::c_int as uint8_t,
                    0xd2 as ::core::ffi::c_int as uint8_t,
                    0xf2 as ::core::ffi::c_int as uint8_t,
                    0xb as ::core::ffi::c_int as uint8_t,
                    0x6c as ::core::ffi::c_int as uint8_t,
                    0x7f as ::core::ffi::c_int as uint8_t,
                    0x90 as ::core::ffi::c_int as uint8_t,
                    0x76 as ::core::ffi::c_int as uint8_t,
                    0xac as ::core::ffi::c_int as uint8_t,
                    0x23 as ::core::ffi::c_int as uint8_t,
                    0x2e as ::core::ffi::c_int as uint8_t,
                    0x3a as ::core::ffi::c_int as uint8_t,
                    0xb2 as ::core::ffi::c_int as uint8_t,
                    0x52 as ::core::ffi::c_int as uint8_t,
                    0x3f as ::core::ffi::c_int as uint8_t,
                    0x39 as ::core::ffi::c_int as uint8_t,
                    0x51 as ::core::ffi::c_int as uint8_t,
                    0x34 as ::core::ffi::c_int as uint8_t,
                    0x34 as ::core::ffi::c_int as uint8_t,
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
                [
                    0xfa as ::core::ffi::c_int as uint8_t,
                    0x6f as ::core::ffi::c_int as uint8_t,
                    0x3 as ::core::ffi::c_int as uint8_t,
                    0x7b as ::core::ffi::c_int as uint8_t,
                    0x47 as ::core::ffi::c_int as uint8_t,
                    0xfc as ::core::ffi::c_int as uint8_t,
                    0x21 as ::core::ffi::c_int as uint8_t,
                    0x82 as ::core::ffi::c_int as uint8_t,
                    0x6b as ::core::ffi::c_int as uint8_t,
                    0x61 as ::core::ffi::c_int as uint8_t,
                    0x1 as ::core::ffi::c_int as uint8_t,
                    0x72 as ::core::ffi::c_int as uint8_t,
                    0xca as ::core::ffi::c_int as uint8_t,
                    0x96 as ::core::ffi::c_int as uint8_t,
                    0x37 as ::core::ffi::c_int as uint8_t,
                    0xe8 as ::core::ffi::c_int as uint8_t,
                    0x2d as ::core::ffi::c_int as uint8_t,
                    0x6e as ::core::ffi::c_int as uint8_t,
                    0x58 as ::core::ffi::c_int as uint8_t,
                    0x1 as ::core::ffi::c_int as uint8_t,
                    0xeb as ::core::ffi::c_int as uint8_t,
                    0x31 as ::core::ffi::c_int as uint8_t,
                    0xcb as ::core::ffi::c_int as uint8_t,
                    0xd3 as ::core::ffi::c_int as uint8_t,
                    0x74 as ::core::ffi::c_int as uint8_t,
                    0x82 as ::core::ffi::c_int as uint8_t,
                    0x71 as ::core::ffi::c_int as uint8_t,
                    0xaf as ::core::ffi::c_int as uint8_t,
                    0xfd as ::core::ffi::c_int as uint8_t,
                    0x4e as ::core::ffi::c_int as uint8_t,
                    0xcb as ::core::ffi::c_int as uint8_t,
                    0x6 as ::core::ffi::c_int as uint8_t,
                    0x64 as ::core::ffi::c_int as uint8_t,
                    0x6e as ::core::ffi::c_int as uint8_t,
                    0x3 as ::core::ffi::c_int as uint8_t,
                    0x29 as ::core::ffi::c_int as uint8_t,
                    0xcb as ::core::ffi::c_int as uint8_t,
                    0xdf as ::core::ffi::c_int as uint8_t,
                    0x3c as ::core::ffi::c_int as uint8_t,
                    0x3c as ::core::ffi::c_int as uint8_t,
                    0xd6 as ::core::ffi::c_int as uint8_t,
                    0x55 as ::core::ffi::c_int as uint8_t,
                    0xb2 as ::core::ffi::c_int as uint8_t,
                    0x8e as ::core::ffi::c_int as uint8_t,
                    0x82 as ::core::ffi::c_int as uint8_t,
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
            ],
        },
        C2Rust_Unnamed_0 {
            id: C2Rust_Unnamed_2 {
                kem: PTLS_HPKE_KEM_P256_SHA256 as uint16_t,
                kdf: PTLS_HPKE_HKDF_SHA512 as uint16_t,
                aead: PTLS_HPKE_AEAD_AES_128_GCM as uint16_t,
            },
            server_pubkey: C2Rust_Unnamed_1 {
                bytes: [
                    0x4 as ::core::ffi::c_int as uint8_t,
                    0x8 as ::core::ffi::c_int as uint8_t,
                    0x5a as ::core::ffi::c_int as uint8_t,
                    0xa5 as ::core::ffi::c_int as uint8_t,
                    0xb6 as ::core::ffi::c_int as uint8_t,
                    0x65 as ::core::ffi::c_int as uint8_t,
                    0xdc as ::core::ffi::c_int as uint8_t,
                    0x38 as ::core::ffi::c_int as uint8_t,
                    0x26 as ::core::ffi::c_int as uint8_t,
                    0xf9 as ::core::ffi::c_int as uint8_t,
                    0x65 as ::core::ffi::c_int as uint8_t,
                    0xc as ::core::ffi::c_int as uint8_t,
                    0xcb as ::core::ffi::c_int as uint8_t,
                    0xcc as ::core::ffi::c_int as uint8_t,
                    0x47 as ::core::ffi::c_int as uint8_t,
                    0x1b as ::core::ffi::c_int as uint8_t,
                    0xe2 as ::core::ffi::c_int as uint8_t,
                    0x68 as ::core::ffi::c_int as uint8_t,
                    0xc8 as ::core::ffi::c_int as uint8_t,
                    0xad as ::core::ffi::c_int as uint8_t,
                    0xa8 as ::core::ffi::c_int as uint8_t,
                    0x66 as ::core::ffi::c_int as uint8_t,
                    0x42 as ::core::ffi::c_int as uint8_t,
                    0x2f as ::core::ffi::c_int as uint8_t,
                    0x73 as ::core::ffi::c_int as uint8_t,
                    0x9e as ::core::ffi::c_int as uint8_t,
                    0x2d as ::core::ffi::c_int as uint8_t,
                    0x53 as ::core::ffi::c_int as uint8_t,
                    0x1d as ::core::ffi::c_int as uint8_t,
                    0x4a as ::core::ffi::c_int as uint8_t,
                    0x88 as ::core::ffi::c_int as uint8_t,
                    0x18 as ::core::ffi::c_int as uint8_t,
                    0xa9 as ::core::ffi::c_int as uint8_t,
                    0x46 as ::core::ffi::c_int as uint8_t,
                    0x6b as ::core::ffi::c_int as uint8_t,
                    0xc6 as ::core::ffi::c_int as uint8_t,
                    0xb4 as ::core::ffi::c_int as uint8_t,
                    0x49 as ::core::ffi::c_int as uint8_t,
                    0x35 as ::core::ffi::c_int as uint8_t,
                    0x70 as ::core::ffi::c_int as uint8_t,
                    0x96 as ::core::ffi::c_int as uint8_t,
                    0x23 as ::core::ffi::c_int as uint8_t,
                    0x29 as ::core::ffi::c_int as uint8_t,
                    0x19 as ::core::ffi::c_int as uint8_t,
                    0xec as ::core::ffi::c_int as uint8_t,
                    0x4f as ::core::ffi::c_int as uint8_t,
                    0xe9 as ::core::ffi::c_int as uint8_t,
                    0x7 as ::core::ffi::c_int as uint8_t,
                    0xc as ::core::ffi::c_int as uint8_t,
                    0xcb as ::core::ffi::c_int as uint8_t,
                    0xac as ::core::ffi::c_int as uint8_t,
                    0x4a as ::core::ffi::c_int as uint8_t,
                    0xac as ::core::ffi::c_int as uint8_t,
                    0x30 as ::core::ffi::c_int as uint8_t,
                    0xf4 as ::core::ffi::c_int as uint8_t,
                    0xa1 as ::core::ffi::c_int as uint8_t,
                    0xa5 as ::core::ffi::c_int as uint8_t,
                    0x3e as ::core::ffi::c_int as uint8_t,
                    0xfc as ::core::ffi::c_int as uint8_t,
                    0xf7 as ::core::ffi::c_int as uint8_t,
                    0xaf as ::core::ffi::c_int as uint8_t,
                    0x90 as ::core::ffi::c_int as uint8_t,
                    0x61 as ::core::ffi::c_int as uint8_t,
                    0xe as ::core::ffi::c_int as uint8_t,
                    0xdd as ::core::ffi::c_int as uint8_t,
                ],
                len: 65 as size_t,
            },
            client_pubkey: C2Rust_Unnamed_1 {
                bytes: [
                    0x4 as ::core::ffi::c_int as uint8_t,
                    0x93 as ::core::ffi::c_int as uint8_t,
                    0xed as ::core::ffi::c_int as uint8_t,
                    0x86 as ::core::ffi::c_int as uint8_t,
                    0x73 as ::core::ffi::c_int as uint8_t,
                    0x5b as ::core::ffi::c_int as uint8_t,
                    0xdf as ::core::ffi::c_int as uint8_t,
                    0xb9 as ::core::ffi::c_int as uint8_t,
                    0x78 as ::core::ffi::c_int as uint8_t,
                    0xcc as ::core::ffi::c_int as uint8_t,
                    0x5 as ::core::ffi::c_int as uint8_t,
                    0x5c as ::core::ffi::c_int as uint8_t,
                    0x98 as ::core::ffi::c_int as uint8_t,
                    0xb4 as ::core::ffi::c_int as uint8_t,
                    0x56 as ::core::ffi::c_int as uint8_t,
                    0x95 as ::core::ffi::c_int as uint8_t,
                    0xad as ::core::ffi::c_int as uint8_t,
                    0x7c as ::core::ffi::c_int as uint8_t,
                    0xe6 as ::core::ffi::c_int as uint8_t,
                    0x1c as ::core::ffi::c_int as uint8_t,
                    0xe7 as ::core::ffi::c_int as uint8_t,
                    0x48 as ::core::ffi::c_int as uint8_t,
                    0xf4 as ::core::ffi::c_int as uint8_t,
                    0xdd as ::core::ffi::c_int as uint8_t,
                    0x63 as ::core::ffi::c_int as uint8_t,
                    0xc5 as ::core::ffi::c_int as uint8_t,
                    0x25 as ::core::ffi::c_int as uint8_t,
                    0xa3 as ::core::ffi::c_int as uint8_t,
                    0xb8 as ::core::ffi::c_int as uint8_t,
                    0xd5 as ::core::ffi::c_int as uint8_t,
                    0x3a as ::core::ffi::c_int as uint8_t,
                    0x15 as ::core::ffi::c_int as uint8_t,
                    0x56 as ::core::ffi::c_int as uint8_t,
                    0x5c as ::core::ffi::c_int as uint8_t,
                    0x68 as ::core::ffi::c_int as uint8_t,
                    0x97 as ::core::ffi::c_int as uint8_t,
                    0x88 as ::core::ffi::c_int as uint8_t,
                    0x80 as ::core::ffi::c_int as uint8_t,
                    0x70 as ::core::ffi::c_int as uint8_t,
                    0x7 as ::core::ffi::c_int as uint8_t,
                    0xc as ::core::ffi::c_int as uint8_t,
                    0x15 as ::core::ffi::c_int as uint8_t,
                    0x79 as ::core::ffi::c_int as uint8_t,
                    0xdb as ::core::ffi::c_int as uint8_t,
                    0x1f as ::core::ffi::c_int as uint8_t,
                    0x86 as ::core::ffi::c_int as uint8_t,
                    0xaa as ::core::ffi::c_int as uint8_t,
                    0xa5 as ::core::ffi::c_int as uint8_t,
                    0x6d as ::core::ffi::c_int as uint8_t,
                    0xeb as ::core::ffi::c_int as uint8_t,
                    0x82 as ::core::ffi::c_int as uint8_t,
                    0x97 as ::core::ffi::c_int as uint8_t,
                    0xe6 as ::core::ffi::c_int as uint8_t,
                    0x4d as ::core::ffi::c_int as uint8_t,
                    0xb7 as ::core::ffi::c_int as uint8_t,
                    0xe8 as ::core::ffi::c_int as uint8_t,
                    0x92 as ::core::ffi::c_int as uint8_t,
                    0x4e as ::core::ffi::c_int as uint8_t,
                    0x72 as ::core::ffi::c_int as uint8_t,
                    0x86 as ::core::ffi::c_int as uint8_t,
                    0x6f as ::core::ffi::c_int as uint8_t,
                    0x9a as ::core::ffi::c_int as uint8_t,
                    0x47 as ::core::ffi::c_int as uint8_t,
                    0x25 as ::core::ffi::c_int as uint8_t,
                    0x80 as ::core::ffi::c_int as uint8_t,
                ],
                len: 65 as size_t,
            },
            dh: C2Rust_Unnamed_1 {
                bytes: [
                    0 as ::core::ffi::c_int as uint8_t,
                    0x63 as ::core::ffi::c_int as uint8_t,
                    0x70 as ::core::ffi::c_int as uint8_t,
                    0x63 as ::core::ffi::c_int as uint8_t,
                    0x7d as ::core::ffi::c_int as uint8_t,
                    0xb3 as ::core::ffi::c_int as uint8_t,
                    0x7e as ::core::ffi::c_int as uint8_t,
                    0xf6 as ::core::ffi::c_int as uint8_t,
                    0x8f as ::core::ffi::c_int as uint8_t,
                    0x3a as ::core::ffi::c_int as uint8_t,
                    0x55 as ::core::ffi::c_int as uint8_t,
                    0xb as ::core::ffi::c_int as uint8_t,
                    0x9a as ::core::ffi::c_int as uint8_t,
                    0xba as ::core::ffi::c_int as uint8_t,
                    0xb6 as ::core::ffi::c_int as uint8_t,
                    0xa4 as ::core::ffi::c_int as uint8_t,
                    0xb9 as ::core::ffi::c_int as uint8_t,
                    0xa3 as ::core::ffi::c_int as uint8_t,
                    0x4a as ::core::ffi::c_int as uint8_t,
                    0x16 as ::core::ffi::c_int as uint8_t,
                    0x8f as ::core::ffi::c_int as uint8_t,
                    0x34 as ::core::ffi::c_int as uint8_t,
                    0x29 as ::core::ffi::c_int as uint8_t,
                    0x26 as ::core::ffi::c_int as uint8_t,
                    0xda as ::core::ffi::c_int as uint8_t,
                    0x14 as ::core::ffi::c_int as uint8_t,
                    0x25 as ::core::ffi::c_int as uint8_t,
                    0xa1 as ::core::ffi::c_int as uint8_t,
                    0x68 as ::core::ffi::c_int as uint8_t,
                    0x49 as ::core::ffi::c_int as uint8_t,
                    0xa0 as ::core::ffi::c_int as uint8_t,
                    0x95 as ::core::ffi::c_int as uint8_t,
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
                len: 32 as size_t,
            },
            expected_secret: [
                0x2 as ::core::ffi::c_int as uint8_t,
                0xf5 as ::core::ffi::c_int as uint8_t,
                0x84 as ::core::ffi::c_int as uint8_t,
                0x73 as ::core::ffi::c_int as uint8_t,
                0x63 as ::core::ffi::c_int as uint8_t,
                0x90 as ::core::ffi::c_int as uint8_t,
                0xfc as ::core::ffi::c_int as uint8_t,
                0x93 as ::core::ffi::c_int as uint8_t,
                0xf5 as ::core::ffi::c_int as uint8_t,
                0xb4 as ::core::ffi::c_int as uint8_t,
                0xad as ::core::ffi::c_int as uint8_t,
                0x3 as ::core::ffi::c_int as uint8_t,
                0x98 as ::core::ffi::c_int as uint8_t,
                0x26 as ::core::ffi::c_int as uint8_t,
                0xa3 as ::core::ffi::c_int as uint8_t,
                0xfa as ::core::ffi::c_int as uint8_t,
                0x8 as ::core::ffi::c_int as uint8_t,
                0xe9 as ::core::ffi::c_int as uint8_t,
                0x91 as ::core::ffi::c_int as uint8_t,
                0x1b as ::core::ffi::c_int as uint8_t,
                0xd1 as ::core::ffi::c_int as uint8_t,
                0x21 as ::core::ffi::c_int as uint8_t,
                0x5a as ::core::ffi::c_int as uint8_t,
                0x3d as ::core::ffi::c_int as uint8_t,
                0xb8 as ::core::ffi::c_int as uint8_t,
                0xe8 as ::core::ffi::c_int as uint8_t,
                0x79 as ::core::ffi::c_int as uint8_t,
                0x1b as ::core::ffi::c_int as uint8_t,
                0xa5 as ::core::ffi::c_int as uint8_t,
                0x33 as ::core::ffi::c_int as uint8_t,
                0xca as ::core::ffi::c_int as uint8_t,
                0xfd as ::core::ffi::c_int as uint8_t,
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
            expected_ciphertext: [
                [
                    0xd3 as ::core::ffi::c_int as uint8_t,
                    0xcf as ::core::ffi::c_int as uint8_t,
                    0x49 as ::core::ffi::c_int as uint8_t,
                    0x84 as ::core::ffi::c_int as uint8_t,
                    0x93 as ::core::ffi::c_int as uint8_t,
                    0x14 as ::core::ffi::c_int as uint8_t,
                    0x84 as ::core::ffi::c_int as uint8_t,
                    0xa0 as ::core::ffi::c_int as uint8_t,
                    0x80 as ::core::ffi::c_int as uint8_t,
                    0xf7 as ::core::ffi::c_int as uint8_t,
                    0x4c as ::core::ffi::c_int as uint8_t,
                    0x1b as ::core::ffi::c_int as uint8_t,
                    0xb2 as ::core::ffi::c_int as uint8_t,
                    0xa6 as ::core::ffi::c_int as uint8_t,
                    0x78 as ::core::ffi::c_int as uint8_t,
                    0x27 as ::core::ffi::c_int as uint8_t,
                    0 as ::core::ffi::c_int as uint8_t,
                    0xdc as ::core::ffi::c_int as uint8_t,
                    0x1f as ::core::ffi::c_int as uint8_t,
                    0xef as ::core::ffi::c_int as uint8_t,
                    0x9a as ::core::ffi::c_int as uint8_t,
                    0xbe as ::core::ffi::c_int as uint8_t,
                    0x84 as ::core::ffi::c_int as uint8_t,
                    0x42 as ::core::ffi::c_int as uint8_t,
                    0xe4 as ::core::ffi::c_int as uint8_t,
                    0x4a as ::core::ffi::c_int as uint8_t,
                    0x6f as ::core::ffi::c_int as uint8_t,
                    0x9 as ::core::ffi::c_int as uint8_t,
                    0x4 as ::core::ffi::c_int as uint8_t,
                    0x4c as ::core::ffi::c_int as uint8_t,
                    0x88 as ::core::ffi::c_int as uint8_t,
                    0x90 as ::core::ffi::c_int as uint8_t,
                    0x72 as ::core::ffi::c_int as uint8_t,
                    0 as ::core::ffi::c_int as uint8_t,
                    0xb3 as ::core::ffi::c_int as uint8_t,
                    0x32 as ::core::ffi::c_int as uint8_t,
                    0 as ::core::ffi::c_int as uint8_t,
                    0x35 as ::core::ffi::c_int as uint8_t,
                    0x43 as ::core::ffi::c_int as uint8_t,
                    0x75 as ::core::ffi::c_int as uint8_t,
                    0x4e as ::core::ffi::c_int as uint8_t,
                    0xb5 as ::core::ffi::c_int as uint8_t,
                    0x19 as ::core::ffi::c_int as uint8_t,
                    0x17 as ::core::ffi::c_int as uint8_t,
                    0xba as ::core::ffi::c_int as uint8_t,
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
                [
                    0xd1 as ::core::ffi::c_int as uint8_t,
                    0x44 as ::core::ffi::c_int as uint8_t,
                    0x14 as ::core::ffi::c_int as uint8_t,
                    0x55 as ::core::ffi::c_int as uint8_t,
                    0x5a as ::core::ffi::c_int as uint8_t,
                    0x47 as ::core::ffi::c_int as uint8_t,
                    0x26 as ::core::ffi::c_int as uint8_t,
                    0x9d as ::core::ffi::c_int as uint8_t,
                    0xfe as ::core::ffi::c_int as uint8_t,
                    0xad as ::core::ffi::c_int as uint8_t,
                    0x9f as ::core::ffi::c_int as uint8_t,
                    0xbf as ::core::ffi::c_int as uint8_t,
                    0x26 as ::core::ffi::c_int as uint8_t,
                    0xab as ::core::ffi::c_int as uint8_t,
                    0xb3 as ::core::ffi::c_int as uint8_t,
                    0x3 as ::core::ffi::c_int as uint8_t,
                    0x36 as ::core::ffi::c_int as uint8_t,
                    0x5e as ::core::ffi::c_int as uint8_t,
                    0x40 as ::core::ffi::c_int as uint8_t,
                    0x70 as ::core::ffi::c_int as uint8_t,
                    0x9a as ::core::ffi::c_int as uint8_t,
                    0x4e as ::core::ffi::c_int as uint8_t,
                    0xd1 as ::core::ffi::c_int as uint8_t,
                    0x6e as ::core::ffi::c_int as uint8_t,
                    0xae as ::core::ffi::c_int as uint8_t,
                    0xfe as ::core::ffi::c_int as uint8_t,
                    0x1f as ::core::ffi::c_int as uint8_t,
                    0x20 as ::core::ffi::c_int as uint8_t,
                    0x70 as ::core::ffi::c_int as uint8_t,
                    0xf1 as ::core::ffi::c_int as uint8_t,
                    0xdd as ::core::ffi::c_int as uint8_t,
                    0xeb as ::core::ffi::c_int as uint8_t,
                    0x1b as ::core::ffi::c_int as uint8_t,
                    0xdd as ::core::ffi::c_int as uint8_t,
                    0x94 as ::core::ffi::c_int as uint8_t,
                    0xd9 as ::core::ffi::c_int as uint8_t,
                    0xe4 as ::core::ffi::c_int as uint8_t,
                    0x11 as ::core::ffi::c_int as uint8_t,
                    0x86 as ::core::ffi::c_int as uint8_t,
                    0xf1 as ::core::ffi::c_int as uint8_t,
                    0x24 as ::core::ffi::c_int as uint8_t,
                    0xe0 as ::core::ffi::c_int as uint8_t,
                    0xac as ::core::ffi::c_int as uint8_t,
                    0xc6 as ::core::ffi::c_int as uint8_t,
                    0x2d as ::core::ffi::c_int as uint8_t,
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
            ],
        },
        C2Rust_Unnamed_0 {
            id: C2Rust_Unnamed_2 {
                kem: 0 as uint16_t,
                kdf: 0,
                aead: 0,
            },
            server_pubkey: C2Rust_Unnamed_1 {
                bytes: [0; 65],
                len: 0,
            },
            client_pubkey: C2Rust_Unnamed_1 {
                bytes: [0; 65],
                len: 0,
            },
            dh: C2Rust_Unnamed_1 {
                bytes: [0; 65],
                len: 0,
            },
            expected_secret: [0; 64],
            expected_ciphertext: [[0; 61]; 2],
        },
    ];
    static mut test: *const C2Rust_Unnamed_0 = ::core::ptr::null::<C2Rust_Unnamed_0>();
    let mut ret: ::core::ffi::c_int = 0;
    test = &raw const all as *const C2Rust_Unnamed_0;
    while !((*test).id.kem as ::core::ffi::c_int == (*test_kem).id as ::core::ffi::c_int
        && (*test).id.kdf as ::core::ffi::c_int == (*test_cipher).id.kdf as ::core::ffi::c_int
        && (*test).id.aead as ::core::ffi::c_int == (*test_cipher).id.aead as ::core::ffi::c_int)
    {
        if (*test).id.kem as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
            note(b"no test vector for given kem / cipher\0".as_ptr() as *const ::core::ffi::c_char);
            return;
        }
        test = test.offset(1);
    }
    let mut secret: [uint8_t; 64] = [0; 64];
    ret = dh_derive(
        test_kem,
        &raw mut secret as *mut uint8_t as *mut ::core::ffi::c_void,
        ptls_iovec_init(
            &raw const (*test).client_pubkey.bytes as *const uint8_t as *const ::core::ffi::c_void,
            (*test).client_pubkey.len,
        ),
        ptls_iovec_init(
            &raw const (*test).server_pubkey.bytes as *const uint8_t as *const ::core::ffi::c_void,
            (*test).server_pubkey.len,
        ),
        ptls_iovec_init(
            &raw const (*test).dh.bytes as *const uint8_t as *const ::core::ffi::c_void,
            (*test).dh.len,
        ),
    );
    _ok(
        (ret == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/hpke.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        135 as ::core::ffi::c_int,
    );
    _ok(
        (memcmp(
            &raw mut secret as *mut uint8_t as *const ::core::ffi::c_void,
            &raw const (*test).expected_secret as *const uint8_t
                as *const ::core::ffi::c_void,
            (*(*test_kem).hash).digest_size,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/hpke.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        136 as ::core::ffi::c_int,
    );
    let mut enc: *mut ptls_aead_context_t = ::core::ptr::null_mut::<ptls_aead_context_t>();
    let mut ciphertext: [uint8_t; 61] = [0; 61];
    ret = key_schedule(
        test_kem,
        test_cipher,
        &raw mut enc,
        1 as ::core::ffi::c_int,
        &raw const (*test).expected_secret as *const uint8_t as *const ::core::ffi::c_void,
        ptls_iovec_init(
            &raw const info as *const uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
        ),
    );
    _ok(
        (ret == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/hpke.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        143 as ::core::ffi::c_int,
    );
    let mut seq: uint64_t = 0 as uint64_t;
    while seq < 2 as uint64_t {
        ptls_aead_encrypt(
            enc,
            &raw mut ciphertext as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw const cleartext as *const uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 29]>() as size_t,
            seq,
            &raw const *(&raw const aad as *const [uint8_t; 7]).offset(seq as isize)
                as *const uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 7]>() as size_t,
        );
        _ok(
            (memcmp(
                &raw mut ciphertext as *mut uint8_t as *const ::core::ffi::c_void,
                &raw const *(&raw const (*test).expected_ciphertext
                    as *const [uint8_t; 61])
                    .offset(seq as isize) as *const uint8_t
                    as *const ::core::ffi::c_void,
                (::core::mem::size_of::<[uint8_t; 29]>() as size_t)
                    .wrapping_add((*(*test_cipher).aead).tag_size),
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/hpke.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            146 as ::core::ffi::c_int,
        );
        seq = seq.wrapping_add(1);
    }
    ptls_aead_free(enc);
    let mut dec: *mut ptls_aead_context_t = ::core::ptr::null_mut::<ptls_aead_context_t>();
    let mut text_recovered: [uint8_t; 29] = [0; 29];
    ret = key_schedule(
        test_kem,
        test_cipher,
        &raw mut dec,
        0 as ::core::ffi::c_int,
        &raw const (*test).expected_secret as *const uint8_t as *const ::core::ffi::c_void,
        ptls_iovec_init(
            &raw const info as *const uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
        ),
    );
    _ok(
        (ret == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/hpke.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        155 as ::core::ffi::c_int,
    );
    let mut seq_0: uint64_t = 0 as uint64_t;
    while seq_0 < 2 as uint64_t {
        _ok(
            (ptls_aead_decrypt(
                dec,
                &raw mut text_recovered as *mut uint8_t as *mut ::core::ffi::c_void,
                &raw const *(&raw const (*test).expected_ciphertext
                    as *const [uint8_t; 61])
                    .offset(seq_0 as isize) as *const uint8_t
                    as *const ::core::ffi::c_void,
                (::core::mem::size_of::<[uint8_t; 29]>() as size_t)
                    .wrapping_add((*(*test_cipher).aead).tag_size),
                seq_0,
                &raw const *(&raw const aad as *const [uint8_t; 7])
                    .offset(seq_0 as isize) as *const uint8_t
                    as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[uint8_t; 7]>() as size_t,
            ) == ::core::mem::size_of::<[uint8_t; 29]>() as usize) as ::core::ffi::c_int,
            b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/hpke.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            159 as ::core::ffi::c_int,
        );
        _ok(
            (memcmp(
                &raw mut text_recovered as *mut uint8_t as *const ::core::ffi::c_void,
                &raw const cleartext as *const uint8_t as *const ::core::ffi::c_void,
                ::core::mem::size_of::<[uint8_t; 29]>() as size_t,
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/hpke.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            160 as ::core::ffi::c_int,
        );
        seq_0 = seq_0.wrapping_add(1);
    }
    ptls_aead_free(dec);
}
#[no_mangle]
pub unsafe extern "C" fn test_hpke(
    mut all_kems: *mut *const ptls_hpke_kem_t,
    mut all_ciphers: *mut *const ptls_hpke_cipher_suite_t,
) {
    let mut kem: *mut *const ptls_hpke_kem_t = all_kems;
    while !(*kem).is_null() {
        let mut cipher: *mut *const ptls_hpke_cipher_suite_t = all_ciphers;
        while !(*cipher).is_null() {
            let mut namebuf: [::core::ffi::c_char; 64] = [0; 64];
            snprintf(
                &raw mut namebuf as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
                b"%s-%s/%s-%s\0".as_ptr() as *const ::core::ffi::c_char,
                (*(**kem).keyex).name,
                (*(**kem).hash).name,
                (*(**cipher).hash).name,
                (*(**cipher).aead).name,
            );
            test_kem = *kem;
            test_cipher = *cipher;
            let mut _name: *const ::core::ffi::c_char =
                &raw mut namebuf as *mut ::core::ffi::c_char;
            enter_subtest(_name);
            test_one_hpke();
            exit_subtest(_name);
            cipher = cipher.offset(1);
        }
        kem = kem.offset(1);
    }
}

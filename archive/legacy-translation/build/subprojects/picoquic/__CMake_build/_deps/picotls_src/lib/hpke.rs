use ::c2rust_bitfields;
extern "C" {
    fn free(__ptr: *mut ::core::ffi::c_void);
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
    static mut ptls_clear_memory:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
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
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const NULL_0: *mut ::core::ffi::c_void =
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
            c2rust_current_block = 7489822446874025458;
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
                c2rust_current_block = 7489822446874025458;
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
            c2rust_current_block = 7489822446874025458;
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
                c2rust_current_block = 7489822446874025458;
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
                    c2rust_current_block = 7489822446874025458;
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
                        c2rust_current_block = 7489822446874025458;
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

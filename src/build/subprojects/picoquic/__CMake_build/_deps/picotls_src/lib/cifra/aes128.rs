use ::c2rust_bitfields;
extern "C" {
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
    fn cf_aes_init(ctx: *mut cf_aes_context, key: *const uint8_t, nkey: size_t);
    fn cf_aes_encrypt(ctx: *const cf_aes_context, in_0: *const uint8_t, out: *mut uint8_t);
    fn cf_aes_decrypt(ctx: *const cf_aes_context, in_0: *const uint8_t, out: *mut uint8_t);
    static cf_aes: cf_prp;
    fn cf_ctr_init(
        ctx: *mut cf_ctr,
        prp: *const cf_prp,
        prpctx: *mut ::core::ffi::c_void,
        nonce: *const uint8_t,
    );
    fn cf_ctr_cipher(ctx: *mut cf_ctr, input: *const uint8_t, output: *mut uint8_t, bytes: size_t);
    fn cf_gcm_encrypt_init(
        prp: *const cf_prp,
        prpctx: *mut ::core::ffi::c_void,
        gcmctx: *mut cf_gcm_ctx,
        header: *const uint8_t,
        nheader: size_t,
        nonce: *const uint8_t,
        nnonce: size_t,
    );
    fn cf_gcm_encrypt_update(
        gcmctx: *mut cf_gcm_ctx,
        plain: *const uint8_t,
        nplain: size_t,
        cipher: *mut uint8_t,
    );
    fn cf_gcm_encrypt_final(gcmctx: *mut cf_gcm_ctx, tag: *mut uint8_t, ntag: size_t);
    fn cf_gcm_decrypt(
        prp: *const cf_prp,
        prpctx: *mut ::core::ffi::c_void,
        cipher: *const uint8_t,
        ncipher: size_t,
        header: *const uint8_t,
        nheader: size_t,
        nonce: *const uint8_t,
        nnonce: size_t,
        tag: *const uint8_t,
        ntag: size_t,
        plain: *mut uint8_t,
    ) -> ::core::ffi::c_int;
    fn cf_sha256_init(ctx: *mut cf_sha256_context);
    fn cf_sha256_update(
        ctx: *mut cf_sha256_context,
        data: *const ::core::ffi::c_void,
        nbytes: size_t,
    );
    fn cf_sha256_digest_final(ctx: *mut cf_sha256_context, hash: *mut uint8_t);
    fn ptls_aead__build_iv(
        algo: *const ptls_aead_algorithm_t,
        iv: *mut uint8_t,
        static_iv: *const uint8_t,
        seq: uint64_t,
    );
    static mut ptls_clear_memory:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type cf_prp_block =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const uint8_t, *mut uint8_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_prp {
    pub blocksz: size_t,
    pub encrypt: cf_prp_block,
    pub decrypt: cf_prp_block,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_aes_context {
    pub rounds: uint32_t,
    pub ks: [uint32_t; 60],
}
pub type cf_gf128 = [uint32_t; 4];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_ctr {
    pub prp: *const cf_prp,
    pub prpctx: *mut ::core::ffi::c_void,
    pub nonce: [uint8_t; 16],
    pub keymat: [uint8_t; 16],
    pub nkeymat: size_t,
    pub counter_offset: size_t,
    pub counter_width: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ghash_ctx {
    pub H: cf_gf128,
    pub Y: cf_gf128,
    pub buffer: [uint8_t; 16],
    pub buffer_used: size_t,
    pub len_aad: uint64_t,
    pub len_cipher: uint64_t,
    pub state: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_gcm_ctx {
    pub ctr: cf_ctr,
    pub gh: ghash_ctx,
    pub Y0: [uint8_t; 16],
    pub e_Y0: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_sha256_context {
    pub H: [uint32_t; 8],
    pub partial: [uint8_t; 64],
    pub blocks: uint32_t,
    pub npartial: size_t,
}
pub type ptls_iovec_t = st_ptls_iovec_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_iovec_t {
    pub base: *mut uint8_t,
    pub len: size_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aesecb_context_t {
    pub super_0: ptls_cipher_context_t,
    pub aes: cf_aes_context,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aesctr_context_t {
    pub super_0: ptls_cipher_context_t,
    pub aes: cf_aes_context,
    pub ctr: cf_ctr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aesgcm_context_t {
    pub super_0: ptls_aead_context_t,
    pub aes: cf_aes_context,
    pub gcm: cf_gcm_ctx,
    pub static_iv: [uint8_t; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha256_context_t {
    pub super_0: ptls_hash_context_t,
    pub ctx: cf_sha256_context,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const PTLS_AES128_KEY_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_AES_BLOCK_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_AES_IV_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_AESGCM_IV_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const PTLS_AESGCM_TAG_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_AESGCM_CONFIDENTIALITY_LIMIT: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
pub const PTLS_SHA256_BLOCK_SIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const PTLS_SHA256_DIGEST_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const PTLS_CIPHER_SUITE_AES_128_GCM_SHA256: ::core::ffi::c_int = 0x1301 as ::core::ffi::c_int;
pub const PTLS_CIPHER_SUITE_NAME_AES_128_GCM_SHA256: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"TLS_AES_128_GCM_SHA256\0")
};
pub const PTLS_TLS12_AESGCM_FIXED_IV_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PTLS_TLS12_AESGCM_RECORD_IV_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
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
unsafe extern "C" fn ptls_aead__do_encrypt(
    mut ctx: *mut ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
    mut supp: *mut ptls_aead_supplementary_encryption_t,
) {
    let mut invec: ptls_iovec_t = ptls_iovec_init(input, inlen);
    (*ctx).do_encrypt_v.expect("non-null function pointer")(
        ctx as *mut st_ptls_aead_context_t,
        output,
        &raw mut invec,
        1 as size_t,
        seq,
        aad,
        aadlen,
    );
    if !supp.is_null() {
        ptls_cipher_init((*supp).ctx, (*supp).input);
        memset(
            &raw mut (*supp).output as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
        );
        ptls_cipher_encrypt(
            (*supp).ctx,
            &raw mut (*supp).output as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw mut (*supp).output as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
        );
    }
}
#[inline]
unsafe extern "C" fn ptls_aead__do_encrypt_v(
    mut ctx: *mut ptls_aead_context_t,
    mut _output: *mut ::core::ffi::c_void,
    mut input: *mut ptls_iovec_t,
    mut incnt: size_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) {
    let mut output: *mut uint8_t = _output as *mut uint8_t;
    (*ctx).do_encrypt_init.expect("non-null function pointer")(
        ctx as *mut st_ptls_aead_context_t,
        seq,
        aad,
        aadlen,
    );
    let mut i: size_t = 0 as size_t;
    while i < incnt {
        output = output.offset(
            (*ctx).do_encrypt_update.expect("non-null function pointer")(
                ctx as *mut st_ptls_aead_context_t,
                output as *mut ::core::ffi::c_void,
                (*input.offset(i as isize)).base as *const ::core::ffi::c_void,
                (*input.offset(i as isize)).len,
            ) as isize,
        );
        i = i.wrapping_add(1);
    }
    (*ctx).do_encrypt_final.expect("non-null function pointer")(
        ctx as *mut st_ptls_aead_context_t,
        output as *mut ::core::ffi::c_void,
    );
}
#[inline]
unsafe extern "C" fn ptls_hash_clone_memcpy(
    mut dst: *mut ::core::ffi::c_void,
    mut src: *const ::core::ffi::c_void,
    mut size: size_t,
) {
    memcpy(dst, src, size);
}
#[inline]
unsafe extern "C" fn aesecb_dispose(mut _ctx: *mut ptls_cipher_context_t) {
    let mut ctx: *mut aesecb_context_t = _ctx as *mut aesecb_context_t;
    ptls_clear_memory.expect("non-null function pointer")(
        ctx as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<aesecb_context_t>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn aesecb_encrypt(
    mut _ctx: *mut ptls_cipher_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut ctx: *mut aesecb_context_t = _ctx as *mut aesecb_context_t;
    cf_aes_encrypt(
        &raw mut (*ctx).aes,
        input as *const uint8_t,
        output as *mut uint8_t,
    );
}
#[inline]
unsafe extern "C" fn aesecb_decrypt(
    mut _ctx: *mut ptls_cipher_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut ctx: *mut aesecb_context_t = _ctx as *mut aesecb_context_t;
    cf_aes_decrypt(
        &raw mut (*ctx).aes,
        input as *const uint8_t,
        output as *mut uint8_t,
    );
}
#[inline]
unsafe extern "C" fn aesecb_setup_crypto(
    mut _ctx: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut aesecb_context_t = _ctx as *mut aesecb_context_t;
    (*ctx).super_0.do_dispose =
        Some(aesecb_dispose as unsafe extern "C" fn(*mut ptls_cipher_context_t) -> ())
            as Option<unsafe extern "C" fn(*mut st_ptls_cipher_context_t) -> ()>;
    (*ctx).super_0.do_init = None;
    (*ctx).super_0.do_transform = (if is_enc != 0 {
        Some(
            aesecb_encrypt
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        )
    } else {
        Some(
            aesecb_decrypt
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        )
    })
        as Option<
            unsafe extern "C" fn(
                *mut st_ptls_cipher_context_t,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                size_t,
            ) -> (),
        >;
    cf_aes_init(
        &raw mut (*ctx).aes,
        key as *const uint8_t,
        (*(*ctx).super_0.algo).key_size,
    );
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn aesctr_dispose(mut _ctx: *mut ptls_cipher_context_t) {
    let mut ctx: *mut aesctr_context_t = _ctx as *mut aesctr_context_t;
    ptls_clear_memory.expect("non-null function pointer")(
        ctx as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<aesctr_context_t>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn aesctr_init(
    mut _ctx: *mut ptls_cipher_context_t,
    mut iv: *const ::core::ffi::c_void,
) {
    let mut ctx: *mut aesctr_context_t = _ctx as *mut aesctr_context_t;
    cf_ctr_init(
        &raw mut (*ctx).ctr,
        &raw const cf_aes,
        &raw mut (*ctx).aes as *mut ::core::ffi::c_void,
        iv as *const uint8_t,
    );
}
#[inline]
unsafe extern "C" fn aesctr_transform(
    mut _ctx: *mut ptls_cipher_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut ctx: *mut aesctr_context_t = _ctx as *mut aesctr_context_t;
    cf_ctr_cipher(
        &raw mut (*ctx).ctr,
        input as *const uint8_t,
        output as *mut uint8_t,
        len,
    );
}
#[inline]
unsafe extern "C" fn aesctr_setup_crypto(
    mut _ctx: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut aesctr_context_t = _ctx as *mut aesctr_context_t;
    (*ctx).super_0.do_dispose =
        Some(aesctr_dispose as unsafe extern "C" fn(*mut ptls_cipher_context_t) -> ())
            as Option<unsafe extern "C" fn(*mut st_ptls_cipher_context_t) -> ()>;
    (*ctx).super_0.do_init = Some(
        aesctr_init
            as unsafe extern "C" fn(*mut ptls_cipher_context_t, *const ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut st_ptls_cipher_context_t, *const ::core::ffi::c_void) -> (),
        >;
    (*ctx).super_0.do_transform = Some(
        aesctr_transform
            as unsafe extern "C" fn(
                *mut ptls_cipher_context_t,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                size_t,
            ) -> (),
    )
        as Option<
            unsafe extern "C" fn(
                *mut st_ptls_cipher_context_t,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                size_t,
            ) -> (),
        >;
    cf_aes_init(
        &raw mut (*ctx).aes,
        key as *const uint8_t,
        (*(*ctx).super_0.algo).key_size,
    );
    return 0 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn aesgcm_dispose_crypto(mut _ctx: *mut ptls_aead_context_t) {
    let mut ctx: *mut aesgcm_context_t = _ctx as *mut aesgcm_context_t;
    ptls_clear_memory.expect("non-null function pointer")(
        (ctx as *mut uint8_t)
            .offset(::core::mem::size_of::<ptls_aead_context_t>() as usize as isize)
            as *mut ::core::ffi::c_void,
        (::core::mem::size_of::<aesgcm_context_t>() as size_t)
            .wrapping_sub(::core::mem::size_of::<ptls_aead_context_t>() as size_t),
    );
}
#[inline]
unsafe extern "C" fn aesgcm_encrypt_init(
    mut _ctx: *mut ptls_aead_context_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) {
    let mut ctx: *mut aesgcm_context_t = _ctx as *mut aesgcm_context_t;
    let mut iv: [uint8_t; 16] = [0; 16];
    ptls_aead__build_iv(
        (*ctx).super_0.algo as *const ptls_aead_algorithm_t,
        &raw mut iv as *mut uint8_t,
        &raw mut (*ctx).static_iv as *mut uint8_t,
        seq,
    );
    cf_gcm_encrypt_init(
        &raw const cf_aes,
        &raw mut (*ctx).aes as *mut ::core::ffi::c_void,
        &raw mut (*ctx).gcm,
        aad as *const uint8_t,
        aadlen,
        &raw mut iv as *mut uint8_t,
        PTLS_AESGCM_IV_SIZE as size_t,
    );
}
#[inline]
unsafe extern "C" fn aesgcm_encrypt_update(
    mut _ctx: *mut ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
) -> size_t {
    let mut ctx: *mut aesgcm_context_t = _ctx as *mut aesgcm_context_t;
    cf_gcm_encrypt_update(
        &raw mut (*ctx).gcm,
        input as *const uint8_t,
        inlen,
        output as *mut uint8_t,
    );
    return inlen;
}
#[inline]
unsafe extern "C" fn aesgcm_encrypt_final(
    mut _ctx: *mut ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
) -> size_t {
    let mut ctx: *mut aesgcm_context_t = _ctx as *mut aesgcm_context_t;
    cf_gcm_encrypt_final(
        &raw mut (*ctx).gcm,
        output as *mut uint8_t,
        PTLS_AESGCM_TAG_SIZE as size_t,
    );
    return PTLS_AESGCM_TAG_SIZE as size_t;
}
#[inline]
unsafe extern "C" fn aesgcm_decrypt(
    mut _ctx: *mut ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) -> size_t {
    let mut ctx: *mut aesgcm_context_t = _ctx as *mut aesgcm_context_t;
    let mut iv: [uint8_t; 16] = [0; 16];
    if inlen < PTLS_AESGCM_TAG_SIZE as size_t {
        return SIZE_MAX as size_t;
    }
    let mut tag_offset: size_t = inlen.wrapping_sub(PTLS_AESGCM_TAG_SIZE as size_t);
    ptls_aead__build_iv(
        (*ctx).super_0.algo as *const ptls_aead_algorithm_t,
        &raw mut iv as *mut uint8_t,
        &raw mut (*ctx).static_iv as *mut uint8_t,
        seq,
    );
    if cf_gcm_decrypt(
        &raw const cf_aes,
        &raw mut (*ctx).aes as *mut ::core::ffi::c_void,
        input as *const uint8_t,
        tag_offset,
        aad as *const uint8_t,
        aadlen,
        &raw mut iv as *mut uint8_t,
        PTLS_AESGCM_IV_SIZE as size_t,
        (input as *mut uint8_t).offset(tag_offset as isize),
        PTLS_AESGCM_TAG_SIZE as size_t,
        output as *mut uint8_t,
    ) != 0 as ::core::ffi::c_int
    {
        return SIZE_MAX as size_t;
    }
    return tag_offset;
}
#[inline]
unsafe extern "C" fn aesgcm_get_iv(
    mut _ctx: *mut ptls_aead_context_t,
    mut iv: *mut ::core::ffi::c_void,
) {
    let mut ctx: *mut aesgcm_context_t = _ctx as *mut aesgcm_context_t;
    memcpy(
        iv,
        &raw mut (*ctx).static_iv as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 12]>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn aesgcm_set_iv(
    mut _ctx: *mut ptls_aead_context_t,
    mut iv: *const ::core::ffi::c_void,
) {
    let mut ctx: *mut aesgcm_context_t = _ctx as *mut aesgcm_context_t;
    memcpy(
        &raw mut (*ctx).static_iv as *mut uint8_t as *mut ::core::ffi::c_void,
        iv,
        ::core::mem::size_of::<[uint8_t; 12]>() as size_t,
    );
}
#[inline]
unsafe extern "C" fn aead_aesgcm_setup_crypto(
    mut _ctx: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut aesgcm_context_t = _ctx as *mut aesgcm_context_t;
    (*ctx).super_0.dispose_crypto =
        Some(aesgcm_dispose_crypto as unsafe extern "C" fn(*mut ptls_aead_context_t) -> ())
            as Option<unsafe extern "C" fn(*mut st_ptls_aead_context_t) -> ()>;
    (*ctx).super_0.do_get_iv = Some(
        aesgcm_get_iv
            as unsafe extern "C" fn(*mut ptls_aead_context_t, *mut ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut st_ptls_aead_context_t, *mut ::core::ffi::c_void) -> (),
        >;
    (*ctx).super_0.do_set_iv = Some(
        aesgcm_set_iv
            as unsafe extern "C" fn(*mut ptls_aead_context_t, *const ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut st_ptls_aead_context_t, *const ::core::ffi::c_void) -> (),
        >;
    if is_enc != 0 {
        (*ctx).super_0.do_encrypt_init = Some(
            aesgcm_encrypt_init
                as unsafe extern "C" fn(
                    *mut ptls_aead_context_t,
                    uint64_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut st_ptls_aead_context_t,
                    uint64_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
            >;
        (*ctx).super_0.do_encrypt_update = Some(
            aesgcm_encrypt_update
                as unsafe extern "C" fn(
                    *mut ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> size_t,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut st_ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> size_t,
            >;
        (*ctx).super_0.do_encrypt_final = Some(
            aesgcm_encrypt_final
                as unsafe extern "C" fn(
                    *mut ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut st_ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
            >;
        (*ctx).super_0.do_encrypt = Some(
            ptls_aead__do_encrypt
                as unsafe extern "C" fn(
                    *mut ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    size_t,
                    uint64_t,
                    *const ::core::ffi::c_void,
                    size_t,
                    *mut ptls_aead_supplementary_encryption_t,
                ) -> (),
        )
            as Option<
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
            >;
        (*ctx).super_0.do_encrypt_v = Some(
            ptls_aead__do_encrypt_v
                as unsafe extern "C" fn(
                    *mut ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                    *mut ptls_iovec_t,
                    size_t,
                    uint64_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut st_ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                    *mut ptls_iovec_t,
                    size_t,
                    uint64_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
            >;
        (*ctx).super_0.do_decrypt = None;
    } else {
        (*ctx).super_0.do_encrypt_init = None;
        (*ctx).super_0.do_encrypt_update = None;
        (*ctx).super_0.do_encrypt_final = None;
        (*ctx).super_0.do_encrypt = None;
        (*ctx).super_0.do_encrypt_v = None;
        (*ctx).super_0.do_decrypt = Some(
            aesgcm_decrypt
                as unsafe extern "C" fn(
                    *mut ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    size_t,
                    uint64_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> size_t,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut st_ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    size_t,
                    uint64_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> size_t,
            >;
    }
    cf_aes_init(
        &raw mut (*ctx).aes,
        key as *const uint8_t,
        (*(*ctx).super_0.algo).key_size,
    );
    memcpy(
        &raw mut (*ctx).static_iv as *mut uint8_t as *mut ::core::ffi::c_void,
        iv,
        ::core::mem::size_of::<[uint8_t; 12]>() as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn aes128ecb_setup_crypto(
    mut ctx: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return aesecb_setup_crypto(ctx, is_enc, key);
}
unsafe extern "C" fn aes128ctr_setup_crypto(
    mut ctx: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return aesctr_setup_crypto(ctx, is_enc, key);
}
unsafe extern "C" fn aead_aes128gcm_setup_crypto(
    mut ctx: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return aead_aesgcm_setup_crypto(ctx, is_enc, key, iv);
}
unsafe extern "C" fn sha256_clone(mut _src: *mut ptls_hash_context_t) -> *mut ptls_hash_context_t {
    let mut dst: *mut sha256_context_t = ::core::ptr::null_mut::<sha256_context_t>();
    let mut src: *mut sha256_context_t = _src as *mut sha256_context_t;
    dst = malloc(::core::mem::size_of::<sha256_context_t>() as size_t) as *mut sha256_context_t;
    if dst.is_null() {
        return ::core::ptr::null_mut::<ptls_hash_context_t>();
    }
    (*dst).super_0 = (*src).super_0;
    ptls_hash_clone_memcpy(
        &raw mut (*dst).ctx as *mut ::core::ffi::c_void,
        &raw mut (*src).ctx as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cf_sha256_context>() as size_t,
    );
    return &raw mut (*dst).super_0;
}
unsafe extern "C" fn sha256_update(
    mut _ctx: *mut ptls_hash_context_t,
    mut src: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut ctx: *mut sha256_context_t = _ctx as *mut sha256_context_t;
    cf_sha256_update(&raw mut (*ctx).ctx, src, len);
}
unsafe extern "C" fn sha256_final(
    mut _ctx: *mut ptls_hash_context_t,
    mut md: *mut ::core::ffi::c_void,
    mut mode: ptls_hash_final_mode_t,
) {
    let mut ctx: *mut sha256_context_t = _ctx as *mut sha256_context_t;
    if mode as ::core::ffi::c_uint
        == PTLS_HASH_FINAL_MODE_SNAPSHOT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut copy: cf_sha256_context = (*ctx).ctx;
        cf_sha256_digest_final(&raw mut copy, md as *mut uint8_t);
        ptls_clear_memory.expect("non-null function pointer")(
            &raw mut copy as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<cf_sha256_context>() as size_t,
        );
        return;
    }
    if !md.is_null() {
        cf_sha256_digest_final(&raw mut (*ctx).ctx, md as *mut uint8_t);
    }
    match mode as ::core::ffi::c_uint {
        0 => {
            ptls_clear_memory.expect("non-null function pointer")(
                &raw mut (*ctx).ctx as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<cf_sha256_context>() as size_t,
            );
            free(ctx as *mut ::core::ffi::c_void);
        }
        1 => {
            cf_sha256_init(&raw mut (*ctx).ctx);
        }
        _ => {}
    };
}
unsafe extern "C" fn sha256_create() -> *mut ptls_hash_context_t {
    let mut ctx: *mut sha256_context_t = ::core::ptr::null_mut::<sha256_context_t>();
    ctx = malloc(::core::mem::size_of::<sha256_context_t>() as size_t) as *mut sha256_context_t;
    if ctx.is_null() {
        return ::core::ptr::null_mut::<ptls_hash_context_t>();
    }
    (*ctx).super_0 = st_ptls_hash_context_t {
        update: Some(
            sha256_update
                as unsafe extern "C" fn(
                    *mut ptls_hash_context_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        ),
        final_0: Some(
            sha256_final
                as unsafe extern "C" fn(
                    *mut ptls_hash_context_t,
                    *mut ::core::ffi::c_void,
                    ptls_hash_final_mode_t,
                ) -> (),
        ),
        clone_: Some(
            sha256_clone
                as unsafe extern "C" fn(*mut ptls_hash_context_t) -> *mut ptls_hash_context_t,
        ),
    };
    cf_sha256_init(&raw mut (*ctx).ctx);
    return &raw mut (*ctx).super_0;
}
#[no_mangle]
pub static mut ptls_minicrypto_sha256: ptls_hash_algorithm_t = unsafe {
    st_ptls_hash_algorithm_t {
        name: b"sha256\0".as_ptr() as *const ::core::ffi::c_char,
        block_size: PTLS_SHA256_BLOCK_SIZE as size_t,
        digest_size: PTLS_SHA256_DIGEST_SIZE as size_t,
        create: Some(sha256_create as unsafe extern "C" fn() -> *mut ptls_hash_context_t),
        empty_digest: [
            0xe3 as ::core::ffi::c_int as uint8_t,
            0xb0 as ::core::ffi::c_int as uint8_t,
            0xc4 as ::core::ffi::c_int as uint8_t,
            0x42 as ::core::ffi::c_int as uint8_t,
            0x98 as ::core::ffi::c_int as uint8_t,
            0xfc as ::core::ffi::c_int as uint8_t,
            0x1c as ::core::ffi::c_int as uint8_t,
            0x14 as ::core::ffi::c_int as uint8_t,
            0x9a as ::core::ffi::c_int as uint8_t,
            0xfb as ::core::ffi::c_int as uint8_t,
            0xf4 as ::core::ffi::c_int as uint8_t,
            0xc8 as ::core::ffi::c_int as uint8_t,
            0x99 as ::core::ffi::c_int as uint8_t,
            0x6f as ::core::ffi::c_int as uint8_t,
            0xb9 as ::core::ffi::c_int as uint8_t,
            0x24 as ::core::ffi::c_int as uint8_t,
            0x27 as ::core::ffi::c_int as uint8_t,
            0xae as ::core::ffi::c_int as uint8_t,
            0x41 as ::core::ffi::c_int as uint8_t,
            0xe4 as ::core::ffi::c_int as uint8_t,
            0x64 as ::core::ffi::c_int as uint8_t,
            0x9b as ::core::ffi::c_int as uint8_t,
            0x93 as ::core::ffi::c_int as uint8_t,
            0x4c as ::core::ffi::c_int as uint8_t,
            0xa4 as ::core::ffi::c_int as uint8_t,
            0x95 as ::core::ffi::c_int as uint8_t,
            0x99 as ::core::ffi::c_int as uint8_t,
            0x1b as ::core::ffi::c_int as uint8_t,
            0x78 as ::core::ffi::c_int as uint8_t,
            0x52 as ::core::ffi::c_int as uint8_t,
            0xb8 as ::core::ffi::c_int as uint8_t,
            0x55 as ::core::ffi::c_int as uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
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
    }
};
#[no_mangle]
pub static mut ptls_minicrypto_aes128ecb: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"AES128-ECB\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: PTLS_AES128_KEY_SIZE as size_t,
        block_size: PTLS_AES_BLOCK_SIZE as size_t,
        iv_size: 0 as size_t,
        context_size: ::core::mem::size_of::<aesecb_context_t>() as size_t,
        setup_crypto: Some(
            aes128ecb_setup_crypto
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
#[no_mangle]
pub static mut ptls_minicrypto_aes128ctr: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"AES128-CTR\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: PTLS_AES128_KEY_SIZE as size_t,
        block_size: 1 as size_t,
        iv_size: PTLS_AES_IV_SIZE as size_t,
        context_size: ::core::mem::size_of::<aesctr_context_t>() as size_t,
        setup_crypto: Some(
            aes128ctr_setup_crypto
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
#[no_mangle]
pub static mut ptls_minicrypto_aes128gcm: ptls_aead_algorithm_t = st_ptls_aead_algorithm_t {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    confidentiality_limit: 0,
    integrity_limit: 0,
    ctr_cipher: ::core::ptr::null::<ptls_cipher_algorithm_t>(),
    ecb_cipher: ::core::ptr::null::<ptls_cipher_algorithm_t>(),
    key_size: 0,
    iv_size: 0,
    tag_size: 0,
    tls12: C2Rust_Unnamed {
        fixed_iv_size: 0,
        record_iv_size: 0,
    },
    non_temporal: [0; 1],
    align_bits: 0,
    context_size: 0,
    setup_crypto: None,
};
#[no_mangle]
pub static mut ptls_minicrypto_aes128gcmsha256: ptls_cipher_suite_t = unsafe {
    st_ptls_cipher_suite_t {
        id: PTLS_CIPHER_SUITE_AES_128_GCM_SHA256 as uint16_t,
        aead: &raw const ptls_minicrypto_aes128gcm,
        hash: &raw const ptls_minicrypto_sha256,
        name: PTLS_CIPHER_SUITE_NAME_AES_128_GCM_SHA256.as_ptr(),
    }
};
unsafe extern "C" fn c2rust_run_static_initializers() {
    ptls_minicrypto_aes128gcm = {
        let mut init = st_ptls_aead_algorithm_t {
            non_temporal: [0; 1],
            name: b"AES128-GCM\0".as_ptr() as *const ::core::ffi::c_char,
            confidentiality_limit: PTLS_AESGCM_CONFIDENTIALITY_LIMIT as uint64_t,
            integrity_limit: 0x40000000000000 as uint64_t,
            ctr_cipher: &raw const ptls_minicrypto_aes128ctr,
            ecb_cipher: &raw const ptls_minicrypto_aes128ecb,
            key_size: PTLS_AES128_KEY_SIZE as size_t,
            iv_size: PTLS_AESGCM_IV_SIZE as size_t,
            tag_size: PTLS_AESGCM_TAG_SIZE as size_t,
            tls12: C2Rust_Unnamed {
                fixed_iv_size: PTLS_TLS12_AESGCM_FIXED_IV_SIZE as size_t,
                record_iv_size: PTLS_TLS12_AESGCM_RECORD_IV_SIZE as size_t,
            },
            align_bits: 0 as uint8_t,
            context_size: ::core::mem::size_of::<aesgcm_context_t>() as size_t,
            setup_crypto: Some(
                aead_aes128gcm_setup_crypto
                    as unsafe extern "C" fn(
                        *mut ptls_aead_context_t,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        };
        init.set_non_temporal(0 as ::core::ffi::c_uint);
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];

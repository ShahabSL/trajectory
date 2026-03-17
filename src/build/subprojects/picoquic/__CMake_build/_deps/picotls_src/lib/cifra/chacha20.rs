use ::c2rust_bitfields;
extern "C" {
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
    fn cf_poly1305_init(ctx: *mut cf_poly1305, r: *const uint8_t, s: *const uint8_t);
    fn cf_poly1305_update(ctx: *mut cf_poly1305, data: *const uint8_t, nbytes: size_t);
    fn cf_poly1305_finish(ctx: *mut cf_poly1305, out: *mut uint8_t);
    fn cf_chacha20_init(
        ctx: *mut cf_chacha20_ctx,
        key: *const uint8_t,
        nkey: size_t,
        nonce: *const uint8_t,
    );
    fn cf_chacha20_cipher(
        ctx: *mut cf_chacha20_ctx,
        input: *const uint8_t,
        output: *mut uint8_t,
        count: size_t,
    );
    fn ptls_cipher_new(
        algo: *const ptls_cipher_algorithm_t,
        is_enc: ::core::ffi::c_int,
        key: *const ::core::ffi::c_void,
    ) -> *mut ptls_cipher_context_t;
    fn ptls_cipher_free(ctx: *mut ptls_cipher_context_t);
    fn ptls_aead__build_iv(
        algo: *const ptls_aead_algorithm_t,
        iv: *mut uint8_t,
        static_iv: *const uint8_t,
        seq: uint64_t,
    );
    static mut ptls_clear_memory:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
    static mut ptls_mem_equal: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            size_t,
        ) -> ::core::ffi::c_int,
    >;
    static ptls_minicrypto_sha256: ptls_hash_algorithm_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_poly1305 {
    pub h: [uint32_t; 17],
    pub r: [uint32_t; 17],
    pub s: [uint8_t; 16],
    pub partial: [uint8_t; 16],
    pub npartial: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_chacha20_ctx {
    pub key0: [uint8_t; 16],
    pub key1: [uint8_t; 16],
    pub nonce: [uint8_t; 16],
    pub constant: *const uint8_t,
    pub block: [uint8_t; 64],
    pub nblock: size_t,
    pub ncounter: size_t,
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
pub struct chacha20_context_t {
    pub super_0: ptls_cipher_context_t,
    pub chacha: cf_chacha20_ctx,
    pub key: [uint8_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct chacha20poly1305_context_t {
    pub super_0: ptls_aead_context_t,
    pub chacha: *mut ptls_cipher_context_t,
    pub static_iv: [uint8_t; 12],
    pub aadlen: size_t,
    pub textlen: size_t,
    pub poly1305_init: Option<
        unsafe extern "C" fn(*mut chacha20poly1305_context_t, *const ::core::ffi::c_void) -> (),
    >,
    pub poly1305_update: Option<
        unsafe extern "C" fn(
            *mut chacha20poly1305_context_t,
            *const ::core::ffi::c_void,
            size_t,
        ) -> (),
    >,
    pub poly1305_finish: Option<
        unsafe extern "C" fn(*mut chacha20poly1305_context_t, *mut ::core::ffi::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cifra_chacha20poly1305_context_t {
    pub super_0: chacha20poly1305_context_t,
    pub poly: cf_poly1305,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const PTLS_CHACHA20_KEY_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const PTLS_CHACHA20_IV_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_CHACHA20POLY1305_IV_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const PTLS_CHACHA20POLY1305_TAG_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_CHACHA20POLY1305_CONFIDENTIALITY_LIMIT: ::core::ffi::c_ulong = UINT64_MAX;
pub const PTLS_CIPHER_SUITE_CHACHA20_POLY1305_SHA256: ::core::ffi::c_int =
    0x1303 as ::core::ffi::c_int;
pub const PTLS_CIPHER_SUITE_NAME_CHACHA20_POLY1305_SHA256: [::core::ffi::c_char; 29] = unsafe {
    ::core::mem::transmute::<[u8; 29], [::core::ffi::c_char; 29]>(
        *b"TLS_CHACHA20_POLY1305_SHA256\0",
    )
};
pub const PTLS_ERROR_CLASS_INTERNAL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const PTLS_TLS12_CHACHAPOLY_FIXED_IV_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const PTLS_TLS12_CHACHAPOLY_RECORD_IV_SIZE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PTLS_ERROR_LIBRARY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 3 as ::core::ffi::c_int;
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
pub const CHACHA20POLY1305_BLOCKSIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
unsafe extern "C" fn chacha20poly1305_write_u64(mut buf: *mut uint8_t, mut v: uint64_t) {
    let c2rust_fresh0 = buf;
    buf = buf.offset(1);
    *c2rust_fresh0 = (v & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh1 = buf;
    buf = buf.offset(1);
    *c2rust_fresh1 = (v >> 8 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh2 = buf;
    buf = buf.offset(1);
    *c2rust_fresh2 = (v >> 16 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh3 = buf;
    buf = buf.offset(1);
    *c2rust_fresh3 = (v >> 24 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh4 = buf;
    buf = buf.offset(1);
    *c2rust_fresh4 = (v >> 32 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh5 = buf;
    buf = buf.offset(1);
    *c2rust_fresh5 = (v >> 40 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh6 = buf;
    buf = buf.offset(1);
    *c2rust_fresh6 = (v >> 48 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    *buf = (v >> 56 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
}
unsafe extern "C" fn chacha20poly1305_encrypt_pad(
    mut ctx: *mut chacha20poly1305_context_t,
    mut n: size_t,
) {
    static mut zeros: [uint8_t; 16] = [
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
    ];
    if n.wrapping_rem(16 as size_t) != 0 as size_t {
        (*ctx).poly1305_update.expect("non-null function pointer")(
            ctx,
            &raw const zeros as *const uint8_t as *const ::core::ffi::c_void,
            (16 as size_t).wrapping_sub(n.wrapping_rem(16 as size_t)),
        );
    }
}
unsafe extern "C" fn chacha20poly1305_finalize(
    mut ctx: *mut chacha20poly1305_context_t,
    mut tag: *mut uint8_t,
) {
    let mut lenbuf: [uint8_t; 16] = [0; 16];
    chacha20poly1305_encrypt_pad(ctx, (*ctx).textlen);
    chacha20poly1305_write_u64(&raw mut lenbuf as *mut uint8_t, (*ctx).aadlen as uint64_t);
    chacha20poly1305_write_u64(
        (&raw mut lenbuf as *mut uint8_t).offset(8 as ::core::ffi::c_int as isize),
        (*ctx).textlen as uint64_t,
    );
    (*ctx).poly1305_update.expect("non-null function pointer")(
        ctx,
        &raw mut lenbuf as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    (*ctx).poly1305_finish.expect("non-null function pointer")(
        ctx,
        tag as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" fn chacha20poly1305_dispose_crypto(mut _ctx: *mut ptls_aead_context_t) {
    let mut ctx: *mut chacha20poly1305_context_t = _ctx as *mut chacha20poly1305_context_t;
    ptls_cipher_free((*ctx).chacha);
}
unsafe extern "C" fn chacha20poly1305_init(
    mut _ctx: *mut ptls_aead_context_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) {
    let mut ctx: *mut chacha20poly1305_context_t = _ctx as *mut chacha20poly1305_context_t;
    let mut tmpbuf: [uint8_t; 64] = [0; 64];
    memset(
        &raw mut tmpbuf as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (16 as ::core::ffi::c_int - PTLS_CHACHA20POLY1305_IV_SIZE) as size_t,
    );
    ptls_aead__build_iv(
        (*ctx).super_0.algo as *const ptls_aead_algorithm_t,
        (&raw mut tmpbuf as *mut uint8_t)
            .offset(16 as ::core::ffi::c_int as isize)
            .offset(-(PTLS_CHACHA20POLY1305_IV_SIZE as isize)),
        &raw mut (*ctx).static_iv as *mut uint8_t,
        seq,
    );
    ptls_cipher_init(
        (*ctx).chacha,
        &raw mut tmpbuf as *mut uint8_t as *const ::core::ffi::c_void,
    );
    memset(
        &raw mut tmpbuf as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    ptls_cipher_encrypt(
        (*ctx).chacha,
        &raw mut tmpbuf as *mut uint8_t as *mut ::core::ffi::c_void,
        &raw mut tmpbuf as *mut uint8_t as *const ::core::ffi::c_void,
        CHACHA20POLY1305_BLOCKSIZE as size_t,
    );
    (*ctx).poly1305_init.expect("non-null function pointer")(
        ctx,
        &raw mut tmpbuf as *mut uint8_t as *const ::core::ffi::c_void,
    );
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut tmpbuf as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    if aadlen != 0 as size_t {
        (*ctx).poly1305_update.expect("non-null function pointer")(ctx, aad, aadlen);
        chacha20poly1305_encrypt_pad(ctx, aadlen);
    }
    (*ctx).aadlen = aadlen;
    (*ctx).textlen = 0 as size_t;
}
unsafe extern "C" fn chacha20poly1305_encrypt_update(
    mut _ctx: *mut ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
) -> size_t {
    let mut ctx: *mut chacha20poly1305_context_t = _ctx as *mut chacha20poly1305_context_t;
    ptls_cipher_encrypt((*ctx).chacha, output, input, inlen);
    (*ctx).poly1305_update.expect("non-null function pointer")(ctx, output, inlen);
    (*ctx).textlen = (*ctx).textlen.wrapping_add(inlen);
    return inlen;
}
unsafe extern "C" fn chacha20poly1305_encrypt_final(
    mut _ctx: *mut ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
) -> size_t {
    let mut ctx: *mut chacha20poly1305_context_t = _ctx as *mut chacha20poly1305_context_t;
    chacha20poly1305_finalize(ctx, output as *mut uint8_t);
    return PTLS_CHACHA20POLY1305_TAG_SIZE as size_t;
}
unsafe extern "C" fn chacha20poly1305_decrypt(
    mut _ctx: *mut ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) -> size_t {
    let mut ctx: *mut chacha20poly1305_context_t = _ctx as *mut chacha20poly1305_context_t;
    let mut tag: [uint8_t; 16] = [0; 16];
    let mut ret: size_t = 0;
    if inlen < ::core::mem::size_of::<[uint8_t; 16]>() as usize {
        return SIZE_MAX as size_t;
    }
    chacha20poly1305_init(&raw mut (*ctx).super_0, seq, aad, aadlen);
    (*ctx).poly1305_update.expect("non-null function pointer")(
        ctx,
        input,
        inlen.wrapping_sub(::core::mem::size_of::<[uint8_t; 16]>() as size_t),
    );
    (*ctx).textlen = inlen.wrapping_sub(::core::mem::size_of::<[uint8_t; 16]>() as size_t);
    chacha20poly1305_finalize(ctx, &raw mut tag as *mut uint8_t);
    if ptls_mem_equal.expect("non-null function pointer")(
        &raw mut tag as *mut uint8_t as *const ::core::ffi::c_void,
        (input as *const uint8_t)
            .offset(inlen as isize)
            .offset(-(::core::mem::size_of::<[uint8_t; 16]>() as usize as isize))
            as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    ) != 0
    {
        ptls_cipher_encrypt(
            (*ctx).chacha,
            output,
            input,
            inlen.wrapping_sub(::core::mem::size_of::<[uint8_t; 16]>() as size_t),
        );
        ret = inlen.wrapping_sub(::core::mem::size_of::<[uint8_t; 16]>() as size_t);
    } else {
        ret = SIZE_MAX as size_t;
    }
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut tag as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    return ret;
}
unsafe extern "C" fn chacha20poly1305_get_iv(
    mut _ctx: *mut ptls_aead_context_t,
    mut iv: *mut ::core::ffi::c_void,
) {
    let mut ctx: *mut chacha20poly1305_context_t = _ctx as *mut chacha20poly1305_context_t;
    memcpy(
        iv,
        &raw mut (*ctx).static_iv as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 12]>() as size_t,
    );
}
unsafe extern "C" fn chacha20poly1305_set_iv(
    mut _ctx: *mut ptls_aead_context_t,
    mut iv: *const ::core::ffi::c_void,
) {
    let mut ctx: *mut chacha20poly1305_context_t = _ctx as *mut chacha20poly1305_context_t;
    memcpy(
        &raw mut (*ctx).static_iv as *mut uint8_t as *mut ::core::ffi::c_void,
        iv,
        ::core::mem::size_of::<[uint8_t; 12]>() as size_t,
    );
}
unsafe extern "C" fn chacha20poly1305_setup_crypto(
    mut _ctx: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
    mut chacha: *const ptls_cipher_algorithm_t,
    mut poly1305_init: Option<
        unsafe extern "C" fn(*mut chacha20poly1305_context_t, *const ::core::ffi::c_void) -> (),
    >,
    mut poly1305_update: Option<
        unsafe extern "C" fn(
            *mut chacha20poly1305_context_t,
            *const ::core::ffi::c_void,
            size_t,
        ) -> (),
    >,
    mut poly1305_finish: Option<
        unsafe extern "C" fn(*mut chacha20poly1305_context_t, *mut ::core::ffi::c_void) -> (),
    >,
) -> ::core::ffi::c_int {
    let mut ctx: *mut chacha20poly1305_context_t = _ctx as *mut chacha20poly1305_context_t;
    (*ctx).super_0.dispose_crypto = Some(
        chacha20poly1305_dispose_crypto as unsafe extern "C" fn(*mut ptls_aead_context_t) -> (),
    )
        as Option<unsafe extern "C" fn(*mut st_ptls_aead_context_t) -> ()>;
    (*ctx).super_0.do_get_iv = Some(
        chacha20poly1305_get_iv
            as unsafe extern "C" fn(*mut ptls_aead_context_t, *mut ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut st_ptls_aead_context_t, *mut ::core::ffi::c_void) -> (),
        >;
    (*ctx).super_0.do_set_iv = Some(
        chacha20poly1305_set_iv
            as unsafe extern "C" fn(*mut ptls_aead_context_t, *const ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut st_ptls_aead_context_t, *const ::core::ffi::c_void) -> (),
        >;
    if is_enc != 0 {
        (*ctx).super_0.do_encrypt_init = Some(
            chacha20poly1305_init
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
            chacha20poly1305_encrypt_update
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
            chacha20poly1305_encrypt_final
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
            chacha20poly1305_decrypt
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
    (*ctx).chacha = ptls_cipher_new(chacha, is_enc, key);
    if (*ctx).chacha.is_null() {
        return PTLS_ERROR_LIBRARY;
    }
    memcpy(
        &raw mut (*ctx).static_iv as *mut uint8_t as *mut ::core::ffi::c_void,
        iv,
        ::core::mem::size_of::<[uint8_t; 12]>() as size_t,
    );
    (*ctx).poly1305_init = poly1305_init;
    (*ctx).poly1305_update = poly1305_update;
    (*ctx).poly1305_finish = poly1305_finish;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn chacha20_dispose(mut _ctx: *mut ptls_cipher_context_t) {
    let mut ctx: *mut chacha20_context_t = _ctx as *mut chacha20_context_t;
    ptls_clear_memory.expect("non-null function pointer")(
        ctx as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<chacha20_context_t>() as size_t,
    );
}
unsafe extern "C" fn chacha20_init(
    mut _ctx: *mut ptls_cipher_context_t,
    mut iv: *const ::core::ffi::c_void,
) {
    let mut ctx: *mut chacha20_context_t = _ctx as *mut chacha20_context_t;
    (*ctx).chacha.nblock = 0 as size_t;
    (*ctx).chacha.ncounter = 0 as size_t;
    memcpy(
        &raw mut (*ctx).chacha.nonce as *mut uint8_t as *mut ::core::ffi::c_void,
        iv,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
}
unsafe extern "C" fn chacha20_transform(
    mut _ctx: *mut ptls_cipher_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut ctx: *mut chacha20_context_t = _ctx as *mut chacha20_context_t;
    cf_chacha20_cipher(
        &raw mut (*ctx).chacha,
        input as *const uint8_t,
        output as *mut uint8_t,
        len,
    );
}
unsafe extern "C" fn chacha20_setup_crypto(
    mut _ctx: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ctx: *mut chacha20_context_t = _ctx as *mut chacha20_context_t;
    (*ctx).super_0.do_dispose =
        Some(chacha20_dispose as unsafe extern "C" fn(*mut ptls_cipher_context_t) -> ())
            as Option<unsafe extern "C" fn(*mut st_ptls_cipher_context_t) -> ()>;
    (*ctx).super_0.do_init = Some(
        chacha20_init
            as unsafe extern "C" fn(*mut ptls_cipher_context_t, *const ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut st_ptls_cipher_context_t, *const ::core::ffi::c_void) -> (),
        >;
    (*ctx).super_0.do_transform = Some(
        chacha20_transform
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
    cf_chacha20_init(
        &raw mut (*ctx).chacha,
        key as *const uint8_t,
        PTLS_CHACHA20_KEY_SIZE as size_t,
        b"01234567\0".as_ptr() as *const ::core::ffi::c_char as *const uint8_t,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn cifra_poly1305_init(
    mut _ctx: *mut chacha20poly1305_context_t,
    mut rs: *const ::core::ffi::c_void,
) {
    let mut ctx: *mut cifra_chacha20poly1305_context_t =
        _ctx as *mut cifra_chacha20poly1305_context_t;
    cf_poly1305_init(
        &raw mut (*ctx).poly,
        rs as *const uint8_t,
        (rs as *const uint8_t).offset(16 as ::core::ffi::c_int as isize),
    );
}
unsafe extern "C" fn cifra_poly1305_update(
    mut _ctx: *mut chacha20poly1305_context_t,
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut ctx: *mut cifra_chacha20poly1305_context_t =
        _ctx as *mut cifra_chacha20poly1305_context_t;
    cf_poly1305_update(&raw mut (*ctx).poly, input as *const uint8_t, len);
}
unsafe extern "C" fn cifra_poly1305_finish(
    mut _ctx: *mut chacha20poly1305_context_t,
    mut tag: *mut ::core::ffi::c_void,
) {
    let mut ctx: *mut cifra_chacha20poly1305_context_t =
        _ctx as *mut cifra_chacha20poly1305_context_t;
    cf_poly1305_finish(&raw mut (*ctx).poly, tag as *mut uint8_t);
}
unsafe extern "C" fn cifra_chacha20poly1305_setup_crypto(
    mut ctx: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return chacha20poly1305_setup_crypto(
        ctx,
        is_enc,
        key,
        iv,
        &raw const ptls_minicrypto_chacha20,
        Some(
            cifra_poly1305_init
                as unsafe extern "C" fn(
                    *mut chacha20poly1305_context_t,
                    *const ::core::ffi::c_void,
                ) -> (),
        ),
        Some(
            cifra_poly1305_update
                as unsafe extern "C" fn(
                    *mut chacha20poly1305_context_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        ),
        Some(
            cifra_poly1305_finish
                as unsafe extern "C" fn(
                    *mut chacha20poly1305_context_t,
                    *mut ::core::ffi::c_void,
                ) -> (),
        ),
    );
}
#[no_mangle]
pub static mut ptls_minicrypto_chacha20: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"CHACHA20\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: PTLS_CHACHA20_KEY_SIZE as size_t,
        block_size: 1 as size_t,
        iv_size: PTLS_CHACHA20_IV_SIZE as size_t,
        context_size: ::core::mem::size_of::<chacha20_context_t>() as size_t,
        setup_crypto: Some(
            chacha20_setup_crypto
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
#[no_mangle]
pub static mut ptls_minicrypto_chacha20poly1305: ptls_aead_algorithm_t = st_ptls_aead_algorithm_t {
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
pub static mut ptls_minicrypto_chacha20poly1305sha256: ptls_cipher_suite_t = unsafe {
    st_ptls_cipher_suite_t {
        id: PTLS_CIPHER_SUITE_CHACHA20_POLY1305_SHA256 as uint16_t,
        aead: &raw const ptls_minicrypto_chacha20poly1305,
        hash: &raw const ptls_minicrypto_sha256,
        name: PTLS_CIPHER_SUITE_NAME_CHACHA20_POLY1305_SHA256.as_ptr(),
    }
};
unsafe extern "C" fn c2rust_run_static_initializers() {
    ptls_minicrypto_chacha20poly1305 = {
        let mut init = st_ptls_aead_algorithm_t {
            non_temporal: [0; 1],
            name: b"CHACHA20-POLY1305\0".as_ptr() as *const ::core::ffi::c_char,
            confidentiality_limit: PTLS_CHACHA20POLY1305_CONFIDENTIALITY_LIMIT as uint64_t,
            integrity_limit: 0x1000000000 as uint64_t,
            ctr_cipher: &raw const ptls_minicrypto_chacha20,
            ecb_cipher: ::core::ptr::null::<ptls_cipher_algorithm_t>(),
            key_size: PTLS_CHACHA20_KEY_SIZE as size_t,
            iv_size: PTLS_CHACHA20POLY1305_IV_SIZE as size_t,
            tag_size: PTLS_CHACHA20POLY1305_TAG_SIZE as size_t,
            tls12: C2Rust_Unnamed {
                fixed_iv_size: PTLS_TLS12_CHACHAPOLY_FIXED_IV_SIZE as size_t,
                record_iv_size: PTLS_TLS12_CHACHAPOLY_RECORD_IV_SIZE as size_t,
            },
            align_bits: 0 as uint8_t,
            context_size: ::core::mem::size_of::<cifra_chacha20poly1305_context_t>() as size_t,
            setup_crypto: Some(
                cifra_chacha20poly1305_setup_crypto
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

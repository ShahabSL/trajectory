use ::c2rust_bitfields;
use ::core::arch::asm;
#[cfg(target_arch = "x86")]
pub use ::core::arch::x86::{
    __m128i, __m256i, _mm256_add_epi64, _mm256_aesenc_epi128, _mm256_aesenclast_epi128,
    _mm256_and_si256, _mm256_broadcastsi128_si256, _mm256_castsi128_si256, _mm256_castsi256_si128,
    _mm256_load_si256, _mm256_loadu_si256, _mm256_setzero_si256, _mm256_shuffle_epi8,
    _mm256_store_si256, _mm256_storeu_si256, _mm256_stream_si256, _mm256_xor_si256, _mm_add_epi64,
    _mm_aesenc_si128, _mm_aesenclast_si128, _mm_aeskeygenassist_si128, _mm_and_si128,
    _mm_cmpeq_epi8, _mm_cmpgt_epi32, _mm_cmplt_epi32, _mm_load_si128, _mm_loadu_si128,
    _mm_movemask_epi8, _mm_or_si128, _mm_set_epi32, _mm_set_epi64x, _mm_setzero_si128, _mm_sfence,
    _mm_shuffle_epi32, _mm_shuffle_epi8, _mm_slli_epi64, _mm_slli_si128, _mm_srli_epi64,
    _mm_store_si128, _mm_storeu_si128, _mm_xor_si128,
};
#[cfg(target_arch = "x86_64")]
pub use ::core::arch::x86_64::{
    __m128i, __m256i, _mm256_add_epi64, _mm256_aesenc_epi128, _mm256_aesenclast_epi128,
    _mm256_and_si256, _mm256_broadcastsi128_si256, _mm256_castsi128_si256, _mm256_castsi256_si128,
    _mm256_load_si256, _mm256_loadu_si256, _mm256_setzero_si256, _mm256_shuffle_epi8,
    _mm256_store_si256, _mm256_storeu_si256, _mm256_stream_si256, _mm256_xor_si256, _mm_add_epi64,
    _mm_aesenc_si128, _mm_aesenclast_si128, _mm_aeskeygenassist_si128, _mm_and_si128,
    _mm_cmpeq_epi8, _mm_cmpgt_epi32, _mm_cmplt_epi32, _mm_insert_epi64, _mm_load_si128,
    _mm_loadu_si128, _mm_movemask_epi8, _mm_or_si128, _mm_set_epi32, _mm_set_epi64x,
    _mm_setzero_si128, _mm_sfence, _mm_shuffle_epi32, _mm_shuffle_epi8, _mm_slli_epi64,
    _mm_slli_si128, _mm_srli_epi64, _mm_store_si128, _mm_storeu_si128, _mm_xor_si128,
};
extern "C" {
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn aligned_alloc(__alignment: size_t, __size: size_t) -> *mut ::core::ffi::c_void;
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
    static mut ptls_clear_memory:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
}
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = usize;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __loadu_si128 {
    pub __v: __m128i_u,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __storeu_si128 {
    pub __v: __m128i_u,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __loadu_si256 {
    pub __v: __m256i_u,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct __storeu_si256 {
    pub __v: __m256i_u,
}
pub type ptls_iovec_t = st_ptls_iovec_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_iovec_t {
    pub base: *mut uint8_t,
    pub len: size_t,
}
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
#[repr(C, align(32))]
pub struct ptls_fusion_aesecb_context(pub C2Rust_ptls_fusion_aesecb_context_Inner);
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_ptls_fusion_aesecb_context_Inner {
    pub keys: C2Rust_Unnamed_0,
    pub rounds: ::core::ffi::c_uint,
    pub aesni256: uint8_t,
}
#[allow(dead_code, non_upper_case_globals)]
const C2Rust_ptls_fusion_aesecb_context_PADDING: usize =
    ::core::mem::size_of::<ptls_fusion_aesecb_context>()
        - ::core::mem::size_of::<C2Rust_ptls_fusion_aesecb_context_Inner>();
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub m128: [__m128i; 15],
    pub m256: [__m256i; 15],
}
pub type ptls_fusion_aesecb_context_t = ptls_fusion_aesecb_context;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptls_fusion_aesgcm_context {
    pub ecb: ptls_fusion_aesecb_context_t,
    pub capacity: size_t,
    pub ghash_cnt: size_t,
}
pub type ptls_fusion_aesgcm_context_t = ptls_fusion_aesgcm_context;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptls_fusion_aesgcm_ghash_precompute128 {
    pub H: __m128i,
    pub r: __m128i,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptls_fusion_aesgcm_context128 {
    pub super_0: ptls_fusion_aesgcm_context,
    pub ghash: [ptls_fusion_aesgcm_ghash_precompute128; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub H: [__m128i; 2],
    pub r: [__m128i; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union ptls_fusion_aesgcm_ghash_precompute256 {
    pub c2rust_unnamed: C2Rust_Unnamed_1,
    pub c2rust_unnamed_0: C2Rust_Unnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub Hx2: __m256i,
    pub rx2: __m256i,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptls_fusion_aesgcm_context256 {
    pub super_0: ptls_fusion_aesgcm_context,
    pub ghash: [ptls_fusion_aesgcm_ghash_precompute256; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ctr_context {
    pub super_0: ptls_cipher_context_t,
    pub fusion: ptls_fusion_aesecb_context_t,
    pub bits: __m128i,
    pub is_ready: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptls_fusion_gfmul_state128 {
    pub hi: __m128i,
    pub lo: __m128i,
    pub mid: __m128i,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aesgcm_context {
    pub super_0: ptls_aead_context_t,
    pub aesgcm: *mut ptls_fusion_aesgcm_context_t,
    pub static_iv: __m128i,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ptls_fusion_gfmul_state256 {
    pub hi: __m256i,
    pub lo: __m256i,
    pub mid: __m256i,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const NULL_0: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const PTLS_AES128_KEY_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_AES256_KEY_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const PTLS_AES_IV_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_AESGCM_IV_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const PTLS_AESGCM_TAG_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_AESGCM_CONFIDENTIALITY_LIMIT: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
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
pub const PTLS_X86_CACHE_LINE_ALIGN_BITS: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
static mut poly_: [uint64_t; 2] = [
    1 as ::core::ffi::c_int as uint64_t,
    0xc200000000000000 as ::core::ffi::c_ulong,
];
static mut byteswap_: [uint8_t; 32] = [
    15 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    12 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
    15 as ::core::ffi::c_int as uint8_t,
    14 as ::core::ffi::c_int as uint8_t,
    13 as ::core::ffi::c_int as uint8_t,
    12 as ::core::ffi::c_int as uint8_t,
    11 as ::core::ffi::c_int as uint8_t,
    10 as ::core::ffi::c_int as uint8_t,
    9 as ::core::ffi::c_int as uint8_t,
    8 as ::core::ffi::c_int as uint8_t,
    7 as ::core::ffi::c_int as uint8_t,
    6 as ::core::ffi::c_int as uint8_t,
    5 as ::core::ffi::c_int as uint8_t,
    4 as ::core::ffi::c_int as uint8_t,
    3 as ::core::ffi::c_int as uint8_t,
    2 as ::core::ffi::c_int as uint8_t,
    1 as ::core::ffi::c_int as uint8_t,
    0 as ::core::ffi::c_int as uint8_t,
];
static mut one_: [uint8_t; 16] = [
    1 as ::core::ffi::c_int as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
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
static mut incr128x2_: [uint8_t; 32] = [
    2 as ::core::ffi::c_int as uint8_t,
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
    2 as ::core::ffi::c_int as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
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
unsafe extern "C" fn transformH(mut H: __m128i) -> __m128i {
    let mut t2: __m128i = _mm_shuffle_epi32(H, 0xff as ::core::ffi::c_int);
    let mut t1: __m128i = H;
    H = _mm_slli_epi64(H, 1 as ::core::ffi::c_int);
    let mut t3: __m128i = _mm_setzero_si128();
    t1 = _mm_srli_epi64(t1, 63 as ::core::ffi::c_int);
    t3 = _mm_cmplt_epi32(t2, t3);
    t1 = _mm_slli_si128(t1, 8 as ::core::ffi::c_int);
    H = _mm_or_si128(t1, H);
    t3 = _mm_and_si128(t3, *(&raw const poly_ as *const uint64_t as *mut __m128i));
    H = _mm_xor_si128(t3, H);
    return H;
}
#[inline]
unsafe extern "C" fn gfmul_firststep128(
    mut gstate: *mut ptls_fusion_gfmul_state128,
    mut X: __m128i,
    mut precompute: *mut ptls_fusion_aesgcm_ghash_precompute128,
) {
    X = _mm_shuffle_epi8(X, *(&raw const byteswap_ as *const uint8_t as *mut __m128i));
    X = _mm_xor_si128((*gstate).lo, X);
    (*gstate).lo = _mm_setzero_si128();
    (*gstate).hi = _mm_setzero_si128();
    (*gstate).mid = _mm_setzero_si128();
    gfmul_do_step128(gstate, X, precompute);
}
#[inline]
unsafe extern "C" fn gfmul_nextstep128(
    mut gstate: *mut ptls_fusion_gfmul_state128,
    mut X: __m128i,
    mut precompute: *mut ptls_fusion_aesgcm_ghash_precompute128,
) {
    X = _mm_shuffle_epi8(X, *(&raw const byteswap_ as *const uint8_t as *mut __m128i));
    gfmul_do_step128(gstate, X, precompute);
}
#[inline]
unsafe extern "C" fn gfmul_reduce128(mut gstate: *mut ptls_fusion_gfmul_state128) {
    (*gstate).lo = gfmul_do_reduce((*gstate).hi, (*gstate).lo, (*gstate).mid);
}
#[inline]
unsafe extern "C" fn gfmul_get_tag128(
    mut gstate: *mut ptls_fusion_gfmul_state128,
    mut ek0: __m128i,
) -> __m128i {
    let mut tag: __m128i = _mm_shuffle_epi8(
        (*gstate).lo,
        *(&raw const byteswap_ as *const uint8_t as *mut __m128i),
    );
    tag = _mm_xor_si128(tag, ek0);
    return tag;
}
#[inline]
unsafe extern "C" fn gfmul_nextstep256(
    mut gstate: *mut ptls_fusion_gfmul_state256,
    mut X: __m256i,
    mut precompute: *mut ptls_fusion_aesgcm_ghash_precompute256,
) {
    X = _mm256_shuffle_epi8(X, *(&raw const byteswap_ as *const uint8_t as *mut __m256i));
    gfmul_do_step256(gstate, X, precompute);
}
#[inline]
unsafe extern "C" fn gfmul_get_tag256(
    mut gstate: *mut ptls_fusion_gfmul_state256,
    mut ek0: __m128i,
) -> __m128i {
    let mut tag: __m128i = _mm_shuffle_epi8(
        _mm256_castsi256_si128((*gstate).lo),
        *(&raw const byteswap_ as *const uint8_t as *mut __m128i),
    );
    tag = _mm_xor_si128(tag, ek0);
    return tag;
}
#[inline]
unsafe extern "C" fn aesecb_encrypt(
    mut ctx: *mut ptls_fusion_aesecb_context_t,
    mut v: __m128i,
) -> __m128i {
    v = _mm_xor_si128(
        v,
        if (*ctx).0.aesni256 as ::core::ffi::c_int != 0 {
            _mm256_castsi256_si128((*ctx).0.keys.m256[0 as ::core::ffi::c_int as usize])
        } else {
            (*ctx).0.keys.m128[0 as ::core::ffi::c_int as usize]
        },
    );
    let mut i: size_t = 1 as size_t;
    while i < (*ctx).0.rounds as size_t {
        v = _mm_aesenc_si128(
            v,
            if (*ctx).0.aesni256 as ::core::ffi::c_int != 0 {
                _mm256_castsi256_si128((*ctx).0.keys.m256[i as usize])
            } else {
                (*ctx).0.keys.m128[i as usize]
            },
        );
        i = i.wrapping_add(1);
    }
    v = _mm_aesenclast_si128(
        v,
        if (*ctx).0.aesni256 as ::core::ffi::c_int != 0 {
            _mm256_castsi256_si128((*ctx).0.keys.m256[(*ctx).0.rounds as usize])
        } else {
            (*ctx).0.keys.m128[(*ctx).0.rounds as usize]
        },
    );
    return v;
}
static mut loadn_mask: [uint8_t; 63] = [
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0xff as ::core::ffi::c_int as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
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
static mut loadn_shuffle: [uint8_t; 31] = [
    0 as ::core::ffi::c_int as uint8_t,
    0x1 as ::core::ffi::c_int as uint8_t,
    0x2 as ::core::ffi::c_int as uint8_t,
    0x3 as ::core::ffi::c_int as uint8_t,
    0x4 as ::core::ffi::c_int as uint8_t,
    0x5 as ::core::ffi::c_int as uint8_t,
    0x6 as ::core::ffi::c_int as uint8_t,
    0x7 as ::core::ffi::c_int as uint8_t,
    0x8 as ::core::ffi::c_int as uint8_t,
    0x9 as ::core::ffi::c_int as uint8_t,
    0xa as ::core::ffi::c_int as uint8_t,
    0xb as ::core::ffi::c_int as uint8_t,
    0xc as ::core::ffi::c_int as uint8_t,
    0xd as ::core::ffi::c_int as uint8_t,
    0xe as ::core::ffi::c_int as uint8_t,
    0xf as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
    0x80 as ::core::ffi::c_int as uint8_t,
];
#[inline]
unsafe extern "C" fn loadn_end_of_page(
    mut p: *const ::core::ffi::c_void,
    mut l: size_t,
) -> __m128i {
    let mut shift: uintptr_t = p as uintptr_t & 15 as ::core::ffi::c_int as uintptr_t;
    let mut pattern: __m128i = _mm_loadu_si128(
        (&raw const loadn_shuffle as *const uint8_t).offset(shift as isize) as *const __m128i_u,
    );
    return _mm_shuffle_epi8(
        _mm_load_si128((p as uintptr_t).wrapping_sub(shift) as *const __m128i),
        pattern,
    );
}
#[inline]
unsafe extern "C" fn loadn128(mut p: *const ::core::ffi::c_void, mut l: size_t) -> __m128i {
    let mut v: __m128i = _mm_setzero_si128();
    let mut mask: __m128i = _mm_loadu_si128(
        (&raw const loadn_mask as *const uint8_t)
            .offset(32 as ::core::ffi::c_int as isize)
            .offset(-(l as isize)) as *mut __m128i,
    );
    let mut mod4k: uintptr_t =
        (p as uintptr_t).wrapping_rem(4096 as ::core::ffi::c_int as uintptr_t);
    if (mod4k <= (4096 as ::core::ffi::c_int - 16 as ::core::ffi::c_int) as uintptr_t)
        as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
        || mod4k.wrapping_add(l as uintptr_t) > 4096 as ::core::ffi::c_int as uintptr_t
    {
        v = _mm_loadu_si128(p as *const __m128i_u);
    } else {
        v = loadn_end_of_page(p, l);
    }
    v = _mm_and_si128(v, mask);
    return v;
}
#[inline]
unsafe extern "C" fn storen128(mut _p: *mut ::core::ffi::c_void, mut l: size_t, mut v: __m128i) {
    let mut buf: [uint8_t; 16] = [0; 16];
    let mut p: *mut uint8_t = _p as *mut uint8_t;
    *(&raw mut buf as *mut uint8_t as *mut __m128i) = v;
    let mut i: size_t = 0 as size_t;
    while i != l {
        *p.offset(i as isize) = buf[i as usize];
        i = i.wrapping_add(1);
    }
}
pub const STATE_EK0_BEEN_FED: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
pub const STATE_EK0_INCOMPLETE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const STATE_SUPP_USED: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const STATE_SUPP_IN_PROCESS: ::core::ffi::c_int = 0x8 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn ptls_fusion_aesgcm_decrypt(
    mut _ctx: *mut ptls_fusion_aesgcm_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
    mut ctr: __m128i,
    mut _aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
    mut tag: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut c2rust_current_block: u64;
    let mut ctx: *mut ptls_fusion_aesgcm_context128 =
        _ctx as *mut ::core::ffi::c_void as *mut ptls_fusion_aesgcm_context128;
    let mut ek0: __m128i = _mm_setzero_si128();
    let mut bits0: __m128i = _mm_setzero_si128();
    let mut bits1: __m128i = _mm_setzero_si128();
    let mut bits2: __m128i = _mm_setzero_si128();
    let mut bits3: __m128i = _mm_setzero_si128();
    let mut bits4: __m128i = _mm_setzero_si128();
    let mut bits5: __m128i = _mm_setzero_si128();
    let mut gstate: ptls_fusion_gfmul_state128 = ptls_fusion_gfmul_state128 {
        hi: _mm_set_epi64x(0 as ::core::ffi::c_int as ::core::ffi::c_longlong),
        lo: _mm_setzero_si128(),
        mid: _mm_setzero_si128(),
    };
    let mut gdatabuf: [__m128i; 6] = [_mm_setzero_si128(); 6];
    let mut ac: __m128i = _mm_shuffle_epi8(
        _mm_set_epi32(
            0 as ::core::ffi::c_int,
            aadlen as ::core::ffi::c_int * 8 as ::core::ffi::c_int,
            0 as ::core::ffi::c_int,
            inlen as ::core::ffi::c_int * 8 as ::core::ffi::c_int,
        ),
        *(&raw const byteswap_ as *const uint8_t as *mut __m128i),
    );
    let mut ghash_precompute: *mut ptls_fusion_aesgcm_ghash_precompute128 = (&raw mut (*ctx).ghash
        as *mut ptls_fusion_aesgcm_ghash_precompute128)
        .offset(aadlen.wrapping_add(15 as size_t).wrapping_div(16 as size_t) as isize)
        .offset(inlen.wrapping_add(15 as size_t).wrapping_div(16 as size_t) as isize)
        .offset(1 as ::core::ffi::c_int as isize);
    let mut gdata: *const __m128i = ::core::ptr::null::<__m128i>();
    let mut gdata_cnt: size_t = 0;
    let mut src_ghash: *const __m128i = input as *const __m128i;
    let mut src_aes: *const __m128i = input as *const __m128i;
    let mut aad: *const __m128i = _aad as *const __m128i;
    let mut dst: *mut __m128i = output as *mut __m128i;
    let mut nondata_aes_cnt: size_t = 0 as size_t;
    let mut src_ghashlen: size_t = inlen;
    let mut src_aeslen: size_t = inlen;
    ctr = _mm_add_epi64(ctr, *(&raw const one_ as *const uint8_t as *mut __m128i));
    bits0 = _mm_xor_si128(
        _mm_shuffle_epi8(
            ctr,
            *(&raw const byteswap_ as *const uint8_t as *mut __m128i),
        ),
        (*ctx).super_0.ecb.0.keys.m128[0 as ::core::ffi::c_int as usize],
    );
    nondata_aes_cnt = nondata_aes_cnt.wrapping_add(1);
    let mut state: ::core::ffi::c_int = STATE_IS_FIRST_RUN | STATE_GHASH_HAS_MORE;
    loop {
        let mut c2rust_current_block_31: u64;
        if (aadlen != 0 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            gdata = &raw mut gdatabuf as *mut __m128i;
            gdata_cnt = 0 as size_t;
            loop {
                if !(gdata_cnt < 6 as size_t) {
                    c2rust_current_block_31 = 7828949454673616476;
                    break;
                }
                if aadlen < 16 as size_t {
                    if aadlen != 0 as size_t {
                        let c2rust_fresh3 = gdata_cnt;
                        gdata_cnt = gdata_cnt.wrapping_add(1);
                        gdatabuf[c2rust_fresh3 as usize] =
                            loadn128(aad as *const ::core::ffi::c_void, aadlen);
                        aadlen = 0 as size_t;
                        nondata_aes_cnt = nondata_aes_cnt.wrapping_add(1);
                    }
                    c2rust_current_block_31 = 1452292811756968377;
                    break;
                } else {
                    let c2rust_fresh4 = aad;
                    aad = aad.offset(1);
                    let c2rust_fresh5 = gdata_cnt;
                    gdata_cnt = gdata_cnt.wrapping_add(1);
                    gdatabuf[c2rust_fresh5 as usize] = _mm_loadu_si128(c2rust_fresh4);
                    aadlen = aadlen.wrapping_sub(16 as size_t);
                    nondata_aes_cnt = nondata_aes_cnt.wrapping_add(1);
                }
            }
        } else if (src_ghashlen >= (6 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as size_t)
            as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            gdata = src_ghash;
            gdata_cnt = 6 as size_t;
            src_ghash = src_ghash.offset(6 as ::core::ffi::c_int as isize);
            src_ghashlen = src_ghashlen
                .wrapping_sub((6 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as size_t);
            c2rust_current_block_31 = 7828949454673616476;
        } else {
            gdata = &raw mut gdatabuf as *mut __m128i;
            gdata_cnt = 0 as size_t;
            c2rust_current_block_31 = 1452292811756968377;
        }
        match c2rust_current_block_31 {
            1452292811756968377 => {
                while gdata_cnt < 6 as size_t {
                    if src_ghashlen < 16 as size_t {
                        if src_ghashlen != 0 as size_t {
                            let c2rust_fresh6 = gdata_cnt;
                            gdata_cnt = gdata_cnt.wrapping_add(1);
                            gdatabuf[c2rust_fresh6 as usize] =
                                loadn128(src_ghash as *const ::core::ffi::c_void, src_ghashlen);
                            src_ghash = (src_ghash as *mut uint8_t).offset(src_ghashlen as isize)
                                as *mut __m128i;
                            src_ghashlen = 0 as size_t;
                        }
                        if gdata_cnt < 6 as size_t
                            && state & STATE_GHASH_HAS_MORE != 0 as ::core::ffi::c_int
                        {
                            let c2rust_fresh7 = gdata_cnt;
                            gdata_cnt = gdata_cnt.wrapping_add(1);
                            gdatabuf[c2rust_fresh7 as usize] = ac;
                            state &= !STATE_GHASH_HAS_MORE;
                        }
                        break;
                    } else {
                        let c2rust_fresh8 = src_ghash;
                        src_ghash = src_ghash.offset(1);
                        let c2rust_fresh9 = gdata_cnt;
                        gdata_cnt = gdata_cnt.wrapping_add(1);
                        gdatabuf[c2rust_fresh9 as usize] = _mm_loadu_si128(c2rust_fresh8);
                        src_ghashlen = src_ghashlen.wrapping_sub(16 as size_t);
                    }
                }
            }
            _ => {}
        }
        if (nondata_aes_cnt == 0 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            c2rust_current_block = 10728732287637542977;
        } else {
            match nondata_aes_cnt {
                0 => {
                    c2rust_current_block = 10728732287637542977;
                }
                1 => {
                    c2rust_current_block = 14850059467078475019;
                }
                2 => {
                    c2rust_current_block = 8323085581075043509;
                }
                3 => {
                    c2rust_current_block = 3316471251920683019;
                }
                4 => {
                    c2rust_current_block = 3291411303413444384;
                }
                5 => {
                    c2rust_current_block = 7510682714197564670;
                }
                _ => {
                    c2rust_current_block = 12556861819962772176;
                }
            }
        }
        match c2rust_current_block {
            10728732287637542977 => {
                ctr = _mm_add_epi64(ctr, *(&raw const one_ as *const uint8_t as *mut __m128i));
                bits0 = _mm_xor_si128(
                    _mm_shuffle_epi8(
                        ctr,
                        *(&raw const byteswap_ as *const uint8_t as *mut __m128i),
                    ),
                    (*ctx).super_0.ecb.0.keys.m128[0 as ::core::ffi::c_int as usize],
                );
                c2rust_current_block = 14850059467078475019;
            }
            _ => {}
        }
        match c2rust_current_block {
            14850059467078475019 => {
                ctr = _mm_add_epi64(ctr, *(&raw const one_ as *const uint8_t as *mut __m128i));
                bits1 = _mm_xor_si128(
                    _mm_shuffle_epi8(
                        ctr,
                        *(&raw const byteswap_ as *const uint8_t as *mut __m128i),
                    ),
                    (*ctx).super_0.ecb.0.keys.m128[0 as ::core::ffi::c_int as usize],
                );
                c2rust_current_block = 8323085581075043509;
            }
            _ => {}
        }
        match c2rust_current_block {
            8323085581075043509 => {
                ctr = _mm_add_epi64(ctr, *(&raw const one_ as *const uint8_t as *mut __m128i));
                bits2 = _mm_xor_si128(
                    _mm_shuffle_epi8(
                        ctr,
                        *(&raw const byteswap_ as *const uint8_t as *mut __m128i),
                    ),
                    (*ctx).super_0.ecb.0.keys.m128[0 as ::core::ffi::c_int as usize],
                );
                c2rust_current_block = 3316471251920683019;
            }
            _ => {}
        }
        match c2rust_current_block {
            3316471251920683019 => {
                ctr = _mm_add_epi64(ctr, *(&raw const one_ as *const uint8_t as *mut __m128i));
                bits3 = _mm_xor_si128(
                    _mm_shuffle_epi8(
                        ctr,
                        *(&raw const byteswap_ as *const uint8_t as *mut __m128i),
                    ),
                    (*ctx).super_0.ecb.0.keys.m128[0 as ::core::ffi::c_int as usize],
                );
                c2rust_current_block = 3291411303413444384;
            }
            _ => {}
        }
        match c2rust_current_block {
            3291411303413444384 => {
                ctr = _mm_add_epi64(ctr, *(&raw const one_ as *const uint8_t as *mut __m128i));
                bits4 = _mm_xor_si128(
                    _mm_shuffle_epi8(
                        ctr,
                        *(&raw const byteswap_ as *const uint8_t as *mut __m128i),
                    ),
                    (*ctx).super_0.ecb.0.keys.m128[0 as ::core::ffi::c_int as usize],
                );
                c2rust_current_block = 7510682714197564670;
            }
            _ => {}
        }
        match c2rust_current_block {
            7510682714197564670 => {
                ctr = _mm_add_epi64(ctr, *(&raw const one_ as *const uint8_t as *mut __m128i));
                bits5 = _mm_xor_si128(
                    _mm_shuffle_epi8(
                        ctr,
                        *(&raw const byteswap_ as *const uint8_t as *mut __m128i),
                    ),
                    (*ctx).super_0.ecb.0.keys.m128[0 as ::core::ffi::c_int as usize],
                );
            }
            _ => {}
        }
        let mut aesi: size_t = 0;
        aesi = 1 as size_t;
        while aesi <= gdata_cnt {
            let mut k: __m128i = (*ctx).super_0.ecb.0.keys.m128[aesi as usize];
            bits0 = _mm_aesenc_si128(bits0, k);
            bits1 = _mm_aesenc_si128(bits1, k);
            bits2 = _mm_aesenc_si128(bits2, k);
            bits3 = _mm_aesenc_si128(bits3, k);
            bits4 = _mm_aesenc_si128(bits4, k);
            bits5 = _mm_aesenc_si128(bits5, k);
            let c2rust_fresh10 = gdata;
            gdata = gdata.offset(1);
            ghash_precompute = ghash_precompute.offset(-1);
            gfmul_nextstep128(
                &raw mut gstate,
                _mm_loadu_si128(c2rust_fresh10),
                ghash_precompute,
            );
            aesi = aesi.wrapping_add(1);
        }
        while aesi < (*ctx).super_0.ecb.0.rounds as size_t {
            let mut k_0: __m128i = (*ctx).super_0.ecb.0.keys.m128[aesi as usize];
            bits0 = _mm_aesenc_si128(bits0, k_0);
            bits1 = _mm_aesenc_si128(bits1, k_0);
            bits2 = _mm_aesenc_si128(bits2, k_0);
            bits3 = _mm_aesenc_si128(bits3, k_0);
            bits4 = _mm_aesenc_si128(bits4, k_0);
            bits5 = _mm_aesenc_si128(bits5, k_0);
            aesi = aesi.wrapping_add(1);
        }
        let mut k_1: __m128i = (*ctx).super_0.ecb.0.keys.m128[aesi as usize];
        bits0 = _mm_aesenclast_si128(bits0, k_1);
        bits1 = _mm_aesenclast_si128(bits1, k_1);
        bits2 = _mm_aesenclast_si128(bits2, k_1);
        bits3 = _mm_aesenclast_si128(bits3, k_1);
        bits4 = _mm_aesenclast_si128(bits4, k_1);
        bits5 = _mm_aesenclast_si128(bits5, k_1);
        if (nondata_aes_cnt == 0 as size_t
            && src_aeslen >= (6 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as size_t)
            as ::core::ffi::c_int as ::core::ffi::c_long
            != 0
        {
            _mm_storeu_si128(
                dst.offset(0 as ::core::ffi::c_int as isize),
                _mm_xor_si128(
                    _mm_loadu_si128(src_aes.offset(0 as ::core::ffi::c_int as isize)),
                    bits0,
                ),
            );
            _mm_storeu_si128(
                dst.offset(1 as ::core::ffi::c_int as isize),
                _mm_xor_si128(
                    _mm_loadu_si128(src_aes.offset(1 as ::core::ffi::c_int as isize)),
                    bits1,
                ),
            );
            _mm_storeu_si128(
                dst.offset(2 as ::core::ffi::c_int as isize),
                _mm_xor_si128(
                    _mm_loadu_si128(src_aes.offset(2 as ::core::ffi::c_int as isize)),
                    bits2,
                ),
            );
            _mm_storeu_si128(
                dst.offset(3 as ::core::ffi::c_int as isize),
                _mm_xor_si128(
                    _mm_loadu_si128(src_aes.offset(3 as ::core::ffi::c_int as isize)),
                    bits3,
                ),
            );
            _mm_storeu_si128(
                dst.offset(4 as ::core::ffi::c_int as isize),
                _mm_xor_si128(
                    _mm_loadu_si128(src_aes.offset(4 as ::core::ffi::c_int as isize)),
                    bits4,
                ),
            );
            _mm_storeu_si128(
                dst.offset(5 as ::core::ffi::c_int as isize),
                _mm_xor_si128(
                    _mm_loadu_si128(src_aes.offset(5 as ::core::ffi::c_int as isize)),
                    bits5,
                ),
            );
            dst = dst.offset(6 as ::core::ffi::c_int as isize);
            src_aes = src_aes.offset(6 as ::core::ffi::c_int as isize);
            src_aeslen = src_aeslen
                .wrapping_sub((6 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as size_t);
        } else {
            if state & STATE_IS_FIRST_RUN != 0 as ::core::ffi::c_int {
                ek0 = bits0;
                state &= !STATE_IS_FIRST_RUN;
            }
            match nondata_aes_cnt {
                0 => {
                    if (src_aeslen > 16 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0
                    {
                        let c2rust_fresh11 = dst;
                        dst = dst.offset(1);
                        let c2rust_fresh12 = src_aes;
                        src_aes = src_aes.offset(1);
                        _mm_storeu_si128(
                            c2rust_fresh11,
                            _mm_xor_si128(_mm_loadu_si128(c2rust_fresh12), bits0),
                        );
                        src_aeslen = src_aeslen.wrapping_sub(16 as size_t);
                    } else {
                        bits0 = bits0;
                        break;
                    }
                    c2rust_current_block = 7427217052296911891;
                }
                1 => {
                    c2rust_current_block = 7427217052296911891;
                }
                2 => {
                    c2rust_current_block = 880812279832167549;
                }
                3 => {
                    c2rust_current_block = 13029489650236943732;
                }
                4 => {
                    c2rust_current_block = 7012586270880649460;
                }
                5 => {
                    c2rust_current_block = 2941155481249580917;
                }
                _ => {
                    c2rust_current_block = 11718254377427810743;
                }
            }
            match c2rust_current_block {
                7427217052296911891 => {
                    if (src_aeslen > 16 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0
                    {
                        let c2rust_fresh13 = dst;
                        dst = dst.offset(1);
                        let c2rust_fresh14 = src_aes;
                        src_aes = src_aes.offset(1);
                        _mm_storeu_si128(
                            c2rust_fresh13,
                            _mm_xor_si128(_mm_loadu_si128(c2rust_fresh14), bits1),
                        );
                        src_aeslen = src_aeslen.wrapping_sub(16 as size_t);
                    } else {
                        bits0 = bits1;
                        break;
                    }
                    c2rust_current_block = 880812279832167549;
                }
                _ => {}
            }
            match c2rust_current_block {
                880812279832167549 => {
                    if (src_aeslen > 16 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0
                    {
                        let c2rust_fresh15 = dst;
                        dst = dst.offset(1);
                        let c2rust_fresh16 = src_aes;
                        src_aes = src_aes.offset(1);
                        _mm_storeu_si128(
                            c2rust_fresh15,
                            _mm_xor_si128(_mm_loadu_si128(c2rust_fresh16), bits2),
                        );
                        src_aeslen = src_aeslen.wrapping_sub(16 as size_t);
                    } else {
                        bits0 = bits2;
                        break;
                    }
                    c2rust_current_block = 13029489650236943732;
                }
                _ => {}
            }
            match c2rust_current_block {
                13029489650236943732 => {
                    if (src_aeslen > 16 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0
                    {
                        let c2rust_fresh17 = dst;
                        dst = dst.offset(1);
                        let c2rust_fresh18 = src_aes;
                        src_aes = src_aes.offset(1);
                        _mm_storeu_si128(
                            c2rust_fresh17,
                            _mm_xor_si128(_mm_loadu_si128(c2rust_fresh18), bits3),
                        );
                        src_aeslen = src_aeslen.wrapping_sub(16 as size_t);
                    } else {
                        bits0 = bits3;
                        break;
                    }
                    c2rust_current_block = 7012586270880649460;
                }
                _ => {}
            }
            match c2rust_current_block {
                7012586270880649460 => {
                    if (src_aeslen > 16 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0
                    {
                        let c2rust_fresh19 = dst;
                        dst = dst.offset(1);
                        let c2rust_fresh20 = src_aes;
                        src_aes = src_aes.offset(1);
                        _mm_storeu_si128(
                            c2rust_fresh19,
                            _mm_xor_si128(_mm_loadu_si128(c2rust_fresh20), bits4),
                        );
                        src_aeslen = src_aeslen.wrapping_sub(16 as size_t);
                    } else {
                        bits0 = bits4;
                        break;
                    }
                    c2rust_current_block = 2941155481249580917;
                }
                _ => {}
            }
            match c2rust_current_block {
                2941155481249580917 => {
                    if (src_aeslen > 16 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0
                    {
                        let c2rust_fresh21 = dst;
                        dst = dst.offset(1);
                        let c2rust_fresh22 = src_aes;
                        src_aes = src_aes.offset(1);
                        _mm_storeu_si128(
                            c2rust_fresh21,
                            _mm_xor_si128(_mm_loadu_si128(c2rust_fresh22), bits5),
                        );
                        src_aeslen = src_aeslen.wrapping_sub(16 as size_t);
                    } else {
                        bits0 = bits5;
                        break;
                    }
                }
                _ => {}
            }
            nondata_aes_cnt = 0 as size_t;
        }
    }
    if src_aeslen == 16 as size_t {
        _mm_storeu_si128(
            dst as *mut __m128i_u,
            _mm_xor_si128(_mm_loadu_si128(src_aes as *const __m128i_u), bits0),
        );
    } else if src_aeslen != 0 as size_t {
        storen128(
            dst as *mut ::core::ffi::c_void,
            src_aeslen,
            _mm_xor_si128(
                loadn128(src_aes as *const ::core::ffi::c_void, src_aeslen),
                bits0,
            ),
        );
    }
    if state & STATE_GHASH_HAS_MORE != 0 as ::core::ffi::c_int {
        ghash_precompute = ghash_precompute.offset(-1);
        gfmul_nextstep128(&raw mut gstate, ac, ghash_precompute);
    }
    gfmul_reduce128(&raw mut gstate);
    let mut calctag: __m128i = gfmul_get_tag128(&raw mut gstate, ek0);
    return (_mm_movemask_epi8(_mm_cmpeq_epi8(
        calctag,
        _mm_loadu_si128(tag as *const __m128i_u),
    )) == 0xffff as ::core::ffi::c_int) as ::core::ffi::c_int;
}
pub const STATE_IS_FIRST_RUN: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const STATE_GHASH_HAS_MORE: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
unsafe extern "C" fn expand_key(mut key: __m128i, mut temp: __m128i) -> __m128i {
    key = _mm_xor_si128(key, _mm_slli_si128(key, 4 as ::core::ffi::c_int));
    key = _mm_xor_si128(key, _mm_slli_si128(key, 4 as ::core::ffi::c_int));
    key = _mm_xor_si128(key, _mm_slli_si128(key, 4 as ::core::ffi::c_int));
    key = _mm_xor_si128(key, temp);
    return key;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_fusion_aesecb_init(
    mut ctx: *mut ptls_fusion_aesecb_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut key_size: size_t,
    mut aesni256: ::core::ffi::c_int,
) {
    let mut i: size_t = 0 as size_t;
    match key_size {
        16 => {
            (*ctx).0.rounds = 10 as ::core::ffi::c_uint;
        }
        32 => {
            (*ctx).0.rounds = 14 as ::core::ffi::c_uint;
        }
        _ => {}
    }
    (*ctx).0.aesni256 = aesni256 as uint8_t;
    let c2rust_fresh0 = i;
    i = i.wrapping_add(1);
    (*ctx).0.keys.m128[c2rust_fresh0 as usize] = _mm_loadu_si128(key as *mut __m128i);
    if key_size == 32 as size_t {
        let c2rust_fresh1 = i;
        i = i.wrapping_add(1);
        (*ctx).0.keys.m128[c2rust_fresh1 as usize] =
            _mm_loadu_si128((key as *mut __m128i).offset(1 as ::core::ffi::c_int as isize));
    }
    loop {
        (*ctx).0.keys.m128[i as usize] = expand_key(
            (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
            _mm_shuffle_epi32(
                _mm_aeskeygenassist_si128(
                    (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                    0x1 as ::core::ffi::c_int,
                ),
                (3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    | 3 as ::core::ffi::c_int,
            ),
        );
        if i == (*ctx).0.rounds as size_t {
            break;
        }
        i = i.wrapping_add(1);
        if key_size > 24 as size_t {
            (*ctx).0.keys.m128[i as usize] = expand_key(
                (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
                _mm_shuffle_epi32(
                    _mm_aeskeygenassist_si128(
                        (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                        0x1 as ::core::ffi::c_int,
                    ),
                    (2 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int,
                ),
            );
            i = i.wrapping_add(1);
        }
        (*ctx).0.keys.m128[i as usize] = expand_key(
            (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
            _mm_shuffle_epi32(
                _mm_aeskeygenassist_si128(
                    (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                    0x2 as ::core::ffi::c_int,
                ),
                (3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    | 3 as ::core::ffi::c_int,
            ),
        );
        if i == (*ctx).0.rounds as size_t {
            break;
        }
        i = i.wrapping_add(1);
        if key_size > 24 as size_t {
            (*ctx).0.keys.m128[i as usize] = expand_key(
                (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
                _mm_shuffle_epi32(
                    _mm_aeskeygenassist_si128(
                        (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                        0x2 as ::core::ffi::c_int,
                    ),
                    (2 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int,
                ),
            );
            i = i.wrapping_add(1);
        }
        (*ctx).0.keys.m128[i as usize] = expand_key(
            (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
            _mm_shuffle_epi32(
                _mm_aeskeygenassist_si128(
                    (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                    0x4 as ::core::ffi::c_int,
                ),
                (3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    | 3 as ::core::ffi::c_int,
            ),
        );
        if i == (*ctx).0.rounds as size_t {
            break;
        }
        i = i.wrapping_add(1);
        if key_size > 24 as size_t {
            (*ctx).0.keys.m128[i as usize] = expand_key(
                (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
                _mm_shuffle_epi32(
                    _mm_aeskeygenassist_si128(
                        (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                        0x4 as ::core::ffi::c_int,
                    ),
                    (2 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int,
                ),
            );
            i = i.wrapping_add(1);
        }
        (*ctx).0.keys.m128[i as usize] = expand_key(
            (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
            _mm_shuffle_epi32(
                _mm_aeskeygenassist_si128(
                    (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                    0x8 as ::core::ffi::c_int,
                ),
                (3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    | 3 as ::core::ffi::c_int,
            ),
        );
        if i == (*ctx).0.rounds as size_t {
            break;
        }
        i = i.wrapping_add(1);
        if key_size > 24 as size_t {
            (*ctx).0.keys.m128[i as usize] = expand_key(
                (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
                _mm_shuffle_epi32(
                    _mm_aeskeygenassist_si128(
                        (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                        0x8 as ::core::ffi::c_int,
                    ),
                    (2 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int,
                ),
            );
            i = i.wrapping_add(1);
        }
        (*ctx).0.keys.m128[i as usize] = expand_key(
            (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
            _mm_shuffle_epi32(
                _mm_aeskeygenassist_si128(
                    (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                    0x10 as ::core::ffi::c_int,
                ),
                (3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    | 3 as ::core::ffi::c_int,
            ),
        );
        if i == (*ctx).0.rounds as size_t {
            break;
        }
        i = i.wrapping_add(1);
        if key_size > 24 as size_t {
            (*ctx).0.keys.m128[i as usize] = expand_key(
                (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
                _mm_shuffle_epi32(
                    _mm_aeskeygenassist_si128(
                        (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                        0x10 as ::core::ffi::c_int,
                    ),
                    (2 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int,
                ),
            );
            i = i.wrapping_add(1);
        }
        (*ctx).0.keys.m128[i as usize] = expand_key(
            (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
            _mm_shuffle_epi32(
                _mm_aeskeygenassist_si128(
                    (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                    0x20 as ::core::ffi::c_int,
                ),
                (3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    | 3 as ::core::ffi::c_int,
            ),
        );
        if i == (*ctx).0.rounds as size_t {
            break;
        }
        i = i.wrapping_add(1);
        if key_size > 24 as size_t {
            (*ctx).0.keys.m128[i as usize] = expand_key(
                (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
                _mm_shuffle_epi32(
                    _mm_aeskeygenassist_si128(
                        (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                        0x20 as ::core::ffi::c_int,
                    ),
                    (2 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int,
                ),
            );
            i = i.wrapping_add(1);
        }
        (*ctx).0.keys.m128[i as usize] = expand_key(
            (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
            _mm_shuffle_epi32(
                _mm_aeskeygenassist_si128(
                    (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                    0x40 as ::core::ffi::c_int,
                ),
                (3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    | 3 as ::core::ffi::c_int,
            ),
        );
        if i == (*ctx).0.rounds as size_t {
            break;
        }
        i = i.wrapping_add(1);
        if key_size > 24 as size_t {
            (*ctx).0.keys.m128[i as usize] = expand_key(
                (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
                _mm_shuffle_epi32(
                    _mm_aeskeygenassist_si128(
                        (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                        0x40 as ::core::ffi::c_int,
                    ),
                    (2 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int,
                ),
            );
            i = i.wrapping_add(1);
        }
        (*ctx).0.keys.m128[i as usize] = expand_key(
            (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
            _mm_shuffle_epi32(
                _mm_aeskeygenassist_si128(
                    (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                    0x80 as ::core::ffi::c_int,
                ),
                (3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    | 3 as ::core::ffi::c_int,
            ),
        );
        if i == (*ctx).0.rounds as size_t {
            break;
        }
        i = i.wrapping_add(1);
        if key_size > 24 as size_t {
            (*ctx).0.keys.m128[i as usize] = expand_key(
                (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
                _mm_shuffle_epi32(
                    _mm_aeskeygenassist_si128(
                        (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                        0x80 as ::core::ffi::c_int,
                    ),
                    (2 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int,
                ),
            );
            i = i.wrapping_add(1);
        }
        (*ctx).0.keys.m128[i as usize] = expand_key(
            (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
            _mm_shuffle_epi32(
                _mm_aeskeygenassist_si128(
                    (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                    0x1b as ::core::ffi::c_int,
                ),
                (3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    | 3 as ::core::ffi::c_int,
            ),
        );
        if i == (*ctx).0.rounds as size_t {
            break;
        }
        i = i.wrapping_add(1);
        if key_size > 24 as size_t {
            (*ctx).0.keys.m128[i as usize] = expand_key(
                (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
                _mm_shuffle_epi32(
                    _mm_aeskeygenassist_si128(
                        (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                        0x1b as ::core::ffi::c_int,
                    ),
                    (2 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int,
                ),
            );
            i = i.wrapping_add(1);
        }
        (*ctx).0.keys.m128[i as usize] = expand_key(
            (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
            _mm_shuffle_epi32(
                _mm_aeskeygenassist_si128(
                    (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                    0x36 as ::core::ffi::c_int,
                ),
                (3 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                    | (3 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                    | 3 as ::core::ffi::c_int,
            ),
        );
        if i == (*ctx).0.rounds as size_t {
            break;
        }
        i = i.wrapping_add(1);
        if key_size > 24 as size_t {
            (*ctx).0.keys.m128[i as usize] = expand_key(
                (*ctx).0.keys.m128[i.wrapping_sub(key_size.wrapping_div(16 as size_t)) as usize],
                _mm_shuffle_epi32(
                    _mm_aeskeygenassist_si128(
                        (*ctx).0.keys.m128[i.wrapping_sub(1 as size_t) as usize],
                        0x36 as ::core::ffi::c_int,
                    ),
                    (2 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 4 as ::core::ffi::c_int
                        | (2 as ::core::ffi::c_int) << 2 as ::core::ffi::c_int
                        | 2 as ::core::ffi::c_int,
                ),
            );
            i = i.wrapping_add(1);
        }
    }
    if (*ctx).0.aesni256 != 0 {
        let mut i_0: size_t = (*ctx).0.rounds as size_t;
        loop {
            (*ctx).0.keys.m256[i_0 as usize] =
                _mm256_broadcastsi128_si256((*ctx).0.keys.m128[i_0 as usize]);
            let c2rust_fresh2 = i_0;
            i_0 = i_0.wrapping_sub(1);
            if !(c2rust_fresh2 != 0 as size_t) {
                break;
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ptls_fusion_aesecb_dispose(mut ctx: *mut ptls_fusion_aesecb_context_t) {
    ptls_clear_memory.expect("non-null function pointer")(
        ctx as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<ptls_fusion_aesecb_context_t>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn ptls_fusion_aesecb_encrypt(
    mut ctx: *mut ptls_fusion_aesecb_context_t,
    mut dst: *mut ::core::ffi::c_void,
    mut src: *const ::core::ffi::c_void,
) {
    let mut v: __m128i = _mm_loadu_si128(src as *const __m128i_u);
    v = aesecb_encrypt(ctx, v);
    _mm_storeu_si128(dst as *mut __m128i_u, v);
}
unsafe extern "C" fn aesgcm_calc_ghash_cnt(mut capacity: size_t) -> size_t {
    return capacity
        .wrapping_add(15 as size_t)
        .wrapping_div(16 as size_t)
        .wrapping_add(2 as size_t);
}
unsafe extern "C" fn setup_one_ghash_entry(mut ctx: *mut ptls_fusion_aesgcm_context_t) {
    let mut H: *mut __m128i = ::core::ptr::null_mut::<__m128i>();
    let mut r: *mut __m128i = ::core::ptr::null_mut::<__m128i>();
    let mut Hprev: *mut __m128i = ::core::ptr::null_mut::<__m128i>();
    let mut H0: __m128i = _mm_setzero_si128();
    if (*ctx).ecb.0.aesni256 != 0 {
        let mut ctx256: *mut ptls_fusion_aesgcm_context256 =
            ctx as *mut ::core::ffi::c_void as *mut ptls_fusion_aesgcm_context256;
        H = (&raw mut (*(&raw mut (*ctx256).ghash as *mut ptls_fusion_aesgcm_ghash_precompute256)
            .offset((*ctx).ghash_cnt.wrapping_div(2 as size_t) as isize))
        .c2rust_unnamed
        .H as *mut __m128i)
            .offset(
                ((*ctx).ghash_cnt.wrapping_rem(2 as size_t) == 0 as size_t) as ::core::ffi::c_int
                    as isize,
            ) as *mut __m128i;
        r = (&raw mut (*(&raw mut (*ctx256).ghash as *mut ptls_fusion_aesgcm_ghash_precompute256)
            .offset((*ctx).ghash_cnt.wrapping_div(2 as size_t) as isize))
        .c2rust_unnamed
        .r as *mut __m128i)
            .offset(
                ((*ctx).ghash_cnt.wrapping_rem(2 as size_t) == 0 as size_t) as ::core::ffi::c_int
                    as isize,
            ) as *mut __m128i;
        Hprev = if (*ctx).ghash_cnt == 0 as size_t {
            ::core::ptr::null_mut::<__m128i>()
        } else {
            (&raw mut (*(&raw mut (*ctx256).ghash as *mut ptls_fusion_aesgcm_ghash_precompute256)
                .offset(
                    (*ctx)
                        .ghash_cnt
                        .wrapping_sub(1 as size_t)
                        .wrapping_div(2 as size_t) as isize,
                ))
            .c2rust_unnamed
            .H as *mut __m128i)
                .offset(
                    ((*ctx)
                        .ghash_cnt
                        .wrapping_sub(1 as size_t)
                        .wrapping_rem(2 as size_t)
                        == 0 as size_t) as ::core::ffi::c_int as isize,
                ) as *mut __m128i
        };
        H0 = (*(&raw mut (*ctx256).ghash as *mut ptls_fusion_aesgcm_ghash_precompute256)
            .offset(0 as ::core::ffi::c_int as isize))
        .c2rust_unnamed
        .H[1 as ::core::ffi::c_int as usize];
    } else {
        let mut ctx128: *mut ptls_fusion_aesgcm_context128 =
            ctx as *mut ::core::ffi::c_void as *mut ptls_fusion_aesgcm_context128;
        H = &raw mut (*(&raw mut (*ctx128).ghash as *mut ptls_fusion_aesgcm_ghash_precompute128)
            .offset((*ctx).ghash_cnt as isize))
        .H;
        r = &raw mut (*(&raw mut (*ctx128).ghash as *mut ptls_fusion_aesgcm_ghash_precompute128)
            .offset((*ctx).ghash_cnt as isize))
        .r;
        Hprev = if (*ctx).ghash_cnt == 0 as size_t {
            ::core::ptr::null_mut::<__m128i>()
        } else {
            &raw mut (*(&raw mut (*ctx128).ghash as *mut ptls_fusion_aesgcm_ghash_precompute128)
                .offset((*ctx).ghash_cnt.wrapping_sub(1 as size_t) as isize))
            .H
        };
        H0 = (*(&raw mut (*ctx128).ghash as *mut ptls_fusion_aesgcm_ghash_precompute128)
            .offset(0 as ::core::ffi::c_int as isize))
        .H;
    }
    if !Hprev.is_null() {
        *H = gfmul(*Hprev, H0);
    }
    *r = _mm_shuffle_epi32(*H, 78 as ::core::ffi::c_int);
    *r = _mm_xor_si128(*r, *H);
    (*ctx).ghash_cnt = (*ctx).ghash_cnt.wrapping_add(1);
}
unsafe extern "C" fn calc_aesgcm_context_size(
    mut ghash_cnt: *mut size_t,
    mut aesni256: ::core::ffi::c_int,
) -> size_t {
    let mut sz: size_t = 0;
    if aesni256 != 0 {
        if (*ghash_cnt).wrapping_rem(2 as size_t) != 0 as size_t {
            *ghash_cnt = (*ghash_cnt).wrapping_add(1);
        }
        sz = (544 as usize).wrapping_add(
            (::core::mem::size_of::<ptls_fusion_aesgcm_ghash_precompute256>() as usize)
                .wrapping_mul(*ghash_cnt)
                .wrapping_div(2 as usize),
        ) as size_t;
    } else {
        sz = (544 as usize).wrapping_add(
            (::core::mem::size_of::<ptls_fusion_aesgcm_ghash_precompute128>() as usize)
                .wrapping_mul(*ghash_cnt),
        ) as size_t;
    }
    return sz;
}
unsafe extern "C" fn new_aesgcm(
    mut key: *const ::core::ffi::c_void,
    mut key_size: size_t,
    mut capacity: size_t,
    mut aesni256: ::core::ffi::c_int,
) -> *mut ptls_fusion_aesgcm_context_t {
    let mut ctx: *mut ptls_fusion_aesgcm_context_t =
        ::core::ptr::null_mut::<ptls_fusion_aesgcm_context_t>();
    let mut ghash_cnt: size_t = aesgcm_calc_ghash_cnt(capacity);
    let mut ctx_size: size_t = calc_aesgcm_context_size(&raw mut ghash_cnt, aesni256);
    ctx = aligned_alloc(32 as size_t, ctx_size) as *mut ptls_fusion_aesgcm_context_t;
    if ctx.is_null() {
        return ::core::ptr::null_mut::<ptls_fusion_aesgcm_context_t>();
    }
    ptls_fusion_aesecb_init(
        &raw mut (*ctx).ecb,
        1 as ::core::ffi::c_int,
        key,
        key_size,
        aesni256,
    );
    (*ctx).capacity = capacity;
    let mut H0: __m128i = aesecb_encrypt(&raw mut (*ctx).ecb, _mm_setzero_si128());
    H0 = _mm_shuffle_epi8(
        H0,
        *(&raw const byteswap_ as *const uint8_t as *mut __m128i),
    );
    H0 = transformH(H0);
    if (*ctx).ecb.0.aesni256 != 0 {
        (*(&raw mut (*(ctx as *mut ptls_fusion_aesgcm_context256)).ghash
            as *mut ptls_fusion_aesgcm_ghash_precompute256)
            .offset(0 as ::core::ffi::c_int as isize))
        .c2rust_unnamed
        .H[1 as ::core::ffi::c_int as usize] = H0;
    } else {
        (*(&raw mut (*(ctx as *mut ptls_fusion_aesgcm_context128)).ghash
            as *mut ptls_fusion_aesgcm_ghash_precompute128)
            .offset(0 as ::core::ffi::c_int as isize))
        .H = H0;
    }
    (*ctx).ghash_cnt = 0 as size_t;
    while (*ctx).ghash_cnt < ghash_cnt {
        setup_one_ghash_entry(ctx);
    }
    return ctx;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_fusion_aesgcm_new(
    mut key: *const ::core::ffi::c_void,
    mut key_size: size_t,
    mut capacity: size_t,
) -> *mut ptls_fusion_aesgcm_context_t {
    return new_aesgcm(key, key_size, capacity, 0 as ::core::ffi::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn ptls_fusion_aesgcm_set_capacity(
    mut ctx: *mut ptls_fusion_aesgcm_context_t,
    mut capacity: size_t,
) -> *mut ptls_fusion_aesgcm_context_t {
    let mut new_ghash_cnt: size_t = aesgcm_calc_ghash_cnt(capacity);
    if new_ghash_cnt <= (*ctx).ghash_cnt {
        return ctx;
    }
    let mut new_ctx_size: size_t = calc_aesgcm_context_size(
        &raw mut new_ghash_cnt,
        (*ctx).ecb.0.aesni256 as ::core::ffi::c_int,
    );
    let mut old_ctx_size: size_t = calc_aesgcm_context_size(
        &raw mut (*ctx).ghash_cnt,
        (*ctx).ecb.0.aesni256 as ::core::ffi::c_int,
    );
    let mut newp: *mut ptls_fusion_aesgcm_context_t =
        ::core::ptr::null_mut::<ptls_fusion_aesgcm_context_t>();
    newp = aligned_alloc(32 as size_t, new_ctx_size) as *mut ptls_fusion_aesgcm_context_t;
    if newp.is_null() {
        return ::core::ptr::null_mut::<ptls_fusion_aesgcm_context_t>();
    }
    memcpy(
        newp as *mut ::core::ffi::c_void,
        ctx as *const ::core::ffi::c_void,
        old_ctx_size,
    );
    ptls_clear_memory.expect("non-null function pointer")(
        ctx as *mut ::core::ffi::c_void,
        old_ctx_size,
    );
    free(ctx as *mut ::core::ffi::c_void);
    ctx = newp;
    (*ctx).capacity = capacity;
    while (*ctx).ghash_cnt < new_ghash_cnt {
        setup_one_ghash_entry(ctx);
    }
    return ctx;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_fusion_aesgcm_free(mut ctx: *mut ptls_fusion_aesgcm_context_t) {
    ptls_clear_memory.expect("non-null function pointer")(
        ctx as *mut ::core::ffi::c_void,
        calc_aesgcm_context_size(
            &raw mut (*ctx).ghash_cnt,
            (*ctx).ecb.0.aesni256 as ::core::ffi::c_int,
        ),
    );
    free(ctx as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn ctr_dispose(mut _ctx: *mut ptls_cipher_context_t) {
    let mut ctx: *mut ctr_context = _ctx as *mut ctr_context;
    ptls_fusion_aesecb_dispose(&raw mut (*ctx).fusion);
    _mm_storeu_si128(&raw mut (*ctx).bits, _mm_setzero_si128());
}
unsafe extern "C" fn ctr_init(
    mut _ctx: *mut ptls_cipher_context_t,
    mut iv: *const ::core::ffi::c_void,
) {
    let mut ctx: *mut ctr_context = _ctx as *mut ctr_context;
    _mm_storeu_si128(
        &raw mut (*ctx).bits,
        aesecb_encrypt(
            &raw mut (*ctx).fusion,
            _mm_loadu_si128(iv as *const __m128i_u),
        ),
    );
    (*ctx).is_ready = 1 as uint8_t;
}
unsafe extern "C" fn ctr_transform(
    mut _ctx: *mut ptls_cipher_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut ctx: *mut ctr_context = _ctx as *mut ctr_context;
    (*ctx).is_ready = 0 as uint8_t;
    if len < 16 as size_t {
        storen128(
            output,
            len,
            _mm_xor_si128(_mm_loadu_si128(&raw mut (*ctx).bits), loadn128(input, len)),
        );
    } else {
        _mm_storeu_si128(
            output as *mut __m128i_u,
            _mm_xor_si128(
                _mm_loadu_si128(&raw mut (*ctx).bits),
                _mm_loadu_si128(input as *const __m128i_u),
            ),
        );
    };
}
unsafe extern "C" fn aesctr_setup(
    mut _ctx: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut key_size: size_t,
) -> ::core::ffi::c_int {
    let mut ctx: *mut ctr_context = _ctx as *mut ctr_context;
    (*ctx).super_0.do_dispose =
        Some(ctr_dispose as unsafe extern "C" fn(*mut ptls_cipher_context_t) -> ())
            as Option<unsafe extern "C" fn(*mut st_ptls_cipher_context_t) -> ()>;
    (*ctx).super_0.do_init = Some(
        ctr_init
            as unsafe extern "C" fn(*mut ptls_cipher_context_t, *const ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut st_ptls_cipher_context_t, *const ::core::ffi::c_void) -> (),
        >;
    (*ctx).super_0.do_transform = Some(
        ctr_transform
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
    ptls_fusion_aesecb_init(
        &raw mut (*ctx).fusion,
        1 as ::core::ffi::c_int,
        key,
        key_size,
        0 as ::core::ffi::c_int,
    );
    (*ctx).is_ready = 0 as uint8_t;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn aes128ctr_setup(
    mut ctx: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return aesctr_setup(ctx, is_enc, key, PTLS_AES128_KEY_SIZE as size_t);
}
unsafe extern "C" fn aes256ctr_setup(
    mut ctx: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return aesctr_setup(ctx, is_enc, key, PTLS_AES256_KEY_SIZE as size_t);
}
unsafe extern "C" fn aesgcm_dispose_crypto(mut _ctx: *mut ptls_aead_context_t) {
    let mut ctx: *mut aesgcm_context = _ctx as *mut aesgcm_context;
    ptls_fusion_aesgcm_free((*ctx).aesgcm);
}
unsafe extern "C" fn aead_do_encrypt_init(
    mut _ctx: *mut ptls_aead_context_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) {
}
unsafe extern "C" fn aead_do_encrypt_update(
    mut _ctx: *mut ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
) -> size_t {
    return SIZE_MAX as size_t;
}
unsafe extern "C" fn aead_do_encrypt_final(
    mut _ctx: *mut ptls_aead_context_t,
    mut _output: *mut ::core::ffi::c_void,
) -> size_t {
    return SIZE_MAX as size_t;
}
#[inline]
unsafe extern "C" fn calc_counter(mut ctx: *mut aesgcm_context, mut seq: uint64_t) -> __m128i {
    let mut ctr: __m128i = _mm_setzero_si128();
    ctr = _mm_insert_epi64(ctr, seq as ::core::ffi::c_longlong, 0 as ::core::ffi::c_int);
    ctr = _mm_slli_si128(ctr, 4 as ::core::ffi::c_int);
    ctr = _mm_xor_si128((*ctx).static_iv, ctr);
    return ctr;
}
#[no_mangle]
pub unsafe extern "C" fn aead_do_encrypt(
    mut _ctx: *mut st_ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
    mut supp: *mut ptls_aead_supplementary_encryption_t,
) {
    let mut ctx: *mut aesgcm_context = _ctx as *mut ::core::ffi::c_void as *mut aesgcm_context;
    if inlen.wrapping_add(aadlen) > (*(*ctx).aesgcm).capacity {
        (*ctx).aesgcm = ptls_fusion_aesgcm_set_capacity((*ctx).aesgcm, inlen.wrapping_add(aadlen));
    }
    ptls_fusion_aesgcm_encrypt(
        (*ctx).aesgcm,
        output,
        input,
        inlen,
        calc_counter(ctx, seq),
        aad,
        aadlen,
        supp,
    );
}
unsafe extern "C" fn aead_do_encrypt_v(
    mut ctx: *mut st_ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *mut ptls_iovec_t,
    mut incnt: size_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) {
}
unsafe extern "C" fn aead_do_decrypt(
    mut _ctx: *mut ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) -> size_t {
    let mut ctx: *mut aesgcm_context = _ctx as *mut ::core::ffi::c_void as *mut aesgcm_context;
    if inlen < 16 as size_t {
        return SIZE_MAX as size_t;
    }
    let mut enclen: size_t = inlen.wrapping_sub(16 as size_t);
    if enclen.wrapping_add(aadlen) > (*(*ctx).aesgcm).capacity {
        (*ctx).aesgcm = ptls_fusion_aesgcm_set_capacity((*ctx).aesgcm, enclen.wrapping_add(aadlen));
    }
    if ptls_fusion_aesgcm_decrypt(
        (*ctx).aesgcm,
        output,
        input,
        enclen,
        calc_counter(ctx, seq),
        aad,
        aadlen,
        (input as *const uint8_t).offset(enclen as isize) as *const ::core::ffi::c_void,
    ) == 0
    {
        return SIZE_MAX as size_t;
    }
    return enclen;
}
#[inline]
unsafe extern "C" fn aesgcm_get_iv(
    mut _ctx: *mut ptls_aead_context_t,
    mut iv: *mut ::core::ffi::c_void,
) {
    let mut ctx: *mut aesgcm_context = _ctx as *mut aesgcm_context;
    let mut m128: __m128i = _mm_shuffle_epi8(
        (*ctx).static_iv,
        *(&raw const byteswap_ as *const uint8_t as *mut __m128i),
    );
    storen128(iv, PTLS_AESGCM_IV_SIZE as size_t, m128);
}
#[inline]
unsafe extern "C" fn aesgcm_set_iv(
    mut _ctx: *mut ptls_aead_context_t,
    mut iv: *const ::core::ffi::c_void,
) {
    let mut ctx: *mut aesgcm_context = _ctx as *mut aesgcm_context;
    (*ctx).static_iv = loadn128(iv, PTLS_AESGCM_IV_SIZE as size_t);
    (*ctx).static_iv = _mm_shuffle_epi8(
        (*ctx).static_iv,
        *(&raw const byteswap_ as *const uint8_t as *mut __m128i),
    );
}
unsafe extern "C" fn aesgcm_setup(
    mut _ctx: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
    mut key_size: size_t,
) -> ::core::ffi::c_int {
    let mut ctx: *mut aesgcm_context = _ctx as *mut aesgcm_context;
    (*ctx).static_iv = loadn128(iv, PTLS_AESGCM_IV_SIZE as size_t);
    (*ctx).static_iv = _mm_shuffle_epi8(
        (*ctx).static_iv,
        *(&raw const byteswap_ as *const uint8_t as *mut __m128i),
    );
    if key.is_null() {
        return 0 as ::core::ffi::c_int;
    }
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
    (*ctx).super_0.do_encrypt_init = Some(
        aead_do_encrypt_init
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
        aead_do_encrypt_update
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
        aead_do_encrypt_final
            as unsafe extern "C" fn(*mut ptls_aead_context_t, *mut ::core::ffi::c_void) -> size_t,
    )
        as Option<
            unsafe extern "C" fn(*mut st_ptls_aead_context_t, *mut ::core::ffi::c_void) -> size_t,
        >;
    (*ctx).super_0.do_encrypt = Some(
        aead_do_encrypt
            as unsafe extern "C" fn(
                *mut st_ptls_aead_context_t,
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
        aead_do_encrypt_v
            as unsafe extern "C" fn(
                *mut st_ptls_aead_context_t,
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
    (*ctx).super_0.do_decrypt = Some(
        aead_do_decrypt
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
    (*ctx).aesgcm = new_aesgcm(key, key_size, 1500 as size_t, 0 as ::core::ffi::c_int);
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn aes128gcm_setup(
    mut ctx: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return aesgcm_setup(ctx, is_enc, key, iv, PTLS_AES128_KEY_SIZE as size_t);
}
unsafe extern "C" fn aes256gcm_setup(
    mut ctx: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return aesgcm_setup(ctx, is_enc, key, iv, PTLS_AES256_KEY_SIZE as size_t);
}
#[no_mangle]
pub static mut ptls_fusion_can_aesni256: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub static mut ptls_fusion_aes128ctr: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"AES128-CTR\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: PTLS_AES128_KEY_SIZE as size_t,
        block_size: 1 as size_t,
        iv_size: PTLS_AES_IV_SIZE as size_t,
        context_size: ::core::mem::size_of::<ctr_context>() as size_t,
        setup_crypto: Some(
            aes128ctr_setup
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
#[no_mangle]
pub static mut ptls_fusion_aes256ctr: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"AES256-CTR\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: PTLS_AES256_KEY_SIZE as size_t,
        block_size: 1 as size_t,
        iv_size: PTLS_AES_IV_SIZE as size_t,
        context_size: ::core::mem::size_of::<ctr_context>() as size_t,
        setup_crypto: Some(
            aes256ctr_setup
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
#[no_mangle]
pub static mut ptls_fusion_aes128gcm: ptls_aead_algorithm_t = st_ptls_aead_algorithm_t {
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
pub static mut ptls_fusion_aes256gcm: ptls_aead_algorithm_t = st_ptls_aead_algorithm_t {
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
#[inline]
unsafe extern "C" fn calc_total_length(mut input: *mut ptls_iovec_t, mut incnt: size_t) -> size_t {
    let mut totlen: size_t = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i < incnt {
        totlen = totlen.wrapping_add((*input.offset(i as isize)).len);
        i = i.wrapping_add(1);
    }
    return totlen;
}
#[inline]
unsafe extern "C" fn reduce_aad128(
    mut gstate: *mut ptls_fusion_gfmul_state128,
    mut ghash: *mut ptls_fusion_aesgcm_ghash_precompute128,
    mut _aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) {
    let mut ghash_precompute: *mut ptls_fusion_aesgcm_ghash_precompute128 =
        ::core::ptr::null_mut::<ptls_fusion_aesgcm_ghash_precompute128>();
    let mut aad: *const uint8_t = _aad as *const uint8_t;
    while (aadlen >= (6 as ::core::ffi::c_int * 16 as ::core::ffi::c_int) as size_t)
        as ::core::ffi::c_int as ::core::ffi::c_long
        != 0
    {
        ghash_precompute = ghash.offset(6 as ::core::ffi::c_int as isize);
        ghash_precompute = ghash_precompute.offset(-1);
        gfmul_firststep128(
            gstate,
            _mm_loadu_si128(aad as *mut ::core::ffi::c_void as *const __m128i_u),
            ghash_precompute,
        );
        aad = aad.offset(16 as ::core::ffi::c_int as isize);
        aadlen = aadlen.wrapping_sub(16 as size_t);
        let mut i: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
        while i < 6 as ::core::ffi::c_int {
            ghash_precompute = ghash_precompute.offset(-1);
            gfmul_nextstep128(
                gstate,
                _mm_loadu_si128(aad as *mut ::core::ffi::c_void as *const __m128i_u),
                ghash_precompute,
            );
            aad = aad.offset(16 as ::core::ffi::c_int as isize);
            aadlen = aadlen.wrapping_sub(16 as size_t);
            i += 1;
        }
        gfmul_reduce128(gstate);
    }
    if (aadlen != 0 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
        ghash_precompute =
            ghash.offset(aadlen.wrapping_add(15 as size_t).wrapping_div(16 as size_t) as isize);
        if (aadlen >= 16 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
            ghash_precompute = ghash_precompute.offset(-1);
            gfmul_firststep128(
                gstate,
                _mm_loadu_si128(aad as *mut ::core::ffi::c_void as *const __m128i_u),
                ghash_precompute,
            );
            aad = aad.offset(16 as ::core::ffi::c_int as isize);
            aadlen = aadlen.wrapping_sub(16 as size_t);
            while aadlen >= 16 as size_t {
                ghash_precompute = ghash_precompute.offset(-1);
                gfmul_nextstep128(
                    gstate,
                    _mm_loadu_si128(aad as *mut ::core::ffi::c_void as *const __m128i_u),
                    ghash_precompute,
                );
                aad = aad.offset(16 as ::core::ffi::c_int as isize);
                aadlen = aadlen.wrapping_sub(16 as size_t);
            }
            if (aadlen != 0 as size_t) as ::core::ffi::c_int as ::core::ffi::c_long != 0 {
                ghash_precompute = ghash_precompute.offset(-1);
                gfmul_nextstep128(
                    gstate,
                    loadn128(aad as *const ::core::ffi::c_void, aadlen),
                    ghash_precompute,
                );
            }
        } else {
            ghash_precompute = ghash_precompute.offset(-1);
            gfmul_firststep128(
                gstate,
                loadn128(aad as *const ::core::ffi::c_void, aadlen),
                ghash_precompute,
            );
        }
        gfmul_reduce128(gstate);
    }
}
#[inline]
unsafe extern "C" fn load_preceding_unaligned(
    mut encbuf: *mut uint8_t,
    mut output: *mut *mut uint8_t,
) -> *mut uint8_t {
    let mut encp: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    encp = encbuf.offset((*output as uintptr_t & 63 as ::core::ffi::c_int as uintptr_t) as isize);
    if encp != encbuf {
        _mm256_store_si256(
            encbuf as *mut ::core::ffi::c_void as *mut __m256i,
            _mm256_load_si256(
                (*output).offset(-(encp.offset_from(encbuf) as ::core::ffi::c_long as isize))
                    as *mut ::core::ffi::c_void as *const __m256i,
            ),
        );
        _mm256_store_si256(
            encbuf.offset(32 as ::core::ffi::c_int as isize) as *mut ::core::ffi::c_void
                as *mut __m256i,
            _mm256_load_si256(
                (*output)
                    .offset(-(encp.offset_from(encbuf) as ::core::ffi::c_long as isize))
                    .offset(32 as ::core::ffi::c_int as isize)
                    as *mut ::core::ffi::c_void as *const __m256i,
            ),
        );
        *output = (*output).offset(-(encp.offset_from(encbuf) as ::core::ffi::c_long as isize));
    }
    return encp;
}
pub const STATE_EK0_READY_0: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const STATE_COPY_128B: ::core::ffi::c_int = 0x2 as ::core::ffi::c_int;
pub const STATE_EK0_READY: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
pub const STATE_EK0_READY_1: ::core::ffi::c_int = 0x1 as ::core::ffi::c_int;
unsafe extern "C" fn non_temporal_setup(
    mut _ctx: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
    mut key_size: size_t,
) -> ::core::ffi::c_int {
    let mut ctx: *mut aesgcm_context = _ctx as *mut aesgcm_context;
    let mut aesni256: ::core::ffi::c_int =
        (is_enc != 0 && ptls_fusion_can_aesni256 != 0) as ::core::ffi::c_int;
    (*ctx).static_iv = loadn128(iv, PTLS_AESGCM_IV_SIZE as size_t);
    (*ctx).static_iv = _mm_shuffle_epi8(
        (*ctx).static_iv,
        *(&raw const byteswap_ as *const uint8_t as *mut __m128i),
    );
    if key.is_null() {
        return 0 as ::core::ffi::c_int;
    }
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
    (*ctx).super_0.do_encrypt_init = None;
    (*ctx).super_0.do_encrypt_update = None;
    (*ctx).super_0.do_encrypt_final = None;
    if is_enc != 0 {
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
        (*ctx).super_0.do_encrypt_v = (if aesni256 != 0 {
            Some(
                non_temporal_encrypt_v256
                    as unsafe extern "C" fn(
                        *mut st_ptls_aead_context_t,
                        *mut ::core::ffi::c_void,
                        *mut ptls_iovec_t,
                        size_t,
                        uint64_t,
                        *const ::core::ffi::c_void,
                        size_t,
                    ) -> (),
            )
        } else {
            Some(
                non_temporal_encrypt_v128
                    as unsafe extern "C" fn(
                        *mut st_ptls_aead_context_t,
                        *mut ::core::ffi::c_void,
                        *mut ptls_iovec_t,
                        size_t,
                        uint64_t,
                        *const ::core::ffi::c_void,
                        size_t,
                    ) -> (),
            )
        })
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
        (*ctx).super_0.do_encrypt = None;
        (*ctx).super_0.do_encrypt_v = None;
        (*ctx).super_0.do_decrypt = Some(
            non_temporal_decrypt128
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
    (*ctx).aesgcm = new_aesgcm(
        key,
        key_size,
        (7 as ::core::ffi::c_int
            * (if ptls_fusion_can_aesni256 != 0 {
                32 as ::core::ffi::c_int
            } else {
                16 as ::core::ffi::c_int
            })) as size_t,
        aesni256,
    );
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn non_temporal_aes128gcm_setup(
    mut ctx: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return non_temporal_setup(ctx, is_enc, key, iv, PTLS_AES128_KEY_SIZE as size_t);
}
unsafe extern "C" fn non_temporal_aes256gcm_setup(
    mut ctx: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return non_temporal_setup(ctx, is_enc, key, iv, PTLS_AES256_KEY_SIZE as size_t);
}
#[no_mangle]
pub static mut ptls_non_temporal_aes128gcm: ptls_aead_algorithm_t = st_ptls_aead_algorithm_t {
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
pub static mut ptls_non_temporal_aes256gcm: ptls_aead_algorithm_t = st_ptls_aead_algorithm_t {
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
pub unsafe extern "C" fn ptls_fusion_is_supported_by_cpu() -> ::core::ffi::c_int {
    let mut leaf1_ecx: ::core::ffi::c_uint = 0;
    let mut leaf7_ebx: ::core::ffi::c_uint = 0;
    let mut leaf7_ecx: ::core::ffi::c_uint = 0;
    let mut leaf_cnt: ::core::ffi::c_uint = 0;
    asm!(
        "cpuid\n", inlateout("ax") 0 as ::core::ffi::c_int => leaf_cnt, out("ecx") _,
        out("edx") _, options(preserves_flags, pure, readonly, att_syntax)
    );
    if leaf_cnt < 7 as ::core::ffi::c_uint {
        return 0 as ::core::ffi::c_int;
    }
    asm!(
        "cpuid\n", lateout("cx") leaf1_ecx, inlateout("ax") 1 as ::core::ffi::c_int => _,
        out("edx") _, options(preserves_flags, pure, readonly, att_syntax)
    );
    asm!(
        "cpuid\n", "mov {restmp0:x}, %bx\n", restmp0 = lateout(reg) leaf7_ebx,
        inlateout("cx") 0 as ::core::ffi::c_int => leaf7_ecx, inlateout("ax") 7 as
        ::core::ffi::c_int => _, out("edx") _, options(preserves_flags, pure, readonly,
        att_syntax)
    );
    if leaf7_ebx & ((1 as ::core::ffi::c_int) << 5 as ::core::ffi::c_int) as ::core::ffi::c_uint
        == 0 as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if leaf1_ecx & ((1 as ::core::ffi::c_int) << 25 as ::core::ffi::c_int) as ::core::ffi::c_uint
        == 0 as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if leaf1_ecx & ((1 as ::core::ffi::c_int) << 1 as ::core::ffi::c_int) as ::core::ffi::c_uint
        == 0 as ::core::ffi::c_uint
    {
        return 0 as ::core::ffi::c_int;
    }
    if leaf7_ecx & 0x600 as ::core::ffi::c_uint != 0 as ::core::ffi::c_uint
        && ptls_fusion_can_aesni256 == 0
    {
        ptls_fusion_can_aesni256 = 1 as ::core::ffi::c_int;
    }
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    ptls_fusion_aes128gcm = {
        let mut init = st_ptls_aead_algorithm_t {
            non_temporal: [0; 1],
            name: b"AES128-GCM\0".as_ptr() as *const ::core::ffi::c_char,
            confidentiality_limit: PTLS_AESGCM_CONFIDENTIALITY_LIMIT as uint64_t,
            integrity_limit: 0x40000000000000 as uint64_t,
            ctr_cipher: &raw const ptls_fusion_aes128ctr,
            ecb_cipher: ::core::ptr::null::<ptls_cipher_algorithm_t>(),
            key_size: PTLS_AES128_KEY_SIZE as size_t,
            iv_size: PTLS_AESGCM_IV_SIZE as size_t,
            tag_size: PTLS_AESGCM_TAG_SIZE as size_t,
            tls12: C2Rust_Unnamed {
                fixed_iv_size: 0 as size_t,
                record_iv_size: 0,
            },
            align_bits: 0 as uint8_t,
            context_size: ::core::mem::size_of::<aesgcm_context>() as size_t,
            setup_crypto: Some(
                aes128gcm_setup
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
    ptls_fusion_aes256gcm = {
        let mut init = st_ptls_aead_algorithm_t {
            non_temporal: [0; 1],
            name: b"AES256-GCM\0".as_ptr() as *const ::core::ffi::c_char,
            confidentiality_limit: PTLS_AESGCM_CONFIDENTIALITY_LIMIT as uint64_t,
            integrity_limit: 0x40000000000000 as uint64_t,
            ctr_cipher: &raw const ptls_fusion_aes256ctr,
            ecb_cipher: ::core::ptr::null::<ptls_cipher_algorithm_t>(),
            key_size: PTLS_AES256_KEY_SIZE as size_t,
            iv_size: PTLS_AESGCM_IV_SIZE as size_t,
            tag_size: PTLS_AESGCM_TAG_SIZE as size_t,
            tls12: C2Rust_Unnamed {
                fixed_iv_size: 0 as size_t,
                record_iv_size: 0,
            },
            align_bits: 0 as uint8_t,
            context_size: ::core::mem::size_of::<aesgcm_context>() as size_t,
            setup_crypto: Some(
                aes256gcm_setup
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
    ptls_non_temporal_aes128gcm = {
        let mut init = st_ptls_aead_algorithm_t {
            non_temporal: [0; 1],
            name: b"AES128-GCM\0".as_ptr() as *const ::core::ffi::c_char,
            confidentiality_limit: PTLS_AESGCM_CONFIDENTIALITY_LIMIT as uint64_t,
            integrity_limit: 0x40000000000000 as uint64_t,
            ctr_cipher: &raw const ptls_fusion_aes128ctr,
            ecb_cipher: ::core::ptr::null::<ptls_cipher_algorithm_t>(),
            key_size: PTLS_AES128_KEY_SIZE as size_t,
            iv_size: PTLS_AESGCM_IV_SIZE as size_t,
            tag_size: PTLS_AESGCM_TAG_SIZE as size_t,
            tls12: C2Rust_Unnamed {
                fixed_iv_size: PTLS_TLS12_AESGCM_FIXED_IV_SIZE as size_t,
                record_iv_size: PTLS_TLS12_AESGCM_RECORD_IV_SIZE as size_t,
            },
            align_bits: PTLS_X86_CACHE_LINE_ALIGN_BITS as uint8_t,
            context_size: ::core::mem::size_of::<aesgcm_context>() as size_t,
            setup_crypto: Some(
                non_temporal_aes128gcm_setup
                    as unsafe extern "C" fn(
                        *mut ptls_aead_context_t,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        };
        init.set_non_temporal(1 as ::core::ffi::c_uint);
        init
    };
    ptls_non_temporal_aes256gcm = {
        let mut init = st_ptls_aead_algorithm_t {
            non_temporal: [0; 1],
            name: b"AES256-GCM\0".as_ptr() as *const ::core::ffi::c_char,
            confidentiality_limit: PTLS_AESGCM_CONFIDENTIALITY_LIMIT as uint64_t,
            integrity_limit: 0x40000000000000 as uint64_t,
            ctr_cipher: &raw const ptls_fusion_aes256ctr,
            ecb_cipher: ::core::ptr::null::<ptls_cipher_algorithm_t>(),
            key_size: PTLS_AES256_KEY_SIZE as size_t,
            iv_size: PTLS_AESGCM_IV_SIZE as size_t,
            tag_size: PTLS_AESGCM_TAG_SIZE as size_t,
            tls12: C2Rust_Unnamed {
                fixed_iv_size: PTLS_TLS12_AESGCM_FIXED_IV_SIZE as size_t,
                record_iv_size: PTLS_TLS12_AESGCM_RECORD_IV_SIZE as size_t,
            },
            align_bits: PTLS_X86_CACHE_LINE_ALIGN_BITS as uint8_t,
            context_size: ::core::mem::size_of::<aesgcm_context>() as size_t,
            setup_crypto: Some(
                non_temporal_aes256gcm_setup
                    as unsafe extern "C" fn(
                        *mut ptls_aead_context_t,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        };
        init.set_non_temporal(1 as ::core::ffi::c_uint);
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];

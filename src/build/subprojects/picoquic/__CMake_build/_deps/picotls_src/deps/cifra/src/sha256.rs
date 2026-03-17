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
    fn cf_blockwise_accumulate(
        partial: *mut uint8_t,
        npartial: *mut size_t,
        nblock: size_t,
        input: *const ::core::ffi::c_void,
        nbytes: size_t,
        process: cf_blockwise_in_fn,
        ctx: *mut ::core::ffi::c_void,
    );
    fn cf_blockwise_acc_pad(
        partial: *mut uint8_t,
        npartial: *mut size_t,
        nblock: size_t,
        fbyte: uint8_t,
        mbyte: uint8_t,
        lbyte: uint8_t,
        nbytes: size_t,
        process: cf_blockwise_in_fn,
        ctx: *mut ::core::ffi::c_void,
    );
    fn abort() -> !;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type cf_chash_init = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type cf_chash_update = Option<
    unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_void, size_t) -> (),
>;
pub type cf_chash_digest =
    Option<unsafe extern "C" fn(*const ::core::ffi::c_void, *mut uint8_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_chash {
    pub hashsz: size_t,
    pub blocksz: size_t,
    pub init: cf_chash_init,
    pub update: cf_chash_update,
    pub digest: cf_chash_digest,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_sha256_context {
    pub H: [uint32_t; 8],
    pub partial: [uint8_t; 64],
    pub blocks: uint32_t,
    pub npartial: size_t,
}
pub type cf_blockwise_in_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const uint8_t) -> ()>;
pub const CF_SHA224_HASHSZ: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const CF_SHA256_HASHSZ: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const CF_SHA256_BLOCKSZ: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn rotr32(mut x: uint32_t, mut n: ::core::ffi::c_uint) -> uint32_t {
    return x >> n | x << (32 as ::core::ffi::c_uint).wrapping_sub(n);
}
#[inline]
unsafe extern "C" fn read32_be(mut buf: *const uint8_t) -> uint32_t {
    return (*buf.offset(0 as ::core::ffi::c_int as isize) as uint32_t) << 24 as ::core::ffi::c_int
        | (*buf.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 16 as ::core::ffi::c_int
        | (*buf.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int
        | *buf.offset(3 as ::core::ffi::c_int as isize) as uint32_t;
}
#[inline]
unsafe extern "C" fn write32_be(mut v: uint32_t, mut buf: *mut uint8_t) {
    let c2rust_fresh0 = buf;
    buf = buf.offset(1);
    *c2rust_fresh0 = (v >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    let c2rust_fresh1 = buf;
    buf = buf.offset(1);
    *c2rust_fresh1 = (v >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    let c2rust_fresh2 = buf;
    buf = buf.offset(1);
    *c2rust_fresh2 = (v >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    *buf = (v & 0xff as uint32_t) as uint8_t;
}
#[inline]
unsafe extern "C" fn write64_be(mut v: uint64_t, mut buf: *mut uint8_t) {
    let c2rust_fresh3 = buf;
    buf = buf.offset(1);
    *c2rust_fresh3 = (v >> 56 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh4 = buf;
    buf = buf.offset(1);
    *c2rust_fresh4 = (v >> 48 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh5 = buf;
    buf = buf.offset(1);
    *c2rust_fresh5 = (v >> 40 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh6 = buf;
    buf = buf.offset(1);
    *c2rust_fresh6 = (v >> 32 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh7 = buf;
    buf = buf.offset(1);
    *c2rust_fresh7 = (v >> 24 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh8 = buf;
    buf = buf.offset(1);
    *c2rust_fresh8 = (v >> 16 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh9 = buf;
    buf = buf.offset(1);
    *c2rust_fresh9 = (v >> 8 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    *buf = (v & 0xff as uint64_t) as uint8_t;
}
static mut K: [uint32_t; 64] = [
    0x428a2f98 as ::core::ffi::c_int as uint32_t,
    0x71374491 as ::core::ffi::c_int as uint32_t,
    0xb5c0fbcf as ::core::ffi::c_uint,
    0xe9b5dba5 as ::core::ffi::c_uint,
    0x3956c25b as ::core::ffi::c_int as uint32_t,
    0x59f111f1 as ::core::ffi::c_int as uint32_t,
    0x923f82a4 as ::core::ffi::c_uint,
    0xab1c5ed5 as ::core::ffi::c_uint,
    0xd807aa98 as ::core::ffi::c_uint,
    0x12835b01 as ::core::ffi::c_int as uint32_t,
    0x243185be as ::core::ffi::c_int as uint32_t,
    0x550c7dc3 as ::core::ffi::c_int as uint32_t,
    0x72be5d74 as ::core::ffi::c_int as uint32_t,
    0x80deb1fe as ::core::ffi::c_uint,
    0x9bdc06a7 as ::core::ffi::c_uint,
    0xc19bf174 as ::core::ffi::c_uint,
    0xe49b69c1 as ::core::ffi::c_uint,
    0xefbe4786 as ::core::ffi::c_uint,
    0xfc19dc6 as ::core::ffi::c_int as uint32_t,
    0x240ca1cc as ::core::ffi::c_int as uint32_t,
    0x2de92c6f as ::core::ffi::c_int as uint32_t,
    0x4a7484aa as ::core::ffi::c_int as uint32_t,
    0x5cb0a9dc as ::core::ffi::c_int as uint32_t,
    0x76f988da as ::core::ffi::c_int as uint32_t,
    0x983e5152 as ::core::ffi::c_uint,
    0xa831c66d as ::core::ffi::c_uint,
    0xb00327c8 as ::core::ffi::c_uint,
    0xbf597fc7 as ::core::ffi::c_uint,
    0xc6e00bf3 as ::core::ffi::c_uint,
    0xd5a79147 as ::core::ffi::c_uint,
    0x6ca6351 as ::core::ffi::c_int as uint32_t,
    0x14292967 as ::core::ffi::c_int as uint32_t,
    0x27b70a85 as ::core::ffi::c_int as uint32_t,
    0x2e1b2138 as ::core::ffi::c_int as uint32_t,
    0x4d2c6dfc as ::core::ffi::c_int as uint32_t,
    0x53380d13 as ::core::ffi::c_int as uint32_t,
    0x650a7354 as ::core::ffi::c_int as uint32_t,
    0x766a0abb as ::core::ffi::c_int as uint32_t,
    0x81c2c92e as ::core::ffi::c_uint,
    0x92722c85 as ::core::ffi::c_uint,
    0xa2bfe8a1 as ::core::ffi::c_uint,
    0xa81a664b as ::core::ffi::c_uint,
    0xc24b8b70 as ::core::ffi::c_uint,
    0xc76c51a3 as ::core::ffi::c_uint,
    0xd192e819 as ::core::ffi::c_uint,
    0xd6990624 as ::core::ffi::c_uint,
    0xf40e3585 as ::core::ffi::c_uint,
    0x106aa070 as ::core::ffi::c_int as uint32_t,
    0x19a4c116 as ::core::ffi::c_int as uint32_t,
    0x1e376c08 as ::core::ffi::c_int as uint32_t,
    0x2748774c as ::core::ffi::c_int as uint32_t,
    0x34b0bcb5 as ::core::ffi::c_int as uint32_t,
    0x391c0cb3 as ::core::ffi::c_int as uint32_t,
    0x4ed8aa4a as ::core::ffi::c_int as uint32_t,
    0x5b9cca4f as ::core::ffi::c_int as uint32_t,
    0x682e6ff3 as ::core::ffi::c_int as uint32_t,
    0x748f82ee as ::core::ffi::c_int as uint32_t,
    0x78a5636f as ::core::ffi::c_int as uint32_t,
    0x84c87814 as ::core::ffi::c_uint,
    0x8cc70208 as ::core::ffi::c_uint,
    0x90befffa as ::core::ffi::c_uint,
    0xa4506ceb as ::core::ffi::c_uint,
    0xbef9a3f7 as ::core::ffi::c_uint,
    0xc67178f2 as ::core::ffi::c_uint,
];
#[no_mangle]
pub unsafe extern "C" fn cf_sha256_init(mut ctx: *mut cf_sha256_context) {
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cf_sha256_context>() as size_t,
    );
    (*ctx).H[0 as ::core::ffi::c_int as usize] = 0x6a09e667 as uint32_t;
    (*ctx).H[1 as ::core::ffi::c_int as usize] = 0xbb67ae85 as ::core::ffi::c_uint as uint32_t;
    (*ctx).H[2 as ::core::ffi::c_int as usize] = 0x3c6ef372 as uint32_t;
    (*ctx).H[3 as ::core::ffi::c_int as usize] = 0xa54ff53a as ::core::ffi::c_uint as uint32_t;
    (*ctx).H[4 as ::core::ffi::c_int as usize] = 0x510e527f as uint32_t;
    (*ctx).H[5 as ::core::ffi::c_int as usize] = 0x9b05688c as ::core::ffi::c_uint as uint32_t;
    (*ctx).H[6 as ::core::ffi::c_int as usize] = 0x1f83d9ab as uint32_t;
    (*ctx).H[7 as ::core::ffi::c_int as usize] = 0x5be0cd19 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn cf_sha224_init(mut ctx: *mut cf_sha256_context) {
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cf_sha256_context>() as size_t,
    );
    (*ctx).H[0 as ::core::ffi::c_int as usize] = 0xc1059ed8 as ::core::ffi::c_uint as uint32_t;
    (*ctx).H[1 as ::core::ffi::c_int as usize] = 0x367cd507 as uint32_t;
    (*ctx).H[2 as ::core::ffi::c_int as usize] = 0x3070dd17 as uint32_t;
    (*ctx).H[3 as ::core::ffi::c_int as usize] = 0xf70e5939 as ::core::ffi::c_uint as uint32_t;
    (*ctx).H[4 as ::core::ffi::c_int as usize] = 0xffc00b31 as ::core::ffi::c_uint as uint32_t;
    (*ctx).H[5 as ::core::ffi::c_int as usize] = 0x68581511 as uint32_t;
    (*ctx).H[6 as ::core::ffi::c_int as usize] = 0x64f98fa7 as uint32_t;
    (*ctx).H[7 as ::core::ffi::c_int as usize] = 0xbefa4fa4 as ::core::ffi::c_uint as uint32_t;
}
unsafe extern "C" fn sha256_update_block(
    mut vctx: *mut ::core::ffi::c_void,
    mut inp: *const uint8_t,
) {
    let mut ctx: *mut cf_sha256_context = vctx as *mut cf_sha256_context;
    let mut W: [uint32_t; 16] = [0; 16];
    let mut a: uint32_t = (*ctx).H[0 as ::core::ffi::c_int as usize];
    let mut b: uint32_t = (*ctx).H[1 as ::core::ffi::c_int as usize];
    let mut c: uint32_t = (*ctx).H[2 as ::core::ffi::c_int as usize];
    let mut d: uint32_t = (*ctx).H[3 as ::core::ffi::c_int as usize];
    let mut e: uint32_t = (*ctx).H[4 as ::core::ffi::c_int as usize];
    let mut f: uint32_t = (*ctx).H[5 as ::core::ffi::c_int as usize];
    let mut g: uint32_t = (*ctx).H[6 as ::core::ffi::c_int as usize];
    let mut h: uint32_t = (*ctx).H[7 as ::core::ffi::c_int as usize];
    let mut Wt: uint32_t = 0;
    let mut t: size_t = 0;
    t = 0 as size_t;
    while t < 64 as size_t {
        if t < 16 as size_t {
            Wt = read32_be(inp as *const uint8_t);
            W[t as usize] = Wt;
            inp = inp.offset(4 as ::core::ffi::c_int as isize);
        } else {
            Wt = (rotr32(
                W[t.wrapping_sub(2 as size_t).wrapping_rem(16 as size_t) as usize],
                17 as ::core::ffi::c_uint,
            ) ^ rotr32(
                W[t.wrapping_sub(2 as size_t).wrapping_rem(16 as size_t) as usize],
                19 as ::core::ffi::c_uint,
            ) ^ W[t.wrapping_sub(2 as size_t).wrapping_rem(16 as size_t) as usize]
                >> 10 as ::core::ffi::c_int)
                .wrapping_add(W[t.wrapping_sub(7 as size_t).wrapping_rem(16 as size_t) as usize])
                .wrapping_add(
                    rotr32(
                        W[t.wrapping_sub(15 as size_t).wrapping_rem(16 as size_t) as usize],
                        7 as ::core::ffi::c_uint,
                    ) ^ rotr32(
                        W[t.wrapping_sub(15 as size_t).wrapping_rem(16 as size_t) as usize],
                        18 as ::core::ffi::c_uint,
                    ) ^ W[t.wrapping_sub(15 as size_t).wrapping_rem(16 as size_t) as usize]
                        >> 3 as ::core::ffi::c_int,
                )
                .wrapping_add(W[t.wrapping_sub(16 as size_t).wrapping_rem(16 as size_t) as usize]);
            W[t.wrapping_rem(16 as size_t) as usize] = Wt;
        }
        let mut T1: uint32_t = h
            .wrapping_add(
                rotr32(e, 6 as ::core::ffi::c_uint)
                    ^ rotr32(e, 11 as ::core::ffi::c_uint)
                    ^ rotr32(e, 25 as ::core::ffi::c_uint),
            )
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(K[t as usize])
            .wrapping_add(Wt);
        let mut T2: uint32_t = (rotr32(a, 2 as ::core::ffi::c_uint)
            ^ rotr32(a, 13 as ::core::ffi::c_uint)
            ^ rotr32(a, 22 as ::core::ffi::c_uint))
        .wrapping_add(a & b ^ a & c ^ b & c);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(T1);
        d = c;
        c = b;
        b = a;
        a = T1.wrapping_add(T2);
        t = t.wrapping_add(1);
    }
    (*ctx).H[0 as ::core::ffi::c_int as usize] =
        (*ctx).H[0 as ::core::ffi::c_int as usize].wrapping_add(a);
    (*ctx).H[1 as ::core::ffi::c_int as usize] =
        (*ctx).H[1 as ::core::ffi::c_int as usize].wrapping_add(b);
    (*ctx).H[2 as ::core::ffi::c_int as usize] =
        (*ctx).H[2 as ::core::ffi::c_int as usize].wrapping_add(c);
    (*ctx).H[3 as ::core::ffi::c_int as usize] =
        (*ctx).H[3 as ::core::ffi::c_int as usize].wrapping_add(d);
    (*ctx).H[4 as ::core::ffi::c_int as usize] =
        (*ctx).H[4 as ::core::ffi::c_int as usize].wrapping_add(e);
    (*ctx).H[5 as ::core::ffi::c_int as usize] =
        (*ctx).H[5 as ::core::ffi::c_int as usize].wrapping_add(f);
    (*ctx).H[6 as ::core::ffi::c_int as usize] =
        (*ctx).H[6 as ::core::ffi::c_int as usize].wrapping_add(g);
    (*ctx).H[7 as ::core::ffi::c_int as usize] =
        (*ctx).H[7 as ::core::ffi::c_int as usize].wrapping_add(h);
    (*ctx).blocks = (*ctx).blocks.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn cf_sha256_update(
    mut ctx: *mut cf_sha256_context,
    mut data: *const ::core::ffi::c_void,
    mut nbytes: size_t,
) {
    cf_blockwise_accumulate(
        &raw mut (*ctx).partial as *mut uint8_t,
        &raw mut (*ctx).npartial,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
        data,
        nbytes,
        Some(
            sha256_update_block
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const uint8_t) -> (),
        ),
        ctx as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_sha224_update(
    mut ctx: *mut cf_sha256_context,
    mut data: *const ::core::ffi::c_void,
    mut nbytes: size_t,
) {
    cf_sha256_update(ctx, data, nbytes);
}
#[no_mangle]
pub unsafe extern "C" fn cf_sha256_digest(
    mut ctx: *const cf_sha256_context,
    mut hash: *mut uint8_t,
) {
    let mut ours: cf_sha256_context = *ctx;
    cf_sha256_digest_final(&raw mut ours, hash);
}
#[no_mangle]
pub unsafe extern "C" fn cf_sha256_digest_final(
    mut ctx: *mut cf_sha256_context,
    mut hash: *mut uint8_t,
) {
    let mut digested_bytes: uint64_t = (*ctx).blocks as uint64_t;
    digested_bytes = digested_bytes
        .wrapping_mul(CF_SHA256_BLOCKSZ as uint64_t)
        .wrapping_add((*ctx).npartial as uint64_t);
    let mut digested_bits: uint64_t = digested_bytes.wrapping_mul(8 as uint64_t);
    let mut padbytes: size_t = (CF_SHA256_BLOCKSZ as size_t).wrapping_sub(
        (digested_bytes as size_t)
            .wrapping_add(8 as size_t)
            .wrapping_rem(CF_SHA256_BLOCKSZ as size_t),
    );
    cf_blockwise_acc_pad(
        &raw mut (*ctx).partial as *mut uint8_t,
        &raw mut (*ctx).npartial,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
        0x80 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        padbytes,
        Some(
            sha256_update_block
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const uint8_t) -> (),
        ),
        ctx as *mut ::core::ffi::c_void,
    );
    let mut buf: [uint8_t; 8] = [0; 8];
    write64_be(digested_bits, &raw mut buf as *mut uint8_t);
    cf_sha256_update(
        ctx,
        &raw mut buf as *mut uint8_t as *const ::core::ffi::c_void,
        8 as size_t,
    );
    if !((*ctx).npartial == 0 as size_t) {
        abort();
    }
    write32_be(
        (*ctx).H[0 as ::core::ffi::c_int as usize],
        hash.offset(0 as ::core::ffi::c_int as isize),
    );
    write32_be(
        (*ctx).H[1 as ::core::ffi::c_int as usize],
        hash.offset(4 as ::core::ffi::c_int as isize),
    );
    write32_be(
        (*ctx).H[2 as ::core::ffi::c_int as usize],
        hash.offset(8 as ::core::ffi::c_int as isize),
    );
    write32_be(
        (*ctx).H[3 as ::core::ffi::c_int as usize],
        hash.offset(12 as ::core::ffi::c_int as isize),
    );
    write32_be(
        (*ctx).H[4 as ::core::ffi::c_int as usize],
        hash.offset(16 as ::core::ffi::c_int as isize),
    );
    write32_be(
        (*ctx).H[5 as ::core::ffi::c_int as usize],
        hash.offset(20 as ::core::ffi::c_int as isize),
    );
    write32_be(
        (*ctx).H[6 as ::core::ffi::c_int as usize],
        hash.offset(24 as ::core::ffi::c_int as isize),
    );
    write32_be(
        (*ctx).H[7 as ::core::ffi::c_int as usize],
        hash.offset(28 as ::core::ffi::c_int as isize),
    );
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cf_sha256_context>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_sha224_digest(
    mut ctx: *const cf_sha256_context,
    mut hash: *mut uint8_t,
) {
    let mut full: [uint8_t; 32] = [0; 32];
    cf_sha256_digest(ctx, &raw mut full as *mut uint8_t);
    memcpy(
        hash as *mut ::core::ffi::c_void,
        &raw mut full as *mut uint8_t as *const ::core::ffi::c_void,
        CF_SHA224_HASHSZ as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_sha224_digest_final(
    mut ctx: *mut cf_sha256_context,
    mut hash: *mut uint8_t,
) {
    let mut full: [uint8_t; 32] = [0; 32];
    cf_sha256_digest_final(ctx, &raw mut full as *mut uint8_t);
    memcpy(
        hash as *mut ::core::ffi::c_void,
        &raw mut full as *mut uint8_t as *const ::core::ffi::c_void,
        CF_SHA224_HASHSZ as size_t,
    );
}
#[no_mangle]
pub static mut cf_sha224: cf_chash = unsafe {
    cf_chash {
        hashsz: CF_SHA224_HASHSZ as size_t,
        blocksz: CF_SHA256_BLOCKSZ as size_t,
        init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut cf_sha256_context) -> ()>,
            cf_chash_init,
        >(Some(
            cf_sha224_init as unsafe extern "C" fn(*mut cf_sha256_context) -> (),
        )),
        update: ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut cf_sha256_context,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
            >,
            cf_chash_update,
        >(Some(
            cf_sha224_update
                as unsafe extern "C" fn(
                    *mut cf_sha256_context,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        )),
        digest: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const cf_sha256_context, *mut uint8_t) -> ()>,
            cf_chash_digest,
        >(Some(
            cf_sha224_digest as unsafe extern "C" fn(*const cf_sha256_context, *mut uint8_t) -> (),
        )),
    }
};
#[no_mangle]
pub static mut cf_sha256: cf_chash = unsafe {
    cf_chash {
        hashsz: CF_SHA256_HASHSZ as size_t,
        blocksz: CF_SHA256_BLOCKSZ as size_t,
        init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut cf_sha256_context) -> ()>,
            cf_chash_init,
        >(Some(
            cf_sha256_init as unsafe extern "C" fn(*mut cf_sha256_context) -> (),
        )),
        update: ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut cf_sha256_context,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
            >,
            cf_chash_update,
        >(Some(
            cf_sha256_update
                as unsafe extern "C" fn(
                    *mut cf_sha256_context,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        )),
        digest: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const cf_sha256_context, *mut uint8_t) -> ()>,
            cf_chash_digest,
        >(Some(
            cf_sha256_digest as unsafe extern "C" fn(*const cf_sha256_context, *mut uint8_t) -> (),
        )),
    }
};

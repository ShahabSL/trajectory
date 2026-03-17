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
pub struct cf_sha512_context {
    pub H: [uint64_t; 8],
    pub partial: [uint8_t; 128],
    pub blocks: uint32_t,
    pub npartial: size_t,
}
pub type cf_blockwise_in_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const uint8_t) -> ()>;
pub const CF_SHA384_HASHSZ: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
pub const CF_SHA384_BLOCKSZ: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const CF_SHA512_HASHSZ: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const CF_SHA512_BLOCKSZ: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn rotr64(mut x: uint64_t, mut n: ::core::ffi::c_uint) -> uint64_t {
    return x >> n | x << (64 as ::core::ffi::c_uint).wrapping_sub(n);
}
#[inline]
unsafe extern "C" fn read32_be(mut buf: *const uint8_t) -> uint32_t {
    return (*buf.offset(0 as ::core::ffi::c_int as isize) as uint32_t) << 24 as ::core::ffi::c_int
        | (*buf.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 16 as ::core::ffi::c_int
        | (*buf.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int
        | *buf.offset(3 as ::core::ffi::c_int as isize) as uint32_t;
}
#[inline]
unsafe extern "C" fn read64_be(mut buf: *const uint8_t) -> uint64_t {
    let mut hi: uint32_t = read32_be(buf as *const uint8_t);
    let mut lo: uint32_t = read32_be(buf.offset(4 as ::core::ffi::c_int as isize));
    return (hi as uint64_t) << 32 as ::core::ffi::c_int | lo as uint64_t;
}
#[inline]
unsafe extern "C" fn write64_be(mut v: uint64_t, mut buf: *mut uint8_t) {
    let c2rust_fresh0 = buf;
    buf = buf.offset(1);
    *c2rust_fresh0 = (v >> 56 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh1 = buf;
    buf = buf.offset(1);
    *c2rust_fresh1 = (v >> 48 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh2 = buf;
    buf = buf.offset(1);
    *c2rust_fresh2 = (v >> 40 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh3 = buf;
    buf = buf.offset(1);
    *c2rust_fresh3 = (v >> 32 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh4 = buf;
    buf = buf.offset(1);
    *c2rust_fresh4 = (v >> 24 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh5 = buf;
    buf = buf.offset(1);
    *c2rust_fresh5 = (v >> 16 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    let c2rust_fresh6 = buf;
    buf = buf.offset(1);
    *c2rust_fresh6 = (v >> 8 as ::core::ffi::c_int & 0xff as uint64_t) as uint8_t;
    *buf = (v & 0xff as uint64_t) as uint8_t;
}
static mut K: [uint64_t; 80] = [
    0x428a2f98d728ae22 as ::core::ffi::c_ulong,
    0x7137449123ef65cd as ::core::ffi::c_ulong,
    0xb5c0fbcfec4d3b2f as ::core::ffi::c_ulong,
    0xe9b5dba58189dbbc as ::core::ffi::c_ulong,
    0x3956c25bf348b538 as ::core::ffi::c_ulong,
    0x59f111f1b605d019 as ::core::ffi::c_ulong,
    0x923f82a4af194f9b as ::core::ffi::c_ulong,
    0xab1c5ed5da6d8118 as ::core::ffi::c_ulong,
    0xd807aa98a3030242 as ::core::ffi::c_ulong,
    0x12835b0145706fbe as ::core::ffi::c_ulong,
    0x243185be4ee4b28c as ::core::ffi::c_ulong,
    0x550c7dc3d5ffb4e2 as ::core::ffi::c_ulong,
    0x72be5d74f27b896f as ::core::ffi::c_ulong,
    0x80deb1fe3b1696b1 as ::core::ffi::c_ulong,
    0x9bdc06a725c71235 as ::core::ffi::c_ulong,
    0xc19bf174cf692694 as ::core::ffi::c_ulong,
    0xe49b69c19ef14ad2 as ::core::ffi::c_ulong,
    0xefbe4786384f25e3 as ::core::ffi::c_ulong,
    0xfc19dc68b8cd5b5 as ::core::ffi::c_ulong,
    0x240ca1cc77ac9c65 as ::core::ffi::c_ulong,
    0x2de92c6f592b0275 as ::core::ffi::c_ulong,
    0x4a7484aa6ea6e483 as ::core::ffi::c_ulong,
    0x5cb0a9dcbd41fbd4 as ::core::ffi::c_ulong,
    0x76f988da831153b5 as ::core::ffi::c_ulong,
    0x983e5152ee66dfab as ::core::ffi::c_ulong,
    0xa831c66d2db43210 as ::core::ffi::c_ulong,
    0xb00327c898fb213f as ::core::ffi::c_ulong,
    0xbf597fc7beef0ee4 as ::core::ffi::c_ulong,
    0xc6e00bf33da88fc2 as ::core::ffi::c_ulong,
    0xd5a79147930aa725 as ::core::ffi::c_ulong,
    0x6ca6351e003826f as ::core::ffi::c_ulong,
    0x142929670a0e6e70 as ::core::ffi::c_ulong,
    0x27b70a8546d22ffc as ::core::ffi::c_ulong,
    0x2e1b21385c26c926 as ::core::ffi::c_ulong,
    0x4d2c6dfc5ac42aed as ::core::ffi::c_ulong,
    0x53380d139d95b3df as ::core::ffi::c_ulong,
    0x650a73548baf63de as ::core::ffi::c_ulong,
    0x766a0abb3c77b2a8 as ::core::ffi::c_ulong,
    0x81c2c92e47edaee6 as ::core::ffi::c_ulong,
    0x92722c851482353b as ::core::ffi::c_ulong,
    0xa2bfe8a14cf10364 as ::core::ffi::c_ulong,
    0xa81a664bbc423001 as ::core::ffi::c_ulong,
    0xc24b8b70d0f89791 as ::core::ffi::c_ulong,
    0xc76c51a30654be30 as ::core::ffi::c_ulong,
    0xd192e819d6ef5218 as ::core::ffi::c_ulong,
    0xd69906245565a910 as ::core::ffi::c_ulong,
    0xf40e35855771202a as ::core::ffi::c_ulong,
    0x106aa07032bbd1b8 as ::core::ffi::c_ulong,
    0x19a4c116b8d2d0c8 as ::core::ffi::c_ulong,
    0x1e376c085141ab53 as ::core::ffi::c_ulong,
    0x2748774cdf8eeb99 as ::core::ffi::c_ulong,
    0x34b0bcb5e19b48a8 as ::core::ffi::c_ulong,
    0x391c0cb3c5c95a63 as ::core::ffi::c_ulong,
    0x4ed8aa4ae3418acb as ::core::ffi::c_ulong,
    0x5b9cca4f7763e373 as ::core::ffi::c_ulong,
    0x682e6ff3d6b2b8a3 as ::core::ffi::c_ulong,
    0x748f82ee5defb2fc as ::core::ffi::c_ulong,
    0x78a5636f43172f60 as ::core::ffi::c_ulong,
    0x84c87814a1f0ab72 as ::core::ffi::c_ulong,
    0x8cc702081a6439ec as ::core::ffi::c_ulong,
    0x90befffa23631e28 as ::core::ffi::c_ulong,
    0xa4506cebde82bde9 as ::core::ffi::c_ulong,
    0xbef9a3f7b2c67915 as ::core::ffi::c_ulong,
    0xc67178f2e372532b as ::core::ffi::c_ulong,
    0xca273eceea26619c as ::core::ffi::c_ulong,
    0xd186b8c721c0c207 as ::core::ffi::c_ulong,
    0xeada7dd6cde0eb1e as ::core::ffi::c_ulong,
    0xf57d4f7fee6ed178 as ::core::ffi::c_ulong,
    0x6f067aa72176fba as ::core::ffi::c_ulong,
    0xa637dc5a2c898a6 as ::core::ffi::c_ulong,
    0x113f9804bef90dae as ::core::ffi::c_ulong,
    0x1b710b35131c471b as ::core::ffi::c_ulong,
    0x28db77f523047d84 as ::core::ffi::c_ulong,
    0x32caab7b40c72493 as ::core::ffi::c_ulong,
    0x3c9ebe0a15c9bebc as ::core::ffi::c_ulong,
    0x431d67c49c100d4c as ::core::ffi::c_ulong,
    0x4cc5d4becb3e42b6 as ::core::ffi::c_ulong,
    0x597f299cfc657e2a as ::core::ffi::c_ulong,
    0x5fcb6fab3ad6faec as ::core::ffi::c_ulong,
    0x6c44198c4a475817 as ::core::ffi::c_ulong,
];
#[no_mangle]
pub unsafe extern "C" fn cf_sha512_init(mut ctx: *mut cf_sha512_context) {
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cf_sha512_context>() as size_t,
    );
    (*ctx).H[0 as ::core::ffi::c_int as usize] =
        0x6a09e667f3bcc908 as ::core::ffi::c_ulong as uint64_t;
    (*ctx).H[1 as ::core::ffi::c_int as usize] =
        0xbb67ae8584caa73b as ::core::ffi::c_ulong as uint64_t;
    (*ctx).H[2 as ::core::ffi::c_int as usize] =
        0x3c6ef372fe94f82b as ::core::ffi::c_ulong as uint64_t;
    (*ctx).H[3 as ::core::ffi::c_int as usize] =
        0xa54ff53a5f1d36f1 as ::core::ffi::c_ulong as uint64_t;
    (*ctx).H[4 as ::core::ffi::c_int as usize] =
        0x510e527fade682d1 as ::core::ffi::c_ulong as uint64_t;
    (*ctx).H[5 as ::core::ffi::c_int as usize] =
        0x9b05688c2b3e6c1f as ::core::ffi::c_ulong as uint64_t;
    (*ctx).H[6 as ::core::ffi::c_int as usize] =
        0x1f83d9abfb41bd6b as ::core::ffi::c_ulong as uint64_t;
    (*ctx).H[7 as ::core::ffi::c_int as usize] =
        0x5be0cd19137e2179 as ::core::ffi::c_ulong as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn cf_sha384_init(mut ctx: *mut cf_sha512_context) {
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cf_sha512_context>() as size_t,
    );
    (*ctx).H[0 as ::core::ffi::c_int as usize] =
        0xcbbb9d5dc1059ed8 as ::core::ffi::c_ulong as uint64_t;
    (*ctx).H[1 as ::core::ffi::c_int as usize] =
        0x629a292a367cd507 as ::core::ffi::c_ulong as uint64_t;
    (*ctx).H[2 as ::core::ffi::c_int as usize] =
        0x9159015a3070dd17 as ::core::ffi::c_ulong as uint64_t;
    (*ctx).H[3 as ::core::ffi::c_int as usize] =
        0x152fecd8f70e5939 as ::core::ffi::c_ulong as uint64_t;
    (*ctx).H[4 as ::core::ffi::c_int as usize] =
        0x67332667ffc00b31 as ::core::ffi::c_ulong as uint64_t;
    (*ctx).H[5 as ::core::ffi::c_int as usize] =
        0x8eb44a8768581511 as ::core::ffi::c_ulong as uint64_t;
    (*ctx).H[6 as ::core::ffi::c_int as usize] =
        0xdb0c2e0d64f98fa7 as ::core::ffi::c_ulong as uint64_t;
    (*ctx).H[7 as ::core::ffi::c_int as usize] =
        0x47b5481dbefa4fa4 as ::core::ffi::c_ulong as uint64_t;
}
unsafe extern "C" fn sha512_update_block(
    mut vctx: *mut ::core::ffi::c_void,
    mut inp: *const uint8_t,
) {
    let mut ctx: *mut cf_sha512_context = vctx as *mut cf_sha512_context;
    let mut W: [uint64_t; 16] = [0; 16];
    let mut a: uint64_t = (*ctx).H[0 as ::core::ffi::c_int as usize];
    let mut b: uint64_t = (*ctx).H[1 as ::core::ffi::c_int as usize];
    let mut c: uint64_t = (*ctx).H[2 as ::core::ffi::c_int as usize];
    let mut d: uint64_t = (*ctx).H[3 as ::core::ffi::c_int as usize];
    let mut e: uint64_t = (*ctx).H[4 as ::core::ffi::c_int as usize];
    let mut f: uint64_t = (*ctx).H[5 as ::core::ffi::c_int as usize];
    let mut g: uint64_t = (*ctx).H[6 as ::core::ffi::c_int as usize];
    let mut h: uint64_t = (*ctx).H[7 as ::core::ffi::c_int as usize];
    let mut Wt: uint64_t = 0;
    let mut t: size_t = 0;
    t = 0 as size_t;
    while t < 80 as size_t {
        if t < 16 as size_t {
            Wt = read64_be(inp as *const uint8_t);
            W[t as usize] = Wt;
            inp = inp.offset(8 as ::core::ffi::c_int as isize);
        } else {
            Wt = (rotr64(
                W[t.wrapping_sub(2 as size_t).wrapping_rem(16 as size_t) as usize],
                19 as ::core::ffi::c_uint,
            ) ^ rotr64(
                W[t.wrapping_sub(2 as size_t).wrapping_rem(16 as size_t) as usize],
                61 as ::core::ffi::c_uint,
            ) ^ W[t.wrapping_sub(2 as size_t).wrapping_rem(16 as size_t) as usize]
                >> 6 as ::core::ffi::c_int)
                .wrapping_add(W[t.wrapping_sub(7 as size_t).wrapping_rem(16 as size_t) as usize])
                .wrapping_add(
                    rotr64(
                        W[t.wrapping_sub(15 as size_t).wrapping_rem(16 as size_t) as usize],
                        1 as ::core::ffi::c_uint,
                    ) ^ rotr64(
                        W[t.wrapping_sub(15 as size_t).wrapping_rem(16 as size_t) as usize],
                        8 as ::core::ffi::c_uint,
                    ) ^ W[t.wrapping_sub(15 as size_t).wrapping_rem(16 as size_t) as usize]
                        >> 7 as ::core::ffi::c_int,
                )
                .wrapping_add(W[t.wrapping_sub(16 as size_t).wrapping_rem(16 as size_t) as usize]);
            W[t.wrapping_rem(16 as size_t) as usize] = Wt;
        }
        let mut T1: uint64_t = h
            .wrapping_add(
                rotr64(e, 14 as ::core::ffi::c_uint)
                    ^ rotr64(e, 18 as ::core::ffi::c_uint)
                    ^ rotr64(e, 41 as ::core::ffi::c_uint),
            )
            .wrapping_add(e & f ^ !e & g)
            .wrapping_add(K[t as usize])
            .wrapping_add(Wt);
        let mut T2: uint64_t = (rotr64(a, 28 as ::core::ffi::c_uint)
            ^ rotr64(a, 34 as ::core::ffi::c_uint)
            ^ rotr64(a, 39 as ::core::ffi::c_uint))
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
pub unsafe extern "C" fn cf_sha512_update(
    mut ctx: *mut cf_sha512_context,
    mut data: *const ::core::ffi::c_void,
    mut nbytes: size_t,
) {
    cf_blockwise_accumulate(
        &raw mut (*ctx).partial as *mut uint8_t,
        &raw mut (*ctx).npartial,
        ::core::mem::size_of::<[uint8_t; 128]>() as size_t,
        data,
        nbytes,
        Some(
            sha512_update_block
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const uint8_t) -> (),
        ),
        ctx as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_sha384_update(
    mut ctx: *mut cf_sha512_context,
    mut data: *const ::core::ffi::c_void,
    mut nbytes: size_t,
) {
    cf_sha512_update(ctx, data, nbytes);
}
#[no_mangle]
pub unsafe extern "C" fn cf_sha512_digest(
    mut ctx: *const cf_sha512_context,
    mut hash: *mut uint8_t,
) {
    let mut ours: cf_sha512_context = *ctx;
    cf_sha512_digest_final(&raw mut ours, hash);
}
#[no_mangle]
pub unsafe extern "C" fn cf_sha512_digest_final(
    mut ctx: *mut cf_sha512_context,
    mut hash: *mut uint8_t,
) {
    let mut digested_bytes: uint64_t = (*ctx).blocks as uint64_t;
    digested_bytes = digested_bytes
        .wrapping_mul(CF_SHA512_BLOCKSZ as uint64_t)
        .wrapping_add((*ctx).npartial as uint64_t);
    let mut digested_bits: uint64_t = digested_bytes.wrapping_mul(8 as uint64_t);
    let mut padbytes: size_t = (CF_SHA512_BLOCKSZ as size_t).wrapping_sub(
        (digested_bytes as size_t)
            .wrapping_add(16 as size_t)
            .wrapping_rem(CF_SHA512_BLOCKSZ as size_t),
    );
    cf_blockwise_acc_pad(
        &raw mut (*ctx).partial as *mut uint8_t,
        &raw mut (*ctx).npartial,
        ::core::mem::size_of::<[uint8_t; 128]>() as size_t,
        0x80 as uint8_t,
        0 as uint8_t,
        0 as uint8_t,
        padbytes,
        Some(
            sha512_update_block
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const uint8_t) -> (),
        ),
        ctx as *mut ::core::ffi::c_void,
    );
    let mut buf: [uint8_t; 8] = [0; 8];
    write64_be(0 as uint64_t, &raw mut buf as *mut uint8_t);
    cf_sha512_update(
        ctx,
        &raw mut buf as *mut uint8_t as *const ::core::ffi::c_void,
        8 as size_t,
    );
    write64_be(digested_bits, &raw mut buf as *mut uint8_t);
    cf_sha512_update(
        ctx,
        &raw mut buf as *mut uint8_t as *const ::core::ffi::c_void,
        8 as size_t,
    );
    if !((*ctx).npartial == 0 as size_t) {
        abort();
    }
    write64_be(
        (*ctx).H[0 as ::core::ffi::c_int as usize],
        hash.offset(0 as ::core::ffi::c_int as isize),
    );
    write64_be(
        (*ctx).H[1 as ::core::ffi::c_int as usize],
        hash.offset(8 as ::core::ffi::c_int as isize),
    );
    write64_be(
        (*ctx).H[2 as ::core::ffi::c_int as usize],
        hash.offset(16 as ::core::ffi::c_int as isize),
    );
    write64_be(
        (*ctx).H[3 as ::core::ffi::c_int as usize],
        hash.offset(24 as ::core::ffi::c_int as isize),
    );
    write64_be(
        (*ctx).H[4 as ::core::ffi::c_int as usize],
        hash.offset(32 as ::core::ffi::c_int as isize),
    );
    write64_be(
        (*ctx).H[5 as ::core::ffi::c_int as usize],
        hash.offset(40 as ::core::ffi::c_int as isize),
    );
    write64_be(
        (*ctx).H[6 as ::core::ffi::c_int as usize],
        hash.offset(48 as ::core::ffi::c_int as isize),
    );
    write64_be(
        (*ctx).H[7 as ::core::ffi::c_int as usize],
        hash.offset(56 as ::core::ffi::c_int as isize),
    );
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cf_sha512_context>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_sha384_digest(
    mut ctx: *const cf_sha512_context,
    mut hash: *mut uint8_t,
) {
    let mut full: [uint8_t; 64] = [0; 64];
    cf_sha512_digest(ctx, &raw mut full as *mut uint8_t);
    memcpy(
        hash as *mut ::core::ffi::c_void,
        &raw mut full as *mut uint8_t as *const ::core::ffi::c_void,
        CF_SHA384_HASHSZ as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_sha384_digest_final(
    mut ctx: *mut cf_sha512_context,
    mut hash: *mut uint8_t,
) {
    let mut full: [uint8_t; 64] = [0; 64];
    cf_sha512_digest_final(ctx, &raw mut full as *mut uint8_t);
    memcpy(
        hash as *mut ::core::ffi::c_void,
        &raw mut full as *mut uint8_t as *const ::core::ffi::c_void,
        CF_SHA384_HASHSZ as size_t,
    );
}
#[no_mangle]
pub static mut cf_sha384: cf_chash = unsafe {
    cf_chash {
        hashsz: CF_SHA384_HASHSZ as size_t,
        blocksz: CF_SHA384_BLOCKSZ as size_t,
        init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut cf_sha512_context) -> ()>,
            cf_chash_init,
        >(Some(
            cf_sha384_init as unsafe extern "C" fn(*mut cf_sha512_context) -> (),
        )),
        update: ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut cf_sha512_context,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
            >,
            cf_chash_update,
        >(Some(
            cf_sha384_update
                as unsafe extern "C" fn(
                    *mut cf_sha512_context,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        )),
        digest: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const cf_sha512_context, *mut uint8_t) -> ()>,
            cf_chash_digest,
        >(Some(
            cf_sha384_digest as unsafe extern "C" fn(*const cf_sha512_context, *mut uint8_t) -> (),
        )),
    }
};
#[no_mangle]
pub static mut cf_sha512: cf_chash = unsafe {
    cf_chash {
        hashsz: CF_SHA512_HASHSZ as size_t,
        blocksz: CF_SHA512_BLOCKSZ as size_t,
        init: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut cf_sha512_context) -> ()>,
            cf_chash_init,
        >(Some(
            cf_sha512_init as unsafe extern "C" fn(*mut cf_sha512_context) -> (),
        )),
        update: ::core::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    *mut cf_sha512_context,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
            >,
            cf_chash_update,
        >(Some(
            cf_sha512_update
                as unsafe extern "C" fn(
                    *mut cf_sha512_context,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        )),
        digest: ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*const cf_sha512_context, *mut uint8_t) -> ()>,
            cf_chash_digest,
        >(Some(
            cf_sha512_digest as unsafe extern "C" fn(*const cf_sha512_context, *mut uint8_t) -> (),
        )),
    }
};

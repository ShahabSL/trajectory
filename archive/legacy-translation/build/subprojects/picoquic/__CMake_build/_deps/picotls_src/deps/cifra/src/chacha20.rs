extern "C" {
    fn cf_blockwise_xor(
        partial: *mut uint8_t,
        npartial: *mut size_t,
        nblock: size_t,
        input: *const ::core::ffi::c_void,
        output: *mut ::core::ffi::c_void,
        nbytes: size_t,
        newblock: cf_blockwise_out_fn,
        ctx: *mut ::core::ffi::c_void,
    );
    fn abort() -> !;
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
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type size_t = usize;
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
pub type cf_blockwise_out_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>;
#[inline]
unsafe extern "C" fn rotl32(mut x: uint32_t, mut n: ::core::ffi::c_uint) -> uint32_t {
    return x << n | x >> (32 as ::core::ffi::c_uint).wrapping_sub(n);
}
#[inline]
unsafe extern "C" fn read32_le(mut buf: *const uint8_t) -> uint32_t {
    return (*buf.offset(3 as ::core::ffi::c_int as isize) as uint32_t) << 24 as ::core::ffi::c_int
        | (*buf.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 16 as ::core::ffi::c_int
        | (*buf.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int
        | *buf.offset(0 as ::core::ffi::c_int as isize) as uint32_t;
}
#[inline]
unsafe extern "C" fn write32_le(mut v: uint32_t, mut buf: *mut uint8_t) {
    let c2rust_fresh0 = buf;
    buf = buf.offset(1);
    *c2rust_fresh0 = (v & 0xff as uint32_t) as uint8_t;
    let c2rust_fresh1 = buf;
    buf = buf.offset(1);
    *c2rust_fresh1 = (v >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    let c2rust_fresh2 = buf;
    buf = buf.offset(1);
    *c2rust_fresh2 = (v >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    *buf = (v >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
}
#[inline]
unsafe extern "C" fn incr_le(mut v: *mut uint8_t, mut len: size_t) {
    let mut i: size_t = 0 as size_t;
    loop {
        let ref mut c2rust_fresh3 = *v.offset(i as isize);
        *c2rust_fresh3 = (*c2rust_fresh3).wrapping_add(1);
        if *c2rust_fresh3 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            return;
        }
        i = i.wrapping_add(1);
        if i == len {
            return;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn cf_chacha20_core(
    mut key0: *const uint8_t,
    mut key1: *const uint8_t,
    mut nonce: *const uint8_t,
    mut constant: *const uint8_t,
    mut out: *mut uint8_t,
) {
    let mut z0: uint32_t = 0;
    let mut z1: uint32_t = 0;
    let mut z2: uint32_t = 0;
    let mut z3: uint32_t = 0;
    let mut z4: uint32_t = 0;
    let mut z5: uint32_t = 0;
    let mut z6: uint32_t = 0;
    let mut z7: uint32_t = 0;
    let mut z8: uint32_t = 0;
    let mut z9: uint32_t = 0;
    let mut za: uint32_t = 0;
    let mut zb: uint32_t = 0;
    let mut zc: uint32_t = 0;
    let mut zd: uint32_t = 0;
    let mut ze: uint32_t = 0;
    let mut zf: uint32_t = 0;
    z0 = read32_le(constant.offset(0 as ::core::ffi::c_int as isize));
    let mut x0: uint32_t = z0;
    z1 = read32_le(constant.offset(4 as ::core::ffi::c_int as isize));
    let mut x1: uint32_t = z1;
    z2 = read32_le(constant.offset(8 as ::core::ffi::c_int as isize));
    let mut x2: uint32_t = z2;
    z3 = read32_le(constant.offset(12 as ::core::ffi::c_int as isize));
    let mut x3: uint32_t = z3;
    z4 = read32_le(key0.offset(0 as ::core::ffi::c_int as isize));
    let mut x4: uint32_t = z4;
    z5 = read32_le(key0.offset(4 as ::core::ffi::c_int as isize));
    let mut x5: uint32_t = z5;
    z6 = read32_le(key0.offset(8 as ::core::ffi::c_int as isize));
    let mut x6: uint32_t = z6;
    z7 = read32_le(key0.offset(12 as ::core::ffi::c_int as isize));
    let mut x7: uint32_t = z7;
    z8 = read32_le(key1.offset(0 as ::core::ffi::c_int as isize));
    let mut x8: uint32_t = z8;
    z9 = read32_le(key1.offset(4 as ::core::ffi::c_int as isize));
    let mut x9: uint32_t = z9;
    za = read32_le(key1.offset(8 as ::core::ffi::c_int as isize));
    let mut xa: uint32_t = za;
    zb = read32_le(key1.offset(12 as ::core::ffi::c_int as isize));
    let mut xb: uint32_t = zb;
    zc = read32_le(nonce.offset(0 as ::core::ffi::c_int as isize));
    let mut xc: uint32_t = zc;
    zd = read32_le(nonce.offset(4 as ::core::ffi::c_int as isize));
    let mut xd: uint32_t = zd;
    ze = read32_le(nonce.offset(8 as ::core::ffi::c_int as isize));
    let mut xe: uint32_t = ze;
    zf = read32_le(nonce.offset(12 as ::core::ffi::c_int as isize));
    let mut xf: uint32_t = zf;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 10 as ::core::ffi::c_int {
        z0 = z0.wrapping_add(z4);
        zc = rotl32(zc ^ z0, 16 as ::core::ffi::c_uint);
        z8 = z8.wrapping_add(zc);
        z4 = rotl32(z4 ^ z8, 12 as ::core::ffi::c_uint);
        z0 = z0.wrapping_add(z4);
        zc = rotl32(zc ^ z0, 8 as ::core::ffi::c_uint);
        z8 = z8.wrapping_add(zc);
        z4 = rotl32(z4 ^ z8, 7 as ::core::ffi::c_uint);
        z1 = z1.wrapping_add(z5);
        zd = rotl32(zd ^ z1, 16 as ::core::ffi::c_uint);
        z9 = z9.wrapping_add(zd);
        z5 = rotl32(z5 ^ z9, 12 as ::core::ffi::c_uint);
        z1 = z1.wrapping_add(z5);
        zd = rotl32(zd ^ z1, 8 as ::core::ffi::c_uint);
        z9 = z9.wrapping_add(zd);
        z5 = rotl32(z5 ^ z9, 7 as ::core::ffi::c_uint);
        z2 = z2.wrapping_add(z6);
        ze = rotl32(ze ^ z2, 16 as ::core::ffi::c_uint);
        za = za.wrapping_add(ze);
        z6 = rotl32(z6 ^ za, 12 as ::core::ffi::c_uint);
        z2 = z2.wrapping_add(z6);
        ze = rotl32(ze ^ z2, 8 as ::core::ffi::c_uint);
        za = za.wrapping_add(ze);
        z6 = rotl32(z6 ^ za, 7 as ::core::ffi::c_uint);
        z3 = z3.wrapping_add(z7);
        zf = rotl32(zf ^ z3, 16 as ::core::ffi::c_uint);
        zb = zb.wrapping_add(zf);
        z7 = rotl32(z7 ^ zb, 12 as ::core::ffi::c_uint);
        z3 = z3.wrapping_add(z7);
        zf = rotl32(zf ^ z3, 8 as ::core::ffi::c_uint);
        zb = zb.wrapping_add(zf);
        z7 = rotl32(z7 ^ zb, 7 as ::core::ffi::c_uint);
        z0 = z0.wrapping_add(z5);
        zf = rotl32(zf ^ z0, 16 as ::core::ffi::c_uint);
        za = za.wrapping_add(zf);
        z5 = rotl32(z5 ^ za, 12 as ::core::ffi::c_uint);
        z0 = z0.wrapping_add(z5);
        zf = rotl32(zf ^ z0, 8 as ::core::ffi::c_uint);
        za = za.wrapping_add(zf);
        z5 = rotl32(z5 ^ za, 7 as ::core::ffi::c_uint);
        z1 = z1.wrapping_add(z6);
        zc = rotl32(zc ^ z1, 16 as ::core::ffi::c_uint);
        zb = zb.wrapping_add(zc);
        z6 = rotl32(z6 ^ zb, 12 as ::core::ffi::c_uint);
        z1 = z1.wrapping_add(z6);
        zc = rotl32(zc ^ z1, 8 as ::core::ffi::c_uint);
        zb = zb.wrapping_add(zc);
        z6 = rotl32(z6 ^ zb, 7 as ::core::ffi::c_uint);
        z2 = z2.wrapping_add(z7);
        zd = rotl32(zd ^ z2, 16 as ::core::ffi::c_uint);
        z8 = z8.wrapping_add(zd);
        z7 = rotl32(z7 ^ z8, 12 as ::core::ffi::c_uint);
        z2 = z2.wrapping_add(z7);
        zd = rotl32(zd ^ z2, 8 as ::core::ffi::c_uint);
        z8 = z8.wrapping_add(zd);
        z7 = rotl32(z7 ^ z8, 7 as ::core::ffi::c_uint);
        z3 = z3.wrapping_add(z4);
        ze = rotl32(ze ^ z3, 16 as ::core::ffi::c_uint);
        z9 = z9.wrapping_add(ze);
        z4 = rotl32(z4 ^ z9, 12 as ::core::ffi::c_uint);
        z3 = z3.wrapping_add(z4);
        ze = rotl32(ze ^ z3, 8 as ::core::ffi::c_uint);
        z9 = z9.wrapping_add(ze);
        z4 = rotl32(z4 ^ z9, 7 as ::core::ffi::c_uint);
        i += 1;
    }
    x0 = x0.wrapping_add(z0);
    x1 = x1.wrapping_add(z1);
    x2 = x2.wrapping_add(z2);
    x3 = x3.wrapping_add(z3);
    x4 = x4.wrapping_add(z4);
    x5 = x5.wrapping_add(z5);
    x6 = x6.wrapping_add(z6);
    x7 = x7.wrapping_add(z7);
    x8 = x8.wrapping_add(z8);
    x9 = x9.wrapping_add(z9);
    xa = xa.wrapping_add(za);
    xb = xb.wrapping_add(zb);
    xc = xc.wrapping_add(zc);
    xd = xd.wrapping_add(zd);
    xe = xe.wrapping_add(ze);
    xf = xf.wrapping_add(zf);
    write32_le(x0, out.offset(0 as ::core::ffi::c_int as isize));
    write32_le(x1, out.offset(4 as ::core::ffi::c_int as isize));
    write32_le(x2, out.offset(8 as ::core::ffi::c_int as isize));
    write32_le(x3, out.offset(12 as ::core::ffi::c_int as isize));
    write32_le(x4, out.offset(16 as ::core::ffi::c_int as isize));
    write32_le(x5, out.offset(20 as ::core::ffi::c_int as isize));
    write32_le(x6, out.offset(24 as ::core::ffi::c_int as isize));
    write32_le(x7, out.offset(28 as ::core::ffi::c_int as isize));
    write32_le(x8, out.offset(32 as ::core::ffi::c_int as isize));
    write32_le(x9, out.offset(36 as ::core::ffi::c_int as isize));
    write32_le(xa, out.offset(40 as ::core::ffi::c_int as isize));
    write32_le(xb, out.offset(44 as ::core::ffi::c_int as isize));
    write32_le(xc, out.offset(48 as ::core::ffi::c_int as isize));
    write32_le(xd, out.offset(52 as ::core::ffi::c_int as isize));
    write32_le(xe, out.offset(56 as ::core::ffi::c_int as isize));
    write32_le(xf, out.offset(60 as ::core::ffi::c_int as isize));
}
static mut chacha20_tau: *const uint8_t =
    b"expand 16-byte k\0".as_ptr() as *const ::core::ffi::c_char as *const uint8_t;
static mut chacha20_sigma: *const uint8_t =
    b"expand 32-byte k\0".as_ptr() as *const ::core::ffi::c_char as *const uint8_t;
unsafe extern "C" fn set_key(
    mut ctx: *mut cf_chacha20_ctx,
    mut key: *const uint8_t,
    mut nkey: size_t,
) {
    match nkey {
        16 => {
            memcpy(
                &raw mut (*ctx).key0 as *mut uint8_t as *mut ::core::ffi::c_void,
                key as *const ::core::ffi::c_void,
                16 as size_t,
            );
            memcpy(
                &raw mut (*ctx).key1 as *mut uint8_t as *mut ::core::ffi::c_void,
                key as *const ::core::ffi::c_void,
                16 as size_t,
            );
            (*ctx).constant = chacha20_tau;
        }
        32 => {
            memcpy(
                &raw mut (*ctx).key0 as *mut uint8_t as *mut ::core::ffi::c_void,
                key as *const ::core::ffi::c_void,
                16 as size_t,
            );
            memcpy(
                &raw mut (*ctx).key1 as *mut uint8_t as *mut ::core::ffi::c_void,
                key.offset(16 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
                16 as size_t,
            );
            (*ctx).constant = chacha20_sigma;
        }
        _ => {
            abort();
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn cf_chacha20_init(
    mut ctx: *mut cf_chacha20_ctx,
    mut key: *const uint8_t,
    mut nkey: size_t,
    mut nonce: *const uint8_t,
) {
    set_key(ctx, key, nkey);
    memset(
        &raw mut (*ctx).nonce as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    memcpy(
        (&raw mut (*ctx).nonce as *mut uint8_t).offset(8 as ::core::ffi::c_int as isize)
            as *mut ::core::ffi::c_void,
        nonce as *const ::core::ffi::c_void,
        8 as size_t,
    );
    (*ctx).nblock = 0 as size_t;
    (*ctx).ncounter = 8 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn cf_chacha20_init_custom(
    mut ctx: *mut cf_chacha20_ctx,
    mut key: *const uint8_t,
    mut nkey: size_t,
    mut nonce: *const uint8_t,
    mut ncounter: size_t,
) {
    if !(ncounter > 0 as size_t) {
        abort();
    }
    set_key(ctx, key, nkey);
    memcpy(
        &raw mut (*ctx).nonce as *mut uint8_t as *mut ::core::ffi::c_void,
        nonce as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    (*ctx).nblock = 0 as size_t;
    (*ctx).ncounter = ncounter;
}
unsafe extern "C" fn cf_chacha20_next_block(
    mut vctx: *mut ::core::ffi::c_void,
    mut out: *mut uint8_t,
) {
    let mut ctx: *mut cf_chacha20_ctx = vctx as *mut cf_chacha20_ctx;
    cf_chacha20_core(
        &raw mut (*ctx).key0 as *mut uint8_t as *const uint8_t,
        &raw mut (*ctx).key1 as *mut uint8_t as *const uint8_t,
        &raw mut (*ctx).nonce as *mut uint8_t as *const uint8_t,
        (*ctx).constant as *const uint8_t,
        out as *mut uint8_t,
    );
    incr_le(&raw mut (*ctx).nonce as *mut uint8_t, (*ctx).ncounter);
}
#[no_mangle]
pub unsafe extern "C" fn cf_chacha20_cipher(
    mut ctx: *mut cf_chacha20_ctx,
    mut input: *const uint8_t,
    mut output: *mut uint8_t,
    mut bytes: size_t,
) {
    cf_blockwise_xor(
        &raw mut (*ctx).block as *mut uint8_t,
        &raw mut (*ctx).nblock,
        64 as size_t,
        input as *const ::core::ffi::c_void,
        output as *mut ::core::ffi::c_void,
        bytes,
        Some(
            cf_chacha20_next_block
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> (),
        ),
        ctx as *mut ::core::ffi::c_void,
    );
}

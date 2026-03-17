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
    fn cf_gf128_frombytes_be(in_0: *const uint8_t, out: *mut uint32_t);
    fn cf_gf128_tobytes_be(in_0: *const uint32_t, out: *mut uint8_t);
    fn cf_gf128_add(x: *const uint32_t, y: *const uint32_t, out: *mut uint32_t);
    fn cf_gf128_mul(x: *const uint32_t, y: *const uint32_t, out: *mut uint32_t);
    fn cf_ctr_init(
        ctx: *mut cf_ctr,
        prp: *const cf_prp,
        prpctx: *mut ::core::ffi::c_void,
        nonce: *const uint8_t,
    );
    fn cf_ctr_custom_counter(ctx: *mut cf_ctr, offset: size_t, width: size_t);
    fn cf_ctr_cipher(ctx: *mut cf_ctr, input: *const uint8_t, output: *mut uint8_t, bytes: size_t);
    fn cf_blockwise_accumulate(
        partial: *mut uint8_t,
        npartial: *mut size_t,
        nblock: size_t,
        input: *const ::core::ffi::c_void,
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
pub type cf_prp_block =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const uint8_t, *mut uint8_t) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_prp {
    pub blocksz: size_t,
    pub encrypt: cf_prp_block,
    pub decrypt: cf_prp_block,
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
pub struct cf_gcm_ctx {
    pub ctr: cf_ctr,
    pub gh: ghash_ctx,
    pub Y0: [uint8_t; 16],
    pub e_Y0: [uint8_t; 16],
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
pub type cf_blockwise_in_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const uint8_t) -> ()>;
#[inline]
unsafe extern "C" fn mem_clean(mut v: *mut ::core::ffi::c_void, mut len: size_t) {
    if len != 0 {
        memset(v as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, len);
        *(v as *mut uint8_t);
    }
}
#[inline]
unsafe extern "C" fn mem_eq(
    mut va: *const ::core::ffi::c_void,
    mut vb: *const ::core::ffi::c_void,
    mut len: size_t,
) -> ::core::ffi::c_uint {
    let mut a: *const uint8_t = va as *const uint8_t;
    let mut b: *const uint8_t = vb as *const uint8_t;
    let mut diff: uint8_t = 0 as uint8_t;
    loop {
        let c2rust_fresh0 = len;
        len = len.wrapping_sub(1);
        if !(c2rust_fresh0 != 0) {
            break;
        }
        let c2rust_fresh1 = a;
        a = a.offset(1);
        let c2rust_fresh2 = b;
        b = b.offset(1);
        diff = (diff as ::core::ffi::c_int
            | *c2rust_fresh1 as ::core::ffi::c_int ^ *c2rust_fresh2 as ::core::ffi::c_int)
            as uint8_t;
    }
    return (diff == 0) as ::core::ffi::c_int as ::core::ffi::c_uint;
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
#[inline]
unsafe extern "C" fn xor_bb(
    mut out: *mut uint8_t,
    mut x: *const uint8_t,
    mut y: *const uint8_t,
    mut len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < len {
        *out.offset(i as isize) = (*x.offset(i as isize) as ::core::ffi::c_int
            ^ *y.offset(i as isize) as ::core::ffi::c_int)
            as uint8_t;
        i = i.wrapping_add(1);
    }
}
pub const STATE_INVALID: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const STATE_AAD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const STATE_CIPHER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
unsafe extern "C" fn ghash_init(mut ctx: *mut ghash_ctx, mut H: *mut uint8_t) {
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<ghash_ctx>() as size_t,
    );
    cf_gf128_frombytes_be(H as *const uint8_t, &raw mut (*ctx).H as *mut uint32_t);
    (*ctx).state = STATE_AAD as ::core::ffi::c_uint;
}
unsafe extern "C" fn ghash_block(mut vctx: *mut ::core::ffi::c_void, mut data: *const uint8_t) {
    let mut ctx: *mut ghash_ctx = vctx as *mut ghash_ctx;
    let mut gfdata: cf_gf128 = [0; 4];
    cf_gf128_frombytes_be(data as *const uint8_t, &raw mut gfdata as *mut uint32_t);
    cf_gf128_add(
        &raw mut gfdata as *mut uint32_t as *const uint32_t,
        &raw mut (*ctx).Y as *mut uint32_t as *const uint32_t,
        &raw mut (*ctx).Y as *mut uint32_t,
    );
    cf_gf128_mul(
        &raw mut (*ctx).Y as *mut uint32_t as *const uint32_t,
        &raw mut (*ctx).H as *mut uint32_t as *const uint32_t,
        &raw mut (*ctx).Y as *mut uint32_t,
    );
}
unsafe extern "C" fn ghash_add(mut ctx: *mut ghash_ctx, mut buf: *const uint8_t, mut n: size_t) {
    cf_blockwise_accumulate(
        &raw mut (*ctx).buffer as *mut uint8_t,
        &raw mut (*ctx).buffer_used,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
        buf as *const ::core::ffi::c_void,
        n,
        Some(ghash_block as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const uint8_t) -> ()),
        ctx as *mut ::core::ffi::c_void,
    );
}
unsafe extern "C" fn ghash_add_pad(mut ctx: *mut ghash_ctx) {
    if (*ctx).buffer_used == 0 as size_t {
        return;
    }
    memset(
        (&raw mut (*ctx).buffer as *mut uint8_t).offset((*ctx).buffer_used as isize)
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (::core::mem::size_of::<[uint8_t; 16]>() as size_t).wrapping_sub((*ctx).buffer_used),
    );
    ghash_block(
        ctx as *mut ::core::ffi::c_void,
        &raw mut (*ctx).buffer as *mut uint8_t,
    );
    (*ctx).buffer_used = 0 as size_t;
}
unsafe extern "C" fn ghash_add_aad(
    mut ctx: *mut ghash_ctx,
    mut buf: *const uint8_t,
    mut n: size_t,
) {
    if !((*ctx).state == 1 as ::core::ffi::c_uint) {
        abort();
    }
    (*ctx).len_aad = ((*ctx).len_aad as ::core::ffi::c_ulong)
        .wrapping_add(n as ::core::ffi::c_ulong) as uint64_t as uint64_t;
    ghash_add(ctx, buf, n);
}
unsafe extern "C" fn ghash_add_cipher(
    mut ctx: *mut ghash_ctx,
    mut buf: *const uint8_t,
    mut n: size_t,
) {
    if (*ctx).state == STATE_AAD as ::core::ffi::c_uint {
        ghash_add_pad(ctx);
        (*ctx).state = STATE_CIPHER as ::core::ffi::c_uint;
    }
    if !((*ctx).state == 2 as ::core::ffi::c_uint) {
        abort();
    }
    (*ctx).len_cipher = ((*ctx).len_cipher as ::core::ffi::c_ulong)
        .wrapping_add(n as ::core::ffi::c_ulong) as uint64_t as uint64_t;
    ghash_add(ctx, buf, n);
}
unsafe extern "C" fn ghash_final(mut ctx: *mut ghash_ctx, mut out: *mut uint8_t) {
    let mut lenbuf: [uint8_t; 8] = [0; 8];
    if (*ctx).state == STATE_AAD as ::core::ffi::c_uint
        || (*ctx).state == STATE_CIPHER as ::core::ffi::c_uint
    {
        ghash_add_pad(ctx);
        (*ctx).state = STATE_INVALID as ::core::ffi::c_uint;
    }
    write64_be(
        (*ctx).len_aad.wrapping_mul(8 as uint64_t),
        &raw mut lenbuf as *mut uint8_t,
    );
    ghash_add(
        ctx,
        &raw mut lenbuf as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 8]>() as size_t,
    );
    write64_be(
        (*ctx).len_cipher.wrapping_mul(8 as uint64_t),
        &raw mut lenbuf as *mut uint8_t,
    );
    ghash_add(
        ctx,
        &raw mut lenbuf as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 8]>() as size_t,
    );
    if !((*ctx).buffer_used == 0 as size_t) {
        abort();
    }
    cf_gf128_tobytes_be(&raw mut (*ctx).Y as *mut uint32_t as *const uint32_t, out);
}
#[no_mangle]
pub unsafe extern "C" fn cf_gcm_encrypt_init(
    mut prp: *const cf_prp,
    mut prpctx: *mut ::core::ffi::c_void,
    mut gcmctx: *mut cf_gcm_ctx,
    mut header: *const uint8_t,
    mut nheader: size_t,
    mut nonce: *const uint8_t,
    mut nnonce: size_t,
) {
    let mut H: [uint8_t; 16] = [
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
    (*prp).encrypt.expect("non-null function pointer")(
        prpctx,
        &raw mut H as *mut uint8_t,
        &raw mut H as *mut uint8_t,
    );
    if nnonce == 12 as size_t {
        memcpy(
            &raw mut (*gcmctx).Y0 as *mut uint8_t as *mut ::core::ffi::c_void,
            nonce as *const ::core::ffi::c_void,
            nnonce,
        );
        (*gcmctx).Y0[14 as ::core::ffi::c_int as usize] = 0 as uint8_t;
        (*gcmctx).Y0[13 as ::core::ffi::c_int as usize] =
            (*gcmctx).Y0[14 as ::core::ffi::c_int as usize];
        (*gcmctx).Y0[12 as ::core::ffi::c_int as usize] =
            (*gcmctx).Y0[13 as ::core::ffi::c_int as usize];
        (*gcmctx).Y0[15 as ::core::ffi::c_int as usize] = 0x1 as uint8_t;
    } else {
        ghash_init(&raw mut (*gcmctx).gh, &raw mut H as *mut uint8_t);
        ghash_add_cipher(&raw mut (*gcmctx).gh, nonce, nnonce);
        ghash_final(&raw mut (*gcmctx).gh, &raw mut (*gcmctx).Y0 as *mut uint8_t);
    }
    ghash_init(&raw mut (*gcmctx).gh, &raw mut H as *mut uint8_t);
    ghash_add_aad(&raw mut (*gcmctx).gh, header, nheader);
    memset(
        &raw mut (*gcmctx).e_Y0 as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    cf_ctr_init(
        &raw mut (*gcmctx).ctr,
        prp,
        prpctx,
        &raw mut (*gcmctx).Y0 as *mut uint8_t as *const uint8_t,
    );
    cf_ctr_custom_counter(&raw mut (*gcmctx).ctr, 12 as size_t, 4 as size_t);
    cf_ctr_cipher(
        &raw mut (*gcmctx).ctr,
        &raw mut (*gcmctx).e_Y0 as *mut uint8_t,
        &raw mut (*gcmctx).e_Y0 as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    mem_clean(
        &raw mut H as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_gcm_encrypt_update(
    mut gcmctx: *mut cf_gcm_ctx,
    mut plain: *const uint8_t,
    mut nplain: size_t,
    mut cipher: *mut uint8_t,
) {
    cf_ctr_cipher(&raw mut (*gcmctx).ctr, plain, cipher, nplain);
    ghash_add_cipher(&raw mut (*gcmctx).gh, cipher, nplain);
}
#[no_mangle]
pub unsafe extern "C" fn cf_gcm_encrypt_final(
    mut gcmctx: *mut cf_gcm_ctx,
    mut tag: *mut uint8_t,
    mut ntag: size_t,
) {
    let mut full_tag: [uint8_t; 16] = [
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
    ghash_final(&raw mut (*gcmctx).gh, &raw mut full_tag as *mut uint8_t);
    if !(ntag > 1 as size_t && ntag <= 16 as size_t) {
        abort();
    }
    xor_bb(
        tag,
        &raw mut full_tag as *mut uint8_t,
        &raw mut (*gcmctx).e_Y0 as *mut uint8_t,
        ntag,
    );
    mem_clean(
        &raw mut full_tag as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    mem_clean(
        gcmctx as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<cf_gcm_ctx>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_gcm_encrypt(
    mut prp: *const cf_prp,
    mut prpctx: *mut ::core::ffi::c_void,
    mut plain: *const uint8_t,
    mut nplain: size_t,
    mut header: *const uint8_t,
    mut nheader: size_t,
    mut nonce: *const uint8_t,
    mut nnonce: size_t,
    mut cipher: *mut uint8_t,
    mut tag: *mut uint8_t,
    mut ntag: size_t,
) {
    let mut gcmctx: cf_gcm_ctx = cf_gcm_ctx {
        ctr: cf_ctr {
            prp: ::core::ptr::null::<cf_prp>(),
            prpctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
            nonce: [0; 16],
            keymat: [0; 16],
            nkeymat: 0,
            counter_offset: 0,
            counter_width: 0,
        },
        gh: ghash_ctx {
            H: [0; 4],
            Y: [0; 4],
            buffer: [0; 16],
            buffer_used: 0,
            len_aad: 0,
            len_cipher: 0,
            state: 0,
        },
        Y0: [0; 16],
        e_Y0: [0; 16],
    };
    cf_gcm_encrypt_init(prp, prpctx, &raw mut gcmctx, header, nheader, nonce, nnonce);
    cf_gcm_encrypt_update(&raw mut gcmctx, plain, nplain, cipher);
    cf_gcm_encrypt_final(&raw mut gcmctx, tag, ntag);
}
#[no_mangle]
pub unsafe extern "C" fn cf_gcm_decrypt(
    mut prp: *const cf_prp,
    mut prpctx: *mut ::core::ffi::c_void,
    mut cipher: *const uint8_t,
    mut ncipher: size_t,
    mut header: *const uint8_t,
    mut nheader: size_t,
    mut nonce: *const uint8_t,
    mut nnonce: size_t,
    mut tag: *const uint8_t,
    mut ntag: size_t,
    mut plain: *mut uint8_t,
) -> ::core::ffi::c_int {
    let mut H: [uint8_t; 16] = [
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
    let mut Y0: [uint8_t; 16] = [0; 16];
    (*prp).encrypt.expect("non-null function pointer")(
        prpctx,
        &raw mut H as *mut uint8_t,
        &raw mut H as *mut uint8_t,
    );
    if nnonce == 12 as size_t {
        memcpy(
            &raw mut Y0 as *mut uint8_t as *mut ::core::ffi::c_void,
            nonce as *const ::core::ffi::c_void,
            nnonce,
        );
        Y0[14 as ::core::ffi::c_int as usize] = 0 as uint8_t;
        Y0[13 as ::core::ffi::c_int as usize] = Y0[14 as ::core::ffi::c_int as usize];
        Y0[12 as ::core::ffi::c_int as usize] = Y0[13 as ::core::ffi::c_int as usize];
        Y0[15 as ::core::ffi::c_int as usize] = 0x1 as uint8_t;
    } else {
        let mut gh: ghash_ctx = ghash_ctx {
            H: [0; 4],
            Y: [0; 4],
            buffer: [0; 16],
            buffer_used: 0,
            len_aad: 0,
            len_cipher: 0,
            state: 0,
        };
        ghash_init(&raw mut gh, &raw mut H as *mut uint8_t);
        ghash_add_cipher(&raw mut gh, nonce, nnonce);
        ghash_final(&raw mut gh, &raw mut Y0 as *mut uint8_t);
    }
    let mut gh_0: ghash_ctx = ghash_ctx {
        H: [0; 4],
        Y: [0; 4],
        buffer: [0; 16],
        buffer_used: 0,
        len_aad: 0,
        len_cipher: 0,
        state: 0,
    };
    ghash_init(&raw mut gh_0, &raw mut H as *mut uint8_t);
    ghash_add_aad(&raw mut gh_0, header, nheader);
    let mut e_Y0: [uint8_t; 16] = [
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
    let mut ctr: cf_ctr = cf_ctr {
        prp: ::core::ptr::null::<cf_prp>(),
        prpctx: ::core::ptr::null_mut::<::core::ffi::c_void>(),
        nonce: [0; 16],
        keymat: [0; 16],
        nkeymat: 0,
        counter_offset: 0,
        counter_width: 0,
    };
    cf_ctr_init(
        &raw mut ctr,
        prp,
        prpctx,
        &raw mut Y0 as *mut uint8_t as *const uint8_t,
    );
    cf_ctr_custom_counter(&raw mut ctr, 12 as size_t, 4 as size_t);
    cf_ctr_cipher(
        &raw mut ctr,
        &raw mut e_Y0 as *mut uint8_t,
        &raw mut e_Y0 as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    ghash_add_cipher(&raw mut gh_0, cipher, ncipher);
    let mut full_tag: [uint8_t; 16] = [0; 16];
    ghash_final(&raw mut gh_0, &raw mut full_tag as *mut uint8_t);
    if !(ntag > 1 as size_t && ntag <= 16 as size_t) {
        abort();
    }
    xor_bb(
        &raw mut full_tag as *mut uint8_t,
        &raw mut full_tag as *mut uint8_t,
        &raw mut e_Y0 as *mut uint8_t,
        ntag,
    );
    let mut err: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
    if !(mem_eq(
        &raw mut full_tag as *mut uint8_t as *const ::core::ffi::c_void,
        tag as *const ::core::ffi::c_void,
        ntag,
    ) == 0)
    {
        cf_ctr_cipher(&raw mut ctr, cipher, plain, ncipher);
        err = 0 as ::core::ffi::c_int;
    }
    mem_clean(
        &raw mut H as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    mem_clean(
        &raw mut Y0 as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    mem_clean(
        &raw mut e_Y0 as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    mem_clean(
        &raw mut full_tag as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    mem_clean(
        &raw mut gh_0 as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<ghash_ctx>() as size_t,
    );
    mem_clean(
        &raw mut ctr as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<cf_ctr>() as size_t,
    );
    return err;
}

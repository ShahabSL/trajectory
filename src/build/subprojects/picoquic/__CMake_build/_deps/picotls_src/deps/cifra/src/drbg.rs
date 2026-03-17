extern "C" {
    fn cf_hmac_init(
        ctx: *mut cf_hmac_ctx,
        hash: *const cf_chash,
        key: *const uint8_t,
        nkey: size_t,
    );
    fn cf_hmac_update(ctx: *mut cf_hmac_ctx, data: *const ::core::ffi::c_void, ndata: size_t);
    fn cf_hmac_finish(ctx: *mut cf_hmac_ctx, out: *mut uint8_t);
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
    static cf_sha256: cf_chash;
    fn abort() -> !;
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
pub union cf_chash_ctx {
    pub ctx: [uint8_t; 360],
    pub u16_0: uint16_t,
    pub u32_0: uint32_t,
    pub u64_0: uint64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_hmac_ctx {
    pub hash: *const cf_chash,
    pub inner: cf_chash_ctx,
    pub outer: cf_chash_ctx,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_hash_drbg_sha256 {
    pub V: [uint8_t; 55],
    pub C: [uint8_t; 55],
    pub reseed_counter: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_hmac_drbg {
    pub V: [uint8_t; 64],
    pub hmac: cf_hmac_ctx,
    pub reseed_counter: uint32_t,
}
#[inline]
unsafe extern "C" fn mem_clean(mut v: *mut ::core::ffi::c_void, mut len: size_t) {
    if len != 0 {
        memset(v as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, len);
        *(v as *mut uint8_t);
    }
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
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
unsafe extern "C" fn hash_df(
    mut H: *const cf_chash,
    mut in1: *const ::core::ffi::c_void,
    mut nin1: size_t,
    mut in2: *const ::core::ffi::c_void,
    mut nin2: size_t,
    mut in3: *const ::core::ffi::c_void,
    mut nin3: size_t,
    mut in4: *const ::core::ffi::c_void,
    mut nin4: size_t,
    mut out: *mut uint8_t,
    mut nout: size_t,
) {
    let mut counter: uint8_t = 1 as uint8_t;
    let mut bits_to_return: uint32_t = nout.wrapping_mul(8 as size_t) as uint32_t;
    let mut cbuf: [uint8_t; 4] = [0; 4];
    let mut block: [uint8_t; 64] = [0; 64];
    write32_be(bits_to_return, &raw mut cbuf as *mut uint8_t);
    while nout != 0 {
        let mut ctx: cf_chash_ctx = cf_chash_ctx { ctx: [0; 360] };
        (*H).init.expect("non-null function pointer")(&raw mut ctx as *mut ::core::ffi::c_void);
        (*H).update.expect("non-null function pointer")(
            &raw mut ctx as *mut ::core::ffi::c_void,
            &raw mut counter as *const ::core::ffi::c_void,
            ::core::mem::size_of::<uint8_t>() as size_t,
        );
        (*H).update.expect("non-null function pointer")(
            &raw mut ctx as *mut ::core::ffi::c_void,
            &raw mut cbuf as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
        );
        (*H).update.expect("non-null function pointer")(
            &raw mut ctx as *mut ::core::ffi::c_void,
            in1,
            nin1,
        );
        (*H).update.expect("non-null function pointer")(
            &raw mut ctx as *mut ::core::ffi::c_void,
            in2,
            nin2,
        );
        (*H).update.expect("non-null function pointer")(
            &raw mut ctx as *mut ::core::ffi::c_void,
            in3,
            nin3,
        );
        (*H).update.expect("non-null function pointer")(
            &raw mut ctx as *mut ::core::ffi::c_void,
            in4,
            nin4,
        );
        (*H).digest.expect("non-null function pointer")(
            &raw mut ctx as *const ::core::ffi::c_void,
            &raw mut block as *mut uint8_t,
        );
        let mut take: size_t = if (*H).hashsz < nout {
            (*H).hashsz
        } else {
            nout
        };
        memcpy(
            out as *mut ::core::ffi::c_void,
            &raw mut block as *mut uint8_t as *const ::core::ffi::c_void,
            take,
        );
        out = out.offset(take as isize);
        nout = nout.wrapping_sub(take);
        counter = (counter as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as uint8_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cf_hash_drbg_sha256_init(
    mut ctx: *mut cf_hash_drbg_sha256,
    mut entropy: *const ::core::ffi::c_void,
    mut nentropy: size_t,
    mut nonce: *const ::core::ffi::c_void,
    mut nnonce: size_t,
    mut persn: *const ::core::ffi::c_void,
    mut npersn: size_t,
) {
    mem_clean(
        ctx as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<cf_hash_drbg_sha256>() as size_t,
    );
    hash_df(
        &raw const cf_sha256,
        entropy,
        nentropy,
        nonce,
        nnonce,
        persn,
        npersn,
        ::core::ptr::null::<::core::ffi::c_void>(),
        0 as size_t,
        &raw mut (*ctx).V as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 55]>() as size_t,
    );
    let mut zero: uint8_t = 0 as uint8_t;
    hash_df(
        &raw const cf_sha256,
        &raw mut zero as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint8_t>() as size_t,
        &raw mut (*ctx).V as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 55]>() as size_t,
        ::core::ptr::null::<::core::ffi::c_void>(),
        0 as size_t,
        ::core::ptr::null::<::core::ffi::c_void>(),
        0 as size_t,
        &raw mut (*ctx).C as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 55]>() as size_t,
    );
    (*ctx).reseed_counter = 1 as uint32_t;
}
unsafe extern "C" fn add(
    mut out: *mut uint8_t,
    mut nout: size_t,
    mut in_0: *const uint8_t,
    mut nin: size_t,
) {
    if !(nout >= nin) {
        abort();
    }
    let mut carry: uint16_t = 0 as uint16_t;
    let mut oi: ::core::ffi::c_int = 0;
    let mut ii: ::core::ffi::c_int = 0;
    oi = nout.wrapping_sub(1 as size_t) as ::core::ffi::c_int;
    ii = nin.wrapping_sub(1 as size_t) as ::core::ffi::c_int;
    while oi >= 0 as ::core::ffi::c_int {
        carry = (carry as ::core::ffi::c_int + *out.offset(oi as isize) as ::core::ffi::c_int)
            as uint16_t;
        if ii >= 0 as ::core::ffi::c_int {
            carry = (carry as ::core::ffi::c_int + *in_0.offset(ii as isize) as ::core::ffi::c_int)
                as uint16_t;
        }
        *out.offset(oi as isize) =
            (carry as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
        carry = (carry as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uint16_t;
        ii -= 1;
        oi -= 1;
    }
}
unsafe extern "C" fn hash_process_addnl(
    mut H: *const cf_chash,
    mut input: *const ::core::ffi::c_void,
    mut ninput: size_t,
    mut V: *mut uint8_t,
    mut nV: size_t,
) {
    if ninput == 0 {
        return;
    }
    let mut two: uint8_t = 2 as uint8_t;
    let mut w: [uint8_t; 64] = [0; 64];
    let mut ctx: cf_chash_ctx = cf_chash_ctx { ctx: [0; 360] };
    (*H).init.expect("non-null function pointer")(&raw mut ctx as *mut ::core::ffi::c_void);
    (*H).update.expect("non-null function pointer")(
        &raw mut ctx as *mut ::core::ffi::c_void,
        &raw mut two as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint8_t>() as size_t,
    );
    (*H).update.expect("non-null function pointer")(
        &raw mut ctx as *mut ::core::ffi::c_void,
        V as *const ::core::ffi::c_void,
        nV,
    );
    (*H).update.expect("non-null function pointer")(
        &raw mut ctx as *mut ::core::ffi::c_void,
        input,
        ninput,
    );
    (*H).digest.expect("non-null function pointer")(
        &raw mut ctx as *const ::core::ffi::c_void,
        &raw mut w as *mut uint8_t,
    );
    add(V, nV, &raw mut w as *mut uint8_t, (*H).hashsz);
}
unsafe extern "C" fn hash_generate(
    mut H: *const cf_chash,
    mut data: *mut uint8_t,
    mut ndata: size_t,
    mut out: *mut ::core::ffi::c_void,
    mut nout: size_t,
) {
    let mut ctx: cf_chash_ctx = cf_chash_ctx { ctx: [0; 360] };
    let mut w: [uint8_t; 64] = [0; 64];
    let mut bout: *mut uint8_t = out as *mut uint8_t;
    let mut one: uint8_t = 1 as uint8_t;
    while nout != 0 {
        (*H).init.expect("non-null function pointer")(&raw mut ctx as *mut ::core::ffi::c_void);
        (*H).update.expect("non-null function pointer")(
            &raw mut ctx as *mut ::core::ffi::c_void,
            data as *const ::core::ffi::c_void,
            ndata,
        );
        (*H).digest.expect("non-null function pointer")(
            &raw mut ctx as *const ::core::ffi::c_void,
            &raw mut w as *mut uint8_t,
        );
        let mut take: size_t = if (*H).hashsz < nout {
            (*H).hashsz
        } else {
            nout
        };
        memcpy(
            bout as *mut ::core::ffi::c_void,
            &raw mut w as *mut uint8_t as *const ::core::ffi::c_void,
            take,
        );
        bout = bout.offset(take as isize);
        nout = nout.wrapping_sub(take);
        add(
            data,
            ndata,
            &raw mut one,
            ::core::mem::size_of::<uint8_t>() as size_t,
        );
    }
}
unsafe extern "C" fn hash_step(
    mut H: *const cf_chash,
    mut V: *mut uint8_t,
    mut nV: size_t,
    mut C: *const uint8_t,
    mut nC: size_t,
    mut reseed_counter: *mut uint32_t,
) {
    let mut h: [uint8_t; 64] = [0; 64];
    let mut three: uint8_t = 3 as uint8_t;
    let mut ctx: cf_chash_ctx = cf_chash_ctx { ctx: [0; 360] };
    (*H).init.expect("non-null function pointer")(&raw mut ctx as *mut ::core::ffi::c_void);
    (*H).update.expect("non-null function pointer")(
        &raw mut ctx as *mut ::core::ffi::c_void,
        &raw mut three as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint8_t>() as size_t,
    );
    (*H).update.expect("non-null function pointer")(
        &raw mut ctx as *mut ::core::ffi::c_void,
        V as *const ::core::ffi::c_void,
        nV,
    );
    (*H).digest.expect("non-null function pointer")(
        &raw mut ctx as *const ::core::ffi::c_void,
        &raw mut h as *mut uint8_t,
    );
    let mut reseed_counter_buf: [uint8_t; 4] = [0; 4];
    write32_be(*reseed_counter, &raw mut reseed_counter_buf as *mut uint8_t);
    add(V, nV, &raw mut h as *mut uint8_t, (*H).hashsz);
    add(V, nV, C, nC);
    add(
        V,
        nV,
        &raw mut reseed_counter_buf as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 4]>() as size_t,
    );
    *reseed_counter = (*reseed_counter).wrapping_add(1 as uint32_t);
}
unsafe extern "C" fn hash_gen_request(
    mut ctx: *mut cf_hash_drbg_sha256,
    mut addnl: *const ::core::ffi::c_void,
    mut naddnl: size_t,
    mut out: *mut ::core::ffi::c_void,
    mut nout: size_t,
) {
    let mut data: [uint8_t; 55] = [0; 55];
    if cf_hash_drbg_sha256_needs_reseed(ctx) != 0 {
        abort();
    }
    hash_process_addnl(
        &raw const cf_sha256,
        addnl,
        naddnl,
        &raw mut (*ctx).V as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 55]>() as size_t,
    );
    if !(::core::mem::size_of::<[uint8_t; 55]>() as usize
        == ::core::mem::size_of::<[uint8_t; 55]>() as usize)
    {
        abort();
    }
    memcpy(
        &raw mut data as *mut uint8_t as *mut ::core::ffi::c_void,
        &raw mut (*ctx).V as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 55]>() as size_t,
    );
    hash_generate(
        &raw const cf_sha256,
        &raw mut data as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 55]>() as size_t,
        out,
        nout,
    );
    hash_step(
        &raw const cf_sha256,
        &raw mut (*ctx).V as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 55]>() as size_t,
        &raw mut (*ctx).C as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 55]>() as size_t,
        &raw mut (*ctx).reseed_counter,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_hash_drbg_sha256_gen_additional(
    mut ctx: *mut cf_hash_drbg_sha256,
    mut addnl: *const ::core::ffi::c_void,
    mut naddnl: size_t,
    mut out: *mut ::core::ffi::c_void,
    mut nout: size_t,
) {
    let mut bout: *mut uint8_t = out as *mut uint8_t;
    while nout != 0 as size_t {
        let mut take: size_t = if (0x10000 as size_t) < nout {
            0x10000 as size_t
        } else {
            nout
        };
        hash_gen_request(ctx, addnl, naddnl, bout as *mut ::core::ffi::c_void, take);
        bout = bout.offset(take as isize);
        nout = nout.wrapping_sub(take);
        addnl = ::core::ptr::null::<::core::ffi::c_void>();
        naddnl = 0 as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cf_hash_drbg_sha256_gen(
    mut ctx: *mut cf_hash_drbg_sha256,
    mut out: *mut ::core::ffi::c_void,
    mut nout: size_t,
) {
    cf_hash_drbg_sha256_gen_additional(
        ctx,
        ::core::ptr::null::<::core::ffi::c_void>(),
        0 as size_t,
        out,
        nout,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_hash_drbg_sha256_reseed(
    mut ctx: *mut cf_hash_drbg_sha256,
    mut entropy: *const ::core::ffi::c_void,
    mut nentropy: size_t,
    mut addnl: *const ::core::ffi::c_void,
    mut naddnl: size_t,
) {
    let mut one: uint8_t = 1 as uint8_t;
    memcpy(
        &raw mut (*ctx).C as *mut uint8_t as *mut ::core::ffi::c_void,
        &raw mut (*ctx).V as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 55]>() as size_t,
    );
    hash_df(
        &raw const cf_sha256,
        &raw mut one as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint8_t>() as size_t,
        &raw mut (*ctx).C as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 55]>() as size_t,
        entropy,
        nentropy,
        addnl,
        naddnl,
        &raw mut (*ctx).V as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 55]>() as size_t,
    );
    let mut zero: uint8_t = 0 as uint8_t;
    hash_df(
        &raw const cf_sha256,
        &raw mut zero as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint8_t>() as size_t,
        &raw mut (*ctx).V as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 55]>() as size_t,
        ::core::ptr::null::<::core::ffi::c_void>(),
        0 as size_t,
        ::core::ptr::null::<::core::ffi::c_void>(),
        0 as size_t,
        &raw mut (*ctx).C as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 55]>() as size_t,
    );
    (*ctx).reseed_counter = 1 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn cf_hash_drbg_sha256_needs_reseed(
    mut ctx: *const cf_hash_drbg_sha256,
) -> uint32_t {
    return ((*ctx).reseed_counter == 0 as uint32_t) as ::core::ffi::c_int as uint32_t;
}
unsafe extern "C" fn hmac_drbg_update(
    mut ctx: *mut cf_hmac_drbg,
    mut in1: *const ::core::ffi::c_void,
    mut nin1: size_t,
    mut in2: *const ::core::ffi::c_void,
    mut nin2: size_t,
    mut in3: *const ::core::ffi::c_void,
    mut nin3: size_t,
) {
    let mut local: cf_hmac_ctx = cf_hmac_ctx {
        hash: ::core::ptr::null::<cf_chash>(),
        inner: cf_chash_ctx { ctx: [0; 360] },
        outer: cf_chash_ctx { ctx: [0; 360] },
    };
    let mut H: *const cf_chash = (*ctx).hmac.hash;
    let mut new_key: [uint8_t; 64] = [0; 64];
    let mut zero: uint8_t = 0 as uint8_t;
    local = (*ctx).hmac;
    cf_hmac_update(
        &raw mut local,
        &raw mut (*ctx).V as *mut uint8_t as *const ::core::ffi::c_void,
        (*H).hashsz,
    );
    cf_hmac_update(
        &raw mut local,
        &raw mut zero as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint8_t>() as size_t,
    );
    cf_hmac_update(&raw mut local, in1, nin1);
    cf_hmac_update(&raw mut local, in2, nin2);
    cf_hmac_update(&raw mut local, in3, nin3);
    cf_hmac_finish(&raw mut local, &raw mut new_key as *mut uint8_t);
    cf_hmac_init(
        &raw mut (*ctx).hmac,
        H,
        &raw mut new_key as *mut uint8_t,
        (*H).hashsz,
    );
    mem_clean(
        &raw mut new_key as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    local = (*ctx).hmac;
    cf_hmac_update(
        &raw mut local,
        &raw mut (*ctx).V as *mut uint8_t as *const ::core::ffi::c_void,
        (*H).hashsz,
    );
    cf_hmac_finish(&raw mut local, &raw mut (*ctx).V as *mut uint8_t);
    if nin1 == 0 as size_t && nin2 == 0 as size_t && nin3 == 0 as size_t {
        return;
    }
    let mut one: uint8_t = 1 as uint8_t;
    local = (*ctx).hmac;
    cf_hmac_update(
        &raw mut local,
        &raw mut (*ctx).V as *mut uint8_t as *const ::core::ffi::c_void,
        (*H).hashsz,
    );
    cf_hmac_update(
        &raw mut local,
        &raw mut one as *const ::core::ffi::c_void,
        ::core::mem::size_of::<uint8_t>() as size_t,
    );
    cf_hmac_update(&raw mut local, in1, nin1);
    cf_hmac_update(&raw mut local, in2, nin2);
    cf_hmac_update(&raw mut local, in3, nin3);
    cf_hmac_finish(&raw mut local, &raw mut new_key as *mut uint8_t);
    cf_hmac_init(
        &raw mut (*ctx).hmac,
        H,
        &raw mut new_key as *mut uint8_t,
        (*H).hashsz,
    );
    mem_clean(
        &raw mut new_key as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    local = (*ctx).hmac;
    cf_hmac_update(
        &raw mut local,
        &raw mut (*ctx).V as *mut uint8_t as *const ::core::ffi::c_void,
        (*H).hashsz,
    );
    cf_hmac_finish(&raw mut local, &raw mut (*ctx).V as *mut uint8_t);
}
#[no_mangle]
pub unsafe extern "C" fn cf_hmac_drbg_init(
    mut ctx: *mut cf_hmac_drbg,
    mut hash: *const cf_chash,
    mut entropy: *const ::core::ffi::c_void,
    mut nentropy: size_t,
    mut nonce: *const ::core::ffi::c_void,
    mut nnonce: size_t,
    mut persn: *const ::core::ffi::c_void,
    mut npersn: size_t,
) {
    mem_clean(
        ctx as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<cf_hmac_drbg>() as size_t,
    );
    if !((*hash).hashsz <= 64 as size_t) {
        abort();
    }
    let mut initial_key: [uint8_t; 64] = [0; 64];
    memset(
        &raw mut initial_key as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (*hash).hashsz,
    );
    memset(
        &raw mut (*ctx).V as *mut uint8_t as *mut ::core::ffi::c_void,
        0x1 as ::core::ffi::c_int,
        (*hash).hashsz,
    );
    cf_hmac_init(
        &raw mut (*ctx).hmac,
        hash,
        &raw mut initial_key as *mut uint8_t,
        (*hash).hashsz,
    );
    hmac_drbg_update(ctx, entropy, nentropy, nonce, nnonce, persn, npersn);
    (*ctx).reseed_counter = 1 as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn cf_hmac_drbg_needs_reseed(mut ctx: *const cf_hmac_drbg) -> uint32_t {
    return ((*ctx).reseed_counter == 0 as uint32_t) as ::core::ffi::c_int as uint32_t;
}
unsafe extern "C" fn hmac_drbg_generate(
    mut ctx: *mut cf_hmac_drbg,
    mut addnl: *const ::core::ffi::c_void,
    mut naddnl: size_t,
    mut out: *mut ::core::ffi::c_void,
    mut nout: size_t,
) {
    if cf_hmac_drbg_needs_reseed(ctx) != 0 {
        abort();
    }
    if naddnl != 0 {
        hmac_drbg_update(
            ctx,
            addnl,
            naddnl,
            ::core::ptr::null::<::core::ffi::c_void>(),
            0 as size_t,
            ::core::ptr::null::<::core::ffi::c_void>(),
            0 as size_t,
        );
    }
    let mut bout: *mut uint8_t = out as *mut uint8_t;
    let mut local: cf_hmac_ctx = cf_hmac_ctx {
        hash: ::core::ptr::null::<cf_chash>(),
        inner: cf_chash_ctx { ctx: [0; 360] },
        outer: cf_chash_ctx { ctx: [0; 360] },
    };
    while nout != 0 {
        local = (*ctx).hmac;
        cf_hmac_update(
            &raw mut local,
            &raw mut (*ctx).V as *mut uint8_t as *const ::core::ffi::c_void,
            (*(*ctx).hmac.hash).hashsz,
        );
        cf_hmac_finish(&raw mut local, &raw mut (*ctx).V as *mut uint8_t);
        let mut take: size_t = if (*(*ctx).hmac.hash).hashsz < nout {
            (*(*ctx).hmac.hash).hashsz
        } else {
            nout
        };
        memcpy(
            bout as *mut ::core::ffi::c_void,
            &raw mut (*ctx).V as *mut uint8_t as *const ::core::ffi::c_void,
            take,
        );
        bout = bout.offset(take as isize);
        nout = nout.wrapping_sub(take);
    }
    hmac_drbg_update(
        ctx,
        addnl,
        naddnl,
        ::core::ptr::null::<::core::ffi::c_void>(),
        0 as size_t,
        ::core::ptr::null::<::core::ffi::c_void>(),
        0 as size_t,
    );
    (*ctx).reseed_counter = (*ctx).reseed_counter.wrapping_add(1);
}
#[no_mangle]
pub unsafe extern "C" fn cf_hmac_drbg_gen_additional(
    mut ctx: *mut cf_hmac_drbg,
    mut addnl: *const ::core::ffi::c_void,
    mut naddnl: size_t,
    mut out: *mut ::core::ffi::c_void,
    mut nout: size_t,
) {
    let mut bout: *mut uint8_t = out as *mut uint8_t;
    while nout != 0 as size_t {
        let mut take: size_t = if (0x10000 as size_t) < nout {
            0x10000 as size_t
        } else {
            nout
        };
        hmac_drbg_generate(ctx, addnl, naddnl, bout as *mut ::core::ffi::c_void, take);
        bout = bout.offset(take as isize);
        nout = nout.wrapping_sub(take);
        addnl = ::core::ptr::null::<::core::ffi::c_void>();
        naddnl = 0 as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cf_hmac_drbg_gen(
    mut ctx: *mut cf_hmac_drbg,
    mut out: *mut ::core::ffi::c_void,
    mut nout: size_t,
) {
    cf_hmac_drbg_gen_additional(
        ctx,
        ::core::ptr::null::<::core::ffi::c_void>(),
        0 as size_t,
        out,
        nout,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_hmac_drbg_reseed(
    mut ctx: *mut cf_hmac_drbg,
    mut entropy: *const ::core::ffi::c_void,
    mut nentropy: size_t,
    mut addnl: *const ::core::ffi::c_void,
    mut naddnl: size_t,
) {
    hmac_drbg_update(
        ctx,
        entropy,
        nentropy,
        addnl,
        naddnl,
        ::core::ptr::null::<::core::ffi::c_void>(),
        0 as size_t,
    );
    (*ctx).reseed_counter = 1 as uint32_t;
}

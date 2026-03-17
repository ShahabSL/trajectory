extern "C" {
    fn cf_hash(h: *const cf_chash, m: *const ::core::ffi::c_void, nm: size_t, out: *mut uint8_t);
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
#[inline]
unsafe extern "C" fn xor_b8(
    mut out: *mut uint8_t,
    mut in_0: *const uint8_t,
    mut b8: uint8_t,
    mut len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < len {
        *out.offset(i as isize) =
            (*in_0.offset(i as isize) as ::core::ffi::c_int ^ b8 as ::core::ffi::c_int) as uint8_t;
        i = i.wrapping_add(1);
    }
}
#[inline]
unsafe extern "C" fn mem_clean(mut v: *mut ::core::ffi::c_void, mut len: size_t) {
    if len != 0 {
        memset(v as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, len);
        *(v as *mut uint8_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cf_hmac_init(
    mut ctx: *mut cf_hmac_ctx,
    mut hash: *const cf_chash,
    mut key: *const uint8_t,
    mut nkey: size_t,
) {
    if ctx.is_null() {
        abort();
    }
    if hash.is_null() {
        abort();
    }
    mem_clean(
        ctx as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<cf_hmac_ctx>() as size_t,
    );
    (*ctx).hash = hash;
    let mut k: [uint8_t; 128] = [0; 128];
    if nkey > (*hash).blocksz {
        if !((*hash).hashsz <= (*hash).blocksz) {
            abort();
        }
        cf_hash(
            hash,
            key as *const ::core::ffi::c_void,
            nkey,
            &raw mut k as *mut uint8_t,
        );
        key = &raw mut k as *mut uint8_t;
        nkey = (*hash).hashsz;
    }
    if &raw mut k as *mut uint8_t != key as *mut uint8_t {
        memcpy(
            &raw mut k as *mut uint8_t as *mut ::core::ffi::c_void,
            key as *const ::core::ffi::c_void,
            nkey,
        );
    }
    if (*hash).blocksz > nkey {
        memset(
            (&raw mut k as *mut uint8_t).offset(nkey as isize) as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (*hash).blocksz.wrapping_sub(nkey),
        );
    }
    let mut blk: [uint8_t; 128] = [0; 128];
    xor_b8(
        &raw mut blk as *mut uint8_t,
        &raw mut k as *mut uint8_t,
        0x36 as uint8_t,
        (*hash).blocksz,
    );
    (*hash).init.expect("non-null function pointer")(
        &raw mut (*ctx).inner as *mut ::core::ffi::c_void,
    );
    (*hash).update.expect("non-null function pointer")(
        &raw mut (*ctx).inner as *mut ::core::ffi::c_void,
        &raw mut blk as *mut uint8_t as *const ::core::ffi::c_void,
        (*hash).blocksz,
    );
    xor_b8(
        &raw mut blk as *mut uint8_t,
        &raw mut k as *mut uint8_t,
        0x5c as uint8_t,
        (*hash).blocksz,
    );
    (*hash).init.expect("non-null function pointer")(
        &raw mut (*ctx).outer as *mut ::core::ffi::c_void,
    );
    (*hash).update.expect("non-null function pointer")(
        &raw mut (*ctx).outer as *mut ::core::ffi::c_void,
        &raw mut blk as *mut uint8_t as *const ::core::ffi::c_void,
        (*hash).blocksz,
    );
    mem_clean(
        &raw mut blk as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 128]>() as size_t,
    );
    mem_clean(
        &raw mut k as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 128]>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_hmac_update(
    mut ctx: *mut cf_hmac_ctx,
    mut data: *const ::core::ffi::c_void,
    mut ndata: size_t,
) {
    if !(!ctx.is_null() && !(*ctx).hash.is_null()) {
        abort();
    }
    (*(*ctx).hash).update.expect("non-null function pointer")(
        &raw mut (*ctx).inner as *mut ::core::ffi::c_void,
        data,
        ndata,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_hmac_finish(mut ctx: *mut cf_hmac_ctx, mut out: *mut uint8_t) {
    if !(!ctx.is_null() && !(*ctx).hash.is_null()) {
        abort();
    }
    if out.is_null() {
        abort();
    }
    let mut innerh: [uint8_t; 64] = [0; 64];
    (*(*ctx).hash).digest.expect("non-null function pointer")(
        &raw mut (*ctx).inner as *const ::core::ffi::c_void,
        &raw mut innerh as *mut uint8_t,
    );
    (*(*ctx).hash).update.expect("non-null function pointer")(
        &raw mut (*ctx).outer as *mut ::core::ffi::c_void,
        &raw mut innerh as *mut uint8_t as *const ::core::ffi::c_void,
        (*(*ctx).hash).hashsz,
    );
    (*(*ctx).hash).digest.expect("non-null function pointer")(
        &raw mut (*ctx).outer as *const ::core::ffi::c_void,
        out,
    );
    mem_clean(
        ctx as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<cf_hmac_ctx>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_hmac(
    mut key: *const uint8_t,
    mut nkey: size_t,
    mut msg: *const uint8_t,
    mut nmsg: size_t,
    mut out: *mut uint8_t,
    mut hash: *const cf_chash,
) {
    let mut ctx: cf_hmac_ctx = cf_hmac_ctx {
        hash: ::core::ptr::null::<cf_chash>(),
        inner: cf_chash_ctx { ctx: [0; 360] },
        outer: cf_chash_ctx { ctx: [0; 360] },
    };
    if out.is_null() {
        abort();
    }
    if hash.is_null() {
        abort();
    }
    cf_hmac_init(&raw mut ctx, hash, key, nkey);
    cf_hmac_update(&raw mut ctx, msg as *const ::core::ffi::c_void, nmsg);
    cf_hmac_finish(&raw mut ctx, out);
}

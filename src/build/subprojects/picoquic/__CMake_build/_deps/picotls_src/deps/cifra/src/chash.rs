extern "C" {
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
#[inline]
unsafe extern "C" fn mem_clean(mut v: *mut ::core::ffi::c_void, mut len: size_t) {
    if len != 0 {
        memset(v as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, len);
        *(v as *mut uint8_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cf_hash(
    mut h: *const cf_chash,
    mut m: *const ::core::ffi::c_void,
    mut nm: size_t,
    mut out: *mut uint8_t,
) {
    let mut ctx: cf_chash_ctx = cf_chash_ctx { ctx: [0; 360] };
    if h.is_null() {
        abort();
    }
    (*h).init.expect("non-null function pointer")(&raw mut ctx as *mut ::core::ffi::c_void);
    (*h).update.expect("non-null function pointer")(
        &raw mut ctx as *mut ::core::ffi::c_void,
        m,
        nm,
    );
    (*h).digest.expect("non-null function pointer")(
        &raw mut ctx as *const ::core::ffi::c_void,
        out,
    );
    mem_clean(
        &raw mut ctx as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<cf_chash_ctx>() as size_t,
    );
}

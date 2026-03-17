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
pub type uint8_t = __uint8_t;
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
pub struct cf_cbc {
    pub prp: *const cf_prp,
    pub prpctx: *mut ::core::ffi::c_void,
    pub block: [uint8_t; 16],
}
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
pub type cf_blockwise_out_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>;
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
#[inline]
unsafe extern "C" fn incr_be(mut v: *mut uint8_t, mut len: size_t) {
    len = len.wrapping_sub(1);
    loop {
        let ref mut c2rust_fresh2 = *v.offset(len as isize);
        *c2rust_fresh2 = (*c2rust_fresh2).wrapping_add(1);
        if *c2rust_fresh2 as ::core::ffi::c_int != 0 as ::core::ffi::c_int {
            return;
        }
        if len == 0 as size_t {
            return;
        }
        len = len.wrapping_sub(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cf_cbc_init(
    mut ctx: *mut cf_cbc,
    mut prp: *const cf_prp,
    mut prpctx: *mut ::core::ffi::c_void,
    mut iv: *const uint8_t,
) {
    (*ctx).prp = prp;
    (*ctx).prpctx = prpctx;
    memcpy(
        &raw mut (*ctx).block as *mut uint8_t as *mut ::core::ffi::c_void,
        iv as *const ::core::ffi::c_void,
        (*prp).blocksz,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_cbc_encrypt(
    mut ctx: *mut cf_cbc,
    mut input: *const uint8_t,
    mut output: *mut uint8_t,
    mut blocks: size_t,
) {
    let mut buf: [uint8_t; 16] = [0; 16];
    let mut nblk: size_t = (*(*ctx).prp).blocksz;
    loop {
        let c2rust_fresh0 = blocks;
        blocks = blocks.wrapping_sub(1);
        if !(c2rust_fresh0 != 0) {
            break;
        }
        xor_bb(
            &raw mut buf as *mut uint8_t,
            input,
            &raw mut (*ctx).block as *mut uint8_t,
            nblk,
        );
        (*(*ctx).prp).encrypt.expect("non-null function pointer")(
            (*ctx).prpctx,
            &raw mut buf as *mut uint8_t,
            &raw mut (*ctx).block as *mut uint8_t,
        );
        memcpy(
            output as *mut ::core::ffi::c_void,
            &raw mut (*ctx).block as *mut uint8_t as *const ::core::ffi::c_void,
            nblk,
        );
        input = input.offset(nblk as isize);
        output = output.offset(nblk as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cf_cbc_decrypt(
    mut ctx: *mut cf_cbc,
    mut input: *const uint8_t,
    mut output: *mut uint8_t,
    mut blocks: size_t,
) {
    let mut buf: [uint8_t; 16] = [0; 16];
    let mut nblk: size_t = (*(*ctx).prp).blocksz;
    loop {
        let c2rust_fresh1 = blocks;
        blocks = blocks.wrapping_sub(1);
        if !(c2rust_fresh1 != 0) {
            break;
        }
        (*(*ctx).prp).decrypt.expect("non-null function pointer")(
            (*ctx).prpctx,
            input,
            &raw mut buf as *mut uint8_t,
        );
        xor_bb(
            output,
            &raw mut buf as *mut uint8_t,
            &raw mut (*ctx).block as *mut uint8_t,
            nblk,
        );
        memcpy(
            &raw mut (*ctx).block as *mut uint8_t as *mut ::core::ffi::c_void,
            input as *const ::core::ffi::c_void,
            nblk,
        );
        input = input.offset(nblk as isize);
        output = output.offset(nblk as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cf_ctr_init(
    mut ctx: *mut cf_ctr,
    mut prp: *const cf_prp,
    mut prpctx: *mut ::core::ffi::c_void,
    mut nonce: *const uint8_t,
) {
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cf_ctr>() as size_t,
    );
    (*ctx).counter_offset = 0 as size_t;
    (*ctx).counter_width = (*prp).blocksz;
    (*ctx).prp = prp;
    (*ctx).prpctx = prpctx;
    (*ctx).nkeymat = 0 as size_t;
    memcpy(
        &raw mut (*ctx).nonce as *mut uint8_t as *mut ::core::ffi::c_void,
        nonce as *const ::core::ffi::c_void,
        (*prp).blocksz,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_ctr_custom_counter(
    mut ctx: *mut cf_ctr,
    mut offset: size_t,
    mut width: size_t,
) {
    if !((*(*ctx).prp).blocksz <= offset.wrapping_add(width)) {
        abort();
    }
    (*ctx).counter_offset = offset;
    (*ctx).counter_width = width;
}
unsafe extern "C" fn ctr_next_block(mut vctx: *mut ::core::ffi::c_void, mut out: *mut uint8_t) {
    let mut ctx: *mut cf_ctr = vctx as *mut cf_ctr;
    (*(*ctx).prp).encrypt.expect("non-null function pointer")(
        (*ctx).prpctx,
        &raw mut (*ctx).nonce as *mut uint8_t,
        out,
    );
    incr_be(
        (&raw mut (*ctx).nonce as *mut uint8_t).offset((*ctx).counter_offset as isize),
        (*ctx).counter_width,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_ctr_cipher(
    mut ctx: *mut cf_ctr,
    mut input: *const uint8_t,
    mut output: *mut uint8_t,
    mut bytes: size_t,
) {
    cf_blockwise_xor(
        &raw mut (*ctx).keymat as *mut uint8_t,
        &raw mut (*ctx).nkeymat,
        (*(*ctx).prp).blocksz,
        input as *const ::core::ffi::c_void,
        output as *mut ::core::ffi::c_void,
        bytes,
        Some(ctr_next_block as unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()),
        ctx as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_ctr_discard_block(mut ctx: *mut cf_ctr) {
    (*ctx).nkeymat = 0 as size_t;
}

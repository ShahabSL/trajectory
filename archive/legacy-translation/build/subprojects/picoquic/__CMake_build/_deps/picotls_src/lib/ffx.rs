extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
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
    fn ptls_cipher_new(
        algo: *const ptls_cipher_algorithm_t,
        is_enc: ::core::ffi::c_int,
        key: *const ::core::ffi::c_void,
    ) -> *mut ptls_cipher_context_t;
    fn ptls_cipher_free(ctx: *mut ptls_cipher_context_t);
    static mut ptls_clear_memory:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
}
pub type __uint8_t = u8;
pub type size_t = usize;
pub type uint8_t = __uint8_t;
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
pub type ptls_cipher_algorithm_t = st_ptls_cipher_algorithm_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_ffx_context_t {
    pub super_0: ptls_cipher_context_t,
    pub enc_ctx: *mut ptls_cipher_context_t,
    pub nb_rounds: ::core::ffi::c_int,
    pub is_enc: ::core::ffi::c_int,
    pub byte_length: size_t,
    pub nb_left: size_t,
    pub nb_right: size_t,
    pub mask_last_byte: uint8_t,
    pub tweaks: [uint8_t; 16],
}
pub type ptls_ffx_context_t = st_ptls_ffx_context_t;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const PTLS_ERROR_CLASS_INTERNAL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const PTLS_ERROR_LIBRARY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 3 as ::core::ffi::c_int;
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
#[no_mangle]
pub unsafe extern "C" fn ptls_ffx_setup_crypto(
    mut _ctx: *mut ptls_cipher_context_t,
    mut algo: *const ptls_cipher_algorithm_t,
    mut is_enc: ::core::ffi::c_int,
    mut nb_rounds: ::core::ffi::c_int,
    mut bit_length: size_t,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ctx: *mut ptls_ffx_context_t = _ctx as *mut ptls_ffx_context_t;
    let mut enc_ctx: *mut ptls_cipher_context_t = ::core::ptr::null_mut::<ptls_cipher_context_t>();
    let mut len: size_t = bit_length
        .wrapping_add(7 as size_t)
        .wrapping_div(8 as size_t);
    let mut last_byte_mask: [uint8_t; 8] = [
        0xff as ::core::ffi::c_int as uint8_t,
        0xfe as ::core::ffi::c_int as uint8_t,
        0xfc as ::core::ffi::c_int as uint8_t,
        0xf8 as ::core::ffi::c_int as uint8_t,
        0xf0 as ::core::ffi::c_int as uint8_t,
        0xe0 as ::core::ffi::c_int as uint8_t,
        0xc0 as ::core::ffi::c_int as uint8_t,
        0x80 as ::core::ffi::c_int as uint8_t,
    ];
    if len <= 32 as size_t && len >= 2 as size_t {
        enc_ctx = ptls_cipher_new(algo, 1 as ::core::ffi::c_int, key);
        if enc_ctx.is_null() {
            ret = PTLS_ERROR_LIBRARY;
        }
    } else {
        ret = PTLS_ERROR_LIBRARY;
    }
    if ret == 0 as ::core::ffi::c_int {
        (*ctx).enc_ctx = enc_ctx;
        (*ctx).nb_rounds = nb_rounds;
        (*ctx).is_enc = is_enc;
        (*ctx).byte_length = len;
        (*ctx).nb_left = (len as ::core::ffi::c_int / 2 as ::core::ffi::c_int) as size_t;
        (*ctx).nb_right = (len as ::core::ffi::c_int as size_t).wrapping_sub((*ctx).nb_left);
        (*ctx).mask_last_byte = last_byte_mask[bit_length.wrapping_rem(8 as size_t) as usize];
        ptls_clear_memory.expect("non-null function pointer")(
            &raw mut (*ctx).tweaks as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
        );
        (*ctx).super_0.do_dispose =
            Some(ffx_dispose as unsafe extern "C" fn(*mut ptls_cipher_context_t) -> ())
                as Option<unsafe extern "C" fn(*mut st_ptls_cipher_context_t) -> ()>;
        (*ctx).super_0.do_init = Some(
            ffx_init
                as unsafe extern "C" fn(
                    *mut st_ptls_cipher_context_t,
                    *const ::core::ffi::c_void,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut st_ptls_cipher_context_t,
                    *const ::core::ffi::c_void,
                ) -> (),
            >;
        (*ctx).super_0.do_transform = Some(
            ffx_encrypt
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
    } else {
        ffx_dispose(_ctx);
    }
    return ret;
}
unsafe extern "C" fn ffx_dispose(mut _ctx: *mut ptls_cipher_context_t) {
    let mut ctx: *mut ptls_ffx_context_t = _ctx as *mut ptls_ffx_context_t;
    if !(*ctx).enc_ctx.is_null() {
        ptls_cipher_free((*ctx).enc_ctx);
    }
    (*ctx).enc_ctx = ::core::ptr::null_mut::<ptls_cipher_context_t>();
    (*ctx).nb_rounds = 0 as ::core::ffi::c_int;
    (*ctx).byte_length = 0 as size_t;
    (*ctx).nb_left = 0 as size_t;
    (*ctx).nb_right = 0 as size_t;
    (*ctx).mask_last_byte = 0 as uint8_t;
    (*ctx).is_enc = 0 as ::core::ffi::c_int;
    (*ctx).super_0.do_dispose = None;
    (*ctx).super_0.do_init = None;
    (*ctx).super_0.do_transform = None;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_ffx_new(
    mut algo: *const ptls_cipher_algorithm_t,
    mut is_enc: ::core::ffi::c_int,
    mut nb_rounds: ::core::ffi::c_int,
    mut bit_length: size_t,
    mut key: *const ::core::ffi::c_void,
) -> *mut ptls_cipher_context_t {
    let mut ctx: *mut ptls_cipher_context_t =
        malloc(::core::mem::size_of::<ptls_ffx_context_t>() as size_t)
            as *mut ptls_cipher_context_t;
    if !ctx.is_null() {
        memset(
            ctx as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<ptls_ffx_context_t>() as size_t,
        );
        if ptls_ffx_setup_crypto(ctx, algo, is_enc, nb_rounds, bit_length, key)
            != 0 as ::core::ffi::c_int
        {
            free(ctx as *mut ::core::ffi::c_void);
            ctx = ::core::ptr::null_mut::<ptls_cipher_context_t>();
        }
    }
    return ctx;
}
unsafe extern "C" fn ptls_ffx_one_pass(
    mut enc_ctx: *mut ptls_cipher_context_t,
    mut source: *mut uint8_t,
    mut source_size: size_t,
    mut target: *mut uint8_t,
    mut target_size: size_t,
    mut mask_last_byte: uint8_t,
    mut confusion: *mut uint8_t,
    mut iv: *mut uint8_t,
    mut tweaks: *mut uint8_t,
    mut round: uint8_t,
    mut nb_rounds: uint8_t,
) {
    static mut zeros: [uint8_t; 16] = [
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
        0 as ::core::ffi::c_int as uint8_t,
    ];
    memcpy(
        iv as *mut ::core::ffi::c_void,
        tweaks as *const ::core::ffi::c_void,
        16 as size_t,
    );
    let ref mut c2rust_fresh0 =
        *iv.offset((round as ::core::ffi::c_int & 15 as ::core::ffi::c_int) as isize);
    *c2rust_fresh0 =
        (*c2rust_fresh0 as ::core::ffi::c_int ^ nb_rounds as ::core::ffi::c_int) as uint8_t;
    let mut i: size_t = 0 as size_t;
    while i < source_size {
        let ref mut c2rust_fresh1 = *iv.offset(i as isize);
        *c2rust_fresh1 = (*c2rust_fresh1 as ::core::ffi::c_int
            ^ *source.offset(i as isize) as ::core::ffi::c_int) as uint8_t;
        i = i.wrapping_add(1);
    }
    ptls_cipher_init(enc_ctx, iv as *const ::core::ffi::c_void);
    ptls_cipher_encrypt(
        enc_ctx,
        confusion as *mut ::core::ffi::c_void,
        &raw const zeros as *const uint8_t as *const ::core::ffi::c_void,
        16 as size_t,
    );
    let mut j: size_t = 0 as size_t;
    while j < target_size.wrapping_sub(1 as size_t) {
        let ref mut c2rust_fresh2 = *target.offset(j as isize);
        *c2rust_fresh2 = (*c2rust_fresh2 as ::core::ffi::c_int
            ^ *confusion.offset(j as isize) as ::core::ffi::c_int)
            as uint8_t;
        j = j.wrapping_add(1);
    }
    let ref mut c2rust_fresh3 = *target.offset(target_size.wrapping_sub(1 as size_t) as isize);
    *c2rust_fresh3 = (*c2rust_fresh3 as ::core::ffi::c_int
        ^ *confusion.offset(target_size.wrapping_sub(1 as size_t) as isize) as ::core::ffi::c_int
            & mask_last_byte as ::core::ffi::c_int) as uint8_t;
}
unsafe extern "C" fn ffx_encrypt(
    mut _ctx: *mut ptls_cipher_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut ctx: *mut ptls_ffx_context_t = _ctx as *mut ptls_ffx_context_t;
    let mut left: [uint8_t; 16] = [0; 16];
    let mut right: [uint8_t; 16] = [0; 16];
    let mut confusion: [uint8_t; 32] = [0; 32];
    let mut iv: [uint8_t; 16] = [0; 16];
    let mut last_byte: uint8_t = 0;
    if len != (*ctx).byte_length {
        memset(output, 0 as ::core::ffi::c_int, len);
        return;
    }
    memcpy(
        &raw mut left as *mut uint8_t as *mut ::core::ffi::c_void,
        input,
        (*ctx).nb_left,
    );
    memcpy(
        &raw mut right as *mut uint8_t as *mut ::core::ffi::c_void,
        (input as *mut uint8_t).offset((*ctx).nb_left as isize) as *const ::core::ffi::c_void,
        (*ctx).nb_right,
    );
    memset(
        (&raw mut left as *mut uint8_t).offset((*ctx).nb_left as isize) as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (16 as size_t).wrapping_sub((*ctx).nb_left),
    );
    memset(
        (&raw mut right as *mut uint8_t).offset((*ctx).nb_right as isize)
            as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        (16 as size_t).wrapping_sub((*ctx).nb_right),
    );
    last_byte = right[(*ctx).nb_right.wrapping_sub(1 as size_t) as usize];
    right[(*ctx).nb_right.wrapping_sub(1 as size_t) as usize] =
        (right[(*ctx).nb_right.wrapping_sub(1 as size_t) as usize] as ::core::ffi::c_int
            & (*ctx).mask_last_byte as ::core::ffi::c_int) as uint8_t;
    if (*ctx).is_enc != 0 {
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < (*ctx).nb_rounds {
            ptls_ffx_one_pass(
                (*ctx).enc_ctx,
                &raw mut right as *mut uint8_t,
                (*ctx).nb_right,
                &raw mut left as *mut uint8_t,
                (*ctx).nb_left,
                0xff as uint8_t,
                &raw mut confusion as *mut uint8_t,
                &raw mut iv as *mut uint8_t,
                &raw mut (*ctx).tweaks as *mut uint8_t,
                i as uint8_t,
                (*ctx).nb_rounds as uint8_t,
            );
            ptls_ffx_one_pass(
                (*ctx).enc_ctx,
                &raw mut left as *mut uint8_t,
                (*ctx).nb_left,
                &raw mut right as *mut uint8_t,
                (*ctx).nb_right,
                (*ctx).mask_last_byte,
                &raw mut confusion as *mut uint8_t,
                &raw mut iv as *mut uint8_t,
                &raw mut (*ctx).tweaks as *mut uint8_t,
                (i + 1 as ::core::ffi::c_int) as uint8_t,
                (*ctx).nb_rounds as uint8_t,
            );
            i += 2 as ::core::ffi::c_int;
        }
    } else {
        let mut i_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i_0 < (*ctx).nb_rounds {
            ptls_ffx_one_pass(
                (*ctx).enc_ctx,
                &raw mut left as *mut uint8_t,
                (*ctx).nb_left,
                &raw mut right as *mut uint8_t,
                (*ctx).nb_right,
                (*ctx).mask_last_byte,
                &raw mut confusion as *mut uint8_t,
                &raw mut iv as *mut uint8_t,
                &raw mut (*ctx).tweaks as *mut uint8_t,
                ((*ctx).nb_rounds - 1 as ::core::ffi::c_int - i_0) as uint8_t,
                (*ctx).nb_rounds as uint8_t,
            );
            ptls_ffx_one_pass(
                (*ctx).enc_ctx,
                &raw mut right as *mut uint8_t,
                (*ctx).nb_right,
                &raw mut left as *mut uint8_t,
                (*ctx).nb_left,
                0xff as uint8_t,
                &raw mut confusion as *mut uint8_t,
                &raw mut iv as *mut uint8_t,
                &raw mut (*ctx).tweaks as *mut uint8_t,
                ((*ctx).nb_rounds - 2 as ::core::ffi::c_int - i_0) as uint8_t,
                (*ctx).nb_rounds as uint8_t,
            );
            i_0 += 2 as ::core::ffi::c_int;
        }
    }
    memcpy(
        output,
        &raw mut left as *mut uint8_t as *const ::core::ffi::c_void,
        (*ctx).nb_left,
    );
    right[(*ctx).nb_right.wrapping_sub(1 as size_t) as usize] =
        (right[(*ctx).nb_right.wrapping_sub(1 as size_t) as usize] as ::core::ffi::c_int
            & (*ctx).mask_last_byte as ::core::ffi::c_int) as uint8_t;
    right[(*ctx).nb_right.wrapping_sub(1 as size_t) as usize] =
        (right[(*ctx).nb_right.wrapping_sub(1 as size_t) as usize] as ::core::ffi::c_int
            | last_byte as ::core::ffi::c_int & !((*ctx).mask_last_byte as ::core::ffi::c_int))
            as uint8_t;
    memcpy(
        (output as *mut uint8_t).offset((*ctx).nb_left as isize) as *mut ::core::ffi::c_void,
        &raw mut right as *mut uint8_t as *const ::core::ffi::c_void,
        (*ctx).nb_right,
    );
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut left as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut right as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
    );
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut confusion as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
}
unsafe extern "C" fn ffx_init(
    mut _ctx: *mut st_ptls_cipher_context_t,
    mut iv: *const ::core::ffi::c_void,
) {
    let mut ctx: *mut ptls_ffx_context_t = _ctx as *mut ptls_ffx_context_t;
    memcpy(
        &raw mut (*ctx).tweaks as *mut uint8_t as *mut ::core::ffi::c_void,
        iv,
        16 as size_t,
    );
}

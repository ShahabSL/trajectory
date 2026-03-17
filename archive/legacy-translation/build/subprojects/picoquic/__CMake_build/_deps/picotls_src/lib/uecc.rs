extern "C" {
    pub type uECC_Curve_t;
    pub type st_ptls_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memcpy(
        __dest: *mut ::core::ffi::c_void,
        __src: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn cf_sha256_init(ctx: *mut cf_sha256_context);
    fn cf_sha256_update(
        ctx: *mut cf_sha256_context,
        data: *const ::core::ffi::c_void,
        nbytes: size_t,
    );
    fn cf_sha256_digest_final(ctx: *mut cf_sha256_context, hash: *mut uint8_t);
    fn uECC_secp256r1() -> uECC_Curve;
    fn uECC_make_key(
        public_key: *mut uint8_t,
        private_key: *mut uint8_t,
        curve: uECC_Curve,
    ) -> ::core::ffi::c_int;
    fn uECC_shared_secret(
        public_key: *const uint8_t,
        private_key: *const uint8_t,
        secret: *mut uint8_t,
        curve: uECC_Curve,
    ) -> ::core::ffi::c_int;
    fn uECC_sign(
        private_key: *const uint8_t,
        message_hash: *const uint8_t,
        hash_size: ::core::ffi::c_uint,
        signature: *mut uint8_t,
        curve: uECC_Curve,
    ) -> ::core::ffi::c_int;
    fn ptls_buffer__do_pushv(
        buf: *mut ptls_buffer_t,
        src: *const ::core::ffi::c_void,
        len: size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_buffer__adjust_asn1_blocksize(
        buf: *mut ptls_buffer_t,
        body_size: size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_buffer_push_asn1_ubigint(
        buf: *mut ptls_buffer_t,
        bignum: *const ::core::ffi::c_void,
        size: size_t,
    ) -> ::core::ffi::c_int;
    static mut ptls_clear_memory:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __intptr_t = ::core::ffi::c_long;
pub type size_t = usize;
pub type intptr_t = __intptr_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_sha256_context {
    pub H: [uint32_t; 8],
    pub partial: [uint8_t; 64],
    pub blocks: uint32_t,
    pub npartial: size_t,
}
pub type uECC_Curve = *const uECC_Curve_t;
pub type ptls_t = st_ptls_t;
pub type ptls_iovec_t = st_ptls_iovec_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_iovec_t {
    pub base: *mut uint8_t,
    pub len: size_t,
}
pub type ptls_buffer_t = st_ptls_buffer_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_buffer_t {
    pub base: *mut uint8_t,
    pub capacity: size_t,
    pub off: size_t,
    pub is_allocated: uint8_t,
    pub align_bits: uint8_t,
}
pub type ptls_sign_certificate_t = st_ptls_sign_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_sign_certificate_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_sign_certificate_t,
            *mut ptls_t,
            *mut *mut ptls_async_job_t,
            *mut uint16_t,
            *mut ptls_buffer_t,
            ptls_iovec_t,
            *const uint16_t,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_async_job_t = st_ptls_async_job_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_async_job_t {
    pub destroy_: Option<unsafe extern "C" fn(*mut st_ptls_async_job_t) -> ()>,
    pub get_fd: Option<unsafe extern "C" fn(*mut st_ptls_async_job_t) -> ::core::ffi::c_int>,
    pub set_completion_callback: Option<
        unsafe extern "C" fn(
            *mut st_ptls_async_job_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
}
pub type ptls_key_exchange_algorithm_t = st_ptls_key_exchange_algorithm_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_key_exchange_algorithm_t {
    pub id: uint16_t,
    pub create: Option<
        unsafe extern "C" fn(
            *const st_ptls_key_exchange_algorithm_t,
            *mut *mut ptls_key_exchange_context_t,
        ) -> ::core::ffi::c_int,
    >,
    pub exchange: Option<
        unsafe extern "C" fn(
            *const st_ptls_key_exchange_algorithm_t,
            *mut ptls_iovec_t,
            *mut ptls_iovec_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
    pub data: intptr_t,
    pub name: *const ::core::ffi::c_char,
}
pub type ptls_key_exchange_context_t = st_ptls_key_exchange_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_key_exchange_context_t {
    pub algo: *const st_ptls_key_exchange_algorithm_t,
    pub pubkey: ptls_iovec_t,
    pub on_exchange: Option<
        unsafe extern "C" fn(
            *mut *mut st_ptls_key_exchange_context_t,
            ::core::ffi::c_int,
            *mut ptls_iovec_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_minicrypto_secp256r1sha256_sign_certificate_t {
    pub super_0: ptls_sign_certificate_t,
    pub key: [uint8_t; 32],
}
pub type ptls_minicrypto_secp256r1sha256_sign_certificate_t =
    st_ptls_minicrypto_secp256r1sha256_sign_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_secp256r1_key_exhchange_t {
    pub super_0: ptls_key_exchange_context_t,
    pub priv_0: [uint8_t; 32],
    pub pub_0: [uint8_t; 65],
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const PTLS_GROUP_SECP256R1: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const PTLS_GROUP_NAME_SECP256R1: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"secp256r1\0") };
pub const PTLS_SIGNATURE_ECDSA_SECP256R1_SHA256: ::core::ffi::c_int = 0x403 as ::core::ffi::c_int;
pub const PTLS_ERROR_CLASS_INTERNAL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const PTLS_ALERT_HANDSHAKE_FAILURE: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const PTLS_ALERT_DECRYPT_ERROR: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const PTLS_ERROR_NO_MEMORY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 1 as ::core::ffi::c_int;
pub const PTLS_ERROR_INCOMPATIBLE_KEY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 4 as ::core::ffi::c_int;
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
pub const SECP256R1_PUBLIC_KEY_SIZE: ::core::ffi::c_int = 65 as ::core::ffi::c_int;
pub const SECP256R1_SHARED_SECRET_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const TYPE_UNCOMPRESSED_PUBLIC_KEY: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn secp256r1_on_exchange(
    mut _ctx: *mut *mut ptls_key_exchange_context_t,
    mut release: ::core::ffi::c_int,
    mut secret: *mut ptls_iovec_t,
    mut peerkey: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut ctx: *mut st_secp256r1_key_exhchange_t = *_ctx as *mut st_secp256r1_key_exhchange_t;
    let mut secbytes: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut ret: ::core::ffi::c_int = 0;
    if secret.is_null() {
        ret = 0 as ::core::ffi::c_int;
    } else if peerkey.len != SECP256R1_PUBLIC_KEY_SIZE as size_t
        || *peerkey.base.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != TYPE_UNCOMPRESSED_PUBLIC_KEY
    {
        ret = PTLS_ALERT_DECRYPT_ERROR;
    } else {
        secbytes = malloc(SECP256R1_SHARED_SECRET_SIZE as size_t) as *mut uint8_t;
        if secbytes.is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        } else if uECC_shared_secret(
            peerkey.base.offset(1 as ::core::ffi::c_int as isize),
            &raw mut (*ctx).priv_0 as *mut uint8_t,
            secbytes,
            uECC_secp256r1(),
        ) == 0
        {
            ret = PTLS_ALERT_DECRYPT_ERROR;
        } else {
            *secret = ptls_iovec_init(
                secbytes as *const ::core::ffi::c_void,
                SECP256R1_SHARED_SECRET_SIZE as size_t,
            );
            ret = 0 as ::core::ffi::c_int;
        }
    }
    if ret != 0 as ::core::ffi::c_int {
        free(secbytes as *mut ::core::ffi::c_void);
    }
    if release != 0 {
        ptls_clear_memory.expect("non-null function pointer")(
            &raw mut (*ctx).priv_0 as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        );
        free(ctx as *mut ::core::ffi::c_void);
        *_ctx = ::core::ptr::null_mut::<ptls_key_exchange_context_t>();
    }
    return ret;
}
unsafe extern "C" fn secp256r1_create_key_exchange(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut _ctx: *mut *mut ptls_key_exchange_context_t,
) -> ::core::ffi::c_int {
    let mut ctx: *mut st_secp256r1_key_exhchange_t =
        ::core::ptr::null_mut::<st_secp256r1_key_exhchange_t>();
    ctx = malloc(::core::mem::size_of::<st_secp256r1_key_exhchange_t>() as size_t)
        as *mut st_secp256r1_key_exhchange_t;
    if ctx.is_null() {
        return PTLS_ERROR_NO_MEMORY;
    }
    (*ctx).super_0 = st_ptls_key_exchange_context_t {
        algo: algo as *const st_ptls_key_exchange_algorithm_t,
        pubkey: ptls_iovec_init(
            &raw mut (*ctx).pub_0 as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 65]>() as size_t,
        ),
        on_exchange: Some(
            secp256r1_on_exchange
                as unsafe extern "C" fn(
                    *mut *mut ptls_key_exchange_context_t,
                    ::core::ffi::c_int,
                    *mut ptls_iovec_t,
                    ptls_iovec_t,
                ) -> ::core::ffi::c_int,
        ),
    };
    (*ctx).pub_0[0 as ::core::ffi::c_int as usize] = TYPE_UNCOMPRESSED_PUBLIC_KEY as uint8_t;
    uECC_make_key(
        (&raw mut (*ctx).pub_0 as *mut uint8_t).offset(1 as ::core::ffi::c_int as isize),
        &raw mut (*ctx).priv_0 as *mut uint8_t,
        uECC_secp256r1(),
    );
    *_ctx = &raw mut (*ctx).super_0;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn secp256r1_key_exchange(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut pubkey: *mut ptls_iovec_t,
    mut secret: *mut ptls_iovec_t,
    mut peerkey: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut priv_0: [uint8_t; 32] = [0; 32];
    let mut pub_0: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut secbytes: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut ret: ::core::ffi::c_int = 0;
    if peerkey.len != SECP256R1_PUBLIC_KEY_SIZE as size_t
        || *peerkey.base.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != TYPE_UNCOMPRESSED_PUBLIC_KEY
    {
        ret = PTLS_ALERT_DECRYPT_ERROR;
    } else {
        pub_0 = malloc(SECP256R1_PUBLIC_KEY_SIZE as size_t) as *mut uint8_t;
        if pub_0.is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        } else {
            secbytes = malloc(SECP256R1_SHARED_SECRET_SIZE as size_t) as *mut uint8_t;
            if secbytes.is_null() {
                ret = PTLS_ERROR_NO_MEMORY;
            } else {
                *pub_0.offset(0 as ::core::ffi::c_int as isize) =
                    TYPE_UNCOMPRESSED_PUBLIC_KEY as uint8_t;
                uECC_make_key(
                    pub_0.offset(1 as ::core::ffi::c_int as isize),
                    &raw mut priv_0 as *mut uint8_t,
                    uECC_secp256r1(),
                );
                if uECC_shared_secret(
                    peerkey.base.offset(1 as ::core::ffi::c_int as isize),
                    &raw mut priv_0 as *mut uint8_t,
                    secbytes,
                    uECC_secp256r1(),
                ) == 0
                {
                    ret = PTLS_ALERT_DECRYPT_ERROR;
                } else {
                    *pubkey = ptls_iovec_init(
                        pub_0 as *const ::core::ffi::c_void,
                        SECP256R1_PUBLIC_KEY_SIZE as size_t,
                    );
                    *secret = ptls_iovec_init(
                        secbytes as *const ::core::ffi::c_void,
                        SECP256R1_SHARED_SECRET_SIZE as size_t,
                    );
                    ret = 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut priv_0 as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    if ret != 0 as ::core::ffi::c_int {
        free(secbytes as *mut ::core::ffi::c_void);
        free(pub_0 as *mut ::core::ffi::c_void);
    }
    return ret;
}
unsafe extern "C" fn secp256r1sha256_sign(
    mut _self: *mut ptls_sign_certificate_t,
    mut tls: *mut ptls_t,
    mut async_0: *mut *mut ptls_async_job_t,
    mut selected_algorithm: *mut uint16_t,
    mut outbuf: *mut ptls_buffer_t,
    mut input: ptls_iovec_t,
    mut algorithms: *const uint16_t,
    mut num_algorithms: size_t,
) -> ::core::ffi::c_int {
    let mut c2rust_current_block: u64;
    let mut self_0: *mut ptls_minicrypto_secp256r1sha256_sign_certificate_t =
        _self as *mut ptls_minicrypto_secp256r1sha256_sign_certificate_t;
    let mut hash: [uint8_t; 32] = [0; 32];
    let mut sig: [uint8_t; 64] = [0; 64];
    let mut i: size_t = 0;
    let mut ret: ::core::ffi::c_int = 0;
    i = 0 as size_t;
    while i != num_algorithms {
        if *algorithms.offset(i as isize) as ::core::ffi::c_int
            == PTLS_SIGNATURE_ECDSA_SECP256R1_SHA256
        {
            break;
        }
        i = i.wrapping_add(1);
    }
    if i == num_algorithms {
        return PTLS_ALERT_HANDSHAKE_FAILURE;
    }
    let mut ctx: cf_sha256_context = cf_sha256_context {
        H: [0; 8],
        partial: [0; 64],
        blocks: 0,
        npartial: 0,
    };
    cf_sha256_init(&raw mut ctx);
    cf_sha256_update(
        &raw mut ctx,
        input.base as *const ::core::ffi::c_void,
        input.len,
    );
    cf_sha256_digest_final(&raw mut ctx, &raw mut hash as *mut uint8_t);
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut ctx as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<cf_sha256_context>() as size_t,
    );
    uECC_sign(
        &raw mut (*self_0).key as *mut uint8_t,
        &raw mut hash as *mut uint8_t,
        ::core::mem::size_of::<[uint8_t; 32]>() as ::core::ffi::c_uint,
        &raw mut sig as *mut uint8_t,
        uECC_secp256r1(),
    );
    let mut c2rust_fresh0: [uint8_t; 1] = [0x30 as ::core::ffi::c_int as uint8_t];
    ret = ptls_buffer__do_pushv(
        outbuf,
        &raw mut c2rust_fresh0 as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 1]>() as size_t,
    );
    if !(ret != 0 as ::core::ffi::c_int) {
        let mut c2rust_fresh1: [uint8_t; 1] = [0xff as ::core::ffi::c_int as uint8_t];
        ret = ptls_buffer__do_pushv(
            outbuf,
            &raw mut c2rust_fresh1 as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 1]>() as size_t,
        );
        if !(ret != 0 as ::core::ffi::c_int) {
            let mut body_start: size_t = (*outbuf).off;
            ret = ptls_buffer_push_asn1_ubigint(
                outbuf,
                &raw mut sig as *mut uint8_t as *const ::core::ffi::c_void,
                32 as size_t,
            );
            if !(ret != 0 as ::core::ffi::c_int) {
                ret = ptls_buffer_push_asn1_ubigint(
                    outbuf,
                    (&raw mut sig as *mut uint8_t).offset(32 as ::core::ffi::c_int as isize)
                        as *const ::core::ffi::c_void,
                    32 as size_t,
                );
                if !(ret != 0 as ::core::ffi::c_int) {
                    let mut body_size: size_t = (*outbuf).off.wrapping_sub(body_start);
                    if body_size < 128 as size_t {
                        *(*outbuf)
                            .base
                            .offset(body_start.wrapping_sub(1 as size_t) as isize) =
                            body_size as uint8_t;
                        c2rust_current_block = 15925075030174552612;
                    } else {
                        ret = ptls_buffer__adjust_asn1_blocksize(outbuf, body_size);
                        if ret != 0 as ::core::ffi::c_int {
                            c2rust_current_block = 14389591666719277441;
                        } else {
                            c2rust_current_block = 15925075030174552612;
                        }
                    }
                    match c2rust_current_block {
                        14389591666719277441 => {}
                        _ => {
                            *selected_algorithm = PTLS_SIGNATURE_ECDSA_SECP256R1_SHA256 as uint16_t;
                            ret = 0 as ::core::ffi::c_int;
                        }
                    }
                }
            }
        }
    }
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut hash as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut sig as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_minicrypto_init_secp256r1sha256_sign_certificate(
    mut self_0: *mut ptls_minicrypto_secp256r1sha256_sign_certificate_t,
    mut key: ptls_iovec_t,
) -> ::core::ffi::c_int {
    if key.len != ::core::mem::size_of::<[uint8_t; 32]>() as usize {
        return PTLS_ERROR_INCOMPATIBLE_KEY;
    }
    (*self_0).super_0.cb = Some(
        secp256r1sha256_sign
            as unsafe extern "C" fn(
                *mut ptls_sign_certificate_t,
                *mut ptls_t,
                *mut *mut ptls_async_job_t,
                *mut uint16_t,
                *mut ptls_buffer_t,
                ptls_iovec_t,
                *const uint16_t,
                size_t,
            ) -> ::core::ffi::c_int,
    )
        as Option<
            unsafe extern "C" fn(
                *mut st_ptls_sign_certificate_t,
                *mut ptls_t,
                *mut *mut ptls_async_job_t,
                *mut uint16_t,
                *mut ptls_buffer_t,
                ptls_iovec_t,
                *const uint16_t,
                size_t,
            ) -> ::core::ffi::c_int,
        >;
    memcpy(
        &raw mut (*self_0).key as *mut uint8_t as *mut ::core::ffi::c_void,
        key.base as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub static mut ptls_minicrypto_secp256r1: ptls_key_exchange_algorithm_t = unsafe {
    st_ptls_key_exchange_algorithm_t {
        id: PTLS_GROUP_SECP256R1 as uint16_t,
        create: Some(
            secp256r1_create_key_exchange
                as unsafe extern "C" fn(
                    *const ptls_key_exchange_algorithm_t,
                    *mut *mut ptls_key_exchange_context_t,
                ) -> ::core::ffi::c_int,
        ),
        exchange: Some(
            secp256r1_key_exchange
                as unsafe extern "C" fn(
                    *const ptls_key_exchange_algorithm_t,
                    *mut ptls_iovec_t,
                    *mut ptls_iovec_t,
                    ptls_iovec_t,
                ) -> ::core::ffi::c_int,
        ),
        data: 0,
        name: PTLS_GROUP_NAME_SECP256R1.as_ptr(),
    }
};
#[no_mangle]
pub static mut ptls_minicrypto_key_exchanges: [*const ptls_key_exchange_algorithm_t; 2] = unsafe {
    [
        &raw const ptls_minicrypto_secp256r1,
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
    ]
};

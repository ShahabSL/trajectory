extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn cf_curve25519_mul(out: *mut uint8_t, scalar: *const uint8_t, point: *const uint8_t);
    fn cf_curve25519_mul_base(out: *mut uint8_t, scalar: *const uint8_t);
    static mut ptls_clear_memory:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
    static mut ptls_mem_equal: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            size_t,
        ) -> ::core::ffi::c_int,
    >;
    fn ptls_minicrypto_random_bytes(buf: *mut ::core::ffi::c_void, len: size_t);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type intptr_t = isize;
pub type ptls_iovec_t = st_ptls_iovec_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_iovec_t {
    pub base: *mut uint8_t,
    pub len: size_t,
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
pub struct st_x25519_key_exchange_t {
    pub super_0: ptls_key_exchange_context_t,
    pub priv_0: [uint8_t; 32],
    pub pub_0: [uint8_t; 32],
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const PTLS_GROUP_X25519: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const PTLS_GROUP_NAME_X25519: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"x25519\0") };
pub const PTLS_ERROR_CLASS_INTERNAL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
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
pub const X25519_KEY_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
unsafe extern "C" fn x25519_create_keypair(mut priv_0: *mut uint8_t, mut pub_0: *mut uint8_t) {
    ptls_minicrypto_random_bytes(
        priv_0 as *mut ::core::ffi::c_void,
        X25519_KEY_SIZE as size_t,
    );
    cf_curve25519_mul_base(pub_0 as *mut uint8_t, priv_0 as *const uint8_t);
}
unsafe extern "C" fn x25519_derive_secret(
    mut secret: *mut ptls_iovec_t,
    mut clientpriv: *const uint8_t,
    mut clientpub: *const uint8_t,
    mut serverpriv: *const uint8_t,
    mut serverpub: *const uint8_t,
) -> ::core::ffi::c_int {
    (*secret).base = malloc(X25519_KEY_SIZE as size_t) as *mut uint8_t;
    if (*secret).base.is_null() {
        return PTLS_ERROR_NO_MEMORY;
    }
    cf_curve25519_mul(
        (*secret).base as *mut uint8_t,
        if !clientpriv.is_null() {
            clientpriv as *const uint8_t
        } else {
            serverpriv as *const uint8_t
        },
        if !clientpriv.is_null() {
            serverpub as *const uint8_t
        } else {
            clientpub as *const uint8_t
        },
    );
    static mut zeros: [uint8_t; 32] = [
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
        0,
    ];
    if ptls_mem_equal.expect("non-null function pointer")(
        (*secret).base as *const ::core::ffi::c_void,
        &raw const zeros as *const uint8_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    ) != 0
    {
        free((*secret).base as *mut ::core::ffi::c_void);
        return PTLS_ERROR_INCOMPATIBLE_KEY;
    }
    (*secret).len = X25519_KEY_SIZE as size_t;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn x25519_on_exchange(
    mut _ctx: *mut *mut ptls_key_exchange_context_t,
    mut release: ::core::ffi::c_int,
    mut secret: *mut ptls_iovec_t,
    mut peerkey: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut ctx: *mut st_x25519_key_exchange_t = *_ctx as *mut st_x25519_key_exchange_t;
    let mut ret: ::core::ffi::c_int = 0;
    if secret.is_null() {
        ret = 0 as ::core::ffi::c_int;
    } else if peerkey.len != X25519_KEY_SIZE as size_t {
        ret = PTLS_ALERT_DECRYPT_ERROR;
    } else {
        ret = x25519_derive_secret(
            secret,
            &raw mut (*ctx).priv_0 as *mut uint8_t,
            &raw mut (*ctx).pub_0 as *mut uint8_t,
            ::core::ptr::null::<uint8_t>(),
            peerkey.base,
        );
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
unsafe extern "C" fn x25519_create_key_exchange(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut _ctx: *mut *mut ptls_key_exchange_context_t,
) -> ::core::ffi::c_int {
    let mut ctx: *mut st_x25519_key_exchange_t =
        ::core::ptr::null_mut::<st_x25519_key_exchange_t>();
    ctx = malloc(::core::mem::size_of::<st_x25519_key_exchange_t>() as size_t)
        as *mut st_x25519_key_exchange_t;
    if ctx.is_null() {
        return PTLS_ERROR_NO_MEMORY;
    }
    (*ctx).super_0 = st_ptls_key_exchange_context_t {
        algo: algo as *const st_ptls_key_exchange_algorithm_t,
        pubkey: ptls_iovec_init(
            &raw mut (*ctx).pub_0 as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
        ),
        on_exchange: Some(
            x25519_on_exchange
                as unsafe extern "C" fn(
                    *mut *mut ptls_key_exchange_context_t,
                    ::core::ffi::c_int,
                    *mut ptls_iovec_t,
                    ptls_iovec_t,
                ) -> ::core::ffi::c_int,
        ),
    };
    x25519_create_keypair(
        &raw mut (*ctx).priv_0 as *mut uint8_t,
        &raw mut (*ctx).pub_0 as *mut uint8_t,
    );
    *_ctx = &raw mut (*ctx).super_0;
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn x25519_key_exchange(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut pubkey: *mut ptls_iovec_t,
    mut secret: *mut ptls_iovec_t,
    mut peerkey: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut priv_0: [uint8_t; 32] = [0; 32];
    let mut pub_0: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut ret: ::core::ffi::c_int = 0;
    if peerkey.len != X25519_KEY_SIZE as size_t {
        ret = PTLS_ALERT_DECRYPT_ERROR;
    } else {
        pub_0 = malloc(X25519_KEY_SIZE as size_t) as *mut uint8_t;
        if pub_0.is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        } else {
            x25519_create_keypair(&raw mut priv_0 as *mut uint8_t, pub_0);
            ret = x25519_derive_secret(
                secret,
                ::core::ptr::null::<uint8_t>(),
                peerkey.base,
                &raw mut priv_0 as *mut uint8_t,
                pub_0,
            );
            if !(ret != 0 as ::core::ffi::c_int) {
                *pubkey = ptls_iovec_init(
                    pub_0 as *const ::core::ffi::c_void,
                    X25519_KEY_SIZE as size_t,
                );
                ret = 0 as ::core::ffi::c_int;
            }
        }
    }
    ptls_clear_memory.expect("non-null function pointer")(
        &raw mut priv_0 as *mut uint8_t as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    if !pub_0.is_null() && ret != 0 as ::core::ffi::c_int {
        ptls_clear_memory.expect("non-null function pointer")(
            pub_0 as *mut ::core::ffi::c_void,
            X25519_KEY_SIZE as size_t,
        );
        free(pub_0 as *mut ::core::ffi::c_void);
    }
    return ret;
}
#[no_mangle]
pub static mut ptls_minicrypto_x25519: ptls_key_exchange_algorithm_t = unsafe {
    st_ptls_key_exchange_algorithm_t {
        id: PTLS_GROUP_X25519 as uint16_t,
        create: Some(
            x25519_create_key_exchange
                as unsafe extern "C" fn(
                    *const ptls_key_exchange_algorithm_t,
                    *mut *mut ptls_key_exchange_context_t,
                ) -> ::core::ffi::c_int,
        ),
        exchange: Some(
            x25519_key_exchange
                as unsafe extern "C" fn(
                    *const ptls_key_exchange_algorithm_t,
                    *mut ptls_iovec_t,
                    *mut ptls_iovec_t,
                    ptls_iovec_t,
                ) -> ::core::ffi::c_int,
        ),
        data: 0,
        name: PTLS_GROUP_NAME_X25519.as_ptr(),
    }
};

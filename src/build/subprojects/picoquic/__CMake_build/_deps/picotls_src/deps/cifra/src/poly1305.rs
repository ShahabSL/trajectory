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
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_poly1305 {
    pub h: [uint32_t; 17],
    pub r: [uint32_t; 17],
    pub s: [uint8_t; 16],
    pub partial: [uint8_t; 16],
    pub npartial: size_t,
}
pub type cf_blockwise_in_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const uint8_t) -> ()>;
#[inline]
unsafe extern "C" fn mask_u32(mut x: uint32_t, mut y: uint32_t) -> uint32_t {
    let mut diff: uint32_t = x ^ y;
    let mut diff_is_zero: uint32_t = !diff & diff.wrapping_sub(1 as uint32_t);
    return -((diff_is_zero >> 31 as ::core::ffi::c_int) as int32_t) as uint32_t;
}
#[inline]
unsafe extern "C" fn mem_clean(mut v: *mut ::core::ffi::c_void, mut len: size_t) {
    if len != 0 {
        memset(v as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, len);
        *(v as *mut uint8_t);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cf_poly1305_init(
    mut ctx: *mut cf_poly1305,
    mut r: *const uint8_t,
    mut s: *const uint8_t,
) {
    memset(
        ctx as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cf_poly1305>() as size_t,
    );
    (*ctx).r[0 as ::core::ffi::c_int as usize] =
        *r.offset(0 as ::core::ffi::c_int as isize) as uint32_t;
    (*ctx).r[1 as ::core::ffi::c_int as usize] =
        *r.offset(1 as ::core::ffi::c_int as isize) as uint32_t;
    (*ctx).r[2 as ::core::ffi::c_int as usize] =
        *r.offset(2 as ::core::ffi::c_int as isize) as uint32_t;
    (*ctx).r[3 as ::core::ffi::c_int as usize] = (*r.offset(3 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int) as uint32_t;
    (*ctx).r[4 as ::core::ffi::c_int as usize] = (*r.offset(4 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & 0xfc as ::core::ffi::c_int) as uint32_t;
    (*ctx).r[5 as ::core::ffi::c_int as usize] =
        *r.offset(5 as ::core::ffi::c_int as isize) as uint32_t;
    (*ctx).r[6 as ::core::ffi::c_int as usize] =
        *r.offset(6 as ::core::ffi::c_int as isize) as uint32_t;
    (*ctx).r[7 as ::core::ffi::c_int as usize] = (*r.offset(7 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int) as uint32_t;
    (*ctx).r[8 as ::core::ffi::c_int as usize] = (*r.offset(8 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & 0xfc as ::core::ffi::c_int) as uint32_t;
    (*ctx).r[9 as ::core::ffi::c_int as usize] =
        *r.offset(9 as ::core::ffi::c_int as isize) as uint32_t;
    (*ctx).r[10 as ::core::ffi::c_int as usize] =
        *r.offset(10 as ::core::ffi::c_int as isize) as uint32_t;
    (*ctx).r[11 as ::core::ffi::c_int as usize] = (*r.offset(11 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int) as uint32_t;
    (*ctx).r[12 as ::core::ffi::c_int as usize] = (*r.offset(12 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & 0xfc as ::core::ffi::c_int) as uint32_t;
    (*ctx).r[13 as ::core::ffi::c_int as usize] =
        *r.offset(13 as ::core::ffi::c_int as isize) as uint32_t;
    (*ctx).r[14 as ::core::ffi::c_int as usize] =
        *r.offset(14 as ::core::ffi::c_int as isize) as uint32_t;
    (*ctx).r[15 as ::core::ffi::c_int as usize] = (*r.offset(15 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int) as uint32_t;
    (*ctx).r[16 as ::core::ffi::c_int as usize] = 0 as uint32_t;
    memcpy(
        &raw mut (*ctx).s as *mut uint8_t as *mut ::core::ffi::c_void,
        s as *const ::core::ffi::c_void,
        16 as size_t,
    );
}
unsafe extern "C" fn poly1305_add(mut h: *mut uint32_t, mut x: *const uint32_t) {
    let mut carry: uint32_t = 0 as uint32_t;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 17 as ::core::ffi::c_int {
        carry = carry.wrapping_add((*h.offset(i as isize)).wrapping_add(*x.offset(i as isize)));
        *h.offset(i as isize) = carry & 0xff as uint32_t;
        carry >>= 8 as ::core::ffi::c_int;
        i += 1;
    }
}
unsafe extern "C" fn poly1305_min_reduce(mut x: *mut uint32_t) {
    let mut carry: uint32_t = 0 as uint32_t;
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        carry = carry.wrapping_add(*x.offset(i as isize));
        *x.offset(i as isize) = carry & 0xff as uint32_t;
        carry >>= 8 as ::core::ffi::c_int;
        i += 1;
    }
    carry = carry.wrapping_add(*x.offset(16 as ::core::ffi::c_int as isize));
    *x.offset(16 as ::core::ffi::c_int as isize) = carry & 0x3 as uint32_t;
    carry = (5 as uint32_t).wrapping_mul(carry >> 2 as ::core::ffi::c_int);
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        carry = carry.wrapping_add(*x.offset(i as isize));
        *x.offset(i as isize) = carry & 0xff as uint32_t;
        carry >>= 8 as ::core::ffi::c_int;
        i += 1;
    }
    let ref mut c2rust_fresh0 = *x.offset(16 as ::core::ffi::c_int as isize);
    *c2rust_fresh0 = (*c2rust_fresh0).wrapping_add(carry);
}
static mut negative_1305: [uint32_t; 17] = [
    0x5 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0 as ::core::ffi::c_int as uint32_t,
    0xfc as ::core::ffi::c_int as uint32_t,
];
unsafe extern "C" fn poly1305_full_reduce(mut x: *mut uint32_t) {
    let mut xsub: [uint32_t; 17] = [0; 17];
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < 17 as size_t {
        xsub[i as usize] = *x.offset(i as isize);
        i = i.wrapping_add(1);
    }
    poly1305_add(
        &raw mut xsub as *mut uint32_t,
        &raw const negative_1305 as *const uint32_t,
    );
    let mut negative_mask: uint32_t = mask_u32(
        xsub[16 as ::core::ffi::c_int as usize] & 0x80 as uint32_t,
        0x80 as uint32_t,
    );
    let mut positive_mask: uint32_t = negative_mask ^ 0xffffffff as uint32_t;
    i = 0 as size_t;
    while i < 17 as size_t {
        *x.offset(i as isize) =
            *x.offset(i as isize) & negative_mask | xsub[i as usize] & positive_mask;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn poly1305_mul(mut x: *mut uint32_t, mut y: *const uint32_t) {
    let mut r: [uint32_t; 17] = [0; 17];
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 17 as ::core::ffi::c_int {
        let mut accum: uint32_t = 0 as uint32_t;
        let mut j: ::core::ffi::c_int = 0;
        j = 0 as ::core::ffi::c_int;
        while j <= i {
            accum = accum
                .wrapping_add((*x.offset(j as isize)).wrapping_mul(*y.offset((i - j) as isize)));
            j += 1;
        }
        j = i + 1 as ::core::ffi::c_int;
        while j < 17 as ::core::ffi::c_int {
            accum = accum.wrapping_add(
                (((5 as ::core::ffi::c_int) << 6 as ::core::ffi::c_int) as uint32_t)
                    .wrapping_mul(*x.offset(j as isize))
                    .wrapping_mul(*y.offset((i + 17 as ::core::ffi::c_int - j) as isize)),
            );
            j += 1;
        }
        r[i as usize] = accum;
        i += 1;
    }
    poly1305_min_reduce(&raw mut r as *mut uint32_t);
    i = 0 as ::core::ffi::c_int;
    while i < 17 as ::core::ffi::c_int {
        *x.offset(i as isize) = r[i as usize];
        i += 1;
    }
}
unsafe extern "C" fn poly1305_block(mut ctx: *mut cf_poly1305, mut c: *const uint32_t) {
    poly1305_add(&raw mut (*ctx).h as *mut uint32_t, c);
    poly1305_mul(
        &raw mut (*ctx).h as *mut uint32_t,
        &raw mut (*ctx).r as *mut uint32_t as *const uint32_t,
    );
}
unsafe extern "C" fn poly1305_whole_block(
    mut vctx: *mut ::core::ffi::c_void,
    mut buf: *const uint8_t,
) {
    let mut ctx: *mut cf_poly1305 = vctx as *mut cf_poly1305;
    let mut c: [uint32_t; 17] = [0; 17];
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 16 as ::core::ffi::c_int {
        c[i as usize] = *buf.offset(i as isize) as uint32_t;
        i += 1;
    }
    c[16 as ::core::ffi::c_int as usize] = 1 as uint32_t;
    poly1305_block(ctx, &raw mut c as *mut uint32_t as *const uint32_t);
}
unsafe extern "C" fn poly1305_last_block(mut ctx: *mut cf_poly1305) {
    let mut c: [uint32_t; 17] = [
        0 as ::core::ffi::c_int as uint32_t,
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
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < (*ctx).npartial {
        c[i as usize] = (*ctx).partial[i as usize] as uint32_t;
        i = i.wrapping_add(1);
    }
    c[(*ctx).npartial as usize] = 1 as uint32_t;
    poly1305_block(ctx, &raw mut c as *mut uint32_t as *const uint32_t);
}
#[no_mangle]
pub unsafe extern "C" fn cf_poly1305_update(
    mut ctx: *mut cf_poly1305,
    mut buf: *const uint8_t,
    mut nbytes: size_t,
) {
    cf_blockwise_accumulate(
        &raw mut (*ctx).partial as *mut uint8_t,
        &raw mut (*ctx).npartial,
        ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
        buf as *const ::core::ffi::c_void,
        nbytes,
        Some(
            poly1305_whole_block
                as unsafe extern "C" fn(*mut ::core::ffi::c_void, *const uint8_t) -> (),
        ),
        ctx as *mut ::core::ffi::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_poly1305_finish(mut ctx: *mut cf_poly1305, mut out: *mut uint8_t) {
    if (*ctx).npartial != 0 {
        poly1305_last_block(ctx);
    }
    let mut s: [uint32_t; 17] = [0; 17];
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < 16 as size_t {
        s[i as usize] = (*ctx).s[i as usize] as uint32_t;
        i = i.wrapping_add(1);
    }
    s[16 as ::core::ffi::c_int as usize] = 0 as uint32_t;
    poly1305_full_reduce(&raw mut (*ctx).h as *mut uint32_t);
    poly1305_add(
        &raw mut (*ctx).h as *mut uint32_t,
        &raw mut s as *mut uint32_t as *const uint32_t,
    );
    i = 0 as size_t;
    while i < 16 as size_t {
        *out.offset(i as isize) = (*ctx).h[i as usize] as uint8_t;
        i = i.wrapping_add(1);
    }
    mem_clean(
        ctx as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<cf_poly1305>() as size_t,
    );
}

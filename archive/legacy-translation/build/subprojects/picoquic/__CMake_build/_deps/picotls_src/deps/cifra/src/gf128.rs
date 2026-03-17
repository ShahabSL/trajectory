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
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type cf_gf128 = [uint32_t; 4];
#[inline]
unsafe extern "C" fn read32_be(mut buf: *const uint8_t) -> uint32_t {
    return (*buf.offset(0 as ::core::ffi::c_int as isize) as uint32_t) << 24 as ::core::ffi::c_int
        | (*buf.offset(1 as ::core::ffi::c_int as isize) as uint32_t) << 16 as ::core::ffi::c_int
        | (*buf.offset(2 as ::core::ffi::c_int as isize) as uint32_t) << 8 as ::core::ffi::c_int
        | *buf.offset(3 as ::core::ffi::c_int as isize) as uint32_t;
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
#[inline]
unsafe extern "C" fn mask_u32(mut x: uint32_t, mut y: uint32_t) -> uint32_t {
    let mut diff: uint32_t = x ^ y;
    let mut diff_is_zero: uint32_t = !diff & diff.wrapping_sub(1 as uint32_t);
    return -((diff_is_zero >> 31 as ::core::ffi::c_int) as int32_t) as uint32_t;
}
#[inline]
unsafe extern "C" fn mask_u8(mut x: uint32_t, mut y: uint32_t) -> uint8_t {
    let mut diff: uint32_t = x ^ y;
    let mut diff_is_zero: uint8_t = (!diff & diff.wrapping_sub(1 as uint32_t)) as uint8_t;
    return -(diff_is_zero as ::core::ffi::c_int >> 7 as ::core::ffi::c_int) as uint8_t;
}
#[inline]
unsafe extern "C" fn select_u8(
    mut i: uint32_t,
    mut tab: *const uint8_t,
    mut n: uint32_t,
) -> uint8_t {
    let mut r: uint8_t = 0 as uint8_t;
    let mut ii: uint32_t = 0;
    ii = 0 as uint32_t;
    while ii < n {
        let mut mask: uint8_t = mask_u8(i, ii);
        r = (r as ::core::ffi::c_int & !(mask as ::core::ffi::c_int)
            | *tab.offset(ii as isize) as ::core::ffi::c_int & mask as ::core::ffi::c_int)
            as uint8_t;
        ii = ii.wrapping_add(1);
    }
    return r;
}
#[inline]
unsafe extern "C" fn select_xor128(
    mut out: *mut uint32_t,
    mut if0: *const uint32_t,
    mut if1: *const uint32_t,
    mut bit: uint8_t,
) {
    let mut mask1: uint32_t = mask_u32(bit as uint32_t, 1 as uint32_t);
    let mut mask0: uint32_t = !mask1;
    *out.offset(0 as ::core::ffi::c_int as isize) ^= *if0.offset(0 as ::core::ffi::c_int as isize)
        & mask0
        | *if1.offset(0 as ::core::ffi::c_int as isize) & mask1;
    *out.offset(1 as ::core::ffi::c_int as isize) ^= *if0.offset(1 as ::core::ffi::c_int as isize)
        & mask0
        | *if1.offset(1 as ::core::ffi::c_int as isize) & mask1;
    *out.offset(2 as ::core::ffi::c_int as isize) ^= *if0.offset(2 as ::core::ffi::c_int as isize)
        & mask0
        | *if1.offset(2 as ::core::ffi::c_int as isize) & mask1;
    *out.offset(3 as ::core::ffi::c_int as isize) ^= *if0.offset(3 as ::core::ffi::c_int as isize)
        & mask0
        | *if1.offset(3 as ::core::ffi::c_int as isize) & mask1;
}
#[no_mangle]
pub unsafe extern "C" fn cf_gf128_tobytes_be(mut in_0: *const uint32_t, mut out: *mut uint8_t) {
    write32_be(
        *in_0.offset(0 as ::core::ffi::c_int as isize),
        out.offset(0 as ::core::ffi::c_int as isize),
    );
    write32_be(
        *in_0.offset(1 as ::core::ffi::c_int as isize),
        out.offset(4 as ::core::ffi::c_int as isize),
    );
    write32_be(
        *in_0.offset(2 as ::core::ffi::c_int as isize),
        out.offset(8 as ::core::ffi::c_int as isize),
    );
    write32_be(
        *in_0.offset(3 as ::core::ffi::c_int as isize),
        out.offset(12 as ::core::ffi::c_int as isize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_gf128_frombytes_be(mut in_0: *const uint8_t, mut out: *mut uint32_t) {
    *out.offset(0 as ::core::ffi::c_int as isize) =
        read32_be(in_0.offset(0 as ::core::ffi::c_int as isize));
    *out.offset(1 as ::core::ffi::c_int as isize) =
        read32_be(in_0.offset(4 as ::core::ffi::c_int as isize));
    *out.offset(2 as ::core::ffi::c_int as isize) =
        read32_be(in_0.offset(8 as ::core::ffi::c_int as isize));
    *out.offset(3 as ::core::ffi::c_int as isize) =
        read32_be(in_0.offset(12 as ::core::ffi::c_int as isize));
}
#[no_mangle]
pub unsafe extern "C" fn cf_gf128_double(mut in_0: *const uint32_t, mut out: *mut uint32_t) {
    let mut table: [uint8_t; 2] = [
        0 as ::core::ffi::c_int as uint8_t,
        0x87 as ::core::ffi::c_int as uint8_t,
    ];
    let mut borrow: uint32_t = 0 as uint32_t;
    let mut inword: uint32_t = 0;
    inword = *in_0.offset(3 as ::core::ffi::c_int as isize);
    *out.offset(3 as ::core::ffi::c_int as isize) = inword << 1 as ::core::ffi::c_int | borrow;
    borrow = inword >> 31 as ::core::ffi::c_int;
    inword = *in_0.offset(2 as ::core::ffi::c_int as isize);
    *out.offset(2 as ::core::ffi::c_int as isize) = inword << 1 as ::core::ffi::c_int | borrow;
    borrow = inword >> 31 as ::core::ffi::c_int;
    inword = *in_0.offset(1 as ::core::ffi::c_int as isize);
    *out.offset(1 as ::core::ffi::c_int as isize) = inword << 1 as ::core::ffi::c_int | borrow;
    borrow = inword >> 31 as ::core::ffi::c_int;
    inword = *in_0.offset(0 as ::core::ffi::c_int as isize);
    *out.offset(0 as ::core::ffi::c_int as isize) = inword << 1 as ::core::ffi::c_int | borrow;
    borrow = inword >> 31 as ::core::ffi::c_int;
    *out.offset(3 as ::core::ffi::c_int as isize) ^=
        select_u8(borrow, &raw mut table as *mut uint8_t, 2 as uint32_t) as uint32_t;
}
#[no_mangle]
pub unsafe extern "C" fn cf_gf128_double_le(mut in_0: *const uint32_t, mut out: *mut uint32_t) {
    let mut table: [uint8_t; 2] = [
        0 as ::core::ffi::c_int as uint8_t,
        0xe1 as ::core::ffi::c_int as uint8_t,
    ];
    let mut borrow: uint32_t = 0 as uint32_t;
    let mut inword: uint32_t = 0;
    inword = *in_0.offset(0 as ::core::ffi::c_int as isize);
    *out.offset(0 as ::core::ffi::c_int as isize) =
        inword >> 1 as ::core::ffi::c_int | borrow << 31 as ::core::ffi::c_int;
    borrow = inword & 1 as uint32_t;
    inword = *in_0.offset(1 as ::core::ffi::c_int as isize);
    *out.offset(1 as ::core::ffi::c_int as isize) =
        inword >> 1 as ::core::ffi::c_int | borrow << 31 as ::core::ffi::c_int;
    borrow = inword & 1 as uint32_t;
    inword = *in_0.offset(2 as ::core::ffi::c_int as isize);
    *out.offset(2 as ::core::ffi::c_int as isize) =
        inword >> 1 as ::core::ffi::c_int | borrow << 31 as ::core::ffi::c_int;
    borrow = inword & 1 as uint32_t;
    inword = *in_0.offset(3 as ::core::ffi::c_int as isize);
    *out.offset(3 as ::core::ffi::c_int as isize) =
        inword >> 1 as ::core::ffi::c_int | borrow << 31 as ::core::ffi::c_int;
    borrow = inword & 1 as uint32_t;
    *out.offset(0 as ::core::ffi::c_int as isize) ^=
        (select_u8(borrow, &raw mut table as *mut uint8_t, 2 as uint32_t) as uint32_t)
            << 24 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cf_gf128_add(
    mut x: *const uint32_t,
    mut y: *const uint32_t,
    mut out: *mut uint32_t,
) {
    *out.offset(0 as ::core::ffi::c_int as isize) =
        *x.offset(0 as ::core::ffi::c_int as isize) ^ *y.offset(0 as ::core::ffi::c_int as isize);
    *out.offset(1 as ::core::ffi::c_int as isize) =
        *x.offset(1 as ::core::ffi::c_int as isize) ^ *y.offset(1 as ::core::ffi::c_int as isize);
    *out.offset(2 as ::core::ffi::c_int as isize) =
        *x.offset(2 as ::core::ffi::c_int as isize) ^ *y.offset(2 as ::core::ffi::c_int as isize);
    *out.offset(3 as ::core::ffi::c_int as isize) =
        *x.offset(3 as ::core::ffi::c_int as isize) ^ *y.offset(3 as ::core::ffi::c_int as isize);
}
#[no_mangle]
pub unsafe extern "C" fn cf_gf128_mul(
    mut x: *const uint32_t,
    mut y: *const uint32_t,
    mut out: *mut uint32_t,
) {
    let mut zero: cf_gf128 = [0 as ::core::ffi::c_int as uint32_t, 0, 0, 0];
    let mut Z: cf_gf128 = [0; 4];
    let mut V: cf_gf128 = [0; 4];
    memset(
        &raw mut Z as *mut uint32_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<cf_gf128>() as size_t,
    );
    memcpy(
        &raw mut V as *mut uint32_t as *mut ::core::ffi::c_void,
        y as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cf_gf128>() as size_t,
    );
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < 128 as ::core::ffi::c_int {
        let mut word: uint32_t = *x.offset((i >> 5 as ::core::ffi::c_int) as isize);
        let mut bit: uint8_t = (word >> 31 as ::core::ffi::c_int - (i & 31 as ::core::ffi::c_int)
            & 1 as uint32_t) as uint8_t;
        select_xor128(
            &raw mut Z as *mut uint32_t,
            &raw mut zero as *mut uint32_t as *const uint32_t,
            &raw mut V as *mut uint32_t as *const uint32_t,
            bit,
        );
        cf_gf128_double_le(
            &raw mut V as *mut uint32_t as *const uint32_t,
            &raw mut V as *mut uint32_t,
        );
        i += 1;
    }
    memcpy(
        out as *mut ::core::ffi::c_void,
        &raw mut Z as *mut uint32_t as *const ::core::ffi::c_void,
        ::core::mem::size_of::<cf_gf128>() as size_t,
    );
}

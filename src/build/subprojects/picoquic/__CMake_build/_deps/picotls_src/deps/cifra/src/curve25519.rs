pub type __uint8_t = u8;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
pub type gf = [int64_t; 16];
static mut _9: [uint8_t; 32] = [
    9 as ::core::ffi::c_int as uint8_t,
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
static mut _121665: gf = [
    0xdb41 as ::core::ffi::c_int as int64_t,
    1 as ::core::ffi::c_int as int64_t,
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
unsafe extern "C" fn set25519(mut r: *mut int64_t, mut a: *const int64_t) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < 16 as size_t {
        *r.offset(i as isize) = *a.offset(i as isize);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn car25519(mut o: *mut int64_t) {
    let mut c: int64_t = 0;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < 16 as size_t {
        let ref mut c2rust_fresh0 = *o.offset(i as isize);
        *c2rust_fresh0 = (*c2rust_fresh0 as ::core::ffi::c_longlong
            + ((1 as ::core::ffi::c_longlong) << 16 as ::core::ffi::c_int))
            as int64_t;
        c = *o.offset(i as isize) >> 16 as ::core::ffi::c_int;
        *o.offset(
            i.wrapping_add(1 as size_t)
                .wrapping_mul((i < 15 as size_t) as ::core::ffi::c_int as size_t)
                as isize,
        ) += c - 1 as int64_t
            + 37 as int64_t
                * (c - 1 as int64_t)
                * (i == 15 as size_t) as ::core::ffi::c_int as int64_t;
        *o.offset(i as isize) -= ((c as uint64_t) << 16 as ::core::ffi::c_int) as int64_t;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn sel25519(mut p: *mut int64_t, mut q: *mut int64_t, mut b: int64_t) {
    let mut tmp: int64_t = 0;
    let mut mask: int64_t = !(b - 1 as int64_t);
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < 16 as size_t {
        tmp = mask & (*p.offset(i as isize) ^ *q.offset(i as isize));
        *p.offset(i as isize) ^= tmp;
        *q.offset(i as isize) ^= tmp;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn pack25519(mut out: *mut uint8_t, mut n: *const int64_t) {
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut b: ::core::ffi::c_int = 0;
    let mut m: gf = [0; 16];
    let mut t: gf = [0; 16];
    set25519(&raw mut t as *mut int64_t, n);
    car25519(&raw mut t as *mut int64_t);
    car25519(&raw mut t as *mut int64_t);
    car25519(&raw mut t as *mut int64_t);
    j = 0 as size_t;
    while j < 2 as size_t {
        m[0 as ::core::ffi::c_int as usize] =
            t[0 as ::core::ffi::c_int as usize] - 0xffed as int64_t;
        i = 1 as size_t;
        while i < 15 as size_t {
            m[i as usize] = t[i as usize]
                - 0xffff as int64_t
                - (m[i.wrapping_sub(1 as size_t) as usize] >> 16 as ::core::ffi::c_int
                    & 1 as int64_t);
            m[i.wrapping_sub(1 as size_t) as usize] &= 0xffff as int64_t;
            i = i.wrapping_add(1);
        }
        m[15 as ::core::ffi::c_int as usize] = t[15 as ::core::ffi::c_int as usize]
            - 0x7fff as int64_t
            - (m[14 as ::core::ffi::c_int as usize] >> 16 as ::core::ffi::c_int & 1 as int64_t);
        b = (m[15 as ::core::ffi::c_int as usize] >> 16 as ::core::ffi::c_int & 1 as int64_t)
            as ::core::ffi::c_int;
        m[14 as ::core::ffi::c_int as usize] &= 0xffff as int64_t;
        sel25519(
            &raw mut t as *mut int64_t,
            &raw mut m as *mut int64_t,
            (1 as ::core::ffi::c_int - b) as int64_t,
        );
        j = j.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < 16 as size_t {
        *out.offset((2 as size_t).wrapping_mul(i) as isize) =
            (t[i as usize] & 0xff as int64_t) as uint8_t;
        *out.offset((2 as size_t).wrapping_mul(i).wrapping_add(1 as size_t) as isize) =
            (t[i as usize] >> 8 as ::core::ffi::c_int) as uint8_t;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn unpack25519(mut o: *mut int64_t, mut n: *const uint8_t) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < 16 as size_t {
        *o.offset(i as isize) = *n.offset((2 as size_t).wrapping_mul(i) as isize) as int64_t
            + ((*n.offset((2 as size_t).wrapping_mul(i).wrapping_add(1 as size_t) as isize)
                as int64_t)
                << 8 as ::core::ffi::c_int);
        i = i.wrapping_add(1);
    }
    *o.offset(15 as ::core::ffi::c_int as isize) &= 0x7fff as int64_t;
}
unsafe extern "C" fn add(mut o: *mut int64_t, mut a: *const int64_t, mut b: *const int64_t) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < 16 as size_t {
        *o.offset(i as isize) = *a.offset(i as isize) + *b.offset(i as isize);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn sub(mut o: *mut int64_t, mut a: *const int64_t, mut b: *const int64_t) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < 16 as size_t {
        *o.offset(i as isize) = *a.offset(i as isize) - *b.offset(i as isize);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn mul(mut o: *mut int64_t, mut a: *const int64_t, mut b: *const int64_t) {
    let mut t: [int64_t; 31] = [0; 31];
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    i = 0 as size_t;
    while i < 31 as size_t {
        t[i as usize] = 0 as int64_t;
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < 16 as size_t {
        j = 0 as size_t;
        while j < 16 as size_t {
            t[i.wrapping_add(j) as usize] += *a.offset(i as isize) * *b.offset(j as isize);
            j = j.wrapping_add(1);
        }
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < 15 as size_t {
        t[i as usize] += 38 as int64_t * t[i.wrapping_add(16 as size_t) as usize];
        i = i.wrapping_add(1);
    }
    i = 0 as size_t;
    while i < 16 as size_t {
        *o.offset(i as isize) = t[i as usize];
        i = i.wrapping_add(1);
    }
    car25519(o);
    car25519(o);
}
unsafe extern "C" fn sqr(mut o: *mut int64_t, mut a: *const int64_t) {
    mul(o, a, a);
}
unsafe extern "C" fn inv25519(mut o: *mut int64_t, mut i: *const int64_t) {
    let mut c: gf = [0; 16];
    let mut a: ::core::ffi::c_int = 0;
    a = 0 as ::core::ffi::c_int;
    while a < 16 as ::core::ffi::c_int {
        c[a as usize] = *i.offset(a as isize);
        a += 1;
    }
    a = 253 as ::core::ffi::c_int;
    while a >= 0 as ::core::ffi::c_int {
        sqr(
            &raw mut c as *mut int64_t,
            &raw mut c as *mut int64_t as *const int64_t,
        );
        if a != 2 as ::core::ffi::c_int && a != 4 as ::core::ffi::c_int {
            mul(
                &raw mut c as *mut int64_t,
                &raw mut c as *mut int64_t as *const int64_t,
                i,
            );
        }
        a -= 1;
    }
    a = 0 as ::core::ffi::c_int;
    while a < 16 as ::core::ffi::c_int {
        *o.offset(a as isize) = c[a as usize];
        a += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn cf_curve25519_mul(
    mut q: *mut uint8_t,
    mut n: *const uint8_t,
    mut p: *const uint8_t,
) {
    let mut z: [uint8_t; 32] = [0; 32];
    let mut x: gf = [0; 16];
    let mut a: gf = [0; 16];
    let mut b: gf = [0; 16];
    let mut c: gf = [0; 16];
    let mut d: gf = [0; 16];
    let mut e: gf = [0; 16];
    let mut f: gf = [0; 16];
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < 31 as size_t {
        z[i as usize] = *n.offset(i as isize);
        i = i.wrapping_add(1);
    }
    z[31 as ::core::ffi::c_int as usize] = (*n.offset(31 as ::core::ffi::c_int as isize)
        as ::core::ffi::c_int
        & 127 as ::core::ffi::c_int
        | 64 as ::core::ffi::c_int) as uint8_t;
    z[0 as ::core::ffi::c_int as usize] = (z[0 as ::core::ffi::c_int as usize]
        as ::core::ffi::c_int
        & 248 as ::core::ffi::c_int) as uint8_t;
    unpack25519(&raw mut x as *mut int64_t, p);
    i = 0 as size_t;
    while i < 16 as size_t {
        b[i as usize] = x[i as usize];
        c[i as usize] = 0 as int64_t;
        a[i as usize] = c[i as usize];
        d[i as usize] = a[i as usize];
        i = i.wrapping_add(1);
    }
    d[0 as ::core::ffi::c_int as usize] = 1 as int64_t;
    a[0 as ::core::ffi::c_int as usize] = d[0 as ::core::ffi::c_int as usize];
    let mut i_0: ::core::ffi::c_int = 0;
    i_0 = 254 as ::core::ffi::c_int;
    while i_0 >= 0 as ::core::ffi::c_int {
        let mut r: int64_t = (z[(i_0 >> 3 as ::core::ffi::c_int) as usize] as ::core::ffi::c_int
            >> (i_0 & 7 as ::core::ffi::c_int)
            & 1 as ::core::ffi::c_int) as int64_t;
        sel25519(&raw mut a as *mut int64_t, &raw mut b as *mut int64_t, r);
        sel25519(&raw mut c as *mut int64_t, &raw mut d as *mut int64_t, r);
        add(
            &raw mut e as *mut int64_t,
            &raw mut a as *mut int64_t as *const int64_t,
            &raw mut c as *mut int64_t as *const int64_t,
        );
        sub(
            &raw mut a as *mut int64_t,
            &raw mut a as *mut int64_t as *const int64_t,
            &raw mut c as *mut int64_t as *const int64_t,
        );
        add(
            &raw mut c as *mut int64_t,
            &raw mut b as *mut int64_t as *const int64_t,
            &raw mut d as *mut int64_t as *const int64_t,
        );
        sub(
            &raw mut b as *mut int64_t,
            &raw mut b as *mut int64_t as *const int64_t,
            &raw mut d as *mut int64_t as *const int64_t,
        );
        sqr(
            &raw mut d as *mut int64_t,
            &raw mut e as *mut int64_t as *const int64_t,
        );
        sqr(
            &raw mut f as *mut int64_t,
            &raw mut a as *mut int64_t as *const int64_t,
        );
        mul(
            &raw mut a as *mut int64_t,
            &raw mut c as *mut int64_t as *const int64_t,
            &raw mut a as *mut int64_t as *const int64_t,
        );
        mul(
            &raw mut c as *mut int64_t,
            &raw mut b as *mut int64_t as *const int64_t,
            &raw mut e as *mut int64_t as *const int64_t,
        );
        add(
            &raw mut e as *mut int64_t,
            &raw mut a as *mut int64_t as *const int64_t,
            &raw mut c as *mut int64_t as *const int64_t,
        );
        sub(
            &raw mut a as *mut int64_t,
            &raw mut a as *mut int64_t as *const int64_t,
            &raw mut c as *mut int64_t as *const int64_t,
        );
        sqr(
            &raw mut b as *mut int64_t,
            &raw mut a as *mut int64_t as *const int64_t,
        );
        sub(
            &raw mut c as *mut int64_t,
            &raw mut d as *mut int64_t as *const int64_t,
            &raw mut f as *mut int64_t as *const int64_t,
        );
        mul(
            &raw mut a as *mut int64_t,
            &raw mut c as *mut int64_t as *const int64_t,
            &raw const _121665 as *const int64_t,
        );
        add(
            &raw mut a as *mut int64_t,
            &raw mut a as *mut int64_t as *const int64_t,
            &raw mut d as *mut int64_t as *const int64_t,
        );
        mul(
            &raw mut c as *mut int64_t,
            &raw mut c as *mut int64_t as *const int64_t,
            &raw mut a as *mut int64_t as *const int64_t,
        );
        mul(
            &raw mut a as *mut int64_t,
            &raw mut d as *mut int64_t as *const int64_t,
            &raw mut f as *mut int64_t as *const int64_t,
        );
        mul(
            &raw mut d as *mut int64_t,
            &raw mut b as *mut int64_t as *const int64_t,
            &raw mut x as *mut int64_t as *const int64_t,
        );
        sqr(
            &raw mut b as *mut int64_t,
            &raw mut e as *mut int64_t as *const int64_t,
        );
        sel25519(&raw mut a as *mut int64_t, &raw mut b as *mut int64_t, r);
        sel25519(&raw mut c as *mut int64_t, &raw mut d as *mut int64_t, r);
        i_0 -= 1;
    }
    inv25519(
        &raw mut c as *mut int64_t,
        &raw mut c as *mut int64_t as *const int64_t,
    );
    mul(
        &raw mut a as *mut int64_t,
        &raw mut a as *mut int64_t as *const int64_t,
        &raw mut c as *mut int64_t as *const int64_t,
    );
    pack25519(
        q as *mut uint8_t,
        &raw mut a as *mut int64_t as *const int64_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_curve25519_mul_base(mut q: *mut uint8_t, mut n: *const uint8_t) {
    cf_curve25519_mul(q, n, &raw const _9 as *const uint8_t);
}

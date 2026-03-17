extern "C" {
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
}
pub type __int8_t = i8;
pub type __uint8_t = u8;
pub type __int16_t = i16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type int8_t = __int8_t;
pub type int16_t = __int16_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uECC_Curve_t {
    pub num_words: wordcount_t,
    pub num_bytes: wordcount_t,
    pub num_n_bits: bitcount_t,
    pub p: [uECC_word_t; 4],
    pub n: [uECC_word_t; 4],
    pub G: [uECC_word_t; 8],
    pub b: [uECC_word_t; 4],
    pub double_jacobian: Option<
        unsafe extern "C" fn(
            *mut uECC_word_t,
            *mut uECC_word_t,
            *mut uECC_word_t,
            uECC_Curve,
        ) -> (),
    >,
    pub mod_sqrt: Option<unsafe extern "C" fn(*mut uECC_word_t, uECC_Curve) -> ()>,
    pub x_side:
        Option<unsafe extern "C" fn(*mut uECC_word_t, *const uECC_word_t, uECC_Curve) -> ()>,
    pub mmod_fast: Option<unsafe extern "C" fn(*mut uECC_word_t, *mut uECC_word_t) -> ()>,
}
pub type uECC_word_t = uint64_t;
pub type uECC_Curve = *const uECC_Curve_t;
pub type bitcount_t = int16_t;
pub type wordcount_t = int8_t;
pub type cmpresult_t = int8_t;
pub type uECC_dword_t = u128;
pub type uECC_RNG_Function =
    Option<unsafe extern "C" fn(*mut uint8_t, ::core::ffi::c_uint) -> ::core::ffi::c_int>;
pub type ssize_t = isize;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct uECC_HashContext {
    pub init_hash: Option<unsafe extern "C" fn(*const uECC_HashContext) -> ()>,
    pub update_hash: Option<
        unsafe extern "C" fn(*const uECC_HashContext, *const uint8_t, ::core::ffi::c_uint) -> (),
    >,
    pub finish_hash: Option<unsafe extern "C" fn(*const uECC_HashContext, *mut uint8_t) -> ()>,
    pub block_size: ::core::ffi::c_uint,
    pub result_size: ::core::ffi::c_uint,
    pub tmp: *mut uint8_t,
}
pub const uECC_WORD_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const HIGH_BIT_SET: ::core::ffi::c_ulonglong = 0x8000000000000000 as ::core::ffi::c_ulonglong;
pub const uECC_WORD_BITS: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const uECC_WORD_BITS_SHIFT: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
pub const uECC_WORD_BITS_MASK: ::core::ffi::c_int = 0x3f as ::core::ffi::c_int;
pub const uECC_RNG_MAX_TRIES: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
unsafe extern "C" fn default_RNG(
    mut dest: *mut uint8_t,
    mut size: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut fd: ::core::ffi::c_int = open(
        b"/dev/urandom\0".as_ptr() as *const ::core::ffi::c_char,
        O_RDONLY | O_CLOEXEC,
    );
    if fd == -(1 as ::core::ffi::c_int) {
        fd = open(
            b"/dev/random\0".as_ptr() as *const ::core::ffi::c_char,
            O_RDONLY | O_CLOEXEC,
        );
        if fd == -(1 as ::core::ffi::c_int) {
            return 0 as ::core::ffi::c_int;
        }
    }
    let mut ptr: *mut ::core::ffi::c_char = dest as *mut ::core::ffi::c_char;
    let mut left: size_t = size as size_t;
    while left > 0 as size_t {
        let mut bytes_read: ssize_t = read(fd, ptr as *mut ::core::ffi::c_void, left);
        if bytes_read <= 0 as ssize_t {
            close(fd);
            return 0 as ::core::ffi::c_int;
        }
        left = left.wrapping_sub(bytes_read as size_t);
        ptr = ptr.offset(bytes_read as isize);
    }
    close(fd);
    return 1 as ::core::ffi::c_int;
}
static mut g_rng_function: uECC_RNG_Function = Some(
    default_RNG as unsafe extern "C" fn(*mut uint8_t, ::core::ffi::c_uint) -> ::core::ffi::c_int,
);
#[no_mangle]
pub unsafe extern "C" fn uECC_set_rng(mut rng_function: uECC_RNG_Function) {
    g_rng_function = rng_function;
}
#[no_mangle]
pub unsafe extern "C" fn uECC_get_rng() -> uECC_RNG_Function {
    return g_rng_function;
}
#[no_mangle]
pub unsafe extern "C" fn uECC_curve_private_key_size(mut curve: uECC_Curve) -> ::core::ffi::c_int {
    return ((*curve).num_n_bits as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
        / 8 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uECC_curve_public_key_size(mut curve: uECC_Curve) -> ::core::ffi::c_int {
    return 2 as ::core::ffi::c_int * (*curve).num_bytes as ::core::ffi::c_int;
}
unsafe extern "C" fn uECC_vli_clear(mut vli: *mut uECC_word_t, mut num_words: wordcount_t) {
    let mut i: wordcount_t = 0;
    i = 0 as wordcount_t;
    while (i as ::core::ffi::c_int) < num_words as ::core::ffi::c_int {
        *vli.offset(i as isize) = 0 as uECC_word_t;
        i += 1;
    }
}
unsafe extern "C" fn uECC_vli_isZero(
    mut vli: *const uECC_word_t,
    mut num_words: wordcount_t,
) -> uECC_word_t {
    let mut bits: uECC_word_t = 0 as uECC_word_t;
    let mut i: wordcount_t = 0;
    i = 0 as wordcount_t;
    while (i as ::core::ffi::c_int) < num_words as ::core::ffi::c_int {
        bits |= *vli.offset(i as isize);
        i += 1;
    }
    return (bits == 0 as uECC_word_t) as ::core::ffi::c_int as uECC_word_t;
}
unsafe extern "C" fn uECC_vli_testBit(
    mut vli: *const uECC_word_t,
    mut bit: bitcount_t,
) -> uECC_word_t {
    return *vli.offset((bit as ::core::ffi::c_int >> uECC_WORD_BITS_SHIFT) as isize)
        & (1 as ::core::ffi::c_int as uECC_word_t)
            << (bit as ::core::ffi::c_int & uECC_WORD_BITS_MASK);
}
unsafe extern "C" fn vli_numDigits(
    mut vli: *const uECC_word_t,
    max_words: wordcount_t,
) -> wordcount_t {
    let mut i: wordcount_t = 0;
    i = (max_words as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as wordcount_t;
    while i as ::core::ffi::c_int >= 0 as ::core::ffi::c_int
        && *vli.offset(i as isize) == 0 as uECC_word_t
    {
        i -= 1;
    }
    return (i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as wordcount_t;
}
unsafe extern "C" fn uECC_vli_numBits(
    mut vli: *const uECC_word_t,
    max_words: wordcount_t,
) -> bitcount_t {
    let mut i: uECC_word_t = 0;
    let mut digit: uECC_word_t = 0;
    let mut num_digits: wordcount_t = vli_numDigits(vli, max_words);
    if num_digits as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        return 0 as bitcount_t;
    }
    digit = *vli.offset((num_digits as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
    i = 0 as uECC_word_t;
    while digit != 0 {
        digit >>= 1 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
    return ((((num_digits as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as bitcount_t
        as ::core::ffi::c_int)
        << uECC_WORD_BITS_SHIFT) as uECC_word_t)
        .wrapping_add(i) as bitcount_t;
}
unsafe extern "C" fn uECC_vli_set(
    mut dest: *mut uECC_word_t,
    mut src: *const uECC_word_t,
    mut num_words: wordcount_t,
) {
    let mut i: wordcount_t = 0;
    i = 0 as wordcount_t;
    while (i as ::core::ffi::c_int) < num_words as ::core::ffi::c_int {
        *dest.offset(i as isize) = *src.offset(i as isize);
        i += 1;
    }
}
unsafe extern "C" fn uECC_vli_cmp_unsafe(
    mut left: *const uECC_word_t,
    mut right: *const uECC_word_t,
    mut num_words: wordcount_t,
) -> cmpresult_t {
    let mut i: wordcount_t = 0;
    i = (num_words as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as wordcount_t;
    while i as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        if *left.offset(i as isize) > *right.offset(i as isize) {
            return 1 as cmpresult_t;
        } else if *left.offset(i as isize) < *right.offset(i as isize) {
            return -(1 as ::core::ffi::c_int) as cmpresult_t;
        }
        i -= 1;
    }
    return 0 as cmpresult_t;
}
unsafe extern "C" fn uECC_vli_equal(
    mut left: *const uECC_word_t,
    mut right: *const uECC_word_t,
    mut num_words: wordcount_t,
) -> uECC_word_t {
    let mut diff: uECC_word_t = 0 as uECC_word_t;
    let mut i: wordcount_t = 0;
    i = (num_words as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as wordcount_t;
    while i as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        diff |= *left.offset(i as isize) ^ *right.offset(i as isize);
        i -= 1;
    }
    return (diff == 0 as uECC_word_t) as ::core::ffi::c_int as uECC_word_t;
}
unsafe extern "C" fn uECC_vli_cmp(
    mut left: *const uECC_word_t,
    mut right: *const uECC_word_t,
    mut num_words: wordcount_t,
) -> cmpresult_t {
    let mut tmp: [uECC_word_t; 4] = [0; 4];
    let mut neg: uECC_word_t =
        (uECC_vli_sub(&raw mut tmp as *mut uECC_word_t, left, right, num_words) != 0)
            as ::core::ffi::c_int as uECC_word_t;
    let mut equal: uECC_word_t = uECC_vli_isZero(&raw mut tmp as *mut uECC_word_t, num_words);
    return ((equal == 0) as ::core::ffi::c_int as uECC_word_t)
        .wrapping_sub((2 as uECC_word_t).wrapping_mul(neg)) as cmpresult_t;
}
unsafe extern "C" fn uECC_vli_rshift1(mut vli: *mut uECC_word_t, mut num_words: wordcount_t) {
    let mut end: *mut uECC_word_t = vli;
    let mut carry: uECC_word_t = 0 as uECC_word_t;
    vli = vli.offset(num_words as ::core::ffi::c_int as isize);
    loop {
        let c2rust_fresh0 = vli;
        vli = vli.offset(-1);
        if !(c2rust_fresh0 > end) {
            break;
        }
        let mut temp: uECC_word_t = *vli;
        *vli = temp >> 1 as ::core::ffi::c_int | carry;
        carry = temp << uECC_WORD_BITS - 1 as ::core::ffi::c_int;
    }
}
unsafe extern "C" fn uECC_vli_add(
    mut result: *mut uECC_word_t,
    mut left: *const uECC_word_t,
    mut right: *const uECC_word_t,
    mut num_words: wordcount_t,
) -> uECC_word_t {
    let mut carry: uECC_word_t = 0 as uECC_word_t;
    let mut i: wordcount_t = 0;
    i = 0 as wordcount_t;
    while (i as ::core::ffi::c_int) < num_words as ::core::ffi::c_int {
        let mut sum: uECC_word_t = (*left.offset(i as isize))
            .wrapping_add(*right.offset(i as isize))
            .wrapping_add(carry);
        if sum != *left.offset(i as isize) {
            carry = (sum < *left.offset(i as isize)) as ::core::ffi::c_int as uECC_word_t;
        }
        *result.offset(i as isize) = sum;
        i += 1;
    }
    return carry;
}
unsafe extern "C" fn uECC_vli_sub(
    mut result: *mut uECC_word_t,
    mut left: *const uECC_word_t,
    mut right: *const uECC_word_t,
    mut num_words: wordcount_t,
) -> uECC_word_t {
    let mut borrow: uECC_word_t = 0 as uECC_word_t;
    let mut i: wordcount_t = 0;
    i = 0 as wordcount_t;
    while (i as ::core::ffi::c_int) < num_words as ::core::ffi::c_int {
        let mut diff: uECC_word_t = (*left.offset(i as isize))
            .wrapping_sub(*right.offset(i as isize))
            .wrapping_sub(borrow);
        if diff != *left.offset(i as isize) {
            borrow = (diff > *left.offset(i as isize)) as ::core::ffi::c_int as uECC_word_t;
        }
        *result.offset(i as isize) = diff;
        i += 1;
    }
    return borrow;
}
unsafe extern "C" fn muladd(
    mut a: uECC_word_t,
    mut b: uECC_word_t,
    mut r0: *mut uECC_word_t,
    mut r1: *mut uECC_word_t,
    mut r2: *mut uECC_word_t,
) {
    let mut p: uECC_dword_t = (a as uECC_dword_t).wrapping_mul(b as uECC_dword_t);
    let mut r01: uECC_dword_t = (*r1 as uECC_dword_t) << uECC_WORD_BITS | *r0 as uECC_dword_t;
    r01 = r01.wrapping_add(p);
    *r2 = (*r2).wrapping_add((r01 < p) as ::core::ffi::c_int as uECC_word_t);
    *r1 = (r01 >> uECC_WORD_BITS) as uECC_word_t;
    *r0 = r01 as uECC_word_t;
}
unsafe extern "C" fn uECC_vli_mult(
    mut result: *mut uECC_word_t,
    mut left: *const uECC_word_t,
    mut right: *const uECC_word_t,
    mut num_words: wordcount_t,
) {
    let mut r0: uECC_word_t = 0 as uECC_word_t;
    let mut r1: uECC_word_t = 0 as uECC_word_t;
    let mut r2: uECC_word_t = 0 as uECC_word_t;
    let mut i: wordcount_t = 0;
    let mut k: wordcount_t = 0;
    k = 0 as wordcount_t;
    while (k as ::core::ffi::c_int) < num_words as ::core::ffi::c_int {
        i = 0 as wordcount_t;
        while i as ::core::ffi::c_int <= k as ::core::ffi::c_int {
            muladd(
                *left.offset(i as isize),
                *right.offset((k as ::core::ffi::c_int - i as ::core::ffi::c_int) as isize),
                &raw mut r0,
                &raw mut r1,
                &raw mut r2,
            );
            i += 1;
        }
        *result.offset(k as isize) = r0;
        r0 = r1;
        r1 = r2;
        r2 = 0 as uECC_word_t;
        k += 1;
    }
    k = num_words;
    while (k as ::core::ffi::c_int)
        < num_words as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int
    {
        i = (k as ::core::ffi::c_int + 1 as ::core::ffi::c_int - num_words as ::core::ffi::c_int)
            as wordcount_t;
        while (i as ::core::ffi::c_int) < num_words as ::core::ffi::c_int {
            muladd(
                *left.offset(i as isize),
                *right.offset((k as ::core::ffi::c_int - i as ::core::ffi::c_int) as isize),
                &raw mut r0,
                &raw mut r1,
                &raw mut r2,
            );
            i += 1;
        }
        *result.offset(k as isize) = r0;
        r0 = r1;
        r1 = r2;
        r2 = 0 as uECC_word_t;
        k += 1;
    }
    *result.offset(
        (num_words as ::core::ffi::c_int * 2 as ::core::ffi::c_int - 1 as ::core::ffi::c_int)
            as isize,
    ) = r0;
}
unsafe extern "C" fn uECC_vli_modAdd(
    mut result: *mut uECC_word_t,
    mut left: *const uECC_word_t,
    mut right: *const uECC_word_t,
    mut mod_0: *const uECC_word_t,
    mut num_words: wordcount_t,
) {
    let mut carry: uECC_word_t = uECC_vli_add(result, left, right, num_words);
    if carry != 0
        || uECC_vli_cmp_unsafe(mod_0, result, num_words) as ::core::ffi::c_int
            != 1 as ::core::ffi::c_int
    {
        uECC_vli_sub(result, result, mod_0, num_words);
    }
}
unsafe extern "C" fn uECC_vli_modSub(
    mut result: *mut uECC_word_t,
    mut left: *const uECC_word_t,
    mut right: *const uECC_word_t,
    mut mod_0: *const uECC_word_t,
    mut num_words: wordcount_t,
) {
    let mut l_borrow: uECC_word_t = uECC_vli_sub(result, left, right, num_words);
    if l_borrow != 0 {
        uECC_vli_add(result, result, mod_0, num_words);
    }
}
unsafe extern "C" fn uECC_vli_mmod(
    mut result: *mut uECC_word_t,
    mut product: *mut uECC_word_t,
    mut mod_0: *const uECC_word_t,
    mut num_words: wordcount_t,
) {
    let mut mod_multiple: [uECC_word_t; 8] = [0; 8];
    let mut tmp: [uECC_word_t; 8] = [0; 8];
    let mut v: [*mut uECC_word_t; 2] = [&raw mut tmp as *mut uECC_word_t, product];
    let mut index: uECC_word_t = 0;
    let mut shift: bitcount_t =
        (num_words as ::core::ffi::c_int * 2 as ::core::ffi::c_int * uECC_WORD_BITS
            - uECC_vli_numBits(mod_0, num_words) as ::core::ffi::c_int) as bitcount_t;
    let mut word_shift: wordcount_t = (shift as ::core::ffi::c_int / uECC_WORD_BITS) as wordcount_t;
    let mut bit_shift: wordcount_t = (shift as ::core::ffi::c_int % uECC_WORD_BITS) as wordcount_t;
    let mut carry: uECC_word_t = 0 as uECC_word_t;
    uECC_vli_clear(&raw mut mod_multiple as *mut uECC_word_t, word_shift);
    if bit_shift as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        index = 0 as uECC_word_t;
        while index < num_words as uECC_word_t {
            mod_multiple[(word_shift as uECC_word_t).wrapping_add(index) as usize] =
                *mod_0.offset(index as isize) << bit_shift as ::core::ffi::c_int | carry;
            carry =
                *mod_0.offset(index as isize) >> uECC_WORD_BITS - bit_shift as ::core::ffi::c_int;
            index = index.wrapping_add(1);
        }
    } else {
        uECC_vli_set(
            (&raw mut mod_multiple as *mut uECC_word_t)
                .offset(word_shift as ::core::ffi::c_int as isize),
            mod_0,
            num_words,
        );
    }
    index = 1 as uECC_word_t;
    while shift as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        let mut borrow: uECC_word_t = 0 as uECC_word_t;
        let mut i: wordcount_t = 0;
        i = 0 as wordcount_t;
        while (i as ::core::ffi::c_int) < num_words as ::core::ffi::c_int * 2 as ::core::ffi::c_int
        {
            let mut diff: uECC_word_t = (*v[index as usize].offset(i as isize))
                .wrapping_sub(mod_multiple[i as usize])
                .wrapping_sub(borrow);
            if diff != *v[index as usize].offset(i as isize) {
                borrow = (diff > *v[index as usize].offset(i as isize)) as ::core::ffi::c_int
                    as uECC_word_t;
            }
            *v[(1 as uECC_word_t).wrapping_sub(index) as usize].offset(i as isize) = diff;
            i += 1;
        }
        index = (index ^ borrow == 0) as ::core::ffi::c_int as uECC_word_t;
        uECC_vli_rshift1(&raw mut mod_multiple as *mut uECC_word_t, num_words);
        mod_multiple[(num_words as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] |=
            mod_multiple[num_words as usize] << uECC_WORD_BITS - 1 as ::core::ffi::c_int;
        uECC_vli_rshift1(
            (&raw mut mod_multiple as *mut uECC_word_t)
                .offset(num_words as ::core::ffi::c_int as isize),
            num_words,
        );
        shift -= 1;
    }
    uECC_vli_set(result, v[index as usize], num_words);
}
unsafe extern "C" fn uECC_vli_modMult(
    mut result: *mut uECC_word_t,
    mut left: *const uECC_word_t,
    mut right: *const uECC_word_t,
    mut mod_0: *const uECC_word_t,
    mut num_words: wordcount_t,
) {
    let mut product: [uECC_word_t; 8] = [0; 8];
    uECC_vli_mult(&raw mut product as *mut uECC_word_t, left, right, num_words);
    uECC_vli_mmod(
        result,
        &raw mut product as *mut uECC_word_t,
        mod_0,
        num_words,
    );
}
unsafe extern "C" fn uECC_vli_modMult_fast(
    mut result: *mut uECC_word_t,
    mut left: *const uECC_word_t,
    mut right: *const uECC_word_t,
    mut curve: uECC_Curve,
) {
    let mut product: [uECC_word_t; 8] = [0; 8];
    uECC_vli_mult(
        &raw mut product as *mut uECC_word_t,
        left,
        right,
        (*curve).num_words,
    );
    (*curve).mmod_fast.expect("non-null function pointer")(
        result,
        &raw mut product as *mut uECC_word_t,
    );
}
unsafe extern "C" fn uECC_vli_modSquare_fast(
    mut result: *mut uECC_word_t,
    mut left: *const uECC_word_t,
    mut curve: uECC_Curve,
) {
    uECC_vli_modMult_fast(result, left, left, curve);
}
unsafe extern "C" fn vli_modInv_update(
    mut uv: *mut uECC_word_t,
    mut mod_0: *const uECC_word_t,
    mut num_words: wordcount_t,
) {
    let mut carry: uECC_word_t = 0 as uECC_word_t;
    if *uv.offset(0 as ::core::ffi::c_int as isize) & 1 as uECC_word_t != 0 {
        carry = uECC_vli_add(uv, uv, mod_0, num_words);
    }
    uECC_vli_rshift1(uv, num_words);
    if carry != 0 {
        let ref mut c2rust_fresh1 =
            *uv.offset((num_words as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize);
        *c2rust_fresh1 = (*c2rust_fresh1 as ::core::ffi::c_ulonglong | HIGH_BIT_SET) as uECC_word_t;
    }
}
unsafe extern "C" fn uECC_vli_modInv(
    mut result: *mut uECC_word_t,
    mut input: *const uECC_word_t,
    mut mod_0: *const uECC_word_t,
    mut num_words: wordcount_t,
) {
    let mut a: [uECC_word_t; 4] = [0; 4];
    let mut b: [uECC_word_t; 4] = [0; 4];
    let mut u: [uECC_word_t; 4] = [0; 4];
    let mut v: [uECC_word_t; 4] = [0; 4];
    let mut cmpResult: cmpresult_t = 0;
    if uECC_vli_isZero(input, num_words) != 0 {
        uECC_vli_clear(result, num_words);
        return;
    }
    uECC_vli_set(&raw mut a as *mut uECC_word_t, input, num_words);
    uECC_vli_set(&raw mut b as *mut uECC_word_t, mod_0, num_words);
    uECC_vli_clear(&raw mut u as *mut uECC_word_t, num_words);
    u[0 as ::core::ffi::c_int as usize] = 1 as uECC_word_t;
    uECC_vli_clear(&raw mut v as *mut uECC_word_t, num_words);
    loop {
        cmpResult = uECC_vli_cmp_unsafe(
            &raw mut a as *mut uECC_word_t,
            &raw mut b as *mut uECC_word_t,
            num_words,
        );
        if !(cmpResult as ::core::ffi::c_int != 0 as ::core::ffi::c_int) {
            break;
        }
        if a[0 as ::core::ffi::c_int as usize] & 1 as uECC_word_t == 0 {
            uECC_vli_rshift1(&raw mut a as *mut uECC_word_t, num_words);
            vli_modInv_update(&raw mut u as *mut uECC_word_t, mod_0, num_words);
        } else if b[0 as ::core::ffi::c_int as usize] & 1 as uECC_word_t == 0 {
            uECC_vli_rshift1(&raw mut b as *mut uECC_word_t, num_words);
            vli_modInv_update(&raw mut v as *mut uECC_word_t, mod_0, num_words);
        } else if cmpResult as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
            uECC_vli_sub(
                &raw mut a as *mut uECC_word_t,
                &raw mut a as *mut uECC_word_t,
                &raw mut b as *mut uECC_word_t,
                num_words,
            );
            uECC_vli_rshift1(&raw mut a as *mut uECC_word_t, num_words);
            if (uECC_vli_cmp_unsafe(
                &raw mut u as *mut uECC_word_t,
                &raw mut v as *mut uECC_word_t,
                num_words,
            ) as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
            {
                uECC_vli_add(
                    &raw mut u as *mut uECC_word_t,
                    &raw mut u as *mut uECC_word_t,
                    mod_0,
                    num_words,
                );
            }
            uECC_vli_sub(
                &raw mut u as *mut uECC_word_t,
                &raw mut u as *mut uECC_word_t,
                &raw mut v as *mut uECC_word_t,
                num_words,
            );
            vli_modInv_update(&raw mut u as *mut uECC_word_t, mod_0, num_words);
        } else {
            uECC_vli_sub(
                &raw mut b as *mut uECC_word_t,
                &raw mut b as *mut uECC_word_t,
                &raw mut a as *mut uECC_word_t,
                num_words,
            );
            uECC_vli_rshift1(&raw mut b as *mut uECC_word_t, num_words);
            if (uECC_vli_cmp_unsafe(
                &raw mut v as *mut uECC_word_t,
                &raw mut u as *mut uECC_word_t,
                num_words,
            ) as ::core::ffi::c_int)
                < 0 as ::core::ffi::c_int
            {
                uECC_vli_add(
                    &raw mut v as *mut uECC_word_t,
                    &raw mut v as *mut uECC_word_t,
                    mod_0,
                    num_words,
                );
            }
            uECC_vli_sub(
                &raw mut v as *mut uECC_word_t,
                &raw mut v as *mut uECC_word_t,
                &raw mut u as *mut uECC_word_t,
                num_words,
            );
            vli_modInv_update(&raw mut v as *mut uECC_word_t, mod_0, num_words);
        }
    }
    uECC_vli_set(result, &raw mut u as *mut uECC_word_t, num_words);
}
pub const num_bytes_secp160r1: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const num_bytes_secp192r1: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const num_bytes_secp224r1: ::core::ffi::c_int = 28 as ::core::ffi::c_int;
pub const num_bytes_secp256r1: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const num_bytes_secp256k1: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const num_words_secp160r1: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const num_words_secp192r1: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const num_words_secp224r1: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const num_words_secp256r1: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const num_words_secp256k1: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
unsafe extern "C" fn double_jacobian_default(
    mut X1: *mut uECC_word_t,
    mut Y1: *mut uECC_word_t,
    mut Z1: *mut uECC_word_t,
    mut curve: uECC_Curve,
) {
    let mut t4: [uECC_word_t; 4] = [0; 4];
    let mut t5: [uECC_word_t; 4] = [0; 4];
    let mut num_words: wordcount_t = (*curve).num_words;
    if uECC_vli_isZero(Z1, num_words) != 0 {
        return;
    }
    uECC_vli_modSquare_fast(&raw mut t4 as *mut uECC_word_t, Y1, curve);
    uECC_vli_modMult_fast(
        &raw mut t5 as *mut uECC_word_t,
        X1,
        &raw mut t4 as *mut uECC_word_t,
        curve,
    );
    uECC_vli_modSquare_fast(
        &raw mut t4 as *mut uECC_word_t,
        &raw mut t4 as *mut uECC_word_t,
        curve,
    );
    uECC_vli_modMult_fast(Y1, Y1, Z1, curve);
    uECC_vli_modSquare_fast(Z1, Z1, curve);
    uECC_vli_modAdd(
        X1,
        X1,
        Z1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modAdd(
        Z1,
        Z1,
        Z1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modSub(
        Z1,
        X1,
        Z1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modMult_fast(X1, X1, Z1, curve);
    uECC_vli_modAdd(
        Z1,
        X1,
        X1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modAdd(
        X1,
        X1,
        Z1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    if uECC_vli_testBit(X1, 0 as bitcount_t) != 0 {
        let mut l_carry: uECC_word_t = uECC_vli_add(
            X1,
            X1,
            &raw const (*curve).p as *const uECC_word_t,
            num_words,
        );
        uECC_vli_rshift1(X1, num_words);
        *X1.offset((num_words as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) |=
            l_carry << uECC_WORD_BITS - 1 as ::core::ffi::c_int;
    } else {
        uECC_vli_rshift1(X1, num_words);
    }
    uECC_vli_modSquare_fast(Z1, X1, curve);
    uECC_vli_modSub(
        Z1,
        Z1,
        &raw mut t5 as *mut uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modSub(
        Z1,
        Z1,
        &raw mut t5 as *mut uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modSub(
        &raw mut t5 as *mut uECC_word_t,
        &raw mut t5 as *mut uECC_word_t,
        Z1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modMult_fast(X1, X1, &raw mut t5 as *mut uECC_word_t, curve);
    uECC_vli_modSub(
        &raw mut t4 as *mut uECC_word_t,
        X1,
        &raw mut t4 as *mut uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_set(X1, Z1, num_words);
    uECC_vli_set(Z1, Y1, num_words);
    uECC_vli_set(Y1, &raw mut t4 as *mut uECC_word_t, num_words);
}
unsafe extern "C" fn x_side_default(
    mut result: *mut uECC_word_t,
    mut x: *const uECC_word_t,
    mut curve: uECC_Curve,
) {
    let mut _3: [uECC_word_t; 4] = [3 as ::core::ffi::c_int as uECC_word_t, 0, 0, 0];
    let mut num_words: wordcount_t = (*curve).num_words;
    uECC_vli_modSquare_fast(result, x, curve);
    uECC_vli_modSub(
        result,
        result,
        &raw mut _3 as *mut uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modMult_fast(result, result, x, curve);
    uECC_vli_modAdd(
        result,
        result,
        &raw const (*curve).b as *const uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
}
unsafe extern "C" fn mod_sqrt_default(mut a: *mut uECC_word_t, mut curve: uECC_Curve) {
    let mut i: bitcount_t = 0;
    let mut p1: [uECC_word_t; 4] = [1 as ::core::ffi::c_int as uECC_word_t, 0, 0, 0];
    let mut l_result: [uECC_word_t; 4] = [1 as ::core::ffi::c_int as uECC_word_t, 0, 0, 0];
    let mut num_words: wordcount_t = (*curve).num_words;
    uECC_vli_add(
        &raw mut p1 as *mut uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        &raw mut p1 as *mut uECC_word_t,
        num_words,
    );
    i = (uECC_vli_numBits(&raw mut p1 as *mut uECC_word_t, num_words) as ::core::ffi::c_int
        - 1 as ::core::ffi::c_int) as bitcount_t;
    while i as ::core::ffi::c_int > 1 as ::core::ffi::c_int {
        uECC_vli_modSquare_fast(
            &raw mut l_result as *mut uECC_word_t,
            &raw mut l_result as *mut uECC_word_t,
            curve,
        );
        if uECC_vli_testBit(&raw mut p1 as *mut uECC_word_t, i) != 0 {
            uECC_vli_modMult_fast(
                &raw mut l_result as *mut uECC_word_t,
                &raw mut l_result as *mut uECC_word_t,
                a,
                curve,
            );
        }
        i -= 1;
    }
    uECC_vli_set(a, &raw mut l_result as *mut uECC_word_t, num_words);
}
static mut curve_secp160r1: uECC_Curve_t = uECC_Curve_t {
    num_words: num_words_secp160r1 as wordcount_t,
    num_bytes: num_bytes_secp160r1 as wordcount_t,
    num_n_bits: 161 as bitcount_t,
    p: [
        0xffffffff7fffffff as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffffffffffff as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffff as ::core::ffi::c_ulonglong as uECC_word_t,
        0,
    ],
    n: [
        0xf927aed3ca752257 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x1f4c8 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x100000000 as ::core::ffi::c_ulonglong as uECC_word_t,
        0,
    ],
    G: [
        0x68c38bb913cbfc82 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x8ef5732846646989 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x4a96b568 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x42351377ac5fb32 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x3168947d59dcc912 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x23a62855 as ::core::ffi::c_ulonglong as uECC_word_t,
        0,
        0,
    ],
    b: [
        0x81d4d4adc565fa45 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x54bd7a8b65acf89f as ::core::ffi::c_ulonglong as uECC_word_t,
        0x1c97befc as ::core::ffi::c_ulonglong as uECC_word_t,
        0,
    ],
    double_jacobian: Some(
        double_jacobian_default
            as unsafe extern "C" fn(
                *mut uECC_word_t,
                *mut uECC_word_t,
                *mut uECC_word_t,
                uECC_Curve,
            ) -> (),
    ),
    mod_sqrt: Some(mod_sqrt_default as unsafe extern "C" fn(*mut uECC_word_t, uECC_Curve) -> ()),
    x_side: Some(
        x_side_default
            as unsafe extern "C" fn(*mut uECC_word_t, *const uECC_word_t, uECC_Curve) -> (),
    ),
    mmod_fast: Some(
        vli_mmod_fast_secp160r1 as unsafe extern "C" fn(*mut uECC_word_t, *mut uECC_word_t) -> (),
    ),
};
#[no_mangle]
pub unsafe extern "C" fn uECC_secp160r1() -> uECC_Curve {
    return &raw const curve_secp160r1;
}
unsafe extern "C" fn vli_mmod_fast_secp160r1(
    mut result: *mut uECC_word_t,
    mut product: *mut uECC_word_t,
) {
    let mut tmp: [uECC_word_t; 6] = [0; 6];
    let mut copy: uECC_word_t = 0;
    uECC_vli_clear(
        &raw mut tmp as *mut uECC_word_t,
        num_words_secp160r1 as wordcount_t,
    );
    uECC_vli_clear(
        (&raw mut tmp as *mut uECC_word_t).offset(num_words_secp160r1 as isize),
        num_words_secp160r1 as wordcount_t,
    );
    omega_mult_secp160r1(
        &raw mut tmp as *mut uint64_t,
        product
            .offset(num_words_secp160r1 as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)),
    );
    *product.offset((num_words_secp160r1 - 1 as ::core::ffi::c_int) as isize) &=
        0xffffffff as uECC_word_t;
    copy = tmp[(num_words_secp160r1 - 1 as ::core::ffi::c_int) as usize];
    tmp[(num_words_secp160r1 - 1 as ::core::ffi::c_int) as usize] &= 0xffffffff as uECC_word_t;
    uECC_vli_add(
        result,
        product,
        &raw mut tmp as *mut uECC_word_t,
        num_words_secp160r1 as wordcount_t,
    );
    uECC_vli_clear(product, num_words_secp160r1 as wordcount_t);
    tmp[(num_words_secp160r1 - 1 as ::core::ffi::c_int) as usize] = copy;
    omega_mult_secp160r1(
        product as *mut uint64_t,
        (&raw mut tmp as *mut uECC_word_t)
            .offset(num_words_secp160r1 as isize)
            .offset(-(1 as ::core::ffi::c_int as isize)),
    );
    uECC_vli_add(result, result, product, num_words_secp160r1 as wordcount_t);
    while uECC_vli_cmp_unsafe(
        result,
        &raw const curve_secp160r1.p as *const uECC_word_t,
        num_words_secp160r1 as wordcount_t,
    ) as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        uECC_vli_sub(
            result,
            result,
            &raw const curve_secp160r1.p as *const uECC_word_t,
            num_words_secp160r1 as wordcount_t,
        );
    }
}
unsafe extern "C" fn omega_mult_secp160r1(mut result: *mut uint64_t, mut right: *const uint64_t) {
    let mut carry: uint32_t = 0;
    let mut i: ::core::ffi::c_uint = 0;
    carry = 0 as uint32_t;
    i = 0 as ::core::ffi::c_uint;
    while i < num_words_secp160r1 as ::core::ffi::c_uint {
        let mut tmp: uint64_t = *right.offset(i as isize) >> 32 as ::core::ffi::c_int
            | *right.offset(i.wrapping_add(1 as ::core::ffi::c_uint) as isize)
                << 32 as ::core::ffi::c_int;
        *result.offset(i as isize) = (tmp << 31 as ::core::ffi::c_int)
            .wrapping_add(tmp)
            .wrapping_add(carry as uint64_t);
        carry = (tmp >> 33 as ::core::ffi::c_int).wrapping_add(
            (*result.offset(i as isize) < tmp || carry != 0 && *result.offset(i as isize) == tmp)
                as ::core::ffi::c_int as uint64_t,
        ) as uint32_t;
        i = i.wrapping_add(1);
    }
    *result.offset(i as isize) = carry as uint64_t;
}
static mut curve_secp192r1: uECC_Curve_t = uECC_Curve_t {
    num_words: num_words_secp192r1 as wordcount_t,
    num_bytes: num_bytes_secp192r1 as wordcount_t,
    num_n_bits: 192 as bitcount_t,
    p: [
        0xffffffffffffffff as ::core::ffi::c_ulonglong as uECC_word_t,
        0xfffffffffffffffe as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffffffffffff as ::core::ffi::c_ulonglong as uECC_word_t,
        0,
    ],
    n: [
        0x146bc9b1b4d22831 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffff99def836 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffffffffffff as ::core::ffi::c_ulonglong as uECC_word_t,
        0,
    ],
    G: [
        0xf4ff0afd82ff1012 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x7cbf20eb43a18800 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x188da80eb03090f6 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x73f977a11e794811 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x631011ed6b24cdd5 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x7192b95ffc8da78 as ::core::ffi::c_ulonglong as uECC_word_t,
        0,
        0,
    ],
    b: [
        0xfeb8deecc146b9b1 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xfa7e9ab72243049 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x64210519e59c80e7 as ::core::ffi::c_ulonglong as uECC_word_t,
        0,
    ],
    double_jacobian: Some(
        double_jacobian_default
            as unsafe extern "C" fn(
                *mut uECC_word_t,
                *mut uECC_word_t,
                *mut uECC_word_t,
                uECC_Curve,
            ) -> (),
    ),
    mod_sqrt: Some(mod_sqrt_default as unsafe extern "C" fn(*mut uECC_word_t, uECC_Curve) -> ()),
    x_side: Some(
        x_side_default
            as unsafe extern "C" fn(*mut uECC_word_t, *const uECC_word_t, uECC_Curve) -> (),
    ),
    mmod_fast: Some(
        vli_mmod_fast_secp192r1 as unsafe extern "C" fn(*mut uint64_t, *mut uint64_t) -> (),
    ),
};
#[no_mangle]
pub unsafe extern "C" fn uECC_secp192r1() -> uECC_Curve {
    return &raw const curve_secp192r1;
}
unsafe extern "C" fn vli_mmod_fast_secp192r1(
    mut result: *mut uint64_t,
    mut product: *mut uint64_t,
) {
    let mut tmp: [uint64_t; 3] = [0; 3];
    let mut carry: ::core::ffi::c_int = 0;
    uECC_vli_set(
        result as *mut uECC_word_t,
        product,
        num_words_secp192r1 as wordcount_t,
    );
    uECC_vli_set(
        &raw mut tmp as *mut uECC_word_t,
        product.offset(3 as ::core::ffi::c_int as isize) as *mut uint64_t,
        num_words_secp192r1 as wordcount_t,
    );
    carry = uECC_vli_add(
        result as *mut uECC_word_t,
        result,
        &raw mut tmp as *mut uint64_t,
        num_words_secp192r1 as wordcount_t,
    ) as ::core::ffi::c_int;
    tmp[0 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    tmp[1 as ::core::ffi::c_int as usize] = *product.offset(3 as ::core::ffi::c_int as isize);
    tmp[2 as ::core::ffi::c_int as usize] = *product.offset(4 as ::core::ffi::c_int as isize);
    carry = (carry as uECC_word_t).wrapping_add(uECC_vli_add(
        result as *mut uECC_word_t,
        result,
        &raw mut tmp as *mut uint64_t,
        num_words_secp192r1 as wordcount_t,
    )) as ::core::ffi::c_int as ::core::ffi::c_int;
    tmp[1 as ::core::ffi::c_int as usize] = *product.offset(5 as ::core::ffi::c_int as isize);
    tmp[0 as ::core::ffi::c_int as usize] = tmp[1 as ::core::ffi::c_int as usize];
    tmp[2 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    carry = (carry as uECC_word_t).wrapping_add(uECC_vli_add(
        result as *mut uECC_word_t,
        result,
        &raw mut tmp as *mut uint64_t,
        num_words_secp192r1 as wordcount_t,
    )) as ::core::ffi::c_int as ::core::ffi::c_int;
    while carry != 0
        || uECC_vli_cmp_unsafe(
            &raw const curve_secp192r1.p as *const uECC_word_t,
            result,
            num_words_secp192r1 as wordcount_t,
        ) as ::core::ffi::c_int
            != 1 as ::core::ffi::c_int
    {
        carry = (carry as uECC_word_t).wrapping_sub(uECC_vli_sub(
            result as *mut uECC_word_t,
            result,
            &raw const curve_secp192r1.p as *const uECC_word_t,
            num_words_secp192r1 as wordcount_t,
        )) as ::core::ffi::c_int as ::core::ffi::c_int;
    }
}
static mut curve_secp224r1: uECC_Curve_t = uECC_Curve_t {
    num_words: num_words_secp224r1 as wordcount_t,
    num_bytes: num_bytes_secp224r1 as wordcount_t,
    num_n_bits: 224 as bitcount_t,
    p: [
        0x1 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffff00000000 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffffffffffff as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffff as ::core::ffi::c_ulonglong as uECC_word_t,
    ],
    n: [
        0x13dd29455c5c2a3d as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffff16a2e0b8f03e as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffffffffffff as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffff as ::core::ffi::c_ulonglong as uECC_word_t,
    ],
    G: [
        0x343280d6115c1d21 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x4a03c1d356c21122 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x6bb4bf7f321390b9 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xb70e0cbd as ::core::ffi::c_ulonglong as uECC_word_t,
        0x44d5819985007e34 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xcd4375a05a074764 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xb5f723fb4c22dfe6 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xbd376388 as ::core::ffi::c_ulonglong as uECC_word_t,
    ],
    b: [
        0x270b39432355ffb4 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x5044b0b7d7bfd8ba as ::core::ffi::c_ulonglong as uECC_word_t,
        0xc04b3abf5413256 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xb4050a85 as ::core::ffi::c_ulonglong as uECC_word_t,
    ],
    double_jacobian: Some(
        double_jacobian_default
            as unsafe extern "C" fn(
                *mut uECC_word_t,
                *mut uECC_word_t,
                *mut uECC_word_t,
                uECC_Curve,
            ) -> (),
    ),
    mod_sqrt: Some(mod_sqrt_secp224r1 as unsafe extern "C" fn(*mut uECC_word_t, uECC_Curve) -> ()),
    x_side: Some(
        x_side_default
            as unsafe extern "C" fn(*mut uECC_word_t, *const uECC_word_t, uECC_Curve) -> (),
    ),
    mmod_fast: Some(
        vli_mmod_fast_secp224r1 as unsafe extern "C" fn(*mut uint64_t, *mut uint64_t) -> (),
    ),
};
#[no_mangle]
pub unsafe extern "C" fn uECC_secp224r1() -> uECC_Curve {
    return &raw const curve_secp224r1;
}
unsafe extern "C" fn mod_sqrt_secp224r1_rs(
    mut d1: *mut uECC_word_t,
    mut e1: *mut uECC_word_t,
    mut f1: *mut uECC_word_t,
    mut d0: *const uECC_word_t,
    mut e0: *const uECC_word_t,
    mut f0: *const uECC_word_t,
) {
    let mut t: [uECC_word_t; 4] = [0; 4];
    uECC_vli_modSquare_fast(
        &raw mut t as *mut uECC_word_t,
        d0,
        &raw const curve_secp224r1,
    );
    uECC_vli_modMult_fast(e1, d0, e0, &raw const curve_secp224r1);
    uECC_vli_modAdd(
        d1,
        &raw mut t as *mut uECC_word_t,
        f0,
        &raw const curve_secp224r1.p as *const uECC_word_t,
        num_words_secp224r1 as wordcount_t,
    );
    uECC_vli_modAdd(
        e1,
        e1,
        e1,
        &raw const curve_secp224r1.p as *const uECC_word_t,
        num_words_secp224r1 as wordcount_t,
    );
    uECC_vli_modMult_fast(
        f1,
        &raw mut t as *mut uECC_word_t,
        f0,
        &raw const curve_secp224r1,
    );
    uECC_vli_modAdd(
        f1,
        f1,
        f1,
        &raw const curve_secp224r1.p as *const uECC_word_t,
        num_words_secp224r1 as wordcount_t,
    );
    uECC_vli_modAdd(
        f1,
        f1,
        f1,
        &raw const curve_secp224r1.p as *const uECC_word_t,
        num_words_secp224r1 as wordcount_t,
    );
}
unsafe extern "C" fn mod_sqrt_secp224r1_rss(
    mut d1: *mut uECC_word_t,
    mut e1: *mut uECC_word_t,
    mut f1: *mut uECC_word_t,
    mut d0: *const uECC_word_t,
    mut e0: *const uECC_word_t,
    mut f0: *const uECC_word_t,
    j: bitcount_t,
) {
    let mut i: bitcount_t = 0;
    uECC_vli_set(d1, d0, num_words_secp224r1 as wordcount_t);
    uECC_vli_set(e1, e0, num_words_secp224r1 as wordcount_t);
    uECC_vli_set(f1, f0, num_words_secp224r1 as wordcount_t);
    i = 1 as bitcount_t;
    while i as ::core::ffi::c_int <= j as ::core::ffi::c_int {
        mod_sqrt_secp224r1_rs(d1, e1, f1, d1, e1, f1);
        i += 1;
    }
}
unsafe extern "C" fn mod_sqrt_secp224r1_rm(
    mut d2: *mut uECC_word_t,
    mut e2: *mut uECC_word_t,
    mut f2: *mut uECC_word_t,
    mut c: *const uECC_word_t,
    mut d0: *const uECC_word_t,
    mut e0: *const uECC_word_t,
    mut d1: *const uECC_word_t,
    mut e1: *const uECC_word_t,
) {
    let mut t1: [uECC_word_t; 4] = [0; 4];
    let mut t2: [uECC_word_t; 4] = [0; 4];
    uECC_vli_modMult_fast(
        &raw mut t1 as *mut uECC_word_t,
        e0,
        e1,
        &raw const curve_secp224r1,
    );
    uECC_vli_modMult_fast(
        &raw mut t1 as *mut uECC_word_t,
        &raw mut t1 as *mut uECC_word_t,
        c,
        &raw const curve_secp224r1,
    );
    uECC_vli_modSub(
        &raw mut t1 as *mut uECC_word_t,
        &raw const curve_secp224r1.p as *const uECC_word_t,
        &raw mut t1 as *mut uECC_word_t,
        &raw const curve_secp224r1.p as *const uECC_word_t,
        num_words_secp224r1 as wordcount_t,
    );
    uECC_vli_modMult_fast(
        &raw mut t2 as *mut uECC_word_t,
        d0,
        d1,
        &raw const curve_secp224r1,
    );
    uECC_vli_modAdd(
        &raw mut t2 as *mut uECC_word_t,
        &raw mut t2 as *mut uECC_word_t,
        &raw mut t1 as *mut uECC_word_t,
        &raw const curve_secp224r1.p as *const uECC_word_t,
        num_words_secp224r1 as wordcount_t,
    );
    uECC_vli_modMult_fast(
        &raw mut t1 as *mut uECC_word_t,
        d0,
        e1,
        &raw const curve_secp224r1,
    );
    uECC_vli_modMult_fast(e2, d1, e0, &raw const curve_secp224r1);
    uECC_vli_modAdd(
        e2,
        e2,
        &raw mut t1 as *mut uECC_word_t,
        &raw const curve_secp224r1.p as *const uECC_word_t,
        num_words_secp224r1 as wordcount_t,
    );
    uECC_vli_modSquare_fast(f2, e2, &raw const curve_secp224r1);
    uECC_vli_modMult_fast(f2, f2, c, &raw const curve_secp224r1);
    uECC_vli_modSub(
        f2,
        &raw const curve_secp224r1.p as *const uECC_word_t,
        f2,
        &raw const curve_secp224r1.p as *const uECC_word_t,
        num_words_secp224r1 as wordcount_t,
    );
    uECC_vli_set(
        d2,
        &raw mut t2 as *mut uECC_word_t,
        num_words_secp224r1 as wordcount_t,
    );
}
unsafe extern "C" fn mod_sqrt_secp224r1_rp(
    mut d1: *mut uECC_word_t,
    mut e1: *mut uECC_word_t,
    mut f1: *mut uECC_word_t,
    mut c: *const uECC_word_t,
    mut r: *const uECC_word_t,
) {
    let mut i: wordcount_t = 0;
    let mut pow2i: wordcount_t = 1 as wordcount_t;
    let mut d0: [uECC_word_t; 4] = [0; 4];
    let mut e0: [uECC_word_t; 4] = [1 as ::core::ffi::c_int as uECC_word_t, 0, 0, 0];
    let mut f0: [uECC_word_t; 4] = [0; 4];
    uECC_vli_set(
        &raw mut d0 as *mut uECC_word_t,
        r,
        num_words_secp224r1 as wordcount_t,
    );
    uECC_vli_modSub(
        &raw mut f0 as *mut uECC_word_t,
        &raw const curve_secp224r1.p as *const uECC_word_t,
        c,
        &raw const curve_secp224r1.p as *const uECC_word_t,
        num_words_secp224r1 as wordcount_t,
    );
    i = 0 as wordcount_t;
    while i as ::core::ffi::c_int <= 6 as ::core::ffi::c_int {
        mod_sqrt_secp224r1_rss(
            d1,
            e1,
            f1,
            &raw mut d0 as *mut uECC_word_t,
            &raw mut e0 as *mut uECC_word_t,
            &raw mut f0 as *mut uECC_word_t,
            pow2i as bitcount_t,
        );
        mod_sqrt_secp224r1_rm(
            d1,
            e1,
            f1,
            c,
            d1,
            e1,
            &raw mut d0 as *mut uECC_word_t,
            &raw mut e0 as *mut uECC_word_t,
        );
        uECC_vli_set(
            &raw mut d0 as *mut uECC_word_t,
            d1,
            num_words_secp224r1 as wordcount_t,
        );
        uECC_vli_set(
            &raw mut e0 as *mut uECC_word_t,
            e1,
            num_words_secp224r1 as wordcount_t,
        );
        uECC_vli_set(
            &raw mut f0 as *mut uECC_word_t,
            f1,
            num_words_secp224r1 as wordcount_t,
        );
        pow2i = (pow2i as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as wordcount_t;
        i += 1;
    }
}
unsafe extern "C" fn mod_sqrt_secp224r1(mut a: *mut uECC_word_t, mut curve: uECC_Curve) {
    let mut i: bitcount_t = 0;
    let mut e1: [uECC_word_t; 4] = [0; 4];
    let mut f1: [uECC_word_t; 4] = [0; 4];
    let mut d0: [uECC_word_t; 4] = [0; 4];
    let mut e0: [uECC_word_t; 4] = [0; 4];
    let mut f0: [uECC_word_t; 4] = [0; 4];
    let mut d1: [uECC_word_t; 4] = [0; 4];
    mod_sqrt_secp224r1_rp(
        &raw mut d0 as *mut uECC_word_t,
        &raw mut e0 as *mut uECC_word_t,
        &raw mut f0 as *mut uECC_word_t,
        a,
        a,
    );
    mod_sqrt_secp224r1_rs(
        &raw mut d1 as *mut uECC_word_t,
        &raw mut e1 as *mut uECC_word_t,
        &raw mut f1 as *mut uECC_word_t,
        &raw mut d0 as *mut uECC_word_t,
        &raw mut e0 as *mut uECC_word_t,
        &raw mut f0 as *mut uECC_word_t,
    );
    i = 1 as bitcount_t;
    while i as ::core::ffi::c_int <= 95 as ::core::ffi::c_int {
        uECC_vli_set(
            &raw mut d0 as *mut uECC_word_t,
            &raw mut d1 as *mut uECC_word_t,
            num_words_secp224r1 as wordcount_t,
        );
        uECC_vli_set(
            &raw mut e0 as *mut uECC_word_t,
            &raw mut e1 as *mut uECC_word_t,
            num_words_secp224r1 as wordcount_t,
        );
        uECC_vli_set(
            &raw mut f0 as *mut uECC_word_t,
            &raw mut f1 as *mut uECC_word_t,
            num_words_secp224r1 as wordcount_t,
        );
        mod_sqrt_secp224r1_rs(
            &raw mut d1 as *mut uECC_word_t,
            &raw mut e1 as *mut uECC_word_t,
            &raw mut f1 as *mut uECC_word_t,
            &raw mut d0 as *mut uECC_word_t,
            &raw mut e0 as *mut uECC_word_t,
            &raw mut f0 as *mut uECC_word_t,
        );
        if uECC_vli_isZero(
            &raw mut d1 as *mut uECC_word_t,
            num_words_secp224r1 as wordcount_t,
        ) != 0
        {
            break;
        }
        i += 1;
    }
    uECC_vli_modInv(
        &raw mut f1 as *mut uECC_word_t,
        &raw mut e0 as *mut uECC_word_t,
        &raw const curve_secp224r1.p as *const uECC_word_t,
        num_words_secp224r1 as wordcount_t,
    );
    uECC_vli_modMult_fast(
        a,
        &raw mut d0 as *mut uECC_word_t,
        &raw mut f1 as *mut uECC_word_t,
        &raw const curve_secp224r1,
    );
}
unsafe extern "C" fn vli_mmod_fast_secp224r1(
    mut result: *mut uint64_t,
    mut product: *mut uint64_t,
) {
    let mut tmp: [uint64_t; 4] = [0; 4];
    let mut carry: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    uECC_vli_set(
        result as *mut uECC_word_t,
        product,
        num_words_secp224r1 as wordcount_t,
    );
    *result.offset((num_words_secp224r1 - 1 as ::core::ffi::c_int) as isize) &=
        0xffffffff as uint64_t;
    tmp[0 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    tmp[1 as ::core::ffi::c_int as usize] =
        (*product.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulonglong
            & 0xffffffff00000000 as ::core::ffi::c_ulonglong) as uint64_t;
    tmp[2 as ::core::ffi::c_int as usize] = *product.offset(4 as ::core::ffi::c_int as isize);
    tmp[3 as ::core::ffi::c_int as usize] =
        *product.offset(5 as ::core::ffi::c_int as isize) & 0xffffffff as uint64_t;
    uECC_vli_add(
        result as *mut uECC_word_t,
        result,
        &raw mut tmp as *mut uint64_t,
        num_words_secp224r1 as wordcount_t,
    );
    tmp[1 as ::core::ffi::c_int as usize] =
        (*product.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulonglong
            & 0xffffffff00000000 as ::core::ffi::c_ulonglong) as uint64_t;
    tmp[2 as ::core::ffi::c_int as usize] = *product.offset(6 as ::core::ffi::c_int as isize);
    tmp[3 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    uECC_vli_add(
        result as *mut uECC_word_t,
        result,
        &raw mut tmp as *mut uint64_t,
        num_words_secp224r1 as wordcount_t,
    );
    tmp[0 as ::core::ffi::c_int as usize] = *product.offset(3 as ::core::ffi::c_int as isize)
        >> 32 as ::core::ffi::c_int
        | *product.offset(4 as ::core::ffi::c_int as isize) << 32 as ::core::ffi::c_int;
    tmp[1 as ::core::ffi::c_int as usize] = *product.offset(4 as ::core::ffi::c_int as isize)
        >> 32 as ::core::ffi::c_int
        | *product.offset(5 as ::core::ffi::c_int as isize) << 32 as ::core::ffi::c_int;
    tmp[2 as ::core::ffi::c_int as usize] = *product.offset(5 as ::core::ffi::c_int as isize)
        >> 32 as ::core::ffi::c_int
        | *product.offset(6 as ::core::ffi::c_int as isize) << 32 as ::core::ffi::c_int;
    tmp[3 as ::core::ffi::c_int as usize] =
        *product.offset(6 as ::core::ffi::c_int as isize) >> 32 as ::core::ffi::c_int;
    carry = (carry as uECC_word_t).wrapping_sub(uECC_vli_sub(
        result as *mut uECC_word_t,
        result,
        &raw mut tmp as *mut uint64_t,
        num_words_secp224r1 as wordcount_t,
    )) as ::core::ffi::c_int as ::core::ffi::c_int;
    tmp[0 as ::core::ffi::c_int as usize] = *product.offset(5 as ::core::ffi::c_int as isize)
        >> 32 as ::core::ffi::c_int
        | *product.offset(6 as ::core::ffi::c_int as isize) << 32 as ::core::ffi::c_int;
    tmp[1 as ::core::ffi::c_int as usize] =
        *product.offset(6 as ::core::ffi::c_int as isize) >> 32 as ::core::ffi::c_int;
    tmp[3 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    tmp[2 as ::core::ffi::c_int as usize] = tmp[3 as ::core::ffi::c_int as usize];
    carry = (carry as uECC_word_t).wrapping_sub(uECC_vli_sub(
        result as *mut uECC_word_t,
        result,
        &raw mut tmp as *mut uint64_t,
        num_words_secp224r1 as wordcount_t,
    )) as ::core::ffi::c_int as ::core::ffi::c_int;
    if carry < 0 as ::core::ffi::c_int {
        loop {
            carry = (carry as uECC_word_t).wrapping_add(uECC_vli_add(
                result as *mut uECC_word_t,
                result,
                &raw const curve_secp224r1.p as *const uECC_word_t,
                num_words_secp224r1 as wordcount_t,
            )) as ::core::ffi::c_int as ::core::ffi::c_int;
            if !(carry < 0 as ::core::ffi::c_int) {
                break;
            }
        }
    } else {
        while uECC_vli_cmp_unsafe(
            &raw const curve_secp224r1.p as *const uECC_word_t,
            result,
            num_words_secp224r1 as wordcount_t,
        ) as ::core::ffi::c_int
            != 1 as ::core::ffi::c_int
        {
            uECC_vli_sub(
                result as *mut uECC_word_t,
                result,
                &raw const curve_secp224r1.p as *const uECC_word_t,
                num_words_secp224r1 as wordcount_t,
            );
        }
    };
}
static mut curve_secp256r1: uECC_Curve_t = uECC_Curve_t {
    num_words: num_words_secp256r1 as wordcount_t,
    num_bytes: num_bytes_secp256r1 as wordcount_t,
    num_n_bits: 256 as bitcount_t,
    p: [
        0xffffffffffffffff as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffff as ::core::ffi::c_ulonglong as uECC_word_t,
        0 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffff00000001 as ::core::ffi::c_ulonglong as uECC_word_t,
    ],
    n: [
        0xf3b9cac2fc632551 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xbce6faada7179e84 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffffffffffff as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffff00000000 as ::core::ffi::c_ulonglong as uECC_word_t,
    ],
    G: [
        0xf4a13945d898c296 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x77037d812deb33a0 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xf8bce6e563a440f2 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x6b17d1f2e12c4247 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xcbb6406837bf51f5 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x2bce33576b315ece as ::core::ffi::c_ulonglong as uECC_word_t,
        0x8ee7eb4a7c0f9e16 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x4fe342e2fe1a7f9b as ::core::ffi::c_ulonglong as uECC_word_t,
    ],
    b: [
        0x3bce3c3e27d2604b as ::core::ffi::c_ulonglong as uECC_word_t,
        0x651d06b0cc53b0f6 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xb3ebbd55769886bc as ::core::ffi::c_ulonglong as uECC_word_t,
        0x5ac635d8aa3a93e7 as ::core::ffi::c_ulonglong as uECC_word_t,
    ],
    double_jacobian: Some(
        double_jacobian_default
            as unsafe extern "C" fn(
                *mut uECC_word_t,
                *mut uECC_word_t,
                *mut uECC_word_t,
                uECC_Curve,
            ) -> (),
    ),
    mod_sqrt: Some(mod_sqrt_default as unsafe extern "C" fn(*mut uECC_word_t, uECC_Curve) -> ()),
    x_side: Some(
        x_side_default
            as unsafe extern "C" fn(*mut uECC_word_t, *const uECC_word_t, uECC_Curve) -> (),
    ),
    mmod_fast: Some(
        vli_mmod_fast_secp256r1 as unsafe extern "C" fn(*mut uint64_t, *mut uint64_t) -> (),
    ),
};
#[no_mangle]
pub unsafe extern "C" fn uECC_secp256r1() -> uECC_Curve {
    return &raw const curve_secp256r1;
}
unsafe extern "C" fn vli_mmod_fast_secp256r1(
    mut result: *mut uint64_t,
    mut product: *mut uint64_t,
) {
    let mut tmp: [uint64_t; 4] = [0; 4];
    let mut carry: ::core::ffi::c_int = 0;
    uECC_vli_set(
        result as *mut uECC_word_t,
        product,
        num_words_secp256r1 as wordcount_t,
    );
    tmp[0 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    tmp[1 as ::core::ffi::c_int as usize] =
        (*product.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulonglong
            & 0xffffffff00000000 as ::core::ffi::c_ulonglong) as uint64_t;
    tmp[2 as ::core::ffi::c_int as usize] = *product.offset(6 as ::core::ffi::c_int as isize);
    tmp[3 as ::core::ffi::c_int as usize] = *product.offset(7 as ::core::ffi::c_int as isize);
    carry = uECC_vli_add(
        &raw mut tmp as *mut uECC_word_t,
        &raw mut tmp as *mut uint64_t,
        &raw mut tmp as *mut uint64_t,
        num_words_secp256r1 as wordcount_t,
    ) as ::core::ffi::c_int;
    carry = (carry as uECC_word_t).wrapping_add(uECC_vli_add(
        result as *mut uECC_word_t,
        result,
        &raw mut tmp as *mut uint64_t,
        num_words_secp256r1 as wordcount_t,
    )) as ::core::ffi::c_int as ::core::ffi::c_int;
    tmp[1 as ::core::ffi::c_int as usize] =
        *product.offset(6 as ::core::ffi::c_int as isize) << 32 as ::core::ffi::c_int;
    tmp[2 as ::core::ffi::c_int as usize] = *product.offset(6 as ::core::ffi::c_int as isize)
        >> 32 as ::core::ffi::c_int
        | *product.offset(7 as ::core::ffi::c_int as isize) << 32 as ::core::ffi::c_int;
    tmp[3 as ::core::ffi::c_int as usize] =
        *product.offset(7 as ::core::ffi::c_int as isize) >> 32 as ::core::ffi::c_int;
    carry = (carry as uECC_word_t).wrapping_add(uECC_vli_add(
        &raw mut tmp as *mut uECC_word_t,
        &raw mut tmp as *mut uint64_t,
        &raw mut tmp as *mut uint64_t,
        num_words_secp256r1 as wordcount_t,
    )) as ::core::ffi::c_int as ::core::ffi::c_int;
    carry = (carry as uECC_word_t).wrapping_add(uECC_vli_add(
        result as *mut uECC_word_t,
        result,
        &raw mut tmp as *mut uint64_t,
        num_words_secp256r1 as wordcount_t,
    )) as ::core::ffi::c_int as ::core::ffi::c_int;
    tmp[0 as ::core::ffi::c_int as usize] = *product.offset(4 as ::core::ffi::c_int as isize);
    tmp[1 as ::core::ffi::c_int as usize] =
        *product.offset(5 as ::core::ffi::c_int as isize) & 0xffffffff as uint64_t;
    tmp[2 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    tmp[3 as ::core::ffi::c_int as usize] = *product.offset(7 as ::core::ffi::c_int as isize);
    carry = (carry as uECC_word_t).wrapping_add(uECC_vli_add(
        result as *mut uECC_word_t,
        result,
        &raw mut tmp as *mut uint64_t,
        num_words_secp256r1 as wordcount_t,
    )) as ::core::ffi::c_int as ::core::ffi::c_int;
    tmp[0 as ::core::ffi::c_int as usize] = *product.offset(4 as ::core::ffi::c_int as isize)
        >> 32 as ::core::ffi::c_int
        | *product.offset(5 as ::core::ffi::c_int as isize) << 32 as ::core::ffi::c_int;
    tmp[1 as ::core::ffi::c_int as usize] =
        ((*product.offset(5 as ::core::ffi::c_int as isize) >> 32 as ::core::ffi::c_int)
            as ::core::ffi::c_ulonglong
            | *product.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulonglong
                & 0xffffffff00000000 as ::core::ffi::c_ulonglong) as uint64_t;
    tmp[2 as ::core::ffi::c_int as usize] = *product.offset(7 as ::core::ffi::c_int as isize);
    tmp[3 as ::core::ffi::c_int as usize] = *product.offset(6 as ::core::ffi::c_int as isize)
        >> 32 as ::core::ffi::c_int
        | *product.offset(4 as ::core::ffi::c_int as isize) << 32 as ::core::ffi::c_int;
    carry = (carry as uECC_word_t).wrapping_add(uECC_vli_add(
        result as *mut uECC_word_t,
        result,
        &raw mut tmp as *mut uint64_t,
        num_words_secp256r1 as wordcount_t,
    )) as ::core::ffi::c_int as ::core::ffi::c_int;
    tmp[0 as ::core::ffi::c_int as usize] = *product.offset(5 as ::core::ffi::c_int as isize)
        >> 32 as ::core::ffi::c_int
        | *product.offset(6 as ::core::ffi::c_int as isize) << 32 as ::core::ffi::c_int;
    tmp[1 as ::core::ffi::c_int as usize] =
        *product.offset(6 as ::core::ffi::c_int as isize) >> 32 as ::core::ffi::c_int;
    tmp[2 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    tmp[3 as ::core::ffi::c_int as usize] = *product.offset(4 as ::core::ffi::c_int as isize)
        & 0xffffffff as uint64_t
        | *product.offset(5 as ::core::ffi::c_int as isize) << 32 as ::core::ffi::c_int;
    carry = (carry as uECC_word_t).wrapping_sub(uECC_vli_sub(
        result as *mut uECC_word_t,
        result,
        &raw mut tmp as *mut uint64_t,
        num_words_secp256r1 as wordcount_t,
    )) as ::core::ffi::c_int as ::core::ffi::c_int;
    tmp[0 as ::core::ffi::c_int as usize] = *product.offset(6 as ::core::ffi::c_int as isize);
    tmp[1 as ::core::ffi::c_int as usize] = *product.offset(7 as ::core::ffi::c_int as isize);
    tmp[2 as ::core::ffi::c_int as usize] = 0 as uint64_t;
    tmp[3 as ::core::ffi::c_int as usize] =
        ((*product.offset(4 as ::core::ffi::c_int as isize) >> 32 as ::core::ffi::c_int)
            as ::core::ffi::c_ulonglong
            | *product.offset(5 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulonglong
                & 0xffffffff00000000 as ::core::ffi::c_ulonglong) as uint64_t;
    carry = (carry as uECC_word_t).wrapping_sub(uECC_vli_sub(
        result as *mut uECC_word_t,
        result,
        &raw mut tmp as *mut uint64_t,
        num_words_secp256r1 as wordcount_t,
    )) as ::core::ffi::c_int as ::core::ffi::c_int;
    tmp[0 as ::core::ffi::c_int as usize] = *product.offset(6 as ::core::ffi::c_int as isize)
        >> 32 as ::core::ffi::c_int
        | *product.offset(7 as ::core::ffi::c_int as isize) << 32 as ::core::ffi::c_int;
    tmp[1 as ::core::ffi::c_int as usize] = *product.offset(7 as ::core::ffi::c_int as isize)
        >> 32 as ::core::ffi::c_int
        | *product.offset(4 as ::core::ffi::c_int as isize) << 32 as ::core::ffi::c_int;
    tmp[2 as ::core::ffi::c_int as usize] = *product.offset(4 as ::core::ffi::c_int as isize)
        >> 32 as ::core::ffi::c_int
        | *product.offset(5 as ::core::ffi::c_int as isize) << 32 as ::core::ffi::c_int;
    tmp[3 as ::core::ffi::c_int as usize] =
        *product.offset(6 as ::core::ffi::c_int as isize) << 32 as ::core::ffi::c_int;
    carry = (carry as uECC_word_t).wrapping_sub(uECC_vli_sub(
        result as *mut uECC_word_t,
        result,
        &raw mut tmp as *mut uint64_t,
        num_words_secp256r1 as wordcount_t,
    )) as ::core::ffi::c_int as ::core::ffi::c_int;
    tmp[0 as ::core::ffi::c_int as usize] = *product.offset(7 as ::core::ffi::c_int as isize);
    tmp[1 as ::core::ffi::c_int as usize] =
        (*product.offset(4 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulonglong
            & 0xffffffff00000000 as ::core::ffi::c_ulonglong) as uint64_t;
    tmp[2 as ::core::ffi::c_int as usize] = *product.offset(5 as ::core::ffi::c_int as isize);
    tmp[3 as ::core::ffi::c_int as usize] =
        (*product.offset(6 as ::core::ffi::c_int as isize) as ::core::ffi::c_ulonglong
            & 0xffffffff00000000 as ::core::ffi::c_ulonglong) as uint64_t;
    carry = (carry as uECC_word_t).wrapping_sub(uECC_vli_sub(
        result as *mut uECC_word_t,
        result,
        &raw mut tmp as *mut uint64_t,
        num_words_secp256r1 as wordcount_t,
    )) as ::core::ffi::c_int as ::core::ffi::c_int;
    if carry < 0 as ::core::ffi::c_int {
        loop {
            carry = (carry as uECC_word_t).wrapping_add(uECC_vli_add(
                result as *mut uECC_word_t,
                result,
                &raw const curve_secp256r1.p as *const uECC_word_t,
                num_words_secp256r1 as wordcount_t,
            )) as ::core::ffi::c_int as ::core::ffi::c_int;
            if !(carry < 0 as ::core::ffi::c_int) {
                break;
            }
        }
    } else {
        while carry != 0
            || uECC_vli_cmp_unsafe(
                &raw const curve_secp256r1.p as *const uECC_word_t,
                result,
                num_words_secp256r1 as wordcount_t,
            ) as ::core::ffi::c_int
                != 1 as ::core::ffi::c_int
        {
            carry = (carry as uECC_word_t).wrapping_sub(uECC_vli_sub(
                result as *mut uECC_word_t,
                result,
                &raw const curve_secp256r1.p as *const uECC_word_t,
                num_words_secp256r1 as wordcount_t,
            )) as ::core::ffi::c_int as ::core::ffi::c_int;
        }
    };
}
static mut curve_secp256k1: uECC_Curve_t = uECC_Curve_t {
    num_words: num_words_secp256k1 as wordcount_t,
    num_bytes: num_bytes_secp256k1 as wordcount_t,
    num_n_bits: 256 as bitcount_t,
    p: [
        0xfffffffefffffc2f as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffffffffffff as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffffffffffff as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffffffffffff as ::core::ffi::c_ulonglong as uECC_word_t,
    ],
    n: [
        0xbfd25e8cd0364141 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xbaaedce6af48a03b as ::core::ffi::c_ulonglong as uECC_word_t,
        0xfffffffffffffffe as ::core::ffi::c_ulonglong as uECC_word_t,
        0xffffffffffffffff as ::core::ffi::c_ulonglong as uECC_word_t,
    ],
    G: [
        0x59f2815b16f81798 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x29bfcdb2dce28d9 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x55a06295ce870b07 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x79be667ef9dcbbac as ::core::ffi::c_ulonglong as uECC_word_t,
        0x9c47d08ffb10d4b8 as ::core::ffi::c_ulonglong as uECC_word_t,
        0xfd17b448a6855419 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x5da4fbfc0e1108a8 as ::core::ffi::c_ulonglong as uECC_word_t,
        0x483ada7726a3c465 as ::core::ffi::c_ulonglong as uECC_word_t,
    ],
    b: [
        0x7 as ::core::ffi::c_ulonglong as uECC_word_t,
        0 as ::core::ffi::c_ulonglong as uECC_word_t,
        0 as ::core::ffi::c_ulonglong as uECC_word_t,
        0 as ::core::ffi::c_ulonglong as uECC_word_t,
    ],
    double_jacobian: Some(
        double_jacobian_secp256k1
            as unsafe extern "C" fn(
                *mut uECC_word_t,
                *mut uECC_word_t,
                *mut uECC_word_t,
                uECC_Curve,
            ) -> (),
    ),
    mod_sqrt: Some(mod_sqrt_default as unsafe extern "C" fn(*mut uECC_word_t, uECC_Curve) -> ()),
    x_side: Some(
        x_side_secp256k1
            as unsafe extern "C" fn(*mut uECC_word_t, *const uECC_word_t, uECC_Curve) -> (),
    ),
    mmod_fast: Some(
        vli_mmod_fast_secp256k1 as unsafe extern "C" fn(*mut uECC_word_t, *mut uECC_word_t) -> (),
    ),
};
#[no_mangle]
pub unsafe extern "C" fn uECC_secp256k1() -> uECC_Curve {
    return &raw const curve_secp256k1;
}
unsafe extern "C" fn double_jacobian_secp256k1(
    mut X1: *mut uECC_word_t,
    mut Y1: *mut uECC_word_t,
    mut Z1: *mut uECC_word_t,
    mut curve: uECC_Curve,
) {
    let mut t4: [uECC_word_t; 4] = [0; 4];
    let mut t5: [uECC_word_t; 4] = [0; 4];
    if uECC_vli_isZero(Z1, num_words_secp256k1 as wordcount_t) != 0 {
        return;
    }
    uECC_vli_modSquare_fast(&raw mut t5 as *mut uECC_word_t, Y1, curve);
    uECC_vli_modMult_fast(
        &raw mut t4 as *mut uECC_word_t,
        X1,
        &raw mut t5 as *mut uECC_word_t,
        curve,
    );
    uECC_vli_modSquare_fast(X1, X1, curve);
    uECC_vli_modSquare_fast(
        &raw mut t5 as *mut uECC_word_t,
        &raw mut t5 as *mut uECC_word_t,
        curve,
    );
    uECC_vli_modMult_fast(Z1, Y1, Z1, curve);
    uECC_vli_modAdd(
        Y1,
        X1,
        X1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words_secp256k1 as wordcount_t,
    );
    uECC_vli_modAdd(
        Y1,
        Y1,
        X1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words_secp256k1 as wordcount_t,
    );
    if uECC_vli_testBit(Y1, 0 as bitcount_t) != 0 {
        let mut carry: uECC_word_t = uECC_vli_add(
            Y1,
            Y1,
            &raw const (*curve).p as *const uECC_word_t,
            num_words_secp256k1 as wordcount_t,
        );
        uECC_vli_rshift1(Y1, num_words_secp256k1 as wordcount_t);
        *Y1.offset((num_words_secp256k1 - 1 as ::core::ffi::c_int) as isize) |=
            carry << uECC_WORD_BITS - 1 as ::core::ffi::c_int;
    } else {
        uECC_vli_rshift1(Y1, num_words_secp256k1 as wordcount_t);
    }
    uECC_vli_modSquare_fast(X1, Y1, curve);
    uECC_vli_modSub(
        X1,
        X1,
        &raw mut t4 as *mut uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words_secp256k1 as wordcount_t,
    );
    uECC_vli_modSub(
        X1,
        X1,
        &raw mut t4 as *mut uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words_secp256k1 as wordcount_t,
    );
    uECC_vli_modSub(
        &raw mut t4 as *mut uECC_word_t,
        &raw mut t4 as *mut uECC_word_t,
        X1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words_secp256k1 as wordcount_t,
    );
    uECC_vli_modMult_fast(Y1, Y1, &raw mut t4 as *mut uECC_word_t, curve);
    uECC_vli_modSub(
        Y1,
        Y1,
        &raw mut t5 as *mut uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words_secp256k1 as wordcount_t,
    );
}
unsafe extern "C" fn x_side_secp256k1(
    mut result: *mut uECC_word_t,
    mut x: *const uECC_word_t,
    mut curve: uECC_Curve,
) {
    uECC_vli_modSquare_fast(result, x, curve);
    uECC_vli_modMult_fast(result, result, x, curve);
    uECC_vli_modAdd(
        result,
        result,
        &raw const (*curve).b as *const uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words_secp256k1 as wordcount_t,
    );
}
unsafe extern "C" fn vli_mmod_fast_secp256k1(
    mut result: *mut uECC_word_t,
    mut product: *mut uECC_word_t,
) {
    let mut tmp: [uECC_word_t; 8] = [0; 8];
    let mut carry: uECC_word_t = 0;
    uECC_vli_clear(
        &raw mut tmp as *mut uECC_word_t,
        num_words_secp256k1 as wordcount_t,
    );
    uECC_vli_clear(
        (&raw mut tmp as *mut uECC_word_t).offset(num_words_secp256k1 as isize),
        num_words_secp256k1 as wordcount_t,
    );
    omega_mult_secp256k1(
        &raw mut tmp as *mut uint64_t,
        product.offset(num_words_secp256k1 as isize),
    );
    carry = uECC_vli_add(
        result,
        product,
        &raw mut tmp as *mut uECC_word_t,
        num_words_secp256k1 as wordcount_t,
    );
    uECC_vli_clear(product, num_words_secp256k1 as wordcount_t);
    omega_mult_secp256k1(
        product as *mut uint64_t,
        (&raw mut tmp as *mut uECC_word_t).offset(num_words_secp256k1 as isize),
    );
    carry = carry.wrapping_add(uECC_vli_add(
        result,
        result,
        product,
        num_words_secp256k1 as wordcount_t,
    ));
    while carry > 0 as uECC_word_t {
        carry = carry.wrapping_sub(1);
        uECC_vli_sub(
            result,
            result,
            &raw const curve_secp256k1.p as *const uECC_word_t,
            num_words_secp256k1 as wordcount_t,
        );
    }
    if uECC_vli_cmp_unsafe(
        result,
        &raw const curve_secp256k1.p as *const uECC_word_t,
        num_words_secp256k1 as wordcount_t,
    ) as ::core::ffi::c_int
        > 0 as ::core::ffi::c_int
    {
        uECC_vli_sub(
            result,
            result,
            &raw const curve_secp256k1.p as *const uECC_word_t,
            num_words_secp256k1 as wordcount_t,
        );
    }
}
unsafe extern "C" fn omega_mult_secp256k1(mut result: *mut uint64_t, mut right: *const uint64_t) {
    let mut r0: uECC_word_t = 0 as uECC_word_t;
    let mut r1: uECC_word_t = 0 as uECC_word_t;
    let mut r2: uECC_word_t = 0 as uECC_word_t;
    let mut k: wordcount_t = 0;
    k = 0 as wordcount_t;
    while (k as ::core::ffi::c_int) < num_words_secp256k1 {
        muladd(
            0x1000003d1 as uECC_word_t,
            *right.offset(k as isize) as uECC_word_t,
            &raw mut r0,
            &raw mut r1,
            &raw mut r2,
        );
        *result.offset(k as isize) = r0 as uint64_t;
        r0 = r1;
        r1 = r2;
        r2 = 0 as uECC_word_t;
        k += 1;
    }
    *result.offset(num_words_secp256k1 as isize) = r0 as uint64_t;
}
unsafe extern "C" fn apply_z(
    mut X1: *mut uECC_word_t,
    mut Y1: *mut uECC_word_t,
    Z: *const uECC_word_t,
    mut curve: uECC_Curve,
) {
    let mut t1: [uECC_word_t; 4] = [0; 4];
    uECC_vli_modSquare_fast(&raw mut t1 as *mut uECC_word_t, Z, curve);
    uECC_vli_modMult_fast(X1, X1, &raw mut t1 as *mut uECC_word_t, curve);
    uECC_vli_modMult_fast(
        &raw mut t1 as *mut uECC_word_t,
        &raw mut t1 as *mut uECC_word_t,
        Z,
        curve,
    );
    uECC_vli_modMult_fast(Y1, Y1, &raw mut t1 as *mut uECC_word_t, curve);
}
unsafe extern "C" fn XYcZ_initial_double(
    mut X1: *mut uECC_word_t,
    mut Y1: *mut uECC_word_t,
    mut X2: *mut uECC_word_t,
    mut Y2: *mut uECC_word_t,
    initial_Z: *const uECC_word_t,
    mut curve: uECC_Curve,
) {
    let mut z: [uECC_word_t; 4] = [0; 4];
    let mut num_words: wordcount_t = (*curve).num_words;
    if !initial_Z.is_null() {
        uECC_vli_set(&raw mut z as *mut uECC_word_t, initial_Z, num_words);
    } else {
        uECC_vli_clear(&raw mut z as *mut uECC_word_t, num_words);
        z[0 as ::core::ffi::c_int as usize] = 1 as uECC_word_t;
    }
    uECC_vli_set(X2, X1, num_words);
    uECC_vli_set(Y2, Y1, num_words);
    apply_z(X1, Y1, &raw mut z as *mut uECC_word_t, curve);
    (*curve).double_jacobian.expect("non-null function pointer")(
        X1,
        Y1,
        &raw mut z as *mut uECC_word_t,
        curve,
    );
    apply_z(X2, Y2, &raw mut z as *mut uECC_word_t, curve);
}
unsafe extern "C" fn XYcZ_add(
    mut X1: *mut uECC_word_t,
    mut Y1: *mut uECC_word_t,
    mut X2: *mut uECC_word_t,
    mut Y2: *mut uECC_word_t,
    mut curve: uECC_Curve,
) {
    let mut t5: [uECC_word_t; 4] = [0; 4];
    let mut num_words: wordcount_t = (*curve).num_words;
    uECC_vli_modSub(
        &raw mut t5 as *mut uECC_word_t,
        X2,
        X1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modSquare_fast(
        &raw mut t5 as *mut uECC_word_t,
        &raw mut t5 as *mut uECC_word_t,
        curve,
    );
    uECC_vli_modMult_fast(X1, X1, &raw mut t5 as *mut uECC_word_t, curve);
    uECC_vli_modMult_fast(X2, X2, &raw mut t5 as *mut uECC_word_t, curve);
    uECC_vli_modSub(
        Y2,
        Y2,
        Y1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modSquare_fast(&raw mut t5 as *mut uECC_word_t, Y2, curve);
    uECC_vli_modSub(
        &raw mut t5 as *mut uECC_word_t,
        &raw mut t5 as *mut uECC_word_t,
        X1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modSub(
        &raw mut t5 as *mut uECC_word_t,
        &raw mut t5 as *mut uECC_word_t,
        X2,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modSub(
        X2,
        X2,
        X1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modMult_fast(Y1, Y1, X2, curve);
    uECC_vli_modSub(
        X2,
        X1,
        &raw mut t5 as *mut uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modMult_fast(Y2, Y2, X2, curve);
    uECC_vli_modSub(
        Y2,
        Y2,
        Y1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_set(X2, &raw mut t5 as *mut uECC_word_t, num_words);
}
unsafe extern "C" fn XYcZ_addC(
    mut X1: *mut uECC_word_t,
    mut Y1: *mut uECC_word_t,
    mut X2: *mut uECC_word_t,
    mut Y2: *mut uECC_word_t,
    mut curve: uECC_Curve,
) {
    let mut t5: [uECC_word_t; 4] = [0; 4];
    let mut t6: [uECC_word_t; 4] = [0; 4];
    let mut t7: [uECC_word_t; 4] = [0; 4];
    let mut num_words: wordcount_t = (*curve).num_words;
    uECC_vli_modSub(
        &raw mut t5 as *mut uECC_word_t,
        X2,
        X1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modSquare_fast(
        &raw mut t5 as *mut uECC_word_t,
        &raw mut t5 as *mut uECC_word_t,
        curve,
    );
    uECC_vli_modMult_fast(X1, X1, &raw mut t5 as *mut uECC_word_t, curve);
    uECC_vli_modMult_fast(X2, X2, &raw mut t5 as *mut uECC_word_t, curve);
    uECC_vli_modAdd(
        &raw mut t5 as *mut uECC_word_t,
        Y2,
        Y1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modSub(
        Y2,
        Y2,
        Y1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modSub(
        &raw mut t6 as *mut uECC_word_t,
        X2,
        X1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modMult_fast(Y1, Y1, &raw mut t6 as *mut uECC_word_t, curve);
    uECC_vli_modAdd(
        &raw mut t6 as *mut uECC_word_t,
        X1,
        X2,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modSquare_fast(X2, Y2, curve);
    uECC_vli_modSub(
        X2,
        X2,
        &raw mut t6 as *mut uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modSub(
        &raw mut t7 as *mut uECC_word_t,
        X1,
        X2,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modMult_fast(Y2, Y2, &raw mut t7 as *mut uECC_word_t, curve);
    uECC_vli_modSub(
        Y2,
        Y2,
        Y1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modSquare_fast(
        &raw mut t7 as *mut uECC_word_t,
        &raw mut t5 as *mut uECC_word_t,
        curve,
    );
    uECC_vli_modSub(
        &raw mut t7 as *mut uECC_word_t,
        &raw mut t7 as *mut uECC_word_t,
        &raw mut t6 as *mut uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modSub(
        &raw mut t6 as *mut uECC_word_t,
        &raw mut t7 as *mut uECC_word_t,
        X1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modMult_fast(
        &raw mut t6 as *mut uECC_word_t,
        &raw mut t6 as *mut uECC_word_t,
        &raw mut t5 as *mut uECC_word_t,
        curve,
    );
    uECC_vli_modSub(
        Y1,
        &raw mut t6 as *mut uECC_word_t,
        Y1,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_set(X1, &raw mut t7 as *mut uECC_word_t, num_words);
}
unsafe extern "C" fn EccPoint_mult(
    mut result: *mut uECC_word_t,
    mut point: *const uECC_word_t,
    mut scalar: *const uECC_word_t,
    mut initial_Z: *const uECC_word_t,
    mut num_bits: bitcount_t,
    mut curve: uECC_Curve,
) {
    let mut Rx: [[uECC_word_t; 4]; 2] = [[0; 4]; 2];
    let mut Ry: [[uECC_word_t; 4]; 2] = [[0; 4]; 2];
    let mut z: [uECC_word_t; 4] = [0; 4];
    let mut i: bitcount_t = 0;
    let mut nb: uECC_word_t = 0;
    let mut num_words: wordcount_t = (*curve).num_words;
    uECC_vli_set(
        &raw mut *(&raw mut Rx as *mut [uECC_word_t; 4]).offset(1 as ::core::ffi::c_int as isize)
            as *mut uECC_word_t,
        point,
        num_words,
    );
    uECC_vli_set(
        &raw mut *(&raw mut Ry as *mut [uECC_word_t; 4]).offset(1 as ::core::ffi::c_int as isize)
            as *mut uECC_word_t,
        point.offset(num_words as ::core::ffi::c_int as isize),
        num_words,
    );
    XYcZ_initial_double(
        &raw mut *(&raw mut Rx as *mut [uECC_word_t; 4]).offset(1 as ::core::ffi::c_int as isize)
            as *mut uECC_word_t,
        &raw mut *(&raw mut Ry as *mut [uECC_word_t; 4]).offset(1 as ::core::ffi::c_int as isize)
            as *mut uECC_word_t,
        &raw mut *(&raw mut Rx as *mut [uECC_word_t; 4]).offset(0 as ::core::ffi::c_int as isize)
            as *mut uECC_word_t,
        &raw mut *(&raw mut Ry as *mut [uECC_word_t; 4]).offset(0 as ::core::ffi::c_int as isize)
            as *mut uECC_word_t,
        initial_Z,
        curve,
    );
    i = (num_bits as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as bitcount_t;
    while i as ::core::ffi::c_int > 0 as ::core::ffi::c_int {
        nb = (uECC_vli_testBit(scalar, i) == 0) as ::core::ffi::c_int as uECC_word_t;
        XYcZ_addC(
            &raw mut *(&raw mut Rx as *mut [uECC_word_t; 4])
                .offset((1 as uECC_word_t).wrapping_sub(nb) as isize)
                as *mut uECC_word_t,
            &raw mut *(&raw mut Ry as *mut [uECC_word_t; 4])
                .offset((1 as uECC_word_t).wrapping_sub(nb) as isize)
                as *mut uECC_word_t,
            &raw mut *(&raw mut Rx as *mut [uECC_word_t; 4]).offset(nb as isize)
                as *mut uECC_word_t,
            &raw mut *(&raw mut Ry as *mut [uECC_word_t; 4]).offset(nb as isize)
                as *mut uECC_word_t,
            curve,
        );
        XYcZ_add(
            &raw mut *(&raw mut Rx as *mut [uECC_word_t; 4]).offset(nb as isize)
                as *mut uECC_word_t,
            &raw mut *(&raw mut Ry as *mut [uECC_word_t; 4]).offset(nb as isize)
                as *mut uECC_word_t,
            &raw mut *(&raw mut Rx as *mut [uECC_word_t; 4])
                .offset((1 as uECC_word_t).wrapping_sub(nb) as isize)
                as *mut uECC_word_t,
            &raw mut *(&raw mut Ry as *mut [uECC_word_t; 4])
                .offset((1 as uECC_word_t).wrapping_sub(nb) as isize)
                as *mut uECC_word_t,
            curve,
        );
        i -= 1;
    }
    nb = (uECC_vli_testBit(scalar, 0 as bitcount_t) == 0) as ::core::ffi::c_int as uECC_word_t;
    XYcZ_addC(
        &raw mut *(&raw mut Rx as *mut [uECC_word_t; 4])
            .offset((1 as uECC_word_t).wrapping_sub(nb) as isize) as *mut uECC_word_t,
        &raw mut *(&raw mut Ry as *mut [uECC_word_t; 4])
            .offset((1 as uECC_word_t).wrapping_sub(nb) as isize) as *mut uECC_word_t,
        &raw mut *(&raw mut Rx as *mut [uECC_word_t; 4]).offset(nb as isize) as *mut uECC_word_t,
        &raw mut *(&raw mut Ry as *mut [uECC_word_t; 4]).offset(nb as isize) as *mut uECC_word_t,
        curve,
    );
    uECC_vli_modSub(
        &raw mut z as *mut uECC_word_t,
        &raw mut *(&raw mut Rx as *mut [uECC_word_t; 4]).offset(1 as ::core::ffi::c_int as isize)
            as *mut uECC_word_t,
        &raw mut *(&raw mut Rx as *mut [uECC_word_t; 4]).offset(0 as ::core::ffi::c_int as isize)
            as *mut uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modMult_fast(
        &raw mut z as *mut uECC_word_t,
        &raw mut z as *mut uECC_word_t,
        &raw mut *(&raw mut Ry as *mut [uECC_word_t; 4])
            .offset((1 as uECC_word_t).wrapping_sub(nb) as isize) as *mut uECC_word_t,
        curve,
    );
    uECC_vli_modMult_fast(
        &raw mut z as *mut uECC_word_t,
        &raw mut z as *mut uECC_word_t,
        point,
        curve,
    );
    uECC_vli_modInv(
        &raw mut z as *mut uECC_word_t,
        &raw mut z as *mut uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    uECC_vli_modMult_fast(
        &raw mut z as *mut uECC_word_t,
        &raw mut z as *mut uECC_word_t,
        point.offset(num_words as ::core::ffi::c_int as isize),
        curve,
    );
    uECC_vli_modMult_fast(
        &raw mut z as *mut uECC_word_t,
        &raw mut z as *mut uECC_word_t,
        &raw mut *(&raw mut Rx as *mut [uECC_word_t; 4])
            .offset((1 as uECC_word_t).wrapping_sub(nb) as isize) as *mut uECC_word_t,
        curve,
    );
    XYcZ_add(
        &raw mut *(&raw mut Rx as *mut [uECC_word_t; 4]).offset(nb as isize) as *mut uECC_word_t,
        &raw mut *(&raw mut Ry as *mut [uECC_word_t; 4]).offset(nb as isize) as *mut uECC_word_t,
        &raw mut *(&raw mut Rx as *mut [uECC_word_t; 4])
            .offset((1 as uECC_word_t).wrapping_sub(nb) as isize) as *mut uECC_word_t,
        &raw mut *(&raw mut Ry as *mut [uECC_word_t; 4])
            .offset((1 as uECC_word_t).wrapping_sub(nb) as isize) as *mut uECC_word_t,
        curve,
    );
    apply_z(
        &raw mut *(&raw mut Rx as *mut [uECC_word_t; 4]).offset(0 as ::core::ffi::c_int as isize)
            as *mut uECC_word_t,
        &raw mut *(&raw mut Ry as *mut [uECC_word_t; 4]).offset(0 as ::core::ffi::c_int as isize)
            as *mut uECC_word_t,
        &raw mut z as *mut uECC_word_t,
        curve,
    );
    uECC_vli_set(
        result,
        &raw mut *(&raw mut Rx as *mut [uECC_word_t; 4]).offset(0 as ::core::ffi::c_int as isize)
            as *mut uECC_word_t,
        num_words,
    );
    uECC_vli_set(
        result.offset(num_words as ::core::ffi::c_int as isize),
        &raw mut *(&raw mut Ry as *mut [uECC_word_t; 4]).offset(0 as ::core::ffi::c_int as isize)
            as *mut uECC_word_t,
        num_words,
    );
}
unsafe extern "C" fn regularize_k(
    k: *const uECC_word_t,
    mut k0: *mut uECC_word_t,
    mut k1: *mut uECC_word_t,
    mut curve: uECC_Curve,
) -> uECC_word_t {
    let mut num_n_words: wordcount_t = (((*curve).num_n_bits as ::core::ffi::c_int
        + (uECC_WORD_SIZE * 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
        / (uECC_WORD_SIZE * 8 as ::core::ffi::c_int))
        as wordcount_t;
    let mut num_n_bits: bitcount_t = (*curve).num_n_bits;
    let mut carry: uECC_word_t = (uECC_vli_add(
        k0,
        k,
        &raw const (*curve).n as *const uECC_word_t,
        num_n_words,
    ) != 0
        || (num_n_bits as ::core::ffi::c_int)
            < num_n_words as bitcount_t as ::core::ffi::c_int
                * uECC_WORD_SIZE
                * 8 as ::core::ffi::c_int
            && uECC_vli_testBit(k0, num_n_bits) != 0)
        as ::core::ffi::c_int as uECC_word_t;
    uECC_vli_add(
        k1,
        k0,
        &raw const (*curve).n as *const uECC_word_t,
        num_n_words,
    );
    return carry;
}
unsafe extern "C" fn uECC_generate_random_int(
    mut random: *mut uECC_word_t,
    mut top: *const uECC_word_t,
    mut num_words: wordcount_t,
) -> ::core::ffi::c_int {
    let mut mask: uECC_word_t = -(1 as ::core::ffi::c_int) as uECC_word_t;
    let mut tries: uECC_word_t = 0;
    let mut num_bits: bitcount_t = uECC_vli_numBits(top, num_words);
    if g_rng_function.is_none() {
        return 0 as ::core::ffi::c_int;
    }
    tries = 0 as uECC_word_t;
    while tries < uECC_RNG_MAX_TRIES as uECC_word_t {
        if g_rng_function.expect("non-null function pointer")(
            random as *mut uint8_t,
            (num_words as ::core::ffi::c_int * uECC_WORD_SIZE) as ::core::ffi::c_uint,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        *random.offset((num_words as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as isize) &= mask
            >> (num_words as ::core::ffi::c_int * uECC_WORD_SIZE * 8 as ::core::ffi::c_int
                - num_bits as ::core::ffi::c_int) as bitcount_t
                as ::core::ffi::c_int;
        if uECC_vli_isZero(random, num_words) == 0
            && uECC_vli_cmp(top, random, num_words) as ::core::ffi::c_int == 1 as ::core::ffi::c_int
        {
            return 1 as ::core::ffi::c_int;
        }
        tries = tries.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn EccPoint_compute_public_key(
    mut result: *mut uECC_word_t,
    mut private_key: *mut uECC_word_t,
    mut curve: uECC_Curve,
) -> uECC_word_t {
    let mut tmp1: [uECC_word_t; 4] = [0; 4];
    let mut tmp2: [uECC_word_t; 4] = [0; 4];
    let mut p2: [*mut uECC_word_t; 2] = [
        &raw mut tmp1 as *mut uECC_word_t,
        &raw mut tmp2 as *mut uECC_word_t,
    ];
    let mut initial_Z: *mut uECC_word_t = ::core::ptr::null_mut::<uECC_word_t>();
    let mut carry: uECC_word_t = 0;
    carry = regularize_k(
        private_key,
        &raw mut tmp1 as *mut uECC_word_t,
        &raw mut tmp2 as *mut uECC_word_t,
        curve,
    );
    if g_rng_function.is_some() {
        if uECC_generate_random_int(
            p2[carry as usize],
            &raw const (*curve).p as *const uECC_word_t,
            (*curve).num_words,
        ) == 0
        {
            return 0 as uECC_word_t;
        }
        initial_Z = p2[carry as usize];
    }
    EccPoint_mult(
        result,
        &raw const (*curve).G as *const uECC_word_t,
        p2[(carry == 0) as ::core::ffi::c_int as usize],
        initial_Z,
        ((*curve).num_n_bits as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as bitcount_t,
        curve,
    );
    if uECC_vli_isZero(
        result,
        ((*curve).num_words as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as wordcount_t,
    ) != 0
    {
        return 0 as uECC_word_t;
    }
    return 1 as uECC_word_t;
}
unsafe extern "C" fn uECC_vli_nativeToBytes(
    mut bytes: *mut uint8_t,
    mut num_bytes: ::core::ffi::c_int,
    mut native: *const uECC_word_t,
) {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i < num_bytes {
        let mut b: ::core::ffi::c_uint =
            (num_bytes - 1 as ::core::ffi::c_int - i) as ::core::ffi::c_uint;
        *bytes.offset(i as isize) = (*native
            .offset(b.wrapping_div(uECC_WORD_SIZE as ::core::ffi::c_uint) as isize)
            >> (8 as ::core::ffi::c_uint)
                .wrapping_mul(b.wrapping_rem(uECC_WORD_SIZE as ::core::ffi::c_uint)))
            as uint8_t;
        i += 1;
    }
}
unsafe extern "C" fn uECC_vli_bytesToNative(
    mut native: *mut uECC_word_t,
    mut bytes: *const uint8_t,
    mut num_bytes: ::core::ffi::c_int,
) {
    let mut i: ::core::ffi::c_int = 0;
    uECC_vli_clear(
        native,
        ((num_bytes + (uECC_WORD_SIZE - 1 as ::core::ffi::c_int)) / uECC_WORD_SIZE) as wordcount_t,
    );
    i = 0 as ::core::ffi::c_int;
    while i < num_bytes {
        let mut b: ::core::ffi::c_uint =
            (num_bytes - 1 as ::core::ffi::c_int - i) as ::core::ffi::c_uint;
        *native.offset(b.wrapping_div(uECC_WORD_SIZE as ::core::ffi::c_uint) as isize) |=
            (*bytes.offset(i as isize) as uECC_word_t)
                << (8 as ::core::ffi::c_uint)
                    .wrapping_mul(b.wrapping_rem(uECC_WORD_SIZE as ::core::ffi::c_uint));
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn uECC_make_key(
    mut public_key: *mut uint8_t,
    mut private_key: *mut uint8_t,
    mut curve: uECC_Curve,
) -> ::core::ffi::c_int {
    let mut _private: [uECC_word_t; 4] = [0; 4];
    let mut _public: [uECC_word_t; 8] = [0; 8];
    let mut tries: uECC_word_t = 0;
    tries = 0 as uECC_word_t;
    while tries < uECC_RNG_MAX_TRIES as uECC_word_t {
        if uECC_generate_random_int(
            &raw mut _private as *mut uECC_word_t,
            &raw const (*curve).n as *const uECC_word_t,
            (((*curve).num_n_bits as ::core::ffi::c_int
                + (uECC_WORD_SIZE * 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                / (uECC_WORD_SIZE * 8 as ::core::ffi::c_int)) as wordcount_t,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if EccPoint_compute_public_key(
            &raw mut _public as *mut uECC_word_t,
            &raw mut _private as *mut uECC_word_t,
            curve,
        ) != 0
        {
            uECC_vli_nativeToBytes(
                private_key,
                ((*curve).num_n_bits as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
                    / 8 as ::core::ffi::c_int,
                &raw mut _private as *mut uECC_word_t,
            );
            uECC_vli_nativeToBytes(
                public_key,
                (*curve).num_bytes as ::core::ffi::c_int,
                &raw mut _public as *mut uECC_word_t,
            );
            uECC_vli_nativeToBytes(
                public_key.offset((*curve).num_bytes as ::core::ffi::c_int as isize),
                (*curve).num_bytes as ::core::ffi::c_int,
                (&raw mut _public as *mut uECC_word_t)
                    .offset((*curve).num_words as ::core::ffi::c_int as isize),
            );
            return 1 as ::core::ffi::c_int;
        }
        tries = tries.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uECC_shared_secret(
    mut public_key: *const uint8_t,
    mut private_key: *const uint8_t,
    mut secret: *mut uint8_t,
    mut curve: uECC_Curve,
) -> ::core::ffi::c_int {
    let mut _public: [uECC_word_t; 8] = [0; 8];
    let mut _private: [uECC_word_t; 4] = [0; 4];
    let mut tmp: [uECC_word_t; 4] = [0; 4];
    let mut p2: [*mut uECC_word_t; 2] = [
        &raw mut _private as *mut uECC_word_t,
        &raw mut tmp as *mut uECC_word_t,
    ];
    let mut initial_Z: *mut uECC_word_t = ::core::ptr::null_mut::<uECC_word_t>();
    let mut carry: uECC_word_t = 0;
    let mut num_words: wordcount_t = (*curve).num_words;
    let mut num_bytes: wordcount_t = (*curve).num_bytes;
    uECC_vli_bytesToNative(
        &raw mut _private as *mut uECC_word_t,
        private_key,
        ((*curve).num_n_bits as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
            / 8 as ::core::ffi::c_int,
    );
    uECC_vli_bytesToNative(
        &raw mut _public as *mut uECC_word_t,
        public_key,
        num_bytes as ::core::ffi::c_int,
    );
    uECC_vli_bytesToNative(
        (&raw mut _public as *mut uECC_word_t).offset(num_words as ::core::ffi::c_int as isize),
        public_key.offset(num_bytes as ::core::ffi::c_int as isize),
        num_bytes as ::core::ffi::c_int,
    );
    carry = regularize_k(
        &raw mut _private as *mut uECC_word_t,
        &raw mut _private as *mut uECC_word_t,
        &raw mut tmp as *mut uECC_word_t,
        curve,
    );
    if g_rng_function.is_some() {
        if uECC_generate_random_int(
            p2[carry as usize],
            &raw const (*curve).p as *const uECC_word_t,
            num_words,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        initial_Z = p2[carry as usize];
    }
    EccPoint_mult(
        &raw mut _public as *mut uECC_word_t,
        &raw mut _public as *mut uECC_word_t,
        p2[(carry == 0) as ::core::ffi::c_int as usize],
        initial_Z,
        ((*curve).num_n_bits as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as bitcount_t,
        curve,
    );
    uECC_vli_nativeToBytes(
        secret,
        num_bytes as ::core::ffi::c_int,
        &raw mut _public as *mut uECC_word_t,
    );
    return (uECC_vli_isZero(
        &raw mut _public as *mut uECC_word_t,
        ((*curve).num_words as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as wordcount_t,
    ) == 0) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uECC_compress(
    mut public_key: *const uint8_t,
    mut compressed: *mut uint8_t,
    mut curve: uECC_Curve,
) {
    let mut i: wordcount_t = 0;
    i = 0 as wordcount_t;
    while (i as ::core::ffi::c_int) < (*curve).num_bytes as ::core::ffi::c_int {
        *compressed.offset((i as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as isize) =
            *public_key.offset(i as isize);
        i += 1;
    }
    *compressed.offset(0 as ::core::ffi::c_int as isize) = (2 as ::core::ffi::c_int
        + (*public_key.offset(
            ((*curve).num_bytes as ::core::ffi::c_int * 2 as ::core::ffi::c_int
                - 1 as ::core::ffi::c_int) as isize,
        ) as ::core::ffi::c_int
            & 0x1 as ::core::ffi::c_int))
        as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn uECC_decompress(
    mut compressed: *const uint8_t,
    mut public_key: *mut uint8_t,
    mut curve: uECC_Curve,
) {
    let mut point: [uECC_word_t; 8] = [0; 8];
    let mut y: *mut uECC_word_t = (&raw mut point as *mut uECC_word_t)
        .offset((*curve).num_words as ::core::ffi::c_int as isize);
    uECC_vli_bytesToNative(
        &raw mut point as *mut uECC_word_t,
        compressed.offset(1 as ::core::ffi::c_int as isize),
        (*curve).num_bytes as ::core::ffi::c_int,
    );
    (*curve).x_side.expect("non-null function pointer")(
        y,
        &raw mut point as *mut uECC_word_t,
        curve,
    );
    (*curve).mod_sqrt.expect("non-null function pointer")(y, curve);
    if *y.offset(0 as ::core::ffi::c_int as isize) & 0x1 as uECC_word_t
        != (*compressed.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            & 0x1 as ::core::ffi::c_int) as uECC_word_t
    {
        uECC_vli_sub(
            y,
            &raw const (*curve).p as *const uECC_word_t,
            y,
            (*curve).num_words,
        );
    }
    uECC_vli_nativeToBytes(
        public_key,
        (*curve).num_bytes as ::core::ffi::c_int,
        &raw mut point as *mut uECC_word_t,
    );
    uECC_vli_nativeToBytes(
        public_key.offset((*curve).num_bytes as ::core::ffi::c_int as isize),
        (*curve).num_bytes as ::core::ffi::c_int,
        y,
    );
}
unsafe extern "C" fn uECC_valid_point(
    mut point: *const uECC_word_t,
    mut curve: uECC_Curve,
) -> ::core::ffi::c_int {
    let mut tmp1: [uECC_word_t; 4] = [0; 4];
    let mut tmp2: [uECC_word_t; 4] = [0; 4];
    let mut num_words: wordcount_t = (*curve).num_words;
    if uECC_vli_isZero(
        point,
        ((*curve).num_words as ::core::ffi::c_int * 2 as ::core::ffi::c_int) as wordcount_t,
    ) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if uECC_vli_cmp_unsafe(
        &raw const (*curve).p as *const uECC_word_t,
        point,
        num_words,
    ) as ::core::ffi::c_int
        != 1 as ::core::ffi::c_int
        || uECC_vli_cmp_unsafe(
            &raw const (*curve).p as *const uECC_word_t,
            point.offset(num_words as ::core::ffi::c_int as isize),
            num_words,
        ) as ::core::ffi::c_int
            != 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    uECC_vli_modSquare_fast(
        &raw mut tmp1 as *mut uECC_word_t,
        point.offset(num_words as ::core::ffi::c_int as isize),
        curve,
    );
    (*curve).x_side.expect("non-null function pointer")(
        &raw mut tmp2 as *mut uECC_word_t,
        point,
        curve,
    );
    return uECC_vli_equal(
        &raw mut tmp1 as *mut uECC_word_t,
        &raw mut tmp2 as *mut uECC_word_t,
        num_words,
    ) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uECC_valid_public_key(
    mut public_key: *const uint8_t,
    mut curve: uECC_Curve,
) -> ::core::ffi::c_int {
    let mut _public: [uECC_word_t; 8] = [0; 8];
    uECC_vli_bytesToNative(
        &raw mut _public as *mut uECC_word_t,
        public_key,
        (*curve).num_bytes as ::core::ffi::c_int,
    );
    uECC_vli_bytesToNative(
        (&raw mut _public as *mut uECC_word_t)
            .offset((*curve).num_words as ::core::ffi::c_int as isize),
        public_key.offset((*curve).num_bytes as ::core::ffi::c_int as isize),
        (*curve).num_bytes as ::core::ffi::c_int,
    );
    return uECC_valid_point(&raw mut _public as *mut uECC_word_t, curve);
}
#[no_mangle]
pub unsafe extern "C" fn uECC_compute_public_key(
    mut private_key: *const uint8_t,
    mut public_key: *mut uint8_t,
    mut curve: uECC_Curve,
) -> ::core::ffi::c_int {
    let mut _private: [uECC_word_t; 4] = [0; 4];
    let mut _public: [uECC_word_t; 8] = [0; 8];
    uECC_vli_bytesToNative(
        &raw mut _private as *mut uECC_word_t,
        private_key,
        ((*curve).num_n_bits as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
            / 8 as ::core::ffi::c_int,
    );
    if uECC_vli_isZero(
        &raw mut _private as *mut uECC_word_t,
        (((*curve).num_n_bits as ::core::ffi::c_int
            + (uECC_WORD_SIZE * 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            / (uECC_WORD_SIZE * 8 as ::core::ffi::c_int)) as wordcount_t,
    ) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if uECC_vli_cmp(
        &raw const (*curve).n as *const uECC_word_t,
        &raw mut _private as *mut uECC_word_t,
        (((*curve).num_n_bits as ::core::ffi::c_int
            + (uECC_WORD_SIZE * 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
            / (uECC_WORD_SIZE * 8 as ::core::ffi::c_int)) as wordcount_t,
    ) as ::core::ffi::c_int
        != 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    if EccPoint_compute_public_key(
        &raw mut _public as *mut uECC_word_t,
        &raw mut _private as *mut uECC_word_t,
        curve,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    uECC_vli_nativeToBytes(
        public_key,
        (*curve).num_bytes as ::core::ffi::c_int,
        &raw mut _public as *mut uECC_word_t,
    );
    uECC_vli_nativeToBytes(
        public_key.offset((*curve).num_bytes as ::core::ffi::c_int as isize),
        (*curve).num_bytes as ::core::ffi::c_int,
        (&raw mut _public as *mut uECC_word_t)
            .offset((*curve).num_words as ::core::ffi::c_int as isize),
    );
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn bits2int(
    mut native: *mut uECC_word_t,
    mut bits: *const uint8_t,
    mut bits_size: ::core::ffi::c_uint,
    mut curve: uECC_Curve,
) {
    let mut num_n_bytes: ::core::ffi::c_uint =
        (((*curve).num_n_bits as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
            / 8 as ::core::ffi::c_int) as ::core::ffi::c_uint;
    let mut num_n_words: ::core::ffi::c_uint = (((*curve).num_n_bits as ::core::ffi::c_int
        + (uECC_WORD_SIZE * 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
        / (uECC_WORD_SIZE * 8 as ::core::ffi::c_int))
        as ::core::ffi::c_uint;
    let mut shift: ::core::ffi::c_int = 0;
    let mut carry: uECC_word_t = 0;
    let mut ptr: *mut uECC_word_t = ::core::ptr::null_mut::<uECC_word_t>();
    if bits_size > num_n_bytes {
        bits_size = num_n_bytes;
    }
    uECC_vli_clear(native, num_n_words as wordcount_t);
    uECC_vli_bytesToNative(native, bits, bits_size as ::core::ffi::c_int);
    if bits_size.wrapping_mul(8 as ::core::ffi::c_uint)
        <= (*curve).num_n_bits as ::core::ffi::c_uint
    {
        return;
    }
    shift = bits_size
        .wrapping_mul(8 as ::core::ffi::c_uint)
        .wrapping_sub((*curve).num_n_bits as ::core::ffi::c_uint) as ::core::ffi::c_int;
    carry = 0 as uECC_word_t;
    ptr = native.offset(num_n_words as isize);
    loop {
        let c2rust_fresh2 = ptr;
        ptr = ptr.offset(-1);
        if !(c2rust_fresh2 > native) {
            break;
        }
        let mut temp: uECC_word_t = *ptr;
        *ptr = temp >> shift | carry;
        carry = temp << uECC_WORD_BITS - shift;
    }
    if uECC_vli_cmp_unsafe(
        &raw const (*curve).n as *const uECC_word_t,
        native,
        num_n_words as wordcount_t,
    ) as ::core::ffi::c_int
        != 1 as ::core::ffi::c_int
    {
        uECC_vli_sub(
            native,
            native,
            &raw const (*curve).n as *const uECC_word_t,
            num_n_words as wordcount_t,
        );
    }
}
unsafe extern "C" fn uECC_sign_with_k_internal(
    mut private_key: *const uint8_t,
    mut message_hash: *const uint8_t,
    mut hash_size: ::core::ffi::c_uint,
    mut k: *mut uECC_word_t,
    mut signature: *mut uint8_t,
    mut curve: uECC_Curve,
) -> ::core::ffi::c_int {
    let mut tmp: [uECC_word_t; 4] = [0; 4];
    let mut s: [uECC_word_t; 4] = [0; 4];
    let mut k2: [*mut uECC_word_t; 2] = [
        &raw mut tmp as *mut uECC_word_t,
        &raw mut s as *mut uECC_word_t,
    ];
    let mut initial_Z: *mut uECC_word_t = ::core::ptr::null_mut::<uECC_word_t>();
    let mut p: [uECC_word_t; 8] = [0; 8];
    let mut carry: uECC_word_t = 0;
    let mut num_words: wordcount_t = (*curve).num_words;
    let mut num_n_words: wordcount_t = (((*curve).num_n_bits as ::core::ffi::c_int
        + (uECC_WORD_SIZE * 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
        / (uECC_WORD_SIZE * 8 as ::core::ffi::c_int))
        as wordcount_t;
    let mut num_n_bits: bitcount_t = (*curve).num_n_bits;
    if uECC_vli_isZero(k, num_words) != 0
        || uECC_vli_cmp(&raw const (*curve).n as *const uECC_word_t, k, num_n_words)
            as ::core::ffi::c_int
            != 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    carry = regularize_k(
        k,
        &raw mut tmp as *mut uECC_word_t,
        &raw mut s as *mut uECC_word_t,
        curve,
    );
    if g_rng_function.is_some() {
        if uECC_generate_random_int(
            k2[carry as usize],
            &raw const (*curve).p as *const uECC_word_t,
            num_words,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        initial_Z = k2[carry as usize];
    }
    EccPoint_mult(
        &raw mut p as *mut uECC_word_t,
        &raw const (*curve).G as *const uECC_word_t,
        k2[(carry == 0) as ::core::ffi::c_int as usize],
        initial_Z,
        (num_n_bits as ::core::ffi::c_int + 1 as ::core::ffi::c_int) as bitcount_t,
        curve,
    );
    if uECC_vli_isZero(&raw mut p as *mut uECC_word_t, num_words) != 0 {
        return 0 as ::core::ffi::c_int;
    }
    if g_rng_function.is_none() {
        uECC_vli_clear(&raw mut tmp as *mut uECC_word_t, num_n_words);
        tmp[0 as ::core::ffi::c_int as usize] = 1 as uECC_word_t;
    } else if uECC_generate_random_int(
        &raw mut tmp as *mut uECC_word_t,
        &raw const (*curve).n as *const uECC_word_t,
        num_n_words,
    ) == 0
    {
        return 0 as ::core::ffi::c_int;
    }
    uECC_vli_modMult(
        k,
        k,
        &raw mut tmp as *mut uECC_word_t,
        &raw const (*curve).n as *const uECC_word_t,
        num_n_words,
    );
    uECC_vli_modInv(
        k,
        k,
        &raw const (*curve).n as *const uECC_word_t,
        num_n_words,
    );
    uECC_vli_modMult(
        k,
        k,
        &raw mut tmp as *mut uECC_word_t,
        &raw const (*curve).n as *const uECC_word_t,
        num_n_words,
    );
    uECC_vli_nativeToBytes(
        signature,
        (*curve).num_bytes as ::core::ffi::c_int,
        &raw mut p as *mut uECC_word_t,
    );
    uECC_vli_bytesToNative(
        &raw mut tmp as *mut uECC_word_t,
        private_key,
        ((*curve).num_n_bits as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
            / 8 as ::core::ffi::c_int,
    );
    s[(num_n_words as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = 0 as uECC_word_t;
    uECC_vli_set(
        &raw mut s as *mut uECC_word_t,
        &raw mut p as *mut uECC_word_t,
        num_words,
    );
    uECC_vli_modMult(
        &raw mut s as *mut uECC_word_t,
        &raw mut tmp as *mut uECC_word_t,
        &raw mut s as *mut uECC_word_t,
        &raw const (*curve).n as *const uECC_word_t,
        num_n_words,
    );
    bits2int(
        &raw mut tmp as *mut uECC_word_t,
        message_hash,
        hash_size,
        curve,
    );
    uECC_vli_modAdd(
        &raw mut s as *mut uECC_word_t,
        &raw mut tmp as *mut uECC_word_t,
        &raw mut s as *mut uECC_word_t,
        &raw const (*curve).n as *const uECC_word_t,
        num_n_words,
    );
    uECC_vli_modMult(
        &raw mut s as *mut uECC_word_t,
        &raw mut s as *mut uECC_word_t,
        k,
        &raw const (*curve).n as *const uECC_word_t,
        num_n_words,
    );
    if uECC_vli_numBits(&raw mut s as *mut uECC_word_t, num_n_words) as ::core::ffi::c_int
        > (*curve).num_bytes as bitcount_t as ::core::ffi::c_int * 8 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    uECC_vli_nativeToBytes(
        signature.offset((*curve).num_bytes as ::core::ffi::c_int as isize),
        (*curve).num_bytes as ::core::ffi::c_int,
        &raw mut s as *mut uECC_word_t,
    );
    return 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn uECC_sign_with_k(
    mut private_key: *const uint8_t,
    mut message_hash: *const uint8_t,
    mut hash_size: ::core::ffi::c_uint,
    mut k: *const uint8_t,
    mut signature: *mut uint8_t,
    mut curve: uECC_Curve,
) -> ::core::ffi::c_int {
    let mut k2: [uECC_word_t; 4] = [0; 4];
    bits2int(
        &raw mut k2 as *mut uECC_word_t,
        k,
        (((*curve).num_n_bits as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
            / 8 as ::core::ffi::c_int) as ::core::ffi::c_uint,
        curve,
    );
    return uECC_sign_with_k_internal(
        private_key,
        message_hash,
        hash_size,
        &raw mut k2 as *mut uECC_word_t,
        signature,
        curve,
    );
}
#[no_mangle]
pub unsafe extern "C" fn uECC_sign(
    mut private_key: *const uint8_t,
    mut message_hash: *const uint8_t,
    mut hash_size: ::core::ffi::c_uint,
    mut signature: *mut uint8_t,
    mut curve: uECC_Curve,
) -> ::core::ffi::c_int {
    let mut k: [uECC_word_t; 4] = [0; 4];
    let mut tries: uECC_word_t = 0;
    tries = 0 as uECC_word_t;
    while tries < uECC_RNG_MAX_TRIES as uECC_word_t {
        if uECC_generate_random_int(
            &raw mut k as *mut uECC_word_t,
            &raw const (*curve).n as *const uECC_word_t,
            (((*curve).num_n_bits as ::core::ffi::c_int
                + (uECC_WORD_SIZE * 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
                / (uECC_WORD_SIZE * 8 as ::core::ffi::c_int)) as wordcount_t,
        ) == 0
        {
            return 0 as ::core::ffi::c_int;
        }
        if uECC_sign_with_k_internal(
            private_key,
            message_hash,
            hash_size,
            &raw mut k as *mut uECC_word_t,
            signature,
            curve,
        ) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        tries = tries.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn HMAC_init(mut hash_context: *const uECC_HashContext, mut K: *const uint8_t) {
    let mut pad: *mut uint8_t = (*hash_context)
        .tmp
        .offset((2 as ::core::ffi::c_uint).wrapping_mul((*hash_context).result_size) as isize);
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < (*hash_context).result_size {
        *pad.offset(i as isize) =
            (*K.offset(i as isize) as ::core::ffi::c_int ^ 0x36 as ::core::ffi::c_int) as uint8_t;
        i = i.wrapping_add(1);
    }
    while i < (*hash_context).block_size {
        *pad.offset(i as isize) = 0x36 as uint8_t;
        i = i.wrapping_add(1);
    }
    (*hash_context)
        .init_hash
        .expect("non-null function pointer")(hash_context as *const uECC_HashContext);
    (*hash_context)
        .update_hash
        .expect("non-null function pointer")(
        hash_context as *const uECC_HashContext,
        pad,
        (*hash_context).block_size,
    );
}
unsafe extern "C" fn HMAC_update(
    mut hash_context: *const uECC_HashContext,
    mut message: *const uint8_t,
    mut message_size: ::core::ffi::c_uint,
) {
    (*hash_context)
        .update_hash
        .expect("non-null function pointer")(
        hash_context as *const uECC_HashContext,
        message,
        message_size,
    );
}
unsafe extern "C" fn HMAC_finish(
    mut hash_context: *const uECC_HashContext,
    mut K: *const uint8_t,
    mut result: *mut uint8_t,
) {
    let mut pad: *mut uint8_t = (*hash_context)
        .tmp
        .offset((2 as ::core::ffi::c_uint).wrapping_mul((*hash_context).result_size) as isize);
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < (*hash_context).result_size {
        *pad.offset(i as isize) =
            (*K.offset(i as isize) as ::core::ffi::c_int ^ 0x5c as ::core::ffi::c_int) as uint8_t;
        i = i.wrapping_add(1);
    }
    while i < (*hash_context).block_size {
        *pad.offset(i as isize) = 0x5c as uint8_t;
        i = i.wrapping_add(1);
    }
    (*hash_context)
        .finish_hash
        .expect("non-null function pointer")(hash_context as *const uECC_HashContext, result);
    (*hash_context)
        .init_hash
        .expect("non-null function pointer")(hash_context as *const uECC_HashContext);
    (*hash_context)
        .update_hash
        .expect("non-null function pointer")(
        hash_context as *const uECC_HashContext,
        pad,
        (*hash_context).block_size,
    );
    (*hash_context)
        .update_hash
        .expect("non-null function pointer")(
        hash_context as *const uECC_HashContext,
        result,
        (*hash_context).result_size,
    );
    (*hash_context)
        .finish_hash
        .expect("non-null function pointer")(hash_context as *const uECC_HashContext, result);
}
unsafe extern "C" fn update_V(
    mut hash_context: *const uECC_HashContext,
    mut K: *mut uint8_t,
    mut V: *mut uint8_t,
) {
    HMAC_init(hash_context, K);
    HMAC_update(hash_context, V, (*hash_context).result_size);
    HMAC_finish(hash_context, K, V);
}
#[no_mangle]
pub unsafe extern "C" fn uECC_sign_deterministic(
    mut private_key: *const uint8_t,
    mut message_hash: *const uint8_t,
    mut hash_size: ::core::ffi::c_uint,
    mut hash_context: *const uECC_HashContext,
    mut signature: *mut uint8_t,
    mut curve: uECC_Curve,
) -> ::core::ffi::c_int {
    let mut K: *mut uint8_t = (*hash_context).tmp;
    let mut V: *mut uint8_t = K.offset((*hash_context).result_size as isize);
    let mut num_bytes: wordcount_t = (*curve).num_bytes;
    let mut num_n_words: wordcount_t = (((*curve).num_n_bits as ::core::ffi::c_int
        + (uECC_WORD_SIZE * 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
        / (uECC_WORD_SIZE * 8 as ::core::ffi::c_int))
        as wordcount_t;
    let mut num_n_bits: bitcount_t = (*curve).num_n_bits;
    let mut tries: uECC_word_t = 0;
    let mut i: ::core::ffi::c_uint = 0;
    i = 0 as ::core::ffi::c_uint;
    while i < (*hash_context).result_size {
        *V.offset(i as isize) = 0x1 as uint8_t;
        *K.offset(i as isize) = 0 as uint8_t;
        i = i.wrapping_add(1);
    }
    HMAC_init(hash_context, K);
    *V.offset((*hash_context).result_size as isize) = 0 as uint8_t;
    HMAC_update(
        hash_context,
        V,
        (*hash_context)
            .result_size
            .wrapping_add(1 as ::core::ffi::c_uint),
    );
    HMAC_update(hash_context, private_key, num_bytes as ::core::ffi::c_uint);
    HMAC_update(hash_context, message_hash, hash_size);
    HMAC_finish(hash_context, K, K);
    update_V(hash_context, K, V);
    HMAC_init(hash_context, K);
    *V.offset((*hash_context).result_size as isize) = 0x1 as uint8_t;
    HMAC_update(
        hash_context,
        V,
        (*hash_context)
            .result_size
            .wrapping_add(1 as ::core::ffi::c_uint),
    );
    HMAC_update(hash_context, private_key, num_bytes as ::core::ffi::c_uint);
    HMAC_update(hash_context, message_hash, hash_size);
    HMAC_finish(hash_context, K, K);
    update_V(hash_context, K, V);
    tries = 0 as uECC_word_t;
    while tries < uECC_RNG_MAX_TRIES as uECC_word_t {
        let mut T: [uECC_word_t; 4] = [0; 4];
        let mut T_ptr: *mut uint8_t = &raw mut T as *mut uECC_word_t as *mut uint8_t;
        let mut T_bytes: wordcount_t = 0 as wordcount_t;
        's_98: loop {
            update_V(hash_context, K, V);
            i = 0 as ::core::ffi::c_uint;
            while i < (*hash_context).result_size {
                let c2rust_fresh3 = T_bytes;
                T_bytes = T_bytes + 1;
                *T_ptr.offset(c2rust_fresh3 as isize) = *V.offset(i as isize);
                if T_bytes as ::core::ffi::c_int
                    >= num_n_words as ::core::ffi::c_int * uECC_WORD_SIZE
                {
                    break 's_98;
                }
                i = i.wrapping_add(1);
            }
        }
        if num_n_words as bitcount_t as ::core::ffi::c_int
            * uECC_WORD_SIZE
            * 8 as ::core::ffi::c_int
            > num_n_bits as ::core::ffi::c_int
        {
            let mut mask: uECC_word_t = -(1 as ::core::ffi::c_int) as uECC_word_t;
            T[(num_n_words as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] &= mask
                >> (num_n_words as ::core::ffi::c_int * uECC_WORD_SIZE * 8 as ::core::ffi::c_int
                    - num_n_bits as ::core::ffi::c_int) as bitcount_t
                    as ::core::ffi::c_int;
        }
        if uECC_sign_with_k_internal(
            private_key,
            message_hash,
            hash_size,
            &raw mut T as *mut uECC_word_t,
            signature,
            curve,
        ) != 0
        {
            return 1 as ::core::ffi::c_int;
        }
        HMAC_init(hash_context, K);
        *V.offset((*hash_context).result_size as isize) = 0 as uint8_t;
        HMAC_update(
            hash_context,
            V,
            (*hash_context)
                .result_size
                .wrapping_add(1 as ::core::ffi::c_uint),
        );
        HMAC_finish(hash_context, K, K);
        update_V(hash_context, K, V);
        tries = tries.wrapping_add(1);
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn smax(mut a: bitcount_t, mut b: bitcount_t) -> bitcount_t {
    return (if a as ::core::ffi::c_int > b as ::core::ffi::c_int {
        a as ::core::ffi::c_int
    } else {
        b as ::core::ffi::c_int
    }) as bitcount_t;
}
#[no_mangle]
pub unsafe extern "C" fn uECC_verify(
    mut public_key: *const uint8_t,
    mut message_hash: *const uint8_t,
    mut hash_size: ::core::ffi::c_uint,
    mut signature: *const uint8_t,
    mut curve: uECC_Curve,
) -> ::core::ffi::c_int {
    let mut u1: [uECC_word_t; 4] = [0; 4];
    let mut u2: [uECC_word_t; 4] = [0; 4];
    let mut z: [uECC_word_t; 4] = [0; 4];
    let mut sum: [uECC_word_t; 8] = [0; 8];
    let mut rx: [uECC_word_t; 4] = [0; 4];
    let mut ry: [uECC_word_t; 4] = [0; 4];
    let mut tx: [uECC_word_t; 4] = [0; 4];
    let mut ty: [uECC_word_t; 4] = [0; 4];
    let mut tz: [uECC_word_t; 4] = [0; 4];
    let mut points: [*const uECC_word_t; 4] = [::core::ptr::null::<uECC_word_t>(); 4];
    let mut point: *const uECC_word_t = ::core::ptr::null::<uECC_word_t>();
    let mut num_bits: bitcount_t = 0;
    let mut i: bitcount_t = 0;
    let mut _public: [uECC_word_t; 8] = [0; 8];
    let mut r: [uECC_word_t; 4] = [0; 4];
    let mut s: [uECC_word_t; 4] = [0; 4];
    let mut num_words: wordcount_t = (*curve).num_words;
    let mut num_n_words: wordcount_t = (((*curve).num_n_bits as ::core::ffi::c_int
        + (uECC_WORD_SIZE * 8 as ::core::ffi::c_int - 1 as ::core::ffi::c_int))
        / (uECC_WORD_SIZE * 8 as ::core::ffi::c_int))
        as wordcount_t;
    rx[(num_n_words as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = 0 as uECC_word_t;
    r[(num_n_words as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = 0 as uECC_word_t;
    s[(num_n_words as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = 0 as uECC_word_t;
    uECC_vli_bytesToNative(
        &raw mut _public as *mut uECC_word_t,
        public_key,
        (*curve).num_bytes as ::core::ffi::c_int,
    );
    uECC_vli_bytesToNative(
        (&raw mut _public as *mut uECC_word_t).offset(num_words as ::core::ffi::c_int as isize),
        public_key.offset((*curve).num_bytes as ::core::ffi::c_int as isize),
        (*curve).num_bytes as ::core::ffi::c_int,
    );
    uECC_vli_bytesToNative(
        &raw mut r as *mut uECC_word_t,
        signature,
        (*curve).num_bytes as ::core::ffi::c_int,
    );
    uECC_vli_bytesToNative(
        &raw mut s as *mut uECC_word_t,
        signature.offset((*curve).num_bytes as ::core::ffi::c_int as isize),
        (*curve).num_bytes as ::core::ffi::c_int,
    );
    if uECC_vli_isZero(&raw mut r as *mut uECC_word_t, num_words) != 0
        || uECC_vli_isZero(&raw mut s as *mut uECC_word_t, num_words) != 0
    {
        return 0 as ::core::ffi::c_int;
    }
    if uECC_vli_cmp_unsafe(
        &raw const (*curve).n as *const uECC_word_t,
        &raw mut r as *mut uECC_word_t,
        num_n_words,
    ) as ::core::ffi::c_int
        != 1 as ::core::ffi::c_int
        || uECC_vli_cmp_unsafe(
            &raw const (*curve).n as *const uECC_word_t,
            &raw mut s as *mut uECC_word_t,
            num_n_words,
        ) as ::core::ffi::c_int
            != 1 as ::core::ffi::c_int
    {
        return 0 as ::core::ffi::c_int;
    }
    uECC_vli_modInv(
        &raw mut z as *mut uECC_word_t,
        &raw mut s as *mut uECC_word_t,
        &raw const (*curve).n as *const uECC_word_t,
        num_n_words,
    );
    u1[(num_n_words as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as usize] = 0 as uECC_word_t;
    bits2int(
        &raw mut u1 as *mut uECC_word_t,
        message_hash,
        hash_size,
        curve,
    );
    uECC_vli_modMult(
        &raw mut u1 as *mut uECC_word_t,
        &raw mut u1 as *mut uECC_word_t,
        &raw mut z as *mut uECC_word_t,
        &raw const (*curve).n as *const uECC_word_t,
        num_n_words,
    );
    uECC_vli_modMult(
        &raw mut u2 as *mut uECC_word_t,
        &raw mut r as *mut uECC_word_t,
        &raw mut z as *mut uECC_word_t,
        &raw const (*curve).n as *const uECC_word_t,
        num_n_words,
    );
    uECC_vli_set(
        &raw mut sum as *mut uECC_word_t,
        &raw mut _public as *mut uECC_word_t,
        num_words,
    );
    uECC_vli_set(
        (&raw mut sum as *mut uECC_word_t).offset(num_words as ::core::ffi::c_int as isize),
        (&raw mut _public as *mut uECC_word_t).offset(num_words as ::core::ffi::c_int as isize),
        num_words,
    );
    uECC_vli_set(
        &raw mut tx as *mut uECC_word_t,
        &raw const (*curve).G as *const uECC_word_t,
        num_words,
    );
    uECC_vli_set(
        &raw mut ty as *mut uECC_word_t,
        (&raw const (*curve).G as *const uECC_word_t)
            .offset(num_words as ::core::ffi::c_int as isize),
        num_words,
    );
    uECC_vli_modSub(
        &raw mut z as *mut uECC_word_t,
        &raw mut sum as *mut uECC_word_t,
        &raw mut tx as *mut uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    XYcZ_add(
        &raw mut tx as *mut uECC_word_t,
        &raw mut ty as *mut uECC_word_t,
        &raw mut sum as *mut uECC_word_t,
        (&raw mut sum as *mut uECC_word_t).offset(num_words as ::core::ffi::c_int as isize),
        curve,
    );
    uECC_vli_modInv(
        &raw mut z as *mut uECC_word_t,
        &raw mut z as *mut uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    apply_z(
        &raw mut sum as *mut uECC_word_t,
        (&raw mut sum as *mut uECC_word_t).offset(num_words as ::core::ffi::c_int as isize),
        &raw mut z as *mut uECC_word_t,
        curve,
    );
    points[0 as ::core::ffi::c_int as usize] = ::core::ptr::null::<uECC_word_t>();
    points[1 as ::core::ffi::c_int as usize] = &raw const (*curve).G as *const uECC_word_t;
    points[2 as ::core::ffi::c_int as usize] = &raw mut _public as *mut uECC_word_t;
    points[3 as ::core::ffi::c_int as usize] = &raw mut sum as *mut uECC_word_t;
    num_bits = smax(
        uECC_vli_numBits(&raw mut u1 as *mut uECC_word_t, num_n_words),
        uECC_vli_numBits(&raw mut u2 as *mut uECC_word_t, num_n_words),
    );
    point = points[((uECC_vli_testBit(
        &raw mut u1 as *mut uECC_word_t,
        (num_bits as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as bitcount_t,
    ) != 0) as ::core::ffi::c_int
        | ((uECC_vli_testBit(
            &raw mut u2 as *mut uECC_word_t,
            (num_bits as ::core::ffi::c_int - 1 as ::core::ffi::c_int) as bitcount_t,
        ) != 0) as ::core::ffi::c_int)
            << 1 as ::core::ffi::c_int) as usize];
    uECC_vli_set(&raw mut rx as *mut uECC_word_t, point, num_words);
    uECC_vli_set(
        &raw mut ry as *mut uECC_word_t,
        point.offset(num_words as ::core::ffi::c_int as isize),
        num_words,
    );
    uECC_vli_clear(&raw mut z as *mut uECC_word_t, num_words);
    z[0 as ::core::ffi::c_int as usize] = 1 as uECC_word_t;
    i = (num_bits as ::core::ffi::c_int - 2 as ::core::ffi::c_int) as bitcount_t;
    while i as ::core::ffi::c_int >= 0 as ::core::ffi::c_int {
        let mut index: uECC_word_t = 0;
        (*curve).double_jacobian.expect("non-null function pointer")(
            &raw mut rx as *mut uECC_word_t,
            &raw mut ry as *mut uECC_word_t,
            &raw mut z as *mut uECC_word_t,
            curve,
        );
        index = ((uECC_vli_testBit(&raw mut u1 as *mut uECC_word_t, i) != 0) as ::core::ffi::c_int
            | ((uECC_vli_testBit(&raw mut u2 as *mut uECC_word_t, i) != 0) as ::core::ffi::c_int)
                << 1 as ::core::ffi::c_int) as uECC_word_t;
        point = points[index as usize];
        if !point.is_null() {
            uECC_vli_set(&raw mut tx as *mut uECC_word_t, point, num_words);
            uECC_vli_set(
                &raw mut ty as *mut uECC_word_t,
                point.offset(num_words as ::core::ffi::c_int as isize),
                num_words,
            );
            apply_z(
                &raw mut tx as *mut uECC_word_t,
                &raw mut ty as *mut uECC_word_t,
                &raw mut z as *mut uECC_word_t,
                curve,
            );
            uECC_vli_modSub(
                &raw mut tz as *mut uECC_word_t,
                &raw mut rx as *mut uECC_word_t,
                &raw mut tx as *mut uECC_word_t,
                &raw const (*curve).p as *const uECC_word_t,
                num_words,
            );
            XYcZ_add(
                &raw mut tx as *mut uECC_word_t,
                &raw mut ty as *mut uECC_word_t,
                &raw mut rx as *mut uECC_word_t,
                &raw mut ry as *mut uECC_word_t,
                curve,
            );
            uECC_vli_modMult_fast(
                &raw mut z as *mut uECC_word_t,
                &raw mut z as *mut uECC_word_t,
                &raw mut tz as *mut uECC_word_t,
                curve,
            );
        }
        i -= 1;
    }
    uECC_vli_modInv(
        &raw mut z as *mut uECC_word_t,
        &raw mut z as *mut uECC_word_t,
        &raw const (*curve).p as *const uECC_word_t,
        num_words,
    );
    apply_z(
        &raw mut rx as *mut uECC_word_t,
        &raw mut ry as *mut uECC_word_t,
        &raw mut z as *mut uECC_word_t,
        curve,
    );
    if uECC_vli_cmp_unsafe(
        &raw const (*curve).n as *const uECC_word_t,
        &raw mut rx as *mut uECC_word_t,
        num_n_words,
    ) as ::core::ffi::c_int
        != 1 as ::core::ffi::c_int
    {
        uECC_vli_sub(
            &raw mut rx as *mut uECC_word_t,
            &raw mut rx as *mut uECC_word_t,
            &raw const (*curve).n as *const uECC_word_t,
            num_n_words,
        );
    }
    return uECC_vli_equal(
        &raw mut rx as *mut uECC_word_t,
        &raw mut r as *mut uECC_word_t,
        num_words,
    ) as ::core::ffi::c_int;
}

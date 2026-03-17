use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn uname(__name: *mut utsname) -> ::core::ffi::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcpy(
        __dest: *mut ::core::ffi::c_char,
        __src: *const ::core::ffi::c_char,
    ) -> *mut ::core::ffi::c_char;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn ptls_aead_new(
        aead: *const ptls_aead_algorithm_t,
        hash: *const ptls_hash_algorithm_t,
        is_enc: ::core::ffi::c_int,
        secret: *const ::core::ffi::c_void,
        label_prefix: *const ::core::ffi::c_char,
    ) -> *mut ptls_aead_context_t;
    fn ptls_aead_free(ctx: *mut ptls_aead_context_t);
    static ptls_minicrypto_chacha20poly1305: ptls_aead_algorithm_t;
    static ptls_minicrypto_aes128gcm: ptls_aead_algorithm_t;
    static ptls_minicrypto_aes256gcm: ptls_aead_algorithm_t;
    static ptls_minicrypto_sha256: ptls_hash_algorithm_t;
    static ptls_minicrypto_sha384: ptls_hash_algorithm_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
    static ptls_openssl_aes128gcm: ptls_aead_algorithm_t;
    static ptls_openssl_aes256gcm: ptls_aead_algorithm_t;
    static ptls_openssl_chacha20poly1305: ptls_aead_algorithm_t;
    static ptls_fusion_aes256gcm: ptls_aead_algorithm_t;
    static ptls_fusion_aes128gcm: ptls_aead_algorithm_t;
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = usize;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct utsname {
    pub sysname: [::core::ffi::c_char; 65],
    pub nodename: [::core::ffi::c_char; 65],
    pub release: [::core::ffi::c_char; 65],
    pub version: [::core::ffi::c_char; 65],
    pub machine: [::core::ffi::c_char; 65],
    pub domainname: [::core::ffi::c_char; 65],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: ::core::ffi::c_int,
    pub _IO_read_ptr: *mut ::core::ffi::c_char,
    pub _IO_read_end: *mut ::core::ffi::c_char,
    pub _IO_read_base: *mut ::core::ffi::c_char,
    pub _IO_write_base: *mut ::core::ffi::c_char,
    pub _IO_write_ptr: *mut ::core::ffi::c_char,
    pub _IO_write_end: *mut ::core::ffi::c_char,
    pub _IO_buf_base: *mut ::core::ffi::c_char,
    pub _IO_buf_end: *mut ::core::ffi::c_char,
    pub _IO_save_base: *mut ::core::ffi::c_char,
    pub _IO_backup_base: *mut ::core::ffi::c_char,
    pub _IO_save_end: *mut ::core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::core::ffi::c_int,
    pub _flags2: ::core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::core::ffi::c_ushort,
    pub _vtable_offset: ::core::ffi::c_schar,
    pub _shortbuf: [::core::ffi::c_char; 1],
    pub _lock: *mut ::core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut ::core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: ::core::ffi::c_int,
    pub _unused2: [::core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type ptls_iovec_t = st_ptls_iovec_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_iovec_t {
    pub base: *mut uint8_t,
    pub len: size_t,
}
pub type ptls_hash_algorithm_t = st_ptls_hash_algorithm_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_hash_algorithm_t {
    pub name: *const ::core::ffi::c_char,
    pub block_size: size_t,
    pub digest_size: size_t,
    pub create: Option<unsafe extern "C" fn() -> *mut ptls_hash_context_t>,
    pub empty_digest: [uint8_t; 64],
}
pub type ptls_hash_context_t = st_ptls_hash_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_hash_context_t {
    pub update: Option<
        unsafe extern "C" fn(*mut st_ptls_hash_context_t, *const ::core::ffi::c_void, size_t) -> (),
    >,
    pub final_0: Option<
        unsafe extern "C" fn(
            *mut st_ptls_hash_context_t,
            *mut ::core::ffi::c_void,
            ptls_hash_final_mode_t,
        ) -> (),
    >,
    pub clone_:
        Option<unsafe extern "C" fn(*mut st_ptls_hash_context_t) -> *mut st_ptls_hash_context_t>,
}
pub type ptls_hash_final_mode_t = en_ptls_hash_final_mode_t;
pub type en_ptls_hash_final_mode_t = ::core::ffi::c_uint;
pub const PTLS_HASH_FINAL_MODE_SNAPSHOT: en_ptls_hash_final_mode_t = 2;
pub const PTLS_HASH_FINAL_MODE_RESET: en_ptls_hash_final_mode_t = 1;
pub const PTLS_HASH_FINAL_MODE_FREE: en_ptls_hash_final_mode_t = 0;
pub type ptls_aead_algorithm_t = st_ptls_aead_algorithm_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_ptls_aead_algorithm_t {
    pub name: *const ::core::ffi::c_char,
    pub confidentiality_limit: uint64_t,
    pub integrity_limit: uint64_t,
    pub ctr_cipher: *const ptls_cipher_algorithm_t,
    pub ecb_cipher: *const ptls_cipher_algorithm_t,
    pub key_size: size_t,
    pub iv_size: size_t,
    pub tag_size: size_t,
    pub tls12: C2Rust_Unnamed,
    #[bitfield(name = "non_temporal", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub non_temporal: [u8; 1],
    pub align_bits: uint8_t,
    pub context_size: size_t,
    pub setup_crypto: Option<
        unsafe extern "C" fn(
            *mut ptls_aead_context_t,
            ::core::ffi::c_int,
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_aead_context_t = st_ptls_aead_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_aead_context_t {
    pub algo: *const st_ptls_aead_algorithm_t,
    pub dispose_crypto: Option<unsafe extern "C" fn(*mut st_ptls_aead_context_t) -> ()>,
    pub do_get_iv:
        Option<unsafe extern "C" fn(*mut st_ptls_aead_context_t, *mut ::core::ffi::c_void) -> ()>,
    pub do_set_iv:
        Option<unsafe extern "C" fn(*mut st_ptls_aead_context_t, *const ::core::ffi::c_void) -> ()>,
    pub do_encrypt_init: Option<
        unsafe extern "C" fn(
            *mut st_ptls_aead_context_t,
            uint64_t,
            *const ::core::ffi::c_void,
            size_t,
        ) -> (),
    >,
    pub do_encrypt_update: Option<
        unsafe extern "C" fn(
            *mut st_ptls_aead_context_t,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            size_t,
        ) -> size_t,
    >,
    pub do_encrypt_final: Option<
        unsafe extern "C" fn(*mut st_ptls_aead_context_t, *mut ::core::ffi::c_void) -> size_t,
    >,
    pub do_encrypt: Option<
        unsafe extern "C" fn(
            *mut st_ptls_aead_context_t,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            size_t,
            uint64_t,
            *const ::core::ffi::c_void,
            size_t,
            *mut ptls_aead_supplementary_encryption_t,
        ) -> (),
    >,
    pub do_encrypt_v: Option<
        unsafe extern "C" fn(
            *mut st_ptls_aead_context_t,
            *mut ::core::ffi::c_void,
            *mut ptls_iovec_t,
            size_t,
            uint64_t,
            *const ::core::ffi::c_void,
            size_t,
        ) -> (),
    >,
    pub do_decrypt: Option<
        unsafe extern "C" fn(
            *mut st_ptls_aead_context_t,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            size_t,
            uint64_t,
            *const ::core::ffi::c_void,
            size_t,
        ) -> size_t,
    >,
}
pub type ptls_aead_supplementary_encryption_t = st_ptls_aead_supplementary_encryption_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_aead_supplementary_encryption_t {
    pub ctx: *mut ptls_cipher_context_t,
    pub input: *const ::core::ffi::c_void,
    pub output: [uint8_t; 16],
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub fixed_iv_size: size_t,
    pub record_iv_size: size_t,
}
pub type ptls_cipher_algorithm_t = st_ptls_cipher_algorithm_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_bench_entry_t {
    pub provider: *const ::core::ffi::c_char,
    pub algo_name: *const ::core::ffi::c_char,
    pub aead: *const ptls_aead_algorithm_t,
    pub hash: *const ptls_hash_algorithm_t,
    pub enabled_by_defaut: ::core::ffi::c_int,
}
pub type ptls_bench_entry_t = st_ptls_bench_entry_t;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const NULL_0: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const CLOCK_PROCESS_CPUTIME_ID: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const OPENSSL_VERSION_MAJOR: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const OPENSSL_VERSION_MINOR: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const OPENSSL_VERSION_PATCH: ::core::ffi::c_int = 13 as ::core::ffi::c_int;
pub const _OPENSSL_VERSION_PRE_RELEASE: ::core::ffi::c_long = 0 as ::core::ffi::c_long;
pub const OPENSSL_VERSION_NUMBER: ::core::ffi::c_long =
    (OPENSSL_VERSION_MAJOR << 28 as ::core::ffi::c_int
        | OPENSSL_VERSION_MINOR << 20 as ::core::ffi::c_int
        | OPENSSL_VERSION_PATCH << 4 as ::core::ffi::c_int) as ::core::ffi::c_long
        | _OPENSSL_VERSION_PRE_RELEASE;
pub const PTLS_MAX_DIGEST_SIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const PTLS_ERROR_CLASS_INTERNAL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const PTLS_ALERT_DECRYPT_ERROR: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const PTLS_ERROR_NO_MEMORY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 1 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn ptls_aead_encrypt(
    mut ctx: *mut ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) -> size_t {
    (*ctx).do_encrypt.expect("non-null function pointer")(
        ctx as *mut st_ptls_aead_context_t,
        output,
        input,
        inlen,
        seq,
        aad,
        aadlen,
        ::core::ptr::null_mut::<ptls_aead_supplementary_encryption_t>(),
    );
    return inlen.wrapping_add((*(*ctx).algo).tag_size);
}
#[inline]
unsafe extern "C" fn ptls_aead_decrypt(
    mut ctx: *mut ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) -> size_t {
    return (*ctx).do_decrypt.expect("non-null function pointer")(
        ctx as *mut st_ptls_aead_context_t,
        output,
        input,
        inlen,
        seq,
        aad,
        aadlen,
    );
}
pub const BENCH_MODE: [::core::ffi::c_char; 8] =
    unsafe { ::core::mem::transmute::<[u8; 8], [::core::ffi::c_char; 8]>(*b"release\0") };
unsafe extern "C" fn bench_time() -> uint64_t {
    let mut tv: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut cpu: timespec = timespec {
        tv_sec: 0,
        tv_nsec: 0,
    };
    if clock_gettime(CLOCK_PROCESS_CPUTIME_ID, &raw mut cpu) == 0 as ::core::ffi::c_int {
        let mut nanos: uint64_t = cpu.tv_nsec as uint64_t;
        let mut micros: uint64_t = nanos.wrapping_div(1000 as uint64_t);
        micros = (micros as ::core::ffi::c_ulonglong).wrapping_add(
            (1000000 as ::core::ffi::c_ulonglong)
                .wrapping_mul(cpu.tv_sec as uint64_t as ::core::ffi::c_ulonglong),
        ) as uint64_t as uint64_t;
        return micros;
    }
    gettimeofday(&raw mut tv, NULL_0);
    return (tv.tv_sec as uint64_t)
        .wrapping_mul(1000000 as uint64_t)
        .wrapping_add(tv.tv_usec as uint64_t);
}
pub const BENCH_BATCH: ::core::ffi::c_int = 1000 as ::core::ffi::c_int;
unsafe extern "C" fn bench_run_one(
    mut e: *mut ptls_aead_context_t,
    mut d: *mut ptls_aead_context_t,
    mut n: size_t,
    mut l: size_t,
    mut t_enc: *mut uint64_t,
    mut t_dec: *mut uint64_t,
    mut s: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut v_in: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut v_enc: [*mut uint8_t; 1000] = [::core::ptr::null_mut::<uint8_t>(); 1000];
    let mut v_dec: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut h: [uint64_t; 4] = [0; 4];
    *t_enc = 0 as uint64_t;
    *t_dec = 0 as uint64_t;
    *s = 0 as uint64_t;
    memset(
        &raw mut v_enc as *mut *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[*mut uint8_t; 1000]>() as size_t,
    );
    memset(
        &raw mut h as *mut uint64_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[uint64_t; 4]>() as size_t,
    );
    v_in = malloc(l) as *mut uint8_t;
    v_dec = malloc(l) as *mut uint8_t;
    if v_in.is_null() || v_dec.is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    }
    let mut i: size_t = 0 as size_t;
    while ret == 0 as ::core::ffi::c_int && i < BENCH_BATCH as size_t {
        v_enc[i as usize] = malloc(l.wrapping_add(PTLS_MAX_DIGEST_SIZE as size_t)) as *mut uint8_t;
        if v_enc[i as usize].is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        }
        i = i.wrapping_add(1);
    }
    if ret == 0 as ::core::ffi::c_int {
        memset(v_in as *mut ::core::ffi::c_void, 0 as ::core::ffi::c_int, l);
        let mut k: size_t = 0 as size_t;
        while k < n {
            let mut e_len: size_t = 0;
            let mut d_len: size_t = 0;
            let mut i_max: size_t = if n.wrapping_sub(k) > BENCH_BATCH as size_t {
                BENCH_BATCH as size_t
            } else {
                n.wrapping_sub(k)
            };
            let mut old_h: uint64_t = h[0 as ::core::ffi::c_int as usize];
            let mut t_start: uint64_t = bench_time();
            let mut t_medium: uint64_t = 0;
            let mut t_end: uint64_t = 0;
            let mut i_0: size_t = 0 as size_t;
            while i_0 < i_max {
                h[0 as ::core::ffi::c_int as usize] =
                    h[0 as ::core::ffi::c_int as usize].wrapping_add(1);
                e_len = ptls_aead_encrypt(
                    e,
                    v_enc[i_0 as usize] as *mut ::core::ffi::c_void,
                    v_in as *const ::core::ffi::c_void,
                    l,
                    h[0 as ::core::ffi::c_int as usize],
                    &raw mut h as *mut uint64_t as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[uint64_t; 4]>() as size_t,
                );
                *s = (*s).wrapping_add(*v_enc[i_0 as usize].offset(l as isize) as uint64_t);
                i_0 = i_0.wrapping_add(1);
            }
            t_medium = bench_time();
            h[0 as ::core::ffi::c_int as usize] = old_h;
            let mut i_1: size_t = 0 as size_t;
            while i_1 < i_max {
                h[0 as ::core::ffi::c_int as usize] =
                    h[0 as ::core::ffi::c_int as usize].wrapping_add(1);
                d_len = ptls_aead_decrypt(
                    d,
                    v_dec as *mut ::core::ffi::c_void,
                    v_enc[i_1 as usize] as *const ::core::ffi::c_void,
                    e_len,
                    h[0 as ::core::ffi::c_int as usize],
                    &raw mut h as *mut uint64_t as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[uint64_t; 4]>() as size_t,
                );
                if d_len != l {
                    ret = PTLS_ALERT_DECRYPT_ERROR;
                    break;
                } else {
                    *s = (*s)
                        .wrapping_add(*v_dec.offset(0 as ::core::ffi::c_int as isize) as uint64_t);
                    i_1 = i_1.wrapping_add(1);
                }
            }
            t_end = bench_time();
            *t_enc = (*t_enc).wrapping_add(t_medium.wrapping_sub(t_start));
            *t_dec = (*t_dec).wrapping_add(t_end.wrapping_sub(t_medium));
            k = k.wrapping_add(i_max);
        }
    }
    if !v_in.is_null() {
        free(v_in as *mut ::core::ffi::c_void);
    }
    let mut i_2: size_t = 0 as size_t;
    while i_2 < BENCH_BATCH as size_t {
        if !v_enc[i_2 as usize].is_null() {
            free(v_enc[i_2 as usize] as *mut ::core::ffi::c_void);
        }
        i_2 = i_2.wrapping_add(1);
    }
    if !v_dec.is_null() {
        free(v_dec as *mut ::core::ffi::c_void);
    }
    return ret;
}
unsafe extern "C" fn bench_mbps(
    mut t: uint64_t,
    mut l: size_t,
    mut n: size_t,
) -> ::core::ffi::c_double {
    let mut x: ::core::ffi::c_double = l as ::core::ffi::c_double;
    x *= n as ::core::ffi::c_double;
    x *= 8 as ::core::ffi::c_int as ::core::ffi::c_double;
    x /= t as ::core::ffi::c_double;
    return x;
}
unsafe extern "C" fn bench_run_aead(
    mut OS: *mut ::core::ffi::c_char,
    mut HW: *mut ::core::ffi::c_char,
    mut basic_ref: ::core::ffi::c_int,
    mut s0: uint64_t,
    mut provider: *const ::core::ffi::c_char,
    mut algo_name: *const ::core::ffi::c_char,
    mut aead: *const ptls_aead_algorithm_t,
    mut hash: *const ptls_hash_algorithm_t,
    mut n: size_t,
    mut l: size_t,
    mut s: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut secret: [uint8_t; 32] = [0; 32];
    let mut e: *mut ptls_aead_context_t = ::core::ptr::null_mut::<ptls_aead_context_t>();
    let mut d: *mut ptls_aead_context_t = ::core::ptr::null_mut::<ptls_aead_context_t>();
    let mut t_e: uint64_t = 0 as uint64_t;
    let mut t_d: uint64_t = 0 as uint64_t;
    let mut p_version: [::core::ffi::c_char; 128] = [0; 128];
    p_version[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    if strcmp(
        provider,
        b"openssl\0".as_ptr() as *const ::core::ffi::c_char,
    ) == 0 as ::core::ffi::c_int
    {
        let mut combined: uint32_t = OPENSSL_VERSION_NUMBER as uint32_t;
        let mut M: ::core::ffi::c_int =
            (combined >> 28 as ::core::ffi::c_int) as ::core::ffi::c_int;
        let mut NN: ::core::ffi::c_int =
            (combined >> 20 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int;
        let mut FF: ::core::ffi::c_int =
            (combined >> 12 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int;
        let mut PP: ::core::ffi::c_int =
            (combined >> 4 as ::core::ffi::c_int & 0xff as uint32_t) as ::core::ffi::c_int;
        let mut letter: ::core::ffi::c_char =
            ('a' as i32 - 1 as ::core::ffi::c_int + PP) as ::core::ffi::c_char;
        sprintf(
            &raw mut p_version as *mut ::core::ffi::c_char,
            b"%d.%d.%d%c\0".as_ptr() as *const ::core::ffi::c_char,
            M,
            NN,
            FF,
            letter as ::core::ffi::c_int,
        );
    }
    *s = (*s).wrapping_add(s0);
    memset(
        &raw mut secret as *mut uint8_t as *mut ::core::ffi::c_void,
        'z' as i32,
        ::core::mem::size_of::<[uint8_t; 32]>() as size_t,
    );
    e = ptls_aead_new(
        aead,
        hash,
        1 as ::core::ffi::c_int,
        &raw mut secret as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    d = ptls_aead_new(
        aead,
        hash,
        0 as ::core::ffi::c_int,
        &raw mut secret as *mut uint8_t as *const ::core::ffi::c_void,
        ::core::ptr::null::<::core::ffi::c_char>(),
    );
    if e.is_null() || d.is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else {
        ret = bench_run_one(e, d, n, l, &raw mut t_e, &raw mut t_d, s);
        if ret == 0 as ::core::ffi::c_int {
            printf(
                b"%s, %s, %d, %s, %d, %s, %s, %s, %d, %d, %d, %d, %.2f, %.2f\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                OS,
                HW,
                (8 as usize).wrapping_mul(::core::mem::size_of::<size_t>() as usize)
                    as ::core::ffi::c_int,
                BENCH_MODE.as_ptr(),
                basic_ref,
                provider,
                &raw mut p_version as *mut ::core::ffi::c_char,
                algo_name,
                n as ::core::ffi::c_int,
                l as ::core::ffi::c_int,
                t_e as ::core::ffi::c_int,
                t_d as ::core::ffi::c_int,
                bench_mbps(t_e, l, n),
                bench_mbps(t_d, l, n),
            );
        }
    }
    if !e.is_null() {
        ptls_aead_free(e);
    }
    if !d.is_null() {
        ptls_aead_free(d);
    }
    return ret;
}
static mut aead_list: [ptls_bench_entry_t; 8] = unsafe {
    [
        st_ptls_bench_entry_t {
            provider: b"minicrypto\0".as_ptr() as *const ::core::ffi::c_char,
            algo_name: b"aes128gcm\0".as_ptr() as *const ::core::ffi::c_char,
            aead: &raw const ptls_minicrypto_aes128gcm,
            hash: &raw const ptls_minicrypto_sha256,
            enabled_by_defaut: 0 as ::core::ffi::c_int,
        },
        st_ptls_bench_entry_t {
            provider: b"minicrypto\0".as_ptr() as *const ::core::ffi::c_char,
            algo_name: b"aes256gcm\0".as_ptr() as *const ::core::ffi::c_char,
            aead: &raw const ptls_minicrypto_aes256gcm,
            hash: &raw const ptls_minicrypto_sha384,
            enabled_by_defaut: 0 as ::core::ffi::c_int,
        },
        st_ptls_bench_entry_t {
            provider: b"minicrypto\0".as_ptr() as *const ::core::ffi::c_char,
            algo_name: b"chacha20poly1305\0".as_ptr() as *const ::core::ffi::c_char,
            aead: &raw const ptls_minicrypto_chacha20poly1305,
            hash: &raw const ptls_minicrypto_sha256,
            enabled_by_defaut: 1 as ::core::ffi::c_int,
        },
        st_ptls_bench_entry_t {
            provider: b"fusion\0".as_ptr() as *const ::core::ffi::c_char,
            algo_name: b"aes128gcm\0".as_ptr() as *const ::core::ffi::c_char,
            aead: &raw const ptls_fusion_aes128gcm,
            hash: &raw const ptls_minicrypto_sha256,
            enabled_by_defaut: 1 as ::core::ffi::c_int,
        },
        st_ptls_bench_entry_t {
            provider: b"fusion\0".as_ptr() as *const ::core::ffi::c_char,
            algo_name: b"aes256gcm\0".as_ptr() as *const ::core::ffi::c_char,
            aead: &raw const ptls_fusion_aes256gcm,
            hash: &raw const ptls_minicrypto_sha384,
            enabled_by_defaut: 1 as ::core::ffi::c_int,
        },
        st_ptls_bench_entry_t {
            provider: b"openssl\0".as_ptr() as *const ::core::ffi::c_char,
            algo_name: b"chacha20poly1305\0".as_ptr() as *const ::core::ffi::c_char,
            aead: &raw const ptls_openssl_chacha20poly1305,
            hash: &raw const ptls_minicrypto_sha256,
            enabled_by_defaut: 1 as ::core::ffi::c_int,
        },
        st_ptls_bench_entry_t {
            provider: b"openssl\0".as_ptr() as *const ::core::ffi::c_char,
            algo_name: b"aes128gcm\0".as_ptr() as *const ::core::ffi::c_char,
            aead: &raw const ptls_openssl_aes128gcm,
            hash: &raw const ptls_minicrypto_sha256,
            enabled_by_defaut: 1 as ::core::ffi::c_int,
        },
        st_ptls_bench_entry_t {
            provider: b"openssl\0".as_ptr() as *const ::core::ffi::c_char,
            algo_name: b"aes256gcm\0".as_ptr() as *const ::core::ffi::c_char,
            aead: &raw const ptls_openssl_aes256gcm,
            hash: &raw const ptls_minicrypto_sha384,
            enabled_by_defaut: 1 as ::core::ffi::c_int,
        },
    ]
};
static mut nb_aead_list: size_t = 0;
unsafe extern "C" fn bench_basic(mut x: *mut uint64_t) -> ::core::ffi::c_int {
    let mut t_start: uint64_t = bench_time();
    let mut a: uint32_t = (*x & 0xffffffff as uint64_t) as uint32_t;
    let mut b: uint32_t = (*x >> 32 as ::core::ffi::c_int) as uint32_t;
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < 10000000 as ::core::ffi::c_int as ::core::ffi::c_uint {
        let mut v: uint32_t = a >> 3 as ::core::ffi::c_int | a << 29 as ::core::ffi::c_int;
        v = v.wrapping_add(a);
        v ^= b;
        b = a;
        a = v;
        i = i.wrapping_add(1);
    }
    *x = (b as uint64_t) << 32 as ::core::ffi::c_int | a as uint64_t;
    return bench_time().wrapping_sub(t_start) as ::core::ffi::c_int;
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut force_all_tests: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut x: uint64_t = 0xdeadbeef as uint64_t;
    let mut s: uint64_t = 0 as uint64_t;
    let mut basic_ref: ::core::ffi::c_int = bench_basic(&raw mut x);
    let mut OS: [::core::ffi::c_char; 128] = [0; 128];
    let mut HW: [::core::ffi::c_char; 128] = [0; 128];
    let mut uts: utsname = utsname {
        sysname: [0; 65],
        nodename: [0; 65],
        release: [0; 65],
        version: [0; 65],
        machine: [0; 65],
        domainname: [0; 65],
    };
    OS[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    HW[0 as ::core::ffi::c_int as usize] = 0 as ::core::ffi::c_char;
    if uname(&raw mut uts) == 0 as ::core::ffi::c_int {
        if strlen(&raw mut uts.sysname as *mut ::core::ffi::c_char).wrapping_add(1 as size_t)
            < ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize
        {
            strcpy(
                &raw mut OS as *mut ::core::ffi::c_char,
                &raw mut uts.sysname as *mut ::core::ffi::c_char,
            );
        }
        if strlen(&raw mut uts.machine as *mut ::core::ffi::c_char).wrapping_add(1 as size_t)
            < ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as usize
        {
            strcpy(
                &raw mut HW as *mut ::core::ffi::c_char,
                &raw mut uts.machine as *mut ::core::ffi::c_char,
            );
        }
    }
    if argc == 2 as ::core::ffi::c_int
        && strcmp(
            *argv.offset(1 as ::core::ffi::c_int as isize),
            b"-f\0".as_ptr() as *const ::core::ffi::c_char,
        ) == 0 as ::core::ffi::c_int
    {
        force_all_tests = 1 as ::core::ffi::c_int;
    } else if argc > 1 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"Usage: %s [-f]\n   Use option \"-f\" to force execution of the slower tests.\n\0"
                .as_ptr() as *const ::core::ffi::c_char,
            *argv.offset(0 as ::core::ffi::c_int as isize),
        );
        exit(-(1 as ::core::ffi::c_int));
    }
    printf(
        b"OS, HW, bits, mode, 10M ops, provider, version, algorithm, N, L, encrypt us, decrypt us, encrypt mbps, decrypt mbps,\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
    );
    let mut i: size_t = 0 as size_t;
    while ret == 0 as ::core::ffi::c_int && i < nb_aead_list {
        if aead_list[i as usize].enabled_by_defaut != 0 || force_all_tests != 0 {
            ret = bench_run_aead(
                &raw mut OS as *mut ::core::ffi::c_char,
                &raw mut HW as *mut ::core::ffi::c_char,
                basic_ref,
                x,
                aead_list[i as usize].provider,
                aead_list[i as usize].algo_name,
                aead_list[i as usize].aead,
                aead_list[i as usize].hash,
                1000 as size_t,
                1500 as size_t,
                &raw mut s,
            );
        }
        i = i.wrapping_add(1);
    }
    if s == 0 as uint64_t {
        printf(
            b"Unexpected value of test sum s = %llx\n\0".as_ptr() as *const ::core::ffi::c_char,
            s as ::core::ffi::c_ulonglong,
        );
    }
    return ret;
}
pub fn main() {
    let mut args_strings: Vec<Vec<u8>> = ::std::env::args()
        .map(|arg| {
            ::std::ffi::CString::new(arg)
                .expect("Failed to convert argument into CString.")
                .into_bytes_with_nul()
        })
        .collect();
    let mut args_ptrs: Vec<*mut ::core::ffi::c_char> = args_strings
        .iter_mut()
        .map(|arg| arg.as_mut_ptr() as *mut ::core::ffi::c_char)
        .chain(::core::iter::once(::core::ptr::null_mut()))
        .collect();
    unsafe {
        ::std::process::exit(main_0(
            (args_ptrs.len() - 1) as ::core::ffi::c_int,
            args_ptrs.as_mut_ptr() as *mut *mut ::core::ffi::c_char,
        ) as i32)
    }
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    nb_aead_list = (::core::mem::size_of::<[ptls_bench_entry_t; 8]>() as size_t)
        .wrapping_div(::core::mem::size_of::<ptls_bench_entry_t>() as size_t);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];

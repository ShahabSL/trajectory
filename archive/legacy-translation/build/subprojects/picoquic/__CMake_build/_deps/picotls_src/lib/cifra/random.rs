extern "C" {
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn open(
        __file: *const ::core::ffi::c_char,
        __oflag: ::core::ffi::c_int,
        ...
    ) -> ::core::ffi::c_int;
    fn perror(__s: *const ::core::ffi::c_char);
    fn abort() -> !;
    fn close(__fd: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn read(__fd: ::core::ffi::c_int, __buf: *mut ::core::ffi::c_void, __nbytes: size_t)
        -> ssize_t;
    fn cf_hash_drbg_sha256_init(
        ctx: *mut cf_hash_drbg_sha256,
        entropy: *const ::core::ffi::c_void,
        nentropy: size_t,
        nonce: *const ::core::ffi::c_void,
        nnonce: size_t,
        persn: *const ::core::ffi::c_void,
        npersn: size_t,
    );
    fn cf_hash_drbg_sha256_needs_reseed(ctx: *const cf_hash_drbg_sha256) -> uint32_t;
    fn cf_hash_drbg_sha256_gen(
        ctx: *mut cf_hash_drbg_sha256,
        out: *mut ::core::ffi::c_void,
        nout: size_t,
    );
}
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __ssize_t = ::core::ffi::c_long;
pub type size_t = usize;
pub type ssize_t = __ssize_t;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cf_hash_drbg_sha256 {
    pub V: [uint8_t; 55],
    pub C: [uint8_t; 55],
    pub reseed_counter: uint32_t,
}
pub const EINTR: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const O_RDONLY: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const __O_CLOEXEC: ::core::ffi::c_int = 0o2000000 as ::core::ffi::c_int;
pub const O_CLOEXEC: ::core::ffi::c_int = __O_CLOEXEC;
unsafe extern "C" fn read_entropy(mut entropy: *mut uint8_t, mut size: size_t) {
    let mut fd: ::core::ffi::c_int = 0;
    fd = open(
        b"/dev/urandom\0".as_ptr() as *const ::core::ffi::c_char,
        O_RDONLY | O_CLOEXEC,
    );
    if fd == -(1 as ::core::ffi::c_int) {
        fd = open(
            b"/dev/random\0".as_ptr() as *const ::core::ffi::c_char,
            O_RDONLY | O_CLOEXEC,
        );
        if fd == -(1 as ::core::ffi::c_int) {
            perror(
                b"ptls_minicrypto_random_bytes: could not open neither /dev/random or /dev/urandom\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
            abort();
        }
    }
    while size != 0 as size_t {
        let mut rret: ssize_t = 0;
        loop {
            rret = read(fd, entropy as *mut ::core::ffi::c_void, size);
            if !(rret == -(1 as ::core::ffi::c_int) as ssize_t && *__errno_location() == EINTR) {
                break;
            }
        }
        if rret < 0 as ssize_t {
            perror(b"ptls_minicrypto_random_bytes\0".as_ptr() as *const ::core::ffi::c_char);
            abort();
        }
        entropy = entropy.offset(rret as isize);
        size = size.wrapping_sub(rret as size_t);
    }
    close(fd);
}
#[no_mangle]
pub unsafe extern "C" fn ptls_minicrypto_random_bytes(
    mut buf: *mut ::core::ffi::c_void,
    mut len: size_t,
) {
    #[thread_local]
    static mut ctx: cf_hash_drbg_sha256 = cf_hash_drbg_sha256 {
        V: [0; 55],
        C: [0; 55],
        reseed_counter: 0,
    };
    if cf_hash_drbg_sha256_needs_reseed(&raw mut ctx) != 0 {
        let mut entropy: [uint8_t; 256] = [0; 256];
        read_entropy(
            &raw mut entropy as *mut uint8_t,
            ::core::mem::size_of::<[uint8_t; 256]>() as size_t,
        );
        cf_hash_drbg_sha256_init(
            &raw mut ctx,
            &raw mut entropy as *mut uint8_t as *const ::core::ffi::c_void,
            (::core::mem::size_of::<[uint8_t; 256]>() as size_t).wrapping_div(2 as size_t),
            (&raw mut entropy as *mut uint8_t).offset(
                (::core::mem::size_of::<[uint8_t; 256]>() as usize).wrapping_div(2 as usize)
                    as isize,
            ) as *const ::core::ffi::c_void,
            (::core::mem::size_of::<[uint8_t; 256]>() as size_t).wrapping_div(2 as size_t),
            b"ptls\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            4 as size_t,
        );
    }
    cf_hash_drbg_sha256_gen(&raw mut ctx, buf, len);
}

extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn clock_gettime(__clock_id: clockid_t, __tp: *mut timespec) -> ::core::ffi::c_int;
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<
            unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void,
        >,
        __arg: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_join(
        __th: pthread_t,
        __thread_return: *mut *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> ::core::ffi::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> ::core::ffi::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn pthread_cond_broadcast(__cond: *mut pthread_cond_t) -> ::core::ffi::c_int;
    fn pthread_cond_wait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
    ) -> ::core::ffi::c_int;
    fn pthread_cond_timedwait(
        __cond: *mut pthread_cond_t,
        __mutex: *mut pthread_mutex_t,
        __abstime: *const timespec,
    ) -> ::core::ffi::c_int;
    fn pthread_kill(__threadid: pthread_t, __signo: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn unlink(__name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn picohash_bytes(key: *const uint8_t, length: uint32_t) -> uint64_t;
    fn inet_pton(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_char,
        __buf: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn inet_ntop(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_void,
        __buf: *mut ::core::ffi::c_char,
        __len: socklen_t,
    ) -> *const ::core::ffi::c_char;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn fopen(
        __filename: *const ::core::ffi::c_char,
        __modes: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn vsnprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn exit(__status: ::core::ffi::c_int) -> !;
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
    fn memcmp(
        __s1: *const ::core::ffi::c_void,
        __s2: *const ::core::ffi::c_void,
        __n: size_t,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn strnlen(__string: *const ::core::ffi::c_char, __maxlen: size_t) -> size_t;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: ::core::ffi::c_uint,
    pub fp_offset: ::core::ffi::c_uint,
    pub overflow_arg_area: *mut ::core::ffi::c_void,
    pub reg_save_area: *mut ::core::ffi::c_void,
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __clockid_t = ::core::ffi::c_int;
pub type __syscall_slong_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
pub type clockid_t = __clockid_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __atomic_wide_counter {
    pub __value64: ::core::ffi::c_ulonglong,
    pub __value32: C2Rust_Unnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed {
    pub __low: ::core::ffi::c_uint,
    pub __high: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: ::core::ffi::c_int,
    pub __count: ::core::ffi::c_uint,
    pub __owner: ::core::ffi::c_int,
    pub __nusers: ::core::ffi::c_uint,
    pub __kind: ::core::ffi::c_int,
    pub __spins: ::core::ffi::c_short,
    pub __elision: ::core::ffi::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [::core::ffi::c_uint; 2],
    pub __g_size: [::core::ffi::c_uint; 2],
    pub __g1_orig_size: ::core::ffi::c_uint,
    pub __wrefs: ::core::ffi::c_uint,
    pub __g_signals: [::core::ffi::c_uint; 2],
}
pub type pthread_t = ::core::ffi::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [::core::ffi::c_char; 4],
    pub __align: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [::core::ffi::c_char; 56],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::core::ffi::c_char; 40],
    pub __align: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [::core::ffi::c_char; 48],
    pub __align: ::core::ffi::c_longlong,
}
pub type socklen_t = __socklen_t;
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::core::ffi::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [::core::ffi::c_char; 118],
    pub __ss_align: ::core::ffi::c_ulong,
}
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::core::ffi::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
pub type va_list = __builtin_va_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_connection_id_t {
    pub id: [uint8_t; 20],
    pub id_len: uint8_t,
}
pub type picoquic_connection_id_t = st_picoquic_connection_id_t;
pub type FILE = _IO_FILE;
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
pub type picoquic_thread_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_event_t {
    pub mutex: pthread_mutex_t,
    pub cond: pthread_cond_t,
}
pub type picoquic_event_t = st_picoquic_event_t;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
pub const CLOCK_REALTIME: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const SIGTERM: ::core::ffi::c_int = 15 as ::core::ffi::c_int;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const PICOQUIC_CONNECTION_ID_MAX_SIZE: ::core::ffi::c_int = 20 as ::core::ffi::c_int;
pub const PICOQUIC_DEFAULT_SOLUTION_DIR: [::core::ffi::c_char; 3] =
    unsafe { ::core::mem::transmute::<[u8; 3], [::core::ffi::c_char; 3]>(*b"./\0") };
pub const PICOQUIC_FILE_SEPARATOR: [::core::ffi::c_char; 2] =
    unsafe { ::core::mem::transmute::<[u8; 2], [::core::ffi::c_char; 2]>(*b"/\0") };
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn picoquic_string_create(
    mut original: *const ::core::ffi::c_char,
    mut len: size_t,
) -> *mut ::core::ffi::c_char {
    let mut allocated: size_t = len.wrapping_add(1 as size_t);
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if allocated > 0 as size_t {
        str = malloc(allocated) as *mut ::core::ffi::c_char;
        if !str.is_null() {
            if original.is_null() || len == 0 as size_t {
                *str.offset(0 as ::core::ffi::c_int as isize) = 0 as ::core::ffi::c_char;
            } else if allocated > len {
                memcpy(
                    str as *mut ::core::ffi::c_void,
                    original as *const ::core::ffi::c_void,
                    len,
                );
                *str.offset(allocated.wrapping_sub(1 as size_t) as isize) =
                    0 as ::core::ffi::c_char;
            } else {
                free(str as *mut ::core::ffi::c_void);
                str = ::core::ptr::null_mut::<::core::ffi::c_char>();
            }
        }
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_string_duplicate(
    mut original: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut str: *mut ::core::ffi::c_char = ::core::ptr::null_mut::<::core::ffi::c_char>();
    if !original.is_null() {
        let mut len: size_t = strlen(original);
        str = picoquic_string_create(original, len);
    }
    return str;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_string_free(
    mut str: *mut ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    if !str.is_null() {
        free(str as *mut ::core::ffi::c_void);
    }
    return ::core::ptr::null_mut::<::core::ffi::c_char>();
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_strip_endofline(
    mut buf: *mut ::core::ffi::c_char,
    mut bufmax: size_t,
    mut line: *const ::core::ffi::c_char,
) -> *mut ::core::ffi::c_char {
    let mut i: size_t = 0 as size_t;
    while i < bufmax {
        let mut c: ::core::ffi::c_int = *line.offset(i as isize) as ::core::ffi::c_int;
        if c == 0 as ::core::ffi::c_int || c == '\r' as i32 || c == '\n' as i32 {
            *buf.offset(i as isize) = 0 as ::core::ffi::c_char;
            break;
        } else {
            *buf.offset(i as isize) = c as ::core::ffi::c_char;
            i = i.wrapping_add(1);
        }
    }
    *buf.offset(bufmax.wrapping_sub(1 as size_t) as isize) = 0 as ::core::ffi::c_char;
    return buf;
}
static mut debug_out: *mut FILE = ::core::ptr::null::<FILE>() as *mut FILE;
#[no_mangle]
pub static mut debug_callback: Option<
    unsafe extern "C" fn(*const ::core::ffi::c_char, *mut ::core::ffi::c_void) -> (),
> = None;
#[no_mangle]
pub static mut debug_callback_argp: *mut ::core::ffi::c_void = NULL;
static mut debug_suspended: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn debug_set_stream(mut F: *mut FILE) {
    debug_out = F;
    debug_callback = None;
    debug_callback_argp = NULL;
}
#[no_mangle]
pub unsafe extern "C" fn debug_set_callback(
    mut cb: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_char, *mut ::core::ffi::c_void) -> (),
    >,
    mut argp: *mut ::core::ffi::c_void,
) {
    debug_callback = cb;
    debug_callback_argp = argp;
    debug_out = ::core::ptr::null_mut::<FILE>();
}
#[no_mangle]
pub unsafe extern "C" fn debug_printf(mut fmt: *const ::core::ffi::c_char, mut c2rust_args: ...) {
    if debug_suspended == 0 as ::core::ffi::c_int
        && (!debug_out.is_null() || debug_callback.is_some())
    {
        if !debug_out.is_null() {
            let mut args: ::core::ffi::VaListImpl;
            args = c2rust_args.clone();
            vfprintf(debug_out, fmt, args.as_va_list());
        } else {
            let mut message: [::core::ffi::c_char; 1024] = [0; 1024];
            let mut message_length: size_t = 0;
            let mut args_0: ::core::ffi::VaListImpl;
            args_0 = c2rust_args.clone();
            vsnprintf(
                &raw mut message as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
                fmt,
                args_0.as_va_list(),
            );
            message_length = strnlen(
                &raw mut message as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 1024]>() as size_t,
            );
            if message_length > 0 as size_t {
                if message[message_length.wrapping_sub(1 as size_t) as usize] as ::core::ffi::c_int
                    == '\n' as i32
                {
                    message[message_length.wrapping_sub(1 as size_t) as usize] =
                        '\0' as i32 as ::core::ffi::c_char;
                }
            }
            debug_callback.expect("non-null function pointer")(
                &raw mut message as *mut ::core::ffi::c_char,
                debug_callback_argp,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn debug_dump(
    mut x: *const ::core::ffi::c_void,
    mut len: ::core::ffi::c_int,
) {
    if debug_suspended == 0 as ::core::ffi::c_int
        && (!debug_out.is_null() || debug_callback.is_some())
    {
        let mut msg: [::core::ffi::c_char; 64] = [0; 64];
        let mut mlen: size_t = 0;
        let mut bytes: *mut uint8_t = x as *mut uint8_t;
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < len {
            snprintf(
                &raw mut msg as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
                b"%04x:  \0".as_ptr() as *const ::core::ffi::c_char,
                i,
            );
            mlen = strnlen(
                &raw mut msg as *mut ::core::ffi::c_char,
                ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
            );
            let mut j: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            while j < 16 as ::core::ffi::c_int && i < len {
                snprintf(
                    (&raw mut msg as *mut ::core::ffi::c_char).offset(mlen as isize),
                    (::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t)
                        .wrapping_sub(mlen),
                    b"%02x \0".as_ptr() as *const ::core::ffi::c_char,
                    *bytes.offset(i as isize) as ::core::ffi::c_int,
                );
                mlen = strnlen(
                    &raw mut msg as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 64]>() as size_t,
                );
                j += 1;
                i += 1;
            }
            if !debug_out.is_null() {
                fprintf(
                    debug_out,
                    b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut msg as *mut ::core::ffi::c_char,
                );
            } else {
                debug_callback.expect("non-null function pointer")(
                    &raw mut msg as *mut ::core::ffi::c_char,
                    debug_callback_argp,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn debug_printf_push_stream(mut f: *mut FILE) {
    if !debug_out.is_null() {
        fprintf(
            stderr,
            b"Nested err out not supported\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    debug_out = f;
}
#[no_mangle]
pub unsafe extern "C" fn debug_printf_pop_stream() {
    if debug_out.is_null() {
        fprintf(
            stderr,
            b"No current err out\n\0".as_ptr() as *const ::core::ffi::c_char,
        );
        exit(1 as ::core::ffi::c_int);
    }
    debug_out = ::core::ptr::null_mut::<FILE>();
}
#[no_mangle]
pub unsafe extern "C" fn debug_printf_suspend() {
    debug_suspended = 1 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn debug_printf_resume() {
    debug_suspended = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn debug_printf_reset(
    mut suspended: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = debug_suspended;
    debug_suspended = suspended;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_sprintf(
    mut buf: *mut ::core::ffi::c_char,
    mut buf_len: size_t,
    mut nb_chars: *mut size_t,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) -> ::core::ffi::c_int {
    let mut args: ::core::ffi::VaListImpl;
    args = c2rust_args.clone();
    let mut res: ::core::ffi::c_int = vsnprintf(buf, buf_len, fmt, args.as_va_list());
    if !nb_chars.is_null() {
        *nb_chars = res as size_t;
    }
    return if res >= 0 as ::core::ffi::c_int {
        (res as size_t >= buf_len) as ::core::ffi::c_int
    } else {
        res
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_print_connection_id_hexa(
    mut buf: *mut ::core::ffi::c_char,
    mut buf_len: size_t,
    mut cnxid: *const picoquic_connection_id_t,
) -> ::core::ffi::c_int {
    static mut hex_to_char: [::core::ffi::c_char; 16] = [
        '0' as i32 as ::core::ffi::c_char,
        '1' as i32 as ::core::ffi::c_char,
        '2' as i32 as ::core::ffi::c_char,
        '3' as i32 as ::core::ffi::c_char,
        '4' as i32 as ::core::ffi::c_char,
        '5' as i32 as ::core::ffi::c_char,
        '6' as i32 as ::core::ffi::c_char,
        '7' as i32 as ::core::ffi::c_char,
        '8' as i32 as ::core::ffi::c_char,
        '9' as i32 as ::core::ffi::c_char,
        'a' as i32 as ::core::ffi::c_char,
        'b' as i32 as ::core::ffi::c_char,
        'c' as i32 as ::core::ffi::c_char,
        'd' as i32 as ::core::ffi::c_char,
        'e' as i32 as ::core::ffi::c_char,
        'f' as i32 as ::core::ffi::c_char,
    ];
    if buf_len
        < ((*cnxid).id_len as size_t)
            .wrapping_mul(2 as size_t)
            .wrapping_add(1 as size_t)
    {
        return -(1 as ::core::ffi::c_int);
    }
    let mut i: ::core::ffi::c_uint = 0 as ::core::ffi::c_uint;
    while i < (*cnxid).id_len as ::core::ffi::c_uint {
        *buf.offset(i.wrapping_mul(2 as ::core::ffi::c_uint) as isize) = hex_to_char
            [((*cnxid).id[i as usize] as ::core::ffi::c_int >> 4 as ::core::ffi::c_int) as usize];
        *buf.offset(
            i.wrapping_mul(2 as ::core::ffi::c_uint)
                .wrapping_add(1 as ::core::ffi::c_uint) as isize,
        ) = hex_to_char
            [((*cnxid).id[i as usize] as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as usize];
        i = i.wrapping_add(1);
    }
    *buf.offset(
        ((*cnxid).id_len as ::core::ffi::c_uint).wrapping_mul(2 as ::core::ffi::c_uint) as isize,
    ) = 0 as ::core::ffi::c_char;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_hexa_digit(
    mut x: ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if x as ::core::ffi::c_int >= '0' as i32 && x as ::core::ffi::c_int <= '9' as i32 {
        ret = x as ::core::ffi::c_int - '0' as i32;
    } else if x as ::core::ffi::c_int >= 'A' as i32 && x as ::core::ffi::c_int <= 'F' as i32 {
        ret = x as ::core::ffi::c_int - 'A' as i32 + 10 as ::core::ffi::c_int;
    } else if x as ::core::ffi::c_int >= 'a' as i32 && x as ::core::ffi::c_int <= 'f' as i32 {
        ret = x as ::core::ffi::c_int - 'a' as i32 + 10 as ::core::ffi::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_hexa(
    mut hex_input: *const ::core::ffi::c_char,
    mut input_length: size_t,
    mut bin_output: *mut uint8_t,
    mut output_max: size_t,
) -> size_t {
    let mut ret: size_t = 0 as size_t;
    if input_length > 0 as size_t
        && input_length & 1 as size_t == 0 as size_t
        && (2 as size_t).wrapping_mul(output_max) >= input_length
    {
        let mut offset: size_t = 0 as size_t;
        while offset < input_length {
            let c2rust_fresh0 = offset;
            offset = offset.wrapping_add(1);
            let mut a: ::core::ffi::c_int =
                picoquic_parse_hexa_digit(*hex_input.offset(c2rust_fresh0 as isize));
            let c2rust_fresh1 = offset;
            offset = offset.wrapping_add(1);
            let mut b: ::core::ffi::c_int =
                picoquic_parse_hexa_digit(*hex_input.offset(c2rust_fresh1 as isize));
            if a < 0 as ::core::ffi::c_int || b < 0 as ::core::ffi::c_int {
                ret = 0 as size_t;
                break;
            } else {
                let c2rust_fresh2 = ret;
                ret = ret.wrapping_add(1);
                *bin_output.offset(c2rust_fresh2 as isize) =
                    (a << 4 as ::core::ffi::c_int | b) as uint8_t;
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_connection_id_hexa(
    mut hex_input: *const ::core::ffi::c_char,
    mut input_length: size_t,
    mut cnx_id: *mut picoquic_connection_id_t,
) -> uint8_t {
    memset(
        cnx_id as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<picoquic_connection_id_t>() as size_t,
    );
    (*cnx_id).id_len = picoquic_parse_hexa(
        hex_input,
        input_length,
        &raw mut (*cnx_id).id as *mut uint8_t,
        18 as size_t,
    ) as uint8_t;
    if (*cnx_id).id_len as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        memset(
            cnx_id as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<picoquic_connection_id_t>() as size_t,
        );
    }
    return (*cnx_id).id_len;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_packet_header_cnxid_lengths(
    mut dest_len: uint8_t,
    mut srce_len: uint8_t,
) -> uint8_t {
    let mut ret: uint8_t = 0;
    ret = (if (dest_len as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
        0 as ::core::ffi::c_int
    } else {
        dest_len as ::core::ffi::c_int - 3 as ::core::ffi::c_int
    }) as uint8_t;
    ret = ((ret as ::core::ffi::c_int) << 4 as ::core::ffi::c_int) as uint8_t;
    ret = (ret as ::core::ffi::c_int
        | if (srce_len as ::core::ffi::c_int) < 4 as ::core::ffi::c_int {
            0 as ::core::ffi::c_int
        } else {
            srce_len as ::core::ffi::c_int - 3 as ::core::ffi::c_int
        }) as uint8_t;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_format_connection_id(
    mut bytes: *mut uint8_t,
    mut bytes_max: size_t,
    mut cnx_id: picoquic_connection_id_t,
) -> uint8_t {
    let mut copied: uint8_t = cnx_id.id_len;
    if copied as size_t > bytes_max || copied as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        copied = 0 as uint8_t;
    } else {
        memcpy(
            bytes as *mut ::core::ffi::c_void,
            &raw mut cnx_id.id as *mut uint8_t as *const ::core::ffi::c_void,
            copied as size_t,
        );
    }
    return copied;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_parse_connection_id(
    mut bytes: *const uint8_t,
    mut len: uint8_t,
    mut cnx_id: *mut picoquic_connection_id_t,
) -> uint8_t {
    if len as ::core::ffi::c_int <= PICOQUIC_CONNECTION_ID_MAX_SIZE {
        (*cnx_id).id_len = len;
        memcpy(
            &raw mut (*cnx_id).id as *mut uint8_t as *mut ::core::ffi::c_void,
            bytes as *const ::core::ffi::c_void,
            len as size_t,
        );
    } else {
        len = 0 as uint8_t;
        (*cnx_id).id_len = 0 as uint8_t;
    }
    return len;
}
#[no_mangle]
pub static mut picoquic_null_connection_id: picoquic_connection_id_t =
    st_picoquic_connection_id_t {
        id: [
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
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
            0 as ::core::ffi::c_int as uint8_t,
        ],
        id_len: 0 as uint8_t,
    };
#[no_mangle]
pub unsafe extern "C" fn picoquic_is_connection_id_null(
    mut cnx_id: *const picoquic_connection_id_t,
) -> ::core::ffi::c_int {
    return if (*cnx_id).id_len as ::core::ffi::c_int == 0 as ::core::ffi::c_int {
        1 as ::core::ffi::c_int
    } else {
        0 as ::core::ffi::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_compare_connection_id(
    mut cnx_id1: *const picoquic_connection_id_t,
    mut cnx_id2: *const picoquic_connection_id_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if (*cnx_id1).id_len as ::core::ffi::c_int == (*cnx_id2).id_len as ::core::ffi::c_int {
        ret = memcmp(
            &raw const (*cnx_id1).id as *const uint8_t as *const ::core::ffi::c_void,
            &raw const (*cnx_id2).id as *const uint8_t as *const ::core::ffi::c_void,
            (*cnx_id1).id_len as size_t,
        );
    } else if (*cnx_id1).id_len as ::core::ffi::c_int > (*cnx_id2).id_len as ::core::ffi::c_int {
        ret = 1 as ::core::ffi::c_int;
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_connection_id_hash(
    mut cid: *const picoquic_connection_id_t,
) -> uint64_t {
    let mut val64: uint64_t = 0 as uint64_t;
    let mut i: size_t = 0 as size_t;
    while i < (*cid).id_len as size_t && i < 8 as size_t {
        val64 <<= 8 as ::core::ffi::c_int;
        val64 = val64.wrapping_add((*cid).id[i as usize] as uint64_t);
        i = i.wrapping_add(1);
    }
    while i < (*cid).id_len as size_t {
        let mut top: uint64_t = val64 >> 56 as ::core::ffi::c_int;
        val64 <<= 8 as ::core::ffi::c_int;
        val64 = val64.wrapping_add((*cid).id[i as usize] as uint64_t);
        val64 = val64.wrapping_add(top.wrapping_mul(0x10001 as uint64_t));
        i = i.wrapping_add(1);
    }
    return val64;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_val64_connection_id(
    mut cnx_id: picoquic_connection_id_t,
) -> uint64_t {
    let mut val64: uint64_t = 0 as uint64_t;
    if (cnx_id.id_len as ::core::ffi::c_int) < 8 as ::core::ffi::c_int {
        let mut i: size_t = 0 as size_t;
        while i < cnx_id.id_len as size_t {
            val64 <<= 8 as ::core::ffi::c_int;
            val64 |= cnx_id.id[i as usize] as uint64_t;
            i = i.wrapping_add(1);
        }
        let mut i_0: size_t = cnx_id.id_len as size_t;
        while i_0 < 8 as size_t {
            val64 <<= 8 as ::core::ffi::c_int;
            i_0 = i_0.wrapping_add(1);
        }
    } else {
        let mut i_1: size_t = 0 as size_t;
        while i_1 < 8 as size_t {
            val64 <<= 8 as ::core::ffi::c_int;
            val64 |= cnx_id.id[i_1 as usize] as uint64_t;
            i_1 = i_1.wrapping_add(1);
        }
    }
    return val64;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set64_connection_id(
    mut cnx_id: *mut picoquic_connection_id_t,
    mut val64: uint64_t,
) {
    let mut i: ::core::ffi::c_int = 7 as ::core::ffi::c_int;
    while i >= 0 as ::core::ffi::c_int {
        (*cnx_id).id[i as usize] = (val64 & 0xff as uint64_t) as uint8_t;
        val64 >>= 8 as ::core::ffi::c_int;
        i -= 1;
    }
    let mut i_0: size_t = 8 as size_t;
    while i_0 < ::core::mem::size_of::<[uint8_t; 20]>() as usize {
        (*cnx_id).id[i_0 as usize] = 0 as uint8_t;
        i_0 = i_0.wrapping_add(1);
    }
    (*cnx_id).id_len = 8 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_hash_addr(mut addr: *const sockaddr) -> uint64_t {
    let mut h: uint64_t = 0;
    if (*addr).sa_family as ::core::ffi::c_int == AF_INET {
        let mut a4: *mut sockaddr_in = addr as *mut sockaddr_in;
        h = picohash_bytes(&raw mut (*a4).sin_addr as *mut uint8_t, 4 as uint32_t);
        h = (h as ::core::ffi::c_ulonglong).wrapping_add(
            (128 as ::core::ffi::c_ulonglong)
                .wrapping_mul((*a4).sin_port as ::core::ffi::c_ulonglong),
        ) as uint64_t as uint64_t;
    } else {
        let mut a6: *mut sockaddr_in6 = addr as *mut sockaddr_in6;
        h = picohash_bytes(&raw mut (*a6).sin6_addr as *mut uint8_t, 16 as uint32_t);
        h = (h as ::core::ffi::c_ulonglong).wrapping_add(
            (128 as ::core::ffi::c_ulonglong)
                .wrapping_mul((*a6).sin6_port as ::core::ffi::c_ulonglong),
        ) as uint64_t as uint64_t;
    }
    return h;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_compare_addr(
    mut expected: *const sockaddr,
    mut actual: *const sockaddr,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if (*expected).sa_family as ::core::ffi::c_int == (*actual).sa_family as ::core::ffi::c_int {
        if (*expected).sa_family as ::core::ffi::c_int == AF_INET {
            let mut ex: *mut sockaddr_in = expected as *mut sockaddr_in;
            let mut ac: *mut sockaddr_in = actual as *mut sockaddr_in;
            if (*ex).sin_port as ::core::ffi::c_int == (*ac).sin_port as ::core::ffi::c_int
                && (*ex).sin_addr.s_addr == (*ac).sin_addr.s_addr
            {
                ret = 0 as ::core::ffi::c_int;
            }
        } else {
            let mut ex_0: *mut sockaddr_in6 = expected as *mut sockaddr_in6;
            let mut ac_0: *mut sockaddr_in6 = actual as *mut sockaddr_in6;
            if (*ex_0).sin6_port as ::core::ffi::c_int == (*ac_0).sin6_port as ::core::ffi::c_int
                && memcmp(
                    &raw mut (*ex_0).sin6_addr as *const ::core::ffi::c_void,
                    &raw mut (*ac_0).sin6_addr as *const ::core::ffi::c_void,
                    16 as size_t,
                ) == 0 as ::core::ffi::c_int
            {
                ret = 0 as ::core::ffi::c_int;
            }
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_compare_ip_addr(
    mut expected: *const sockaddr,
    mut actual: *const sockaddr,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if (*expected).sa_family as ::core::ffi::c_int == (*actual).sa_family as ::core::ffi::c_int {
        if (*expected).sa_family as ::core::ffi::c_int == AF_INET6 {
            let mut ex: *mut sockaddr_in6 = expected as *mut sockaddr_in6;
            let mut ac: *mut sockaddr_in6 = actual as *mut sockaddr_in6;
            ret = memcmp(
                &raw mut (*ex).sin6_addr as *const ::core::ffi::c_void,
                &raw mut (*ac).sin6_addr as *const ::core::ffi::c_void,
                16 as size_t,
            );
        } else {
            let mut ex_0: *mut sockaddr_in = expected as *mut sockaddr_in;
            let mut ac_0: *mut sockaddr_in = actual as *mut sockaddr_in;
            ret = if (*ex_0).sin_addr.s_addr == (*ac_0).sin_addr.s_addr {
                0 as ::core::ffi::c_int
            } else {
                -(1 as ::core::ffi::c_int)
            };
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_addr_port(mut addr: *const sockaddr) -> uint16_t {
    let mut port: uint16_t = (if (*addr).sa_family as ::core::ffi::c_int == AF_INET6 {
        (*(addr as *mut sockaddr_in6)).sin6_port as ::core::ffi::c_int
    } else {
        (*(addr as *mut sockaddr_in)).sin_port as ::core::ffi::c_int
    }) as uint16_t;
    return port;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_addr_port(mut addr: *const sockaddr, mut port: uint16_t) {
    if (*addr).sa_family as ::core::ffi::c_int == AF_INET6 {
        (*(addr as *mut sockaddr_in6)).sin6_port = port as in_port_t;
    } else {
        (*(addr as *mut sockaddr_in)).sin_port = port as in_port_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_addr_length(mut addr: *const sockaddr) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if (*addr).sa_family as ::core::ffi::c_int == AF_INET {
        len = ::core::mem::size_of::<sockaddr_in>() as ::core::ffi::c_int;
    } else if (*addr).sa_family as ::core::ffi::c_int == AF_INET6 {
        len = ::core::mem::size_of::<sockaddr_in6>() as ::core::ffi::c_int;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_store_addr(
    mut stored_addr: *mut sockaddr_storage,
    mut addr: *const sockaddr,
) {
    let mut len: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if addr.is_null() || {
        len = picoquic_addr_length(addr);
        len == 0 as ::core::ffi::c_int
    } {
        (*stored_addr).ss_family = 0 as sa_family_t;
    } else {
        memcpy(
            stored_addr as *mut ::core::ffi::c_void,
            addr as *const ::core::ffi::c_void,
            len as size_t,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_ip_addr(
    mut addr: *mut sockaddr,
    mut ip_addr: *mut *mut uint8_t,
    mut ip_addr_len: *mut uint8_t,
) {
    if (*addr).sa_family as ::core::ffi::c_int == AF_INET {
        *ip_addr = &raw mut (*(addr as *mut sockaddr_in)).sin_addr as *mut uint8_t;
        *ip_addr_len = 4 as uint8_t;
    } else if (*addr).sa_family as ::core::ffi::c_int == AF_INET6 {
        *ip_addr = &raw mut (*(addr as *mut sockaddr_in6)).sin6_addr as *mut uint8_t;
        *ip_addr_len = 16 as uint8_t;
    } else {
        *ip_addr = ::core::ptr::null_mut::<uint8_t>();
        *ip_addr_len = 0 as uint8_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_store_text_addr(
    mut stored_addr: *mut sockaddr_storage,
    mut ip_address_text: *const ::core::ffi::c_char,
    mut port: uint16_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut ipv4_addr: *mut sockaddr_in = stored_addr as *mut sockaddr_in;
    let mut ipv6_addr: *mut sockaddr_in6 = stored_addr as *mut sockaddr_in6;
    memset(
        stored_addr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_storage>() as size_t,
    );
    if inet_pton(
        AF_INET,
        ip_address_text,
        &raw mut (*ipv4_addr).sin_addr as *mut ::core::ffi::c_void,
    ) == 1 as ::core::ffi::c_int
    {
        (*ipv4_addr).sin_family = AF_INET as sa_family_t;
        (*ipv4_addr).sin_port = __bswap_16(port) as in_port_t;
    } else if inet_pton(
        AF_INET6,
        ip_address_text,
        &raw mut (*ipv6_addr).sin6_addr as *mut ::core::ffi::c_void,
    ) == 1 as ::core::ffi::c_int
    {
        (*ipv6_addr).sin6_family = AF_INET6 as sa_family_t;
        (*ipv6_addr).sin6_port = __bswap_16(port) as in_port_t;
    } else {
        ret = -(1 as ::core::ffi::c_int);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_addr_text(
    mut addr: *const sockaddr,
    mut text: *mut ::core::ffi::c_char,
    mut text_size: size_t,
) -> *const ::core::ffi::c_char {
    let mut addr_buffer: [::core::ffi::c_char; 128] = [0; 128];
    let mut addr_text: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut ret_text: *const ::core::ffi::c_char = b"?:?\0".as_ptr() as *const ::core::ffi::c_char;
    if !addr.is_null() {
        match (*addr).sa_family as ::core::ffi::c_int {
            AF_INET => {
                addr_text = inet_ntop(
                    AF_INET,
                    &raw mut (*(addr as *mut sockaddr_in)).sin_addr as *const ::core::ffi::c_void,
                    &raw mut addr_buffer as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as socklen_t,
                );
                if picoquic_sprintf(
                    text,
                    text_size,
                    ::core::ptr::null_mut::<size_t>(),
                    b"%s:%d\0".as_ptr() as *const ::core::ffi::c_char,
                    addr_text,
                    __bswap_16((*(addr as *mut sockaddr_in)).sin_port as __uint16_t)
                        as ::core::ffi::c_int,
                ) == 0 as ::core::ffi::c_int
                {
                    ret_text = text;
                }
            }
            AF_INET6 => {
                addr_text = inet_ntop(
                    AF_INET6,
                    &raw mut (*(addr as *mut sockaddr_in6)).sin6_addr as *const ::core::ffi::c_void,
                    &raw mut addr_buffer as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 128]>() as socklen_t,
                );
                if picoquic_sprintf(
                    text,
                    text_size,
                    ::core::ptr::null_mut::<size_t>(),
                    b"[%s]:%d\0".as_ptr() as *const ::core::ffi::c_char,
                    addr_text,
                    __bswap_16((*(addr as *mut sockaddr_in6)).sin6_port as __uint16_t)
                        as ::core::ffi::c_int,
                ) == 0 as ::core::ffi::c_int
                {
                    ret_text = text;
                }
            }
            _ => {}
        }
    }
    return ret_text;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_store_loopback_addr(
    mut stored_addr: *mut sockaddr_storage,
    mut addr_family: ::core::ffi::c_int,
    mut port: uint16_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = -(1 as ::core::ffi::c_int);
    if addr_family == AF_INET {
        ret = picoquic_store_text_addr(
            stored_addr,
            b"127.0.0.1\0".as_ptr() as *const ::core::ffi::c_char,
            port,
        );
    } else if addr_family == AF_INET6 {
        ret = picoquic_store_text_addr(
            stored_addr,
            b"::1\0".as_ptr() as *const ::core::ffi::c_char,
            port,
        );
    }
    return ret;
}
#[no_mangle]
pub static mut picoquic_solution_dir: *const ::core::ffi::c_char =
    ::core::ptr::null::<::core::ffi::c_char>();
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_solution_dir(mut solution_dir: *const ::core::ffi::c_char) {
    picoquic_solution_dir = solution_dir;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_get_input_path(
    mut target_file_path: *mut ::core::ffi::c_char,
    mut file_path_max: size_t,
    mut solution_path: *const ::core::ffi::c_char,
    mut file_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    if solution_path.is_null() {
        solution_path = PICOQUIC_DEFAULT_SOLUTION_DIR.as_ptr();
    }
    let mut separator: *const ::core::ffi::c_char = PICOQUIC_FILE_SEPARATOR.as_ptr();
    let mut solution_path_length: size_t = strlen(solution_path);
    if solution_path_length != 0 as size_t
        && *solution_path.offset(solution_path_length.wrapping_sub(1 as size_t) as isize)
            as ::core::ffi::c_int
            == *separator.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
    {
        separator = b"\0".as_ptr() as *const ::core::ffi::c_char;
    }
    let mut ret: ::core::ffi::c_int = picoquic_sprintf(
        target_file_path,
        file_path_max,
        ::core::ptr::null_mut::<size_t>(),
        b"%s%s%s\0".as_ptr() as *const ::core::ffi::c_char,
        solution_path,
        separator,
        file_name,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_file_open_ex(
    mut file_name: *const ::core::ffi::c_char,
    mut flags: *const ::core::ffi::c_char,
    mut last_err: *mut ::core::ffi::c_int,
) -> *mut FILE {
    let mut F: *mut FILE = ::core::ptr::null_mut::<FILE>();
    F = fopen(file_name, flags) as *mut FILE;
    if F.is_null() && !last_err.is_null() {
        *last_err = *__errno_location();
    }
    return F;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_file_open(
    mut file_name: *const ::core::ffi::c_char,
    mut flags: *const ::core::ffi::c_char,
) -> *mut FILE {
    return picoquic_file_open_ex(
        file_name,
        flags,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_file_close(mut F: *mut FILE) -> *mut FILE {
    if !F.is_null() {
        fclose(F);
    }
    return ::core::ptr::null_mut::<FILE>();
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_file_delete(
    mut file_name: *const ::core::ffi::c_char,
    mut last_err: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    ret = unlink(file_name);
    if !last_err.is_null() && ret != 0 as ::core::ffi::c_int {
        *last_err = *__errno_location();
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_fixed_skip(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut size: uint64_t,
) -> *const uint8_t {
    return if size <= bytes_max.offset_from(bytes) as ::core::ffi::c_long as uint64_t {
        bytes.offset(size as isize)
    } else {
        ::core::ptr::null::<uint8_t>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_varint_skip(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    if bytes < bytes_max {
        let mut v_len: uint8_t = ((1 as ::core::ffi::c_int as uint8_t as ::core::ffi::c_int)
            << (*bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int
                & 3 as ::core::ffi::c_int)) as uint8_t;
        return picoquic_frames_fixed_skip(bytes, bytes_max, v_len as uint64_t);
    } else {
        return ::core::ptr::null::<uint8_t>();
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_varint_decode(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut n64: *mut uint64_t,
) -> *const uint8_t {
    let mut length: uint8_t = 0;
    if bytes < bytes_max && {
        length = ((1 as ::core::ffi::c_int as uint8_t as ::core::ffi::c_int)
            << (*bytes.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
                >> 6 as ::core::ffi::c_int
                & 3 as ::core::ffi::c_int)) as uint8_t;
        bytes.offset(length as ::core::ffi::c_int as isize) <= bytes_max
    } {
        let c2rust_fresh3 = bytes;
        bytes = bytes.offset(1);
        let mut v: uint64_t =
            (*c2rust_fresh3 as ::core::ffi::c_int & 0x3f as ::core::ffi::c_int) as uint64_t;
        loop {
            length = length.wrapping_sub(1);
            if !(length as ::core::ffi::c_int > 0 as ::core::ffi::c_int) {
                break;
            }
            v <<= 8 as ::core::ffi::c_int;
            let c2rust_fresh4 = bytes;
            bytes = bytes.offset(1);
            v = v.wrapping_add(*c2rust_fresh4 as uint64_t);
        }
        *n64 = v;
    } else {
        bytes = ::core::ptr::null::<uint8_t>();
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_varlen_decode(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut n: *mut size_t,
) -> *const uint8_t {
    let mut len: uint64_t = 0 as uint64_t;
    bytes = picoquic_frames_varint_decode(bytes, bytes_max, &raw mut len);
    *n = len as size_t;
    return if *n == len as size_t {
        bytes
    } else {
        ::core::ptr::null::<uint8_t>()
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_uint8_decode(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut n: *mut uint8_t,
) -> *const uint8_t {
    if bytes < bytes_max {
        let c2rust_fresh5 = bytes;
        bytes = bytes.offset(1);
        *n = *c2rust_fresh5;
    } else {
        bytes = ::core::ptr::null::<uint8_t>();
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_uint16_decode(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut n: *mut uint16_t,
) -> *const uint8_t {
    if bytes.offset(::core::mem::size_of::<uint16_t>() as usize as isize) <= bytes_max {
        *n = ((*bytes.offset(0 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *bytes.offset(1 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
            as uint16_t;
        bytes = bytes.offset(::core::mem::size_of::<uint16_t>() as usize as isize);
    } else {
        bytes = ::core::ptr::null::<uint8_t>();
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_uint32_decode(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut n: *mut uint32_t,
) -> *const uint8_t {
    if bytes.offset(::core::mem::size_of::<uint32_t>() as usize as isize) <= bytes_max {
        *n = (((*bytes.offset(0 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *bytes.offset(1 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
            as uint32_t)
            << 16 as ::core::ffi::c_int
            | ((*bytes
                .offset(2 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *bytes
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t;
        bytes = bytes.offset(::core::mem::size_of::<uint32_t>() as usize as isize);
    } else {
        bytes = ::core::ptr::null::<uint8_t>();
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_uint64_decode(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut n: *mut uint64_t,
) -> *const uint8_t {
    if bytes.offset(::core::mem::size_of::<uint64_t>() as usize as isize) <= bytes_max {
        *n = (((((*bytes.offset(0 as ::core::ffi::c_int as isize) as uint16_t
            as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
            | *bytes.offset(1 as ::core::ffi::c_int as isize) as uint16_t as ::core::ffi::c_int)
            as uint32_t)
            << 16 as ::core::ffi::c_int
            | ((*bytes
                .offset(2 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *bytes
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t) as uint64_t)
            << 32 as ::core::ffi::c_int
            | ((((*bytes
                .offset(4 as ::core::ffi::c_int as isize)
                .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *bytes
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int) as uint32_t)
                << 16 as ::core::ffi::c_int
                | ((*bytes
                    .offset(4 as ::core::ffi::c_int as isize)
                    .offset(2 as ::core::ffi::c_int as isize)
                    .offset(0 as ::core::ffi::c_int as isize) as uint16_t
                    as ::core::ffi::c_int)
                    << 8 as ::core::ffi::c_int
                    | *bytes
                        .offset(4 as ::core::ffi::c_int as isize)
                        .offset(2 as ::core::ffi::c_int as isize)
                        .offset(1 as ::core::ffi::c_int as isize) as uint16_t
                        as ::core::ffi::c_int) as uint32_t) as uint64_t;
        bytes = bytes.offset(::core::mem::size_of::<uint64_t>() as usize as isize);
    } else {
        bytes = ::core::ptr::null::<uint8_t>();
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_length_data_skip(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
) -> *const uint8_t {
    let mut length: uint64_t = 0;
    bytes = picoquic_frames_varint_decode(bytes, bytes_max, &raw mut length);
    if !bytes.is_null() {
        bytes = picoquic_frames_fixed_skip(bytes, bytes_max, length);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_cid_decode(
    mut bytes: *const uint8_t,
    mut bytes_max: *const uint8_t,
    mut cid: *mut picoquic_connection_id_t,
) -> *const uint8_t {
    bytes = picoquic_frames_uint8_decode(bytes, bytes_max, &raw mut (*cid).id_len);
    if (*cid).id_len as ::core::ffi::c_int > PICOQUIC_CONNECTION_ID_MAX_SIZE
        || bytes.offset((*cid).id_len as ::core::ffi::c_int as isize) > bytes_max
    {
        bytes = ::core::ptr::null::<uint8_t>();
    } else {
        memset(
            &raw mut (*cid).id as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 20]>() as size_t,
        );
        memcpy(
            &raw mut (*cid).id as *mut uint8_t as *mut ::core::ffi::c_void,
            bytes as *const ::core::ffi::c_void,
            (*cid).id_len as size_t,
        );
        bytes = bytes.offset((*cid).id_len as ::core::ffi::c_int as isize);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_varint_encode_length(mut n64: uint64_t) -> size_t {
    let mut len: size_t = 8 as size_t;
    if n64 < 16384 as uint64_t {
        if n64 < 64 as uint64_t {
            len = 1 as size_t;
        } else {
            len = 2 as size_t;
        }
    } else if n64 < 1073741824 as uint64_t {
        len = 4 as size_t;
    }
    return len;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_varint_encode(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut n64: uint64_t,
) -> *mut uint8_t {
    if n64 < 16384 as uint64_t {
        if n64 < 64 as uint64_t {
            if bytes.offset(1 as ::core::ffi::c_int as isize) <= bytes_max as *mut uint8_t {
                let c2rust_fresh6 = bytes;
                bytes = bytes.offset(1);
                *c2rust_fresh6 = n64 as uint8_t;
            } else {
                bytes = ::core::ptr::null_mut::<uint8_t>();
            }
        } else if bytes.offset(2 as ::core::ffi::c_int as isize) <= bytes_max as *mut uint8_t {
            let c2rust_fresh7 = bytes;
            bytes = bytes.offset(1);
            *c2rust_fresh7 = (n64 >> 8 as ::core::ffi::c_int | 0x40 as uint64_t) as uint8_t;
            let c2rust_fresh8 = bytes;
            bytes = bytes.offset(1);
            *c2rust_fresh8 = n64 as uint8_t;
        } else {
            bytes = ::core::ptr::null_mut::<uint8_t>();
        }
    } else if n64 < 1073741824 as uint64_t {
        if bytes.offset(4 as ::core::ffi::c_int as isize) <= bytes_max as *mut uint8_t {
            let c2rust_fresh9 = bytes;
            bytes = bytes.offset(1);
            *c2rust_fresh9 = (n64 >> 24 as ::core::ffi::c_int | 0x80 as uint64_t) as uint8_t;
            let c2rust_fresh10 = bytes;
            bytes = bytes.offset(1);
            *c2rust_fresh10 = (n64 >> 16 as ::core::ffi::c_int) as uint8_t;
            let c2rust_fresh11 = bytes;
            bytes = bytes.offset(1);
            *c2rust_fresh11 = (n64 >> 8 as ::core::ffi::c_int) as uint8_t;
            let c2rust_fresh12 = bytes;
            bytes = bytes.offset(1);
            *c2rust_fresh12 = n64 as uint8_t;
        } else {
            bytes = ::core::ptr::null_mut::<uint8_t>();
        }
    } else if bytes.offset(8 as ::core::ffi::c_int as isize) <= bytes_max as *mut uint8_t {
        let c2rust_fresh13 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh13 = (n64 >> 56 as ::core::ffi::c_int | 0xc0 as uint64_t) as uint8_t;
        let c2rust_fresh14 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh14 = (n64 >> 48 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh15 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh15 = (n64 >> 40 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh16 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh16 = (n64 >> 32 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh17 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh17 = (n64 >> 24 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh18 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh18 = (n64 >> 16 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh19 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh19 = (n64 >> 8 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh20 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh20 = n64 as uint8_t;
    } else {
        bytes = ::core::ptr::null_mut::<uint8_t>();
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_varlen_encode(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut n: size_t,
) -> *mut uint8_t {
    return picoquic_frames_varint_encode(bytes, bytes_max, n as uint64_t);
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_uint8_encode(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut n: uint8_t,
) -> *mut uint8_t {
    if bytes.offset(::core::mem::size_of::<uint8_t>() as usize as isize) > bytes_max as *mut uint8_t
    {
        bytes = ::core::ptr::null_mut::<uint8_t>();
    } else {
        let c2rust_fresh21 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh21 = n;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_uint16_encode(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut n: uint16_t,
) -> *mut uint8_t {
    if bytes.offset(::core::mem::size_of::<uint16_t>() as usize as isize)
        > bytes_max as *mut uint8_t
    {
        bytes = ::core::ptr::null_mut::<uint8_t>();
    } else {
        let c2rust_fresh22 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh22 = (n as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh23 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh23 = n as uint8_t;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_uint24_encode(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut n: uint32_t,
) -> *mut uint8_t {
    if bytes.offset(3 as ::core::ffi::c_int as isize) > bytes_max as *mut uint8_t {
        bytes = ::core::ptr::null_mut::<uint8_t>();
    } else {
        let c2rust_fresh24 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh24 = (n >> 16 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh25 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh25 = (n >> 8 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh26 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh26 = n as uint8_t;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_uint32_encode(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut n: uint32_t,
) -> *mut uint8_t {
    if bytes.offset(::core::mem::size_of::<uint32_t>() as usize as isize)
        > bytes_max as *mut uint8_t
    {
        bytes = ::core::ptr::null_mut::<uint8_t>();
    } else {
        let c2rust_fresh27 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh27 = (n >> 24 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh28 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh28 = (n >> 16 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh29 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh29 = (n >> 8 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh30 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh30 = n as uint8_t;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_uint64_encode(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut n: uint64_t,
) -> *mut uint8_t {
    if bytes.offset(::core::mem::size_of::<uint64_t>() as usize as isize)
        > bytes_max as *mut uint8_t
    {
        bytes = ::core::ptr::null_mut::<uint8_t>();
    } else {
        let c2rust_fresh31 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh31 = (n >> 56 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh32 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh32 = (n >> 48 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh33 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh33 = (n >> 40 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh34 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh34 = (n >> 32 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh35 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh35 = (n >> 24 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh36 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh36 = (n >> 16 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh37 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh37 = (n >> 8 as ::core::ffi::c_int) as uint8_t;
        let c2rust_fresh38 = bytes;
        bytes = bytes.offset(1);
        *c2rust_fresh38 = n as uint8_t;
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_length_data_encode(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut l: size_t,
    mut v: *const uint8_t,
) -> *mut uint8_t {
    bytes = picoquic_frames_varlen_encode(bytes, bytes_max, l);
    if !bytes.is_null() && bytes.offset(l as isize) <= bytes_max as *mut uint8_t {
        memcpy(
            bytes as *mut ::core::ffi::c_void,
            v as *const ::core::ffi::c_void,
            l,
        );
        bytes = bytes.offset(l as isize);
    } else {
        bytes = ::core::ptr::null_mut::<uint8_t>();
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_cid_encode(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut cid: *const picoquic_connection_id_t,
) -> *mut uint8_t {
    return picoquic_frames_length_data_encode(
        bytes,
        bytes_max,
        (*cid).id_len as size_t,
        &raw const (*cid).id as *const uint8_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_frames_charz_encode(
    mut bytes: *mut uint8_t,
    mut bytes_max: *const uint8_t,
    mut s: *const ::core::ffi::c_char,
) -> *mut uint8_t {
    if s.is_null() {
        bytes = picoquic_frames_varlen_encode(bytes, bytes_max, 0 as size_t);
    } else {
        let mut l: size_t = strlen(s);
        bytes = picoquic_frames_length_data_encode(bytes, bytes_max, l, s as *const uint8_t);
    }
    return bytes;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_constant_time_memcmp(
    mut x: *const uint8_t,
    mut y: *const uint8_t,
    mut l: size_t,
) -> ::core::ffi::c_int {
    let mut ret: uint64_t = 0 as uint64_t;
    while l > 0 as size_t {
        let c2rust_fresh39 = x;
        x = x.offset(1);
        let c2rust_fresh40 = y;
        y = y.offset(1);
        ret = ret.wrapping_add(
            (*c2rust_fresh39 as ::core::ffi::c_int ^ *c2rust_fresh40 as ::core::ffi::c_int)
                as uint64_t,
        );
        l = l.wrapping_sub(1);
    }
    return if ret == 0 as uint64_t {
        0 as ::core::ffi::c_int
    } else {
        -(1 as ::core::ffi::c_int)
    };
}
unsafe extern "C" fn picoquic_set_abs_delay(mut ts: *mut timespec, mut microsec_wait: uint64_t) {
    clock_gettime(CLOCK_REALTIME, ts);
    (*ts).tv_sec = ((*ts).tv_sec as ::core::ffi::c_ulong)
        .wrapping_add(microsec_wait.wrapping_div(1000000 as uint64_t) as ::core::ffi::c_ulong)
        as __time_t as __time_t;
    (*ts).tv_nsec = ((*ts).tv_nsec as ::core::ffi::c_ulong).wrapping_add(
        microsec_wait
            .wrapping_rem(1000000 as uint64_t)
            .wrapping_mul(1000 as uint64_t) as ::core::ffi::c_ulong,
    ) as __syscall_slong_t as __syscall_slong_t;
    if (*ts).tv_nsec > 1000000000 as __syscall_slong_t {
        (*ts).tv_sec += 1;
        (*ts).tv_nsec -= 1000000000 as __syscall_slong_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_thread(
    mut thread: *mut pthread_t,
    mut thread_fn: picoquic_thread_fn,
    mut arg: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = pthread_create(
        thread,
        ::core::ptr::null::<pthread_attr_t>(),
        thread_fn
            as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> *mut ::core::ffi::c_void>,
        arg,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_wait_thread(mut thread: pthread_t) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ret = pthread_join(thread, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>());
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_thread(mut thread: *mut pthread_t) {
    if pthread_join(*thread, ::core::ptr::null_mut::<*mut ::core::ffi::c_void>())
        != 0 as ::core::ffi::c_int
    {
        pthread_kill(*thread, SIGTERM);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_mutex(
    mut mutex: *mut pthread_mutex_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int =
        pthread_mutex_init(mutex, ::core::ptr::null::<pthread_mutexattr_t>());
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_mutex(
    mut mutex: *mut pthread_mutex_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = pthread_mutex_destroy(mutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_lock_mutex(
    mut mutex: *mut pthread_mutex_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = pthread_mutex_lock(mutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_unlock_mutex(
    mut mutex: *mut pthread_mutex_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = pthread_mutex_unlock(mutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_create_event(
    mut event: *mut picoquic_event_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    memset(
        event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<picoquic_event_t>() as size_t,
    );
    ret = pthread_mutex_init(
        &raw mut (*event).mutex,
        ::core::ptr::null::<pthread_mutexattr_t>(),
    );
    if ret == 0 as ::core::ffi::c_int {
        ret = pthread_cond_init(
            &raw mut (*event).cond,
            ::core::ptr::null::<pthread_condattr_t>(),
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_delete_event(mut event: *mut picoquic_event_t) {
    pthread_mutex_destroy(&raw mut (*event).mutex);
    pthread_cond_destroy(&raw mut (*event).cond);
    memset(
        event as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<picoquic_event_t>() as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_signal_event(
    mut event: *mut picoquic_event_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    pthread_mutex_lock(&raw mut (*event).mutex);
    ret = pthread_cond_broadcast(&raw mut (*event).cond);
    pthread_mutex_unlock(&raw mut (*event).mutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_wait_for_event(
    mut event: *mut picoquic_event_t,
    mut microsec_wait: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    pthread_mutex_lock(&raw mut (*event).mutex);
    if microsec_wait == UINT64_MAX as uint64_t {
        ret = pthread_cond_wait(&raw mut (*event).cond, &raw mut (*event).mutex);
    } else {
        let mut abstime: timespec = timespec {
            tv_sec: 0,
            tv_nsec: 0,
        };
        picoquic_set_abs_delay(&raw mut abstime, microsec_wait);
        ret = pthread_cond_timedwait(
            &raw mut (*event).cond,
            &raw mut (*event).mutex,
            &raw mut abstime,
        );
    }
    pthread_mutex_unlock(&raw mut (*event).mutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_test_random(mut random_context: *mut uint64_t) -> uint64_t {
    let mut z: uint64_t = 0;
    *random_context = (*random_context as ::core::ffi::c_ulong)
        .wrapping_add(0x9e3779b97f4a7c15 as ::core::ffi::c_ulong) as uint64_t
        as uint64_t;
    z = *random_context;
    z = ((z ^ z >> 30 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong)
        .wrapping_mul(0xbf58476d1ce4e5b9 as ::core::ffi::c_ulonglong) as uint64_t;
    z = ((z ^ z >> 27 as ::core::ffi::c_int) as ::core::ffi::c_ulonglong)
        .wrapping_mul(0x94d049bb133111eb as ::core::ffi::c_ulonglong) as uint64_t;
    return z ^ z >> 31 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_test_random_bytes(
    mut random_context: *mut uint64_t,
    mut bytes: *mut uint8_t,
    mut bytes_max: size_t,
) {
    let mut byte_index: size_t = 0 as size_t;
    while byte_index < bytes_max {
        let mut v: uint64_t = picoquic_test_random(random_context);
        let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while i < 8 as ::core::ffi::c_int && byte_index < bytes_max {
            let c2rust_fresh41 = byte_index;
            byte_index = byte_index.wrapping_add(1);
            *bytes.offset(c2rust_fresh41 as isize) = (v & 0xff as uint64_t) as uint8_t;
            v >>= 8 as ::core::ffi::c_int;
            i += 1;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_test_uniform_random(
    mut random_context: *mut uint64_t,
    mut rnd_max: uint64_t,
) -> uint64_t {
    let mut rnd: uint64_t = 0 as uint64_t;
    if rnd_max > 0 as uint64_t {
        let mut rnd_min: uint64_t = (UINT64_MAX as uint64_t).wrapping_rem(rnd_max);
        loop {
            rnd = picoquic_test_random(random_context);
            if !(rnd < rnd_min) {
                break;
            }
        }
        rnd = rnd.wrapping_rem(rnd_max);
    }
    return rnd;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_test_gauss_random(
    mut random_context: *mut uint64_t,
) -> ::core::ffi::c_double {
    let mut dx: ::core::ffi::c_double = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
    let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while i < 12 as ::core::ffi::c_int {
        let mut d: ::core::ffi::c_double = 0.;
        let mut r: uint64_t = picoquic_test_random(random_context);
        r ^= r >> 17 as ::core::ffi::c_int;
        r ^= r >> 34 as ::core::ffi::c_int;
        d = (r & 0x1ffff as uint64_t) as ::core::ffi::c_double + 0.5f64;
        d /= 0x20000 as ::core::ffi::c_int as ::core::ffi::c_double;
        dx += d;
        i += 1;
    }
    dx -= 6.0f64;
    return dx;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_uint8_to_str(
    mut text: *mut ::core::ffi::c_char,
    mut text_len: size_t,
    mut data: *const uint8_t,
    mut data_len: size_t,
) -> *mut ::core::ffi::c_char {
    let mut render_length: size_t = data_len;
    let mut rendered: size_t = 0;
    if render_length.wrapping_add(1 as size_t) > text_len {
        if text_len > 4 as size_t {
            render_length = text_len.wrapping_sub(4 as size_t);
        } else {
            render_length = 0 as size_t;
        }
    }
    rendered = 0 as size_t;
    while rendered < render_length {
        let mut c: ::core::ffi::c_int = *data.offset(rendered as isize) as ::core::ffi::c_int;
        if c < ' ' as i32 || c >= 127 as ::core::ffi::c_int {
            c = '?' as i32;
        }
        *text.offset(rendered as isize) = c as ::core::ffi::c_char;
        rendered = rendered.wrapping_add(1);
    }
    if rendered < data_len {
        let mut i: size_t = 0 as size_t;
        while i < 3 as size_t && rendered.wrapping_add(1 as size_t) < text_len {
            *text.offset(rendered as isize) = '.' as i32 as ::core::ffi::c_char;
            i = i.wrapping_add(1);
            rendered = rendered.wrapping_add(1);
        }
    }
    *text.offset(rendered as isize) = 0 as ::core::ffi::c_char;
    return text;
}

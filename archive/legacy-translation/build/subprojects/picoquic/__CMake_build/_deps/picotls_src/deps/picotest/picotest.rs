extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn vfprintf(
        __s: *mut FILE,
        __format: *const ::core::ffi::c_char,
        __arg: ::core::ffi::VaList,
    ) -> ::core::ffi::c_int;
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
pub type va_list = __builtin_va_list;
pub type size_t = usize;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
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
#[inline]
unsafe extern "C" fn vprintf(
    mut __fmt: *const ::core::ffi::c_char,
    mut __arg: ::core::ffi::VaList,
) -> ::core::ffi::c_int {
    return vfprintf(stdout, __fmt, __arg.as_va_list());
}
#[no_mangle]
pub static mut test_index: [::core::ffi::c_int; 32] = [
    1 as ::core::ffi::c_int,
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
static mut test_fail: [::core::ffi::c_int; 32] = [0; 32];
static mut test_level: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
unsafe extern "C" fn indent() {
    let mut i: ::core::ffi::c_int = 0;
    i = 0 as ::core::ffi::c_int;
    while i != test_level {
        printf(b"    \0".as_ptr() as *const ::core::ffi::c_char);
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn note(mut fmt: *const ::core::ffi::c_char, mut c2rust_args: ...) {
    let mut arg: ::core::ffi::VaListImpl;
    indent();
    printf(b"# \0".as_ptr() as *const ::core::ffi::c_char);
    arg = c2rust_args.clone();
    vprintf(fmt, arg.as_va_list());
    printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    fflush(stdout);
}
#[no_mangle]
pub unsafe extern "C" fn _ok(
    mut cond: ::core::ffi::c_int,
    mut fmt: *const ::core::ffi::c_char,
    mut c2rust_args: ...
) {
    let mut arg: ::core::ffi::VaListImpl;
    if cond == 0 {
        test_fail[test_level as usize] = 1 as ::core::ffi::c_int;
    }
    indent();
    let c2rust_fresh0 = test_index[test_level as usize];
    test_index[test_level as usize] = test_index[test_level as usize] + 1;
    printf(
        b"%s %d - \0".as_ptr() as *const ::core::ffi::c_char,
        if cond != 0 {
            b"ok\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"not ok\0".as_ptr() as *const ::core::ffi::c_char
        },
        c2rust_fresh0,
    );
    arg = c2rust_args.clone();
    vprintf(fmt, arg.as_va_list());
    printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
    fflush(stdout);
}
#[no_mangle]
pub unsafe extern "C" fn done_testing() -> ::core::ffi::c_int {
    indent();
    printf(
        b"1..%d\n\0".as_ptr() as *const ::core::ffi::c_char,
        test_index[test_level as usize] - 1 as ::core::ffi::c_int,
    );
    fflush(stdout);
    return test_fail[test_level as usize];
}
#[no_mangle]
pub unsafe extern "C" fn enter_subtest(mut name: *const ::core::ffi::c_char) {
    test_level += 1;
    test_index[test_level as usize] = 1 as ::core::ffi::c_int;
    test_fail[test_level as usize] = 0 as ::core::ffi::c_int;
    note(
        b"Subtest: %s\0".as_ptr() as *const ::core::ffi::c_char,
        name,
    );
}
#[no_mangle]
pub unsafe extern "C" fn exit_subtest(mut name: *const ::core::ffi::c_char) {
    done_testing();
    test_level -= 1;
    _ok(
        (test_fail[(test_level + 1 as ::core::ffi::c_int) as usize] == 0) as ::core::ffi::c_int,
        b"%s\0".as_ptr() as *const ::core::ffi::c_char,
        name,
    );
    test_index[(test_level + 1 as ::core::ffi::c_int) as usize] = 0 as ::core::ffi::c_int;
    test_fail[(test_level + 1 as ::core::ffi::c_int) as usize] = 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn test_is_at(
    mut index: ::core::ffi::c_int,
    mut c2rust_args: ...
) -> ::core::ffi::c_int {
    let mut arg: ::core::ffi::VaListImpl;
    arg = c2rust_args.clone();
    let mut level: size_t = 0;
    level = 0 as size_t;
    while index == test_index[level as usize] && index != 0 as ::core::ffi::c_int {
        index = arg.arg::<::core::ffi::c_int>();
        level = level.wrapping_add(1);
    }
    return (index == test_index[level as usize] && index == 0 as ::core::ffi::c_int)
        as ::core::ffi::c_int;
}

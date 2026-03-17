extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn picohash_create(
        nb_bin: size_t,
        picohash_hash: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> uint64_t>,
        picohash_compute: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                *const ::core::ffi::c_void,
            ) -> ::core::ffi::c_int,
        >,
    ) -> *mut picohash_table;
    fn picohash_retrieve(
        hash_table: *mut picohash_table,
        key: *const ::core::ffi::c_void,
    ) -> *mut picohash_item;
    fn picohash_insert(
        hash_table: *mut picohash_table,
        key: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn picohash_delete(hash_table: *mut picohash_table, delete_key_too: ::core::ffi::c_int);
    fn picoquic_compare_connection_id(
        cnx_id1: *const picoquic_connection_id_t,
        cnx_id2: *const picoquic_connection_id_t,
    ) -> ::core::ffi::c_int;
    fn picoquic_connection_id_hash(cid: *const picoquic_connection_id_t) -> uint64_t;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint64_t = u64;
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
pub type uint8_t = __uint8_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _picohash_item {
    pub hash: uint64_t,
    pub next_in_bin: *mut _picohash_item,
    pub key: *const ::core::ffi::c_void,
}
pub type picohash_item = _picohash_item;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct picohash_table {
    pub hash_bin: *mut *mut picohash_item,
    pub nb_bin: size_t,
    pub count: size_t,
    pub picohash_hash: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> uint64_t>,
    pub picohash_compare: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    pub picohash_key_to_item:
        Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> *mut picohash_item>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_connection_id_t {
    pub id: [uint8_t; 20],
    pub id_len: uint8_t,
}
pub type picoquic_connection_id_t = st_picoquic_connection_id_t;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
unsafe extern "C" fn picoquic_cid_hash(mut key: *const ::core::ffi::c_void) -> uint64_t {
    let mut cid: *const picoquic_connection_id_t = key as *const picoquic_connection_id_t;
    return picoquic_connection_id_hash(cid);
}
unsafe extern "C" fn picoquic_cid_compare(
    mut key0: *const ::core::ffi::c_void,
    mut key1: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut cid0: *const picoquic_connection_id_t = key0 as *const picoquic_connection_id_t;
    let mut cid1: *const picoquic_connection_id_t = key1 as *const picoquic_connection_id_t;
    return picoquic_compare_connection_id(cid0, cid1);
}
#[no_mangle]
pub unsafe extern "C" fn cidset_create() -> *mut picohash_table {
    return picohash_create(
        32 as size_t,
        Some(picoquic_cid_hash as unsafe extern "C" fn(*const ::core::ffi::c_void) -> uint64_t),
        Some(
            picoquic_cid_compare
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn cidset_delete(mut cids: *mut picohash_table) -> *mut picohash_table {
    picohash_delete(cids, 1 as ::core::ffi::c_int);
    return ::core::ptr::null_mut::<picohash_table>();
}
#[no_mangle]
pub unsafe extern "C" fn cidset_insert(
    mut cids: *mut picohash_table,
    mut cid: *const picoquic_connection_id_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut item: *const picohash_item = picohash_retrieve(cids, cid as *const ::core::ffi::c_void);
    if item.is_null() {
        let mut key: *mut picoquic_connection_id_t =
            malloc(::core::mem::size_of::<picoquic_connection_id_t>() as size_t)
                as *mut picoquic_connection_id_t;
        if key.is_null() {
            ret = -(1 as ::core::ffi::c_int);
        } else {
            *key = *cid;
            ret = picohash_insert(cids, key as *const ::core::ffi::c_void);
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn cidset_has_cid(
    mut cids: *mut picohash_table,
    mut cid: *const picoquic_connection_id_t,
) -> ::core::ffi::c_int {
    return (picohash_retrieve(cids, cid as *const ::core::ffi::c_void)
        != NULL as *mut picohash_item) as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cidset_iterate(
    mut cids: *const picohash_table,
    mut cb: Option<
        unsafe extern "C" fn(
            *const picoquic_connection_id_t,
            *mut ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    mut cbptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut i: size_t = 0 as size_t;
    while ret == 0 as ::core::ffi::c_int && i < (*cids).nb_bin {
        let mut item: *mut picohash_item = *(*cids).hash_bin.offset(i as isize);
        while ret == 0 as ::core::ffi::c_int && !item.is_null() {
            ret = cb.expect("non-null function pointer")(
                (*item).key as *const picoquic_connection_id_t,
                cbptr,
            );
            item = (*item).next_in_bin as *mut picohash_item;
        }
        i = i.wrapping_add(1);
    }
    return ret;
}
unsafe extern "C" fn print_cid(
    mut cid: *const picoquic_connection_id_t,
    mut cbptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut f: *mut FILE = cbptr as *mut FILE;
    fprintf(f, b"  <\0".as_ptr() as *const ::core::ffi::c_char);
    let mut i: uint8_t = 0 as uint8_t;
    while (i as ::core::ffi::c_int) < (*cid).id_len as ::core::ffi::c_int {
        fprintf(
            f,
            b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
            (*cid).id[i as usize] as ::core::ffi::c_int,
        );
        i = i.wrapping_add(1);
    }
    fprintf(f, b">\n\0".as_ptr() as *const ::core::ffi::c_char);
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn cidset_print(mut f: *mut FILE, mut cids: *mut picohash_table) {
    cidset_iterate(
        cids,
        Some(
            print_cid
                as unsafe extern "C" fn(
                    *const picoquic_connection_id_t,
                    *mut ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
        f as *mut ::core::ffi::c_void,
    );
}

extern "C" {
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
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
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn picohash_create_ex(
    mut nb_bin: size_t,
    mut picohash_hash: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> uint64_t>,
    mut picohash_compare: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
    mut picohash_key_to_item: Option<
        unsafe extern "C" fn(*const ::core::ffi::c_void) -> *mut picohash_item,
    >,
) -> *mut picohash_table {
    let mut t: *mut picohash_table =
        malloc(::core::mem::size_of::<picohash_table>() as size_t) as *mut picohash_table;
    let mut items_length: size_t =
        (::core::mem::size_of::<*mut picohash_item>() as size_t).wrapping_mul(nb_bin);
    (*t).hash_bin = ::core::ptr::null_mut::<*mut picohash_item>();
    if !t.is_null()
        && items_length.wrapping_div(::core::mem::size_of::<*mut picohash_item>() as size_t)
            == nb_bin
    {
        (*t).hash_bin =
            malloc((::core::mem::size_of::<*mut picohash_item>() as size_t).wrapping_mul(nb_bin))
                as *mut *mut picohash_item;
    }
    if (*t).hash_bin.is_null() {
        free(t as *mut ::core::ffi::c_void);
        t = ::core::ptr::null_mut::<picohash_table>();
    } else {
        memset(
            (*t).hash_bin as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            (::core::mem::size_of::<*mut picohash_item>() as size_t).wrapping_mul(nb_bin),
        );
        (*t).nb_bin = nb_bin;
        (*t).count = 0 as size_t;
        (*t).picohash_hash = picohash_hash;
        (*t).picohash_compare = picohash_compare;
        (*t).picohash_key_to_item = picohash_key_to_item;
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn picohash_create(
    mut nb_bin: size_t,
    mut picohash_hash: Option<unsafe extern "C" fn(*const ::core::ffi::c_void) -> uint64_t>,
    mut picohash_compare: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
) -> *mut picohash_table {
    return picohash_create_ex(nb_bin, picohash_hash, picohash_compare, None);
}
#[no_mangle]
pub unsafe extern "C" fn picohash_retrieve(
    mut hash_table: *mut picohash_table,
    mut key: *const ::core::ffi::c_void,
) -> *mut picohash_item {
    let mut hash: uint64_t = (*hash_table)
        .picohash_hash
        .expect("non-null function pointer")(key);
    let mut bin: uint32_t = hash.wrapping_rem((*hash_table).nb_bin as uint64_t) as uint32_t;
    let mut item: *mut picohash_item = *(*hash_table).hash_bin.offset(bin as isize);
    while !item.is_null() {
        if (*hash_table)
            .picohash_compare
            .expect("non-null function pointer")(key, (*item).key)
            == 0 as ::core::ffi::c_int
        {
            break;
        }
        item = (*item).next_in_bin as *mut picohash_item;
    }
    return item;
}
#[no_mangle]
pub unsafe extern "C" fn picohash_insert(
    mut hash_table: *mut picohash_table,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut hash: uint64_t = (*hash_table)
        .picohash_hash
        .expect("non-null function pointer")(key);
    let mut bin: uint32_t = hash.wrapping_rem((*hash_table).nb_bin as uint64_t) as uint32_t;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut item: *mut picohash_item = ::core::ptr::null_mut::<picohash_item>();
    if (*hash_table).picohash_key_to_item.is_none() {
        item = malloc(::core::mem::size_of::<picohash_item>() as size_t) as *mut picohash_item;
    } else {
        item = (*hash_table)
            .picohash_key_to_item
            .expect("non-null function pointer")(key);
    }
    if item.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        (*item).hash = hash;
        (*item).key = key;
        (*item).next_in_bin = *(*hash_table).hash_bin.offset(bin as isize) as *mut _picohash_item;
        let ref mut c2rust_fresh0 = *(*hash_table).hash_bin.offset(bin as isize);
        *c2rust_fresh0 = item;
        (*hash_table).count = (*hash_table).count.wrapping_add(1);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picohash_delete_item(
    mut hash_table: *mut picohash_table,
    mut item: *mut picohash_item,
    mut delete_key_too: ::core::ffi::c_int,
) {
    let mut bin: uint32_t = (*item).hash.wrapping_rem((*hash_table).nb_bin as uint64_t) as uint32_t;
    let mut previous: *mut picohash_item = *(*hash_table).hash_bin.offset(bin as isize);
    let mut shall_delete: *const ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>();
    if previous == item {
        let ref mut c2rust_fresh1 = *(*hash_table).hash_bin.offset(bin as isize);
        *c2rust_fresh1 = (*item).next_in_bin as *mut picohash_item;
        (*hash_table).count = (*hash_table).count.wrapping_sub(1);
    } else {
        while !previous.is_null() {
            if (*previous).next_in_bin == item {
                (*previous).next_in_bin = (*item).next_in_bin;
                (*hash_table).count = (*hash_table).count.wrapping_sub(1);
                break;
            } else {
                previous = (*previous).next_in_bin as *mut picohash_item;
            }
        }
    }
    shall_delete = (*item).key;
    if (*hash_table).picohash_key_to_item.is_none() {
        free(item as *mut ::core::ffi::c_void);
    }
    if delete_key_too != 0 {
        free(shall_delete as *mut ::core::ffi::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picohash_delete_key(
    mut hash_table: *mut picohash_table,
    mut key: *mut ::core::ffi::c_void,
    mut delete_key_too: ::core::ffi::c_int,
) {
    let mut item: *mut picohash_item = picohash_retrieve(hash_table, key);
    if !item.is_null() {
        picohash_delete_item(hash_table, item, delete_key_too);
    } else if delete_key_too != 0 {
        free(key);
    }
}
#[no_mangle]
pub unsafe extern "C" fn picohash_delete(
    mut hash_table: *mut picohash_table,
    mut delete_key_too: ::core::ffi::c_int,
) {
    let mut i: uint32_t = 0 as uint32_t;
    while (i as size_t) < (*hash_table).nb_bin {
        let mut item: *mut picohash_item = *(*hash_table).hash_bin.offset(i as isize);
        while !item.is_null() {
            let mut tmp: *mut picohash_item = item;
            let mut key_to_delete: *const ::core::ffi::c_void = (*tmp).key;
            item = (*item).next_in_bin as *mut picohash_item;
            if (*hash_table).picohash_key_to_item.is_none() {
                free(tmp as *mut ::core::ffi::c_void);
            }
            if delete_key_too != 0 {
                free(key_to_delete as *mut ::core::ffi::c_void);
            }
        }
        i = i.wrapping_add(1);
    }
    free((*hash_table).hash_bin as *mut ::core::ffi::c_void);
    free(hash_table as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn picohash_hash_mix(mut hash: uint64_t, mut h2: uint64_t) -> uint64_t {
    h2 ^= hash << 17 as ::core::ffi::c_int ^ hash >> 37 as ::core::ffi::c_int;
    hash ^= h2 << 31 as ::core::ffi::c_int ^ h2 >> 17 as ::core::ffi::c_int;
    return hash;
}
#[no_mangle]
pub unsafe extern "C" fn picohash_bytes(mut key: *const uint8_t, mut length: uint32_t) -> uint64_t {
    let mut hash: uint64_t = 0xdeadbeef as uint64_t;
    let mut i: uint32_t = 0 as uint32_t;
    while i < length {
        hash ^= *key.offset(i as isize) as uint64_t;
        hash ^= hash << 31 as ::core::ffi::c_int ^ hash >> 17 as ::core::ffi::c_int;
        i = i.wrapping_add(1);
    }
    return hash;
}

use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type stack_st;
    pub type bignum_ctx;
    pub type evp_cipher_st;
    pub type evp_cipher_ctx_st;
    pub type evp_md_st;
    pub type evp_md_ctx_st;
    pub type evp_mac_st;
    pub type evp_mac_ctx_st;
    pub type evp_pkey_st;
    pub type evp_pkey_ctx_st;
    pub type hmac_ctx_st;
    pub type ec_key_st;
    pub type x509_st;
    pub type x509_store_st;
    pub type x509_store_ctx_st;
    pub type x509_lookup_st;
    pub type x509_lookup_method_st;
    pub type X509_VERIFY_PARAM_st;
    pub type ossl_lib_ctx_st;
    pub type engine_st;
    pub type ec_group_st;
    pub type ec_point_st;
    pub type stack_st_X509;
    pub type st_ptls_t;
    pub type st_ptls_key_schedule_t;
    pub type st_ptls_traffic_protection_t;
    pub type async_job_st;
    pub type async_wait_ctx_st;
    static mut stderr: *mut FILE;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
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
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn OPENSSL_sk_num(_: *const OPENSSL_STACK) -> ::core::ffi::c_int;
    fn OPENSSL_sk_value(_: *const OPENSSL_STACK, _: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_void;
    fn OPENSSL_sk_new_null() -> *mut OPENSSL_STACK;
    fn OPENSSL_sk_pop_free(
        st: *mut OPENSSL_STACK,
        func: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
    );
    fn OPENSSL_sk_push(
        st: *mut OPENSSL_STACK,
        data: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn CRYPTO_free(
        ptr: *mut ::core::ffi::c_void,
        file: *const ::core::ffi::c_char,
        line: ::core::ffi::c_int,
    );
    fn BN_CTX_new() -> *mut BN_CTX;
    fn BN_CTX_free(c: *mut BN_CTX);
    fn EC_GROUP_free(group: *mut EC_GROUP);
    fn EC_GROUP_get_curve_name(group: *const EC_GROUP) -> ::core::ffi::c_int;
    fn EC_GROUP_get_degree(group: *const EC_GROUP) -> ::core::ffi::c_int;
    fn EC_GROUP_new_by_curve_name(nid: ::core::ffi::c_int) -> *mut EC_GROUP;
    fn EC_POINT_new(group: *const EC_GROUP) -> *mut EC_POINT;
    fn EC_POINT_free(point: *mut EC_POINT);
    fn EC_POINT_point2oct(
        group: *const EC_GROUP,
        p: *const EC_POINT,
        form: point_conversion_form_t,
        buf: *mut ::core::ffi::c_uchar,
        len: size_t,
        ctx: *mut BN_CTX,
    ) -> size_t;
    fn EC_POINT_oct2point(
        group: *const EC_GROUP,
        p: *mut EC_POINT,
        buf: *const ::core::ffi::c_uchar,
        len: size_t,
        ctx: *mut BN_CTX,
    ) -> ::core::ffi::c_int;
    fn EC_KEY_new() -> *mut EC_KEY;
    fn EC_KEY_free(key: *mut EC_KEY);
    fn EC_KEY_get0_group(key: *const EC_KEY) -> *const EC_GROUP;
    fn EC_KEY_set_group(key: *mut EC_KEY, group: *const EC_GROUP) -> ::core::ffi::c_int;
    fn EC_KEY_get0_public_key(key: *const EC_KEY) -> *const EC_POINT;
    fn EC_KEY_generate_key(key: *mut EC_KEY) -> ::core::ffi::c_int;
    fn ECDH_compute_key(
        out: *mut ::core::ffi::c_void,
        outlen: size_t,
        pub_key: *const EC_POINT,
        ecdh: *const EC_KEY,
        KDF: Option<
            unsafe extern "C" fn(
                *const ::core::ffi::c_void,
                size_t,
                *mut ::core::ffi::c_void,
                *mut size_t,
            ) -> *mut ::core::ffi::c_void,
        >,
    ) -> ::core::ffi::c_int;
    fn EVP_MD_CTX_new() -> *mut EVP_MD_CTX;
    fn EVP_MD_CTX_free(ctx: *mut EVP_MD_CTX);
    fn EVP_EncryptInit_ex(
        ctx: *mut EVP_CIPHER_CTX,
        cipher: *const EVP_CIPHER,
        impl_0: *mut ENGINE,
        key: *const ::core::ffi::c_uchar,
        iv: *const ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    fn EVP_EncryptUpdate(
        ctx: *mut EVP_CIPHER_CTX,
        out: *mut ::core::ffi::c_uchar,
        outl: *mut ::core::ffi::c_int,
        in_0: *const ::core::ffi::c_uchar,
        inl: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn EVP_EncryptFinal_ex(
        ctx: *mut EVP_CIPHER_CTX,
        out: *mut ::core::ffi::c_uchar,
        outl: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn EVP_DecryptInit_ex(
        ctx: *mut EVP_CIPHER_CTX,
        cipher: *const EVP_CIPHER,
        impl_0: *mut ENGINE,
        key: *const ::core::ffi::c_uchar,
        iv: *const ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    fn EVP_DecryptUpdate(
        ctx: *mut EVP_CIPHER_CTX,
        out: *mut ::core::ffi::c_uchar,
        outl: *mut ::core::ffi::c_int,
        in_0: *const ::core::ffi::c_uchar,
        inl: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn EVP_DecryptFinal_ex(
        ctx: *mut EVP_CIPHER_CTX,
        outm: *mut ::core::ffi::c_uchar,
        outl: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn EVP_DigestSign(
        ctx: *mut EVP_MD_CTX,
        sigret: *mut ::core::ffi::c_uchar,
        siglen: *mut size_t,
        tbs: *const ::core::ffi::c_uchar,
        tbslen: size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_DigestVerify(
        ctx: *mut EVP_MD_CTX,
        sigret: *const ::core::ffi::c_uchar,
        siglen: size_t,
        tbs: *const ::core::ffi::c_uchar,
        tbslen: size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_DigestSignInit(
        ctx: *mut EVP_MD_CTX,
        pctx: *mut *mut EVP_PKEY_CTX,
        type_0: *const EVP_MD,
        e: *mut ENGINE,
        pkey: *mut EVP_PKEY,
    ) -> ::core::ffi::c_int;
    fn EVP_DigestSignUpdate(
        ctx: *mut EVP_MD_CTX,
        data: *const ::core::ffi::c_void,
        dsize: size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_DigestSignFinal(
        ctx: *mut EVP_MD_CTX,
        sigret: *mut ::core::ffi::c_uchar,
        siglen: *mut size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_DigestVerifyInit(
        ctx: *mut EVP_MD_CTX,
        pctx: *mut *mut EVP_PKEY_CTX,
        type_0: *const EVP_MD,
        e: *mut ENGINE,
        pkey: *mut EVP_PKEY,
    ) -> ::core::ffi::c_int;
    fn EVP_DigestVerifyUpdate(
        ctx: *mut EVP_MD_CTX,
        data: *const ::core::ffi::c_void,
        dsize: size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_DigestVerifyFinal(
        ctx: *mut EVP_MD_CTX,
        sig: *const ::core::ffi::c_uchar,
        siglen: size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_CIPHER_CTX_new() -> *mut EVP_CIPHER_CTX;
    fn EVP_CIPHER_CTX_free(c: *mut EVP_CIPHER_CTX);
    fn EVP_CIPHER_CTX_set_padding(
        c: *mut EVP_CIPHER_CTX,
        pad: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn EVP_CIPHER_CTX_ctrl(
        ctx: *mut EVP_CIPHER_CTX,
        type_0: ::core::ffi::c_int,
        arg: ::core::ffi::c_int,
        ptr: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn EVP_sha256() -> *const EVP_MD;
    fn EVP_sha384() -> *const EVP_MD;
    fn EVP_sha512() -> *const EVP_MD;
    fn EVP_bf_ecb() -> *const EVP_CIPHER;
    fn EVP_aes_128_ecb() -> *const EVP_CIPHER;
    fn EVP_aes_128_ctr() -> *const EVP_CIPHER;
    fn EVP_aes_128_gcm() -> *const EVP_CIPHER;
    fn EVP_aes_256_ecb() -> *const EVP_CIPHER;
    fn EVP_aes_256_ctr() -> *const EVP_CIPHER;
    fn EVP_aes_256_gcm() -> *const EVP_CIPHER;
    fn EVP_chacha20() -> *const EVP_CIPHER;
    fn EVP_chacha20_poly1305() -> *const EVP_CIPHER;
    fn EVP_MAC_fetch(
        libctx: *mut OSSL_LIB_CTX,
        algorithm: *const ::core::ffi::c_char,
        properties: *const ::core::ffi::c_char,
    ) -> *mut EVP_MAC;
    fn EVP_MAC_free(mac: *mut EVP_MAC);
    fn EVP_MAC_CTX_new(mac: *mut EVP_MAC) -> *mut EVP_MAC_CTX;
    fn EVP_MAC_CTX_free(ctx: *mut EVP_MAC_CTX);
    fn EVP_MAC_CTX_get_mac_size(ctx: *mut EVP_MAC_CTX) -> size_t;
    fn EVP_MAC_update(
        ctx: *mut EVP_MAC_CTX,
        data: *const ::core::ffi::c_uchar,
        datalen: size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_MAC_final(
        ctx: *mut EVP_MAC_CTX,
        out: *mut ::core::ffi::c_uchar,
        outl: *mut size_t,
        outsize: size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_PKEY_get_id(pkey: *const EVP_PKEY) -> ::core::ffi::c_int;
    fn EVP_PKEY_get1_EC_KEY(pkey: *mut EVP_PKEY) -> *mut ec_key_st;
    fn EVP_PKEY_new() -> *mut EVP_PKEY;
    fn EVP_PKEY_up_ref(pkey: *mut EVP_PKEY) -> ::core::ffi::c_int;
    fn EVP_PKEY_free(pkey: *mut EVP_PKEY);
    fn EVP_PKEY_copy_parameters(to: *mut EVP_PKEY, from: *const EVP_PKEY) -> ::core::ffi::c_int;
    fn EVP_PKEY_set1_encoded_public_key(
        pkey: *mut EVP_PKEY,
        pub_0: *const ::core::ffi::c_uchar,
        publen: size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_PKEY_get1_encoded_public_key(
        pkey: *mut EVP_PKEY,
        ppub: *mut *mut ::core::ffi::c_uchar,
    ) -> size_t;
    fn EVP_PKEY_CTX_new(pkey: *mut EVP_PKEY, e: *mut ENGINE) -> *mut EVP_PKEY_CTX;
    fn EVP_PKEY_CTX_new_id(id: ::core::ffi::c_int, e: *mut ENGINE) -> *mut EVP_PKEY_CTX;
    fn EVP_PKEY_CTX_free(ctx: *mut EVP_PKEY_CTX);
    fn EVP_PKEY_derive_init(ctx: *mut EVP_PKEY_CTX) -> ::core::ffi::c_int;
    fn EVP_PKEY_derive_set_peer(ctx: *mut EVP_PKEY_CTX, peer: *mut EVP_PKEY) -> ::core::ffi::c_int;
    fn EVP_PKEY_derive(
        ctx: *mut EVP_PKEY_CTX,
        key: *mut ::core::ffi::c_uchar,
        keylen: *mut size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_PKEY_keygen_init(ctx: *mut EVP_PKEY_CTX) -> ::core::ffi::c_int;
    fn EVP_PKEY_keygen(ctx: *mut EVP_PKEY_CTX, ppkey: *mut *mut EVP_PKEY) -> ::core::ffi::c_int;
    fn RAND_bytes(buf: *mut ::core::ffi::c_uchar, num: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn EVP_PKEY_CTX_set_rsa_padding(
        ctx: *mut EVP_PKEY_CTX,
        pad_mode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn EVP_PKEY_CTX_set_rsa_pss_saltlen(
        ctx: *mut EVP_PKEY_CTX,
        saltlen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn EVP_PKEY_CTX_set_rsa_mgf1_md(
        ctx: *mut EVP_PKEY_CTX,
        md: *const EVP_MD,
    ) -> ::core::ffi::c_int;
    fn SHA256_Init(c: *mut SHA256_CTX) -> ::core::ffi::c_int;
    fn SHA256_Update(
        c: *mut SHA256_CTX,
        data: *const ::core::ffi::c_void,
        len: size_t,
    ) -> ::core::ffi::c_int;
    fn SHA256_Final(md: *mut ::core::ffi::c_uchar, c: *mut SHA256_CTX) -> ::core::ffi::c_int;
    fn SHA384_Init(c: *mut SHA512_CTX) -> ::core::ffi::c_int;
    fn SHA384_Update(
        c: *mut SHA512_CTX,
        data: *const ::core::ffi::c_void,
        len: size_t,
    ) -> ::core::ffi::c_int;
    fn SHA384_Final(md: *mut ::core::ffi::c_uchar, c: *mut SHA512_CTX) -> ::core::ffi::c_int;
    fn SHA512_Init(c: *mut SHA512_CTX) -> ::core::ffi::c_int;
    fn SHA512_Update(
        c: *mut SHA512_CTX,
        data: *const ::core::ffi::c_void,
        len: size_t,
    ) -> ::core::ffi::c_int;
    fn SHA512_Final(md: *mut ::core::ffi::c_uchar, c: *mut SHA512_CTX) -> ::core::ffi::c_int;
    fn X509_verify_cert(ctx: *mut X509_STORE_CTX) -> ::core::ffi::c_int;
    fn X509_STORE_new() -> *mut X509_STORE;
    fn X509_STORE_free(v: *mut X509_STORE);
    fn X509_STORE_up_ref(v: *mut X509_STORE) -> ::core::ffi::c_int;
    fn X509_STORE_CTX_new() -> *mut X509_STORE_CTX;
    fn X509_STORE_CTX_free(ctx: *mut X509_STORE_CTX);
    fn X509_STORE_CTX_init(
        ctx: *mut X509_STORE_CTX,
        trust_store: *mut X509_STORE,
        target: *mut X509,
        untrusted: *mut stack_st_X509,
    ) -> ::core::ffi::c_int;
    fn X509_STORE_add_lookup(v: *mut X509_STORE, m: *mut X509_LOOKUP_METHOD) -> *mut X509_LOOKUP;
    fn X509_LOOKUP_hash_dir() -> *mut X509_LOOKUP_METHOD;
    fn X509_LOOKUP_file() -> *mut X509_LOOKUP_METHOD;
    fn X509_LOOKUP_ctrl(
        ctx: *mut X509_LOOKUP,
        cmd: ::core::ffi::c_int,
        argc: *const ::core::ffi::c_char,
        argl: ::core::ffi::c_long,
        ret: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn X509_STORE_CTX_get_error(ctx: *const X509_STORE_CTX) -> ::core::ffi::c_int;
    fn X509_STORE_CTX_get0_param(ctx: *const X509_STORE_CTX) -> *mut X509_VERIFY_PARAM;
    fn X509_VERIFY_PARAM_set_purpose(
        param: *mut X509_VERIFY_PARAM,
        purpose: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn X509_VERIFY_PARAM_set_depth(param: *mut X509_VERIFY_PARAM, depth: ::core::ffi::c_int);
    fn X509_VERIFY_PARAM_set1_host(
        param: *mut X509_VERIFY_PARAM,
        name: *const ::core::ffi::c_char,
        namelen: size_t,
    ) -> ::core::ffi::c_int;
    fn X509_VERIFY_PARAM_set_hostflags(param: *mut X509_VERIFY_PARAM, flags: ::core::ffi::c_uint);
    fn X509_VERIFY_PARAM_set1_ip_asc(
        param: *mut X509_VERIFY_PARAM,
        ipasc: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn i2d_PUBKEY(a: *const EVP_PKEY, out: *mut *mut ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    fn d2i_X509(
        a: *mut *mut X509,
        in_0: *mut *const ::core::ffi::c_uchar,
        len: ::core::ffi::c_long,
    ) -> *mut X509;
    fn i2d_X509(a: *const X509, out: *mut *mut ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    fn X509_free(a: *mut X509);
    fn X509_get_pubkey(x: *mut X509) -> *mut EVP_PKEY;
    fn ptls_buffer_reserve(buf: *mut ptls_buffer_t, delta: size_t) -> ::core::ffi::c_int;
    fn ptls_buffer__do_pushv(
        buf: *mut ptls_buffer_t,
        src: *const ::core::ffi::c_void,
        len: size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_is_server(tls: *mut ptls_t) -> ::core::ffi::c_int;
    fn ptls_aead__build_iv(
        algo: *const ptls_aead_algorithm_t,
        iv: *mut uint8_t,
        static_iv: *const uint8_t,
        seq: uint64_t,
    );
    static mut ptls_clear_memory:
        Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>;
    static mut ptls_mem_equal: Option<
        unsafe extern "C" fn(
            *const ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            size_t,
        ) -> ::core::ffi::c_int,
    >;
    fn ptls_server_name_is_ipaddr(name: *const ::core::ffi::c_char) -> ::core::ffi::c_int;
    fn HMAC_size(e: *const HMAC_CTX) -> size_t;
    fn HMAC_CTX_new() -> *mut HMAC_CTX;
    fn HMAC_CTX_free(ctx: *mut HMAC_CTX);
    fn HMAC_Update(
        ctx: *mut HMAC_CTX,
        data: *const ::core::ffi::c_uchar,
        len: size_t,
    ) -> ::core::ffi::c_int;
    fn HMAC_Final(
        ctx: *mut HMAC_CTX,
        md: *mut ::core::ffi::c_uchar,
        len: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn ASYNC_WAIT_CTX_new() -> *mut ASYNC_WAIT_CTX;
    fn ASYNC_WAIT_CTX_free(ctx: *mut ASYNC_WAIT_CTX);
    fn ASYNC_WAIT_CTX_get_all_fds(
        ctx: *mut ASYNC_WAIT_CTX,
        fd: *mut ::core::ffi::c_int,
        numfds: *mut size_t,
    ) -> ::core::ffi::c_int;
    fn ASYNC_start_job(
        job: *mut *mut ASYNC_JOB,
        ctx: *mut ASYNC_WAIT_CTX,
        ret: *mut ::core::ffi::c_int,
        func: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        args: *mut ::core::ffi::c_void,
        size: size_t,
    ) -> ::core::ffi::c_int;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
pub type __intptr_t = ::core::ffi::c_long;
pub type ssize_t = __ssize_t;
pub type size_t = usize;
pub type intptr_t = __intptr_t;
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
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type OPENSSL_STACK = stack_st;
pub type OPENSSL_sk_freefunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type BN_CTX = bignum_ctx;
pub type EVP_CIPHER = evp_cipher_st;
pub type EVP_CIPHER_CTX = evp_cipher_ctx_st;
pub type EVP_MD = evp_md_st;
pub type EVP_MD_CTX = evp_md_ctx_st;
pub type EVP_MAC = evp_mac_st;
pub type EVP_MAC_CTX = evp_mac_ctx_st;
pub type EVP_PKEY = evp_pkey_st;
pub type EVP_PKEY_CTX = evp_pkey_ctx_st;
pub type HMAC_CTX = hmac_ctx_st;
pub type EC_KEY = ec_key_st;
pub type X509 = x509_st;
pub type X509_STORE = x509_store_st;
pub type X509_STORE_CTX = x509_store_ctx_st;
pub type X509_LOOKUP = x509_lookup_st;
pub type X509_LOOKUP_METHOD = x509_lookup_method_st;
pub type X509_VERIFY_PARAM = X509_VERIFY_PARAM_st;
pub type OSSL_LIB_CTX = ossl_lib_ctx_st;
pub type ENGINE = engine_st;
pub type point_conversion_form_t = ::core::ffi::c_uint;
pub const POINT_CONVERSION_HYBRID: point_conversion_form_t = 6;
pub const POINT_CONVERSION_UNCOMPRESSED: point_conversion_form_t = 4;
pub const POINT_CONVERSION_COMPRESSED: point_conversion_form_t = 2;
pub type EC_GROUP = ec_group_st;
pub type EC_POINT = ec_point_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA256state_st {
    pub h: [::core::ffi::c_uint; 8],
    pub Nl: ::core::ffi::c_uint,
    pub Nh: ::core::ffi::c_uint,
    pub data: [::core::ffi::c_uint; 16],
    pub num: ::core::ffi::c_uint,
    pub md_len: ::core::ffi::c_uint,
}
pub type SHA256_CTX = SHA256state_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SHA512state_st {
    pub h: [::core::ffi::c_ulonglong; 8],
    pub Nl: ::core::ffi::c_ulonglong,
    pub Nh: ::core::ffi::c_ulonglong,
    pub u: C2Rust_Unnamed,
    pub num: ::core::ffi::c_uint,
    pub md_len: ::core::ffi::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed {
    pub d: [::core::ffi::c_ulonglong; 16],
    pub p: [::core::ffi::c_uchar; 128],
}
pub type SHA512_CTX = SHA512state_st;
pub type sk_X509_freefunc = Option<unsafe extern "C" fn(*mut X509) -> ()>;
pub type ptls_t = st_ptls_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_ptls_context_t {
    pub random_bytes: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> ()>,
    pub get_time: *mut ptls_get_time_t,
    pub key_exchanges: *mut *const ptls_key_exchange_algorithm_t,
    pub cipher_suites: *mut *const ptls_cipher_suite_t,
    pub certificates: C2Rust_Unnamed_12,
    pub pre_shared_key: C2Rust_Unnamed_11,
    pub ech: C2Rust_Unnamed_8,
    pub on_client_hello: *mut ptls_on_client_hello_t,
    pub emit_certificate: *mut ptls_emit_certificate_t,
    pub sign_certificate: *mut ptls_sign_certificate_t,
    pub verify_certificate: *mut ptls_verify_certificate_t,
    pub ticket_lifetime: uint32_t,
    pub max_early_data_size: uint32_t,
    pub max_buffer_size: size_t,
    pub hkdf_label_prefix__obsolete: *const ::core::ffi::c_char,
    #[bitfield(
        name = "require_dhe_on_psk",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    #[bitfield(name = "use_exporter", ty = "::core::ffi::c_uint", bits = "1..=1")]
    #[bitfield(
        name = "send_change_cipher_spec",
        ty = "::core::ffi::c_uint",
        bits = "2..=2"
    )]
    #[bitfield(
        name = "require_client_authentication",
        ty = "::core::ffi::c_uint",
        bits = "3..=3"
    )]
    #[bitfield(
        name = "omit_end_of_early_data",
        ty = "::core::ffi::c_uint",
        bits = "4..=4"
    )]
    #[bitfield(
        name = "use_raw_public_keys",
        ty = "::core::ffi::c_uint",
        bits = "5..=5"
    )]
    #[bitfield(
        name = "server_cipher_preference",
        ty = "::core::ffi::c_uint",
        bits = "6..=6"
    )]
    #[bitfield(
        name = "server_cipher_chacha_priority",
        ty = "::core::ffi::c_uint",
        bits = "7..=7"
    )]
    pub require_dhe_on_psk_use_exporter_send_change_cipher_spec_require_client_authentication_omit_end_of_early_data_use_raw_public_keys_server_cipher_preference_server_cipher_chacha_priority:
        [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
    pub encrypt_ticket: *mut ptls_encrypt_ticket_t,
    pub save_ticket: *mut ptls_save_ticket_t,
    pub log_event: *mut ptls_log_event_t,
    pub update_open_count: *mut ptls_update_open_count_t,
    pub update_traffic_key: *mut ptls_update_traffic_key_t,
    pub decompress_certificate: *mut ptls_decompress_certificate_t,
    pub on_extension: *mut ptls_on_extension_t,
    pub tls12_cipher_suites: *mut *const ptls_cipher_suite_t,
    pub ticket_context: C2Rust_Unnamed_1,
    pub client_ca_names: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_0 {
    pub list: *const ptls_iovec_t,
    pub count: size_t,
}
pub type ptls_iovec_t = st_ptls_iovec_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_iovec_t {
    pub base: *mut uint8_t,
    pub len: size_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub bytes: [uint8_t; 32],
    #[bitfield(name = "is_set", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub is_set: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
}
pub type ptls_cipher_suite_t = st_ptls_cipher_suite_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_cipher_suite_t {
    pub id: uint16_t,
    pub aead: *const ptls_aead_algorithm_t,
    pub hash: *const ptls_hash_algorithm_t,
    pub name: *const ::core::ffi::c_char,
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
    pub tls12: C2Rust_Unnamed_2,
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
pub struct C2Rust_Unnamed_2 {
    pub fixed_iv_size: size_t,
    pub record_iv_size: size_t,
}
pub type ptls_cipher_algorithm_t = st_ptls_cipher_algorithm_t;
pub type ptls_on_extension_t = st_ptls_on_extension_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_on_extension_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_on_extension_t,
            *mut ptls_t,
            uint8_t,
            uint16_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_decompress_certificate_t = st_ptls_decompress_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_decompress_certificate_t {
    pub supported_algorithms: *const uint16_t,
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_decompress_certificate_t,
            *mut ptls_t,
            uint16_t,
            ptls_iovec_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_update_traffic_key_t = st_ptls_update_traffic_key_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_update_traffic_key_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_update_traffic_key_t,
            *mut ptls_t,
            ::core::ffi::c_int,
            size_t,
            *const ::core::ffi::c_void,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_update_open_count_t = st_ptls_update_open_count_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_update_open_count_t {
    pub cb: Option<unsafe extern "C" fn(*mut st_ptls_update_open_count_t, ssize_t) -> ()>,
}
pub type ptls_log_event_t = st_ptls_log_event_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_log_event_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_log_event_t,
            *mut ptls_t,
            *const ::core::ffi::c_char,
            *const ::core::ffi::c_char,
            ...
        ) -> (),
    >,
}
pub type ptls_save_ticket_t = st_ptls_save_ticket_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_save_ticket_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_save_ticket_t,
            *mut ptls_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_encrypt_ticket_t = st_ptls_encrypt_ticket_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_encrypt_ticket_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_encrypt_ticket_t,
            *mut ptls_t,
            ::core::ffi::c_int,
            *mut ptls_buffer_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_buffer_t = st_ptls_buffer_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_buffer_t {
    pub base: *mut uint8_t,
    pub capacity: size_t,
    pub off: size_t,
    pub is_allocated: uint8_t,
    pub align_bits: uint8_t,
}
pub type ptls_verify_certificate_t = st_ptls_verify_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_verify_certificate_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_verify_certificate_t,
            *mut ptls_t,
            *const ::core::ffi::c_char,
            *mut Option<
                unsafe extern "C" fn(
                    *mut ::core::ffi::c_void,
                    uint16_t,
                    ptls_iovec_t,
                    ptls_iovec_t,
                ) -> ::core::ffi::c_int,
            >,
            *mut *mut ::core::ffi::c_void,
            *mut ptls_iovec_t,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
    pub algos: *const uint16_t,
}
pub type ptls_sign_certificate_t = st_ptls_sign_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_sign_certificate_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_sign_certificate_t,
            *mut ptls_t,
            *mut *mut ptls_async_job_t,
            *mut uint16_t,
            *mut ptls_buffer_t,
            ptls_iovec_t,
            *const uint16_t,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_async_job_t = st_ptls_async_job_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_async_job_t {
    pub destroy_: Option<unsafe extern "C" fn(*mut st_ptls_async_job_t) -> ()>,
    pub get_fd: Option<unsafe extern "C" fn(*mut st_ptls_async_job_t) -> ::core::ffi::c_int>,
    pub set_completion_callback: Option<
        unsafe extern "C" fn(
            *mut st_ptls_async_job_t,
            Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
            *mut ::core::ffi::c_void,
        ) -> (),
    >,
}
pub type ptls_emit_certificate_t = st_ptls_emit_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_emit_certificate_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_emit_certificate_t,
            *mut ptls_t,
            *mut ptls_message_emitter_t,
            *mut ptls_key_schedule_t,
            ptls_iovec_t,
            ::core::ffi::c_int,
            *const uint16_t,
            size_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_key_schedule_t = st_ptls_key_schedule_t;
pub type ptls_message_emitter_t = st_ptls_message_emitter_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_message_emitter_t {
    pub buf: *mut ptls_buffer_t,
    pub enc: *mut st_ptls_traffic_protection_t,
    pub record_header_length: size_t,
    pub begin_message:
        Option<unsafe extern "C" fn(*mut st_ptls_message_emitter_t) -> ::core::ffi::c_int>,
    pub commit_message:
        Option<unsafe extern "C" fn(*mut st_ptls_message_emitter_t) -> ::core::ffi::c_int>,
}
pub type ptls_on_client_hello_t = st_ptls_on_client_hello_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_on_client_hello_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_on_client_hello_t,
            *mut ptls_t,
            *mut ptls_on_client_hello_parameters_t,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_on_client_hello_parameters_t = st_ptls_on_client_hello_parameters_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_ptls_on_client_hello_parameters_t {
    pub server_name: ptls_iovec_t,
    pub raw_message: ptls_iovec_t,
    pub cipher_suites: ptls_iovec_t,
    pub negotiated_protocols: C2Rust_Unnamed_7,
    pub signature_algorithms: C2Rust_Unnamed_6,
    pub certificate_compression_algorithms: C2Rust_Unnamed_5,
    pub server_certificate_types: C2Rust_Unnamed_4,
    pub psk_identities: C2Rust_Unnamed_3,
    #[bitfield(
        name = "incompatible_version",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    pub incompatible_version: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub list: *const ptls_client_hello_psk_identity_t,
    pub count: size_t,
}
pub type ptls_client_hello_psk_identity_t = st_ptls_client_hello_psk_identity_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_client_hello_psk_identity_t {
    pub identity: ptls_iovec_t,
    pub obfuscated_ticket_age: uint32_t,
    pub binder: ptls_iovec_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_4 {
    pub list: *const uint8_t,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_5 {
    pub list: *const uint16_t,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_6 {
    pub list: *const uint16_t,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_7 {
    pub list: *mut ptls_iovec_t,
    pub count: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_8 {
    pub client: C2Rust_Unnamed_10,
    pub server: C2Rust_Unnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_9 {
    pub create_opener: *mut ptls_ech_create_opener_t,
    pub retry_configs: ptls_iovec_t,
}
pub type ptls_ech_create_opener_t = st_ptls_ech_create_opener_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_ech_create_opener_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_ech_create_opener_t,
            *mut *const ptls_hpke_kem_t,
            *mut *const ptls_hpke_cipher_suite_t,
            *mut ptls_t,
            uint8_t,
            ptls_hpke_cipher_suite_id_t,
            ptls_iovec_t,
            ptls_iovec_t,
        ) -> *mut ptls_aead_context_t,
    >,
}
pub type ptls_hpke_cipher_suite_id_t = st_ptls_hpke_cipher_suite_id_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_hpke_cipher_suite_id_t {
    pub kdf: uint16_t,
    pub aead: uint16_t,
}
pub type ptls_hpke_cipher_suite_t = st_ptls_hpke_cipher_suite_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_hpke_cipher_suite_t {
    pub id: ptls_hpke_cipher_suite_id_t,
    pub name: *const ::core::ffi::c_char,
    pub hash: *const ptls_hash_algorithm_t,
    pub aead: *const ptls_aead_algorithm_t,
}
pub type ptls_hpke_kem_t = st_ptls_hpke_kem_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_hpke_kem_t {
    pub id: uint16_t,
    pub keyex: *const ptls_key_exchange_algorithm_t,
    pub hash: *const ptls_hash_algorithm_t,
}
pub type ptls_key_exchange_algorithm_t = st_ptls_key_exchange_algorithm_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_key_exchange_algorithm_t {
    pub id: uint16_t,
    pub create: Option<
        unsafe extern "C" fn(
            *const st_ptls_key_exchange_algorithm_t,
            *mut *mut ptls_key_exchange_context_t,
        ) -> ::core::ffi::c_int,
    >,
    pub exchange: Option<
        unsafe extern "C" fn(
            *const st_ptls_key_exchange_algorithm_t,
            *mut ptls_iovec_t,
            *mut ptls_iovec_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
    pub data: intptr_t,
    pub name: *const ::core::ffi::c_char,
}
pub type ptls_key_exchange_context_t = st_ptls_key_exchange_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_key_exchange_context_t {
    pub algo: *const st_ptls_key_exchange_algorithm_t,
    pub pubkey: ptls_iovec_t,
    pub on_exchange: Option<
        unsafe extern "C" fn(
            *mut *mut st_ptls_key_exchange_context_t,
            ::core::ffi::c_int,
            *mut ptls_iovec_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_10 {
    pub ciphers: *mut *const ptls_hpke_cipher_suite_t,
    pub kems: *mut *const ptls_hpke_kem_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_11 {
    pub identity: ptls_iovec_t,
    pub secret: ptls_iovec_t,
    pub hash: *const ptls_hash_algorithm_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_12 {
    pub list: *mut ptls_iovec_t,
    pub count: size_t,
}
pub type ptls_get_time_t = st_ptls_get_time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_get_time_t {
    pub cb: Option<unsafe extern "C" fn(*mut st_ptls_get_time_t) -> uint64_t>,
}
pub type ptls_context_t = st_ptls_context_t;
pub type ASYNC_JOB = async_job_st;
pub type ASYNC_WAIT_CTX = async_wait_ctx_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_x9_62_keyex_context_t {
    pub super_0: ptls_key_exchange_context_t,
    pub bn_ctx: *mut BN_CTX,
    pub privkey: *mut EC_KEY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_evp_keyex_context_t {
    pub super_0: ptls_key_exchange_context_t,
    pub privkey: *mut EVP_PKEY,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cipher_context_t {
    pub super_0: ptls_cipher_context_t,
    pub evp: *mut EVP_CIPHER_CTX,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aead_crypto_context_t {
    pub super_0: ptls_aead_context_t,
    pub evp_ctx: *mut EVP_CIPHER_CTX,
    pub static_iv: [uint8_t; 32],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha256_context_t {
    pub super_0: ptls_hash_context_t,
    pub ctx: SHA256_CTX,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha384_context_t {
    pub super_0: ptls_hash_context_t,
    pub ctx: SHA512_CTX,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sha512_context_t {
    pub super_0: ptls_hash_context_t,
    pub ctx: SHA512_CTX,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_openssl_signature_scheme_t {
    pub scheme_id: uint16_t,
    pub scheme_md: Option<unsafe extern "C" fn() -> *const EVP_MD>,
}
pub type ptls_openssl_signature_scheme_t = st_ptls_openssl_signature_scheme_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct st_ptls_openssl_sign_certificate_t {
    pub super_0: ptls_sign_certificate_t,
    pub key: *mut EVP_PKEY,
    pub schemes: *const ptls_openssl_signature_scheme_t,
    #[bitfield(name = "async_0", ty = "::core::ffi::c_uint", bits = "0..=0")]
    pub async_0: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
pub type ptls_openssl_sign_certificate_t = st_ptls_openssl_sign_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct async_sign_ctx {
    pub super_0: ptls_async_job_t,
    pub scheme: *const ptls_openssl_signature_scheme_t,
    pub ctx: *mut EVP_MD_CTX,
    pub waitctx: *mut ASYNC_WAIT_CTX,
    pub job: *mut ASYNC_JOB,
    pub siglen: size_t,
    pub sig: [uint8_t; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_openssl_raw_pubkey_verify_certificate_t {
    pub super_0: ptls_verify_certificate_t,
    pub expected_pubkey: *mut EVP_PKEY,
}
pub type ptls_openssl_raw_pubkey_verify_certificate_t =
    st_ptls_openssl_raw_pubkey_verify_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_openssl_override_verify_certificate_t {
    pub cb: Option<
        unsafe extern "C" fn(
            *mut st_ptls_openssl_override_verify_certificate_t,
            *mut ptls_t,
            ::core::ffi::c_int,
            ::core::ffi::c_int,
            *mut X509,
            *mut stack_st_X509,
        ) -> ::core::ffi::c_int,
    >,
}
pub type ptls_openssl_override_verify_certificate_t = st_ptls_openssl_override_verify_certificate_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_openssl_verify_certificate_t {
    pub super_0: ptls_verify_certificate_t,
    pub cert_store: *mut X509_STORE,
    pub override_callback: *mut ptls_openssl_override_verify_certificate_t,
}
pub type ptls_openssl_verify_certificate_t = st_ptls_openssl_verify_certificate_t;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NID_X9_62_id_ecPublicKey: ::core::ffi::c_int = 408;
pub const NID_X9_62_prime256v1: ::core::ffi::c_int = 415 as ::core::ffi::c_int;
pub const NID_secp384r1: ::core::ffi::c_int = 715 as ::core::ffi::c_int;
pub const NID_secp521r1: ::core::ffi::c_int = 716 as ::core::ffi::c_int;
pub const NID_rsaEncryption: ::core::ffi::c_int = 6;
pub const NID_X25519: ::core::ffi::c_int = 1034 as ::core::ffi::c_int;
pub const NID_ED25519: ::core::ffi::c_int = 1087;
pub const EVP_MAX_MD_SIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const EVP_MAX_IV_LENGTH: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const EVP_MAX_BLOCK_LENGTH: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const EVP_PKEY_RSA: ::core::ffi::c_int = NID_rsaEncryption;
pub const EVP_PKEY_EC: ::core::ffi::c_int = NID_X9_62_id_ecPublicKey;
pub const EVP_PKEY_ED25519: ::core::ffi::c_int = NID_ED25519;
pub const EVP_CTRL_AEAD_SET_IVLEN: ::core::ffi::c_int = 0x9 as ::core::ffi::c_int;
pub const EVP_CTRL_AEAD_GET_TAG: ::core::ffi::c_int = 0x10 as ::core::ffi::c_int;
pub const EVP_CTRL_AEAD_SET_TAG: ::core::ffi::c_int = 0x11 as ::core::ffi::c_int;
pub const EVP_CTRL_GCM_SET_IVLEN: ::core::ffi::c_int = EVP_CTRL_AEAD_SET_IVLEN;
pub const EVP_CTRL_GCM_GET_TAG: ::core::ffi::c_int = EVP_CTRL_AEAD_GET_TAG;
pub const EVP_CTRL_GCM_SET_TAG: ::core::ffi::c_int = EVP_CTRL_AEAD_SET_TAG;
pub const RSA_PKCS1_PSS_PADDING: ::core::ffi::c_int = 6 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn ossl_check_X509_type(mut ptr: *mut X509) -> *mut X509 {
    return ptr;
}
#[inline]
unsafe extern "C" fn ossl_check_const_X509_sk_type(
    mut sk: *const stack_st_X509,
) -> *const OPENSSL_STACK {
    return sk as *const OPENSSL_STACK;
}
#[inline]
unsafe extern "C" fn ossl_check_X509_freefunc_type(
    mut fr: sk_X509_freefunc,
) -> OPENSSL_sk_freefunc {
    return ::core::mem::transmute::<sk_X509_freefunc, OPENSSL_sk_freefunc>(fr);
}
#[inline]
unsafe extern "C" fn ossl_check_X509_sk_type(mut sk: *mut stack_st_X509) -> *mut OPENSSL_STACK {
    return sk as *mut OPENSSL_STACK;
}
pub const X509_L_FILE_LOAD: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const X509_L_ADD_DIR: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const X509_V_ERR_UNABLE_TO_GET_ISSUER_CERT: ::core::ffi::c_int = 2;
pub const X509_V_ERR_CERT_NOT_YET_VALID: ::core::ffi::c_int = 9;
pub const X509_V_ERR_CERT_HAS_EXPIRED: ::core::ffi::c_int = 10;
pub const X509_V_ERR_OUT_OF_MEM: ::core::ffi::c_int = 17;
pub const X509_V_ERR_UNABLE_TO_GET_ISSUER_CERT_LOCALLY: ::core::ffi::c_int = 20;
pub const X509_V_ERR_CERT_REVOKED: ::core::ffi::c_int = 23;
pub const X509_V_ERR_CERT_UNTRUSTED: ::core::ffi::c_int = 27;
pub const X509_V_ERR_CERT_REJECTED: ::core::ffi::c_int = 28;
pub const X509_V_ERR_HOSTNAME_MISMATCH: ::core::ffi::c_int = 62;
pub const X509_V_ERR_INVALID_CA: ::core::ffi::c_int = 79;
pub const X509_PURPOSE_SSL_CLIENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const X509_PURPOSE_SSL_SERVER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const X509_CHECK_FLAG_NO_PARTIAL_WILDCARDS: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
pub const PTLS_AES128_KEY_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_AES256_KEY_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const PTLS_AES_BLOCK_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_AES_IV_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_AESGCM_IV_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const PTLS_AESGCM_TAG_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_AESGCM_CONFIDENTIALITY_LIMIT: ::core::ffi::c_int = 0x2000000 as ::core::ffi::c_int;
pub const PTLS_CHACHA20_KEY_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const PTLS_CHACHA20_IV_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_CHACHA20POLY1305_IV_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const PTLS_CHACHA20POLY1305_TAG_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_CHACHA20POLY1305_CONFIDENTIALITY_LIMIT: ::core::ffi::c_ulong = UINT64_MAX;
pub const PTLS_BLOWFISH_KEY_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_BLOWFISH_BLOCK_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PTLS_SHA256_BLOCK_SIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const PTLS_SHA256_DIGEST_SIZE: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const PTLS_SHA384_BLOCK_SIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const PTLS_SHA384_DIGEST_SIZE: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
pub const PTLS_SHA512_BLOCK_SIZE: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const PTLS_SHA512_DIGEST_SIZE: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const PTLS_CIPHER_SUITE_AES_128_GCM_SHA256: ::core::ffi::c_int = 0x1301 as ::core::ffi::c_int;
pub const PTLS_CIPHER_SUITE_NAME_AES_128_GCM_SHA256: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"TLS_AES_128_GCM_SHA256\0")
};
pub const PTLS_CIPHER_SUITE_AES_256_GCM_SHA384: ::core::ffi::c_int = 0x1302 as ::core::ffi::c_int;
pub const PTLS_CIPHER_SUITE_NAME_AES_256_GCM_SHA384: [::core::ffi::c_char; 23] = unsafe {
    ::core::mem::transmute::<[u8; 23], [::core::ffi::c_char; 23]>(*b"TLS_AES_256_GCM_SHA384\0")
};
pub const PTLS_CIPHER_SUITE_CHACHA20_POLY1305_SHA256: ::core::ffi::c_int =
    0x1303 as ::core::ffi::c_int;
pub const PTLS_CIPHER_SUITE_NAME_CHACHA20_POLY1305_SHA256: [::core::ffi::c_char; 29] = unsafe {
    ::core::mem::transmute::<[u8; 29], [::core::ffi::c_char; 29]>(
        *b"TLS_CHACHA20_POLY1305_SHA256\0",
    )
};
pub const PTLS_CIPHER_SUITE_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256: ::core::ffi::c_int =
    0xc02b as ::core::ffi::c_int;
pub const PTLS_CIPHER_SUITE_NAME_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256: [::core::ffi::c_char; 30] = unsafe {
    ::core::mem::transmute::<[u8; 30], [::core::ffi::c_char; 30]>(
        *b"ECDHE-ECDSA-AES128-GCM-SHA256\0",
    )
};
pub const PTLS_CIPHER_SUITE_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384: ::core::ffi::c_int =
    0xc02c as ::core::ffi::c_int;
pub const PTLS_CIPHER_SUITE_NAME_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384: [::core::ffi::c_char; 30] = unsafe {
    ::core::mem::transmute::<[u8; 30], [::core::ffi::c_char; 30]>(
        *b"ECDHE-ECDSA-AES256-GCM-SHA384\0",
    )
};
pub const PTLS_CIPHER_SUITE_ECDHE_RSA_WITH_AES_128_GCM_SHA256: ::core::ffi::c_int =
    0xc02f as ::core::ffi::c_int;
pub const PTLS_CIPHER_SUITE_NAME_ECDHE_RSA_WITH_AES_128_GCM_SHA256: [::core::ffi::c_char; 28] = unsafe {
    ::core::mem::transmute::<[u8; 28], [::core::ffi::c_char; 28]>(*b"ECDHE-RSA-AES128-GCM-SHA256\0")
};
pub const PTLS_CIPHER_SUITE_ECDHE_RSA_WITH_AES_256_GCM_SHA384: ::core::ffi::c_int =
    0xc030 as ::core::ffi::c_int;
pub const PTLS_CIPHER_SUITE_NAME_ECDHE_RSA_WITH_AES_256_GCM_SHA384: [::core::ffi::c_char; 28] = unsafe {
    ::core::mem::transmute::<[u8; 28], [::core::ffi::c_char; 28]>(*b"ECDHE-RSA-AES256-GCM-SHA384\0")
};
pub const PTLS_CIPHER_SUITE_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256: ::core::ffi::c_int =
    0xcca8 as ::core::ffi::c_int;
pub const PTLS_CIPHER_SUITE_NAME_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256: [::core::ffi::c_char;
    28] = unsafe {
    ::core::mem::transmute::<[u8; 28], [::core::ffi::c_char; 28]>(*b"ECDHE-RSA-CHACHA20-POLY1305\0")
};
pub const PTLS_CIPHER_SUITE_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256: ::core::ffi::c_int =
    0xcca9 as ::core::ffi::c_int;
pub const PTLS_CIPHER_SUITE_NAME_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256: [::core::ffi::c_char;
    30] = unsafe {
    ::core::mem::transmute::<[u8; 30], [::core::ffi::c_char; 30]>(
        *b"ECDHE-ECDSA-CHACHA20-POLY1305\0",
    )
};
pub const PTLS_GROUP_SECP256R1: ::core::ffi::c_int = 23 as ::core::ffi::c_int;
pub const PTLS_GROUP_NAME_SECP256R1: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"secp256r1\0") };
pub const PTLS_GROUP_SECP384R1: ::core::ffi::c_int = 24 as ::core::ffi::c_int;
pub const PTLS_GROUP_NAME_SECP384R1: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"secp384r1\0") };
pub const PTLS_GROUP_SECP521R1: ::core::ffi::c_int = 25 as ::core::ffi::c_int;
pub const PTLS_GROUP_NAME_SECP521R1: [::core::ffi::c_char; 10] =
    unsafe { ::core::mem::transmute::<[u8; 10], [::core::ffi::c_char; 10]>(*b"secp521r1\0") };
pub const PTLS_GROUP_X25519: ::core::ffi::c_int = 29 as ::core::ffi::c_int;
pub const PTLS_GROUP_NAME_X25519: [::core::ffi::c_char; 7] =
    unsafe { ::core::mem::transmute::<[u8; 7], [::core::ffi::c_char; 7]>(*b"x25519\0") };
pub const PTLS_SIGNATURE_ECDSA_SECP256R1_SHA256: ::core::ffi::c_int = 0x403 as ::core::ffi::c_int;
pub const PTLS_SIGNATURE_ECDSA_SECP384R1_SHA384: ::core::ffi::c_int = 0x503 as ::core::ffi::c_int;
pub const PTLS_SIGNATURE_ECDSA_SECP521R1_SHA512: ::core::ffi::c_int = 0x603 as ::core::ffi::c_int;
pub const PTLS_SIGNATURE_RSA_PSS_RSAE_SHA256: ::core::ffi::c_int = 0x804 as ::core::ffi::c_int;
pub const PTLS_SIGNATURE_RSA_PSS_RSAE_SHA384: ::core::ffi::c_int = 0x805 as ::core::ffi::c_int;
pub const PTLS_SIGNATURE_RSA_PSS_RSAE_SHA512: ::core::ffi::c_int = 0x806 as ::core::ffi::c_int;
pub const PTLS_SIGNATURE_ED25519: ::core::ffi::c_int = 0x807 as ::core::ffi::c_int;
pub const PTLS_HPKE_KEM_P256_SHA256: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const PTLS_HPKE_KEM_P384_SHA384: ::core::ffi::c_int = 17 as ::core::ffi::c_int;
pub const PTLS_HPKE_KEM_X25519_SHA256: ::core::ffi::c_int = 32 as ::core::ffi::c_int;
pub const PTLS_HPKE_HKDF_SHA256: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PTLS_HPKE_HKDF_SHA384: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PTLS_HPKE_HKDF_SHA512: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PTLS_HPKE_AEAD_AES_128_GCM: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const PTLS_HPKE_AEAD_AES_256_GCM: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PTLS_HPKE_AEAD_CHACHA20POLY1305: ::core::ffi::c_int = 3 as ::core::ffi::c_int;
pub const PTLS_ERROR_CLASS_INTERNAL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const PTLS_ALERT_HANDSHAKE_FAILURE: ::core::ffi::c_int = 40 as ::core::ffi::c_int;
pub const PTLS_ALERT_BAD_CERTIFICATE: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
pub const PTLS_ALERT_CERTIFICATE_REVOKED: ::core::ffi::c_int = 44 as ::core::ffi::c_int;
pub const PTLS_ALERT_CERTIFICATE_EXPIRED: ::core::ffi::c_int = 45 as ::core::ffi::c_int;
pub const PTLS_ALERT_CERTIFICATE_UNKNOWN: ::core::ffi::c_int = 46 as ::core::ffi::c_int;
pub const PTLS_ALERT_ILLEGAL_PARAMETER: ::core::ffi::c_int = 47 as ::core::ffi::c_int;
pub const PTLS_ALERT_UNKNOWN_CA: ::core::ffi::c_int = 48 as ::core::ffi::c_int;
pub const PTLS_ALERT_DECODE_ERROR: ::core::ffi::c_int = 50 as ::core::ffi::c_int;
pub const PTLS_ALERT_DECRYPT_ERROR: ::core::ffi::c_int = 51 as ::core::ffi::c_int;
pub const PTLS_ALERT_CERTIFICATE_REQUIRED: ::core::ffi::c_int = 116 as ::core::ffi::c_int;
pub const PTLS_TLS12_AESGCM_FIXED_IV_SIZE: ::core::ffi::c_int = 4 as ::core::ffi::c_int;
pub const PTLS_TLS12_AESGCM_RECORD_IV_SIZE: ::core::ffi::c_int = 8 as ::core::ffi::c_int;
pub const PTLS_TLS12_CHACHAPOLY_FIXED_IV_SIZE: ::core::ffi::c_int = 12 as ::core::ffi::c_int;
pub const PTLS_TLS12_CHACHAPOLY_RECORD_IV_SIZE: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
pub const PTLS_ERROR_NO_MEMORY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 1 as ::core::ffi::c_int;
pub const PTLS_ERROR_LIBRARY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 3 as ::core::ffi::c_int;
pub const PTLS_ERROR_INCOMPATIBLE_KEY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 4 as ::core::ffi::c_int;
pub const PTLS_ERROR_ASYNC_OPERATION: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 11 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn ptls_iovec_init(
    mut p: *const ::core::ffi::c_void,
    mut len: size_t,
) -> ptls_iovec_t {
    let mut r: ptls_iovec_t = st_ptls_iovec_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    };
    r.base = p as *mut uint8_t;
    r.len = len;
    return r;
}
#[inline]
unsafe extern "C" fn ptls_cipher_init(
    mut ctx: *mut ptls_cipher_context_t,
    mut iv: *const ::core::ffi::c_void,
) {
    (*ctx).do_init.expect("non-null function pointer")(ctx as *mut st_ptls_cipher_context_t, iv);
}
#[inline]
unsafe extern "C" fn ptls_cipher_encrypt(
    mut ctx: *mut ptls_cipher_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    (*ctx).do_transform.expect("non-null function pointer")(
        ctx as *mut st_ptls_cipher_context_t,
        output,
        input,
        len,
    );
}
#[inline]
unsafe extern "C" fn ptls_aead__do_encrypt(
    mut ctx: *mut ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
    mut supp: *mut ptls_aead_supplementary_encryption_t,
) {
    let mut invec: ptls_iovec_t = ptls_iovec_init(input, inlen);
    (*ctx).do_encrypt_v.expect("non-null function pointer")(
        ctx as *mut st_ptls_aead_context_t,
        output,
        &raw mut invec,
        1 as size_t,
        seq,
        aad,
        aadlen,
    );
    if !supp.is_null() {
        ptls_cipher_init((*supp).ctx, (*supp).input);
        memset(
            &raw mut (*supp).output as *mut uint8_t as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
        );
        ptls_cipher_encrypt(
            (*supp).ctx,
            &raw mut (*supp).output as *mut uint8_t as *mut ::core::ffi::c_void,
            &raw mut (*supp).output as *mut uint8_t as *const ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
        );
    }
}
#[inline]
unsafe extern "C" fn ptls_aead__do_encrypt_v(
    mut ctx: *mut ptls_aead_context_t,
    mut _output: *mut ::core::ffi::c_void,
    mut input: *mut ptls_iovec_t,
    mut incnt: size_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) {
    let mut output: *mut uint8_t = _output as *mut uint8_t;
    (*ctx).do_encrypt_init.expect("non-null function pointer")(
        ctx as *mut st_ptls_aead_context_t,
        seq,
        aad,
        aadlen,
    );
    let mut i: size_t = 0 as size_t;
    while i < incnt {
        output = output.offset(
            (*ctx).do_encrypt_update.expect("non-null function pointer")(
                ctx as *mut st_ptls_aead_context_t,
                output as *mut ::core::ffi::c_void,
                (*input.offset(i as isize)).base as *const ::core::ffi::c_void,
                (*input.offset(i as isize)).len,
            ) as isize,
        );
        i = i.wrapping_add(1);
    }
    (*ctx).do_encrypt_final.expect("non-null function pointer")(
        ctx as *mut st_ptls_aead_context_t,
        output as *mut ::core::ffi::c_void,
    );
}
#[inline]
unsafe extern "C" fn ptls_hash_clone_memcpy(
    mut dst: *mut ::core::ffi::c_void,
    mut src: *const ::core::ffi::c_void,
    mut size: size_t,
) {
    memcpy(dst, src, size);
}
pub const ASYNC_ERR: ::core::ffi::c_int = 0;
pub const ASYNC_NO_JOBS: ::core::ffi::c_int = 1;
pub const ASYNC_PAUSE: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const ASYNC_FINISH: ::core::ffi::c_int = 3;
static mut rsa_signature_schemes: [ptls_openssl_signature_scheme_t; 4] = unsafe {
    [
        st_ptls_openssl_signature_scheme_t {
            scheme_id: PTLS_SIGNATURE_RSA_PSS_RSAE_SHA256 as uint16_t,
            scheme_md: Some(EVP_sha256 as unsafe extern "C" fn() -> *const EVP_MD),
        },
        st_ptls_openssl_signature_scheme_t {
            scheme_id: PTLS_SIGNATURE_RSA_PSS_RSAE_SHA384 as uint16_t,
            scheme_md: Some(EVP_sha384 as unsafe extern "C" fn() -> *const EVP_MD),
        },
        st_ptls_openssl_signature_scheme_t {
            scheme_id: PTLS_SIGNATURE_RSA_PSS_RSAE_SHA512 as uint16_t,
            scheme_md: Some(EVP_sha512 as unsafe extern "C" fn() -> *const EVP_MD),
        },
        st_ptls_openssl_signature_scheme_t {
            scheme_id: UINT16_MAX as uint16_t,
            scheme_md: None,
        },
    ]
};
static mut secp256r1_signature_schemes: [ptls_openssl_signature_scheme_t; 2] = unsafe {
    [
        st_ptls_openssl_signature_scheme_t {
            scheme_id: PTLS_SIGNATURE_ECDSA_SECP256R1_SHA256 as uint16_t,
            scheme_md: Some(EVP_sha256 as unsafe extern "C" fn() -> *const EVP_MD),
        },
        st_ptls_openssl_signature_scheme_t {
            scheme_id: UINT16_MAX as uint16_t,
            scheme_md: None,
        },
    ]
};
static mut secp384r1_signature_schemes: [ptls_openssl_signature_scheme_t; 2] = unsafe {
    [
        st_ptls_openssl_signature_scheme_t {
            scheme_id: PTLS_SIGNATURE_ECDSA_SECP384R1_SHA384 as uint16_t,
            scheme_md: Some(EVP_sha384 as unsafe extern "C" fn() -> *const EVP_MD),
        },
        st_ptls_openssl_signature_scheme_t {
            scheme_id: UINT16_MAX as uint16_t,
            scheme_md: None,
        },
    ]
};
static mut secp521r1_signature_schemes: [ptls_openssl_signature_scheme_t; 2] = unsafe {
    [
        st_ptls_openssl_signature_scheme_t {
            scheme_id: PTLS_SIGNATURE_ECDSA_SECP521R1_SHA512 as uint16_t,
            scheme_md: Some(EVP_sha512 as unsafe extern "C" fn() -> *const EVP_MD),
        },
        st_ptls_openssl_signature_scheme_t {
            scheme_id: UINT16_MAX as uint16_t,
            scheme_md: None,
        },
    ]
};
static mut ed25519_signature_schemes: [ptls_openssl_signature_scheme_t; 2] = [
    st_ptls_openssl_signature_scheme_t {
        scheme_id: PTLS_SIGNATURE_ED25519 as uint16_t,
        scheme_md: None,
    },
    st_ptls_openssl_signature_scheme_t {
        scheme_id: UINT16_MAX as uint16_t,
        scheme_md: None,
    },
];
static mut default_signature_schemes: [uint16_t; 8] = [
    PTLS_SIGNATURE_ED25519 as uint16_t,
    PTLS_SIGNATURE_ECDSA_SECP256R1_SHA256 as uint16_t,
    PTLS_SIGNATURE_ECDSA_SECP384R1_SHA384 as uint16_t,
    PTLS_SIGNATURE_ECDSA_SECP521R1_SHA512 as uint16_t,
    PTLS_SIGNATURE_RSA_PSS_RSAE_SHA512 as uint16_t,
    PTLS_SIGNATURE_RSA_PSS_RSAE_SHA384 as uint16_t,
    PTLS_SIGNATURE_RSA_PSS_RSAE_SHA256 as uint16_t,
    UINT16_MAX as uint16_t,
];
#[no_mangle]
pub unsafe extern "C" fn ptls_openssl_lookup_signature_schemes(
    mut key: *mut EVP_PKEY,
) -> *const ptls_openssl_signature_scheme_t {
    let mut schemes: *const ptls_openssl_signature_scheme_t =
        ::core::ptr::null::<ptls_openssl_signature_scheme_t>();
    match EVP_PKEY_get_id(key) {
        EVP_PKEY_RSA => {
            schemes = &raw const rsa_signature_schemes as *const ptls_openssl_signature_scheme_t;
        }
        EVP_PKEY_EC => {
            let mut eckey: *mut EC_KEY = EVP_PKEY_get1_EC_KEY(key) as *mut EC_KEY;
            match EC_GROUP_get_curve_name(EC_KEY_get0_group(eckey)) {
                NID_X9_62_prime256v1 => {
                    schemes = &raw const secp256r1_signature_schemes
                        as *const ptls_openssl_signature_scheme_t;
                }
                NID_secp384r1 => {
                    schemes = &raw const secp384r1_signature_schemes
                        as *const ptls_openssl_signature_scheme_t;
                }
                NID_secp521r1 => {
                    schemes = &raw const secp521r1_signature_schemes
                        as *const ptls_openssl_signature_scheme_t;
                }
                _ => {}
            }
            EC_KEY_free(eckey);
        }
        EVP_PKEY_ED25519 => {
            schemes =
                &raw const ed25519_signature_schemes as *const ptls_openssl_signature_scheme_t;
        }
        _ => {}
    }
    return schemes;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_openssl_select_signature_scheme(
    mut available: *const ptls_openssl_signature_scheme_t,
    mut algorithms: *const uint16_t,
    mut num_algorithms: size_t,
) -> *const ptls_openssl_signature_scheme_t {
    let mut scheme: *const ptls_openssl_signature_scheme_t =
        ::core::ptr::null::<ptls_openssl_signature_scheme_t>();
    scheme = available;
    while (*scheme).scheme_id as ::core::ffi::c_int != UINT16_MAX {
        let mut i: size_t = 0 as size_t;
        while i != num_algorithms {
            if *algorithms.offset(i as isize) as ::core::ffi::c_int
                == (*scheme).scheme_id as ::core::ffi::c_int
            {
                return scheme;
            }
            i = i.wrapping_add(1);
        }
        scheme = scheme.offset(1);
    }
    return ::core::ptr::null::<ptls_openssl_signature_scheme_t>();
}
#[no_mangle]
pub unsafe extern "C" fn ptls_openssl_random_bytes(
    mut buf: *mut ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut ret: ::core::ffi::c_int =
        RAND_bytes(buf as *mut ::core::ffi::c_uchar, len as ::core::ffi::c_int);
    if ret != 1 as ::core::ffi::c_int {
        fprintf(
            stderr,
            b"RAND_bytes() failed with code: %d\n\0".as_ptr() as *const ::core::ffi::c_char,
            ret,
        );
        abort();
    }
}
unsafe extern "C" fn ecdh_gerenate_key(mut group: *mut EC_GROUP) -> *mut EC_KEY {
    let mut key: *mut EC_KEY = ::core::ptr::null_mut::<EC_KEY>();
    key = EC_KEY_new();
    if key.is_null() {
        return ::core::ptr::null_mut::<EC_KEY>();
    }
    if EC_KEY_set_group(key, group) == 0 || EC_KEY_generate_key(key) == 0 {
        EC_KEY_free(key);
        return ::core::ptr::null_mut::<EC_KEY>();
    }
    return key;
}
unsafe extern "C" fn ecdh_calc_secret(
    mut out: *mut ptls_iovec_t,
    mut group: *const EC_GROUP,
    mut privkey: *mut EC_KEY,
    mut peer_point: *mut EC_POINT,
) -> ::core::ffi::c_int {
    let mut secret: ptls_iovec_t = st_ptls_iovec_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    };
    let mut ret: ::core::ffi::c_int = 0;
    secret.len = ((EC_GROUP_get_degree(group) + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int)
        as size_t;
    secret.base = malloc(secret.len) as *mut uint8_t;
    if secret.base.is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else if ECDH_compute_key(
        secret.base as *mut ::core::ffi::c_void,
        secret.len,
        peer_point,
        privkey,
        None,
    ) <= 0 as ::core::ffi::c_int
    {
        ret = PTLS_ALERT_HANDSHAKE_FAILURE;
    } else {
        ret = 0 as ::core::ffi::c_int;
    }
    if ret == 0 as ::core::ffi::c_int {
        *out = secret;
    } else {
        free(secret.base as *mut ::core::ffi::c_void);
        *out = st_ptls_iovec_t {
            base: ::core::ptr::null_mut::<uint8_t>(),
            len: 0,
        };
    }
    return ret;
}
unsafe extern "C" fn x9_62_decode_point(
    mut group: *const EC_GROUP,
    mut vec: ptls_iovec_t,
    mut bn_ctx: *mut BN_CTX,
) -> *mut EC_POINT {
    let mut point: *mut EC_POINT = ::core::ptr::null_mut::<EC_POINT>();
    point = EC_POINT_new(group);
    if point.is_null() {
        return ::core::ptr::null_mut::<EC_POINT>();
    }
    if EC_POINT_oct2point(group, point, vec.base, vec.len, bn_ctx) == 0 {
        EC_POINT_free(point);
        return ::core::ptr::null_mut::<EC_POINT>();
    }
    return point;
}
unsafe extern "C" fn x9_62_encode_point(
    mut group: *const EC_GROUP,
    mut point: *const EC_POINT,
    mut bn_ctx: *mut BN_CTX,
) -> ptls_iovec_t {
    let mut vec: ptls_iovec_t = st_ptls_iovec_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    };
    vec.len = EC_POINT_point2oct(
        group,
        point,
        POINT_CONVERSION_UNCOMPRESSED,
        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
        0 as size_t,
        bn_ctx,
    );
    if vec.len == 0 as size_t {
        return st_ptls_iovec_t {
            base: ::core::ptr::null_mut::<uint8_t>(),
            len: 0,
        };
    }
    vec.base = malloc(vec.len) as *mut uint8_t;
    if vec.base.is_null() {
        return st_ptls_iovec_t {
            base: ::core::ptr::null_mut::<uint8_t>(),
            len: 0,
        };
    }
    if EC_POINT_point2oct(
        group,
        point,
        POINT_CONVERSION_UNCOMPRESSED,
        vec.base as *mut ::core::ffi::c_uchar,
        vec.len,
        bn_ctx,
    ) != vec.len
    {
        free(vec.base as *mut ::core::ffi::c_void);
        return st_ptls_iovec_t {
            base: ::core::ptr::null_mut::<uint8_t>(),
            len: 0,
        };
    }
    return vec;
}
unsafe extern "C" fn x9_62_free_context(mut ctx: *mut st_x9_62_keyex_context_t) {
    free((*ctx).super_0.pubkey.base as *mut ::core::ffi::c_void);
    if !(*ctx).privkey.is_null() {
        EC_KEY_free((*ctx).privkey);
    }
    if !(*ctx).bn_ctx.is_null() {
        BN_CTX_free((*ctx).bn_ctx);
    }
    free(ctx as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn x9_62_on_exchange(
    mut _ctx: *mut *mut ptls_key_exchange_context_t,
    mut release: ::core::ffi::c_int,
    mut secret: *mut ptls_iovec_t,
    mut peerkey: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut ctx: *mut st_x9_62_keyex_context_t = *_ctx as *mut st_x9_62_keyex_context_t;
    let mut group: *const EC_GROUP = EC_KEY_get0_group((*ctx).privkey);
    let mut peer_point: *mut EC_POINT = ::core::ptr::null_mut::<EC_POINT>();
    let mut ret: ::core::ffi::c_int = 0;
    if secret.is_null() {
        ret = 0 as ::core::ffi::c_int;
    } else {
        peer_point = x9_62_decode_point(group, peerkey, (*ctx).bn_ctx);
        if peer_point.is_null() {
            ret = PTLS_ALERT_DECODE_ERROR;
        } else {
            ret = ecdh_calc_secret(secret, group, (*ctx).privkey, peer_point);
            ret != 0 as ::core::ffi::c_int;
        }
    }
    if !peer_point.is_null() {
        EC_POINT_free(peer_point);
    }
    if release != 0 {
        x9_62_free_context(ctx);
        *_ctx = ::core::ptr::null_mut::<ptls_key_exchange_context_t>();
    }
    return ret;
}
unsafe extern "C" fn x9_62_create_context(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut ctx: *mut *mut st_x9_62_keyex_context_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    *ctx = malloc(::core::mem::size_of::<st_x9_62_keyex_context_t>() as size_t)
        as *mut st_x9_62_keyex_context_t;
    if (*ctx).is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else {
        **ctx = st_x9_62_keyex_context_t {
            super_0: st_ptls_key_exchange_context_t {
                algo: algo as *const st_ptls_key_exchange_algorithm_t,
                pubkey: st_ptls_iovec_t {
                    base: ::core::ptr::null_mut::<uint8_t>(),
                    len: 0,
                },
                on_exchange: Some(
                    x9_62_on_exchange
                        as unsafe extern "C" fn(
                            *mut *mut ptls_key_exchange_context_t,
                            ::core::ffi::c_int,
                            *mut ptls_iovec_t,
                            ptls_iovec_t,
                        ) -> ::core::ffi::c_int,
                ),
            },
            bn_ctx: ::core::ptr::null_mut::<BN_CTX>(),
            privkey: ::core::ptr::null_mut::<EC_KEY>(),
        };
        (**ctx).bn_ctx = BN_CTX_new();
        if (**ctx).bn_ctx.is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        } else {
            ret = 0 as ::core::ffi::c_int;
        }
    }
    if ret != 0 as ::core::ffi::c_int && !(*ctx).is_null() {
        x9_62_free_context(*ctx);
        *ctx = ::core::ptr::null_mut::<st_x9_62_keyex_context_t>();
    }
    return ret;
}
unsafe extern "C" fn x9_62_setup_pubkey(
    mut ctx: *mut st_x9_62_keyex_context_t,
) -> ::core::ffi::c_int {
    let mut group: *const EC_GROUP = EC_KEY_get0_group((*ctx).privkey);
    let mut pubkey: *const EC_POINT = EC_KEY_get0_public_key((*ctx).privkey);
    (*ctx).super_0.pubkey = x9_62_encode_point(group, pubkey, (*ctx).bn_ctx);
    if (*ctx).super_0.pubkey.base.is_null() {
        return PTLS_ERROR_NO_MEMORY;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn x9_62_create_key_exchange(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut _ctx: *mut *mut ptls_key_exchange_context_t,
) -> ::core::ffi::c_int {
    let mut group: *mut EC_GROUP = ::core::ptr::null_mut::<EC_GROUP>();
    let mut ctx: *mut st_x9_62_keyex_context_t =
        ::core::ptr::null_mut::<st_x9_62_keyex_context_t>();
    let mut ret: ::core::ffi::c_int = 0;
    group = EC_GROUP_new_by_curve_name((*algo).data as ::core::ffi::c_int);
    if group.is_null() {
        ret = PTLS_ERROR_LIBRARY;
    } else {
        ret = x9_62_create_context(algo, &raw mut ctx);
        if !(ret != 0 as ::core::ffi::c_int) {
            (*ctx).privkey = ecdh_gerenate_key(group);
            if (*ctx).privkey.is_null() {
                ret = PTLS_ERROR_LIBRARY;
            } else {
                ret = x9_62_setup_pubkey(ctx);
                if !(ret != 0 as ::core::ffi::c_int) {
                    ret = 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    if !group.is_null() {
        EC_GROUP_free(group);
    }
    if ret == 0 as ::core::ffi::c_int {
        *_ctx = &raw mut (*ctx).super_0;
    } else {
        if !ctx.is_null() {
            x9_62_free_context(ctx);
        }
        *_ctx = ::core::ptr::null_mut::<ptls_key_exchange_context_t>();
    }
    return ret;
}
unsafe extern "C" fn x9_62_init_key(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut _ctx: *mut *mut ptls_key_exchange_context_t,
    mut eckey: *mut EC_KEY,
) -> ::core::ffi::c_int {
    let mut ctx: *mut st_x9_62_keyex_context_t =
        ::core::ptr::null_mut::<st_x9_62_keyex_context_t>();
    let mut ret: ::core::ffi::c_int = 0;
    ret = x9_62_create_context(algo, &raw mut ctx);
    if !(ret != 0 as ::core::ffi::c_int) {
        (*ctx).privkey = eckey;
        ret = x9_62_setup_pubkey(ctx);
        if !(ret != 0 as ::core::ffi::c_int) {
            ret = 0 as ::core::ffi::c_int;
        }
    }
    if ret == 0 as ::core::ffi::c_int {
        *_ctx = &raw mut (*ctx).super_0;
    } else {
        if !ctx.is_null() {
            x9_62_free_context(ctx);
        }
        *_ctx = ::core::ptr::null_mut::<ptls_key_exchange_context_t>();
    }
    return ret;
}
unsafe extern "C" fn x9_62_key_exchange(
    mut group: *mut EC_GROUP,
    mut pubkey: *mut ptls_iovec_t,
    mut secret: *mut ptls_iovec_t,
    mut peerkey: ptls_iovec_t,
    mut bn_ctx: *mut BN_CTX,
) -> ::core::ffi::c_int {
    let mut peer_point: *mut EC_POINT = ::core::ptr::null_mut::<EC_POINT>();
    let mut privkey: *mut EC_KEY = ::core::ptr::null_mut::<EC_KEY>();
    let mut ret: ::core::ffi::c_int = 0;
    *pubkey = st_ptls_iovec_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    };
    *secret = st_ptls_iovec_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    };
    peer_point = x9_62_decode_point(group, peerkey, bn_ctx);
    if peer_point.is_null() {
        ret = PTLS_ALERT_DECODE_ERROR;
    } else {
        privkey = ecdh_gerenate_key(group);
        if privkey.is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        } else {
            *pubkey = x9_62_encode_point(group, EC_KEY_get0_public_key(privkey), bn_ctx);
            if (*pubkey).base.is_null() {
                ret = PTLS_ERROR_NO_MEMORY;
            } else {
                (*secret).len = ((EC_GROUP_get_degree(group) + 7 as ::core::ffi::c_int)
                    / 8 as ::core::ffi::c_int) as size_t;
                (*secret).base = malloc((*secret).len) as *mut uint8_t;
                if (*secret).base.is_null() {
                    ret = PTLS_ERROR_NO_MEMORY;
                } else if ECDH_compute_key(
                    (*secret).base as *mut ::core::ffi::c_void,
                    (*secret).len,
                    peer_point,
                    privkey,
                    None,
                ) <= 0 as ::core::ffi::c_int
                {
                    ret = PTLS_ALERT_HANDSHAKE_FAILURE;
                } else {
                    ret = 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    if !peer_point.is_null() {
        EC_POINT_free(peer_point);
    }
    if !privkey.is_null() {
        EC_KEY_free(privkey);
    }
    if ret != 0 as ::core::ffi::c_int {
        free((*pubkey).base as *mut ::core::ffi::c_void);
        *pubkey = st_ptls_iovec_t {
            base: ::core::ptr::null_mut::<uint8_t>(),
            len: 0,
        };
        free((*secret).base as *mut ::core::ffi::c_void);
        *secret = st_ptls_iovec_t {
            base: ::core::ptr::null_mut::<uint8_t>(),
            len: 0,
        };
    }
    return ret;
}
unsafe extern "C" fn secp_key_exchange(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut pubkey: *mut ptls_iovec_t,
    mut secret: *mut ptls_iovec_t,
    mut peerkey: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut group: *mut EC_GROUP = ::core::ptr::null_mut::<EC_GROUP>();
    let mut bn_ctx: *mut BN_CTX = ::core::ptr::null_mut::<BN_CTX>();
    let mut ret: ::core::ffi::c_int = 0;
    group = EC_GROUP_new_by_curve_name((*algo).data as ::core::ffi::c_int);
    if group.is_null() {
        ret = PTLS_ERROR_LIBRARY;
    } else {
        bn_ctx = BN_CTX_new();
        if bn_ctx.is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        } else {
            ret = x9_62_key_exchange(group, pubkey, secret, peerkey, bn_ctx);
        }
    }
    if !bn_ctx.is_null() {
        BN_CTX_free(bn_ctx);
    }
    if !group.is_null() {
        EC_GROUP_free(group);
    }
    return ret;
}
unsafe extern "C" fn evp_keyex_free(mut ctx: *mut st_evp_keyex_context_t) {
    if !(*ctx).privkey.is_null() {
        EVP_PKEY_free((*ctx).privkey);
    }
    if !(*ctx).super_0.pubkey.base.is_null() {
        CRYPTO_free(
            (*ctx).super_0.pubkey.base as *mut ::core::ffi::c_void,
            b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/lib/openssl.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            503 as ::core::ffi::c_int,
        );
    }
    free(ctx as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn evp_keyex_on_exchange(
    mut _ctx: *mut *mut ptls_key_exchange_context_t,
    mut release: ::core::ffi::c_int,
    mut secret: *mut ptls_iovec_t,
    mut peerkey: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut ctx: *mut st_evp_keyex_context_t =
        *_ctx as *mut ::core::ffi::c_void as *mut st_evp_keyex_context_t;
    let mut evppeer: *mut EVP_PKEY = ::core::ptr::null_mut::<EVP_PKEY>();
    let mut evpctx: *mut EVP_PKEY_CTX = ::core::ptr::null_mut::<EVP_PKEY_CTX>();
    let mut ret: ::core::ffi::c_int = 0;
    if secret.is_null() {
        ret = 0 as ::core::ffi::c_int;
    } else {
        (*secret).base = ::core::ptr::null_mut::<uint8_t>();
        if peerkey.len != (*ctx).super_0.pubkey.len {
            ret = PTLS_ALERT_DECRYPT_ERROR;
        } else {
            evppeer = EVP_PKEY_new();
            if evppeer.is_null() {
                ret = PTLS_ERROR_NO_MEMORY;
            } else if EVP_PKEY_copy_parameters(evppeer, (*ctx).privkey) <= 0 as ::core::ffi::c_int {
                ret = PTLS_ERROR_LIBRARY;
            } else if EVP_PKEY_set1_encoded_public_key(evppeer, peerkey.base, peerkey.len)
                <= 0 as ::core::ffi::c_int
            {
                ret = PTLS_ERROR_LIBRARY;
            } else {
                evpctx = EVP_PKEY_CTX_new((*ctx).privkey, ::core::ptr::null_mut::<ENGINE>());
                if evpctx.is_null() {
                    ret = PTLS_ERROR_LIBRARY;
                } else if EVP_PKEY_derive_init(evpctx) <= 0 as ::core::ffi::c_int {
                    ret = PTLS_ERROR_LIBRARY;
                } else if EVP_PKEY_derive_set_peer(evpctx, evppeer) <= 0 as ::core::ffi::c_int {
                    ret = PTLS_ERROR_LIBRARY;
                } else if EVP_PKEY_derive(
                    evpctx,
                    ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                    &raw mut (*secret).len,
                ) <= 0 as ::core::ffi::c_int
                {
                    ret = PTLS_ERROR_LIBRARY;
                } else {
                    (*secret).base = malloc((*secret).len) as *mut uint8_t;
                    if (*secret).base.is_null() {
                        ret = PTLS_ERROR_NO_MEMORY;
                    } else if EVP_PKEY_derive(
                        evpctx,
                        (*secret).base as *mut ::core::ffi::c_uchar,
                        &raw mut (*secret).len,
                    ) <= 0 as ::core::ffi::c_int
                    {
                        ret = PTLS_ERROR_LIBRARY;
                    } else {
                        ret = 0 as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    if !evpctx.is_null() {
        EVP_PKEY_CTX_free(evpctx);
    }
    if !evppeer.is_null() {
        EVP_PKEY_free(evppeer);
    }
    if ret != 0 as ::core::ffi::c_int {
        free((*secret).base as *mut ::core::ffi::c_void);
    }
    if release != 0 {
        evp_keyex_free(ctx);
        *_ctx = ::core::ptr::null_mut::<ptls_key_exchange_context_t>();
    }
    return ret;
}
unsafe extern "C" fn evp_keyex_init(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut _ctx: *mut *mut ptls_key_exchange_context_t,
    mut pkey: *mut EVP_PKEY,
) -> ::core::ffi::c_int {
    let mut ctx: *mut st_evp_keyex_context_t = ::core::ptr::null_mut::<st_evp_keyex_context_t>();
    let mut ret: ::core::ffi::c_int = 0;
    ctx = malloc(::core::mem::size_of::<st_evp_keyex_context_t>() as size_t)
        as *mut st_evp_keyex_context_t;
    if ctx.is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else {
        *ctx = st_evp_keyex_context_t {
            super_0: st_ptls_key_exchange_context_t {
                algo: algo as *const st_ptls_key_exchange_algorithm_t,
                pubkey: st_ptls_iovec_t {
                    base: ::core::ptr::null_mut::<uint8_t>(),
                    len: 0,
                },
                on_exchange: Some(
                    evp_keyex_on_exchange
                        as unsafe extern "C" fn(
                            *mut *mut ptls_key_exchange_context_t,
                            ::core::ffi::c_int,
                            *mut ptls_iovec_t,
                            ptls_iovec_t,
                        ) -> ::core::ffi::c_int,
                ),
            },
            privkey: pkey,
        };
        (*ctx).super_0.pubkey.len =
            EVP_PKEY_get1_encoded_public_key((*ctx).privkey, &raw mut (*ctx).super_0.pubkey.base);
        if (*ctx).super_0.pubkey.len == 0 as size_t {
            (*ctx).super_0.pubkey.base = ::core::ptr::null_mut::<uint8_t>();
            ret = PTLS_ERROR_NO_MEMORY;
        } else {
            *_ctx = &raw mut (*ctx).super_0;
            ret = 0 as ::core::ffi::c_int;
        }
    }
    if ret != 0 as ::core::ffi::c_int && !ctx.is_null() {
        (*ctx).privkey = ::core::ptr::null_mut::<EVP_PKEY>();
        evp_keyex_free(ctx);
    }
    return ret;
}
unsafe extern "C" fn evp_keyex_create(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut ctx: *mut *mut ptls_key_exchange_context_t,
) -> ::core::ffi::c_int {
    let mut evpctx: *mut EVP_PKEY_CTX = ::core::ptr::null_mut::<EVP_PKEY_CTX>();
    let mut pkey: *mut EVP_PKEY = ::core::ptr::null_mut::<EVP_PKEY>();
    let mut ret: ::core::ffi::c_int = 0;
    evpctx = EVP_PKEY_CTX_new_id(
        (*algo).data as ::core::ffi::c_int,
        ::core::ptr::null_mut::<ENGINE>(),
    );
    if evpctx.is_null() {
        ret = PTLS_ERROR_LIBRARY;
    } else if EVP_PKEY_keygen_init(evpctx) <= 0 as ::core::ffi::c_int {
        ret = PTLS_ERROR_LIBRARY;
    } else if EVP_PKEY_keygen(evpctx, &raw mut pkey) <= 0 as ::core::ffi::c_int {
        ret = PTLS_ERROR_LIBRARY;
    } else {
        ret = evp_keyex_init(algo, ctx, pkey);
        if !(ret != 0 as ::core::ffi::c_int) {
            pkey = ::core::ptr::null_mut::<EVP_PKEY>();
            ret = 0 as ::core::ffi::c_int;
        }
    }
    if !pkey.is_null() {
        EVP_PKEY_free(pkey);
    }
    if !evpctx.is_null() {
        EVP_PKEY_CTX_free(evpctx);
    }
    return ret;
}
unsafe extern "C" fn evp_keyex_exchange(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut outpubkey: *mut ptls_iovec_t,
    mut secret: *mut ptls_iovec_t,
    mut peerkey: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut ctx: *mut ptls_key_exchange_context_t =
        ::core::ptr::null_mut::<ptls_key_exchange_context_t>();
    let mut ret: ::core::ffi::c_int = 0;
    (*outpubkey).base = ::core::ptr::null_mut::<uint8_t>();
    ret = evp_keyex_create(algo, &raw mut ctx);
    if !(ret != 0 as ::core::ffi::c_int) {
        (*outpubkey).base = malloc((*ctx).pubkey.len) as *mut uint8_t;
        if (*outpubkey).base.is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        } else {
            memcpy(
                (*outpubkey).base as *mut ::core::ffi::c_void,
                (*ctx).pubkey.base as *const ::core::ffi::c_void,
                (*ctx).pubkey.len,
            );
            (*outpubkey).len = (*ctx).pubkey.len;
            ret = evp_keyex_on_exchange(&raw mut ctx, 1 as ::core::ffi::c_int, secret, peerkey);
        }
    }
    if !ctx.is_null() {
        evp_keyex_on_exchange(
            &raw mut ctx,
            1 as ::core::ffi::c_int,
            ::core::ptr::null_mut::<ptls_iovec_t>(),
            ptls_iovec_init(::core::ptr::null::<::core::ffi::c_void>(), 0 as size_t),
        );
    }
    if ret != 0 as ::core::ffi::c_int {
        free((*outpubkey).base as *mut ::core::ffi::c_void);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_openssl_create_key_exchange(
    mut ctx: *mut *mut ptls_key_exchange_context_t,
    mut pkey: *mut EVP_PKEY,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    let mut id: ::core::ffi::c_int = 0;
    id = EVP_PKEY_get_id(pkey);
    match id {
        EVP_PKEY_EC => {
            let mut eckey: *mut EC_KEY = EVP_PKEY_get1_EC_KEY(pkey) as *mut EC_KEY;
            let mut algo: *const ptls_key_exchange_algorithm_t =
                ::core::ptr::null::<ptls_key_exchange_algorithm_t>();
            match EC_GROUP_get_curve_name(EC_KEY_get0_group(eckey)) {
                NID_X9_62_prime256v1 => {
                    algo = &raw const ptls_openssl_secp256r1;
                }
                NID_secp384r1 => {
                    algo = &raw const ptls_openssl_secp384r1;
                }
                NID_secp521r1 => {
                    algo = &raw const ptls_openssl_secp521r1;
                }
                _ => {
                    EC_KEY_free(eckey);
                    return PTLS_ERROR_INCOMPATIBLE_KEY;
                }
            }
            ret = x9_62_init_key(algo, ctx, eckey);
            if ret != 0 as ::core::ffi::c_int {
                EC_KEY_free(eckey);
                return ret;
            }
            return 0 as ::core::ffi::c_int;
        }
        NID_X25519 => {
            ret = evp_keyex_init(&raw const ptls_openssl_x25519, ctx, pkey);
            if ret != 0 as ::core::ffi::c_int {
                return ret;
            }
            EVP_PKEY_up_ref(pkey);
            return 0 as ::core::ffi::c_int;
        }
        _ => return PTLS_ERROR_INCOMPATIBLE_KEY,
    };
}
unsafe extern "C" fn async_sign_ctx_free(mut _self: *mut ptls_async_job_t) {
    let mut self_0: *mut async_sign_ctx = _self as *mut ::core::ffi::c_void as *mut async_sign_ctx;
    if !(*self_0).job.is_null() {
        let mut ret: ::core::ffi::c_int = 0;
        while ASYNC_start_job(
            &raw mut (*self_0).job,
            (*self_0).waitctx,
            &raw mut ret,
            None,
            NULL,
            0 as size_t,
        ) == ASYNC_PAUSE
        {}
    }
    EVP_MD_CTX_free((*self_0).ctx);
    ASYNC_WAIT_CTX_free((*self_0).waitctx);
    free(self_0 as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn async_sign_ctx_get_fd(
    mut _self: *mut ptls_async_job_t,
) -> ::core::ffi::c_int {
    let mut self_0: *mut async_sign_ctx = _self as *mut ::core::ffi::c_void as *mut async_sign_ctx;
    let mut fds: [::core::ffi::c_int; 1] = [0; 1];
    let mut numfds: size_t = 0;
    ASYNC_WAIT_CTX_get_all_fds(
        (*self_0).waitctx,
        ::core::ptr::null_mut::<::core::ffi::c_int>(),
        &raw mut numfds,
    );
    ASYNC_WAIT_CTX_get_all_fds(
        (*self_0).waitctx,
        &raw mut fds as *mut ::core::ffi::c_int,
        &raw mut numfds,
    );
    return fds[0 as ::core::ffi::c_int as usize];
}
unsafe extern "C" fn async_sign_ctx_new(
    mut scheme: *const ptls_openssl_signature_scheme_t,
    mut ctx: *mut EVP_MD_CTX,
    mut siglen: size_t,
) -> *mut ptls_async_job_t {
    let mut self_0: *mut async_sign_ctx = ::core::ptr::null_mut::<async_sign_ctx>();
    self_0 = malloc((64 as size_t).wrapping_add(siglen)) as *mut async_sign_ctx;
    if self_0.is_null() {
        return ::core::ptr::null_mut::<ptls_async_job_t>();
    }
    (*self_0).super_0 = st_ptls_async_job_t {
        destroy_: Some(async_sign_ctx_free as unsafe extern "C" fn(*mut ptls_async_job_t) -> ()),
        get_fd: Some(
            async_sign_ctx_get_fd
                as unsafe extern "C" fn(*mut ptls_async_job_t) -> ::core::ffi::c_int,
        ),
        set_completion_callback: None,
    };
    (*self_0).scheme = scheme;
    (*self_0).ctx = ctx;
    (*self_0).waitctx = ASYNC_WAIT_CTX_new();
    (*self_0).job = ::core::ptr::null_mut::<ASYNC_JOB>();
    (*self_0).siglen = siglen;
    memset(
        &raw mut (*self_0).sig as *mut uint8_t as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        siglen,
    );
    return &raw mut (*self_0).super_0;
}
unsafe extern "C" fn do_sign_async_job(mut _async: *mut ::core::ffi::c_void) -> ::core::ffi::c_int {
    let mut async_0: *mut async_sign_ctx = *(_async as *mut *mut async_sign_ctx);
    return EVP_DigestSignFinal(
        (*async_0).ctx,
        &raw mut (*async_0).sig as *mut ::core::ffi::c_uchar,
        &raw mut (*async_0).siglen,
    );
}
unsafe extern "C" fn do_sign_async(
    mut outbuf: *mut ptls_buffer_t,
    mut _async: *mut *mut ptls_async_job_t,
) -> ::core::ffi::c_int {
    let mut async_0: *mut async_sign_ctx =
        *_async as *mut ::core::ffi::c_void as *mut async_sign_ctx;
    let mut ret: ::core::ffi::c_int = 0;
    match ASYNC_start_job(
        &raw mut (*async_0).job,
        (*async_0).waitctx,
        &raw mut ret,
        Some(
            do_sign_async_job
                as unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        ),
        &raw mut async_0 as *mut ::core::ffi::c_void,
        ::core::mem::size_of::<*mut async_sign_ctx>() as size_t,
    ) {
        ASYNC_PAUSE => return PTLS_ERROR_ASYNC_OPERATION,
        ASYNC_ERR => {
            ret = PTLS_ERROR_LIBRARY;
        }
        ASYNC_NO_JOBS => {
            ret = PTLS_ERROR_LIBRARY;
        }
        ASYNC_FINISH => {
            (*async_0).job = ::core::ptr::null_mut::<ASYNC_JOB>();
            ret = ptls_buffer__do_pushv(
                outbuf,
                &raw mut (*async_0).sig as *mut uint8_t as *const ::core::ffi::c_void,
                (*async_0).siglen,
            );
            if !(ret != 0 as ::core::ffi::c_int) {
                ret = 0 as ::core::ffi::c_int;
            }
        }
        _ => {
            ret = PTLS_ERROR_LIBRARY;
        }
    }
    async_sign_ctx_free(&raw mut (*async_0).super_0);
    *_async = ::core::ptr::null_mut::<ptls_async_job_t>();
    return ret;
}
unsafe extern "C" fn do_sign(
    mut key: *mut EVP_PKEY,
    mut scheme: *const ptls_openssl_signature_scheme_t,
    mut outbuf: *mut ptls_buffer_t,
    mut input: ptls_iovec_t,
    mut async_0: *mut *mut ptls_async_job_t,
) -> ::core::ffi::c_int {
    let mut c2rust_current_block: u64;
    let mut ctx: *mut EVP_MD_CTX = ::core::ptr::null_mut::<EVP_MD_CTX>();
    let mut md: *const EVP_MD = if (*scheme).scheme_md.is_some() {
        (*scheme).scheme_md.expect("non-null function pointer")()
    } else {
        ::core::ptr::null::<EVP_MD>()
    };
    let mut pkey_ctx: *mut EVP_PKEY_CTX = ::core::ptr::null_mut::<EVP_PKEY_CTX>();
    let mut siglen: size_t = 0;
    let mut ret: ::core::ffi::c_int = 0;
    ctx = EVP_MD_CTX_new();
    if ctx.is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else if EVP_DigestSignInit(
        ctx,
        &raw mut pkey_ctx,
        md,
        ::core::ptr::null_mut::<ENGINE>(),
        key,
    ) != 1 as ::core::ffi::c_int
    {
        ret = PTLS_ERROR_LIBRARY;
    } else {
        if EVP_PKEY_get_id(key) == EVP_PKEY_ED25519 {
            if EVP_DigestSign(
                ctx,
                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                &raw mut siglen,
                input.base,
                input.len,
            ) != 1 as ::core::ffi::c_int
            {
                ret = PTLS_ERROR_LIBRARY;
                c2rust_current_block = 8241805954030087821;
            } else {
                ret = ptls_buffer_reserve(outbuf, siglen);
                if ret != 0 as ::core::ffi::c_int {
                    c2rust_current_block = 8241805954030087821;
                } else if EVP_DigestSign(
                    ctx,
                    (*outbuf).base.offset((*outbuf).off as isize),
                    &raw mut siglen,
                    input.base,
                    input.len,
                ) != 1 as ::core::ffi::c_int
                {
                    ret = PTLS_ERROR_LIBRARY;
                    c2rust_current_block = 8241805954030087821;
                } else {
                    c2rust_current_block = 11636175345244025579;
                }
            }
        } else {
            if EVP_PKEY_get_id(key) == EVP_PKEY_RSA {
                if EVP_PKEY_CTX_set_rsa_padding(pkey_ctx, RSA_PKCS1_PSS_PADDING)
                    != 1 as ::core::ffi::c_int
                {
                    ret = PTLS_ERROR_LIBRARY;
                    c2rust_current_block = 8241805954030087821;
                } else if EVP_PKEY_CTX_set_rsa_pss_saltlen(pkey_ctx, -(1 as ::core::ffi::c_int))
                    != 1 as ::core::ffi::c_int
                {
                    ret = PTLS_ERROR_LIBRARY;
                    c2rust_current_block = 8241805954030087821;
                } else if EVP_PKEY_CTX_set_rsa_mgf1_md(pkey_ctx, md) != 1 as ::core::ffi::c_int {
                    ret = PTLS_ERROR_LIBRARY;
                    c2rust_current_block = 8241805954030087821;
                } else {
                    c2rust_current_block = 12124785117276362961;
                }
            } else {
                c2rust_current_block = 12124785117276362961;
            }
            match c2rust_current_block {
                8241805954030087821 => {}
                _ => {
                    if EVP_DigestSignUpdate(
                        ctx,
                        input.base as *const ::core::ffi::c_void,
                        input.len,
                    ) != 1 as ::core::ffi::c_int
                    {
                        ret = PTLS_ERROR_LIBRARY;
                        c2rust_current_block = 8241805954030087821;
                    } else if EVP_DigestSignFinal(
                        ctx,
                        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                        &raw mut siglen,
                    ) != 1 as ::core::ffi::c_int
                    {
                        ret = PTLS_ERROR_LIBRARY;
                        c2rust_current_block = 8241805954030087821;
                    } else if !async_0.is_null() {
                        *async_0 = async_sign_ctx_new(scheme, ctx, siglen);
                        if (*async_0).is_null() {
                            ret = PTLS_ERROR_NO_MEMORY;
                        } else {
                            return do_sign_async(outbuf, async_0);
                        }
                        c2rust_current_block = 8241805954030087821;
                    } else {
                        ret = ptls_buffer_reserve(outbuf, siglen);
                        if ret != 0 as ::core::ffi::c_int {
                            c2rust_current_block = 8241805954030087821;
                        } else if EVP_DigestSignFinal(
                            ctx,
                            (*outbuf).base.offset((*outbuf).off as isize),
                            &raw mut siglen,
                        ) != 1 as ::core::ffi::c_int
                        {
                            ret = PTLS_ERROR_LIBRARY;
                            c2rust_current_block = 8241805954030087821;
                        } else {
                            c2rust_current_block = 11636175345244025579;
                        }
                    }
                }
            }
        }
        match c2rust_current_block {
            8241805954030087821 => {}
            _ => {
                (*outbuf).off = (*outbuf).off.wrapping_add(siglen);
                ret = 0 as ::core::ffi::c_int;
            }
        }
    }
    if !ctx.is_null() {
        EVP_MD_CTX_free(ctx);
    }
    return ret;
}
unsafe extern "C" fn cipher_dispose(mut _ctx: *mut ptls_cipher_context_t) {
    let mut ctx: *mut cipher_context_t = _ctx as *mut cipher_context_t;
    EVP_CIPHER_CTX_free((*ctx).evp);
}
unsafe extern "C" fn cipher_do_init(
    mut _ctx: *mut ptls_cipher_context_t,
    mut iv: *const ::core::ffi::c_void,
) {
    let mut ctx: *mut cipher_context_t = _ctx as *mut cipher_context_t;
    let mut ret: ::core::ffi::c_int = 0;
    ret = EVP_EncryptInit_ex(
        (*ctx).evp,
        ::core::ptr::null::<EVP_CIPHER>(),
        ::core::ptr::null_mut::<ENGINE>(),
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        iv as *const ::core::ffi::c_uchar,
    );
}
unsafe extern "C" fn cipher_setup_crypto(
    mut _ctx: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut cipher: *const EVP_CIPHER,
    mut do_transform: Option<
        unsafe extern "C" fn(
            *mut ptls_cipher_context_t,
            *mut ::core::ffi::c_void,
            *const ::core::ffi::c_void,
            size_t,
        ) -> (),
    >,
) -> ::core::ffi::c_int {
    let mut c2rust_current_block: u64;
    let mut ctx: *mut cipher_context_t = _ctx as *mut cipher_context_t;
    (*ctx).super_0.do_dispose =
        Some(cipher_dispose as unsafe extern "C" fn(*mut ptls_cipher_context_t) -> ())
            as Option<unsafe extern "C" fn(*mut st_ptls_cipher_context_t) -> ()>;
    (*ctx).super_0.do_init = Some(
        cipher_do_init
            as unsafe extern "C" fn(*mut ptls_cipher_context_t, *const ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut st_ptls_cipher_context_t, *const ::core::ffi::c_void) -> (),
        >;
    (*ctx).super_0.do_transform = do_transform
        as Option<
            unsafe extern "C" fn(
                *mut st_ptls_cipher_context_t,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                size_t,
            ) -> (),
        >;
    (*ctx).evp = EVP_CIPHER_CTX_new();
    if (*ctx).evp.is_null() {
        return PTLS_ERROR_NO_MEMORY;
    }
    if is_enc != 0 {
        if EVP_EncryptInit_ex(
            (*ctx).evp,
            cipher,
            ::core::ptr::null_mut::<ENGINE>(),
            key as *const ::core::ffi::c_uchar,
            ::core::ptr::null::<::core::ffi::c_uchar>(),
        ) == 0
        {
            c2rust_current_block = 5391711031761144926;
        } else {
            c2rust_current_block = 14523784380283086299;
        }
    } else if EVP_DecryptInit_ex(
        (*ctx).evp,
        cipher,
        ::core::ptr::null_mut::<ENGINE>(),
        key as *const ::core::ffi::c_uchar,
        ::core::ptr::null::<::core::ffi::c_uchar>(),
    ) == 0
    {
        c2rust_current_block = 5391711031761144926;
    } else {
        EVP_CIPHER_CTX_set_padding((*ctx).evp, 0 as ::core::ffi::c_int);
        c2rust_current_block = 14523784380283086299;
    }
    match c2rust_current_block {
        14523784380283086299 => return 0 as ::core::ffi::c_int,
        _ => {
            EVP_CIPHER_CTX_free((*ctx).evp);
            return PTLS_ERROR_LIBRARY;
        }
    };
}
unsafe extern "C" fn cipher_encrypt(
    mut _ctx: *mut ptls_cipher_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut _len: size_t,
) {
    let mut ctx: *mut cipher_context_t = _ctx as *mut cipher_context_t;
    let mut len: ::core::ffi::c_int = _len as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = EVP_EncryptUpdate(
        (*ctx).evp,
        output as *mut ::core::ffi::c_uchar,
        &raw mut len,
        input as *const ::core::ffi::c_uchar,
        len,
    );
}
unsafe extern "C" fn cipher_decrypt(
    mut _ctx: *mut ptls_cipher_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut _len: size_t,
) {
    let mut ctx: *mut cipher_context_t = _ctx as *mut cipher_context_t;
    let mut len: ::core::ffi::c_int = _len as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = EVP_DecryptUpdate(
        (*ctx).evp,
        output as *mut ::core::ffi::c_uchar,
        &raw mut len,
        input as *const ::core::ffi::c_uchar,
        len,
    );
}
unsafe extern "C" fn aes128ecb_setup_crypto(
    mut ctx: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return cipher_setup_crypto(
        ctx,
        is_enc,
        key,
        EVP_aes_128_ecb(),
        if is_enc != 0 {
            Some(
                cipher_encrypt
                    as unsafe extern "C" fn(
                        *mut ptls_cipher_context_t,
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        size_t,
                    ) -> (),
            )
        } else {
            Some(
                cipher_decrypt
                    as unsafe extern "C" fn(
                        *mut ptls_cipher_context_t,
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        size_t,
                    ) -> (),
            )
        },
    );
}
unsafe extern "C" fn aes256ecb_setup_crypto(
    mut ctx: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return cipher_setup_crypto(
        ctx,
        is_enc,
        key,
        EVP_aes_256_ecb(),
        if is_enc != 0 {
            Some(
                cipher_encrypt
                    as unsafe extern "C" fn(
                        *mut ptls_cipher_context_t,
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        size_t,
                    ) -> (),
            )
        } else {
            Some(
                cipher_decrypt
                    as unsafe extern "C" fn(
                        *mut ptls_cipher_context_t,
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        size_t,
                    ) -> (),
            )
        },
    );
}
unsafe extern "C" fn aes128ctr_setup_crypto(
    mut ctx: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return cipher_setup_crypto(
        ctx,
        1 as ::core::ffi::c_int,
        key,
        EVP_aes_128_ctr(),
        Some(
            cipher_encrypt
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        ),
    );
}
unsafe extern "C" fn aes256ctr_setup_crypto(
    mut ctx: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return cipher_setup_crypto(
        ctx,
        1 as ::core::ffi::c_int,
        key,
        EVP_aes_256_ctr(),
        Some(
            cipher_encrypt
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        ),
    );
}
unsafe extern "C" fn chacha20_setup_crypto(
    mut ctx: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return cipher_setup_crypto(
        ctx,
        1 as ::core::ffi::c_int,
        key,
        EVP_chacha20(),
        Some(
            cipher_encrypt
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        ),
    );
}
unsafe extern "C" fn bfecb_setup_crypto(
    mut ctx: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return cipher_setup_crypto(
        ctx,
        is_enc,
        key,
        EVP_bf_ecb(),
        if is_enc != 0 {
            Some(
                cipher_encrypt
                    as unsafe extern "C" fn(
                        *mut ptls_cipher_context_t,
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        size_t,
                    ) -> (),
            )
        } else {
            Some(
                cipher_decrypt
                    as unsafe extern "C" fn(
                        *mut ptls_cipher_context_t,
                        *mut ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                        size_t,
                    ) -> (),
            )
        },
    );
}
unsafe extern "C" fn aead_dispose_crypto(mut _ctx: *mut ptls_aead_context_t) {
    let mut ctx: *mut aead_crypto_context_t = _ctx as *mut aead_crypto_context_t;
    if !(*ctx).evp_ctx.is_null() {
        EVP_CIPHER_CTX_free((*ctx).evp_ctx);
    }
}
unsafe extern "C" fn aead_get_iv(
    mut _ctx: *mut ptls_aead_context_t,
    mut iv: *mut ::core::ffi::c_void,
) {
    let mut ctx: *mut aead_crypto_context_t = _ctx as *mut aead_crypto_context_t;
    memcpy(
        iv,
        &raw mut (*ctx).static_iv as *mut uint8_t as *const ::core::ffi::c_void,
        (*(*ctx).super_0.algo).iv_size,
    );
}
unsafe extern "C" fn aead_set_iv(
    mut _ctx: *mut ptls_aead_context_t,
    mut iv: *const ::core::ffi::c_void,
) {
    let mut ctx: *mut aead_crypto_context_t = _ctx as *mut aead_crypto_context_t;
    memcpy(
        &raw mut (*ctx).static_iv as *mut uint8_t as *mut ::core::ffi::c_void,
        iv,
        (*(*ctx).super_0.algo).iv_size,
    );
}
unsafe extern "C" fn aead_do_encrypt_init(
    mut _ctx: *mut ptls_aead_context_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) {
    let mut ctx: *mut aead_crypto_context_t = _ctx as *mut aead_crypto_context_t;
    let mut iv: [uint8_t; 32] = [0; 32];
    let mut ret: ::core::ffi::c_int = 0;
    ptls_aead__build_iv(
        (*ctx).super_0.algo as *const ptls_aead_algorithm_t,
        &raw mut iv as *mut uint8_t,
        &raw mut (*ctx).static_iv as *mut uint8_t,
        seq,
    );
    ret = EVP_EncryptInit_ex(
        (*ctx).evp_ctx,
        ::core::ptr::null::<EVP_CIPHER>(),
        ::core::ptr::null_mut::<ENGINE>(),
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        &raw mut iv as *mut uint8_t,
    );
    if aadlen != 0 as size_t {
        let mut blocklen: ::core::ffi::c_int = 0;
        ret = EVP_EncryptUpdate(
            (*ctx).evp_ctx,
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            &raw mut blocklen,
            aad as *const ::core::ffi::c_uchar,
            aadlen as ::core::ffi::c_int,
        );
    }
}
unsafe extern "C" fn aead_do_encrypt_update(
    mut _ctx: *mut ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
) -> size_t {
    let mut ctx: *mut aead_crypto_context_t = _ctx as *mut aead_crypto_context_t;
    let mut blocklen: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    ret = EVP_EncryptUpdate(
        (*ctx).evp_ctx,
        output as *mut ::core::ffi::c_uchar,
        &raw mut blocklen,
        input as *const ::core::ffi::c_uchar,
        inlen as ::core::ffi::c_int,
    );
    return blocklen as size_t;
}
unsafe extern "C" fn aead_do_encrypt_final(
    mut _ctx: *mut ptls_aead_context_t,
    mut _output: *mut ::core::ffi::c_void,
) -> size_t {
    let mut ctx: *mut aead_crypto_context_t = _ctx as *mut aead_crypto_context_t;
    let mut output: *mut uint8_t = _output as *mut uint8_t;
    let mut off: size_t = 0 as size_t;
    let mut tag_size: size_t = (*(*ctx).super_0.algo).tag_size;
    let mut blocklen: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    ret = EVP_EncryptFinal_ex(
        (*ctx).evp_ctx,
        output.offset(off as isize),
        &raw mut blocklen,
    );
    off = off.wrapping_add(blocklen as size_t);
    ret = EVP_CIPHER_CTX_ctrl(
        (*ctx).evp_ctx,
        EVP_CTRL_GCM_GET_TAG,
        tag_size as ::core::ffi::c_int,
        output.offset(off as isize) as *mut ::core::ffi::c_void,
    );
    off = off.wrapping_add(tag_size);
    return off;
}
unsafe extern "C" fn aead_do_decrypt(
    mut _ctx: *mut ptls_aead_context_t,
    mut _output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) -> size_t {
    let mut ctx: *mut aead_crypto_context_t = _ctx as *mut aead_crypto_context_t;
    let mut output: *mut uint8_t = _output as *mut uint8_t;
    let mut iv: [uint8_t; 32] = [0; 32];
    let mut off: size_t = 0 as size_t;
    let mut tag_size: size_t = (*(*ctx).super_0.algo).tag_size;
    let mut blocklen: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    if inlen < tag_size {
        return SIZE_MAX as size_t;
    }
    ptls_aead__build_iv(
        (*ctx).super_0.algo as *const ptls_aead_algorithm_t,
        &raw mut iv as *mut uint8_t,
        &raw mut (*ctx).static_iv as *mut uint8_t,
        seq,
    );
    ret = EVP_DecryptInit_ex(
        (*ctx).evp_ctx,
        ::core::ptr::null::<EVP_CIPHER>(),
        ::core::ptr::null_mut::<ENGINE>(),
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        &raw mut iv as *mut uint8_t,
    );
    if aadlen != 0 as size_t {
        ret = EVP_DecryptUpdate(
            (*ctx).evp_ctx,
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            &raw mut blocklen,
            aad as *const ::core::ffi::c_uchar,
            aadlen as ::core::ffi::c_int,
        );
    }
    ret = EVP_DecryptUpdate(
        (*ctx).evp_ctx,
        output.offset(off as isize),
        &raw mut blocklen,
        input as *const ::core::ffi::c_uchar,
        inlen.wrapping_sub(tag_size) as ::core::ffi::c_int,
    );
    off = off.wrapping_add(blocklen as size_t);
    if EVP_CIPHER_CTX_ctrl(
        (*ctx).evp_ctx,
        EVP_CTRL_GCM_SET_TAG,
        tag_size as ::core::ffi::c_int,
        (input as *mut uint8_t)
            .offset(inlen as isize)
            .offset(-(tag_size as isize)) as *mut ::core::ffi::c_void,
    ) == 0
    {
        return SIZE_MAX as size_t;
    }
    if EVP_DecryptFinal_ex(
        (*ctx).evp_ctx,
        output.offset(off as isize),
        &raw mut blocklen,
    ) == 0
    {
        return SIZE_MAX as size_t;
    }
    off = off.wrapping_add(blocklen as size_t);
    return off;
}
unsafe extern "C" fn aead_setup_crypto(
    mut _ctx: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
    mut cipher: *const EVP_CIPHER,
) -> ::core::ffi::c_int {
    let mut c2rust_current_block: u64;
    let mut ctx: *mut aead_crypto_context_t = _ctx as *mut aead_crypto_context_t;
    let mut ret: ::core::ffi::c_int = 0;
    (*ctx).super_0.dispose_crypto =
        Some(aead_dispose_crypto as unsafe extern "C" fn(*mut ptls_aead_context_t) -> ())
            as Option<unsafe extern "C" fn(*mut st_ptls_aead_context_t) -> ()>;
    (*ctx).super_0.do_get_iv = Some(
        aead_get_iv
            as unsafe extern "C" fn(*mut ptls_aead_context_t, *mut ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut st_ptls_aead_context_t, *mut ::core::ffi::c_void) -> (),
        >;
    (*ctx).super_0.do_set_iv = Some(
        aead_set_iv
            as unsafe extern "C" fn(*mut ptls_aead_context_t, *const ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut st_ptls_aead_context_t, *const ::core::ffi::c_void) -> (),
        >;
    if is_enc != 0 {
        (*ctx).super_0.do_encrypt_init = Some(
            aead_do_encrypt_init
                as unsafe extern "C" fn(
                    *mut ptls_aead_context_t,
                    uint64_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut st_ptls_aead_context_t,
                    uint64_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
            >;
        (*ctx).super_0.do_encrypt_update = Some(
            aead_do_encrypt_update
                as unsafe extern "C" fn(
                    *mut ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> size_t,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut st_ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> size_t,
            >;
        (*ctx).super_0.do_encrypt_final = Some(
            aead_do_encrypt_final
                as unsafe extern "C" fn(
                    *mut ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut st_ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                ) -> size_t,
            >;
        (*ctx).super_0.do_encrypt = Some(
            ptls_aead__do_encrypt
                as unsafe extern "C" fn(
                    *mut ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    size_t,
                    uint64_t,
                    *const ::core::ffi::c_void,
                    size_t,
                    *mut ptls_aead_supplementary_encryption_t,
                ) -> (),
        )
            as Option<
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
            >;
        (*ctx).super_0.do_encrypt_v = Some(
            ptls_aead__do_encrypt_v
                as unsafe extern "C" fn(
                    *mut ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                    *mut ptls_iovec_t,
                    size_t,
                    uint64_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        )
            as Option<
                unsafe extern "C" fn(
                    *mut st_ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                    *mut ptls_iovec_t,
                    size_t,
                    uint64_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
            >;
        (*ctx).super_0.do_decrypt = None;
    } else {
        (*ctx).super_0.do_encrypt_init = None;
        (*ctx).super_0.do_encrypt_update = None;
        (*ctx).super_0.do_encrypt_final = None;
        (*ctx).super_0.do_encrypt = None;
        (*ctx).super_0.do_encrypt_v = None;
        (*ctx).super_0.do_decrypt = Some(
            aead_do_decrypt
                as unsafe extern "C" fn(
                    *mut ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    size_t,
                    uint64_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> size_t,
        )
            as Option<
                unsafe extern "C" fn(
                    *mut st_ptls_aead_context_t,
                    *mut ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                    size_t,
                    uint64_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> size_t,
            >;
    }
    (*ctx).evp_ctx = ::core::ptr::null_mut::<EVP_CIPHER_CTX>();
    (*ctx).evp_ctx = EVP_CIPHER_CTX_new();
    if (*ctx).evp_ctx.is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else {
        if is_enc != 0 {
            if EVP_EncryptInit_ex(
                (*ctx).evp_ctx,
                cipher,
                ::core::ptr::null_mut::<ENGINE>(),
                key as *const ::core::ffi::c_uchar,
                ::core::ptr::null::<::core::ffi::c_uchar>(),
            ) == 0
            {
                ret = PTLS_ERROR_LIBRARY;
                c2rust_current_block = 3486837565587853475;
            } else {
                c2rust_current_block = 5601891728916014340;
            }
        } else if EVP_DecryptInit_ex(
            (*ctx).evp_ctx,
            cipher,
            ::core::ptr::null_mut::<ENGINE>(),
            key as *const ::core::ffi::c_uchar,
            ::core::ptr::null::<::core::ffi::c_uchar>(),
        ) == 0
        {
            ret = PTLS_ERROR_LIBRARY;
            c2rust_current_block = 3486837565587853475;
        } else {
            c2rust_current_block = 5601891728916014340;
        }
        match c2rust_current_block {
            3486837565587853475 => {}
            _ => {
                if EVP_CIPHER_CTX_ctrl(
                    (*ctx).evp_ctx,
                    EVP_CTRL_GCM_SET_IVLEN,
                    (*(*ctx).super_0.algo).iv_size as ::core::ffi::c_int,
                    NULL,
                ) == 0
                {
                    ret = PTLS_ERROR_LIBRARY;
                } else {
                    memcpy(
                        &raw mut (*ctx).static_iv as *mut uint8_t as *mut ::core::ffi::c_void,
                        iv,
                        (*(*ctx).super_0.algo).iv_size,
                    );
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    aead_dispose_crypto(&raw mut (*ctx).super_0);
    return ret;
}
unsafe extern "C" fn aead_aes128gcm_setup_crypto(
    mut ctx: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return aead_setup_crypto(ctx, is_enc, key, iv, EVP_aes_128_gcm());
}
unsafe extern "C" fn aead_aes256gcm_setup_crypto(
    mut ctx: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return aead_setup_crypto(ctx, is_enc, key, iv, EVP_aes_256_gcm());
}
unsafe extern "C" fn aead_chacha20poly1305_setup_crypto(
    mut ctx: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return aead_setup_crypto(ctx, is_enc, key, iv, EVP_chacha20_poly1305());
}
unsafe extern "C" fn sha256_final(
    mut _ctx: *mut ptls_hash_context_t,
    mut md: *mut ::core::ffi::c_void,
    mut mode: ptls_hash_final_mode_t,
) {
    let mut ctx: *mut sha256_context_t = _ctx as *mut sha256_context_t;
    if mode as ::core::ffi::c_uint
        == PTLS_HASH_FINAL_MODE_SNAPSHOT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut copy: SHA256_CTX = (*ctx).ctx;
        SHA256_Final(md as *mut ::core::ffi::c_uchar, &raw mut copy);
        ptls_clear_memory.expect("non-null function pointer")(
            &raw mut copy as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<SHA256_CTX>() as size_t,
        );
        return;
    }
    if !md.is_null() {
        SHA256_Final(md as *mut ::core::ffi::c_uchar, &raw mut (*ctx).ctx);
    }
    match mode as ::core::ffi::c_uint {
        0 => {
            ptls_clear_memory.expect("non-null function pointer")(
                &raw mut (*ctx).ctx as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<SHA256_CTX>() as size_t,
            );
            free(ctx as *mut ::core::ffi::c_void);
        }
        1 => {
            SHA256_Init(&raw mut (*ctx).ctx);
        }
        _ => {}
    };
}
unsafe extern "C" fn sha256_clone(mut _src: *mut ptls_hash_context_t) -> *mut ptls_hash_context_t {
    let mut dst: *mut sha256_context_t = ::core::ptr::null_mut::<sha256_context_t>();
    let mut src: *mut sha256_context_t = _src as *mut sha256_context_t;
    dst = malloc(::core::mem::size_of::<sha256_context_t>() as size_t) as *mut sha256_context_t;
    if dst.is_null() {
        return ::core::ptr::null_mut::<ptls_hash_context_t>();
    }
    (*dst).super_0 = (*src).super_0;
    ptls_hash_clone_memcpy(
        &raw mut (*dst).ctx as *mut ::core::ffi::c_void,
        &raw mut (*src).ctx as *const ::core::ffi::c_void,
        ::core::mem::size_of::<SHA256_CTX>() as size_t,
    );
    return &raw mut (*dst).super_0;
}
unsafe extern "C" fn sha256_create() -> *mut ptls_hash_context_t {
    let mut ctx: *mut sha256_context_t = ::core::ptr::null_mut::<sha256_context_t>();
    ctx = malloc(::core::mem::size_of::<sha256_context_t>() as size_t) as *mut sha256_context_t;
    if ctx.is_null() {
        return ::core::ptr::null_mut::<ptls_hash_context_t>();
    }
    (*ctx).super_0 = st_ptls_hash_context_t {
        update: Some(
            sha256_update
                as unsafe extern "C" fn(
                    *mut ptls_hash_context_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        ),
        final_0: Some(
            sha256_final
                as unsafe extern "C" fn(
                    *mut ptls_hash_context_t,
                    *mut ::core::ffi::c_void,
                    ptls_hash_final_mode_t,
                ) -> (),
        ),
        clone_: Some(
            sha256_clone
                as unsafe extern "C" fn(*mut ptls_hash_context_t) -> *mut ptls_hash_context_t,
        ),
    };
    SHA256_Init(&raw mut (*ctx).ctx);
    return &raw mut (*ctx).super_0;
}
unsafe extern "C" fn sha256_update(
    mut _ctx: *mut ptls_hash_context_t,
    mut src: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut ctx: *mut sha256_context_t = _ctx as *mut sha256_context_t;
    SHA256_Update(&raw mut (*ctx).ctx, src, len);
}
unsafe extern "C" fn sha384_final(
    mut _ctx: *mut ptls_hash_context_t,
    mut md: *mut ::core::ffi::c_void,
    mut mode: ptls_hash_final_mode_t,
) {
    let mut ctx: *mut sha384_context_t = _ctx as *mut sha384_context_t;
    if mode as ::core::ffi::c_uint
        == PTLS_HASH_FINAL_MODE_SNAPSHOT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut copy: SHA512_CTX = (*ctx).ctx;
        SHA384_Final(md as *mut ::core::ffi::c_uchar, &raw mut copy);
        ptls_clear_memory.expect("non-null function pointer")(
            &raw mut copy as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<SHA512_CTX>() as size_t,
        );
        return;
    }
    if !md.is_null() {
        SHA384_Final(md as *mut ::core::ffi::c_uchar, &raw mut (*ctx).ctx);
    }
    match mode as ::core::ffi::c_uint {
        0 => {
            ptls_clear_memory.expect("non-null function pointer")(
                &raw mut (*ctx).ctx as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<SHA512_CTX>() as size_t,
            );
            free(ctx as *mut ::core::ffi::c_void);
        }
        1 => {
            SHA384_Init(&raw mut (*ctx).ctx);
        }
        _ => {}
    };
}
unsafe extern "C" fn sha384_create() -> *mut ptls_hash_context_t {
    let mut ctx: *mut sha384_context_t = ::core::ptr::null_mut::<sha384_context_t>();
    ctx = malloc(::core::mem::size_of::<sha384_context_t>() as size_t) as *mut sha384_context_t;
    if ctx.is_null() {
        return ::core::ptr::null_mut::<ptls_hash_context_t>();
    }
    (*ctx).super_0 = st_ptls_hash_context_t {
        update: Some(
            sha384_update
                as unsafe extern "C" fn(
                    *mut ptls_hash_context_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        ),
        final_0: Some(
            sha384_final
                as unsafe extern "C" fn(
                    *mut ptls_hash_context_t,
                    *mut ::core::ffi::c_void,
                    ptls_hash_final_mode_t,
                ) -> (),
        ),
        clone_: Some(
            sha384_clone
                as unsafe extern "C" fn(*mut ptls_hash_context_t) -> *mut ptls_hash_context_t,
        ),
    };
    SHA384_Init(&raw mut (*ctx).ctx);
    return &raw mut (*ctx).super_0;
}
unsafe extern "C" fn sha384_clone(mut _src: *mut ptls_hash_context_t) -> *mut ptls_hash_context_t {
    let mut dst: *mut sha384_context_t = ::core::ptr::null_mut::<sha384_context_t>();
    let mut src: *mut sha384_context_t = _src as *mut sha384_context_t;
    dst = malloc(::core::mem::size_of::<sha384_context_t>() as size_t) as *mut sha384_context_t;
    if dst.is_null() {
        return ::core::ptr::null_mut::<ptls_hash_context_t>();
    }
    (*dst).super_0 = (*src).super_0;
    ptls_hash_clone_memcpy(
        &raw mut (*dst).ctx as *mut ::core::ffi::c_void,
        &raw mut (*src).ctx as *const ::core::ffi::c_void,
        ::core::mem::size_of::<SHA512_CTX>() as size_t,
    );
    return &raw mut (*dst).super_0;
}
unsafe extern "C" fn sha384_update(
    mut _ctx: *mut ptls_hash_context_t,
    mut src: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut ctx: *mut sha384_context_t = _ctx as *mut sha384_context_t;
    SHA384_Update(&raw mut (*ctx).ctx, src, len);
}
unsafe extern "C" fn sha512_update(
    mut _ctx: *mut ptls_hash_context_t,
    mut src: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut ctx: *mut sha512_context_t = _ctx as *mut sha512_context_t;
    SHA512_Update(&raw mut (*ctx).ctx, src, len);
}
unsafe extern "C" fn sha512_create() -> *mut ptls_hash_context_t {
    let mut ctx: *mut sha512_context_t = ::core::ptr::null_mut::<sha512_context_t>();
    ctx = malloc(::core::mem::size_of::<sha512_context_t>() as size_t) as *mut sha512_context_t;
    if ctx.is_null() {
        return ::core::ptr::null_mut::<ptls_hash_context_t>();
    }
    (*ctx).super_0 = st_ptls_hash_context_t {
        update: Some(
            sha512_update
                as unsafe extern "C" fn(
                    *mut ptls_hash_context_t,
                    *const ::core::ffi::c_void,
                    size_t,
                ) -> (),
        ),
        final_0: Some(
            sha512_final
                as unsafe extern "C" fn(
                    *mut ptls_hash_context_t,
                    *mut ::core::ffi::c_void,
                    ptls_hash_final_mode_t,
                ) -> (),
        ),
        clone_: Some(
            sha512_clone
                as unsafe extern "C" fn(*mut ptls_hash_context_t) -> *mut ptls_hash_context_t,
        ),
    };
    SHA512_Init(&raw mut (*ctx).ctx);
    return &raw mut (*ctx).super_0;
}
unsafe extern "C" fn sha512_final(
    mut _ctx: *mut ptls_hash_context_t,
    mut md: *mut ::core::ffi::c_void,
    mut mode: ptls_hash_final_mode_t,
) {
    let mut ctx: *mut sha512_context_t = _ctx as *mut sha512_context_t;
    if mode as ::core::ffi::c_uint
        == PTLS_HASH_FINAL_MODE_SNAPSHOT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut copy: SHA512_CTX = (*ctx).ctx;
        SHA512_Final(md as *mut ::core::ffi::c_uchar, &raw mut copy);
        ptls_clear_memory.expect("non-null function pointer")(
            &raw mut copy as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<SHA512_CTX>() as size_t,
        );
        return;
    }
    if !md.is_null() {
        SHA512_Final(md as *mut ::core::ffi::c_uchar, &raw mut (*ctx).ctx);
    }
    match mode as ::core::ffi::c_uint {
        0 => {
            ptls_clear_memory.expect("non-null function pointer")(
                &raw mut (*ctx).ctx as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<SHA512_CTX>() as size_t,
            );
            free(ctx as *mut ::core::ffi::c_void);
        }
        1 => {
            SHA512_Init(&raw mut (*ctx).ctx);
        }
        _ => {}
    };
}
unsafe extern "C" fn sha512_clone(mut _src: *mut ptls_hash_context_t) -> *mut ptls_hash_context_t {
    let mut dst: *mut sha512_context_t = ::core::ptr::null_mut::<sha512_context_t>();
    let mut src: *mut sha512_context_t = _src as *mut sha512_context_t;
    dst = malloc(::core::mem::size_of::<sha512_context_t>() as size_t) as *mut sha512_context_t;
    if dst.is_null() {
        return ::core::ptr::null_mut::<ptls_hash_context_t>();
    }
    (*dst).super_0 = (*src).super_0;
    ptls_hash_clone_memcpy(
        &raw mut (*dst).ctx as *mut ::core::ffi::c_void,
        &raw mut (*src).ctx as *const ::core::ffi::c_void,
        ::core::mem::size_of::<SHA512_CTX>() as size_t,
    );
    return &raw mut (*dst).super_0;
}
unsafe extern "C" fn sign_certificate(
    mut _self: *mut ptls_sign_certificate_t,
    mut tls: *mut ptls_t,
    mut async_0: *mut *mut ptls_async_job_t,
    mut selected_algorithm: *mut uint16_t,
    mut outbuf: *mut ptls_buffer_t,
    mut input: ptls_iovec_t,
    mut algorithms: *const uint16_t,
    mut num_algorithms: size_t,
) -> ::core::ffi::c_int {
    let mut self_0: *mut ptls_openssl_sign_certificate_t =
        _self as *mut ptls_openssl_sign_certificate_t;
    let mut scheme: *const ptls_openssl_signature_scheme_t =
        ::core::ptr::null::<ptls_openssl_signature_scheme_t>();
    if !async_0.is_null() && !(*async_0).is_null() {
        let mut sign_ctx: *mut async_sign_ctx = *async_0 as *mut async_sign_ctx;
        *selected_algorithm = (*(*sign_ctx).scheme).scheme_id;
        return do_sign_async(outbuf, async_0);
    }
    scheme = ptls_openssl_select_signature_scheme((*self_0).schemes, algorithms, num_algorithms);
    if scheme.is_null() {
        return PTLS_ALERT_HANDSHAKE_FAILURE;
    }
    *selected_algorithm = (*scheme).scheme_id;
    if (*self_0).async_0() == 0 && !async_0.is_null() {
        async_0 = ::core::ptr::null_mut::<*mut ptls_async_job_t>();
    }
    return do_sign((*self_0).key, scheme, outbuf, input, async_0);
}
unsafe extern "C" fn to_x509(mut vec: ptls_iovec_t) -> *mut X509 {
    let mut p: *const uint8_t = vec.base;
    return d2i_X509(
        ::core::ptr::null_mut::<*mut X509>(),
        &raw mut p,
        vec.len as ::core::ffi::c_long,
    );
}
unsafe extern "C" fn verify_sign(
    mut verify_ctx: *mut ::core::ffi::c_void,
    mut algo: uint16_t,
    mut data: ptls_iovec_t,
    mut signature: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut c2rust_current_block: u64;
    let mut key: *mut EVP_PKEY = verify_ctx as *mut EVP_PKEY;
    let mut scheme: *const ptls_openssl_signature_scheme_t =
        ::core::ptr::null::<ptls_openssl_signature_scheme_t>();
    let mut ctx: *mut EVP_MD_CTX = ::core::ptr::null_mut::<EVP_MD_CTX>();
    let mut pkey_ctx: *mut EVP_PKEY_CTX = ::core::ptr::null_mut::<EVP_PKEY_CTX>();
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if !data.base.is_null() {
        scheme = ptls_openssl_lookup_signature_schemes(key);
        if scheme.is_null() {
            ret = PTLS_ERROR_LIBRARY;
        } else {
            loop {
                if !((*scheme).scheme_id as ::core::ffi::c_int != UINT16_MAX) {
                    c2rust_current_block = 11006700562992250127;
                    break;
                }
                if (*scheme).scheme_id as ::core::ffi::c_int == algo as ::core::ffi::c_int {
                    c2rust_current_block = 6059233811595615302;
                    break;
                }
                scheme = scheme.offset(1);
            }
            match c2rust_current_block {
                11006700562992250127 => {
                    ret = PTLS_ALERT_ILLEGAL_PARAMETER;
                }
                _ => {
                    ctx = EVP_MD_CTX_new();
                    if ctx.is_null() {
                        ret = PTLS_ERROR_NO_MEMORY;
                    } else {
                        if EVP_PKEY_get_id(key) == EVP_PKEY_ED25519 {
                            if EVP_DigestVerifyInit(
                                ctx,
                                &raw mut pkey_ctx,
                                ::core::ptr::null::<EVP_MD>(),
                                ::core::ptr::null_mut::<ENGINE>(),
                                key,
                            ) != 1 as ::core::ffi::c_int
                            {
                                ret = PTLS_ERROR_LIBRARY;
                                c2rust_current_block = 13192023357945725858;
                            } else if EVP_DigestVerify(
                                ctx,
                                signature.base,
                                signature.len,
                                data.base,
                                data.len,
                            ) != 1 as ::core::ffi::c_int
                            {
                                ret = PTLS_ERROR_LIBRARY;
                                c2rust_current_block = 13192023357945725858;
                            } else {
                                c2rust_current_block = 11932355480408055363;
                            }
                        } else if EVP_DigestVerifyInit(
                            ctx,
                            &raw mut pkey_ctx,
                            (*scheme).scheme_md.expect("non-null function pointer")(),
                            ::core::ptr::null_mut::<ENGINE>(),
                            key,
                        ) != 1 as ::core::ffi::c_int
                        {
                            ret = PTLS_ERROR_LIBRARY;
                            c2rust_current_block = 13192023357945725858;
                        } else {
                            if EVP_PKEY_get_id(key) == EVP_PKEY_RSA {
                                if EVP_PKEY_CTX_set_rsa_padding(pkey_ctx, RSA_PKCS1_PSS_PADDING)
                                    != 1 as ::core::ffi::c_int
                                {
                                    ret = PTLS_ERROR_LIBRARY;
                                    c2rust_current_block = 13192023357945725858;
                                } else if EVP_PKEY_CTX_set_rsa_pss_saltlen(
                                    pkey_ctx,
                                    -(1 as ::core::ffi::c_int),
                                ) != 1 as ::core::ffi::c_int
                                {
                                    ret = PTLS_ERROR_LIBRARY;
                                    c2rust_current_block = 13192023357945725858;
                                } else if EVP_PKEY_CTX_set_rsa_mgf1_md(
                                    pkey_ctx,
                                    (*scheme).scheme_md.expect("non-null function pointer")(),
                                ) != 1 as ::core::ffi::c_int
                                {
                                    ret = PTLS_ERROR_LIBRARY;
                                    c2rust_current_block = 13192023357945725858;
                                } else {
                                    c2rust_current_block = 16203760046146113240;
                                }
                            } else {
                                c2rust_current_block = 16203760046146113240;
                            }
                            match c2rust_current_block {
                                13192023357945725858 => {}
                                _ => {
                                    if EVP_DigestVerifyUpdate(
                                        ctx,
                                        data.base as *const ::core::ffi::c_void,
                                        data.len,
                                    ) != 1 as ::core::ffi::c_int
                                    {
                                        ret = PTLS_ERROR_LIBRARY;
                                        c2rust_current_block = 13192023357945725858;
                                    } else if EVP_DigestVerifyFinal(
                                        ctx,
                                        signature.base,
                                        signature.len,
                                    ) != 1 as ::core::ffi::c_int
                                    {
                                        ret = PTLS_ALERT_DECRYPT_ERROR;
                                        c2rust_current_block = 13192023357945725858;
                                    } else {
                                        c2rust_current_block = 11932355480408055363;
                                    }
                                }
                            }
                        }
                        match c2rust_current_block {
                            13192023357945725858 => {}
                            _ => {
                                ret = 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    if !ctx.is_null() {
        EVP_MD_CTX_free(ctx);
    }
    EVP_PKEY_free(key);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_openssl_init_sign_certificate(
    mut self_0: *mut ptls_openssl_sign_certificate_t,
    mut key: *mut EVP_PKEY,
) -> ::core::ffi::c_int {
    *self_0 = {
        let mut init = st_ptls_openssl_sign_certificate_t {
            async_0: [0; 1],
            c2rust_padding: [0; 7],
            super_0: st_ptls_sign_certificate_t {
                cb: Some(
                    sign_certificate
                        as unsafe extern "C" fn(
                            *mut ptls_sign_certificate_t,
                            *mut ptls_t,
                            *mut *mut ptls_async_job_t,
                            *mut uint16_t,
                            *mut ptls_buffer_t,
                            ptls_iovec_t,
                            *const uint16_t,
                            size_t,
                        ) -> ::core::ffi::c_int,
                ),
            },
            key: ::core::ptr::null_mut::<EVP_PKEY>(),
            schemes: ::core::ptr::null::<ptls_openssl_signature_scheme_t>(),
        };
        init.set_async_0(0 as ::core::ffi::c_uint);
        init
    };
    (*self_0).schemes = ptls_openssl_lookup_signature_schemes(key);
    if (*self_0).schemes.is_null() {
        return PTLS_ERROR_INCOMPATIBLE_KEY;
    }
    EVP_PKEY_up_ref(key);
    (*self_0).key = key;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_openssl_dispose_sign_certificate(
    mut self_0: *mut ptls_openssl_sign_certificate_t,
) {
    EVP_PKEY_free((*self_0).key);
}
unsafe extern "C" fn serialize_cert(
    mut cert: *mut X509,
    mut dst: *mut ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut len: ::core::ffi::c_int =
        i2d_X509(cert, ::core::ptr::null_mut::<*mut ::core::ffi::c_uchar>());
    (*dst).base = malloc(len as size_t) as *mut uint8_t;
    if (*dst).base.is_null() {
        return PTLS_ERROR_NO_MEMORY;
    }
    let mut p: *mut ::core::ffi::c_uchar = (*dst).base as *mut ::core::ffi::c_uchar;
    (*dst).len = i2d_X509(cert, &raw mut p) as size_t;
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_openssl_load_certificates(
    mut ctx: *mut ptls_context_t,
    mut cert: *mut X509,
    mut chain: *mut stack_st_X509,
) -> ::core::ffi::c_int {
    let mut c2rust_current_block: u64;
    let mut list: *mut ptls_iovec_t = ::core::ptr::null_mut::<ptls_iovec_t>();
    let mut slot: size_t = 0 as size_t;
    let mut count: size_t = ((cert != NULL as *mut X509) as ::core::ffi::c_int
        + (if !chain.is_null() {
            OPENSSL_sk_num(ossl_check_const_X509_sk_type(chain))
        } else {
            0 as ::core::ffi::c_int
        })) as size_t;
    let mut ret: ::core::ffi::c_int = 0;
    list = malloc((::core::mem::size_of::<ptls_iovec_t>() as size_t).wrapping_mul(count))
        as *mut ptls_iovec_t;
    if list.is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else {
        if !cert.is_null() {
            let c2rust_fresh0 = slot;
            slot = slot.wrapping_add(1);
            ret = serialize_cert(cert, list.offset(c2rust_fresh0 as isize));
            if ret != 0 as ::core::ffi::c_int {
                c2rust_current_block = 1556303795062163505;
            } else {
                c2rust_current_block = 11875828834189669668;
            }
        } else {
            c2rust_current_block = 11875828834189669668;
        }
        match c2rust_current_block {
            1556303795062163505 => {}
            _ => {
                if !chain.is_null() {
                    let mut i: ::core::ffi::c_int = 0;
                    i = 0 as ::core::ffi::c_int;
                    loop {
                        if !(i != OPENSSL_sk_num(ossl_check_const_X509_sk_type(chain))) {
                            c2rust_current_block = 7651349459974463963;
                            break;
                        }
                        let c2rust_fresh1 = slot;
                        slot = slot.wrapping_add(1);
                        ret = serialize_cert(
                            OPENSSL_sk_value(ossl_check_const_X509_sk_type(chain), i) as *mut X509,
                            list.offset(c2rust_fresh1 as isize),
                        );
                        if ret != 0 as ::core::ffi::c_int {
                            c2rust_current_block = 1556303795062163505;
                            break;
                        }
                        i += 1;
                    }
                } else {
                    c2rust_current_block = 7651349459974463963;
                }
                match c2rust_current_block {
                    1556303795062163505 => {}
                    _ => {
                        (*ctx).certificates.list = list;
                        (*ctx).certificates.count = count;
                        ret = 0 as ::core::ffi::c_int;
                    }
                }
            }
        }
    }
    if ret != 0 as ::core::ffi::c_int && !list.is_null() {
        let mut i_0: size_t = 0;
        i_0 = 0 as size_t;
        while i_0 != slot {
            free((*list.offset(i_0 as isize)).base as *mut ::core::ffi::c_void);
            i_0 = i_0.wrapping_add(1);
        }
        free(list as *mut ::core::ffi::c_void);
    }
    return ret;
}
unsafe extern "C" fn verify_cert_chain(
    mut store: *mut X509_STORE,
    mut cert: *mut X509,
    mut chain: *mut stack_st_X509,
    mut is_server: ::core::ffi::c_int,
    mut server_name: *const ::core::ffi::c_char,
    mut ossl_x509_err: *mut ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut verify_ctx: *mut X509_STORE_CTX = ::core::ptr::null_mut::<X509_STORE_CTX>();
    let mut ret: ::core::ffi::c_int = 0;
    *ossl_x509_err = 0 as ::core::ffi::c_int;
    verify_ctx = X509_STORE_CTX_new();
    if verify_ctx.is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else if X509_STORE_CTX_init(verify_ctx, store, cert, chain) != 1 as ::core::ffi::c_int {
        ret = PTLS_ERROR_LIBRARY;
    } else {
        let mut params: *mut X509_VERIFY_PARAM = X509_STORE_CTX_get0_param(verify_ctx);
        X509_VERIFY_PARAM_set_purpose(
            params,
            if is_server != 0 {
                X509_PURPOSE_SSL_CLIENT
            } else {
                X509_PURPOSE_SSL_SERVER
            },
        );
        X509_VERIFY_PARAM_set_depth(params, 98 as ::core::ffi::c_int);
        if is_server == 0 && !server_name.is_null() {
            if ptls_server_name_is_ipaddr(server_name) != 0 {
                X509_VERIFY_PARAM_set1_ip_asc(params, server_name);
            } else {
                X509_VERIFY_PARAM_set1_host(params, server_name, strlen(server_name));
                X509_VERIFY_PARAM_set_hostflags(
                    params,
                    X509_CHECK_FLAG_NO_PARTIAL_WILDCARDS as ::core::ffi::c_uint,
                );
            }
        }
        if X509_verify_cert(verify_ctx) != 1 as ::core::ffi::c_int {
            *ossl_x509_err = X509_STORE_CTX_get_error(verify_ctx);
            match *ossl_x509_err {
                X509_V_ERR_OUT_OF_MEM => {
                    ret = PTLS_ERROR_NO_MEMORY;
                }
                X509_V_ERR_CERT_REVOKED => {
                    ret = PTLS_ALERT_CERTIFICATE_REVOKED;
                }
                X509_V_ERR_CERT_NOT_YET_VALID | X509_V_ERR_CERT_HAS_EXPIRED => {
                    ret = PTLS_ALERT_CERTIFICATE_EXPIRED;
                }
                X509_V_ERR_UNABLE_TO_GET_ISSUER_CERT
                | X509_V_ERR_UNABLE_TO_GET_ISSUER_CERT_LOCALLY
                | X509_V_ERR_CERT_UNTRUSTED
                | X509_V_ERR_CERT_REJECTED => {
                    ret = PTLS_ALERT_UNKNOWN_CA;
                }
                X509_V_ERR_HOSTNAME_MISMATCH | X509_V_ERR_INVALID_CA => {
                    ret = PTLS_ALERT_BAD_CERTIFICATE;
                }
                _ => {
                    ret = PTLS_ALERT_CERTIFICATE_UNKNOWN;
                }
            }
        } else {
            ret = 0 as ::core::ffi::c_int;
        }
    }
    if !verify_ctx.is_null() {
        X509_STORE_CTX_free(verify_ctx);
    }
    return ret;
}
unsafe extern "C" fn verify_cert(
    mut _self: *mut ptls_verify_certificate_t,
    mut tls: *mut ptls_t,
    mut server_name: *const ::core::ffi::c_char,
    mut verifier: *mut Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            uint16_t,
            ptls_iovec_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
    mut verify_data: *mut *mut ::core::ffi::c_void,
    mut certs: *mut ptls_iovec_t,
    mut num_certs: size_t,
) -> ::core::ffi::c_int {
    let mut c2rust_current_block: u64;
    let mut self_0: *mut ptls_openssl_verify_certificate_t =
        _self as *mut ptls_openssl_verify_certificate_t;
    let mut cert: *mut X509 = ::core::ptr::null_mut::<X509>();
    let mut chain: *mut stack_st_X509 = OPENSSL_sk_new_null() as *mut stack_st_X509;
    let mut i: size_t = 0;
    let mut ossl_x509_err: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    if num_certs != 0 as size_t {
        cert = to_x509(*certs.offset(0 as ::core::ffi::c_int as isize));
        if cert.is_null() {
            ret = PTLS_ALERT_BAD_CERTIFICATE;
            c2rust_current_block = 5603961367501910430;
        } else {
            i = 1 as size_t;
            loop {
                if !(i != num_certs) {
                    c2rust_current_block = 1917311967535052937;
                    break;
                }
                let mut interm: *mut X509 = to_x509(*certs.offset(i as isize));
                if interm.is_null() {
                    ret = PTLS_ALERT_BAD_CERTIFICATE;
                    c2rust_current_block = 5603961367501910430;
                    break;
                } else {
                    OPENSSL_sk_push(
                        ossl_check_X509_sk_type(chain),
                        ossl_check_X509_type(interm) as *const ::core::ffi::c_void,
                    );
                    i = i.wrapping_add(1);
                }
            }
            match c2rust_current_block {
                5603961367501910430 => {}
                _ => {
                    ret = verify_cert_chain(
                        (*self_0).cert_store,
                        cert,
                        chain,
                        ptls_is_server(tls),
                        server_name,
                        &raw mut ossl_x509_err,
                    );
                    c2rust_current_block = 10599921512955367680;
                }
            }
        }
    } else {
        ret = PTLS_ALERT_CERTIFICATE_REQUIRED;
        ossl_x509_err = 0 as ::core::ffi::c_int;
        c2rust_current_block = 10599921512955367680;
    }
    match c2rust_current_block {
        10599921512955367680 => {
            if !(*self_0).override_callback.is_null() {
                ret = (*(*self_0).override_callback)
                    .cb
                    .expect("non-null function pointer")(
                    (*self_0).override_callback
                        as *mut st_ptls_openssl_override_verify_certificate_t,
                    tls,
                    ret,
                    ossl_x509_err,
                    cert,
                    chain,
                );
            }
            if !(ret != 0 as ::core::ffi::c_int || num_certs == 0 as size_t) {
                *verify_data = X509_get_pubkey(cert) as *mut ::core::ffi::c_void;
                if (*verify_data).is_null() {
                    ret = PTLS_ALERT_BAD_CERTIFICATE;
                } else {
                    *verifier = Some(
                        verify_sign
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                uint16_t,
                                ptls_iovec_t,
                                ptls_iovec_t,
                            )
                                -> ::core::ffi::c_int,
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                uint16_t,
                                ptls_iovec_t,
                                ptls_iovec_t,
                            ) -> ::core::ffi::c_int,
                        >;
                }
            }
        }
        _ => {}
    }
    if !chain.is_null() {
        OPENSSL_sk_pop_free(
            ossl_check_X509_sk_type(chain),
            ossl_check_X509_freefunc_type(Some(X509_free as unsafe extern "C" fn(*mut X509) -> ()))
                as Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>,
        );
    }
    if !cert.is_null() {
        X509_free(cert);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_openssl_init_verify_certificate(
    mut self_0: *mut ptls_openssl_verify_certificate_t,
    mut store: *mut X509_STORE,
) -> ::core::ffi::c_int {
    *self_0 = st_ptls_openssl_verify_certificate_t {
        super_0: st_ptls_verify_certificate_t {
            cb: Some(
                verify_cert
                    as unsafe extern "C" fn(
                        *mut ptls_verify_certificate_t,
                        *mut ptls_t,
                        *const ::core::ffi::c_char,
                        *mut Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                uint16_t,
                                ptls_iovec_t,
                                ptls_iovec_t,
                            ) -> ::core::ffi::c_int,
                        >,
                        *mut *mut ::core::ffi::c_void,
                        *mut ptls_iovec_t,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
            algos: &raw const default_signature_schemes as *const uint16_t,
        },
        cert_store: ::core::ptr::null_mut::<X509_STORE>(),
        override_callback: ::core::ptr::null_mut::<ptls_openssl_override_verify_certificate_t>(),
    };
    if !store.is_null() {
        X509_STORE_up_ref(store);
        (*self_0).cert_store = store;
    } else {
        (*self_0).cert_store = ptls_openssl_create_default_certificate_store();
        if (*self_0).cert_store.is_null() {
            return -(1 as ::core::ffi::c_int);
        }
    }
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_openssl_dispose_verify_certificate(
    mut self_0: *mut ptls_openssl_verify_certificate_t,
) {
    X509_STORE_free((*self_0).cert_store);
}
#[no_mangle]
pub unsafe extern "C" fn ptls_openssl_create_default_certificate_store() -> *mut X509_STORE {
    let mut store: *mut X509_STORE = ::core::ptr::null_mut::<X509_STORE>();
    let mut lookup: *mut X509_LOOKUP = ::core::ptr::null_mut::<X509_LOOKUP>();
    store = X509_STORE_new();
    if !store.is_null() {
        lookup = X509_STORE_add_lookup(store, X509_LOOKUP_file());
        if !lookup.is_null() {
            X509_LOOKUP_ctrl(
                lookup,
                X509_L_FILE_LOAD,
                ::core::ptr::null::<::core::ffi::c_char>(),
                3 as ::core::ffi::c_int as ::core::ffi::c_long,
                ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
            );
            lookup = X509_STORE_add_lookup(store, X509_LOOKUP_hash_dir());
            if !lookup.is_null() {
                X509_LOOKUP_ctrl(
                    lookup,
                    X509_L_ADD_DIR,
                    ::core::ptr::null::<::core::ffi::c_char>(),
                    3 as ::core::ffi::c_int as ::core::ffi::c_long,
                    ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
                );
                return store;
            }
        }
    }
    if !store.is_null() {
        X509_STORE_free(store);
    }
    return ::core::ptr::null_mut::<X509_STORE>();
}
unsafe extern "C" fn verify_raw_cert(
    mut _self: *mut ptls_verify_certificate_t,
    mut tls: *mut ptls_t,
    mut server_name: *const ::core::ffi::c_char,
    mut verifier: *mut Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_void,
            uint16_t,
            ptls_iovec_t,
            ptls_iovec_t,
        ) -> ::core::ffi::c_int,
    >,
    mut verify_data: *mut *mut ::core::ffi::c_void,
    mut certs: *mut ptls_iovec_t,
    mut num_certs: size_t,
) -> ::core::ffi::c_int {
    let mut r: ::core::ffi::c_int = 0;
    let mut self_0: *mut ptls_openssl_raw_pubkey_verify_certificate_t =
        _self as *mut ptls_openssl_raw_pubkey_verify_certificate_t;
    let mut ret: ::core::ffi::c_int = PTLS_ALERT_BAD_CERTIFICATE;
    let mut expected_pubkey: ptls_iovec_t = st_ptls_iovec_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    };
    if !(num_certs != 1 as size_t) {
        r = i2d_PUBKEY((*self_0).expected_pubkey, &raw mut expected_pubkey.base);
        if r <= 0 as ::core::ffi::c_int {
            ret = PTLS_ALERT_BAD_CERTIFICATE;
        } else {
            expected_pubkey.len = r as size_t;
            if !((*certs.offset(0 as ::core::ffi::c_int as isize)).len != expected_pubkey.len) {
                if !(ptls_mem_equal.expect("non-null function pointer")(
                    expected_pubkey.base as *const ::core::ffi::c_void,
                    (*certs.offset(0 as ::core::ffi::c_int as isize)).base
                        as *const ::core::ffi::c_void,
                    (*certs.offset(0 as ::core::ffi::c_int as isize)).len,
                ) == 0)
                {
                    EVP_PKEY_up_ref((*self_0).expected_pubkey);
                    *verify_data = (*self_0).expected_pubkey as *mut ::core::ffi::c_void;
                    *verifier = Some(
                        verify_sign
                            as unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                uint16_t,
                                ptls_iovec_t,
                                ptls_iovec_t,
                            )
                                -> ::core::ffi::c_int,
                    )
                        as Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                uint16_t,
                                ptls_iovec_t,
                                ptls_iovec_t,
                            ) -> ::core::ffi::c_int,
                        >;
                    ret = 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    CRYPTO_free(
        expected_pubkey.base as *mut ::core::ffi::c_void,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/lib/openssl.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        1737 as ::core::ffi::c_int,
    );
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_openssl_raw_pubkey_init_verify_certificate(
    mut self_0: *mut ptls_openssl_raw_pubkey_verify_certificate_t,
    mut expected_pubkey: *mut EVP_PKEY,
) -> ::core::ffi::c_int {
    EVP_PKEY_up_ref(expected_pubkey);
    *self_0 = st_ptls_openssl_raw_pubkey_verify_certificate_t {
        super_0: st_ptls_verify_certificate_t {
            cb: Some(
                verify_raw_cert
                    as unsafe extern "C" fn(
                        *mut ptls_verify_certificate_t,
                        *mut ptls_t,
                        *const ::core::ffi::c_char,
                        *mut Option<
                            unsafe extern "C" fn(
                                *mut ::core::ffi::c_void,
                                uint16_t,
                                ptls_iovec_t,
                                ptls_iovec_t,
                            ) -> ::core::ffi::c_int,
                        >,
                        *mut *mut ::core::ffi::c_void,
                        *mut ptls_iovec_t,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
            algos: &raw const default_signature_schemes as *const uint16_t,
        },
        expected_pubkey: expected_pubkey,
    };
    return 0 as ::core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_openssl_raw_pubkey_dispose_verify_certificate(
    mut self_0: *mut ptls_openssl_raw_pubkey_verify_certificate_t,
) {
    EVP_PKEY_free((*self_0).expected_pubkey);
}
pub const TICKET_LABEL_SIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
pub const TICKET_IV_SIZE: ::core::ffi::c_int = EVP_MAX_IV_LENGTH;
#[no_mangle]
pub unsafe extern "C" fn ptls_openssl_encrypt_ticket(
    mut buf: *mut ptls_buffer_t,
    mut src: ptls_iovec_t,
    mut cb: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_uchar,
            *mut ::core::ffi::c_uchar,
            *mut EVP_CIPHER_CTX,
            *mut HMAC_CTX,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
) -> ::core::ffi::c_int {
    let mut cctx: *mut EVP_CIPHER_CTX = ::core::ptr::null_mut::<EVP_CIPHER_CTX>();
    let mut hctx: *mut HMAC_CTX = ::core::ptr::null_mut::<HMAC_CTX>();
    let mut dst: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut clen: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    cctx = EVP_CIPHER_CTX_new();
    if cctx.is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else {
        hctx = HMAC_CTX_new();
        if hctx.is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        } else {
            ret = ptls_buffer_reserve(
                buf,
                ((TICKET_LABEL_SIZE + TICKET_IV_SIZE) as size_t)
                    .wrapping_add(src.len)
                    .wrapping_add(EVP_MAX_BLOCK_LENGTH as size_t)
                    .wrapping_add(EVP_MAX_MD_SIZE as size_t),
            );
            if !(ret != 0 as ::core::ffi::c_int) {
                dst = (*buf).base.offset((*buf).off as isize);
                if Some(cb.expect("non-null function pointer")).expect("non-null function pointer")(
                    dst as *mut ::core::ffi::c_uchar,
                    dst.offset(TICKET_LABEL_SIZE as isize),
                    cctx,
                    hctx,
                    1 as ::core::ffi::c_int,
                ) == 0
                {
                    ret = PTLS_ERROR_LIBRARY;
                } else {
                    dst = dst.offset((TICKET_LABEL_SIZE + TICKET_IV_SIZE) as isize);
                    if EVP_EncryptUpdate(
                        cctx,
                        dst as *mut ::core::ffi::c_uchar,
                        &raw mut clen,
                        src.base,
                        src.len as ::core::ffi::c_int,
                    ) == 0
                    {
                        ret = PTLS_ERROR_LIBRARY;
                    } else {
                        dst = dst.offset(clen as isize);
                        if EVP_EncryptFinal_ex(
                            cctx,
                            dst as *mut ::core::ffi::c_uchar,
                            &raw mut clen,
                        ) == 0
                        {
                            ret = PTLS_ERROR_LIBRARY;
                        } else {
                            dst = dst.offset(clen as isize);
                            if HMAC_Update(
                                hctx,
                                (*buf).base.offset((*buf).off as isize),
                                dst.offset_from((*buf).base.offset((*buf).off as isize))
                                    as ::core::ffi::c_long
                                    as size_t,
                            ) == 0
                                || HMAC_Final(
                                    hctx,
                                    dst as *mut ::core::ffi::c_uchar,
                                    ::core::ptr::null_mut::<::core::ffi::c_uint>(),
                                ) == 0
                            {
                                ret = PTLS_ERROR_LIBRARY;
                            } else {
                                dst = dst.offset(HMAC_size(hctx) as isize);
                                (*buf).off = (*buf).off.wrapping_add(
                                    dst.offset_from((*buf).base.offset((*buf).off as isize))
                                        as ::core::ffi::c_long
                                        as size_t,
                                );
                                ret = 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    if !cctx.is_null() {
        EVP_CIPHER_CTX_free(cctx);
    }
    if !hctx.is_null() {
        HMAC_CTX_free(hctx);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_openssl_decrypt_ticket(
    mut buf: *mut ptls_buffer_t,
    mut src: ptls_iovec_t,
    mut cb: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_uchar,
            *mut ::core::ffi::c_uchar,
            *mut EVP_CIPHER_CTX,
            *mut HMAC_CTX,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
) -> ::core::ffi::c_int {
    let mut hmac_size: size_t = 0;
    let mut hmac: [uint8_t; 64] = [0; 64];
    let mut cctx: *mut EVP_CIPHER_CTX = ::core::ptr::null_mut::<EVP_CIPHER_CTX>();
    let mut hctx: *mut HMAC_CTX = ::core::ptr::null_mut::<HMAC_CTX>();
    let mut clen: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    cctx = EVP_CIPHER_CTX_new();
    if cctx.is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else {
        hctx = HMAC_CTX_new();
        if hctx.is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        } else if src.len < (TICKET_LABEL_SIZE + TICKET_IV_SIZE) as size_t {
            ret = PTLS_ALERT_DECODE_ERROR;
        } else if Some(cb.expect("non-null function pointer")).expect("non-null function pointer")(
            src.base as *mut ::core::ffi::c_uchar,
            src.base.offset(TICKET_LABEL_SIZE as isize),
            cctx,
            hctx,
            0 as ::core::ffi::c_int,
        ) == 0
        {
            ret = PTLS_ERROR_LIBRARY;
        } else {
            hmac_size = HMAC_size(hctx);
            if src.len < ((TICKET_LABEL_SIZE + TICKET_IV_SIZE) as size_t).wrapping_add(hmac_size) {
                ret = PTLS_ALERT_DECODE_ERROR;
            } else {
                src.len = src.len.wrapping_sub(hmac_size);
                hmac = [0; 64];
                if HMAC_Update(hctx, src.base, src.len) == 0
                    || HMAC_Final(
                        hctx,
                        &raw mut hmac as *mut ::core::ffi::c_uchar,
                        ::core::ptr::null_mut::<::core::ffi::c_uint>(),
                    ) == 0
                {
                    ret = PTLS_ERROR_LIBRARY;
                } else if ptls_mem_equal.expect("non-null function pointer")(
                    src.base.offset(src.len as isize) as *const ::core::ffi::c_void,
                    &raw mut hmac as *mut uint8_t as *const ::core::ffi::c_void,
                    hmac_size,
                ) == 0
                {
                    ret = PTLS_ALERT_HANDSHAKE_FAILURE;
                } else {
                    src.base = src
                        .base
                        .offset((TICKET_LABEL_SIZE + TICKET_IV_SIZE) as isize);
                    src.len = src
                        .len
                        .wrapping_sub((TICKET_LABEL_SIZE + TICKET_IV_SIZE) as size_t);
                    ret = ptls_buffer_reserve(buf, src.len);
                    if !(ret != 0 as ::core::ffi::c_int) {
                        if EVP_DecryptUpdate(
                            cctx,
                            (*buf).base.offset((*buf).off as isize),
                            &raw mut clen,
                            src.base,
                            src.len as ::core::ffi::c_int,
                        ) == 0
                        {
                            ret = PTLS_ERROR_LIBRARY;
                        } else {
                            (*buf).off = (*buf).off.wrapping_add(clen as size_t);
                            if EVP_DecryptFinal_ex(
                                cctx,
                                (*buf).base.offset((*buf).off as isize),
                                &raw mut clen,
                            ) == 0
                            {
                                ret = PTLS_ERROR_LIBRARY;
                            } else {
                                (*buf).off = (*buf).off.wrapping_add(clen as size_t);
                                ret = 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    if !cctx.is_null() {
        EVP_CIPHER_CTX_free(cctx);
    }
    if !hctx.is_null() {
        HMAC_CTX_free(hctx);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_openssl_encrypt_ticket_evp(
    mut buf: *mut ptls_buffer_t,
    mut src: ptls_iovec_t,
    mut cb: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_uchar,
            *mut ::core::ffi::c_uchar,
            *mut EVP_CIPHER_CTX,
            *mut EVP_MAC_CTX,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
) -> ::core::ffi::c_int {
    let mut cctx: *mut EVP_CIPHER_CTX = ::core::ptr::null_mut::<EVP_CIPHER_CTX>();
    let mut mac: *mut EVP_MAC = ::core::ptr::null_mut::<EVP_MAC>();
    let mut hctx: *mut EVP_MAC_CTX = ::core::ptr::null_mut::<EVP_MAC_CTX>();
    let mut hlen: size_t = 0;
    let mut dst: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut clen: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    cctx = EVP_CIPHER_CTX_new();
    if cctx.is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else {
        mac = EVP_MAC_fetch(
            ::core::ptr::null_mut::<OSSL_LIB_CTX>(),
            b"HMAC\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if mac.is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        } else {
            hctx = EVP_MAC_CTX_new(mac);
            if hctx.is_null() {
                ret = PTLS_ERROR_NO_MEMORY;
            } else {
                ret = ptls_buffer_reserve(
                    buf,
                    ((TICKET_LABEL_SIZE + TICKET_IV_SIZE) as size_t)
                        .wrapping_add(src.len)
                        .wrapping_add(EVP_MAX_BLOCK_LENGTH as size_t)
                        .wrapping_add(EVP_MAX_MD_SIZE as size_t),
                );
                if !(ret != 0 as ::core::ffi::c_int) {
                    dst = (*buf).base.offset((*buf).off as isize);
                    if Some(cb.expect("non-null function pointer"))
                        .expect("non-null function pointer")(
                        dst as *mut ::core::ffi::c_uchar,
                        dst.offset(TICKET_LABEL_SIZE as isize),
                        cctx,
                        hctx,
                        1 as ::core::ffi::c_int,
                    ) == 0
                    {
                        ret = PTLS_ERROR_LIBRARY;
                    } else {
                        dst = dst.offset((TICKET_LABEL_SIZE + TICKET_IV_SIZE) as isize);
                        if EVP_EncryptUpdate(
                            cctx,
                            dst as *mut ::core::ffi::c_uchar,
                            &raw mut clen,
                            src.base,
                            src.len as ::core::ffi::c_int,
                        ) == 0
                        {
                            ret = PTLS_ERROR_LIBRARY;
                        } else {
                            dst = dst.offset(clen as isize);
                            if EVP_EncryptFinal_ex(
                                cctx,
                                dst as *mut ::core::ffi::c_uchar,
                                &raw mut clen,
                            ) == 0
                            {
                                ret = PTLS_ERROR_LIBRARY;
                            } else {
                                dst = dst.offset(clen as isize);
                                if EVP_MAC_update(
                                    hctx,
                                    (*buf).base.offset((*buf).off as isize),
                                    dst.offset_from((*buf).base.offset((*buf).off as isize))
                                        as ::core::ffi::c_long
                                        as size_t,
                                ) == 0
                                    || EVP_MAC_final(
                                        hctx,
                                        dst as *mut ::core::ffi::c_uchar,
                                        &raw mut hlen,
                                        EVP_MAC_CTX_get_mac_size(hctx),
                                    ) == 0
                                {
                                    ret = PTLS_ERROR_LIBRARY;
                                } else {
                                    dst = dst.offset(hlen as isize);
                                    (*buf).off = (*buf).off.wrapping_add(
                                        dst.offset_from((*buf).base.offset((*buf).off as isize))
                                            as ::core::ffi::c_long
                                            as size_t,
                                    );
                                    ret = 0 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !cctx.is_null() {
        EVP_CIPHER_CTX_free(cctx);
    }
    if !hctx.is_null() {
        EVP_MAC_CTX_free(hctx);
    }
    if !mac.is_null() {
        EVP_MAC_free(mac);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_openssl_decrypt_ticket_evp(
    mut buf: *mut ptls_buffer_t,
    mut src: ptls_iovec_t,
    mut cb: Option<
        unsafe extern "C" fn(
            *mut ::core::ffi::c_uchar,
            *mut ::core::ffi::c_uchar,
            *mut EVP_CIPHER_CTX,
            *mut EVP_MAC_CTX,
            ::core::ffi::c_int,
        ) -> ::core::ffi::c_int,
    >,
) -> ::core::ffi::c_int {
    let mut hmac_size: size_t = 0;
    let mut hmac: [uint8_t; 64] = [0; 64];
    let mut cctx: *mut EVP_CIPHER_CTX = ::core::ptr::null_mut::<EVP_CIPHER_CTX>();
    let mut mac: *mut EVP_MAC = ::core::ptr::null_mut::<EVP_MAC>();
    let mut hctx: *mut EVP_MAC_CTX = ::core::ptr::null_mut::<EVP_MAC_CTX>();
    let mut hlen: size_t = 0;
    let mut clen: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    cctx = EVP_CIPHER_CTX_new();
    if cctx.is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else {
        mac = EVP_MAC_fetch(
            ::core::ptr::null_mut::<OSSL_LIB_CTX>(),
            b"HMAC\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
        );
        if mac.is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        } else {
            hctx = EVP_MAC_CTX_new(mac);
            if hctx.is_null() {
                ret = PTLS_ERROR_NO_MEMORY;
            } else if src.len < (TICKET_LABEL_SIZE + TICKET_IV_SIZE) as size_t {
                ret = PTLS_ALERT_DECODE_ERROR;
            } else if Some(cb.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                src.base as *mut ::core::ffi::c_uchar,
                src.base.offset(TICKET_LABEL_SIZE as isize),
                cctx,
                hctx,
                0 as ::core::ffi::c_int,
            ) == 0
            {
                ret = PTLS_ERROR_LIBRARY;
            } else {
                hmac_size = EVP_MAC_CTX_get_mac_size(hctx);
                if src.len
                    < ((TICKET_LABEL_SIZE + TICKET_IV_SIZE) as size_t).wrapping_add(hmac_size)
                {
                    ret = PTLS_ALERT_DECODE_ERROR;
                } else {
                    src.len = src.len.wrapping_sub(hmac_size);
                    hmac = [0; 64];
                    if EVP_MAC_update(hctx, src.base, src.len) == 0
                        || EVP_MAC_final(
                            hctx,
                            &raw mut hmac as *mut ::core::ffi::c_uchar,
                            &raw mut hlen,
                            ::core::mem::size_of::<[uint8_t; 64]>() as size_t,
                        ) == 0
                    {
                        ret = PTLS_ERROR_LIBRARY;
                    } else if ptls_mem_equal.expect("non-null function pointer")(
                        src.base.offset(src.len as isize) as *const ::core::ffi::c_void,
                        &raw mut hmac as *mut uint8_t as *const ::core::ffi::c_void,
                        hmac_size,
                    ) == 0
                    {
                        ret = PTLS_ALERT_HANDSHAKE_FAILURE;
                    } else {
                        src.base = src
                            .base
                            .offset((TICKET_LABEL_SIZE + TICKET_IV_SIZE) as isize);
                        src.len = src
                            .len
                            .wrapping_sub((TICKET_LABEL_SIZE + TICKET_IV_SIZE) as size_t);
                        ret = ptls_buffer_reserve(buf, src.len);
                        if !(ret != 0 as ::core::ffi::c_int) {
                            if EVP_DecryptUpdate(
                                cctx,
                                (*buf).base.offset((*buf).off as isize),
                                &raw mut clen,
                                src.base,
                                src.len as ::core::ffi::c_int,
                            ) == 0
                            {
                                ret = PTLS_ERROR_LIBRARY;
                            } else {
                                (*buf).off = (*buf).off.wrapping_add(clen as size_t);
                                if EVP_DecryptFinal_ex(
                                    cctx,
                                    (*buf).base.offset((*buf).off as isize),
                                    &raw mut clen,
                                ) == 0
                                {
                                    ret = PTLS_ERROR_LIBRARY;
                                } else {
                                    (*buf).off = (*buf).off.wrapping_add(clen as size_t);
                                    ret = 0 as ::core::ffi::c_int;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    if !cctx.is_null() {
        EVP_CIPHER_CTX_free(cctx);
    }
    if !hctx.is_null() {
        EVP_MAC_CTX_free(hctx);
    }
    if !mac.is_null() {
        EVP_MAC_free(mac);
    }
    return ret;
}
#[no_mangle]
pub static mut ptls_openssl_secp256r1: ptls_key_exchange_algorithm_t = unsafe {
    st_ptls_key_exchange_algorithm_t {
        id: PTLS_GROUP_SECP256R1 as uint16_t,
        create: Some(
            x9_62_create_key_exchange
                as unsafe extern "C" fn(
                    *const ptls_key_exchange_algorithm_t,
                    *mut *mut ptls_key_exchange_context_t,
                ) -> ::core::ffi::c_int,
        ),
        exchange: Some(
            secp_key_exchange
                as unsafe extern "C" fn(
                    *const ptls_key_exchange_algorithm_t,
                    *mut ptls_iovec_t,
                    *mut ptls_iovec_t,
                    ptls_iovec_t,
                ) -> ::core::ffi::c_int,
        ),
        data: NID_X9_62_prime256v1 as intptr_t,
        name: PTLS_GROUP_NAME_SECP256R1.as_ptr(),
    }
};
#[no_mangle]
pub static mut ptls_openssl_secp384r1: ptls_key_exchange_algorithm_t = unsafe {
    st_ptls_key_exchange_algorithm_t {
        id: PTLS_GROUP_SECP384R1 as uint16_t,
        create: Some(
            x9_62_create_key_exchange
                as unsafe extern "C" fn(
                    *const ptls_key_exchange_algorithm_t,
                    *mut *mut ptls_key_exchange_context_t,
                ) -> ::core::ffi::c_int,
        ),
        exchange: Some(
            secp_key_exchange
                as unsafe extern "C" fn(
                    *const ptls_key_exchange_algorithm_t,
                    *mut ptls_iovec_t,
                    *mut ptls_iovec_t,
                    ptls_iovec_t,
                ) -> ::core::ffi::c_int,
        ),
        data: NID_secp384r1 as intptr_t,
        name: PTLS_GROUP_NAME_SECP384R1.as_ptr(),
    }
};
#[no_mangle]
pub static mut ptls_openssl_secp521r1: ptls_key_exchange_algorithm_t = unsafe {
    st_ptls_key_exchange_algorithm_t {
        id: PTLS_GROUP_SECP521R1 as uint16_t,
        create: Some(
            x9_62_create_key_exchange
                as unsafe extern "C" fn(
                    *const ptls_key_exchange_algorithm_t,
                    *mut *mut ptls_key_exchange_context_t,
                ) -> ::core::ffi::c_int,
        ),
        exchange: Some(
            secp_key_exchange
                as unsafe extern "C" fn(
                    *const ptls_key_exchange_algorithm_t,
                    *mut ptls_iovec_t,
                    *mut ptls_iovec_t,
                    ptls_iovec_t,
                ) -> ::core::ffi::c_int,
        ),
        data: NID_secp521r1 as intptr_t,
        name: PTLS_GROUP_NAME_SECP521R1.as_ptr(),
    }
};
#[no_mangle]
pub static mut ptls_openssl_x25519: ptls_key_exchange_algorithm_t = unsafe {
    st_ptls_key_exchange_algorithm_t {
        id: PTLS_GROUP_X25519 as uint16_t,
        create: Some(
            evp_keyex_create
                as unsafe extern "C" fn(
                    *const ptls_key_exchange_algorithm_t,
                    *mut *mut ptls_key_exchange_context_t,
                ) -> ::core::ffi::c_int,
        ),
        exchange: Some(
            evp_keyex_exchange
                as unsafe extern "C" fn(
                    *const ptls_key_exchange_algorithm_t,
                    *mut ptls_iovec_t,
                    *mut ptls_iovec_t,
                    ptls_iovec_t,
                ) -> ::core::ffi::c_int,
        ),
        data: NID_X25519 as intptr_t,
        name: PTLS_GROUP_NAME_X25519.as_ptr(),
    }
};
#[no_mangle]
pub static mut ptls_openssl_key_exchanges: [*const ptls_key_exchange_algorithm_t; 2] = unsafe {
    [
        &raw const ptls_openssl_secp256r1,
        ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
    ]
};
#[no_mangle]
pub static mut ptls_openssl_aes128ecb: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"AES128-ECB\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: PTLS_AES128_KEY_SIZE as size_t,
        block_size: PTLS_AES_BLOCK_SIZE as size_t,
        iv_size: 0 as size_t,
        context_size: ::core::mem::size_of::<cipher_context_t>() as size_t,
        setup_crypto: Some(
            aes128ecb_setup_crypto
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
#[no_mangle]
pub static mut ptls_openssl_aes128ctr: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"AES128-CTR\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: PTLS_AES128_KEY_SIZE as size_t,
        block_size: 1 as size_t,
        iv_size: PTLS_AES_IV_SIZE as size_t,
        context_size: ::core::mem::size_of::<cipher_context_t>() as size_t,
        setup_crypto: Some(
            aes128ctr_setup_crypto
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
#[no_mangle]
pub static mut ptls_openssl_aes128gcm: ptls_aead_algorithm_t = st_ptls_aead_algorithm_t {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    confidentiality_limit: 0,
    integrity_limit: 0,
    ctr_cipher: ::core::ptr::null::<ptls_cipher_algorithm_t>(),
    ecb_cipher: ::core::ptr::null::<ptls_cipher_algorithm_t>(),
    key_size: 0,
    iv_size: 0,
    tag_size: 0,
    tls12: C2Rust_Unnamed_2 {
        fixed_iv_size: 0,
        record_iv_size: 0,
    },
    non_temporal: [0; 1],
    align_bits: 0,
    context_size: 0,
    setup_crypto: None,
};
#[no_mangle]
pub static mut ptls_openssl_aes256ecb: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"AES256-ECB\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: PTLS_AES256_KEY_SIZE as size_t,
        block_size: PTLS_AES_BLOCK_SIZE as size_t,
        iv_size: 0 as size_t,
        context_size: ::core::mem::size_of::<cipher_context_t>() as size_t,
        setup_crypto: Some(
            aes256ecb_setup_crypto
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
#[no_mangle]
pub static mut ptls_openssl_aes256ctr: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"AES256-CTR\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: PTLS_AES256_KEY_SIZE as size_t,
        block_size: 1 as size_t,
        iv_size: PTLS_AES_IV_SIZE as size_t,
        context_size: ::core::mem::size_of::<cipher_context_t>() as size_t,
        setup_crypto: Some(
            aes256ctr_setup_crypto
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
#[no_mangle]
pub static mut ptls_openssl_aes256gcm: ptls_aead_algorithm_t = st_ptls_aead_algorithm_t {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    confidentiality_limit: 0,
    integrity_limit: 0,
    ctr_cipher: ::core::ptr::null::<ptls_cipher_algorithm_t>(),
    ecb_cipher: ::core::ptr::null::<ptls_cipher_algorithm_t>(),
    key_size: 0,
    iv_size: 0,
    tag_size: 0,
    tls12: C2Rust_Unnamed_2 {
        fixed_iv_size: 0,
        record_iv_size: 0,
    },
    non_temporal: [0; 1],
    align_bits: 0,
    context_size: 0,
    setup_crypto: None,
};
#[no_mangle]
pub static mut ptls_openssl_sha256: ptls_hash_algorithm_t = unsafe {
    st_ptls_hash_algorithm_t {
        name: b"sha256\0".as_ptr() as *const ::core::ffi::c_char,
        block_size: PTLS_SHA256_BLOCK_SIZE as size_t,
        digest_size: PTLS_SHA256_DIGEST_SIZE as size_t,
        create: Some(sha256_create as unsafe extern "C" fn() -> *mut ptls_hash_context_t),
        empty_digest: [
            0xe3 as ::core::ffi::c_int as uint8_t,
            0xb0 as ::core::ffi::c_int as uint8_t,
            0xc4 as ::core::ffi::c_int as uint8_t,
            0x42 as ::core::ffi::c_int as uint8_t,
            0x98 as ::core::ffi::c_int as uint8_t,
            0xfc as ::core::ffi::c_int as uint8_t,
            0x1c as ::core::ffi::c_int as uint8_t,
            0x14 as ::core::ffi::c_int as uint8_t,
            0x9a as ::core::ffi::c_int as uint8_t,
            0xfb as ::core::ffi::c_int as uint8_t,
            0xf4 as ::core::ffi::c_int as uint8_t,
            0xc8 as ::core::ffi::c_int as uint8_t,
            0x99 as ::core::ffi::c_int as uint8_t,
            0x6f as ::core::ffi::c_int as uint8_t,
            0xb9 as ::core::ffi::c_int as uint8_t,
            0x24 as ::core::ffi::c_int as uint8_t,
            0x27 as ::core::ffi::c_int as uint8_t,
            0xae as ::core::ffi::c_int as uint8_t,
            0x41 as ::core::ffi::c_int as uint8_t,
            0xe4 as ::core::ffi::c_int as uint8_t,
            0x64 as ::core::ffi::c_int as uint8_t,
            0x9b as ::core::ffi::c_int as uint8_t,
            0x93 as ::core::ffi::c_int as uint8_t,
            0x4c as ::core::ffi::c_int as uint8_t,
            0xa4 as ::core::ffi::c_int as uint8_t,
            0x95 as ::core::ffi::c_int as uint8_t,
            0x99 as ::core::ffi::c_int as uint8_t,
            0x1b as ::core::ffi::c_int as uint8_t,
            0x78 as ::core::ffi::c_int as uint8_t,
            0x52 as ::core::ffi::c_int as uint8_t,
            0xb8 as ::core::ffi::c_int as uint8_t,
            0x55 as ::core::ffi::c_int as uint8_t,
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
            0,
        ],
    }
};
#[no_mangle]
pub static mut ptls_openssl_sha384: ptls_hash_algorithm_t = unsafe {
    st_ptls_hash_algorithm_t {
        name: b"sha384\0".as_ptr() as *const ::core::ffi::c_char,
        block_size: PTLS_SHA384_BLOCK_SIZE as size_t,
        digest_size: PTLS_SHA384_DIGEST_SIZE as size_t,
        create: Some(sha384_create as unsafe extern "C" fn() -> *mut ptls_hash_context_t),
        empty_digest: [
            0x38 as ::core::ffi::c_int as uint8_t,
            0xb0 as ::core::ffi::c_int as uint8_t,
            0x60 as ::core::ffi::c_int as uint8_t,
            0xa7 as ::core::ffi::c_int as uint8_t,
            0x51 as ::core::ffi::c_int as uint8_t,
            0xac as ::core::ffi::c_int as uint8_t,
            0x96 as ::core::ffi::c_int as uint8_t,
            0x38 as ::core::ffi::c_int as uint8_t,
            0x4c as ::core::ffi::c_int as uint8_t,
            0xd9 as ::core::ffi::c_int as uint8_t,
            0x32 as ::core::ffi::c_int as uint8_t,
            0x7e as ::core::ffi::c_int as uint8_t,
            0xb1 as ::core::ffi::c_int as uint8_t,
            0xb1 as ::core::ffi::c_int as uint8_t,
            0xe3 as ::core::ffi::c_int as uint8_t,
            0x6a as ::core::ffi::c_int as uint8_t,
            0x21 as ::core::ffi::c_int as uint8_t,
            0xfd as ::core::ffi::c_int as uint8_t,
            0xb7 as ::core::ffi::c_int as uint8_t,
            0x11 as ::core::ffi::c_int as uint8_t,
            0x14 as ::core::ffi::c_int as uint8_t,
            0xbe as ::core::ffi::c_int as uint8_t,
            0x7 as ::core::ffi::c_int as uint8_t,
            0x43 as ::core::ffi::c_int as uint8_t,
            0x4c as ::core::ffi::c_int as uint8_t,
            0xc as ::core::ffi::c_int as uint8_t,
            0xc7 as ::core::ffi::c_int as uint8_t,
            0xbf as ::core::ffi::c_int as uint8_t,
            0x63 as ::core::ffi::c_int as uint8_t,
            0xf6 as ::core::ffi::c_int as uint8_t,
            0xe1 as ::core::ffi::c_int as uint8_t,
            0xda as ::core::ffi::c_int as uint8_t,
            0x27 as ::core::ffi::c_int as uint8_t,
            0x4e as ::core::ffi::c_int as uint8_t,
            0xde as ::core::ffi::c_int as uint8_t,
            0xbf as ::core::ffi::c_int as uint8_t,
            0xe7 as ::core::ffi::c_int as uint8_t,
            0x6f as ::core::ffi::c_int as uint8_t,
            0x65 as ::core::ffi::c_int as uint8_t,
            0xfb as ::core::ffi::c_int as uint8_t,
            0xd5 as ::core::ffi::c_int as uint8_t,
            0x1a as ::core::ffi::c_int as uint8_t,
            0xd2 as ::core::ffi::c_int as uint8_t,
            0xf1 as ::core::ffi::c_int as uint8_t,
            0x48 as ::core::ffi::c_int as uint8_t,
            0x98 as ::core::ffi::c_int as uint8_t,
            0xb9 as ::core::ffi::c_int as uint8_t,
            0x5b as ::core::ffi::c_int as uint8_t,
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
        ],
    }
};
#[no_mangle]
pub static mut ptls_openssl_sha512: ptls_hash_algorithm_t = unsafe {
    st_ptls_hash_algorithm_t {
        name: b"sha512\0".as_ptr() as *const ::core::ffi::c_char,
        block_size: PTLS_SHA512_BLOCK_SIZE as size_t,
        digest_size: PTLS_SHA512_DIGEST_SIZE as size_t,
        create: Some(sha512_create as unsafe extern "C" fn() -> *mut ptls_hash_context_t),
        empty_digest: [
            0xcf as ::core::ffi::c_int as uint8_t,
            0x83 as ::core::ffi::c_int as uint8_t,
            0xe1 as ::core::ffi::c_int as uint8_t,
            0x35 as ::core::ffi::c_int as uint8_t,
            0x7e as ::core::ffi::c_int as uint8_t,
            0xef as ::core::ffi::c_int as uint8_t,
            0xb8 as ::core::ffi::c_int as uint8_t,
            0xbd as ::core::ffi::c_int as uint8_t,
            0xf1 as ::core::ffi::c_int as uint8_t,
            0x54 as ::core::ffi::c_int as uint8_t,
            0x28 as ::core::ffi::c_int as uint8_t,
            0x50 as ::core::ffi::c_int as uint8_t,
            0xd6 as ::core::ffi::c_int as uint8_t,
            0x6d as ::core::ffi::c_int as uint8_t,
            0x80 as ::core::ffi::c_int as uint8_t,
            0x7 as ::core::ffi::c_int as uint8_t,
            0xd6 as ::core::ffi::c_int as uint8_t,
            0x20 as ::core::ffi::c_int as uint8_t,
            0xe4 as ::core::ffi::c_int as uint8_t,
            0x5 as ::core::ffi::c_int as uint8_t,
            0xb as ::core::ffi::c_int as uint8_t,
            0x57 as ::core::ffi::c_int as uint8_t,
            0x15 as ::core::ffi::c_int as uint8_t,
            0xdc as ::core::ffi::c_int as uint8_t,
            0x83 as ::core::ffi::c_int as uint8_t,
            0xf4 as ::core::ffi::c_int as uint8_t,
            0xa9 as ::core::ffi::c_int as uint8_t,
            0x21 as ::core::ffi::c_int as uint8_t,
            0xd3 as ::core::ffi::c_int as uint8_t,
            0x6c as ::core::ffi::c_int as uint8_t,
            0xe9 as ::core::ffi::c_int as uint8_t,
            0xce as ::core::ffi::c_int as uint8_t,
            0x47 as ::core::ffi::c_int as uint8_t,
            0xd0 as ::core::ffi::c_int as uint8_t,
            0xd1 as ::core::ffi::c_int as uint8_t,
            0x3c as ::core::ffi::c_int as uint8_t,
            0x5d as ::core::ffi::c_int as uint8_t,
            0x85 as ::core::ffi::c_int as uint8_t,
            0xf2 as ::core::ffi::c_int as uint8_t,
            0xb0 as ::core::ffi::c_int as uint8_t,
            0xff as ::core::ffi::c_int as uint8_t,
            0x83 as ::core::ffi::c_int as uint8_t,
            0x18 as ::core::ffi::c_int as uint8_t,
            0xd2 as ::core::ffi::c_int as uint8_t,
            0x87 as ::core::ffi::c_int as uint8_t,
            0x7e as ::core::ffi::c_int as uint8_t,
            0xec as ::core::ffi::c_int as uint8_t,
            0x2f as ::core::ffi::c_int as uint8_t,
            0x63 as ::core::ffi::c_int as uint8_t,
            0xb9 as ::core::ffi::c_int as uint8_t,
            0x31 as ::core::ffi::c_int as uint8_t,
            0xbd as ::core::ffi::c_int as uint8_t,
            0x47 as ::core::ffi::c_int as uint8_t,
            0x41 as ::core::ffi::c_int as uint8_t,
            0x7a as ::core::ffi::c_int as uint8_t,
            0x81 as ::core::ffi::c_int as uint8_t,
            0xa5 as ::core::ffi::c_int as uint8_t,
            0x38 as ::core::ffi::c_int as uint8_t,
            0x32 as ::core::ffi::c_int as uint8_t,
            0x7a as ::core::ffi::c_int as uint8_t,
            0xf9 as ::core::ffi::c_int as uint8_t,
            0x27 as ::core::ffi::c_int as uint8_t,
            0xda as ::core::ffi::c_int as uint8_t,
            0x3e as ::core::ffi::c_int as uint8_t,
        ],
    }
};
#[no_mangle]
pub static mut ptls_openssl_aes128gcmsha256: ptls_cipher_suite_t = unsafe {
    st_ptls_cipher_suite_t {
        id: PTLS_CIPHER_SUITE_AES_128_GCM_SHA256 as uint16_t,
        aead: &raw const ptls_openssl_aes128gcm,
        hash: &raw const ptls_openssl_sha256,
        name: PTLS_CIPHER_SUITE_NAME_AES_128_GCM_SHA256.as_ptr(),
    }
};
#[no_mangle]
pub static mut ptls_openssl_tls12_ecdhe_rsa_aes128gcmsha256: ptls_cipher_suite_t = unsafe {
    st_ptls_cipher_suite_t {
        id: PTLS_CIPHER_SUITE_ECDHE_RSA_WITH_AES_128_GCM_SHA256 as uint16_t,
        aead: &raw const ptls_openssl_aes128gcm,
        hash: &raw const ptls_openssl_sha256,
        name: PTLS_CIPHER_SUITE_NAME_ECDHE_RSA_WITH_AES_128_GCM_SHA256.as_ptr(),
    }
};
#[no_mangle]
pub static mut ptls_openssl_tls12_ecdhe_ecdsa_aes128gcmsha256: ptls_cipher_suite_t = unsafe {
    st_ptls_cipher_suite_t {
        id: PTLS_CIPHER_SUITE_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256 as uint16_t,
        aead: &raw const ptls_openssl_aes128gcm,
        hash: &raw const ptls_openssl_sha256,
        name: PTLS_CIPHER_SUITE_NAME_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256.as_ptr(),
    }
};
#[no_mangle]
pub static mut ptls_openssl_aes256gcmsha384: ptls_cipher_suite_t = unsafe {
    st_ptls_cipher_suite_t {
        id: PTLS_CIPHER_SUITE_AES_256_GCM_SHA384 as uint16_t,
        aead: &raw const ptls_openssl_aes256gcm,
        hash: &raw const ptls_openssl_sha384,
        name: PTLS_CIPHER_SUITE_NAME_AES_256_GCM_SHA384.as_ptr(),
    }
};
#[no_mangle]
pub static mut ptls_openssl_tls12_ecdhe_rsa_aes256gcmsha384: ptls_cipher_suite_t = unsafe {
    st_ptls_cipher_suite_t {
        id: PTLS_CIPHER_SUITE_ECDHE_RSA_WITH_AES_256_GCM_SHA384 as uint16_t,
        aead: &raw const ptls_openssl_aes256gcm,
        hash: &raw const ptls_openssl_sha384,
        name: PTLS_CIPHER_SUITE_NAME_ECDHE_RSA_WITH_AES_256_GCM_SHA384.as_ptr(),
    }
};
#[no_mangle]
pub static mut ptls_openssl_tls12_ecdhe_ecdsa_aes256gcmsha384: ptls_cipher_suite_t = unsafe {
    st_ptls_cipher_suite_t {
        id: PTLS_CIPHER_SUITE_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384 as uint16_t,
        aead: &raw const ptls_openssl_aes256gcm,
        hash: &raw const ptls_openssl_sha384,
        name: PTLS_CIPHER_SUITE_NAME_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384.as_ptr(),
    }
};
#[no_mangle]
pub static mut ptls_openssl_chacha20: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"CHACHA20\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: PTLS_CHACHA20_KEY_SIZE as size_t,
        block_size: 1 as size_t,
        iv_size: PTLS_CHACHA20_IV_SIZE as size_t,
        context_size: ::core::mem::size_of::<cipher_context_t>() as size_t,
        setup_crypto: Some(
            chacha20_setup_crypto
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
#[no_mangle]
pub static mut ptls_openssl_chacha20poly1305: ptls_aead_algorithm_t = st_ptls_aead_algorithm_t {
    name: ::core::ptr::null::<::core::ffi::c_char>(),
    confidentiality_limit: 0,
    integrity_limit: 0,
    ctr_cipher: ::core::ptr::null::<ptls_cipher_algorithm_t>(),
    ecb_cipher: ::core::ptr::null::<ptls_cipher_algorithm_t>(),
    key_size: 0,
    iv_size: 0,
    tag_size: 0,
    tls12: C2Rust_Unnamed_2 {
        fixed_iv_size: 0,
        record_iv_size: 0,
    },
    non_temporal: [0; 1],
    align_bits: 0,
    context_size: 0,
    setup_crypto: None,
};
#[no_mangle]
pub static mut ptls_openssl_chacha20poly1305sha256: ptls_cipher_suite_t = unsafe {
    st_ptls_cipher_suite_t {
        id: PTLS_CIPHER_SUITE_CHACHA20_POLY1305_SHA256 as uint16_t,
        aead: &raw const ptls_openssl_chacha20poly1305,
        hash: &raw const ptls_openssl_sha256,
        name: PTLS_CIPHER_SUITE_NAME_CHACHA20_POLY1305_SHA256.as_ptr(),
    }
};
#[no_mangle]
pub static mut ptls_openssl_tls12_ecdhe_rsa_chacha20poly1305sha256: ptls_cipher_suite_t = unsafe {
    st_ptls_cipher_suite_t {
        id: PTLS_CIPHER_SUITE_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256 as uint16_t,
        aead: &raw const ptls_openssl_chacha20poly1305,
        hash: &raw const ptls_openssl_sha256,
        name: PTLS_CIPHER_SUITE_NAME_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256.as_ptr(),
    }
};
#[no_mangle]
pub static mut ptls_openssl_tls12_ecdhe_ecdsa_chacha20poly1305sha256: ptls_cipher_suite_t = unsafe {
    st_ptls_cipher_suite_t {
        id: PTLS_CIPHER_SUITE_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256 as uint16_t,
        aead: &raw const ptls_openssl_chacha20poly1305,
        hash: &raw const ptls_openssl_sha256,
        name: PTLS_CIPHER_SUITE_NAME_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256.as_ptr(),
    }
};
#[no_mangle]
pub static mut ptls_openssl_cipher_suites: [*const ptls_cipher_suite_t; 4] = unsafe {
    [
        &raw const ptls_openssl_aes256gcmsha384,
        &raw const ptls_openssl_aes128gcmsha256,
        &raw const ptls_openssl_chacha20poly1305sha256,
        ::core::ptr::null::<ptls_cipher_suite_t>(),
    ]
};
#[no_mangle]
pub static mut ptls_openssl_cipher_suites_all: [*const ptls_cipher_suite_t; 4] = unsafe {
    [
        &raw const ptls_openssl_aes256gcmsha384,
        &raw const ptls_openssl_aes128gcmsha256,
        &raw const ptls_openssl_chacha20poly1305sha256,
        ::core::ptr::null::<ptls_cipher_suite_t>(),
    ]
};
#[no_mangle]
pub static mut ptls_openssl_tls12_cipher_suites: [*const ptls_cipher_suite_t; 7] = unsafe {
    [
        &raw const ptls_openssl_tls12_ecdhe_rsa_aes128gcmsha256,
        &raw const ptls_openssl_tls12_ecdhe_ecdsa_aes128gcmsha256,
        &raw const ptls_openssl_tls12_ecdhe_rsa_aes256gcmsha384,
        &raw const ptls_openssl_tls12_ecdhe_ecdsa_aes256gcmsha384,
        &raw const ptls_openssl_tls12_ecdhe_rsa_chacha20poly1305sha256,
        &raw const ptls_openssl_tls12_ecdhe_ecdsa_chacha20poly1305sha256,
        ::core::ptr::null::<ptls_cipher_suite_t>(),
    ]
};
#[no_mangle]
pub static mut ptls_openssl_bfecb: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"BF-ECB\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: PTLS_BLOWFISH_KEY_SIZE as size_t,
        block_size: PTLS_BLOWFISH_BLOCK_SIZE as size_t,
        iv_size: 0 as size_t,
        context_size: ::core::mem::size_of::<cipher_context_t>() as size_t,
        setup_crypto: Some(
            bfecb_setup_crypto
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
#[no_mangle]
pub static mut ptls_openssl_hpke_kem_p256sha256: ptls_hpke_kem_t = unsafe {
    st_ptls_hpke_kem_t {
        id: PTLS_HPKE_KEM_P256_SHA256 as uint16_t,
        keyex: &raw const ptls_openssl_secp256r1,
        hash: &raw const ptls_openssl_sha256,
    }
};
#[no_mangle]
pub static mut ptls_openssl_hpke_kem_p384sha384: ptls_hpke_kem_t = unsafe {
    st_ptls_hpke_kem_t {
        id: PTLS_HPKE_KEM_P384_SHA384 as uint16_t,
        keyex: &raw const ptls_openssl_secp384r1,
        hash: &raw const ptls_openssl_sha384,
    }
};
#[no_mangle]
pub static mut ptls_openssl_hpke_kem_x25519sha256: ptls_hpke_kem_t = unsafe {
    st_ptls_hpke_kem_t {
        id: PTLS_HPKE_KEM_X25519_SHA256 as uint16_t,
        keyex: &raw const ptls_openssl_x25519,
        hash: &raw const ptls_openssl_sha256,
    }
};
#[no_mangle]
pub static mut ptls_openssl_hpke_kems: [*const ptls_hpke_kem_t; 4] = unsafe {
    [
        &raw const ptls_openssl_hpke_kem_p384sha384,
        &raw const ptls_openssl_hpke_kem_x25519sha256,
        &raw const ptls_openssl_hpke_kem_p256sha256,
        ::core::ptr::null::<ptls_hpke_kem_t>(),
    ]
};
#[no_mangle]
pub static mut ptls_openssl_hpke_aes128gcmsha256: ptls_hpke_cipher_suite_t = unsafe {
    st_ptls_hpke_cipher_suite_t {
        id: st_ptls_hpke_cipher_suite_id_t {
            kdf: PTLS_HPKE_HKDF_SHA256 as uint16_t,
            aead: PTLS_HPKE_AEAD_AES_128_GCM as uint16_t,
        },
        name: b"HKDF-SHA256/AES-128-GCM\0".as_ptr() as *const ::core::ffi::c_char,
        hash: &raw const ptls_openssl_sha256,
        aead: &raw const ptls_openssl_aes128gcm,
    }
};
#[no_mangle]
pub static mut ptls_openssl_hpke_aes128gcmsha512: ptls_hpke_cipher_suite_t = unsafe {
    st_ptls_hpke_cipher_suite_t {
        id: st_ptls_hpke_cipher_suite_id_t {
            kdf: PTLS_HPKE_HKDF_SHA512 as uint16_t,
            aead: PTLS_HPKE_AEAD_AES_128_GCM as uint16_t,
        },
        name: b"HKDF-SHA512/AES-128-GCM\0".as_ptr() as *const ::core::ffi::c_char,
        hash: &raw const ptls_openssl_sha512,
        aead: &raw const ptls_openssl_aes128gcm,
    }
};
#[no_mangle]
pub static mut ptls_openssl_hpke_aes256gcmsha384: ptls_hpke_cipher_suite_t = unsafe {
    st_ptls_hpke_cipher_suite_t {
        id: st_ptls_hpke_cipher_suite_id_t {
            kdf: PTLS_HPKE_HKDF_SHA384 as uint16_t,
            aead: PTLS_HPKE_AEAD_AES_256_GCM as uint16_t,
        },
        name: b"HKDF-SHA384/AES-256-GCM\0".as_ptr() as *const ::core::ffi::c_char,
        hash: &raw const ptls_openssl_sha384,
        aead: &raw const ptls_openssl_aes256gcm,
    }
};
#[no_mangle]
pub static mut ptls_openssl_hpke_chacha20poly1305sha256: ptls_hpke_cipher_suite_t = unsafe {
    st_ptls_hpke_cipher_suite_t {
        id: st_ptls_hpke_cipher_suite_id_t {
            kdf: PTLS_HPKE_HKDF_SHA256 as uint16_t,
            aead: PTLS_HPKE_AEAD_CHACHA20POLY1305 as uint16_t,
        },
        name: b"HKDF-SHA256/ChaCha20Poly1305\0".as_ptr() as *const ::core::ffi::c_char,
        hash: &raw const ptls_openssl_sha256,
        aead: &raw const ptls_openssl_chacha20poly1305,
    }
};
#[no_mangle]
pub static mut ptls_openssl_hpke_cipher_suites: [*const ptls_hpke_cipher_suite_t; 5] = unsafe {
    [
        &raw const ptls_openssl_hpke_aes128gcmsha256,
        &raw const ptls_openssl_hpke_aes256gcmsha384,
        &raw const ptls_openssl_hpke_chacha20poly1305sha256,
        &raw const ptls_openssl_hpke_aes128gcmsha512,
        ::core::ptr::null::<ptls_hpke_cipher_suite_t>(),
    ]
};
unsafe extern "C" fn c2rust_run_static_initializers() {
    ptls_openssl_aes128gcm = {
        let mut init = st_ptls_aead_algorithm_t {
            non_temporal: [0; 1],
            name: b"AES128-GCM\0".as_ptr() as *const ::core::ffi::c_char,
            confidentiality_limit: PTLS_AESGCM_CONFIDENTIALITY_LIMIT as uint64_t,
            integrity_limit: 0x40000000000000 as uint64_t,
            ctr_cipher: &raw const ptls_openssl_aes128ctr,
            ecb_cipher: &raw const ptls_openssl_aes128ecb,
            key_size: PTLS_AES128_KEY_SIZE as size_t,
            iv_size: PTLS_AESGCM_IV_SIZE as size_t,
            tag_size: PTLS_AESGCM_TAG_SIZE as size_t,
            tls12: C2Rust_Unnamed_2 {
                fixed_iv_size: PTLS_TLS12_AESGCM_FIXED_IV_SIZE as size_t,
                record_iv_size: PTLS_TLS12_AESGCM_RECORD_IV_SIZE as size_t,
            },
            align_bits: 0 as uint8_t,
            context_size: ::core::mem::size_of::<aead_crypto_context_t>() as size_t,
            setup_crypto: Some(
                aead_aes128gcm_setup_crypto
                    as unsafe extern "C" fn(
                        *mut ptls_aead_context_t,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        };
        init.set_non_temporal(0 as ::core::ffi::c_uint);
        init
    };
    ptls_openssl_aes256gcm = {
        let mut init = st_ptls_aead_algorithm_t {
            non_temporal: [0; 1],
            name: b"AES256-GCM\0".as_ptr() as *const ::core::ffi::c_char,
            confidentiality_limit: PTLS_AESGCM_CONFIDENTIALITY_LIMIT as uint64_t,
            integrity_limit: 0x40000000000000 as uint64_t,
            ctr_cipher: &raw const ptls_openssl_aes256ctr,
            ecb_cipher: &raw const ptls_openssl_aes256ecb,
            key_size: PTLS_AES256_KEY_SIZE as size_t,
            iv_size: PTLS_AESGCM_IV_SIZE as size_t,
            tag_size: PTLS_AESGCM_TAG_SIZE as size_t,
            tls12: C2Rust_Unnamed_2 {
                fixed_iv_size: PTLS_TLS12_AESGCM_FIXED_IV_SIZE as size_t,
                record_iv_size: PTLS_TLS12_AESGCM_RECORD_IV_SIZE as size_t,
            },
            align_bits: 0 as uint8_t,
            context_size: ::core::mem::size_of::<aead_crypto_context_t>() as size_t,
            setup_crypto: Some(
                aead_aes256gcm_setup_crypto
                    as unsafe extern "C" fn(
                        *mut ptls_aead_context_t,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        };
        init.set_non_temporal(0 as ::core::ffi::c_uint);
        init
    };
    ptls_openssl_chacha20poly1305 = {
        let mut init = st_ptls_aead_algorithm_t {
            non_temporal: [0; 1],
            name: b"CHACHA20-POLY1305\0".as_ptr() as *const ::core::ffi::c_char,
            confidentiality_limit: PTLS_CHACHA20POLY1305_CONFIDENTIALITY_LIMIT as uint64_t,
            integrity_limit: 0x1000000000 as uint64_t,
            ctr_cipher: &raw const ptls_openssl_chacha20,
            ecb_cipher: ::core::ptr::null::<ptls_cipher_algorithm_t>(),
            key_size: PTLS_CHACHA20_KEY_SIZE as size_t,
            iv_size: PTLS_CHACHA20POLY1305_IV_SIZE as size_t,
            tag_size: PTLS_CHACHA20POLY1305_TAG_SIZE as size_t,
            tls12: C2Rust_Unnamed_2 {
                fixed_iv_size: PTLS_TLS12_CHACHAPOLY_FIXED_IV_SIZE as size_t,
                record_iv_size: PTLS_TLS12_CHACHAPOLY_RECORD_IV_SIZE as size_t,
            },
            align_bits: 0 as uint8_t,
            context_size: ::core::mem::size_of::<aead_crypto_context_t>() as size_t,
            setup_crypto: Some(
                aead_chacha20poly1305_setup_crypto
                    as unsafe extern "C" fn(
                        *mut ptls_aead_context_t,
                        ::core::ffi::c_int,
                        *const ::core::ffi::c_void,
                        *const ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
        };
        init.set_non_temporal(0 as ::core::ffi::c_uint);
        init
    };
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];

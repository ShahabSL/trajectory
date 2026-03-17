use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type stack_st;
    pub type ossl_provider_st;
    pub type bio_st;
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
    pub type ossl_init_settings_st;
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
    fn select(
        __nfds: ::core::ffi::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> ::core::ffi::c_int;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn abort() -> !;
    fn OPENSSL_sk_num(_: *const OPENSSL_STACK) -> ::core::ffi::c_int;
    fn OPENSSL_sk_value(_: *const OPENSSL_STACK, _: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_void;
    fn OPENSSL_sk_new_null() -> *mut OPENSSL_STACK;
    fn OPENSSL_sk_free(_: *mut OPENSSL_STACK);
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
    fn OPENSSL_init_crypto(
        opts: uint64_t,
        settings: *const OPENSSL_INIT_SETTINGS,
    ) -> ::core::ffi::c_int;
    fn BIO_free(a: *mut BIO) -> ::core::ffi::c_int;
    fn BIO_new_mem_buf(buf: *const ::core::ffi::c_void, len: ::core::ffi::c_int) -> *mut BIO;
    fn BN_CTX_new() -> *mut BN_CTX;
    fn BN_CTX_free(c: *mut BN_CTX);
    fn EVP_MD_CTX_new() -> *mut EVP_MD_CTX;
    fn EVP_MD_CTX_free(ctx_0: *mut EVP_MD_CTX);
    fn EVP_EncryptInit_ex(
        ctx_0: *mut EVP_CIPHER_CTX,
        cipher: *const EVP_CIPHER,
        impl_0: *mut ENGINE,
        key: *const ::core::ffi::c_uchar,
        iv: *const ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    fn EVP_EncryptUpdate(
        ctx_0: *mut EVP_CIPHER_CTX,
        out: *mut ::core::ffi::c_uchar,
        outl: *mut ::core::ffi::c_int,
        in_0: *const ::core::ffi::c_uchar,
        inl: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn EVP_EncryptFinal_ex(
        ctx_0: *mut EVP_CIPHER_CTX,
        out: *mut ::core::ffi::c_uchar,
        outl: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn EVP_DecryptInit_ex(
        ctx_0: *mut EVP_CIPHER_CTX,
        cipher: *const EVP_CIPHER,
        impl_0: *mut ENGINE,
        key: *const ::core::ffi::c_uchar,
        iv: *const ::core::ffi::c_uchar,
    ) -> ::core::ffi::c_int;
    fn EVP_DecryptUpdate(
        ctx_0: *mut EVP_CIPHER_CTX,
        out: *mut ::core::ffi::c_uchar,
        outl: *mut ::core::ffi::c_int,
        in_0: *const ::core::ffi::c_uchar,
        inl: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn EVP_DecryptFinal_ex(
        ctx_0: *mut EVP_CIPHER_CTX,
        outm: *mut ::core::ffi::c_uchar,
        outl: *mut ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn EVP_DigestSign(
        ctx_0: *mut EVP_MD_CTX,
        sigret: *mut ::core::ffi::c_uchar,
        siglen: *mut size_t,
        tbs: *const ::core::ffi::c_uchar,
        tbslen: size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_DigestVerify(
        ctx_0: *mut EVP_MD_CTX,
        sigret: *const ::core::ffi::c_uchar,
        siglen: size_t,
        tbs: *const ::core::ffi::c_uchar,
        tbslen: size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_DigestSignInit(
        ctx_0: *mut EVP_MD_CTX,
        pctx: *mut *mut EVP_PKEY_CTX,
        type_0: *const EVP_MD,
        e: *mut ENGINE,
        pkey: *mut EVP_PKEY,
    ) -> ::core::ffi::c_int;
    fn EVP_DigestSignUpdate(
        ctx_0: *mut EVP_MD_CTX,
        data: *const ::core::ffi::c_void,
        dsize: size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_DigestSignFinal(
        ctx_0: *mut EVP_MD_CTX,
        sigret: *mut ::core::ffi::c_uchar,
        siglen: *mut size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_DigestVerifyInit(
        ctx_0: *mut EVP_MD_CTX,
        pctx: *mut *mut EVP_PKEY_CTX,
        type_0: *const EVP_MD,
        e: *mut ENGINE,
        pkey: *mut EVP_PKEY,
    ) -> ::core::ffi::c_int;
    fn EVP_DigestVerifyUpdate(
        ctx_0: *mut EVP_MD_CTX,
        data: *const ::core::ffi::c_void,
        dsize: size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_DigestVerifyFinal(
        ctx_0: *mut EVP_MD_CTX,
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
        ctx_0: *mut EVP_CIPHER_CTX,
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
    fn EVP_MAC_CTX_free(ctx_0: *mut EVP_MAC_CTX);
    fn EVP_MAC_CTX_get_mac_size(ctx_0: *mut EVP_MAC_CTX) -> size_t;
    fn EVP_MAC_update(
        ctx_0: *mut EVP_MAC_CTX,
        data: *const ::core::ffi::c_uchar,
        datalen: size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_MAC_final(
        ctx_0: *mut EVP_MAC_CTX,
        out: *mut ::core::ffi::c_uchar,
        outl: *mut size_t,
        outsize: size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_PKEY_get_id(pkey: *const EVP_PKEY) -> ::core::ffi::c_int;
    fn EVP_PKEY_set1_EC_KEY(pkey: *mut EVP_PKEY, key: *mut ec_key_st) -> ::core::ffi::c_int;
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
    fn EVP_PKEY_CTX_free(ctx_0: *mut EVP_PKEY_CTX);
    fn EVP_PKEY_derive_init(ctx_0: *mut EVP_PKEY_CTX) -> ::core::ffi::c_int;
    fn EVP_PKEY_derive_set_peer(
        ctx_0: *mut EVP_PKEY_CTX,
        peer: *mut EVP_PKEY,
    ) -> ::core::ffi::c_int;
    fn EVP_PKEY_derive(
        ctx_0: *mut EVP_PKEY_CTX,
        key: *mut ::core::ffi::c_uchar,
        keylen: *mut size_t,
    ) -> ::core::ffi::c_int;
    fn EVP_PKEY_keygen_init(ctx_0: *mut EVP_PKEY_CTX) -> ::core::ffi::c_int;
    fn EVP_PKEY_keygen(ctx_0: *mut EVP_PKEY_CTX, ppkey: *mut *mut EVP_PKEY) -> ::core::ffi::c_int;
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
        ctx_0: *mut BN_CTX,
    ) -> size_t;
    fn EC_POINT_oct2point(
        group: *const EC_GROUP,
        p: *mut EC_POINT,
        buf: *const ::core::ffi::c_uchar,
        len: size_t,
        ctx_0: *mut BN_CTX,
    ) -> ::core::ffi::c_int;
    fn EC_KEY_new() -> *mut EC_KEY;
    fn EC_KEY_new_by_curve_name(nid: ::core::ffi::c_int) -> *mut EC_KEY;
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
    fn EVP_PKEY_CTX_set_rsa_padding(
        ctx_0: *mut EVP_PKEY_CTX,
        pad_mode: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn EVP_PKEY_CTX_set_rsa_pss_saltlen(
        ctx_0: *mut EVP_PKEY_CTX,
        saltlen: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn EVP_PKEY_CTX_set_rsa_mgf1_md(
        ctx_0: *mut EVP_PKEY_CTX,
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
    fn X509_verify_cert(ctx_0: *mut X509_STORE_CTX) -> ::core::ffi::c_int;
    fn X509_STORE_new() -> *mut X509_STORE;
    fn X509_STORE_free(v: *mut X509_STORE);
    fn X509_STORE_up_ref(v: *mut X509_STORE) -> ::core::ffi::c_int;
    fn X509_STORE_set_verify_cb(ctx_0: *mut X509_STORE, verify_cb: X509_STORE_CTX_verify_cb);
    fn X509_STORE_CTX_new() -> *mut X509_STORE_CTX;
    fn X509_STORE_CTX_free(ctx_0: *mut X509_STORE_CTX);
    fn X509_STORE_CTX_init(
        ctx_0: *mut X509_STORE_CTX,
        trust_store: *mut X509_STORE,
        target: *mut X509,
        untrusted: *mut stack_st_X509,
    ) -> ::core::ffi::c_int;
    fn X509_STORE_add_lookup(v: *mut X509_STORE, m: *mut X509_LOOKUP_METHOD) -> *mut X509_LOOKUP;
    fn X509_LOOKUP_hash_dir() -> *mut X509_LOOKUP_METHOD;
    fn X509_LOOKUP_file() -> *mut X509_LOOKUP_METHOD;
    fn X509_LOOKUP_ctrl(
        ctx_0: *mut X509_LOOKUP,
        cmd: ::core::ffi::c_int,
        argc: *const ::core::ffi::c_char,
        argl: ::core::ffi::c_long,
        ret: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn X509_STORE_CTX_get_error(ctx_0: *const X509_STORE_CTX) -> ::core::ffi::c_int;
    fn X509_STORE_CTX_get0_param(ctx_0: *const X509_STORE_CTX) -> *mut X509_VERIFY_PARAM;
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
    fn X509_free(a: *mut X509);
    fn d2i_X509(
        a: *mut *mut X509,
        in_0: *mut *const ::core::ffi::c_uchar,
        len: ::core::ffi::c_long,
    ) -> *mut X509;
    fn i2d_X509(a: *const X509, out: *mut *mut ::core::ffi::c_uchar) -> ::core::ffi::c_int;
    fn X509_get_pubkey(x: *mut X509) -> *mut EVP_PKEY;
    fn PEM_read_bio_X509(
        out: *mut BIO,
        x: *mut *mut X509,
        cb: Option<pem_password_cb>,
        u: *mut ::core::ffi::c_void,
    ) -> *mut X509;
    fn PEM_read_bio_PrivateKey(
        out: *mut BIO,
        x: *mut *mut EVP_PKEY,
        cb: Option<pem_password_cb>,
        u: *mut ::core::ffi::c_void,
    ) -> *mut EVP_PKEY;
    fn RAND_bytes(buf: *mut ::core::ffi::c_uchar, num: ::core::ffi::c_int) -> ::core::ffi::c_int;
    fn ENGINE_by_id(id: *const ::core::ffi::c_char) -> *mut ENGINE;
    fn ENGINE_ctrl_cmd_string(
        e: *mut ENGINE,
        cmd_name: *const ::core::ffi::c_char,
        arg: *const ::core::ffi::c_char,
        cmd_optional: ::core::ffi::c_int,
    ) -> ::core::ffi::c_int;
    fn ENGINE_free(e: *mut ENGINE) -> ::core::ffi::c_int;
    fn ENGINE_set_default_RSA(e: *mut ENGINE) -> ::core::ffi::c_int;
    fn OSSL_PROVIDER_load(
        _: *mut OSSL_LIB_CTX,
        name: *const ::core::ffi::c_char,
    ) -> *mut OSSL_PROVIDER;
    fn OSSL_PROVIDER_unload(prov: *mut OSSL_PROVIDER) -> ::core::ffi::c_int;
    fn ptls_buffer__release_memory(buf: *mut ptls_buffer_t);
    fn ptls_buffer_reserve(buf: *mut ptls_buffer_t, delta: size_t) -> ::core::ffi::c_int;
    fn ptls_buffer__do_pushv(
        buf: *mut ptls_buffer_t,
        src: *const ::core::ffi::c_void,
        len: size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_client_new(ctx_0: *mut ptls_context_t) -> *mut ptls_t;
    fn ptls_server_new(ctx_0: *mut ptls_context_t) -> *mut ptls_t;
    fn ptls_free(tls: *mut ptls_t);
    fn ptls_get_async_job(tls: *mut ptls_t) -> *mut ptls_async_job_t;
    fn ptls_get_cipher(tls: *mut ptls_t) -> *const ptls_cipher_suite_t;
    fn ptls_handshake(
        tls: *mut ptls_t,
        sendbuf: *mut ptls_buffer_t,
        input: *const ::core::ffi::c_void,
        inlen: *mut size_t,
        args: *mut ptls_handshake_properties_t,
    ) -> ::core::ffi::c_int;
    fn ptls_is_server(tls: *mut ptls_t) -> ::core::ffi::c_int;
    fn ptls_calc_hash(
        algo: *const ptls_hash_algorithm_t,
        output: *mut ::core::ffi::c_void,
        src: *const ::core::ffi::c_void,
        len: size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_cipher_new(
        algo: *const ptls_cipher_algorithm_t,
        is_enc: ::core::ffi::c_int,
        key: *const ::core::ffi::c_void,
    ) -> *mut ptls_cipher_context_t;
    fn ptls_cipher_free(ctx_0: *mut ptls_cipher_context_t);
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
    fn ptls_hpke_setup_base_r(
        kem: *const ptls_hpke_kem_t,
        cipher: *const ptls_hpke_cipher_suite_t,
        keyex: *mut ptls_key_exchange_context_t,
        ctx_0: *mut *mut ptls_aead_context_t,
        pk_s: ptls_iovec_t,
        info: ptls_iovec_t,
    ) -> ::core::ffi::c_int;
    static mut ptls_get_time: ptls_get_time_t;
    fn ptls_minicrypto_random_bytes(buf: *mut ::core::ffi::c_void, len: size_t);
    fn ptls_minicrypto_init_secp256r1sha256_sign_certificate(
        self_0: *mut ptls_minicrypto_secp256r1sha256_sign_certificate_t,
        key: ptls_iovec_t,
    ) -> ::core::ffi::c_int;
    static ptls_minicrypto_x25519: ptls_key_exchange_algorithm_t;
    static ptls_minicrypto_secp256r1: ptls_key_exchange_algorithm_t;
    static mut ptls_minicrypto_key_exchanges: [*const ptls_key_exchange_algorithm_t; 0];
    static mut ptls_minicrypto_cipher_suites: [*const ptls_cipher_suite_t; 0];
    fn HMAC_size(e: *const HMAC_CTX) -> size_t;
    fn HMAC_CTX_new() -> *mut HMAC_CTX;
    fn HMAC_CTX_free(ctx_0: *mut HMAC_CTX);
    fn HMAC_Update(
        ctx_0: *mut HMAC_CTX,
        data: *const ::core::ffi::c_uchar,
        len: size_t,
    ) -> ::core::ffi::c_int;
    fn HMAC_Final(
        ctx_0: *mut HMAC_CTX,
        md: *mut ::core::ffi::c_uchar,
        len: *mut ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    fn ASYNC_WAIT_CTX_new() -> *mut ASYNC_WAIT_CTX;
    fn ASYNC_WAIT_CTX_free(ctx_0: *mut ASYNC_WAIT_CTX);
    fn ASYNC_WAIT_CTX_get_all_fds(
        ctx_0: *mut ASYNC_WAIT_CTX,
        fd: *mut ::core::ffi::c_int,
        numfds: *mut size_t,
    ) -> ::core::ffi::c_int;
    fn ASYNC_start_job(
        job: *mut *mut ASYNC_JOB,
        ctx_0: *mut ASYNC_WAIT_CTX,
        ret: *mut ::core::ffi::c_int,
        func: Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ::core::ffi::c_int>,
        args: *mut ::core::ffi::c_void,
        size: size_t,
    ) -> ::core::ffi::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut ::core::ffi::c_void) -> ::core::ffi::c_int;
    fn note(fmt: *const ::core::ffi::c_char, ...);
    fn _ok(cond: ::core::ffi::c_int, fmt: *const ::core::ffi::c_char, ...);
    fn done_testing() -> ::core::ffi::c_int;
    fn enter_subtest(name: *const ::core::ffi::c_char);
    fn exit_subtest(name: *const ::core::ffi::c_char);
    fn ptls_ffx_setup_crypto(
        _ctx: *mut ptls_cipher_context_t,
        algo: *const ptls_cipher_algorithm_t,
        is_enc: ::core::ffi::c_int,
        nb_rounds: ::core::ffi::c_int,
        bit_length: size_t,
        key: *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    static mut ctx: *mut ptls_context_t;
    static mut ctx_peer: *mut ptls_context_t;
    static mut verify_certificate: *mut ptls_verify_certificate_t;
    static mut ffx_variants: [st_ptls_ffx_test_variants_t; 7];
    fn test_key_exchange(
        client: *const ptls_key_exchange_algorithm_t,
        server: *const ptls_key_exchange_algorithm_t,
    );
    fn test_picotls();
    fn test_hpke(
        all_kems: *mut *const ptls_hpke_kem_t,
        all_ciphers: *mut *const ptls_hpke_cipher_suite_t,
    );
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __time_t = ::core::ffi::c_long;
pub type __suseconds_t = ::core::ffi::c_long;
pub type __ssize_t = ::core::ffi::c_long;
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
pub type ssize_t = __ssize_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type intptr_t = isize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = ::core::ffi::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
pub type OPENSSL_STACK = stack_st;
pub type OPENSSL_sk_freefunc = Option<unsafe extern "C" fn(*mut ::core::ffi::c_void) -> ()>;
pub type OSSL_PROVIDER = ossl_provider_st;
pub type BIO = bio_st;
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
pub type OPENSSL_INIT_SETTINGS = ossl_init_settings_st;
pub type ENGINE = engine_st;
pub type pem_password_cb = unsafe extern "C" fn(
    *mut ::core::ffi::c_char,
    ::core::ffi::c_int,
    ::core::ffi::c_int,
    *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int;
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
pub type X509_STORE_CTX_verify_cb =
    Option<unsafe extern "C" fn(::core::ffi::c_int, *mut X509_STORE_CTX) -> ::core::ffi::c_int>;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_raw_extension_t {
    pub type_0: uint16_t,
    pub data: ptls_iovec_t,
}
pub type ptls_raw_extension_t = st_ptls_raw_extension_t;
pub type en_ptls_early_data_acceptance_t = ::core::ffi::c_uint;
pub const PTLS_EARLY_DATA_ACCEPTED: en_ptls_early_data_acceptance_t = 2;
pub const PTLS_EARLY_DATA_REJECTED: en_ptls_early_data_acceptance_t = 1;
pub const PTLS_EARLY_DATA_ACCEPTANCE_UNKNOWN: en_ptls_early_data_acceptance_t = 0;
pub type ptls_early_data_acceptance_t = en_ptls_early_data_acceptance_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_handshake_properties_t {
    pub c2rust_unnamed: C2Rust_Unnamed_13,
    pub additional_extensions: *mut ptls_raw_extension_t,
    pub collect_extension: Option<
        unsafe extern "C" fn(
            *mut ptls_t,
            *mut st_ptls_handshake_properties_t,
            uint16_t,
        ) -> ::core::ffi::c_int,
    >,
    pub collected_extensions: Option<
        unsafe extern "C" fn(
            *mut ptls_t,
            *mut st_ptls_handshake_properties_t,
            *mut ptls_raw_extension_t,
        ) -> ::core::ffi::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_13 {
    pub client: C2Rust_Unnamed_17,
    pub server: C2Rust_Unnamed_14,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_14 {
    pub selected_psk_binder: C2Rust_Unnamed_16,
    pub cookie: C2Rust_Unnamed_15,
    #[bitfield(name = "enforce_retry", ty = "::core::ffi::c_uint", bits = "0..=0")]
    #[bitfield(name = "retry_uses_cookie", ty = "::core::ffi::c_uint", bits = "1..=1")]
    pub enforce_retry_retry_uses_cookie: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_15 {
    pub key: *const ::core::ffi::c_void,
    pub additional_data: ptls_iovec_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_16 {
    pub base: [uint8_t; 64],
    pub len: size_t,
}
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2Rust_Unnamed_17 {
    pub negotiated_protocols: C2Rust_Unnamed_19,
    pub session_ticket: ptls_iovec_t,
    pub max_early_data_size: *mut size_t,
    pub early_data_acceptance: ptls_early_data_acceptance_t,
    #[bitfield(
        name = "negotiate_before_key_exchange",
        ty = "::core::ffi::c_uint",
        bits = "0..=0"
    )]
    pub negotiate_before_key_exchange: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 3],
    pub ech: C2Rust_Unnamed_18,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_18 {
    pub configs: ptls_iovec_t,
    pub retry_configs: *mut ptls_iovec_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_19 {
    pub list: *const ptls_iovec_t,
    pub count: size_t,
}
pub type ptls_handshake_properties_t = st_ptls_handshake_properties_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_minicrypto_secp256r1sha256_sign_certificate_t {
    pub super_0: ptls_sign_certificate_t,
    pub key: [uint8_t; 32],
}
pub type ptls_minicrypto_secp256r1sha256_sign_certificate_t =
    st_ptls_minicrypto_secp256r1sha256_sign_certificate_t;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_ffx_context_t {
    pub super_0: ptls_cipher_context_t,
    pub enc_ctx: *mut ptls_cipher_context_t,
    pub nb_rounds: ::core::ffi::c_int,
    pub is_enc: ::core::ffi::c_int,
    pub byte_length: size_t,
    pub nb_left: size_t,
    pub nb_right: size_t,
    pub mask_last_byte: uint8_t,
    pub tweaks: [uint8_t; 16],
}
pub type ptls_ffx_context_t = st_ptls_ffx_context_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_ffx_test_variants_t {
    pub algo: *const ptls_cipher_algorithm_t,
    pub bit_length: ::core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_20 {
    pub algo: *const ptls_hash_algorithm_t,
    pub expected: [uint8_t; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_21 {
    pub conns: [C2Rust_Unnamed_22; 10],
    pub first_pending: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_22 {
    pub next_pending: size_t,
    pub tls: *mut ptls_t,
    pub wait_fd: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const NULL_0: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const SIZE_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const __NFDBITS: ::core::ffi::c_int =
    8 as ::core::ffi::c_int * ::core::mem::size_of::<__fd_mask>() as ::core::ffi::c_int;
pub const NID_X9_62_id_ecPublicKey: ::core::ffi::c_int = 408;
pub const NID_X9_62_prime256v1: ::core::ffi::c_int = 415 as ::core::ffi::c_int;
pub const NID_secp384r1: ::core::ffi::c_int = 715 as ::core::ffi::c_int;
pub const NID_secp521r1: ::core::ffi::c_int = 716 as ::core::ffi::c_int;
pub const NID_rsaEncryption: ::core::ffi::c_int = 6;
pub const NID_X25519: ::core::ffi::c_int = 1034 as ::core::ffi::c_int;
pub const NID_ED25519: ::core::ffi::c_int = 1087;
pub const OPENSSL_INIT_LOAD_CRYPTO_STRINGS: ::core::ffi::c_long = 0x2 as ::core::ffi::c_long;
pub const OPENSSL_INIT_ADD_ALL_CIPHERS: ::core::ffi::c_long = 0x4 as ::core::ffi::c_long;
pub const OPENSSL_INIT_ADD_ALL_DIGESTS: ::core::ffi::c_long = 0x8 as ::core::ffi::c_long;
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
unsafe extern "C" fn ossl_check_X509_sk_type(mut sk: *mut stack_st_X509) -> *mut OPENSSL_STACK {
    return sk as *mut OPENSSL_STACK;
}
#[inline]
unsafe extern "C" fn ossl_check_X509_freefunc_type(
    mut fr: sk_X509_freefunc,
) -> OPENSSL_sk_freefunc {
    return ::core::mem::transmute::<sk_X509_freefunc, OPENSSL_sk_freefunc>(fr);
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
unsafe extern "C" fn ptls_new(
    mut ctx_0: *mut ptls_context_t,
    mut is_server: ::core::ffi::c_int,
) -> *mut ptls_t {
    return if is_server != 0 {
        ptls_server_new(ctx_0)
    } else {
        ptls_client_new(ctx_0)
    };
}
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
unsafe extern "C" fn ptls_buffer_init(
    mut buf: *mut ptls_buffer_t,
    mut smallbuf: *mut ::core::ffi::c_void,
    mut smallbuf_size: size_t,
) {
    (*buf).base = smallbuf as *mut uint8_t;
    (*buf).off = 0 as size_t;
    (*buf).capacity = smallbuf_size;
    (*buf).is_allocated = 0 as uint8_t;
    (*buf).align_bits = 0 as uint8_t;
}
#[inline]
unsafe extern "C" fn ptls_buffer_dispose(mut buf: *mut ptls_buffer_t) {
    ptls_buffer__release_memory(buf);
    *buf = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0 as size_t,
        off: 0 as size_t,
        is_allocated: 0 as uint8_t,
        align_bits: 0 as uint8_t,
    };
}
#[inline]
unsafe extern "C" fn ptls_cipher_init(
    mut ctx_0: *mut ptls_cipher_context_t,
    mut iv: *const ::core::ffi::c_void,
) {
    (*ctx_0).do_init.expect("non-null function pointer")(
        ctx_0 as *mut st_ptls_cipher_context_t,
        iv,
    );
}
#[inline]
unsafe extern "C" fn ptls_cipher_encrypt(
    mut ctx_0: *mut ptls_cipher_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    (*ctx_0).do_transform.expect("non-null function pointer")(
        ctx_0 as *mut st_ptls_cipher_context_t,
        output,
        input,
        len,
    );
}
#[inline]
unsafe extern "C" fn ptls_aead__do_encrypt(
    mut ctx_0: *mut ptls_aead_context_t,
    mut output: *mut ::core::ffi::c_void,
    mut input: *const ::core::ffi::c_void,
    mut inlen: size_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
    mut supp: *mut ptls_aead_supplementary_encryption_t,
) {
    let mut invec: ptls_iovec_t = ptls_iovec_init(input, inlen);
    (*ctx_0).do_encrypt_v.expect("non-null function pointer")(
        ctx_0 as *mut st_ptls_aead_context_t,
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
    mut ctx_0: *mut ptls_aead_context_t,
    mut _output: *mut ::core::ffi::c_void,
    mut input: *mut ptls_iovec_t,
    mut incnt: size_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) {
    let mut output: *mut uint8_t = _output as *mut uint8_t;
    (*ctx_0).do_encrypt_init.expect("non-null function pointer")(
        ctx_0 as *mut st_ptls_aead_context_t,
        seq,
        aad,
        aadlen,
    );
    let mut i: size_t = 0 as size_t;
    while i < incnt {
        output = output.offset((*ctx_0)
            .do_encrypt_update
            .expect("non-null function pointer")(
            ctx_0 as *mut st_ptls_aead_context_t,
            output as *mut ::core::ffi::c_void,
            (*input.offset(i as isize)).base as *const ::core::ffi::c_void,
            (*input.offset(i as isize)).len,
        ) as isize);
        i = i.wrapping_add(1);
    }
    (*ctx_0)
        .do_encrypt_final
        .expect("non-null function pointer")(
        ctx_0 as *mut st_ptls_aead_context_t,
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
pub const X509_PURPOSE_SSL_CLIENT: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const X509_PURPOSE_SSL_SERVER: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const X509_CHECK_FLAG_NO_PARTIAL_WILDCARDS: ::core::ffi::c_int = 0x4 as ::core::ffi::c_int;
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
unsafe extern "C" fn x9_62_free_context(mut ctx_0: *mut st_x9_62_keyex_context_t) {
    free((*ctx_0).super_0.pubkey.base as *mut ::core::ffi::c_void);
    if !(*ctx_0).privkey.is_null() {
        EC_KEY_free((*ctx_0).privkey);
    }
    if !(*ctx_0).bn_ctx.is_null() {
        BN_CTX_free((*ctx_0).bn_ctx);
    }
    free(ctx_0 as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn x9_62_on_exchange(
    mut _ctx: *mut *mut ptls_key_exchange_context_t,
    mut release: ::core::ffi::c_int,
    mut secret: *mut ptls_iovec_t,
    mut peerkey: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut ctx_0: *mut st_x9_62_keyex_context_t = *_ctx as *mut st_x9_62_keyex_context_t;
    let mut group: *const EC_GROUP = EC_KEY_get0_group((*ctx_0).privkey);
    let mut peer_point: *mut EC_POINT = ::core::ptr::null_mut::<EC_POINT>();
    let mut ret: ::core::ffi::c_int = 0;
    if secret.is_null() {
        ret = 0 as ::core::ffi::c_int;
    } else {
        peer_point = x9_62_decode_point(group, peerkey, (*ctx_0).bn_ctx);
        if peer_point.is_null() {
            ret = PTLS_ALERT_DECODE_ERROR;
        } else {
            ret = ecdh_calc_secret(secret, group, (*ctx_0).privkey, peer_point);
            ret != 0 as ::core::ffi::c_int;
        }
    }
    if !peer_point.is_null() {
        EC_POINT_free(peer_point);
    }
    if release != 0 {
        x9_62_free_context(ctx_0);
        *_ctx = ::core::ptr::null_mut::<ptls_key_exchange_context_t>();
    }
    return ret;
}
unsafe extern "C" fn x9_62_create_context(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut ctx_0: *mut *mut st_x9_62_keyex_context_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0;
    *ctx_0 = malloc(::core::mem::size_of::<st_x9_62_keyex_context_t>() as size_t)
        as *mut st_x9_62_keyex_context_t;
    if (*ctx_0).is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else {
        **ctx_0 = st_x9_62_keyex_context_t {
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
        (**ctx_0).bn_ctx = BN_CTX_new();
        if (**ctx_0).bn_ctx.is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        } else {
            ret = 0 as ::core::ffi::c_int;
        }
    }
    if ret != 0 as ::core::ffi::c_int && !(*ctx_0).is_null() {
        x9_62_free_context(*ctx_0);
        *ctx_0 = ::core::ptr::null_mut::<st_x9_62_keyex_context_t>();
    }
    return ret;
}
unsafe extern "C" fn x9_62_setup_pubkey(
    mut ctx_0: *mut st_x9_62_keyex_context_t,
) -> ::core::ffi::c_int {
    let mut group: *const EC_GROUP = EC_KEY_get0_group((*ctx_0).privkey);
    let mut pubkey: *const EC_POINT = EC_KEY_get0_public_key((*ctx_0).privkey);
    (*ctx_0).super_0.pubkey = x9_62_encode_point(group, pubkey, (*ctx_0).bn_ctx);
    if (*ctx_0).super_0.pubkey.base.is_null() {
        return PTLS_ERROR_NO_MEMORY;
    }
    return 0 as ::core::ffi::c_int;
}
unsafe extern "C" fn x9_62_create_key_exchange(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut _ctx: *mut *mut ptls_key_exchange_context_t,
) -> ::core::ffi::c_int {
    let mut group: *mut EC_GROUP = ::core::ptr::null_mut::<EC_GROUP>();
    let mut ctx_0: *mut st_x9_62_keyex_context_t =
        ::core::ptr::null_mut::<st_x9_62_keyex_context_t>();
    let mut ret: ::core::ffi::c_int = 0;
    group = EC_GROUP_new_by_curve_name((*algo).data as ::core::ffi::c_int);
    if group.is_null() {
        ret = PTLS_ERROR_LIBRARY;
    } else {
        ret = x9_62_create_context(algo, &raw mut ctx_0);
        if !(ret != 0 as ::core::ffi::c_int) {
            (*ctx_0).privkey = ecdh_gerenate_key(group);
            if (*ctx_0).privkey.is_null() {
                ret = PTLS_ERROR_LIBRARY;
            } else {
                ret = x9_62_setup_pubkey(ctx_0);
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
        *_ctx = &raw mut (*ctx_0).super_0;
    } else {
        if !ctx_0.is_null() {
            x9_62_free_context(ctx_0);
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
    let mut ctx_0: *mut st_x9_62_keyex_context_t =
        ::core::ptr::null_mut::<st_x9_62_keyex_context_t>();
    let mut ret: ::core::ffi::c_int = 0;
    ret = x9_62_create_context(algo, &raw mut ctx_0);
    if !(ret != 0 as ::core::ffi::c_int) {
        (*ctx_0).privkey = eckey;
        ret = x9_62_setup_pubkey(ctx_0);
        if !(ret != 0 as ::core::ffi::c_int) {
            ret = 0 as ::core::ffi::c_int;
        }
    }
    if ret == 0 as ::core::ffi::c_int {
        *_ctx = &raw mut (*ctx_0).super_0;
    } else {
        if !ctx_0.is_null() {
            x9_62_free_context(ctx_0);
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
unsafe extern "C" fn evp_keyex_free(mut ctx_0: *mut st_evp_keyex_context_t) {
    if !(*ctx_0).privkey.is_null() {
        EVP_PKEY_free((*ctx_0).privkey);
    }
    if !(*ctx_0).super_0.pubkey.base.is_null() {
        CRYPTO_free(
            (*ctx_0).super_0.pubkey.base as *mut ::core::ffi::c_void,
            b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/../lib/openssl.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            503 as ::core::ffi::c_int,
        );
    }
    free(ctx_0 as *mut ::core::ffi::c_void);
}
unsafe extern "C" fn evp_keyex_on_exchange(
    mut _ctx: *mut *mut ptls_key_exchange_context_t,
    mut release: ::core::ffi::c_int,
    mut secret: *mut ptls_iovec_t,
    mut peerkey: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut ctx_0: *mut st_evp_keyex_context_t =
        *_ctx as *mut ::core::ffi::c_void as *mut st_evp_keyex_context_t;
    let mut evppeer: *mut EVP_PKEY = ::core::ptr::null_mut::<EVP_PKEY>();
    let mut evpctx: *mut EVP_PKEY_CTX = ::core::ptr::null_mut::<EVP_PKEY_CTX>();
    let mut ret: ::core::ffi::c_int = 0;
    if secret.is_null() {
        ret = 0 as ::core::ffi::c_int;
    } else {
        (*secret).base = ::core::ptr::null_mut::<uint8_t>();
        if peerkey.len != (*ctx_0).super_0.pubkey.len {
            ret = PTLS_ALERT_DECRYPT_ERROR;
        } else {
            evppeer = EVP_PKEY_new();
            if evppeer.is_null() {
                ret = PTLS_ERROR_NO_MEMORY;
            } else if EVP_PKEY_copy_parameters(evppeer, (*ctx_0).privkey) <= 0 as ::core::ffi::c_int
            {
                ret = PTLS_ERROR_LIBRARY;
            } else if EVP_PKEY_set1_encoded_public_key(evppeer, peerkey.base, peerkey.len)
                <= 0 as ::core::ffi::c_int
            {
                ret = PTLS_ERROR_LIBRARY;
            } else {
                evpctx = EVP_PKEY_CTX_new((*ctx_0).privkey, ::core::ptr::null_mut::<ENGINE>());
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
        evp_keyex_free(ctx_0);
        *_ctx = ::core::ptr::null_mut::<ptls_key_exchange_context_t>();
    }
    return ret;
}
unsafe extern "C" fn evp_keyex_init(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut _ctx: *mut *mut ptls_key_exchange_context_t,
    mut pkey: *mut EVP_PKEY,
) -> ::core::ffi::c_int {
    let mut ctx_0: *mut st_evp_keyex_context_t = ::core::ptr::null_mut::<st_evp_keyex_context_t>();
    let mut ret: ::core::ffi::c_int = 0;
    ctx_0 = malloc(::core::mem::size_of::<st_evp_keyex_context_t>() as size_t)
        as *mut st_evp_keyex_context_t;
    if ctx_0.is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else {
        *ctx_0 = st_evp_keyex_context_t {
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
        (*ctx_0).super_0.pubkey.len = EVP_PKEY_get1_encoded_public_key(
            (*ctx_0).privkey,
            &raw mut (*ctx_0).super_0.pubkey.base,
        );
        if (*ctx_0).super_0.pubkey.len == 0 as size_t {
            (*ctx_0).super_0.pubkey.base = ::core::ptr::null_mut::<uint8_t>();
            ret = PTLS_ERROR_NO_MEMORY;
        } else {
            *_ctx = &raw mut (*ctx_0).super_0;
            ret = 0 as ::core::ffi::c_int;
        }
    }
    if ret != 0 as ::core::ffi::c_int && !ctx_0.is_null() {
        (*ctx_0).privkey = ::core::ptr::null_mut::<EVP_PKEY>();
        evp_keyex_free(ctx_0);
    }
    return ret;
}
unsafe extern "C" fn evp_keyex_create(
    mut algo: *const ptls_key_exchange_algorithm_t,
    mut ctx_0: *mut *mut ptls_key_exchange_context_t,
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
        ret = evp_keyex_init(algo, ctx_0, pkey);
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
    let mut ctx_0: *mut ptls_key_exchange_context_t =
        ::core::ptr::null_mut::<ptls_key_exchange_context_t>();
    let mut ret: ::core::ffi::c_int = 0;
    (*outpubkey).base = ::core::ptr::null_mut::<uint8_t>();
    ret = evp_keyex_create(algo, &raw mut ctx_0);
    if !(ret != 0 as ::core::ffi::c_int) {
        (*outpubkey).base = malloc((*ctx_0).pubkey.len) as *mut uint8_t;
        if (*outpubkey).base.is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        } else {
            memcpy(
                (*outpubkey).base as *mut ::core::ffi::c_void,
                (*ctx_0).pubkey.base as *const ::core::ffi::c_void,
                (*ctx_0).pubkey.len,
            );
            (*outpubkey).len = (*ctx_0).pubkey.len;
            ret = evp_keyex_on_exchange(&raw mut ctx_0, 1 as ::core::ffi::c_int, secret, peerkey);
        }
    }
    if !ctx_0.is_null() {
        evp_keyex_on_exchange(
            &raw mut ctx_0,
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
    mut ctx_0: *mut *mut ptls_key_exchange_context_t,
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
            ret = x9_62_init_key(algo, ctx_0, eckey);
            if ret != 0 as ::core::ffi::c_int {
                EC_KEY_free(eckey);
                return ret;
            }
            return 0 as ::core::ffi::c_int;
        }
        NID_X25519 => {
            ret = evp_keyex_init(&raw const ptls_openssl_x25519, ctx_0, pkey);
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
            NULL_0,
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
    mut ctx_0: *mut EVP_MD_CTX,
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
    (*self_0).ctx = ctx_0;
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
    let mut ctx_0: *mut EVP_MD_CTX = ::core::ptr::null_mut::<EVP_MD_CTX>();
    let mut md: *const EVP_MD = if (*scheme).scheme_md.is_some() {
        (*scheme).scheme_md.expect("non-null function pointer")()
    } else {
        ::core::ptr::null::<EVP_MD>()
    };
    let mut pkey_ctx: *mut EVP_PKEY_CTX = ::core::ptr::null_mut::<EVP_PKEY_CTX>();
    let mut siglen: size_t = 0;
    let mut ret: ::core::ffi::c_int = 0;
    ctx_0 = EVP_MD_CTX_new();
    if ctx_0.is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else if EVP_DigestSignInit(
        ctx_0,
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
                ctx_0,
                ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                &raw mut siglen,
                input.base,
                input.len,
            ) != 1 as ::core::ffi::c_int
            {
                ret = PTLS_ERROR_LIBRARY;
                c2rust_current_block = 10239400055296667058;
            } else {
                ret = ptls_buffer_reserve(outbuf, siglen);
                if ret != 0 as ::core::ffi::c_int {
                    c2rust_current_block = 10239400055296667058;
                } else if EVP_DigestSign(
                    ctx_0,
                    (*outbuf).base.offset((*outbuf).off as isize),
                    &raw mut siglen,
                    input.base,
                    input.len,
                ) != 1 as ::core::ffi::c_int
                {
                    ret = PTLS_ERROR_LIBRARY;
                    c2rust_current_block = 10239400055296667058;
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
                    c2rust_current_block = 10239400055296667058;
                } else if EVP_PKEY_CTX_set_rsa_pss_saltlen(pkey_ctx, -(1 as ::core::ffi::c_int))
                    != 1 as ::core::ffi::c_int
                {
                    ret = PTLS_ERROR_LIBRARY;
                    c2rust_current_block = 10239400055296667058;
                } else if EVP_PKEY_CTX_set_rsa_mgf1_md(pkey_ctx, md) != 1 as ::core::ffi::c_int {
                    ret = PTLS_ERROR_LIBRARY;
                    c2rust_current_block = 10239400055296667058;
                } else {
                    c2rust_current_block = 12124785117276362961;
                }
            } else {
                c2rust_current_block = 12124785117276362961;
            }
            match c2rust_current_block {
                10239400055296667058 => {}
                _ => {
                    if EVP_DigestSignUpdate(
                        ctx_0,
                        input.base as *const ::core::ffi::c_void,
                        input.len,
                    ) != 1 as ::core::ffi::c_int
                    {
                        ret = PTLS_ERROR_LIBRARY;
                        c2rust_current_block = 10239400055296667058;
                    } else if EVP_DigestSignFinal(
                        ctx_0,
                        ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
                        &raw mut siglen,
                    ) != 1 as ::core::ffi::c_int
                    {
                        ret = PTLS_ERROR_LIBRARY;
                        c2rust_current_block = 10239400055296667058;
                    } else if !async_0.is_null() {
                        *async_0 = async_sign_ctx_new(scheme, ctx_0, siglen);
                        if (*async_0).is_null() {
                            ret = PTLS_ERROR_NO_MEMORY;
                        } else {
                            return do_sign_async(outbuf, async_0);
                        }
                        c2rust_current_block = 10239400055296667058;
                    } else {
                        ret = ptls_buffer_reserve(outbuf, siglen);
                        if ret != 0 as ::core::ffi::c_int {
                            c2rust_current_block = 10239400055296667058;
                        } else if EVP_DigestSignFinal(
                            ctx_0,
                            (*outbuf).base.offset((*outbuf).off as isize),
                            &raw mut siglen,
                        ) != 1 as ::core::ffi::c_int
                        {
                            ret = PTLS_ERROR_LIBRARY;
                            c2rust_current_block = 10239400055296667058;
                        } else {
                            c2rust_current_block = 11636175345244025579;
                        }
                    }
                }
            }
        }
        match c2rust_current_block {
            10239400055296667058 => {}
            _ => {
                (*outbuf).off = (*outbuf).off.wrapping_add(siglen);
                ret = 0 as ::core::ffi::c_int;
            }
        }
    }
    if !ctx_0.is_null() {
        EVP_MD_CTX_free(ctx_0);
    }
    return ret;
}
unsafe extern "C" fn cipher_dispose(mut _ctx: *mut ptls_cipher_context_t) {
    let mut ctx_0: *mut cipher_context_t = _ctx as *mut cipher_context_t;
    EVP_CIPHER_CTX_free((*ctx_0).evp);
}
unsafe extern "C" fn cipher_do_init(
    mut _ctx: *mut ptls_cipher_context_t,
    mut iv: *const ::core::ffi::c_void,
) {
    let mut ctx_0: *mut cipher_context_t = _ctx as *mut cipher_context_t;
    let mut ret: ::core::ffi::c_int = 0;
    ret = EVP_EncryptInit_ex(
        (*ctx_0).evp,
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
    let mut ctx_0: *mut cipher_context_t = _ctx as *mut cipher_context_t;
    (*ctx_0).super_0.do_dispose =
        Some(cipher_dispose as unsafe extern "C" fn(*mut ptls_cipher_context_t) -> ())
            as Option<unsafe extern "C" fn(*mut st_ptls_cipher_context_t) -> ()>;
    (*ctx_0).super_0.do_init = Some(
        cipher_do_init
            as unsafe extern "C" fn(*mut ptls_cipher_context_t, *const ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut st_ptls_cipher_context_t, *const ::core::ffi::c_void) -> (),
        >;
    (*ctx_0).super_0.do_transform = do_transform
        as Option<
            unsafe extern "C" fn(
                *mut st_ptls_cipher_context_t,
                *mut ::core::ffi::c_void,
                *const ::core::ffi::c_void,
                size_t,
            ) -> (),
        >;
    (*ctx_0).evp = EVP_CIPHER_CTX_new();
    if (*ctx_0).evp.is_null() {
        return PTLS_ERROR_NO_MEMORY;
    }
    if is_enc != 0 {
        if EVP_EncryptInit_ex(
            (*ctx_0).evp,
            cipher,
            ::core::ptr::null_mut::<ENGINE>(),
            key as *const ::core::ffi::c_uchar,
            ::core::ptr::null::<::core::ffi::c_uchar>(),
        ) == 0
        {
            c2rust_current_block = 1128862024615669987;
        } else {
            c2rust_current_block = 14523784380283086299;
        }
    } else if EVP_DecryptInit_ex(
        (*ctx_0).evp,
        cipher,
        ::core::ptr::null_mut::<ENGINE>(),
        key as *const ::core::ffi::c_uchar,
        ::core::ptr::null::<::core::ffi::c_uchar>(),
    ) == 0
    {
        c2rust_current_block = 1128862024615669987;
    } else {
        EVP_CIPHER_CTX_set_padding((*ctx_0).evp, 0 as ::core::ffi::c_int);
        c2rust_current_block = 14523784380283086299;
    }
    match c2rust_current_block {
        14523784380283086299 => return 0 as ::core::ffi::c_int,
        _ => {
            EVP_CIPHER_CTX_free((*ctx_0).evp);
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
    let mut ctx_0: *mut cipher_context_t = _ctx as *mut cipher_context_t;
    let mut len: ::core::ffi::c_int = _len as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = EVP_EncryptUpdate(
        (*ctx_0).evp,
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
    let mut ctx_0: *mut cipher_context_t = _ctx as *mut cipher_context_t;
    let mut len: ::core::ffi::c_int = _len as ::core::ffi::c_int;
    let mut ret: ::core::ffi::c_int = EVP_DecryptUpdate(
        (*ctx_0).evp,
        output as *mut ::core::ffi::c_uchar,
        &raw mut len,
        input as *const ::core::ffi::c_uchar,
        len,
    );
}
unsafe extern "C" fn aes128ecb_setup_crypto(
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return cipher_setup_crypto(
        ctx_0,
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
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return cipher_setup_crypto(
        ctx_0,
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
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return cipher_setup_crypto(
        ctx_0,
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
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return cipher_setup_crypto(
        ctx_0,
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
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return cipher_setup_crypto(
        ctx_0,
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
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return cipher_setup_crypto(
        ctx_0,
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
    let mut ctx_0: *mut aead_crypto_context_t = _ctx as *mut aead_crypto_context_t;
    if !(*ctx_0).evp_ctx.is_null() {
        EVP_CIPHER_CTX_free((*ctx_0).evp_ctx);
    }
}
unsafe extern "C" fn aead_get_iv(
    mut _ctx: *mut ptls_aead_context_t,
    mut iv: *mut ::core::ffi::c_void,
) {
    let mut ctx_0: *mut aead_crypto_context_t = _ctx as *mut aead_crypto_context_t;
    memcpy(
        iv,
        &raw mut (*ctx_0).static_iv as *mut uint8_t as *const ::core::ffi::c_void,
        (*(*ctx_0).super_0.algo).iv_size,
    );
}
unsafe extern "C" fn aead_set_iv(
    mut _ctx: *mut ptls_aead_context_t,
    mut iv: *const ::core::ffi::c_void,
) {
    let mut ctx_0: *mut aead_crypto_context_t = _ctx as *mut aead_crypto_context_t;
    memcpy(
        &raw mut (*ctx_0).static_iv as *mut uint8_t as *mut ::core::ffi::c_void,
        iv,
        (*(*ctx_0).super_0.algo).iv_size,
    );
}
unsafe extern "C" fn aead_do_encrypt_init(
    mut _ctx: *mut ptls_aead_context_t,
    mut seq: uint64_t,
    mut aad: *const ::core::ffi::c_void,
    mut aadlen: size_t,
) {
    let mut ctx_0: *mut aead_crypto_context_t = _ctx as *mut aead_crypto_context_t;
    let mut iv: [uint8_t; 32] = [0; 32];
    let mut ret: ::core::ffi::c_int = 0;
    ptls_aead__build_iv(
        (*ctx_0).super_0.algo as *const ptls_aead_algorithm_t,
        &raw mut iv as *mut uint8_t,
        &raw mut (*ctx_0).static_iv as *mut uint8_t,
        seq,
    );
    ret = EVP_EncryptInit_ex(
        (*ctx_0).evp_ctx,
        ::core::ptr::null::<EVP_CIPHER>(),
        ::core::ptr::null_mut::<ENGINE>(),
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        &raw mut iv as *mut uint8_t,
    );
    if aadlen != 0 as size_t {
        let mut blocklen: ::core::ffi::c_int = 0;
        ret = EVP_EncryptUpdate(
            (*ctx_0).evp_ctx,
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
    let mut ctx_0: *mut aead_crypto_context_t = _ctx as *mut aead_crypto_context_t;
    let mut blocklen: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    ret = EVP_EncryptUpdate(
        (*ctx_0).evp_ctx,
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
    let mut ctx_0: *mut aead_crypto_context_t = _ctx as *mut aead_crypto_context_t;
    let mut output: *mut uint8_t = _output as *mut uint8_t;
    let mut off: size_t = 0 as size_t;
    let mut tag_size: size_t = (*(*ctx_0).super_0.algo).tag_size;
    let mut blocklen: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    ret = EVP_EncryptFinal_ex(
        (*ctx_0).evp_ctx,
        output.offset(off as isize),
        &raw mut blocklen,
    );
    off = off.wrapping_add(blocklen as size_t);
    ret = EVP_CIPHER_CTX_ctrl(
        (*ctx_0).evp_ctx,
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
    let mut ctx_0: *mut aead_crypto_context_t = _ctx as *mut aead_crypto_context_t;
    let mut output: *mut uint8_t = _output as *mut uint8_t;
    let mut iv: [uint8_t; 32] = [0; 32];
    let mut off: size_t = 0 as size_t;
    let mut tag_size: size_t = (*(*ctx_0).super_0.algo).tag_size;
    let mut blocklen: ::core::ffi::c_int = 0;
    let mut ret: ::core::ffi::c_int = 0;
    if inlen < tag_size {
        return SIZE_MAX as size_t;
    }
    ptls_aead__build_iv(
        (*ctx_0).super_0.algo as *const ptls_aead_algorithm_t,
        &raw mut iv as *mut uint8_t,
        &raw mut (*ctx_0).static_iv as *mut uint8_t,
        seq,
    );
    ret = EVP_DecryptInit_ex(
        (*ctx_0).evp_ctx,
        ::core::ptr::null::<EVP_CIPHER>(),
        ::core::ptr::null_mut::<ENGINE>(),
        ::core::ptr::null::<::core::ffi::c_uchar>(),
        &raw mut iv as *mut uint8_t,
    );
    if aadlen != 0 as size_t {
        ret = EVP_DecryptUpdate(
            (*ctx_0).evp_ctx,
            ::core::ptr::null_mut::<::core::ffi::c_uchar>(),
            &raw mut blocklen,
            aad as *const ::core::ffi::c_uchar,
            aadlen as ::core::ffi::c_int,
        );
    }
    ret = EVP_DecryptUpdate(
        (*ctx_0).evp_ctx,
        output.offset(off as isize),
        &raw mut blocklen,
        input as *const ::core::ffi::c_uchar,
        inlen.wrapping_sub(tag_size) as ::core::ffi::c_int,
    );
    off = off.wrapping_add(blocklen as size_t);
    if EVP_CIPHER_CTX_ctrl(
        (*ctx_0).evp_ctx,
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
        (*ctx_0).evp_ctx,
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
    let mut ctx_0: *mut aead_crypto_context_t = _ctx as *mut aead_crypto_context_t;
    let mut ret: ::core::ffi::c_int = 0;
    (*ctx_0).super_0.dispose_crypto =
        Some(aead_dispose_crypto as unsafe extern "C" fn(*mut ptls_aead_context_t) -> ())
            as Option<unsafe extern "C" fn(*mut st_ptls_aead_context_t) -> ()>;
    (*ctx_0).super_0.do_get_iv = Some(
        aead_get_iv
            as unsafe extern "C" fn(*mut ptls_aead_context_t, *mut ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut st_ptls_aead_context_t, *mut ::core::ffi::c_void) -> (),
        >;
    (*ctx_0).super_0.do_set_iv = Some(
        aead_set_iv
            as unsafe extern "C" fn(*mut ptls_aead_context_t, *const ::core::ffi::c_void) -> (),
    )
        as Option<
            unsafe extern "C" fn(*mut st_ptls_aead_context_t, *const ::core::ffi::c_void) -> (),
        >;
    if is_enc != 0 {
        (*ctx_0).super_0.do_encrypt_init = Some(
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
        (*ctx_0).super_0.do_encrypt_update = Some(
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
        (*ctx_0).super_0.do_encrypt_final = Some(
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
        (*ctx_0).super_0.do_encrypt = Some(
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
        (*ctx_0).super_0.do_encrypt_v = Some(
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
        (*ctx_0).super_0.do_decrypt = None;
    } else {
        (*ctx_0).super_0.do_encrypt_init = None;
        (*ctx_0).super_0.do_encrypt_update = None;
        (*ctx_0).super_0.do_encrypt_final = None;
        (*ctx_0).super_0.do_encrypt = None;
        (*ctx_0).super_0.do_encrypt_v = None;
        (*ctx_0).super_0.do_decrypt = Some(
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
    (*ctx_0).evp_ctx = ::core::ptr::null_mut::<EVP_CIPHER_CTX>();
    (*ctx_0).evp_ctx = EVP_CIPHER_CTX_new();
    if (*ctx_0).evp_ctx.is_null() {
        ret = PTLS_ERROR_NO_MEMORY;
    } else {
        if is_enc != 0 {
            if EVP_EncryptInit_ex(
                (*ctx_0).evp_ctx,
                cipher,
                ::core::ptr::null_mut::<ENGINE>(),
                key as *const ::core::ffi::c_uchar,
                ::core::ptr::null::<::core::ffi::c_uchar>(),
            ) == 0
            {
                ret = PTLS_ERROR_LIBRARY;
                c2rust_current_block = 11615362312668390118;
            } else {
                c2rust_current_block = 5601891728916014340;
            }
        } else if EVP_DecryptInit_ex(
            (*ctx_0).evp_ctx,
            cipher,
            ::core::ptr::null_mut::<ENGINE>(),
            key as *const ::core::ffi::c_uchar,
            ::core::ptr::null::<::core::ffi::c_uchar>(),
        ) == 0
        {
            ret = PTLS_ERROR_LIBRARY;
            c2rust_current_block = 11615362312668390118;
        } else {
            c2rust_current_block = 5601891728916014340;
        }
        match c2rust_current_block {
            11615362312668390118 => {}
            _ => {
                if EVP_CIPHER_CTX_ctrl(
                    (*ctx_0).evp_ctx,
                    EVP_CTRL_GCM_SET_IVLEN,
                    (*(*ctx_0).super_0.algo).iv_size as ::core::ffi::c_int,
                    NULL_0,
                ) == 0
                {
                    ret = PTLS_ERROR_LIBRARY;
                } else {
                    memcpy(
                        &raw mut (*ctx_0).static_iv as *mut uint8_t as *mut ::core::ffi::c_void,
                        iv,
                        (*(*ctx_0).super_0.algo).iv_size,
                    );
                    return 0 as ::core::ffi::c_int;
                }
            }
        }
    }
    aead_dispose_crypto(&raw mut (*ctx_0).super_0);
    return ret;
}
unsafe extern "C" fn aead_aes128gcm_setup_crypto(
    mut ctx_0: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return aead_setup_crypto(ctx_0, is_enc, key, iv, EVP_aes_128_gcm());
}
unsafe extern "C" fn aead_aes256gcm_setup_crypto(
    mut ctx_0: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return aead_setup_crypto(ctx_0, is_enc, key, iv, EVP_aes_256_gcm());
}
unsafe extern "C" fn aead_chacha20poly1305_setup_crypto(
    mut ctx_0: *mut ptls_aead_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
    mut iv: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return aead_setup_crypto(ctx_0, is_enc, key, iv, EVP_chacha20_poly1305());
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
unsafe extern "C" fn sha256_final(
    mut _ctx: *mut ptls_hash_context_t,
    mut md: *mut ::core::ffi::c_void,
    mut mode: ptls_hash_final_mode_t,
) {
    let mut ctx_0: *mut sha256_context_t = _ctx as *mut sha256_context_t;
    if mode as ::core::ffi::c_uint
        == PTLS_HASH_FINAL_MODE_SNAPSHOT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut copy: SHA256_CTX = (*ctx_0).ctx;
        SHA256_Final(md as *mut ::core::ffi::c_uchar, &raw mut copy);
        ptls_clear_memory.expect("non-null function pointer")(
            &raw mut copy as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<SHA256_CTX>() as size_t,
        );
        return;
    }
    if !md.is_null() {
        SHA256_Final(md as *mut ::core::ffi::c_uchar, &raw mut (*ctx_0).ctx);
    }
    match mode as ::core::ffi::c_uint {
        0 => {
            ptls_clear_memory.expect("non-null function pointer")(
                &raw mut (*ctx_0).ctx as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<SHA256_CTX>() as size_t,
            );
            free(ctx_0 as *mut ::core::ffi::c_void);
        }
        1 => {
            SHA256_Init(&raw mut (*ctx_0).ctx);
        }
        _ => {}
    };
}
unsafe extern "C" fn sha256_update(
    mut _ctx: *mut ptls_hash_context_t,
    mut src: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut ctx_0: *mut sha256_context_t = _ctx as *mut sha256_context_t;
    SHA256_Update(&raw mut (*ctx_0).ctx, src, len);
}
unsafe extern "C" fn sha256_create() -> *mut ptls_hash_context_t {
    let mut ctx_0: *mut sha256_context_t = ::core::ptr::null_mut::<sha256_context_t>();
    ctx_0 = malloc(::core::mem::size_of::<sha256_context_t>() as size_t) as *mut sha256_context_t;
    if ctx_0.is_null() {
        return ::core::ptr::null_mut::<ptls_hash_context_t>();
    }
    (*ctx_0).super_0 = st_ptls_hash_context_t {
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
    SHA256_Init(&raw mut (*ctx_0).ctx);
    return &raw mut (*ctx_0).super_0;
}
unsafe extern "C" fn sha384_create() -> *mut ptls_hash_context_t {
    let mut ctx_0: *mut sha384_context_t = ::core::ptr::null_mut::<sha384_context_t>();
    ctx_0 = malloc(::core::mem::size_of::<sha384_context_t>() as size_t) as *mut sha384_context_t;
    if ctx_0.is_null() {
        return ::core::ptr::null_mut::<ptls_hash_context_t>();
    }
    (*ctx_0).super_0 = st_ptls_hash_context_t {
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
    SHA384_Init(&raw mut (*ctx_0).ctx);
    return &raw mut (*ctx_0).super_0;
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
unsafe extern "C" fn sha384_final(
    mut _ctx: *mut ptls_hash_context_t,
    mut md: *mut ::core::ffi::c_void,
    mut mode: ptls_hash_final_mode_t,
) {
    let mut ctx_0: *mut sha384_context_t = _ctx as *mut sha384_context_t;
    if mode as ::core::ffi::c_uint
        == PTLS_HASH_FINAL_MODE_SNAPSHOT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut copy: SHA512_CTX = (*ctx_0).ctx;
        SHA384_Final(md as *mut ::core::ffi::c_uchar, &raw mut copy);
        ptls_clear_memory.expect("non-null function pointer")(
            &raw mut copy as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<SHA512_CTX>() as size_t,
        );
        return;
    }
    if !md.is_null() {
        SHA384_Final(md as *mut ::core::ffi::c_uchar, &raw mut (*ctx_0).ctx);
    }
    match mode as ::core::ffi::c_uint {
        0 => {
            ptls_clear_memory.expect("non-null function pointer")(
                &raw mut (*ctx_0).ctx as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<SHA512_CTX>() as size_t,
            );
            free(ctx_0 as *mut ::core::ffi::c_void);
        }
        1 => {
            SHA384_Init(&raw mut (*ctx_0).ctx);
        }
        _ => {}
    };
}
unsafe extern "C" fn sha384_update(
    mut _ctx: *mut ptls_hash_context_t,
    mut src: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut ctx_0: *mut sha384_context_t = _ctx as *mut sha384_context_t;
    SHA384_Update(&raw mut (*ctx_0).ctx, src, len);
}
unsafe extern "C" fn sha512_final(
    mut _ctx: *mut ptls_hash_context_t,
    mut md: *mut ::core::ffi::c_void,
    mut mode: ptls_hash_final_mode_t,
) {
    let mut ctx_0: *mut sha512_context_t = _ctx as *mut sha512_context_t;
    if mode as ::core::ffi::c_uint
        == PTLS_HASH_FINAL_MODE_SNAPSHOT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        let mut copy: SHA512_CTX = (*ctx_0).ctx;
        SHA512_Final(md as *mut ::core::ffi::c_uchar, &raw mut copy);
        ptls_clear_memory.expect("non-null function pointer")(
            &raw mut copy as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<SHA512_CTX>() as size_t,
        );
        return;
    }
    if !md.is_null() {
        SHA512_Final(md as *mut ::core::ffi::c_uchar, &raw mut (*ctx_0).ctx);
    }
    match mode as ::core::ffi::c_uint {
        0 => {
            ptls_clear_memory.expect("non-null function pointer")(
                &raw mut (*ctx_0).ctx as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<SHA512_CTX>() as size_t,
            );
            free(ctx_0 as *mut ::core::ffi::c_void);
        }
        1 => {
            SHA512_Init(&raw mut (*ctx_0).ctx);
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
unsafe extern "C" fn sha512_update(
    mut _ctx: *mut ptls_hash_context_t,
    mut src: *const ::core::ffi::c_void,
    mut len: size_t,
) {
    let mut ctx_0: *mut sha512_context_t = _ctx as *mut sha512_context_t;
    SHA512_Update(&raw mut (*ctx_0).ctx, src, len);
}
unsafe extern "C" fn sha512_create() -> *mut ptls_hash_context_t {
    let mut ctx_0: *mut sha512_context_t = ::core::ptr::null_mut::<sha512_context_t>();
    ctx_0 = malloc(::core::mem::size_of::<sha512_context_t>() as size_t) as *mut sha512_context_t;
    if ctx_0.is_null() {
        return ::core::ptr::null_mut::<ptls_hash_context_t>();
    }
    (*ctx_0).super_0 = st_ptls_hash_context_t {
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
    SHA512_Init(&raw mut (*ctx_0).ctx);
    return &raw mut (*ctx_0).super_0;
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
    let mut ctx_0: *mut EVP_MD_CTX = ::core::ptr::null_mut::<EVP_MD_CTX>();
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
                    c2rust_current_block = 12520558700972726212;
                    break;
                }
                scheme = scheme.offset(1);
            }
            match c2rust_current_block {
                11006700562992250127 => {
                    ret = PTLS_ALERT_ILLEGAL_PARAMETER;
                }
                _ => {
                    ctx_0 = EVP_MD_CTX_new();
                    if ctx_0.is_null() {
                        ret = PTLS_ERROR_NO_MEMORY;
                    } else {
                        if EVP_PKEY_get_id(key) == EVP_PKEY_ED25519 {
                            if EVP_DigestVerifyInit(
                                ctx_0,
                                &raw mut pkey_ctx,
                                ::core::ptr::null::<EVP_MD>(),
                                ::core::ptr::null_mut::<ENGINE>(),
                                key,
                            ) != 1 as ::core::ffi::c_int
                            {
                                ret = PTLS_ERROR_LIBRARY;
                                c2rust_current_block = 56049084192931913;
                            } else if EVP_DigestVerify(
                                ctx_0,
                                signature.base,
                                signature.len,
                                data.base,
                                data.len,
                            ) != 1 as ::core::ffi::c_int
                            {
                                ret = PTLS_ERROR_LIBRARY;
                                c2rust_current_block = 56049084192931913;
                            } else {
                                c2rust_current_block = 11932355480408055363;
                            }
                        } else if EVP_DigestVerifyInit(
                            ctx_0,
                            &raw mut pkey_ctx,
                            (*scheme).scheme_md.expect("non-null function pointer")(),
                            ::core::ptr::null_mut::<ENGINE>(),
                            key,
                        ) != 1 as ::core::ffi::c_int
                        {
                            ret = PTLS_ERROR_LIBRARY;
                            c2rust_current_block = 56049084192931913;
                        } else {
                            if EVP_PKEY_get_id(key) == EVP_PKEY_RSA {
                                if EVP_PKEY_CTX_set_rsa_padding(pkey_ctx, RSA_PKCS1_PSS_PADDING)
                                    != 1 as ::core::ffi::c_int
                                {
                                    ret = PTLS_ERROR_LIBRARY;
                                    c2rust_current_block = 56049084192931913;
                                } else if EVP_PKEY_CTX_set_rsa_pss_saltlen(
                                    pkey_ctx,
                                    -(1 as ::core::ffi::c_int),
                                ) != 1 as ::core::ffi::c_int
                                {
                                    ret = PTLS_ERROR_LIBRARY;
                                    c2rust_current_block = 56049084192931913;
                                } else if EVP_PKEY_CTX_set_rsa_mgf1_md(
                                    pkey_ctx,
                                    (*scheme).scheme_md.expect("non-null function pointer")(),
                                ) != 1 as ::core::ffi::c_int
                                {
                                    ret = PTLS_ERROR_LIBRARY;
                                    c2rust_current_block = 56049084192931913;
                                } else {
                                    c2rust_current_block = 16203760046146113240;
                                }
                            } else {
                                c2rust_current_block = 16203760046146113240;
                            }
                            match c2rust_current_block {
                                56049084192931913 => {}
                                _ => {
                                    if EVP_DigestVerifyUpdate(
                                        ctx_0,
                                        data.base as *const ::core::ffi::c_void,
                                        data.len,
                                    ) != 1 as ::core::ffi::c_int
                                    {
                                        ret = PTLS_ERROR_LIBRARY;
                                        c2rust_current_block = 56049084192931913;
                                    } else if EVP_DigestVerifyFinal(
                                        ctx_0,
                                        signature.base,
                                        signature.len,
                                    ) != 1 as ::core::ffi::c_int
                                    {
                                        ret = PTLS_ALERT_DECRYPT_ERROR;
                                        c2rust_current_block = 56049084192931913;
                                    } else {
                                        c2rust_current_block = 11932355480408055363;
                                    }
                                }
                            }
                        }
                        match c2rust_current_block {
                            56049084192931913 => {}
                            _ => {
                                ret = 0 as ::core::ffi::c_int;
                            }
                        }
                    }
                }
            }
        }
    }
    if !ctx_0.is_null() {
        EVP_MD_CTX_free(ctx_0);
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
    mut ctx_0: *mut ptls_context_t,
    mut cert: *mut X509,
    mut chain: *mut stack_st_X509,
) -> ::core::ffi::c_int {
    let mut c2rust_current_block: u64;
    let mut list: *mut ptls_iovec_t = ::core::ptr::null_mut::<ptls_iovec_t>();
    let mut slot: size_t = 0 as size_t;
    let mut count: size_t = ((cert != NULL_0 as *mut X509) as ::core::ffi::c_int
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
                c2rust_current_block = 13701241046389949905;
            } else {
                c2rust_current_block = 11875828834189669668;
            }
        } else {
            c2rust_current_block = 11875828834189669668;
        }
        match c2rust_current_block {
            13701241046389949905 => {}
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
                            c2rust_current_block = 13701241046389949905;
                            break;
                        }
                        i += 1;
                    }
                } else {
                    c2rust_current_block = 7651349459974463963;
                }
                match c2rust_current_block {
                    13701241046389949905 => {}
                    _ => {
                        (*ctx_0).certificates.list = list;
                        (*ctx_0).certificates.count = count;
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
            c2rust_current_block = 906253772178046446;
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
                    c2rust_current_block = 906253772178046446;
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
                906253772178046446 => {}
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
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/../lib/openssl.c\0"
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
pub const SECP256R1_PRIVATE_KEY: [::core::ffi::c_char; 33] = unsafe {
    ::core::mem::transmute::<
        [u8; 33],
        [::core::ffi::c_char; 33],
    >(
        *b"\xC1t\xB4\xF9^\xFEz\x01\x0E\xBEJ\xE83\xB26\x13\xFCe\xE9e\x91\xA89\x9E\x9A\x80\xFB\xAB\xD1\xFF\xBA:\0",
    )
};
pub const SECP256R1_CERTIFICATE: [::core::ffi::c_char; 613] = unsafe {
    ::core::mem::transmute::<
        [u8; 613],
        [::core::ffi::c_char; 613],
    >(
        *b"0\x82\x02`0\x82\x01H\xA0\x03\x02\x01\x02\x02\x01\x010\r\x06\t*\x86H\x86\xF7\r\x01\x01\x0B\x05\x000\x1A1\x180\x16\x06\x03U\x04\x03\x13\x0Fpicotls test ca0\x1E\x17\r180223053104Z\x17\r280221053104Z0\x1B1\x190\x17\x06\x03U\x04\x03\x13\x10test.example.com0Y0\x13\x06\x07*\x86H\xCE=\x02\x01\x06\x08*\x86H\xCE=\x03\x01\x07\x03B\0\x04\xDA\xC8\xA5@T\xBA3\xDA\x18\xA9A\x7FIS\xDF`\xE6\xA6=\xB6\x8ES:\x9F\xDD\x19\x14^\xAB\x03\xCF\xBC\xFB6\x98\x16$\x8F\x07)m\x15\xD8O0\xE8\td\xFB\x14\xFC\x86|\xD4\x06\xC2\xFD\x9D\xE8\x99?H\x8C+\xA3{0y0\t\x06\x03U\x1D\x13\x04\x020\x000,\x06\t`\x86H\x01\x86\xF8B\x01\r\x04\x1F\x16\x1DOpenSSL Generated Certificate0\x1D\x06\x03U\x1D\x0E\x04\x16\x04\x14\xEE0\x86\x16\xA1\xD2i\xADd\xE4\xD7wk\xB2\xFD\\O\x01\xA2\xB50\x1F\x06\x03U\x1D#\x04\x180\x16\x80\x14\xBFy\xCA\x97\xB2`x \x96\xAAFW\x9C\xDF\xA7\xB2#\xF5%c0\r\x06\t*\x86H\x86\xF7\r\x01\x01\x0B\x05\0\x03\x82\x01\x01\0\x8F\xAC\x9C\x01m\x81\xAA\x8C\xAE]\xB5\x16t\xEA\xE8\xEB&[\xB1f\xD5k\xD4My\rm\x87\xA9\xB6\xBFt-\xC1\xB2.R\xB6K\xCA\r\x01E8X\x1A\xD2jm \x98ZQ\xB0o,?\x0F\x12\x88\xED|\t\xA5t\0!=K\xD2-T\xAAS\x8Bd\xF9\x1E\xEA\xA5\x8A\xE7a^V\x92R6>\xA0hY\x9C}\xB3\xE8\\Kwn\xDE(\xED\x18\x91\xA9\x9C9\xD2\x96\xCC\x98\x05\x8Ct\xDC\x1E\x12[8\xBDV\xCB\xA3\xE8\xE1*Z+\xD22E\xC1\x10\x85 lk4\xEAf\x91\x0E.\xB8d\x87\x9F\x07\xBC#O#\xAD\xBE\x89\xDF\n\x98G\xE9c\x02\xD3A\xF4-\xA4\xCE\xDD\xE3\xD8A\x08\xFE\xDFG\xC0\xE7c\x8E\x1F\xF0K\xC5\xAE\xAB\xC0\xBA8>\xE3\x90\x9C\x08\xBDu\x1C\xB9\xB8TC\x1D\x99B\xE0\xA2\xB7u\xBB\x14\x03y\x9A\xF6\x07\xD8\xA5\xAB+:p\x8Bw\x85p\x8A\x988\x9B5\t\xF6bk)J\xA7\xA7\xF9;\xDE\xD8\xC8\x90W\xF2v*#\x0B\x01h\xC6\x9A\xF2\0",
    )
};
pub const ECH_CONFIG_LIST: [::core::ffi::c_char; 102] = unsafe {
    ::core::mem::transmute::<
        [u8; 102],
        [::core::ffi::c_char; 102],
    >(
        *b"\0c\xFE\r\0_\x12\0\x10\0A\x04\xFE\x8C\x19\xCE\t\x05\x19\x1E\xBC)\x8A\x92Ey%1\xF2o\x0C\xEC\xE2F\x069\xE8\xBC9\xCB\x7Fpj\x82jw\x9BL\xF9i\xB8\xA0\xE59\xC7\xF6/\xB3\xD3\n\xD6\xAA\x8F\x80\xE3\x0F\x1D\x12\x8A\xAF\xD6\x8A,\xE7.\xA0\0\x08\0\x02\0\x02\0\x01\0\x01@\x0Bexample.com\0\0\0",
    )
};
pub const ECH_PRIVATE_KEY: [::core::ffi::c_char; 242] = unsafe {
    ::core::mem::transmute::<
        [u8; 242],
        [::core::ffi::c_char; 242],
    >(
        *b"-----BEGIN PRIVATE KEY-----\nMIGHAgEAMBMGByqGSM49AgEGCCqGSM49AwEHBG0wawIBAQQg885/2uV+GjENh/Hr\nvebzKL4Kmc28rfTWWJzyneS4/9KhRANCAAT+jBnOCQUZHrwpipJFeSUx8m8M7OJG\nBjnovDnLf3Bqgmp3m0z5abig5TnH9i+z0wrWqo+A4w8dEoqv1oos5y6g\n-----END PRIVATE KEY-----\n\0",
    )
};
pub const RSA_PRIVATE_KEY: [::core::ffi::c_char; 1676] = unsafe {
    ::core::mem::transmute::<
        [u8; 1676],
        [::core::ffi::c_char; 1676],
    >(
        *b"-----BEGIN RSA PRIVATE KEY-----\nMIIEowIBAAKCAQEA5soWzSG7iyawQlHM1yaX2dUAATUkhpbg2WPFOEem7E3zYzc6\nA/Z+bViFlfEgL37cbDUb4pnOAHrrsjGgkyBYh5i9iCTVfCk+H6SOHZJORO1Tq8X9\nC7WcNcshpSdm2Pa8hmv9hsHbLSeoPNeg8NkTPwMVaMZ2GpdmiyAmhzSZ2H9mzNI7\nntPW/XCchVf+ax2yt9haZ+mQE2NPYwHDjqCtdGkP5ZXXnYhJSBzSEhxfGckIiKDy\nOxiNkLFLvUdT4ERSFBjauP2cSI0XoOUsiBxJNwHH310AU8jZbveSTcXGYgEuu2MI\nuDo7Vhkq5+TCqXsIFNbjy0taOoPRvUbPsbqFlQIDAQABAoIBAQCWcUv1wjR/2+Nw\nB+Swp267R9bt8pdxyK6f5yKrskGErremiFygMrFtVBQYjws9CsRjISehSkN4GqjE\nCweygJZVJeL++YvUmQnvFJSzgCjXU6GEStbOKD/A7T5sa0fmzMhOE907V+kpAT3x\nE1rNRaP/ImJ1X1GjuefVb0rOPiK/dehFQWfsUkOvh+J3PU76wcnexxzJgxhVxdfX\nqNa7UDsWzTImUjcHIfnhXc1K/oSKk6HjImQi/oE4lgoJUCEDaUbq0nXNrM0EmTTv\nOQ5TVP5Lds9p8UDEa55eZllGXam0zKjhDKtkQ/5UfnxsAv2adY5cuH+XN0ExfKD8\nwIZ5qINtAoGBAPRbQGZZkP/HOYA4YZ9HYAUQwFS9IZrQ8Y7C/UbL01Xli13nKalH\nxXdG6Zv6Yv0FCJKA3N945lEof9rwriwhuZbyrA1TcKok/s7HR8Bhcsm2DzRD5OiC\n3HK+Xy+6fBaMebffqBPp3Lfj/lSPNt0w/8DdrKBTw/cAy40g0n1zEu07AoGBAPHJ\nV4IfQBiblCqDh77FfQRUNR4hVbbl00Gviigiw563nk7sxdrOJ1edTyTOUBHtM3zg\nAT9sYz2CUXvsyEPqzMDANWMb9e2R//NcP6aM4k7WQRnwkZkp0WOIH95U2o1MHCYc\n5meAHVf2UMl+64xU2ZfY3rjMmPLjWMt0hKYsOmtvAoGAClIQVkJSLXtsok2/Ucrh\n81TRysJyOOe6TB1QNT1Gn8oiKMUqrUuqu27zTvM0WxtrUUTAD3A7yhG71LN1p8eE\n3ytAuQ9dItKNMI6aKTX0czCNU9fKQ0fDp9UCkDGALDOisHFx1+V4vQuUIl4qIw1+\nv9adA+iFzljqP/uy6DmEAyECgYAyWCgecf9YoFxzlbuYH2rukdIVmf9M/AHG9ZQg\n00xEKhuOd4KjErXiamDmWwcVFHzaDZJ08E6hqhbpZN42Nhe4Ms1q+5FzjCjtNVIT\njdY5cCdSDWNjru9oeBmao7R2I1jhHrdi6awyeplLu1+0cp50HbYSaJeYS3pbssFE\nEIWBhQKBgG3xleD4Sg9rG2OWQz5IrvLFg/Hy7YWyushVez61kZeLDnt9iM2um76k\n/xFNIW0a+eL2VxRTCbXr9z86hjc/6CeSJHKYFQl4zsSAZkaIJ0+HbrhDNBAYh9b2\nmRdX+OMdZ7Z5J3Glt8ENFRqe8RlESMpAKxjR+dID0bjwAjVr2KCh\n-----END RSA PRIVATE KEY-----\n\0",
    )
};
pub const RSA_CERTIFICATE: [::core::ffi::c_char; 1193] = unsafe {
    ::core::mem::transmute::<
        [u8; 1193],
        [::core::ffi::c_char; 1193],
    >(
        *b"-----BEGIN CERTIFICATE-----\nMIIDQjCCAiqgAwIBAgIBBTANBgkqhkiG9w0BAQsFADAaMRgwFgYDVQQDEw9waWNv\ndGxzIHRlc3QgY2EwHhcNMjExMjEzMDY1MzQwWhcNMzExMjExMDY1MzQwWjAbMRkw\nFwYDVQQDExB0ZXN0LmV4YW1wbGUuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8A\nMIIBCgKCAQEA5soWzSG7iyawQlHM1yaX2dUAATUkhpbg2WPFOEem7E3zYzc6A/Z+\nbViFlfEgL37cbDUb4pnOAHrrsjGgkyBYh5i9iCTVfCk+H6SOHZJORO1Tq8X9C7Wc\nNcshpSdm2Pa8hmv9hsHbLSeoPNeg8NkTPwMVaMZ2GpdmiyAmhzSZ2H9mzNI7ntPW\n/XCchVf+ax2yt9haZ+mQE2NPYwHDjqCtdGkP5ZXXnYhJSBzSEhxfGckIiKDyOxiN\nkLFLvUdT4ERSFBjauP2cSI0XoOUsiBxJNwHH310AU8jZbveSTcXGYgEuu2MIuDo7\nVhkq5+TCqXsIFNbjy0taOoPRvUbPsbqFlQIDAQABo4GRMIGOMAkGA1UdEwQCMAAw\nLAYJYIZIAYb4QgENBB8WHU9wZW5TU0wgR2VuZXJhdGVkIENlcnRpZmljYXRlMB0G\nA1UdDgQWBBQTW9cOMFPyPZ60/hut8dD0N4qemDAfBgNVHSMEGDAWgBS/ecqXsmB4\nIJaqRlec36eyI/UlYzATBgNVHSUEDDAKBggrBgEFBQcDATANBgkqhkiG9w0BAQsF\nAAOCAQEAYTglgIYqxhbmErQar8yFmRRJp93Zul+PnCuq1nkGPokJoytszoQtGBfw\nftgcMyTH3TOR22XThQafi/qWj3gz//oicZ09AuDfk/GMweWPjPGSs2lNUCbC9FqW\n75JpYWsKqk8s0GwetZ710rX/65wJQAb4EcibMdWq98C/HUwQspXiXBXkEMDbMF5Q\ns41vyeASk03jff+ofvTZl33sPurltO2oyRtDfUKWFAMBS7Bk/h/d3ZIwmv7DjXVw\nZKjxMZbXSmlgdngzBCBYZb5p+VkGXHqVjd07KhZd4nn5sqLy2i1COWB4OCb0xUHr\nQxHvmJiqQ57FTFDypV0sKZRLuY9ovQ==\n-----END CERTIFICATE-----\n\0",
    )
};
unsafe extern "C" fn test_bf() {
    static mut key: [uint8_t; 16] = [
        0 as ::core::ffi::c_int as uint8_t,
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
    static mut plaintext: [uint8_t; 8] = [
        0x4e as ::core::ffi::c_int as uint8_t,
        0xf9 as ::core::ffi::c_int as uint8_t,
        0x97 as ::core::ffi::c_int as uint8_t,
        0x45 as ::core::ffi::c_int as uint8_t,
        0x61 as ::core::ffi::c_int as uint8_t,
        0x98 as ::core::ffi::c_int as uint8_t,
        0xdd as ::core::ffi::c_int as uint8_t,
        0x78 as ::core::ffi::c_int as uint8_t,
    ];
    static mut expected: [uint8_t; 8] = [
        0xe1 as ::core::ffi::c_int as uint8_t,
        0xc0 as ::core::ffi::c_int as uint8_t,
        0x30 as ::core::ffi::c_int as uint8_t,
        0xe7 as ::core::ffi::c_int as uint8_t,
        0x4c as ::core::ffi::c_int as uint8_t,
        0x14 as ::core::ffi::c_int as uint8_t,
        0xd2 as ::core::ffi::c_int as uint8_t,
        0x61 as ::core::ffi::c_int as uint8_t,
    ];
    let mut encrypted: [uint8_t; 8] = [0; 8];
    let mut decrypted: [uint8_t; 8] = [0; 8];
    let mut ctx_0: *mut ptls_cipher_context_t = ptls_cipher_new(
        &raw const ptls_openssl_bfecb,
        1 as ::core::ffi::c_int,
        &raw const key as *const uint8_t as *const ::core::ffi::c_void,
    );
    ptls_cipher_encrypt(
        ctx_0,
        &raw mut encrypted as *mut uint8_t as *mut ::core::ffi::c_void,
        &raw const plaintext as *const uint8_t as *const ::core::ffi::c_void,
        PTLS_BLOWFISH_BLOCK_SIZE as size_t,
    );
    ptls_cipher_free(ctx_0);
    _ok(
        (memcmp(
            &raw mut encrypted as *mut uint8_t as *const ::core::ffi::c_void,
            &raw const expected as *const uint8_t as *const ::core::ffi::c_void,
            8 as size_t,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/openssl.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        114 as ::core::ffi::c_int,
    );
    ctx_0 = ptls_cipher_new(
        &raw const ptls_openssl_bfecb,
        0 as ::core::ffi::c_int,
        &raw const key as *const uint8_t as *const ::core::ffi::c_void,
    );
    ptls_cipher_encrypt(
        ctx_0,
        &raw mut decrypted as *mut uint8_t as *mut ::core::ffi::c_void,
        b"deadbeef\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
        PTLS_BLOWFISH_BLOCK_SIZE as size_t,
    );
    ptls_cipher_encrypt(
        ctx_0,
        &raw mut decrypted as *mut uint8_t as *mut ::core::ffi::c_void,
        &raw mut encrypted as *mut uint8_t as *const ::core::ffi::c_void,
        PTLS_BLOWFISH_BLOCK_SIZE as size_t,
    );
    ptls_cipher_free(ctx_0);
    _ok(
        (memcmp(
            &raw mut decrypted as *mut uint8_t as *const ::core::ffi::c_void,
            &raw const plaintext as *const uint8_t as *const ::core::ffi::c_void,
            8 as size_t,
        ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/openssl.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        121 as ::core::ffi::c_int,
    );
}
unsafe extern "C" fn test_key_exchanges() {
    test_key_exchange(
        &raw const ptls_openssl_secp256r1,
        &raw const ptls_openssl_secp256r1,
    );
    test_key_exchange(
        &raw const ptls_openssl_secp256r1,
        &raw const ptls_minicrypto_secp256r1,
    );
    test_key_exchange(
        &raw const ptls_minicrypto_secp256r1,
        &raw const ptls_openssl_secp256r1,
    );
    test_key_exchange(
        &raw const ptls_openssl_secp384r1,
        &raw const ptls_openssl_secp384r1,
    );
    test_key_exchange(
        &raw const ptls_openssl_secp521r1,
        &raw const ptls_openssl_secp521r1,
    );
    test_key_exchange(
        &raw const ptls_openssl_x25519,
        &raw const ptls_openssl_x25519,
    );
    test_key_exchange(
        &raw const ptls_openssl_x25519,
        &raw const ptls_minicrypto_x25519,
    );
    test_key_exchange(
        &raw const ptls_minicrypto_x25519,
        &raw const ptls_openssl_x25519,
    );
}
unsafe extern "C" fn test_sign_verify(
    mut key: *mut EVP_PKEY,
    mut schemes: *const ptls_openssl_signature_scheme_t,
) {
    let mut i: size_t = 0 as size_t;
    while (*schemes.offset(i as isize)).scheme_id as ::core::ffi::c_int != UINT16_MAX {
        note(
            b"scheme 0x%04x\0".as_ptr() as *const ::core::ffi::c_char,
            (*schemes.offset(i as isize)).scheme_id as ::core::ffi::c_int,
        );
        let mut message: *const ::core::ffi::c_void =
            b"hello world\0".as_ptr() as *const ::core::ffi::c_char as *const ::core::ffi::c_void;
        let mut sigbuf: ptls_buffer_t = st_ptls_buffer_t {
            base: ::core::ptr::null_mut::<uint8_t>(),
            capacity: 0,
            off: 0,
            is_allocated: 0,
            align_bits: 0,
        };
        let mut sigbuf_small: [uint8_t; 1024] = [0; 1024];
        ptls_buffer_init(
            &raw mut sigbuf,
            &raw mut sigbuf_small as *mut uint8_t as *mut ::core::ffi::c_void,
            ::core::mem::size_of::<[uint8_t; 1024]>() as size_t,
        );
        _ok(
            (do_sign(
                key,
                schemes.offset(i as isize),
                &raw mut sigbuf,
                ptls_iovec_init(message, strlen(message as *const ::core::ffi::c_char)),
                ::core::ptr::null_mut::<*mut ptls_async_job_t>(),
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/openssl.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            155 as ::core::ffi::c_int,
        );
        EVP_PKEY_up_ref(key);
        _ok(
            (verify_sign(
                key as *mut ::core::ffi::c_void,
                (*schemes.offset(i as isize)).scheme_id,
                ptls_iovec_init(message, strlen(message as *const ::core::ffi::c_char)),
                ptls_iovec_init(sigbuf.base as *const ::core::ffi::c_void, sigbuf.off),
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/openssl.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            158 as ::core::ffi::c_int,
        );
        ptls_buffer_dispose(&raw mut sigbuf);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn test_sha() {
    static mut text: *const ::core::ffi::c_char = b"Alice was beginning to get very tired of sitting by her sister on the bank, and of having nothing to do: once or twice she had peeped into the book her sister was reading, but it had no pictures or conversations in it, and where is the use of a book, thought Alice, without pictures or conversations?\0"
        .as_ptr() as *const ::core::ffi::c_char;
    static mut all: [C2Rust_Unnamed_20; 4] = unsafe {
        [
            C2Rust_Unnamed_20 {
                algo: &raw const ptls_openssl_sha256,
                expected: [
                    0x9b as ::core::ffi::c_int as uint8_t,
                    0x5d as ::core::ffi::c_int as uint8_t,
                    0x38 as ::core::ffi::c_int as uint8_t,
                    0x9a as ::core::ffi::c_int as uint8_t,
                    0xa5 as ::core::ffi::c_int as uint8_t,
                    0xfd as ::core::ffi::c_int as uint8_t,
                    0xc8 as ::core::ffi::c_int as uint8_t,
                    0x3a as ::core::ffi::c_int as uint8_t,
                    0xf5 as ::core::ffi::c_int as uint8_t,
                    0x59 as ::core::ffi::c_int as uint8_t,
                    0x8e as ::core::ffi::c_int as uint8_t,
                    0x90 as ::core::ffi::c_int as uint8_t,
                    0xd7 as ::core::ffi::c_int as uint8_t,
                    0x4e as ::core::ffi::c_int as uint8_t,
                    0x99 as ::core::ffi::c_int as uint8_t,
                    0xb2 as ::core::ffi::c_int as uint8_t,
                    0xbc as ::core::ffi::c_int as uint8_t,
                    0xeb as ::core::ffi::c_int as uint8_t,
                    0x97 as ::core::ffi::c_int as uint8_t,
                    0x45 as ::core::ffi::c_int as uint8_t,
                    0x7a as ::core::ffi::c_int as uint8_t,
                    0xc5 as ::core::ffi::c_int as uint8_t,
                    0xda as ::core::ffi::c_int as uint8_t,
                    0xde as ::core::ffi::c_int as uint8_t,
                    0xd5 as ::core::ffi::c_int as uint8_t,
                    0xd2 as ::core::ffi::c_int as uint8_t,
                    0x18 as ::core::ffi::c_int as uint8_t,
                    0x1c as ::core::ffi::c_int as uint8_t,
                    0x33 as ::core::ffi::c_int as uint8_t,
                    0x5c as ::core::ffi::c_int as uint8_t,
                    0x93 as ::core::ffi::c_int as uint8_t,
                    0x41 as ::core::ffi::c_int as uint8_t,
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
            },
            C2Rust_Unnamed_20 {
                algo: &raw const ptls_openssl_sha384,
                expected: [
                    0x41 as ::core::ffi::c_int as uint8_t,
                    0x7a as ::core::ffi::c_int as uint8_t,
                    0x7e as ::core::ffi::c_int as uint8_t,
                    0xda as ::core::ffi::c_int as uint8_t,
                    0x89 as ::core::ffi::c_int as uint8_t,
                    0x55 as ::core::ffi::c_int as uint8_t,
                    0xc6 as ::core::ffi::c_int as uint8_t,
                    0xb4 as ::core::ffi::c_int as uint8_t,
                    0x31 as ::core::ffi::c_int as uint8_t,
                    0xde as ::core::ffi::c_int as uint8_t,
                    0x73 as ::core::ffi::c_int as uint8_t,
                    0x2c as ::core::ffi::c_int as uint8_t,
                    0x8d as ::core::ffi::c_int as uint8_t,
                    0xc9 as ::core::ffi::c_int as uint8_t,
                    0x3b as ::core::ffi::c_int as uint8_t,
                    0xcc as ::core::ffi::c_int as uint8_t,
                    0xc7 as ::core::ffi::c_int as uint8_t,
                    0xbc as ::core::ffi::c_int as uint8_t,
                    0xe8 as ::core::ffi::c_int as uint8_t,
                    0x96 as ::core::ffi::c_int as uint8_t,
                    0x91 as ::core::ffi::c_int as uint8_t,
                    0x7a as ::core::ffi::c_int as uint8_t,
                    0xa6 as ::core::ffi::c_int as uint8_t,
                    0xa2 as ::core::ffi::c_int as uint8_t,
                    0xf8 as ::core::ffi::c_int as uint8_t,
                    0x73 as ::core::ffi::c_int as uint8_t,
                    0x7e as ::core::ffi::c_int as uint8_t,
                    0xb9 as ::core::ffi::c_int as uint8_t,
                    0xff as ::core::ffi::c_int as uint8_t,
                    0x9 as ::core::ffi::c_int as uint8_t,
                    0xc6 as ::core::ffi::c_int as uint8_t,
                    0x32 as ::core::ffi::c_int as uint8_t,
                    0x31 as ::core::ffi::c_int as uint8_t,
                    0x7b as ::core::ffi::c_int as uint8_t,
                    0xe1 as ::core::ffi::c_int as uint8_t,
                    0x5b as ::core::ffi::c_int as uint8_t,
                    0xd7 as ::core::ffi::c_int as uint8_t,
                    0xaa as ::core::ffi::c_int as uint8_t,
                    0xf2 as ::core::ffi::c_int as uint8_t,
                    0xbd as ::core::ffi::c_int as uint8_t,
                    0x2a as ::core::ffi::c_int as uint8_t,
                    0x5c as ::core::ffi::c_int as uint8_t,
                    0x3a as ::core::ffi::c_int as uint8_t,
                    0xda as ::core::ffi::c_int as uint8_t,
                    0x3b as ::core::ffi::c_int as uint8_t,
                    0x24 as ::core::ffi::c_int as uint8_t,
                    0x75 as ::core::ffi::c_int as uint8_t,
                    0x92 as ::core::ffi::c_int as uint8_t,
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
            },
            C2Rust_Unnamed_20 {
                algo: &raw const ptls_openssl_sha512,
                expected: [
                    0x40 as ::core::ffi::c_int as uint8_t,
                    0x9d as ::core::ffi::c_int as uint8_t,
                    0x7f as ::core::ffi::c_int as uint8_t,
                    0x12 as ::core::ffi::c_int as uint8_t,
                    0x8e as ::core::ffi::c_int as uint8_t,
                    0x32 as ::core::ffi::c_int as uint8_t,
                    0x96 as ::core::ffi::c_int as uint8_t,
                    0x89 as ::core::ffi::c_int as uint8_t,
                    0xdc as ::core::ffi::c_int as uint8_t,
                    0xa5 as ::core::ffi::c_int as uint8_t,
                    0x72 as ::core::ffi::c_int as uint8_t,
                    0xe4 as ::core::ffi::c_int as uint8_t,
                    0xa5 as ::core::ffi::c_int as uint8_t,
                    0x39 as ::core::ffi::c_int as uint8_t,
                    0xb4 as ::core::ffi::c_int as uint8_t,
                    0x2b as ::core::ffi::c_int as uint8_t,
                    0xf0 as ::core::ffi::c_int as uint8_t,
                    0x24 as ::core::ffi::c_int as uint8_t,
                    0xe5 as ::core::ffi::c_int as uint8_t,
                    0x42 as ::core::ffi::c_int as uint8_t,
                    0x7a as ::core::ffi::c_int as uint8_t,
                    0x61 as ::core::ffi::c_int as uint8_t,
                    0x77 as ::core::ffi::c_int as uint8_t,
                    0x69 as ::core::ffi::c_int as uint8_t,
                    0xda as ::core::ffi::c_int as uint8_t,
                    0xd5 as ::core::ffi::c_int as uint8_t,
                    0xfd as ::core::ffi::c_int as uint8_t,
                    0x72 as ::core::ffi::c_int as uint8_t,
                    0x85 as ::core::ffi::c_int as uint8_t,
                    0x83 as ::core::ffi::c_int as uint8_t,
                    0x39 as ::core::ffi::c_int as uint8_t,
                    0x1 as ::core::ffi::c_int as uint8_t,
                    0x31 as ::core::ffi::c_int as uint8_t,
                    0xa6 as ::core::ffi::c_int as uint8_t,
                    0xc8 as ::core::ffi::c_int as uint8_t,
                    0x2f as ::core::ffi::c_int as uint8_t,
                    0x6a as ::core::ffi::c_int as uint8_t,
                    0x9 as ::core::ffi::c_int as uint8_t,
                    0xfe as ::core::ffi::c_int as uint8_t,
                    0xa0 as ::core::ffi::c_int as uint8_t,
                    0x54 as ::core::ffi::c_int as uint8_t,
                    0xc as ::core::ffi::c_int as uint8_t,
                    0xe3 as ::core::ffi::c_int as uint8_t,
                    0x89 as ::core::ffi::c_int as uint8_t,
                    0xdb as ::core::ffi::c_int as uint8_t,
                    0x8c as ::core::ffi::c_int as uint8_t,
                    0x4a as ::core::ffi::c_int as uint8_t,
                    0x83 as ::core::ffi::c_int as uint8_t,
                    0x2f as ::core::ffi::c_int as uint8_t,
                    0x90 as ::core::ffi::c_int as uint8_t,
                    0x94 as ::core::ffi::c_int as uint8_t,
                    0x54 as ::core::ffi::c_int as uint8_t,
                    0x93 as ::core::ffi::c_int as uint8_t,
                    0x3f as ::core::ffi::c_int as uint8_t,
                    0xe9 as ::core::ffi::c_int as uint8_t,
                    0x8a as ::core::ffi::c_int as uint8_t,
                    0x32 as ::core::ffi::c_int as uint8_t,
                    0x3f as ::core::ffi::c_int as uint8_t,
                    0x85 as ::core::ffi::c_int as uint8_t,
                    0x24 as ::core::ffi::c_int as uint8_t,
                    0xa5 as ::core::ffi::c_int as uint8_t,
                    0x9b as ::core::ffi::c_int as uint8_t,
                    0x5b as ::core::ffi::c_int as uint8_t,
                    0x2 as ::core::ffi::c_int as uint8_t,
                ],
            },
            C2Rust_Unnamed_20 {
                algo: ::core::ptr::null::<ptls_hash_algorithm_t>(),
                expected: [0; 64],
            },
        ]
    };
    let mut i: size_t = 0 as size_t;
    while !all[i as usize].algo.is_null() {
        let mut actual: [uint8_t; 64] = [0; 64];
        note(
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            (*all[i as usize].algo).name,
        );
        let mut ret: ::core::ffi::c_int = ptls_calc_hash(
            all[i as usize].algo,
            &raw mut actual as *mut uint8_t as *mut ::core::ffi::c_void,
            text as *const ::core::ffi::c_void,
            strlen(text),
        );
        _ok(
            (ret == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/openssl.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            190 as ::core::ffi::c_int,
        );
        _ok(
            (memcmp(
                &raw mut actual as *mut uint8_t as *const ::core::ffi::c_void,
                &raw const (*(&raw const all as *const C2Rust_Unnamed_20)
                    .offset(i as isize))
                    .expected as *const uint8_t as *const ::core::ffi::c_void,
                (*all[i as usize].algo).digest_size,
            ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
            b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
            b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/openssl.c\0"
                .as_ptr() as *const ::core::ffi::c_char,
            191 as ::core::ffi::c_int,
        );
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn test_rsa_sign() {
    let mut sc: *mut ptls_openssl_sign_certificate_t =
        (*ctx).sign_certificate as *mut ptls_openssl_sign_certificate_t;
    test_sign_verify((*sc).key, (*sc).schemes);
}
unsafe extern "C" fn do_test_ecdsa_sign(
    mut nid: ::core::ffi::c_int,
    mut schemes: *const ptls_openssl_signature_scheme_t,
) {
    let mut pkey: *mut EVP_PKEY = ::core::ptr::null_mut::<EVP_PKEY>();
    let mut eckey: *mut EC_KEY = EC_KEY_new_by_curve_name(nid);
    EC_KEY_generate_key(eckey);
    pkey = EVP_PKEY_new();
    EVP_PKEY_set1_EC_KEY(pkey, eckey as *mut ec_key_st);
    EC_KEY_free(eckey);
    test_sign_verify(pkey, schemes);
    EVP_PKEY_free(pkey);
}
unsafe extern "C" fn test_ecdsa_sign() {
    do_test_ecdsa_sign(
        NID_X9_62_prime256v1,
        &raw const secp256r1_signature_schemes as *const ptls_openssl_signature_scheme_t,
    );
    do_test_ecdsa_sign(
        NID_secp384r1,
        &raw const secp384r1_signature_schemes as *const ptls_openssl_signature_scheme_t,
    );
    do_test_ecdsa_sign(
        NID_secp521r1,
        &raw const secp521r1_signature_schemes as *const ptls_openssl_signature_scheme_t,
    );
}
unsafe extern "C" fn test_ed25519_sign() {
    let mut pkey: *mut EVP_PKEY = ::core::ptr::null_mut::<EVP_PKEY>();
    let mut pctx: *mut EVP_PKEY_CTX =
        EVP_PKEY_CTX_new_id(EVP_PKEY_ED25519, ::core::ptr::null_mut::<ENGINE>());
    EVP_PKEY_keygen_init(pctx);
    EVP_PKEY_keygen(pctx, &raw mut pkey);
    EVP_PKEY_CTX_free(pctx);
    test_sign_verify(
        pkey,
        &raw const ed25519_signature_schemes as *const ptls_openssl_signature_scheme_t,
    );
    EVP_PKEY_free(pkey);
}
unsafe extern "C" fn x509_from_pem(mut pem: *const ::core::ffi::c_char) -> *mut X509 {
    let mut bio: *mut BIO = BIO_new_mem_buf(
        pem as *mut ::core::ffi::c_void,
        strlen(pem) as ::core::ffi::c_int,
    );
    let mut cert: *mut X509 =
        PEM_read_bio_X509(bio, ::core::ptr::null_mut::<*mut X509>(), None, NULL_0);
    BIO_free(bio);
    return cert;
}
unsafe extern "C" fn key_from_pem(
    mut pem: *const ::core::ffi::c_char,
) -> *mut ptls_key_exchange_context_t {
    let mut bio: *mut BIO = BIO_new_mem_buf(
        pem as *mut ::core::ffi::c_void,
        strlen(pem) as ::core::ffi::c_int,
    );
    let mut pkey: *mut EVP_PKEY =
        PEM_read_bio_PrivateKey(bio, ::core::ptr::null_mut::<*mut EVP_PKEY>(), None, NULL_0);
    BIO_free(bio);
    let mut ctx_0: *mut ptls_key_exchange_context_t =
        ::core::ptr::null_mut::<ptls_key_exchange_context_t>();
    let mut ret: ::core::ffi::c_int = ptls_openssl_create_key_exchange(&raw mut ctx_0, pkey);
    EVP_PKEY_free(pkey);
    return ctx_0;
}
unsafe extern "C" fn test_cert_verify() {
    let mut cert: *mut X509 = x509_from_pem(RSA_CERTIFICATE.as_ptr());
    let mut chain: *mut stack_st_X509 = OPENSSL_sk_new_null() as *mut stack_st_X509;
    let mut store: *mut X509_STORE = X509_STORE_new();
    let mut ret: ::core::ffi::c_int = 0;
    let mut ossl_x509_err: ::core::ffi::c_int = 0;
    ret = verify_cert_chain(
        store,
        cert,
        chain,
        0 as ::core::ffi::c_int,
        b"test.example.com\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut ossl_x509_err,
    );
    _ok(
        (ret == 48 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/openssl.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        278 as ::core::ffi::c_int,
    );
    let mut lookup: *mut X509_LOOKUP = X509_STORE_add_lookup(store, X509_LOOKUP_file());
    ret = X509_LOOKUP_ctrl(
        lookup,
        X509_L_FILE_LOAD,
        b"t/assets/test-ca.crt\0".as_ptr() as *const ::core::ffi::c_char,
        1 as ::core::ffi::c_int as ::core::ffi::c_long,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    _ok(
        ret,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/openssl.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        283 as ::core::ffi::c_int,
    );
    ret = verify_cert_chain(
        store,
        cert,
        chain,
        0 as ::core::ffi::c_int,
        b"test.example.com\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut ossl_x509_err,
    );
    _ok(
        (ret == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/openssl.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        285 as ::core::ffi::c_int,
    );
    ret = verify_cert_chain(
        store,
        cert,
        chain,
        0 as ::core::ffi::c_int,
        b"test2.example.com\0".as_ptr() as *const ::core::ffi::c_char,
        &raw mut ossl_x509_err,
    );
    _ok(
        (ret == 42 as ::core::ffi::c_int) as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/openssl.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        290 as ::core::ffi::c_int,
    );
    X509_free(cert);
    OPENSSL_sk_free(ossl_check_X509_sk_type(chain));
    X509_STORE_free(store);
}
unsafe extern "C" fn setup_certificate(mut dst: *mut ptls_iovec_t) {
    let mut cert: *mut X509 = x509_from_pem(RSA_CERTIFICATE.as_ptr());
    (*dst).base = ::core::ptr::null_mut::<uint8_t>();
    (*dst).len = i2d_X509(cert, &raw mut (*dst).base) as size_t;
    X509_free(cert);
}
unsafe extern "C" fn setup_sign_certificate(mut sc: *mut ptls_openssl_sign_certificate_t) {
    let mut bio: *mut BIO = BIO_new_mem_buf(
        RSA_PRIVATE_KEY.as_ptr() as *const ::core::ffi::c_void,
        strlen(RSA_PRIVATE_KEY.as_ptr()) as ::core::ffi::c_int,
    );
    let mut pkey: *mut EVP_PKEY =
        PEM_read_bio_PrivateKey(bio, ::core::ptr::null_mut::<*mut EVP_PKEY>(), None, NULL_0);
    BIO_free(bio);
    ptls_openssl_init_sign_certificate(sc, pkey);
    EVP_PKEY_free(pkey);
}
unsafe extern "C" fn verify_cert_cb(
    mut ok: ::core::ffi::c_int,
    mut ctx_0: *mut X509_STORE_CTX,
) -> ::core::ffi::c_int {
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn ptls_ffx_ptls_openssl_aes128ctr_b31_r6_setup(
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return ptls_ffx_setup_crypto(
        ctx_0,
        &raw const ptls_openssl_aes128ctr,
        is_enc,
        6 as ::core::ffi::c_int,
        31 as size_t,
        key,
    );
}
static mut ptls_ffx_ptls_openssl_aes128ctr_b31_r6: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"ptls_openssl_aes128ctr-ffx-b31-r6\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: 16 as size_t,
        block_size: ((31 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int)
            as size_t,
        iv_size: 16 as size_t,
        context_size: ::core::mem::size_of::<ptls_ffx_context_t>() as size_t,
        setup_crypto: Some(
            ptls_ffx_ptls_openssl_aes128ctr_b31_r6_setup
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
unsafe extern "C" fn ptls_ffx_ptls_openssl_aes128ctr_b53_r4_setup(
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return ptls_ffx_setup_crypto(
        ctx_0,
        &raw const ptls_openssl_aes128ctr,
        is_enc,
        4 as ::core::ffi::c_int,
        53 as size_t,
        key,
    );
}
static mut ptls_ffx_ptls_openssl_aes128ctr_b53_r4: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"ptls_openssl_aes128ctr-ffx-b53-r4\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: 16 as size_t,
        block_size: ((53 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int)
            as size_t,
        iv_size: 16 as size_t,
        context_size: ::core::mem::size_of::<ptls_ffx_context_t>() as size_t,
        setup_crypto: Some(
            ptls_ffx_ptls_openssl_aes128ctr_b53_r4_setup
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
static mut ptls_ffx_ptls_openssl_aes128ctr_b125_r8: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"ptls_openssl_aes128ctr-ffx-b125-r8\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: 16 as size_t,
        block_size: ((125 as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
            / 8 as ::core::ffi::c_int) as size_t,
        iv_size: 16 as size_t,
        context_size: ::core::mem::size_of::<ptls_ffx_context_t>() as size_t,
        setup_crypto: Some(
            ptls_ffx_ptls_openssl_aes128ctr_b125_r8_setup
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
unsafe extern "C" fn ptls_ffx_ptls_openssl_aes128ctr_b125_r8_setup(
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return ptls_ffx_setup_crypto(
        ctx_0,
        &raw const ptls_openssl_aes128ctr,
        is_enc,
        8 as ::core::ffi::c_int,
        125 as size_t,
        key,
    );
}
unsafe extern "C" fn ptls_ffx_ptls_openssl_chacha20_b256_r8_setup(
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return ptls_ffx_setup_crypto(
        ctx_0,
        &raw const ptls_openssl_chacha20,
        is_enc,
        8 as ::core::ffi::c_int,
        256 as size_t,
        key,
    );
}
static mut ptls_ffx_ptls_openssl_chacha20_b256_r8: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"ptls_openssl_chacha20-ffx-b256-r8\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: 32 as size_t,
        block_size: ((256 as ::core::ffi::c_int + 7 as ::core::ffi::c_int)
            / 8 as ::core::ffi::c_int) as size_t,
        iv_size: 16 as size_t,
        context_size: ::core::mem::size_of::<ptls_ffx_context_t>() as size_t,
        setup_crypto: Some(
            ptls_ffx_ptls_openssl_chacha20_b256_r8_setup
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
unsafe extern "C" fn ptls_ffx_ptls_openssl_chacha20_b32_r6_setup(
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return ptls_ffx_setup_crypto(
        ctx_0,
        &raw const ptls_openssl_chacha20,
        is_enc,
        6 as ::core::ffi::c_int,
        32 as size_t,
        key,
    );
}
unsafe extern "C" fn ptls_ffx_ptls_openssl_chacha20_b57_r4_setup(
    mut ctx_0: *mut ptls_cipher_context_t,
    mut is_enc: ::core::ffi::c_int,
    mut key: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    return ptls_ffx_setup_crypto(
        ctx_0,
        &raw const ptls_openssl_chacha20,
        is_enc,
        4 as ::core::ffi::c_int,
        57 as size_t,
        key,
    );
}
static mut ptls_ffx_ptls_openssl_chacha20_b32_r6: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"ptls_openssl_chacha20-ffx-b32-r6\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: 32 as size_t,
        block_size: ((32 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int)
            as size_t,
        iv_size: 16 as size_t,
        context_size: ::core::mem::size_of::<ptls_ffx_context_t>() as size_t,
        setup_crypto: Some(
            ptls_ffx_ptls_openssl_chacha20_b32_r6_setup
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
static mut ptls_ffx_ptls_openssl_chacha20_b57_r4: ptls_cipher_algorithm_t = unsafe {
    st_ptls_cipher_algorithm_t {
        name: b"ptls_openssl_chacha20-ffx-b57-r4\0".as_ptr() as *const ::core::ffi::c_char,
        key_size: 32 as size_t,
        block_size: ((57 as ::core::ffi::c_int + 7 as ::core::ffi::c_int) / 8 as ::core::ffi::c_int)
            as size_t,
        iv_size: 16 as size_t,
        context_size: ::core::mem::size_of::<ptls_ffx_context_t>() as size_t,
        setup_crypto: Some(
            ptls_ffx_ptls_openssl_chacha20_b57_r4_setup
                as unsafe extern "C" fn(
                    *mut ptls_cipher_context_t,
                    ::core::ffi::c_int,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    }
};
unsafe extern "C" fn test_all_hpke() {
    test_hpke(
        &raw mut ptls_openssl_hpke_kems as *mut *const ptls_hpke_kem_t,
        &raw mut ptls_openssl_hpke_cipher_suites as *mut *const ptls_hpke_cipher_suite_t,
    );
}
unsafe extern "C" fn create_ech_opener(
    mut self_0: *mut ptls_ech_create_opener_t,
    mut kem: *mut *const ptls_hpke_kem_t,
    mut cipher: *mut *const ptls_hpke_cipher_suite_t,
    mut tls: *mut ptls_t,
    mut config_id: uint8_t,
    mut cipher_id: ptls_hpke_cipher_suite_id_t,
    mut enc: ptls_iovec_t,
    mut info_prefix: ptls_iovec_t,
) -> *mut ptls_aead_context_t {
    static mut pem: *mut ptls_key_exchange_context_t =
        ::core::ptr::null::<ptls_key_exchange_context_t>() as *mut ptls_key_exchange_context_t;
    if pem.is_null() {
        pem = key_from_pem(ECH_PRIVATE_KEY.as_ptr());
    }
    *cipher = ::core::ptr::null::<ptls_hpke_cipher_suite_t>();
    let mut i: size_t = 0 as size_t;
    while !ptls_openssl_hpke_cipher_suites[i as usize].is_null() {
        if (*ptls_openssl_hpke_cipher_suites[i as usize]).id.kdf as ::core::ffi::c_int
            == cipher_id.kdf as ::core::ffi::c_int
            && (*ptls_openssl_hpke_cipher_suites[i as usize]).id.aead as ::core::ffi::c_int
                == cipher_id.aead as ::core::ffi::c_int
        {
            *cipher = ptls_openssl_hpke_cipher_suites[i as usize];
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if (*cipher).is_null() {
        return ::core::ptr::null_mut::<ptls_aead_context_t>();
    }
    let mut aead: *mut ptls_aead_context_t = ::core::ptr::null_mut::<ptls_aead_context_t>();
    let mut infobuf: ptls_buffer_t = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0,
        off: 0,
        is_allocated: 0,
        align_bits: 0,
    };
    let mut ret: ::core::ffi::c_int = 0;
    ptls_buffer_init(
        &raw mut infobuf,
        b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as size_t,
    );
    ret = ptls_buffer__do_pushv(
        &raw mut infobuf,
        info_prefix.base as *const ::core::ffi::c_void,
        info_prefix.len,
    );
    if !(ret != 0 as ::core::ffi::c_int) {
        ret = ptls_buffer__do_pushv(
            &raw mut infobuf,
            (b"\0c\xFE\r\0_\x12\0\x10\0A\x04\xFE\x8C\x19\xCE\t\x05\x19\x1E\xBC)\x8A\x92Ey%1\xF2o\x0C\xEC\xE2F\x069\xE8\xBC9\xCB\x7Fpj\x82jw\x9BL\xF9i\xB8\xA0\xE59\xC7\xF6/\xB3\xD3\n\xD6\xAA\x8F\x80\xE3\x0F\x1D\x12\x8A\xAF\xD6\x8A,\xE7.\xA0\0\x08\0\x02\0\x02\0\x01\0\x01@\x0Bexample.com\0\0\0"
                .as_ptr() as *const ::core::ffi::c_char as *const uint8_t)
                .offset(2 as ::core::ffi::c_int as isize) as *const ::core::ffi::c_void,
            (::core::mem::size_of::<[::core::ffi::c_char; 102]>() as size_t)
                .wrapping_sub(3 as size_t),
        );
        if !(ret != 0 as ::core::ffi::c_int) {
            ret = ptls_hpke_setup_base_r(
                &raw const ptls_openssl_hpke_kem_p256sha256,
                *cipher,
                pem,
                &raw mut aead,
                enc,
                ptls_iovec_init(infobuf.base as *const ::core::ffi::c_void, infobuf.off),
            );
        }
    }
    ptls_buffer_dispose(&raw mut infobuf);
    return aead;
}
unsafe extern "C" fn load_engine(mut name: *const ::core::ffi::c_char) -> *mut ENGINE {
    let mut e: *mut ENGINE = ::core::ptr::null_mut::<ENGINE>();
    e = ENGINE_by_id(b"dynamic\0".as_ptr() as *const ::core::ffi::c_char);
    if e.is_null() {
        return ::core::ptr::null_mut::<ENGINE>();
    }
    if ENGINE_ctrl_cmd_string(
        e,
        b"SO_PATH\0".as_ptr() as *const ::core::ffi::c_char,
        name,
        0 as ::core::ffi::c_int,
    ) == 0
        || ENGINE_ctrl_cmd_string(
            e,
            b"LOAD\0".as_ptr() as *const ::core::ffi::c_char,
            ::core::ptr::null::<::core::ffi::c_char>(),
            0 as ::core::ffi::c_int,
        ) == 0
    {
        ENGINE_free(e);
        return ::core::ptr::null_mut::<ENGINE>();
    }
    return e;
}
static mut qat: C2Rust_Unnamed_21 = C2Rust_Unnamed_21 {
    conns: [C2Rust_Unnamed_22 {
        next_pending: 0,
        tls: ::core::ptr::null::<ptls_t>() as *mut ptls_t,
        wait_fd: 0,
    }; 10],
    first_pending: 0,
};
unsafe extern "C" fn qat_set_pending(mut index: size_t) {
    qat.conns[index as usize].next_pending = qat.first_pending;
    qat.first_pending = index;
}
unsafe extern "C" fn many_handshakes() {
    let mut c2rust_current_block: u64;
    let mut client: *mut ptls_t = ptls_new(ctx, 0 as ::core::ffi::c_int);
    let mut resp_sample_conn: *mut ptls_t = ::core::ptr::null_mut::<ptls_t>();
    let mut clientbuf: ptls_buffer_t = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0,
        off: 0,
        is_allocated: 0,
        align_bits: 0,
    };
    let mut resp_sample: ptls_buffer_t = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0,
        off: 0,
        is_allocated: 0,
        align_bits: 0,
    };
    let mut ret: ::core::ffi::c_int = 0;
    ptls_buffer_init(
        &raw mut clientbuf,
        b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as size_t,
    );
    ret = ptls_handshake(
        client,
        &raw mut clientbuf,
        ::core::ptr::null::<::core::ffi::c_void>(),
        ::core::ptr::null_mut::<size_t>(),
        ::core::ptr::null_mut::<ptls_handshake_properties_t>(),
    );
    _ok(
        (ret == 0x200 as ::core::ffi::c_int + 2 as ::core::ffi::c_int)
            as ::core::ffi::c_int,
        b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
        b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/openssl.c\0"
            .as_ptr() as *const ::core::ffi::c_char,
        415 as ::core::ffi::c_int,
    );
    ptls_buffer_init(
        &raw mut resp_sample,
        b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as size_t,
    );
    qat.first_pending = 0 as size_t;
    let mut i: size_t = 0 as size_t;
    while i
        < ((::core::mem::size_of::<[::core::ffi::c_char; 1]>() as usize != 0 as usize)
            as ::core::ffi::c_int as usize)
            .wrapping_mul(::core::mem::size_of::<[C2Rust_Unnamed_22; 10]>() as usize)
            .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_22>() as usize)
    {
        qat.conns[i as usize].next_pending = i.wrapping_add(1 as size_t);
        qat.conns[i as usize].tls = ::core::ptr::null_mut::<ptls_t>();
        qat.conns[i as usize].wait_fd = -(1 as ::core::ffi::c_int);
        i = i.wrapping_add(1);
    }
    qat.conns[((::core::mem::size_of::<[::core::ffi::c_char; 1]>() as usize != 0 as usize)
        as ::core::ffi::c_int as usize)
        .wrapping_mul(::core::mem::size_of::<[C2Rust_Unnamed_22; 10]>() as usize)
        .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_22>() as usize)
        .wrapping_sub(1 as usize) as usize]
        .next_pending = SIZE_MAX as size_t;
    let mut start: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut end: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    gettimeofday(&raw mut start, NULL_0);
    static mut num_total: size_t = 10000 as size_t;
    let mut num_issued: size_t = 0 as size_t;
    let mut num_running: size_t = 0 as size_t;
    loop {
        if qat.first_pending != SIZE_MAX as size_t {
            let mut offending: size_t = qat.first_pending;
            qat.first_pending = qat.conns[offending as usize].next_pending;
            qat.conns[offending as usize].next_pending = SIZE_MAX as size_t;
            if qat.conns[offending as usize].tls.is_null() {
                qat.conns[offending as usize].tls = ptls_new(ctx_peer, 1 as ::core::ffi::c_int);
                if resp_sample_conn.is_null() {
                    resp_sample_conn = qat.conns[offending as usize].tls;
                }
                num_issued = num_issued.wrapping_add(1);
                num_running = num_running.wrapping_add(1);
            }
            let mut hsbuf: ptls_buffer_t = st_ptls_buffer_t {
                base: ::core::ptr::null_mut::<uint8_t>(),
                capacity: 0,
                off: 0,
                is_allocated: 0,
                align_bits: 0,
            };
            let mut hsbuf_small: [uint8_t; 8192] = [0; 8192];
            ptls_buffer_init(
                &raw mut hsbuf,
                &raw mut hsbuf_small as *mut uint8_t as *mut ::core::ffi::c_void,
                ::core::mem::size_of::<[uint8_t; 8192]>() as size_t,
            );
            let mut inlen: size_t = if ptls_get_cipher(qat.conns[offending as usize].tls).is_null()
            {
                clientbuf.off
            } else {
                0 as size_t
            };
            let mut hsret: ::core::ffi::c_int = ptls_handshake(
                qat.conns[offending as usize].tls,
                &raw mut hsbuf,
                clientbuf.base as *const ::core::ffi::c_void,
                &raw mut inlen,
                ::core::ptr::null_mut::<ptls_handshake_properties_t>(),
            );
            if resp_sample_conn == qat.conns[offending as usize].tls {
                ret = ptls_buffer__do_pushv(
                    &raw mut resp_sample,
                    hsbuf.base as *const ::core::ffi::c_void,
                    hsbuf.off,
                );
                if ret != 0 as ::core::ffi::c_int {
                    c2rust_current_block = 7189308829251266000;
                    break;
                }
            }
            ptls_buffer_dispose(&raw mut hsbuf);
            match hsret {
                0 => {
                    if qat.conns[offending as usize].tls == resp_sample_conn {
                        resp_sample_conn =
                            1 as ::core::ffi::c_int as *mut ::core::ffi::c_void as *mut ptls_t;
                    }
                    ptls_free(qat.conns[offending as usize].tls);
                    qat.conns[offending as usize].tls = ::core::ptr::null_mut::<ptls_t>();
                    num_running = num_running.wrapping_sub(1);
                    if num_issued < num_total {
                        qat_set_pending(offending);
                    }
                }
                PTLS_ERROR_ASYNC_OPERATION => {
                    let mut job: *mut ptls_async_job_t =
                        ptls_get_async_job(qat.conns[offending as usize].tls);
                    qat.conns[offending as usize].wait_fd =
                        (*job).get_fd.expect("non-null function pointer")(
                            job as *mut st_ptls_async_job_t,
                        );
                }
                _ => {
                    fprintf(
                        stderr,
                        b"ptls_handshake returned %d\n\0".as_ptr() as *const ::core::ffi::c_char,
                        hsret,
                    );
                    abort();
                }
            }
        } else {
            if num_running == 0 as size_t {
                c2rust_current_block = 5807581744382915773;
                break;
            }
            let mut rfds: fd_set = fd_set { fds_bits: [0; 16] };
            let mut __i: ::core::ffi::c_uint = 0;
            let mut __arr: *mut fd_set = &raw mut rfds;
            __i = 0 as ::core::ffi::c_uint;
            while (__i as usize)
                < (::core::mem::size_of::<fd_set>() as usize)
                    .wrapping_div(::core::mem::size_of::<__fd_mask>() as usize)
            {
                (*__arr).fds_bits[__i as usize] = 0 as __fd_mask;
                __i = __i.wrapping_add(1);
            }
            let mut nfds: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
            let mut i_0: size_t = 0 as size_t;
            while i_0
                < ((::core::mem::size_of::<[::core::ffi::c_char; 1]>() as usize != 0 as usize)
                    as ::core::ffi::c_int as usize)
                    .wrapping_mul(::core::mem::size_of::<[C2Rust_Unnamed_22; 10]>() as usize)
                    .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_22>() as usize)
            {
                if qat.conns[i_0 as usize].wait_fd != -(1 as ::core::ffi::c_int) {
                    rfds.fds_bits[(qat.conns[i_0 as usize].wait_fd / __NFDBITS) as usize] |= ((1
                        as ::core::ffi::c_ulong)
                        << qat.conns[i_0 as usize].wait_fd % __NFDBITS)
                        as __fd_mask;
                    if nfds <= qat.conns[i_0 as usize].wait_fd {
                        nfds = qat.conns[i_0 as usize].wait_fd + 1 as ::core::ffi::c_int;
                    }
                }
                i_0 = i_0.wrapping_add(1);
            }
            if select(
                nfds,
                &raw mut rfds,
                ::core::ptr::null_mut::<fd_set>(),
                ::core::ptr::null_mut::<fd_set>(),
                ::core::ptr::null_mut::<timeval>(),
            ) > 0 as ::core::ffi::c_int
            {
                let mut i_1: size_t = 0 as size_t;
                while i_1
                    < ((::core::mem::size_of::<[::core::ffi::c_char; 1]>() as usize != 0 as usize)
                        as ::core::ffi::c_int as usize)
                        .wrapping_mul(::core::mem::size_of::<[C2Rust_Unnamed_22; 10]>() as usize)
                        .wrapping_div(::core::mem::size_of::<C2Rust_Unnamed_22>() as usize)
                {
                    if qat.conns[i_1 as usize].wait_fd != -(1 as ::core::ffi::c_int)
                        && rfds.fds_bits[(qat.conns[i_1 as usize].wait_fd / __NFDBITS) as usize]
                            & ((1 as ::core::ffi::c_ulong)
                                << qat.conns[i_1 as usize].wait_fd % __NFDBITS)
                                as __fd_mask
                            != 0 as __fd_mask
                    {
                        qat.conns[i_1 as usize].wait_fd = -(1 as ::core::ffi::c_int);
                        qat_set_pending(i_1);
                    }
                    i_1 = i_1.wrapping_add(1);
                }
            }
        }
    }
    match c2rust_current_block {
        7189308829251266000 => {
            return;
        }
        _ => {
            gettimeofday(&raw mut end, NULL_0);
            note(
                b"run %zu handshakes in %f seconds\0".as_ptr() as *const ::core::ffi::c_char,
                num_total,
                end.tv_sec as ::core::ffi::c_double
                    + end.tv_usec as ::core::ffi::c_double / 1000000.0f64
                    - (start.tv_sec as ::core::ffi::c_double
                        + start.tv_usec as ::core::ffi::c_double / 1000000.0f64),
            );
            clientbuf.off = 0 as size_t;
            let mut resplen: size_t = resp_sample.off;
            _ok(
                (ptls_handshake(
                    client,
                    &raw mut clientbuf,
                    resp_sample.base as *const ::core::ffi::c_void,
                    &raw mut resplen,
                    ::core::ptr::null_mut::<ptls_handshake_properties_t>(),
                ) == 0 as ::core::ffi::c_int) as ::core::ffi::c_int,
                b"%s %d\0".as_ptr() as *const ::core::ffi::c_char,
                b"/tmp/trajectory-slipstream-upstream/build/subprojects/picoquic/__CMake_build/_deps/picotls-src/t/openssl.c\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                511 as ::core::ffi::c_int,
            );
            ptls_buffer_dispose(&raw mut clientbuf);
            ptls_buffer_dispose(&raw mut resp_sample);
            ptls_free(client);
            return;
        }
    };
}
unsafe fn main_0(
    mut argc: ::core::ffi::c_int,
    mut argv: *mut *mut ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut openssl_sign_certificate: ptls_openssl_sign_certificate_t =
        st_ptls_openssl_sign_certificate_t {
            super_0: st_ptls_sign_certificate_t { cb: None },
            key: ::core::ptr::null_mut::<EVP_PKEY>(),
            schemes: ::core::ptr::null::<ptls_openssl_signature_scheme_t>(),
            async_0: [0; 1],
            c2rust_padding: [0; 7],
        };
    let mut openssl_verify_certificate: ptls_openssl_verify_certificate_t =
        st_ptls_openssl_verify_certificate_t {
            super_0: st_ptls_verify_certificate_t {
                cb: None,
                algos: ::core::ptr::null::<uint16_t>(),
            },
            cert_store: ::core::ptr::null_mut::<X509_STORE>(),
            override_callback: ::core::ptr::null_mut::<ptls_openssl_override_verify_certificate_t>(
            ),
        };
    let mut ech_create_opener: ptls_ech_create_opener_t = st_ptls_ech_create_opener_t {
        cb: Some(
            create_ech_opener
                as unsafe extern "C" fn(
                    *mut ptls_ech_create_opener_t,
                    *mut *const ptls_hpke_kem_t,
                    *mut *const ptls_hpke_cipher_suite_t,
                    *mut ptls_t,
                    uint8_t,
                    ptls_hpke_cipher_suite_id_t,
                    ptls_iovec_t,
                    ptls_iovec_t,
                ) -> *mut ptls_aead_context_t,
        ),
    };
    OPENSSL_init_crypto(
        OPENSSL_INIT_LOAD_CRYPTO_STRINGS as uint64_t,
        ::core::ptr::null::<OPENSSL_INIT_SETTINGS>(),
    );
    OPENSSL_init_crypto(
        (OPENSSL_INIT_ADD_ALL_CIPHERS | OPENSSL_INIT_ADD_ALL_DIGESTS) as uint64_t,
        ::core::ptr::null::<OPENSSL_INIT_SETTINGS>(),
    );
    let mut legacy: *mut OSSL_PROVIDER = OSSL_PROVIDER_load(
        ::core::ptr::null_mut::<OSSL_LIB_CTX>(),
        b"legacy\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut dflt: *mut OSSL_PROVIDER = OSSL_PROVIDER_load(
        ::core::ptr::null_mut::<OSSL_LIB_CTX>(),
        b"default\0".as_ptr() as *const ::core::ffi::c_char,
    );
    let mut _name: *const ::core::ffi::c_char = b"bf\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name);
    test_bf();
    exit_subtest(_name);
    let mut _name_0: *const ::core::ffi::c_char =
        b"key-exchange\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_0);
    test_key_exchanges();
    exit_subtest(_name_0);
    let mut cert: ptls_iovec_t = st_ptls_iovec_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        len: 0,
    };
    setup_certificate(&raw mut cert);
    setup_sign_certificate(&raw mut openssl_sign_certificate);
    let mut cert_store: *mut X509_STORE = X509_STORE_new();
    X509_STORE_set_verify_cb(
        cert_store,
        Some(
            verify_cert_cb
                as unsafe extern "C" fn(
                    ::core::ffi::c_int,
                    *mut X509_STORE_CTX,
                ) -> ::core::ffi::c_int,
        ),
    );
    ptls_openssl_init_verify_certificate(&raw mut openssl_verify_certificate, cert_store);
    let mut openssl_ctx: ptls_context_t = {
        let mut init = st_ptls_context_t {
            require_dhe_on_psk_use_exporter_send_change_cipher_spec_require_client_authentication_omit_end_of_early_data_use_raw_public_keys_server_cipher_preference_server_cipher_chacha_priority: [0; 1],
            c2rust_padding: [0; 7],
            random_bytes: Some(
                ptls_openssl_random_bytes
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> (),
            ),
            get_time: &raw mut ptls_get_time,
            key_exchanges: &raw mut ptls_openssl_key_exchanges
                as *mut *const ptls_key_exchange_algorithm_t,
            cipher_suites: &raw mut ptls_openssl_cipher_suites_all
                as *mut *const ptls_cipher_suite_t,
            certificates: C2Rust_Unnamed_12 {
                list: &raw mut cert,
                count: 1 as size_t,
            },
            pre_shared_key: C2Rust_Unnamed_11 {
                identity: st_ptls_iovec_t {
                    base: ::core::ptr::null_mut::<uint8_t>(),
                    len: 0,
                },
                secret: st_ptls_iovec_t {
                    base: ::core::ptr::null_mut::<uint8_t>(),
                    len: 0,
                },
                hash: ::core::ptr::null::<ptls_hash_algorithm_t>(),
            },
            ech: C2Rust_Unnamed_8 {
                client: C2Rust_Unnamed_10 {
                    ciphers: &raw mut ptls_openssl_hpke_cipher_suites
                        as *mut *const ptls_hpke_cipher_suite_t,
                    kems: &raw mut ptls_openssl_hpke_kems as *mut *const ptls_hpke_kem_t,
                },
                server: C2Rust_Unnamed_9 {
                    create_opener: &raw mut ech_create_opener,
                    retry_configs: st_ptls_iovec_t {
                        base: ECH_CONFIG_LIST.as_ptr() as *mut uint8_t,
                        len: (::core::mem::size_of::<[::core::ffi::c_char; 102]>()
                            as size_t)
                            .wrapping_sub(1 as size_t),
                    },
                },
            },
            on_client_hello: ::core::ptr::null_mut::<ptls_on_client_hello_t>(),
            emit_certificate: ::core::ptr::null_mut::<ptls_emit_certificate_t>(),
            sign_certificate: &raw mut openssl_sign_certificate.super_0,
            verify_certificate: ::core::ptr::null_mut::<ptls_verify_certificate_t>(),
            ticket_lifetime: 0,
            max_early_data_size: 0,
            max_buffer_size: 0,
            hkdf_label_prefix__obsolete: ::core::ptr::null::<::core::ffi::c_char>(),
            encrypt_ticket: ::core::ptr::null_mut::<ptls_encrypt_ticket_t>(),
            save_ticket: ::core::ptr::null_mut::<ptls_save_ticket_t>(),
            log_event: ::core::ptr::null_mut::<ptls_log_event_t>(),
            update_open_count: ::core::ptr::null_mut::<ptls_update_open_count_t>(),
            update_traffic_key: ::core::ptr::null_mut::<ptls_update_traffic_key_t>(),
            decompress_certificate: ::core::ptr::null_mut::<
                ptls_decompress_certificate_t,
            >(),
            on_extension: ::core::ptr::null_mut::<ptls_on_extension_t>(),
            tls12_cipher_suites: &raw mut ptls_openssl_tls12_cipher_suites
                as *mut *const ptls_cipher_suite_t,
            ticket_context: C2Rust_Unnamed_1 {
                bytes: [0; 32],
                is_set: [0; 1],
                c2rust_padding: [0; 3],
            },
            client_ca_names: C2Rust_Unnamed_0 {
                list: ::core::ptr::null::<ptls_iovec_t>(),
                count: 0,
            },
        };
        init.set_require_dhe_on_psk(0);
        init.set_use_exporter(0);
        init.set_send_change_cipher_spec(0);
        init.set_require_client_authentication(0);
        init.set_omit_end_of_early_data(0);
        init.set_use_raw_public_keys(0);
        init.set_server_cipher_preference(0);
        init.set_server_cipher_chacha_priority(0);
        init
    };
    let mut openssl_ctx_sha256only: ptls_context_t = openssl_ctx;
    while (*(**openssl_ctx_sha256only
        .cipher_suites
        .offset(0 as ::core::ffi::c_int as isize))
    .hash)
        .digest_size
        != 32 as size_t
    {
        openssl_ctx_sha256only.cipher_suites = openssl_ctx_sha256only.cipher_suites.offset(1);
    }
    ctx_peer = &raw mut openssl_ctx;
    ctx = ctx_peer;
    verify_certificate = &raw mut openssl_verify_certificate.super_0;
    let mut i: size_t = 0;
    i = 0 as size_t;
    while !ffx_variants[i as usize].algo.is_null() {
        i = i.wrapping_add(1);
    }
    ffx_variants[i as usize] = st_ptls_ffx_test_variants_t {
        algo: &raw const ptls_ffx_ptls_openssl_aes128ctr_b125_r8,
        bit_length: 125 as ::core::ffi::c_int,
    };
    let mut i_0: size_t = 0;
    i_0 = 0 as size_t;
    while !ffx_variants[i_0 as usize].algo.is_null() {
        i_0 = i_0.wrapping_add(1);
    }
    ffx_variants[i_0 as usize] = st_ptls_ffx_test_variants_t {
        algo: &raw const ptls_ffx_ptls_openssl_aes128ctr_b31_r6,
        bit_length: 31 as ::core::ffi::c_int,
    };
    let mut i_1: size_t = 0;
    i_1 = 0 as size_t;
    while !ffx_variants[i_1 as usize].algo.is_null() {
        i_1 = i_1.wrapping_add(1);
    }
    ffx_variants[i_1 as usize] = st_ptls_ffx_test_variants_t {
        algo: &raw const ptls_ffx_ptls_openssl_aes128ctr_b53_r4,
        bit_length: 53 as ::core::ffi::c_int,
    };
    let mut i_2: size_t = 0;
    i_2 = 0 as size_t;
    while !ffx_variants[i_2 as usize].algo.is_null() {
        i_2 = i_2.wrapping_add(1);
    }
    ffx_variants[i_2 as usize] = st_ptls_ffx_test_variants_t {
        algo: &raw const ptls_ffx_ptls_openssl_chacha20_b256_r8,
        bit_length: 256 as ::core::ffi::c_int,
    };
    let mut i_3: size_t = 0;
    i_3 = 0 as size_t;
    while !ffx_variants[i_3 as usize].algo.is_null() {
        i_3 = i_3.wrapping_add(1);
    }
    ffx_variants[i_3 as usize] = st_ptls_ffx_test_variants_t {
        algo: &raw const ptls_ffx_ptls_openssl_chacha20_b32_r6,
        bit_length: 32 as ::core::ffi::c_int,
    };
    let mut i_4: size_t = 0;
    i_4 = 0 as size_t;
    while !ffx_variants[i_4 as usize].algo.is_null() {
        i_4 = i_4.wrapping_add(1);
    }
    ffx_variants[i_4 as usize] = st_ptls_ffx_test_variants_t {
        algo: &raw const ptls_ffx_ptls_openssl_chacha20_b57_r4,
        bit_length: 57 as ::core::ffi::c_int,
    };
    let mut _name_1: *const ::core::ffi::c_char = b"sha\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_1);
    test_sha();
    exit_subtest(_name_1);
    let mut _name_2: *const ::core::ffi::c_char =
        b"rsa-sign\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_2);
    test_rsa_sign();
    exit_subtest(_name_2);
    let mut _name_3: *const ::core::ffi::c_char =
        b"ecdsa-sign\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_3);
    test_ecdsa_sign();
    exit_subtest(_name_3);
    let mut _name_4: *const ::core::ffi::c_char =
        b"ed25519-sign\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_4);
    test_ed25519_sign();
    exit_subtest(_name_4);
    let mut _name_5: *const ::core::ffi::c_char =
        b"cert-verify\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_5);
    test_cert_verify();
    exit_subtest(_name_5);
    let mut _name_6: *const ::core::ffi::c_char =
        b"picotls\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_6);
    test_picotls();
    exit_subtest(_name_6);
    ctx_peer = &raw mut openssl_ctx_sha256only;
    ctx = ctx_peer;
    let mut _name_7: *const ::core::ffi::c_char =
        b"picotls\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_7);
    test_picotls();
    exit_subtest(_name_7);
    ctx = &raw mut openssl_ctx_sha256only;
    ctx_peer = &raw mut openssl_ctx;
    let mut _name_8: *const ::core::ffi::c_char =
        b"picotls\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_8);
    test_picotls();
    exit_subtest(_name_8);
    ctx = &raw mut openssl_ctx;
    ctx_peer = &raw mut openssl_ctx_sha256only;
    let mut _name_9: *const ::core::ffi::c_char =
        b"picotls\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_9);
    test_picotls();
    exit_subtest(_name_9);
    let mut minicrypto_sign_certificate: ptls_minicrypto_secp256r1sha256_sign_certificate_t =
        st_ptls_minicrypto_secp256r1sha256_sign_certificate_t {
            super_0: st_ptls_sign_certificate_t { cb: None },
            key: [0; 32],
        };
    let mut minicrypto_certificate: ptls_iovec_t = ptls_iovec_init(
        SECP256R1_CERTIFICATE.as_ptr() as *const ::core::ffi::c_void,
        (::core::mem::size_of::<[::core::ffi::c_char; 613]>() as size_t).wrapping_sub(1 as size_t),
    );
    ptls_minicrypto_init_secp256r1sha256_sign_certificate(
        &raw mut minicrypto_sign_certificate,
        ptls_iovec_init(
            SECP256R1_PRIVATE_KEY.as_ptr() as *const ::core::ffi::c_void,
            (::core::mem::size_of::<[::core::ffi::c_char; 33]>() as size_t)
                .wrapping_sub(1 as size_t),
        ),
    );
    let mut minicrypto_ctx: ptls_context_t = {
        let mut init = st_ptls_context_t {
            require_dhe_on_psk_use_exporter_send_change_cipher_spec_require_client_authentication_omit_end_of_early_data_use_raw_public_keys_server_cipher_preference_server_cipher_chacha_priority: [0; 1],
            c2rust_padding: [0; 7],
            random_bytes: Some(
                ptls_minicrypto_random_bytes
                    as unsafe extern "C" fn(*mut ::core::ffi::c_void, size_t) -> (),
            ),
            get_time: &raw mut ptls_get_time,
            key_exchanges: &raw mut ptls_minicrypto_key_exchanges
                as *mut *const ptls_key_exchange_algorithm_t,
            cipher_suites: &raw mut ptls_minicrypto_cipher_suites
                as *mut *const ptls_cipher_suite_t,
            certificates: C2Rust_Unnamed_12 {
                list: &raw mut minicrypto_certificate,
                count: 1 as size_t,
            },
            pre_shared_key: C2Rust_Unnamed_11 {
                identity: st_ptls_iovec_t {
                    base: ::core::ptr::null_mut::<uint8_t>(),
                    len: 0,
                },
                secret: st_ptls_iovec_t {
                    base: ::core::ptr::null_mut::<uint8_t>(),
                    len: 0,
                },
                hash: ::core::ptr::null::<ptls_hash_algorithm_t>(),
            },
            ech: C2Rust_Unnamed_8 {
                client: C2Rust_Unnamed_10 {
                    ciphers: ::core::ptr::null_mut::<*const ptls_hpke_cipher_suite_t>(),
                    kems: ::core::ptr::null_mut::<*const ptls_hpke_kem_t>(),
                },
                server: C2Rust_Unnamed_9 {
                    create_opener: ::core::ptr::null_mut::<ptls_ech_create_opener_t>(),
                    retry_configs: st_ptls_iovec_t {
                        base: ::core::ptr::null_mut::<uint8_t>(),
                        len: 0,
                    },
                },
            },
            on_client_hello: ::core::ptr::null_mut::<ptls_on_client_hello_t>(),
            emit_certificate: ::core::ptr::null_mut::<ptls_emit_certificate_t>(),
            sign_certificate: &raw mut minicrypto_sign_certificate.super_0,
            verify_certificate: ::core::ptr::null_mut::<ptls_verify_certificate_t>(),
            ticket_lifetime: 0,
            max_early_data_size: 0,
            max_buffer_size: 0,
            hkdf_label_prefix__obsolete: ::core::ptr::null::<::core::ffi::c_char>(),
            encrypt_ticket: ::core::ptr::null_mut::<ptls_encrypt_ticket_t>(),
            save_ticket: ::core::ptr::null_mut::<ptls_save_ticket_t>(),
            log_event: ::core::ptr::null_mut::<ptls_log_event_t>(),
            update_open_count: ::core::ptr::null_mut::<ptls_update_open_count_t>(),
            update_traffic_key: ::core::ptr::null_mut::<ptls_update_traffic_key_t>(),
            decompress_certificate: ::core::ptr::null_mut::<
                ptls_decompress_certificate_t,
            >(),
            on_extension: ::core::ptr::null_mut::<ptls_on_extension_t>(),
            tls12_cipher_suites: ::core::ptr::null_mut::<*const ptls_cipher_suite_t>(),
            ticket_context: C2Rust_Unnamed_1 {
                bytes: [0; 32],
                is_set: [0; 1],
                c2rust_padding: [0; 3],
            },
            client_ca_names: C2Rust_Unnamed_0 {
                list: ::core::ptr::null::<ptls_iovec_t>(),
                count: 0,
            },
        };
        init.set_require_dhe_on_psk(0);
        init.set_use_exporter(0);
        init.set_send_change_cipher_spec(0);
        init.set_require_client_authentication(0);
        init.set_omit_end_of_early_data(0);
        init.set_use_raw_public_keys(0);
        init.set_server_cipher_preference(0);
        init.set_server_cipher_chacha_priority(0);
        init
    };
    ctx = &raw mut openssl_ctx;
    ctx_peer = &raw mut minicrypto_ctx;
    let mut _name_10: *const ::core::ffi::c_char =
        b"vs. minicrypto\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_10);
    test_picotls();
    exit_subtest(_name_10);
    ctx = &raw mut minicrypto_ctx;
    ctx_peer = &raw mut openssl_ctx;
    let mut _name_11: *const ::core::ffi::c_char =
        b"minicrypto vs.\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_11);
    test_picotls();
    exit_subtest(_name_11);
    let mut _name_12: *const ::core::ffi::c_char = b"hpke\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_12);
    test_all_hpke();
    exit_subtest(_name_12);
    static mut fast_keyex: [*const ptls_key_exchange_algorithm_t; 2] = unsafe {
        [
            &raw const ptls_openssl_x25519,
            ::core::ptr::null::<ptls_key_exchange_algorithm_t>(),
        ]
    };
    static mut fast_cipher: [*const ptls_cipher_suite_t; 2] = unsafe {
        [
            &raw const ptls_openssl_aes128gcmsha256,
            ::core::ptr::null::<ptls_cipher_suite_t>(),
        ]
    };
    openssl_ctx.key_exchanges = &raw mut fast_keyex as *mut *const ptls_key_exchange_algorithm_t;
    openssl_ctx.cipher_suites = &raw mut fast_cipher as *mut *const ptls_cipher_suite_t;
    ctx = &raw mut openssl_ctx;
    ctx_peer = &raw mut openssl_ctx;
    openssl_sign_certificate.set_async_0(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    let mut _name_13: *const ::core::ffi::c_char =
        b"many-handshakes-non-async\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_13);
    many_handshakes();
    exit_subtest(_name_13);
    openssl_sign_certificate.set_async_0(0 as ::core::ffi::c_uint as ::core::ffi::c_uint);
    let mut _name_14: *const ::core::ffi::c_char =
        b"many-handshakes-async\0".as_ptr() as *const ::core::ffi::c_char;
    enter_subtest(_name_14);
    many_handshakes();
    exit_subtest(_name_14);
    let mut engine_name: *const ::core::ffi::c_char =
        b"qatengine\0".as_ptr() as *const ::core::ffi::c_char;
    let mut qatengine: *mut ENGINE = ::core::ptr::null_mut::<ENGINE>();
    qatengine = ENGINE_by_id(engine_name);
    if !qatengine.is_null() || {
        qatengine = load_engine(engine_name);
        !qatengine.is_null()
    } {
        ENGINE_set_default_RSA(qatengine);
        ptls_openssl_dispose_sign_certificate(&raw mut openssl_sign_certificate);
        setup_sign_certificate(&raw mut openssl_sign_certificate);
        let mut _name_15: *const ::core::ffi::c_char =
            b"many-handshakes-qatengine\0".as_ptr() as *const ::core::ffi::c_char;
        enter_subtest(_name_15);
        many_handshakes();
        exit_subtest(_name_15);
    } else {
        note(
            b"%s not found\0".as_ptr() as *const ::core::ffi::c_char,
            engine_name,
        );
    }
    let mut ret: ::core::ffi::c_int = done_testing();
    OSSL_PROVIDER_unload(dflt);
    OSSL_PROVIDER_unload(legacy);
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

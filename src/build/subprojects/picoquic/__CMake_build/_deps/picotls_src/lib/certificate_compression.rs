extern "C" {
    pub type st_ptls_t;
    pub type st_ptls_key_schedule_t;
    pub type st_ptls_traffic_protection_t;
    fn malloc(__size: size_t) -> *mut ::core::ffi::c_void;
    fn free(__ptr: *mut ::core::ffi::c_void);
    fn BrotliDecoderDecompress(
        encoded_size: size_t,
        encoded_buffer: *const uint8_t,
        decoded_size: *mut size_t,
        decoded_buffer: *mut uint8_t,
    ) -> BrotliDecoderResult;
    fn BrotliEncoderCompress(
        quality: ::core::ffi::c_int,
        lgwin: ::core::ffi::c_int,
        mode: BrotliEncoderMode,
        input_size: size_t,
        input_buffer: *const uint8_t,
        encoded_size: *mut size_t,
        encoded_buffer: *mut uint8_t,
    ) -> ::core::ffi::c_int;
    fn ptls_buffer__release_memory(buf: *mut ptls_buffer_t);
    fn ptls_buffer__do_pushv(
        buf: *mut ptls_buffer_t,
        src: *const ::core::ffi::c_void,
        len: size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_buffer__adjust_quic_blocksize(
        buf: *mut ptls_buffer_t,
        body_size: size_t,
    ) -> ::core::ffi::c_int;
    fn ptls_build_certificate_message(
        buf: *mut ptls_buffer_t,
        request_context: ptls_iovec_t,
        certificates: *mut ptls_iovec_t,
        num_certificates: size_t,
        ocsp_status: ptls_iovec_t,
    ) -> ::core::ffi::c_int;
    fn ptls__key_schedule_update_hash(
        sched: *mut ptls_key_schedule_t,
        msg: *const uint8_t,
        msglen: size_t,
        use_outer: ::core::ffi::c_int,
    );
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type BrotliDecoderResult = ::core::ffi::c_uint;
pub const BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT: BrotliDecoderResult = 3;
pub const BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT: BrotliDecoderResult = 2;
pub const BROTLI_DECODER_RESULT_SUCCESS: BrotliDecoderResult = 1;
pub const BROTLI_DECODER_RESULT_ERROR: BrotliDecoderResult = 0;
pub type BrotliEncoderMode = ::core::ffi::c_uint;
pub const BROTLI_MODE_FONT: BrotliEncoderMode = 2;
pub const BROTLI_MODE_TEXT: BrotliEncoderMode = 1;
pub const BROTLI_MODE_GENERIC: BrotliEncoderMode = 0;
pub type ptls_t = st_ptls_t;
pub type ptls_iovec_t = st_ptls_iovec_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_iovec_t {
    pub base: *mut uint8_t,
    pub len: size_t,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_emit_compressed_certificate_t {
    pub super_0: ptls_emit_certificate_t,
    pub algo: uint16_t,
    pub with_ocsp_status: st_ptls_compressed_certificate_entry_t,
    pub without_ocsp_status: st_ptls_compressed_certificate_entry_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_compressed_certificate_entry_t {
    pub uncompressed_length: uint32_t,
    pub bytes: ptls_iovec_t,
}
pub type ptls_emit_compressed_certificate_t = st_ptls_emit_compressed_certificate_t;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const UINT16_MAX: ::core::ffi::c_int = 65535 as ::core::ffi::c_int;
pub const BROTLI_TRUE: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const BROTLI_MAX_QUALITY: ::core::ffi::c_int = 11 as ::core::ffi::c_int;
pub const BROTLI_DEFAULT_WINDOW: ::core::ffi::c_int = 22 as ::core::ffi::c_int;
pub const PTLS_ERROR_CLASS_INTERNAL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const PTLS_ALERT_BAD_CERTIFICATE: ::core::ffi::c_int = 42 as ::core::ffi::c_int;
pub const PTLS_ERROR_NO_MEMORY: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 1 as ::core::ffi::c_int;
pub const PTLS_ERROR_COMPRESSION_FAILURE: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 8 as ::core::ffi::c_int;
pub const PTLS_ERROR_DELEGATE: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 10 as ::core::ffi::c_int;
pub const PTLS_ERROR_BLOCK_OVERFLOW: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 12 as ::core::ffi::c_int;
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
pub const PTLS_CERTIFICATE_COMPRESSION_ALGORITHM_BROTLI: ::core::ffi::c_int =
    2 as ::core::ffi::c_int;
#[inline]
unsafe extern "C" fn decompress_certificate(
    mut self_0: *mut ptls_decompress_certificate_t,
    mut tls: *mut ptls_t,
    mut algorithm: uint16_t,
    mut output: ptls_iovec_t,
    mut input: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut decoded_size: size_t = 0;
    if !(algorithm as ::core::ffi::c_int != PTLS_CERTIFICATE_COMPRESSION_ALGORITHM_BROTLI) {
        decoded_size = output.len;
        if !(BrotliDecoderDecompress(
            input.len,
            input.base as *const uint8_t,
            &raw mut decoded_size,
            output.base as *mut uint8_t,
        ) as ::core::ffi::c_uint
            != BROTLI_DECODER_RESULT_SUCCESS as ::core::ffi::c_int as ::core::ffi::c_uint)
        {
            if !(decoded_size != output.len) {
                return 0 as ::core::ffi::c_int;
            }
        }
    }
    return PTLS_ALERT_BAD_CERTIFICATE;
}
static mut algorithms: [uint16_t; 2] = [
    PTLS_CERTIFICATE_COMPRESSION_ALGORITHM_BROTLI as uint16_t,
    UINT16_MAX as uint16_t,
];
#[no_mangle]
pub static mut ptls_decompress_certificate: ptls_decompress_certificate_t = unsafe {
    st_ptls_decompress_certificate_t {
        supported_algorithms: &raw const algorithms as *const uint16_t,
        cb: Some(
            decompress_certificate
                as unsafe extern "C" fn(
                    *mut ptls_decompress_certificate_t,
                    *mut ptls_t,
                    uint16_t,
                    ptls_iovec_t,
                    ptls_iovec_t,
                ) -> ::core::ffi::c_int,
        ),
    }
};
unsafe extern "C" fn emit_compressed_certificate(
    mut _self: *mut ptls_emit_certificate_t,
    mut tls: *mut ptls_t,
    mut emitter: *mut ptls_message_emitter_t,
    mut key_sched: *mut ptls_key_schedule_t,
    mut context: ptls_iovec_t,
    mut push_status_request: ::core::ffi::c_int,
    mut compress_algos: *const uint16_t,
    mut num_compress_algos: size_t,
) -> ::core::ffi::c_int {
    let mut c2rust_current_block: u64;
    let mut self_0: *mut ptls_emit_compressed_certificate_t =
        _self as *mut ::core::ffi::c_void as *mut ptls_emit_compressed_certificate_t;
    let mut entry: *mut st_ptls_compressed_certificate_entry_t =
        ::core::ptr::null_mut::<st_ptls_compressed_certificate_entry_t>();
    let mut ret: ::core::ffi::c_int = 0;
    let mut i: size_t = 0 as size_t;
    loop {
        if !(i != num_compress_algos) {
            c2rust_current_block = 820271813250567934;
            break;
        }
        if *compress_algos.offset(i as isize) as ::core::ffi::c_int
            == PTLS_CERTIFICATE_COMPRESSION_ALGORITHM_BROTLI
        {
            c2rust_current_block = 12080852825468645174;
            break;
        }
        i = i.wrapping_add(1);
    }
    match c2rust_current_block {
        820271813250567934 => {
            ret = PTLS_ERROR_DELEGATE;
        }
        _ => {
            entry = &raw mut (*self_0).without_ocsp_status
                as *mut st_ptls_compressed_certificate_entry_t;
            if push_status_request != 0
                && (*self_0).with_ocsp_status.uncompressed_length != 0 as uint32_t
            {
                entry = &raw mut (*self_0).with_ocsp_status
                    as *mut st_ptls_compressed_certificate_entry_t;
            }
            let mut _emitter: *mut ptls_message_emitter_t = emitter;
            ret = (*_emitter)
                .begin_message
                .expect("non-null function pointer")(
                _emitter as *mut st_ptls_message_emitter_t
            );
            if !(ret != 0 as ::core::ffi::c_int) {
                let mut _buf: *mut ptls_buffer_t = (*_emitter).buf;
                let mut _key_sched: *mut ptls_key_schedule_t = key_sched;
                let mut mess_start: size_t = (*_buf).off;
                let mut c2rust_fresh0: [uint8_t; 1] = [25 as ::core::ffi::c_int as uint8_t];
                ret = ptls_buffer__do_pushv(
                    _buf,
                    &raw mut c2rust_fresh0 as *mut uint8_t as *const ::core::ffi::c_void,
                    ::core::mem::size_of::<[uint8_t; 1]>() as size_t,
                );
                if !(ret != 0 as ::core::ffi::c_int) {
                    let mut capacity: size_t = 3 as size_t;
                    ret = ptls_buffer__do_pushv(
                        _buf,
                        b"\0\0\0\0\0\0\0\0".as_ptr() as *const ::core::ffi::c_char as *mut uint8_t
                            as *const ::core::ffi::c_void,
                        (if capacity != -(1 as ::core::ffi::c_int) as size_t {
                            capacity
                        } else {
                            1 as size_t
                        }),
                    );
                    if !(ret != 0 as ::core::ffi::c_int) {
                        let mut body_start: size_t = (*_buf).off;
                        let mut _v: uint16_t = 2 as uint16_t;
                        let mut c2rust_fresh1: [uint8_t; 2] = [
                            (_v as ::core::ffi::c_int >> 8 as ::core::ffi::c_int) as uint8_t,
                            _v as uint8_t,
                        ];
                        ret = ptls_buffer__do_pushv(
                            (*emitter).buf,
                            &raw mut c2rust_fresh1 as *mut uint8_t as *const ::core::ffi::c_void,
                            ::core::mem::size_of::<[uint8_t; 2]>() as size_t,
                        );
                        if !(ret != 0 as ::core::ffi::c_int) {
                            let mut _v_0: uint32_t = (*entry).uncompressed_length;
                            let mut c2rust_fresh2: [uint8_t; 3] = [
                                (_v_0 >> 16 as ::core::ffi::c_int) as uint8_t,
                                (_v_0 >> 8 as ::core::ffi::c_int) as uint8_t,
                                _v_0 as uint8_t,
                            ];
                            ret = ptls_buffer__do_pushv(
                                (*emitter).buf,
                                &raw mut c2rust_fresh2 as *mut uint8_t
                                    as *const ::core::ffi::c_void,
                                ::core::mem::size_of::<[uint8_t; 3]>() as size_t,
                            );
                            if !(ret != 0 as ::core::ffi::c_int) {
                                let mut capacity_0: size_t = 3 as size_t;
                                ret = ptls_buffer__do_pushv(
                                    (*emitter).buf,
                                    b"\0\0\0\0\0\0\0\0".as_ptr() as *const ::core::ffi::c_char
                                        as *mut uint8_t
                                        as *const ::core::ffi::c_void,
                                    (if capacity_0 != -(1 as ::core::ffi::c_int) as size_t {
                                        capacity_0
                                    } else {
                                        1 as size_t
                                    }),
                                );
                                if !(ret != 0 as ::core::ffi::c_int) {
                                    let mut body_start_0: size_t = (*(*emitter).buf).off;
                                    ret = ptls_buffer__do_pushv(
                                        (*emitter).buf,
                                        (*entry).bytes.base as *const ::core::ffi::c_void,
                                        (*entry).bytes.len,
                                    );
                                    if !(ret != 0 as ::core::ffi::c_int) {
                                        let mut body_size: size_t =
                                            (*(*emitter).buf).off.wrapping_sub(body_start_0);
                                        if capacity_0 != -(1 as ::core::ffi::c_int) as size_t {
                                            if capacity_0
                                                < ::core::mem::size_of::<size_t>() as usize
                                                && body_size
                                                    >= (1 as ::core::ffi::c_int as size_t)
                                                        << capacity_0.wrapping_mul(8 as size_t)
                                            {
                                                ret = 0x200 as ::core::ffi::c_int
                                                    + 12 as ::core::ffi::c_int;
                                                c2rust_current_block = 12981011657373283918;
                                            } else {
                                                while capacity_0 != 0 as size_t {
                                                    *(*(*emitter).buf).base.offset(
                                                        body_start_0.wrapping_sub(capacity_0)
                                                            as isize,
                                                    ) = (body_size
                                                        >> (8 as size_t).wrapping_mul(
                                                            capacity_0.wrapping_sub(1 as size_t),
                                                        ))
                                                        as uint8_t;
                                                    capacity_0 = capacity_0.wrapping_sub(1);
                                                }
                                                c2rust_current_block = 1924505913685386279;
                                            }
                                        } else {
                                            ret = ptls_buffer__adjust_quic_blocksize(
                                                (*emitter).buf,
                                                body_size,
                                            );
                                            if ret != 0 as ::core::ffi::c_int {
                                                c2rust_current_block = 12981011657373283918;
                                            } else {
                                                c2rust_current_block = 1924505913685386279;
                                            }
                                        }
                                        match c2rust_current_block {
                                            12981011657373283918 => {}
                                            _ => {
                                                let mut body_size_0: size_t =
                                                    (*_buf).off.wrapping_sub(body_start);
                                                if capacity != -(1 as ::core::ffi::c_int) as size_t
                                                {
                                                    if capacity
                                                        < ::core::mem::size_of::<size_t>() as usize
                                                        && body_size_0
                                                            >= (1 as ::core::ffi::c_int as size_t)
                                                                << capacity
                                                                    .wrapping_mul(8 as size_t)
                                                    {
                                                        ret = PTLS_ERROR_BLOCK_OVERFLOW;
                                                        c2rust_current_block = 12981011657373283918;
                                                    } else {
                                                        while capacity != 0 as size_t {
                                                            *(*_buf).base.offset(
                                                                body_start.wrapping_sub(capacity)
                                                                    as isize,
                                                            ) = (body_size_0
                                                                >> (8 as size_t).wrapping_mul(
                                                                    capacity
                                                                        .wrapping_sub(1 as size_t),
                                                                ))
                                                                as uint8_t;
                                                            capacity = capacity.wrapping_sub(1);
                                                        }
                                                        c2rust_current_block = 6072622540298447352;
                                                    }
                                                } else {
                                                    ret = ptls_buffer__adjust_quic_blocksize(
                                                        _buf,
                                                        body_size_0,
                                                    );
                                                    if ret != 0 as ::core::ffi::c_int {
                                                        c2rust_current_block = 12981011657373283918;
                                                    } else {
                                                        c2rust_current_block = 6072622540298447352;
                                                    }
                                                }
                                                match c2rust_current_block {
                                                    12981011657373283918 => {}
                                                    _ => {
                                                        if !_key_sched.is_null() {
                                                            ptls__key_schedule_update_hash(
                                                                _key_sched,
                                                                (*_buf)
                                                                    .base
                                                                    .offset(mess_start as isize),
                                                                (*_buf)
                                                                    .off
                                                                    .wrapping_sub(mess_start),
                                                                0 as ::core::ffi::c_int,
                                                            );
                                                        }
                                                        ret = (*_emitter)
                                                            .commit_message
                                                            .expect("non-null function pointer")(
                                                            _emitter
                                                                as *mut st_ptls_message_emitter_t,
                                                        );
                                                        if !(ret != 0 as ::core::ffi::c_int) {
                                                            ret = 0 as ::core::ffi::c_int;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    return ret;
}
unsafe extern "C" fn build_compressed(
    mut entry: *mut st_ptls_compressed_certificate_entry_t,
    mut certificates: *mut ptls_iovec_t,
    mut num_certificates: size_t,
    mut ocsp_status: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut uncompressed: ptls_buffer_t = st_ptls_buffer_t {
        base: ::core::ptr::null_mut::<uint8_t>(),
        capacity: 0,
        off: 0,
        is_allocated: 0,
        align_bits: 0,
    };
    let mut ret: ::core::ffi::c_int = 0;
    ptls_buffer_init(
        &raw mut uncompressed,
        b"\0".as_ptr() as *const ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as size_t,
    );
    ret = ptls_build_certificate_message(
        &raw mut uncompressed,
        ptls_iovec_init(::core::ptr::null::<::core::ffi::c_void>(), 0 as size_t),
        certificates,
        num_certificates,
        ocsp_status,
    );
    if !(ret != 0 as ::core::ffi::c_int) {
        (*entry).uncompressed_length = uncompressed.off as uint32_t;
        (*entry).bytes.len = uncompressed.off.wrapping_sub(1 as size_t);
        (*entry).bytes.base = malloc((*entry).bytes.len) as *mut uint8_t;
        if (*entry).bytes.base.is_null() {
            ret = PTLS_ERROR_NO_MEMORY;
        } else if BrotliEncoderCompress(
            BROTLI_MAX_QUALITY,
            BROTLI_DEFAULT_WINDOW,
            BROTLI_MODE_GENERIC,
            uncompressed.off,
            uncompressed.base as *const uint8_t,
            &raw mut (*entry).bytes.len,
            (*entry).bytes.base as *mut uint8_t,
        ) != BROTLI_TRUE
        {
            ret = PTLS_ERROR_COMPRESSION_FAILURE;
        } else {
            ret = 0 as ::core::ffi::c_int;
        }
    }
    if ret != 0 as ::core::ffi::c_int {
        free((*entry).bytes.base as *mut ::core::ffi::c_void);
        *entry = st_ptls_compressed_certificate_entry_t {
            uncompressed_length: 0 as uint32_t,
            bytes: st_ptls_iovec_t {
                base: ::core::ptr::null_mut::<uint8_t>(),
                len: 0,
            },
        };
    }
    ptls_buffer_dispose(&raw mut uncompressed);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_init_compressed_certificate(
    mut self_0: *mut ptls_emit_compressed_certificate_t,
    mut certificates: *mut ptls_iovec_t,
    mut num_certificates: size_t,
    mut ocsp_status: ptls_iovec_t,
) -> ::core::ffi::c_int {
    let mut c2rust_current_block: u64;
    let mut ret: ::core::ffi::c_int = 0;
    *self_0 = st_ptls_emit_compressed_certificate_t {
        super_0: st_ptls_emit_certificate_t {
            cb: Some(
                emit_compressed_certificate
                    as unsafe extern "C" fn(
                        *mut ptls_emit_certificate_t,
                        *mut ptls_t,
                        *mut ptls_message_emitter_t,
                        *mut ptls_key_schedule_t,
                        ptls_iovec_t,
                        ::core::ffi::c_int,
                        *const uint16_t,
                        size_t,
                    ) -> ::core::ffi::c_int,
            ),
        },
        algo: PTLS_CERTIFICATE_COMPRESSION_ALGORITHM_BROTLI as uint16_t,
        with_ocsp_status: st_ptls_compressed_certificate_entry_t {
            uncompressed_length: 0,
            bytes: st_ptls_iovec_t {
                base: ::core::ptr::null_mut::<uint8_t>(),
                len: 0,
            },
        },
        without_ocsp_status: st_ptls_compressed_certificate_entry_t {
            uncompressed_length: 0,
            bytes: st_ptls_iovec_t {
                base: ::core::ptr::null_mut::<uint8_t>(),
                len: 0,
            },
        },
    };
    ret = build_compressed(
        &raw mut (*self_0).without_ocsp_status,
        certificates,
        num_certificates,
        ptls_iovec_init(::core::ptr::null::<::core::ffi::c_void>(), 0 as size_t),
    );
    if !(ret != 0 as ::core::ffi::c_int) {
        if ocsp_status.len != 0 as size_t {
            ret = build_compressed(
                &raw mut (*self_0).with_ocsp_status,
                certificates,
                num_certificates,
                ocsp_status,
            );
            if ret != 0 as ::core::ffi::c_int {
                c2rust_current_block = 8312082517902622202;
            } else {
                c2rust_current_block = 17179679302217393232;
            }
        } else {
            c2rust_current_block = 17179679302217393232;
        }
        match c2rust_current_block {
            8312082517902622202 => {}
            _ => {
                ret = 0 as ::core::ffi::c_int;
            }
        }
    }
    if ret != 0 as ::core::ffi::c_int {
        ptls_dispose_compressed_certificate(self_0);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_dispose_compressed_certificate(
    mut self_0: *mut ptls_emit_compressed_certificate_t,
) {
    free((*self_0).with_ocsp_status.bytes.base as *mut ::core::ffi::c_void);
    free((*self_0).without_ocsp_status.bytes.base as *mut ::core::ffi::c_void);
}

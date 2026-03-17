pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type size_t = usize;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_ptls_minicrypto_log_ctx_t {
    pub ctx: *mut ::core::ffi::c_void,
    pub fn_0: Option<
        unsafe extern "C" fn(*mut ::core::ffi::c_void, *const ::core::ffi::c_char, ...) -> (),
    >,
}
pub type ptls_minicrypto_log_ctx_t = st_ptls_minicrypto_log_ctx_t;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const PTLS_ERROR_CLASS_INTERNAL: ::core::ffi::c_int = 0x200 as ::core::ffi::c_int;
pub const PTLS_ERROR_BER_MALFORMED_TYPE: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 53 as ::core::ffi::c_int;
pub const PTLS_ERROR_BER_MALFORMED_LENGTH: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 54 as ::core::ffi::c_int;
pub const PTLS_ERROR_BER_EXCESSIVE_LENGTH: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 55 as ::core::ffi::c_int;
pub const PTLS_ERROR_BER_ELEMENT_TOO_SHORT: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 56 as ::core::ffi::c_int;
pub const PTLS_ERROR_BER_UNEXPECTED_EOC: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 57 as ::core::ffi::c_int;
pub const PTLS_ERROR_DER_INDEFINITE_LENGTH: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 58 as ::core::ffi::c_int;
pub const PTLS_ERROR_INCORRECT_ASN1_SYNTAX: ::core::ffi::c_int =
    PTLS_ERROR_CLASS_INTERNAL + 59 as ::core::ffi::c_int;
static mut asn1_type_classes: [*const ::core::ffi::c_char; 4] = [
    b"Universal\0".as_ptr() as *const ::core::ffi::c_char,
    b"Application\0".as_ptr() as *const ::core::ffi::c_char,
    b"Context-specific\0".as_ptr() as *const ::core::ffi::c_char,
    b"Private\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut asn1_universal_types: [*const ::core::ffi::c_char; 31] = [
    b"End-of-Content\0".as_ptr() as *const ::core::ffi::c_char,
    b"BOOLEAN\0".as_ptr() as *const ::core::ffi::c_char,
    b"INTEGER\0".as_ptr() as *const ::core::ffi::c_char,
    b"BIT STRING\0".as_ptr() as *const ::core::ffi::c_char,
    b"OCTET STRING\0".as_ptr() as *const ::core::ffi::c_char,
    b"NULL\0".as_ptr() as *const ::core::ffi::c_char,
    b"OBJECT IDENTIFIER\0".as_ptr() as *const ::core::ffi::c_char,
    b"Object Descriptor\0".as_ptr() as *const ::core::ffi::c_char,
    b"EXTERNAL\0".as_ptr() as *const ::core::ffi::c_char,
    b"REAL\0".as_ptr() as *const ::core::ffi::c_char,
    b"ENUMERATED\0".as_ptr() as *const ::core::ffi::c_char,
    b"EMBEDDED PDV\0".as_ptr() as *const ::core::ffi::c_char,
    b"UTF8String\0".as_ptr() as *const ::core::ffi::c_char,
    b"RELATIVE-OID\0".as_ptr() as *const ::core::ffi::c_char,
    b"Reserved (16)\0".as_ptr() as *const ::core::ffi::c_char,
    b"Reserved (17)\0".as_ptr() as *const ::core::ffi::c_char,
    b"SEQUENCE\0".as_ptr() as *const ::core::ffi::c_char,
    b"SET\0".as_ptr() as *const ::core::ffi::c_char,
    b"NumericString\0".as_ptr() as *const ::core::ffi::c_char,
    b"PrintableString\0".as_ptr() as *const ::core::ffi::c_char,
    b"T61String\0".as_ptr() as *const ::core::ffi::c_char,
    b"VideotexString\0".as_ptr() as *const ::core::ffi::c_char,
    b"IA5String\0".as_ptr() as *const ::core::ffi::c_char,
    b"UTCTime\0".as_ptr() as *const ::core::ffi::c_char,
    b"GeneralizedTime\0".as_ptr() as *const ::core::ffi::c_char,
    b"GraphicString\0".as_ptr() as *const ::core::ffi::c_char,
    b"VisibleString\0".as_ptr() as *const ::core::ffi::c_char,
    b"GeneralString\0".as_ptr() as *const ::core::ffi::c_char,
    b"UniversalString\0".as_ptr() as *const ::core::ffi::c_char,
    b"CHARACTER STRING\0".as_ptr() as *const ::core::ffi::c_char,
    b"BMPString\0".as_ptr() as *const ::core::ffi::c_char,
];
static mut nb_asn1_universal_types: size_t = 0;
unsafe extern "C" fn ptls_asn1_print_indent(
    mut level: ::core::ffi::c_int,
    mut log_ctx: *mut ptls_minicrypto_log_ctx_t,
) {
    let mut indent: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while indent <= level {
        (*log_ctx).fn_0.expect("non-null function pointer")(
            (*log_ctx).ctx,
            b"   \0".as_ptr() as *const ::core::ffi::c_char,
        );
        indent += 1;
    }
}
#[no_mangle]
pub unsafe extern "C" fn ptls_asn1_error_message(
    mut error_label: *const ::core::ffi::c_char,
    mut bytes_max: size_t,
    mut byte_index: size_t,
    mut level: ::core::ffi::c_int,
    mut log_ctx: *mut ptls_minicrypto_log_ctx_t,
) -> size_t {
    if !log_ctx.is_null() {
        ptls_asn1_print_indent(level, log_ctx);
        (*log_ctx).fn_0.expect("non-null function pointer")(
            (*log_ctx).ctx,
            b"Error: %s (near position: %d (0x%x) out of %d)\0".as_ptr()
                as *const ::core::ffi::c_char,
            error_label,
            byte_index as ::core::ffi::c_int,
            byte_index as uint32_t,
            bytes_max as ::core::ffi::c_int,
        );
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_asn1_dump_content(
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut byte_index: size_t,
    mut log_ctx: *mut ptls_minicrypto_log_ctx_t,
) {
    if !log_ctx.is_null() && bytes_max > byte_index {
        let mut nb_bytes: size_t = bytes_max.wrapping_sub(byte_index);
        (*log_ctx).fn_0.expect("non-null function pointer")(
            (*log_ctx).ctx,
            b" \0".as_ptr() as *const ::core::ffi::c_char,
        );
        let mut i: size_t = 0 as size_t;
        while i < 16 as size_t && i < nb_bytes {
            (*log_ctx).fn_0.expect("non-null function pointer")(
                (*log_ctx).ctx,
                b"%02x\0".as_ptr() as *const ::core::ffi::c_char,
                *bytes.offset(byte_index.wrapping_add(i) as isize) as ::core::ffi::c_int,
            );
            i = i.wrapping_add(1);
        }
        if nb_bytes > 16 as size_t {
            (*log_ctx).fn_0.expect("non-null function pointer")(
                (*log_ctx).ctx,
                b"...\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn ptls_asn1_read_type(
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut structure_bit: *mut ::core::ffi::c_int,
    mut type_class: *mut ::core::ffi::c_int,
    mut type_number: *mut uint32_t,
    mut decode_error: *mut ::core::ffi::c_int,
    mut level: ::core::ffi::c_int,
    mut log_ctx: *mut ptls_minicrypto_log_ctx_t,
) -> size_t {
    let mut byte_index: size_t = 1 as size_t;
    let mut first_byte: uint8_t = *bytes.offset(0 as ::core::ffi::c_int as isize);
    *structure_bit =
        first_byte as ::core::ffi::c_int >> 5 as ::core::ffi::c_int & 1 as ::core::ffi::c_int;
    *type_class =
        first_byte as ::core::ffi::c_int >> 6 as ::core::ffi::c_int & 3 as ::core::ffi::c_int;
    *type_number = (first_byte as ::core::ffi::c_int & 31 as ::core::ffi::c_int) as uint32_t;
    if *type_number == 31 as uint32_t {
        let mut long_type: uint32_t = 0 as uint32_t;
        let type_number_limit: uint32_t = 0x7fffffff as uint32_t;
        let mut next_byte: ::core::ffi::c_int = 0;
        let mut end_found: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
        while byte_index < bytes_max && long_type <= type_number_limit {
            let c2rust_fresh0 = byte_index;
            byte_index = byte_index.wrapping_add(1);
            next_byte = *bytes.offset(c2rust_fresh0 as isize) as ::core::ffi::c_int;
            long_type <<= 7 as ::core::ffi::c_int;
            long_type |= (next_byte & 127 as ::core::ffi::c_int) as uint32_t;
            if !(next_byte & 128 as ::core::ffi::c_int == 0 as ::core::ffi::c_int) {
                continue;
            }
            end_found = 1 as ::core::ffi::c_int;
            break;
        }
        if end_found != 0 {
            *type_number = long_type;
        } else {
            byte_index = ptls_asn1_error_message(
                b"Incorrect type coding\0".as_ptr() as *const ::core::ffi::c_char,
                bytes_max,
                byte_index,
                level,
                log_ctx,
            );
            *decode_error = PTLS_ERROR_BER_MALFORMED_TYPE;
        }
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_asn1_print_type(
    mut type_class: ::core::ffi::c_int,
    mut type_number: uint32_t,
    mut level: ::core::ffi::c_int,
    mut log_ctx: *mut ptls_minicrypto_log_ctx_t,
) {
    ptls_asn1_print_indent(level, log_ctx);
    if type_class == 0 as ::core::ffi::c_int && (type_number as size_t) < nb_asn1_universal_types {
        (*log_ctx).fn_0.expect("non-null function pointer")(
            (*log_ctx).ctx,
            b"%s\0".as_ptr() as *const ::core::ffi::c_char,
            asn1_universal_types[type_number as usize],
        );
    } else if type_class == 2 as ::core::ffi::c_int {
        (*log_ctx).fn_0.expect("non-null function pointer")(
            (*log_ctx).ctx,
            b"[%d]\0".as_ptr() as *const ::core::ffi::c_char,
            type_number,
        );
    } else {
        (*log_ctx).fn_0.expect("non-null function pointer")(
            (*log_ctx).ctx,
            b"%s[%d]\0".as_ptr() as *const ::core::ffi::c_char,
            asn1_type_classes[type_class as usize],
            type_number,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn ptls_asn1_read_length(
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut byte_index: size_t,
    mut length: *mut uint32_t,
    mut indefinite_length: *mut ::core::ffi::c_int,
    mut last_byte: *mut size_t,
    mut decode_error: *mut ::core::ffi::c_int,
    mut level: ::core::ffi::c_int,
    mut log_ctx: *mut ptls_minicrypto_log_ctx_t,
) -> size_t {
    let mut length_of_length: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    *indefinite_length = 0 as ::core::ffi::c_int;
    *length = 0 as uint32_t;
    *last_byte = bytes_max;
    if byte_index < bytes_max {
        let c2rust_fresh1 = byte_index;
        byte_index = byte_index.wrapping_add(1);
        *length = *bytes.offset(c2rust_fresh1 as isize) as uint32_t;
        if *length & 128 as uint32_t != 0 as uint32_t {
            length_of_length = (*length & 127 as uint32_t) as ::core::ffi::c_int;
            *length = 0 as uint32_t;
            if byte_index.wrapping_add(length_of_length as size_t) >= bytes_max {
                byte_index = ptls_asn1_error_message(
                    b"Incorrect length coding\0".as_ptr() as *const ::core::ffi::c_char,
                    bytes_max,
                    byte_index,
                    level,
                    log_ctx,
                );
                *decode_error = PTLS_ERROR_BER_MALFORMED_LENGTH;
            } else {
                let mut i: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
                while i < length_of_length && byte_index < bytes_max {
                    *length <<= 8 as ::core::ffi::c_int;
                    let c2rust_fresh2 = byte_index;
                    byte_index = byte_index.wrapping_add(1);
                    *length |= *bytes.offset(c2rust_fresh2 as isize) as uint32_t;
                    i += 1;
                }
                if length_of_length == 0 as ::core::ffi::c_int {
                    *last_byte = bytes_max;
                    *indefinite_length = 1 as ::core::ffi::c_int;
                } else {
                    *last_byte = byte_index.wrapping_add(*length as size_t);
                }
            }
        } else {
            *last_byte = byte_index.wrapping_add(*length as size_t);
        }
        if *decode_error == 0 as ::core::ffi::c_int {
            if *last_byte > bytes_max {
                byte_index = ptls_asn1_error_message(
                    b"Length larger than message\0".as_ptr() as *const ::core::ffi::c_char,
                    bytes_max,
                    byte_index,
                    level,
                    log_ctx,
                );
                *decode_error = PTLS_ERROR_BER_EXCESSIVE_LENGTH;
            }
        }
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_asn1_get_expected_type_and_length(
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut byte_index: size_t,
    mut expected_type: uint8_t,
    mut length: *mut uint32_t,
    mut indefinite_length: *mut ::core::ffi::c_int,
    mut last_byte: *mut size_t,
    mut decode_error: *mut ::core::ffi::c_int,
    mut log_ctx: *mut ptls_minicrypto_log_ctx_t,
) -> size_t {
    let mut is_indefinite: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    if *bytes.offset(byte_index as isize) as ::core::ffi::c_int
        != expected_type as ::core::ffi::c_int
    {
        byte_index = ptls_asn1_error_message(
            b"Unexpected type\0".as_ptr() as *const ::core::ffi::c_char,
            bytes_max,
            byte_index,
            0 as ::core::ffi::c_int,
            log_ctx,
        );
        *decode_error = PTLS_ERROR_INCORRECT_ASN1_SYNTAX;
    } else {
        byte_index = byte_index.wrapping_add(1);
        byte_index = ptls_asn1_read_length(
            bytes,
            bytes_max,
            byte_index,
            length,
            &raw mut is_indefinite,
            last_byte,
            decode_error,
            0 as ::core::ffi::c_int,
            log_ctx,
        );
        if !indefinite_length.is_null() {
            *indefinite_length = is_indefinite;
        } else if is_indefinite != 0 {
            byte_index = ptls_asn1_error_message(
                b"Incorrect length for DER\0".as_ptr() as *const ::core::ffi::c_char,
                bytes_max,
                byte_index,
                0 as ::core::ffi::c_int,
                log_ctx,
            );
            *decode_error = PTLS_ERROR_DER_INDEFINITE_LENGTH;
        }
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_asn1_validation_recursive(
    mut bytes: *const uint8_t,
    mut bytes_max: size_t,
    mut decode_error: *mut ::core::ffi::c_int,
    mut level: ::core::ffi::c_int,
    mut log_ctx: *mut ptls_minicrypto_log_ctx_t,
) -> size_t {
    let mut structure_bit: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut type_class: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut type_number: uint32_t = 0 as uint32_t;
    let mut length: uint32_t = 0 as uint32_t;
    let mut indefinite_length: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut last_byte: size_t = 0 as size_t;
    let mut byte_index: size_t = ptls_asn1_read_type(
        bytes,
        bytes_max,
        &raw mut structure_bit,
        &raw mut type_class,
        &raw mut type_number,
        decode_error,
        level,
        log_ctx,
    );
    if *decode_error == 0 as ::core::ffi::c_int && !log_ctx.is_null() {
        ptls_asn1_print_type(type_class, type_number, level, log_ctx);
    }
    byte_index = ptls_asn1_read_length(
        bytes,
        bytes_max,
        byte_index,
        &raw mut length,
        &raw mut indefinite_length,
        &raw mut last_byte,
        decode_error,
        level,
        log_ctx,
    );
    if last_byte <= bytes_max {
        if structure_bit != 0 {
            if !log_ctx.is_null() {
                (*log_ctx).fn_0.expect("non-null function pointer")(
                    (*log_ctx).ctx,
                    b" {\n\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
            while byte_index < last_byte {
                if indefinite_length != 0 as ::core::ffi::c_int
                    && *bytes.offset(byte_index as isize) as ::core::ffi::c_int
                        == 0 as ::core::ffi::c_int
                {
                    if byte_index.wrapping_add(2 as size_t) > bytes_max
                        || *bytes.offset(byte_index.wrapping_add(1 as size_t) as isize)
                            as ::core::ffi::c_int
                            != 0 as ::core::ffi::c_int
                    {
                        byte_index = ptls_asn1_error_message(
                            b"EOC: unexpected end of content\0".as_ptr()
                                as *const ::core::ffi::c_char,
                            bytes_max,
                            byte_index,
                            level + 1 as ::core::ffi::c_int,
                            log_ctx,
                        );
                        *decode_error = PTLS_ERROR_BER_UNEXPECTED_EOC;
                        byte_index = bytes_max;
                        break;
                    } else {
                        if !log_ctx.is_null() {
                            ptls_asn1_print_indent(level, log_ctx);
                            (*log_ctx).fn_0.expect("non-null function pointer")(
                                (*log_ctx).ctx,
                                b"EOC\n\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                        byte_index = byte_index.wrapping_add(2 as size_t);
                        break;
                    }
                } else {
                    byte_index = byte_index.wrapping_add(ptls_asn1_validation_recursive(
                        bytes.offset(byte_index as isize),
                        last_byte.wrapping_sub(byte_index),
                        decode_error,
                        level + 1 as ::core::ffi::c_int,
                        log_ctx,
                    ));
                    if *decode_error != 0 {
                        byte_index = bytes_max;
                        break;
                    } else if !log_ctx.is_null() {
                        if byte_index < last_byte {
                            (*log_ctx).fn_0.expect("non-null function pointer")(
                                (*log_ctx).ctx,
                                b",\0".as_ptr() as *const ::core::ffi::c_char,
                            );
                        }
                        (*log_ctx).fn_0.expect("non-null function pointer")(
                            (*log_ctx).ctx,
                            b"\n\0".as_ptr() as *const ::core::ffi::c_char,
                        );
                    }
                }
            }
            if !log_ctx.is_null() {
                ptls_asn1_print_indent(level, log_ctx);
                (*log_ctx).fn_0.expect("non-null function pointer")(
                    (*log_ctx).ctx,
                    b"}\0".as_ptr() as *const ::core::ffi::c_char,
                );
            }
        } else {
            ptls_asn1_dump_content(bytes, last_byte, byte_index, log_ctx);
            byte_index = last_byte;
        }
    }
    return byte_index;
}
#[no_mangle]
pub unsafe extern "C" fn ptls_asn1_validation(
    mut bytes: *const uint8_t,
    mut length: size_t,
    mut log_ctx: *mut ptls_minicrypto_log_ctx_t,
) -> ::core::ffi::c_int {
    let mut decode_error: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut decoded: size_t = ptls_asn1_validation_recursive(
        bytes,
        length,
        &raw mut decode_error,
        0 as ::core::ffi::c_int,
        log_ctx,
    );
    if decode_error == 0 as ::core::ffi::c_int && decoded < length {
        decode_error = PTLS_ERROR_BER_ELEMENT_TOO_SHORT;
        if !log_ctx.is_null() {
            (*log_ctx).fn_0.expect("non-null function pointer")(
                (*log_ctx).ctx,
                b"Type too short, %d bytes only out of %d\n\0".as_ptr()
                    as *const ::core::ffi::c_char,
                decoded as ::core::ffi::c_int,
                length as ::core::ffi::c_int,
            );
        }
    }
    return decode_error;
}
unsafe extern "C" fn c2rust_run_static_initializers() {
    nb_asn1_universal_types = (::core::mem::size_of::<[*const ::core::ffi::c_char; 31]>()
        as size_t)
        .wrapping_div(::core::mem::size_of::<*const ::core::ffi::c_char>() as size_t);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [c2rust_run_static_initializers];

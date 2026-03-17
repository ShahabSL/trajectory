extern "C" {
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
    fn abort() -> !;
}
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type size_t = usize;
pub type cf_blockwise_in_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *const uint8_t) -> ()>;
pub type cf_blockwise_out_fn =
    Option<unsafe extern "C" fn(*mut ::core::ffi::c_void, *mut uint8_t) -> ()>;
#[inline]
unsafe extern "C" fn xor_bb(
    mut out: *mut uint8_t,
    mut x: *const uint8_t,
    mut y: *const uint8_t,
    mut len: size_t,
) {
    let mut i: size_t = 0;
    i = 0 as size_t;
    while i < len {
        *out.offset(i as isize) = (*x.offset(i as isize) as ::core::ffi::c_int
            ^ *y.offset(i as isize) as ::core::ffi::c_int)
            as uint8_t;
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cf_blockwise_accumulate(
    mut partial: *mut uint8_t,
    mut npartial: *mut size_t,
    mut nblock: size_t,
    mut inp: *const ::core::ffi::c_void,
    mut nbytes: size_t,
    mut process: cf_blockwise_in_fn,
    mut ctx: *mut ::core::ffi::c_void,
) {
    cf_blockwise_accumulate_final(
        partial, npartial, nblock, inp, nbytes, process, process, ctx,
    );
}
#[no_mangle]
pub unsafe extern "C" fn cf_blockwise_accumulate_final(
    mut partial: *mut uint8_t,
    mut npartial: *mut size_t,
    mut nblock: size_t,
    mut inp: *const ::core::ffi::c_void,
    mut nbytes: size_t,
    mut process: cf_blockwise_in_fn,
    mut process_final: cf_blockwise_in_fn,
    mut ctx: *mut ::core::ffi::c_void,
) {
    let mut bufin: *const uint8_t = inp as *const uint8_t;
    if !(!partial.is_null() && *npartial < nblock) {
        abort();
    }
    if !(!inp.is_null() || nbytes == 0) {
        abort();
    }
    if !(process.is_some() && !ctx.is_null()) {
        abort();
    }
    if *npartial != 0 && nbytes != 0 {
        let mut space: size_t = nblock.wrapping_sub(*npartial);
        let mut taken: size_t = if space < nbytes { space } else { nbytes };
        memcpy(
            partial.offset(*npartial as isize) as *mut ::core::ffi::c_void,
            bufin as *const ::core::ffi::c_void,
            taken,
        );
        bufin = bufin.offset(taken as isize);
        nbytes = nbytes.wrapping_sub(taken);
        *npartial = (*npartial).wrapping_add(taken);
        if *npartial == nblock {
            if nbytes == 0 as size_t {
                process_final.expect("non-null function pointer")(ctx, partial);
            } else {
                process.expect("non-null function pointer")(ctx, partial);
            }
            *npartial = 0 as size_t;
        }
    }
    while nbytes >= nblock {
        if !(*npartial == 0 as size_t) {
            abort();
        }
        if nbytes == nblock {
            process_final.expect("non-null function pointer")(ctx, bufin);
        } else {
            process.expect("non-null function pointer")(ctx, bufin);
        }
        bufin = bufin.offset(nblock as isize);
        nbytes = nbytes.wrapping_sub(nblock);
    }
    while nbytes != 0 {
        let mut space_0: size_t = nblock.wrapping_sub(*npartial);
        let mut taken_0: size_t = if space_0 < nbytes { space_0 } else { nbytes };
        memcpy(
            partial.offset(*npartial as isize) as *mut ::core::ffi::c_void,
            bufin as *const ::core::ffi::c_void,
            taken_0,
        );
        bufin = bufin.offset(taken_0 as isize);
        nbytes = nbytes.wrapping_sub(taken_0);
        *npartial = (*npartial).wrapping_add(taken_0);
        if !(*npartial < nblock) {
            abort();
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn cf_blockwise_xor(
    mut partial: *mut uint8_t,
    mut npartial: *mut size_t,
    mut nblock: size_t,
    mut inp: *const ::core::ffi::c_void,
    mut outp: *mut ::core::ffi::c_void,
    mut nbytes: size_t,
    mut process: cf_blockwise_out_fn,
    mut ctx: *mut ::core::ffi::c_void,
) {
    let mut inb: *const uint8_t = inp as *const uint8_t;
    let mut outb: *mut uint8_t = outp as *mut uint8_t;
    if !(!partial.is_null() && *npartial < nblock) {
        abort();
    }
    if !(!inp.is_null() || nbytes == 0) {
        abort();
    }
    if !(process.is_some() && !ctx.is_null()) {
        abort();
    }
    while nbytes != 0 {
        if *npartial == 0 as size_t {
            process.expect("non-null function pointer")(ctx, partial);
            *npartial = nblock;
        }
        let mut offset: size_t = nblock.wrapping_sub(*npartial);
        let mut taken: size_t = if *npartial < nbytes {
            *npartial
        } else {
            nbytes
        };
        xor_bb(outb, inb, partial.offset(offset as isize), taken);
        *npartial = (*npartial).wrapping_sub(taken);
        nbytes = nbytes.wrapping_sub(taken);
        outb = outb.offset(taken as isize);
        inb = inb.offset(taken as isize);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cf_blockwise_acc_byte(
    mut partial: *mut uint8_t,
    mut npartial: *mut size_t,
    mut nblock: size_t,
    mut byte: uint8_t,
    mut nbytes: size_t,
    mut process: cf_blockwise_in_fn,
    mut ctx: *mut ::core::ffi::c_void,
) {
    let mut filled: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    while nbytes != 0 {
        let mut start: size_t = *npartial;
        let mut count: size_t = if nbytes < nblock.wrapping_sub(start) {
            nbytes
        } else {
            nblock.wrapping_sub(start)
        };
        if filled == 0 {
            memset(
                partial.offset(start as isize) as *mut ::core::ffi::c_void,
                byte as ::core::ffi::c_int,
                count,
            );
        }
        if start == 0 as size_t && count == nblock {
            filled = 1 as ::core::ffi::c_int;
        }
        if start.wrapping_add(count) == nblock {
            process.expect("non-null function pointer")(ctx, partial);
            *npartial = 0 as size_t;
        } else {
            *npartial = (*npartial).wrapping_add(count);
        }
        nbytes = nbytes.wrapping_sub(count);
    }
}
#[no_mangle]
pub unsafe extern "C" fn cf_blockwise_acc_pad(
    mut partial: *mut uint8_t,
    mut npartial: *mut size_t,
    mut nblock: size_t,
    mut fbyte: uint8_t,
    mut mbyte: uint8_t,
    mut lbyte: uint8_t,
    mut nbytes: size_t,
    mut process: cf_blockwise_in_fn,
    mut ctx: *mut ::core::ffi::c_void,
) {
    match nbytes {
        0 => {}
        1 => {
            fbyte = (fbyte as ::core::ffi::c_int ^ lbyte as ::core::ffi::c_int) as uint8_t;
            cf_blockwise_accumulate(
                partial,
                npartial,
                nblock,
                &raw mut fbyte as *const ::core::ffi::c_void,
                1 as size_t,
                process,
                ctx,
            );
        }
        2 => {
            cf_blockwise_accumulate(
                partial,
                npartial,
                nblock,
                &raw mut fbyte as *const ::core::ffi::c_void,
                1 as size_t,
                process,
                ctx,
            );
            cf_blockwise_accumulate(
                partial,
                npartial,
                nblock,
                &raw mut lbyte as *const ::core::ffi::c_void,
                1 as size_t,
                process,
                ctx,
            );
        }
        _ => {
            cf_blockwise_accumulate(
                partial,
                npartial,
                nblock,
                &raw mut fbyte as *const ::core::ffi::c_void,
                1 as size_t,
                process,
                ctx,
            );
            if lbyte as ::core::ffi::c_int != mbyte as ::core::ffi::c_int {
                cf_blockwise_acc_byte(
                    partial,
                    npartial,
                    nblock,
                    mbyte,
                    nbytes.wrapping_sub(2 as size_t),
                    process,
                    ctx,
                );
                cf_blockwise_accumulate(
                    partial,
                    npartial,
                    nblock,
                    &raw mut lbyte as *const ::core::ffi::c_void,
                    1 as size_t,
                    process,
                    ctx,
                );
            } else {
                cf_blockwise_acc_byte(
                    partial,
                    npartial,
                    nblock,
                    mbyte,
                    nbytes.wrapping_sub(1 as size_t),
                    process,
                    ctx,
                );
            }
        }
    };
}

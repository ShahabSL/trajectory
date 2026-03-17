extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn picoquic_file_open(
        file_name: *const ::core::ffi::c_char,
        flags: *const ::core::ffi::c_char,
    ) -> *mut FILE;
    fn picoquic_file_close(F: *mut FILE) -> *mut FILE;
    fn byteread_vint(s: *mut bytestream, value: *mut uint64_t) -> ::core::ffi::c_int;
    fn byteread_cid(s: *mut bytestream, cid: *mut picoquic_connection_id_t) -> ::core::ffi::c_int;
    fn fileread_binlog(
        f_binlog: *mut FILE,
        cb: Option<
            unsafe extern "C" fn(*mut bytestream, *mut ::core::ffi::c_void) -> ::core::ffi::c_int,
        >,
        cbptr: *mut ::core::ffi::c_void,
    ) -> ::core::ffi::c_int;
    fn picoquic_open_cc_log_file_for_read(
        bin_cc_log_name: *const ::core::ffi::c_char,
        flags: *mut uint16_t,
        log_time: *mut uint64_t,
    ) -> *mut FILE;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int64_t = i64;
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
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquic_connection_id_t {
    pub id: [uint8_t; 20],
    pub id_len: uint8_t,
}
pub type picoquic_connection_id_t = st_picoquic_connection_id_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bytestream {
    pub data: *mut uint8_t,
    pub size: size_t,
    pub ptr: size_t,
}
pub type csv_cb_data = csv_cb_data_st;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct csv_cb_data_st {
    pub f: *mut FILE,
    pub starttime: uint64_t,
    pub idx: ::core::ffi::c_int,
}
pub const picoquic_log_event_cc_update: C2Rust_Unnamed = 56;
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const picoquic_log_event_frame_recv: C2Rust_Unnamed = 131;
pub const picoquic_log_event_frame_sent: C2Rust_Unnamed = 130;
pub const picoquic_log_event_info_message: C2Rust_Unnamed = 58;
pub const picoquic_log_event_stream_update: C2Rust_Unnamed = 57;
pub const picoquic_log_event_alpn_update: C2Rust_Unnamed = 55;
pub const picoquic_log_event_param_update: C2Rust_Unnamed = 54;
pub const picoquic_log_event_version_update: C2Rust_Unnamed = 53;
pub const picoquic_log_event_tls_key_retired: C2Rust_Unnamed = 33;
pub const picoquic_log_event_tls_key_update: C2Rust_Unnamed = 32;
pub const picoquic_log_event_packet_buffered: C2Rust_Unnamed = 21;
pub const picoquic_log_event_packet_dropped: C2Rust_Unnamed = 20;
pub const picoquic_log_event_packet_lost: C2Rust_Unnamed = 19;
pub const picoquic_log_event_connection_id_update: C2Rust_Unnamed = 18;
pub const picoquic_log_event_connection_close: C2Rust_Unnamed = 17;
pub const picoquic_log_event_new_connection: C2Rust_Unnamed = 16;
pub const picoquic_log_event_packet_recv: C2Rust_Unnamed = 9;
pub const picoquic_log_event_packet_sent: C2Rust_Unnamed = 8;
pub const picoquic_log_event_pdu_recv: C2Rust_Unnamed = 3;
pub const picoquic_log_event_pdu_sent: C2Rust_Unnamed = 2;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn picoquic_cc_log_file_to_csv(
    mut bin_cc_log_name: *const ::core::ffi::c_char,
    mut csv_cc_log_name: *const ::core::ffi::c_char,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut log_time: uint64_t = 0 as uint64_t;
    let mut flags: uint16_t = 0;
    let mut f_binlog: *mut FILE =
        picoquic_open_cc_log_file_for_read(bin_cc_log_name, &raw mut flags, &raw mut log_time);
    let mut f_csvlog: *mut FILE = picoquic_file_open(
        csv_cc_log_name,
        b"w\0".as_ptr() as *const ::core::ffi::c_char,
    );
    if f_binlog.is_null() || f_csvlog.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        ret = picoquic_cc_bin_to_csv(f_binlog, f_csvlog);
    }
    picoquic_file_close(f_csvlog);
    picoquic_file_close(f_binlog);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_cc_bin_to_csv(
    mut f_binlog: *mut FILE,
    mut f_csvlog: *mut FILE,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    ret |= (fprintf(f_csvlog, b"time, \0".as_ptr() as *const ::core::ffi::c_char)
        <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(f_csvlog, b"path, \0".as_ptr() as *const ::core::ffi::c_char)
        <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"sequence, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"highest ack, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"high ack time, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"last time ack, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(f_csvlog, b"cwin, \0".as_ptr() as *const ::core::ffi::c_char)
        <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"one-way-delay, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"rtt-sample, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(f_csvlog, b"SRTT, \0".as_ptr() as *const ::core::ffi::c_char)
        <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"RTT min, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"Bandwidth (B/s), \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"Receive rate (B/s), \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"Send MTU, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"pacing packet time(us), \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"nb retrans, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"nb spurious, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"cwin blkd, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"flow blkd, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"stream blkd, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"app limited, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"cc_state, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"cc_param, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"bw_max, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(
        f_csvlog,
        b"transit, \0".as_ptr() as *const ::core::ffi::c_char,
    ) <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    ret |= (fprintf(f_csvlog, b"\n\0".as_ptr() as *const ::core::ffi::c_char)
        <= 0 as ::core::ffi::c_int) as ::core::ffi::c_int;
    if ret == 0 as ::core::ffi::c_int {
        let mut data: csv_cb_data = csv_cb_data_st {
            f: ::core::ptr::null_mut::<FILE>(),
            starttime: 0,
            idx: 0,
        };
        data.f = f_csvlog;
        data.starttime = 0 as uint64_t;
        data.idx = 0 as ::core::ffi::c_int;
        ret = fileread_binlog(
            f_binlog,
            Some(
                csv_cb
                    as unsafe extern "C" fn(
                        *mut bytestream,
                        *mut ::core::ffi::c_void,
                    ) -> ::core::ffi::c_int,
            ),
            &raw mut data as *mut ::core::ffi::c_void,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn csv_cb(
    mut s: *mut bytestream,
    mut ptr: *mut ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut data: *mut csv_cb_data = ptr as *mut csv_cb_data;
    let mut f_csvlog: *mut FILE = (*data).f;
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut cid: picoquic_connection_id_t = st_picoquic_connection_id_t {
        id: [0; 20],
        id_len: 0,
    };
    ret |= byteread_cid(s, &raw mut cid);
    let mut time: uint64_t = 0 as uint64_t;
    ret |= byteread_vint(s, &raw mut time);
    let mut path_id: uint64_t = 0 as uint64_t;
    ret |= byteread_vint(s, &raw mut path_id);
    let mut id: uint64_t = 0 as uint64_t;
    ret |= byteread_vint(s, &raw mut id);
    if (*data).idx == 0 as ::core::ffi::c_int {
        (*data).starttime = time;
    }
    (*data).idx += 1;
    time = time.wrapping_sub((*data).starttime);
    if ret == 0 as ::core::ffi::c_int
        && id == picoquic_log_event_cc_update as ::core::ffi::c_int as uint64_t
    {
        let mut sequence: uint64_t = 0 as uint64_t;
        let mut packet_rcvd: uint64_t = 0 as uint64_t;
        let mut highest_ack: uint64_t = UINT64_MAX as uint64_t;
        let mut high_ack_time: uint64_t = 0 as uint64_t;
        let mut last_time_ack: uint64_t = 0 as uint64_t;
        let mut cwin: uint64_t = 0 as uint64_t;
        let mut one_way_delay: uint64_t = 0 as uint64_t;
        let mut rtt_sample: uint64_t = 0 as uint64_t;
        let mut SRTT: uint64_t = 0 as uint64_t;
        let mut RTT_min: uint64_t = 0 as uint64_t;
        let mut bandwidth_estimate: uint64_t = 0 as uint64_t;
        let mut receive_rate_estimate: uint64_t = 0 as uint64_t;
        let mut Send_MTU: uint64_t = 0 as uint64_t;
        let mut pacing_packet_time: uint64_t = 0 as uint64_t;
        let mut nb_retrans: uint64_t = 0 as uint64_t;
        let mut nb_spurious: uint64_t = 0 as uint64_t;
        let mut cwin_blkd: uint64_t = 0 as uint64_t;
        let mut flow_blkd: uint64_t = 0 as uint64_t;
        let mut stream_blkd: uint64_t = 0 as uint64_t;
        let mut cc_state: uint64_t = 0 as uint64_t;
        let mut cc_param: uint64_t = 0 as uint64_t;
        let mut bw_max: uint64_t = 0 as uint64_t;
        let mut bytes_in_transit: uint64_t = 0 as uint64_t;
        let mut app_limited: uint64_t = 0 as uint64_t;
        ret |= byteread_vint(s, &raw mut sequence);
        ret |= byteread_vint(s, &raw mut packet_rcvd);
        if packet_rcvd != 0 as uint64_t {
            ret |= byteread_vint(s, &raw mut highest_ack);
            ret |= byteread_vint(s, &raw mut high_ack_time);
            ret |= byteread_vint(s, &raw mut last_time_ack);
        }
        ret |= byteread_vint(s, &raw mut cwin);
        ret |= byteread_vint(s, &raw mut one_way_delay);
        ret |= byteread_vint(s, &raw mut rtt_sample);
        ret |= byteread_vint(s, &raw mut SRTT);
        ret |= byteread_vint(s, &raw mut RTT_min);
        ret |= byteread_vint(s, &raw mut bandwidth_estimate);
        ret |= byteread_vint(s, &raw mut receive_rate_estimate);
        ret |= byteread_vint(s, &raw mut Send_MTU);
        ret |= byteread_vint(s, &raw mut pacing_packet_time);
        ret |= byteread_vint(s, &raw mut nb_retrans);
        ret |= byteread_vint(s, &raw mut nb_spurious);
        ret |= byteread_vint(s, &raw mut cwin_blkd);
        ret |= byteread_vint(s, &raw mut flow_blkd);
        ret |= byteread_vint(s, &raw mut stream_blkd);
        byteread_vint(s, &raw mut cc_state);
        byteread_vint(s, &raw mut cc_param);
        byteread_vint(s, &raw mut bw_max);
        byteread_vint(s, &raw mut bytes_in_transit);
        byteread_vint(s, &raw mut app_limited);
        if ret != 0 as ::core::ffi::c_int
            || fprintf(
                f_csvlog,
                b"%lu, %lu, %lu, %ld, %lu, %lu, %lu, %lu, %lu, %lu, %lu, %lu, %lu, %lu, %lu, %lu, %lu, %lu, %lu, %lu, %lu, %lu, %lu, %lu, %lu,\0"
                    .as_ptr() as *const ::core::ffi::c_char,
                time,
                path_id,
                sequence,
                highest_ack as int64_t,
                high_ack_time,
                last_time_ack,
                cwin,
                one_way_delay,
                rtt_sample,
                SRTT,
                RTT_min,
                bandwidth_estimate,
                receive_rate_estimate,
                Send_MTU,
                pacing_packet_time,
                nb_retrans,
                nb_spurious,
                cwin_blkd,
                flow_blkd,
                stream_blkd,
                app_limited,
                cc_state,
                cc_param,
                bw_max,
                bytes_in_transit,
            ) <= 0 as ::core::ffi::c_int
        {
            ret = -(1 as ::core::ffi::c_int);
        }
        if ret != 0 as ::core::ffi::c_int
            || fprintf(f_csvlog, b"\n\0".as_ptr() as *const ::core::ffi::c_char)
                <= 0 as ::core::ffi::c_int
        {
            ret = -(1 as ::core::ffi::c_int);
        }
    }
    return ret;
}

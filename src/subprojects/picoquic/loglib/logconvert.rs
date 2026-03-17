pub type picoquic_packet_type_enum = ::core::ffi::c_uint;
pub const picoquic_packet_type_max: picoquic_packet_type_enum = 7;
pub const picoquic_packet_1rtt_protected: picoquic_packet_type_enum = 6;
pub const picoquic_packet_0rtt_protected: picoquic_packet_type_enum = 5;
pub const picoquic_packet_handshake: picoquic_packet_type_enum = 4;
pub const picoquic_packet_retry: picoquic_packet_type_enum = 3;
pub const picoquic_packet_initial: picoquic_packet_type_enum = 2;
pub const picoquic_packet_version_negotiation: picoquic_packet_type_enum = 1;
pub const picoquic_packet_error: picoquic_packet_type_enum = 0;
pub type picoquic_frame_type_enum_t = ::core::ffi::c_uint;
pub const picoquic_frame_type_observed_address_v6: picoquic_frame_type_enum_t = 10453415;
pub const picoquic_frame_type_observed_address_v4: picoquic_frame_type_enum_t = 10453414;
pub const picoquic_frame_type_path_blocked: picoquic_frame_type_enum_t = 354585613;
pub const picoquic_frame_type_max_path_id: picoquic_frame_type_enum_t = 354585612;
pub const picoquic_frame_type_bdp: picoquic_frame_type_enum_t = 60377;
pub const picoquic_frame_type_path_available: picoquic_frame_type_enum_t = 354585608;
pub const picoquic_frame_type_path_backup: picoquic_frame_type_enum_t = 354585607;
pub const picoquic_frame_type_path_abandon: picoquic_frame_type_enum_t = 354585605;
pub const picoquic_frame_type_path_ack_ecn: picoquic_frame_type_enum_t = 354585601;
pub const picoquic_frame_type_path_ack: picoquic_frame_type_enum_t = 354585600;
pub const picoquic_frame_type_time_stamp: picoquic_frame_type_enum_t = 757;
pub const picoquic_frame_type_immediate_ack: picoquic_frame_type_enum_t = 31;
pub const picoquic_frame_type_ack_frequency: picoquic_frame_type_enum_t = 175;
pub const picoquic_frame_type_datagram_l: picoquic_frame_type_enum_t = 49;
pub const picoquic_frame_type_datagram: picoquic_frame_type_enum_t = 48;
pub const picoquic_frame_type_handshake_done: picoquic_frame_type_enum_t = 30;
pub const picoquic_frame_type_application_close: picoquic_frame_type_enum_t = 29;
pub const picoquic_frame_type_connection_close: picoquic_frame_type_enum_t = 28;
pub const picoquic_frame_type_path_response: picoquic_frame_type_enum_t = 27;
pub const picoquic_frame_type_path_challenge: picoquic_frame_type_enum_t = 26;
pub const picoquic_frame_type_path_retire_connection_id: picoquic_frame_type_enum_t = 354585610;
pub const picoquic_frame_type_retire_connection_id: picoquic_frame_type_enum_t = 25;
pub const picoquic_frame_type_path_new_connection_id: picoquic_frame_type_enum_t = 354585609;
pub const picoquic_frame_type_new_connection_id: picoquic_frame_type_enum_t = 24;
pub const picoquic_frame_type_streams_blocked_unidir: picoquic_frame_type_enum_t = 23;
pub const picoquic_frame_type_streams_blocked_bidir: picoquic_frame_type_enum_t = 22;
pub const picoquic_frame_type_stream_data_blocked: picoquic_frame_type_enum_t = 21;
pub const picoquic_frame_type_data_blocked: picoquic_frame_type_enum_t = 20;
pub const picoquic_frame_type_max_streams_unidir: picoquic_frame_type_enum_t = 19;
pub const picoquic_frame_type_max_streams_bidir: picoquic_frame_type_enum_t = 18;
pub const picoquic_frame_type_max_stream_data: picoquic_frame_type_enum_t = 17;
pub const picoquic_frame_type_max_data: picoquic_frame_type_enum_t = 16;
pub const picoquic_frame_type_stream_range_max: picoquic_frame_type_enum_t = 15;
pub const picoquic_frame_type_stream_range_min: picoquic_frame_type_enum_t = 8;
pub const picoquic_frame_type_new_token: picoquic_frame_type_enum_t = 7;
pub const picoquic_frame_type_crypto_hs: picoquic_frame_type_enum_t = 6;
pub const picoquic_frame_type_stop_sending: picoquic_frame_type_enum_t = 5;
pub const picoquic_frame_type_reset_stream: picoquic_frame_type_enum_t = 4;
pub const picoquic_frame_type_ack_ecn: picoquic_frame_type_enum_t = 3;
pub const picoquic_frame_type_ack: picoquic_frame_type_enum_t = 2;
pub const picoquic_frame_type_poll: picoquic_frame_type_enum_t = 32;
pub const picoquic_frame_type_ping: picoquic_frame_type_enum_t = 1;
pub const picoquic_frame_type_padding: picoquic_frame_type_enum_t = 0;
#[no_mangle]
pub unsafe extern "C" fn ptype2str(
    mut ptype: picoquic_packet_type_enum,
) -> *const ::core::ffi::c_char {
    match ptype as ::core::ffi::c_uint {
        0 => return b"error\0".as_ptr() as *const ::core::ffi::c_char,
        1 => return b"version_negotiation\0".as_ptr() as *const ::core::ffi::c_char,
        2 => return b"initial\0".as_ptr() as *const ::core::ffi::c_char,
        3 => return b"retry\0".as_ptr() as *const ::core::ffi::c_char,
        4 => return b"handshake\0".as_ptr() as *const ::core::ffi::c_char,
        5 => return b"0RTT\0".as_ptr() as *const ::core::ffi::c_char,
        6 => return b"1RTT\0".as_ptr() as *const ::core::ffi::c_char,
        7 | _ => return b"unknown\0".as_ptr() as *const ::core::ffi::c_char,
    };
}
#[no_mangle]
pub unsafe extern "C" fn ftype2str(
    mut ftype: picoquic_frame_type_enum_t,
) -> *const ::core::ffi::c_char {
    if ftype as ::core::ffi::c_int >= picoquic_frame_type_stream_range_min as ::core::ffi::c_int
        && ftype as ::core::ffi::c_int <= picoquic_frame_type_stream_range_max as ::core::ffi::c_int
    {
        return b"stream\0".as_ptr() as *const ::core::ffi::c_char;
    }
    match ftype as ::core::ffi::c_uint {
        0 => return b"padding\0".as_ptr() as *const ::core::ffi::c_char,
        4 => return b"reset_stream\0".as_ptr() as *const ::core::ffi::c_char,
        28 | 29 => return b"connection_close\0".as_ptr() as *const ::core::ffi::c_char,
        16 => return b"max_data\0".as_ptr() as *const ::core::ffi::c_char,
        17 => return b"max_stream_data\0".as_ptr() as *const ::core::ffi::c_char,
        18 | 19 => return b"max_streams\0".as_ptr() as *const ::core::ffi::c_char,
        1 => return b"ping\0".as_ptr() as *const ::core::ffi::c_char,
        32 => return b"poll\0".as_ptr() as *const ::core::ffi::c_char,
        20 => return b"data_blocked\0".as_ptr() as *const ::core::ffi::c_char,
        21 => return b"stream_data_blocked\0".as_ptr() as *const ::core::ffi::c_char,
        22 | 23 => return b"streams_blocked\0".as_ptr() as *const ::core::ffi::c_char,
        24 => return b"new_connection_id\0".as_ptr() as *const ::core::ffi::c_char,
        354585609 => {
            return b"path_new_connection_id\0".as_ptr() as *const ::core::ffi::c_char;
        }
        5 => return b"stop_sending\0".as_ptr() as *const ::core::ffi::c_char,
        2 => return b"ack\0".as_ptr() as *const ::core::ffi::c_char,
        26 => return b"path_challenge\0".as_ptr() as *const ::core::ffi::c_char,
        27 => return b"path_response\0".as_ptr() as *const ::core::ffi::c_char,
        6 => return b"crypto\0".as_ptr() as *const ::core::ffi::c_char,
        7 => return b"new_token\0".as_ptr() as *const ::core::ffi::c_char,
        3 => return b"ack\0".as_ptr() as *const ::core::ffi::c_char,
        354585600 => return b"path_ack\0".as_ptr() as *const ::core::ffi::c_char,
        354585601 => return b"path_ack\0".as_ptr() as *const ::core::ffi::c_char,
        25 => return b"retire_connection_id\0".as_ptr() as *const ::core::ffi::c_char,
        354585610 => {
            return b"path_retire_connection_id\0".as_ptr() as *const ::core::ffi::c_char;
        }
        30 => return b"handshake_done\0".as_ptr() as *const ::core::ffi::c_char,
        48 | 49 => return b"datagram\0".as_ptr() as *const ::core::ffi::c_char,
        175 => return b"ack_frequency\0".as_ptr() as *const ::core::ffi::c_char,
        31 => return b"immediate_ack\0".as_ptr() as *const ::core::ffi::c_char,
        757 => return b"time_stamp\0".as_ptr() as *const ::core::ffi::c_char,
        354585605 => return b"path_abandon\0".as_ptr() as *const ::core::ffi::c_char,
        354585607 => return b"path_backup\0".as_ptr() as *const ::core::ffi::c_char,
        354585608 => return b"path_available\0".as_ptr() as *const ::core::ffi::c_char,
        60377 => return b"bdp\0".as_ptr() as *const ::core::ffi::c_char,
        354585612 => return b"max_path_id\0".as_ptr() as *const ::core::ffi::c_char,
        354585613 => return b"path_blocked\0".as_ptr() as *const ::core::ffi::c_char,
        10453414 => {
            return b"observed_address_v4\0".as_ptr() as *const ::core::ffi::c_char;
        }
        10453415 => {
            return b"observed_address_v6\0".as_ptr() as *const ::core::ffi::c_char;
        }
        _ => return b"unknown\0".as_ptr() as *const ::core::ffi::c_char,
    };
}

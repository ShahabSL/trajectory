extern "C" {
    fn picoquic_test_gauss_random(random_context: *mut uint64_t) -> ::core::ffi::c_double;
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
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type sa_family_t = ::core::ffi::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    pub __ss_padding: [::core::ffi::c_char; 118],
    pub __ss_align: ::core::ffi::c_ulong,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::core::ffi::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquictest_sim_packet_t {
    pub next_packet: *mut st_picoquictest_sim_packet_t,
    pub arrival_time: uint64_t,
    pub length: size_t,
    pub addr_from: sockaddr_storage,
    pub addr_to: sockaddr_storage,
    pub ecn_mark: uint8_t,
    pub bytes: [uint8_t; 1536],
}
pub type picoquictest_sim_packet_t = st_picoquictest_sim_packet_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct st_picoquictest_sim_link_t {
    pub next_send_time: uint64_t,
    pub queue_time: uint64_t,
    pub resume_time: uint64_t,
    pub queue_delay_max: uint64_t,
    pub picosec_per_byte: uint64_t,
    pub microsec_latency: uint64_t,
    pub loss_mask: *mut uint64_t,
    pub packets_dropped: uint64_t,
    pub packets_sent: uint64_t,
    pub jitter: uint64_t,
    pub jitter_seed: uint64_t,
    pub path_mtu: size_t,
    pub first_packet: *mut picoquictest_sim_packet_t,
    pub last_packet: *mut picoquictest_sim_packet_t,
    pub red_drop_mask: uint64_t,
    pub red_queue_max: uint64_t,
    pub l4s_max: uint64_t,
    pub bucket_increase_per_microsec: ::core::ffi::c_double,
    pub bucket_max: uint64_t,
    pub bucket_current: ::core::ffi::c_double,
    pub bucket_arrival_last: uint64_t,
    pub is_switched_off: ::core::ffi::c_int,
    pub is_unreachable: ::core::ffi::c_int,
}
pub type picoquictest_sim_link_t = st_picoquictest_sim_link_t;
pub const UINT64_MAX: ::core::ffi::c_ulong = 18446744073709551615 as ::core::ffi::c_ulong;
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const PICOQUIC_MAX_PACKET_SIZE: ::core::ffi::c_int = 1536 as ::core::ffi::c_int;
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const PICOQUIC_ECN_CE: ::core::ffi::c_int = 0x3 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn picoquictest_sim_link_create(
    mut data_rate_in_gps: ::core::ffi::c_double,
    mut microsec_latency: uint64_t,
    mut loss_mask: *mut uint64_t,
    mut queue_delay_max: uint64_t,
    mut current_time: uint64_t,
) -> *mut picoquictest_sim_link_t {
    let mut link: *mut picoquictest_sim_link_t = malloc(::core::mem::size_of::<
        picoquictest_sim_link_t,
    >() as size_t) as *mut picoquictest_sim_link_t;
    if !link.is_null() {
        let mut pico_d: ::core::ffi::c_double =
            if data_rate_in_gps <= 0 as ::core::ffi::c_int as ::core::ffi::c_double {
                0 as ::core::ffi::c_int as ::core::ffi::c_double
            } else {
                8000.0f64 / data_rate_in_gps
            };
        memset(
            link as *mut ::core::ffi::c_void,
            0 as ::core::ffi::c_int,
            ::core::mem::size_of::<picoquictest_sim_link_t>() as size_t,
        );
        pico_d *= 1.024f64 * 1.024f64;
        (*link).next_send_time = current_time;
        (*link).queue_time = current_time;
        (*link).queue_delay_max = queue_delay_max;
        (*link).picosec_per_byte = pico_d as uint64_t;
        (*link).microsec_latency = microsec_latency;
        (*link).packets_dropped = 0 as uint64_t;
        (*link).packets_sent = 0 as uint64_t;
        (*link).first_packet = ::core::ptr::null_mut::<picoquictest_sim_packet_t>();
        (*link).last_packet = ::core::ptr::null_mut::<picoquictest_sim_packet_t>();
        (*link).loss_mask = loss_mask;
        (*link).jitter_seed = 0xdeadbeefbabac001 as uint64_t;
        (*link).jitter = 0 as uint64_t;
        (*link).path_mtu = PICOQUIC_MAX_PACKET_SIZE as size_t;
        (*link).red_drop_mask = 0 as uint64_t;
        (*link).red_queue_max = 0 as uint64_t;
        (*link).bucket_increase_per_microsec = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*link).bucket_max = 0 as uint64_t;
        (*link).bucket_current = 0 as ::core::ffi::c_int as ::core::ffi::c_double;
        (*link).bucket_arrival_last = current_time;
    }
    return link;
}
#[no_mangle]
pub unsafe extern "C" fn picoquictest_sim_link_delete(mut link: *mut picoquictest_sim_link_t) {
    let mut packet: *mut picoquictest_sim_packet_t =
        ::core::ptr::null_mut::<picoquictest_sim_packet_t>();
    loop {
        packet = (*link).first_packet;
        if packet.is_null() {
            break;
        }
        (*link).first_packet = (*packet).next_packet as *mut picoquictest_sim_packet_t;
        free(packet as *mut ::core::ffi::c_void);
    }
    free(link as *mut ::core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn picoquictest_sim_link_create_packet() -> *mut picoquictest_sim_packet_t {
    let mut packet: *mut picoquictest_sim_packet_t =
        malloc(::core::mem::size_of::<picoquictest_sim_packet_t>() as size_t)
            as *mut picoquictest_sim_packet_t;
    if !packet.is_null() {
        (*packet).next_packet = ::core::ptr::null_mut::<st_picoquictest_sim_packet_t>();
        (*packet).arrival_time = 0 as uint64_t;
        (*packet).length = 0 as size_t;
        (*packet).ecn_mark = 0 as uint8_t;
    }
    return packet;
}
#[no_mangle]
pub unsafe extern "C" fn picoquictest_sim_link_next_arrival(
    mut link: *mut picoquictest_sim_link_t,
    mut current_time: uint64_t,
) -> uint64_t {
    let mut packet: *mut picoquictest_sim_packet_t = (*link).first_packet;
    if !packet.is_null() && (*packet).arrival_time < current_time {
        current_time = (*packet).arrival_time;
    }
    return current_time;
}
#[no_mangle]
pub unsafe extern "C" fn picoquictest_sim_link_dequeue(
    mut link: *mut picoquictest_sim_link_t,
    mut current_time: uint64_t,
) -> *mut picoquictest_sim_packet_t {
    let mut packet: *mut picoquictest_sim_packet_t = (*link).first_packet;
    if !packet.is_null() && (*packet).arrival_time <= current_time {
        (*link).first_packet = (*packet).next_packet as *mut picoquictest_sim_packet_t;
        if (*link).first_packet.is_null() {
            (*link).last_packet = ::core::ptr::null_mut::<picoquictest_sim_packet_t>();
        }
    } else {
        packet = ::core::ptr::null_mut::<picoquictest_sim_packet_t>();
    }
    return packet;
}
unsafe extern "C" fn picoquictest_sim_link_testloss(
    mut loss_mask: *mut uint64_t,
) -> ::core::ffi::c_int {
    let mut loss_bit: uint64_t = 0 as uint64_t;
    if !loss_mask.is_null() {
        loss_bit =
            (*loss_mask as ::core::ffi::c_ulonglong & 1 as ::core::ffi::c_ulonglong) as uint64_t;
        *loss_mask >>= 1 as ::core::ffi::c_int;
        *loss_mask |= loss_bit << 63 as ::core::ffi::c_int;
    }
    return loss_bit as ::core::ffi::c_int;
}
unsafe extern "C" fn picoquictest_sim_link_jitter(
    mut link: *mut picoquictest_sim_link_t,
) -> uint64_t {
    let mut jitter: uint64_t = (*link).jitter;
    let mut x: ::core::ffi::c_double = picoquic_test_gauss_random(&raw mut (*link).jitter_seed);
    if x < -3.0f64 {
        x = -3.0f64;
    }
    x /= 3.0f64;
    jitter = jitter.wrapping_add((x * jitter as ::core::ffi::c_double) as int64_t as uint64_t);
    return jitter;
}
#[no_mangle]
pub unsafe extern "C" fn picoquictest_sim_link_submit(
    mut link: *mut picoquictest_sim_link_t,
    mut packet: *mut picoquictest_sim_packet_t,
    mut current_time: uint64_t,
) {
    let mut queue_delay: uint64_t = if current_time > (*link).queue_time {
        0 as uint64_t
    } else {
        (*link).queue_time.wrapping_sub(current_time)
    };
    let mut transmit_time: uint64_t = (*link)
        .picosec_per_byte
        .wrapping_mul((*packet).length as uint64_t)
        >> 20 as ::core::ffi::c_int;
    let mut should_drop: uint64_t = 0 as uint64_t;
    if transmit_time <= 0 as uint64_t {
        transmit_time = 1 as uint64_t;
    }
    if (*link).bucket_increase_per_microsec > 0 as ::core::ffi::c_int as ::core::ffi::c_double {
        let mut delta_microsec: uint64_t = current_time.wrapping_sub((*link).bucket_arrival_last);
        (*link).bucket_arrival_last = current_time;
        (*link).bucket_current +=
            delta_microsec as ::core::ffi::c_double * (*link).bucket_increase_per_microsec;
        if (*link).bucket_current > (*link).bucket_max as ::core::ffi::c_double {
            (*link).bucket_current = (*link).bucket_max as ::core::ffi::c_double;
        }
        if (*link).bucket_current > (*packet).length as ::core::ffi::c_double {
            (*link).bucket_current -= (*packet).length as ::core::ffi::c_double;
        } else {
            should_drop = 1 as uint64_t;
        }
    } else if (*link).queue_delay_max > 0 as uint64_t && queue_delay >= (*link).queue_delay_max {
        if (*link).red_drop_mask == 0 as uint64_t || queue_delay >= (*link).red_queue_max {
            should_drop = 1 as uint64_t;
        } else {
            should_drop = (*link).red_drop_mask & 1 as uint64_t;
            (*link).red_drop_mask >>= 1 as ::core::ffi::c_int;
            (*link).red_drop_mask |= should_drop << 63 as ::core::ffi::c_int;
        }
    }
    if should_drop == 0 {
        (*link).queue_time = current_time
            .wrapping_add(queue_delay)
            .wrapping_add(transmit_time);
        if (*link).l4s_max > 0 as uint64_t && queue_delay >= (*link).l4s_max {
            (*packet).ecn_mark = PICOQUIC_ECN_CE as uint8_t;
        }
        if (*packet).length > (*link).path_mtu
            || picoquictest_sim_link_testloss((*link).loss_mask) != 0 as ::core::ffi::c_int
            || (*link).is_switched_off != 0
        {
            (*link).packets_dropped = (*link).packets_dropped.wrapping_add(1);
            free(packet as *mut ::core::ffi::c_void);
        } else {
            (*link).packets_sent = (*link).packets_sent.wrapping_add(1);
            if (*link).last_packet.is_null() {
                (*link).first_packet = packet;
            } else {
                (*(*link).last_packet).next_packet = packet as *mut st_picoquictest_sim_packet_t;
            }
            (*link).last_packet = packet;
            (*packet).next_packet = ::core::ptr::null_mut::<st_picoquictest_sim_packet_t>();
            (*packet).arrival_time = (*link).queue_time.wrapping_add((*link).microsec_latency);
            if (*link).jitter != 0 as uint64_t {
                (*packet).arrival_time = (*packet)
                    .arrival_time
                    .wrapping_add(picoquictest_sim_link_jitter(link));
            }
            if (*packet).arrival_time < (*link).resume_time {
                (*packet).arrival_time = (*link).resume_time;
            }
        }
    } else {
        (*link).packets_dropped = (*link).packets_dropped.wrapping_add(1);
        free(packet as *mut ::core::ffi::c_void);
    };
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_test_simlink_suspend(
    mut link: *mut picoquictest_sim_link_t,
    mut time_end_of_interval: uint64_t,
    mut simulate_receive: ::core::ffi::c_int,
) {
    let mut packet: *mut picoquictest_sim_packet_t =
        ::core::ptr::null_mut::<picoquictest_sim_packet_t>();
    let mut first_old: *mut picoquictest_sim_packet_t =
        ::core::ptr::null_mut::<picoquictest_sim_packet_t>();
    if simulate_receive != 0 {
        (*link).resume_time = time_end_of_interval;
        packet = (*link).first_packet;
        while !packet.is_null() && (*packet).arrival_time < time_end_of_interval {
            (*packet).arrival_time = time_end_of_interval;
            packet = (*packet).next_packet as *mut picoquictest_sim_packet_t;
        }
    } else {
        (*link).queue_time = time_end_of_interval;
        first_old = (*link).first_packet;
        (*link).first_packet = ::core::ptr::null_mut::<picoquictest_sim_packet_t>();
        (*link).last_packet = ::core::ptr::null_mut::<picoquictest_sim_packet_t>();
        packet = first_old;
        while !packet.is_null() {
            let mut next_packet: *mut picoquictest_sim_packet_t =
                (*packet).next_packet as *mut picoquictest_sim_packet_t;
            (*packet).next_packet = ::core::ptr::null_mut::<st_picoquictest_sim_packet_t>();
            picoquictest_sim_link_submit(link, packet, time_end_of_interval);
            packet = next_packet;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn sim_link_one_test(
    mut loss_mask: *mut uint64_t,
    mut queue_delay_max: uint64_t,
    mut nb_losses: uint64_t,
) -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut current_time: uint64_t = 0 as uint64_t;
    let mut departure_time: uint64_t = 0 as uint64_t;
    let mut link: *mut picoquictest_sim_link_t = picoquictest_sim_link_create(
        0.01f64,
        10000 as uint64_t,
        loss_mask,
        queue_delay_max,
        current_time,
    );
    let mut dequeued: uint64_t = 0 as uint64_t;
    let mut queued: uint64_t = 0 as uint64_t;
    let nb_packets: uint64_t = 16 as uint64_t;
    if link.is_null() {
        ret = -(1 as ::core::ffi::c_int);
    } else {
        while ret == 0 as ::core::ffi::c_int {
            if queued >= nb_packets {
                departure_time = UINT64_MAX as uint64_t;
            }
            current_time = picoquictest_sim_link_next_arrival(link, departure_time);
            let mut packet: *mut picoquictest_sim_packet_t =
                picoquictest_sim_link_dequeue(link, current_time);
            if !packet.is_null() {
                dequeued = dequeued.wrapping_add(1);
                free(packet as *mut ::core::ffi::c_void);
            } else {
                if !(queued < nb_packets) {
                    break;
                }
                packet = picoquictest_sim_link_create_packet();
                if packet.is_null() {
                    ret = -(1 as ::core::ffi::c_int);
                } else {
                    (*packet).length = ::core::mem::size_of::<[uint8_t; 1536]>() as usize as size_t;
                    picoquictest_sim_link_submit(link, packet, departure_time);
                    departure_time = departure_time.wrapping_add(250 as uint64_t);
                    queued = queued.wrapping_add(1);
                }
            }
        }
        if dequeued.wrapping_add(nb_losses) != nb_packets {
            ret = -(1 as ::core::ffi::c_int);
        }
        picoquictest_sim_link_delete(link);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn sim_link_test() -> ::core::ffi::c_int {
    let mut ret: ::core::ffi::c_int = 0 as ::core::ffi::c_int;
    let mut loss_mask: uint64_t = 0 as uint64_t;
    ret = sim_link_one_test(&raw mut loss_mask, 0 as uint64_t, 0 as uint64_t);
    if ret == 0 as ::core::ffi::c_int {
        loss_mask = 8 as uint64_t;
        ret = sim_link_one_test(&raw mut loss_mask, 0 as uint64_t, 1 as uint64_t);
    }
    if ret == 0 as ::core::ffi::c_int {
        loss_mask = 0x18 as uint64_t;
        ret = sim_link_one_test(&raw mut loss_mask, 0 as uint64_t, 2 as uint64_t);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn picoquic_set_test_address(
    mut addr: *mut sockaddr_in,
    mut addr_val: uint32_t,
    mut port: uint16_t,
) {
    memset(
        addr as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<sockaddr_in>() as size_t,
    );
    (*addr).sin_family = AF_INET as sa_family_t;
    (*addr).sin_addr.s_addr = addr_val as in_addr_t;
    (*addr).sin_port = port as in_port_t;
}

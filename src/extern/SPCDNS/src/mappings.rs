extern "C" {
    fn memset(
        __s: *mut ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type __int32_t = i32;
pub type __compar_fn_t = Option<
    unsafe extern "C" fn(
        *const ::core::ffi::c_void,
        *const ::core::ffi::c_void,
    ) -> ::core::ffi::c_int,
>;
pub type dns_type = ::core::ffi::c_uint;
pub const RR_UNKNOWN: dns_type = 65535;
pub const RR_PRIVATE: dns_type = 65280;
pub const RR_DLV: dns_type = 32769;
pub const RR_TA: dns_type = 32768;
pub const RR_AMTRELAY: dns_type = 260;
pub const RR_DOA: dns_type = 259;
pub const RR_AVC: dns_type = 258;
pub const RR_CAA: dns_type = 257;
pub const RR_URI: dns_type = 256;
pub const RR_ANY: dns_type = 255;
pub const RR_MAILA: dns_type = 254;
pub const RR_MAILB: dns_type = 253;
pub const RR_AXFR: dns_type = 252;
pub const RR_IXFR: dns_type = 251;
pub const RR_TSIG: dns_type = 250;
pub const RR_TKEY: dns_type = 249;
pub const RR_EUI64: dns_type = 109;
pub const RR_EUI48: dns_type = 108;
pub const RR_LP: dns_type = 107;
pub const RR_L64: dns_type = 106;
pub const RR_L32: dns_type = 105;
pub const RR_NID: dns_type = 104;
pub const RR_UNSPEC: dns_type = 103;
pub const RR_GID: dns_type = 102;
pub const RR_UID: dns_type = 101;
pub const RR_UINFO: dns_type = 100;
pub const RR_SPF: dns_type = 99;
pub const RR_HTTPS: dns_type = 65;
pub const RR_SVCB: dns_type = 64;
pub const RR_ZONEMD: dns_type = 63;
pub const RR_CSYNC: dns_type = 62;
pub const RR_OPENPGPKEY: dns_type = 61;
pub const RR_CDNSKEY: dns_type = 60;
pub const RR_CDS: dns_type = 59;
pub const RR_TALINK: dns_type = 58;
pub const RR_RKEY: dns_type = 57;
pub const RR_NINFO: dns_type = 56;
pub const RR_HIP: dns_type = 55;
pub const RR_SMIMEA: dns_type = 53;
pub const RR_TLSA: dns_type = 52;
pub const RR_NSEC3PARAM: dns_type = 51;
pub const RR_NSEC3: dns_type = 50;
pub const RR_DHCID: dns_type = 49;
pub const RR_DNSKEY: dns_type = 48;
pub const RR_NSEC: dns_type = 47;
pub const RR_RRSIG: dns_type = 46;
pub const RR_ISECKEY: dns_type = 45;
pub const RR_SSHFP: dns_type = 44;
pub const RR_DS: dns_type = 43;
pub const RR_APL: dns_type = 42;
pub const RR_OPT: dns_type = 41;
pub const RR_SINK: dns_type = 40;
pub const RR_DNAME: dns_type = 39;
pub const RR_A6: dns_type = 38;
pub const RR_CERT: dns_type = 37;
pub const RR_KX: dns_type = 36;
pub const RR_NAPTR: dns_type = 35;
pub const RR_ATMA: dns_type = 34;
pub const RR_SRV: dns_type = 33;
pub const RR_NIMLOC: dns_type = 32;
pub const RR_EID: dns_type = 31;
pub const RR_NXT: dns_type = 30;
pub const RR_LOC: dns_type = 29;
pub const RR_AAAA: dns_type = 28;
pub const RR_GPOS: dns_type = 27;
pub const RR_PX: dns_type = 26;
pub const RR_KEY: dns_type = 25;
pub const RR_SIG: dns_type = 24;
pub const RR_NSAP_PTR: dns_type = 23;
pub const RR_NSAP: dns_type = 22;
pub const RR_RT: dns_type = 21;
pub const RR_ISDN: dns_type = 20;
pub const RR_X25: dns_type = 19;
pub const RR_AFSDB: dns_type = 18;
pub const RR_RP: dns_type = 17;
pub const RR_TXT: dns_type = 16;
pub const RR_MX: dns_type = 15;
pub const RR_MINFO: dns_type = 14;
pub const RR_HINFO: dns_type = 13;
pub const RR_PTR: dns_type = 12;
pub const RR_WKS: dns_type = 11;
pub const RR_NULL: dns_type = 10;
pub const RR_MR: dns_type = 9;
pub const RR_MG: dns_type = 8;
pub const RR_MB: dns_type = 7;
pub const RR_SOA: dns_type = 6;
pub const RR_CNAME: dns_type = 5;
pub const RR_MF: dns_type = 4;
pub const RR_MD: dns_type = 3;
pub const RR_NS: dns_type = 2;
pub const RR_A: dns_type = 1;
pub type dns_type_t = dns_type;
pub type dns_class = ::core::ffi::c_uint;
pub const CLASS_UNKNOWN: dns_class = 65535;
pub const CLASS_PRIVATE: dns_class = 65280;
pub const CLASS_ANY: dns_class = 255;
pub const CLASS_NONE: dns_class = 254;
pub const CLASS_HS: dns_class = 4;
pub const CLASS_CH: dns_class = 3;
pub const CLASS_CS: dns_class = 2;
pub const CLASS_IN: dns_class = 1;
pub type dns_class_t = dns_class;
pub type dns_op = ::core::ffi::c_uint;
pub const OP_UNKNOWN: dns_op = 1;
pub const OP_UPDATE: dns_op = 5;
pub const OP_NOTIFY: dns_op = 4;
pub const OP_STATUS: dns_op = 2;
pub const OP_IQUERY: dns_op = 1;
pub const OP_QUERY: dns_op = 0;
pub type dns_op_t = dns_op;
pub type dns_rcode = ::core::ffi::c_uint;
pub const RCODE_BAD_STRING: dns_rcode = 3843;
pub const RCODE_NO_MEMORY: dns_rcode = 3842;
pub const RCODE_PRIVATE: dns_rcode = 3841;
pub const RCODE_BADCOOKIE: dns_rcode = 23;
pub const RCODE_BADTRUC: dns_rcode = 22;
pub const RCODE_BADALG: dns_rcode = 21;
pub const RCODE_BADNAME: dns_rcode = 20;
pub const RCODE_BADMODE: dns_rcode = 19;
pub const RCODE_BADTIME: dns_rcode = 18;
pub const RCODE_BADKEY: dns_rcode = 17;
pub const RCODE_BADSIG: dns_rcode = 16;
pub const RCODE_BADVERS: dns_rcode = 16;
pub const RCODE_NOTZONE: dns_rcode = 10;
pub const RCODE_NOTAUTH: dns_rcode = 9;
pub const RCODE_NXRRSET: dns_rcode = 8;
pub const RCODE_YXRRSET: dns_rcode = 7;
pub const RCODE_YXDOMAIN: dns_rcode = 6;
pub const RCODE_REFUSED: dns_rcode = 5;
pub const RCODE_NOT_IMPLEMENTED: dns_rcode = 4;
pub const RCODE_NAME_ERROR: dns_rcode = 3;
pub const RCODE_SERVER_FAILURE: dns_rcode = 2;
pub const RCODE_FORMAT_ERROR: dns_rcode = 1;
pub const RCODE_OKAY: dns_rcode = 0;
pub type dns_rcode_t = dns_rcode;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct int_string_map {
    pub value: ::core::ffi::c_int,
    pub text: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct string_int_map {
    pub text: *const ::core::ffi::c_char,
    pub value: ::core::ffi::c_int,
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const NULL_0: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
#[inline]
unsafe extern "C" fn bsearch(
    mut __key: *const ::core::ffi::c_void,
    mut __base: *const ::core::ffi::c_void,
    mut __nmemb: size_t,
    mut __size: size_t,
    mut __compar: __compar_fn_t,
) -> *mut ::core::ffi::c_void {
    let mut __l: size_t = 0;
    let mut __u: size_t = 0;
    let mut __idx: size_t = 0;
    let mut __p: *const ::core::ffi::c_void = ::core::ptr::null::<::core::ffi::c_void>();
    let mut __comparison: ::core::ffi::c_int = 0;
    __l = 0 as size_t;
    __u = __nmemb;
    while __l < __u {
        __idx = __l.wrapping_add(__u).wrapping_div(2 as size_t);
        __p = (__base as *const ::core::ffi::c_char).offset(__idx.wrapping_mul(__size) as isize)
            as *const ::core::ffi::c_void;
        __comparison = Some(__compar.expect("non-null function pointer"))
            .expect("non-null function pointer")(__key, __p);
        if __comparison < 0 as ::core::ffi::c_int {
            __u = __idx;
        } else if __comparison > 0 as ::core::ffi::c_int {
            __l = __idx.wrapping_add(1 as size_t);
        } else {
            return __p as *mut ::core::ffi::c_void;
        }
    }
    return NULL;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -(128 as ::core::ffi::c_int) && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
static mut cm_dns_rcode: [int_string_map; 21] = [
    int_string_map {
        value: RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"No error\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_FORMAT_ERROR as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Format error\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_SERVER_FAILURE as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Server failure\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_NAME_ERROR as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Non-existant domain\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_NOT_IMPLEMENTED as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Not implemented\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_REFUSED as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Query refused\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_YXDOMAIN as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Name exists when it should not\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_YXRRSET as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"RRset exists when it should not\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_NXRRSET as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"RRset does not exist\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_NOTAUTH as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Server not authoritative\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_NOTZONE as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Zone not in zone section\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BADVERS as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Bad OPT version/TSIG failed\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BADKEY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Key not recognized\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BADTIME as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Signature out of time window\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BADMODE as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Bad TKEY mode\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BADNAME as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Duplicate key name\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BADALG as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Algorithm not supported\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BADTRUC as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Bad truncation\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BADCOOKIE as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Bad/missing server cookie\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_NO_MEMORY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"No memory\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BAD_STRING as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"Bad sring\0".as_ptr() as *const ::core::ffi::c_char,
    },
];
pub const RCODE_COUNT: usize = (::core::mem::size_of::<[int_string_map; 21]>() as usize)
    .wrapping_div(::core::mem::size_of::<int_string_map>() as usize);
#[no_mangle]
pub static mut c_dns_rcode_enum: [int_string_map; 22] = [
    int_string_map {
        value: RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"OKAY\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_FORMAT_ERROR as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"FORMAT_ERROR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_SERVER_FAILURE as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"SERVER_FAILURE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_NAME_ERROR as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NAME_ERROR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_NOT_IMPLEMENTED as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NOT_IMPLEMENTED\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_REFUSED as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"REFUSED\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_YXDOMAIN as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"YXDOMAIN\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_YXRRSET as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"YXRRSET\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_NXRRSET as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NXRRSET\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_NOTAUTH as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NOTAUTH\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_NOTZONE as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NOTZONE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BADVERS as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"BADVERS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BADKEY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"BADKEY\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BADTIME as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"BADTIME\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BADMODE as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"BADMODE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BADNAME as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"BADNAME\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BADALG as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"BADALG\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BADTRUC as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"BADTRUNC\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BADCOOKIE as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"BADCOOKIE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_NO_MEMORY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NO_MEMORY\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RCODE_BAD_STRING as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"BAD_STRING\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: 0 as ::core::ffi::c_int,
        text: ::core::ptr::null::<::core::ffi::c_char>(),
    },
];
static mut cm_dns_rcode_is: [string_int_map; 21] = [
    string_int_map {
        text: b"BADALG\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_BADALG as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"BADCOOKIE\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_BADCOOKIE as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"BADKEY\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_BADKEY as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"BADMODE\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_BADMODE as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"BADNAME\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_BADNAME as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"BADTIME\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_BADTIME as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"BADTRUNC\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_BADTRUC as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"BADVERS\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_BADVERS as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"BAD_STRING\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_BAD_STRING as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"FORMAT_ERROR\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_FORMAT_ERROR as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NAME_ERROR\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_NAME_ERROR as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NOTAUTH\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_NOTAUTH as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NOTZONE\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_NOTZONE as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NOT_IMPLEMENTED\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_NOT_IMPLEMENTED as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NO_MEMORY\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_NO_MEMORY as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NXRRSET\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_NXRRSET as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"OKAY\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"REFUSED\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_REFUSED as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"SERVER_FAILURE\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_SERVER_FAILURE as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"YXDOMAIN\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_YXDOMAIN as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"YXRRSET\0".as_ptr() as *const ::core::ffi::c_char,
        value: RCODE_YXRRSET as ::core::ffi::c_int as ::core::ffi::c_int,
    },
];
static mut cm_dns_type: [int_string_map; 90] = [
    int_string_map {
        value: RR_A as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"A\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_NS as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_MD as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"MD\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_MF as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"MF\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_CNAME as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"CNAME\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_SOA as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"SOA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_MB as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"MB\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_MG as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"MG\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_MR as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"MR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_NULL as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NULL\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_WKS as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"WKS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_PTR as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"PTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_HINFO as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"HINFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_MINFO as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"MINFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_MX as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"MX\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_TXT as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"TXT\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_RP as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"RP\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_AFSDB as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"AFSDB\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_X25 as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"X25\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_ISDN as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"ISDN\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_RT as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"RT\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_NSAP as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NSAP\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_NSAP_PTR as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NSAP-PTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_SIG as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"SIG\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_KEY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"KEY\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_PX as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"PX\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_GPOS as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"GPOS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_AAAA as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"AAAA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_LOC as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"LOC\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_NXT as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NXT\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_EID as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"EID\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_NIMLOC as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NIMLOC\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_SRV as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"SRV\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_ATMA as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"ATMA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_NAPTR as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NAPTR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_KX as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"KX\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_CERT as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"CERT\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_A6 as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"A6\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_DNAME as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"DNAME\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_SINK as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"SINK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_OPT as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"OPT\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_APL as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"APL\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_DS as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"DS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_SSHFP as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"SSHFP\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_ISECKEY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"ISECKEY\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_RRSIG as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"RRSIG\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_NSEC as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NSEC\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_DNSKEY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"DNSKEY\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_DHCID as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"DHCID\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_NSEC3 as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NSEC3\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_NSEC3PARAM as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NSEC3PARAM\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_TLSA as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"TLSA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_SMIMEA as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"SMIMEA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_HIP as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"HIP\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_NINFO as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NINFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_RKEY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"RKEY\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_TALINK as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"TALINK\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_CDS as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"CDS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_CDNSKEY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"CDNSKEY\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_OPENPGPKEY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"OPENPGPKEY\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_CSYNC as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"CSYNC\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_ZONEMD as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"ZONEMD\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_HTTPS as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"HTTPS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_SPF as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"SPF\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_UINFO as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"UINFO\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_UID as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"UID\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_GID as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"GID\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_UNSPEC as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"UNSPEC\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_NID as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NID\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_L32 as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"L32\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_L64 as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"L64\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_LP as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"LP\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_EUI48 as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"EUI48\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_EUI64 as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"EUI64\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_TKEY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"TKEY\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_TSIG as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"TSIG\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_IXFR as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"IXFR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_AXFR as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"AXFR\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_MAILB as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"MAILB\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_MAILA as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"MAILA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_ANY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"ANY\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_URI as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"URI\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_CAA as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"CAA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_AVC as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"AVC\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_DOA as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"DOA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_AMTRELAY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"AMTRELAY\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_TA as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"TA\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_DLV as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"DLV\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_PRIVATE as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"PRIVATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: RR_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"UNKNOWN\0".as_ptr() as *const ::core::ffi::c_char,
    },
];
pub const TYPE_COUNT: usize = (::core::mem::size_of::<[int_string_map; 90]>() as usize)
    .wrapping_div(::core::mem::size_of::<int_string_map>() as usize);
static mut cm_dns_type_is: [string_int_map; 90] = [
    string_int_map {
        text: b"A\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_A as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"A6\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_A6 as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"AAAA\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_AAAA as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"AFSDB\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_AFSDB as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"AMTRELAY\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_AMTRELAY as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"ANY\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_ANY as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"APL\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_APL as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"ATMA\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_ATMA as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"AVC\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_AVC as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"AXFR\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_AXFR as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"CAA\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_CAA as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"CDNSKEY\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_CDNSKEY as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"CDS\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_CDS as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"CERT\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_CERT as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"CNAME\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_CNAME as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"CSYNC\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_CSYNC as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"DHCID\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_DHCID as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"DLV\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_DLV as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"DNAME\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_DNAME as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"DNSKEY\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_DNSKEY as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"DOA\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_DOA as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"DS\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_DS as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"EID\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_EID as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"EUI48\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_EUI48 as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"EUI64\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_EUI64 as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"GID\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_GID as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"GPOS\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_GPOS as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"HINFO\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_HINFO as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"HIP\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_HIP as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"HTTPS\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_HTTPS as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"ISDN\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_ISDN as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"ISECKEY\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_ISECKEY as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"IXFR\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_IXFR as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"KEY\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_KEY as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"KX\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_KX as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"L32\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_L32 as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"L64\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_L64 as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"LOC\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_LOC as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"LP\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_LP as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"MAILA\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_MAILA as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"MAILB\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_MAILB as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"MB\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_MB as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"MD\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_MD as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"MF\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_MF as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"MG\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_MG as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"MINFO\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_MINFO as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"MR\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_MR as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"MX\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_MX as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NAPTR\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_NAPTR as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NID\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_NID as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NIMLOC\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_NIMLOC as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NINFO\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_NINFO as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NS\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_NS as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NSAP\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_NSAP as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NSAP-PTR\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_NSAP_PTR as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NSEC\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_NSEC as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NSEC3\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_NSEC3 as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NSEC3PARAM\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_NSEC3PARAM as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NULL\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_NULL as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NXT\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_NXT as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"OPENPGPKEY\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_OPENPGPKEY as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"OPT\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_OPT as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"PRIVATE\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_PRIVATE as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"PTR\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_PTR as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"PX\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_PX as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"RKEY\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_RKEY as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"RP\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_RP as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"RRSIG\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_RRSIG as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"RT\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_RT as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"SIG\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_SIG as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"SINK\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_SINK as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"SMIMEA\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_SMIMEA as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"SOA\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_SOA as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"SPF\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_SPF as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"SRV\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_SRV as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"SSHFP\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_SSHFP as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"TA\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_TA as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"TALINK\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_TALINK as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"TKEY\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_TKEY as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"TLSA\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_TLSA as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"TSIG\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_TSIG as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"TXT\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_TXT as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"UID\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_UID as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"UINFO\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_UINFO as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"UNKNOWN\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"UNSPEC\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_UNSPEC as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"URI\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_URI as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"WKS\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_WKS as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"X25\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_X25 as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"ZONEMD\0".as_ptr() as *const ::core::ffi::c_char,
        value: RR_ZONEMD as ::core::ffi::c_int as ::core::ffi::c_int,
    },
];
static mut cm_dns_class: [int_string_map; 8] = [
    int_string_map {
        value: CLASS_IN as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"IN\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: CLASS_CS as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"CS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: CLASS_CH as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"CH\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: CLASS_HS as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"HS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: CLASS_NONE as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NONE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: CLASS_ANY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"ANY\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: CLASS_PRIVATE as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"PRIVATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: CLASS_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"UNKNOWN\0".as_ptr() as *const ::core::ffi::c_char,
    },
];
pub const CLASS_COUNT: usize = (::core::mem::size_of::<[int_string_map; 8]>() as usize)
    .wrapping_div(::core::mem::size_of::<int_string_map>() as usize);
static mut cm_dns_class_is: [string_int_map; 8] = [
    string_int_map {
        text: b"ANY\0".as_ptr() as *const ::core::ffi::c_char,
        value: CLASS_ANY as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"CH\0".as_ptr() as *const ::core::ffi::c_char,
        value: CLASS_CH as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"CS\0".as_ptr() as *const ::core::ffi::c_char,
        value: CLASS_CS as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"HS\0".as_ptr() as *const ::core::ffi::c_char,
        value: CLASS_HS as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"IN\0".as_ptr() as *const ::core::ffi::c_char,
        value: CLASS_IN as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"NONE\0".as_ptr() as *const ::core::ffi::c_char,
        value: CLASS_NONE as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"PRIVATE\0".as_ptr() as *const ::core::ffi::c_char,
        value: CLASS_PRIVATE as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"UNKNOWN\0".as_ptr() as *const ::core::ffi::c_char,
        value: CLASS_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_int,
    },
];
static mut cm_dns_op: [int_string_map; 5] = [
    int_string_map {
        value: OP_QUERY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"QUERY\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: OP_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"UKNOWN\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: OP_STATUS as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"STATUS\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: OP_NOTIFY as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"NOTIFY\0".as_ptr() as *const ::core::ffi::c_char,
    },
    int_string_map {
        value: OP_UPDATE as ::core::ffi::c_int as ::core::ffi::c_int,
        text: b"UPDATE\0".as_ptr() as *const ::core::ffi::c_char,
    },
];
pub const OP_COUNT: usize = (::core::mem::size_of::<[int_string_map; 5]>() as usize)
    .wrapping_div(::core::mem::size_of::<int_string_map>() as usize);
static mut cm_dns_op_is: [string_int_map; 5] = [
    string_int_map {
        text: b"NOTIFY\0".as_ptr() as *const ::core::ffi::c_char,
        value: OP_NOTIFY as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"QUERY\0".as_ptr() as *const ::core::ffi::c_char,
        value: OP_QUERY as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"STATUS\0".as_ptr() as *const ::core::ffi::c_char,
        value: OP_STATUS as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"UNKNOWN\0".as_ptr() as *const ::core::ffi::c_char,
        value: OP_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_int,
    },
    string_int_map {
        text: b"UPDATE\0".as_ptr() as *const ::core::ffi::c_char,
        value: OP_UPDATE as ::core::ffi::c_int as ::core::ffi::c_int,
    },
];
unsafe extern "C" fn intstr_cmp(
    mut needle: *const ::core::ffi::c_void,
    mut haystack: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut pism: *const int_string_map = haystack as *const int_string_map;
    let mut pi: *const ::core::ffi::c_int = needle as *const ::core::ffi::c_int;
    '_c2rust_label: {
        if !needle.is_null() {
        } else {
            __assert_fail(
                b"needle != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/mappings.c\0".as_ptr() as *const ::core::ffi::c_char,
                373 as ::core::ffi::c_uint,
                b"int intstr_cmp(const void *, const void *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !haystack.is_null() {
        } else {
            __assert_fail(
                b"haystack != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/mappings.c\0".as_ptr() as *const ::core::ffi::c_char,
                374 as ::core::ffi::c_uint,
                b"int intstr_cmp(const void *, const void *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return *pi - (*pism).value;
}
unsafe extern "C" fn strint_cmp(
    mut needle: *const ::core::ffi::c_void,
    mut haystack: *const ::core::ffi::c_void,
) -> ::core::ffi::c_int {
    let mut psim: *const string_int_map = haystack as *const string_int_map;
    let mut key: *const ::core::ffi::c_char = needle as *const ::core::ffi::c_char;
    '_c2rust_label: {
        if !needle.is_null() {
        } else {
            __assert_fail(
                b"needle != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/mappings.c\0".as_ptr() as *const ::core::ffi::c_char,
                386 as ::core::ffi::c_uint,
                b"int strint_cmp(const void *, const void *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !haystack.is_null() {
        } else {
            __assert_fail(
                b"haystack != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/mappings.c\0".as_ptr() as *const ::core::ffi::c_char,
                387 as ::core::ffi::c_uint,
                b"int strint_cmp(const void *, const void *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return strcmp(key, (*psim).text);
}
unsafe extern "C" fn itosdef(
    mut v: ::core::ffi::c_int,
    mut pitab: *const int_string_map,
    mut itabcnt: size_t,
    mut def: *const ::core::ffi::c_char,
) -> *const ::core::ffi::c_char {
    let mut pism: *mut int_string_map = ::core::ptr::null_mut::<int_string_map>();
    '_c2rust_label: {
        if v >= 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"v >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/mappings.c\0".as_ptr() as *const ::core::ffi::c_char,
                403 as ::core::ffi::c_uint,
                b"const char *itosdef(int, const struct int_string_map *, size_t, const char *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !pitab.is_null() {
        } else {
            __assert_fail(
                b"pitab != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/mappings.c\0".as_ptr() as *const ::core::ffi::c_char,
                404 as ::core::ffi::c_uint,
                b"const char *itosdef(int, const struct int_string_map *, size_t, const char *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if itabcnt > 0 as size_t {
        } else {
            __assert_fail(
                b"itabcnt > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/mappings.c\0".as_ptr() as *const ::core::ffi::c_char,
                405 as ::core::ffi::c_uint,
                b"const char *itosdef(int, const struct int_string_map *, size_t, const char *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if !def.is_null() {
        } else {
            __assert_fail(
                b"def != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/mappings.c\0".as_ptr() as *const ::core::ffi::c_char,
                406 as ::core::ffi::c_uint,
                b"const char *itosdef(int, const struct int_string_map *, size_t, const char *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    pism = bsearch(
        &raw mut v as *const ::core::ffi::c_void,
        pitab as *const ::core::ffi::c_void,
        itabcnt,
        ::core::mem::size_of::<int_string_map>() as size_t,
        Some(
            intstr_cmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    ) as *mut int_string_map;
    if !pism.is_null() {
        return (*pism).text;
    } else {
        return def;
    };
}
unsafe extern "C" fn stoidef(
    mut tag: *const ::core::ffi::c_char,
    mut pstab: *const string_int_map,
    mut stabcnt: size_t,
    mut def: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut psim: *mut string_int_map = ::core::ptr::null_mut::<string_int_map>();
    let mut len: size_t = strlen(tag).wrapping_add(1 as size_t);
    let mut buffer: [::core::ffi::c_char; 16] = [0; 16];
    let mut max: size_t = if len > 15 as size_t {
        15 as size_t
    } else {
        len
    };
    memset(
        &raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<[::core::ffi::c_char; 16]>() as size_t,
    );
    let mut i: size_t = 0 as size_t;
    while i < max {
        buffer[i as usize] = ({
            let mut __res: ::core::ffi::c_int = 0;
            if ::core::mem::size_of::<::core::ffi::c_char>() as usize > 1 as usize {
                if 0 != 0 {
                    let mut __c: ::core::ffi::c_int = *tag.offset(i as isize) as ::core::ffi::c_int;
                    __res =
                        (if __c < -(128 as ::core::ffi::c_int) || __c > 255 as ::core::ffi::c_int {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                } else {
                    __res = toupper(*tag.offset(i as isize) as ::core::ffi::c_int);
                }
            } else {
                __res = *(*__ctype_toupper_loc())
                    .offset(*tag.offset(i as isize) as ::core::ffi::c_int as isize)
                    as ::core::ffi::c_int;
            }
            __res
        }) as ::core::ffi::c_char;
        i = i.wrapping_add(1);
    }
    psim = bsearch(
        &raw mut buffer as *mut ::core::ffi::c_char as *const ::core::ffi::c_void,
        pstab as *const ::core::ffi::c_void,
        stabcnt,
        ::core::mem::size_of::<string_int_map>() as size_t,
        Some(
            strint_cmp
                as unsafe extern "C" fn(
                    *const ::core::ffi::c_void,
                    *const ::core::ffi::c_void,
                ) -> ::core::ffi::c_int,
        ),
    ) as *mut string_int_map;
    if !psim.is_null() {
        return (*psim).value;
    } else {
        return def;
    };
}
#[no_mangle]
pub unsafe extern "C" fn dns_rcode_enum(mut r: dns_rcode_t) -> *const ::core::ffi::c_char {
    return itosdef(
        r as ::core::ffi::c_int,
        &raw const c_dns_rcode_enum as *const int_string_map,
        RCODE_COUNT,
        b"X-UNKN\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dns_rcode_text(mut r: dns_rcode_t) -> *const ::core::ffi::c_char {
    return itosdef(
        r as ::core::ffi::c_int,
        &raw const cm_dns_rcode as *const int_string_map,
        RCODE_COUNT,
        b"Unknown error\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dns_type_text(mut t: dns_type_t) -> *const ::core::ffi::c_char {
    return itosdef(
        t as ::core::ffi::c_int,
        &raw const cm_dns_type as *const int_string_map,
        TYPE_COUNT,
        b"X-UNKN\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dns_class_text(mut c: dns_class_t) -> *const ::core::ffi::c_char {
    return itosdef(
        c as ::core::ffi::c_int,
        &raw const cm_dns_class as *const int_string_map,
        CLASS_COUNT,
        b"X-UNKN\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dns_op_text(mut o: dns_op_t) -> *const ::core::ffi::c_char {
    return itosdef(
        o as ::core::ffi::c_int,
        &raw const cm_dns_op as *const int_string_map,
        OP_COUNT,
        b"X-UNKNOWN\0".as_ptr() as *const ::core::ffi::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dns_rcode_value(mut tag: *const ::core::ffi::c_char) -> dns_rcode_t {
    return stoidef(
        tag,
        &raw const cm_dns_rcode_is as *const string_int_map,
        RCODE_COUNT,
        RCODE_NOT_IMPLEMENTED as ::core::ffi::c_int,
    ) as dns_rcode_t;
}
#[no_mangle]
pub unsafe extern "C" fn dns_type_value(mut tag: *const ::core::ffi::c_char) -> dns_type_t {
    return stoidef(
        tag,
        &raw const cm_dns_type_is as *const string_int_map,
        TYPE_COUNT,
        RR_UNKNOWN as ::core::ffi::c_int,
    ) as dns_type_t;
}
#[no_mangle]
pub unsafe extern "C" fn dns_class_value(mut tag: *const ::core::ffi::c_char) -> dns_class_t {
    return stoidef(
        tag,
        &raw const cm_dns_class_is as *const string_int_map,
        CLASS_COUNT,
        CLASS_UNKNOWN as ::core::ffi::c_int,
    ) as dns_class_t;
}
#[no_mangle]
pub unsafe extern "C" fn dns_op_value(mut tag: *const ::core::ffi::c_char) -> dns_op_t {
    return stoidef(
        tag,
        &raw const cm_dns_op_is as *const string_int_map,
        OP_COUNT,
        OP_UNKNOWN as ::core::ffi::c_int,
    ) as dns_op_t;
}

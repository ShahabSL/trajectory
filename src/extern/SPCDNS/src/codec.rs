extern "C" {
    fn sprintf(
        __s: *mut ::core::ffi::c_char,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn snprintf(
        __s: *mut ::core::ffi::c_char,
        __maxlen: size_t,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn strtod(
        __nptr: *const ::core::ffi::c_char,
        __endptr: *mut *mut ::core::ffi::c_char,
    ) -> ::core::ffi::c_double;
    fn ldiv(__numer: ::core::ffi::c_long, __denom: ::core::ffi::c_long) -> ldiv_t;
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
    fn memchr(
        __s: *const ::core::ffi::c_void,
        __c: ::core::ffi::c_int,
        __n: size_t,
    ) -> *mut ::core::ffi::c_void;
    fn strcmp(
        __s1: *const ::core::ffi::c_char,
        __s2: *const ::core::ffi::c_char,
    ) -> ::core::ffi::c_int;
    fn strchr(__s: *const ::core::ffi::c_char, __c: ::core::ffi::c_int)
        -> *mut ::core::ffi::c_char;
    fn strlen(__s: *const ::core::ffi::c_char) -> size_t;
    fn __errno_location() -> *mut ::core::ffi::c_int;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    fn log10(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn modf(
        __x: ::core::ffi::c_double,
        __iptr: *mut ::core::ffi::c_double,
    ) -> ::core::ffi::c_double;
    fn pow(__x: ::core::ffi::c_double, __y: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn fabs(__x: ::core::ffi::c_double) -> ::core::ffi::c_double;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ldiv_t {
    pub quot: ::core::ffi::c_long,
    pub rem: ::core::ffi::c_long,
}
pub type C2Rust_Unnamed = ::core::ffi::c_uint;
pub const _ISalnum: C2Rust_Unnamed = 8;
pub const _ISpunct: C2Rust_Unnamed = 4;
pub const _IScntrl: C2Rust_Unnamed = 2;
pub const _ISblank: C2Rust_Unnamed = 1;
pub const _ISgraph: C2Rust_Unnamed = 32768;
pub const _ISprint: C2Rust_Unnamed = 16384;
pub const _ISspace: C2Rust_Unnamed = 8192;
pub const _ISxdigit: C2Rust_Unnamed = 4096;
pub const _ISdigit: C2Rust_Unnamed = 2048;
pub const _ISalpha: C2Rust_Unnamed = 1024;
pub const _ISlower: C2Rust_Unnamed = 512;
pub const _ISupper: C2Rust_Unnamed = 256;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type uintptr_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2Rust_Unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2Rust_Unnamed_0 {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_addr_t = uint32_t;
pub type dns_packet_t = uintptr_t;
pub type dns_decoded_t = uintptr_t;
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
pub type edns0_type = ::core::ffi::c_uint;
pub const EDNS0RR_NSID: edns0_type = 3;
pub type edns0_type_t = edns0_type;
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
pub type TTL = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_question_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_generic_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_a_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub address: in_addr_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_ns_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub nsdname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_md_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub madname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_mf_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub madname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_cname_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub cname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_soa_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub mname: *const ::core::ffi::c_char,
    pub rname: *const ::core::ffi::c_char,
    pub serial: uint32_t,
    pub refresh: uint32_t,
    pub retry: uint32_t,
    pub expire: uint32_t,
    pub minimum: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_mb_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub madname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_mg_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub mgmname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_mr_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub newname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_null_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_wks_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub address: in_addr_t,
    pub protocol: ::core::ffi::c_int,
    pub numbits: size_t,
    pub bits: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_ptr_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub ptr: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_hinfo_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub cpu: *const ::core::ffi::c_char,
    pub os: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_minfo_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub rmailbx: *const ::core::ffi::c_char,
    pub emailbx: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_mx_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub preference: ::core::ffi::c_int,
    pub exchange: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_txt_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub len: size_t,
    pub text: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_rp_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub mbox: *const ::core::ffi::c_char,
    pub domain: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_afsdb_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub subtype: ::core::ffi::c_int,
    pub hostname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_x25_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub psdnaddress: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_isdn_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub isdnaddress: *const ::core::ffi::c_char,
    pub sa: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_rt_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub preference: ::core::ffi::c_int,
    pub host: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_nsap_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub length: *const ::core::ffi::c_char,
    pub nsapaddress: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_nsap_ptr_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub owner: *const ::core::ffi::c_char,
}
pub type dnskey_algorithm = ::core::ffi::c_uint;
pub const DNSKEYA_RSVP: dnskey_algorithm = 255;
pub const DNSKEYA_PRIVATEOID: dnskey_algorithm = 254;
pub const DNSKEYA_PRIVATEDNS: dnskey_algorithm = 253;
pub const DNSKEYA_INDIRECT: dnskey_algorithm = 252;
pub const DNSKEYA_RSASHA1: dnskey_algorithm = 5;
pub const DNSKEYA_ECC: dnskey_algorithm = 4;
pub const DNSKEYA_DSA: dnskey_algorithm = 3;
pub const DNSKEYA_DH: dnskey_algorithm = 2;
pub const DNSKEYA_RSAMD5: dnskey_algorithm = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_sig_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub covered: dns_type_t,
    pub algorithm: dnskey_algorithm,
    pub labels: ::core::ffi::c_int,
    pub originttl: TTL,
    pub sigexpire: ::core::ffi::c_ulong,
    pub timesigned: ::core::ffi::c_ulong,
    pub keyfootprint: uint16_t,
    pub signer: *const ::core::ffi::c_char,
    pub sigsize: size_t,
    pub signature: *mut uint8_t,
}
pub type dnskey_protocol = ::core::ffi::c_uint;
pub const DNSKEYP_ALL: dnskey_protocol = 255;
pub const DNSKEYP_IPSEC: dnskey_protocol = 4;
pub const DNSKEYP_DNSSEC: dnskey_protocol = 3;
pub const DNSKEYP_EMAIL: dnskey_protocol = 2;
pub const DNSKEYP_TLS: dnskey_protocol = 1;
pub const DNSKEYP_NONE: dnskey_protocol = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub union dnskey_key {
    pub md5: C2Rust_Unnamed_2,
    pub unknown: C2Rust_Unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_1 {
    pub size: size_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_2 {
    pub expsize: size_t,
    pub exponent: *mut uint8_t,
    pub modsize: size_t,
    pub modulus: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_key_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub flags: C2Rust_Unnamed_3,
    pub signatory: ::core::ffi::c_int,
    pub protocol: dnskey_protocol,
    pub algorithm: dnskey_algorithm,
    pub key: dnskey_key,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2Rust_Unnamed_3 {
    pub authentication: bool,
    pub confidential: bool,
    pub experimental: bool,
    pub user: bool,
    pub zone: bool,
    pub host: bool,
    pub ipsec: bool,
    pub email: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_px_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub map822: *const ::core::ffi::c_char,
    pub mapx400: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dnsgpos_angle {
    pub deg: ::core::ffi::c_int,
    pub min: ::core::ffi::c_int,
    pub sec: ::core::ffi::c_int,
    pub frac: ::core::ffi::c_int,
    pub nw: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_gpos_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub longitude: dnsgpos_angle,
    pub latitude: dnsgpos_angle,
    pub altitude: ::core::ffi::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_aaaa_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub address: in6_addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_loc_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub version: ::core::ffi::c_int,
    pub size: ::core::ffi::c_ulonglong,
    pub horiz_pre: ::core::ffi::c_ulonglong,
    pub vert_pre: ::core::ffi::c_ulonglong,
    pub latitude: dnsgpos_angle,
    pub longitude: dnsgpos_angle,
    pub altitude: ::core::ffi::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_nxt_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub next: *const ::core::ffi::c_char,
    pub numbits: size_t,
    pub bitmap: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_eid_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub rawdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_nimloc_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub rawdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_srv_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub priority: ::core::ffi::c_int,
    pub weight: ::core::ffi::c_int,
    pub port: ::core::ffi::c_int,
    pub target: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_atm_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub rawdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_naptr_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub order: ::core::ffi::c_int,
    pub preference: ::core::ffi::c_int,
    pub flags: *const ::core::ffi::c_char,
    pub services: *const ::core::ffi::c_char,
    pub regexp: *const ::core::ffi::c_char,
    pub replacement: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_kx_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub rawdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_cert_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub rawdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_a6_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub mask: size_t,
    pub address: in6_addr,
    pub prefixname: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_dname_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub rawdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_sink_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub rawdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edns0_opt_t {
    pub code: edns0_type_t,
    pub len: size_t,
    pub data: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_edns0opt_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub udp_payload: size_t,
    pub version: ::core::ffi::c_int,
    pub fdo: bool,
    pub fug: ::core::ffi::c_int,
    pub z: ::core::ffi::c_uint,
    pub numopts: size_t,
    pub opts: *mut edns0_opt_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dnsapl_record {
    pub addressfamily: ::core::ffi::c_int,
    pub prefix: ::core::ffi::c_int,
    pub afdlength: size_t,
    pub afdpart: *mut uint8_t,
    pub negate: bool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_apl_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub numrecs: size_t,
    pub recs: *mut dnsapl_record,
}
pub type dnsds_digest = ::core::ffi::c_uint;
pub const DNSDS_SHA1: dnsds_digest = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_ds_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub keytag: dnskey_protocol,
    pub algorithm: dnskey_algorithm,
    pub digest: dnsds_digest,
    pub digestlen: size_t,
    pub digestdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_rrsig_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub covered: dns_type_t,
    pub algorithm: dnskey_algorithm,
    pub labels: ::core::ffi::c_int,
    pub originttl: TTL,
    pub sigexpire: ::core::ffi::c_ulong,
    pub timesigned: ::core::ffi::c_ulong,
    pub keyfootprint: uint16_t,
    pub signer: *const ::core::ffi::c_char,
    pub sigsize: size_t,
    pub signature: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_nsec_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub next: *const ::core::ffi::c_char,
    pub numbits: size_t,
    pub bitmap: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_dnskey_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub zonekey: bool,
    pub sep: bool,
    pub protocol: dnskey_protocol,
    pub algorithm: dnskey_algorithm,
    pub keysize: size_t,
    pub key: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_spf_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub len: size_t,
    pub text: *const ::core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_tsig_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub algorithm: *const ::core::ffi::c_char,
    pub timesigned: uint64_t,
    pub fudge: ::core::ffi::c_uint,
    pub MACsize: size_t,
    pub MAC: *mut uint8_t,
    pub id: ::core::ffi::c_int,
    pub error: ::core::ffi::c_int,
    pub lenother: size_t,
    pub other: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_x_t {
    pub name: *const ::core::ffi::c_char,
    pub type_0: dns_type_t,
    pub class: dns_class_t,
    pub ttl: TTL,
    pub size: size_t,
    pub rawdata: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union dns_answer_t {
    pub generic: dns_generic_t,
    pub x: dns_x_t,
    pub a: dns_a_t,
    pub ns: dns_ns_t,
    pub md: dns_md_t,
    pub mf: dns_mf_t,
    pub cname: dns_cname_t,
    pub soa: dns_soa_t,
    pub mb: dns_mb_t,
    pub mg: dns_mg_t,
    pub mr: dns_mr_t,
    pub null: dns_null_t,
    pub wks: dns_wks_t,
    pub ptr: dns_ptr_t,
    pub hinfo: dns_hinfo_t,
    pub minfo: dns_minfo_t,
    pub mx: dns_mx_t,
    pub txt: dns_txt_t,
    pub rp: dns_rp_t,
    pub afsdb: dns_afsdb_t,
    pub x25: dns_x25_t,
    pub isdn: dns_isdn_t,
    pub rt: dns_rt_t,
    pub nsap: dns_nsap_t,
    pub nsap_ptr: dns_nsap_ptr_t,
    pub sig: dns_sig_t,
    pub key: dns_key_t,
    pub px: dns_px_t,
    pub gpos: dns_gpos_t,
    pub aaaa: dns_aaaa_t,
    pub loc: dns_loc_t,
    pub nxt: dns_nxt_t,
    pub eid: dns_eid_t,
    pub nimloc: dns_nimloc_t,
    pub srv: dns_srv_t,
    pub atm: dns_atm_t,
    pub naptr: dns_naptr_t,
    pub kx: dns_kx_t,
    pub cert: dns_cert_t,
    pub a6: dns_a6_t,
    pub dname: dns_dname_t,
    pub sink: dns_sink_t,
    pub opt: dns_edns0opt_t,
    pub apl: dns_apl_t,
    pub ds: dns_ds_t,
    pub rrsig: dns_rrsig_t,
    pub nsec: dns_nsec_t,
    pub dnskey: dns_dnskey_t,
    pub spf: dns_spf_t,
    pub tsig: dns_tsig_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dns_query_t {
    pub id: ::core::ffi::c_int,
    pub query: bool,
    pub opcode: dns_op_t,
    pub aa: bool,
    pub tc: bool,
    pub rd: bool,
    pub ra: bool,
    pub z: bool,
    pub ad: bool,
    pub cd: bool,
    pub rcode: dns_rcode_t,
    pub qdcount: size_t,
    pub ancount: size_t,
    pub nscount: size_t,
    pub arcount: size_t,
    pub questions: *mut dns_question_t,
    pub answers: *mut dns_answer_t,
    pub nameservers: *mut dns_answer_t,
    pub additional: *mut dns_answer_t,
}
pub type block__s = block;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct block {
    pub size: size_t,
    pub ptr: *mut uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct edns_context {
    pub packet: block__s,
    pub segments: segments__s,
    pub edns: bool,
    pub base: *mut uint8_t,
    pub rcode: dns_rcode_t,
}
pub type segments__s = segments;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segments {
    pub idx: size_t,
    pub seg: [segment; 128],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct segment {
    pub name: *const ::core::ffi::c_char,
    pub offset: size_t,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct idns_header {
    pub id: uint16_t,
    pub opcode: uint8_t,
    pub rcode: uint8_t,
    pub qdcount: uint16_t,
    pub ancount: uint16_t,
    pub nscount: uint16_t,
    pub arcount: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ddns_context {
    pub packet: block__s,
    pub parse: block__s,
    pub dest: block__s,
    pub response: *mut dns_query_t,
    pub edns: bool,
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int
        | (__bsx as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) << 8 as ::core::ffi::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn toupper(mut __c: ::core::ffi::c_int) -> ::core::ffi::c_int {
    return if __c >= -(128 as ::core::ffi::c_int) && __c < 256 as ::core::ffi::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize) as ::core::ffi::c_int
    } else {
        __c
    };
}
pub const INT32_MAX: ::core::ffi::c_int = 2147483647 as ::core::ffi::c_int;
pub const MAX_DOMAIN_LABEL: ::core::ffi::c_int = 64 as ::core::ffi::c_int;
pub const MEM_ALIGN: usize = ::core::mem::size_of::<dns_decoded_t>();
pub const MEM_MASK: usize =
    !(::core::mem::size_of::<dns_decoded_t>() as usize).wrapping_sub(1 as usize);
pub const MAXSEG: ::core::ffi::c_int = 128 as ::core::ffi::c_int;
pub const LOC_BIAS: ::core::ffi::c_ulong =
    (INT32_MAX as ::core::ffi::c_ulong).wrapping_add(1 as ::core::ffi::c_ulong);
pub const LOC_LAT_MAX: ::core::ffi::c_ulong =
    (90 as ::core::ffi::c_ulong).wrapping_mul(3600000 as ::core::ffi::c_ulong);
pub const LOC_LNG_MAX: ::core::ffi::c_ulong =
    (180 as ::core::ffi::c_ulong).wrapping_mul(3600000 as ::core::ffi::c_ulong);
pub const LOC_ALT_BIAS: ::core::ffi::c_long = 10000000 as ::core::ffi::c_long;
unsafe extern "C" fn query_okay(mut query: *const dns_query_t) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if !query.is_null() {
        } else {
            __assert_fail(
                b"query != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                171 as ::core::ffi::c_uint,
                b"int query_okay(const dns_query_t *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*query).id >= 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"query->id >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                172 as ::core::ffi::c_uint,
                b"int query_okay(const dns_query_t *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*query).id <= 65535 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"query->id <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                173 as ::core::ffi::c_uint,
                b"int query_okay(const dns_query_t *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if (*query).opcode as ::core::ffi::c_uint <= 5 as ::core::ffi::c_uint {
        } else {
            __assert_fail(
                b"query->opcode <= 5\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                174 as ::core::ffi::c_uint,
                b"int query_okay(const dns_query_t *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if (*query).rcode as ::core::ffi::c_uint <= 15 as ::core::ffi::c_uint {
        } else {
            __assert_fail(
                b"query->rcode <= 15\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                175 as ::core::ffi::c_uint,
                b"int query_okay(const dns_query_t *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if (*query).qdcount <= 65535 as size_t {
        } else {
            __assert_fail(
                b"query->qdcount <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                176 as ::core::ffi::c_uint,
                b"int query_okay(const dns_query_t *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_5: {
        if (*query).ancount <= 65535 as size_t {
        } else {
            __assert_fail(
                b"query->ancount <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                177 as ::core::ffi::c_uint,
                b"int query_okay(const dns_query_t *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_6: {
        if (*query).nscount <= 65535 as size_t {
        } else {
            __assert_fail(
                b"query->nscount <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                178 as ::core::ffi::c_uint,
                b"int query_okay(const dns_query_t *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_7: {
        if (*query).arcount <= 65535 as size_t {
        } else {
            __assert_fail(
                b"query->arcount <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                179 as ::core::ffi::c_uint,
                b"int query_okay(const dns_query_t *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn pblock_okay(mut block: *const block__s) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if !block.is_null() {
        } else {
            __assert_fail(
                b"block != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                185 as ::core::ffi::c_uint,
                b"int pblock_okay(const block__s *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !(*block).ptr.is_null() {
        } else {
            __assert_fail(
                b"block->ptr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                186 as ::core::ffi::c_uint,
                b"int pblock_okay(const block__s *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*block).size > 0 as size_t {
        } else {
            __assert_fail(
                b"block->size > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                187 as ::core::ffi::c_uint,
                b"int pblock_okay(const block__s *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn block_okay(block: block__s) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if !block.ptr.is_null() {
        } else {
            __assert_fail(
                b"block.ptr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                193 as ::core::ffi::c_uint,
                b"int block_okay(const block__s)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if block.size > 0 as size_t {
        } else {
            __assert_fail(
                b"block.size > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                194 as ::core::ffi::c_uint,
                b"int block_okay(const block__s)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn econtext_okay(mut data: *const edns_context) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if !data.is_null() {
        } else {
            __assert_fail(
                b"data != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                200 as ::core::ffi::c_uint,
                b"int econtext_okay(const edns_context *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if block_okay((*data).packet) != 0 {
        } else {
            __assert_fail(
                b"block_okay(data->packet)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                201 as ::core::ffi::c_uint,
                b"int econtext_okay(const edns_context *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !(*data).base.is_null() {
        } else {
            __assert_fail(
                b"data->base != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                202 as ::core::ffi::c_uint,
                b"int econtext_okay(const edns_context *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    return 1 as ::core::ffi::c_int;
}
unsafe extern "C" fn dcontext_okay(mut data: *const ddns_context) -> ::core::ffi::c_int {
    '_c2rust_label: {
        if !data.is_null() {
        } else {
            __assert_fail(
                b"data != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                208 as ::core::ffi::c_uint,
                b"int dcontext_okay(const ddns_context *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !(*data).response.is_null() {
        } else {
            __assert_fail(
                b"data->response != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                209 as ::core::ffi::c_uint,
                b"int dcontext_okay(const ddns_context *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if block_okay((*data).packet) != 0 {
        } else {
            __assert_fail(
                b"block_okay(data->packet)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                210 as ::core::ffi::c_uint,
                b"int dcontext_okay(const ddns_context *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if block_okay((*data).parse) != 0 {
        } else {
            __assert_fail(
                b"block_okay(data->parse)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                211 as ::core::ffi::c_uint,
                b"int dcontext_okay(const ddns_context *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if block_okay((*data).dest) != 0 {
        } else {
            __assert_fail(
                b"block_okay(data->dest)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                212 as ::core::ffi::c_uint,
                b"int dcontext_okay(const ddns_context *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    return 1 as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn write_uint16(mut parse: *mut block__s, mut value: uint16_t) {
    '_c2rust_label: {
        if pblock_okay(parse) != 0 {
        } else {
            __assert_fail(
                b"pblock_okay(parse)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                221 as ::core::ffi::c_uint,
                b"void write_uint16(block__s *, uint16_t)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*parse).size >= 2 as size_t {
        } else {
            __assert_fail(
                b"parse->size >= 2\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                222 as ::core::ffi::c_uint,
                b"void write_uint16(block__s *, uint16_t)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    *(*parse).ptr.offset(0 as ::core::ffi::c_int as isize) =
        (value as ::core::ffi::c_int >> 8 as ::core::ffi::c_int & 0xff as ::core::ffi::c_int)
            as uint8_t;
    *(*parse).ptr.offset(1 as ::core::ffi::c_int as isize) =
        (value as ::core::ffi::c_int & 0xff as ::core::ffi::c_int) as uint8_t;
    (*parse).ptr = (*parse).ptr.offset(2 as ::core::ffi::c_int as isize);
    (*parse).size = (*parse).size.wrapping_sub(2 as size_t);
}
#[inline]
unsafe extern "C" fn write_uint32(mut parse: *mut block__s, mut value: uint32_t) {
    '_c2rust_label: {
        if pblock_okay(parse) != 0 {
        } else {
            __assert_fail(
                b"pblock_okay(parse)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                234 as ::core::ffi::c_uint,
                b"void write_uint32(block__s *, uint32_t)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*parse).size >= 4 as size_t {
        } else {
            __assert_fail(
                b"parse->size >= 4\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                235 as ::core::ffi::c_uint,
                b"void write_uint32(block__s *, uint32_t)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    *(*parse).ptr.offset(0 as ::core::ffi::c_int as isize) =
        (value >> 24 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    *(*parse).ptr.offset(1 as ::core::ffi::c_int as isize) =
        (value >> 16 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    *(*parse).ptr.offset(2 as ::core::ffi::c_int as isize) =
        (value >> 8 as ::core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    *(*parse).ptr.offset(3 as ::core::ffi::c_int as isize) = (value & 0xff as uint32_t) as uint8_t;
    (*parse).ptr = (*parse).ptr.offset(4 as ::core::ffi::c_int as isize);
    (*parse).size = (*parse).size.wrapping_sub(4 as size_t);
}
unsafe extern "C" fn segment_find(
    mut src: *const ::core::ffi::c_char,
    mut seg: *const segments__s,
) -> *const segment {
    '_c2rust_label: {
        if !src.is_null() {
        } else {
            __assert_fail(
                b"src != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                249 as ::core::ffi::c_uint,
                b"const struct segment *segment_find(const char *, const segments__s *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !seg.is_null() {
        } else {
            __assert_fail(
                b"seg != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                250 as ::core::ffi::c_uint,
                b"const struct segment *segment_find(const char *, const segments__s *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    let mut i: size_t = 0 as size_t;
    while i < (*seg).idx {
        if strcmp(src, (*seg).seg[i as usize].name) == 0 as ::core::ffi::c_int {
            return (&raw const (*seg).seg as *const segment).offset(i as isize) as *const segment;
        }
        i = i.wrapping_add(1);
    }
    return ::core::ptr::null::<segment>();
}
unsafe extern "C" fn encode_segment(
    mut psrc: *mut *const ::core::ffi::c_char,
    mut base: *const uint8_t,
    mut block: *mut block__s,
    mut offset: *mut size_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if !psrc.is_null() {
        } else {
            __assert_fail(
                b"psrc != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                263 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_segment(const char **, const uint8_t *, block__s *, size_t *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !(*psrc).is_null() {
        } else {
            __assert_fail(
                b"*psrc != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                264 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_segment(const char **, const uint8_t *, block__s *, size_t *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !base.is_null() {
        } else {
            __assert_fail(
                b"base != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                265 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_segment(const char **, const uint8_t *, block__s *, size_t *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if !block.is_null() {
        } else {
            __assert_fail(
                b"block != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                266 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_segment(const char **, const uint8_t *, block__s *, size_t *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if !offset.is_null() {
        } else {
            __assert_fail(
                b"offset != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                267 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_segment(const char **, const uint8_t *, block__s *, size_t *)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    let mut p: *mut ::core::ffi::c_char = strchr(*psrc, '.' as i32);
    if p.is_null() {
        return RCODE_NAME_ERROR;
    }
    let mut len: size_t = p.offset_from(*psrc) as ::core::ffi::c_long as size_t;
    if len >= MAX_DOMAIN_LABEL as size_t {
        return RCODE_NAME_ERROR;
    }
    if (*block).size < len.wrapping_add(1 as size_t) {
        return RCODE_NO_MEMORY;
    }
    *offset = (*block).ptr.offset_from(base) as ::core::ffi::c_long as size_t;
    let c2rust_fresh4 = (*block).ptr;
    (*block).ptr = (*block).ptr.offset(1);
    *c2rust_fresh4 = len as uint8_t;
    memcpy(
        (*block).ptr as *mut ::core::ffi::c_void,
        *psrc as *const ::core::ffi::c_void,
        len,
    );
    (*block).ptr = (*block).ptr.offset(len as isize);
    (*block).size = (*block).size.wrapping_sub(len.wrapping_add(1 as size_t));
    *psrc = p.offset(1 as ::core::ffi::c_int as isize);
    return RCODE_OKAY;
}
unsafe extern "C" fn encode_domain(
    mut data: *mut edns_context,
    mut name: *const ::core::ffi::c_char,
) -> dns_rcode_t {
    let mut segment: *const segment = ::core::ptr::null::<segment>();
    let mut rc: dns_rcode_t = RCODE_OKAY;
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                304 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_domain(edns_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !name.is_null() {
        } else {
            __assert_fail(
                b"name != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                305 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_domain(edns_context *, const char *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    while *name as ::core::ffi::c_int != '.' as i32 && *name as ::core::ffi::c_int != '\0' as i32 {
        '_c2rust_label_1: {
            if *name as ::core::ffi::c_int != '.' as i32 {
            } else {
                __assert_fail(
                    b"*name != '.'\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                    309 as ::core::ffi::c_uint,
                    b"dns_rcode_t encode_domain(edns_context *, const char *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        segment = segment_find(name, &raw mut (*data).segments);
        if segment.is_null() {
            if (*data).segments.idx == MAXSEG as size_t {
                return RCODE_NO_MEMORY;
            }
            (*data).segments.seg[(*data).segments.idx as usize].name = name;
            rc = encode_segment(
                &raw mut name,
                (*data).base,
                &raw mut (*data).packet,
                &raw mut (*(&raw mut (*data).segments.seg as *mut segment)
                    .offset((*data).segments.idx as isize))
                .offset,
            );
            if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return rc;
            }
            (*data).segments.idx = (*data).segments.idx.wrapping_add(1);
        } else {
            if (*data).packet.size < 2 as size_t {
                return RCODE_NO_MEMORY;
            }
            let c2rust_fresh1 = (*data).packet.ptr;
            (*data).packet.ptr = (*data).packet.ptr.offset(1);
            *c2rust_fresh1 = (((*segment).offset >> 8 as ::core::ffi::c_int) as uint8_t
                as ::core::ffi::c_int
                | 0xc0 as ::core::ffi::c_int) as uint8_t;
            let c2rust_fresh2 = (*data).packet.ptr;
            (*data).packet.ptr = (*data).packet.ptr.offset(1);
            *c2rust_fresh2 = (*segment).offset as uint8_t;
            (*data).packet.size = (*data).packet.size.wrapping_sub(2 as size_t);
            return RCODE_OKAY;
        }
    }
    if (*data).packet.size == 0 as size_t {
        return RCODE_NO_MEMORY;
    }
    let c2rust_fresh3 = (*data).packet.ptr;
    (*data).packet.ptr = (*data).packet.ptr.offset(1);
    *c2rust_fresh3 = 0 as uint8_t;
    (*data).packet.size = (*data).packet.size.wrapping_sub(1);
    return RCODE_OKAY;
}
unsafe extern "C" fn encode_string(
    mut data: *mut edns_context,
    mut text: *const ::core::ffi::c_char,
    mut size: size_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                351 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_string(edns_context *, const char *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !text.is_null() {
        } else {
            __assert_fail(
                b"text != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                352 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_string(edns_context *, const char *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if size > 255 as size_t {
        return RCODE_BAD_STRING;
    }
    if (*data).packet.size < size.wrapping_add(1 as size_t) {
        return RCODE_NO_MEMORY;
    }
    let c2rust_fresh5 = (*data).packet.ptr;
    (*data).packet.ptr = (*data).packet.ptr.offset(1);
    *c2rust_fresh5 = size as uint8_t;
    memcpy(
        (*data).packet.ptr as *mut ::core::ffi::c_void,
        text as *const ::core::ffi::c_void,
        size,
    );
    (*data).packet.ptr = (*data).packet.ptr.offset(size as isize);
    (*data).packet.size = (*data)
        .packet
        .size
        .wrapping_sub(size.wrapping_add(1 as size_t));
    return RCODE_OKAY;
}
unsafe extern "C" fn encode_question(
    mut data: *mut edns_context,
    mut pquestion: *const dns_question_t,
) -> dns_rcode_t {
    let mut rc: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                374 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_question(edns_context *, const dns_question_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !pquestion.is_null() {
        } else {
            __assert_fail(
                b"pquestion != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                375 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_question(edns_context *, const dns_question_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !(*pquestion).name.is_null() {
        } else {
            __assert_fail(
                b"pquestion->name != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                376 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_question(edns_context *, const dns_question_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if (*pquestion).type_0 as ::core::ffi::c_uint
            != RR_OPT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"pquestion->type != RR_OPT\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                377 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_question(edns_context *, const dns_question_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if (*pquestion).class as ::core::ffi::c_uint >= 1 as ::core::ffi::c_uint {
        } else {
            __assert_fail(
                b"pquestion->class >= 1\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                378 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_question(edns_context *, const dns_question_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if (*pquestion).class as ::core::ffi::c_uint <= 4 as ::core::ffi::c_uint {
        } else {
            __assert_fail(
                b"pquestion->class <= 4\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                379 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_question(edns_context *, const dns_question_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    rc = encode_domain(data, (*pquestion).name) as ::core::ffi::c_int;
    if rc != RCODE_OKAY as ::core::ffi::c_int {
        return rc as dns_rcode_t;
    }
    if (*data).packet.size < 4 as size_t {
        return RCODE_NO_MEMORY;
    }
    write_uint16(&raw mut (*data).packet, (*pquestion).type_0 as uint16_t);
    write_uint16(&raw mut (*data).packet, (*pquestion).class as uint16_t);
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn encode_rr_a(
    mut data: *mut edns_context,
    mut a: *const dns_a_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                398 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_a(edns_context *, const dns_a_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !a.is_null() {
        } else {
            __assert_fail(
                b"a != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                399 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_a(edns_context *, const dns_a_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*data).packet.size < 4 as size_t {
        return RCODE_NO_MEMORY;
    }
    memcpy(
        (*data).packet.ptr as *mut ::core::ffi::c_void,
        &raw const (*a).address as *const ::core::ffi::c_void,
        4 as size_t,
    );
    (*data).packet.ptr = (*data).packet.ptr.offset(4 as ::core::ffi::c_int as isize);
    (*data).packet.size = (*data).packet.size.wrapping_sub(4 as size_t);
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn encode_rr_soa(
    mut data: *mut edns_context,
    mut soa: *const dns_soa_t,
) -> dns_rcode_t {
    let mut rc: dns_rcode_t = RCODE_OKAY;
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                416 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_soa(edns_context *, const dns_soa_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !soa.is_null() {
        } else {
            __assert_fail(
                b"soa != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                417 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_soa(edns_context *, const dns_soa_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !(*soa).mname.is_null() {
        } else {
            __assert_fail(
                b"soa->mname != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                418 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_soa(edns_context *, const dns_soa_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if !(*soa).rname.is_null() {
        } else {
            __assert_fail(
                b"soa->rname != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                419 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_soa(edns_context *, const dns_soa_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    rc = encode_domain(data, (*soa).mname);
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    rc = encode_domain(data, (*soa).rname);
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    if (*data).packet.size < 20 as size_t {
        return RCODE_NO_MEMORY;
    }
    write_uint32(&raw mut (*data).packet, (*soa).serial);
    write_uint32(&raw mut (*data).packet, (*soa).refresh);
    write_uint32(&raw mut (*data).packet, (*soa).retry);
    write_uint32(&raw mut (*data).packet, (*soa).expire);
    write_uint32(&raw mut (*data).packet, (*soa).minimum);
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn encode_rr_aaaa(
    mut data: *mut edns_context,
    mut aaaa: *const dns_aaaa_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                440 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_aaaa(edns_context *, const dns_aaaa_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !aaaa.is_null() {
        } else {
            __assert_fail(
                b"aaaa != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                441 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_aaaa(edns_context *, const dns_aaaa_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*data).packet.size < 16 as size_t {
        return RCODE_NO_MEMORY;
    }
    memcpy(
        (*data).packet.ptr as *mut ::core::ffi::c_void,
        &raw const (*aaaa).address as *const ::core::ffi::c_void,
        16 as size_t,
    );
    (*data).packet.ptr = (*data).packet.ptr.offset(16 as ::core::ffi::c_int as isize);
    (*data).packet.size = (*data).packet.size.wrapping_sub(16 as size_t);
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn encode_rr_srv(
    mut data: *mut edns_context,
    mut srv: *const dns_srv_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                456 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_srv(edns_context *, const dns_srv_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !srv.is_null() {
        } else {
            __assert_fail(
                b"srv != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                457 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_srv(edns_context *, const dns_srv_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*srv).priority <= 65535 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"srv->priority <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                458 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_srv(edns_context *, const dns_srv_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if (*srv).weight <= 65535 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"srv->weight <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                459 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_srv(edns_context *, const dns_srv_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if (*srv).port <= 65535 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"srv->port <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                460 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_srv(edns_context *, const dns_srv_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if !(*srv).target.is_null() {
        } else {
            __assert_fail(
                b"srv->target != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                461 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_srv(edns_context *, const dns_srv_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*data).packet.size < 7 as size_t {
        return RCODE_NO_MEMORY;
    }
    write_uint16(&raw mut (*data).packet, (*srv).priority as uint16_t);
    write_uint16(&raw mut (*data).packet, (*srv).weight as uint16_t);
    write_uint16(&raw mut (*data).packet, (*srv).port as uint16_t);
    return encode_domain(data, (*srv).target);
}
#[inline]
unsafe extern "C" fn encode_rr_wks(
    mut data: *mut edns_context,
    mut wks: *const dns_wks_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                476 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_wks(edns_context *, const dns_wks_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !wks.is_null() {
        } else {
            __assert_fail(
                b"wks != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                477 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_wks(edns_context *, const dns_wks_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*wks).protocol <= 65535 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"wks->protocol <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                478 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_wks(edns_context *, const dns_wks_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if (*wks).numbits <= 8192 as size_t {
        } else {
            __assert_fail(
                b"wks->numbits <= 8192\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                479 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_wks(edns_context *, const dns_wks_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if !(*wks).bits.is_null() {
        } else {
            __assert_fail(
                b"wks->bits != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                480 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_wks(edns_context *, const dns_wks_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*data).packet.size < (*wks).numbits.wrapping_add(6 as size_t) {
        return RCODE_NO_MEMORY;
    }
    memcpy(
        (*data).packet.ptr as *mut ::core::ffi::c_void,
        &raw const (*wks).address as *const ::core::ffi::c_void,
        4 as size_t,
    );
    (*data).packet.ptr = (*data).packet.ptr.offset(4 as ::core::ffi::c_int as isize);
    (*data).packet.size = (*data).packet.size.wrapping_sub(4 as size_t);
    write_uint16(&raw mut (*data).packet, (*wks).protocol as uint16_t);
    memcpy(
        (*data).packet.ptr as *mut ::core::ffi::c_void,
        (*wks).bits as *const ::core::ffi::c_void,
        (*wks).numbits,
    );
    (*data).packet.ptr = (*data).packet.ptr.offset((*wks).numbits as isize);
    (*data).packet.size = (*data).packet.size.wrapping_sub((*wks).numbits);
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn encode_rr_gpos(
    mut data: *mut edns_context,
    mut gpos: *const dns_gpos_t,
) -> dns_rcode_t {
    let mut rc: dns_rcode_t = RCODE_OKAY;
    let mut lat: ::core::ffi::c_double = 0.;
    let mut lng: ::core::ffi::c_double = 0.;
    let mut text: [::core::ffi::c_char; 12] = [0; 12];
    let mut textlen: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                505 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_gpos(edns_context *, const dns_gpos_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !gpos.is_null() {
        } else {
            __assert_fail(
                b"gpos != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                506 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_gpos(edns_context *, const dns_gpos_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*gpos).longitude.deg <= 180 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"gpos->longitude.deg <= 180\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                507 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_gpos(edns_context *, const dns_gpos_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if (*gpos).longitude.min < 60 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"gpos->longitude.min < 60\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                508 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_gpos(edns_context *, const dns_gpos_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if (*gpos).longitude.sec < 60 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"gpos->longitude.sec < 60\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                509 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_gpos(edns_context *, const dns_gpos_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if (*gpos).longitude.frac < 1000 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"gpos->longitude.frac < 1000\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                510 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_gpos(edns_context *, const dns_gpos_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_5: {
        if (*gpos).latitude.deg <= 90 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"gpos->latitude.deg <= 90\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                511 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_gpos(edns_context *, const dns_gpos_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_6: {
        if (*gpos).latitude.min < 60 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"gpos->latitude.min < 60\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                512 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_gpos(edns_context *, const dns_gpos_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_7: {
        if (*gpos).latitude.sec < 60 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"gpos->latitude.sec < 60\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                513 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_gpos(edns_context *, const dns_gpos_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_8: {
        if (*gpos).latitude.frac < 1000 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"gpos->latitude.frac < 1000\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                514 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_gpos(edns_context *, const dns_gpos_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    lat = (*gpos).latitude.deg as ::core::ffi::c_double
        + (*gpos).latitude.min as ::core::ffi::c_double / 60.0f64
        + (*gpos).latitude.sec as ::core::ffi::c_double / 3600.0f64
        + (*gpos).latitude.frac as ::core::ffi::c_double / 3600000.0f64;
    if !(*gpos).latitude.nw {
        lat = -lat;
    }
    lng = (*gpos).longitude.deg as ::core::ffi::c_double
        + (*gpos).longitude.min as ::core::ffi::c_double / 60.0f64
        + (*gpos).longitude.sec as ::core::ffi::c_double / 3600.0f64
        + (*gpos).longitude.frac as ::core::ffi::c_double / 3600000.0f64;
    if (*gpos).longitude.nw {
        lng = -lng;
    }
    textlen = snprintf(
        &raw mut text as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t,
        b"%f\0".as_ptr() as *const ::core::ffi::c_char,
        lng,
    );
    rc = encode_string(
        data,
        &raw mut text as *mut ::core::ffi::c_char,
        textlen as size_t,
    );
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    textlen = snprintf(
        &raw mut text as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t,
        b"%f\0".as_ptr() as *const ::core::ffi::c_char,
        lat,
    );
    rc = encode_string(
        data,
        &raw mut text as *mut ::core::ffi::c_char,
        textlen as size_t,
    );
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    textlen = snprintf(
        &raw mut text as *mut ::core::ffi::c_char,
        ::core::mem::size_of::<[::core::ffi::c_char; 12]>() as size_t,
        b"%f\0".as_ptr() as *const ::core::ffi::c_char,
        (*gpos).altitude,
    );
    return encode_string(
        data,
        &raw mut text as *mut ::core::ffi::c_char,
        textlen as size_t,
    );
}
unsafe extern "C" fn eloc_scale(
    mut scale: ::core::ffi::c_ulonglong,
    mut def: ::core::ffi::c_ulong,
) -> uint8_t {
    let mut ip: ::core::ffi::c_double = 0.;
    let mut rs: ::core::ffi::c_double = 0.;
    let mut smul: ::core::ffi::c_int = 0;
    let mut spow: ::core::ffi::c_int = 0;
    if scale == 0 as ::core::ffi::c_ulonglong {
        scale = def as ::core::ffi::c_ulonglong;
    }
    modf(log10(scale as ::core::ffi::c_double), &raw mut ip);
    rs = pow(10.0f64, ip);
    smul = (scale as ::core::ffi::c_double / rs) as ::core::ffi::c_int;
    spow = ip as ::core::ffi::c_int;
    '_c2rust_label: {
        if smul >= 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"smul >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                564 as ::core::ffi::c_uint,
                b"uint8_t eloc_scale(unsigned long long, unsigned long)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if smul <= 9 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"smul <= 9\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                565 as ::core::ffi::c_uint,
                b"uint8_t eloc_scale(unsigned long long, unsigned long)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if spow >= 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"spow >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                566 as ::core::ffi::c_uint,
                b"uint8_t eloc_scale(unsigned long long, unsigned long)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if spow <= 9 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"spow <= 9\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                567 as ::core::ffi::c_uint,
                b"uint8_t eloc_scale(unsigned long long, unsigned long)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    return (smul << 4 as ::core::ffi::c_int | spow) as uint8_t;
}
#[inline]
unsafe extern "C" fn encode_rr_loc(
    mut data: *mut edns_context,
    mut loc: *const dns_loc_t,
) -> dns_rcode_t {
    let mut v: uint32_t = 0;
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                578 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_loc(edns_context *, const dns_loc_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !loc.is_null() {
        } else {
            __assert_fail(
                b"loc != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                579 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_loc(edns_context *, const dns_loc_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*loc).size <= 9000000000 as ::core::ffi::c_ulonglong {
        } else {
            __assert_fail(
                b"loc->size <= 9000000000uLL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                580 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_loc(edns_context *, const dns_loc_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if (*loc).horiz_pre <= 9000000000 as ::core::ffi::c_ulonglong {
        } else {
            __assert_fail(
                b"loc->horiz_pre <= 9000000000uLL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                581 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_loc(edns_context *, const dns_loc_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if (*loc).vert_pre <= 9000000000 as ::core::ffi::c_ulonglong {
        } else {
            __assert_fail(
                b"loc->vert_pre <= 9000000000uLL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                582 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_loc(edns_context *, const dns_loc_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if (*loc).latitude.deg <= 180 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"loc->latitude.deg <= 180\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                583 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_loc(edns_context *, const dns_loc_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_5: {
        if (*loc).longitude.min < 60 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"loc->longitude.min < 60\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                584 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_loc(edns_context *, const dns_loc_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_6: {
        if (*loc).longitude.sec < 60 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"loc->longitude.sec < 60\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                585 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_loc(edns_context *, const dns_loc_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_7: {
        if (*loc).longitude.frac < 1000 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"loc->longitude.frac < 1000\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                586 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_loc(edns_context *, const dns_loc_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_8: {
        if (*loc).latitude.deg <= 90 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"loc->latitude.deg <= 90\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                587 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_loc(edns_context *, const dns_loc_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_9: {
        if (*loc).latitude.min < 60 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"loc->latitude.min < 60\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                588 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_loc(edns_context *, const dns_loc_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_10: {
        if (*loc).latitude.sec < 60 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"loc->latitude.sec < 60\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                589 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_loc(edns_context *, const dns_loc_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_11: {
        if (*loc).latitude.frac < 1000 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"loc->latitude.frac < 1000\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                590 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_loc(edns_context *, const dns_loc_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*data).packet.size < 16 as size_t {
        return RCODE_NO_MEMORY;
    }
    let c2rust_fresh6 = (*data).packet.ptr;
    (*data).packet.ptr = (*data).packet.ptr.offset(1);
    *c2rust_fresh6 = 0 as uint8_t;
    let c2rust_fresh7 = (*data).packet.ptr;
    (*data).packet.ptr = (*data).packet.ptr.offset(1);
    *c2rust_fresh7 = eloc_scale((*loc).size, 100 as ::core::ffi::c_ulong);
    let c2rust_fresh8 = (*data).packet.ptr;
    (*data).packet.ptr = (*data).packet.ptr.offset(1);
    *c2rust_fresh8 = eloc_scale((*loc).horiz_pre, 1000000 as ::core::ffi::c_ulong);
    let c2rust_fresh9 = (*data).packet.ptr;
    (*data).packet.ptr = (*data).packet.ptr.offset(1);
    *c2rust_fresh9 = eloc_scale((*loc).vert_pre, 1000 as ::core::ffi::c_ulong);
    v = ((*loc).latitude.deg as ::core::ffi::c_ulong)
        .wrapping_mul(3600000 as ::core::ffi::c_ulong)
        .wrapping_add(
            ((*loc).latitude.min as ::core::ffi::c_ulong)
                .wrapping_mul(60000 as ::core::ffi::c_ulong),
        )
        .wrapping_add(
            ((*loc).latitude.sec as ::core::ffi::c_ulong)
                .wrapping_mul(1000 as ::core::ffi::c_ulong),
        )
        .wrapping_add((*loc).latitude.frac as ::core::ffi::c_ulong) as uint32_t;
    '_c2rust_label_12: {
        if v as ::core::ffi::c_ulong
            <= (90 as ::core::ffi::c_ulong).wrapping_mul(3600000 as ::core::ffi::c_ulong)
        {
        } else {
            __assert_fail(
                b"v <= LOC_LAT_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                605 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_loc(edns_context *, const dns_loc_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*loc).latitude.nw {
        v = (v as ::core::ffi::c_ulong).wrapping_add(LOC_BIAS) as uint32_t as uint32_t;
    } else {
        v = LOC_BIAS.wrapping_sub(v as ::core::ffi::c_ulong) as uint32_t;
    }
    write_uint32(&raw mut (*data).packet, v);
    v = ((*loc).longitude.deg as ::core::ffi::c_ulong)
        .wrapping_mul(3600000 as ::core::ffi::c_ulong)
        .wrapping_add(
            ((*loc).longitude.min as ::core::ffi::c_ulong)
                .wrapping_mul(60000 as ::core::ffi::c_ulong),
        )
        .wrapping_add(
            ((*loc).longitude.sec as ::core::ffi::c_ulong)
                .wrapping_mul(1000 as ::core::ffi::c_ulong),
        )
        .wrapping_add((*loc).longitude.frac as ::core::ffi::c_ulong) as uint32_t;
    '_c2rust_label_13: {
        if v as ::core::ffi::c_ulong
            <= (180 as ::core::ffi::c_ulong).wrapping_mul(3600000 as ::core::ffi::c_ulong)
        {
        } else {
            __assert_fail(
                b"v <= LOC_LNG_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                618 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_loc(edns_context *, const dns_loc_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if !(*loc).longitude.nw {
        v = (v as ::core::ffi::c_ulong).wrapping_add(LOC_BIAS) as uint32_t as uint32_t;
    } else {
        v = LOC_BIAS.wrapping_sub(v as ::core::ffi::c_ulong) as uint32_t;
    }
    write_uint32(&raw mut (*data).packet, v);
    write_uint32(
        &raw mut (*data).packet,
        ((*loc).altitude as ::core::ffi::c_uint as ::core::ffi::c_long + LOC_ALT_BIAS) as uint32_t,
    );
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn encode_edns0rr_nsid(
    mut data: *mut edns_context,
    mut opt: *const edns0_opt_t,
) -> dns_rcode_t {
    let mut newlen: size_t = 0;
    let mut nidx: size_t = 0;
    let mut i: size_t = 0;
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                641 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_edns0rr_nsid(edns_context *, const edns0_opt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !opt.is_null() {
        } else {
            __assert_fail(
                b"opt != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                642 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_edns0rr_nsid(edns_context *, const edns0_opt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*opt).code as ::core::ffi::c_uint
            == EDNS0RR_NSID as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"opt->code == EDNS0RR_NSID\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                643 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_edns0rr_nsid(edns_context *, const edns0_opt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if (*opt).len <= 65535 as size_t {
        } else {
            __assert_fail(
                b"opt->len <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                644 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_edns0rr_nsid(edns_context *, const edns0_opt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    newlen = (*opt).len.wrapping_mul(2 as size_t);
    if (*data).packet.size
        < newlen
            .wrapping_add(::core::mem::size_of::<uint16_t>() as size_t)
            .wrapping_add(::core::mem::size_of::<uint16_t>() as size_t)
    {
        return RCODE_NO_MEMORY;
    }
    write_uint16(&raw mut (*data).packet, (*opt).code as uint16_t);
    write_uint16(&raw mut (*data).packet, newlen as uint16_t);
    nidx = 0 as size_t;
    i = nidx;
    while i < (*opt).len {
        sprintf(
            (*data).packet.ptr.offset(nidx as isize) as *mut uint8_t as *mut ::core::ffi::c_char,
            b"%02X\0".as_ptr() as *const ::core::ffi::c_char,
            *(*opt).data.offset(i as isize) as ::core::ffi::c_int,
        );
        i = i.wrapping_add(1);
        nidx = nidx.wrapping_add(2 as size_t);
    }
    (*data).packet.ptr = (*data).packet.ptr.offset(newlen as isize);
    (*data).packet.size = (*data).packet.size.wrapping_sub(newlen);
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn encode_edns0rr_raw(
    mut data: *mut edns_context,
    mut opt: *const edns0_opt_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                674 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_edns0rr_raw(edns_context *, const edns0_opt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !opt.is_null() {
        } else {
            __assert_fail(
                b"opt != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                675 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_edns0rr_raw(edns_context *, const edns0_opt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*opt).code as ::core::ffi::c_uint <= 65535 as ::core::ffi::c_uint {
        } else {
            __assert_fail(
                b"opt->code <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                676 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_edns0rr_raw(edns_context *, const edns0_opt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if (*opt).len <= 65535 as size_t {
        } else {
            __assert_fail(
                b"opt->len <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                677 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_edns0rr_raw(edns_context *, const edns0_opt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*data).packet.size
        < (*opt)
            .len
            .wrapping_add(::core::mem::size_of::<uint16_t>() as size_t)
            .wrapping_add(::core::mem::size_of::<uint16_t>() as size_t)
    {
        return RCODE_NO_MEMORY;
    }
    write_uint16(&raw mut (*data).packet, (*opt).code as uint16_t);
    write_uint16(&raw mut (*data).packet, (*opt).len as uint16_t);
    memcpy(
        (*data).packet.ptr as *mut ::core::ffi::c_void,
        (*opt).data as *const ::core::ffi::c_void,
        (*opt).len,
    );
    (*data).packet.ptr = (*data).packet.ptr.offset((*opt).len as isize);
    (*data).packet.size = (*data).packet.size.wrapping_sub((*opt).len);
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn encode_rr_opt(
    mut data: *mut edns_context,
    mut opt: *const dns_edns0opt_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                697 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_opt(edns_context *, const dns_edns0opt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !opt.is_null() {
        } else {
            __assert_fail(
                b"opt != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                698 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_opt(edns_context *, const dns_edns0opt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*opt).class as size_t == (*opt).udp_payload {
        } else {
            __assert_fail(
                b"opt->class == opt->udp_payload\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                699 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_opt(edns_context *, const dns_edns0opt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if (*opt).version == 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"opt->version == 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                700 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_opt(edns_context *, const dns_edns0opt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if (*opt).udp_payload <= 65535 as size_t {
        } else {
            __assert_fail(
                b"opt->udp_payload <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                701 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_opt(edns_context *, const dns_edns0opt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*data).edns {
        return RCODE_FORMAT_ERROR;
    }
    if (*data).packet.size < 11 as size_t {
        return RCODE_NO_MEMORY;
    }
    (*data).edns = true_0 != 0;
    let mut i: size_t = 0 as size_t;
    while i < (*opt).numopts {
        let mut rc: dns_rcode_t = RCODE_OKAY;
        match (*(*opt).opts.offset(i as isize)).code as ::core::ffi::c_uint {
            3 => {
                rc = encode_edns0rr_nsid(data, (*opt).opts.offset(i as isize) as *mut edns0_opt_t);
            }
            _ => {
                rc = encode_edns0rr_raw(data, (*opt).opts.offset(i as isize) as *mut edns0_opt_t);
            }
        }
        if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
            return rc;
        }
        i = i.wrapping_add(1);
    }
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn encode_rr_naptr(
    mut data: *mut edns_context,
    mut naptr: *const dns_naptr_t,
) -> dns_rcode_t {
    let mut rc: dns_rcode_t = RCODE_OKAY;
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                736 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_naptr(edns_context *, const dns_naptr_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !naptr.is_null() {
        } else {
            __assert_fail(
                b"naptr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                737 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_naptr(edns_context *, const dns_naptr_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*naptr).type_0 as ::core::ffi::c_uint
            == RR_NAPTR as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"naptr->type == RR_NAPTR\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                738 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_naptr(edns_context *, const dns_naptr_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if (*naptr).class as ::core::ffi::c_uint
            == CLASS_IN as ::core::ffi::c_int as ::core::ffi::c_uint
        {
        } else {
            __assert_fail(
                b"naptr->class == CLASS_IN\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                739 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_naptr(edns_context *, const dns_naptr_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_3: {
        if (*naptr).order >= 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"naptr->order >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                740 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_naptr(edns_context *, const dns_naptr_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if (*naptr).order <= 65535 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"naptr->order <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                741 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_naptr(edns_context *, const dns_naptr_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_5: {
        if (*naptr).preference >= 0 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"naptr->preference >= 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                742 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_naptr(edns_context *, const dns_naptr_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_6: {
        if (*naptr).preference <= 65535 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"naptr->preference <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                743 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_naptr(edns_context *, const dns_naptr_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_7: {
        if !(*naptr).flags.is_null() {
        } else {
            __assert_fail(
                b"naptr->flags != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                744 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_naptr(edns_context *, const dns_naptr_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_8: {
        if !(*naptr).services.is_null() {
        } else {
            __assert_fail(
                b"naptr->services != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                745 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_naptr(edns_context *, const dns_naptr_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_9: {
        if !(*naptr).regexp.is_null() {
        } else {
            __assert_fail(
                b"naptr->regexp != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                746 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_naptr(edns_context *, const dns_naptr_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_10: {
        if !(*naptr).replacement.is_null() {
        } else {
            __assert_fail(
                b"naptr->replacement != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                747 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_naptr(edns_context *, const dns_naptr_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*data).packet.size < 4 as size_t {
        return RCODE_NO_MEMORY;
    }
    write_uint16(&raw mut (*data).packet, (*naptr).order as uint16_t);
    write_uint16(&raw mut (*data).packet, (*naptr).preference as uint16_t);
    rc = encode_string(data, (*naptr).flags, strlen((*naptr).flags));
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    rc = encode_string(data, (*naptr).services, strlen((*naptr).services));
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    rc = encode_string(data, (*naptr).regexp, strlen((*naptr).regexp));
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    rc = encode_domain(data, (*naptr).replacement);
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn encode_rr_minfo(
    mut data: *mut edns_context,
    mut minfo: *const dns_minfo_t,
) -> dns_rcode_t {
    let mut rc: dns_rcode_t = RCODE_OKAY;
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                769 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_minfo(edns_context *, const dns_minfo_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !minfo.is_null() {
        } else {
            __assert_fail(
                b"minfo != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                770 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_minfo(edns_context *, const dns_minfo_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !(*minfo).rmailbx.is_null() {
        } else {
            __assert_fail(
                b"minfo->rmailbx != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                771 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_minfo(edns_context *, const dns_minfo_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if !(*minfo).emailbx.is_null() {
        } else {
            __assert_fail(
                b"minfo->emailbx != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                772 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_minfo(edns_context *, const dns_minfo_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    rc = encode_domain(data, (*minfo).rmailbx);
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    return encode_domain(data, (*minfo).emailbx);
}
#[inline]
unsafe extern "C" fn encode_rr_mx(
    mut data: *mut edns_context,
    mut mx: *const dns_mx_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                783 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_mx(edns_context *, const dns_mx_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !mx.is_null() {
        } else {
            __assert_fail(
                b"mx != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                784 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_mx(edns_context *, const dns_mx_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*mx).preference <= 65535 as ::core::ffi::c_int {
        } else {
            __assert_fail(
                b"mx->preference <= UINT16_MAX\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                785 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_mx(edns_context *, const dns_mx_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if !(*mx).exchange.is_null() {
        } else {
            __assert_fail(
                b"mx->exchange != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                786 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_mx(edns_context *, const dns_mx_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*data).packet.size < 2 as size_t {
        return RCODE_NO_MEMORY;
    }
    write_uint16(&raw mut (*data).packet, (*mx).preference as uint16_t);
    return encode_domain(data, (*mx).exchange);
}
#[inline]
unsafe extern "C" fn encode_rr_hinfo(
    mut data: *mut edns_context,
    mut hinfo: *const dns_hinfo_t,
) -> dns_rcode_t {
    let mut rc: dns_rcode_t = RCODE_OKAY;
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                801 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_hinfo(edns_context *, const dns_hinfo_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !hinfo.is_null() {
        } else {
            __assert_fail(
                b"hinfo != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                802 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_hinfo(edns_context *, const dns_hinfo_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !(*hinfo).cpu.is_null() {
        } else {
            __assert_fail(
                b"hinfo->cpu != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                803 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_hinfo(edns_context *, const dns_hinfo_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if !(*hinfo).os.is_null() {
        } else {
            __assert_fail(
                b"hinfo->os != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                804 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_hinfo(edns_context *, const dns_hinfo_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    rc = encode_string(data, (*hinfo).cpu, strlen((*hinfo).cpu));
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    return encode_string(data, (*hinfo).os, strlen((*hinfo).os));
}
#[inline]
unsafe extern "C" fn encode_rr_txt(
    mut data: *mut edns_context,
    mut txt: *const dns_txt_t,
) -> dns_rcode_t {
    let mut p: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
    let mut max: size_t = 0;
    let mut rc: dns_rcode_t = RCODE_OKAY;
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                819 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_txt(edns_context *, const dns_txt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !txt.is_null() {
        } else {
            __assert_fail(
                b"txt != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                820 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_txt(edns_context *, const dns_txt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if (*txt).len > 0 as size_t {
        } else {
            __assert_fail(
                b"txt->len > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                821 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_txt(edns_context *, const dns_txt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if !(*txt).text.is_null() {
        } else {
            __assert_fail(
                b"txt->text != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                822 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_txt(edns_context *, const dns_txt_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    p = (*txt).text;
    max = (*txt).len;
    while max > 0 as size_t {
        let mut chunk: size_t = if max < 255 as size_t {
            max
        } else {
            255 as size_t
        };
        rc = encode_string(data, p, chunk);
        if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
            return rc;
        }
        max = max.wrapping_sub(chunk);
        p = p.offset(chunk as isize);
    }
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn encode_rr_x(
    mut data: *mut edns_context,
    mut x: *const dns_x_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                845 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_x(edns_context *, const dns_x_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !x.is_null() {
        } else {
            __assert_fail(
                b"x != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                846 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_x(edns_context *, const dns_x_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if !(*x).rawdata.is_null() {
        } else {
            __assert_fail(
                b"x->rawdata != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                847 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_rr_x(edns_context *, const dns_x_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*data).packet.size < (*x).size {
        return RCODE_NO_MEMORY;
    }
    memcpy(
        (*data).packet.ptr as *mut ::core::ffi::c_void,
        (*x).rawdata as *const ::core::ffi::c_void,
        (*x).size,
    );
    (*data).packet.ptr = (*data).packet.ptr.offset((*x).size as isize);
    (*data).packet.size = (*data).packet.size.wrapping_sub((*x).size);
    return RCODE_OKAY;
}
unsafe extern "C" fn encode_answer(
    mut data: *mut edns_context,
    mut answer: *mut dns_answer_t,
) -> dns_rcode_t {
    let mut rc: dns_rcode_t = RCODE_OKAY;
    let mut prdlen: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut pdata: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    '_c2rust_label: {
        if econtext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"econtext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                870 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_answer(edns_context *, dns_answer_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !answer.is_null() {
        } else {
            __assert_fail(
                b"answer != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                871 as ::core::ffi::c_uint,
                b"dns_rcode_t encode_answer(edns_context *, dns_answer_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    rc = encode_domain(data, (*answer).generic.name);
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    if (*data).packet.size < 10 as size_t {
        return RCODE_NO_MEMORY;
    }
    if (*answer).generic.type_0 as ::core::ffi::c_uint
        == RR_OPT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*answer).opt.class = (*answer).opt.udp_payload as dns_class_t;
        (*answer).opt.ttl = (((*data).rcode as ::core::ffi::c_uint >> 4 as ::core::ffi::c_int
            & 0xff as ::core::ffi::c_uint)
            << 24 as ::core::ffi::c_int
            | (((*answer).opt.version & 0xff as ::core::ffi::c_int) << 16 as ::core::ffi::c_int)
                as ::core::ffi::c_uint
            | ((if (*answer).opt.fdo as ::core::ffi::c_int != 0 {
                0x80 as ::core::ffi::c_int
            } else {
                0 as ::core::ffi::c_int
            }) << 8 as ::core::ffi::c_int) as ::core::ffi::c_uint
            | __bswap_16(((*answer).opt.z & 0xffff as ::core::ffi::c_uint) as __uint16_t)
                as ::core::ffi::c_uint) as TTL;
    }
    write_uint16(
        &raw mut (*data).packet,
        (*answer).generic.type_0 as uint16_t,
    );
    write_uint16(&raw mut (*data).packet, (*answer).generic.class as uint16_t);
    write_uint32(&raw mut (*data).packet, (*answer).generic.ttl as uint32_t);
    prdlen = (*data).packet.ptr;
    (*data).packet.ptr = (*data)
        .packet
        .ptr
        .offset(::core::mem::size_of::<uint16_t>() as usize as isize);
    (*data).packet.size = ((*data).packet.size as ::core::ffi::c_ulong)
        .wrapping_sub(::core::mem::size_of::<uint16_t>() as usize as ::core::ffi::c_ulong)
        as size_t as size_t;
    pdata = (*data).packet.ptr;
    match (*answer).generic.type_0 as ::core::ffi::c_uint {
        1 => {
            rc = encode_rr_a(data, &raw mut (*answer).a);
        }
        6 => {
            rc = encode_rr_soa(data, &raw mut (*answer).soa);
        }
        35 => {
            rc = encode_rr_naptr(data, &raw mut (*answer).naptr);
        }
        28 => {
            rc = encode_rr_aaaa(data, &raw mut (*answer).aaaa);
        }
        33 => {
            rc = encode_rr_srv(data, &raw mut (*answer).srv);
        }
        11 => {
            rc = encode_rr_wks(data, &raw mut (*answer).wks);
        }
        27 => {
            rc = encode_rr_gpos(data, &raw mut (*answer).gpos);
        }
        29 => {
            rc = encode_rr_loc(data, &raw mut (*answer).loc);
        }
        41 => {
            rc = encode_rr_opt(data, &raw mut (*answer).opt);
        }
        26 | 17 | 14 => {
            rc = encode_rr_minfo(data, &raw mut (*answer).minfo);
        }
        18 | 21 | 15 => {
            rc = encode_rr_mx(data, &raw mut (*answer).mx);
        }
        22 | 20 | 13 => {
            rc = encode_rr_hinfo(data, &raw mut (*answer).hinfo);
        }
        19 | 99 | 16 => {
            rc = encode_rr_txt(data, &raw mut (*answer).txt);
        }
        23 | 3 | 4 | 7 | 8 | 9 | 2 | 12 | 5 => {
            rc = encode_domain(data, (*answer).cname.cname);
        }
        10 => {
            rc = encode_rr_x(data, &raw mut (*answer).x);
        }
        _ => {
            if (*answer).generic.type_0 as ::core::ffi::c_uint
                >= RR_PRIVATE as ::core::ffi::c_int as ::core::ffi::c_uint
                && ((*answer).generic.type_0 as ::core::ffi::c_uint)
                    < RR_UNKNOWN as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                rc = encode_rr_x(data, &raw mut (*answer).x);
            } else {
                rc = RCODE_NOT_IMPLEMENTED;
            }
        }
    }
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    let mut c2rust_fresh0: block__s = block {
        size: 2 as size_t,
        ptr: prdlen,
    };
    write_uint16(
        &raw mut c2rust_fresh0,
        (*data).packet.ptr.offset_from(pdata) as ::core::ffi::c_long as uint16_t,
    );
    return rc;
}
#[no_mangle]
pub unsafe extern "C" fn dns_encode(
    mut dest: *mut dns_packet_t,
    mut plen: *mut size_t,
    mut query: *const dns_query_t,
) -> dns_rcode_t {
    let mut header: *mut idns_header = ::core::ptr::null_mut::<idns_header>();
    let mut buffer: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    let mut data: edns_context = edns_context {
        packet: block {
            size: 0,
            ptr: ::core::ptr::null_mut::<uint8_t>(),
        },
        segments: segments {
            idx: 0,
            seg: [segment {
                name: ::core::ptr::null::<::core::ffi::c_char>(),
                offset: 0,
            }; 128],
        },
        edns: false,
        base: ::core::ptr::null_mut::<uint8_t>(),
        rcode: RCODE_OKAY,
    };
    let mut rc: dns_rcode_t = RCODE_OKAY;
    '_c2rust_label: {
        if !dest.is_null() {
        } else {
            __assert_fail(
                b"dest != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                990 as ::core::ffi::c_uint,
                b"dns_rcode_t dns_encode(dns_packet_t *, size_t *, const dns_query_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !plen.is_null() {
        } else {
            __assert_fail(
                b"plen != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                991 as ::core::ffi::c_uint,
                b"dns_rcode_t dns_encode(dns_packet_t *, size_t *, const dns_query_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if *plen >= ::core::mem::size_of::<idns_header>() as usize {
        } else {
            __assert_fail(
                b"*plen >= sizeof(struct idns_header)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                992 as ::core::ffi::c_uint,
                b"dns_rcode_t dns_encode(dns_packet_t *, size_t *, const dns_query_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if query_okay(query) != 0 {
        } else {
            __assert_fail(
                b"query_okay(query)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                993 as ::core::ffi::c_uint,
                b"dns_rcode_t dns_encode(dns_packet_t *, size_t *, const dns_query_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    memset(
        dest as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        *plen,
    );
    buffer = dest as *mut uint8_t;
    header = buffer as *mut idns_header;
    (*header).id = __bswap_16((*query).id as __uint16_t) as uint16_t;
    (*header).opcode = (((*query).opcode as ::core::ffi::c_uint & 0xf as ::core::ffi::c_uint)
        << 3 as ::core::ffi::c_int) as uint8_t;
    (*header).rcode =
        ((*query).rcode as ::core::ffi::c_uint & 0xf as ::core::ffi::c_uint) as uint8_t;
    (*header).qdcount = __bswap_16((*query).qdcount as __uint16_t) as uint16_t;
    (*header).ancount = __bswap_16((*query).ancount as __uint16_t) as uint16_t;
    (*header).nscount = __bswap_16((*query).nscount as __uint16_t) as uint16_t;
    (*header).arcount = __bswap_16((*query).arcount as __uint16_t) as uint16_t;
    if !(*query).query {
        (*header).opcode =
            ((*header).opcode as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as uint8_t;
    }
    if (*query).aa {
        (*header).opcode =
            ((*header).opcode as ::core::ffi::c_int | 0x4 as ::core::ffi::c_int) as uint8_t;
    }
    if (*query).tc {
        (*header).opcode =
            ((*header).opcode as ::core::ffi::c_int | 0x2 as ::core::ffi::c_int) as uint8_t;
    }
    if (*query).rd {
        (*header).opcode =
            ((*header).opcode as ::core::ffi::c_int | 0x1 as ::core::ffi::c_int) as uint8_t;
    }
    if (*query).ra {
        (*header).rcode =
            ((*header).rcode as ::core::ffi::c_int | 0x80 as ::core::ffi::c_int) as uint8_t;
    }
    if (*query).z {
        (*header).rcode =
            ((*header).rcode as ::core::ffi::c_int | 0x40 as ::core::ffi::c_int) as uint8_t;
    }
    if (*query).ad {
        (*header).rcode =
            ((*header).rcode as ::core::ffi::c_int | 0x20 as ::core::ffi::c_int) as uint8_t;
    }
    if (*query).cd {
        (*header).rcode =
            ((*header).rcode as ::core::ffi::c_int | 0x10 as ::core::ffi::c_int) as uint8_t;
    }
    data.packet.size = (*plen).wrapping_sub(::core::mem::size_of::<idns_header>() as size_t);
    data.packet.ptr = buffer.offset(::core::mem::size_of::<idns_header>() as isize) as *mut uint8_t;
    data.base = buffer;
    data.segments.idx = 0 as size_t;
    data.edns = false_0 != 0;
    data.rcode = (*query).rcode;
    let mut i: size_t = 0 as size_t;
    while i < (*query).qdcount {
        rc = encode_question(
            &raw mut data,
            (*query).questions.offset(i as isize) as *mut dns_question_t,
        );
        if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
            return rc;
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: size_t = 0 as size_t;
    while i_0 < (*query).ancount {
        rc = encode_answer(
            &raw mut data,
            (*query).answers.offset(i_0 as isize) as *mut dns_answer_t,
        );
        if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
            return rc;
        }
        i_0 = i_0.wrapping_add(1);
    }
    let mut i_1: size_t = 0 as size_t;
    while i_1 < (*query).nscount {
        rc = encode_answer(
            &raw mut data,
            (*query).nameservers.offset(i_1 as isize) as *mut dns_answer_t,
        );
        if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
            return rc;
        }
        i_1 = i_1.wrapping_add(1);
    }
    if data.edns {
        return RCODE_FORMAT_ERROR;
    }
    let mut i_2: size_t = 0 as size_t;
    while i_2 < (*query).arcount {
        rc = encode_answer(
            &raw mut data,
            (*query).additional.offset(i_2 as isize) as *mut dns_answer_t,
        );
        if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
            return rc;
        }
        i_2 = i_2.wrapping_add(1);
    }
    *plen = data.packet.ptr.offset_from(buffer) as ::core::ffi::c_long as size_t;
    return RCODE_OKAY;
}
unsafe extern "C" fn align_memory(mut pool: *mut block__s) -> bool {
    let mut newsize: size_t = 0;
    let mut delta: size_t = 0;
    '_c2rust_label: {
        if pblock_okay(pool) != 0 {
        } else {
            __assert_fail(
                b"pblock_okay(pool)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1088 as ::core::ffi::c_uint,
                b"_Bool align_memory(block__s *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if (*pool).size < MEM_ALIGN {
        return false_0 != 0;
    }
    newsize = (*pool).size & MEM_MASK;
    if newsize == (*pool).size {
        return true_0 != 0;
    }
    '_c2rust_label_0: {
        if newsize < (*pool).size {
        } else {
            __assert_fail(
                b"newsize < pool->size\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1097 as ::core::ffi::c_uint,
                b"_Bool align_memory(block__s *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    delta = newsize.wrapping_add(MEM_ALIGN).wrapping_sub((*pool).size);
    '_c2rust_label_1: {
        if delta < (*pool).size {
        } else {
            __assert_fail(
                b"delta < pool->size\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1099 as ::core::ffi::c_uint,
                b"_Bool align_memory(block__s *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    (*pool).ptr = (*pool).ptr.offset(delta as isize);
    (*pool).size = (*pool).size.wrapping_sub(delta);
    return true_0 != 0;
}
unsafe extern "C" fn alloc_struct(
    mut pool: *mut block__s,
    mut size: size_t,
) -> *mut ::core::ffi::c_void {
    let mut ptr: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
    '_c2rust_label: {
        if pblock_okay(pool) != 0 {
        } else {
            __assert_fail(
                b"pblock_okay(pool)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1113 as ::core::ffi::c_uint,
                b"void *alloc_struct(block__s *, size_t)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if (*pool).size == 0 as size_t {
        return NULL;
    }
    if !align_memory(pool) {
        return NULL;
    }
    if (*pool).size < size {
        return NULL;
    }
    ptr = (*pool).ptr;
    (*pool).ptr = (*pool).ptr.offset(size as isize);
    (*pool).size = (*pool).size.wrapping_sub(size);
    return ptr as *mut ::core::ffi::c_void;
}
#[inline]
unsafe extern "C" fn read_uint16(mut parse: *mut block__s) -> uint16_t {
    let mut val: uint16_t = 0;
    '_c2rust_label: {
        if pblock_okay(parse) != 0 {
        } else {
            __assert_fail(
                b"pblock_okay(parse)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1135 as ::core::ffi::c_uint,
                b"uint16_t read_uint16(block__s *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*parse).size >= 2 as size_t {
        } else {
            __assert_fail(
                b"parse->size >= 2\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1136 as ::core::ffi::c_uint,
                b"uint16_t read_uint16(block__s *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    val = ((*(*parse).ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        << 8 as ::core::ffi::c_int
        | *(*parse).ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        as uint16_t;
    (*parse).ptr = (*parse).ptr.offset(2 as ::core::ffi::c_int as isize);
    (*parse).size = (*parse).size.wrapping_sub(2 as size_t);
    return val;
}
#[inline]
unsafe extern "C" fn read_uint32(mut parse: *mut block__s) -> uint32_t {
    let mut val: uint32_t = 0;
    '_c2rust_label: {
        if pblock_okay(parse) != 0 {
        } else {
            __assert_fail(
                b"pblock_okay(parse)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1155 as ::core::ffi::c_uint,
                b"uint32_t read_uint32(block__s *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if (*parse).size >= 4 as size_t {
        } else {
            __assert_fail(
                b"parse->size >= 4\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1156 as ::core::ffi::c_uint,
                b"uint32_t read_uint32(block__s *)\0".as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    val = ((*(*parse).ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        << 24 as ::core::ffi::c_int
        | (*(*parse).ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 16 as ::core::ffi::c_int
        | (*(*parse).ptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
            << 8 as ::core::ffi::c_int
        | *(*parse).ptr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
        as uint32_t;
    (*parse).ptr = (*parse).ptr.offset(4 as ::core::ffi::c_int as isize);
    (*parse).size = (*parse).size.wrapping_sub(4 as size_t);
    return val;
}
unsafe extern "C" fn read_raw(
    mut data: *mut ddns_context,
    mut result: *mut *mut uint8_t,
    mut len: size_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1175 as ::core::ffi::c_uint,
                b"dns_rcode_t read_raw(ddns_context *, uint8_t **, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !result.is_null() {
        } else {
            __assert_fail(
                b"result != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1176 as ::core::ffi::c_uint,
                b"dns_rcode_t read_raw(ddns_context *, uint8_t **, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if len > 0 as size_t {
        if len > (*data).parse.size {
            return RCODE_FORMAT_ERROR;
        }
        if !align_memory(&raw mut (*data).dest) {
            return RCODE_NO_MEMORY;
        }
        if len > (*data).dest.size {
            return RCODE_NO_MEMORY;
        }
        *result = (*data).dest.ptr;
        memcpy(
            (*data).dest.ptr as *mut ::core::ffi::c_void,
            (*data).parse.ptr as *const ::core::ffi::c_void,
            len,
        );
        (*data).parse.ptr = (*data).parse.ptr.offset(len as isize);
        (*data).parse.size = (*data).parse.size.wrapping_sub(len);
        (*data).dest.ptr = (*data).dest.ptr.offset(len as isize);
        (*data).dest.size = (*data).dest.size.wrapping_sub(len);
    } else {
        *result = ::core::ptr::null_mut::<uint8_t>();
    }
    return RCODE_OKAY;
}
unsafe extern "C" fn read_string(
    mut data: *mut ddns_context,
    mut result: *mut *const ::core::ffi::c_char,
) -> dns_rcode_t {
    let mut len: size_t = 0;
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1217 as ::core::ffi::c_uint,
                b"dns_rcode_t read_string(ddns_context *, const char **)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !result.is_null() {
        } else {
            __assert_fail(
                b"result != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1218 as ::core::ffi::c_uint,
                b"dns_rcode_t read_string(ddns_context *, const char **)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    len = *(*data).parse.ptr as size_t;
    if (*data).dest.size < len.wrapping_add(1 as size_t) {
        return RCODE_NO_MEMORY;
    }
    if (*data).parse.size < len.wrapping_add(1 as size_t) {
        return RCODE_FORMAT_ERROR;
    }
    *result = (*data).dest.ptr as *mut ::core::ffi::c_char;
    memcpy(
        (*data).dest.ptr as *mut ::core::ffi::c_void,
        (*data).parse.ptr.offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t
            as *const ::core::ffi::c_void,
        len,
    );
    (*data).parse.ptr = (*data)
        .parse
        .ptr
        .offset(len.wrapping_add(1 as size_t) as isize);
    (*data).parse.size = (*data)
        .parse
        .size
        .wrapping_sub(len.wrapping_add(1 as size_t));
    (*data).dest.ptr = (*data).dest.ptr.offset(len as isize);
    (*data).dest.size = (*data).dest.size.wrapping_sub(len);
    let c2rust_fresh14 = (*data).dest.ptr;
    (*data).dest.ptr = (*data).dest.ptr.offset(1);
    *c2rust_fresh14 = '\0' as i32 as uint8_t;
    (*data).dest.size = (*data).dest.size.wrapping_sub(1);
    return RCODE_OKAY;
}
unsafe extern "C" fn read_domain(
    mut data: *mut ddns_context,
    mut result: *mut *const ::core::ffi::c_char,
) -> dns_rcode_t {
    let mut parse: *mut block__s = &raw mut (*data).parse;
    let mut tmp: block__s = block {
        size: 0,
        ptr: ::core::ptr::null_mut::<uint8_t>(),
    };
    let mut len: size_t = 0;
    let mut loop_0: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1253 as ::core::ffi::c_uint,
                b"dns_rcode_t read_domain(ddns_context *, const char **)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !result.is_null() {
        } else {
            __assert_fail(
                b"result != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1254 as ::core::ffi::c_uint,
                b"dns_rcode_t read_domain(ddns_context *, const char **)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    *result = (*data).dest.ptr as *mut ::core::ffi::c_char;
    loop_0 = 0 as ::core::ffi::c_int;
    loop {
        if (*(*parse).ptr as ::core::ffi::c_int) < 64 as ::core::ffi::c_int {
            len = *(*parse).ptr as size_t;
            if (*parse).size < len.wrapping_add(1 as size_t) {
                return RCODE_FORMAT_ERROR;
            }
            if (*data).dest.size < len.wrapping_add(1 as size_t) {
                return RCODE_NO_MEMORY;
            }
            if len != 0 {
                memcpy(
                    (*data).dest.ptr as *mut ::core::ffi::c_void,
                    (*parse).ptr.offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t
                        as *const ::core::ffi::c_void,
                    len,
                );
                (*parse).ptr = (*parse).ptr.offset(len.wrapping_add(1 as size_t) as isize);
                (*parse).size = (*parse).size.wrapping_sub(len.wrapping_add(1 as size_t));
            }
            (*data).dest.size = (*data)
                .dest
                .size
                .wrapping_sub(len.wrapping_add(1 as size_t));
            (*data).dest.ptr = (*data).dest.ptr.offset(len as isize);
            let c2rust_fresh10 = (*data).dest.ptr;
            (*data).dest.ptr = (*data).dest.ptr.offset(1);
            *c2rust_fresh10 = '.' as i32 as uint8_t;
        } else if *(*parse).ptr as ::core::ffi::c_int >= 192 as ::core::ffi::c_int {
            loop_0 += 1;
            if loop_0 == 256 as ::core::ffi::c_int {
                return RCODE_FORMAT_ERROR;
            }
            if (*parse).size < 2 as size_t {
                return RCODE_FORMAT_ERROR;
            }
            len =
                (read_uint16(parse) as ::core::ffi::c_int & 0x3fff as ::core::ffi::c_int) as size_t;
            if len >= (*data).packet.size {
                return RCODE_FORMAT_ERROR;
            }
            tmp.ptr = (*data).packet.ptr.offset(len as isize) as *mut uint8_t;
            tmp.size = (*data)
                .packet
                .size
                .wrapping_sub(
                    tmp.ptr.offset_from((*data).packet.ptr) as ::core::ffi::c_long as size_t,
                );
            parse = &raw mut tmp;
        } else if *(*parse).ptr as ::core::ffi::c_int >= 64 as ::core::ffi::c_int
            && *(*parse).ptr as ::core::ffi::c_int <= 127 as ::core::ffi::c_int
        {
            return RCODE_FORMAT_ERROR;
        } else {
            return RCODE_FORMAT_ERROR;
        }
        if (*parse).size < 1 as size_t {
            return RCODE_FORMAT_ERROR;
        }
        if !(*(*parse).ptr != 0) {
            break;
        }
    }
    (*parse).ptr = (*parse).ptr.offset(1);
    (*parse).size = (*parse).size.wrapping_sub(1);
    let c2rust_fresh11 = (*data).dest.ptr;
    (*data).dest.ptr = (*data).dest.ptr.offset(1);
    *c2rust_fresh11 = '\0' as i32 as uint8_t;
    (*data).dest.size = (*data).dest.size.wrapping_sub(1);
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn decode_edns0rr_nsid(
    mut data: *mut ddns_context,
    mut opt: *mut edns0_opt_t,
) -> dns_rcode_t {
    static mut hexdigits: [::core::ffi::c_char; 17] = unsafe {
        ::core::mem::transmute::<[u8; 17], [::core::ffi::c_char; 17]>(*b"0123456789ABCDEF\0")
    };
    if (*opt).len.wrapping_rem(2 as size_t) == 1 as size_t {
        return RCODE_FORMAT_ERROR;
    }
    if (*data).dest.size < (*opt).len.wrapping_div(2 as size_t) {
        return RCODE_NO_MEMORY;
    }
    let mut i: size_t = 0 as size_t;
    while i < (*opt).len {
        let mut phexh: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        let mut phexl: *const ::core::ffi::c_char = ::core::ptr::null::<::core::ffi::c_char>();
        if *(*__ctype_b_loc())
            .offset(*(*data).parse.ptr.offset(i as isize) as ::core::ffi::c_int as isize)
            as ::core::ffi::c_int
            & _ISxdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            == 0
        {
            return RCODE_FORMAT_ERROR;
        }
        if *(*__ctype_b_loc()).offset(
            *(*data)
                .parse
                .ptr
                .offset(i.wrapping_add(1 as size_t) as isize) as ::core::ffi::c_int
                as isize,
        ) as ::core::ffi::c_int
            & _ISxdigit as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
            == 0
        {
            return RCODE_FORMAT_ERROR;
        }
        phexh = memchr(
            &raw const hexdigits as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<uint8_t>() as usize > 1 as usize {
                    if 0 != 0 {
                        let mut __c: ::core::ffi::c_int =
                            *(*data).parse.ptr.offset(i as isize) as ::core::ffi::c_int;
                        __res = (if __c < -(128 as ::core::ffi::c_int)
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res =
                            toupper(*(*data).parse.ptr.offset(i as isize) as ::core::ffi::c_int);
                    }
                } else {
                    __res =
                        *(*__ctype_toupper_loc())
                            .offset(*(*data).parse.ptr.offset(i as isize) as ::core::ffi::c_int
                                as isize) as ::core::ffi::c_int;
                }
                __res
            }),
            16 as size_t,
        ) as *const ::core::ffi::c_char;
        phexl = memchr(
            &raw const hexdigits as *const ::core::ffi::c_char as *const ::core::ffi::c_void,
            ({
                let mut __res: ::core::ffi::c_int = 0;
                if ::core::mem::size_of::<uint8_t>() as usize > 1 as usize {
                    if 0 != 0 {
                        let mut __c: ::core::ffi::c_int = *(*data)
                            .parse
                            .ptr
                            .offset(i.wrapping_add(1 as size_t) as isize)
                            as ::core::ffi::c_int;
                        __res = (if __c < -(128 as ::core::ffi::c_int)
                            || __c > 255 as ::core::ffi::c_int
                        {
                            __c as __int32_t
                        } else {
                            *(*__ctype_toupper_loc()).offset(__c as isize)
                        }) as ::core::ffi::c_int;
                    } else {
                        __res = toupper(
                            *(*data)
                                .parse
                                .ptr
                                .offset(i.wrapping_add(1 as size_t) as isize)
                                as ::core::ffi::c_int,
                        );
                    }
                } else {
                    __res = *(*__ctype_toupper_loc()).offset(
                        *(*data)
                            .parse
                            .ptr
                            .offset(i.wrapping_add(1 as size_t) as isize)
                            as ::core::ffi::c_int as isize,
                    ) as ::core::ffi::c_int;
                }
                __res
            }),
            16 as size_t,
        ) as *const ::core::ffi::c_char;
        '_c2rust_label_1: {
            if !phexh.is_null() {
            } else {
                __assert_fail(
                    b"phexh != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1368 as ::core::ffi::c_uint,
                    b"dns_rcode_t decode_edns0rr_nsid(ddns_context *, edns0_opt_t *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_2: {
            if !phexl.is_null() {
            } else {
                __assert_fail(
                    b"phexl != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1369 as ::core::ffi::c_uint,
                    b"dns_rcode_t decode_edns0rr_nsid(ddns_context *, edns0_opt_t *)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        *(*data).dest.ptr = ((phexh.offset_from(&raw const hexdigits as *const ::core::ffi::c_char)
            as ::core::ffi::c_long)
            << 4 as ::core::ffi::c_int
            | phexl.offset_from(&raw const hexdigits as *const ::core::ffi::c_char)
                as ::core::ffi::c_long) as uint8_t;
        (*data).dest.ptr = (*data).dest.ptr.offset(1);
        (*data).dest.size = (*data).dest.size.wrapping_sub(1);
        i = i.wrapping_add(2 as size_t);
    }
    (*data).parse.ptr = (*data).parse.ptr.offset((*opt).len as isize);
    (*data).parse.size = (*data).parse.size.wrapping_sub((*opt).len);
    (*opt).len = (*opt).len.wrapping_div(2 as size_t);
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn decode_edns0rr_raw(
    mut data: *mut ddns_context,
    mut opt: *mut edns0_opt_t,
) -> dns_rcode_t {
    if (*data).dest.size < (*opt).len {
        return RCODE_NO_MEMORY;
    }
    memcpy(
        (*data).dest.ptr as *mut ::core::ffi::c_void,
        (*data).parse.ptr as *const ::core::ffi::c_void,
        (*opt).len,
    );
    (*data).parse.ptr = (*data).parse.ptr.offset((*opt).len as isize);
    (*data).parse.size = (*data).parse.size.wrapping_sub((*opt).len);
    (*data).dest.ptr = (*data).dest.ptr.offset((*opt).len as isize);
    (*data).dest.size = (*data).dest.size.wrapping_sub((*opt).len);
    return RCODE_OKAY;
}
unsafe extern "C" fn decode_question(
    mut data: *mut ddns_context,
    mut pquest: *mut dns_question_t,
) -> dns_rcode_t {
    let mut rc: dns_rcode_t = RCODE_OKAY;
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1410 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_question(ddns_context *, dns_question_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !pquest.is_null() {
        } else {
            __assert_fail(
                b"pquest != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1411 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_question(ddns_context *, dns_question_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    rc = read_domain(data, &raw mut (*pquest).name);
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    if (*data).parse.size < 4 as size_t {
        return RCODE_FORMAT_ERROR;
    }
    (*pquest).type_0 = read_uint16(&raw mut (*data).parse) as dns_type_t;
    (*pquest).class = read_uint16(&raw mut (*data).parse) as dns_class_t;
    if (*pquest).type_0 as ::core::ffi::c_uint
        == RR_OPT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        return RCODE_FORMAT_ERROR;
    }
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn decode_rr_soa(
    mut data: *mut ddns_context,
    mut psoa: *mut dns_soa_t,
    mut len: size_t,
) -> dns_rcode_t {
    let mut rc: dns_rcode_t = RCODE_OKAY;
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1444 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_soa(ddns_context *, dns_soa_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !psoa.is_null() {
        } else {
            __assert_fail(
                b"psoa != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1445 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_soa(ddns_context *, dns_soa_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    rc = read_domain(data, &raw mut (*psoa).mname);
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    rc = read_domain(data, &raw mut (*psoa).rname);
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    if len < 20 as size_t {
        return RCODE_FORMAT_ERROR;
    }
    (*psoa).serial = read_uint32(&raw mut (*data).parse);
    (*psoa).refresh = read_uint32(&raw mut (*data).parse);
    (*psoa).retry = read_uint32(&raw mut (*data).parse);
    (*psoa).expire = read_uint32(&raw mut (*data).parse);
    (*psoa).minimum = read_uint32(&raw mut (*data).parse);
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn decode_rr_a(
    mut data: *mut ddns_context,
    mut pa: *mut dns_a_t,
    mut len: size_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1472 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_a(ddns_context *, dns_a_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !pa.is_null() {
        } else {
            __assert_fail(
                b"pa != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1473 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_a(ddns_context *, dns_a_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if len != 4 as size_t {
        return RCODE_FORMAT_ERROR;
    }
    memcpy(
        &raw mut (*pa).address as *mut ::core::ffi::c_void,
        (*data).parse.ptr as *const ::core::ffi::c_void,
        4 as size_t,
    );
    (*data).parse.ptr = (*data).parse.ptr.offset(4 as ::core::ffi::c_int as isize);
    (*data).parse.size = (*data).parse.size.wrapping_sub(4 as size_t);
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn decode_rr_aaaa(
    mut data: *mut ddns_context,
    mut pa: *mut dns_aaaa_t,
    mut len: size_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1490 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_aaaa(ddns_context *, dns_aaaa_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !pa.is_null() {
        } else {
            __assert_fail(
                b"pa != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1491 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_aaaa(ddns_context *, dns_aaaa_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if len != 16 as size_t {
        return RCODE_FORMAT_ERROR;
    }
    memcpy(
        &raw mut (*pa).address.__in6_u.__u6_addr8 as *mut uint8_t as *mut ::core::ffi::c_void,
        (*data).parse.ptr as *const ::core::ffi::c_void,
        16 as size_t,
    );
    (*data).parse.ptr = (*data).parse.ptr.offset(16 as ::core::ffi::c_int as isize);
    (*data).parse.size = (*data).parse.size.wrapping_sub(16 as size_t);
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn decode_rr_wks(
    mut data: *mut ddns_context,
    mut pwks: *mut dns_wks_t,
    mut len: size_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1508 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_wks(ddns_context *, dns_wks_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !pwks.is_null() {
        } else {
            __assert_fail(
                b"pwks != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1509 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_wks(ddns_context *, dns_wks_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if len < 6 as size_t {
        return RCODE_FORMAT_ERROR;
    }
    memcpy(
        &raw mut (*pwks).address as *mut ::core::ffi::c_void,
        (*data).parse.ptr as *const ::core::ffi::c_void,
        4 as size_t,
    );
    (*data).parse.ptr = (*data).parse.ptr.offset(4 as ::core::ffi::c_int as isize);
    (*data).parse.size = (*data).parse.size.wrapping_sub(4 as size_t);
    (*pwks).protocol = read_uint16(&raw mut (*data).parse) as ::core::ffi::c_int;
    (*pwks).numbits = len.wrapping_sub(6 as size_t);
    return read_raw(data, &raw mut (*pwks).bits, (*pwks).numbits);
}
#[inline]
unsafe extern "C" fn decode_rr_mx(
    mut data: *mut ddns_context,
    mut pmx: *mut dns_mx_t,
    mut len: size_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1530 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_mx(ddns_context *, dns_mx_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !pmx.is_null() {
        } else {
            __assert_fail(
                b"pmx != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1531 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_mx(ddns_context *, dns_mx_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if len < 4 as size_t {
        return RCODE_FORMAT_ERROR;
    }
    (*pmx).preference = read_uint16(&raw mut (*data).parse) as ::core::ffi::c_int;
    return read_domain(data, &raw mut (*pmx).exchange);
}
#[inline]
unsafe extern "C" fn decode_rr_txt(
    mut data: *mut ddns_context,
    mut ptxt: *mut dns_txt_t,
    mut len: size_t,
) -> dns_rcode_t {
    let mut slen: size_t = 0;
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1549 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_txt(ddns_context *, dns_txt_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !ptxt.is_null() {
        } else {
            __assert_fail(
                b"ptxt != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1550 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_txt(ddns_context *, dns_txt_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    (*ptxt).text = (*data).dest.ptr as *const ::core::ffi::c_char;
    (*ptxt).len = 0 as size_t;
    while len != 0 {
        if (*data).parse.size < 1 as size_t {
            return RCODE_FORMAT_ERROR;
        }
        let c2rust_fresh12 = (*data).parse.ptr;
        (*data).parse.ptr = (*data).parse.ptr.offset(1);
        slen = *c2rust_fresh12 as size_t;
        (*data).parse.size = (*data).parse.size.wrapping_sub(1);
        len = len.wrapping_sub(1);
        if slen > len {
            return RCODE_FORMAT_ERROR;
        }
        if (*data).parse.size < slen {
            return RCODE_FORMAT_ERROR;
        }
        if (*data).dest.size < slen {
            return RCODE_NO_MEMORY;
        }
        memcpy(
            (*data).dest.ptr as *mut ::core::ffi::c_void,
            (*data).parse.ptr as *const ::core::ffi::c_void,
            slen,
        );
        '_c2rust_label_1: {
            if slen <= len {
            } else {
                __assert_fail(
                    b"slen <= len\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1580 as ::core::ffi::c_uint,
                    b"dns_rcode_t decode_rr_txt(ddns_context *, dns_txt_t *, size_t)\0".as_ptr()
                        as *const ::core::ffi::c_char,
                );
            }
        };
        (*ptxt).len = (*ptxt).len.wrapping_add(slen);
        (*data).dest.ptr = (*data).dest.ptr.offset(slen as isize);
        (*data).dest.size = (*data).dest.size.wrapping_sub(slen);
        (*data).parse.ptr = (*data).parse.ptr.offset(slen as isize);
        (*data).parse.size = (*data).parse.size.wrapping_sub(slen);
        len = len.wrapping_sub(slen);
    }
    if (*data).dest.size == 0 as size_t {
        return RCODE_NO_MEMORY;
    }
    let c2rust_fresh13 = (*data).dest.ptr;
    (*data).dest.ptr = (*data).dest.ptr.offset(1);
    *c2rust_fresh13 = '\0' as i32 as uint8_t;
    (*data).dest.size = (*data).dest.size.wrapping_sub(1);
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn decode_rr_hinfo(
    mut data: *mut ddns_context,
    mut phinfo: *mut dns_hinfo_t,
) -> dns_rcode_t {
    let mut rc: dns_rcode_t = RCODE_OKAY;
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1607 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_hinfo(ddns_context *, dns_hinfo_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !phinfo.is_null() {
        } else {
            __assert_fail(
                b"phinfo != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1608 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_hinfo(ddns_context *, dns_hinfo_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    rc = read_string(data, &raw mut (*phinfo).cpu);
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    rc = read_string(data, &raw mut (*phinfo).os);
    return rc;
}
#[inline]
unsafe extern "C" fn decode_rr_srv(
    mut data: *mut ddns_context,
    mut psrv: *mut dns_srv_t,
    mut len: size_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1624 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_srv(ddns_context *, dns_srv_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !psrv.is_null() {
        } else {
            __assert_fail(
                b"psrv != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1625 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_srv(ddns_context *, dns_srv_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if len < 7 as size_t {
        return RCODE_FORMAT_ERROR;
    }
    (*psrv).priority = read_uint16(&raw mut (*data).parse) as ::core::ffi::c_int;
    (*psrv).weight = read_uint16(&raw mut (*data).parse) as ::core::ffi::c_int;
    (*psrv).port = read_uint16(&raw mut (*data).parse) as ::core::ffi::c_int;
    return read_domain(data, &raw mut (*psrv).target);
}
#[inline]
unsafe extern "C" fn decode_rr_naptr(
    mut data: *mut ddns_context,
    mut pnaptr: *mut dns_naptr_t,
    mut len: size_t,
) -> dns_rcode_t {
    let mut rc: dns_rcode_t = RCODE_OKAY;
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1646 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_naptr(ddns_context *, dns_naptr_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !pnaptr.is_null() {
        } else {
            __assert_fail(
                b"pnaptr != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1647 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_naptr(ddns_context *, dns_naptr_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if len < 4 as size_t {
        return RCODE_FORMAT_ERROR;
    }
    (*pnaptr).order = read_uint16(&raw mut (*data).parse) as ::core::ffi::c_int;
    (*pnaptr).preference = read_uint16(&raw mut (*data).parse) as ::core::ffi::c_int;
    rc = read_string(data, &raw mut (*pnaptr).flags);
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    rc = read_string(data, &raw mut (*pnaptr).services);
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    rc = read_string(data, &raw mut (*pnaptr).regexp);
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    return read_domain(data, &raw mut (*pnaptr).replacement);
}
#[inline]
unsafe extern "C" fn decode_rr_minfo(
    mut data: *mut ddns_context,
    mut pminfo: *mut dns_minfo_t,
) -> dns_rcode_t {
    let mut rc: dns_rcode_t = RCODE_OKAY;
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1673 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_minfo(ddns_context *, dns_minfo_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !pminfo.is_null() {
        } else {
            __assert_fail(
                b"pminfo != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1674 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_minfo(ddns_context *, dns_minfo_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    rc = read_domain(data, &raw mut (*pminfo).rmailbx);
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    return read_domain(data, &raw mut (*pminfo).emailbx);
}
unsafe extern "C" fn dloc_double(
    mut data: *mut ddns_context,
    mut pvalue: *mut ::core::ffi::c_double,
) -> dns_rcode_t {
    let mut len: size_t = 0;
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1690 as ::core::ffi::c_uint,
                b"dns_rcode_t dloc_double(ddns_context *, double *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !pvalue.is_null() {
        } else {
            __assert_fail(
                b"pvalue != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1691 as ::core::ffi::c_uint,
                b"dns_rcode_t dloc_double(ddns_context *, double *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    len = *(*data).parse.ptr as size_t;
    if len > (*data).parse.size.wrapping_sub(1 as size_t) {
        return RCODE_FORMAT_ERROR;
    }
    let mut buffer: [::core::ffi::c_char; 72] = [0; 72];
    if len >= ::core::mem::size_of::<[::core::ffi::c_char; 72]>() as usize {
        return RCODE_FORMAT_ERROR;
    }
    memcpy(
        &raw mut buffer as *mut ::core::ffi::c_char as *mut ::core::ffi::c_void,
        (*data).parse.ptr.offset(1 as ::core::ffi::c_int as isize) as *mut uint8_t
            as *const ::core::ffi::c_void,
        len,
    );
    let c2rust_fresh16 = len;
    len = len.wrapping_add(1);
    buffer[c2rust_fresh16 as usize] = '\0' as i32 as ::core::ffi::c_char;
    (*data).parse.ptr = (*data).parse.ptr.offset(len as isize);
    (*data).parse.size = (*data).parse.size.wrapping_sub(len);
    *__errno_location() = 0 as ::core::ffi::c_int;
    *pvalue = strtod(
        &raw mut buffer as *mut ::core::ffi::c_char,
        ::core::ptr::null_mut::<*mut ::core::ffi::c_char>(),
    );
    if *__errno_location() != 0 {
        return RCODE_FORMAT_ERROR;
    }
    return RCODE_OKAY;
}
unsafe extern "C" fn dgpos_angle(mut pa: *mut dnsgpos_angle, mut v: ::core::ffi::c_double) {
    let mut ip: ::core::ffi::c_double = 0.;
    v = modf(v, &raw mut ip) * 60.0f64;
    (*pa).deg = ip as ::core::ffi::c_int;
    v = modf(v, &raw mut ip) * 60.0f64;
    (*pa).min = ip as ::core::ffi::c_int;
    v = modf(v, &raw mut ip) * 1000.0f64;
    (*pa).sec = ip as ::core::ffi::c_int;
    (*pa).frac = v as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn decode_rr_gpos(
    mut data: *mut ddns_context,
    mut pgpos: *mut dns_gpos_t,
) -> dns_rcode_t {
    let mut rc: dns_rcode_t = RCODE_OKAY;
    let mut lat: ::core::ffi::c_double = 0.;
    let mut lng: ::core::ffi::c_double = 0.;
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1749 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_gpos(ddns_context *, dns_gpos_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !pgpos.is_null() {
        } else {
            __assert_fail(
                b"pgpos != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1750 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_gpos(ddns_context *, dns_gpos_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    rc = dloc_double(data, &raw mut lng);
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    rc = dloc_double(data, &raw mut lat);
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    if lng < 0.0f64 {
        (*pgpos).longitude.nw = true_0 != 0;
        lng = fabs(lng);
    } else {
        (*pgpos).longitude.nw = false_0 != 0;
    }
    if lat >= 0.0f64 {
        (*pgpos).latitude.nw = true_0 != 0;
    } else {
        (*pgpos).latitude.nw = false_0 != 0;
        lat = fabs(lat);
    }
    dgpos_angle(&raw mut (*pgpos).longitude, lng);
    dgpos_angle(&raw mut (*pgpos).latitude, lat);
    return dloc_double(data, &raw mut (*pgpos).altitude);
}
unsafe extern "C" fn dloc_scale(
    mut presult: *mut ::core::ffi::c_ulonglong,
    scale: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let mut spow: ::core::ffi::c_int = 0;
    let mut smul: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !presult.is_null() {
        } else {
            __assert_fail(
                b"presult != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1794 as ::core::ffi::c_uint,
                b"int dloc_scale(unsigned long long *, const int)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    smul = scale >> 4 as ::core::ffi::c_int;
    spow = scale & 0xf as ::core::ffi::c_int;
    if spow > 9 as ::core::ffi::c_int || smul > 9 as ::core::ffi::c_int {
        return RCODE_FORMAT_ERROR as ::core::ffi::c_int;
    }
    *presult = (pow(10.0f64, spow as ::core::ffi::c_double) * smul as ::core::ffi::c_double)
        as ::core::ffi::c_ulong as ::core::ffi::c_ulonglong;
    return RCODE_OKAY as ::core::ffi::c_int;
}
unsafe extern "C" fn dloc_angle(mut pa: *mut dnsgpos_angle, v: ::core::ffi::c_long) {
    let mut partial: ldiv_t = ldiv_t { quot: 0, rem: 0 };
    '_c2rust_label: {
        if !pa.is_null() {
        } else {
            __assert_fail(
                b"pa != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1815 as ::core::ffi::c_uint,
                b"void dloc_angle(dnsgpos_angle *, const long)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    partial = ldiv(v, 1000 as ::core::ffi::c_long);
    (*pa).frac = partial.rem as ::core::ffi::c_int;
    partial = ldiv(partial.quot, 60 as ::core::ffi::c_long);
    (*pa).sec = partial.rem as ::core::ffi::c_int;
    partial = ldiv(partial.quot, 60 as ::core::ffi::c_long);
    (*pa).min = partial.rem as ::core::ffi::c_int;
    (*pa).deg = partial.quot as ::core::ffi::c_int;
}
#[inline]
unsafe extern "C" fn decode_rr_loc(
    mut data: *mut ddns_context,
    mut ploc: *mut dns_loc_t,
    mut len: size_t,
) -> dns_rcode_t {
    let mut rc: dns_rcode_t = RCODE_OKAY;
    let mut lat: ::core::ffi::c_ulong = 0;
    let mut lng: ::core::ffi::c_ulong = 0;
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1838 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_loc(ddns_context *, dns_loc_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !ploc.is_null() {
        } else {
            __assert_fail(
                b"ploc != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1839 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_loc(ddns_context *, dns_loc_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if len < 16 as size_t {
        return RCODE_FORMAT_ERROR;
    }
    (*ploc).version =
        *(*data).parse.ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int;
    if (*ploc).version != 0 as ::core::ffi::c_int {
        return RCODE_FORMAT_ERROR;
    }
    rc = dloc_scale(
        &raw mut (*ploc).size,
        *(*data).parse.ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
    ) as dns_rcode_t;
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    rc = dloc_scale(
        &raw mut (*ploc).horiz_pre,
        *(*data).parse.ptr.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
    ) as dns_rcode_t;
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    rc = dloc_scale(
        &raw mut (*ploc).vert_pre,
        *(*data).parse.ptr.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int,
    ) as dns_rcode_t;
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    (*data).parse.ptr = (*data).parse.ptr.offset(4 as ::core::ffi::c_int as isize);
    lat = read_uint32(&raw mut (*data).parse) as ::core::ffi::c_ulong;
    lng = read_uint32(&raw mut (*data).parse) as ::core::ffi::c_ulong;
    (*ploc).altitude = read_uint32(&raw mut (*data).parse) as ::core::ffi::c_long - LOC_ALT_BIAS;
    if lat >= LOC_BIAS {
        (*ploc).latitude.nw = true_0 != 0;
        lat = lat.wrapping_sub(LOC_BIAS);
    } else {
        (*ploc).latitude.nw = false_0 != 0;
        lat = LOC_BIAS.wrapping_sub(lat);
    }
    if lng >= LOC_BIAS {
        (*ploc).longitude.nw = false_0 != 0;
        lng = lng.wrapping_sub(LOC_BIAS);
    } else {
        (*ploc).longitude.nw = true_0 != 0;
        lng = LOC_BIAS.wrapping_sub(lng);
    }
    if lat > LOC_LAT_MAX {
        return RCODE_FORMAT_ERROR;
    }
    if lng > LOC_LNG_MAX {
        return RCODE_FORMAT_ERROR;
    }
    dloc_angle(&raw mut (*ploc).latitude, lat as ::core::ffi::c_long);
    dloc_angle(&raw mut (*ploc).longitude, lng as ::core::ffi::c_long);
    return RCODE_OKAY;
}
#[inline]
unsafe extern "C" fn decode_rr_opt(
    mut data: *mut ddns_context,
    mut opt: *mut dns_edns0opt_t,
    mut len: size_t,
) -> dns_rcode_t {
    '_c2rust_label: {
        if !data.is_null() {
        } else {
            __assert_fail(
                b"data != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1903 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_opt(ddns_context *, dns_edns0opt_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !opt.is_null() {
        } else {
            __assert_fail(
                b"opt != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1904 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_rr_opt(ddns_context *, dns_edns0opt_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    if (*data).edns {
        return RCODE_FORMAT_ERROR;
    }
    (*data).edns = true_0 != 0;
    (*opt).numopts = 0 as size_t;
    (*opt).opts = ::core::ptr::null_mut::<edns0_opt_t>();
    (*opt).version = ((*opt).ttl >> 16 as ::core::ffi::c_int & 0xff as TTL) as ::core::ffi::c_int;
    (*opt).z = __bswap_16(((*opt).ttl & 0xffff as TTL) as __uint16_t) as ::core::ffi::c_uint;
    (*(*data).response).rcode = ((*(*data).response).rcode as ::core::ffi::c_uint
        | ((*opt).ttl >> 20 as ::core::ffi::c_int & 0xff0 as TTL) as ::core::ffi::c_uint)
        as dns_rcode_t;
    if len != 0 {
        let mut scan: *mut uint8_t = ::core::ptr::null_mut::<uint8_t>();
        let mut length: size_t = 0;
        '_c2rust_label_1: {
            if dcontext_okay(data) != 0 {
            } else {
                __assert_fail(
                    b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1921 as ::core::ffi::c_uint,
                    b"dns_rcode_t decode_rr_opt(ddns_context *, dns_edns0opt_t *, size_t)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        '_c2rust_label_2: {
            if len > 4 as size_t {
            } else {
                __assert_fail(
                    b"len > 4\0".as_ptr() as *const ::core::ffi::c_char,
                    b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                    1922 as ::core::ffi::c_uint,
                    b"dns_rcode_t decode_rr_opt(ddns_context *, dns_edns0opt_t *, size_t)\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                );
            }
        };
        scan = (*data).parse.ptr;
        (*opt).numopts = 0 as size_t;
        length = len;
        while length > 0 as size_t {
            let mut size: size_t = 0;
            (*opt).numopts = (*opt).numopts.wrapping_add(1);
            size = (((*scan.offset(2 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 8 as ::core::ffi::c_int
                | *scan.offset(3 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                + 4 as ::core::ffi::c_int) as size_t;
            scan = scan.offset(size as isize);
            if size > length {
                return RCODE_FORMAT_ERROR;
            }
            length = length.wrapping_sub(size);
        }
        (*opt).opts = alloc_struct(
            &raw mut (*data).dest,
            (::core::mem::size_of::<edns0_opt_t>() as size_t).wrapping_mul((*opt).numopts),
        ) as *mut edns0_opt_t;
        if (*opt).opts.is_null() {
            return RCODE_NO_MEMORY;
        }
        let mut i: size_t = 0 as size_t;
        while i < (*opt).numopts {
            let mut rc: dns_rcode_t = RCODE_OKAY;
            (*(*opt).opts.offset(i as isize)).code =
                read_uint16(&raw mut (*data).parse) as edns0_type_t;
            (*(*opt).opts.offset(i as isize)).len = read_uint16(&raw mut (*data).parse) as size_t;
            if !align_memory(&raw mut (*data).dest) {
                return RCODE_NO_MEMORY;
            }
            let ref mut c2rust_fresh15 = (*(*opt).opts.offset(i as isize)).data;
            *c2rust_fresh15 = (*data).dest.ptr;
            match (*(*opt).opts.offset(i as isize)).code as ::core::ffi::c_uint {
                3 => {
                    rc = decode_edns0rr_nsid(
                        data,
                        (*opt).opts.offset(i as isize) as *mut edns0_opt_t,
                    );
                }
                _ => {
                    rc = decode_edns0rr_raw(
                        data,
                        (*opt).opts.offset(i as isize) as *mut edns0_opt_t,
                    );
                }
            }
            if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint
            {
                return rc;
            }
            i = i.wrapping_add(1);
        }
    }
    return RCODE_OKAY;
}
unsafe extern "C" fn decode_answer(
    mut data: *mut ddns_context,
    mut pans: *mut dns_answer_t,
) -> dns_rcode_t {
    let mut len: size_t = 0;
    let mut rest: size_t = 0;
    let mut rc: dns_rcode_t = RCODE_OKAY;
    '_c2rust_label: {
        if dcontext_okay(data) != 0 {
        } else {
            __assert_fail(
                b"dcontext_okay(data)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1983 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_answer(ddns_context *, dns_answer_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !pans.is_null() {
        } else {
            __assert_fail(
                b"pans != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                1984 as ::core::ffi::c_uint,
                b"dns_rcode_t decode_answer(ddns_context *, dns_answer_t *)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    rc = read_domain(data, &raw mut (*pans).generic.name);
    if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
        return rc;
    }
    if (*data).parse.size < 10 as size_t {
        return RCODE_FORMAT_ERROR;
    }
    (*pans).generic.type_0 = read_uint16(&raw mut (*data).parse) as dns_type_t;
    if (*pans).generic.type_0 as ::core::ffi::c_uint
        == RR_OPT as ::core::ffi::c_int as ::core::ffi::c_uint
    {
        (*pans).generic.class = CLASS_UNKNOWN;
        (*pans).generic.ttl = 0 as TTL;
        (*pans).opt.udp_payload = read_uint16(&raw mut (*data).parse) as size_t;
        (*(*data).response).rcode =
            (((*(*data).parse.ptr.offset(0 as ::core::ffi::c_int as isize) as ::core::ffi::c_int)
                << 4 as ::core::ffi::c_int) as ::core::ffi::c_uint
                | (*(*data).response).rcode as ::core::ffi::c_uint) as dns_rcode_t;
        if *(*data).parse.ptr.offset(1 as ::core::ffi::c_int as isize) as ::core::ffi::c_int
            != 0 as ::core::ffi::c_int
        {
            return RCODE_FORMAT_ERROR;
        }
        (*data).parse.ptr = (*data).parse.ptr.offset(2 as ::core::ffi::c_int as isize);
        (*data).parse.size = (*data).parse.size.wrapping_sub(2 as size_t);
        (*pans).opt.fug = read_uint16(&raw mut (*data).parse) as ::core::ffi::c_int;
        (*pans).opt.fdo = (*pans).opt.fug > 0x7fff as ::core::ffi::c_int;
        (*pans).opt.fug &= 0x7fff as ::core::ffi::c_int;
    } else {
        (*pans).generic.class = read_uint16(&raw mut (*data).parse) as dns_class_t;
        (*pans).generic.ttl = read_uint32(&raw mut (*data).parse) as TTL;
    }
    len = read_uint16(&raw mut (*data).parse) as size_t;
    rest = (*data).packet.size.wrapping_sub(
        (*data).parse.ptr.offset_from((*data).packet.ptr) as ::core::ffi::c_long as size_t
    );
    if len > rest {
        return RCODE_FORMAT_ERROR;
    }
    match (*pans).generic.type_0 as ::core::ffi::c_uint {
        1 => return decode_rr_a(data, &raw mut (*pans).a, len),
        6 => return decode_rr_soa(data, &raw mut (*pans).soa, len),
        35 => return decode_rr_naptr(data, &raw mut (*pans).naptr, len),
        28 => return decode_rr_aaaa(data, &raw mut (*pans).aaaa, len),
        33 => return decode_rr_srv(data, &raw mut (*pans).srv, len),
        11 => return decode_rr_wks(data, &raw mut (*pans).wks, len),
        27 => return decode_rr_gpos(data, &raw mut (*pans).gpos),
        29 => return decode_rr_loc(data, &raw mut (*pans).loc, len),
        41 => return decode_rr_opt(data, &raw mut (*pans).opt, len),
        26 | 17 | 14 => return decode_rr_minfo(data, &raw mut (*pans).minfo),
        18 | 21 | 15 => return decode_rr_mx(data, &raw mut (*pans).mx, len),
        22 | 20 | 13 => return decode_rr_hinfo(data, &raw mut (*pans).hinfo),
        19 | 99 | 16 => return decode_rr_txt(data, &raw mut (*pans).txt, len),
        23 | 3 | 4 | 7 | 8 | 9 | 2 | 12 | 5 => {
            return read_domain(data, &raw mut (*pans).cname.cname);
        }
        10 | _ => {
            (*pans).x.size = len;
            return read_raw(data, &raw mut (*pans).x.rawdata, len);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn dns_decode(
    mut presponse: *mut dns_decoded_t,
    mut prsize: *mut size_t,
    mut buffer: *const dns_packet_t,
    mut len: size_t,
) -> dns_rcode_t {
    let mut header: *const idns_header = ::core::ptr::null::<idns_header>();
    let mut response: *mut dns_query_t = ::core::ptr::null_mut::<dns_query_t>();
    let mut context: ddns_context = ddns_context {
        packet: block {
            size: 0,
            ptr: ::core::ptr::null_mut::<uint8_t>(),
        },
        parse: block {
            size: 0,
            ptr: ::core::ptr::null_mut::<uint8_t>(),
        },
        dest: block {
            size: 0,
            ptr: ::core::ptr::null_mut::<uint8_t>(),
        },
        response: ::core::ptr::null_mut::<dns_query_t>(),
        edns: false,
    };
    let mut rc: dns_rcode_t = RCODE_OKAY;
    '_c2rust_label: {
        if !presponse.is_null() {
        } else {
            __assert_fail(
                b"presponse != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                2105 as ::core::ffi::c_uint,
                b"dns_rcode_t dns_decode(dns_decoded_t *, size_t *, const dns_packet_t *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !prsize.is_null() {
        } else {
            __assert_fail(
                b"prsize != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                2106 as ::core::ffi::c_uint,
                b"dns_rcode_t dns_decode(dns_decoded_t *, size_t *, const dns_packet_t *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if *prsize >= ::core::mem::size_of::<dns_query_t>() as usize {
        } else {
            __assert_fail(
                b"*prsize >= sizeof(dns_query_t)\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                2107 as ::core::ffi::c_uint,
                b"dns_rcode_t dns_decode(dns_decoded_t *, size_t *, const dns_packet_t *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_2: {
        if !buffer.is_null() {
        } else {
            __assert_fail(
                b"buffer != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                2108 as ::core::ffi::c_uint,
                b"dns_rcode_t dns_decode(dns_decoded_t *, size_t *, const dns_packet_t *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    if len < ::core::mem::size_of::<idns_header>() as usize {
        return RCODE_FORMAT_ERROR;
    }
    context.packet.ptr = buffer as *mut uint8_t;
    context.packet.size = len;
    context.parse.ptr = context
        .packet
        .ptr
        .offset(::core::mem::size_of::<idns_header>() as isize)
        as *mut uint8_t;
    context.parse.size = len.wrapping_sub(::core::mem::size_of::<idns_header>() as size_t);
    context.dest.ptr = presponse as *mut uint8_t;
    context.dest.size = *prsize;
    context.edns = false_0 != 0;
    response = context.dest.ptr as *mut dns_query_t;
    context.response = alloc_struct(
        &raw mut context.dest,
        ::core::mem::size_of::<dns_query_t>() as size_t,
    ) as *mut dns_query_t;
    '_c2rust_label_3: {
        if !context.response.is_null() {
        } else {
            __assert_fail(
                b"context.response != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                2134 as ::core::ffi::c_uint,
                b"dns_rcode_t dns_decode(dns_decoded_t *, size_t *, const dns_packet_t *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_4: {
        if context.response == response {
        } else {
            __assert_fail(
                b"context.response == response\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/codec.c\0".as_ptr() as *const ::core::ffi::c_char,
                2135 as ::core::ffi::c_uint,
                b"dns_rcode_t dns_decode(dns_decoded_t *, size_t *, const dns_packet_t *, size_t)\0"
                    .as_ptr() as *const ::core::ffi::c_char,
            );
        }
    };
    memset(
        response as *mut ::core::ffi::c_void,
        0 as ::core::ffi::c_int,
        ::core::mem::size_of::<dns_query_t>() as size_t,
    );
    (*response).questions = ::core::ptr::null_mut::<dns_question_t>();
    (*response).answers = ::core::ptr::null_mut::<dns_answer_t>();
    (*response).nameservers = ::core::ptr::null_mut::<dns_answer_t>();
    (*response).additional = ::core::ptr::null_mut::<dns_answer_t>();
    header = buffer as *mut idns_header;
    (*response).id = __bswap_16((*header).id as __uint16_t) as ::core::ffi::c_int;
    (*response).opcode = ((*header).opcode as ::core::ffi::c_int >> 3 as ::core::ffi::c_int
        & 0xf as ::core::ffi::c_int) as dns_op_t;
    (*response).query = (*header).opcode as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
        != 0x80 as ::core::ffi::c_int;
    (*response).aa = (*header).opcode as ::core::ffi::c_int & 0x4 as ::core::ffi::c_int
        == 0x4 as ::core::ffi::c_int;
    (*response).tc = (*header).opcode as ::core::ffi::c_int & 0x2 as ::core::ffi::c_int
        == 0x2 as ::core::ffi::c_int;
    (*response).rd = (*header).opcode as ::core::ffi::c_int & 0x1 as ::core::ffi::c_int
        == 0x1 as ::core::ffi::c_int;
    (*response).ra = (*header).rcode as ::core::ffi::c_int & 0x80 as ::core::ffi::c_int
        == 0x80 as ::core::ffi::c_int;
    (*response).z = (*header).rcode as ::core::ffi::c_int & 0x40 as ::core::ffi::c_int
        == 0x40 as ::core::ffi::c_int;
    (*response).ad = (*header).rcode as ::core::ffi::c_int & 0x20 as ::core::ffi::c_int
        == 0x20 as ::core::ffi::c_int;
    (*response).cd = (*header).rcode as ::core::ffi::c_int & 0x10 as ::core::ffi::c_int
        == 0x10 as ::core::ffi::c_int;
    (*response).rcode =
        ((*header).rcode as ::core::ffi::c_int & 0xf as ::core::ffi::c_int) as dns_rcode_t;
    (*response).qdcount = __bswap_16((*header).qdcount as __uint16_t) as size_t;
    (*response).ancount = __bswap_16((*header).ancount as __uint16_t) as size_t;
    (*response).nscount = __bswap_16((*header).nscount as __uint16_t) as size_t;
    (*response).arcount = __bswap_16((*header).arcount as __uint16_t) as size_t;
    (*response).questions = alloc_struct(
        &raw mut context.dest,
        (*response)
            .qdcount
            .wrapping_mul(::core::mem::size_of::<dns_question_t>() as size_t),
    ) as *mut dns_question_t;
    (*response).answers = alloc_struct(
        &raw mut context.dest,
        (*response)
            .ancount
            .wrapping_mul(::core::mem::size_of::<dns_answer_t>() as size_t),
    ) as *mut dns_answer_t;
    (*response).nameservers = alloc_struct(
        &raw mut context.dest,
        (*response)
            .nscount
            .wrapping_mul(::core::mem::size_of::<dns_answer_t>() as size_t),
    ) as *mut dns_answer_t;
    (*response).additional = alloc_struct(
        &raw mut context.dest,
        (*response)
            .arcount
            .wrapping_mul(::core::mem::size_of::<dns_answer_t>() as size_t),
    ) as *mut dns_answer_t;
    if (*response).qdcount != 0 && (*response).questions.is_null()
        || (*response).ancount != 0 && (*response).answers.is_null()
        || (*response).nscount != 0 && (*response).nameservers.is_null()
        || (*response).arcount != 0 && (*response).additional.is_null()
    {
        return RCODE_NO_MEMORY;
    }
    let mut i: size_t = 0 as size_t;
    while i < (*response).qdcount {
        rc = decode_question(
            &raw mut context,
            (*response).questions.offset(i as isize) as *mut dns_question_t,
        );
        if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
            return rc;
        }
        i = i.wrapping_add(1);
    }
    let mut i_0: size_t = 0 as size_t;
    while i_0 < (*response).ancount {
        rc = decode_answer(
            &raw mut context,
            (*response).answers.offset(i_0 as isize) as *mut dns_answer_t,
        );
        if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
            return rc;
        }
        i_0 = i_0.wrapping_add(1);
    }
    let mut i_1: size_t = 0 as size_t;
    while i_1 < (*response).nscount {
        rc = decode_answer(
            &raw mut context,
            (*response).nameservers.offset(i_1 as isize) as *mut dns_answer_t,
        );
        if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
            return rc;
        }
        i_1 = i_1.wrapping_add(1);
    }
    let mut i_2: size_t = 0 as size_t;
    while i_2 < (*response).arcount {
        rc = decode_answer(
            &raw mut context,
            (*response).additional.offset(i_2 as isize) as *mut dns_answer_t,
        );
        if rc as ::core::ffi::c_uint != RCODE_OKAY as ::core::ffi::c_int as ::core::ffi::c_uint {
            return rc;
        }
        i_2 = i_2.wrapping_add(1);
    }
    *prsize =
        context.dest.ptr.offset_from(presponse as *mut uint8_t) as ::core::ffi::c_long as size_t;
    return RCODE_OKAY;
}
pub const NULL: *mut ::core::ffi::c_void =
    ::core::ptr::null::<::core::ffi::c_void>() as *mut ::core::ffi::c_void;
pub const true_0: ::core::ffi::c_int = 1 as ::core::ffi::c_int;
pub const false_0: ::core::ffi::c_int = 0 as ::core::ffi::c_int;

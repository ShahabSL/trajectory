extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const ::core::ffi::c_ushort;
    fn __assert_fail(
        __assertion: *const ::core::ffi::c_char,
        __file: *const ::core::ffi::c_char,
        __line: ::core::ffi::c_uint,
        __function: *const ::core::ffi::c_char,
    ) -> !;
    fn inet_ntop(
        __af: ::core::ffi::c_int,
        __cp: *const ::core::ffi::c_void,
        __buf: *mut ::core::ffi::c_char,
        __len: socklen_t,
    ) -> *const ::core::ffi::c_char;
    fn dns_rcode_text(_: dns_rcode_t) -> *const ::core::ffi::c_char;
    fn dns_type_text(_: dns_type_t) -> *const ::core::ffi::c_char;
    fn dns_class_text(_: dns_class_t) -> *const ::core::ffi::c_char;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const ::core::ffi::c_char,
        ...
    ) -> ::core::ffi::c_int;
    fn printf(__format: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
    fn fputs(__s: *const ::core::ffi::c_char, __stream: *mut FILE) -> ::core::ffi::c_int;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = ::core::ffi::c_long;
pub type __off64_t = ::core::ffi::c_long;
pub type __socklen_t = ::core::ffi::c_uint;
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
pub type size_t = usize;
pub type socklen_t = __socklen_t;
pub type in_addr_t = uint32_t;
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
pub const PF_INET: ::core::ffi::c_int = 2 as ::core::ffi::c_int;
pub const PF_INET6: ::core::ffi::c_int = 10 as ::core::ffi::c_int;
pub const AF_INET: ::core::ffi::c_int = PF_INET;
pub const AF_INET6: ::core::ffi::c_int = PF_INET6;
#[no_mangle]
pub unsafe extern "C" fn dns_print_result(mut presult: *mut dns_query_t) {
    dns_print_header(presult);
    dns_print_question(
        b"QUESTIONS\0".as_ptr() as *const ::core::ffi::c_char,
        (*presult).questions,
        (*presult).qdcount,
    );
    dns_print_answer(
        b"ANSWERS\0".as_ptr() as *const ::core::ffi::c_char,
        (*presult).answers,
        (*presult).ancount,
    );
    dns_print_answer(
        b"NAMESERVERS\0".as_ptr() as *const ::core::ffi::c_char,
        (*presult).nameservers,
        (*presult).nscount,
    );
    dns_print_answer(
        b"ADDITIONAL\0".as_ptr() as *const ::core::ffi::c_char,
        (*presult).additional,
        (*presult).arcount,
    );
}
#[no_mangle]
pub unsafe extern "C" fn dns_print_header(mut presult: *mut dns_query_t) {
    printf(
        b"; Questions            = %lu\n; Answers              = %lu\n; Name Servers         = %lu\n; Additional Records   = %lu\n; Authoritative Result = %s\n; Truncated Result     = %s\n; Recursion Desired    = %s\n; Recursion Available  = %s\n; Authentic Data       = %s\n; Checking disabled    = %s\n; Result               = %s\n\0"
            .as_ptr() as *const ::core::ffi::c_char,
        (*presult).qdcount as ::core::ffi::c_ulong,
        (*presult).ancount as ::core::ffi::c_ulong,
        (*presult).nscount as ::core::ffi::c_ulong,
        (*presult).arcount as ::core::ffi::c_ulong,
        if (*presult).aa as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
        if (*presult).tc as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
        if (*presult).rd as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
        if (*presult).ra as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
        if (*presult).ad as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
        if (*presult).cd as ::core::ffi::c_int != 0 {
            b"true\0".as_ptr() as *const ::core::ffi::c_char
        } else {
            b"false\0".as_ptr() as *const ::core::ffi::c_char
        },
        dns_rcode_text((*presult).rcode),
    );
}
#[no_mangle]
pub unsafe extern "C" fn dns_print_question(
    mut tag: *const ::core::ffi::c_char,
    mut pquest: *mut dns_question_t,
    mut cnt: size_t,
) {
    '_c2rust_label: {
        if !tag.is_null() {
        } else {
            __assert_fail(
                b"tag != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                84 as ::core::ffi::c_uint,
                b"void dns_print_question(const char *, dns_question_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !pquest.is_null() {
        } else {
            __assert_fail(
                b"pquest != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                85 as ::core::ffi::c_uint,
                b"void dns_print_question(const char *, dns_question_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(
        b"\n;;; %s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        tag,
    );
    let mut i: size_t = 0 as size_t;
    while i < cnt {
        printf(
            b";%s %s %s\n\0".as_ptr() as *const ::core::ffi::c_char,
            (*pquest.offset(i as isize)).name,
            dns_class_text((*pquest.offset(i as isize)).class),
            dns_type_text((*pquest.offset(i as isize)).type_0),
        );
        i = i.wrapping_add(1);
    }
}
#[no_mangle]
pub unsafe extern "C" fn dns_print_answer(
    mut tag: *const ::core::ffi::c_char,
    mut pans: *mut dns_answer_t,
    mut cnt: size_t,
) {
    let mut ipaddr: [::core::ffi::c_char; 46] = [0; 46];
    '_c2rust_label: {
        if !tag.is_null() {
        } else {
            __assert_fail(
                b"tag != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                105 as ::core::ffi::c_uint,
                b"void dns_print_answer(const char *, dns_answer_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !pans.is_null() {
        } else {
            __assert_fail(
                b"pans != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                106 as ::core::ffi::c_uint,
                b"void dns_print_answer(const char *, dns_answer_t *, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    printf(
        b"\n;;; %s\n\n\0".as_ptr() as *const ::core::ffi::c_char,
        tag,
    );
    let mut i: size_t = 0 as size_t;
    while i < cnt {
        if (*pans.offset(i as isize)).generic.type_0 as ::core::ffi::c_uint
            != RR_OPT as ::core::ffi::c_int as ::core::ffi::c_uint
        {
            printf(
                b"%-16s\t%5lu\t%s\t%s\t\0".as_ptr() as *const ::core::ffi::c_char,
                (*pans.offset(i as isize)).generic.name,
                (*pans.offset(i as isize)).generic.ttl as ::core::ffi::c_ulong,
                dns_class_text((*pans.offset(i as isize)).generic.class),
                dns_type_text((*pans.offset(i as isize)).generic.type_0),
            );
        } else {
            printf(b"; OPT RR\0".as_ptr() as *const ::core::ffi::c_char);
        }
        match (*pans.offset(i as isize)).generic.type_0 as ::core::ffi::c_uint {
            2 => {
                printf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pans.offset(i as isize)).ns.nsdname,
                );
            }
            1 => {
                inet_ntop(
                    AF_INET,
                    &raw mut (*pans.offset(i as isize)).a.address as *const ::core::ffi::c_void,
                    &raw mut ipaddr as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 46]>() as socklen_t,
                );
                printf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut ipaddr as *mut ::core::ffi::c_char,
                );
            }
            28 => {
                inet_ntop(
                    AF_INET6,
                    &raw mut (*pans.offset(i as isize)).aaaa.address as *const ::core::ffi::c_void,
                    &raw mut ipaddr as *mut ::core::ffi::c_char,
                    ::core::mem::size_of::<[::core::ffi::c_char; 46]>() as socklen_t,
                );
                printf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    &raw mut ipaddr as *mut ::core::ffi::c_char,
                );
            }
            5 => {
                printf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pans.offset(i as isize)).cname.cname,
                );
            }
            15 => {
                printf(
                    b"%5d %s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pans.offset(i as isize)).mx.preference,
                    (*pans.offset(i as isize)).mx.exchange,
                );
            }
            12 => {
                printf(
                    b"%s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pans.offset(i as isize)).ptr.ptr,
                );
            }
            13 => {
                printf(
                    b"\"%s\" \"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pans.offset(i as isize)).hinfo.cpu,
                    (*pans.offset(i as isize)).hinfo.os,
                );
            }
            14 => {
                printf(
                    b"(\n\t\t\"%s\"\n\t\t\"%s\" )\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pans.offset(i as isize)).minfo.rmailbx,
                    (*pans.offset(i as isize)).minfo.emailbx,
                );
            }
            99 | 16 => {
                if (*pans.offset(i as isize)).txt.len < 30 as size_t {
                    printf(
                        b"\"%s\"\0".as_ptr() as *const ::core::ffi::c_char,
                        (*pans.offset(i as isize)).txt.text,
                    );
                } else {
                    let mut len: size_t = 0;
                    let mut max: ::core::ffi::c_int = 0;
                    let mut off: size_t = 0;
                    printf(b"(\0".as_ptr() as *const ::core::ffi::c_char);
                    len = (*pans.offset(i as isize)).txt.len;
                    off = 0 as size_t;
                    while len != 0 {
                        max = if len > 64 as size_t {
                            64 as ::core::ffi::c_int
                        } else {
                            len as ::core::ffi::c_int
                        };
                        printf(
                            b"\n\t\"%*.*s\"\0".as_ptr() as *const ::core::ffi::c_char,
                            max,
                            max,
                            (*pans.offset(i as isize)).txt.text.offset(off as isize)
                                as *const ::core::ffi::c_char,
                        );
                        off = off.wrapping_add(max as size_t);
                        len = len.wrapping_sub(max as size_t);
                    }
                    printf(b"\n\t\t)\n\0".as_ptr() as *const ::core::ffi::c_char);
                }
            }
            6 => {
                printf(
                    b"%s %s (\n\t\t%10lu   ; Serial\n\t\t%10lu   ; Refresh\n\t\t%10lu   ; Retry\n\t\t%10lu   ; Expire\n\t\t%10lu ) ; Miminum\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    (*pans.offset(i as isize)).soa.mname,
                    (*pans.offset(i as isize)).soa.rname,
                    (*pans.offset(i as isize)).soa.serial as ::core::ffi::c_ulong,
                    (*pans.offset(i as isize)).soa.refresh as ::core::ffi::c_ulong,
                    (*pans.offset(i as isize)).soa.retry as ::core::ffi::c_ulong,
                    (*pans.offset(i as isize)).soa.expire as ::core::ffi::c_ulong,
                    (*pans.offset(i as isize)).soa.minimum as ::core::ffi::c_ulong,
                );
            }
            35 => {
                printf(
                    b"%5d %5d (\n\t\t\"%s\"\n\t\t\"%s\"\n\t\t\"%s\"\n\t\t%s )\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*pans.offset(i as isize)).naptr.order,
                    (*pans.offset(i as isize)).naptr.preference,
                    (*pans.offset(i as isize)).naptr.flags,
                    (*pans.offset(i as isize)).naptr.services,
                    (*pans.offset(i as isize)).naptr.regexp,
                    (*pans.offset(i as isize)).naptr.replacement,
                );
            }
            29 => {
                printf(
                    b"(\n\t\t%3d %2d %2d %s ; Latitude\n\t\t%3d %2d %2d %s ; Longitude\n\t\t%11ld ; Altitude\n\t\t%11llu ; Size\n\t\t%11llu ; Horizontal Precision\n\t\t%11llu ; Vertical Precision\n\t\t)\n\0"
                        .as_ptr() as *const ::core::ffi::c_char,
                    (*pans.offset(i as isize)).loc.latitude.deg,
                    (*pans.offset(i as isize)).loc.latitude.min,
                    (*pans.offset(i as isize)).loc.latitude.sec,
                    if (*pans.offset(i as isize)).loc.latitude.nw as ::core::ffi::c_int
                        != 0
                    {
                        b"N\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"S\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    (*pans.offset(i as isize)).loc.longitude.deg,
                    (*pans.offset(i as isize)).loc.longitude.min,
                    (*pans.offset(i as isize)).loc.longitude.sec,
                    if (*pans.offset(i as isize)).loc.longitude.nw as ::core::ffi::c_int
                        != 0
                    {
                        b"W\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"E\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    (*pans.offset(i as isize)).loc.altitude,
                    (*pans.offset(i as isize)).loc.size,
                    (*pans.offset(i as isize)).loc.horiz_pre,
                    (*pans.offset(i as isize)).loc.vert_pre,
                );
            }
            33 => {
                printf(
                    b"%5d %5d %5d %s\0".as_ptr() as *const ::core::ffi::c_char,
                    (*pans.offset(i as isize)).srv.priority,
                    (*pans.offset(i as isize)).srv.weight,
                    (*pans.offset(i as isize)).srv.port,
                    (*pans.offset(i as isize)).srv.target,
                );
            }
            41 => {
                printf(
                    b"\n;\tpayload = %lu\n;\tDO      = %s\n;\t#opts   = %lu\n\0".as_ptr()
                        as *const ::core::ffi::c_char,
                    (*pans.offset(i as isize)).opt.udp_payload as ::core::ffi::c_ulong,
                    if (*pans.offset(i as isize)).opt.fdo as ::core::ffi::c_int != 0 {
                        b"true\0".as_ptr() as *const ::core::ffi::c_char
                    } else {
                        b"false\0".as_ptr() as *const ::core::ffi::c_char
                    },
                    (*pans.offset(i as isize)).opt.numopts as ::core::ffi::c_ulong,
                );
            }
            _ => {}
        }
        printf(b"\n\0".as_ptr() as *const ::core::ffi::c_char);
        i = i.wrapping_add(1);
    }
}
pub const LINESIZE: ::core::ffi::c_int = 16 as ::core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn dns_dump_memory(
    mut out: *mut FILE,
    mut data: *const ::core::ffi::c_void,
    mut size: size_t,
    mut offset: size_t,
) {
    let mut block: *const ::core::ffi::c_uchar = data as *const ::core::ffi::c_uchar;
    let mut ascii: [::core::ffi::c_char; 17] = [0; 17];
    let mut skip: ::core::ffi::c_int = 0;
    let mut j: ::core::ffi::c_int = 0;
    '_c2rust_label: {
        if !out.is_null() {
        } else {
            __assert_fail(
                b"out != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                273 as ::core::ffi::c_uint,
                b"void dns_dump_memory(FILE *, const void *, size_t, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_0: {
        if !block.is_null() {
        } else {
            __assert_fail(
                b"block != NULL\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                274 as ::core::ffi::c_uint,
                b"void dns_dump_memory(FILE *, const void *, size_t, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    '_c2rust_label_1: {
        if size > 0 as size_t {
        } else {
            __assert_fail(
                b"size > 0\0".as_ptr() as *const ::core::ffi::c_char,
                b"../extern/SPCDNS/src/output.c\0".as_ptr() as *const ::core::ffi::c_char,
                275 as ::core::ffi::c_uint,
                b"void dns_dump_memory(FILE *, const void *, size_t, size_t)\0".as_ptr()
                    as *const ::core::ffi::c_char,
            );
        }
    };
    while size > 0 as size_t {
        fprintf(
            out,
            b"%08lX: \0".as_ptr() as *const ::core::ffi::c_char,
            offset as ::core::ffi::c_ulong,
        );
        skip = offset.wrapping_rem(LINESIZE as size_t) as ::core::ffi::c_int;
        j = 0 as ::core::ffi::c_int;
        while skip != 0 {
            fputs(b"   \0".as_ptr() as *const ::core::ffi::c_char, out);
            ascii[j as usize] = ' ' as i32 as ::core::ffi::c_char;
            j += 1;
            skip -= 1;
        }
        loop {
            fprintf(
                out,
                b"%02x \0".as_ptr() as *const ::core::ffi::c_char,
                *block as ::core::ffi::c_int,
            );
            if *(*__ctype_b_loc()).offset(*block as ::core::ffi::c_int as isize)
                as ::core::ffi::c_int
                & _ISprint as ::core::ffi::c_int as ::core::ffi::c_ushort as ::core::ffi::c_int
                != 0
            {
                ascii[j as usize] = *block as ::core::ffi::c_char;
            } else {
                ascii[j as usize] = '.' as i32 as ::core::ffi::c_char;
            }
            block = block.offset(1);
            offset = offset.wrapping_add(1);
            j += 1;
            size = size.wrapping_sub(1);
            if !(j < LINESIZE && size > 0 as size_t) {
                break;
            }
        }
        ascii[j as usize] = '\0' as i32 as ::core::ffi::c_char;
        if j < LINESIZE {
            let mut i: ::core::ffi::c_int = 0;
            i = j;
            while i < LINESIZE {
                fputs(b"   \0".as_ptr() as *const ::core::ffi::c_char, out);
                i += 1;
            }
        }
        fprintf(
            out,
            b"%s\n\0".as_ptr() as *const ::core::ffi::c_char,
            &raw mut ascii as *mut ::core::ffi::c_char,
        );
    }
}

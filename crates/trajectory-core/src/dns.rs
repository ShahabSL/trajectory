use anyhow::{bail, Context, Result};

pub const TYPE_TXT: u16 = 16;
pub const TYPE_A: u16 = 1;
pub const TYPE_AAAA: u16 = 28;
pub const TYPE_NS: u16 = 2;
pub const TYPE_SOA: u16 = 6;
pub const CLASS_IN: u16 = 1;
pub const TYPE_OPT: u16 = 41;
const BASE36_BLOCK_BYTES: usize = 31;
const BASE36_FULL_BLOCK_CHARS: usize = 48;
const BASE36_FIRST_LABEL_CHARS: usize = 61;
const BASE36_COMPACT_FIRST_LABEL_CHARS: usize = 62;
const BASE36_NEXT_LABEL_CHARS: usize = 63;
const BASE36_WIDTHS: [usize; BASE36_BLOCK_BYTES + 1] = [
    0, 2, 4, 5, 7, 8, 10, 11, 13, 14, 16, 18, 19, 21, 22, 24, 25, 27, 28, 30, 31, 33, 35, 36, 38,
    39, 41, 42, 44, 45, 47, 48,
];

#[derive(Clone, Debug)]
pub struct DnsQuery {
    pub id: u16,
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16,
    pub udp_payload_size: Option<u16>,
    question: Vec<u8>,
}

pub fn txt_response_wire_len(query: &DnsQuery, envelope_len: usize) -> usize {
    let rdata_len = if envelope_len == 0 {
        1
    } else {
        envelope_len + envelope_len.div_ceil(255)
    };
    12 + query.question.len() + 12 + rdata_len
}

pub fn envelope_qname_len(envelope_len: usize, domain: &str) -> Result<usize> {
    envelope_qname_len_with_first_label(envelope_len, domain, BASE36_FIRST_LABEL_CHARS, 2)
}

pub fn compact_envelope_qname_len(envelope_len: usize, domain: &str) -> Result<usize> {
    envelope_qname_len_with_first_label(envelope_len, domain, BASE36_COMPACT_FIRST_LABEL_CHARS, 1)
}

fn envelope_qname_len_with_first_label(
    envelope_len: usize,
    domain: &str,
    first_label_payload_chars: usize,
    prefix_chars: usize,
) -> Result<usize> {
    let encoded_len = base36_encoded_len(envelope_len)?;
    let label_count = 1 + encoded_len
        .saturating_sub(first_label_payload_chars)
        .div_ceil(BASE36_NEXT_LABEL_CHARS);
    let qname_len = encoded_len
        .checked_add(prefix_chars)
        .and_then(|len| len.checked_add(label_count))
        .and_then(|len| len.checked_add(normalize_domain(domain).len()))
        .context("encoded DNS query name length overflow")?;
    Ok(qname_len)
}

pub fn envelope_to_qname(envelope: &[u8], domain: &str) -> Result<String> {
    envelope_to_qname_with_prefix(envelope, domain, 't', true)
}

pub fn envelope_to_compact_qname(envelope: &[u8], domain: &str) -> Result<String> {
    envelope_to_qname_with_prefix(envelope, domain, 'u', false)
}

fn envelope_to_qname_with_prefix(
    envelope: &[u8],
    domain: &str,
    prefix: char,
    include_remainder: bool,
) -> Result<String> {
    let encoded = base36_encode_envelope(envelope)?;
    let mut labels = Vec::new();
    let mut first = String::with_capacity(63);
    first.push(prefix);
    if include_remainder {
        let remainder = base36_remainder_len(envelope.len());
        first.push(base36_digit(remainder as u8));
    }
    let first_label_payload_chars = if include_remainder {
        BASE36_FIRST_LABEL_CHARS
    } else {
        BASE36_COMPACT_FIRST_LABEL_CHARS
    };
    let mut pos = encoded.len().min(first_label_payload_chars);
    first.push_str(&encoded[..pos]);
    labels.push(first);
    while pos < encoded.len() {
        let end = (pos + BASE36_NEXT_LABEL_CHARS).min(encoded.len());
        let chunk = &encoded[pos..end];
        labels.push(chunk.to_string());
        pos = end;
    }
    labels.push(normalize_domain(domain));
    let qname = labels.join(".");
    if qname.len() > 253 {
        bail!("encoded DNS query name exceeds 253 bytes");
    }
    Ok(qname)
}

pub fn qname_to_envelope(qname: &str, domain: &str) -> Result<Vec<u8>> {
    let qname = normalize_domain(qname);
    let domain = normalize_domain(domain);
    let suffix = format!(".{domain}");
    let left = qname
        .strip_suffix(&suffix)
        .or_else(|| (qname == domain).then_some(""))
        .context("query name is outside tunnel domain")?;
    let mut encoded = String::new();
    let mut labels = left
        .split('.')
        .filter(|label| !label.is_empty())
        .enumerate();
    let Some((_, first_label)) = labels.next() else {
        bail!("empty tunnel payload");
    };
    let explicit_remainder = if let Some(first_payload) = first_label.strip_prefix('t') {
        let mut first_chars = first_payload.chars();
        let remainder_char = first_chars
            .next()
            .context("tunnel label missing block remainder")?;
        let remainder =
            base36_value(remainder_char).context("invalid tunnel block remainder")? as usize;
        encoded.extend(first_chars);
        Some(remainder)
    } else if let Some(first_payload) = first_label.strip_prefix('u') {
        encoded.extend(first_payload.chars());
        None
    } else {
        bail!("tunnel label missing known prefix");
    };
    for (_, label) in labels {
        encoded.push_str(label);
    }
    let remainder = match explicit_remainder {
        Some(0) if !encoded.is_empty() => bail!("invalid explicit tunnel block remainder"),
        Some(remainder) => remainder,
        None => infer_base36_remainder(encoded.len())?,
    };
    base36_decode_envelope(&encoded, remainder)
}

fn base36_remainder_len(envelope_len: usize) -> usize {
    match envelope_len % BASE36_BLOCK_BYTES {
        0 if envelope_len > 0 => BASE36_BLOCK_BYTES,
        remainder => remainder,
    }
}

fn base36_encoded_len(envelope_len: usize) -> Result<usize> {
    if envelope_len == 0 {
        return Ok(0);
    }
    let full_blocks = envelope_len / BASE36_BLOCK_BYTES;
    let remainder = envelope_len % BASE36_BLOCK_BYTES;
    let full_len = full_blocks
        .checked_mul(BASE36_FULL_BLOCK_CHARS)
        .context("encoded DNS query name length overflow")?;
    if remainder == 0 {
        Ok(full_len)
    } else {
        full_len
            .checked_add(BASE36_WIDTHS[remainder])
            .context("encoded DNS query name length overflow")
    }
}

fn infer_base36_remainder(encoded_len: usize) -> Result<usize> {
    if encoded_len == 0 {
        return Ok(0);
    }
    let final_width = encoded_len % BASE36_FULL_BLOCK_CHARS;
    if final_width == 0 {
        return Ok(BASE36_BLOCK_BYTES);
    }
    BASE36_WIDTHS
        .iter()
        .position(|width| *width == final_width)
        .filter(|remainder| *remainder > 0)
        .context("invalid compact base36 tunnel payload length")
}

fn base36_encode_envelope(envelope: &[u8]) -> Result<String> {
    let mut encoded = String::with_capacity(base36_encoded_len(envelope.len())?);
    for chunk in envelope.chunks(BASE36_BLOCK_BYTES) {
        base36_encode_block(chunk, &mut encoded)?;
    }
    Ok(encoded)
}

fn base36_encode_block(block: &[u8], out: &mut String) -> Result<()> {
    let width = *BASE36_WIDTHS
        .get(block.len())
        .context("invalid base36 block length")?;
    if block.is_empty() {
        return Ok(());
    }
    let mut work = block.to_vec();
    let mut digits = Vec::new();
    while work.iter().any(|byte| *byte != 0) {
        let mut carry = 0u16;
        for byte in &mut work {
            let value = (carry << 8) | u16::from(*byte);
            *byte = (value / 36) as u8;
            carry = value % 36;
        }
        digits.push(base36_digit(carry as u8));
    }
    if digits.len() > width {
        bail!("base36 block exceeded fixed width");
    }
    for _ in digits.len()..width {
        out.push('0');
    }
    for digit in digits.iter().rev() {
        out.push(*digit);
    }
    Ok(())
}

fn base36_decode_envelope(encoded: &str, remainder: usize) -> Result<Vec<u8>> {
    if remainder == 0 {
        if encoded.is_empty() {
            return Ok(Vec::new());
        }
        bail!("empty tunnel remainder with non-empty payload");
    }
    if remainder > BASE36_BLOCK_BYTES {
        bail!("invalid tunnel block remainder");
    }
    let last_width = BASE36_WIDTHS[remainder];
    if encoded.len() < last_width {
        bail!("truncated base36 tunnel payload");
    }
    let full_width = encoded.len() - last_width;
    if !full_width.is_multiple_of(BASE36_FULL_BLOCK_CHARS) {
        bail!("invalid base36 tunnel payload length");
    }
    let mut out =
        Vec::with_capacity((full_width / BASE36_FULL_BLOCK_CHARS) * BASE36_BLOCK_BYTES + remainder);
    let mut pos = 0;
    while pos < full_width {
        let block = base36_decode_block(
            &encoded[pos..pos + BASE36_FULL_BLOCK_CHARS],
            BASE36_BLOCK_BYTES,
        )?;
        out.extend_from_slice(&block);
        pos += BASE36_FULL_BLOCK_CHARS;
    }
    let block = base36_decode_block(&encoded[pos..], remainder)?;
    out.extend_from_slice(&block);
    Ok(out)
}

fn base36_decode_block(digits: &str, byte_len: usize) -> Result<Vec<u8>> {
    if digits.len()
        != *BASE36_WIDTHS
            .get(byte_len)
            .context("invalid base36 block length")?
    {
        bail!("invalid base36 block width");
    }
    let mut out = vec![0u8; byte_len];
    for ch in digits.chars() {
        let mut carry = u16::from(base36_value(ch).context("invalid base36 tunnel payload")?);
        for byte in out.iter_mut().rev() {
            let value = u16::from(*byte) * 36 + carry;
            *byte = (value & 0xff) as u8;
            carry = value >> 8;
        }
        if carry != 0 {
            bail!("base36 tunnel payload overflows block");
        }
    }
    Ok(out)
}

fn base36_digit(value: u8) -> char {
    match value {
        0..=9 => (b'0' + value) as char,
        10..=35 => (b'a' + (value - 10)) as char,
        _ => unreachable!("base36 digit out of range"),
    }
}

fn base36_value(value: char) -> Option<u8> {
    match value {
        '0'..='9' => Some(value as u8 - b'0'),
        'a'..='z' => Some(value as u8 - b'a' + 10),
        'A'..='Z' => Some(value as u8 - b'A' + 10),
        _ => None,
    }
}

pub fn build_query(id: u16, qname: &str, udp_payload_size: u16) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(512);
    put_u16(&mut out, id);
    put_u16(&mut out, 0x0100);
    put_u16(&mut out, 1);
    put_u16(&mut out, 0);
    put_u16(&mut out, 0);
    put_u16(&mut out, 1);
    encode_name(&mut out, qname)?;
    put_u16(&mut out, TYPE_TXT);
    put_u16(&mut out, CLASS_IN);

    out.push(0);
    put_u16(&mut out, TYPE_OPT);
    put_u16(&mut out, udp_payload_size);
    put_u32(&mut out, 0);
    put_u16(&mut out, 0);
    Ok(out)
}

pub fn parse_query(bytes: &[u8]) -> Result<DnsQuery> {
    if bytes.len() < 12 {
        bail!("DNS query too short");
    }
    let id = u16::from_be_bytes(bytes[0..2].try_into().unwrap());
    let qdcount = u16::from_be_bytes(bytes[4..6].try_into().unwrap());
    let arcount = u16::from_be_bytes(bytes[10..12].try_into().unwrap()) as usize;
    if qdcount == 0 {
        bail!("DNS query has no question");
    }
    let (qname, after_name) = decode_name(bytes, 12)?;
    if after_name + 4 > bytes.len() {
        bail!("DNS question truncated");
    }
    let qtype = u16::from_be_bytes(bytes[after_name..after_name + 2].try_into().unwrap());
    let qclass = u16::from_be_bytes(bytes[after_name + 2..after_name + 4].try_into().unwrap());
    let mut pos = after_name + 4;
    let mut udp_payload_size = None;
    for _ in 0..arcount {
        let (_name, after_additional_name) = decode_name(bytes, pos)?;
        if after_additional_name + 10 > bytes.len() {
            bail!("DNS additional record truncated");
        }
        let rr_type = u16::from_be_bytes(
            bytes[after_additional_name..after_additional_name + 2]
                .try_into()
                .unwrap(),
        );
        let rr_class = u16::from_be_bytes(
            bytes[after_additional_name + 2..after_additional_name + 4]
                .try_into()
                .unwrap(),
        );
        let rdlen = u16::from_be_bytes(
            bytes[after_additional_name + 8..after_additional_name + 10]
                .try_into()
                .unwrap(),
        ) as usize;
        pos = after_additional_name + 10;
        if pos + rdlen > bytes.len() {
            bail!("DNS additional rdata truncated");
        }
        if rr_type == TYPE_OPT {
            udp_payload_size = Some(rr_class);
        }
        pos += rdlen;
    }
    Ok(DnsQuery {
        id,
        qname,
        qtype,
        qclass,
        udp_payload_size,
        question: bytes[12..after_name + 4].to_vec(),
    })
}

pub fn build_empty_response(query: &DnsQuery, ttl: u32) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(12 + query.question.len());
    put_u16(&mut out, query.id);
    put_u16(&mut out, 0x8180);
    put_u16(&mut out, 1);
    put_u16(&mut out, 0);
    put_u16(&mut out, 0);
    put_u16(&mut out, 0);
    let _ = ttl;
    out.extend_from_slice(&query.question);
    Ok(out)
}

pub fn build_ns_response(query: &DnsQuery, zone: &str, ttl: u32) -> Result<Vec<u8>> {
    let ns_name = format!("ns.{}", normalize_domain(zone));
    let mut rdata = Vec::new();
    encode_name(&mut rdata, &ns_name)?;
    build_single_answer_response(query, TYPE_NS, ttl, &rdata)
}

pub fn build_soa_response(query: &DnsQuery, zone: &str, ttl: u32) -> Result<Vec<u8>> {
    let zone = normalize_domain(zone);
    let mut rdata = Vec::new();
    encode_name(&mut rdata, &format!("ns.{zone}"))?;
    encode_name(&mut rdata, &format!("hostmaster.{zone}"))?;
    put_u32(&mut rdata, 1);
    put_u32(&mut rdata, 300);
    put_u32(&mut rdata, 60);
    put_u32(&mut rdata, 300);
    put_u32(&mut rdata, 0);
    build_single_answer_response(query, TYPE_SOA, ttl, &rdata)
}

pub fn build_a_response(query: &DnsQuery, ttl: u32) -> Result<Vec<u8>> {
    build_single_answer_response(query, TYPE_A, ttl, &[0, 0, 0, 0])
}

pub fn build_aaaa_response(query: &DnsQuery, ttl: u32) -> Result<Vec<u8>> {
    build_single_answer_response(query, TYPE_AAAA, ttl, &[0; 16])
}

pub fn build_txt_response(query: &DnsQuery, envelope: &[u8], ttl: u32) -> Result<Vec<u8>> {
    let mut rdata = Vec::new();
    if envelope.is_empty() {
        rdata.push(0);
    } else {
        for chunk in envelope.chunks(255) {
            rdata.push(chunk.len() as u8);
            rdata.extend_from_slice(chunk);
        }
    }
    if rdata.len() > u16::MAX as usize {
        bail!("TXT response too large");
    }

    build_single_answer_response(query, TYPE_TXT, ttl, &rdata)
}

fn build_single_answer_response(
    query: &DnsQuery,
    answer_type: u16,
    ttl: u32,
    rdata: &[u8],
) -> Result<Vec<u8>> {
    if rdata.len() > u16::MAX as usize {
        bail!("DNS response rdata too large");
    }
    let mut out = Vec::with_capacity(12 + query.question.len() + 16 + rdata.len());
    put_u16(&mut out, query.id);
    put_u16(&mut out, 0x8180);
    put_u16(&mut out, 1);
    put_u16(&mut out, 1);
    put_u16(&mut out, 0);
    put_u16(&mut out, 0);
    out.extend_from_slice(&query.question);
    out.extend_from_slice(&[0xc0, 0x0c]);
    put_u16(&mut out, answer_type);
    put_u16(&mut out, CLASS_IN);
    put_u32(&mut out, ttl);
    put_u16(&mut out, rdata.len() as u16);
    out.extend_from_slice(rdata);
    Ok(out)
}

pub fn parse_txt_response(bytes: &[u8]) -> Result<Vec<u8>> {
    if bytes.len() < 12 {
        bail!("DNS response too short");
    }
    let flags = u16::from_be_bytes(bytes[2..4].try_into().unwrap());
    if flags & 0x8000 == 0 {
        bail!("DNS message is not a response");
    }
    let qdcount = u16::from_be_bytes(bytes[4..6].try_into().unwrap()) as usize;
    let ancount = u16::from_be_bytes(bytes[6..8].try_into().unwrap()) as usize;
    let nscount = u16::from_be_bytes(bytes[8..10].try_into().unwrap()) as usize;
    let arcount = u16::from_be_bytes(bytes[10..12].try_into().unwrap()) as usize;
    let mut pos = 12;
    let mut first_question = None::<(String, u16, u16)>;
    for _ in 0..qdcount {
        let (qname, next) = decode_name(bytes, pos)?;
        if next + 4 > bytes.len() {
            bail!("DNS question truncated");
        }
        let qtype = u16::from_be_bytes(bytes[next..next + 2].try_into().unwrap());
        let qclass = u16::from_be_bytes(bytes[next + 2..next + 4].try_into().unwrap());
        if first_question.is_none() {
            first_question = Some((qname, qtype, qclass));
        }
        pos = next.checked_add(4).context("DNS question overflow")?;
        if pos > bytes.len() {
            bail!("DNS question truncated");
        }
    }
    for _ in 0..ancount {
        let (_, next) = decode_name(bytes, pos)?;
        pos = next;
        if pos + 10 > bytes.len() {
            bail!("DNS answer truncated");
        }
        let ty = u16::from_be_bytes(bytes[pos..pos + 2].try_into().unwrap());
        let class = u16::from_be_bytes(bytes[pos + 2..pos + 4].try_into().unwrap());
        let rdlen = u16::from_be_bytes(bytes[pos + 8..pos + 10].try_into().unwrap()) as usize;
        pos += 10;
        let end = pos.checked_add(rdlen).context("DNS rdata overflow")?;
        if end > bytes.len() {
            bail!("DNS rdata truncated");
        }
        if ty == TYPE_TXT && class == CLASS_IN {
            let mut txt = Vec::new();
            let mut rpos = pos;
            while rpos < end {
                let len = bytes[rpos] as usize;
                rpos += 1;
                if rpos + len > end {
                    bail!("TXT chunk truncated");
                }
                txt.extend_from_slice(&bytes[rpos..rpos + len]);
                rpos += len;
            }
            return Ok(txt);
        }
        pos = end;
    }
    let question = first_question
        .map(|(qname, qtype, qclass)| format!("{qname} type={qtype} class={qclass}"))
        .unwrap_or_else(|| "none".to_string());
    bail!(
        "DNS response did not contain TXT answer (flags=0x{flags:04x}, rcode={}, answers={ancount}, authorities={nscount}, additionals={arcount}, question={question})",
        flags & 0x000f
    )
}

fn encode_name(out: &mut Vec<u8>, name: &str) -> Result<()> {
    for label in normalize_domain(name).split('.') {
        if label.is_empty() || label.len() > 63 {
            bail!("invalid DNS label length");
        }
        out.push(label.len() as u8);
        out.extend_from_slice(label.as_bytes());
    }
    out.push(0);
    Ok(())
}

fn decode_name(bytes: &[u8], start: usize) -> Result<(String, usize)> {
    let mut labels = Vec::new();
    let mut pos = start;
    let mut next = None;
    let mut jumps = 0;
    loop {
        if pos >= bytes.len() {
            bail!("DNS name truncated");
        }
        let len = bytes[pos];
        if len & 0xc0 == 0xc0 {
            if pos + 1 >= bytes.len() {
                bail!("DNS compression pointer truncated");
            }
            let ptr = (((len & 0x3f) as usize) << 8) | bytes[pos + 1] as usize;
            next.get_or_insert(pos + 2);
            pos = ptr;
            jumps += 1;
            if jumps > 8 {
                bail!("too many DNS compression jumps");
            }
            continue;
        }
        if len == 0 {
            let end = next.unwrap_or(pos + 1);
            return Ok((labels.join("."), end));
        }
        if len > 63 {
            bail!("invalid DNS label length");
        }
        pos += 1;
        let end = pos + len as usize;
        if end > bytes.len() {
            bail!("DNS label truncated");
        }
        labels.push(
            std::str::from_utf8(&bytes[pos..end])
                .context("DNS label is not utf-8")?
                .to_ascii_lowercase(),
        );
        pos = end;
    }
}

fn normalize_domain(domain: &str) -> String {
    domain.trim_end_matches('.').to_ascii_lowercase()
}

fn put_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_be_bytes());
}

fn put_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_be_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qname_envelope_roundtrip() {
        let payload = b"hello world, but inside DNS";
        let qname = envelope_to_qname(payload, "tun.example.com").unwrap();
        assert!(qname.ends_with(".tun.example.com"));
        assert_eq!(
            qname_to_envelope(&qname, "tun.example.com").unwrap(),
            payload
        );
    }

    #[test]
    fn qname_envelope_roundtrip_preserves_leading_zeroes() {
        for len in [0usize, 1, 2, 30, 31, 32, 62, 96, 128, 144] {
            let mut payload = vec![0u8; len];
            for (index, byte) in payload.iter_mut().enumerate().skip(len / 3) {
                *byte = (index as u8).wrapping_mul(17).wrapping_add(3);
            }
            let qname = envelope_to_qname(&payload, "tun.example.com").unwrap();
            assert!(qname.split('.').all(|label| label.len() <= 63));
            assert_eq!(
                envelope_qname_len(payload.len(), "tun.example.com").unwrap(),
                qname.len()
            );
            assert_eq!(
                qname_to_envelope(&qname, "tun.example.com").unwrap(),
                payload
            );
        }
    }

    #[test]
    fn compact_qname_envelope_roundtrip_infers_remainder() {
        let domain = "t.7-b.cc";
        for len in [0usize, 1, 2, 30, 31, 32, 62, 96, 128, 144, 154] {
            let payload = (0..len)
                .map(|index| (index as u8).wrapping_mul(31).wrapping_add(7))
                .collect::<Vec<_>>();
            let qname = envelope_to_compact_qname(&payload, domain).unwrap();
            assert!(qname.starts_with('u'));
            assert!(qname.split('.').all(|label| label.len() <= 63));
            assert_eq!(
                compact_envelope_qname_len(payload.len(), domain).unwrap(),
                qname.len()
            );
            assert_eq!(qname_to_envelope(&qname, domain).unwrap(), payload);
        }
    }

    #[test]
    fn compact_qname_exhaustive_roundtrip_and_boundaries() {
        let domain = "t.7-b.cc";
        for len in 0usize..=155 {
            let zeroes = vec![0u8; len];
            let qname = envelope_to_compact_qname(&zeroes, domain).unwrap();
            assert_eq!(qname_to_envelope(&qname, domain).unwrap(), zeroes);

            let ones = vec![0xffu8; len];
            let qname = envelope_to_compact_qname(&ones, domain).unwrap();
            assert_eq!(qname_to_envelope(&qname, domain).unwrap(), ones);
        }

        envelope_to_compact_qname(&vec![0u8; 155], domain).unwrap();
        assert!(envelope_to_compact_qname(&vec![0u8; 156], domain).is_err());
        envelope_to_compact_qname(&vec![0u8; 150], "tun.example.com").unwrap();
        assert!(envelope_to_compact_qname(&vec![0u8; 151], "tun.example.com").is_err());
    }

    #[test]
    fn qname_parser_rejects_noncanonical_lengths() {
        assert!(qname_to_envelope("u0.t.7-b.cc", "t.7-b.cc").is_err());
        assert!(qname_to_envelope("t0abc.t.7-b.cc", "t.7-b.cc").is_err());
        assert_eq!(
            qname_to_envelope("t0.t.7-b.cc", "t.7-b.cc").unwrap(),
            Vec::<u8>::new()
        );
    }

    #[test]
    fn base36_fixed_width_handles_max_blocks() {
        for len in 1..=BASE36_BLOCK_BYTES {
            let payload = vec![0xff; len];
            let qname = envelope_to_qname(&payload, "tun.example.com").unwrap();
            assert_eq!(
                qname_to_envelope(&qname, "tun.example.com").unwrap(),
                payload
            );
        }
    }

    #[test]
    fn dns_txt_roundtrip() {
        let query_bytes = build_query(7, "t-aa.example.com", 1232).unwrap();
        let parsed = parse_query(&query_bytes).unwrap();
        assert_eq!(parsed.udp_payload_size, Some(1232));
        let response = build_txt_response(&parsed, b"payload", 0).unwrap();
        assert_eq!(parse_txt_response(&response).unwrap(), b"payload");
    }

    #[test]
    fn qname_uses_full_label_capacity() {
        let payload = vec![7u8; 96];
        let qname = envelope_to_qname(&payload, "tun.example.com").unwrap();
        let labels = qname.split('.').collect::<Vec<_>>();

        assert_eq!(labels[0].len(), 63);
        assert_eq!(labels[1].len(), 63);
        assert!(labels.iter().all(|label| label.len() <= 63));
        assert_eq!(
            qname_to_envelope(&qname, "tun.example.com").unwrap(),
            payload
        );
    }

    #[test]
    fn envelope_qname_len_matches_encoded_name() {
        for len in [1usize, 2, 38, 39, 96, 128, 144] {
            let payload = vec![7u8; len];
            let qname = envelope_to_qname(&payload, "tun.example.com").unwrap();
            assert_eq!(
                envelope_qname_len(payload.len(), "tun.example.com").unwrap(),
                qname.len()
            );
        }
    }

    #[test]
    fn txt_response_wire_len_matches_builder() {
        let query_bytes = build_query(7, "t-aa.example.com", 1232).unwrap();
        let parsed = parse_query(&query_bytes).unwrap();

        for envelope_len in [0usize, 1, 254, 255, 256, 511, 512] {
            let envelope = vec![b'x'; envelope_len];
            let response = build_txt_response(&parsed, &envelope, 0).unwrap();
            assert_eq!(txt_response_wire_len(&parsed, envelope_len), response.len());
        }
    }
}

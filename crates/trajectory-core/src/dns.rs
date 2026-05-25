use anyhow::{bail, Context, Result};
use data_encoding::BASE32_NOPAD;

pub const TYPE_TXT: u16 = 16;
pub const TYPE_A: u16 = 1;
pub const TYPE_AAAA: u16 = 28;
pub const TYPE_NS: u16 = 2;
pub const TYPE_SOA: u16 = 6;
pub const CLASS_IN: u16 = 1;
pub const TYPE_OPT: u16 = 41;

#[derive(Clone, Debug)]
pub struct DnsQuery {
    pub id: u16,
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16,
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

pub fn envelope_to_qname(envelope: &[u8], domain: &str) -> Result<String> {
    let encoded = BASE32_NOPAD.encode(envelope).to_ascii_lowercase();
    let mut labels = Vec::new();
    let mut pos = 0;
    while pos < encoded.len() {
        let end = (pos + 50).min(encoded.len());
        let chunk = &encoded[pos..end];
        if pos == 0 {
            labels.push(format!("t-{chunk}"));
        } else {
            labels.push(chunk.to_string());
        }
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
    for (index, label) in left
        .split('.')
        .filter(|label| !label.is_empty())
        .enumerate()
    {
        if index == 0 {
            let stripped = label
                .strip_prefix("t-")
                .context("tunnel label missing t- prefix")?;
            encoded.push_str(stripped);
        } else {
            encoded.push_str(label);
        }
    }
    if encoded.is_empty() {
        bail!("empty tunnel payload");
    }
    BASE32_NOPAD
        .decode(encoded.to_ascii_uppercase().as_bytes())
        .map_err(|_| anyhow::anyhow!("invalid base32 tunnel payload"))
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
    if qdcount == 0 {
        bail!("DNS query has no question");
    }
    let (qname, after_name) = decode_name(bytes, 12)?;
    if after_name + 4 > bytes.len() {
        bail!("DNS question truncated");
    }
    let qtype = u16::from_be_bytes(bytes[after_name..after_name + 2].try_into().unwrap());
    let qclass = u16::from_be_bytes(bytes[after_name + 2..after_name + 4].try_into().unwrap());
    Ok(DnsQuery {
        id,
        qname,
        qtype,
        qclass,
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
    fn dns_txt_roundtrip() {
        let query_bytes = build_query(7, "t-aa.example.com", 1232).unwrap();
        let parsed = parse_query(&query_bytes).unwrap();
        let response = build_txt_response(&parsed, b"payload", 0).unwrap();
        assert_eq!(parse_txt_response(&response).unwrap(), b"payload");
    }

    #[test]
    fn qname_uses_conservative_label_capacity() {
        let payload = vec![7u8; 96];
        let qname = envelope_to_qname(&payload, "tun.example.com").unwrap();
        let labels = qname.split('.').collect::<Vec<_>>();

        assert_eq!(labels[0].len(), 52);
        assert!(labels.iter().all(|label| label.len() <= 63));
        assert_eq!(
            qname_to_envelope(&qname, "tun.example.com").unwrap(),
            payload
        );
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

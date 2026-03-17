use anyhow::{anyhow, bail, Context, Result};
use data_encoding::BASE32_NOPAD;
use hickory_proto::op::ResponseCode;
use hickory_proto::rr::{Name, RecordType};
use rand::{thread_rng, Rng};
use std::str::FromStr;

pub const PROTOCOL_VERSION: u8 = 1;
pub const FLAG_DATA: u8 = 1 << 0;
pub const FLAG_FIN: u8 = 1 << 1;
pub const FLAG_DOWNLINK: u8 = 1 << 2;
pub const FLAG_GAP: u8 = 1 << 3;
pub const MAX_QUERY_PAYLOAD: usize = 120;
pub const MAX_RESPONSE_PAYLOAD: usize = 4096;
pub const RESPONSE_CHUNK_SIZE: usize = 1024;
pub const DNS_MAX_PAYLOAD: u16 = 1400;
pub const WINDOW_SIZE: usize = 80;
pub const DOWNLINK_WINDOW: usize = 40;
pub const POLL_WINDOW: usize = 80;
pub const QUERY_TIMEOUT_MS: u64 = 250;
pub const KEEPALIVE_MS: u64 = 100;
pub const MAX_INFLIGHT_PER_RESOLVER: usize = 28;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestPacket {
    pub request_id: u32,
    pub session_id: u64,
    pub flags: u8,
    pub down_ack: u32,
    pub seq: u32,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResponsePacket {
    pub request_id: u32,
    pub ack: u32,
    pub flags: u8,
    pub down_seq: u32,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParsedQuery {
    pub id: u16,
    pub recursion_desired: bool,
    pub qtype: u16,
    pub max_payload: u16,
    pub question_wire: Vec<u8>,
    pub name_wire: Vec<u8>,
}

impl RequestPacket {
    pub fn encode(&self) -> Result<Vec<u8>> {
        if self.payload.len() > u16::MAX as usize {
            bail!("request payload too large");
        }
        let mut out = Vec::with_capacity(24 + self.payload.len());
        out.push(PROTOCOL_VERSION);
        out.push(self.flags);
        out.extend_from_slice(&self.request_id.to_be_bytes());
        out.extend_from_slice(&self.session_id.to_be_bytes());
        out.extend_from_slice(&self.down_ack.to_be_bytes());
        out.extend_from_slice(&self.seq.to_be_bytes());
        out.extend_from_slice(&(self.payload.len() as u16).to_be_bytes());
        out.extend_from_slice(&self.payload);
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 24 {
            bail!("short request");
        }
        if bytes[0] != PROTOCOL_VERSION {
            bail!("bad version");
        }
        let payload_len = u16::from_be_bytes([bytes[22], bytes[23]]) as usize;
        if bytes.len() != 24 + payload_len {
            bail!("bad request length");
        }
        Ok(Self {
            request_id: u32::from_be_bytes(bytes[2..6].try_into().unwrap()),
            session_id: u64::from_be_bytes(bytes[6..14].try_into().unwrap()),
            flags: bytes[1],
            down_ack: u32::from_be_bytes(bytes[14..18].try_into().unwrap()),
            seq: u32::from_be_bytes(bytes[18..22].try_into().unwrap()),
            payload: bytes[24..].to_vec(),
        })
    }
}

impl ResponsePacket {
    pub fn encode(&self) -> Result<Vec<u8>> {
        if self.payload.len() > u16::MAX as usize {
            bail!("response payload too large");
        }
        let mut out = Vec::with_capacity(16 + self.payload.len());
        out.push(PROTOCOL_VERSION);
        out.push(self.flags);
        out.extend_from_slice(&self.request_id.to_be_bytes());
        out.extend_from_slice(&self.ack.to_be_bytes());
        out.extend_from_slice(&self.down_seq.to_be_bytes());
        out.extend_from_slice(&(self.payload.len() as u16).to_be_bytes());
        out.extend_from_slice(&self.payload);
        Ok(out)
    }

    pub fn decode(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 16 {
            bail!("short response");
        }
        if bytes[0] != PROTOCOL_VERSION {
            bail!("bad version");
        }
        let payload_len = u16::from_be_bytes([bytes[14], bytes[15]]) as usize;
        if bytes.len() != 16 + payload_len {
            bail!("bad response length");
        }
        Ok(Self {
            request_id: u32::from_be_bytes(bytes[2..6].try_into().unwrap()),
            ack: u32::from_be_bytes(bytes[6..10].try_into().unwrap()),
            flags: bytes[1],
            down_seq: u32::from_be_bytes(bytes[10..14].try_into().unwrap()),
            payload: bytes[16..].to_vec(),
        })
    }
}

pub fn encode_name(request: &RequestPacket, domain: &str) -> Result<Name> {
    let domain_name = normalize_domain(domain)?;
    let mut labels = Vec::new();
    let nonce = format!("{:08x}", thread_rng().gen::<u32>());
    labels.push(nonce);
    let encoded = BASE32_NOPAD.encode(&request.encode()?);
    for chunk in encoded.as_bytes().chunks(63) {
        labels.push(String::from_utf8(chunk.to_vec()).context("payload label is not utf8")?);
    }
    let mut fqdn = labels.join(".");
    fqdn.push('.');
    fqdn.push_str(domain_name.trim_end_matches('.'));
    fqdn.push('.');
    Name::from_ascii(&fqdn).with_context(|| format!("invalid query name {fqdn}"))
}

pub fn decode_name(name: &Name, domain: &str) -> Result<RequestPacket> {
    let query_name = name.to_ascii();
    let suffix = normalize_domain(domain)?;
    let suffix = suffix.trim_end_matches('.');
    let suffix_with_dot = format!(".{suffix}.");
    if !query_name.ends_with(&suffix_with_dot) {
        bail!("wrong suffix");
    }
    let prefix = &query_name[..query_name.len() - suffix_with_dot.len()];
    let labels: Vec<&str> = prefix.split('.').filter(|label| !label.is_empty()).collect();
    if labels.len() < 2 {
        bail!("missing payload labels");
    }
    let payload_labels = &labels[1..];
    let payload = payload_labels.join("").to_ascii_uppercase();
    let bytes = BASE32_NOPAD
        .decode(payload.as_bytes())
        .map_err(|_| anyhow!("bad base32 payload"))?;
    RequestPacket::decode(&bytes)
}

pub fn build_query(request: &RequestPacket, domain: &str) -> Result<(u16, Vec<u8>)> {
    let qname = encode_query_name(request, domain)?;
    build_query_wire(qname, u16::from(RecordType::TXT))
}

pub fn build_probe_query(domain: &str) -> Result<(u16, Vec<u8>)> {
    let domain_name = normalize_domain(domain)?;
    let labels = domain_name
        .trim_end_matches('.')
        .split('.')
        .filter(|label| !label.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();
    let qname = encode_labels(&labels)?;
    build_query_wire(qname, u16::from(RecordType::A))
}

pub fn parse_dns_id(bytes: &[u8]) -> Result<u16> {
    if bytes.len() < 2 {
        bail!("short dns packet");
    }
    Ok(u16::from_be_bytes([bytes[0], bytes[1]]))
}

fn build_query_wire(qname: Vec<u8>, qtype: u16) -> Result<(u16, Vec<u8>)> {
    let id = thread_rng().gen::<u16>();
    let mut out = Vec::with_capacity(64 + qname.len());
    out.extend_from_slice(&id.to_be_bytes());
    out.extend_from_slice(&0x0100u16.to_be_bytes());
    out.extend_from_slice(&1u16.to_be_bytes());
    out.extend_from_slice(&0u16.to_be_bytes());
    out.extend_from_slice(&0u16.to_be_bytes());
    out.extend_from_slice(&1u16.to_be_bytes());
    out.extend_from_slice(&qname);
    out.extend_from_slice(&qtype.to_be_bytes());
    out.extend_from_slice(&1u16.to_be_bytes());
    append_opt(&mut out, DNS_MAX_PAYLOAD);
    Ok((id, out))
}

pub fn parse_query(bytes: &[u8]) -> Result<ParsedQuery> {
    if bytes.len() < 12 {
        bail!("short dns query");
    }
    let qdcount = u16::from_be_bytes([bytes[4], bytes[5]]) as usize;
    let arcount = u16::from_be_bytes([bytes[10], bytes[11]]) as usize;
    if qdcount == 0 {
        bail!("missing question");
    }

    let mut offset = 12usize;
    let name_end = skip_name(bytes, offset)?;
    let question_end = name_end.checked_add(4).context("short question")?;
    if question_end > bytes.len() {
        bail!("short question");
    }

    let qtype = u16::from_be_bytes([bytes[name_end], bytes[name_end + 1]]);
    let question_wire = bytes[offset..question_end].to_vec();
    let name_wire = bytes[offset..name_end].to_vec();
    offset = question_end;

    // Skip remaining questions if present.
    for _ in 1..qdcount {
        offset = skip_name(bytes, offset)?;
        offset = offset.checked_add(4).context("short extra question")?;
        if offset > bytes.len() {
            bail!("short extra question");
        }
    }

    let ancount = u16::from_be_bytes([bytes[6], bytes[7]]) as usize;
    let nscount = u16::from_be_bytes([bytes[8], bytes[9]]) as usize;
    for _ in 0..ancount {
        offset = skip_rr(bytes, offset)?;
    }
    for _ in 0..nscount {
        offset = skip_rr(bytes, offset)?;
    }

    let mut max_payload = DNS_MAX_PAYLOAD;
    for _ in 0..arcount {
        let rr_name_end = skip_name(bytes, offset)?;
        if rr_name_end + 10 > bytes.len() {
            bail!("short additional rr");
        }
        let rr_type = u16::from_be_bytes([bytes[rr_name_end], bytes[rr_name_end + 1]]);
        let class = u16::from_be_bytes([bytes[rr_name_end + 2], bytes[rr_name_end + 3]]);
        let rdlen = u16::from_be_bytes([bytes[rr_name_end + 8], bytes[rr_name_end + 9]]) as usize;
        offset = rr_name_end + 10;
        if offset + rdlen > bytes.len() {
            bail!("short additional rdata");
        }
        if rr_type == 41 {
            max_payload = class.min(DNS_MAX_PAYLOAD);
        }
        offset += rdlen;
    }

    Ok(ParsedQuery {
        id: u16::from_be_bytes([bytes[0], bytes[1]]),
        recursion_desired: (u16::from_be_bytes([bytes[2], bytes[3]]) & 0x0100) != 0,
        qtype,
        max_payload,
        question_wire,
        name_wire,
    })
}

pub fn decode_query_request(query: &ParsedQuery, domain: &str) -> Result<RequestPacket> {
    decode_name_wire(&query.name_wire, domain)
}

pub fn build_empty_response(query: &ParsedQuery, response_code: ResponseCode) -> Result<Vec<u8>> {
    let mut out = Vec::with_capacity(64 + query.question_wire.len());
    out.extend_from_slice(&query.id.to_be_bytes());
    let flags = 0x8400u16 | (response_code.low() as u16) | if query.recursion_desired { 0x0100 } else { 0 };
    out.extend_from_slice(&flags.to_be_bytes());
    out.extend_from_slice(&1u16.to_be_bytes());
    out.extend_from_slice(&0u16.to_be_bytes());
    out.extend_from_slice(&0u16.to_be_bytes());
    out.extend_from_slice(&1u16.to_be_bytes());
    out.extend_from_slice(&query.question_wire);
    append_opt(&mut out, query.max_payload);
    Ok(out)
}

pub fn build_response(query: &ParsedQuery, response: &ResponsePacket) -> Result<Vec<u8>> {
    let encoded = response.encode()?;
    let txt_rdata = encode_txt_rdata(&encoded);

    let mut out = Vec::with_capacity(128 + query.question_wire.len() + txt_rdata.len());
    out.extend_from_slice(&query.id.to_be_bytes());
    let flags = 0x8400u16 | if query.recursion_desired { 0x0100 } else { 0 };
    out.extend_from_slice(&flags.to_be_bytes());
    out.extend_from_slice(&1u16.to_be_bytes());
    out.extend_from_slice(&1u16.to_be_bytes());
    out.extend_from_slice(&0u16.to_be_bytes());
    out.extend_from_slice(&1u16.to_be_bytes());
    out.extend_from_slice(&query.question_wire);
    out.extend_from_slice(&0xc00cu16.to_be_bytes());
    out.extend_from_slice(&(u16::from(RecordType::TXT)).to_be_bytes());
    out.extend_from_slice(&1u16.to_be_bytes());
    out.extend_from_slice(&0u32.to_be_bytes());
    out.extend_from_slice(&(txt_rdata.len() as u16).to_be_bytes());
    out.extend_from_slice(&txt_rdata);
    append_opt(&mut out, query.max_payload);
    Ok(out)
}

pub fn parse_response(bytes: &[u8]) -> Result<ResponsePacket> {
    let (_, response) = parse_response_meta(bytes)?;
    response.context("missing answer")
}

pub fn parse_response_meta(bytes: &[u8]) -> Result<(u16, Option<ResponsePacket>)> {
    if bytes.len() < 12 {
        bail!("short dns response");
    }
    let dns_id = u16::from_be_bytes([bytes[0], bytes[1]]);
    let flags = u16::from_be_bytes([bytes[2], bytes[3]]);
    let qdcount = u16::from_be_bytes([bytes[4], bytes[5]]) as usize;
    let ancount = u16::from_be_bytes([bytes[6], bytes[7]]) as usize;
    let mut offset = 12usize;
    for _ in 0..qdcount {
        offset = skip_name(bytes, offset)?;
        offset = offset.checked_add(4).context("short question")?;
        if offset > bytes.len() {
            bail!("short question");
        }
    }
    if ancount == 0 {
        return Ok((dns_id, None));
    }
    if flags & 0x000f != 0 {
        return Ok((dns_id, None));
    }
    offset = skip_name(bytes, offset)?;
    if offset + 10 > bytes.len() {
        bail!("short answer");
    }
    let rr_type = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]);
    let rdlen = u16::from_be_bytes([bytes[offset + 8], bytes[offset + 9]]) as usize;
    offset += 10;
    if rr_type != u16::from(RecordType::TXT) || offset + rdlen > bytes.len() {
        bail!("unexpected answer type");
    }
    let mut rdata = Vec::with_capacity(rdlen);
    let end = offset + rdlen;
    while offset < end {
        let chunk_len = bytes[offset] as usize;
        offset += 1;
        if offset + chunk_len > end {
            bail!("short txt chunk");
        }
        rdata.extend_from_slice(&bytes[offset..offset + chunk_len]);
        offset += chunk_len;
    }
    Ok((dns_id, Some(ResponsePacket::decode(&rdata)?)))
}

pub fn normalize_domain(domain: &str) -> Result<String> {
    let trimmed = domain.trim_end_matches('.');
    if trimmed.is_empty() {
        bail!("empty domain");
    }
    Ok(Name::from_str(trimmed)?.to_ascii())
}

fn encode_query_name(request: &RequestPacket, domain: &str) -> Result<Vec<u8>> {
    let domain_name = normalize_domain(domain)?;
    let mut labels = Vec::new();
    labels.push(format!("{:08x}", thread_rng().gen::<u32>()));
    let encoded = BASE32_NOPAD.encode(&request.encode()?);
    for chunk in encoded.as_bytes().chunks(63) {
        labels.push(String::from_utf8(chunk.to_vec()).context("payload label is not utf8")?);
    }
    labels.extend(
        domain_name
            .trim_end_matches('.')
            .split('.')
            .filter(|label| !label.is_empty())
            .map(ToOwned::to_owned),
    );
    encode_labels(&labels)
}

fn decode_name_wire(name_wire: &[u8], domain: &str) -> Result<RequestPacket> {
    let suffix = normalize_domain(domain)?;
    let suffix_labels = suffix
        .trim_end_matches('.')
        .split('.')
        .filter(|label| !label.is_empty())
        .collect::<Vec<_>>();

    let mut labels = Vec::new();
    let mut offset = 0usize;
    while offset < name_wire.len() {
        let len = *name_wire.get(offset).context("short qname")? as usize;
        offset += 1;
        if len == 0 {
            break;
        }
        let end = offset.checked_add(len).context("qname overflow")?;
        let label = std::str::from_utf8(name_wire.get(offset..end).context("short qname label")?)
            .context("non-utf8 qname label")?;
        labels.push(label.to_ascii_lowercase());
        offset = end;
    }

    if labels.len() < suffix_labels.len() + 2 {
        bail!("missing payload labels");
    }
    if !labels[labels.len() - suffix_labels.len()..]
        .iter()
        .zip(suffix_labels.iter())
        .all(|(left, right)| left.eq_ignore_ascii_case(right))
    {
        bail!("wrong suffix");
    }

    let payload_labels = &labels[1..labels.len() - suffix_labels.len()];
    let payload = payload_labels.join("").to_ascii_uppercase();
    let bytes = BASE32_NOPAD
        .decode(payload.as_bytes())
        .map_err(|_| anyhow!("bad base32 payload"))?;
    RequestPacket::decode(&bytes)
}

fn encode_labels(labels: &[String]) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    for label in labels {
        if label.len() > 63 {
            bail!("label too long");
        }
        out.push(label.len() as u8);
        out.extend_from_slice(label.as_bytes());
    }
    out.push(0);
    Ok(out)
}

fn append_opt(out: &mut Vec<u8>, payload: u16) {
    out.push(0);
    out.extend_from_slice(&41u16.to_be_bytes());
    out.extend_from_slice(&payload.to_be_bytes());
    out.extend_from_slice(&0u32.to_be_bytes());
    out.extend_from_slice(&0u16.to_be_bytes());
}

fn encode_txt_rdata(payload: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(payload.len() + payload.len().div_ceil(255));
    for chunk in payload.chunks(255) {
        out.push(chunk.len() as u8);
        out.extend_from_slice(chunk);
    }
    out
}

fn skip_name(bytes: &[u8], mut offset: usize) -> Result<usize> {
    loop {
        let len = *bytes.get(offset).context("short name")?;
        if len & 0xc0 == 0xc0 {
            if offset + 2 > bytes.len() {
                bail!("short compression pointer");
            }
            return Ok(offset + 2);
        }
        offset += 1;
        if len == 0 {
            return Ok(offset);
        }
        offset = offset.checked_add(len as usize).context("name overflow")?;
        if offset > bytes.len() {
            bail!("short label");
        }
    }
}

fn skip_rr(bytes: &[u8], offset: usize) -> Result<usize> {
    let name_end = skip_name(bytes, offset)?;
    if name_end + 10 > bytes.len() {
        bail!("short rr");
    }
    let rdlen = u16::from_be_bytes([bytes[name_end + 8], bytes[name_end + 9]]) as usize;
    let end = name_end.checked_add(10 + rdlen).context("rr overflow")?;
    if end > bytes.len() {
        bail!("short rr rdata");
    }
    Ok(end)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_packets() {
        let request = RequestPacket {
            request_id: 7,
            session_id: 9,
            flags: FLAG_DATA | FLAG_FIN,
            down_ack: 12,
            seq: 13,
            payload: vec![1, 2, 3, 4, 5],
        };
        let bytes = request.encode().unwrap();
        assert_eq!(RequestPacket::decode(&bytes).unwrap(), request);

        let response = ResponsePacket {
            request_id: 8,
            ack: 14,
            flags: FLAG_DOWNLINK,
            down_seq: 2,
            payload: vec![9, 8, 7],
        };
        let bytes = response.encode().unwrap();
        assert_eq!(ResponsePacket::decode(&bytes).unwrap(), response);
    }

    #[test]
    fn roundtrip_dns() {
        let request = RequestPacket {
            request_id: 1,
            session_id: 2,
            flags: FLAG_DATA,
            down_ack: 3,
            seq: 4,
            payload: vec![5; 32],
        };
        let (_, wire) = build_query(&request, "t.7-b.cc").unwrap();
        let message = parse_query(&wire).unwrap();
        let decoded = decode_query_request(&message, "t.7-b.cc").unwrap();
        assert_eq!(decoded, request);

        let response = ResponsePacket {
            request_id: request.request_id,
            ack: 5,
            flags: FLAG_DOWNLINK,
            down_seq: 6,
            payload: vec![7; 24],
        };
        let wire = build_response(&message, &response).unwrap();
        let decoded = parse_response(&wire).unwrap();
        assert_eq!(decoded, response);
    }
}

use crate::auth::ClientAccessKey;
use anyhow::{bail, Context, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    XChaCha20Poly1305, XNonce,
};
use std::collections::HashMap;

const MAGIC: &[u8; 4] = b"TRJ2";
const VERSION: u8 = 1;
const MAX_ACK_RANGES: usize = 64;
const MAX_STREAM_RANGES: usize = 64;
const MAX_FRAMES: usize = 64;
const MAX_FRAME_LEN: usize = 4096;
const MAX_HOST_LEN: usize = 253;
const SEALED_HEADER_LEN: usize = 4 + 1 + 24;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Direction {
    ClientToServer,
    ServerToClient,
}

impl Direction {
    fn wire(self) -> u8 {
        match self {
            Self::ClientToServer => 0,
            Self::ServerToClient => 1,
        }
    }

    fn from_wire(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Self::ClientToServer),
            1 => Ok(Self::ServerToClient),
            _ => bail!("invalid packet direction"),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AckRange {
    pub first: u64,
    pub last: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StreamRange {
    pub start: u64,
    pub end: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Frame {
    Open {
        stream_id: u64,
        host: String,
        port: u16,
    },
    Data {
        stream_id: u64,
        offset: u64,
        fin: bool,
        bytes: Vec<u8>,
    },
    Close {
        stream_id: u64,
        code: u16,
    },
    Ping {
        nonce: u64,
    },
    StreamAck {
        stream_id: u64,
        cumulative_offset: u64,
        max_stream_data: u64,
        fin_offset: Option<u64>,
        ranges: Vec<StreamRange>,
    },
    PathChallenge {
        nonce: u64,
        response_bytes: u16,
    },
    PathResponse {
        nonce: u64,
        bytes: Vec<u8>,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Packet {
    pub conn_id: u64,
    pub packet_no: u64,
    pub max_response_bytes: u16,
    pub stream_ack_offset: Option<u64>,
    pub ack_ranges: Vec<AckRange>,
    pub frames: Vec<Frame>,
}

impl Packet {
    pub fn new(conn_id: u64, packet_no: u64) -> Self {
        Self {
            conn_id,
            packet_no,
            max_response_bytes: 900,
            stream_ack_offset: None,
            ack_ranges: Vec::new(),
            frames: Vec::new(),
        }
    }

    pub fn encoded_len(&self) -> usize {
        encode_packet(self)
            .map(|bytes| bytes.len())
            .unwrap_or(usize::MAX)
    }
}

pub fn seal_packet(
    key: &ClientAccessKey,
    direction: Direction,
    packet: &Packet,
) -> Result<Vec<u8>> {
    let plaintext = encode_packet(packet)?;
    let nonce = packet_nonce(direction, packet.conn_id, packet.packet_no);
    let cipher = XChaCha20Poly1305::new_from_slice(&key.secret)
        .map_err(|_| anyhow::anyhow!("invalid AEAD key length"))?;

    let mut aad = Vec::with_capacity(SEALED_HEADER_LEN);
    put_u32(&mut aad, key.client_id);
    aad.push(direction.wire());
    aad.extend_from_slice(&nonce);

    let ciphertext = cipher
        .encrypt(
            XNonce::from_slice(&nonce),
            Payload {
                msg: &plaintext,
                aad: &aad,
            },
        )
        .map_err(|_| anyhow::anyhow!("packet encryption failed"))?;

    let mut out = Vec::with_capacity(SEALED_HEADER_LEN + ciphertext.len());
    put_u32(&mut out, key.client_id);
    out.push(direction.wire());
    out.extend_from_slice(&nonce);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

pub fn open_packet_with_key(
    key: &ClientAccessKey,
    expected_direction: Direction,
    envelope: &[u8],
) -> Result<Packet> {
    let (client_id, plaintext) = open_sealed(envelope, |client_id| {
        if client_id == key.client_id {
            Some(key)
        } else {
            None
        }
    })?;
    if client_id != key.client_id {
        bail!("response client id mismatch");
    }
    let direction = envelope_direction(envelope)?;
    if direction != expected_direction {
        bail!("packet direction mismatch");
    }
    let packet = decode_packet(&plaintext)?;
    verify_packet_nonce(envelope, expected_direction, &packet)?;
    Ok(packet)
}

pub fn open_packet_with_registry(
    registry: &HashMap<u32, ClientAccessKey>,
    expected_direction: Direction,
    envelope: &[u8],
) -> Result<(ClientAccessKey, Packet)> {
    let mut found = None;
    let (_, plaintext) = open_sealed(envelope, |client_id| {
        let key = registry.get(&client_id)?;
        found = Some(key.clone());
        Some(key)
    })?;
    let direction = envelope_direction(envelope)?;
    if direction != expected_direction {
        bail!("packet direction mismatch");
    }
    let key = found.context("authorized key disappeared")?;
    let packet = decode_packet(&plaintext)?;
    verify_packet_nonce(envelope, expected_direction, &packet)?;
    Ok((key, packet))
}

fn open_sealed<'a, F>(envelope: &[u8], key_for: F) -> Result<(u32, Vec<u8>)>
where
    F: FnOnce(u32) -> Option<&'a ClientAccessKey>,
{
    if envelope.len() < SEALED_HEADER_LEN + 16 {
        bail!("sealed packet too short");
    }
    let client_id = u32::from_be_bytes(envelope[0..4].try_into().unwrap());
    let _direction = Direction::from_wire(envelope[4])?;
    let nonce: [u8; 24] = envelope[5..29].try_into().unwrap();
    let ciphertext = &envelope[SEALED_HEADER_LEN..];
    let key = key_for(client_id).context("unknown client id")?;
    let cipher = XChaCha20Poly1305::new_from_slice(&key.secret)
        .map_err(|_| anyhow::anyhow!("invalid AEAD key length"))?;

    let plaintext = cipher
        .decrypt(
            XNonce::from_slice(&nonce),
            Payload {
                msg: ciphertext,
                aad: &envelope[..SEALED_HEADER_LEN],
            },
        )
        .map_err(|_| anyhow::anyhow!("packet authentication failed"))?;
    Ok((client_id, plaintext))
}

fn envelope_direction(envelope: &[u8]) -> Result<Direction> {
    if envelope.len() < SEALED_HEADER_LEN {
        bail!("sealed packet too short");
    }
    Direction::from_wire(envelope[4])
}

fn verify_packet_nonce(envelope: &[u8], direction: Direction, packet: &Packet) -> Result<()> {
    if envelope.len() < SEALED_HEADER_LEN {
        bail!("sealed packet too short");
    }
    let nonce: [u8; 24] = envelope[5..29].try_into().unwrap();
    if nonce != packet_nonce(direction, packet.conn_id, packet.packet_no) {
        bail!("packet nonce does not match packet fields");
    }
    Ok(())
}

fn packet_nonce(direction: Direction, conn_id: u64, packet_no: u64) -> [u8; 24] {
    let mut nonce = [0u8; 24];
    nonce[0] = direction.wire();
    nonce[1..9].copy_from_slice(&conn_id.to_be_bytes());
    nonce[9..17].copy_from_slice(&packet_no.to_be_bytes());
    nonce[17..24].copy_from_slice(b"trj2-v1");
    nonce
}

pub fn encode_packet(packet: &Packet) -> Result<Vec<u8>> {
    if packet.ack_ranges.len() > MAX_ACK_RANGES {
        bail!("too many ack ranges");
    }
    if packet.frames.len() > MAX_FRAMES {
        bail!("too many frames");
    }

    let mut out = Vec::with_capacity(128);
    out.extend_from_slice(MAGIC);
    out.push(VERSION);
    put_u64(&mut out, packet.conn_id);
    put_u64(&mut out, packet.packet_no);
    put_u16(&mut out, packet.max_response_bytes);
    let mut flags = 0u8;
    if packet.stream_ack_offset.is_some() {
        flags |= 1;
    }
    out.push(flags);
    if let Some(offset) = packet.stream_ack_offset {
        put_u64(&mut out, offset);
    }
    out.push(packet.ack_ranges.len() as u8);
    for range in &packet.ack_ranges {
        if range.first > range.last {
            bail!("invalid ack range");
        }
        put_u64(&mut out, range.first);
        put_u64(&mut out, range.last);
    }
    out.push(packet.frames.len() as u8);
    for frame in &packet.frames {
        encode_frame(&mut out, frame)?;
    }
    Ok(out)
}

pub fn decode_packet(bytes: &[u8]) -> Result<Packet> {
    let mut cur = Cursor::new(bytes);
    let magic = cur.take(4)?;
    if magic != MAGIC {
        bail!("invalid packet magic");
    }
    let version = cur.u8()?;
    if version != VERSION {
        bail!("unsupported packet version {version}");
    }
    let conn_id = cur.u64()?;
    let packet_no = cur.u64()?;
    let max_response_bytes = cur.u16()?;
    let flags = cur.u8()?;
    if flags & !1 != 0 {
        bail!("unsupported packet flags");
    }
    let stream_ack_offset = if flags & 1 != 0 {
        Some(cur.u64()?)
    } else {
        None
    };
    let ack_count = cur.u8()? as usize;
    if ack_count > MAX_ACK_RANGES {
        bail!("too many ack ranges");
    }
    let mut ack_ranges = Vec::with_capacity(ack_count);
    for _ in 0..ack_count {
        let first = cur.u64()?;
        let last = cur.u64()?;
        if first > last {
            bail!("invalid ack range");
        }
        ack_ranges.push(AckRange { first, last });
    }
    let frame_count = cur.u8()? as usize;
    if frame_count > MAX_FRAMES {
        bail!("too many frames");
    }
    let mut frames = Vec::with_capacity(frame_count);
    for _ in 0..frame_count {
        frames.push(decode_frame(&mut cur)?);
    }
    if !cur.is_empty() {
        bail!("trailing packet bytes");
    }
    Ok(Packet {
        conn_id,
        packet_no,
        max_response_bytes,
        stream_ack_offset,
        ack_ranges,
        frames,
    })
}

fn encode_frame(out: &mut Vec<u8>, frame: &Frame) -> Result<()> {
    let mut body = Vec::new();
    let ty = match frame {
        Frame::Open {
            stream_id,
            host,
            port,
        } => {
            let host_bytes = host.as_bytes();
            if host_bytes.len() > MAX_HOST_LEN {
                bail!("invalid host length");
            }
            put_u64(&mut body, *stream_id);
            put_u16(&mut body, host_bytes.len() as u16);
            body.extend_from_slice(host_bytes);
            put_u16(&mut body, *port);
            1
        }
        Frame::Data {
            stream_id,
            offset,
            fin,
            bytes,
        } => {
            if bytes.len() > MAX_FRAME_LEN {
                bail!("data frame too large");
            }
            put_u64(&mut body, *stream_id);
            put_u64(&mut body, *offset);
            body.push(u8::from(*fin));
            put_u16(&mut body, bytes.len() as u16);
            body.extend_from_slice(bytes);
            2
        }
        Frame::Close { stream_id, code } => {
            put_u64(&mut body, *stream_id);
            put_u16(&mut body, *code);
            3
        }
        Frame::Ping { nonce } => {
            put_u64(&mut body, *nonce);
            4
        }
        Frame::StreamAck {
            stream_id,
            cumulative_offset,
            max_stream_data,
            fin_offset,
            ranges,
        } => {
            validate_stream_ack(*cumulative_offset, *max_stream_data, *fin_offset, ranges)?;
            put_u64(&mut body, *stream_id);
            put_u64(&mut body, *cumulative_offset);
            put_u64(&mut body, *max_stream_data);
            match fin_offset {
                Some(offset) => {
                    body.push(1);
                    put_u64(&mut body, *offset);
                }
                None => body.push(0),
            }
            body.push(ranges.len() as u8);
            for range in ranges {
                put_u64(&mut body, range.start);
                put_u64(&mut body, range.end);
            }
            5
        }
        Frame::PathChallenge {
            nonce,
            response_bytes,
        } => {
            put_u64(&mut body, *nonce);
            put_u16(&mut body, *response_bytes);
            6
        }
        Frame::PathResponse { nonce, bytes } => {
            if bytes.len() > MAX_FRAME_LEN {
                bail!("path response frame too large");
            }
            put_u64(&mut body, *nonce);
            put_u16(&mut body, bytes.len() as u16);
            body.extend_from_slice(bytes);
            7
        }
    };
    if body.len() > u16::MAX as usize {
        bail!("frame too large");
    }
    out.push(ty);
    put_u16(out, body.len() as u16);
    out.extend_from_slice(&body);
    Ok(())
}

fn decode_frame(cur: &mut Cursor<'_>) -> Result<Frame> {
    let ty = cur.u8()?;
    let len = cur.u16()? as usize;
    if len > MAX_FRAME_LEN + 32 {
        bail!("frame length exceeds limit");
    }
    let body = cur.take(len)?;
    let mut cur = Cursor::new(body);
    let frame = match ty {
        1 => {
            let stream_id = cur.u64()?;
            let host_len = cur.u16()? as usize;
            if host_len > MAX_HOST_LEN {
                bail!("invalid host length");
            }
            let host = std::str::from_utf8(cur.take(host_len)?)
                .context("host is not utf-8")?
                .to_string();
            let port = cur.u16()?;
            Frame::Open {
                stream_id,
                host,
                port,
            }
        }
        2 => {
            let stream_id = cur.u64()?;
            let offset = cur.u64()?;
            let fin = match cur.u8()? {
                0 => false,
                1 => true,
                _ => bail!("invalid fin value"),
            };
            let data_len = cur.u16()? as usize;
            let bytes = cur.take(data_len)?.to_vec();
            Frame::Data {
                stream_id,
                offset,
                fin,
                bytes,
            }
        }
        3 => {
            let stream_id = cur.u64()?;
            let code = cur.u16()?;
            Frame::Close { stream_id, code }
        }
        4 => {
            let nonce = cur.u64()?;
            Frame::Ping { nonce }
        }
        5 => {
            let stream_id = cur.u64()?;
            let cumulative_offset = cur.u64()?;
            let max_stream_data = cur.u64()?;
            let fin_offset = match cur.u8()? {
                0 => None,
                1 => Some(cur.u64()?),
                other => bail!("invalid stream ack fin flag {other}"),
            };
            let range_count = cur.u8()? as usize;
            if range_count > MAX_STREAM_RANGES {
                bail!("too many stream ack ranges");
            }
            let mut ranges = Vec::with_capacity(range_count);
            for _ in 0..range_count {
                ranges.push(StreamRange {
                    start: cur.u64()?,
                    end: cur.u64()?,
                });
            }
            validate_stream_ack(cumulative_offset, max_stream_data, fin_offset, &ranges)?;
            Frame::StreamAck {
                stream_id,
                cumulative_offset,
                max_stream_data,
                fin_offset,
                ranges,
            }
        }
        6 => {
            let nonce = cur.u64()?;
            let response_bytes = cur.u16()?;
            Frame::PathChallenge {
                nonce,
                response_bytes,
            }
        }
        7 => {
            let nonce = cur.u64()?;
            let len = cur.u16()? as usize;
            let bytes = cur.take(len)?.to_vec();
            Frame::PathResponse { nonce, bytes }
        }
        _ => bail!("unknown frame type {ty}"),
    };
    if !cur.is_empty() {
        bail!("trailing frame bytes");
    }
    Ok(frame)
}

fn validate_stream_ack(
    cumulative_offset: u64,
    max_stream_data: u64,
    fin_offset: Option<u64>,
    ranges: &[StreamRange],
) -> Result<()> {
    if cumulative_offset > max_stream_data {
        bail!("stream ack exceeds stream credit");
    }
    if let Some(fin_offset) = fin_offset {
        if cumulative_offset > fin_offset {
            bail!("stream ack cumulative offset exceeds final offset");
        }
        if fin_offset > max_stream_data {
            bail!("stream ack final offset exceeds stream credit");
        }
    }
    if ranges.len() > MAX_STREAM_RANGES {
        bail!("too many stream ack ranges");
    }
    let mut previous_end = cumulative_offset;
    for range in ranges {
        if range.start < cumulative_offset {
            bail!("stream ack range below cumulative offset");
        }
        if range.start <= previous_end {
            bail!("stream ack ranges are not canonical");
        }
        if range.start >= range.end {
            bail!("empty stream ack range");
        }
        if range.end > max_stream_data {
            bail!("stream ack range exceeds stream credit");
        }
        if fin_offset.is_some_and(|offset| range.end > offset) {
            bail!("stream ack range exceeds final offset");
        }
        previous_end = range.end;
    }
    Ok(())
}

fn put_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_be_bytes());
}

fn put_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_be_bytes());
}

fn put_u64(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_be_bytes());
}

struct Cursor<'a> {
    bytes: &'a [u8],
    pos: usize,
}

impl<'a> Cursor<'a> {
    fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, pos: 0 }
    }

    fn take(&mut self, len: usize) -> Result<&'a [u8]> {
        let end = self
            .pos
            .checked_add(len)
            .context("packet cursor overflow")?;
        if end > self.bytes.len() {
            bail!("packet truncated");
        }
        let out = &self.bytes[self.pos..end];
        self.pos = end;
        Ok(out)
    }

    fn u8(&mut self) -> Result<u8> {
        Ok(self.take(1)?[0])
    }

    fn u16(&mut self) -> Result<u16> {
        Ok(u16::from_be_bytes(self.take(2)?.try_into().unwrap()))
    }

    fn u64(&mut self) -> Result<u64> {
        Ok(u64::from_be_bytes(self.take(8)?.try_into().unwrap()))
    }

    fn is_empty(&self) -> bool {
        self.pos == self.bytes.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_roundtrip() {
        let packet = Packet {
            conn_id: 9,
            packet_no: 11,
            max_response_bytes: 700,
            stream_ack_offset: Some(4096),
            ack_ranges: vec![AckRange { first: 1, last: 3 }],
            frames: vec![
                Frame::Open {
                    stream_id: 44,
                    host: "example.com".into(),
                    port: 443,
                },
                Frame::Data {
                    stream_id: 44,
                    offset: 7,
                    fin: true,
                    bytes: b"hello".to_vec(),
                },
                Frame::StreamAck {
                    stream_id: 44,
                    cumulative_offset: 1024,
                    max_stream_data: 8192,
                    fin_offset: Some(4096),
                    ranges: vec![StreamRange {
                        start: 2048,
                        end: 3072,
                    }],
                },
                Frame::PathChallenge {
                    nonce: 55,
                    response_bytes: 64,
                },
                Frame::PathResponse {
                    nonce: 55,
                    bytes: vec![7; 64],
                },
            ],
        };
        let encoded = encode_packet(&packet).unwrap();
        let decoded = decode_packet(&encoded).unwrap();
        assert_eq!(decoded, packet);
    }

    #[test]
    fn stream_ack_rejects_non_canonical_ranges() {
        let mut packet = Packet::new(1, 2);
        packet.frames.push(Frame::StreamAck {
            stream_id: 1,
            cumulative_offset: 100,
            max_stream_data: 1000,
            fin_offset: None,
            ranges: vec![
                StreamRange {
                    start: 200,
                    end: 300,
                },
                StreamRange {
                    start: 300,
                    end: 400,
                },
            ],
        });
        assert!(encode_packet(&packet).is_err());
    }

    #[test]
    fn sealed_packet_rejects_tamper() {
        let key = ClientAccessKey::generate();
        let packet = Packet::new(1, 2);
        let mut sealed = seal_packet(&key, Direction::ClientToServer, &packet).unwrap();
        let last = sealed.len() - 1;
        sealed[last] ^= 1;
        assert!(open_packet_with_key(&key, Direction::ClientToServer, &sealed).is_err());
    }
}

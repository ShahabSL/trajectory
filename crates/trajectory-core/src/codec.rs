use crate::auth::ClientAccessKey;
use anyhow::{bail, Context, Result};
use chacha20poly1305::{
    aead::{Aead, KeyInit, Payload},
    ChaCha20Poly1305, Nonce,
};
use std::collections::HashMap;

const VERSION: u8 = 4;
const MAX_ACK_RANGES: usize = 64;
const MAX_STREAM_RANGES: usize = 64;
const MAX_FRAMES: usize = 64;
const MAX_FRAME_LEN: usize = 4096;
const MAX_HOST_LEN: usize = 253;
const SEALED_FIXED_HEADER_LEN: usize = 4;
const AEAD_TAG_LEN: usize = 16;
const DEFAULT_MAX_RESPONSE_BYTES: u16 = 900;
const PACKET_FLAG_STREAM_ACK_OFFSET: u8 = 0x01;
const PACKET_FLAG_COMPACT_DATA: u8 = 0x02;
const PACKET_FLAG_COMPACT_FIN: u8 = 0x04;
const PACKET_MARKER_COMPACT_DATA: u8 = 0x80;
const PACKET_MARKER_COMPACT_FIN: u8 = 0x01;
const PACKET_MARKER_COMPACT_STREAM_ZERO: u8 = 0x02;
const FRONTIER_SHORT_SEALED_MARKER: u8 = 0xff;
const FRONTIER_SHORT_CONN_ALIAS_MASK: u64 = 0x001f_ffff;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PacketEncoding {
    Standard,
    Frontier,
}

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
            max_response_bytes: DEFAULT_MAX_RESPONSE_BYTES,
            stream_ack_offset: None,
            ack_ranges: Vec::new(),
            frames: Vec::new(),
        }
    }

    pub fn encoded_len(&self) -> usize {
        encoded_packet_len(self).unwrap_or(usize::MAX)
    }
}

pub fn seal_packet(
    key: &ClientAccessKey,
    direction: Direction,
    packet: &Packet,
) -> Result<Vec<u8>> {
    seal_packet_with_encoding(key, direction, packet, PacketEncoding::Standard)
}

pub fn seal_packet_frontier(
    key: &ClientAccessKey,
    direction: Direction,
    packet: &Packet,
) -> Result<Vec<u8>> {
    seal_packet_with_encoding(key, direction, packet, PacketEncoding::Frontier)
}

pub fn seal_packet_frontier_short(
    key: &ClientAccessKey,
    direction: Direction,
    packet: &Packet,
) -> Result<Vec<u8>> {
    seal_packet_frontier_short_with_alias(
        key,
        direction,
        packet,
        frontier_short_conn_alias(packet.conn_id),
    )
}

fn seal_packet_frontier_short_with_alias(
    key: &ClientAccessKey,
    direction: Direction,
    packet: &Packet,
    alias: u32,
) -> Result<Vec<u8>> {
    let plaintext = encode_packet_with_encoding(packet, PacketEncoding::Frontier)?;
    let header = frontier_short_sealed_header(alias, packet.packet_no);
    let nonce = packet_nonce(direction, packet.packet_no);
    let packet_key = connection_packet_key(key, packet.conn_id);
    let cipher = ChaCha20Poly1305::new_from_slice(&packet_key)
        .map_err(|_| anyhow::anyhow!("invalid AEAD key length"))?;

    let ciphertext = cipher
        .encrypt(
            Nonce::from_slice(&nonce),
            Payload {
                msg: &plaintext,
                aad: &header,
            },
        )
        .map_err(|_| anyhow::anyhow!("packet encryption failed"))?;

    let mut out = Vec::with_capacity(header.len() + ciphertext.len());
    out.extend_from_slice(&header);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

fn seal_packet_with_encoding(
    key: &ClientAccessKey,
    direction: Direction,
    packet: &Packet,
    encoding: PacketEncoding,
) -> Result<Vec<u8>> {
    let plaintext = encode_packet_with_encoding(packet, encoding)?;
    let header = sealed_header(key.client_id, packet.conn_id, packet.packet_no);
    let nonce = packet_nonce(direction, packet.packet_no);
    let packet_key = connection_packet_key(key, packet.conn_id);
    let cipher = ChaCha20Poly1305::new_from_slice(&packet_key)
        .map_err(|_| anyhow::anyhow!("invalid AEAD key length"))?;

    let ciphertext = cipher
        .encrypt(
            Nonce::from_slice(&nonce),
            Payload {
                msg: &plaintext,
                aad: &header,
            },
        )
        .map_err(|_| anyhow::anyhow!("packet encryption failed"))?;

    let mut out = Vec::with_capacity(header.len() + ciphertext.len());
    out.extend_from_slice(&header);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

pub fn open_packet_with_key(
    key: &ClientAccessKey,
    expected_direction: Direction,
    envelope: &[u8],
) -> Result<Packet> {
    let opened = open_sealed(envelope, expected_direction, |client_id| {
        if client_id == key.client_id {
            Some(key)
        } else {
            None
        }
    })?;
    if opened.client_id != key.client_id {
        bail!("response client id mismatch");
    }
    decode_packet(
        &opened.plaintext,
        opened.conn_id,
        opened.packet_no,
        DEFAULT_MAX_RESPONSE_BYTES,
    )
}

pub fn open_packet_with_registry(
    registry: &HashMap<u32, ClientAccessKey>,
    expected_direction: Direction,
    envelope: &[u8],
) -> Result<(ClientAccessKey, Packet)> {
    let mut found = None;
    let opened = open_sealed(envelope, expected_direction, |client_id| {
        let key = registry.get(&client_id)?;
        found = Some(key.clone());
        Some(key)
    })?;
    let key = found.context("authorized key disappeared")?;
    let packet = decode_packet(
        &opened.plaintext,
        opened.conn_id,
        opened.packet_no,
        DEFAULT_MAX_RESPONSE_BYTES,
    )?;
    Ok((key, packet))
}

pub fn open_packet_frontier_short(
    key: &ClientAccessKey,
    conn_id: u64,
    expected_direction: Direction,
    envelope: &[u8],
) -> Result<Packet> {
    let header = parse_frontier_short_sealed_header(envelope)?;
    let expected_alias = frontier_short_conn_alias(conn_id);
    if header.alias != expected_alias {
        bail!("frontier short connection alias mismatch");
    }
    let ciphertext = &envelope[header.header_len..];
    let packet_key = connection_packet_key(key, conn_id);
    let cipher = ChaCha20Poly1305::new_from_slice(&packet_key)
        .map_err(|_| anyhow::anyhow!("invalid AEAD key length"))?;
    let nonce = packet_nonce(expected_direction, header.packet_no);

    let plaintext = cipher
        .decrypt(
            Nonce::from_slice(&nonce),
            Payload {
                msg: ciphertext,
                aad: &envelope[..header.header_len],
            },
        )
        .map_err(|_| anyhow::anyhow!("packet authentication failed"))?;
    decode_packet(
        &plaintext,
        conn_id,
        header.packet_no,
        DEFAULT_MAX_RESPONSE_BYTES,
    )
}

pub fn frontier_short_sealed_alias(envelope: &[u8]) -> Result<Option<u32>> {
    if envelope.first().copied() != Some(FRONTIER_SHORT_SEALED_MARKER) {
        return Ok(None);
    }
    parse_frontier_short_sealed_header(envelope).map(|header| Some(header.alias))
}

pub fn frontier_short_conn_alias(conn_id: u64) -> u32 {
    (conn_id & FRONTIER_SHORT_CONN_ALIAS_MASK) as u32
}

struct OpenedSealed {
    client_id: u32,
    conn_id: u64,
    packet_no: u64,
    plaintext: Vec<u8>,
}

fn open_sealed<'a, F>(
    envelope: &[u8],
    expected_direction: Direction,
    key_for: F,
) -> Result<OpenedSealed>
where
    F: FnOnce(u32) -> Option<&'a ClientAccessKey>,
{
    let header = parse_sealed_header(envelope)?;
    let ciphertext = &envelope[header.header_len..];
    let key = key_for(header.client_id).context("unknown client id")?;
    let packet_key = connection_packet_key(key, header.conn_id);
    let cipher = ChaCha20Poly1305::new_from_slice(&packet_key)
        .map_err(|_| anyhow::anyhow!("invalid AEAD key length"))?;
    let nonce = packet_nonce(expected_direction, header.packet_no);

    let plaintext = cipher
        .decrypt(
            Nonce::from_slice(&nonce),
            Payload {
                msg: ciphertext,
                aad: &envelope[..header.header_len],
            },
        )
        .map_err(|_| anyhow::anyhow!("packet authentication failed"))?;
    Ok(OpenedSealed {
        client_id: header.client_id,
        conn_id: header.conn_id,
        packet_no: header.packet_no,
        plaintext,
    })
}

fn packet_nonce(direction: Direction, packet_no: u64) -> [u8; 12] {
    let mut nonce = [0u8; 12];
    nonce[0] = direction.wire();
    nonce[4..12].copy_from_slice(&packet_no.to_be_bytes());
    nonce
}

fn sealed_header(client_id: u32, conn_id: u64, packet_no: u64) -> Vec<u8> {
    let mut out = Vec::with_capacity(sealed_header_len(conn_id, packet_no));
    put_u32(&mut out, client_id);
    put_var_u64(&mut out, conn_id);
    put_var_u64(&mut out, packet_no);
    out
}

fn frontier_short_sealed_header(alias: u32, packet_no: u64) -> Vec<u8> {
    let mut out = Vec::with_capacity(frontier_short_sealed_header_len(alias, packet_no));
    out.push(FRONTIER_SHORT_SEALED_MARKER);
    put_var_u64(&mut out, alias as u64);
    put_var_u64(&mut out, packet_no);
    out
}

struct SealedHeader {
    client_id: u32,
    conn_id: u64,
    packet_no: u64,
    header_len: usize,
}

struct FrontierShortSealedHeader {
    alias: u32,
    packet_no: u64,
    header_len: usize,
}

fn parse_sealed_header(envelope: &[u8]) -> Result<SealedHeader> {
    if envelope.len() < SEALED_FIXED_HEADER_LEN + 2 + AEAD_TAG_LEN {
        bail!("sealed packet too short");
    }
    let client_id = u32::from_be_bytes(envelope[0..4].try_into().unwrap());
    let (conn_id, after_conn_id) =
        parse_header_var_u64(envelope, SEALED_FIXED_HEADER_LEN).context("sealed connection id")?;
    let (packet_no, header_len) =
        parse_header_var_u64(envelope, after_conn_id).context("sealed packet number")?;
    if envelope.len() < header_len + AEAD_TAG_LEN {
        bail!("sealed packet too short");
    }
    Ok(SealedHeader {
        client_id,
        conn_id,
        packet_no,
        header_len,
    })
}

fn parse_frontier_short_sealed_header(envelope: &[u8]) -> Result<FrontierShortSealedHeader> {
    if envelope.first().copied() != Some(FRONTIER_SHORT_SEALED_MARKER) {
        bail!("not a frontier short sealed packet");
    }
    let (alias, after_alias) =
        parse_header_var_u64(envelope, 1).context("frontier short connection alias")?;
    if alias > FRONTIER_SHORT_CONN_ALIAS_MASK {
        bail!("frontier short connection alias exceeds mask");
    }
    let (packet_no, header_len) =
        parse_header_var_u64(envelope, after_alias).context("frontier short packet number")?;
    if envelope.len() < header_len + AEAD_TAG_LEN {
        bail!("sealed packet too short");
    }
    Ok(FrontierShortSealedHeader {
        alias: alias as u32,
        packet_no,
        header_len,
    })
}

fn connection_packet_key(key: &ClientAccessKey, conn_id: u64) -> [u8; 32] {
    let mut material = Vec::with_capacity(32);
    material.extend_from_slice(b"trajectory-v3 packet key");
    material.extend_from_slice(&conn_id.to_be_bytes());
    *blake3::keyed_hash(&key.secret, &material).as_bytes()
}

pub fn sealed_packet_len(packet: &Packet) -> Result<usize> {
    sealed_packet_len_with_encoding(packet, PacketEncoding::Standard)
}

pub fn sealed_packet_len_frontier(packet: &Packet) -> Result<usize> {
    sealed_packet_len_with_encoding(packet, PacketEncoding::Frontier)
}

pub fn sealed_packet_len_frontier_short(packet: &Packet) -> Result<usize> {
    checked_add_len(
        checked_add_len(
            frontier_short_sealed_header_len(
                frontier_short_conn_alias(packet.conn_id),
                packet.packet_no,
            ),
            AEAD_TAG_LEN,
        )?,
        encoded_packet_len_with_encoding(packet, PacketEncoding::Frontier)?,
    )
}

fn sealed_packet_len_with_encoding(packet: &Packet, encoding: PacketEncoding) -> Result<usize> {
    checked_add_len(
        checked_add_len(
            sealed_header_len(packet.conn_id, packet.packet_no),
            AEAD_TAG_LEN,
        )?,
        encoded_packet_len_with_encoding(packet, encoding)?,
    )
}

pub fn sealed_packet_len_with_extra_frame(packet: &Packet, frame: &Frame) -> Result<usize> {
    let mut packet = packet.clone();
    packet.frames.push(frame.clone());
    checked_add_len(
        checked_add_len(
            sealed_header_len(packet.conn_id, packet.packet_no),
            AEAD_TAG_LEN,
        )?,
        encoded_packet_len(&packet)?,
    )
}

fn sealed_header_len(conn_id: u64, packet_no: u64) -> usize {
    SEALED_FIXED_HEADER_LEN + var_len_u64(conn_id) + var_len_u64(packet_no)
}

fn frontier_short_sealed_header_len(alias: u32, packet_no: u64) -> usize {
    1 + var_len_u64(alias as u64) + var_len_u64(packet_no)
}

pub fn encoded_packet_len(packet: &Packet) -> Result<usize> {
    encoded_packet_len_with_encoding(packet, PacketEncoding::Standard)
}

pub fn encoded_packet_len_frontier(packet: &Packet) -> Result<usize> {
    encoded_packet_len_with_encoding(packet, PacketEncoding::Frontier)
}

fn encoded_packet_len_with_encoding(packet: &Packet, encoding: PacketEncoding) -> Result<usize> {
    if let Some(frame) = compact_data_frame(packet) {
        return compact_data_packet_len(frame, encoding);
    }
    if packet.ack_ranges.len() > MAX_ACK_RANGES {
        bail!("too many ack ranges");
    }
    if packet.frames.len() > MAX_FRAMES {
        bail!("too many frames");
    }

    let mut len = 1;
    len = checked_add_len(len, 1)?;
    if let Some(offset) = packet.stream_ack_offset {
        len = checked_add_len(len, var_len_u64(offset))?;
    }
    len = checked_add_len(len, 1)?;
    for range in &packet.ack_ranges {
        if range.first > range.last {
            bail!("invalid ack range");
        }
        len = checked_add_len(len, var_len_u64(range.first))?;
        len = checked_add_len(len, var_len_u64(range.last - range.first))?;
    }
    len = checked_add_len(len, 1)?;
    for frame in &packet.frames {
        len = checked_add_len(len, frame_encoded_len(frame)?)?;
    }
    Ok(len)
}

pub fn encoded_packet_len_with_extra_frame(packet: &Packet, frame: &Frame) -> Result<usize> {
    if packet.frames.len() >= MAX_FRAMES {
        bail!("too many frames");
    }
    let mut packet = packet.clone();
    packet.frames.push(frame.clone());
    encoded_packet_len(&packet)
}

pub fn encode_packet(packet: &Packet) -> Result<Vec<u8>> {
    encode_packet_with_encoding(packet, PacketEncoding::Standard)
}

pub fn encode_packet_frontier(packet: &Packet) -> Result<Vec<u8>> {
    encode_packet_with_encoding(packet, PacketEncoding::Frontier)
}

fn encode_packet_with_encoding(packet: &Packet, encoding: PacketEncoding) -> Result<Vec<u8>> {
    if let Some(frame) = compact_data_frame(packet) {
        let mut out = Vec::with_capacity(compact_data_packet_len(frame, encoding)?);
        let Frame::Data {
            stream_id,
            offset,
            bytes,
            ..
        } = frame
        else {
            unreachable!("compact data frame checked");
        };
        match encoding {
            PacketEncoding::Standard => {
                let flags = PACKET_FLAG_COMPACT_DATA
                    | if compact_data_fin(frame) {
                        PACKET_FLAG_COMPACT_FIN
                    } else {
                        0
                    };
                out.push(VERSION);
                out.push(flags);
                put_var_u64(&mut out, *stream_id);
            }
            PacketEncoding::Frontier => {
                out.push(
                    PACKET_MARKER_COMPACT_DATA
                        | if compact_data_fin(frame) {
                            PACKET_MARKER_COMPACT_FIN
                        } else {
                            0
                        }
                        | if *stream_id == 0 {
                            PACKET_MARKER_COMPACT_STREAM_ZERO
                        } else {
                            0
                        },
                );
                if *stream_id != 0 {
                    put_var_u64(&mut out, *stream_id);
                }
            }
        }
        put_var_u64(&mut out, *offset);
        out.extend_from_slice(bytes);
        return Ok(out);
    }

    let mut out = Vec::with_capacity(encoded_packet_len(packet)?);
    out.push(VERSION);
    let mut flags = 0u8;
    if packet.stream_ack_offset.is_some() {
        flags |= PACKET_FLAG_STREAM_ACK_OFFSET;
    }
    out.push(flags);
    if let Some(offset) = packet.stream_ack_offset {
        put_var_u64(&mut out, offset);
    }
    out.push(packet.ack_ranges.len() as u8);
    for range in &packet.ack_ranges {
        if range.first > range.last {
            bail!("invalid ack range");
        }
        put_var_u64(&mut out, range.first);
        put_var_u64(&mut out, range.last - range.first);
    }
    out.push(packet.frames.len() as u8);
    for frame in &packet.frames {
        encode_frame(&mut out, frame)?;
    }
    Ok(out)
}

fn decode_packet(
    bytes: &[u8],
    conn_id: u64,
    packet_no: u64,
    max_response_bytes: u16,
) -> Result<Packet> {
    let mut cur = Cursor::new(bytes);
    let marker = cur.u8()?;
    if marker & PACKET_MARKER_COMPACT_DATA != 0 {
        if marker
            & !(PACKET_MARKER_COMPACT_DATA
                | PACKET_MARKER_COMPACT_FIN
                | PACKET_MARKER_COMPACT_STREAM_ZERO)
            != 0
        {
            bail!("unsupported compact packet marker");
        }
        return decode_compact_data_packet(
            &mut cur,
            conn_id,
            packet_no,
            max_response_bytes,
            marker & PACKET_MARKER_COMPACT_FIN != 0,
            marker & PACKET_MARKER_COMPACT_STREAM_ZERO != 0,
        );
    }
    if marker != VERSION {
        bail!("unsupported packet version {marker}");
    }
    let flags = cur.u8()?;
    if flags & PACKET_FLAG_COMPACT_DATA != 0 {
        if flags & !(PACKET_FLAG_COMPACT_DATA | PACKET_FLAG_COMPACT_FIN) != 0 {
            bail!("unsupported compact packet flags");
        }
        return decode_compact_data_packet(
            &mut cur,
            conn_id,
            packet_no,
            max_response_bytes,
            flags & PACKET_FLAG_COMPACT_FIN != 0,
            false,
        );
    }
    if flags & !(PACKET_FLAG_STREAM_ACK_OFFSET) != 0 {
        bail!("unsupported packet flags");
    }
    let stream_ack_offset = if flags & PACKET_FLAG_STREAM_ACK_OFFSET != 0 {
        Some(cur.var_u64()?)
    } else {
        None
    };
    let ack_count = cur.u8()? as usize;
    if ack_count > MAX_ACK_RANGES {
        bail!("too many ack ranges");
    }
    let mut ack_ranges = Vec::with_capacity(ack_count);
    for _ in 0..ack_count {
        let first = cur.var_u64()?;
        let span = cur.var_u64()?;
        let last = first
            .checked_add(span)
            .context("ack range exceeds packet number space")?;
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

fn decode_compact_data_packet(
    cur: &mut Cursor<'_>,
    conn_id: u64,
    packet_no: u64,
    max_response_bytes: u16,
    fin: bool,
    implicit_stream_zero: bool,
) -> Result<Packet> {
    let stream_id = if implicit_stream_zero {
        0
    } else {
        cur.var_u64()?
    };
    let offset = cur.var_u64()?;
    let bytes = cur.take_remaining()?.to_vec();
    if bytes.len() > MAX_FRAME_LEN {
        bail!("data frame too large");
    }
    Ok(Packet {
        conn_id,
        packet_no,
        max_response_bytes,
        stream_ack_offset: None,
        ack_ranges: Vec::new(),
        frames: vec![Frame::Data {
            stream_id,
            offset,
            fin,
            bytes,
        }],
    })
}

fn encode_frame(out: &mut Vec<u8>, frame: &Frame) -> Result<()> {
    match frame {
        Frame::Open {
            stream_id,
            host,
            port,
        } => {
            let host_bytes = host.as_bytes();
            if host_bytes.len() > MAX_HOST_LEN {
                bail!("invalid host length");
            }
            out.push(1);
            put_var_u64(out, *stream_id);
            put_var_u64(out, host_bytes.len() as u64);
            out.extend_from_slice(host_bytes);
            put_var_u64(out, *port as u64);
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
            out.push(2);
            put_var_u64(out, *stream_id);
            put_var_u64(out, *offset);
            out.push(u8::from(*fin));
            put_var_u64(out, bytes.len() as u64);
            out.extend_from_slice(bytes);
        }
        Frame::Close { stream_id, code } => {
            out.push(3);
            put_var_u64(out, *stream_id);
            put_var_u64(out, *code as u64);
        }
        Frame::Ping { nonce } => {
            out.push(4);
            put_var_u64(out, *nonce);
        }
        Frame::StreamAck {
            stream_id,
            cumulative_offset,
            max_stream_data,
            fin_offset,
            ranges,
        } => {
            validate_stream_ack(*cumulative_offset, *max_stream_data, *fin_offset, ranges)?;
            out.push(5);
            put_var_u64(out, *stream_id);
            put_var_u64(out, *cumulative_offset);
            put_var_u64(out, *max_stream_data);
            match fin_offset {
                Some(offset) => {
                    out.push(1);
                    put_var_u64(out, *offset);
                }
                None => out.push(0),
            }
            out.push(ranges.len() as u8);
            let mut previous_end = *cumulative_offset;
            for range in ranges {
                put_var_u64(out, range.start - previous_end);
                put_var_u64(out, range.end - range.start);
                previous_end = range.end;
            }
        }
        Frame::PathChallenge {
            nonce,
            response_bytes,
        } => {
            out.push(6);
            put_var_u64(out, *nonce);
            put_var_u64(out, *response_bytes as u64);
        }
        Frame::PathResponse { nonce, bytes } => {
            if bytes.len() > MAX_FRAME_LEN {
                bail!("path response frame too large");
            }
            out.push(7);
            put_var_u64(out, *nonce);
            put_var_u64(out, bytes.len() as u64);
            out.extend_from_slice(bytes);
        }
    }
    Ok(())
}

fn decode_frame(cur: &mut Cursor<'_>) -> Result<Frame> {
    let ty = cur.u8()?;
    match ty {
        1 => {
            let stream_id = cur.var_u64()?;
            let host_len = usize::try_from(cur.var_u64()?).context("host length exceeds usize")?;
            if host_len > MAX_HOST_LEN {
                bail!("invalid host length");
            }
            let host = std::str::from_utf8(cur.take(host_len)?)
                .context("host is not utf-8")?
                .to_string();
            let port = u16_from_var(cur.var_u64()?, "port")?;
            Ok(Frame::Open {
                stream_id,
                host,
                port,
            })
        }
        2 => {
            let stream_id = cur.var_u64()?;
            let offset = cur.var_u64()?;
            let fin = match cur.u8()? {
                0 => false,
                1 => true,
                _ => bail!("invalid fin value"),
            };
            let data_len = usize::try_from(cur.var_u64()?).context("data length exceeds usize")?;
            if data_len > MAX_FRAME_LEN {
                bail!("data frame too large");
            }
            let bytes = cur.take(data_len)?.to_vec();
            Ok(Frame::Data {
                stream_id,
                offset,
                fin,
                bytes,
            })
        }
        3 => {
            let stream_id = cur.var_u64()?;
            let code = u16_from_var(cur.var_u64()?, "close code")?;
            Ok(Frame::Close { stream_id, code })
        }
        4 => {
            let nonce = cur.var_u64()?;
            Ok(Frame::Ping { nonce })
        }
        5 => {
            let stream_id = cur.var_u64()?;
            let cumulative_offset = cur.var_u64()?;
            let max_stream_data = cur.var_u64()?;
            let fin_offset = match cur.u8()? {
                0 => None,
                1 => Some(cur.var_u64()?),
                other => bail!("invalid stream ack fin flag {other}"),
            };
            let range_count = cur.u8()? as usize;
            if range_count > MAX_STREAM_RANGES {
                bail!("too many stream ack ranges");
            }
            let mut ranges = Vec::with_capacity(range_count);
            let mut previous_end = cumulative_offset;
            for _ in 0..range_count {
                let start_delta = cur.var_u64()?;
                let start = previous_end
                    .checked_add(start_delta)
                    .context("stream ack range start overflow")?;
                let len = cur.var_u64()?;
                let end = start
                    .checked_add(len)
                    .context("stream ack range end overflow")?;
                ranges.push(StreamRange { start, end });
                previous_end = end;
            }
            validate_stream_ack(cumulative_offset, max_stream_data, fin_offset, &ranges)?;
            Ok(Frame::StreamAck {
                stream_id,
                cumulative_offset,
                max_stream_data,
                fin_offset,
                ranges,
            })
        }
        6 => {
            let nonce = cur.var_u64()?;
            let response_bytes = u16_from_var(cur.var_u64()?, "response_bytes")?;
            Ok(Frame::PathChallenge {
                nonce,
                response_bytes,
            })
        }
        7 => {
            let nonce = cur.var_u64()?;
            let len =
                usize::try_from(cur.var_u64()?).context("path response length exceeds usize")?;
            if len > MAX_FRAME_LEN {
                bail!("path response frame too large");
            }
            let bytes = cur.take(len)?.to_vec();
            Ok(Frame::PathResponse { nonce, bytes })
        }
        _ => bail!("unknown frame type {ty}"),
    }
}

fn frame_encoded_len(frame: &Frame) -> Result<usize> {
    let mut len = 1usize;
    match frame {
        Frame::Open {
            stream_id,
            host,
            port,
        } => {
            let host_bytes = host.as_bytes();
            if host_bytes.len() > MAX_HOST_LEN {
                bail!("invalid host length");
            }
            len = checked_add_len(len, var_len_u64(*stream_id))?;
            len = checked_add_len(len, var_len_u64(host_bytes.len() as u64))?;
            len = checked_add_len(len, host_bytes.len())?;
            len = checked_add_len(len, var_len_u64(*port as u64))?;
        }
        Frame::Data {
            stream_id,
            offset,
            bytes,
            ..
        } => {
            if bytes.len() > MAX_FRAME_LEN {
                bail!("data frame too large");
            }
            len = checked_add_len(len, var_len_u64(*stream_id))?;
            len = checked_add_len(len, var_len_u64(*offset))?;
            len = checked_add_len(len, 1)?;
            len = checked_add_len(len, var_len_u64(bytes.len() as u64))?;
            len = checked_add_len(len, bytes.len())?;
        }
        Frame::Close { stream_id, code } => {
            len = checked_add_len(len, var_len_u64(*stream_id))?;
            len = checked_add_len(len, var_len_u64(*code as u64))?;
        }
        Frame::Ping { nonce } => {
            len = checked_add_len(len, var_len_u64(*nonce))?;
        }
        Frame::StreamAck {
            stream_id,
            cumulative_offset,
            max_stream_data,
            fin_offset,
            ranges,
        } => {
            validate_stream_ack(*cumulative_offset, *max_stream_data, *fin_offset, ranges)?;
            len = checked_add_len(len, var_len_u64(*stream_id))?;
            len = checked_add_len(len, var_len_u64(*cumulative_offset))?;
            len = checked_add_len(len, var_len_u64(*max_stream_data))?;
            len = checked_add_len(len, 1)?;
            if let Some(offset) = fin_offset {
                len = checked_add_len(len, var_len_u64(*offset))?;
            }
            len = checked_add_len(len, 1)?;
            let mut previous_end = *cumulative_offset;
            for range in ranges {
                len = checked_add_len(len, var_len_u64(range.start - previous_end))?;
                len = checked_add_len(len, var_len_u64(range.end - range.start))?;
                previous_end = range.end;
            }
        }
        Frame::PathChallenge {
            nonce,
            response_bytes,
        } => {
            len = checked_add_len(len, var_len_u64(*nonce))?;
            len = checked_add_len(len, var_len_u64(*response_bytes as u64))?;
        }
        Frame::PathResponse { nonce, bytes } => {
            if bytes.len() > MAX_FRAME_LEN {
                bail!("path response frame too large");
            }
            len = checked_add_len(len, var_len_u64(*nonce))?;
            len = checked_add_len(len, var_len_u64(bytes.len() as u64))?;
            len = checked_add_len(len, bytes.len())?;
        }
    }
    Ok(len)
}

fn compact_data_frame(packet: &Packet) -> Option<&Frame> {
    if packet.stream_ack_offset.is_some()
        || !packet.ack_ranges.is_empty()
        || packet.frames.len() != 1
    {
        return None;
    }
    match &packet.frames[0] {
        frame @ Frame::Data { bytes, .. } if bytes.len() <= MAX_FRAME_LEN => Some(frame),
        _ => None,
    }
}

fn compact_data_fin(frame: &Frame) -> bool {
    match frame {
        Frame::Data { fin, .. } => *fin,
        _ => false,
    }
}

fn compact_data_packet_len(frame: &Frame, encoding: PacketEncoding) -> Result<usize> {
    let Frame::Data {
        stream_id,
        offset,
        bytes,
        ..
    } = frame
    else {
        bail!("compact packet requires data frame");
    };
    if bytes.len() > MAX_FRAME_LEN {
        bail!("data frame too large");
    }
    let mut len = match encoding {
        PacketEncoding::Standard => 2usize,
        PacketEncoding::Frontier => 1usize,
    };
    if encoding == PacketEncoding::Standard || *stream_id != 0 {
        len = checked_add_len(len, var_len_u64(*stream_id))?;
    }
    let len = checked_add_len(len, var_len_u64(*offset))?;
    checked_add_len(len, bytes.len())
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

fn put_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_be_bytes());
}

fn put_var_u64(out: &mut Vec<u8>, mut value: u64) {
    while value >= 0x80 {
        out.push((value as u8) | 0x80);
        value >>= 7;
    }
    out.push(value as u8);
}

fn var_len_u64(mut value: u64) -> usize {
    let mut len = 1;
    while value >= 0x80 {
        len += 1;
        value >>= 7;
    }
    len
}

fn checked_add_len(lhs: usize, rhs: usize) -> Result<usize> {
    lhs.checked_add(rhs)
        .context("encoded packet length overflow")
}

fn u16_from_var(value: u64, field: &str) -> Result<u16> {
    u16::try_from(value).with_context(|| format!("{field} exceeds u16"))
}

fn parse_header_var_u64(bytes: &[u8], start: usize) -> Result<(u64, usize)> {
    let mut value = 0u64;
    for index in 0..10 {
        let pos = start
            .checked_add(index)
            .context("sealed header cursor overflow")?;
        let Some(&byte) = bytes.get(pos) else {
            bail!("sealed packet truncated");
        };
        if index == 9 && byte & 0xfe != 0 {
            bail!("sealed header varint exceeds u64");
        }
        value |= ((byte & 0x7f) as u64) << (index * 7);
        if byte & 0x80 == 0 {
            if index + 1 != var_len_u64(value) {
                bail!("sealed header varint is not canonical");
            }
            return Ok((value, pos + 1));
        }
    }
    bail!("sealed header varint exceeds u64")
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

    fn var_u64(&mut self) -> Result<u64> {
        let mut value = 0u64;
        for index in 0..10 {
            let byte = self.u8()?;
            if index == 9 && byte & 0xfe != 0 {
                bail!("varint exceeds u64");
            }
            value |= ((byte & 0x7f) as u64) << (index * 7);
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        bail!("varint exceeds u64")
    }

    fn is_empty(&self) -> bool {
        self.pos == self.bytes.len()
    }

    fn take_remaining(&mut self) -> Result<&'a [u8]> {
        self.take(self.bytes.len().saturating_sub(self.pos))
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
        let decoded = decode_packet(
            &encoded,
            packet.conn_id,
            packet.packet_no,
            packet.max_response_bytes,
        )
        .unwrap();
        assert_eq!(decoded, packet);
    }

    #[test]
    fn encoded_len_matches_actual_wire_bytes() {
        let mut packet = Packet {
            conn_id: u64::MAX - 1,
            packet_no: 16384,
            max_response_bytes: 4096,
            stream_ack_offset: Some(4096),
            ack_ranges: vec![
                AckRange { first: 1, last: 3 },
                AckRange {
                    first: 1024,
                    last: 2048,
                },
            ],
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
        let actual = encode_packet(&packet).unwrap().len();
        assert_eq!(encoded_packet_len(&packet).unwrap(), actual);
        assert_eq!(packet.encoded_len(), actual);

        let extra = Frame::Ping { nonce: 99 };
        let with_extra = encoded_packet_len_with_extra_frame(&packet, &extra).unwrap();
        packet.frames.push(extra);
        assert_eq!(with_extra, encode_packet(&packet).unwrap().len());
    }

    #[test]
    fn compact_data_packet_roundtrips_and_matches_len() {
        let key = ClientAccessKey::generate();
        let mut packet = Packet::new(17, 3);
        packet.frames.push(Frame::Data {
            stream_id: 1,
            offset: 16_384,
            fin: true,
            bytes: vec![7; 123],
        });

        let encoded = encode_packet(&packet).unwrap();
        assert_eq!(encoded_packet_len(&packet).unwrap(), encoded.len());
        assert_eq!(encoded[0], VERSION);
        assert_eq!(
            encoded[1],
            PACKET_FLAG_COMPACT_DATA | PACKET_FLAG_COMPACT_FIN
        );
        assert_eq!(
            decode_packet(
                &encoded,
                packet.conn_id,
                packet.packet_no,
                packet.max_response_bytes
            )
            .unwrap(),
            packet
        );
        assert_eq!(
            sealed_packet_len(&packet).unwrap(),
            seal_packet(&key, Direction::ClientToServer, &packet)
                .unwrap()
                .len()
        );

        let frontier = encode_packet_frontier(&packet).unwrap();
        assert_eq!(
            frontier[0],
            PACKET_MARKER_COMPACT_DATA | PACKET_MARKER_COMPACT_FIN
        );
        assert_eq!(
            encoded_packet_len_frontier(&packet).unwrap(),
            frontier.len()
        );
        assert_eq!(
            sealed_packet_len_frontier(&packet).unwrap(),
            seal_packet_frontier(&key, Direction::ClientToServer, &packet)
                .unwrap()
                .len()
        );
        assert_eq!(
            decode_packet(
                &frontier,
                packet.conn_id,
                packet.packet_no,
                packet.max_response_bytes
            )
            .unwrap(),
            packet
        );

        let mut non_compact = packet.clone();
        non_compact.ack_ranges.push(AckRange { first: 1, last: 1 });
        assert!(encoded.len() < encode_packet(&non_compact).unwrap().len());

        let with_extra =
            encoded_packet_len_with_extra_frame(&packet, &Frame::Ping { nonce: 9 }).unwrap();
        let mut expanded = packet.clone();
        expanded.frames.push(Frame::Ping { nonce: 9 });
        assert_eq!(with_extra, encode_packet(&expanded).unwrap().len());
    }

    #[test]
    fn legacy_compact_data_packet_still_decodes() {
        let packet = Packet {
            conn_id: 17,
            packet_no: 3,
            max_response_bytes: 900,
            stream_ack_offset: None,
            ack_ranges: Vec::new(),
            frames: vec![Frame::Data {
                stream_id: 1,
                offset: 16_384,
                fin: true,
                bytes: vec![7; 123],
            }],
        };
        let mut legacy = vec![VERSION, PACKET_FLAG_COMPACT_DATA | PACKET_FLAG_COMPACT_FIN];
        put_var_u64(&mut legacy, 1);
        put_var_u64(&mut legacy, 16_384);
        legacy.extend_from_slice(&[7; 123]);

        assert_eq!(
            decode_packet(
                &legacy,
                packet.conn_id,
                packet.packet_no,
                packet.max_response_bytes
            )
            .unwrap(),
            packet
        );
    }

    #[test]
    fn compact_data_packet_elides_stream_zero() {
        let mut zero = Packet::new(17, 3);
        zero.frames.push(Frame::Data {
            stream_id: 0,
            offset: 16_384,
            fin: false,
            bytes: vec![7; 123],
        });
        let mut one = zero.clone();
        one.frames = vec![Frame::Data {
            stream_id: 1,
            offset: 16_384,
            fin: false,
            bytes: vec![7; 123],
        }];

        let zero_encoded = encode_packet_frontier(&zero).unwrap();
        let one_encoded = encode_packet_frontier(&one).unwrap();
        assert_eq!(
            zero_encoded[0],
            PACKET_MARKER_COMPACT_DATA | PACKET_MARKER_COMPACT_STREAM_ZERO
        );
        assert_eq!(zero_encoded.len() + 1, one_encoded.len());
        assert_eq!(
            decode_packet(
                &zero_encoded,
                zero.conn_id,
                zero.packet_no,
                zero.max_response_bytes
            )
            .unwrap(),
            zero
        );
    }

    #[test]
    fn frontier_short_sealed_header_roundtrips() {
        let mut key = ClientAccessKey::generate();
        key.client_id = 0x1234_5678;
        let mut packet = Packet::new(0x0000_0000_0012_3456, 16_384);
        packet.frames.push(Frame::Data {
            stream_id: 0,
            offset: 1024,
            fin: false,
            bytes: vec![9; 123],
        });

        let full = seal_packet_frontier(&key, Direction::ClientToServer, &packet).unwrap();
        let short = seal_packet_frontier_short(&key, Direction::ClientToServer, &packet).unwrap();
        assert_eq!(
            frontier_short_sealed_alias(&short).unwrap(),
            Some(frontier_short_conn_alias(packet.conn_id))
        );
        assert_eq!(sealed_packet_len_frontier(&packet).unwrap(), full.len());
        assert_eq!(
            sealed_packet_len_frontier_short(&packet).unwrap(),
            short.len()
        );
        assert!(short.len() < full.len());
        assert_eq!(
            open_packet_frontier_short(&key, packet.conn_id, Direction::ClientToServer, &short)
                .unwrap(),
            packet
        );
        assert!(frontier_short_sealed_alias(&full).unwrap().is_none());
    }

    #[test]
    fn small_conn_id_uses_shorter_sealed_header() {
        let key = ClientAccessKey::generate();
        let mut small = Packet::new(17, 3);
        small.frames.push(Frame::Data {
            stream_id: 1,
            offset: 0,
            fin: false,
            bytes: vec![1; 64],
        });

        let mut large = small.clone();
        large.conn_id = u64::MAX - 1;

        let sealed_small = seal_packet(&key, Direction::ClientToServer, &small).unwrap();
        let sealed_large = seal_packet(&key, Direction::ClientToServer, &large).unwrap();
        assert_eq!(sealed_packet_len(&small).unwrap(), sealed_small.len());
        assert_eq!(sealed_packet_len(&large).unwrap(), sealed_large.len());
        assert!(sealed_small.len() < sealed_large.len());
        assert_eq!(
            open_packet_with_key(&key, Direction::ClientToServer, &sealed_small).unwrap(),
            small
        );
        assert_eq!(
            open_packet_with_key(&key, Direction::ClientToServer, &sealed_large).unwrap(),
            large
        );
    }

    #[test]
    fn sealed_header_varints_must_be_canonical() {
        let bytes = [0x80, 0x00];
        assert!(parse_header_var_u64(&bytes, 0).is_err());

        let bytes = [0x81, 0x00];
        assert!(parse_header_var_u64(&bytes, 0).is_err());

        let bytes = [0x80, 0x01];
        assert_eq!(parse_header_var_u64(&bytes, 0).unwrap(), (128, 2));
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

    #[test]
    fn sealed_len_matches_actual_wire_bytes() {
        let key = ClientAccessKey::generate();
        let mut packet = Packet::new(99, 16_384);
        packet.ack_ranges = vec![AckRange { first: 7, last: 9 }];
        packet.frames.push(Frame::Data {
            stream_id: 3,
            offset: 0,
            fin: false,
            bytes: vec![1; 96],
        });

        let sealed = seal_packet(&key, Direction::ClientToServer, &packet).unwrap();
        assert_eq!(sealed_packet_len(&packet).unwrap(), sealed.len());

        let extra = Frame::Ping { nonce: 123 };
        let with_extra = sealed_packet_len_with_extra_frame(&packet, &extra).unwrap();
        packet.frames.push(extra);
        assert_eq!(
            with_extra,
            seal_packet(&key, Direction::ClientToServer, &packet)
                .unwrap()
                .len()
        );
    }
}

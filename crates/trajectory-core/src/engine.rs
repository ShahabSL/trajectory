use crate::codec::{AckRange, Frame, StreamRange};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fmt;

const REPAIR_AFTER_NEW_SLICES: usize = 16;
const REPAIR_WINDOW_BYTES: u64 = 32 * 1024;
const PACKET_HISTORY_LIMIT: usize = 4096;

#[derive(Clone, Debug, Default)]
pub struct PacketHistory {
    seen: BTreeSet<u64>,
}

impl PacketHistory {
    pub fn insert(&mut self, packet_no: u64) {
        self.seen.insert(packet_no);
        while self.seen.len() > PACKET_HISTORY_LIMIT {
            let Some(oldest) = self.seen.iter().next().copied() else {
                break;
            };
            self.seen.remove(&oldest);
        }
    }

    pub fn is_acked(&self, packet_no: u64) -> bool {
        self.seen.contains(&packet_no)
    }

    pub fn ack_ranges(&self, limit: usize) -> Vec<AckRange> {
        if limit == 0 {
            return Vec::new();
        }
        let mut ranges = Vec::new();
        let mut iter = self.seen.iter().rev().copied();
        let Some(mut last) = iter.next() else {
            return ranges;
        };
        let mut first = last;
        for packet_no in iter {
            if first.checked_sub(1) == Some(packet_no) {
                first = packet_no;
                continue;
            }
            ranges.push(AckRange { first, last });
            if ranges.len() >= limit {
                return ranges;
            }
            first = packet_no;
            last = packet_no;
        }
        ranges.push(AckRange { first, last });
        ranges.truncate(limit);
        ranges
    }

    pub fn len(&self) -> usize {
        self.seen.len()
    }

    pub fn is_empty(&self) -> bool {
        self.seen.is_empty()
    }
}

pub fn ack_ranges_contain(ranges: &[AckRange], packet_no: u64) -> bool {
    ranges
        .iter()
        .any(|range| range.first <= packet_no && packet_no <= range.last)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SendBufferError {
    OffsetOverflow,
    Gap { expected: u64, actual: u64 },
    ConflictingFinalOffset { existing: u64, actual: u64 },
}

impl fmt::Display for SendBufferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OffsetOverflow => write!(f, "stream offset overflow"),
            Self::Gap { expected, actual } => {
                write!(
                    f,
                    "send buffer cannot retain a gap: expected offset {expected}, got {actual}"
                )
            }
            Self::ConflictingFinalOffset { existing, actual } => {
                write!(
                    f,
                    "send buffer final offset conflict: existing {existing}, got {actual}"
                )
            }
        }
    }
}

impl std::error::Error for SendBufferError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StreamAssemblerError {
    OffsetOverflow,
    ConflictingFinalOffset { existing: u64, actual: u64 },
    ReceiveWindowExceeded { max: u64, actual: u64 },
}

impl fmt::Display for StreamAssemblerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OffsetOverflow => write!(f, "stream offset overflow"),
            Self::ConflictingFinalOffset { existing, actual } => {
                write!(
                    f,
                    "stream final offset conflict: existing {existing}, got {actual}"
                )
            }
            Self::ReceiveWindowExceeded { max, actual } => {
                write!(
                    f,
                    "stream receive window exceeded: max offset {max}, got {actual}"
                )
            }
        }
    }
}

impl std::error::Error for StreamAssemblerError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SendBufferMode {
    New,
    Repair,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SendBufferSlice {
    pub mode: SendBufferMode,
    pub offset: u64,
    pub fin: bool,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, Default)]
pub struct RetainedByteSendBuffer {
    base_offset: u64,
    bytes: VecDeque<u8>,
    send_cursor: u64,
    repair_cursor: u64,
    new_since_repair: usize,
    fin_offset: Option<u64>,
    zero_fin_needs_ack: bool,
    zero_fin_sent: bool,
    acked: BTreeMap<u64, u64>,
    peer_max_data: Option<u64>,
}

impl RetainedByteSendBuffer {
    pub fn base_offset(&self) -> u64 {
        self.base_offset
    }

    pub fn end_offset(&self) -> u64 {
        self.base_offset.saturating_add(self.bytes.len() as u64)
    }

    pub fn send_cursor(&self) -> u64 {
        self.send_cursor
    }

    pub fn repair_cursor(&self) -> u64 {
        self.repair_cursor
    }

    pub fn retained_len(&self) -> usize {
        self.bytes.len()
    }

    pub fn has_retained_bytes(&self) -> bool {
        !self.bytes.is_empty()
    }

    pub fn has_pending_send(&self) -> bool {
        self.peek_next(1).is_some()
    }

    pub fn is_finished(&self) -> bool {
        self.fin_offset == Some(self.base_offset)
            && self.bytes.is_empty()
            && !self.zero_fin_needs_ack
    }

    pub fn append(
        &mut self,
        offset: u64,
        fin: bool,
        bytes: Vec<u8>,
    ) -> Result<(), SendBufferError> {
        let mut end = offset
            .checked_add(bytes.len() as u64)
            .ok_or(SendBufferError::OffsetOverflow)?;
        if fin {
            if let Some(existing) = self.fin_offset {
                if existing != end {
                    return Err(SendBufferError::ConflictingFinalOffset {
                        existing,
                        actual: end,
                    });
                }
            }
            self.fin_offset = Some(end);
            self.zero_fin_needs_ack =
                bytes.is_empty() && end == self.end_offset() && self.send_cursor >= end;
            self.zero_fin_sent = false;
        }
        if end <= self.base_offset {
            return Ok(());
        }

        let mut offset = offset;
        let mut start = 0usize;
        if offset < self.base_offset {
            start = (self.base_offset - offset) as usize;
            offset = self.base_offset;
            end = offset
                .checked_add((bytes.len() - start) as u64)
                .ok_or(SendBufferError::OffsetOverflow)?;
        }

        let retained_end = self.end_offset();
        if offset > retained_end {
            return Err(SendBufferError::Gap {
                expected: retained_end,
                actual: offset,
            });
        }
        if end <= retained_end {
            self.clamp_cursors();
            return Ok(());
        }

        let overlap = retained_end.saturating_sub(offset) as usize;
        let append_start = start + overlap;
        self.bytes.extend(bytes[append_start..].iter().copied());
        self.clamp_cursors();
        Ok(())
    }

    pub fn mark_fin_at_end(&mut self) {
        self.fin_offset = Some(self.end_offset());
        self.zero_fin_needs_ack = false;
        self.zero_fin_sent = false;
    }

    pub fn ack_zero_fin(&mut self, offset: u64) {
        if self.zero_fin_sent && self.fin_offset == Some(offset) {
            self.zero_fin_needs_ack = false;
        }
    }

    pub fn ack(&mut self, stream_ack_offset: u64) {
        let new_base = stream_ack_offset
            .min(self.send_cursor)
            .min(self.end_offset());
        if new_base <= self.base_offset {
            self.clamp_cursors();
            return;
        }
        self.drain_to(new_base);
        self.clamp_cursors();
    }

    pub fn apply_stream_ack(
        &mut self,
        cumulative_offset: u64,
        ranges: &[StreamRange],
        max_stream_data: u64,
        fin_offset: Option<u64>,
    ) {
        let max_stream_data = max_stream_data.max(cumulative_offset);
        self.peer_max_data = Some(
            self.peer_max_data
                .map_or(max_stream_data, |current| current.max(max_stream_data)),
        );

        let cumulative_end = cumulative_offset.min(self.send_cursor);
        self.insert_acked_range(self.base_offset, cumulative_end);
        for range in ranges {
            let start = range.start.max(self.base_offset);
            let end = range.end.min(self.send_cursor);
            self.insert_acked_range(start, end);
        }
        self.drain_acked_prefix();
        if self.zero_fin_sent
            && fin_offset == self.fin_offset
            && fin_offset.is_some_and(|offset| cumulative_offset >= offset)
        {
            self.zero_fin_needs_ack = false;
        }
        self.clamp_cursors();
    }

    pub fn peek_next(&self, max_len: usize) -> Option<SendBufferSlice> {
        if max_len == 0 {
            return None;
        }
        if self.bytes.is_empty() {
            if self.zero_fin_needs_ack && self.fin_offset == Some(self.base_offset) {
                return Some(SendBufferSlice {
                    mode: SendBufferMode::Repair,
                    offset: self.base_offset,
                    fin: true,
                    bytes: Vec::new(),
                });
            }
            return None;
        }

        let repair_window_end = self.repair_window_end();
        let next_repair = self.next_repair_offset(repair_window_end);
        let repair_available = next_repair.is_some();
        let repair_due = repair_available
            && (self.send_cursor >= self.end_offset()
                || self.new_since_repair >= REPAIR_AFTER_NEW_SLICES);
        if repair_due {
            let offset = next_repair.expect("repair availability checked");
            return self.slice_at_capped(
                SendBufferMode::Repair,
                offset,
                max_len,
                self.repair_slice_upper(offset, repair_window_end),
            );
        }

        let new_data_end = self.new_data_end();
        if self.send_cursor < new_data_end {
            return self.slice_at_capped(
                SendBufferMode::New,
                self.send_cursor,
                max_len,
                new_data_end,
            );
        }

        if repair_available {
            let offset = next_repair.expect("repair availability checked");
            return self.slice_at_capped(
                SendBufferMode::Repair,
                offset,
                max_len,
                self.repair_slice_upper(offset, repair_window_end),
            );
        }

        None
    }

    pub fn mark_sent(&mut self, slice: &SendBufferSlice) {
        let end = slice.offset.saturating_add(slice.bytes.len() as u64);
        if slice.fin {
            if slice.bytes.is_empty() && self.fin_offset == Some(slice.offset) {
                self.zero_fin_sent = true;
            } else {
                self.zero_fin_needs_ack = false;
            }
        }
        match slice.mode {
            SendBufferMode::New => {
                if slice.offset == self.send_cursor {
                    self.send_cursor = end.min(self.end_offset());
                    self.new_since_repair = self.new_since_repair.saturating_add(1);
                }
            }
            SendBufferMode::Repair => {
                self.repair_cursor = end;
                self.new_since_repair = 0;
                if self.repair_cursor >= self.send_cursor {
                    self.repair_cursor = self.base_offset;
                }
            }
        }
        self.clamp_cursors();
    }

    fn slice_at_capped(
        &self,
        mode: SendBufferMode,
        offset: u64,
        max_len: usize,
        upper: u64,
    ) -> Option<SendBufferSlice> {
        if offset < self.base_offset || offset >= self.end_offset() {
            return None;
        }
        let start = (offset - self.base_offset) as usize;
        let upper = upper.min(self.end_offset());
        let available = (upper - offset) as usize;
        if available == 0 {
            return None;
        }
        let len = available.min(max_len);
        let bytes = self.bytes.iter().skip(start).take(len).copied().collect();
        let end = offset.saturating_add(len as u64);
        Some(SendBufferSlice {
            mode,
            offset,
            fin: self.fin_offset == Some(end),
            bytes,
        })
    }

    fn drain_to(&mut self, new_base: u64) {
        if new_base <= self.base_offset {
            return;
        }
        let new_base = new_base.min(self.end_offset());
        let drain = (new_base - self.base_offset) as usize;
        self.bytes.drain(..drain);
        self.base_offset = new_base;
        self.prune_acked_ranges();
    }

    fn insert_acked_range(&mut self, start: u64, end: u64) {
        if start >= end {
            return;
        }
        let mut start = start;
        let mut end = end;
        if let Some((&prev_start, &prev_end)) = self.acked.range(..=start).next_back() {
            if prev_end >= start {
                start = prev_start;
                end = end.max(prev_end);
                self.acked.remove(&prev_start);
            }
        }
        while let Some((&next_start, &next_end)) = self.acked.range(start..).next() {
            if next_start > end {
                break;
            }
            end = end.max(next_end);
            self.acked.remove(&next_start);
        }
        self.acked.insert(start, end);
    }

    fn drain_acked_prefix(&mut self) {
        while let Some((&start, &end)) = self.acked.range(..=self.base_offset).next_back() {
            if end <= self.base_offset {
                self.acked.remove(&start);
                continue;
            }
            let new_base = end.min(self.send_cursor).min(self.end_offset());
            if new_base <= self.base_offset {
                break;
            }
            self.acked.remove(&start);
            self.drain_to(new_base);
        }
    }

    fn prune_acked_ranges(&mut self) {
        if self.acked.is_empty() {
            return;
        }
        let base = self.base_offset;
        let sent = self.send_cursor;
        let ranges: Vec<(u64, u64)> = self
            .acked
            .iter()
            .filter_map(|(&start, &end)| {
                let start = start.max(base);
                let end = end.min(sent);
                (start < end).then_some((start, end))
            })
            .collect();
        self.acked.clear();
        for (start, end) in ranges {
            self.insert_acked_range(start, end);
        }
    }

    fn new_data_end(&self) -> u64 {
        self.peer_max_data
            .unwrap_or(u64::MAX)
            .min(self.end_offset())
    }

    fn clamp_cursors(&mut self) {
        let end = self.end_offset();
        self.send_cursor = self.send_cursor.clamp(self.base_offset, end);
        self.repair_cursor = self.repair_cursor.clamp(self.base_offset, end);
        if self.repair_cursor >= self.send_cursor {
            self.repair_cursor = self.base_offset;
        }
        if self.base_offset >= self.send_cursor {
            self.new_since_repair = 0;
        }
        let repair_window_end = self.repair_window_end();
        if self.repair_cursor >= repair_window_end {
            self.repair_cursor = self.base_offset;
        }
        self.prune_acked_ranges();
    }

    fn repair_window_end(&self) -> u64 {
        self.send_cursor
            .min(self.base_offset.saturating_add(REPAIR_WINDOW_BYTES))
            .min(self.end_offset())
    }

    fn next_repair_offset(&self, repair_window_end: u64) -> Option<u64> {
        let start =
            if self.repair_cursor >= self.base_offset && self.repair_cursor < repair_window_end {
                self.repair_cursor
            } else {
                self.base_offset
            };
        self.first_unacked_at_or_after(start, repair_window_end)
            .or_else(|| {
                (start > self.base_offset)
                    .then(|| self.first_unacked_at_or_after(self.base_offset, repair_window_end))
                    .flatten()
            })
    }

    fn first_unacked_at_or_after(&self, start: u64, upper: u64) -> Option<u64> {
        let mut pos = start.max(self.base_offset);
        while pos < upper {
            match self.acked.range(..=pos).next_back() {
                Some((_, &end)) if end > pos => {
                    pos = end;
                }
                _ => return Some(pos),
            }
        }
        None
    }

    fn repair_slice_upper(&self, offset: u64, repair_window_end: u64) -> u64 {
        self.acked
            .range(offset..)
            .find_map(|(&start, _)| (start > offset).then_some(start))
            .unwrap_or(repair_window_end)
            .min(repair_window_end)
    }
}

#[derive(Clone, Debug, Default)]
pub struct StreamAssembler {
    next_offset: u64,
    fin_offset: Option<u64>,
    pending: BTreeMap<u64, Vec<u8>>,
}

impl StreamAssembler {
    pub fn next_offset(&self) -> u64 {
        self.next_offset
    }

    pub fn pending_len(&self) -> usize {
        self.pending.len()
    }

    pub fn first_pending_offset(&self) -> Option<u64> {
        self.pending.keys().next().copied()
    }

    pub fn selective_ack_ranges(&self, limit: usize) -> Vec<StreamRange> {
        if limit == 0 {
            return Vec::new();
        }
        let mut ranges = Vec::<StreamRange>::new();
        for (&offset, bytes) in &self.pending {
            if bytes.is_empty() {
                continue;
            }
            let start = offset.max(self.next_offset);
            let end = offset.saturating_add(bytes.len() as u64);
            if end <= start {
                continue;
            }
            if let Some(last) = ranges.last_mut() {
                if start <= last.end {
                    last.end = last.end.max(end);
                    continue;
                }
            }
            if ranges.len() >= limit {
                break;
            }
            ranges.push(StreamRange { start, end });
        }
        ranges
    }

    pub fn max_stream_data(&self, receive_window: u64) -> u64 {
        self.next_offset.saturating_add(receive_window)
    }

    pub fn stream_ack_frame(
        &self,
        stream_id: u64,
        receive_window: u64,
        range_limit: usize,
    ) -> Frame {
        let max_stream_data = self
            .max_stream_data(receive_window)
            .max(self.fin_offset.unwrap_or(0));
        let ranges = self
            .selective_ack_ranges(range_limit)
            .into_iter()
            .filter_map(|range| {
                let end = range.end.min(max_stream_data);
                (range.start < end).then_some(StreamRange {
                    start: range.start,
                    end,
                })
            })
            .collect();
        Frame::StreamAck {
            stream_id,
            cumulative_offset: self.next_offset,
            max_stream_data,
            fin_offset: self.fin_offset,
            ranges,
        }
    }

    pub fn is_finished(&self) -> bool {
        self.fin_offset == Some(self.next_offset)
    }

    pub fn insert(&mut self, offset: u64, fin: bool, bytes: Vec<u8>) -> Vec<u8> {
        self.try_insert(offset, fin, bytes).unwrap_or_default()
    }

    pub fn try_insert(
        &mut self,
        offset: u64,
        fin: bool,
        bytes: Vec<u8>,
    ) -> Result<Vec<u8>, StreamAssemblerError> {
        self.try_insert_with_window(offset, fin, bytes, u64::MAX)
    }

    pub fn try_insert_with_window(
        &mut self,
        offset: u64,
        fin: bool,
        bytes: Vec<u8>,
        receive_window: u64,
    ) -> Result<Vec<u8>, StreamAssemblerError> {
        let end = offset
            .checked_add(bytes.len() as u64)
            .ok_or(StreamAssemblerError::OffsetOverflow)?;
        let max_stream_data = self.max_stream_data(receive_window);
        if end > max_stream_data {
            return Err(StreamAssemblerError::ReceiveWindowExceeded {
                max: max_stream_data,
                actual: end,
            });
        }
        if fin {
            if let Some(existing) = self.fin_offset {
                if existing != end {
                    return Err(StreamAssemblerError::ConflictingFinalOffset {
                        existing,
                        actual: end,
                    });
                }
            }
            self.fin_offset = Some(end);
        }
        if end <= self.next_offset {
            return Ok(Vec::new());
        }

        let (offset, bytes) = if offset < self.next_offset {
            let trim = (self.next_offset - offset) as usize;
            (self.next_offset, bytes[trim..].to_vec())
        } else {
            (offset, bytes)
        };

        self.pending
            .entry(offset)
            .and_modify(|existing| {
                if bytes.len() > existing.len() {
                    *existing = bytes.clone();
                }
            })
            .or_insert(bytes);
        let mut out = Vec::new();
        while let Some(offset) = self
            .pending
            .range(..=self.next_offset)
            .next_back()
            .map(|(offset, _)| *offset)
        {
            let bytes = self.pending.remove(&offset).expect("pending key exists");
            let end = offset.saturating_add(bytes.len() as u64);
            if end <= self.next_offset {
                continue;
            }
            let trim = (self.next_offset - offset) as usize;
            self.next_offset = end;
            out.extend_from_slice(&bytes[trim..]);
        }
        Ok(out)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PathStats {
    pub srtt_ms: f64,
    pub rttvar_ms: f64,
    pub timeout_ms: u64,
    pub successes: u64,
    pub failures: u64,
    pub max_response_bytes: u16,
}

impl Default for PathStats {
    fn default() -> Self {
        Self {
            srtt_ms: 100.0,
            rttvar_ms: 50.0,
            timeout_ms: 500,
            successes: 0,
            failures: 0,
            max_response_bytes: 900,
        }
    }
}

impl PathStats {
    pub fn on_rtt(&mut self, sample_ms: u64) {
        let sample = sample_ms.max(1) as f64;
        if self.successes == 0 {
            self.srtt_ms = sample;
            self.rttvar_ms = sample / 2.0;
        } else {
            let err = (self.srtt_ms - sample).abs();
            self.rttvar_ms = 0.75 * self.rttvar_ms + 0.25 * err;
            self.srtt_ms = 0.875 * self.srtt_ms + 0.125 * sample;
        }
        self.successes += 1;
        self.timeout_ms = (self.srtt_ms + 4.0 * self.rttvar_ms)
            .clamp(80.0, 2_500.0)
            .round() as u64;
    }

    pub fn on_failure(&mut self) {
        self.failures += 1;
        self.timeout_ms = (self.timeout_ms.saturating_mul(2)).min(5_000);
        if self.max_response_bytes > 512 {
            self.max_response_bytes = self.max_response_bytes.saturating_sub(64).max(512);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packet_history_produces_sparse_ranges() {
        let mut history = PacketHistory::default();
        for packet in [1, 2, 3, 7, 9, 10] {
            history.insert(packet);
        }
        assert!(history.ack_ranges(0).is_empty());
        assert_eq!(
            history.ack_ranges(8),
            vec![
                AckRange { first: 9, last: 10 },
                AckRange { first: 7, last: 7 },
                AckRange { first: 1, last: 3 }
            ]
        );
    }

    #[test]
    fn packet_history_is_bounded() {
        let mut history = PacketHistory::default();
        for packet in 0..(PACKET_HISTORY_LIMIT as u64 + 16) {
            history.insert(packet);
        }

        assert_eq!(history.len(), PACKET_HISTORY_LIMIT);
        assert!(!history.is_acked(0));
        assert!(history.is_acked(16));
    }

    #[test]
    fn assembler_handles_reorder_and_duplicates() {
        let mut asm = StreamAssembler::default();
        assert_eq!(asm.insert(5, false, b"world".to_vec()), b"");
        assert_eq!(asm.insert(0, false, b"hello".to_vec()), b"helloworld");
        assert_eq!(asm.insert(0, false, b"hello".to_vec()), b"");
    }

    #[test]
    fn assembler_rejects_data_beyond_receive_window() {
        let mut asm = StreamAssembler::default();
        assert_eq!(
            asm.try_insert_with_window(8, false, b"x".to_vec(), 8)
                .unwrap_err(),
            StreamAssemblerError::ReceiveWindowExceeded { max: 8, actual: 9 }
        );
        assert_eq!(
            asm.try_insert_with_window(7, false, b"x".to_vec(), 8)
                .unwrap(),
            b""
        );
        assert_eq!(
            asm.try_insert_with_window(0, false, b"abcdefgh".to_vec(), 8)
                .unwrap(),
            b"abcdefgh"
        );
    }

    #[test]
    fn assembler_drains_overlapping_retransmits() {
        let mut asm = StreamAssembler::default();
        assert_eq!(asm.insert(4, false, b"efgh".to_vec()), b"");
        assert_eq!(asm.insert(0, false, b"abcdef".to_vec()), b"abcdefgh");
        assert_eq!(asm.next_offset(), 8);
    }

    #[test]
    fn assembler_reports_selective_ack_ranges_above_gap() {
        let mut asm = StreamAssembler::default();
        assert_eq!(asm.insert(8, false, b"ijkl".to_vec()), b"");
        assert_eq!(asm.insert(4, false, b"efgh".to_vec()), b"");

        assert_eq!(
            asm.selective_ack_ranges(4),
            vec![StreamRange { start: 4, end: 12 }]
        );
        assert_eq!(asm.max_stream_data(1024), 1024);

        assert_eq!(asm.insert(0, false, b"abcd".to_vec()), b"abcdefghijkl");
        assert!(asm.selective_ack_ranges(4).is_empty());
        assert_eq!(asm.max_stream_data(1024), 1036);
    }

    #[test]
    fn assembler_builds_stream_ack_frame() {
        let mut asm = StreamAssembler::default();
        assert_eq!(asm.insert(32, false, b"later".to_vec()), b"");

        assert_eq!(
            asm.stream_ack_frame(7, 4096, 2),
            Frame::StreamAck {
                stream_id: 7,
                cumulative_offset: 0,
                max_stream_data: 4096,
                fin_offset: None,
                ranges: vec![StreamRange { start: 32, end: 37 }],
            }
        );
    }

    #[test]
    fn send_buffer_sends_new_bytes_then_repairs() {
        let mut send = RetainedByteSendBuffer::default();
        send.append(0, false, b"abcdef".to_vec()).unwrap();

        let first = send.peek_next(2).unwrap();
        assert_eq!(first.mode, SendBufferMode::New);
        assert_eq!(first.offset, 0);
        assert_eq!(first.bytes, b"ab");
        send.mark_sent(&first);
        assert_eq!(send.send_cursor(), 2);

        let second = send.peek_next(4).unwrap();
        assert_eq!(second.mode, SendBufferMode::New);
        assert_eq!(second.offset, 2);
        assert_eq!(second.bytes, b"cdef");
        send.mark_sent(&second);
        assert_eq!(send.send_cursor(), 6);

        let second_repair = send.peek_next(3).unwrap();
        assert_eq!(second_repair.mode, SendBufferMode::Repair);
        assert_eq!(second_repair.offset, 0);
        assert_eq!(second_repair.bytes, b"abc");
        send.mark_sent(&second_repair);
        assert_eq!(send.send_cursor(), 6);
        assert_eq!(send.repair_cursor(), 3);
    }

    #[test]
    fn send_buffer_interleaves_repair_during_long_new_burst() {
        let mut send = RetainedByteSendBuffer::default();
        send.append(0, false, vec![b'x'; REPAIR_WINDOW_BYTES as usize + 16])
            .unwrap();

        for expected_offset in (0..REPAIR_AFTER_NEW_SLICES).map(|index| index * 128) {
            let next = send.peek_next(128).unwrap();
            assert_eq!(next.mode, SendBufferMode::New);
            assert_eq!(next.offset, expected_offset as u64);
            send.mark_sent(&next);
        }

        let repair = send.peek_next(1024).unwrap();
        assert_eq!(repair.mode, SendBufferMode::Repair);
        assert_eq!(repair.offset, 0);
        send.mark_sent(&repair);
        assert_eq!(send.repair_cursor(), 1024);

        let next = send.peek_next(128).unwrap();
        assert_eq!(next.mode, SendBufferMode::New);
        assert_eq!(next.offset, (REPAIR_AFTER_NEW_SLICES * 128) as u64);
    }

    #[test]
    fn send_buffer_retries_blocking_base_while_ack_stalls() {
        let mut send = RetainedByteSendBuffer::default();
        send.append(
            0,
            false,
            vec![b'x'; REPAIR_WINDOW_BYTES as usize + (REPAIR_AFTER_NEW_SLICES * 4096 * 5)],
        )
        .unwrap();

        for _ in 0..REPAIR_AFTER_NEW_SLICES {
            let next = send.peek_next(4096).unwrap();
            assert_eq!(next.mode, SendBufferMode::New);
            send.mark_sent(&next);
        }

        for expected_repair_offset in [0, 1024, 2048, 3072] {
            let repair = send.peek_next(1024).unwrap();
            assert_eq!(repair.mode, SendBufferMode::Repair);
            assert_eq!(repair.offset, expected_repair_offset);
            send.mark_sent(&repair);

            for _ in 0..REPAIR_AFTER_NEW_SLICES {
                let next = send.peek_next(4096).unwrap();
                assert_eq!(next.mode, SendBufferMode::New);
                send.mark_sent(&next);
            }
        }
    }

    #[test]
    fn send_buffer_ack_prunes_prefix_and_clamps_repair_cursor() {
        let mut send = RetainedByteSendBuffer::default();
        send.append(0, false, b"abcdefgh".to_vec()).unwrap();
        let first = send.peek_next(8).unwrap();
        send.mark_sent(&first);
        let repair = send.peek_next(2).unwrap();
        send.mark_sent(&repair);

        send.ack(5);
        assert_eq!(send.base_offset(), 5);
        assert_eq!(send.retained_len(), 3);
        assert_eq!(send.send_cursor(), 8);
        assert_eq!(send.repair_cursor(), 5);
        assert_eq!(send.peek_next(8).unwrap().bytes, b"fgh");

        send.ack(3);
        assert_eq!(send.base_offset(), 5);
        assert_eq!(send.retained_len(), 3);
    }

    #[test]
    fn send_buffer_skips_selectively_acked_repair_ranges() {
        let mut send = RetainedByteSendBuffer::default();
        send.append(0, false, b"abcdefghijklmnop".to_vec()).unwrap();
        let first = send.peek_next(16).unwrap();
        send.mark_sent(&first);

        send.apply_stream_ack(0, &[StreamRange { start: 4, end: 16 }], 4096, None);

        let repair = send.peek_next(16).unwrap();
        assert_eq!(repair.mode, SendBufferMode::Repair);
        assert_eq!(repair.offset, 0);
        assert_eq!(repair.bytes, b"abcd");
        send.mark_sent(&repair);

        let repair = send.peek_next(16).unwrap();
        assert_eq!(repair.mode, SendBufferMode::Repair);
        assert_eq!(repair.offset, 0);
        assert_eq!(repair.bytes, b"abcd");
    }

    #[test]
    fn send_buffer_stream_credit_limits_new_bytes() {
        let mut send = RetainedByteSendBuffer::default();
        send.append(0, false, b"abcdefghij".to_vec()).unwrap();
        send.apply_stream_ack(0, &[], 4, None);

        let first = send.peek_next(16).unwrap();
        assert_eq!(first.mode, SendBufferMode::New);
        assert_eq!(first.offset, 0);
        assert_eq!(first.bytes, b"abcd");
        send.mark_sent(&first);

        let repair = send.peek_next(16).unwrap();
        assert_eq!(repair.mode, SendBufferMode::Repair);
        assert_eq!(repair.bytes, b"abcd");

        send.apply_stream_ack(4, &[], 8, None);
        let next = send.peek_next(16).unwrap();
        assert_eq!(next.mode, SendBufferMode::New);
        assert_eq!(next.offset, 4);
        assert_eq!(next.bytes, b"efgh");
    }

    #[test]
    fn send_buffer_future_ack_does_not_drop_unsent_bytes() {
        let mut send = RetainedByteSendBuffer::default();
        send.append(0, false, b"abcdefghij".to_vec()).unwrap();
        let first = send.peek_next(4).unwrap();
        send.mark_sent(&first);

        send.ack(8);
        assert_eq!(send.base_offset(), 4);
        assert_eq!(send.retained_len(), 6);
        assert_eq!(send.send_cursor(), 4);
        let next = send.peek_next(8).unwrap();
        assert_eq!(next.mode, SendBufferMode::New);
        assert_eq!(next.offset, 4);
        assert_eq!(next.bytes, b"efghij");
    }

    #[test]
    fn send_buffer_handles_overlap_and_fin_completion() {
        let mut send = RetainedByteSendBuffer::default();
        send.append(0, false, b"abcd".to_vec()).unwrap();
        send.append(2, true, b"cdef".to_vec()).unwrap();
        assert_eq!(send.retained_len(), 6);

        let all = send.peek_next(16).unwrap();
        assert_eq!(all.offset, 0);
        assert!(all.fin);
        assert_eq!(all.bytes, b"abcdef");
        send.mark_sent(&all);
        send.ack(6);
        assert!(send.is_finished());
    }

    #[test]
    fn send_buffer_reports_gaps_and_offset_overflow() {
        let mut send = RetainedByteSendBuffer::default();
        assert_eq!(
            send.append(5, false, b"x".to_vec()).unwrap_err(),
            SendBufferError::Gap {
                expected: 0,
                actual: 5
            }
        );
        assert_eq!(
            send.append(u64::MAX, false, b"x".to_vec()).unwrap_err(),
            SendBufferError::OffsetOverflow
        );
    }

    #[test]
    fn zero_byte_fin_is_sent_until_stream_ack_carries_final_offset() {
        let mut send = RetainedByteSendBuffer::default();
        send.append(0, false, b"abc".to_vec()).unwrap();
        let data = send.peek_next(16).unwrap();
        send.mark_sent(&data);
        send.ack(3);

        send.append(3, true, Vec::new()).unwrap();
        assert!(!send.is_finished());
        let fin = send.peek_next(16).unwrap();
        assert!(fin.fin);
        assert_eq!(fin.offset, 3);
        assert!(fin.bytes.is_empty());
        send.mark_sent(&fin);
        assert!(!send.is_finished());

        send.apply_stream_ack(3, &[], 4096, Some(3));
        assert!(send.is_finished());
    }
}

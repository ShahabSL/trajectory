use trajectory_core::auth::ClientAccessKey;
use trajectory_core::codec::{
    open_packet_with_key, seal_packet, AckRange, Direction, Frame, Packet,
};
use trajectory_core::dns::{
    build_query, build_txt_response, envelope_to_qname, parse_query, parse_txt_response,
    qname_to_envelope,
};
use trajectory_core::engine::{ack_ranges_contain, PacketHistory, StreamAssembler};

#[test]
fn encrypted_packet_roundtrip_rejects_wrong_direction() {
    let key = ClientAccessKey::generate();
    let mut packet = Packet::new(100, 7);
    packet.ack_ranges.push(AckRange { first: 1, last: 3 });
    packet.frames.push(Frame::Data {
        stream_id: 1,
        offset: 0,
        fin: true,
        bytes: b"hello".to_vec(),
    });

    let sealed = seal_packet(&key, Direction::ClientToServer, &packet).unwrap();
    let opened = open_packet_with_key(&key, Direction::ClientToServer, &sealed).unwrap();
    assert_eq!(opened, packet);
    assert!(open_packet_with_key(&key, Direction::ServerToClient, &sealed).is_err());
}

#[test]
fn dns_carrier_roundtrip() {
    let payload = b"secret encrypted packet bytes";
    let qname = envelope_to_qname(payload, "tun.example.com").unwrap();
    assert!(qname.len() <= 253);
    assert!(qname.split('.').all(|label| label.len() <= 63));
    assert_eq!(
        qname_to_envelope(&qname, "tun.example.com").unwrap(),
        payload
    );

    let query = build_query(123, &qname, 1232).unwrap();
    let parsed = parse_query(&query).unwrap();
    let response = build_txt_response(&parsed, payload, 0).unwrap();
    assert_eq!(parse_txt_response(&response).unwrap(), payload);
}

#[test]
fn reliability_primitives_handle_sparse_ack_and_reorder() {
    let mut history = PacketHistory::default();
    for packet_no in [1, 2, 4, 9, 10] {
        history.insert(packet_no);
    }
    let ranges = history.ack_ranges(8);
    assert!(ack_ranges_contain(&ranges, 10));
    assert!(ack_ranges_contain(&ranges, 4));
    assert!(!ack_ranges_contain(&ranges, 8));

    let mut assembler = StreamAssembler::default();
    assert!(assembler.insert(6, false, b"world".to_vec()).is_empty());
    assert_eq!(
        assembler.insert(0, false, b"hello ".to_vec()),
        b"hello world"
    );
}

#[test]
fn packet_wire_can_carry_multiple_streams_on_one_connection() {
    let key = ClientAccessKey::generate();
    let mut packet = Packet::new(0xfeed_cafe, 42);
    packet.frames.push(Frame::Open {
        stream_id: 7,
        host: "first.example".into(),
        port: 443,
    });
    packet.frames.push(Frame::Open {
        stream_id: 11,
        host: "second.example".into(),
        port: 443,
    });
    packet.frames.push(Frame::Data {
        stream_id: 7,
        offset: 0,
        fin: false,
        bytes: b"first".to_vec(),
    });
    packet.frames.push(Frame::Data {
        stream_id: 11,
        offset: 0,
        fin: false,
        bytes: b"second".to_vec(),
    });

    let sealed = seal_packet(&key, Direction::ClientToServer, &packet).unwrap();
    let opened = open_packet_with_key(&key, Direction::ClientToServer, &sealed).unwrap();
    assert_eq!(opened.conn_id, packet.conn_id);
    assert_eq!(opened.frames, packet.frames);
}

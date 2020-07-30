pub struct PacketHeader {
    kind: PacketKind,
    extension: u8,
    connection_id: u16,
    timestamp_us: u32,
    timestamp_diff_us: u32,
    window_size: u32,
    seq_number: u16,
    ack_number: u16,
    extensions: Vec<Ext>,
}

pub enum PacketKind {
    Data,
    Fin,
    State,
    Reset,
    Syn,
}

pub struct Ext {
    kind: ExtKind,
    len: u8,
    payload: Vec<u8>,
}

pub enum ExtKind {
    None,
    SelectiveAck,
    Unknown,
}

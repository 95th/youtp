use crate::transport::Transport;

pub struct UtpSocket<T: Transport> {
    transport: T,
    state: State,
    recv_conn_id: u32,
    send_conn_id: u32,
    window_size: u32,
    seq_number: u16,
    ack_number: u16,
}

enum State {
    SynSent,
    SynReceived,
    State,
    Connected,
}

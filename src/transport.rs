use std::{
    io,
    net::SocketAddr,
    task::{Context, Poll},
};

/// A datagram transport.
pub trait Transport {
    /// Send given buffer to the target socket address
    ///
    /// Returns `Poll::Pending` if the transport was not ready.
    /// Otherwise returns the number of bytes sent.
    fn poll_send_to(
        &self,
        cx: &mut Context<'_>,
        buf: &[u8],
        target: &SocketAddr,
    ) -> Poll<io::Result<usize>>;

    /// Attempt to receive data in the provided buffer.
    ///
    /// Returns `Poll::Pending` if the transport was not ready.
    /// Otherwise returns the number of bytes received and
    /// the sender address.
    fn poll_recv_from(
        &self,
        cx: &mut Context<'_>,
        buf: &mut [u8],
    ) -> Poll<io::Result<(usize, SocketAddr)>>;
}

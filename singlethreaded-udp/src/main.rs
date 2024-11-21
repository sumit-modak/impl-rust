use std::net::UdpSocket;
use std::os::fd::AsFd;

fn main() -> std::io::Result<()> {
    println!("PID: {:?}", std::process::id());
    let socket = UdpSocket::bind("127.0.0.1:7879")?;
    println!("Socket fd: {:?}", socket.as_fd());

    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    let mut buf = Vec::with_capacity(65535);
    let (amt, src) = socket.recv_from(&mut buf)?;

    // Redeclare `buf` as slice of the received data and send reverse data back to origin.
    let buf = &mut buf[..amt];
    buf.reverse();
    socket.send_to(buf, &src)?;

    // Socket is closed here
    drop(socket);

    Ok(())
}

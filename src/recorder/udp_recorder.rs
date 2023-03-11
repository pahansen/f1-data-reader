use std::fs::File;
use std::io::Write;
use std::net::UdpSocket;

pub fn record(f1_log_path: &str) -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:20777")?; // for UDP4
    let mut buf = [0; 2048];
    let mut file = File::create(f1_log_path)?;

    loop {
        let (number_of_bytes, _src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
        file.write_all(&buf[..number_of_bytes])?;
    }
}

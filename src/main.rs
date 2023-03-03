// use std::fs::File;
// use std::io::Write;
// use std::net::UdpSocket;

// fn main() -> std::io::Result<()> {
//     let socket = UdpSocket::bind("0.0.0.0:20777")?; // for UDP4
//                                                     //let socket = UdpSocket::bind("[::]:2000")?;  // for UDP4/6
//     let mut buf = [0; 2048];
//     let mut file = File::create("/workspaces/f1-data-reader/f1_logs/foo1.binlog")?;

//     loop {
//         let (number_of_bytes, _src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
//         file.write_all(&buf[..number_of_bytes])?;
//     }
// }

use std::fs::File;
use std::io::BufReader;
use std::io::Read;

fn main() {
    let my_buf =
        BufReader::new(File::open("/workspaces/f1-data-reader/f1_logs/foo1.binlog").unwrap());
    let bytes = my_buf.bytes();
    for byte_or_error in bytes {
        let byte = byte_or_error.unwrap();
        println!("{:b}", byte);
    }
}

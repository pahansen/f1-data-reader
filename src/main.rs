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

// use binrw::BinRead;

// #[derive(BinRead)]
// #[br(little)]
// struct PacketHeader {
//     m_packet_format: u16,
//     m_game_major_version: u8,
//     m_game_minor_version: u8,
//     m_packet_version: u8,
//     m_packet_id: u8,
//     m_session_uid: u64,
//     m_session_time: f64,
//     m_frame_identifier: u32,
//     m_player_car_index: u8,
//     m_secondaryPlayerCarIndex: u8,
// }

// fn main() -> std::io::Result<()> {
//     let mut file = std::fs::File::open("/workspaces/f1-data-reader/f1_logs/foo1.bin")?;

//     loop {
//         // Read a single UDP message from the file
//         let message = PacketHeader::read(&mut file);

//         // Print the ID, flag, and value of the message
//         println!("{}", message.m_packet_format);
//     }
// }

use std::io::Seek;

use binrw::BinRead;

#[derive(Debug, BinRead)]
#[br(little)]
struct PacketHeader {
    m_packet_format: u16,
    m_game_major_version: u8,
    m_game_minor_version: u8,
    m_packet_version: u8,
    m_packet_id: u8,
    m_session_uid: u64,
    m_session_time: f32,
    m_frame_identifier: u32,
    m_player_car_index: u8,
    m_secondaryPlayerCarIndex: u8,
}

fn main() -> std::io::Result<()> {
    let mut file = std::fs::File::open("/workspaces/f1-data-reader/f1logs/foo1.bin")?;
    while let Ok(message) = PacketHeader::read(&mut file) {
        println!(
            "packet_id: {}, session_uid: {}",
            message.m_packet_id, message.m_session_uid
        );
        // Skip messages that are not implemented
        match message.m_packet_id {
            0 => file.seek(std::io::SeekFrom::Current(1440))?,
            1 => file.seek(std::io::SeekFrom::Current(608))?,
            2 => file.seek(std::io::SeekFrom::Current(948))?,
            3 => file.seek(std::io::SeekFrom::Current(16))?,
            4 => file.seek(std::io::SeekFrom::Current(1233))?,
            5 => file.seek(std::io::SeekFrom::Current(1078))?,
            6 => file.seek(std::io::SeekFrom::Current(1323))?,
            7 => file.seek(std::io::SeekFrom::Current(1034))?,
            8 => file.seek(std::io::SeekFrom::Current(991))?,
            9 => file.seek(std::io::SeekFrom::Current(1167))?,
            10 => file.seek(std::io::SeekFrom::Current(924))?,
            11 => file.seek(std::io::SeekFrom::Current(1131))?,

            _ => 0, // Do nothing
        };
    }
    Ok(())
}

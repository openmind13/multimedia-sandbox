use std::io::Read;
use std::net;

use ac_ffmpeg::format::demuxer;

fn main() {
    println!("multimedia-rs");

    listen_udp_socket();
}

const ADDRESS: &str = "localhost:3000";

fn listen_udp_socket() {
    // let socket = net::UdpSocket::bind(ADDRESS).unwrap();
    // println!("listen {}", ADDRESS);

    let mut input = Input::new(ADDRESS);
    println!("listen {}", ADDRESS);

    let ffmpeg_io = ac_ffmpeg::format::io::IO::from_read_stream(input);

    let demuxer_handle = std::thread::spawn(move || {
        // TODO build with stream info
        let mut demuxer = match ac_ffmpeg::format::demuxer::Demuxer::builder().build(ffmpeg_io) {
            Ok(demuxer) => demuxer,
            Err(e) => {
                println!("error in building demuxer: {}", e);
                return;
            }
        };

        // let demuxer = demuxer.find_stream_info(None).map_err(|(_, err)| err);

        loop {
            match demuxer.take() {
                Ok(Some(packet)) => {
                    println!("have packet");
                }
                Ok(None) => {
                    println!("empty packet");
                }
                Err(err) => {
                    println!("got error from demuxer {}", err);
                }
            }
        }
    });

    println!("after building");
    // let remuxer = demuxer::Demuxer::take

    // loop {
    //     let mut buf = [0u8; 1500];
    //     input.read(&mut buf).unwrap();
    // }

    demuxer_handle.join().unwrap();
}

type InputStream = ac_ffmpeg::format::demuxer::DemuxerWithStreamInfo<Input>;

#[derive(Debug)]
struct Input {
    udp_socket: net::UdpSocket,
}

impl Input {
    pub fn new(addr: &str) -> Self {
        Input {
            udp_socket: net::UdpSocket::bind(addr).unwrap(),
        }
    }
}

impl std::io::Read for Input {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        match self.udp_socket.recv_from(buf) {
            Ok((data_len, addr)) => {
                println!("{} bytes from {}", data_len, addr);
                Ok(data_len)
            }
            Err(err) => Err(err),
        }
    }
}

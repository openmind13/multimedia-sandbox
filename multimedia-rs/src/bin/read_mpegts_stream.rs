use ac_ffmpeg::{format::demuxer, Error};
use std::{io::Read, net};

fn main() {
    println!("read mpegts stream");
}

const ADDRESS: &str = "127.0.0.1:3000";
fn listen_udp_socket() {
    let input = Input::new(ADDRESS);
    println!("listen {}", ADDRESS);

    let ffmpeg_io = ac_ffmpeg::format::io::IO::from_read_stream(input);

    let demuxer_handle = std::thread::spawn(move || {
        let mut demuxer = get_demuxer(ffmpeg_io).unwrap();
        for (index, stream) in demuxer.streams().iter().enumerate() {
            let params = stream.codec_parameters();
            println!("Stream #{}:", index);
            println!("  duration: {}", stream.duration().as_f64().unwrap_or(0f64));
        }
    });

    println!("after building");
    demuxer_handle.join().unwrap();
}

fn get_demuxer<T: Read>(
    io: ac_ffmpeg::format::io::IO<T>,
) -> Result<ac_ffmpeg::format::demuxer::DemuxerWithStreamInfo<T>, Error> {
    demuxer::Demuxer::builder()
        .build(io)?
        .find_stream_info(None)
        .map_err(|(_, err)| err)
}

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

impl Read for Input {
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

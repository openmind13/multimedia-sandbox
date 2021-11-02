use ac_ffmpeg::{
    codec::{self, Decoder},
    format::{
        demuxer::{Demuxer, DemuxerWithStreamInfo, SeekTarget},
        io::IO,
    },
    time::Timestamp,
    Error,
};

use std::fs::File;

fn main() {
    let file_mp4 = "../bbb_video.mp4";
    let file_mpegts = "../bbb_video.ts";
    if let Err(err) = print_info(file_mpegts) {
        eprintln!("Error: {}", err);
    }
}

fn open_input(path: &str) -> Result<DemuxerWithStreamInfo<File>, Error> {
    let input = File::open(path)
        .map_err(|err| Error::new(format!("unable to open input file {}: {}", path, err)))?;
    let io = IO::from_seekable_read_stream(input);
    Demuxer::builder()
        .build(io)?
        .find_stream_info(None)
        .map_err(|(_, err)| err)
}

fn print_info(input: &str) -> Result<(), Error> {
    let mut demuxer = open_input(input)?;
    for (index, stream) in demuxer.streams().iter().enumerate() {
        let params = stream.codec_parameters();

        println!("Stream #{}", index);
        println!(" duration: {}", stream.duration().as_f64().unwrap_or(0f64));

        if let Some(params) = params.as_audio_codec_parameters() {
            println!("  type: audio");
            println!("  codec: {}", params.decoder_name().unwrap_or("N/A"));
            println!("  sample format: {}", params.sample_format().name());
            println!("  sample rate: {}", params.sample_rate());
            println!("  channels: {}", params.channel_layout().channels());
        } else if let Some(params) = params.as_video_codec_parameters() {
            println!("  type: video");
            println!("  codec: {}", params.decoder_name().unwrap_or("N/A"));
            println!("  width: {}", params.width());
            println!("  height: {}", params.height());
            println!("  pixel format: {}", params.pixel_format().name());
        } else {
            println!("  type unknown");
        }
    }

    println!("\nSeeking to START + 5s...");
    demuxer.seek_to_timestamp(Timestamp::from_secs(5), SeekTarget::From)?;
    println!("\nPackets:");

    while let Some(packet) = demuxer.take()? {
        // if packet.stream_index() == 0 {
        //     println!("index == 0");
        // }
        println!(
            "  packet (stream #{}, timestamp: {})",
            packet.stream_index(),
            packet.pts().as_f32().unwrap_or(0f32)
        );
    }

    Ok(())
}

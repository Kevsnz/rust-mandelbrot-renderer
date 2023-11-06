use ffmpeg_next::{
    codec, encoder, format,
    frame::{self},
    picture, Packet, Rational,
};

pub(crate) const FRAME_RATE: i32 = 60;
pub(crate) const BITRATE: usize = 25_000_000;


pub struct SetEncoder {
    octx: format::context::Output,
    encoder: encoder::video::Encoder,
    converter: ffmpeg_next::software::scaling::context::Context,
    frame_counter: i64,
}

impl SetEncoder {
    pub fn new(file: &str, w: u32, h: u32) -> SetEncoder {
        let mut octx = format::output(&file).unwrap();
        let mut ost = octx
            .add_stream(ffmpeg_next::encoder::find(codec::Id::H264).unwrap())
            .unwrap();

        let mut encoder = ost.codec().encoder().video().unwrap();
        encoder.set_width(w);
        encoder.set_height(h);
        encoder.set_format(format::Pixel::YUV420P);
        encoder.set_frame_rate(Some(Rational::new(FRAME_RATE, 1)));
        encoder.set_time_base(Rational::new(1, FRAME_RATE));
        encoder.set_bit_rate(BITRATE);
        ost.set_parameters(&encoder);

        let mut encoder = encoder.open().unwrap();
        encoder.set_parameters(ost.parameters()).unwrap();

        let converter = ffmpeg_next::software::scaling::context::Context::get(
            format::Pixel::RGBA,
            w,
            h,
            encoder.format(),
            encoder.width(),
            encoder.height(),
            ffmpeg_next::software::scaling::flag::Flags::FAST_BILINEAR,
        )
        .unwrap();

        SetEncoder {
            octx,
            encoder,
            converter,
            frame_counter: 0,
        }
    }

    pub fn open(&mut self) {
        self.octx.write_header().unwrap();
    }

    pub fn add_frame(&mut self, raw_frame: &Vec<u8>, w: u32, h: u32) {
        let mut frame = frame::Video::new(format::Pixel::RGBA, w, h);
        frame.data_mut(0).copy_from_slice(raw_frame);
        frame.set_kind(picture::Type::None);
        let mut frame2 = frame::Video::empty();
        self.converter.run(&frame, &mut frame2).unwrap();
        frame2.set_pts(self.frame_counter.into());

        self.send_frame_to_encoder(&frame2);
        self.receive_and_process_encoded_packets();

        self.frame_counter += 1;
    }

    pub fn finalize(&mut self) {
        self.send_eof_to_encoder();
        self.receive_and_process_encoded_packets();

        self.octx.write_trailer().unwrap();
    }

    fn send_frame_to_encoder(&mut self, frame: &frame::Video) {
        self.encoder.send_frame(frame).unwrap();
    }

    fn send_eof_to_encoder(&mut self) {
        self.encoder.send_eof().unwrap();
    }

    fn receive_and_process_encoded_packets(&mut self) {
        let mut encoded = Packet::empty();
        while self.encoder.receive_packet(&mut encoded).is_ok() {
            encoded.set_stream(0);
            encoded.rescale_ts(
                Rational::new(1, FRAME_RATE),
                self.octx.stream(0).unwrap().time_base(),
            );
            encoded.write_interleaved(&mut self.octx).unwrap();
        }
    }
}

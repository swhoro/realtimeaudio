use std::error::Error;

use cpal::SupportedStreamConfig;

struct Decoder {
    opus_decoder: opus::Decoder,
}

impl Decoder {
    pub fn new(sample_rate: u32, channel: opus::Channels) -> Result<Decoder, opus::Error> {
        Ok(Decoder {
            opus_decoder: opus::Decoder::new(sample_rate, channel)?,
        })
    }
}

impl Decoder {
    pub fn decode(
        &mut self,
        play_config: &SupportedStreamConfig,
        data: &[u8],
    ) -> Result<usize, Box<dyn Error>> {
        self.opus_decoder
            .decode_float(data, decoded_data, end_of_stream)?
    }
}

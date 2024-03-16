use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BuildStreamError, Device, Host, Sample, SampleFormat, SizedSample, Stream,
    SupportedStreamConfig,
};
use std::error::Error;

use crate::sample_type::SampleType;

const MAX_OUT_SIZE: usize = 3 * 1276;

pub struct Encoder {
    opus_encoder: opus::Encoder,
}

impl Encoder {
    pub fn new(
        sample_rate: u32,
        channel: opus::Channels,
        mode: opus::Application,
    ) -> Result<Encoder, opus::Error> {
        Ok(Encoder {
            opus_encoder: opus::Encoder::new(sample_rate, channel, mode)?,
        })
    }

    pub fn encode(
        &mut self,
        record_config: &SupportedStreamConfig,
        data: Vec<SampleType>,
    ) -> Result<Vec<u8>, Box<dyn Error>> {
        // cut to 10ms * record_config.sample_rate().0
        let frame_size =
            record_config.channels() as usize * record_config.sample_rate().0 as usize / 1000 * 10;

        let encoded_data = match record_config.sample_format() {
            cpal::SampleFormat::I16 => {
                let data: Vec<i16> = data.iter().map(|&s| s.into()).collect::<Vec<_>>();
                let mut framed_data: Vec<Vec<i16>> = Vec::new();
                let mut i = 0;
                while i < data.len() {
                    if i + frame_size as usize > data.len() {
                        break;
                    }
                    let frame_data = data[i..i + frame_size as usize].to_vec();
                    framed_data.push(frame_data);
                    i += frame_size;
                }
                let mut out = Vec::new();
                for frame_data in framed_data {
                    out.append(
                        &mut self
                            .opus_encoder
                            .encode_vec(frame_data.as_ref(), MAX_OUT_SIZE)?,
                    );
                }
                out
            }
            cpal::SampleFormat::F32 => {
                let data: Vec<f32> = data.iter().map(|&s| s.into()).collect::<Vec<_>>();
                let mut framed_data: Vec<Vec<f32>> = Vec::new();
                let mut i = 0;
                while i < data.len() {
                    if i + frame_size as usize > data.len() {
                        break;
                    }
                    let frame_data = data[i..i + frame_size as usize].to_vec();
                    framed_data.push(frame_data);
                    i += frame_size;
                }
                let mut out = Vec::new();
                for frame_data in framed_data {
                    out.append(
                        &mut self
                            .opus_encoder
                            .encode_vec_float(frame_data.as_ref(), MAX_OUT_SIZE)?,
                    );
                }
                out
            }
            sample_format => panic!("Unsupported sample format '{sample_format}'"),
        };

        Ok(encoded_data)
    }
}

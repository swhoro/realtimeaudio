use config::Config;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BuildStreamError, Device, Host, Sample, SizedSample,
};
use err::RError;
use opus;
use std::{
    error::Error,
    sync::{mpsc, Arc, Mutex},
    thread,
};

mod config;
mod encoder;
mod err;
mod record;
mod sample_type;
mod decoder;
use crate::{encoder::Encoder, record::Record, sample_type::SampleType};

const MAX_FRAME_SIZE: usize = 6 * 960;

fn play_encoded_data(host: Host, data: Vec<Vec<u8>>) -> Result<(), Box<dyn Error>> {
    // ready to play
    let play_device = host
        .default_output_device()
        .expect("no output device available");
    let play_config = play_device
        .default_output_config()
        .expect("no output config available");

    println!("play framerate: {}", play_config.sample_rate().0);
    let (tx, rx) = mpsc::sync_channel(2);

    println!("Begin playing...");
    let stream = play_device
        .build_output_stream(
            &play_config.clone().into(),
            move |data: &mut [f32], _| {
                let play_data: Vec<f32> = match rx.recv() {
                    Ok(value) => value,
                    Err(_) => return,
                };
                // println!("play data with len:{}", play_data.len());
                let mut i = 0;
                for sample in data.iter_mut() {
                    if i >= play_data.len() {
                        return;
                    }
                    *sample = play_data[i];
                    i += 1;
                }
            },
            |err| println!("an error occurred on stream: {}", err),
            None,
        )
        .expect("build stream error");

    let mut decoder =
        opus::Decoder::new(play_config.sample_rate().0, opus::Channels::Stereo).unwrap();
    stream.play()?;

    let mut i = 0;
    println!("data len: {}", data.len());
    loop {
        if i >= data.len() {
            break;
        }

        // println!("write data: {}", i);
        let this_frame = &data[i];
        let mut decoded_data = vec![0.0; MAX_FRAME_SIZE];
        decoder.decode_float(this_frame, decoded_data.as_mut(), false)?;
        tx.send(decoded_data)?;
        i += 1;
    }

    println!("end write");
    std::thread::sleep(std::time::Duration::from_secs(1));
    println!("end sleep");
    drop(tx);
    drop(stream);
    println!("dropped stream");
    println!("Done!");
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    // begin recording
    let host = cpal::default_host();
    let config = Config::new(host);

    let record_device = match &config.record_device {
        Some(device) => device,
        None => return Err(RError::DeviceError("no record device".to_string()).into()),
    };
    let record_device_config = match &config.record_device_config {
        Some(config) => config,
        None => return Err(RError::DeviceError("no record config".to_string()).into()),
    };

    let record = Record::new(record_device, record_device_config)?;

    thread::sleep(std::time::Duration::from_secs(5));
    record.pause()?;
    let data = record.collect();

    // encoding
    let channel = match record_device_config.channels() {
        1 => opus::Channels::Mono,
        2 => opus::Channels::Stereo,
        _ => opus::Channels::Stereo,
    };
    let mut encoder = Encoder::new(
        record_device_config.sample_rate().0,
        channel,
        opus::Application::Audio,
    )?;
    let encoded_data = encoder.encode(record_device_config, data).unwrap();

    // decoding

    // let holder: Vec<SampleType> = Vec::new();
    // let holder = Arc::new(Mutex::new(holder));
    // let holder_2 = holder.clone();

    // println!("Begin recording...");
    // let stream = match record_config.sample_format() {
    //     cpal::SampleFormat::I16 => build_stream::<i16>(holder_2, &record_device, &record_config)?,
    //     cpal::SampleFormat::F32 => build_stream::<f32>(holder_2, &record_device, &record_config)?,
    //     sample_format => panic!("Unsupported sample format '{sample_format}'"),
    // };
    // stream.play()?;
    // std::thread::sleep(std::time::Duration::from_secs(5));
    // drop(stream);
    // println!("Done!");

    // cut to 10ms * record_config.
    // let frame_size: usize =
    //     record_config.channels() as usize * record_config.sample_rate().0 as usize / 1000 * 10;

    // // encoding
    // println!("Begin encoding...");
    // let mut encoder = opus::Encoder::new(
    //     record_config.sample_rate().0,
    //     opus::Channels::Stereo,
    //     opus::Application::Audio,
    // )?;

    // let encoded_data = match record_config.sample_format() {
    //     cpal::SampleFormat::I16 => {
    //         let data: Vec<i16> = holder
    //             .lock()
    //             .unwrap()
    //             .iter()
    //             .map(|&s| s.into())
    //             .collect::<Vec<_>>();
    //         let mut modified_data: Vec<Vec<i16>> = Vec::new();
    //         let mut i = 0;
    //         while i < data.len() {
    //             if i + frame_size as usize > data.len() {
    //                 break;
    //             }
    //             let frame_data = data[i..i + frame_size as usize].to_vec();
    //             modified_data.push(frame_data);
    //             i += frame_size;
    //         }
    //         let mut out = Vec::new();
    //         for frame_data in modified_data {
    //             out.push(encoder.encode_vec(frame_data.as_ref(), MAX_OUT_SIZE)?);
    //         }
    //         out
    //     }
    //     cpal::SampleFormat::F32 => {
    //         let data: Vec<f32> = holder
    //             .lock()
    //             .unwrap()
    //             .iter()
    //             .map(|&s| s.into())
    //             .collect::<Vec<_>>();
    //         let mut modified_data: Vec<Vec<f32>> = Vec::new();
    //         let mut i = 0;
    //         while i < data.len() {
    //             if i + frame_size as usize > data.len() {
    //                 break;
    //             }
    //             let frame_data = data[i..i + frame_size as usize].to_vec();
    //             modified_data.push(frame_data);
    //             i += frame_size;
    //         }
    //         let mut out = Vec::new();
    //         for frame_data in modified_data {
    //             out.push(encoder.encode_vec_float(frame_data.as_ref(), MAX_OUT_SIZE)?);
    //         }
    //         out
    //     }
    //     sample_format => panic!("Unsupported sample format '{sample_format}'"),
    // };
    // println!("Done!");

    // play_encoded_data(host, encoded_data).expect("Failed to play encoded data");

    Ok(())
}

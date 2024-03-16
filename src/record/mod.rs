use cpal::{
    traits::{DeviceTrait, StreamTrait},
    BuildStreamError, Device, Sample, SizedSample, Stream, SupportedStreamConfig,
};
use std::{
    error::Error,
    mem,
    sync::{mpsc, mpsc::Sender, Arc, Mutex},
    thread,
};

use crate::sample_type::SampleType;

#[derive(PartialEq)]
enum Signal {
    START,
    PAUSE,
    STOP,
}

pub struct Record {
    stream: Stream,
    data: Arc<Mutex<Vec<SampleType>>>,
    signal: Sender<Signal>,
}

fn build_stream<T>(
    send: Sender<Vec<SampleType>>,
    device: &Device,
    config: &cpal::SupportedStreamConfig,
) -> Result<cpal::Stream, BuildStreamError>
where
    T: Sample + Into<SampleType> + SizedSample,
{
    let this_config = config.clone();
    device.build_input_stream(
        &this_config.into(),
        move |data: &[T], _| {
            let data = data.iter().map(|&sample| sample.into()).collect();
            send.send(data);
        },
        move |err| println!("an error occurred on stream: {}", err),
        None,
    )
}

impl Record {
    pub fn new(
        device: &Device,
        record_config: &SupportedStreamConfig,
    ) -> Result<Record, Box<dyn Error>> {
        let (send_data, recv_data) = mpsc::channel();

        let stream = match record_config.sample_format() {
            cpal::SampleFormat::I16 => build_stream::<i16>(send_data, &device, &record_config)?,
            cpal::SampleFormat::F32 => build_stream::<f32>(send_data, &device, &record_config)?,
            sample_format => return Err(format!("Unsupported sample format {}",sample_format).into()),
        };
        stream.pause()?;

        let data = Arc::new(Mutex::new(Vec::new()));
        let data_write = data.clone();
        let (send_signal, recv_signal) = mpsc::channel();

        thread::spawn(move || loop {
            if let Ok(s) = recv_signal.try_recv() {
                match s {
                    Signal::START => (),
                    Signal::PAUSE => continue,
                    Signal::STOP => break,
                };
            };

            if let Ok(mut data) = recv_data.try_recv() {
                if let Ok(mut data_vec) = data_write.lock() {
                    data_vec.append(data.as_mut())
                }
            };
        });

        Ok(Record {
            stream: stream,
            data: data,
            signal: send_signal,
        })
    }

    pub fn play(&self) -> Result<(), Box<dyn Error>> {
        self.stream.play()?;
        self.signal.send(Signal::START)?;
        Ok(())
    }

    pub fn pause(&self) -> Result<(), Box<dyn Error>> {
        self.stream.pause()?;
        self.signal.send(Signal::PAUSE)?;
        Ok(())
    }

    pub fn collect(&self) -> Vec<SampleType> {
        mem::replace(&mut *self.data.lock().unwrap(), Vec::new())
    }
}

impl Drop for Record {
    fn drop(&mut self) {
        self.signal.send(Signal::STOP);
    }
}

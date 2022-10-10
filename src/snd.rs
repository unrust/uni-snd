use std;
use std::sync::mpsc::{channel, Receiver, Sender};

use super::{SoundError, SoundGenerator};
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{self, Device, Sample, SampleFormat, Stream};
use uni_app::App;

/// This is the sound API that allows you to send events to your generator.
pub struct SoundDriver<T: Send + 'static> {
    config: Option<cpal::StreamConfig>,
    tx: Option<Sender<T>>,
    generator: Option<Box<dyn SoundGenerator<T>>>,
    device: Option<Device>,
    format: Option<SampleFormat>,
    stream: Option<Stream>,
    err: SoundError,
}

impl<T: Send + 'static> SoundDriver<T> {
    /// After calling [`SoundDriver::new`], you can call this function to see if the audio initialization was a success.
    pub fn get_error(&self) -> SoundError {
        self.err
    }

    /// Initialize the sound device and provide the generator to the driver.
    pub fn new(generator: Box<dyn SoundGenerator<T>>) -> Self {
        let host = cpal::default_host();
        let mut stream_config = None;
        let mut err = SoundError::NoError;
        let mut device = None;
        let mut format = None;
        if let Some(dev) = host.default_output_device() {
            match dev.default_output_config() {
                Ok(config) => {
                    App::print(format!(
                        "sound device : {} {:?}\n",
                        dev.name().unwrap_or_else(|_| "?".to_owned()),
                        config
                    ));
                    format = Some(config.sample_format());
                    stream_config = Some(config.into());
                    device = Some(dev);
                }
                Err(e) => {
                    err = SoundError::UnknownStreamFormat;
                    App::print(format!(
                        "error : uni-snd - could not get default output configuration : {:?}\n",
                        e
                    ));
                }
            }
        } else {
            err = SoundError::NoDevice;
            App::print("warning : no sound device detected\n");
        }
        Self {
            config: stream_config,
            tx: None,
            device,
            format,
            stream: None,
            generator: Some(generator),
            err,
        }
    }
    /// Send an event to the generator
    pub fn send_event(&mut self, event: T) {
        if let Some(ref mut tx) = self.tx {
            tx.send(event).unwrap();
        }
    }
    fn get_sample_rate(&self) -> f32 {
        if let Some(ref config) = self.config {
            config.sample_rate.0 as f32
        } else {
            1.0
        }
    }
    /// This will call the generator init function.
    /// It starts the sound thread and the audio loop.
    pub fn start(&mut self) {
        if self.config.is_none() || self.device.is_none() || self.generator.is_none() {
            App::print("no sound");
            return;
        }
        let (tx, rx) = channel();
        self.tx = Some(tx);
        let sample_rate = self.get_sample_rate();
        let config = self.config.take().unwrap();
        let device = self.device.take().unwrap();
        let mut generator = self.generator.take().unwrap();
        generator.init(sample_rate);

        let stream_res = match self.format {
            Some(SampleFormat::F32) => build_stream::<f32, T>(&device, &config, generator, rx),
            Some(SampleFormat::I16) => build_stream::<i16, T>(&device, &config, generator, rx),
            Some(SampleFormat::U16) => build_stream::<u16, T>(&device, &config, generator, rx),
            None => Err(String::new()),
        };
        match stream_res {
            Ok(str) => {
                App::print("starting audio loop\n");
                str.play().unwrap_or_else(|e| {
                    App::print(format!("error : uni-snd - could not start play {}", e));
                });
                self.stream = Some(str);
            }
            Err(e) => {
                self.err = SoundError::OutputStream;
                App::print(format!(
                    "error : uni-snd - could not build output stream : {}\n",
                    e
                ))
            }
        }
    }
}

fn build_stream<S, T: Send + 'static>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut generator: Box<dyn SoundGenerator<T>>,
    rx: Receiver<T>,
) -> Result<Stream, String>
where
    S: cpal::Sample,
{
    let channels = config.channels as usize;
    let err_fn = |err| {
        App::print(&format!(
            "error : uni-snd - an error occurred on stream: {}",
            err
        ))
    };
    device
        .build_output_stream(
            config,
            move |data: &mut [S], _: &cpal::OutputCallbackInfo| {
                for event in rx.try_iter() {
                    generator.handle_event(event);
                }
                write_data(data, channels, &mut generator);
            },
            err_fn,
        )
        .map_err(|e| format!("error : uni-snd - could not build output stream {}", e))
}

fn write_data<S, T: Send + 'static>(
    output: &mut [S],
    channels: usize,
    generator: &mut Box<dyn SoundGenerator<T>>,
) where
    S: Sample,
{
    for frame in output.chunks_mut(channels) {
        for sample in frame.iter_mut() {
            let val = generator.next_value();
            *sample = Sample::from::<f32>(&val);
        }
    }
}

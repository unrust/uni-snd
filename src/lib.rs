#![recursion_limit = "256"]
extern crate cpal;
extern crate uni_app;

pub mod snd;
pub use self::snd::*;

#[derive(Debug, Clone, Copy)]
/// error produced when creating the [`SoundDriver`]
pub enum SoundError {
    /// sound initialization was a success
    NoError,
    /// no sound device was found
    NoDevice,
    /// could not create an output stream
    OutputStream,
    /// unsupported output stream format
    UnknownStreamFormat,
}

/// You must provide a struct implementing this trait to the driver.
///
/// This is what generates the samples to be send to the audio output.
pub trait SoundGenerator<T>: Send {
    /// the sound driver calls this function during initialization to provide the audio interface sample rate.
    fn init(&mut self, sample_rate: f32);
    /// Because the sound generator runs in a separate thread,
    /// you can only communicate with it through events using [`SoundDriver::send_event`].
    /// This is where you should handle those events.
    fn handle_event(&mut self, evt: T);
    /// This is the function generating the samples.
    /// Remember this is stereo output, you have to generate samples alternatively for the left and right channels.
    /// Sample values should be between -1.0 and 1.0.
    fn next_value(&mut self) -> f32;
}

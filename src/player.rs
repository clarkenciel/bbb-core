use std::fmt;
use std::fmt::{Display};
use std::panic;
use std::sync::{Arc, Mutex};
use pa;

use signal::ExprSignal;
use sample::Signal;

pub struct Player {
    pa: pa::PortAudio,
    stream_settings: pa::OutputStreamSettings<i8>,
    stream: Option<pa::Stream<pa::NonBlocking, pa::Output<i8>>>,
}

#[derive(Copy, Clone, Debug)]
enum Error {
    Initialization(pa::Error),
    NoDeviceFound
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Initialization(cause) =>
                write!(f, "Failed to initialize PortAudio: {}", cause),
            Error::NoDeviceFound =>
                write!(f, "No audio device could be found"),
        }
    }
}

impl Player {
    pub fn new(sample_rate: f64, buffer_size: u32) -> Result<Self, String> {
        pa::PortAudio::new()
            .map_err(Error::Initialization)
            .and_then(|audio| {
                panic::catch_unwind(|| {
                    audio.default_output_stream_settings(1, sample_rate, buffer_size)
                })
                .map_err(|_| Error::NoDeviceFound)
                .and_then(|result| {
                    result
                        .map(|settings| Player {
                            pa: audio,
                            stream_settings: settings,
                            stream: None,
                        })
                        .map_err(Error::Initialization)
                })
            })
            .map_err(|e| format!("{}", e))
    }

    pub fn play(&mut self, stream: Arc<Mutex<ExprSignal>>) -> Result<(), String> {
        let callback = move |pa::OutputStreamCallbackArgs { buffer, .. }| {
            stream
                .lock()
                .map(|mut stream| for output_sample in buffer.iter_mut() {
                    *output_sample = stream.next()[0];
                })
                .ok();

            pa::Continue
        };

        if let Some(ref mut existing_stream) = self.stream {
            existing_stream.stop().ok();
        }

        self.pa
            .open_non_blocking_stream(self.stream_settings, callback)
            .and_then(|mut stream| stream.start().and(Ok(stream)))
            .map(|stream| { self.stream = Some(stream); })
            .map_err(|e| format!("Could not open audio stream: {}", e))
    }

    pub fn stop(&mut self) -> Result<(), String> {
        let output;
        if let Some(ref mut stream) = self.stream {
            output = stream.stop();
        } else {
            return Ok(());
        }

        output.map(|_| self.stream = None).map_err(|e| {
            format!("Could not stop audio stream: {}", e)
        })
    }
}

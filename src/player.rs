use std::sync::{Arc, Mutex};
use pa;

use signal::ExprSignal;
use sample::Signal;

pub struct Player {
    pa: pa::PortAudio,
    stream_settings: pa::OutputStreamSettings<i8>,
    stream: Option<pa::Stream<pa::NonBlocking, pa::Output<i8>>>,
}

impl Player {
    pub fn new(sample_rate: f64, buffer_size: u32) -> Result<Self, String> {
        pa::PortAudio::new()
            .and_then(|audio| {
                let result = audio.default_output_device().and_then(|device| {
                    audio.device_info(device).and_then(|info| {
                        let latency = info.default_low_output_latency;
                        let params = pa::StreamParameters::new(device, 1, false, latency);
                        Ok(pa::OutputStreamSettings::new(params, sample_rate, buffer_size))
                    })
                });

                match result {
                    Ok(settings) => Ok(Player {
                        pa: audio,
                        stream_settings: settings,
                        stream: None,
                    }),
                    Err(e) => Err(e),
                }
            })
            .map_err(|e| {
                format!(
                    "Could not create new Player due to an error in PortAudio: {}",
                    e
                )
            })
    }

    pub fn play(&mut self, stream: Arc<Mutex<ExprSignal>>) -> Result<(), String> {
        let callback = move |pa::OutputStreamCallbackArgs { buffer, .. }| {
            stream
                .lock()
                .map(|mut stream| {
                    println!("stream time: {}", stream.time);
                    for output_sample in buffer.iter_mut() {
                        let val = stream.next()[0];
                        *output_sample = val;
                    }
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
            return Ok(())
        }

        output
            .map(|_| self.stream = None)
            .map_err(|e| format!("Could not stop audio stream: {}", e))
    }
}

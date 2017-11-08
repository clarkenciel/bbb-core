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
    pub fn new(sample_rate: u32) -> Result<Self, String> {
        pa::PortAudio::new()
            .and_then(|audio| {
                let result = audio.default_output_device().and_then(|device| {
                    audio.device_info(device).and_then(|info| {
                        let latency = info.default_low_output_latency;
                        let params = pa::StreamParameters::new(device, 1, false, latency);
                        Ok(pa::OutputStreamSettings::new(params, sample_rate as f64, 1))
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

    pub fn play(&mut self, stream: Arc<Mutex<ExprSignal>>) {
        let callback = move |pa::OutputStreamCallbackArgs { buffer, .. }| {
            stream.lock().map(|mut stream| {
                for output_sample in buffer.iter_mut() {
                    *output_sample = stream.next()[0];
                }
            }).ok();
            pa::Continue
        };

        if let Some(ref mut existing_stream) = self.stream {
            existing_stream.stop().ok();
        }

        self.stream = Some(
            self.pa
                .open_non_blocking_stream(self.stream_settings, callback)
                .unwrap(),
        );
    }
}

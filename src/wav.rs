use std::error::Error;

use hound::{WavWriter, WavSpec, SampleFormat};
use signal::ExprSignal;
use sample::Signal;

pub struct Recorder {
    spec: WavSpec,
}

impl Recorder {
    pub fn new(sample_rate: u32) -> Self {
        Recorder {
            spec: WavSpec {
                channels: 1,
                sample_rate: sample_rate,
                bits_per_sample: 8,
                sample_format: SampleFormat::Int,
            },
        }
    }

    pub fn record<'a>(
        &self,
        filename: &'a str,
        duration: f32,
        signal: &'a mut ExprSignal,
    ) -> Result<(), String> {
        WavWriter::create(filename, self.spec)
            .and_then(|mut writer| {
                for _ in 0..(self.spec.sample_rate as f32 * duration) as u32 {
                    match writer.write_sample(signal.next()[0]) {
                        Ok(_) => continue,
                        err => return err
                    }
                }
                writer.finalize()
            })
            .map_err(|e| e.description().to_owned())
    }
}

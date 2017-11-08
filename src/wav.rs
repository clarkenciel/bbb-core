use std::error::Error;

use hound::{WavWriter, WavSpec, SampleFormat};
use signal::ExprSignal;
use sample::Signal;
use SAMPLE_RATE;
use TIME_STEP;

pub fn record<'a>(
    filename: &'a str,
    duration: u32,
    signal: &'a mut ExprSignal,
) -> Result<(), String> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    WavWriter::create(filename, spec)
        .and_then(|mut writer| {
            for _ in 0..(SAMPLE_RATE * duration) / TIME_STEP {
                let samp = signal.next()[0];
                for _ in 0..TIME_STEP {
                    match writer.write_sample(samp as i16) {
                        Ok(_) => continue,
                        err => return err
                    }
                }
            }
            writer.finalize()
        })
        .map_err(|e| e.description().to_owned())
}

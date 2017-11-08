use std::error::Error;

use hound::{WavWriter, WavSpec, SampleFormat};
use signal::ExprSignal;
use sample::Signal;
use SAMPLE_RATE;

pub fn record<'a>(
    filename: &'a str,
    duration: u32,
    signal: &'a mut ExprSignal,
) -> Result<(), String> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: SAMPLE_RATE,
        bits_per_sample: 8,
        sample_format: SampleFormat::Int,
    };

    WavWriter::create(filename, spec)
        .and_then(|mut writer| {
            for _ in 0..SAMPLE_RATE * duration {
                let samp = signal.next()[0];
                match writer.write_sample(samp as i8) {
                    Ok(_) => continue,
                    err => return err
                }
            }
            writer.finalize()
        })
        .map_err(|e| e.description().to_owned())
}

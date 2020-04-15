use riff_wave::WaveWriter;
use std::fs::File;
use std::io::{BufReader, BufWriter};

fn main() {
    {
        let duration = 1;
        let frequency = 440.0; //Hz
        let amplitude = i16::max_value() / 4;
        let channels = 1;
        let sample_size = 16;
        let sample_rate = 48000;

        let file = File::create("sine.wav").unwrap();
        let writer = BufWriter::new(file);
        let mut writer = WaveWriter::new(channels, sample_rate, sample_size, writer).unwrap();

        let num_samples = duration * sample_rate;
        let sample_length = 1.0 / sample_rate as f64;
        let b = 2.0 * frequency * std::f64::consts::PI;

        let wav_gen = (0..num_samples).map(|x| {
            let pre_scale = (x as f64 * sample_length * b).sin();
            let post_scale = pre_scale * amplitude as f64;
            post_scale.floor() as i16
        });

        for n in wav_gen {
            writer.write_sample_i16(n).unwrap();
        }

        let amplitude = i16::max_value() / 2;
        let clip_at = i16::max_value() / 4;
        let file = File::create("clipped.wav").unwrap();
        let writer = BufWriter::new(file);
        let mut writer = WaveWriter::new(channels, sample_rate, sample_size, writer).unwrap();

        let wav_gen = (0..num_samples).map(|x| {
            let pre_scale = (x as f64 * sample_length * b).sin();
            let post_scale = pre_scale * amplitude as f64;
            let pre_clip = post_scale.floor() as i16;

            if pre_clip > clip_at {
                clip_at
            } else if pre_clip < clip_at * -1 {
                clip_at * -1
            } else {
                pre_clip
            }
        });

        for n in wav_gen {
            writer.write_sample_i16(n).unwrap();
        }
    }

    let device = rodio::default_output_device().unwrap();
    let sink = rodio::Sink::new(&device);

    /*
    let file = File::open("sine.wav").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);
    */

    let file = File::open("clipped.wav").unwrap();
    let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(source);

    sink.sleep_until_end();
}

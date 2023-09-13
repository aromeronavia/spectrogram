use std::path::Path;

use hound;
use rustfft::algorithm::Radix4;
use rustfft::num_complex::Complex;
use rustfft::{Fft, FftDirection};
use image::{ImageBuffer, Rgb};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    let path = args.get(1).expect("no file");

    let mut reader = hound::WavReader::open(path).expect("Not a valid WAV file");

    let samples: Vec<_> = reader.samples::<i16>().map(|s| s.unwrap() as f32).collect();

    let frame_size = 1024;
    let overlap = frame_size / 2;
    let fft = Radix4::new(frame_size, FftDirection::Forward);

    let img_width = frame_size / 2;
    let img_height = samples.len() / frame_size;
    let mut img = ImageBuffer::new(img_width as u32, img_height as u32);

    for (i, frame) in samples.windows(frame_size).step_by(overlap).enumerate() {
        if i >= img_height {
            break;
        }

        let mut frame2: Vec<Complex<f32>> = frame
            .iter()
            .enumerate()
            .map(|(j, &s)| {
                let window = 0.54
                    - 0.46
                        * (2.0 * std::f32::consts::PI * j as f32 / (frame_size as f32 - 1.0)).cos();
                Complex::new(s * window, 0.0)
            })
            .collect();

        fft.process(&mut frame2);

        for (j, value) in frame2.iter().enumerate().take(img_width) {
            if j >= img_width {
                break;
            }
            let magnitude = (value.norm() * 255.0) as u8;
            img.put_pixel(j as u32, i as u32, Rgb([magnitude, magnitude, magnitude]));
        }
    }

    img.save(Path::new("spectrogram.png")).unwrap();
}

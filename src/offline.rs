use hound::{SampleFormat, WavSpec, WavWriter};

use crate::phys::WombBed;


pub fn render_bed_to_wav(
    path: &str,
    minutes: u32,
    sample_rate: u32,
    seed: u64,
    bpm: f32,
    heart_db: f32,
    noise_db: f32,
) -> anyhow::Result<()> {
    let total_samples = minutes as usize * 60 * sample_rate as usize;
    let mut bed: WombBed = WombBed::new(sample_rate, seed, bpm, heart_db, noise_db);

    // First pass: generate and track peak for simple headroom
    let mut left: Vec<f32> = Vec::with_capacity(total_samples);
    let mut right: Vec<f32> = Vec::with_capacity(total_samples);
    let mut peak = 0.0f32;

    println!("Generating {} samples...", total_samples);

    for _ in 0..total_samples {
        let mono = bed.next_sample();
        // Tiny stereo decorrelation (placeholder)
        let l = mono;
        let r = mono * 0.98;

        peak = peak.max(l.abs()).max(r.abs());
        left.push(l);
        right.push(r);
    }

    // Normalize softly to keep ~-1 dBFS headroom
    let target_peak = 0.89f32;
    let gain = if peak > 0.0 { (target_peak / peak).min(1.0) } else { 1.0 };

    println!("Peak: {:.2} dBFS", 20.0 * peak.log10());
    println!("Target: {:.2} dBFS", 20.0 * target_peak.log10());
    println!("Gain: {:.2}", gain);

    let spec = WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };
    let mut writer = WavWriter::create(path, spec)?;

    println!("Writing samples...");

    for i in 0..total_samples {
        writer.write_sample((left[i]  * gain).clamp(-1.0, 1.0))?;  // f32
        writer.write_sample((right[i] * gain).clamp(-1.0, 1.0))?;
    }
    writer.finalize()?;
    Ok(())
}   




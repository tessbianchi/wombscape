use clap::{Parser, ValueEnum};

mod offline;
mod dsp;
mod phys;


#[derive(Parser, Debug)]
#[command(
    name = "wombscape",
    about = "Offline fetalized audio renderer (voice → womb, or synth a womb bed)",
    version,
    author
)]
struct Args {
    /// Input WAV. If omitted, a synthetic womb bed is generated.
    #[arg(long)]
    input: Option<String>,

    /// Output WAV path (default: out.wav)
    #[arg(long, default_value = "out.wav")]
    output: String,

    /// Minutes of audio (used only if --input is omitted)
    #[arg(long, default_value_t = 1)]
    minutes: u32,

    /// Target sample rate
    #[arg(long, default_value_t = 48_000)]
    sr: u32,

    /// Primary low-pass cutoff (Hz), e.g., 500–800 for “in-womb” vibe
    #[arg(long, default_value_t = 700.0)]
    lp: f32,

    /// Choose a sound “mood” (controls a bunch of internal defaults)
    #[arg(long, value_enum, default_value_t = Preset::Womb)]
    preset: Preset,

    /// Random seed for reproducible renders (synthetic bed, jitter, etc.)
    #[arg(long)]
    seed: Option<u64>,

    /// Print per-bar parameters while rendering (cutoff, heart rate, etc.)
    #[arg(long)]
    explain: bool,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Preset {
    Womb,
    Soft,
    Industrial,
    Choral,
}

fn main() -> anyhow::Result<()> {
    // let args = Args::parse();

    // You’ll wire these to your offline pipeline:
    //   - process input file if provided
    //   - otherwise render synthetic bed for N minutes

    // Settings for a 1-minute render
    let sample_rate = 48_000u32;
    let minutes = 1u32;
    let seed = 42u64;

    // Bed levels: tweak to taste (dBFS)
    let maternal_hr_bpm = 110.0;
    let heart_level_db = -15.0;
    let noise_level_db = -36.0;

    offline::render_bed_to_wav(
        "data/out.wav",  
        minutes,
        sample_rate,
        seed,
        maternal_hr_bpm,
        heart_level_db,
        noise_level_db,
    )?;

    println!("wrote data/out.wav");
    Ok(())

    // if let Some(ref input) = args.input {
    //     println!(
    //         "Processing input file:\n  in: {}\n  out: {}\n minutes {}\n sr: {}\n  lp: {} Hz\n  preset: {:?}\n  explain: {}",
    //         input, args.output, args.minutes, args.sr, args.lp, args.preset, args.explain
    //     );
    //     // offline::process_wav_file(input, &args.output, args.sr, args.lp, args.preset, args.seed, args.explain)?;
    // } else {
    //     println!(
    //         "Rendering synthetic bed:\n  out: {}\n  minutes: {}\n  sr: {}\n  lp: {} Hz\n  preset: {:?}\n  seed: {:?}\n  explain: {}",
    //         args.output, args.minutes, args.sr, args.lp, args.preset, args.seed, args.explain
    //     );
    //     // offline::render_synthetic(&args.output, args.sr, args.minutes, args.lp, args.preset, args.seed, args.explain)?;
    // }

    // Ok(())
}
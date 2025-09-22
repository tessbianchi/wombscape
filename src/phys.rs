use rand::{SeedableRng, rngs::StdRng, Rng};
use crate::dsp::{PinkNoise, AttackDecayEnvelope, EnvState};

/// A synthetic "womb bed" of maternal heartbeat + pink noise + slow breathing.
/// Call `next_sample()` once per audio sample. Works offline or live.
pub struct WombBed {
    sample_rate: u32,
    rng: StdRng,

    // heartbeat parameters
    heart_rate_bpm: f32,
    samples_per_beat: f32,
    sample_counter: u64,

    // heartbeat envelopes ("lub" and "dub")
    lub_envelope: AttackDecayEnvelope,
    dub_envelope: AttackDecayEnvelope,
    dub_offset: f32,

    // noise & breathing
    pink_noise: PinkNoise,
    breathing_phase: f32,

    // gain levels
    heart_level: f32,
    noise_level: f32,
}


impl WombBed {
    pub fn new(
        sample_rate: u32,
        seed: u64,
        heart_rate_bpm: f32,
        heart_level_db: f32,
        noise_level_db: f32,
    ) -> Self {
        let samples_per_beat: f32 = (60.0 / heart_rate_bpm) * sample_rate as f32;
        println!("samples_per_beat: {}", samples_per_beat);
        Self {
            sample_rate,
            rng: StdRng::seed_from_u64(seed),
            heart_rate_bpm,
            samples_per_beat,
            sample_counter: 0,
            lub_envelope: AttackDecayEnvelope::new(sample_rate, 8.0, 70.0),  // attack/decay in ms
            dub_envelope: AttackDecayEnvelope::new(sample_rate, 10.0, 90.0),
            pink_noise: PinkNoise::new(),
            breathing_phase: 0.0,
            heart_level: db_to_linear(heart_level_db),
            noise_level: db_to_linear(noise_level_db),
            dub_offset: 0.12 * sample_rate as f32,
        }
    }

    pub fn set_heart_rate(&mut self, bpm: f32) {
        self.heart_rate_bpm = bpm;
        self.samples_per_beat = (60.0 / bpm) * self.sample_rate as f32;
    }

    /// Generate one mono sample of the womb bed.
    pub fn next_sample(&mut self) -> f32 {
        let position_in_beat = (self.sample_counter as f32) % self.samples_per_beat;

        // println!("sample_counter: {}, position_in_beat: {}", self.sample_counter, position_in_beat);

        // add small timing jitter once per beat
        if position_in_beat < 1.0 {
            let jitter_fraction = self.rng.gen_range(-0.004f32..0.004); // ±0.4%
            let spb = (60.0 / self.heart_rate_bpm)
                * (1.0 + jitter_fraction)
                * self.sample_rate as f32;
            
            self.samples_per_beat = spb;

        }

        // trigger envelopes for "lub" and "dub"
        if position_in_beat < 0.5 && self.lub_envelope.get_state() != EnvState::Attacking {
            println!("lub start");
            // println!("position_in_beat: {}, dub_offset: {}", position_in_beat, self.dub_offset);
            self.lub_envelope.trigger();
        } else if (position_in_beat > self.dub_offset) && (position_in_beat < self.dub_offset + 2.0) && self.dub_envelope.get_state() != EnvState::Attacking {
            // println!("dub envelope state: {:?}", self.dub_envelope.get_state());
            println!("dub start");
            self.dub_envelope.trigger();
        }

        // compute heartbeat envelopes
        let lub = self.lub_envelope.next_value();
        let dub = 0.7 * self.dub_envelope.next_value();
        let heart_thump = (lub + dub) * self.heart_level;

        // pink noise + breathing modulation
        let breath_rate_hz = 0.1; // ~6 breaths per minute
        self.breathing_phase += breath_rate_hz / self.sample_rate as f32;
        if self.breathing_phase > 1.0 { self.breathing_phase -= 1.0; }
        let breathing_mod = 0.85 + 0.15 * (2.0 * std::f32::consts::PI * self.breathing_phase).sin();

        let white = self.rng.r#gen::<f32>()* 2.0 - 1.0;
        let pink = self.pink_noise.next_sample(white) * self.noise_level * breathing_mod;

        self.sample_counter += 1;
        pink + heart_thump
    }
}

fn db_to_linear(db: f32) -> f32 {
    10f32.powf(db / 20.0)
}


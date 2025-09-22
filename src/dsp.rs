/// Pink-ish noise using a one-pole lowpass on white noise.
/// Not perfect 1/f, but good enough for a womb bed.
pub struct PinkNoise {
    state: f32,
}
impl PinkNoise {
    pub fn new() -> Self { Self { state: 0.0 } }
    pub fn next_sample(&mut self, white: f32) -> f32 {
        self.state = 0.985 * self.state + 0.015 * white;
        self.state
    }
}

/// Simple attack-decay envelope for percussive sounds.
pub struct AttackDecayEnvelope {
    attack_coeff: f32,
    decay_coeff: f32,
    level: f32,
    state: EnvState,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnvState { Idle, Attacking, Decaying }

impl AttackDecayEnvelope {
    pub fn new(sr: u32, attack_ms: f32, decay_ms: f32) -> Self {
        let attack_samples = (attack_ms / 1000.0) * sr as f32;
        let decay_samples = (decay_ms / 1000.0) * sr as f32;
        Self {
            attack_coeff: (-1.0 / attack_samples.max(1.0)).exp(),
            decay_coeff: (-1.0 / decay_samples.max(1.0)).exp(),
            level: 0.0,
            state: EnvState::Idle,
        }
    }

    pub fn trigger(&mut self) {
        self.state = EnvState::Attacking;
    }

    pub fn next_value(&mut self) -> f32 {
        match self.state {
            EnvState::Idle => {
                self.level *= self.decay_coeff;
            }
            EnvState::Attacking => {
                self.level = 1.0 + (self.level - 1.0) * self.attack_coeff;
                if self.level > 0.98 {
                    self.state = EnvState::Decaying;
                }
            }
            EnvState::Decaying => {
                self.level *= self.decay_coeff;
                if self.level < 0.0005 {
                    self.level = 0.0;
                    self.state = EnvState::Idle;
                }
            }
        }
        self.level
    }

    pub fn get_state(&self) -> EnvState {
        self.state
    }
}
